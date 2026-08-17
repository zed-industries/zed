#[cfg(feature = "Win32_System_Performance_HardwareCounterProfiling")]
pub mod HardwareCounterProfiling;
#[inline]
pub unsafe fn BackupPerfRegistryToFileW<P0, P1>(szfilename: P0, szcommentstring: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("loadperf.dll" "system" fn BackupPerfRegistryToFileW(szfilename : windows_core::PCWSTR, szcommentstring : windows_core::PCWSTR) -> u32);
    BackupPerfRegistryToFileW(szfilename.param().abi(), szcommentstring.param().abi())
}
#[inline]
pub unsafe fn InstallPerfDllA<P0, P1>(szcomputername: P0, lpinifile: P1, dwflags: usize) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("loadperf.dll" "system" fn InstallPerfDllA(szcomputername : windows_core::PCSTR, lpinifile : windows_core::PCSTR, dwflags : usize) -> u32);
    InstallPerfDllA(szcomputername.param().abi(), lpinifile.param().abi(), dwflags)
}
#[inline]
pub unsafe fn InstallPerfDllW<P0, P1>(szcomputername: P0, lpinifile: P1, dwflags: usize) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("loadperf.dll" "system" fn InstallPerfDllW(szcomputername : windows_core::PCWSTR, lpinifile : windows_core::PCWSTR, dwflags : usize) -> u32);
    InstallPerfDllW(szcomputername.param().abi(), lpinifile.param().abi(), dwflags)
}
#[inline]
pub unsafe fn LoadPerfCounterTextStringsA<P0, P1>(lpcommandline: P0, bquietmodearg: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("loadperf.dll" "system" fn LoadPerfCounterTextStringsA(lpcommandline : windows_core::PCSTR, bquietmodearg : super::super::Foundation:: BOOL) -> u32);
    LoadPerfCounterTextStringsA(lpcommandline.param().abi(), bquietmodearg.param().abi())
}
#[inline]
pub unsafe fn LoadPerfCounterTextStringsW<P0, P1>(lpcommandline: P0, bquietmodearg: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("loadperf.dll" "system" fn LoadPerfCounterTextStringsW(lpcommandline : windows_core::PCWSTR, bquietmodearg : super::super::Foundation:: BOOL) -> u32);
    LoadPerfCounterTextStringsW(lpcommandline.param().abi(), bquietmodearg.param().abi())
}
#[inline]
pub unsafe fn PdhAddCounterA<P0>(hquery: isize, szfullcounterpath: P0, dwuserdata: usize, phcounter: *mut isize) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhAddCounterA(hquery : isize, szfullcounterpath : windows_core::PCSTR, dwuserdata : usize, phcounter : *mut isize) -> u32);
    PdhAddCounterA(hquery, szfullcounterpath.param().abi(), dwuserdata, phcounter)
}
#[inline]
pub unsafe fn PdhAddCounterW<P0>(hquery: isize, szfullcounterpath: P0, dwuserdata: usize, phcounter: *mut isize) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhAddCounterW(hquery : isize, szfullcounterpath : windows_core::PCWSTR, dwuserdata : usize, phcounter : *mut isize) -> u32);
    PdhAddCounterW(hquery, szfullcounterpath.param().abi(), dwuserdata, phcounter)
}
#[inline]
pub unsafe fn PdhAddEnglishCounterA<P0>(hquery: isize, szfullcounterpath: P0, dwuserdata: usize, phcounter: *mut isize) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhAddEnglishCounterA(hquery : isize, szfullcounterpath : windows_core::PCSTR, dwuserdata : usize, phcounter : *mut isize) -> u32);
    PdhAddEnglishCounterA(hquery, szfullcounterpath.param().abi(), dwuserdata, phcounter)
}
#[inline]
pub unsafe fn PdhAddEnglishCounterW<P0>(hquery: isize, szfullcounterpath: P0, dwuserdata: usize, phcounter: *mut isize) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhAddEnglishCounterW(hquery : isize, szfullcounterpath : windows_core::PCWSTR, dwuserdata : usize, phcounter : *mut isize) -> u32);
    PdhAddEnglishCounterW(hquery, szfullcounterpath.param().abi(), dwuserdata, phcounter)
}
#[inline]
pub unsafe fn PdhBindInputDataSourceA<P0>(phdatasource: *mut isize, logfilenamelist: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhBindInputDataSourceA(phdatasource : *mut isize, logfilenamelist : windows_core::PCSTR) -> u32);
    PdhBindInputDataSourceA(phdatasource, logfilenamelist.param().abi())
}
#[inline]
pub unsafe fn PdhBindInputDataSourceW<P0>(phdatasource: *mut isize, logfilenamelist: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhBindInputDataSourceW(phdatasource : *mut isize, logfilenamelist : windows_core::PCWSTR) -> u32);
    PdhBindInputDataSourceW(phdatasource, logfilenamelist.param().abi())
}
#[inline]
pub unsafe fn PdhBrowseCountersA(pbrowsedlgdata: *const PDH_BROWSE_DLG_CONFIG_A) -> u32 {
    windows_targets::link!("pdh.dll" "system" fn PdhBrowseCountersA(pbrowsedlgdata : *const PDH_BROWSE_DLG_CONFIG_A) -> u32);
    PdhBrowseCountersA(pbrowsedlgdata)
}
#[inline]
pub unsafe fn PdhBrowseCountersHA(pbrowsedlgdata: *const PDH_BROWSE_DLG_CONFIG_HA) -> u32 {
    windows_targets::link!("pdh.dll" "system" fn PdhBrowseCountersHA(pbrowsedlgdata : *const PDH_BROWSE_DLG_CONFIG_HA) -> u32);
    PdhBrowseCountersHA(pbrowsedlgdata)
}
#[inline]
pub unsafe fn PdhBrowseCountersHW(pbrowsedlgdata: *const PDH_BROWSE_DLG_CONFIG_HW) -> u32 {
    windows_targets::link!("pdh.dll" "system" fn PdhBrowseCountersHW(pbrowsedlgdata : *const PDH_BROWSE_DLG_CONFIG_HW) -> u32);
    PdhBrowseCountersHW(pbrowsedlgdata)
}
#[inline]
pub unsafe fn PdhBrowseCountersW(pbrowsedlgdata: *const PDH_BROWSE_DLG_CONFIG_W) -> u32 {
    windows_targets::link!("pdh.dll" "system" fn PdhBrowseCountersW(pbrowsedlgdata : *const PDH_BROWSE_DLG_CONFIG_W) -> u32);
    PdhBrowseCountersW(pbrowsedlgdata)
}
#[inline]
pub unsafe fn PdhCalculateCounterFromRawValue(hcounter: isize, dwformat: PDH_FMT, rawvalue1: *const PDH_RAW_COUNTER, rawvalue2: *const PDH_RAW_COUNTER, fmtvalue: *mut PDH_FMT_COUNTERVALUE) -> u32 {
    windows_targets::link!("pdh.dll" "system" fn PdhCalculateCounterFromRawValue(hcounter : isize, dwformat : PDH_FMT, rawvalue1 : *const PDH_RAW_COUNTER, rawvalue2 : *const PDH_RAW_COUNTER, fmtvalue : *mut PDH_FMT_COUNTERVALUE) -> u32);
    PdhCalculateCounterFromRawValue(hcounter, dwformat, rawvalue1, rawvalue2, fmtvalue)
}
#[inline]
pub unsafe fn PdhCloseLog(hlog: isize, dwflags: u32) -> u32 {
    windows_targets::link!("pdh.dll" "system" fn PdhCloseLog(hlog : isize, dwflags : u32) -> u32);
    PdhCloseLog(hlog, dwflags)
}
#[inline]
pub unsafe fn PdhCloseQuery(hquery: isize) -> u32 {
    windows_targets::link!("pdh.dll" "system" fn PdhCloseQuery(hquery : isize) -> u32);
    PdhCloseQuery(hquery)
}
#[inline]
pub unsafe fn PdhCollectQueryData(hquery: isize) -> u32 {
    windows_targets::link!("pdh.dll" "system" fn PdhCollectQueryData(hquery : isize) -> u32);
    PdhCollectQueryData(hquery)
}
#[inline]
pub unsafe fn PdhCollectQueryDataEx<P0>(hquery: isize, dwintervaltime: u32, hnewdataevent: P0) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhCollectQueryDataEx(hquery : isize, dwintervaltime : u32, hnewdataevent : super::super::Foundation:: HANDLE) -> u32);
    PdhCollectQueryDataEx(hquery, dwintervaltime, hnewdataevent.param().abi())
}
#[inline]
pub unsafe fn PdhCollectQueryDataWithTime(hquery: isize, plltimestamp: *mut i64) -> u32 {
    windows_targets::link!("pdh.dll" "system" fn PdhCollectQueryDataWithTime(hquery : isize, plltimestamp : *mut i64) -> u32);
    PdhCollectQueryDataWithTime(hquery, plltimestamp)
}
#[inline]
pub unsafe fn PdhComputeCounterStatistics(hcounter: isize, dwformat: PDH_FMT, dwfirstentry: u32, dwnumentries: u32, lprawvaluearray: *const PDH_RAW_COUNTER, data: *mut PDH_STATISTICS) -> u32 {
    windows_targets::link!("pdh.dll" "system" fn PdhComputeCounterStatistics(hcounter : isize, dwformat : PDH_FMT, dwfirstentry : u32, dwnumentries : u32, lprawvaluearray : *const PDH_RAW_COUNTER, data : *mut PDH_STATISTICS) -> u32);
    PdhComputeCounterStatistics(hcounter, dwformat, dwfirstentry, dwnumentries, lprawvaluearray, data)
}
#[inline]
pub unsafe fn PdhConnectMachineA<P0>(szmachinename: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhConnectMachineA(szmachinename : windows_core::PCSTR) -> u32);
    PdhConnectMachineA(szmachinename.param().abi())
}
#[inline]
pub unsafe fn PdhConnectMachineW<P0>(szmachinename: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhConnectMachineW(szmachinename : windows_core::PCWSTR) -> u32);
    PdhConnectMachineW(szmachinename.param().abi())
}
#[inline]
pub unsafe fn PdhCreateSQLTablesA<P0>(szdatasource: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhCreateSQLTablesA(szdatasource : windows_core::PCSTR) -> u32);
    PdhCreateSQLTablesA(szdatasource.param().abi())
}
#[inline]
pub unsafe fn PdhCreateSQLTablesW<P0>(szdatasource: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhCreateSQLTablesW(szdatasource : windows_core::PCWSTR) -> u32);
    PdhCreateSQLTablesW(szdatasource.param().abi())
}
#[inline]
pub unsafe fn PdhEnumLogSetNamesA<P0>(szdatasource: P0, mszdatasetnamelist: windows_core::PSTR, pcchbufferlength: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhEnumLogSetNamesA(szdatasource : windows_core::PCSTR, mszdatasetnamelist : windows_core::PSTR, pcchbufferlength : *mut u32) -> u32);
    PdhEnumLogSetNamesA(szdatasource.param().abi(), core::mem::transmute(mszdatasetnamelist), pcchbufferlength)
}
#[inline]
pub unsafe fn PdhEnumLogSetNamesW<P0>(szdatasource: P0, mszdatasetnamelist: windows_core::PWSTR, pcchbufferlength: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhEnumLogSetNamesW(szdatasource : windows_core::PCWSTR, mszdatasetnamelist : windows_core::PWSTR, pcchbufferlength : *mut u32) -> u32);
    PdhEnumLogSetNamesW(szdatasource.param().abi(), core::mem::transmute(mszdatasetnamelist), pcchbufferlength)
}
#[inline]
pub unsafe fn PdhEnumMachinesA<P0>(szdatasource: P0, mszmachinelist: windows_core::PSTR, pcchbuffersize: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhEnumMachinesA(szdatasource : windows_core::PCSTR, mszmachinelist : windows_core::PSTR, pcchbuffersize : *mut u32) -> u32);
    PdhEnumMachinesA(szdatasource.param().abi(), core::mem::transmute(mszmachinelist), pcchbuffersize)
}
#[inline]
pub unsafe fn PdhEnumMachinesHA(hdatasource: isize, mszmachinelist: windows_core::PSTR, pcchbuffersize: *mut u32) -> u32 {
    windows_targets::link!("pdh.dll" "system" fn PdhEnumMachinesHA(hdatasource : isize, mszmachinelist : windows_core::PSTR, pcchbuffersize : *mut u32) -> u32);
    PdhEnumMachinesHA(hdatasource, core::mem::transmute(mszmachinelist), pcchbuffersize)
}
#[inline]
pub unsafe fn PdhEnumMachinesHW(hdatasource: isize, mszmachinelist: windows_core::PWSTR, pcchbuffersize: *mut u32) -> u32 {
    windows_targets::link!("pdh.dll" "system" fn PdhEnumMachinesHW(hdatasource : isize, mszmachinelist : windows_core::PWSTR, pcchbuffersize : *mut u32) -> u32);
    PdhEnumMachinesHW(hdatasource, core::mem::transmute(mszmachinelist), pcchbuffersize)
}
#[inline]
pub unsafe fn PdhEnumMachinesW<P0>(szdatasource: P0, mszmachinelist: windows_core::PWSTR, pcchbuffersize: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhEnumMachinesW(szdatasource : windows_core::PCWSTR, mszmachinelist : windows_core::PWSTR, pcchbuffersize : *mut u32) -> u32);
    PdhEnumMachinesW(szdatasource.param().abi(), core::mem::transmute(mszmachinelist), pcchbuffersize)
}
#[inline]
pub unsafe fn PdhEnumObjectItemsA<P0, P1, P2>(szdatasource: P0, szmachinename: P1, szobjectname: P2, mszcounterlist: windows_core::PSTR, pcchcounterlistlength: *mut u32, mszinstancelist: windows_core::PSTR, pcchinstancelistlength: *mut u32, dwdetaillevel: PERF_DETAIL, dwflags: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhEnumObjectItemsA(szdatasource : windows_core::PCSTR, szmachinename : windows_core::PCSTR, szobjectname : windows_core::PCSTR, mszcounterlist : windows_core::PSTR, pcchcounterlistlength : *mut u32, mszinstancelist : windows_core::PSTR, pcchinstancelistlength : *mut u32, dwdetaillevel : PERF_DETAIL, dwflags : u32) -> u32);
    PdhEnumObjectItemsA(szdatasource.param().abi(), szmachinename.param().abi(), szobjectname.param().abi(), core::mem::transmute(mszcounterlist), pcchcounterlistlength, core::mem::transmute(mszinstancelist), pcchinstancelistlength, dwdetaillevel, dwflags)
}
#[inline]
pub unsafe fn PdhEnumObjectItemsHA<P0, P1>(hdatasource: isize, szmachinename: P0, szobjectname: P1, mszcounterlist: windows_core::PSTR, pcchcounterlistlength: *mut u32, mszinstancelist: windows_core::PSTR, pcchinstancelistlength: *mut u32, dwdetaillevel: PERF_DETAIL, dwflags: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhEnumObjectItemsHA(hdatasource : isize, szmachinename : windows_core::PCSTR, szobjectname : windows_core::PCSTR, mszcounterlist : windows_core::PSTR, pcchcounterlistlength : *mut u32, mszinstancelist : windows_core::PSTR, pcchinstancelistlength : *mut u32, dwdetaillevel : PERF_DETAIL, dwflags : u32) -> u32);
    PdhEnumObjectItemsHA(hdatasource, szmachinename.param().abi(), szobjectname.param().abi(), core::mem::transmute(mszcounterlist), pcchcounterlistlength, core::mem::transmute(mszinstancelist), pcchinstancelistlength, dwdetaillevel, dwflags)
}
#[inline]
pub unsafe fn PdhEnumObjectItemsHW<P0, P1>(hdatasource: isize, szmachinename: P0, szobjectname: P1, mszcounterlist: windows_core::PWSTR, pcchcounterlistlength: *mut u32, mszinstancelist: windows_core::PWSTR, pcchinstancelistlength: *mut u32, dwdetaillevel: PERF_DETAIL, dwflags: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhEnumObjectItemsHW(hdatasource : isize, szmachinename : windows_core::PCWSTR, szobjectname : windows_core::PCWSTR, mszcounterlist : windows_core::PWSTR, pcchcounterlistlength : *mut u32, mszinstancelist : windows_core::PWSTR, pcchinstancelistlength : *mut u32, dwdetaillevel : PERF_DETAIL, dwflags : u32) -> u32);
    PdhEnumObjectItemsHW(hdatasource, szmachinename.param().abi(), szobjectname.param().abi(), core::mem::transmute(mszcounterlist), pcchcounterlistlength, core::mem::transmute(mszinstancelist), pcchinstancelistlength, dwdetaillevel, dwflags)
}
#[inline]
pub unsafe fn PdhEnumObjectItemsW<P0, P1, P2>(szdatasource: P0, szmachinename: P1, szobjectname: P2, mszcounterlist: windows_core::PWSTR, pcchcounterlistlength: *mut u32, mszinstancelist: windows_core::PWSTR, pcchinstancelistlength: *mut u32, dwdetaillevel: PERF_DETAIL, dwflags: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhEnumObjectItemsW(szdatasource : windows_core::PCWSTR, szmachinename : windows_core::PCWSTR, szobjectname : windows_core::PCWSTR, mszcounterlist : windows_core::PWSTR, pcchcounterlistlength : *mut u32, mszinstancelist : windows_core::PWSTR, pcchinstancelistlength : *mut u32, dwdetaillevel : PERF_DETAIL, dwflags : u32) -> u32);
    PdhEnumObjectItemsW(szdatasource.param().abi(), szmachinename.param().abi(), szobjectname.param().abi(), core::mem::transmute(mszcounterlist), pcchcounterlistlength, core::mem::transmute(mszinstancelist), pcchinstancelistlength, dwdetaillevel, dwflags)
}
#[inline]
pub unsafe fn PdhEnumObjectsA<P0, P1, P2>(szdatasource: P0, szmachinename: P1, mszobjectlist: windows_core::PSTR, pcchbuffersize: *mut u32, dwdetaillevel: PERF_DETAIL, brefresh: P2) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhEnumObjectsA(szdatasource : windows_core::PCSTR, szmachinename : windows_core::PCSTR, mszobjectlist : windows_core::PSTR, pcchbuffersize : *mut u32, dwdetaillevel : PERF_DETAIL, brefresh : super::super::Foundation:: BOOL) -> u32);
    PdhEnumObjectsA(szdatasource.param().abi(), szmachinename.param().abi(), core::mem::transmute(mszobjectlist), pcchbuffersize, dwdetaillevel, brefresh.param().abi())
}
#[inline]
pub unsafe fn PdhEnumObjectsHA<P0, P1>(hdatasource: isize, szmachinename: P0, mszobjectlist: windows_core::PSTR, pcchbuffersize: *mut u32, dwdetaillevel: PERF_DETAIL, brefresh: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhEnumObjectsHA(hdatasource : isize, szmachinename : windows_core::PCSTR, mszobjectlist : windows_core::PSTR, pcchbuffersize : *mut u32, dwdetaillevel : PERF_DETAIL, brefresh : super::super::Foundation:: BOOL) -> u32);
    PdhEnumObjectsHA(hdatasource, szmachinename.param().abi(), core::mem::transmute(mszobjectlist), pcchbuffersize, dwdetaillevel, brefresh.param().abi())
}
#[inline]
pub unsafe fn PdhEnumObjectsHW<P0, P1>(hdatasource: isize, szmachinename: P0, mszobjectlist: windows_core::PWSTR, pcchbuffersize: *mut u32, dwdetaillevel: PERF_DETAIL, brefresh: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhEnumObjectsHW(hdatasource : isize, szmachinename : windows_core::PCWSTR, mszobjectlist : windows_core::PWSTR, pcchbuffersize : *mut u32, dwdetaillevel : PERF_DETAIL, brefresh : super::super::Foundation:: BOOL) -> u32);
    PdhEnumObjectsHW(hdatasource, szmachinename.param().abi(), core::mem::transmute(mszobjectlist), pcchbuffersize, dwdetaillevel, brefresh.param().abi())
}
#[inline]
pub unsafe fn PdhEnumObjectsW<P0, P1, P2>(szdatasource: P0, szmachinename: P1, mszobjectlist: windows_core::PWSTR, pcchbuffersize: *mut u32, dwdetaillevel: PERF_DETAIL, brefresh: P2) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhEnumObjectsW(szdatasource : windows_core::PCWSTR, szmachinename : windows_core::PCWSTR, mszobjectlist : windows_core::PWSTR, pcchbuffersize : *mut u32, dwdetaillevel : PERF_DETAIL, brefresh : super::super::Foundation:: BOOL) -> u32);
    PdhEnumObjectsW(szdatasource.param().abi(), szmachinename.param().abi(), core::mem::transmute(mszobjectlist), pcchbuffersize, dwdetaillevel, brefresh.param().abi())
}
#[inline]
pub unsafe fn PdhExpandCounterPathA<P0>(szwildcardpath: P0, mszexpandedpathlist: windows_core::PSTR, pcchpathlistlength: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhExpandCounterPathA(szwildcardpath : windows_core::PCSTR, mszexpandedpathlist : windows_core::PSTR, pcchpathlistlength : *mut u32) -> u32);
    PdhExpandCounterPathA(szwildcardpath.param().abi(), core::mem::transmute(mszexpandedpathlist), pcchpathlistlength)
}
#[inline]
pub unsafe fn PdhExpandCounterPathW<P0>(szwildcardpath: P0, mszexpandedpathlist: windows_core::PWSTR, pcchpathlistlength: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhExpandCounterPathW(szwildcardpath : windows_core::PCWSTR, mszexpandedpathlist : windows_core::PWSTR, pcchpathlistlength : *mut u32) -> u32);
    PdhExpandCounterPathW(szwildcardpath.param().abi(), core::mem::transmute(mszexpandedpathlist), pcchpathlistlength)
}
#[inline]
pub unsafe fn PdhExpandWildCardPathA<P0, P1>(szdatasource: P0, szwildcardpath: P1, mszexpandedpathlist: windows_core::PSTR, pcchpathlistlength: *mut u32, dwflags: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhExpandWildCardPathA(szdatasource : windows_core::PCSTR, szwildcardpath : windows_core::PCSTR, mszexpandedpathlist : windows_core::PSTR, pcchpathlistlength : *mut u32, dwflags : u32) -> u32);
    PdhExpandWildCardPathA(szdatasource.param().abi(), szwildcardpath.param().abi(), core::mem::transmute(mszexpandedpathlist), pcchpathlistlength, dwflags)
}
#[inline]
pub unsafe fn PdhExpandWildCardPathHA<P0>(hdatasource: isize, szwildcardpath: P0, mszexpandedpathlist: windows_core::PSTR, pcchpathlistlength: *mut u32, dwflags: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhExpandWildCardPathHA(hdatasource : isize, szwildcardpath : windows_core::PCSTR, mszexpandedpathlist : windows_core::PSTR, pcchpathlistlength : *mut u32, dwflags : u32) -> u32);
    PdhExpandWildCardPathHA(hdatasource, szwildcardpath.param().abi(), core::mem::transmute(mszexpandedpathlist), pcchpathlistlength, dwflags)
}
#[inline]
pub unsafe fn PdhExpandWildCardPathHW<P0>(hdatasource: isize, szwildcardpath: P0, mszexpandedpathlist: windows_core::PWSTR, pcchpathlistlength: *mut u32, dwflags: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhExpandWildCardPathHW(hdatasource : isize, szwildcardpath : windows_core::PCWSTR, mszexpandedpathlist : windows_core::PWSTR, pcchpathlistlength : *mut u32, dwflags : u32) -> u32);
    PdhExpandWildCardPathHW(hdatasource, szwildcardpath.param().abi(), core::mem::transmute(mszexpandedpathlist), pcchpathlistlength, dwflags)
}
#[inline]
pub unsafe fn PdhExpandWildCardPathW<P0, P1>(szdatasource: P0, szwildcardpath: P1, mszexpandedpathlist: windows_core::PWSTR, pcchpathlistlength: *mut u32, dwflags: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhExpandWildCardPathW(szdatasource : windows_core::PCWSTR, szwildcardpath : windows_core::PCWSTR, mszexpandedpathlist : windows_core::PWSTR, pcchpathlistlength : *mut u32, dwflags : u32) -> u32);
    PdhExpandWildCardPathW(szdatasource.param().abi(), szwildcardpath.param().abi(), core::mem::transmute(mszexpandedpathlist), pcchpathlistlength, dwflags)
}
#[inline]
pub unsafe fn PdhFormatFromRawValue(dwcountertype: u32, dwformat: PDH_FMT, ptimebase: Option<*const i64>, prawvalue1: *const PDH_RAW_COUNTER, prawvalue2: *const PDH_RAW_COUNTER, pfmtvalue: *mut PDH_FMT_COUNTERVALUE) -> u32 {
    windows_targets::link!("pdh.dll" "system" fn PdhFormatFromRawValue(dwcountertype : u32, dwformat : PDH_FMT, ptimebase : *const i64, prawvalue1 : *const PDH_RAW_COUNTER, prawvalue2 : *const PDH_RAW_COUNTER, pfmtvalue : *mut PDH_FMT_COUNTERVALUE) -> u32);
    PdhFormatFromRawValue(dwcountertype, dwformat, core::mem::transmute(ptimebase.unwrap_or(std::ptr::null())), prawvalue1, prawvalue2, pfmtvalue)
}
#[inline]
pub unsafe fn PdhGetCounterInfoA<P0>(hcounter: isize, bretrieveexplaintext: P0, pdwbuffersize: *mut u32, lpbuffer: Option<*mut PDH_COUNTER_INFO_A>) -> u32
where
    P0: windows_core::Param<super::super::Foundation::BOOLEAN>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhGetCounterInfoA(hcounter : isize, bretrieveexplaintext : super::super::Foundation:: BOOLEAN, pdwbuffersize : *mut u32, lpbuffer : *mut PDH_COUNTER_INFO_A) -> u32);
    PdhGetCounterInfoA(hcounter, bretrieveexplaintext.param().abi(), pdwbuffersize, core::mem::transmute(lpbuffer.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn PdhGetCounterInfoW<P0>(hcounter: isize, bretrieveexplaintext: P0, pdwbuffersize: *mut u32, lpbuffer: Option<*mut PDH_COUNTER_INFO_W>) -> u32
where
    P0: windows_core::Param<super::super::Foundation::BOOLEAN>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhGetCounterInfoW(hcounter : isize, bretrieveexplaintext : super::super::Foundation:: BOOLEAN, pdwbuffersize : *mut u32, lpbuffer : *mut PDH_COUNTER_INFO_W) -> u32);
    PdhGetCounterInfoW(hcounter, bretrieveexplaintext.param().abi(), pdwbuffersize, core::mem::transmute(lpbuffer.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn PdhGetCounterTimeBase(hcounter: isize, ptimebase: *mut i64) -> u32 {
    windows_targets::link!("pdh.dll" "system" fn PdhGetCounterTimeBase(hcounter : isize, ptimebase : *mut i64) -> u32);
    PdhGetCounterTimeBase(hcounter, ptimebase)
}
#[inline]
pub unsafe fn PdhGetDataSourceTimeRangeA<P0>(szdatasource: P0, pdwnumentries: *mut u32, pinfo: *mut PDH_TIME_INFO, pdwbuffersize: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhGetDataSourceTimeRangeA(szdatasource : windows_core::PCSTR, pdwnumentries : *mut u32, pinfo : *mut PDH_TIME_INFO, pdwbuffersize : *mut u32) -> u32);
    PdhGetDataSourceTimeRangeA(szdatasource.param().abi(), pdwnumentries, pinfo, pdwbuffersize)
}
#[inline]
pub unsafe fn PdhGetDataSourceTimeRangeH(hdatasource: isize, pdwnumentries: *mut u32, pinfo: *mut PDH_TIME_INFO, pdwbuffersize: *mut u32) -> u32 {
    windows_targets::link!("pdh.dll" "system" fn PdhGetDataSourceTimeRangeH(hdatasource : isize, pdwnumentries : *mut u32, pinfo : *mut PDH_TIME_INFO, pdwbuffersize : *mut u32) -> u32);
    PdhGetDataSourceTimeRangeH(hdatasource, pdwnumentries, pinfo, pdwbuffersize)
}
#[inline]
pub unsafe fn PdhGetDataSourceTimeRangeW<P0>(szdatasource: P0, pdwnumentries: *mut u32, pinfo: *mut PDH_TIME_INFO, pdwbuffersize: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhGetDataSourceTimeRangeW(szdatasource : windows_core::PCWSTR, pdwnumentries : *mut u32, pinfo : *mut PDH_TIME_INFO, pdwbuffersize : *mut u32) -> u32);
    PdhGetDataSourceTimeRangeW(szdatasource.param().abi(), pdwnumentries, pinfo, pdwbuffersize)
}
#[inline]
pub unsafe fn PdhGetDefaultPerfCounterA<P0, P1, P2>(szdatasource: P0, szmachinename: P1, szobjectname: P2, szdefaultcountername: windows_core::PSTR, pcchbuffersize: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhGetDefaultPerfCounterA(szdatasource : windows_core::PCSTR, szmachinename : windows_core::PCSTR, szobjectname : windows_core::PCSTR, szdefaultcountername : windows_core::PSTR, pcchbuffersize : *mut u32) -> u32);
    PdhGetDefaultPerfCounterA(szdatasource.param().abi(), szmachinename.param().abi(), szobjectname.param().abi(), core::mem::transmute(szdefaultcountername), pcchbuffersize)
}
#[inline]
pub unsafe fn PdhGetDefaultPerfCounterHA<P0, P1>(hdatasource: isize, szmachinename: P0, szobjectname: P1, szdefaultcountername: windows_core::PSTR, pcchbuffersize: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhGetDefaultPerfCounterHA(hdatasource : isize, szmachinename : windows_core::PCSTR, szobjectname : windows_core::PCSTR, szdefaultcountername : windows_core::PSTR, pcchbuffersize : *mut u32) -> u32);
    PdhGetDefaultPerfCounterHA(hdatasource, szmachinename.param().abi(), szobjectname.param().abi(), core::mem::transmute(szdefaultcountername), pcchbuffersize)
}
#[inline]
pub unsafe fn PdhGetDefaultPerfCounterHW<P0, P1>(hdatasource: isize, szmachinename: P0, szobjectname: P1, szdefaultcountername: windows_core::PWSTR, pcchbuffersize: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhGetDefaultPerfCounterHW(hdatasource : isize, szmachinename : windows_core::PCWSTR, szobjectname : windows_core::PCWSTR, szdefaultcountername : windows_core::PWSTR, pcchbuffersize : *mut u32) -> u32);
    PdhGetDefaultPerfCounterHW(hdatasource, szmachinename.param().abi(), szobjectname.param().abi(), core::mem::transmute(szdefaultcountername), pcchbuffersize)
}
#[inline]
pub unsafe fn PdhGetDefaultPerfCounterW<P0, P1, P2>(szdatasource: P0, szmachinename: P1, szobjectname: P2, szdefaultcountername: windows_core::PWSTR, pcchbuffersize: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhGetDefaultPerfCounterW(szdatasource : windows_core::PCWSTR, szmachinename : windows_core::PCWSTR, szobjectname : windows_core::PCWSTR, szdefaultcountername : windows_core::PWSTR, pcchbuffersize : *mut u32) -> u32);
    PdhGetDefaultPerfCounterW(szdatasource.param().abi(), szmachinename.param().abi(), szobjectname.param().abi(), core::mem::transmute(szdefaultcountername), pcchbuffersize)
}
#[inline]
pub unsafe fn PdhGetDefaultPerfObjectA<P0, P1>(szdatasource: P0, szmachinename: P1, szdefaultobjectname: windows_core::PSTR, pcchbuffersize: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhGetDefaultPerfObjectA(szdatasource : windows_core::PCSTR, szmachinename : windows_core::PCSTR, szdefaultobjectname : windows_core::PSTR, pcchbuffersize : *mut u32) -> u32);
    PdhGetDefaultPerfObjectA(szdatasource.param().abi(), szmachinename.param().abi(), core::mem::transmute(szdefaultobjectname), pcchbuffersize)
}
#[inline]
pub unsafe fn PdhGetDefaultPerfObjectHA<P0>(hdatasource: isize, szmachinename: P0, szdefaultobjectname: windows_core::PSTR, pcchbuffersize: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhGetDefaultPerfObjectHA(hdatasource : isize, szmachinename : windows_core::PCSTR, szdefaultobjectname : windows_core::PSTR, pcchbuffersize : *mut u32) -> u32);
    PdhGetDefaultPerfObjectHA(hdatasource, szmachinename.param().abi(), core::mem::transmute(szdefaultobjectname), pcchbuffersize)
}
#[inline]
pub unsafe fn PdhGetDefaultPerfObjectHW<P0>(hdatasource: isize, szmachinename: P0, szdefaultobjectname: windows_core::PWSTR, pcchbuffersize: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhGetDefaultPerfObjectHW(hdatasource : isize, szmachinename : windows_core::PCWSTR, szdefaultobjectname : windows_core::PWSTR, pcchbuffersize : *mut u32) -> u32);
    PdhGetDefaultPerfObjectHW(hdatasource, szmachinename.param().abi(), core::mem::transmute(szdefaultobjectname), pcchbuffersize)
}
#[inline]
pub unsafe fn PdhGetDefaultPerfObjectW<P0, P1>(szdatasource: P0, szmachinename: P1, szdefaultobjectname: windows_core::PWSTR, pcchbuffersize: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhGetDefaultPerfObjectW(szdatasource : windows_core::PCWSTR, szmachinename : windows_core::PCWSTR, szdefaultobjectname : windows_core::PWSTR, pcchbuffersize : *mut u32) -> u32);
    PdhGetDefaultPerfObjectW(szdatasource.param().abi(), szmachinename.param().abi(), core::mem::transmute(szdefaultobjectname), pcchbuffersize)
}
#[inline]
pub unsafe fn PdhGetDllVersion(lpdwversion: Option<*mut PDH_DLL_VERSION>) -> u32 {
    windows_targets::link!("pdh.dll" "system" fn PdhGetDllVersion(lpdwversion : *mut PDH_DLL_VERSION) -> u32);
    PdhGetDllVersion(core::mem::transmute(lpdwversion.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn PdhGetFormattedCounterArrayA(hcounter: isize, dwformat: PDH_FMT, lpdwbuffersize: *mut u32, lpdwitemcount: *mut u32, itembuffer: Option<*mut PDH_FMT_COUNTERVALUE_ITEM_A>) -> u32 {
    windows_targets::link!("pdh.dll" "system" fn PdhGetFormattedCounterArrayA(hcounter : isize, dwformat : PDH_FMT, lpdwbuffersize : *mut u32, lpdwitemcount : *mut u32, itembuffer : *mut PDH_FMT_COUNTERVALUE_ITEM_A) -> u32);
    PdhGetFormattedCounterArrayA(hcounter, dwformat, lpdwbuffersize, lpdwitemcount, core::mem::transmute(itembuffer.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn PdhGetFormattedCounterArrayW(hcounter: isize, dwformat: PDH_FMT, lpdwbuffersize: *mut u32, lpdwitemcount: *mut u32, itembuffer: Option<*mut PDH_FMT_COUNTERVALUE_ITEM_W>) -> u32 {
    windows_targets::link!("pdh.dll" "system" fn PdhGetFormattedCounterArrayW(hcounter : isize, dwformat : PDH_FMT, lpdwbuffersize : *mut u32, lpdwitemcount : *mut u32, itembuffer : *mut PDH_FMT_COUNTERVALUE_ITEM_W) -> u32);
    PdhGetFormattedCounterArrayW(hcounter, dwformat, lpdwbuffersize, lpdwitemcount, core::mem::transmute(itembuffer.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn PdhGetFormattedCounterValue(hcounter: isize, dwformat: PDH_FMT, lpdwtype: Option<*mut u32>, pvalue: *mut PDH_FMT_COUNTERVALUE) -> u32 {
    windows_targets::link!("pdh.dll" "system" fn PdhGetFormattedCounterValue(hcounter : isize, dwformat : PDH_FMT, lpdwtype : *mut u32, pvalue : *mut PDH_FMT_COUNTERVALUE) -> u32);
    PdhGetFormattedCounterValue(hcounter, dwformat, core::mem::transmute(lpdwtype.unwrap_or(std::ptr::null_mut())), pvalue)
}
#[inline]
pub unsafe fn PdhGetLogFileSize(hlog: isize, llsize: *mut i64) -> u32 {
    windows_targets::link!("pdh.dll" "system" fn PdhGetLogFileSize(hlog : isize, llsize : *mut i64) -> u32);
    PdhGetLogFileSize(hlog, llsize)
}
#[inline]
pub unsafe fn PdhGetLogSetGUID(hlog: isize, pguid: Option<*mut windows_core::GUID>, prunid: Option<*mut i32>) -> u32 {
    windows_targets::link!("pdh.dll" "system" fn PdhGetLogSetGUID(hlog : isize, pguid : *mut windows_core::GUID, prunid : *mut i32) -> u32);
    PdhGetLogSetGUID(hlog, core::mem::transmute(pguid.unwrap_or(std::ptr::null_mut())), core::mem::transmute(prunid.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn PdhGetRawCounterArrayA(hcounter: isize, lpdwbuffersize: *mut u32, lpdwitemcount: *mut u32, itembuffer: Option<*mut PDH_RAW_COUNTER_ITEM_A>) -> u32 {
    windows_targets::link!("pdh.dll" "system" fn PdhGetRawCounterArrayA(hcounter : isize, lpdwbuffersize : *mut u32, lpdwitemcount : *mut u32, itembuffer : *mut PDH_RAW_COUNTER_ITEM_A) -> u32);
    PdhGetRawCounterArrayA(hcounter, lpdwbuffersize, lpdwitemcount, core::mem::transmute(itembuffer.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn PdhGetRawCounterArrayW(hcounter: isize, lpdwbuffersize: *mut u32, lpdwitemcount: *mut u32, itembuffer: Option<*mut PDH_RAW_COUNTER_ITEM_W>) -> u32 {
    windows_targets::link!("pdh.dll" "system" fn PdhGetRawCounterArrayW(hcounter : isize, lpdwbuffersize : *mut u32, lpdwitemcount : *mut u32, itembuffer : *mut PDH_RAW_COUNTER_ITEM_W) -> u32);
    PdhGetRawCounterArrayW(hcounter, lpdwbuffersize, lpdwitemcount, core::mem::transmute(itembuffer.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn PdhGetRawCounterValue(hcounter: isize, lpdwtype: Option<*mut u32>, pvalue: *mut PDH_RAW_COUNTER) -> u32 {
    windows_targets::link!("pdh.dll" "system" fn PdhGetRawCounterValue(hcounter : isize, lpdwtype : *mut u32, pvalue : *mut PDH_RAW_COUNTER) -> u32);
    PdhGetRawCounterValue(hcounter, core::mem::transmute(lpdwtype.unwrap_or(std::ptr::null_mut())), pvalue)
}
#[inline]
pub unsafe fn PdhIsRealTimeQuery(hquery: isize) -> super::super::Foundation::BOOL {
    windows_targets::link!("pdh.dll" "system" fn PdhIsRealTimeQuery(hquery : isize) -> super::super::Foundation:: BOOL);
    PdhIsRealTimeQuery(hquery)
}
#[inline]
pub unsafe fn PdhLookupPerfIndexByNameA<P0, P1>(szmachinename: P0, sznamebuffer: P1, pdwindex: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhLookupPerfIndexByNameA(szmachinename : windows_core::PCSTR, sznamebuffer : windows_core::PCSTR, pdwindex : *mut u32) -> u32);
    PdhLookupPerfIndexByNameA(szmachinename.param().abi(), sznamebuffer.param().abi(), pdwindex)
}
#[inline]
pub unsafe fn PdhLookupPerfIndexByNameW<P0, P1>(szmachinename: P0, sznamebuffer: P1, pdwindex: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhLookupPerfIndexByNameW(szmachinename : windows_core::PCWSTR, sznamebuffer : windows_core::PCWSTR, pdwindex : *mut u32) -> u32);
    PdhLookupPerfIndexByNameW(szmachinename.param().abi(), sznamebuffer.param().abi(), pdwindex)
}
#[inline]
pub unsafe fn PdhLookupPerfNameByIndexA<P0>(szmachinename: P0, dwnameindex: u32, sznamebuffer: windows_core::PSTR, pcchnamebuffersize: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhLookupPerfNameByIndexA(szmachinename : windows_core::PCSTR, dwnameindex : u32, sznamebuffer : windows_core::PSTR, pcchnamebuffersize : *mut u32) -> u32);
    PdhLookupPerfNameByIndexA(szmachinename.param().abi(), dwnameindex, core::mem::transmute(sznamebuffer), pcchnamebuffersize)
}
#[inline]
pub unsafe fn PdhLookupPerfNameByIndexW<P0>(szmachinename: P0, dwnameindex: u32, sznamebuffer: windows_core::PWSTR, pcchnamebuffersize: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhLookupPerfNameByIndexW(szmachinename : windows_core::PCWSTR, dwnameindex : u32, sznamebuffer : windows_core::PWSTR, pcchnamebuffersize : *mut u32) -> u32);
    PdhLookupPerfNameByIndexW(szmachinename.param().abi(), dwnameindex, core::mem::transmute(sznamebuffer), pcchnamebuffersize)
}
#[inline]
pub unsafe fn PdhMakeCounterPathA(pcounterpathelements: *const PDH_COUNTER_PATH_ELEMENTS_A, szfullpathbuffer: windows_core::PSTR, pcchbuffersize: *mut u32, dwflags: PDH_PATH_FLAGS) -> u32 {
    windows_targets::link!("pdh.dll" "system" fn PdhMakeCounterPathA(pcounterpathelements : *const PDH_COUNTER_PATH_ELEMENTS_A, szfullpathbuffer : windows_core::PSTR, pcchbuffersize : *mut u32, dwflags : PDH_PATH_FLAGS) -> u32);
    PdhMakeCounterPathA(pcounterpathelements, core::mem::transmute(szfullpathbuffer), pcchbuffersize, dwflags)
}
#[inline]
pub unsafe fn PdhMakeCounterPathW(pcounterpathelements: *const PDH_COUNTER_PATH_ELEMENTS_W, szfullpathbuffer: windows_core::PWSTR, pcchbuffersize: *mut u32, dwflags: PDH_PATH_FLAGS) -> u32 {
    windows_targets::link!("pdh.dll" "system" fn PdhMakeCounterPathW(pcounterpathelements : *const PDH_COUNTER_PATH_ELEMENTS_W, szfullpathbuffer : windows_core::PWSTR, pcchbuffersize : *mut u32, dwflags : PDH_PATH_FLAGS) -> u32);
    PdhMakeCounterPathW(pcounterpathelements, core::mem::transmute(szfullpathbuffer), pcchbuffersize, dwflags)
}
#[inline]
pub unsafe fn PdhOpenLogA<P0, P1>(szlogfilename: P0, dwaccessflags: PDH_LOG, lpdwlogtype: *mut PDH_LOG_TYPE, hquery: isize, dwmaxsize: u32, szusercaption: P1, phlog: *mut isize) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhOpenLogA(szlogfilename : windows_core::PCSTR, dwaccessflags : PDH_LOG, lpdwlogtype : *mut PDH_LOG_TYPE, hquery : isize, dwmaxsize : u32, szusercaption : windows_core::PCSTR, phlog : *mut isize) -> u32);
    PdhOpenLogA(szlogfilename.param().abi(), dwaccessflags, lpdwlogtype, hquery, dwmaxsize, szusercaption.param().abi(), phlog)
}
#[inline]
pub unsafe fn PdhOpenLogW<P0, P1>(szlogfilename: P0, dwaccessflags: PDH_LOG, lpdwlogtype: *mut PDH_LOG_TYPE, hquery: isize, dwmaxsize: u32, szusercaption: P1, phlog: *mut isize) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhOpenLogW(szlogfilename : windows_core::PCWSTR, dwaccessflags : PDH_LOG, lpdwlogtype : *mut PDH_LOG_TYPE, hquery : isize, dwmaxsize : u32, szusercaption : windows_core::PCWSTR, phlog : *mut isize) -> u32);
    PdhOpenLogW(szlogfilename.param().abi(), dwaccessflags, lpdwlogtype, hquery, dwmaxsize, szusercaption.param().abi(), phlog)
}
#[inline]
pub unsafe fn PdhOpenQueryA<P0>(szdatasource: P0, dwuserdata: usize, phquery: *mut isize) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhOpenQueryA(szdatasource : windows_core::PCSTR, dwuserdata : usize, phquery : *mut isize) -> u32);
    PdhOpenQueryA(szdatasource.param().abi(), dwuserdata, phquery)
}
#[inline]
pub unsafe fn PdhOpenQueryH(hdatasource: isize, dwuserdata: usize, phquery: *mut isize) -> u32 {
    windows_targets::link!("pdh.dll" "system" fn PdhOpenQueryH(hdatasource : isize, dwuserdata : usize, phquery : *mut isize) -> u32);
    PdhOpenQueryH(hdatasource, dwuserdata, phquery)
}
#[inline]
pub unsafe fn PdhOpenQueryW<P0>(szdatasource: P0, dwuserdata: usize, phquery: *mut isize) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhOpenQueryW(szdatasource : windows_core::PCWSTR, dwuserdata : usize, phquery : *mut isize) -> u32);
    PdhOpenQueryW(szdatasource.param().abi(), dwuserdata, phquery)
}
#[inline]
pub unsafe fn PdhParseCounterPathA<P0>(szfullpathbuffer: P0, pcounterpathelements: Option<*mut PDH_COUNTER_PATH_ELEMENTS_A>, pdwbuffersize: *mut u32, dwflags: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhParseCounterPathA(szfullpathbuffer : windows_core::PCSTR, pcounterpathelements : *mut PDH_COUNTER_PATH_ELEMENTS_A, pdwbuffersize : *mut u32, dwflags : u32) -> u32);
    PdhParseCounterPathA(szfullpathbuffer.param().abi(), core::mem::transmute(pcounterpathelements.unwrap_or(std::ptr::null_mut())), pdwbuffersize, dwflags)
}
#[inline]
pub unsafe fn PdhParseCounterPathW<P0>(szfullpathbuffer: P0, pcounterpathelements: Option<*mut PDH_COUNTER_PATH_ELEMENTS_W>, pdwbuffersize: *mut u32, dwflags: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhParseCounterPathW(szfullpathbuffer : windows_core::PCWSTR, pcounterpathelements : *mut PDH_COUNTER_PATH_ELEMENTS_W, pdwbuffersize : *mut u32, dwflags : u32) -> u32);
    PdhParseCounterPathW(szfullpathbuffer.param().abi(), core::mem::transmute(pcounterpathelements.unwrap_or(std::ptr::null_mut())), pdwbuffersize, dwflags)
}
#[inline]
pub unsafe fn PdhParseInstanceNameA<P0>(szinstancestring: P0, szinstancename: windows_core::PSTR, pcchinstancenamelength: *mut u32, szparentname: windows_core::PSTR, pcchparentnamelength: *mut u32, lpindex: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhParseInstanceNameA(szinstancestring : windows_core::PCSTR, szinstancename : windows_core::PSTR, pcchinstancenamelength : *mut u32, szparentname : windows_core::PSTR, pcchparentnamelength : *mut u32, lpindex : *mut u32) -> u32);
    PdhParseInstanceNameA(szinstancestring.param().abi(), core::mem::transmute(szinstancename), pcchinstancenamelength, core::mem::transmute(szparentname), pcchparentnamelength, lpindex)
}
#[inline]
pub unsafe fn PdhParseInstanceNameW<P0>(szinstancestring: P0, szinstancename: windows_core::PWSTR, pcchinstancenamelength: *mut u32, szparentname: windows_core::PWSTR, pcchparentnamelength: *mut u32, lpindex: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhParseInstanceNameW(szinstancestring : windows_core::PCWSTR, szinstancename : windows_core::PWSTR, pcchinstancenamelength : *mut u32, szparentname : windows_core::PWSTR, pcchparentnamelength : *mut u32, lpindex : *mut u32) -> u32);
    PdhParseInstanceNameW(szinstancestring.param().abi(), core::mem::transmute(szinstancename), pcchinstancenamelength, core::mem::transmute(szparentname), pcchparentnamelength, lpindex)
}
#[inline]
pub unsafe fn PdhReadRawLogRecord(hlog: isize, ftrecord: super::super::Foundation::FILETIME, prawlogrecord: Option<*mut PDH_RAW_LOG_RECORD>, pdwbufferlength: *mut u32) -> u32 {
    windows_targets::link!("pdh.dll" "system" fn PdhReadRawLogRecord(hlog : isize, ftrecord : super::super::Foundation:: FILETIME, prawlogrecord : *mut PDH_RAW_LOG_RECORD, pdwbufferlength : *mut u32) -> u32);
    PdhReadRawLogRecord(hlog, core::mem::transmute(ftrecord), core::mem::transmute(prawlogrecord.unwrap_or(std::ptr::null_mut())), pdwbufferlength)
}
#[inline]
pub unsafe fn PdhRemoveCounter(hcounter: isize) -> u32 {
    windows_targets::link!("pdh.dll" "system" fn PdhRemoveCounter(hcounter : isize) -> u32);
    PdhRemoveCounter(hcounter)
}
#[inline]
pub unsafe fn PdhSelectDataSourceA<P0>(hwndowner: P0, dwflags: PDH_SELECT_DATA_SOURCE_FLAGS, szdatasource: windows_core::PSTR, pcchbufferlength: *mut u32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhSelectDataSourceA(hwndowner : super::super::Foundation:: HWND, dwflags : PDH_SELECT_DATA_SOURCE_FLAGS, szdatasource : windows_core::PSTR, pcchbufferlength : *mut u32) -> u32);
    PdhSelectDataSourceA(hwndowner.param().abi(), dwflags, core::mem::transmute(szdatasource), pcchbufferlength)
}
#[inline]
pub unsafe fn PdhSelectDataSourceW<P0>(hwndowner: P0, dwflags: PDH_SELECT_DATA_SOURCE_FLAGS, szdatasource: windows_core::PWSTR, pcchbufferlength: *mut u32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhSelectDataSourceW(hwndowner : super::super::Foundation:: HWND, dwflags : PDH_SELECT_DATA_SOURCE_FLAGS, szdatasource : windows_core::PWSTR, pcchbufferlength : *mut u32) -> u32);
    PdhSelectDataSourceW(hwndowner.param().abi(), dwflags, core::mem::transmute(szdatasource), pcchbufferlength)
}
#[inline]
pub unsafe fn PdhSetCounterScaleFactor(hcounter: isize, lfactor: i32) -> u32 {
    windows_targets::link!("pdh.dll" "system" fn PdhSetCounterScaleFactor(hcounter : isize, lfactor : i32) -> u32);
    PdhSetCounterScaleFactor(hcounter, lfactor)
}
#[inline]
pub unsafe fn PdhSetDefaultRealTimeDataSource(dwdatasourceid: REAL_TIME_DATA_SOURCE_ID_FLAGS) -> u32 {
    windows_targets::link!("pdh.dll" "system" fn PdhSetDefaultRealTimeDataSource(dwdatasourceid : REAL_TIME_DATA_SOURCE_ID_FLAGS) -> u32);
    PdhSetDefaultRealTimeDataSource(dwdatasourceid)
}
#[inline]
pub unsafe fn PdhSetLogSetRunID(hlog: isize, runid: i32) -> u32 {
    windows_targets::link!("pdh.dll" "system" fn PdhSetLogSetRunID(hlog : isize, runid : i32) -> u32);
    PdhSetLogSetRunID(hlog, runid)
}
#[inline]
pub unsafe fn PdhSetQueryTimeRange(hquery: isize, pinfo: *const PDH_TIME_INFO) -> u32 {
    windows_targets::link!("pdh.dll" "system" fn PdhSetQueryTimeRange(hquery : isize, pinfo : *const PDH_TIME_INFO) -> u32);
    PdhSetQueryTimeRange(hquery, pinfo)
}
#[inline]
pub unsafe fn PdhUpdateLogA<P0>(hlog: isize, szuserstring: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhUpdateLogA(hlog : isize, szuserstring : windows_core::PCSTR) -> u32);
    PdhUpdateLogA(hlog, szuserstring.param().abi())
}
#[inline]
pub unsafe fn PdhUpdateLogFileCatalog(hlog: isize) -> u32 {
    windows_targets::link!("pdh.dll" "system" fn PdhUpdateLogFileCatalog(hlog : isize) -> u32);
    PdhUpdateLogFileCatalog(hlog)
}
#[inline]
pub unsafe fn PdhUpdateLogW<P0>(hlog: isize, szuserstring: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhUpdateLogW(hlog : isize, szuserstring : windows_core::PCWSTR) -> u32);
    PdhUpdateLogW(hlog, szuserstring.param().abi())
}
#[inline]
pub unsafe fn PdhValidatePathA<P0>(szfullpathbuffer: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhValidatePathA(szfullpathbuffer : windows_core::PCSTR) -> u32);
    PdhValidatePathA(szfullpathbuffer.param().abi())
}
#[inline]
pub unsafe fn PdhValidatePathExA<P0>(hdatasource: isize, szfullpathbuffer: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhValidatePathExA(hdatasource : isize, szfullpathbuffer : windows_core::PCSTR) -> u32);
    PdhValidatePathExA(hdatasource, szfullpathbuffer.param().abi())
}
#[inline]
pub unsafe fn PdhValidatePathExW<P0>(hdatasource: isize, szfullpathbuffer: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhValidatePathExW(hdatasource : isize, szfullpathbuffer : windows_core::PCWSTR) -> u32);
    PdhValidatePathExW(hdatasource, szfullpathbuffer.param().abi())
}
#[inline]
pub unsafe fn PdhValidatePathW<P0>(szfullpathbuffer: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhValidatePathW(szfullpathbuffer : windows_core::PCWSTR) -> u32);
    PdhValidatePathW(szfullpathbuffer.param().abi())
}
#[inline]
pub unsafe fn PdhVerifySQLDBA<P0>(szdatasource: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhVerifySQLDBA(szdatasource : windows_core::PCSTR) -> u32);
    PdhVerifySQLDBA(szdatasource.param().abi())
}
#[inline]
pub unsafe fn PdhVerifySQLDBW<P0>(szdatasource: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("pdh.dll" "system" fn PdhVerifySQLDBW(szdatasource : windows_core::PCWSTR) -> u32);
    PdhVerifySQLDBW(szdatasource.param().abi())
}
#[inline]
pub unsafe fn PerfAddCounters<P0>(hquery: P0, pcounters: *mut PERF_COUNTER_IDENTIFIER, cbcounters: u32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn PerfAddCounters(hquery : super::super::Foundation:: HANDLE, pcounters : *mut PERF_COUNTER_IDENTIFIER, cbcounters : u32) -> u32);
    PerfAddCounters(hquery.param().abi(), pcounters, cbcounters)
}
#[inline]
pub unsafe fn PerfCloseQueryHandle<P0>(hquery: P0) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn PerfCloseQueryHandle(hquery : super::super::Foundation:: HANDLE) -> u32);
    PerfCloseQueryHandle(hquery.param().abi())
}
#[inline]
pub unsafe fn PerfCreateInstance<P0, P1>(providerhandle: P0, countersetguid: *const windows_core::GUID, name: P1, id: u32) -> *mut PERF_COUNTERSET_INSTANCE
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn PerfCreateInstance(providerhandle : super::super::Foundation:: HANDLE, countersetguid : *const windows_core::GUID, name : windows_core::PCWSTR, id : u32) -> *mut PERF_COUNTERSET_INSTANCE);
    PerfCreateInstance(providerhandle.param().abi(), countersetguid, name.param().abi(), id)
}
#[inline]
pub unsafe fn PerfDecrementULongCounterValue<P0>(provider: P0, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn PerfDecrementULongCounterValue(provider : super::super::Foundation:: HANDLE, instance : *mut PERF_COUNTERSET_INSTANCE, counterid : u32, value : u32) -> u32);
    PerfDecrementULongCounterValue(provider.param().abi(), instance, counterid, value)
}
#[inline]
pub unsafe fn PerfDecrementULongLongCounterValue<P0>(provider: P0, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u64) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn PerfDecrementULongLongCounterValue(provider : super::super::Foundation:: HANDLE, instance : *mut PERF_COUNTERSET_INSTANCE, counterid : u32, value : u64) -> u32);
    PerfDecrementULongLongCounterValue(provider.param().abi(), instance, counterid, value)
}
#[inline]
pub unsafe fn PerfDeleteCounters<P0>(hquery: P0, pcounters: *mut PERF_COUNTER_IDENTIFIER, cbcounters: u32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn PerfDeleteCounters(hquery : super::super::Foundation:: HANDLE, pcounters : *mut PERF_COUNTER_IDENTIFIER, cbcounters : u32) -> u32);
    PerfDeleteCounters(hquery.param().abi(), pcounters, cbcounters)
}
#[inline]
pub unsafe fn PerfDeleteInstance<P0>(provider: P0, instanceblock: *const PERF_COUNTERSET_INSTANCE) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn PerfDeleteInstance(provider : super::super::Foundation:: HANDLE, instanceblock : *const PERF_COUNTERSET_INSTANCE) -> u32);
    PerfDeleteInstance(provider.param().abi(), instanceblock)
}
#[inline]
pub unsafe fn PerfEnumerateCounterSet<P0>(szmachine: P0, pcountersetids: Option<&mut [windows_core::GUID]>, pccountersetidsactual: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn PerfEnumerateCounterSet(szmachine : windows_core::PCWSTR, pcountersetids : *mut windows_core::GUID, ccountersetids : u32, pccountersetidsactual : *mut u32) -> u32);
    PerfEnumerateCounterSet(szmachine.param().abi(), core::mem::transmute(pcountersetids.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pcountersetids.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pccountersetidsactual)
}
#[inline]
pub unsafe fn PerfEnumerateCounterSetInstances<P0>(szmachine: P0, pcountersetid: *const windows_core::GUID, pinstances: Option<*mut PERF_INSTANCE_HEADER>, cbinstances: u32, pcbinstancesactual: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn PerfEnumerateCounterSetInstances(szmachine : windows_core::PCWSTR, pcountersetid : *const windows_core::GUID, pinstances : *mut PERF_INSTANCE_HEADER, cbinstances : u32, pcbinstancesactual : *mut u32) -> u32);
    PerfEnumerateCounterSetInstances(szmachine.param().abi(), pcountersetid, core::mem::transmute(pinstances.unwrap_or(std::ptr::null_mut())), cbinstances, pcbinstancesactual)
}
#[inline]
pub unsafe fn PerfIncrementULongCounterValue<P0>(provider: P0, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn PerfIncrementULongCounterValue(provider : super::super::Foundation:: HANDLE, instance : *mut PERF_COUNTERSET_INSTANCE, counterid : u32, value : u32) -> u32);
    PerfIncrementULongCounterValue(provider.param().abi(), instance, counterid, value)
}
#[inline]
pub unsafe fn PerfIncrementULongLongCounterValue<P0>(provider: P0, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u64) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn PerfIncrementULongLongCounterValue(provider : super::super::Foundation:: HANDLE, instance : *mut PERF_COUNTERSET_INSTANCE, counterid : u32, value : u64) -> u32);
    PerfIncrementULongLongCounterValue(provider.param().abi(), instance, counterid, value)
}
#[inline]
pub unsafe fn PerfOpenQueryHandle<P0>(szmachine: P0, phquery: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn PerfOpenQueryHandle(szmachine : windows_core::PCWSTR, phquery : *mut super::super::Foundation:: HANDLE) -> u32);
    PerfOpenQueryHandle(szmachine.param().abi(), phquery)
}
#[inline]
pub unsafe fn PerfQueryCounterData<P0>(hquery: P0, pcounterblock: Option<*mut PERF_DATA_HEADER>, cbcounterblock: u32, pcbcounterblockactual: *mut u32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn PerfQueryCounterData(hquery : super::super::Foundation:: HANDLE, pcounterblock : *mut PERF_DATA_HEADER, cbcounterblock : u32, pcbcounterblockactual : *mut u32) -> u32);
    PerfQueryCounterData(hquery.param().abi(), core::mem::transmute(pcounterblock.unwrap_or(std::ptr::null_mut())), cbcounterblock, pcbcounterblockactual)
}
#[inline]
pub unsafe fn PerfQueryCounterInfo<P0>(hquery: P0, pcounters: Option<*mut PERF_COUNTER_IDENTIFIER>, cbcounters: u32, pcbcountersactual: *mut u32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn PerfQueryCounterInfo(hquery : super::super::Foundation:: HANDLE, pcounters : *mut PERF_COUNTER_IDENTIFIER, cbcounters : u32, pcbcountersactual : *mut u32) -> u32);
    PerfQueryCounterInfo(hquery.param().abi(), core::mem::transmute(pcounters.unwrap_or(std::ptr::null_mut())), cbcounters, pcbcountersactual)
}
#[inline]
pub unsafe fn PerfQueryCounterSetRegistrationInfo<P0>(szmachine: P0, pcountersetid: *const windows_core::GUID, requestcode: PerfRegInfoType, requestlangid: u32, pbreginfo: Option<&mut [u8]>, pcbreginfoactual: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn PerfQueryCounterSetRegistrationInfo(szmachine : windows_core::PCWSTR, pcountersetid : *const windows_core::GUID, requestcode : PerfRegInfoType, requestlangid : u32, pbreginfo : *mut u8, cbreginfo : u32, pcbreginfoactual : *mut u32) -> u32);
    PerfQueryCounterSetRegistrationInfo(szmachine.param().abi(), pcountersetid, requestcode, requestlangid, core::mem::transmute(pbreginfo.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pbreginfo.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbreginfoactual)
}
#[inline]
pub unsafe fn PerfQueryInstance<P0, P1>(providerhandle: P0, countersetguid: *const windows_core::GUID, name: P1, id: u32) -> *mut PERF_COUNTERSET_INSTANCE
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn PerfQueryInstance(providerhandle : super::super::Foundation:: HANDLE, countersetguid : *const windows_core::GUID, name : windows_core::PCWSTR, id : u32) -> *mut PERF_COUNTERSET_INSTANCE);
    PerfQueryInstance(providerhandle.param().abi(), countersetguid, name.param().abi(), id)
}
#[inline]
pub unsafe fn PerfSetCounterRefValue<P0>(provider: P0, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, address: *const core::ffi::c_void) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn PerfSetCounterRefValue(provider : super::super::Foundation:: HANDLE, instance : *mut PERF_COUNTERSET_INSTANCE, counterid : u32, address : *const core::ffi::c_void) -> u32);
    PerfSetCounterRefValue(provider.param().abi(), instance, counterid, address)
}
#[inline]
pub unsafe fn PerfSetCounterSetInfo<P0>(providerhandle: P0, template: *mut PERF_COUNTERSET_INFO, templatesize: u32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn PerfSetCounterSetInfo(providerhandle : super::super::Foundation:: HANDLE, template : *mut PERF_COUNTERSET_INFO, templatesize : u32) -> u32);
    PerfSetCounterSetInfo(providerhandle.param().abi(), template, templatesize)
}
#[inline]
pub unsafe fn PerfSetULongCounterValue<P0>(provider: P0, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn PerfSetULongCounterValue(provider : super::super::Foundation:: HANDLE, instance : *mut PERF_COUNTERSET_INSTANCE, counterid : u32, value : u32) -> u32);
    PerfSetULongCounterValue(provider.param().abi(), instance, counterid, value)
}
#[inline]
pub unsafe fn PerfSetULongLongCounterValue<P0>(provider: P0, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u64) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn PerfSetULongLongCounterValue(provider : super::super::Foundation:: HANDLE, instance : *mut PERF_COUNTERSET_INSTANCE, counterid : u32, value : u64) -> u32);
    PerfSetULongLongCounterValue(provider.param().abi(), instance, counterid, value)
}
#[inline]
pub unsafe fn PerfStartProvider(providerguid: *const windows_core::GUID, controlcallback: PERFLIBREQUEST, phprovider: *mut super::super::Foundation::HANDLE) -> u32 {
    windows_targets::link!("advapi32.dll" "system" fn PerfStartProvider(providerguid : *const windows_core::GUID, controlcallback : PERFLIBREQUEST, phprovider : *mut super::super::Foundation:: HANDLE) -> u32);
    PerfStartProvider(providerguid, controlcallback, phprovider)
}
#[inline]
pub unsafe fn PerfStartProviderEx(providerguid: *const windows_core::GUID, providercontext: Option<*const PERF_PROVIDER_CONTEXT>, provider: *mut super::super::Foundation::HANDLE) -> u32 {
    windows_targets::link!("advapi32.dll" "system" fn PerfStartProviderEx(providerguid : *const windows_core::GUID, providercontext : *const PERF_PROVIDER_CONTEXT, provider : *mut super::super::Foundation:: HANDLE) -> u32);
    PerfStartProviderEx(providerguid, core::mem::transmute(providercontext.unwrap_or(std::ptr::null())), provider)
}
#[inline]
pub unsafe fn PerfStopProvider<P0>(providerhandle: P0) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn PerfStopProvider(providerhandle : super::super::Foundation:: HANDLE) -> u32);
    PerfStopProvider(providerhandle.param().abi())
}
#[inline]
pub unsafe fn QueryPerformanceCounter(lpperformancecount: *mut i64) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn QueryPerformanceCounter(lpperformancecount : *mut i64) -> super::super::Foundation:: BOOL);
    QueryPerformanceCounter(lpperformancecount).ok()
}
#[inline]
pub unsafe fn QueryPerformanceFrequency(lpfrequency: *mut i64) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn QueryPerformanceFrequency(lpfrequency : *mut i64) -> super::super::Foundation:: BOOL);
    QueryPerformanceFrequency(lpfrequency).ok()
}
#[inline]
pub unsafe fn RestorePerfRegistryFromFileW<P0, P1>(szfilename: P0, szlangid: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("loadperf.dll" "system" fn RestorePerfRegistryFromFileW(szfilename : windows_core::PCWSTR, szlangid : windows_core::PCWSTR) -> u32);
    RestorePerfRegistryFromFileW(szfilename.param().abi(), szlangid.param().abi())
}
#[inline]
pub unsafe fn SetServiceAsTrustedA<P0, P1>(szreserved: P0, szservicename: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("loadperf.dll" "system" fn SetServiceAsTrustedA(szreserved : windows_core::PCSTR, szservicename : windows_core::PCSTR) -> u32);
    SetServiceAsTrustedA(szreserved.param().abi(), szservicename.param().abi())
}
#[inline]
pub unsafe fn SetServiceAsTrustedW<P0, P1>(szreserved: P0, szservicename: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("loadperf.dll" "system" fn SetServiceAsTrustedW(szreserved : windows_core::PCWSTR, szservicename : windows_core::PCWSTR) -> u32);
    SetServiceAsTrustedW(szreserved.param().abi(), szservicename.param().abi())
}
#[inline]
pub unsafe fn UnloadPerfCounterTextStringsA<P0, P1>(lpcommandline: P0, bquietmodearg: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("loadperf.dll" "system" fn UnloadPerfCounterTextStringsA(lpcommandline : windows_core::PCSTR, bquietmodearg : super::super::Foundation:: BOOL) -> u32);
    UnloadPerfCounterTextStringsA(lpcommandline.param().abi(), bquietmodearg.param().abi())
}
#[inline]
pub unsafe fn UnloadPerfCounterTextStringsW<P0, P1>(lpcommandline: P0, bquietmodearg: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("loadperf.dll" "system" fn UnloadPerfCounterTextStringsW(lpcommandline : windows_core::PCWSTR, bquietmodearg : super::super::Foundation:: BOOL) -> u32);
    UnloadPerfCounterTextStringsW(lpcommandline.param().abi(), bquietmodearg.param().abi())
}
#[inline]
pub unsafe fn UpdatePerfNameFilesA<P0, P1, P2>(sznewctrfilepath: P0, sznewhlpfilepath: P1, szlanguageid: P2, dwflags: usize) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("loadperf.dll" "system" fn UpdatePerfNameFilesA(sznewctrfilepath : windows_core::PCSTR, sznewhlpfilepath : windows_core::PCSTR, szlanguageid : windows_core::PCSTR, dwflags : usize) -> u32);
    UpdatePerfNameFilesA(sznewctrfilepath.param().abi(), sznewhlpfilepath.param().abi(), szlanguageid.param().abi(), dwflags)
}
#[inline]
pub unsafe fn UpdatePerfNameFilesW<P0, P1, P2>(sznewctrfilepath: P0, sznewhlpfilepath: P1, szlanguageid: P2, dwflags: usize) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("loadperf.dll" "system" fn UpdatePerfNameFilesW(sznewctrfilepath : windows_core::PCWSTR, sznewhlpfilepath : windows_core::PCWSTR, szlanguageid : windows_core::PCWSTR, dwflags : usize) -> u32);
    UpdatePerfNameFilesW(sznewctrfilepath.param().abi(), sznewhlpfilepath.param().abi(), szlanguageid.param().abi(), dwflags)
}
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
impl DICounterItem {}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct DICounterItem_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
}
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
impl DILogFileItem {}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct DILogFileItem_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
}
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
impl DISystemMonitor {}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct DISystemMonitor_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
}
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
impl DISystemMonitorEvents {}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct DISystemMonitorEvents_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
}
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
impl DISystemMonitorInternal {}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct DISystemMonitorInternal_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
}
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
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AlertThresholds(&self) -> windows_core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AlertThresholds)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetAlertThresholds(&self, alerts: *const super::Com::SAFEARRAY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAlertThresholds)(windows_core::Interface::as_raw(self), alerts).ok()
    }
    pub unsafe fn EventLog(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EventLog)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetEventLog<P0>(&self, log: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetEventLog)(windows_core::Interface::as_raw(self), log.param().abi()).ok()
    }
    pub unsafe fn SampleInterval(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SampleInterval)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSampleInterval(&self, interval: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSampleInterval)(windows_core::Interface::as_raw(self), interval).ok()
    }
    pub unsafe fn Task(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Task)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetTask<P0>(&self, task: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetTask)(windows_core::Interface::as_raw(self), task.param().abi()).ok()
    }
    pub unsafe fn TaskRunAsSelf(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TaskRunAsSelf)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetTaskRunAsSelf<P0>(&self, runasself: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetTaskRunAsSelf)(windows_core::Interface::as_raw(self), runasself.param().abi()).ok()
    }
    pub unsafe fn TaskArguments(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TaskArguments)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetTaskArguments<P0>(&self, task: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetTaskArguments)(windows_core::Interface::as_raw(self), task.param().abi()).ok()
    }
    pub unsafe fn TaskUserTextArguments(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TaskUserTextArguments)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetTaskUserTextArguments<P0>(&self, task: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetTaskUserTextArguments)(windows_core::Interface::as_raw(self), task.param().abi()).ok()
    }
    pub unsafe fn TriggerDataCollectorSet(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TriggerDataCollectorSet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetTriggerDataCollectorSet<P0>(&self, name: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetTriggerDataCollectorSet)(windows_core::Interface::as_raw(self), name.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAlertDataCollector_Vtbl {
    pub base__: IDataCollector_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub AlertThresholds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AlertThresholds: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetAlertThresholds: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetAlertThresholds: usize,
    pub EventLog: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetEventLog: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SampleInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetSampleInterval: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Task: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetTask: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub TaskRunAsSelf: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetTaskRunAsSelf: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub TaskArguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetTaskArguments: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub TaskUserTextArguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetTaskUserTextArguments: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub TriggerDataCollectorSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetTriggerDataCollectorSet: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LogApiNamesOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetLogApiNamesOnly<P0>(&self, logapinames: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetLogApiNamesOnly)(windows_core::Interface::as_raw(self), logapinames.param().abi()).ok()
    }
    pub unsafe fn LogApisRecursively(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LogApisRecursively)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetLogApisRecursively<P0>(&self, logrecursively: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetLogApisRecursively)(windows_core::Interface::as_raw(self), logrecursively.param().abi()).ok()
    }
    pub unsafe fn ExePath(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ExePath)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetExePath<P0>(&self, exepath: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetExePath)(windows_core::Interface::as_raw(self), exepath.param().abi()).ok()
    }
    pub unsafe fn LogFilePath(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LogFilePath)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetLogFilePath<P0>(&self, logfilepath: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetLogFilePath)(windows_core::Interface::as_raw(self), logfilepath.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IncludeModules(&self) -> windows_core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IncludeModules)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetIncludeModules(&self, includemodules: *const super::Com::SAFEARRAY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetIncludeModules)(windows_core::Interface::as_raw(self), includemodules).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IncludeApis(&self) -> windows_core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IncludeApis)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetIncludeApis(&self, includeapis: *const super::Com::SAFEARRAY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetIncludeApis)(windows_core::Interface::as_raw(self), includeapis).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExcludeApis(&self) -> windows_core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ExcludeApis)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetExcludeApis(&self, excludeapis: *const super::Com::SAFEARRAY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetExcludeApis)(windows_core::Interface::as_raw(self), excludeapis).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IApiTracingDataCollector_Vtbl {
    pub base__: IDataCollector_Vtbl,
    pub LogApiNamesOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetLogApiNamesOnly: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub LogApisRecursively: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetLogApisRecursively: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ExePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetExePath: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub LogFilePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetLogFilePath: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub IncludeModules: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IncludeModules: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetIncludeModules: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetIncludeModules: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub IncludeApis: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IncludeApis: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetIncludeApis: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetIncludeApis: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ExcludeApis: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ExcludeApis: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetExcludeApis: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetExcludeApis: usize,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FileMaxCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetFileMaxCount(&self, count: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFileMaxCount)(windows_core::Interface::as_raw(self), count).ok()
    }
    pub unsafe fn FileMaxRecursiveDepth(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FileMaxRecursiveDepth)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetFileMaxRecursiveDepth(&self, depth: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFileMaxRecursiveDepth)(windows_core::Interface::as_raw(self), depth).ok()
    }
    pub unsafe fn FileMaxTotalSize(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FileMaxTotalSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetFileMaxTotalSize(&self, size: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFileMaxTotalSize)(windows_core::Interface::as_raw(self), size).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Files(&self) -> windows_core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Files)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetFiles(&self, files: *const super::Com::SAFEARRAY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFiles)(windows_core::Interface::as_raw(self), files).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ManagementQueries(&self) -> windows_core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ManagementQueries)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetManagementQueries(&self, queries: *const super::Com::SAFEARRAY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetManagementQueries)(windows_core::Interface::as_raw(self), queries).ok()
    }
    pub unsafe fn QueryNetworkAdapters(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryNetworkAdapters)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetQueryNetworkAdapters<P0>(&self, network: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetQueryNetworkAdapters)(windows_core::Interface::as_raw(self), network.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RegistryKeys(&self) -> windows_core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RegistryKeys)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetRegistryKeys(&self, query: *const super::Com::SAFEARRAY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetRegistryKeys)(windows_core::Interface::as_raw(self), query).ok()
    }
    pub unsafe fn RegistryMaxRecursiveDepth(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RegistryMaxRecursiveDepth)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetRegistryMaxRecursiveDepth(&self, depth: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetRegistryMaxRecursiveDepth)(windows_core::Interface::as_raw(self), depth).ok()
    }
    pub unsafe fn SystemStateFile(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SystemStateFile)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSystemStateFile<P0>(&self, filename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetSystemStateFile)(windows_core::Interface::as_raw(self), filename.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IConfigurationDataCollector_Vtbl {
    pub base__: IDataCollector_Vtbl,
    pub FileMaxCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetFileMaxCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub FileMaxRecursiveDepth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetFileMaxRecursiveDepth: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub FileMaxTotalSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetFileMaxTotalSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Files: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Files: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetFiles: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetFiles: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ManagementQueries: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ManagementQueries: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetManagementQueries: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetManagementQueries: usize,
    pub QueryNetworkAdapters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetQueryNetworkAdapters: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RegistryKeys: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RegistryKeys: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetRegistryKeys: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetRegistryKeys: usize,
    pub RegistryMaxRecursiveDepth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetRegistryMaxRecursiveDepth: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SystemStateFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetSystemStateFile: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICounterItem, ICounterItem_Vtbl, 0x771a9520_ee28_11ce_941e_008029004347);
impl core::ops::Deref for ICounterItem {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICounterItem, windows_core::IUnknown);
impl ICounterItem {
    pub unsafe fn Value(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetColor(&self, color: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetColor)(windows_core::Interface::as_raw(self), color).ok()
    }
    pub unsafe fn Color(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Color)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetWidth(&self, iwidth: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetWidth)(windows_core::Interface::as_raw(self), iwidth).ok()
    }
    pub unsafe fn Width(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Width)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetLineStyle(&self, ilinestyle: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLineStyle)(windows_core::Interface::as_raw(self), ilinestyle).ok()
    }
    pub unsafe fn LineStyle(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LineStyle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetScaleFactor(&self, iscale: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetScaleFactor)(windows_core::Interface::as_raw(self), iscale).ok()
    }
    pub unsafe fn ScaleFactor(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ScaleFactor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Path(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Path)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetValue(&self, value: *mut f64, status: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), value, status).ok()
    }
    pub unsafe fn GetStatistics(&self, max: *mut f64, min: *mut f64, avg: *mut f64, status: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetStatistics)(windows_core::Interface::as_raw(self), max, min, avg, status).ok()
    }
}
#[repr(C)]
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
    pub Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64, *mut i32) -> windows_core::HRESULT,
    pub GetStatistics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64, *mut f64, *mut f64, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICounterItem2, ICounterItem2_Vtbl, 0xeefcd4e1_ea1c_4435_b7f4_e341ba03b4f9);
impl core::ops::Deref for ICounterItem2 {
    type Target = ICounterItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICounterItem2, windows_core::IUnknown, ICounterItem);
impl ICounterItem2 {
    pub unsafe fn SetSelected<P0>(&self, bstate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetSelected)(windows_core::Interface::as_raw(self), bstate.param().abi()).ok()
    }
    pub unsafe fn Selected(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Selected)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetVisible<P0>(&self, bstate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetVisible)(windows_core::Interface::as_raw(self), bstate.param().abi()).ok()
    }
    pub unsafe fn Visible(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Visible)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetDataAt(&self, iindex: i32, iwhich: SysmonDataType) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDataAt)(windows_core::Interface::as_raw(self), iindex, iwhich, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ICounterItem2_Vtbl {
    pub base__: ICounterItem_Vtbl,
    pub SetSelected: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Selected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetVisible: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Visible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub GetDataAt: unsafe extern "system" fn(*mut core::ffi::c_void, i32, SysmonDataType, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item<P0>(&self, index: P0) -> windows_core::Result<DICounterItem>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<P0>(&self, pathname: P0) -> windows_core::Result<DICounterItem>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), pathname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Remove<P0>(&self, index: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), index.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICounters_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
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
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DataCollectorSet(&self) -> windows_core::Result<IDataCollectorSet> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DataCollectorSet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetDataCollectorSet<P0>(&self, group: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDataCollectorSet>,
    {
        (windows_core::Interface::vtable(self).SetDataCollectorSet)(windows_core::Interface::as_raw(self), group.param().abi()).ok()
    }
    pub unsafe fn DataCollectorType(&self) -> windows_core::Result<DataCollectorType> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DataCollectorType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn FileName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FileName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetFileName<P0>(&self, name: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetFileName)(windows_core::Interface::as_raw(self), name.param().abi()).ok()
    }
    pub unsafe fn FileNameFormat(&self) -> windows_core::Result<AutoPathFormat> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FileNameFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetFileNameFormat(&self, format: AutoPathFormat) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFileNameFormat)(windows_core::Interface::as_raw(self), format).ok()
    }
    pub unsafe fn FileNameFormatPattern(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FileNameFormatPattern)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetFileNameFormatPattern<P0>(&self, pattern: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetFileNameFormatPattern)(windows_core::Interface::as_raw(self), pattern.param().abi()).ok()
    }
    pub unsafe fn LatestOutputLocation(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LatestOutputLocation)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetLatestOutputLocation<P0>(&self, path: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetLatestOutputLocation)(windows_core::Interface::as_raw(self), path.param().abi()).ok()
    }
    pub unsafe fn LogAppend(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LogAppend)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetLogAppend<P0>(&self, append: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetLogAppend)(windows_core::Interface::as_raw(self), append.param().abi()).ok()
    }
    pub unsafe fn LogCircular(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LogCircular)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetLogCircular<P0>(&self, circular: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetLogCircular)(windows_core::Interface::as_raw(self), circular.param().abi()).ok()
    }
    pub unsafe fn LogOverwrite(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LogOverwrite)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetLogOverwrite<P0>(&self, overwrite: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetLogOverwrite)(windows_core::Interface::as_raw(self), overwrite.param().abi()).ok()
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), name.param().abi()).ok()
    }
    pub unsafe fn OutputLocation(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OutputLocation)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Index(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Index)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetIndex(&self, index: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetIndex)(windows_core::Interface::as_raw(self), index).ok()
    }
    pub unsafe fn Xml(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Xml)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetXml<P0>(&self, xml: P0) -> windows_core::Result<IValueMap>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SetXml)(windows_core::Interface::as_raw(self), xml.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateOutputLocation<P0>(&self, latest: P0) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateOutputLocation)(windows_core::Interface::as_raw(self), latest.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDataCollector_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub DataCollectorSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DataCollectorSet: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetDataCollectorSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetDataCollectorSet: usize,
    pub DataCollectorType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DataCollectorType) -> windows_core::HRESULT,
    pub FileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetFileName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub FileNameFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AutoPathFormat) -> windows_core::HRESULT,
    pub SetFileNameFormat: unsafe extern "system" fn(*mut core::ffi::c_void, AutoPathFormat) -> windows_core::HRESULT,
    pub FileNameFormatPattern: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetFileNameFormatPattern: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub LatestOutputLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetLatestOutputLocation: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub LogAppend: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetLogAppend: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub LogCircular: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetLogCircular: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub LogOverwrite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetLogOverwrite: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub OutputLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Index: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Xml: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetXml: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetXml: usize,
    pub CreateOutputLocation: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item<P0>(&self, index: P0) -> windows_core::Result<IDataCollector>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<P0>(&self, collector: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDataCollector>,
    {
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), collector.param().abi()).ok()
    }
    pub unsafe fn Remove<P0>(&self, collector: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), collector.param().abi()).ok()
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddRange<P0>(&self, collectors: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDataCollectorCollection>,
    {
        (windows_core::Interface::vtable(self).AddRange)(windows_core::Interface::as_raw(self), collectors.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateDataCollectorFromXml<P0>(&self, bstrxml: P0, pvalidation: *mut Option<IValueMap>, pcollector: *mut Option<IDataCollector>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).CreateDataCollectorFromXml)(windows_core::Interface::as_raw(self), bstrxml.param().abi(), core::mem::transmute(pvalidation), core::mem::transmute(pcollector)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateDataCollector(&self, r#type: DataCollectorType) -> windows_core::Result<IDataCollector> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateDataCollector)(windows_core::Interface::as_raw(self), r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDataCollectorCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddRange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateDataCollectorFromXml: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateDataCollectorFromXml: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateDataCollector: unsafe extern "system" fn(*mut core::ffi::c_void, DataCollectorType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateDataCollector: usize,
}
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
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DataCollectors(&self) -> windows_core::Result<IDataCollectorCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DataCollectors)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Duration(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Duration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDuration(&self, seconds: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDuration)(windows_core::Interface::as_raw(self), seconds).ok()
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDescription<P0>(&self, description: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), description.param().abi()).ok()
    }
    pub unsafe fn DescriptionUnresolved(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DescriptionUnresolved)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DisplayName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDisplayName<P0>(&self, displayname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDisplayName)(windows_core::Interface::as_raw(self), displayname.param().abi()).ok()
    }
    pub unsafe fn DisplayNameUnresolved(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DisplayNameUnresolved)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Keywords(&self) -> windows_core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Keywords)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetKeywords(&self, keywords: *const super::Com::SAFEARRAY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetKeywords)(windows_core::Interface::as_raw(self), keywords).ok()
    }
    pub unsafe fn LatestOutputLocation(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LatestOutputLocation)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetLatestOutputLocation<P0>(&self, path: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetLatestOutputLocation)(windows_core::Interface::as_raw(self), path.param().abi()).ok()
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn OutputLocation(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OutputLocation)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RootPath(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RootPath)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetRootPath<P0>(&self, folder: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetRootPath)(windows_core::Interface::as_raw(self), folder.param().abi()).ok()
    }
    pub unsafe fn Segment(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Segment)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSegment<P0>(&self, segment: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetSegment)(windows_core::Interface::as_raw(self), segment.param().abi()).ok()
    }
    pub unsafe fn SegmentMaxDuration(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SegmentMaxDuration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSegmentMaxDuration(&self, seconds: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSegmentMaxDuration)(windows_core::Interface::as_raw(self), seconds).ok()
    }
    pub unsafe fn SegmentMaxSize(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SegmentMaxSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSegmentMaxSize(&self, size: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSegmentMaxSize)(windows_core::Interface::as_raw(self), size).ok()
    }
    pub unsafe fn SerialNumber(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SerialNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSerialNumber(&self, index: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSerialNumber)(windows_core::Interface::as_raw(self), index).ok()
    }
    pub unsafe fn Server(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Server)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Status(&self) -> windows_core::Result<DataCollectorSetStatus> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Status)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Subdirectory(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Subdirectory)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSubdirectory<P0>(&self, folder: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetSubdirectory)(windows_core::Interface::as_raw(self), folder.param().abi()).ok()
    }
    pub unsafe fn SubdirectoryFormat(&self) -> windows_core::Result<AutoPathFormat> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SubdirectoryFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSubdirectoryFormat(&self, format: AutoPathFormat) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSubdirectoryFormat)(windows_core::Interface::as_raw(self), format).ok()
    }
    pub unsafe fn SubdirectoryFormatPattern(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SubdirectoryFormatPattern)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSubdirectoryFormatPattern<P0>(&self, pattern: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetSubdirectoryFormatPattern)(windows_core::Interface::as_raw(self), pattern.param().abi()).ok()
    }
    pub unsafe fn Task(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Task)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetTask<P0>(&self, task: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetTask)(windows_core::Interface::as_raw(self), task.param().abi()).ok()
    }
    pub unsafe fn TaskRunAsSelf(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TaskRunAsSelf)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetTaskRunAsSelf<P0>(&self, runasself: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetTaskRunAsSelf)(windows_core::Interface::as_raw(self), runasself.param().abi()).ok()
    }
    pub unsafe fn TaskArguments(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TaskArguments)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetTaskArguments<P0>(&self, task: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetTaskArguments)(windows_core::Interface::as_raw(self), task.param().abi()).ok()
    }
    pub unsafe fn TaskUserTextArguments(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TaskUserTextArguments)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetTaskUserTextArguments<P0>(&self, usertext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetTaskUserTextArguments)(windows_core::Interface::as_raw(self), usertext.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Schedules(&self) -> windows_core::Result<IScheduleCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Schedules)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SchedulesEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SchedulesEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSchedulesEnabled<P0>(&self, enabled: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetSchedulesEnabled)(windows_core::Interface::as_raw(self), enabled.param().abi()).ok()
    }
    pub unsafe fn UserAccount(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UserAccount)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Xml(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Xml)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Security(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Security)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSecurity<P0>(&self, bstrsecurity: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetSecurity)(windows_core::Interface::as_raw(self), bstrsecurity.param().abi()).ok()
    }
    pub unsafe fn StopOnCompletion(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StopOnCompletion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetStopOnCompletion<P0>(&self, stop: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetStopOnCompletion)(windows_core::Interface::as_raw(self), stop.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DataManager(&self) -> windows_core::Result<IDataManager> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DataManager)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetCredentials<P0, P1>(&self, user: P0, password: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetCredentials)(windows_core::Interface::as_raw(self), user.param().abi(), password.param().abi()).ok()
    }
    pub unsafe fn Query<P0, P1>(&self, name: P0, server: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Query)(windows_core::Interface::as_raw(self), name.param().abi(), server.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Commit<P0, P1>(&self, name: P0, server: P1, mode: CommitMode) -> windows_core::Result<IValueMap>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self), name.param().abi(), server.param().abi(), mode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Delete(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Start<P0>(&self, synchronous: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self), synchronous.param().abi()).ok()
    }
    pub unsafe fn Stop<P0>(&self, synchronous: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self), synchronous.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetXml<P0>(&self, xml: P0) -> windows_core::Result<IValueMap>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SetXml)(windows_core::Interface::as_raw(self), xml.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetValue<P0, P1>(&self, key: P0, value: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), key.param().abi(), value.param().abi()).ok()
    }
    pub unsafe fn GetValue<P0>(&self, key: P0) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), key.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDataCollectorSet_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub DataCollectors: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DataCollectors: usize,
    pub Duration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetDuration: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub DescriptionUnresolved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub DisplayNameUnresolved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Keywords: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Keywords: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetKeywords: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetKeywords: usize,
    pub LatestOutputLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetLatestOutputLocation: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub OutputLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub RootPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetRootPath: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Segment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetSegment: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SegmentMaxDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetSegmentMaxDuration: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SegmentMaxSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetSegmentMaxSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SerialNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetSerialNumber: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Server: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DataCollectorSetStatus) -> windows_core::HRESULT,
    pub Subdirectory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetSubdirectory: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SubdirectoryFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AutoPathFormat) -> windows_core::HRESULT,
    pub SetSubdirectoryFormat: unsafe extern "system" fn(*mut core::ffi::c_void, AutoPathFormat) -> windows_core::HRESULT,
    pub SubdirectoryFormatPattern: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetSubdirectoryFormatPattern: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Task: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetTask: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub TaskRunAsSelf: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetTaskRunAsSelf: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub TaskArguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetTaskArguments: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub TaskUserTextArguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetTaskUserTextArguments: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Schedules: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Schedules: usize,
    pub SchedulesEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetSchedulesEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub UserAccount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Xml: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Security: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetSecurity: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub StopOnCompletion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetStopOnCompletion: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub DataManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DataManager: usize,
    pub SetCredentials: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Query: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, CommitMode, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Commit: usize,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetXml: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetXml: usize,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item<P0>(&self, index: P0) -> windows_core::Result<IDataCollectorSet>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<P0>(&self, set: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDataCollectorSet>,
    {
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), set.param().abi()).ok()
    }
    pub unsafe fn Remove<P0>(&self, set: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), set.param().abi()).ok()
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddRange<P0>(&self, sets: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDataCollectorSetCollection>,
    {
        (windows_core::Interface::vtable(self).AddRange)(windows_core::Interface::as_raw(self), sets.param().abi()).ok()
    }
    pub unsafe fn GetDataCollectorSets<P0, P1>(&self, server: P0, filter: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).GetDataCollectorSets)(windows_core::Interface::as_raw(self), server.param().abi(), filter.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDataCollectorSetCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddRange: usize,
    pub GetDataCollectorSets: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetEnabled<P0>(&self, fenabled: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetEnabled)(windows_core::Interface::as_raw(self), fenabled.param().abi()).ok()
    }
    pub unsafe fn CheckBeforeRunning(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CheckBeforeRunning)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetCheckBeforeRunning<P0>(&self, fcheck: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetCheckBeforeRunning)(windows_core::Interface::as_raw(self), fcheck.param().abi()).ok()
    }
    pub unsafe fn MinFreeDisk(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MinFreeDisk)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMinFreeDisk(&self, minfreedisk: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMinFreeDisk)(windows_core::Interface::as_raw(self), minfreedisk).ok()
    }
    pub unsafe fn MaxSize(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MaxSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMaxSize(&self, ulmaxsize: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMaxSize)(windows_core::Interface::as_raw(self), ulmaxsize).ok()
    }
    pub unsafe fn MaxFolderCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MaxFolderCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMaxFolderCount(&self, ulmaxfoldercount: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMaxFolderCount)(windows_core::Interface::as_raw(self), ulmaxfoldercount).ok()
    }
    pub unsafe fn ResourcePolicy(&self) -> windows_core::Result<ResourcePolicy> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ResourcePolicy)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetResourcePolicy(&self, policy: ResourcePolicy) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetResourcePolicy)(windows_core::Interface::as_raw(self), policy).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn FolderActions(&self) -> windows_core::Result<IFolderActionCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FolderActions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ReportSchema(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReportSchema)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetReportSchema<P0>(&self, reportschema: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetReportSchema)(windows_core::Interface::as_raw(self), reportschema.param().abi()).ok()
    }
    pub unsafe fn ReportFileName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReportFileName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetReportFileName<P0>(&self, pbstrfilename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetReportFileName)(windows_core::Interface::as_raw(self), pbstrfilename.param().abi()).ok()
    }
    pub unsafe fn RuleTargetFileName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RuleTargetFileName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetRuleTargetFileName<P0>(&self, filename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetRuleTargetFileName)(windows_core::Interface::as_raw(self), filename.param().abi()).ok()
    }
    pub unsafe fn EventsFileName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EventsFileName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetEventsFileName<P0>(&self, pbstrfilename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetEventsFileName)(windows_core::Interface::as_raw(self), pbstrfilename.param().abi()).ok()
    }
    pub unsafe fn Rules(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Rules)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetRules<P0>(&self, bstrxml: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetRules)(windows_core::Interface::as_raw(self), bstrxml.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Run<P0>(&self, steps: DataManagerSteps, bstrfolder: P0) -> windows_core::Result<IValueMap>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Run)(windows_core::Interface::as_raw(self), steps, bstrfolder.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Extract<P0, P1>(&self, cabfilename: P0, destinationpath: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Extract)(windows_core::Interface::as_raw(self), cabfilename.param().abi(), destinationpath.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
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
    #[cfg(feature = "Win32_System_Com")]
    pub FolderActions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    FolderActions: usize,
    pub ReportSchema: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetReportSchema: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ReportFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetReportFileName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub RuleTargetFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetRuleTargetFileName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub EventsFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetEventsFileName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Rules: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetRules: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Run: unsafe extern "system" fn(*mut core::ffi::c_void, DataManagerSteps, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Run: usize,
    pub Extract: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Age)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAge(&self, ulage: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAge)(windows_core::Interface::as_raw(self), ulage).ok()
    }
    pub unsafe fn Size(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Size)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSize(&self, ulage: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSize)(windows_core::Interface::as_raw(self), ulage).ok()
    }
    pub unsafe fn Actions(&self) -> windows_core::Result<FolderActionSteps> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Actions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetActions(&self, steps: FolderActionSteps) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetActions)(windows_core::Interface::as_raw(self), steps).ok()
    }
    pub unsafe fn SendCabTo(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SendCabTo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSendCabTo<P0>(&self, bstrdestination: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetSendCabTo)(windows_core::Interface::as_raw(self), bstrdestination.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFolderAction_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Age: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetAge: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Actions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FolderActionSteps) -> windows_core::HRESULT,
    pub SetActions: unsafe extern "system" fn(*mut core::ffi::c_void, FolderActionSteps) -> windows_core::HRESULT,
    pub SendCabTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetSendCabTo: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item<P0>(&self, index: P0) -> windows_core::Result<IFolderAction>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<P0>(&self, action: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFolderAction>,
    {
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), action.param().abi()).ok()
    }
    pub unsafe fn Remove<P0>(&self, index: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), index.param().abi()).ok()
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddRange<P0>(&self, actions: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFolderActionCollection>,
    {
        (windows_core::Interface::vtable(self).AddRange)(windows_core::Interface::as_raw(self), actions.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateFolderAction(&self) -> windows_core::Result<IFolderAction> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateFolderAction)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFolderActionCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddRange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateFolderAction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateFolderAction: usize,
}
windows_core::imp::define_interface!(ILogFileItem, ILogFileItem_Vtbl, 0xd6b518dd_05c7_418a_89e6_4f9ce8c6841e);
impl core::ops::Deref for ILogFileItem {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ILogFileItem, windows_core::IUnknown);
impl ILogFileItem {
    pub unsafe fn Path(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Path)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ILogFileItem_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item<P0>(&self, index: P0) -> windows_core::Result<DILogFileItem>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<P0>(&self, pathname: P0) -> windows_core::Result<DILogFileItem>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), pathname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Remove<P0>(&self, index: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), index.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ILogFiles_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DataSourceName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDataSourceName<P0>(&self, dsn: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDataSourceName)(windows_core::Interface::as_raw(self), dsn.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PerformanceCounters(&self) -> windows_core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PerformanceCounters)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetPerformanceCounters(&self, counters: *const super::Com::SAFEARRAY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPerformanceCounters)(windows_core::Interface::as_raw(self), counters).ok()
    }
    pub unsafe fn LogFileFormat(&self) -> windows_core::Result<FileFormat> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LogFileFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetLogFileFormat(&self, format: FileFormat) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLogFileFormat)(windows_core::Interface::as_raw(self), format).ok()
    }
    pub unsafe fn SampleInterval(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SampleInterval)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSampleInterval(&self, interval: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSampleInterval)(windows_core::Interface::as_raw(self), interval).ok()
    }
    pub unsafe fn SegmentMaxRecords(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SegmentMaxRecords)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSegmentMaxRecords(&self, records: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSegmentMaxRecords)(windows_core::Interface::as_raw(self), records).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IPerformanceCounterDataCollector_Vtbl {
    pub base__: IDataCollector_Vtbl,
    pub DataSourceName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDataSourceName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub PerformanceCounters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PerformanceCounters: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetPerformanceCounters: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetPerformanceCounters: usize,
    pub LogFileFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FileFormat) -> windows_core::HRESULT,
    pub SetLogFileFormat: unsafe extern "system" fn(*mut core::ffi::c_void, FileFormat) -> windows_core::HRESULT,
    pub SampleInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetSampleInterval: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SegmentMaxRecords: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetSegmentMaxRecords: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
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
    pub unsafe fn StartDate(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StartDate)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetStartDate<P0>(&self, start: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetStartDate)(windows_core::Interface::as_raw(self), start.param().abi()).ok()
    }
    pub unsafe fn EndDate(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EndDate)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetEndDate<P0>(&self, end: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetEndDate)(windows_core::Interface::as_raw(self), end.param().abi()).ok()
    }
    pub unsafe fn StartTime(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StartTime)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetStartTime<P0>(&self, start: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetStartTime)(windows_core::Interface::as_raw(self), start.param().abi()).ok()
    }
    pub unsafe fn Days(&self) -> windows_core::Result<WeekDays> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Days)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDays(&self, days: WeekDays) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDays)(windows_core::Interface::as_raw(self), days).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISchedule_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub StartDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetStartDate: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub EndDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetEndDate: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub StartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetStartTime: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Days: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WeekDays) -> windows_core::HRESULT,
    pub SetDays: unsafe extern "system" fn(*mut core::ffi::c_void, WeekDays) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item<P0>(&self, index: P0) -> windows_core::Result<ISchedule>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<P0>(&self, pschedule: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISchedule>,
    {
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), pschedule.param().abi()).ok()
    }
    pub unsafe fn Remove<P0>(&self, vschedule: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), vschedule.param().abi()).ok()
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddRange<P0>(&self, pschedules: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IScheduleCollection>,
    {
        (windows_core::Interface::vtable(self).AddRange)(windows_core::Interface::as_raw(self), pschedules.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateSchedule(&self) -> windows_core::Result<ISchedule> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateSchedule)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IScheduleCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddRange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateSchedule: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateSchedule: usize,
}
windows_core::imp::define_interface!(ISystemMonitor, ISystemMonitor_Vtbl, 0x194eb241_c32c_11cf_9398_00aa00a3ddea);
impl core::ops::Deref for ISystemMonitor {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISystemMonitor, windows_core::IUnknown);
impl ISystemMonitor {
    pub unsafe fn Appearance(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Appearance)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAppearance(&self, iappearance: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAppearance)(windows_core::Interface::as_raw(self), iappearance).ok()
    }
    pub unsafe fn BackColor(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BackColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetBackColor(&self, color: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBackColor)(windows_core::Interface::as_raw(self), color).ok()
    }
    pub unsafe fn BorderStyle(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BorderStyle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetBorderStyle(&self, iborderstyle: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBorderStyle)(windows_core::Interface::as_raw(self), iborderstyle).ok()
    }
    pub unsafe fn ForeColor(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ForeColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetForeColor(&self, color: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetForeColor)(windows_core::Interface::as_raw(self), color).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Font(&self) -> windows_core::Result<super::Ole::IFontDisp> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Font)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn putref_Font<P0>(&self, pfont: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Ole::IFontDisp>,
    {
        (windows_core::Interface::vtable(self).putref_Font)(windows_core::Interface::as_raw(self), pfont.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Counters(&self) -> windows_core::Result<ICounters> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Counters)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetShowVerticalGrid<P0>(&self, bstate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetShowVerticalGrid)(windows_core::Interface::as_raw(self), bstate.param().abi()).ok()
    }
    pub unsafe fn ShowVerticalGrid(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ShowVerticalGrid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetShowHorizontalGrid<P0>(&self, bstate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetShowHorizontalGrid)(windows_core::Interface::as_raw(self), bstate.param().abi()).ok()
    }
    pub unsafe fn ShowHorizontalGrid(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ShowHorizontalGrid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetShowLegend<P0>(&self, bstate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetShowLegend)(windows_core::Interface::as_raw(self), bstate.param().abi()).ok()
    }
    pub unsafe fn ShowLegend(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ShowLegend)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetShowScaleLabels<P0>(&self, bstate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetShowScaleLabels)(windows_core::Interface::as_raw(self), bstate.param().abi()).ok()
    }
    pub unsafe fn ShowScaleLabels(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ShowScaleLabels)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetShowValueBar<P0>(&self, bstate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetShowValueBar)(windows_core::Interface::as_raw(self), bstate.param().abi()).ok()
    }
    pub unsafe fn ShowValueBar(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ShowValueBar)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMaximumScale(&self, ivalue: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMaximumScale)(windows_core::Interface::as_raw(self), ivalue).ok()
    }
    pub unsafe fn MaximumScale(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MaximumScale)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMinimumScale(&self, ivalue: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMinimumScale)(windows_core::Interface::as_raw(self), ivalue).ok()
    }
    pub unsafe fn MinimumScale(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MinimumScale)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetUpdateInterval(&self, fvalue: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetUpdateInterval)(windows_core::Interface::as_raw(self), fvalue).ok()
    }
    pub unsafe fn UpdateInterval(&self) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UpdateInterval)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDisplayType(&self, edisplaytype: DisplayTypeConstants) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDisplayType)(windows_core::Interface::as_raw(self), edisplaytype).ok()
    }
    pub unsafe fn DisplayType(&self) -> windows_core::Result<DisplayTypeConstants> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DisplayType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetManualUpdate<P0>(&self, bstate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetManualUpdate)(windows_core::Interface::as_raw(self), bstate.param().abi()).ok()
    }
    pub unsafe fn ManualUpdate(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ManualUpdate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetGraphTitle<P0>(&self, bstitle: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetGraphTitle)(windows_core::Interface::as_raw(self), bstitle.param().abi()).ok()
    }
    pub unsafe fn GraphTitle(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GraphTitle)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetYAxisLabel<P0>(&self, bstitle: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetYAxisLabel)(windows_core::Interface::as_raw(self), bstitle.param().abi()).ok()
    }
    pub unsafe fn YAxisLabel(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).YAxisLabel)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CollectSample(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CollectSample)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn UpdateGraph(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UpdateGraph)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BrowseCounters(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).BrowseCounters)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DisplayProperties(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DisplayProperties)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Counter(&self, iindex: i32) -> windows_core::Result<ICounterItem> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Counter)(windows_core::Interface::as_raw(self), iindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AddCounter<P0>(&self, bspath: P0) -> windows_core::Result<ICounterItem>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AddCounter)(windows_core::Interface::as_raw(self), bspath.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DeleteCounter<P0>(&self, pctr: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICounterItem>,
    {
        (windows_core::Interface::vtable(self).DeleteCounter)(windows_core::Interface::as_raw(self), pctr.param().abi()).ok()
    }
    pub unsafe fn BackColorCtl(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BackColorCtl)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetBackColorCtl(&self, color: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBackColorCtl)(windows_core::Interface::as_raw(self), color).ok()
    }
    pub unsafe fn SetLogFileName<P0>(&self, bsfilename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetLogFileName)(windows_core::Interface::as_raw(self), bsfilename.param().abi()).ok()
    }
    pub unsafe fn LogFileName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LogFileName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetLogViewStart(&self, starttime: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLogViewStart)(windows_core::Interface::as_raw(self), starttime).ok()
    }
    pub unsafe fn LogViewStart(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LogViewStart)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetLogViewStop(&self, stoptime: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLogViewStop)(windows_core::Interface::as_raw(self), stoptime).ok()
    }
    pub unsafe fn LogViewStop(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LogViewStop)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GridColor(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GridColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetGridColor(&self, color: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetGridColor)(windows_core::Interface::as_raw(self), color).ok()
    }
    pub unsafe fn TimeBarColor(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TimeBarColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetTimeBarColor(&self, color: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTimeBarColor)(windows_core::Interface::as_raw(self), color).ok()
    }
    pub unsafe fn Highlight(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Highlight)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetHighlight<P0>(&self, bstate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetHighlight)(windows_core::Interface::as_raw(self), bstate.param().abi()).ok()
    }
    pub unsafe fn ShowToolbar(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ShowToolbar)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetShowToolbar<P0>(&self, bstate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetShowToolbar)(windows_core::Interface::as_raw(self), bstate.param().abi()).ok()
    }
    pub unsafe fn Paste(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Paste)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Copy(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Copy)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetReadOnly<P0>(&self, bstate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetReadOnly)(windows_core::Interface::as_raw(self), bstate.param().abi()).ok()
    }
    pub unsafe fn ReadOnly(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReadOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetReportValueType(&self, ereportvaluetype: ReportValueTypeConstants) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetReportValueType)(windows_core::Interface::as_raw(self), ereportvaluetype).ok()
    }
    pub unsafe fn ReportValueType(&self) -> windows_core::Result<ReportValueTypeConstants> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReportValueType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMonitorDuplicateInstances<P0>(&self, bstate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetMonitorDuplicateInstances)(windows_core::Interface::as_raw(self), bstate.param().abi()).ok()
    }
    pub unsafe fn MonitorDuplicateInstances(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MonitorDuplicateInstances)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDisplayFilter(&self, ivalue: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDisplayFilter)(windows_core::Interface::as_raw(self), ivalue).ok()
    }
    pub unsafe fn DisplayFilter(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DisplayFilter)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LogFiles(&self) -> windows_core::Result<ILogFiles> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LogFiles)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDataSourceType(&self, edatasourcetype: DataSourceTypeConstants) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDataSourceType)(windows_core::Interface::as_raw(self), edatasourcetype).ok()
    }
    pub unsafe fn DataSourceType(&self) -> windows_core::Result<DataSourceTypeConstants> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DataSourceType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSqlDsnName<P0>(&self, bssqldsnname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetSqlDsnName)(windows_core::Interface::as_raw(self), bssqldsnname.param().abi()).ok()
    }
    pub unsafe fn SqlDsnName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SqlDsnName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSqlLogSetName<P0>(&self, bssqllogsetname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetSqlLogSetName)(windows_core::Interface::as_raw(self), bssqllogsetname.param().abi()).ok()
    }
    pub unsafe fn SqlLogSetName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SqlLogSetName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
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
    pub SetGraphTitle: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GraphTitle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetYAxisLabel: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub YAxisLabel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CollectSample: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UpdateGraph: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BrowseCounters: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisplayProperties: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Counter: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddCounter: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteCounter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BackColorCtl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetBackColorCtl: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetLogFileName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub LogFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
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
    pub SetSqlDsnName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SqlDsnName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetSqlLogSetName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SqlLogSetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISystemMonitor2, ISystemMonitor2_Vtbl, 0x08e3206a_5fd2_4fde_a8a5_8cb3b63d2677);
impl core::ops::Deref for ISystemMonitor2 {
    type Target = ISystemMonitor;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISystemMonitor2, windows_core::IUnknown, ISystemMonitor);
impl ISystemMonitor2 {
    pub unsafe fn SetEnableDigitGrouping<P0>(&self, bstate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetEnableDigitGrouping)(windows_core::Interface::as_raw(self), bstate.param().abi()).ok()
    }
    pub unsafe fn EnableDigitGrouping(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnableDigitGrouping)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetEnableToolTips<P0>(&self, bstate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetEnableToolTips)(windows_core::Interface::as_raw(self), bstate.param().abi()).ok()
    }
    pub unsafe fn EnableToolTips(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnableToolTips)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetShowTimeAxisLabels<P0>(&self, bstate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetShowTimeAxisLabels)(windows_core::Interface::as_raw(self), bstate.param().abi()).ok()
    }
    pub unsafe fn ShowTimeAxisLabels(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ShowTimeAxisLabels)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetChartScroll<P0>(&self, bscroll: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetChartScroll)(windows_core::Interface::as_raw(self), bscroll.param().abi()).ok()
    }
    pub unsafe fn ChartScroll(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ChartScroll)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDataPointCount(&self, inewcount: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDataPointCount)(windows_core::Interface::as_raw(self), inewcount).ok()
    }
    pub unsafe fn DataPointCount(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DataPointCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ScaleToFit<P0>(&self, bselectedcountersonly: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).ScaleToFit)(windows_core::Interface::as_raw(self), bselectedcountersonly.param().abi()).ok()
    }
    pub unsafe fn SaveAs<P0>(&self, bstrfilename: P0, esysmonfiletype: SysmonFileType) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SaveAs)(windows_core::Interface::as_raw(self), bstrfilename.param().abi(), esysmonfiletype).ok()
    }
    pub unsafe fn Relog<P0>(&self, bstrfilename: P0, esysmonfiletype: SysmonFileType, ifilter: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Relog)(windows_core::Interface::as_raw(self), bstrfilename.param().abi(), esysmonfiletype, ifilter).ok()
    }
    pub unsafe fn ClearData(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ClearData)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn LogSourceStartTime(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LogSourceStartTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn LogSourceStopTime(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LogSourceStopTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetLogViewRange(&self, starttime: f64, stoptime: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLogViewRange)(windows_core::Interface::as_raw(self), starttime, stoptime).ok()
    }
    pub unsafe fn GetLogViewRange(&self, starttime: *mut f64, stoptime: *mut f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLogViewRange)(windows_core::Interface::as_raw(self), starttime, stoptime).ok()
    }
    pub unsafe fn BatchingLock<P0>(&self, flock: P0, ebatchreason: SysmonBatchReason) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).BatchingLock)(windows_core::Interface::as_raw(self), flock.param().abi(), ebatchreason).ok()
    }
    pub unsafe fn LoadSettings<P0>(&self, bstrsettingfilename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).LoadSettings)(windows_core::Interface::as_raw(self), bstrsettingfilename.param().abi()).ok()
    }
}
#[repr(C)]
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
    pub SaveAs: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, SysmonFileType) -> windows_core::HRESULT,
    pub Relog: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, SysmonFileType, i32) -> windows_core::HRESULT,
    pub ClearData: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LogSourceStartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub LogSourceStopTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetLogViewRange: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64) -> windows_core::HRESULT,
    pub GetLogViewRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64, *mut f64) -> windows_core::HRESULT,
    pub BatchingLock: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, SysmonBatchReason) -> windows_core::HRESULT,
    pub LoadSettings: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISystemMonitorEvents, ISystemMonitorEvents_Vtbl, 0xee660ea0_4abd_11cf_943a_008029004347);
impl core::ops::Deref for ISystemMonitorEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISystemMonitorEvents, windows_core::IUnknown);
impl ISystemMonitorEvents {
    pub unsafe fn OnCounterSelected(&self, index: i32) {
        (windows_core::Interface::vtable(self).OnCounterSelected)(windows_core::Interface::as_raw(self), index)
    }
    pub unsafe fn OnCounterAdded(&self, index: i32) {
        (windows_core::Interface::vtable(self).OnCounterAdded)(windows_core::Interface::as_raw(self), index)
    }
    pub unsafe fn OnCounterDeleted(&self, index: i32) {
        (windows_core::Interface::vtable(self).OnCounterDeleted)(windows_core::Interface::as_raw(self), index)
    }
    pub unsafe fn OnSampleCollected(&self) {
        (windows_core::Interface::vtable(self).OnSampleCollected)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn OnDblClick(&self, index: i32) {
        (windows_core::Interface::vtable(self).OnDblClick)(windows_core::Interface::as_raw(self), index)
    }
}
#[repr(C)]
pub struct ISystemMonitorEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnCounterSelected: unsafe extern "system" fn(*mut core::ffi::c_void, i32),
    pub OnCounterAdded: unsafe extern "system" fn(*mut core::ffi::c_void, i32),
    pub OnCounterDeleted: unsafe extern "system" fn(*mut core::ffi::c_void, i32),
    pub OnSampleCollected: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub OnDblClick: unsafe extern "system" fn(*mut core::ffi::c_void, i32),
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BufferSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetBufferSize(&self, size: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBufferSize)(windows_core::Interface::as_raw(self), size).ok()
    }
    pub unsafe fn BuffersLost(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BuffersLost)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetBuffersLost(&self, buffers: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBuffersLost)(windows_core::Interface::as_raw(self), buffers).ok()
    }
    pub unsafe fn BuffersWritten(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BuffersWritten)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetBuffersWritten(&self, buffers: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBuffersWritten)(windows_core::Interface::as_raw(self), buffers).ok()
    }
    pub unsafe fn ClockType(&self) -> windows_core::Result<ClockType> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ClockType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetClockType(&self, clock: ClockType) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetClockType)(windows_core::Interface::as_raw(self), clock).ok()
    }
    pub unsafe fn EventsLost(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EventsLost)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetEventsLost(&self, events: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetEventsLost)(windows_core::Interface::as_raw(self), events).ok()
    }
    pub unsafe fn ExtendedModes(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ExtendedModes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetExtendedModes(&self, mode: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetExtendedModes)(windows_core::Interface::as_raw(self), mode).ok()
    }
    pub unsafe fn FlushTimer(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FlushTimer)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetFlushTimer(&self, seconds: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFlushTimer)(windows_core::Interface::as_raw(self), seconds).ok()
    }
    pub unsafe fn FreeBuffers(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FreeBuffers)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetFreeBuffers(&self, buffers: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFreeBuffers)(windows_core::Interface::as_raw(self), buffers).ok()
    }
    pub unsafe fn Guid(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Guid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetGuid(&self, guid: windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetGuid)(windows_core::Interface::as_raw(self), core::mem::transmute(guid)).ok()
    }
    pub unsafe fn IsKernelTrace(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsKernelTrace)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MaximumBuffers(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MaximumBuffers)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMaximumBuffers(&self, buffers: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMaximumBuffers)(windows_core::Interface::as_raw(self), buffers).ok()
    }
    pub unsafe fn MinimumBuffers(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MinimumBuffers)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMinimumBuffers(&self, buffers: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMinimumBuffers)(windows_core::Interface::as_raw(self), buffers).ok()
    }
    pub unsafe fn NumberOfBuffers(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NumberOfBuffers)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetNumberOfBuffers(&self, buffers: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetNumberOfBuffers)(windows_core::Interface::as_raw(self), buffers).ok()
    }
    pub unsafe fn PreallocateFile(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PreallocateFile)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPreallocateFile<P0>(&self, allocate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetPreallocateFile)(windows_core::Interface::as_raw(self), allocate.param().abi()).ok()
    }
    pub unsafe fn ProcessMode(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ProcessMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetProcessMode<P0>(&self, process: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetProcessMode)(windows_core::Interface::as_raw(self), process.param().abi()).ok()
    }
    pub unsafe fn RealTimeBuffersLost(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RealTimeBuffersLost)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetRealTimeBuffersLost(&self, buffers: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetRealTimeBuffersLost)(windows_core::Interface::as_raw(self), buffers).ok()
    }
    pub unsafe fn SessionId(&self) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SessionId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSessionId(&self, id: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSessionId)(windows_core::Interface::as_raw(self), id).ok()
    }
    pub unsafe fn SessionName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SessionName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSessionName<P0>(&self, name: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetSessionName)(windows_core::Interface::as_raw(self), name.param().abi()).ok()
    }
    pub unsafe fn SessionThreadId(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SessionThreadId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSessionThreadId(&self, tid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSessionThreadId)(windows_core::Interface::as_raw(self), tid).ok()
    }
    pub unsafe fn StreamMode(&self) -> windows_core::Result<StreamMode> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StreamMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetStreamMode(&self, mode: StreamMode) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetStreamMode)(windows_core::Interface::as_raw(self), mode).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn TraceDataProviders(&self) -> windows_core::Result<ITraceDataProviderCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TraceDataProviders)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
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
    pub SessionName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetSessionName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SessionThreadId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetSessionThreadId: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub StreamMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut StreamMode) -> windows_core::HRESULT,
    pub SetStreamMode: unsafe extern "system" fn(*mut core::ffi::c_void, StreamMode) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub TraceDataProviders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    TraceDataProviders: usize,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DisplayName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDisplayName<P0>(&self, name: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDisplayName)(windows_core::Interface::as_raw(self), name.param().abi()).ok()
    }
    pub unsafe fn Guid(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Guid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetGuid(&self, guid: windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetGuid)(windows_core::Interface::as_raw(self), core::mem::transmute(guid)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Level(&self) -> windows_core::Result<IValueMap> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Level)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn KeywordsAny(&self) -> windows_core::Result<IValueMap> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).KeywordsAny)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn KeywordsAll(&self) -> windows_core::Result<IValueMap> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).KeywordsAll)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> windows_core::Result<IValueMap> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn FilterEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FilterEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetFilterEnabled<P0>(&self, filterenabled: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetFilterEnabled)(windows_core::Interface::as_raw(self), filterenabled.param().abi()).ok()
    }
    pub unsafe fn FilterType(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FilterType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetFilterType(&self, ultype: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFilterType)(windows_core::Interface::as_raw(self), ultype).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn FilterData(&self) -> windows_core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FilterData)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetFilterData(&self, pdata: *const super::Com::SAFEARRAY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFilterData)(windows_core::Interface::as_raw(self), pdata).ok()
    }
    pub unsafe fn Query<P0, P1>(&self, bstrname: P0, bstrserver: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Query)(windows_core::Interface::as_raw(self), bstrname.param().abi(), bstrserver.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Resolve<P0>(&self, pfrom: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
    {
        (windows_core::Interface::vtable(self).Resolve)(windows_core::Interface::as_raw(self), pfrom.param().abi()).ok()
    }
    pub unsafe fn SetSecurity<P0>(&self, sddl: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetSecurity)(windows_core::Interface::as_raw(self), sddl.param().abi()).ok()
    }
    pub unsafe fn GetSecurity(&self, securityinfo: u32) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSecurity)(windows_core::Interface::as_raw(self), securityinfo, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRegisteredProcesses(&self) -> windows_core::Result<IValueMap> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRegisteredProcesses)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITraceDataProvider_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Guid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SetGuid: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Level: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Level: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub KeywordsAny: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    KeywordsAny: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub KeywordsAll: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    KeywordsAll: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
    pub FilterEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetFilterEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub FilterType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetFilterType: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub FilterData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    FilterData: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetFilterData: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetFilterData: usize,
    pub Query: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Resolve: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Resolve: usize,
    pub SetSecurity: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetSecurity: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRegisteredProcesses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRegisteredProcesses: usize,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item<P0>(&self, index: P0) -> windows_core::Result<ITraceDataProvider>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<P0>(&self, pprovider: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITraceDataProvider>,
    {
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), pprovider.param().abi()).ok()
    }
    pub unsafe fn Remove<P0>(&self, vprovider: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), vprovider.param().abi()).ok()
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddRange<P0>(&self, providers: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITraceDataProviderCollection>,
    {
        (windows_core::Interface::vtable(self).AddRange)(windows_core::Interface::as_raw(self), providers.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateTraceDataProvider(&self) -> windows_core::Result<ITraceDataProvider> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTraceDataProvider)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetTraceDataProviders<P0>(&self, server: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).GetTraceDataProviders)(windows_core::Interface::as_raw(self), server.param().abi()).ok()
    }
    pub unsafe fn GetTraceDataProvidersByProcess<P0>(&self, server: P0, pid: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).GetTraceDataProvidersByProcess)(windows_core::Interface::as_raw(self), server.param().abi(), pid).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITraceDataProviderCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddRange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateTraceDataProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateTraceDataProvider: usize,
    pub GetTraceDataProviders: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetTraceDataProvidersByProcess: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, u32) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item<P0>(&self, index: P0) -> windows_core::Result<IValueMapItem>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDescription<P0>(&self, description: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), description.param().abi()).ok()
    }
    pub unsafe fn Value(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetValue<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn ValueMapType(&self) -> windows_core::Result<ValueMapType> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ValueMapType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetValueMapType(&self, r#type: ValueMapType) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetValueMapType)(windows_core::Interface::as_raw(self), r#type).ok()
    }
    pub unsafe fn Add<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn Remove<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddRange<P0>(&self, map: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IValueMap>,
    {
        (windows_core::Interface::vtable(self).AddRange)(windows_core::Interface::as_raw(self), map.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateValueMapItem(&self) -> windows_core::Result<IValueMapItem> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateValueMapItem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IValueMap_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub ValueMapType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ValueMapType) -> windows_core::HRESULT,
    pub SetValueMapType: unsafe extern "system" fn(*mut core::ffi::c_void, ValueMapType) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddRange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateValueMapItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateValueMapItem: usize,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDescription<P0>(&self, description: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), description.param().abi()).ok()
    }
    pub unsafe fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetEnabled<P0>(&self, enabled: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetEnabled)(windows_core::Interface::as_raw(self), enabled.param().abi()).ok()
    }
    pub unsafe fn Key(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Key)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetKey<P0>(&self, key: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetKey)(windows_core::Interface::as_raw(self), key.param().abi()).ok()
    }
    pub unsafe fn Value(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetValue<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn ValueMapType(&self) -> windows_core::Result<ValueMapType> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ValueMapType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetValueMapType(&self, r#type: ValueMapType) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetValueMapType)(windows_core::Interface::as_raw(self), r#type).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IValueMapItem_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Key: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetKey: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub ValueMapType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ValueMapType) -> windows_core::HRESULT,
    pub SetValueMapType: unsafe extern "system" fn(*mut core::ffi::c_void, ValueMapType) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(_ICounterItemUnion, _ICounterItemUnion_Vtbl, 0xde1a6b74_9182_4c41_8e2c_24c2cd30ee83);
impl core::ops::Deref for _ICounterItemUnion {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(_ICounterItemUnion, windows_core::IUnknown);
impl _ICounterItemUnion {
    pub unsafe fn Value(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetColor(&self, color: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetColor)(windows_core::Interface::as_raw(self), color).ok()
    }
    pub unsafe fn Color(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Color)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetWidth(&self, iwidth: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetWidth)(windows_core::Interface::as_raw(self), iwidth).ok()
    }
    pub unsafe fn Width(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Width)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetLineStyle(&self, ilinestyle: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLineStyle)(windows_core::Interface::as_raw(self), ilinestyle).ok()
    }
    pub unsafe fn LineStyle(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LineStyle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetScaleFactor(&self, iscale: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetScaleFactor)(windows_core::Interface::as_raw(self), iscale).ok()
    }
    pub unsafe fn ScaleFactor(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ScaleFactor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Path(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Path)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetValue(&self, value: *mut f64, status: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), value, status).ok()
    }
    pub unsafe fn GetStatistics(&self, max: *mut f64, min: *mut f64, avg: *mut f64, status: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetStatistics)(windows_core::Interface::as_raw(self), max, min, avg, status).ok()
    }
    pub unsafe fn SetSelected<P0>(&self, bstate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetSelected)(windows_core::Interface::as_raw(self), bstate.param().abi()).ok()
    }
    pub unsafe fn Selected(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Selected)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetVisible<P0>(&self, bstate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetVisible)(windows_core::Interface::as_raw(self), bstate.param().abi()).ok()
    }
    pub unsafe fn Visible(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Visible)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetDataAt(&self, iindex: i32, iwhich: SysmonDataType) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDataAt)(windows_core::Interface::as_raw(self), iindex, iwhich, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
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
    pub Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64, *mut i32) -> windows_core::HRESULT,
    pub GetStatistics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64, *mut f64, *mut f64, *mut i32) -> windows_core::HRESULT,
    pub SetSelected: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Selected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetVisible: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Visible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub GetDataAt: unsafe extern "system" fn(*mut core::ffi::c_void, i32, SysmonDataType, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(_ISystemMonitorUnion, _ISystemMonitorUnion_Vtbl, 0xc8a77338_265f_4de5_aa25_c7da1ce5a8f4);
impl core::ops::Deref for _ISystemMonitorUnion {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(_ISystemMonitorUnion, windows_core::IUnknown);
impl _ISystemMonitorUnion {
    pub unsafe fn Appearance(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Appearance)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAppearance(&self, iappearance: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAppearance)(windows_core::Interface::as_raw(self), iappearance).ok()
    }
    pub unsafe fn BackColor(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BackColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetBackColor(&self, color: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBackColor)(windows_core::Interface::as_raw(self), color).ok()
    }
    pub unsafe fn BorderStyle(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BorderStyle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetBorderStyle(&self, iborderstyle: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBorderStyle)(windows_core::Interface::as_raw(self), iborderstyle).ok()
    }
    pub unsafe fn ForeColor(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ForeColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetForeColor(&self, color: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetForeColor)(windows_core::Interface::as_raw(self), color).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Font(&self) -> windows_core::Result<super::Ole::IFontDisp> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Font)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn putref_Font<P0>(&self, pfont: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Ole::IFontDisp>,
    {
        (windows_core::Interface::vtable(self).putref_Font)(windows_core::Interface::as_raw(self), pfont.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Counters(&self) -> windows_core::Result<ICounters> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Counters)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetShowVerticalGrid<P0>(&self, bstate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetShowVerticalGrid)(windows_core::Interface::as_raw(self), bstate.param().abi()).ok()
    }
    pub unsafe fn ShowVerticalGrid(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ShowVerticalGrid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetShowHorizontalGrid<P0>(&self, bstate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetShowHorizontalGrid)(windows_core::Interface::as_raw(self), bstate.param().abi()).ok()
    }
    pub unsafe fn ShowHorizontalGrid(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ShowHorizontalGrid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetShowLegend<P0>(&self, bstate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetShowLegend)(windows_core::Interface::as_raw(self), bstate.param().abi()).ok()
    }
    pub unsafe fn ShowLegend(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ShowLegend)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetShowScaleLabels<P0>(&self, bstate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetShowScaleLabels)(windows_core::Interface::as_raw(self), bstate.param().abi()).ok()
    }
    pub unsafe fn ShowScaleLabels(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ShowScaleLabels)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetShowValueBar<P0>(&self, bstate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetShowValueBar)(windows_core::Interface::as_raw(self), bstate.param().abi()).ok()
    }
    pub unsafe fn ShowValueBar(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ShowValueBar)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMaximumScale(&self, ivalue: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMaximumScale)(windows_core::Interface::as_raw(self), ivalue).ok()
    }
    pub unsafe fn MaximumScale(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MaximumScale)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMinimumScale(&self, ivalue: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMinimumScale)(windows_core::Interface::as_raw(self), ivalue).ok()
    }
    pub unsafe fn MinimumScale(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MinimumScale)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetUpdateInterval(&self, fvalue: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetUpdateInterval)(windows_core::Interface::as_raw(self), fvalue).ok()
    }
    pub unsafe fn UpdateInterval(&self) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UpdateInterval)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDisplayType(&self, edisplaytype: DisplayTypeConstants) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDisplayType)(windows_core::Interface::as_raw(self), edisplaytype).ok()
    }
    pub unsafe fn DisplayType(&self) -> windows_core::Result<DisplayTypeConstants> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DisplayType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetManualUpdate<P0>(&self, bstate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetManualUpdate)(windows_core::Interface::as_raw(self), bstate.param().abi()).ok()
    }
    pub unsafe fn ManualUpdate(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ManualUpdate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetGraphTitle<P0>(&self, bstitle: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetGraphTitle)(windows_core::Interface::as_raw(self), bstitle.param().abi()).ok()
    }
    pub unsafe fn GraphTitle(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GraphTitle)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetYAxisLabel<P0>(&self, bstitle: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetYAxisLabel)(windows_core::Interface::as_raw(self), bstitle.param().abi()).ok()
    }
    pub unsafe fn YAxisLabel(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).YAxisLabel)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CollectSample(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CollectSample)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn UpdateGraph(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UpdateGraph)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BrowseCounters(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).BrowseCounters)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DisplayProperties(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DisplayProperties)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Counter(&self, iindex: i32) -> windows_core::Result<ICounterItem> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Counter)(windows_core::Interface::as_raw(self), iindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AddCounter<P0>(&self, bspath: P0) -> windows_core::Result<ICounterItem>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AddCounter)(windows_core::Interface::as_raw(self), bspath.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DeleteCounter<P0>(&self, pctr: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICounterItem>,
    {
        (windows_core::Interface::vtable(self).DeleteCounter)(windows_core::Interface::as_raw(self), pctr.param().abi()).ok()
    }
    pub unsafe fn BackColorCtl(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BackColorCtl)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetBackColorCtl(&self, color: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBackColorCtl)(windows_core::Interface::as_raw(self), color).ok()
    }
    pub unsafe fn SetLogFileName<P0>(&self, bsfilename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetLogFileName)(windows_core::Interface::as_raw(self), bsfilename.param().abi()).ok()
    }
    pub unsafe fn LogFileName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LogFileName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetLogViewStart(&self, starttime: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLogViewStart)(windows_core::Interface::as_raw(self), starttime).ok()
    }
    pub unsafe fn LogViewStart(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LogViewStart)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetLogViewStop(&self, stoptime: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLogViewStop)(windows_core::Interface::as_raw(self), stoptime).ok()
    }
    pub unsafe fn LogViewStop(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LogViewStop)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GridColor(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GridColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetGridColor(&self, color: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetGridColor)(windows_core::Interface::as_raw(self), color).ok()
    }
    pub unsafe fn TimeBarColor(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TimeBarColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetTimeBarColor(&self, color: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTimeBarColor)(windows_core::Interface::as_raw(self), color).ok()
    }
    pub unsafe fn Highlight(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Highlight)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetHighlight<P0>(&self, bstate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetHighlight)(windows_core::Interface::as_raw(self), bstate.param().abi()).ok()
    }
    pub unsafe fn ShowToolbar(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ShowToolbar)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetShowToolbar<P0>(&self, bstate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetShowToolbar)(windows_core::Interface::as_raw(self), bstate.param().abi()).ok()
    }
    pub unsafe fn Paste(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Paste)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Copy(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Copy)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetReadOnly<P0>(&self, bstate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetReadOnly)(windows_core::Interface::as_raw(self), bstate.param().abi()).ok()
    }
    pub unsafe fn ReadOnly(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReadOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetReportValueType(&self, ereportvaluetype: ReportValueTypeConstants) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetReportValueType)(windows_core::Interface::as_raw(self), ereportvaluetype).ok()
    }
    pub unsafe fn ReportValueType(&self) -> windows_core::Result<ReportValueTypeConstants> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReportValueType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMonitorDuplicateInstances<P0>(&self, bstate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetMonitorDuplicateInstances)(windows_core::Interface::as_raw(self), bstate.param().abi()).ok()
    }
    pub unsafe fn MonitorDuplicateInstances(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MonitorDuplicateInstances)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDisplayFilter(&self, ivalue: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDisplayFilter)(windows_core::Interface::as_raw(self), ivalue).ok()
    }
    pub unsafe fn DisplayFilter(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DisplayFilter)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LogFiles(&self) -> windows_core::Result<ILogFiles> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LogFiles)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDataSourceType(&self, edatasourcetype: DataSourceTypeConstants) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDataSourceType)(windows_core::Interface::as_raw(self), edatasourcetype).ok()
    }
    pub unsafe fn DataSourceType(&self) -> windows_core::Result<DataSourceTypeConstants> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DataSourceType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSqlDsnName<P0>(&self, bssqldsnname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetSqlDsnName)(windows_core::Interface::as_raw(self), bssqldsnname.param().abi()).ok()
    }
    pub unsafe fn SqlDsnName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SqlDsnName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSqlLogSetName<P0>(&self, bssqllogsetname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetSqlLogSetName)(windows_core::Interface::as_raw(self), bssqllogsetname.param().abi()).ok()
    }
    pub unsafe fn SqlLogSetName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SqlLogSetName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetEnableDigitGrouping<P0>(&self, bstate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetEnableDigitGrouping)(windows_core::Interface::as_raw(self), bstate.param().abi()).ok()
    }
    pub unsafe fn EnableDigitGrouping(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnableDigitGrouping)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetEnableToolTips<P0>(&self, bstate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetEnableToolTips)(windows_core::Interface::as_raw(self), bstate.param().abi()).ok()
    }
    pub unsafe fn EnableToolTips(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnableToolTips)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetShowTimeAxisLabels<P0>(&self, bstate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetShowTimeAxisLabels)(windows_core::Interface::as_raw(self), bstate.param().abi()).ok()
    }
    pub unsafe fn ShowTimeAxisLabels(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ShowTimeAxisLabels)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetChartScroll<P0>(&self, bscroll: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetChartScroll)(windows_core::Interface::as_raw(self), bscroll.param().abi()).ok()
    }
    pub unsafe fn ChartScroll(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ChartScroll)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDataPointCount(&self, inewcount: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDataPointCount)(windows_core::Interface::as_raw(self), inewcount).ok()
    }
    pub unsafe fn DataPointCount(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DataPointCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ScaleToFit<P0>(&self, bselectedcountersonly: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).ScaleToFit)(windows_core::Interface::as_raw(self), bselectedcountersonly.param().abi()).ok()
    }
    pub unsafe fn SaveAs<P0>(&self, bstrfilename: P0, esysmonfiletype: SysmonFileType) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SaveAs)(windows_core::Interface::as_raw(self), bstrfilename.param().abi(), esysmonfiletype).ok()
    }
    pub unsafe fn Relog<P0>(&self, bstrfilename: P0, esysmonfiletype: SysmonFileType, ifilter: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Relog)(windows_core::Interface::as_raw(self), bstrfilename.param().abi(), esysmonfiletype, ifilter).ok()
    }
    pub unsafe fn ClearData(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ClearData)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn LogSourceStartTime(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LogSourceStartTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn LogSourceStopTime(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LogSourceStopTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetLogViewRange(&self, starttime: f64, stoptime: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLogViewRange)(windows_core::Interface::as_raw(self), starttime, stoptime).ok()
    }
    pub unsafe fn GetLogViewRange(&self, starttime: *mut f64, stoptime: *mut f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLogViewRange)(windows_core::Interface::as_raw(self), starttime, stoptime).ok()
    }
    pub unsafe fn BatchingLock<P0>(&self, flock: P0, ebatchreason: SysmonBatchReason) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).BatchingLock)(windows_core::Interface::as_raw(self), flock.param().abi(), ebatchreason).ok()
    }
    pub unsafe fn LoadSettings<P0>(&self, bstrsettingfilename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).LoadSettings)(windows_core::Interface::as_raw(self), bstrsettingfilename.param().abi()).ok()
    }
}
#[repr(C)]
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
    pub SetGraphTitle: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GraphTitle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetYAxisLabel: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub YAxisLabel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CollectSample: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UpdateGraph: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BrowseCounters: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisplayProperties: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Counter: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddCounter: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteCounter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BackColorCtl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetBackColorCtl: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetLogFileName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub LogFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
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
    pub SetSqlDsnName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SqlDsnName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetSqlLogSetName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SqlLogSetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
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
    pub SaveAs: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, SysmonFileType) -> windows_core::HRESULT,
    pub Relog: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, SysmonFileType, i32) -> windows_core::HRESULT,
    pub ClearData: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LogSourceStartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub LogSourceStopTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetLogViewRange: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64) -> windows_core::HRESULT,
    pub GetLogViewRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64, *mut f64) -> windows_core::HRESULT,
    pub BatchingLock: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, SysmonBatchReason) -> windows_core::HRESULT,
    pub LoadSettings: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
pub const DATA_SOURCE_REGISTRY: REAL_TIME_DATA_SOURCE_ID_FLAGS = REAL_TIME_DATA_SOURCE_ID_FLAGS(1u32);
pub const DATA_SOURCE_WBEM: REAL_TIME_DATA_SOURCE_ID_FLAGS = REAL_TIME_DATA_SOURCE_ID_FLAGS(4u32);
pub const DIID_DICounterItem: windows_core::GUID = windows_core::GUID::from_u128(0xc08c4ff2_0e2e_11cf_942c_008029004347);
pub const DIID_DILogFileItem: windows_core::GUID = windows_core::GUID::from_u128(0x8d093ffc_f777_4917_82d1_833fbc54c58f);
pub const DIID_DISystemMonitor: windows_core::GUID = windows_core::GUID::from_u128(0x13d73d81_c32e_11cf_9398_00aa00a3ddea);
pub const DIID_DISystemMonitorEvents: windows_core::GUID = windows_core::GUID::from_u128(0x84979930_4ab3_11cf_943a_008029004347);
pub const DIID_DISystemMonitorInternal: windows_core::GUID = windows_core::GUID::from_u128(0x194eb242_c32c_11cf_9398_00aa00a3ddea);
pub const H_WBEM_DATASOURCE: i32 = -1i32;
pub const LIBID_SystemMonitor: windows_core::GUID = windows_core::GUID::from_u128(0x1b773e42_2509_11cf_942f_008029004347);
pub const MAX_COUNTER_PATH: u32 = 256u32;
pub const MAX_PERF_OBJECTS_IN_QUERY_FUNCTION: i32 = 64i32;
pub const PDH_ACCESS_DENIED: u32 = 3221228507u32;
pub const PDH_ASYNC_QUERY_TIMEOUT: u32 = 2147485659u32;
pub const PDH_BINARY_LOG_CORRUPT: u32 = 3221228535u32;
pub const PDH_CALC_NEGATIVE_DENOMINATOR: u32 = 2147485654u32;
pub const PDH_CALC_NEGATIVE_TIMEBASE: u32 = 2147485655u32;
pub const PDH_CALC_NEGATIVE_VALUE: u32 = 2147485656u32;
pub const PDH_CANNOT_CONNECT_MACHINE: u32 = 3221228483u32;
pub const PDH_CANNOT_CONNECT_WMI_SERVER: u32 = 3221228520u32;
pub const PDH_CANNOT_READ_NAME_STRINGS: u32 = 3221228488u32;
pub const PDH_CANNOT_SET_DEFAULT_REALTIME_DATASOURCE: u32 = 2147485660u32;
pub const PDH_COUNTER_ALREADY_IN_QUERY: u32 = 3221228534u32;
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
pub const PDH_DATA_SOURCE_IS_LOG_FILE: u32 = 3221228494u32;
pub const PDH_DATA_SOURCE_IS_REAL_TIME: u32 = 3221228495u32;
pub const PDH_DIALOG_CANCELLED: u32 = 2147485657u32;
pub const PDH_END_OF_LOG_FILE: u32 = 2147485658u32;
pub const PDH_ENTRY_NOT_IN_LOG_FILE: u32 = 3221228493u32;
pub const PDH_FILE_ALREADY_EXISTS: u32 = 3221228498u32;
pub const PDH_FILE_NOT_FOUND: u32 = 3221228497u32;
pub const PDH_FLAGS_FILE_BROWSER_ONLY: PDH_SELECT_DATA_SOURCE_FLAGS = PDH_SELECT_DATA_SOURCE_FLAGS(1u32);
pub const PDH_FLAGS_NONE: PDH_SELECT_DATA_SOURCE_FLAGS = PDH_SELECT_DATA_SOURCE_FLAGS(0u32);
pub const PDH_FMT_DOUBLE: PDH_FMT = PDH_FMT(512u32);
pub const PDH_FMT_LARGE: PDH_FMT = PDH_FMT(1024u32);
pub const PDH_FMT_LONG: PDH_FMT = PDH_FMT(256u32);
pub const PDH_FUNCTION_NOT_FOUND: u32 = 3221228478u32;
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
pub const PDH_LOGSVC_NOT_OPENED: u32 = 3221228505u32;
pub const PDH_LOGSVC_QUERY_NOT_FOUND: u32 = 3221228504u32;
pub const PDH_LOG_FILE_CREATE_ERROR: u32 = 3221228489u32;
pub const PDH_LOG_FILE_OPEN_ERROR: u32 = 3221228490u32;
pub const PDH_LOG_FILE_TOO_SMALL: u32 = 3221228508u32;
pub const PDH_LOG_READ_ACCESS: PDH_LOG = PDH_LOG(65536u32);
pub const PDH_LOG_SAMPLE_TOO_SMALL: u32 = 3221228536u32;
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
pub const PDH_REFRESHCOUNTERS: u32 = 4u32;
pub const PDH_RETRY: u32 = 2147485652u32;
pub const PDH_SQL_ALLOCCON_FAILED: u32 = 3221228513u32;
pub const PDH_SQL_ALLOC_FAILED: u32 = 3221228512u32;
pub const PDH_SQL_ALTER_DETAIL_FAILED: u32 = 3221228541u32;
pub const PDH_SQL_BIND_FAILED: u32 = 3221228519u32;
pub const PDH_SQL_CONNECT_FAILED: u32 = 3221228518u32;
pub const PDH_SQL_EXEC_DIRECT_FAILED: u32 = 3221228514u32;
pub const PDH_SQL_FETCH_FAILED: u32 = 3221228515u32;
pub const PDH_SQL_MORE_RESULTS_FAILED: u32 = 3221228517u32;
pub const PDH_SQL_ROWCOUNT_FAILED: u32 = 3221228516u32;
pub const PDH_STRING_NOT_FOUND: u32 = 3221228500u32;
pub const PDH_UNABLE_MAP_NAME_FILES: u32 = 2147486677u32;
pub const PDH_UNABLE_READ_LOG_HEADER: u32 = 3221228496u32;
pub const PDH_UNKNOWN_LOGSVC_COMMAND: u32 = 3221228503u32;
pub const PDH_UNKNOWN_LOG_FORMAT: u32 = 3221228502u32;
pub const PDH_UNMATCHED_APPEND_COUNTER: u32 = 3221228540u32;
pub const PDH_VERSION: PDH_DLL_VERSION = PDH_DLL_VERSION(1283u32);
pub const PDH_WBEM_ERROR: u32 = 3221228506u32;
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
pub const PERF_COUNTERSET_MULTI_INSTANCES: u32 = 2u32;
pub const PERF_COUNTERSET_SINGLE_AGGREGATE: u32 = 4u32;
pub const PERF_COUNTERSET_SINGLE_INSTANCE: u32 = 0u32;
pub const PERF_COUNTER_BASE: u32 = 196608u32;
pub const PERF_COUNTER_ELAPSED: u32 = 262144u32;
pub const PERF_COUNTER_FRACTION: u32 = 131072u32;
pub const PERF_COUNTER_HISTOGRAM: u32 = 393216u32;
pub const PERF_COUNTER_HISTOGRAM_TYPE: u32 = 2147483648u32;
pub const PERF_COUNTER_PRECISION: u32 = 458752u32;
pub const PERF_COUNTER_QUEUELEN: u32 = 327680u32;
pub const PERF_COUNTER_RATE: u32 = 65536u32;
pub const PERF_COUNTER_VALUE: u32 = 0u32;
pub const PERF_DATA_REVISION: u32 = 1u32;
pub const PERF_DATA_VERSION: u32 = 1u32;
pub const PERF_DELTA_BASE: u32 = 8388608u32;
pub const PERF_DELTA_COUNTER: u32 = 4194304u32;
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
pub const PERF_INVERSE_COUNTER: u32 = 16777216u32;
pub const PERF_MAX_INSTANCE_NAME: u32 = 1024u32;
pub const PERF_METADATA_MULTIPLE_INSTANCES: i32 = -2i32;
pub const PERF_METADATA_NO_INSTANCES: i32 = -3i32;
pub const PERF_MULTIPLE_COUNTERS: PerfCounterDataType = PerfCounterDataType(2i32);
pub const PERF_MULTIPLE_INSTANCES: PerfCounterDataType = PerfCounterDataType(4i32);
pub const PERF_MULTI_COUNTER: u32 = 33554432u32;
pub const PERF_NO_INSTANCES: i32 = -1i32;
pub const PERF_NO_UNIQUE_ID: i32 = -1i32;
pub const PERF_NUMBER_DECIMAL: u32 = 65536u32;
pub const PERF_NUMBER_DEC_1000: u32 = 131072u32;
pub const PERF_NUMBER_HEX: u32 = 0u32;
pub const PERF_OBJECT_TIMER: u32 = 2097152u32;
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
pub const PLA_CAPABILITY_AUTOLOGGER: u32 = 32u32;
pub const PLA_CAPABILITY_LEGACY_SESSION: u32 = 8u32;
pub const PLA_CAPABILITY_LEGACY_SVC: u32 = 16u32;
pub const PLA_CAPABILITY_LOCAL: u32 = 268435456u32;
pub const PLA_CAPABILITY_V1_SESSION: u32 = 2u32;
pub const PLA_CAPABILITY_V1_SVC: u32 = 1u32;
pub const PLA_CAPABILITY_V1_SYSTEM: u32 = 4u32;
pub const S_PDH: windows_core::GUID = windows_core::GUID::from_u128(0x04d66358_c4a1_419b_8023_23b73902de2c);
pub const WINPERF_LOG_DEBUG: u32 = 2u32;
pub const WINPERF_LOG_NONE: u32 = 0u32;
pub const WINPERF_LOG_USER: u32 = 1u32;
pub const WINPERF_LOG_VERBOSE: u32 = 3u32;
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
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AutoPathFormat(pub i32);
impl windows_core::TypeKind for AutoPathFormat {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AutoPathFormat {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AutoPathFormat").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ClockType(pub i32);
impl windows_core::TypeKind for ClockType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ClockType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ClockType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CommitMode(pub i32);
impl windows_core::TypeKind for CommitMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CommitMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CommitMode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DataCollectorSetStatus(pub i32);
impl windows_core::TypeKind for DataCollectorSetStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DataCollectorSetStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DataCollectorSetStatus").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DataCollectorType(pub i32);
impl windows_core::TypeKind for DataCollectorType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DataCollectorType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DataCollectorType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DataManagerSteps(pub i32);
impl windows_core::TypeKind for DataManagerSteps {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DataManagerSteps {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DataManagerSteps").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DataSourceTypeConstants(pub i32);
impl windows_core::TypeKind for DataSourceTypeConstants {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DataSourceTypeConstants {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DataSourceTypeConstants").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DisplayTypeConstants(pub i32);
impl windows_core::TypeKind for DisplayTypeConstants {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DisplayTypeConstants {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DisplayTypeConstants").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FileFormat(pub i32);
impl windows_core::TypeKind for FileFormat {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FileFormat {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FileFormat").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FolderActionSteps(pub i32);
impl windows_core::TypeKind for FolderActionSteps {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FolderActionSteps {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FolderActionSteps").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PDH_DLL_VERSION(pub u32);
impl windows_core::TypeKind for PDH_DLL_VERSION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PDH_DLL_VERSION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PDH_DLL_VERSION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PDH_FMT(pub u32);
impl windows_core::TypeKind for PDH_FMT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PDH_FMT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PDH_FMT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PDH_LOG(pub u32);
impl windows_core::TypeKind for PDH_LOG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PDH_LOG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PDH_LOG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PDH_LOG_TYPE(pub u32);
impl windows_core::TypeKind for PDH_LOG_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PDH_LOG_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PDH_LOG_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PDH_PATH_FLAGS(pub u32);
impl windows_core::TypeKind for PDH_PATH_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PDH_PATH_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PDH_PATH_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PDH_SELECT_DATA_SOURCE_FLAGS(pub u32);
impl windows_core::TypeKind for PDH_SELECT_DATA_SOURCE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PDH_SELECT_DATA_SOURCE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PDH_SELECT_DATA_SOURCE_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PERF_COUNTER_AGGREGATE_FUNC(pub u32);
impl windows_core::TypeKind for PERF_COUNTER_AGGREGATE_FUNC {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PERF_COUNTER_AGGREGATE_FUNC {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PERF_COUNTER_AGGREGATE_FUNC").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PERF_DETAIL(pub u32);
impl windows_core::TypeKind for PERF_DETAIL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PERF_DETAIL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PERF_DETAIL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PerfCounterDataType(pub i32);
impl windows_core::TypeKind for PerfCounterDataType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PerfCounterDataType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PerfCounterDataType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PerfRegInfoType(pub i32);
impl windows_core::TypeKind for PerfRegInfoType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PerfRegInfoType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PerfRegInfoType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct REAL_TIME_DATA_SOURCE_ID_FLAGS(pub u32);
impl windows_core::TypeKind for REAL_TIME_DATA_SOURCE_ID_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for REAL_TIME_DATA_SOURCE_ID_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("REAL_TIME_DATA_SOURCE_ID_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ReportValueTypeConstants(pub i32);
impl windows_core::TypeKind for ReportValueTypeConstants {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ReportValueTypeConstants {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ReportValueTypeConstants").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ResourcePolicy(pub i32);
impl windows_core::TypeKind for ResourcePolicy {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ResourcePolicy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ResourcePolicy").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct StreamMode(pub i32);
impl windows_core::TypeKind for StreamMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for StreamMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("StreamMode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SysmonBatchReason(pub i32);
impl windows_core::TypeKind for SysmonBatchReason {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SysmonBatchReason {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SysmonBatchReason").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SysmonDataType(pub i32);
impl windows_core::TypeKind for SysmonDataType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SysmonDataType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SysmonDataType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SysmonFileType(pub i32);
impl windows_core::TypeKind for SysmonFileType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SysmonFileType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SysmonFileType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ValueMapType(pub i32);
impl windows_core::TypeKind for ValueMapType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ValueMapType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ValueMapType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WeekDays(pub i32);
impl windows_core::TypeKind for WeekDays {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WeekDays {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WeekDays").field(&self.0).finish()
    }
}
pub const AppearPropPage: windows_core::GUID = windows_core::GUID::from_u128(0xe49741e9_93a8_4ab1_8e96_bf4482282e9c);
pub const BootTraceSession: windows_core::GUID = windows_core::GUID::from_u128(0x03837538_098b_11d8_9414_505054503030);
pub const BootTraceSessionCollection: windows_core::GUID = windows_core::GUID::from_u128(0x03837539_098b_11d8_9414_505054503030);
pub const CounterItem: windows_core::GUID = windows_core::GUID::from_u128(0xc4d2d8e0_d1dd_11ce_940f_008029004348);
pub const CounterItem2: windows_core::GUID = windows_core::GUID::from_u128(0x43196c62_c31f_4ce3_a02e_79efe0f6a525);
pub const CounterPropPage: windows_core::GUID = windows_core::GUID::from_u128(0xcf948561_ede8_11ce_941e_008029004347);
pub const Counters: windows_core::GUID = windows_core::GUID::from_u128(0xb2b066d2_2aac_11cf_942f_008029004347);
pub const DataCollectorSet: windows_core::GUID = windows_core::GUID::from_u128(0x03837521_098b_11d8_9414_505054503030);
pub const DataCollectorSetCollection: windows_core::GUID = windows_core::GUID::from_u128(0x03837525_098b_11d8_9414_505054503030);
pub const GeneralPropPage: windows_core::GUID = windows_core::GUID::from_u128(0xc3e5d3d2_1a03_11cf_942d_008029004347);
pub const GraphPropPage: windows_core::GUID = windows_core::GUID::from_u128(0xc3e5d3d3_1a03_11cf_942d_008029004347);
pub const LegacyDataCollectorSet: windows_core::GUID = windows_core::GUID::from_u128(0x03837526_098b_11d8_9414_505054503030);
pub const LegacyDataCollectorSetCollection: windows_core::GUID = windows_core::GUID::from_u128(0x03837527_098b_11d8_9414_505054503030);
pub const LegacyTraceSession: windows_core::GUID = windows_core::GUID::from_u128(0x03837528_098b_11d8_9414_505054503030);
pub const LegacyTraceSessionCollection: windows_core::GUID = windows_core::GUID::from_u128(0x03837529_098b_11d8_9414_505054503030);
pub const LogFileItem: windows_core::GUID = windows_core::GUID::from_u128(0x16ec5be8_df93_4237_94e4_9ee918111d71);
pub const LogFiles: windows_core::GUID = windows_core::GUID::from_u128(0x2735d9fd_f6b9_4f19_a5d9_e2d068584bc5);
#[repr(C)]
#[derive(Clone, Copy, Debug)]
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
impl windows_core::TypeKind for PDH_BROWSE_DLG_CONFIG_A {
    type TypeKind = windows_core::CopyType;
}
impl Default for PDH_BROWSE_DLG_CONFIG_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PDH_BROWSE_DLG_CONFIG_HA {
    pub _bitfield: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub hDataSource: isize,
    pub szReturnPathBuffer: windows_core::PSTR,
    pub cchReturnPathLength: u32,
    pub pCallBack: CounterPathCallBack,
    pub dwCallBackArg: usize,
    pub CallBackStatus: i32,
    pub dwDefaultDetailLevel: PERF_DETAIL,
    pub szDialogBoxCaption: windows_core::PSTR,
}
impl windows_core::TypeKind for PDH_BROWSE_DLG_CONFIG_HA {
    type TypeKind = windows_core::CopyType;
}
impl Default for PDH_BROWSE_DLG_CONFIG_HA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PDH_BROWSE_DLG_CONFIG_HW {
    pub _bitfield: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub hDataSource: isize,
    pub szReturnPathBuffer: windows_core::PWSTR,
    pub cchReturnPathLength: u32,
    pub pCallBack: CounterPathCallBack,
    pub dwCallBackArg: usize,
    pub CallBackStatus: i32,
    pub dwDefaultDetailLevel: PERF_DETAIL,
    pub szDialogBoxCaption: windows_core::PWSTR,
}
impl windows_core::TypeKind for PDH_BROWSE_DLG_CONFIG_HW {
    type TypeKind = windows_core::CopyType;
}
impl Default for PDH_BROWSE_DLG_CONFIG_HW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
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
impl windows_core::TypeKind for PDH_BROWSE_DLG_CONFIG_W {
    type TypeKind = windows_core::CopyType;
}
impl Default for PDH_BROWSE_DLG_CONFIG_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
impl windows_core::TypeKind for PDH_COUNTER_INFO_A {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for PDH_COUNTER_INFO_A_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for PDH_COUNTER_INFO_A_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PDH_COUNTER_INFO_A_0_0 {
    pub szMachineName: windows_core::PSTR,
    pub szObjectName: windows_core::PSTR,
    pub szInstanceName: windows_core::PSTR,
    pub szParentInstance: windows_core::PSTR,
    pub dwInstanceIndex: u32,
    pub szCounterName: windows_core::PSTR,
}
impl windows_core::TypeKind for PDH_COUNTER_INFO_A_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for PDH_COUNTER_INFO_A_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
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
impl windows_core::TypeKind for PDH_COUNTER_INFO_W {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for PDH_COUNTER_INFO_W_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for PDH_COUNTER_INFO_W_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PDH_COUNTER_INFO_W_0_0 {
    pub szMachineName: windows_core::PWSTR,
    pub szObjectName: windows_core::PWSTR,
    pub szInstanceName: windows_core::PWSTR,
    pub szParentInstance: windows_core::PWSTR,
    pub dwInstanceIndex: u32,
    pub szCounterName: windows_core::PWSTR,
}
impl windows_core::TypeKind for PDH_COUNTER_INFO_W_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for PDH_COUNTER_INFO_W_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PDH_COUNTER_PATH_ELEMENTS_A {
    pub szMachineName: windows_core::PSTR,
    pub szObjectName: windows_core::PSTR,
    pub szInstanceName: windows_core::PSTR,
    pub szParentInstance: windows_core::PSTR,
    pub dwInstanceIndex: u32,
    pub szCounterName: windows_core::PSTR,
}
impl windows_core::TypeKind for PDH_COUNTER_PATH_ELEMENTS_A {
    type TypeKind = windows_core::CopyType;
}
impl Default for PDH_COUNTER_PATH_ELEMENTS_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PDH_COUNTER_PATH_ELEMENTS_W {
    pub szMachineName: windows_core::PWSTR,
    pub szObjectName: windows_core::PWSTR,
    pub szInstanceName: windows_core::PWSTR,
    pub szParentInstance: windows_core::PWSTR,
    pub dwInstanceIndex: u32,
    pub szCounterName: windows_core::PWSTR,
}
impl windows_core::TypeKind for PDH_COUNTER_PATH_ELEMENTS_W {
    type TypeKind = windows_core::CopyType;
}
impl Default for PDH_COUNTER_PATH_ELEMENTS_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PDH_DATA_ITEM_PATH_ELEMENTS_A {
    pub szMachineName: windows_core::PSTR,
    pub ObjectGUID: windows_core::GUID,
    pub dwItemId: u32,
    pub szInstanceName: windows_core::PSTR,
}
impl windows_core::TypeKind for PDH_DATA_ITEM_PATH_ELEMENTS_A {
    type TypeKind = windows_core::CopyType;
}
impl Default for PDH_DATA_ITEM_PATH_ELEMENTS_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PDH_DATA_ITEM_PATH_ELEMENTS_W {
    pub szMachineName: windows_core::PWSTR,
    pub ObjectGUID: windows_core::GUID,
    pub dwItemId: u32,
    pub szInstanceName: windows_core::PWSTR,
}
impl windows_core::TypeKind for PDH_DATA_ITEM_PATH_ELEMENTS_W {
    type TypeKind = windows_core::CopyType;
}
impl Default for PDH_DATA_ITEM_PATH_ELEMENTS_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PDH_FMT_COUNTERVALUE {
    pub CStatus: u32,
    pub Anonymous: PDH_FMT_COUNTERVALUE_0,
}
impl windows_core::TypeKind for PDH_FMT_COUNTERVALUE {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for PDH_FMT_COUNTERVALUE_0 {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for PDH_FMT_COUNTERVALUE_ITEM_A {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for PDH_FMT_COUNTERVALUE_ITEM_W {
    type TypeKind = windows_core::CopyType;
}
impl Default for PDH_FMT_COUNTERVALUE_ITEM_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
impl windows_core::TypeKind for PDH_LOG_SERVICE_QUERY_INFO_A {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for PDH_LOG_SERVICE_QUERY_INFO_A_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for PDH_LOG_SERVICE_QUERY_INFO_A_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for PDH_LOG_SERVICE_QUERY_INFO_A_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for PDH_LOG_SERVICE_QUERY_INFO_A_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for PDH_LOG_SERVICE_QUERY_INFO_A_0_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for PDH_LOG_SERVICE_QUERY_INFO_A_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
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
impl windows_core::TypeKind for PDH_LOG_SERVICE_QUERY_INFO_W {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for PDH_LOG_SERVICE_QUERY_INFO_W_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for PDH_LOG_SERVICE_QUERY_INFO_W_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for PDH_LOG_SERVICE_QUERY_INFO_W_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for PDH_LOG_SERVICE_QUERY_INFO_W_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for PDH_LOG_SERVICE_QUERY_INFO_W_0_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for PDH_LOG_SERVICE_QUERY_INFO_W_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PDH_RAW_COUNTER {
    pub CStatus: u32,
    pub TimeStamp: super::super::Foundation::FILETIME,
    pub FirstValue: i64,
    pub SecondValue: i64,
    pub MultiCount: u32,
}
impl windows_core::TypeKind for PDH_RAW_COUNTER {
    type TypeKind = windows_core::CopyType;
}
impl Default for PDH_RAW_COUNTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PDH_RAW_COUNTER_ITEM_A {
    pub szName: windows_core::PSTR,
    pub RawValue: PDH_RAW_COUNTER,
}
impl windows_core::TypeKind for PDH_RAW_COUNTER_ITEM_A {
    type TypeKind = windows_core::CopyType;
}
impl Default for PDH_RAW_COUNTER_ITEM_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PDH_RAW_COUNTER_ITEM_W {
    pub szName: windows_core::PWSTR,
    pub RawValue: PDH_RAW_COUNTER,
}
impl windows_core::TypeKind for PDH_RAW_COUNTER_ITEM_W {
    type TypeKind = windows_core::CopyType;
}
impl Default for PDH_RAW_COUNTER_ITEM_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PDH_RAW_LOG_RECORD {
    pub dwStructureSize: u32,
    pub dwRecordType: PDH_LOG_TYPE,
    pub dwItems: u32,
    pub RawBytes: [u8; 1],
}
impl windows_core::TypeKind for PDH_RAW_LOG_RECORD {
    type TypeKind = windows_core::CopyType;
}
impl Default for PDH_RAW_LOG_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PDH_STATISTICS {
    pub dwFormat: u32,
    pub count: u32,
    pub min: PDH_FMT_COUNTERVALUE,
    pub max: PDH_FMT_COUNTERVALUE,
    pub mean: PDH_FMT_COUNTERVALUE,
}
impl windows_core::TypeKind for PDH_STATISTICS {
    type TypeKind = windows_core::CopyType;
}
impl Default for PDH_STATISTICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PDH_TIME_INFO {
    pub StartTime: i64,
    pub EndTime: i64,
    pub SampleCount: u32,
}
impl windows_core::TypeKind for PDH_TIME_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for PDH_TIME_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PERF_COUNTERSET_INFO {
    pub CounterSetGuid: windows_core::GUID,
    pub ProviderGuid: windows_core::GUID,
    pub NumCounters: u32,
    pub InstanceType: u32,
}
impl windows_core::TypeKind for PERF_COUNTERSET_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for PERF_COUNTERSET_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PERF_COUNTERSET_INSTANCE {
    pub CounterSetGuid: windows_core::GUID,
    pub dwSize: u32,
    pub InstanceId: u32,
    pub InstanceNameOffset: u32,
    pub InstanceNameSize: u32,
}
impl windows_core::TypeKind for PERF_COUNTERSET_INSTANCE {
    type TypeKind = windows_core::CopyType;
}
impl Default for PERF_COUNTERSET_INSTANCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PERF_COUNTERSET_REG_INFO {
    pub CounterSetGuid: windows_core::GUID,
    pub CounterSetType: u32,
    pub DetailLevel: u32,
    pub NumCounters: u32,
    pub InstanceType: u32,
}
impl windows_core::TypeKind for PERF_COUNTERSET_REG_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for PERF_COUNTERSET_REG_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PERF_COUNTER_BLOCK {
    pub ByteLength: u32,
}
impl windows_core::TypeKind for PERF_COUNTER_BLOCK {
    type TypeKind = windows_core::CopyType;
}
impl Default for PERF_COUNTER_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PERF_COUNTER_DATA {
    pub dwDataSize: u32,
    pub dwSize: u32,
}
impl windows_core::TypeKind for PERF_COUNTER_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for PERF_COUNTER_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for PERF_COUNTER_DEFINITION {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for PERF_COUNTER_DEFINITION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for PERF_COUNTER_DEFINITION {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl Default for PERF_COUNTER_DEFINITION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PERF_COUNTER_HEADER {
    pub dwStatus: u32,
    pub dwType: PerfCounterDataType,
    pub dwSize: u32,
    pub Reserved: u32,
}
impl windows_core::TypeKind for PERF_COUNTER_HEADER {
    type TypeKind = windows_core::CopyType;
}
impl Default for PERF_COUNTER_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PERF_COUNTER_IDENTIFIER {
    pub CounterSetGuid: windows_core::GUID,
    pub Status: u32,
    pub Size: u32,
    pub CounterId: u32,
    pub InstanceId: u32,
    pub Index: u32,
    pub Reserved: u32,
}
impl windows_core::TypeKind for PERF_COUNTER_IDENTIFIER {
    type TypeKind = windows_core::CopyType;
}
impl Default for PERF_COUNTER_IDENTIFIER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PERF_COUNTER_IDENTITY {
    pub CounterSetGuid: windows_core::GUID,
    pub BufferSize: u32,
    pub CounterId: u32,
    pub InstanceId: u32,
    pub MachineOffset: u32,
    pub NameOffset: u32,
    pub Reserved: u32,
}
impl windows_core::TypeKind for PERF_COUNTER_IDENTITY {
    type TypeKind = windows_core::CopyType;
}
impl Default for PERF_COUNTER_IDENTITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PERF_COUNTER_INFO {
    pub CounterId: u32,
    pub Type: u32,
    pub Attrib: u64,
    pub Size: u32,
    pub DetailLevel: u32,
    pub Scale: i32,
    pub Offset: u32,
}
impl windows_core::TypeKind for PERF_COUNTER_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for PERF_COUNTER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for PERF_COUNTER_REG_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for PERF_COUNTER_REG_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for PERF_DATA_BLOCK {
    type TypeKind = windows_core::CopyType;
}
impl Default for PERF_DATA_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PERF_DATA_HEADER {
    pub dwTotalSize: u32,
    pub dwNumCounters: u32,
    pub PerfTimeStamp: i64,
    pub PerfTime100NSec: i64,
    pub PerfFreq: i64,
    pub SystemTime: super::super::Foundation::SYSTEMTIME,
}
impl windows_core::TypeKind for PERF_DATA_HEADER {
    type TypeKind = windows_core::CopyType;
}
impl Default for PERF_DATA_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PERF_INSTANCE_DEFINITION {
    pub ByteLength: u32,
    pub ParentObjectTitleIndex: u32,
    pub ParentObjectInstance: u32,
    pub UniqueID: i32,
    pub NameOffset: u32,
    pub NameLength: u32,
}
impl windows_core::TypeKind for PERF_INSTANCE_DEFINITION {
    type TypeKind = windows_core::CopyType;
}
impl Default for PERF_INSTANCE_DEFINITION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PERF_INSTANCE_HEADER {
    pub Size: u32,
    pub InstanceId: u32,
}
impl windows_core::TypeKind for PERF_INSTANCE_HEADER {
    type TypeKind = windows_core::CopyType;
}
impl Default for PERF_INSTANCE_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PERF_MULTI_COUNTERS {
    pub dwSize: u32,
    pub dwCounters: u32,
}
impl windows_core::TypeKind for PERF_MULTI_COUNTERS {
    type TypeKind = windows_core::CopyType;
}
impl Default for PERF_MULTI_COUNTERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PERF_MULTI_INSTANCES {
    pub dwTotalSize: u32,
    pub dwInstances: u32,
}
impl windows_core::TypeKind for PERF_MULTI_INSTANCES {
    type TypeKind = windows_core::CopyType;
}
impl Default for PERF_MULTI_INSTANCES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for PERF_OBJECT_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for PERF_OBJECT_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for PERF_OBJECT_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl Default for PERF_OBJECT_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
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
impl windows_core::TypeKind for PERF_PROVIDER_CONTEXT {
    type TypeKind = windows_core::CopyType;
}
impl Default for PERF_PROVIDER_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PERF_STRING_BUFFER_HEADER {
    pub dwSize: u32,
    pub dwCounters: u32,
}
impl windows_core::TypeKind for PERF_STRING_BUFFER_HEADER {
    type TypeKind = windows_core::CopyType;
}
impl Default for PERF_STRING_BUFFER_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PERF_STRING_COUNTER_HEADER {
    pub dwCounterId: u32,
    pub dwOffset: u32,
}
impl windows_core::TypeKind for PERF_STRING_COUNTER_HEADER {
    type TypeKind = windows_core::CopyType;
}
impl Default for PERF_STRING_COUNTER_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ServerDataCollectorSet: windows_core::GUID = windows_core::GUID::from_u128(0x03837531_098b_11d8_9414_505054503030);
pub const ServerDataCollectorSetCollection: windows_core::GUID = windows_core::GUID::from_u128(0x03837532_098b_11d8_9414_505054503030);
pub const SourcePropPage: windows_core::GUID = windows_core::GUID::from_u128(0x0cf32aa1_7571_11d0_93c4_00aa00a3ddea);
pub const SystemDataCollectorSet: windows_core::GUID = windows_core::GUID::from_u128(0x03837546_098b_11d8_9414_505054503030);
pub const SystemDataCollectorSetCollection: windows_core::GUID = windows_core::GUID::from_u128(0x03837547_098b_11d8_9414_505054503030);
pub const SystemMonitor: windows_core::GUID = windows_core::GUID::from_u128(0xc4d2d8e0_d1dd_11ce_940f_008029004347);
pub const SystemMonitor2: windows_core::GUID = windows_core::GUID::from_u128(0x7f30578c_5f38_4612_acfe_6ed04c7b7af8);
pub const TraceDataProvider: windows_core::GUID = windows_core::GUID::from_u128(0x03837513_098b_11d8_9414_505054503030);
pub const TraceDataProviderCollection: windows_core::GUID = windows_core::GUID::from_u128(0x03837511_098b_11d8_9414_505054503030);
pub const TraceSession: windows_core::GUID = windows_core::GUID::from_u128(0x0383751c_098b_11d8_9414_505054503030);
pub const TraceSessionCollection: windows_core::GUID = windows_core::GUID::from_u128(0x03837530_098b_11d8_9414_505054503030);
pub type CounterPathCallBack = Option<unsafe extern "system" fn(param0: usize) -> i32>;
pub type PERFLIBREQUEST = Option<unsafe extern "system" fn(requestcode: u32, buffer: *mut core::ffi::c_void, buffersize: u32) -> u32>;
pub type PERF_MEM_ALLOC = Option<unsafe extern "system" fn(allocsize: usize, pcontext: *mut core::ffi::c_void) -> *mut core::ffi::c_void>;
pub type PERF_MEM_FREE = Option<unsafe extern "system" fn(pbuffer: *mut core::ffi::c_void, pcontext: *mut core::ffi::c_void)>;
pub type PLA_CABEXTRACT_CALLBACK = Option<unsafe extern "system" fn(filename: windows_core::PCWSTR, context: *mut core::ffi::c_void)>;
pub type PM_CLOSE_PROC = Option<unsafe extern "system" fn() -> u32>;
pub type PM_COLLECT_PROC = Option<unsafe extern "system" fn(pvaluename: windows_core::PCWSTR, ppdata: *mut *mut core::ffi::c_void, pcbtotalbytes: *mut u32, pnumobjecttypes: *mut u32) -> u32>;
pub type PM_OPEN_PROC = Option<unsafe extern "system" fn(pcontext: windows_core::PCWSTR) -> u32>;
#[cfg(feature = "implement")]
core::include!("impl.rs");
