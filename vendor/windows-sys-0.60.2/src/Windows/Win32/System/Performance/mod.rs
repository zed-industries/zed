#[cfg(feature = "Win32_System_Performance_HardwareCounterProfiling")]
pub mod HardwareCounterProfiling;
windows_targets::link!("loadperf.dll" "system" fn BackupPerfRegistryToFileW(szfilename : windows_sys::core::PCWSTR, szcommentstring : windows_sys::core::PCWSTR) -> u32);
windows_targets::link!("loadperf.dll" "system" fn InstallPerfDllA(szcomputername : windows_sys::core::PCSTR, lpinifile : windows_sys::core::PCSTR, dwflags : usize) -> u32);
windows_targets::link!("loadperf.dll" "system" fn InstallPerfDllW(szcomputername : windows_sys::core::PCWSTR, lpinifile : windows_sys::core::PCWSTR, dwflags : usize) -> u32);
windows_targets::link!("loadperf.dll" "system" fn LoadPerfCounterTextStringsA(lpcommandline : windows_sys::core::PCSTR, bquietmodearg : windows_sys::core::BOOL) -> u32);
windows_targets::link!("loadperf.dll" "system" fn LoadPerfCounterTextStringsW(lpcommandline : windows_sys::core::PCWSTR, bquietmodearg : windows_sys::core::BOOL) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhAddCounterA(hquery : PDH_HQUERY, szfullcounterpath : windows_sys::core::PCSTR, dwuserdata : usize, phcounter : *mut PDH_HCOUNTER) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhAddCounterW(hquery : PDH_HQUERY, szfullcounterpath : windows_sys::core::PCWSTR, dwuserdata : usize, phcounter : *mut PDH_HCOUNTER) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhAddEnglishCounterA(hquery : PDH_HQUERY, szfullcounterpath : windows_sys::core::PCSTR, dwuserdata : usize, phcounter : *mut PDH_HCOUNTER) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhAddEnglishCounterW(hquery : PDH_HQUERY, szfullcounterpath : windows_sys::core::PCWSTR, dwuserdata : usize, phcounter : *mut PDH_HCOUNTER) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhBindInputDataSourceA(phdatasource : *mut PDH_HLOG, logfilenamelist : windows_sys::core::PCSTR) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhBindInputDataSourceW(phdatasource : *mut PDH_HLOG, logfilenamelist : windows_sys::core::PCWSTR) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhBrowseCountersA(pbrowsedlgdata : *const PDH_BROWSE_DLG_CONFIG_A) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhBrowseCountersHA(pbrowsedlgdata : *const PDH_BROWSE_DLG_CONFIG_HA) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhBrowseCountersHW(pbrowsedlgdata : *const PDH_BROWSE_DLG_CONFIG_HW) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhBrowseCountersW(pbrowsedlgdata : *const PDH_BROWSE_DLG_CONFIG_W) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhCalculateCounterFromRawValue(hcounter : PDH_HCOUNTER, dwformat : PDH_FMT, rawvalue1 : *const PDH_RAW_COUNTER, rawvalue2 : *const PDH_RAW_COUNTER, fmtvalue : *mut PDH_FMT_COUNTERVALUE) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhCloseLog(hlog : PDH_HLOG, dwflags : u32) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhCloseQuery(hquery : PDH_HQUERY) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhCollectQueryData(hquery : PDH_HQUERY) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhCollectQueryDataEx(hquery : PDH_HQUERY, dwintervaltime : u32, hnewdataevent : super::super::Foundation:: HANDLE) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhCollectQueryDataWithTime(hquery : PDH_HQUERY, plltimestamp : *mut i64) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhComputeCounterStatistics(hcounter : PDH_HCOUNTER, dwformat : PDH_FMT, dwfirstentry : u32, dwnumentries : u32, lprawvaluearray : *const PDH_RAW_COUNTER, data : *mut PDH_STATISTICS) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhConnectMachineA(szmachinename : windows_sys::core::PCSTR) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhConnectMachineW(szmachinename : windows_sys::core::PCWSTR) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhCreateSQLTablesA(szdatasource : windows_sys::core::PCSTR) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhCreateSQLTablesW(szdatasource : windows_sys::core::PCWSTR) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhEnumLogSetNamesA(szdatasource : windows_sys::core::PCSTR, mszdatasetnamelist : windows_sys::core::PSTR, pcchbufferlength : *mut u32) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhEnumLogSetNamesW(szdatasource : windows_sys::core::PCWSTR, mszdatasetnamelist : windows_sys::core::PWSTR, pcchbufferlength : *mut u32) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhEnumMachinesA(szdatasource : windows_sys::core::PCSTR, mszmachinelist : windows_sys::core::PSTR, pcchbuffersize : *mut u32) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhEnumMachinesHA(hdatasource : PDH_HLOG, mszmachinelist : windows_sys::core::PSTR, pcchbuffersize : *mut u32) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhEnumMachinesHW(hdatasource : PDH_HLOG, mszmachinelist : windows_sys::core::PWSTR, pcchbuffersize : *mut u32) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhEnumMachinesW(szdatasource : windows_sys::core::PCWSTR, mszmachinelist : windows_sys::core::PWSTR, pcchbuffersize : *mut u32) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhEnumObjectItemsA(szdatasource : windows_sys::core::PCSTR, szmachinename : windows_sys::core::PCSTR, szobjectname : windows_sys::core::PCSTR, mszcounterlist : windows_sys::core::PSTR, pcchcounterlistlength : *mut u32, mszinstancelist : windows_sys::core::PSTR, pcchinstancelistlength : *mut u32, dwdetaillevel : PERF_DETAIL, dwflags : u32) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhEnumObjectItemsHA(hdatasource : PDH_HLOG, szmachinename : windows_sys::core::PCSTR, szobjectname : windows_sys::core::PCSTR, mszcounterlist : windows_sys::core::PSTR, pcchcounterlistlength : *mut u32, mszinstancelist : windows_sys::core::PSTR, pcchinstancelistlength : *mut u32, dwdetaillevel : PERF_DETAIL, dwflags : u32) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhEnumObjectItemsHW(hdatasource : PDH_HLOG, szmachinename : windows_sys::core::PCWSTR, szobjectname : windows_sys::core::PCWSTR, mszcounterlist : windows_sys::core::PWSTR, pcchcounterlistlength : *mut u32, mszinstancelist : windows_sys::core::PWSTR, pcchinstancelistlength : *mut u32, dwdetaillevel : PERF_DETAIL, dwflags : u32) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhEnumObjectItemsW(szdatasource : windows_sys::core::PCWSTR, szmachinename : windows_sys::core::PCWSTR, szobjectname : windows_sys::core::PCWSTR, mszcounterlist : windows_sys::core::PWSTR, pcchcounterlistlength : *mut u32, mszinstancelist : windows_sys::core::PWSTR, pcchinstancelistlength : *mut u32, dwdetaillevel : PERF_DETAIL, dwflags : u32) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhEnumObjectsA(szdatasource : windows_sys::core::PCSTR, szmachinename : windows_sys::core::PCSTR, mszobjectlist : windows_sys::core::PSTR, pcchbuffersize : *mut u32, dwdetaillevel : PERF_DETAIL, brefresh : windows_sys::core::BOOL) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhEnumObjectsHA(hdatasource : PDH_HLOG, szmachinename : windows_sys::core::PCSTR, mszobjectlist : windows_sys::core::PSTR, pcchbuffersize : *mut u32, dwdetaillevel : PERF_DETAIL, brefresh : windows_sys::core::BOOL) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhEnumObjectsHW(hdatasource : PDH_HLOG, szmachinename : windows_sys::core::PCWSTR, mszobjectlist : windows_sys::core::PWSTR, pcchbuffersize : *mut u32, dwdetaillevel : PERF_DETAIL, brefresh : windows_sys::core::BOOL) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhEnumObjectsW(szdatasource : windows_sys::core::PCWSTR, szmachinename : windows_sys::core::PCWSTR, mszobjectlist : windows_sys::core::PWSTR, pcchbuffersize : *mut u32, dwdetaillevel : PERF_DETAIL, brefresh : windows_sys::core::BOOL) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhExpandCounterPathA(szwildcardpath : windows_sys::core::PCSTR, mszexpandedpathlist : windows_sys::core::PSTR, pcchpathlistlength : *mut u32) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhExpandCounterPathW(szwildcardpath : windows_sys::core::PCWSTR, mszexpandedpathlist : windows_sys::core::PWSTR, pcchpathlistlength : *mut u32) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhExpandWildCardPathA(szdatasource : windows_sys::core::PCSTR, szwildcardpath : windows_sys::core::PCSTR, mszexpandedpathlist : windows_sys::core::PSTR, pcchpathlistlength : *mut u32, dwflags : u32) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhExpandWildCardPathHA(hdatasource : PDH_HLOG, szwildcardpath : windows_sys::core::PCSTR, mszexpandedpathlist : windows_sys::core::PSTR, pcchpathlistlength : *mut u32, dwflags : u32) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhExpandWildCardPathHW(hdatasource : PDH_HLOG, szwildcardpath : windows_sys::core::PCWSTR, mszexpandedpathlist : windows_sys::core::PWSTR, pcchpathlistlength : *mut u32, dwflags : u32) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhExpandWildCardPathW(szdatasource : windows_sys::core::PCWSTR, szwildcardpath : windows_sys::core::PCWSTR, mszexpandedpathlist : windows_sys::core::PWSTR, pcchpathlistlength : *mut u32, dwflags : u32) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhFormatFromRawValue(dwcountertype : u32, dwformat : PDH_FMT, ptimebase : *const i64, prawvalue1 : *const PDH_RAW_COUNTER, prawvalue2 : *const PDH_RAW_COUNTER, pfmtvalue : *mut PDH_FMT_COUNTERVALUE) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhGetCounterInfoA(hcounter : PDH_HCOUNTER, bretrieveexplaintext : bool, pdwbuffersize : *mut u32, lpbuffer : *mut PDH_COUNTER_INFO_A) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhGetCounterInfoW(hcounter : PDH_HCOUNTER, bretrieveexplaintext : bool, pdwbuffersize : *mut u32, lpbuffer : *mut PDH_COUNTER_INFO_W) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhGetCounterTimeBase(hcounter : PDH_HCOUNTER, ptimebase : *mut i64) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhGetDataSourceTimeRangeA(szdatasource : windows_sys::core::PCSTR, pdwnumentries : *mut u32, pinfo : *mut PDH_TIME_INFO, pdwbuffersize : *mut u32) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhGetDataSourceTimeRangeH(hdatasource : PDH_HLOG, pdwnumentries : *mut u32, pinfo : *mut PDH_TIME_INFO, pdwbuffersize : *mut u32) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhGetDataSourceTimeRangeW(szdatasource : windows_sys::core::PCWSTR, pdwnumentries : *mut u32, pinfo : *mut PDH_TIME_INFO, pdwbuffersize : *mut u32) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhGetDefaultPerfCounterA(szdatasource : windows_sys::core::PCSTR, szmachinename : windows_sys::core::PCSTR, szobjectname : windows_sys::core::PCSTR, szdefaultcountername : windows_sys::core::PSTR, pcchbuffersize : *mut u32) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhGetDefaultPerfCounterHA(hdatasource : PDH_HLOG, szmachinename : windows_sys::core::PCSTR, szobjectname : windows_sys::core::PCSTR, szdefaultcountername : windows_sys::core::PSTR, pcchbuffersize : *mut u32) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhGetDefaultPerfCounterHW(hdatasource : PDH_HLOG, szmachinename : windows_sys::core::PCWSTR, szobjectname : windows_sys::core::PCWSTR, szdefaultcountername : windows_sys::core::PWSTR, pcchbuffersize : *mut u32) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhGetDefaultPerfCounterW(szdatasource : windows_sys::core::PCWSTR, szmachinename : windows_sys::core::PCWSTR, szobjectname : windows_sys::core::PCWSTR, szdefaultcountername : windows_sys::core::PWSTR, pcchbuffersize : *mut u32) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhGetDefaultPerfObjectA(szdatasource : windows_sys::core::PCSTR, szmachinename : windows_sys::core::PCSTR, szdefaultobjectname : windows_sys::core::PSTR, pcchbuffersize : *mut u32) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhGetDefaultPerfObjectHA(hdatasource : PDH_HLOG, szmachinename : windows_sys::core::PCSTR, szdefaultobjectname : windows_sys::core::PSTR, pcchbuffersize : *mut u32) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhGetDefaultPerfObjectHW(hdatasource : PDH_HLOG, szmachinename : windows_sys::core::PCWSTR, szdefaultobjectname : windows_sys::core::PWSTR, pcchbuffersize : *mut u32) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhGetDefaultPerfObjectW(szdatasource : windows_sys::core::PCWSTR, szmachinename : windows_sys::core::PCWSTR, szdefaultobjectname : windows_sys::core::PWSTR, pcchbuffersize : *mut u32) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhGetDllVersion(lpdwversion : *mut PDH_DLL_VERSION) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhGetFormattedCounterArrayA(hcounter : PDH_HCOUNTER, dwformat : PDH_FMT, lpdwbuffersize : *mut u32, lpdwitemcount : *mut u32, itembuffer : *mut PDH_FMT_COUNTERVALUE_ITEM_A) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhGetFormattedCounterArrayW(hcounter : PDH_HCOUNTER, dwformat : PDH_FMT, lpdwbuffersize : *mut u32, lpdwitemcount : *mut u32, itembuffer : *mut PDH_FMT_COUNTERVALUE_ITEM_W) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhGetFormattedCounterValue(hcounter : PDH_HCOUNTER, dwformat : PDH_FMT, lpdwtype : *mut u32, pvalue : *mut PDH_FMT_COUNTERVALUE) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhGetLogFileSize(hlog : PDH_HLOG, llsize : *mut i64) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhGetLogSetGUID(hlog : PDH_HLOG, pguid : *mut windows_sys::core::GUID, prunid : *mut i32) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhGetRawCounterArrayA(hcounter : PDH_HCOUNTER, lpdwbuffersize : *mut u32, lpdwitemcount : *mut u32, itembuffer : *mut PDH_RAW_COUNTER_ITEM_A) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhGetRawCounterArrayW(hcounter : PDH_HCOUNTER, lpdwbuffersize : *mut u32, lpdwitemcount : *mut u32, itembuffer : *mut PDH_RAW_COUNTER_ITEM_W) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhGetRawCounterValue(hcounter : PDH_HCOUNTER, lpdwtype : *mut u32, pvalue : *mut PDH_RAW_COUNTER) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhIsRealTimeQuery(hquery : PDH_HQUERY) -> windows_sys::core::BOOL);
windows_targets::link!("pdh.dll" "system" fn PdhLookupPerfIndexByNameA(szmachinename : windows_sys::core::PCSTR, sznamebuffer : windows_sys::core::PCSTR, pdwindex : *mut u32) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhLookupPerfIndexByNameW(szmachinename : windows_sys::core::PCWSTR, sznamebuffer : windows_sys::core::PCWSTR, pdwindex : *mut u32) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhLookupPerfNameByIndexA(szmachinename : windows_sys::core::PCSTR, dwnameindex : u32, sznamebuffer : windows_sys::core::PSTR, pcchnamebuffersize : *mut u32) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhLookupPerfNameByIndexW(szmachinename : windows_sys::core::PCWSTR, dwnameindex : u32, sznamebuffer : windows_sys::core::PWSTR, pcchnamebuffersize : *mut u32) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhMakeCounterPathA(pcounterpathelements : *const PDH_COUNTER_PATH_ELEMENTS_A, szfullpathbuffer : windows_sys::core::PSTR, pcchbuffersize : *mut u32, dwflags : PDH_PATH_FLAGS) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhMakeCounterPathW(pcounterpathelements : *const PDH_COUNTER_PATH_ELEMENTS_W, szfullpathbuffer : windows_sys::core::PWSTR, pcchbuffersize : *mut u32, dwflags : PDH_PATH_FLAGS) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhOpenLogA(szlogfilename : windows_sys::core::PCSTR, dwaccessflags : PDH_LOG, lpdwlogtype : *mut PDH_LOG_TYPE, hquery : PDH_HQUERY, dwmaxsize : u32, szusercaption : windows_sys::core::PCSTR, phlog : *mut PDH_HLOG) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhOpenLogW(szlogfilename : windows_sys::core::PCWSTR, dwaccessflags : PDH_LOG, lpdwlogtype : *mut PDH_LOG_TYPE, hquery : PDH_HQUERY, dwmaxsize : u32, szusercaption : windows_sys::core::PCWSTR, phlog : *mut PDH_HLOG) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhOpenQueryA(szdatasource : windows_sys::core::PCSTR, dwuserdata : usize, phquery : *mut PDH_HQUERY) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhOpenQueryH(hdatasource : PDH_HLOG, dwuserdata : usize, phquery : *mut PDH_HQUERY) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhOpenQueryW(szdatasource : windows_sys::core::PCWSTR, dwuserdata : usize, phquery : *mut PDH_HQUERY) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhParseCounterPathA(szfullpathbuffer : windows_sys::core::PCSTR, pcounterpathelements : *mut PDH_COUNTER_PATH_ELEMENTS_A, pdwbuffersize : *mut u32, dwflags : u32) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhParseCounterPathW(szfullpathbuffer : windows_sys::core::PCWSTR, pcounterpathelements : *mut PDH_COUNTER_PATH_ELEMENTS_W, pdwbuffersize : *mut u32, dwflags : u32) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhParseInstanceNameA(szinstancestring : windows_sys::core::PCSTR, szinstancename : windows_sys::core::PSTR, pcchinstancenamelength : *mut u32, szparentname : windows_sys::core::PSTR, pcchparentnamelength : *mut u32, lpindex : *mut u32) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhParseInstanceNameW(szinstancestring : windows_sys::core::PCWSTR, szinstancename : windows_sys::core::PWSTR, pcchinstancenamelength : *mut u32, szparentname : windows_sys::core::PWSTR, pcchparentnamelength : *mut u32, lpindex : *mut u32) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhReadRawLogRecord(hlog : PDH_HLOG, ftrecord : super::super::Foundation:: FILETIME, prawlogrecord : *mut PDH_RAW_LOG_RECORD, pdwbufferlength : *mut u32) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhRemoveCounter(hcounter : PDH_HCOUNTER) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhSelectDataSourceA(hwndowner : super::super::Foundation:: HWND, dwflags : PDH_SELECT_DATA_SOURCE_FLAGS, szdatasource : windows_sys::core::PSTR, pcchbufferlength : *mut u32) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhSelectDataSourceW(hwndowner : super::super::Foundation:: HWND, dwflags : PDH_SELECT_DATA_SOURCE_FLAGS, szdatasource : windows_sys::core::PWSTR, pcchbufferlength : *mut u32) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhSetCounterScaleFactor(hcounter : PDH_HCOUNTER, lfactor : i32) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhSetDefaultRealTimeDataSource(dwdatasourceid : REAL_TIME_DATA_SOURCE_ID_FLAGS) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhSetLogSetRunID(hlog : PDH_HLOG, runid : i32) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhSetQueryTimeRange(hquery : PDH_HQUERY, pinfo : *const PDH_TIME_INFO) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhUpdateLogA(hlog : PDH_HLOG, szuserstring : windows_sys::core::PCSTR) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhUpdateLogFileCatalog(hlog : PDH_HLOG) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhUpdateLogW(hlog : PDH_HLOG, szuserstring : windows_sys::core::PCWSTR) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhValidatePathA(szfullpathbuffer : windows_sys::core::PCSTR) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhValidatePathExA(hdatasource : PDH_HLOG, szfullpathbuffer : windows_sys::core::PCSTR) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhValidatePathExW(hdatasource : PDH_HLOG, szfullpathbuffer : windows_sys::core::PCWSTR) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhValidatePathW(szfullpathbuffer : windows_sys::core::PCWSTR) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhVerifySQLDBA(szdatasource : windows_sys::core::PCSTR) -> u32);
windows_targets::link!("pdh.dll" "system" fn PdhVerifySQLDBW(szdatasource : windows_sys::core::PCWSTR) -> u32);
windows_targets::link!("advapi32.dll" "system" fn PerfAddCounters(hquery : super::super::Foundation:: HANDLE, pcounters : *mut PERF_COUNTER_IDENTIFIER, cbcounters : u32) -> u32);
windows_targets::link!("advapi32.dll" "system" fn PerfCloseQueryHandle(hquery : super::super::Foundation:: HANDLE) -> u32);
windows_targets::link!("advapi32.dll" "system" fn PerfCreateInstance(providerhandle : super::super::Foundation:: HANDLE, countersetguid : *const windows_sys::core::GUID, name : windows_sys::core::PCWSTR, id : u32) -> *mut PERF_COUNTERSET_INSTANCE);
windows_targets::link!("advapi32.dll" "system" fn PerfDecrementULongCounterValue(provider : super::super::Foundation:: HANDLE, instance : *mut PERF_COUNTERSET_INSTANCE, counterid : u32, value : u32) -> u32);
windows_targets::link!("advapi32.dll" "system" fn PerfDecrementULongLongCounterValue(provider : super::super::Foundation:: HANDLE, instance : *mut PERF_COUNTERSET_INSTANCE, counterid : u32, value : u64) -> u32);
windows_targets::link!("advapi32.dll" "system" fn PerfDeleteCounters(hquery : super::super::Foundation:: HANDLE, pcounters : *mut PERF_COUNTER_IDENTIFIER, cbcounters : u32) -> u32);
windows_targets::link!("advapi32.dll" "system" fn PerfDeleteInstance(provider : super::super::Foundation:: HANDLE, instanceblock : *const PERF_COUNTERSET_INSTANCE) -> u32);
windows_targets::link!("advapi32.dll" "system" fn PerfEnumerateCounterSet(szmachine : windows_sys::core::PCWSTR, pcountersetids : *mut windows_sys::core::GUID, ccountersetids : u32, pccountersetidsactual : *mut u32) -> u32);
windows_targets::link!("advapi32.dll" "system" fn PerfEnumerateCounterSetInstances(szmachine : windows_sys::core::PCWSTR, pcountersetid : *const windows_sys::core::GUID, pinstances : *mut PERF_INSTANCE_HEADER, cbinstances : u32, pcbinstancesactual : *mut u32) -> u32);
windows_targets::link!("advapi32.dll" "system" fn PerfIncrementULongCounterValue(provider : super::super::Foundation:: HANDLE, instance : *mut PERF_COUNTERSET_INSTANCE, counterid : u32, value : u32) -> u32);
windows_targets::link!("advapi32.dll" "system" fn PerfIncrementULongLongCounterValue(provider : super::super::Foundation:: HANDLE, instance : *mut PERF_COUNTERSET_INSTANCE, counterid : u32, value : u64) -> u32);
windows_targets::link!("advapi32.dll" "system" fn PerfOpenQueryHandle(szmachine : windows_sys::core::PCWSTR, phquery : *mut super::super::Foundation:: HANDLE) -> u32);
windows_targets::link!("advapi32.dll" "system" fn PerfQueryCounterData(hquery : super::super::Foundation:: HANDLE, pcounterblock : *mut PERF_DATA_HEADER, cbcounterblock : u32, pcbcounterblockactual : *mut u32) -> u32);
windows_targets::link!("advapi32.dll" "system" fn PerfQueryCounterInfo(hquery : super::super::Foundation:: HANDLE, pcounters : *mut PERF_COUNTER_IDENTIFIER, cbcounters : u32, pcbcountersactual : *mut u32) -> u32);
windows_targets::link!("advapi32.dll" "system" fn PerfQueryCounterSetRegistrationInfo(szmachine : windows_sys::core::PCWSTR, pcountersetid : *const windows_sys::core::GUID, requestcode : PerfRegInfoType, requestlangid : u32, pbreginfo : *mut u8, cbreginfo : u32, pcbreginfoactual : *mut u32) -> u32);
windows_targets::link!("advapi32.dll" "system" fn PerfQueryInstance(providerhandle : super::super::Foundation:: HANDLE, countersetguid : *const windows_sys::core::GUID, name : windows_sys::core::PCWSTR, id : u32) -> *mut PERF_COUNTERSET_INSTANCE);
windows_targets::link!("advapi32.dll" "system" fn PerfSetCounterRefValue(provider : super::super::Foundation:: HANDLE, instance : *mut PERF_COUNTERSET_INSTANCE, counterid : u32, address : *const core::ffi::c_void) -> u32);
windows_targets::link!("advapi32.dll" "system" fn PerfSetCounterSetInfo(providerhandle : super::super::Foundation:: HANDLE, template : *mut PERF_COUNTERSET_INFO, templatesize : u32) -> u32);
windows_targets::link!("advapi32.dll" "system" fn PerfSetULongCounterValue(provider : super::super::Foundation:: HANDLE, instance : *mut PERF_COUNTERSET_INSTANCE, counterid : u32, value : u32) -> u32);
windows_targets::link!("advapi32.dll" "system" fn PerfSetULongLongCounterValue(provider : super::super::Foundation:: HANDLE, instance : *mut PERF_COUNTERSET_INSTANCE, counterid : u32, value : u64) -> u32);
windows_targets::link!("advapi32.dll" "system" fn PerfStartProvider(providerguid : *const windows_sys::core::GUID, controlcallback : PERFLIBREQUEST, phprovider : *mut super::super::Foundation:: HANDLE) -> u32);
windows_targets::link!("advapi32.dll" "system" fn PerfStartProviderEx(providerguid : *const windows_sys::core::GUID, providercontext : *const PERF_PROVIDER_CONTEXT, provider : *mut super::super::Foundation:: HANDLE) -> u32);
windows_targets::link!("advapi32.dll" "system" fn PerfStopProvider(providerhandle : super::super::Foundation:: HANDLE) -> u32);
windows_targets::link!("kernel32.dll" "system" fn QueryPerformanceCounter(lpperformancecount : *mut i64) -> windows_sys::core::BOOL);
windows_targets::link!("kernel32.dll" "system" fn QueryPerformanceFrequency(lpfrequency : *mut i64) -> windows_sys::core::BOOL);
windows_targets::link!("loadperf.dll" "system" fn RestorePerfRegistryFromFileW(szfilename : windows_sys::core::PCWSTR, szlangid : windows_sys::core::PCWSTR) -> u32);
windows_targets::link!("loadperf.dll" "system" fn SetServiceAsTrustedA(szreserved : windows_sys::core::PCSTR, szservicename : windows_sys::core::PCSTR) -> u32);
windows_targets::link!("loadperf.dll" "system" fn SetServiceAsTrustedW(szreserved : windows_sys::core::PCWSTR, szservicename : windows_sys::core::PCWSTR) -> u32);
windows_targets::link!("loadperf.dll" "system" fn UnloadPerfCounterTextStringsA(lpcommandline : windows_sys::core::PCSTR, bquietmodearg : windows_sys::core::BOOL) -> u32);
windows_targets::link!("loadperf.dll" "system" fn UnloadPerfCounterTextStringsW(lpcommandline : windows_sys::core::PCWSTR, bquietmodearg : windows_sys::core::BOOL) -> u32);
windows_targets::link!("loadperf.dll" "system" fn UpdatePerfNameFilesA(sznewctrfilepath : windows_sys::core::PCSTR, sznewhlpfilepath : windows_sys::core::PCSTR, szlanguageid : windows_sys::core::PCSTR, dwflags : usize) -> u32);
windows_targets::link!("loadperf.dll" "system" fn UpdatePerfNameFilesW(sznewctrfilepath : windows_sys::core::PCWSTR, sznewhlpfilepath : windows_sys::core::PCWSTR, szlanguageid : windows_sys::core::PCWSTR, dwflags : usize) -> u32);
pub const AppearPropPage: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe49741e9_93a8_4ab1_8e96_bf4482282e9c);
pub type AutoPathFormat = i32;
pub const BootTraceSession: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x03837538_098b_11d8_9414_505054503030);
pub const BootTraceSessionCollection: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x03837539_098b_11d8_9414_505054503030);
pub type ClockType = i32;
pub type CommitMode = i32;
pub const CounterItem: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc4d2d8e0_d1dd_11ce_940f_008029004348);
pub const CounterItem2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x43196c62_c31f_4ce3_a02e_79efe0f6a525);
pub type CounterPathCallBack = Option<unsafe extern "system" fn(param0: usize) -> i32>;
pub const CounterPropPage: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xcf948561_ede8_11ce_941e_008029004347);
pub const Counters: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb2b066d2_2aac_11cf_942f_008029004347);
pub const DATA_SOURCE_REGISTRY: REAL_TIME_DATA_SOURCE_ID_FLAGS = 1u32;
pub const DATA_SOURCE_WBEM: REAL_TIME_DATA_SOURCE_ID_FLAGS = 4u32;
pub const DIID_DICounterItem: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc08c4ff2_0e2e_11cf_942c_008029004347);
pub const DIID_DILogFileItem: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8d093ffc_f777_4917_82d1_833fbc54c58f);
pub const DIID_DISystemMonitor: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x13d73d81_c32e_11cf_9398_00aa00a3ddea);
pub const DIID_DISystemMonitorEvents: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x84979930_4ab3_11cf_943a_008029004347);
pub const DIID_DISystemMonitorInternal: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x194eb242_c32c_11cf_9398_00aa00a3ddea);
pub const DataCollectorSet: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x03837521_098b_11d8_9414_505054503030);
pub const DataCollectorSetCollection: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x03837525_098b_11d8_9414_505054503030);
pub type DataCollectorSetStatus = i32;
pub type DataCollectorType = i32;
pub type DataManagerSteps = i32;
pub type DataSourceTypeConstants = i32;
pub type DisplayTypeConstants = i32;
pub type FileFormat = i32;
pub type FolderActionSteps = i32;
pub const GeneralPropPage: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc3e5d3d2_1a03_11cf_942d_008029004347);
pub const GraphPropPage: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc3e5d3d3_1a03_11cf_942d_008029004347);
pub const H_WBEM_DATASOURCE: i32 = -1i32;
pub const LIBID_SystemMonitor: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b773e42_2509_11cf_942f_008029004347);
pub const LegacyDataCollectorSet: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x03837526_098b_11d8_9414_505054503030);
pub const LegacyDataCollectorSetCollection: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x03837527_098b_11d8_9414_505054503030);
pub const LegacyTraceSession: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x03837528_098b_11d8_9414_505054503030);
pub const LegacyTraceSessionCollection: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x03837529_098b_11d8_9414_505054503030);
pub const LogFileItem: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x16ec5be8_df93_4237_94e4_9ee918111d71);
pub const LogFiles: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2735d9fd_f6b9_4f19_a5d9_e2d068584bc5);
pub const MAX_COUNTER_PATH: u32 = 256u32;
pub const MAX_PERF_OBJECTS_IN_QUERY_FUNCTION: i32 = 64i32;
pub const PDH_ACCESS_DENIED: u32 = 3221228507u32;
pub const PDH_ASYNC_QUERY_TIMEOUT: u32 = 2147485659u32;
pub const PDH_BINARY_LOG_CORRUPT: u32 = 3221228535u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PDH_BROWSE_DLG_CONFIG_A {
    pub _bitfield: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub szDataSource: windows_sys::core::PSTR,
    pub szReturnPathBuffer: windows_sys::core::PSTR,
    pub cchReturnPathLength: u32,
    pub pCallBack: CounterPathCallBack,
    pub dwCallBackArg: usize,
    pub CallBackStatus: i32,
    pub dwDefaultDetailLevel: PERF_DETAIL,
    pub szDialogBoxCaption: windows_sys::core::PSTR,
}
impl Default for PDH_BROWSE_DLG_CONFIG_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PDH_BROWSE_DLG_CONFIG_HA {
    pub _bitfield: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub hDataSource: PDH_HLOG,
    pub szReturnPathBuffer: windows_sys::core::PSTR,
    pub cchReturnPathLength: u32,
    pub pCallBack: CounterPathCallBack,
    pub dwCallBackArg: usize,
    pub CallBackStatus: i32,
    pub dwDefaultDetailLevel: PERF_DETAIL,
    pub szDialogBoxCaption: windows_sys::core::PSTR,
}
impl Default for PDH_BROWSE_DLG_CONFIG_HA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PDH_BROWSE_DLG_CONFIG_HW {
    pub _bitfield: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub hDataSource: PDH_HLOG,
    pub szReturnPathBuffer: windows_sys::core::PWSTR,
    pub cchReturnPathLength: u32,
    pub pCallBack: CounterPathCallBack,
    pub dwCallBackArg: usize,
    pub CallBackStatus: i32,
    pub dwDefaultDetailLevel: PERF_DETAIL,
    pub szDialogBoxCaption: windows_sys::core::PWSTR,
}
impl Default for PDH_BROWSE_DLG_CONFIG_HW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PDH_BROWSE_DLG_CONFIG_W {
    pub _bitfield: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub szDataSource: windows_sys::core::PWSTR,
    pub szReturnPathBuffer: windows_sys::core::PWSTR,
    pub cchReturnPathLength: u32,
    pub pCallBack: CounterPathCallBack,
    pub dwCallBackArg: usize,
    pub CallBackStatus: i32,
    pub dwDefaultDetailLevel: PERF_DETAIL,
    pub szDialogBoxCaption: windows_sys::core::PWSTR,
}
impl Default for PDH_BROWSE_DLG_CONFIG_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
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
    pub szFullPath: windows_sys::core::PSTR,
    pub Anonymous: PDH_COUNTER_INFO_A_0,
    pub szExplainText: windows_sys::core::PSTR,
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
#[derive(Clone, Copy)]
pub struct PDH_COUNTER_INFO_A_0_0 {
    pub szMachineName: windows_sys::core::PSTR,
    pub szObjectName: windows_sys::core::PSTR,
    pub szInstanceName: windows_sys::core::PSTR,
    pub szParentInstance: windows_sys::core::PSTR,
    pub dwInstanceIndex: u32,
    pub szCounterName: windows_sys::core::PSTR,
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
    pub szFullPath: windows_sys::core::PWSTR,
    pub Anonymous: PDH_COUNTER_INFO_W_0,
    pub szExplainText: windows_sys::core::PWSTR,
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
#[derive(Clone, Copy)]
pub struct PDH_COUNTER_INFO_W_0_0 {
    pub szMachineName: windows_sys::core::PWSTR,
    pub szObjectName: windows_sys::core::PWSTR,
    pub szInstanceName: windows_sys::core::PWSTR,
    pub szParentInstance: windows_sys::core::PWSTR,
    pub dwInstanceIndex: u32,
    pub szCounterName: windows_sys::core::PWSTR,
}
impl Default for PDH_COUNTER_INFO_W_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PDH_COUNTER_PATH_ELEMENTS_A {
    pub szMachineName: windows_sys::core::PSTR,
    pub szObjectName: windows_sys::core::PSTR,
    pub szInstanceName: windows_sys::core::PSTR,
    pub szParentInstance: windows_sys::core::PSTR,
    pub dwInstanceIndex: u32,
    pub szCounterName: windows_sys::core::PSTR,
}
impl Default for PDH_COUNTER_PATH_ELEMENTS_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PDH_COUNTER_PATH_ELEMENTS_W {
    pub szMachineName: windows_sys::core::PWSTR,
    pub szObjectName: windows_sys::core::PWSTR,
    pub szInstanceName: windows_sys::core::PWSTR,
    pub szParentInstance: windows_sys::core::PWSTR,
    pub dwInstanceIndex: u32,
    pub szCounterName: windows_sys::core::PWSTR,
}
impl Default for PDH_COUNTER_PATH_ELEMENTS_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
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
pub const PDH_CVERSION_WIN50: PDH_DLL_VERSION = 1280u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PDH_DATA_ITEM_PATH_ELEMENTS_A {
    pub szMachineName: windows_sys::core::PSTR,
    pub ObjectGUID: windows_sys::core::GUID,
    pub dwItemId: u32,
    pub szInstanceName: windows_sys::core::PSTR,
}
impl Default for PDH_DATA_ITEM_PATH_ELEMENTS_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PDH_DATA_ITEM_PATH_ELEMENTS_W {
    pub szMachineName: windows_sys::core::PWSTR,
    pub ObjectGUID: windows_sys::core::GUID,
    pub dwItemId: u32,
    pub szInstanceName: windows_sys::core::PWSTR,
}
impl Default for PDH_DATA_ITEM_PATH_ELEMENTS_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PDH_DATA_SOURCE_IS_LOG_FILE: u32 = 3221228494u32;
pub const PDH_DATA_SOURCE_IS_REAL_TIME: u32 = 3221228495u32;
pub const PDH_DIALOG_CANCELLED: u32 = 2147485657u32;
pub type PDH_DLL_VERSION = u32;
pub const PDH_END_OF_LOG_FILE: u32 = 2147485658u32;
pub const PDH_ENTRY_NOT_IN_LOG_FILE: u32 = 3221228493u32;
pub const PDH_FILE_ALREADY_EXISTS: u32 = 3221228498u32;
pub const PDH_FILE_NOT_FOUND: u32 = 3221228497u32;
pub const PDH_FLAGS_FILE_BROWSER_ONLY: PDH_SELECT_DATA_SOURCE_FLAGS = 1u32;
pub const PDH_FLAGS_NONE: PDH_SELECT_DATA_SOURCE_FLAGS = 0u32;
pub type PDH_FMT = u32;
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
    pub AnsiStringValue: windows_sys::core::PCSTR,
    pub WideStringValue: windows_sys::core::PCWSTR,
}
impl Default for PDH_FMT_COUNTERVALUE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PDH_FMT_COUNTERVALUE_ITEM_A {
    pub szName: windows_sys::core::PSTR,
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
    pub szName: windows_sys::core::PWSTR,
    pub FmtValue: PDH_FMT_COUNTERVALUE,
}
impl Default for PDH_FMT_COUNTERVALUE_ITEM_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PDH_FMT_DOUBLE: PDH_FMT = 512u32;
pub const PDH_FMT_LARGE: PDH_FMT = 1024u32;
pub const PDH_FMT_LONG: PDH_FMT = 256u32;
pub const PDH_FUNCTION_NOT_FOUND: u32 = 3221228478u32;
pub type PDH_HCOUNTER = *mut core::ffi::c_void;
pub type PDH_HLOG = *mut core::ffi::c_void;
pub type PDH_HQUERY = *mut core::ffi::c_void;
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
pub type PDH_LOG = u32;
pub const PDH_LOGSVC_NOT_OPENED: u32 = 3221228505u32;
pub const PDH_LOGSVC_QUERY_NOT_FOUND: u32 = 3221228504u32;
pub const PDH_LOG_FILE_CREATE_ERROR: u32 = 3221228489u32;
pub const PDH_LOG_FILE_OPEN_ERROR: u32 = 3221228490u32;
pub const PDH_LOG_FILE_TOO_SMALL: u32 = 3221228508u32;
pub const PDH_LOG_READ_ACCESS: PDH_LOG = 65536u32;
pub const PDH_LOG_SAMPLE_TOO_SMALL: u32 = 3221228536u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PDH_LOG_SERVICE_QUERY_INFO_A {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwLogQuota: u32,
    pub szLogFileCaption: windows_sys::core::PSTR,
    pub szDefaultDir: windows_sys::core::PSTR,
    pub szBaseFileName: windows_sys::core::PSTR,
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
#[derive(Clone, Copy)]
pub struct PDH_LOG_SERVICE_QUERY_INFO_A_0_0 {
    pub PdlAutoNameInterval: u32,
    pub PdlAutoNameUnits: u32,
    pub PdlCommandFilename: windows_sys::core::PSTR,
    pub PdlCounterList: windows_sys::core::PSTR,
    pub PdlAutoNameFormat: u32,
    pub PdlSampleInterval: u32,
    pub PdlLogStartTime: super::super::Foundation::FILETIME,
    pub PdlLogEndTime: super::super::Foundation::FILETIME,
}
impl Default for PDH_LOG_SERVICE_QUERY_INFO_A_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
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
    pub TlLogFileName: windows_sys::core::PSTR,
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
    pub szLogFileCaption: windows_sys::core::PWSTR,
    pub szDefaultDir: windows_sys::core::PWSTR,
    pub szBaseFileName: windows_sys::core::PWSTR,
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
#[derive(Clone, Copy)]
pub struct PDH_LOG_SERVICE_QUERY_INFO_W_0_0 {
    pub PdlAutoNameInterval: u32,
    pub PdlAutoNameUnits: u32,
    pub PdlCommandFilename: windows_sys::core::PWSTR,
    pub PdlCounterList: windows_sys::core::PWSTR,
    pub PdlAutoNameFormat: u32,
    pub PdlSampleInterval: u32,
    pub PdlLogStartTime: super::super::Foundation::FILETIME,
    pub PdlLogEndTime: super::super::Foundation::FILETIME,
}
impl Default for PDH_LOG_SERVICE_QUERY_INFO_W_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
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
    pub TlLogFileName: windows_sys::core::PWSTR,
}
impl Default for PDH_LOG_SERVICE_QUERY_INFO_W_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PDH_LOG_TYPE = u32;
pub const PDH_LOG_TYPE_BINARY: PDH_LOG_TYPE = 8u32;
pub const PDH_LOG_TYPE_CSV: PDH_LOG_TYPE = 1u32;
pub const PDH_LOG_TYPE_NOT_FOUND: u32 = 3221228491u32;
pub const PDH_LOG_TYPE_PERFMON: PDH_LOG_TYPE = 6u32;
pub const PDH_LOG_TYPE_RETIRED_BIN: u32 = 3u32;
pub const PDH_LOG_TYPE_SQL: PDH_LOG_TYPE = 7u32;
pub const PDH_LOG_TYPE_TRACE_GENERIC: u32 = 5u32;
pub const PDH_LOG_TYPE_TRACE_KERNEL: u32 = 4u32;
pub const PDH_LOG_TYPE_TSV: PDH_LOG_TYPE = 2u32;
pub const PDH_LOG_TYPE_UNDEFINED: PDH_LOG_TYPE = 0u32;
pub const PDH_LOG_UPDATE_ACCESS: PDH_LOG = 262144u32;
pub const PDH_LOG_WRITE_ACCESS: PDH_LOG = 131072u32;
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
pub type PDH_PATH_FLAGS = u32;
pub const PDH_PATH_WBEM_INPUT: PDH_PATH_FLAGS = 2u32;
pub const PDH_PATH_WBEM_NONE: PDH_PATH_FLAGS = 0u32;
pub const PDH_PATH_WBEM_RESULT: PDH_PATH_FLAGS = 1u32;
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
#[derive(Clone, Copy, Default)]
pub struct PDH_RAW_COUNTER {
    pub CStatus: u32,
    pub TimeStamp: super::super::Foundation::FILETIME,
    pub FirstValue: i64,
    pub SecondValue: i64,
    pub MultiCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PDH_RAW_COUNTER_ITEM_A {
    pub szName: windows_sys::core::PSTR,
    pub RawValue: PDH_RAW_COUNTER,
}
impl Default for PDH_RAW_COUNTER_ITEM_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PDH_RAW_COUNTER_ITEM_W {
    pub szName: windows_sys::core::PWSTR,
    pub RawValue: PDH_RAW_COUNTER,
}
impl Default for PDH_RAW_COUNTER_ITEM_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
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
pub type PDH_SELECT_DATA_SOURCE_FLAGS = u32;
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
#[derive(Clone, Copy, Default)]
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
pub const PDH_VERSION: PDH_DLL_VERSION = 1283u32;
pub const PDH_WBEM_ERROR: u32 = 3221228506u32;
pub type PERFLIBREQUEST = Option<unsafe extern "system" fn(requestcode: u32, buffer: *mut core::ffi::c_void, buffersize: u32) -> u32>;
pub const PERF_ADD_COUNTER: u32 = 1u32;
pub const PERF_AGGREGATE_AVG: PERF_COUNTER_AGGREGATE_FUNC = 2u32;
pub const PERF_AGGREGATE_INSTANCE: windows_sys::core::PCWSTR = windows_sys::core::w!("_Total");
pub const PERF_AGGREGATE_MAX: PERF_COUNTER_AGGREGATE_FUNC = 4u32;
pub const PERF_AGGREGATE_MIN: PERF_COUNTER_AGGREGATE_FUNC = 3u32;
pub const PERF_AGGREGATE_TOTAL: PERF_COUNTER_AGGREGATE_FUNC = 1u32;
pub const PERF_AGGREGATE_UNDEFINED: PERF_COUNTER_AGGREGATE_FUNC = 0u32;
pub const PERF_ATTRIB_BY_REFERENCE: u64 = 1u64;
pub const PERF_ATTRIB_DISPLAY_AS_HEX: u64 = 16u64;
pub const PERF_ATTRIB_DISPLAY_AS_REAL: u64 = 8u64;
pub const PERF_ATTRIB_NO_DISPLAYABLE: u64 = 2u64;
pub const PERF_ATTRIB_NO_GROUP_SEPARATOR: u64 = 4u64;
pub const PERF_COLLECT_END: u32 = 6u32;
pub const PERF_COLLECT_START: u32 = 5u32;
pub const PERF_COUNTERSET: PerfCounterDataType = 6i32;
pub const PERF_COUNTERSET_FLAG_AGGREGATE: u32 = 4u32;
pub const PERF_COUNTERSET_FLAG_HISTORY: u32 = 8u32;
pub const PERF_COUNTERSET_FLAG_INSTANCE: u32 = 16u32;
pub const PERF_COUNTERSET_FLAG_MULTIPLE: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PERF_COUNTERSET_INFO {
    pub CounterSetGuid: windows_sys::core::GUID,
    pub ProviderGuid: windows_sys::core::GUID,
    pub NumCounters: u32,
    pub InstanceType: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PERF_COUNTERSET_INSTANCE {
    pub CounterSetGuid: windows_sys::core::GUID,
    pub dwSize: u32,
    pub InstanceId: u32,
    pub InstanceNameOffset: u32,
    pub InstanceNameSize: u32,
}
pub const PERF_COUNTERSET_MULTI_INSTANCES: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PERF_COUNTERSET_REG_INFO {
    pub CounterSetGuid: windows_sys::core::GUID,
    pub CounterSetType: u32,
    pub DetailLevel: u32,
    pub NumCounters: u32,
    pub InstanceType: u32,
}
pub const PERF_COUNTERSET_SINGLE_AGGREGATE: u32 = 4u32;
pub const PERF_COUNTERSET_SINGLE_INSTANCE: u32 = 0u32;
pub type PERF_COUNTER_AGGREGATE_FUNC = u32;
pub const PERF_COUNTER_BASE: u32 = 196608u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PERF_COUNTER_BLOCK {
    pub ByteLength: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PERF_COUNTER_DATA {
    pub dwDataSize: u32,
    pub dwSize: u32,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct PERF_COUNTER_DEFINITION {
    pub ByteLength: u32,
    pub CounterNameTitleIndex: u32,
    pub CounterNameTitle: windows_sys::core::PWSTR,
    pub CounterHelpTitleIndex: u32,
    pub CounterHelpTitle: windows_sys::core::PWSTR,
    pub DefaultScale: i32,
    pub DetailLevel: u32,
    pub CounterType: u32,
    pub CounterSize: u32,
    pub CounterOffset: u32,
}
#[cfg(target_arch = "x86")]
impl Default for PERF_COUNTER_DEFINITION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy, Default)]
pub struct PERF_COUNTER_HEADER {
    pub dwStatus: u32,
    pub dwType: PerfCounterDataType,
    pub dwSize: u32,
    pub Reserved: u32,
}
pub const PERF_COUNTER_HISTOGRAM: u32 = 393216u32;
pub const PERF_COUNTER_HISTOGRAM_TYPE: u32 = 2147483648u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PERF_COUNTER_IDENTIFIER {
    pub CounterSetGuid: windows_sys::core::GUID,
    pub Status: u32,
    pub Size: u32,
    pub CounterId: u32,
    pub InstanceId: u32,
    pub Index: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PERF_COUNTER_IDENTITY {
    pub CounterSetGuid: windows_sys::core::GUID,
    pub BufferSize: u32,
    pub CounterId: u32,
    pub InstanceId: u32,
    pub MachineOffset: u32,
    pub NameOffset: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy, Default)]
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
pub type PERF_DETAIL = u32;
pub const PERF_DETAIL_ADVANCED: PERF_DETAIL = 200u32;
pub const PERF_DETAIL_EXPERT: PERF_DETAIL = 300u32;
pub const PERF_DETAIL_NOVICE: PERF_DETAIL = 100u32;
pub const PERF_DETAIL_WIZARD: PERF_DETAIL = 400u32;
pub const PERF_DISPLAY_NOSHOW: u32 = 1073741824u32;
pub const PERF_DISPLAY_NO_SUFFIX: u32 = 0u32;
pub const PERF_DISPLAY_PERCENT: u32 = 536870912u32;
pub const PERF_DISPLAY_PER_SEC: u32 = 268435456u32;
pub const PERF_DISPLAY_SECONDS: u32 = 805306368u32;
pub const PERF_ENUM_INSTANCES: u32 = 3u32;
pub const PERF_ERROR_RETURN: PerfCounterDataType = 0i32;
pub const PERF_FILTER: u32 = 9u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PERF_INSTANCE_DEFINITION {
    pub ByteLength: u32,
    pub ParentObjectTitleIndex: u32,
    pub ParentObjectInstance: u32,
    pub UniqueID: i32,
    pub NameOffset: u32,
    pub NameLength: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
pub const PERF_MULTIPLE_COUNTERS: PerfCounterDataType = 2i32;
pub const PERF_MULTIPLE_INSTANCES: PerfCounterDataType = 4i32;
pub const PERF_MULTI_COUNTER: u32 = 33554432u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PERF_MULTI_COUNTERS {
    pub dwSize: u32,
    pub dwCounters: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy)]
pub struct PERF_OBJECT_TYPE {
    pub TotalByteLength: u32,
    pub DefinitionLength: u32,
    pub HeaderLength: u32,
    pub ObjectNameTitleIndex: u32,
    pub ObjectNameTitle: windows_sys::core::PWSTR,
    pub ObjectHelpTitleIndex: u32,
    pub ObjectHelpTitle: windows_sys::core::PWSTR,
    pub DetailLevel: u32,
    pub NumCounters: u32,
    pub DefaultCounter: i32,
    pub NumInstances: i32,
    pub CodePage: u32,
    pub PerfTime: i64,
    pub PerfFreq: i64,
}
#[cfg(target_arch = "x86")]
impl Default for PERF_OBJECT_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy)]
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
pub const PERF_REG_COUNTERSET_ENGLISH_NAME: PerfRegInfoType = 9i32;
pub const PERF_REG_COUNTERSET_HELP_STRING: PerfRegInfoType = 4i32;
pub const PERF_REG_COUNTERSET_NAME_STRING: PerfRegInfoType = 3i32;
pub const PERF_REG_COUNTERSET_STRUCT: PerfRegInfoType = 1i32;
pub const PERF_REG_COUNTER_ENGLISH_NAMES: PerfRegInfoType = 10i32;
pub const PERF_REG_COUNTER_HELP_STRINGS: PerfRegInfoType = 6i32;
pub const PERF_REG_COUNTER_NAME_STRINGS: PerfRegInfoType = 5i32;
pub const PERF_REG_COUNTER_STRUCT: PerfRegInfoType = 2i32;
pub const PERF_REG_PROVIDER_GUID: PerfRegInfoType = 8i32;
pub const PERF_REG_PROVIDER_NAME: PerfRegInfoType = 7i32;
pub const PERF_REMOVE_COUNTER: u32 = 2u32;
pub const PERF_SINGLE_COUNTER: PerfCounterDataType = 1i32;
pub const PERF_SIZE_DWORD: u32 = 0u32;
pub const PERF_SIZE_LARGE: u32 = 256u32;
pub const PERF_SIZE_VARIABLE_LEN: u32 = 768u32;
pub const PERF_SIZE_ZERO: u32 = 512u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PERF_STRING_BUFFER_HEADER {
    pub dwSize: u32,
    pub dwCounters: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
pub const PERF_WILDCARD_INSTANCE: windows_sys::core::PCWSTR = windows_sys::core::w!("*");
pub const PLAL_ALERT_CMD_LINE_A_NAME: u32 = 512u32;
pub const PLAL_ALERT_CMD_LINE_C_NAME: u32 = 1024u32;
pub const PLAL_ALERT_CMD_LINE_D_TIME: u32 = 2048u32;
pub const PLAL_ALERT_CMD_LINE_L_VAL: u32 = 4096u32;
pub const PLAL_ALERT_CMD_LINE_MASK: u32 = 32512u32;
pub const PLAL_ALERT_CMD_LINE_M_VAL: u32 = 8192u32;
pub const PLAL_ALERT_CMD_LINE_SINGLE: u32 = 256u32;
pub const PLAL_ALERT_CMD_LINE_U_TEXT: u32 = 16384u32;
pub type PLA_CABEXTRACT_CALLBACK = Option<unsafe extern "system" fn(filename: windows_sys::core::PCWSTR, context: *mut core::ffi::c_void)>;
pub const PLA_CAPABILITY_AUTOLOGGER: u32 = 32u32;
pub const PLA_CAPABILITY_LEGACY_SESSION: u32 = 8u32;
pub const PLA_CAPABILITY_LEGACY_SVC: u32 = 16u32;
pub const PLA_CAPABILITY_LOCAL: u32 = 268435456u32;
pub const PLA_CAPABILITY_V1_SESSION: u32 = 2u32;
pub const PLA_CAPABILITY_V1_SVC: u32 = 1u32;
pub const PLA_CAPABILITY_V1_SYSTEM: u32 = 4u32;
pub type PM_CLOSE_PROC = Option<unsafe extern "system" fn() -> u32>;
pub type PM_COLLECT_PROC = Option<unsafe extern "system" fn(pvaluename: windows_sys::core::PCWSTR, ppdata: *mut *mut core::ffi::c_void, pcbtotalbytes: *mut u32, pnumobjecttypes: *mut u32) -> u32>;
pub type PM_OPEN_PROC = Option<unsafe extern "system" fn(pcontext: windows_sys::core::PCWSTR) -> u32>;
pub type PerfCounterDataType = i32;
pub type PerfRegInfoType = i32;
pub type REAL_TIME_DATA_SOURCE_ID_FLAGS = u32;
pub type ReportValueTypeConstants = i32;
pub type ResourcePolicy = i32;
pub const S_PDH: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x04d66358_c4a1_419b_8023_23b73902de2c);
pub const ServerDataCollectorSet: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x03837531_098b_11d8_9414_505054503030);
pub const ServerDataCollectorSetCollection: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x03837532_098b_11d8_9414_505054503030);
pub const SourcePropPage: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cf32aa1_7571_11d0_93c4_00aa00a3ddea);
pub type StreamMode = i32;
pub type SysmonBatchReason = i32;
pub type SysmonDataType = i32;
pub type SysmonFileType = i32;
pub const SystemDataCollectorSet: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x03837546_098b_11d8_9414_505054503030);
pub const SystemDataCollectorSetCollection: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x03837547_098b_11d8_9414_505054503030);
pub const SystemMonitor: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc4d2d8e0_d1dd_11ce_940f_008029004347);
pub const SystemMonitor2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7f30578c_5f38_4612_acfe_6ed04c7b7af8);
pub const TraceDataProvider: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x03837513_098b_11d8_9414_505054503030);
pub const TraceDataProviderCollection: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x03837511_098b_11d8_9414_505054503030);
pub const TraceSession: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0383751c_098b_11d8_9414_505054503030);
pub const TraceSessionCollection: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x03837530_098b_11d8_9414_505054503030);
pub type ValueMapType = i32;
pub const WINPERF_LOG_DEBUG: u32 = 2u32;
pub const WINPERF_LOG_NONE: u32 = 0u32;
pub const WINPERF_LOG_USER: u32 = 1u32;
pub const WINPERF_LOG_VERBOSE: u32 = 3u32;
pub type WeekDays = i32;
pub const plaAlert: DataCollectorType = 3i32;
pub const plaApiTrace: DataCollectorType = 4i32;
pub const plaBinary: FileFormat = 3i32;
pub const plaBoth: StreamMode = 3i32;
pub const plaBuffering: StreamMode = 4i32;
pub const plaCommaSeparated: FileFormat = 0i32;
pub const plaCompiling: DataCollectorSetStatus = 2i32;
pub const plaComputer: AutoPathFormat = 2i32;
pub const plaConfiguration: DataCollectorType = 2i32;
pub const plaCreateCab: FolderActionSteps = 1i32;
pub const plaCreateHtml: DataManagerSteps = 4i32;
pub const plaCreateNew: CommitMode = 1i32;
pub const plaCreateOrModify: CommitMode = 3i32;
pub const plaCreateReport: DataManagerSteps = 1i32;
pub const plaCycle: ClockType = 3i32;
pub const plaDeleteCab: FolderActionSteps = 8i32;
pub const plaDeleteData: FolderActionSteps = 2i32;
pub const plaDeleteLargest: ResourcePolicy = 0i32;
pub const plaDeleteOldest: ResourcePolicy = 1i32;
pub const plaDeleteReport: FolderActionSteps = 16i32;
pub const plaEveryday: WeekDays = 127i32;
pub const plaFile: StreamMode = 1i32;
pub const plaFlag: ValueMapType = 2i32;
pub const plaFlagArray: ValueMapType = 3i32;
pub const plaFlushTrace: CommitMode = 32i32;
pub const plaFolderActions: DataManagerSteps = 8i32;
pub const plaFriday: WeekDays = 32i32;
pub const plaIndex: ValueMapType = 1i32;
pub const plaModify: CommitMode = 2i32;
pub const plaMonday: WeekDays = 2i32;
pub const plaMonthDayHour: AutoPathFormat = 256i32;
pub const plaMonthDayHourMinute: AutoPathFormat = 16384i32;
pub const plaNone: AutoPathFormat = 0i32;
pub const plaPattern: AutoPathFormat = 1i32;
pub const plaPending: DataCollectorSetStatus = 3i32;
pub const plaPerformance: ClockType = 1i32;
pub const plaPerformanceCounter: DataCollectorType = 0i32;
pub const plaRealTime: StreamMode = 2i32;
pub const plaResourceFreeing: DataManagerSteps = 16i32;
pub const plaRunOnce: WeekDays = 0i32;
pub const plaRunRules: DataManagerSteps = 2i32;
pub const plaRunning: DataCollectorSetStatus = 1i32;
pub const plaSaturday: WeekDays = 64i32;
pub const plaSendCab: FolderActionSteps = 4i32;
pub const plaSerialNumber: AutoPathFormat = 512i32;
pub const plaSql: FileFormat = 2i32;
pub const plaStopped: DataCollectorSetStatus = 0i32;
pub const plaSunday: WeekDays = 1i32;
pub const plaSystem: ClockType = 2i32;
pub const plaTabSeparated: FileFormat = 1i32;
pub const plaThursday: WeekDays = 16i32;
pub const plaTimeStamp: ClockType = 0i32;
pub const plaTrace: DataCollectorType = 1i32;
pub const plaTuesday: WeekDays = 4i32;
pub const plaUndefined: DataCollectorSetStatus = 4i32;
pub const plaUpdateRunningInstance: CommitMode = 16i32;
pub const plaValidateOnly: CommitMode = 4096i32;
pub const plaValidation: ValueMapType = 4i32;
pub const plaWednesday: WeekDays = 8i32;
pub const plaYearDayOfYear: AutoPathFormat = 1024i32;
pub const plaYearMonth: AutoPathFormat = 2048i32;
pub const plaYearMonthDay: AutoPathFormat = 4096i32;
pub const plaYearMonthDayHour: AutoPathFormat = 8192i32;
pub const sysmonAverage: ReportValueTypeConstants = 2i32;
pub const sysmonBatchAddCounters: SysmonBatchReason = 2i32;
pub const sysmonBatchAddFiles: SysmonBatchReason = 1i32;
pub const sysmonBatchAddFilesAutoCounters: SysmonBatchReason = 3i32;
pub const sysmonBatchNone: SysmonBatchReason = 0i32;
pub const sysmonChartArea: DisplayTypeConstants = 4i32;
pub const sysmonChartStackedArea: DisplayTypeConstants = 5i32;
pub const sysmonCurrentActivity: DataSourceTypeConstants = 1i32;
pub const sysmonCurrentValue: ReportValueTypeConstants = 1i32;
pub const sysmonDataAvg: SysmonDataType = 1i32;
pub const sysmonDataCount: SysmonDataType = 5i32;
pub const sysmonDataMax: SysmonDataType = 3i32;
pub const sysmonDataMin: SysmonDataType = 2i32;
pub const sysmonDataTime: SysmonDataType = 4i32;
pub const sysmonDefaultValue: ReportValueTypeConstants = 0i32;
pub const sysmonFileBlg: SysmonFileType = 5i32;
pub const sysmonFileCsv: SysmonFileType = 3i32;
pub const sysmonFileGif: SysmonFileType = 7i32;
pub const sysmonFileHtml: SysmonFileType = 1i32;
pub const sysmonFileReport: SysmonFileType = 2i32;
pub const sysmonFileRetiredBlg: SysmonFileType = 6i32;
pub const sysmonFileTsv: SysmonFileType = 4i32;
pub const sysmonHistogram: DisplayTypeConstants = 2i32;
pub const sysmonLineGraph: DisplayTypeConstants = 1i32;
pub const sysmonLogFiles: DataSourceTypeConstants = 2i32;
pub const sysmonMaximum: ReportValueTypeConstants = 4i32;
pub const sysmonMinimum: ReportValueTypeConstants = 3i32;
pub const sysmonNullDataSource: DataSourceTypeConstants = -1i32;
pub const sysmonReport: DisplayTypeConstants = 3i32;
pub const sysmonSqlLog: DataSourceTypeConstants = 3i32;
