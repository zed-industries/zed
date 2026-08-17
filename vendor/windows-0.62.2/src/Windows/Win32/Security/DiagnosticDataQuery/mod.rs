#[inline]
pub unsafe fn DdqCancelDiagnosticRecordOperation(hsession: HDIAGNOSTIC_DATA_QUERY_SESSION) -> windows_core::Result<()> {
    windows_core::link!("diagnosticdataquery.dll" "system" fn DdqCancelDiagnosticRecordOperation(hsession : HDIAGNOSTIC_DATA_QUERY_SESSION) -> windows_core::HRESULT);
    unsafe { DdqCancelDiagnosticRecordOperation(hsession).ok() }
}
#[inline]
pub unsafe fn DdqCloseSession(hsession: HDIAGNOSTIC_DATA_QUERY_SESSION) -> windows_core::Result<()> {
    windows_core::link!("diagnosticdataquery.dll" "system" fn DdqCloseSession(hsession : HDIAGNOSTIC_DATA_QUERY_SESSION) -> windows_core::HRESULT);
    unsafe { DdqCloseSession(hsession).ok() }
}
#[inline]
pub unsafe fn DdqCreateSession(accesslevel: DdqAccessLevel) -> windows_core::Result<HDIAGNOSTIC_DATA_QUERY_SESSION> {
    windows_core::link!("diagnosticdataquery.dll" "system" fn DdqCreateSession(accesslevel : DdqAccessLevel, hsession : *mut HDIAGNOSTIC_DATA_QUERY_SESSION) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DdqCreateSession(accesslevel, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn DdqExtractDiagnosticReport<P2, P3>(hsession: HDIAGNOSTIC_DATA_QUERY_SESSION, reportstoretype: u32, reportkey: P2, destinationpath: P3) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("diagnosticdataquery.dll" "system" fn DdqExtractDiagnosticReport(hsession : HDIAGNOSTIC_DATA_QUERY_SESSION, reportstoretype : u32, reportkey : windows_core::PCWSTR, destinationpath : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { DdqExtractDiagnosticReport(hsession, reportstoretype, reportkey.param().abi(), destinationpath.param().abi()).ok() }
}
#[inline]
pub unsafe fn DdqFreeDiagnosticRecordLocaleTags(htagdescription: HDIAGNOSTIC_EVENT_TAG_DESCRIPTION) -> windows_core::Result<()> {
    windows_core::link!("diagnosticdataquery.dll" "system" fn DdqFreeDiagnosticRecordLocaleTags(htagdescription : HDIAGNOSTIC_EVENT_TAG_DESCRIPTION) -> windows_core::HRESULT);
    unsafe { DdqFreeDiagnosticRecordLocaleTags(htagdescription).ok() }
}
#[inline]
pub unsafe fn DdqFreeDiagnosticRecordPage(hrecord: HDIAGNOSTIC_RECORD) -> windows_core::Result<()> {
    windows_core::link!("diagnosticdataquery.dll" "system" fn DdqFreeDiagnosticRecordPage(hrecord : HDIAGNOSTIC_RECORD) -> windows_core::HRESULT);
    unsafe { DdqFreeDiagnosticRecordPage(hrecord).ok() }
}
#[inline]
pub unsafe fn DdqFreeDiagnosticRecordProducerCategories(hcategorydescription: HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION) -> windows_core::Result<()> {
    windows_core::link!("diagnosticdataquery.dll" "system" fn DdqFreeDiagnosticRecordProducerCategories(hcategorydescription : HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION) -> windows_core::HRESULT);
    unsafe { DdqFreeDiagnosticRecordProducerCategories(hcategorydescription).ok() }
}
#[inline]
pub unsafe fn DdqFreeDiagnosticRecordProducers(hproducerdescription: HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION) -> windows_core::Result<()> {
    windows_core::link!("diagnosticdataquery.dll" "system" fn DdqFreeDiagnosticRecordProducers(hproducerdescription : HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION) -> windows_core::HRESULT);
    unsafe { DdqFreeDiagnosticRecordProducers(hproducerdescription).ok() }
}
#[inline]
pub unsafe fn DdqFreeDiagnosticReport(hreport: HDIAGNOSTIC_REPORT) -> windows_core::Result<()> {
    windows_core::link!("diagnosticdataquery.dll" "system" fn DdqFreeDiagnosticReport(hreport : HDIAGNOSTIC_REPORT) -> windows_core::HRESULT);
    unsafe { DdqFreeDiagnosticReport(hreport).ok() }
}
#[inline]
pub unsafe fn DdqGetDiagnosticDataAccessLevelAllowed() -> windows_core::Result<DdqAccessLevel> {
    windows_core::link!("diagnosticdataquery.dll" "system" fn DdqGetDiagnosticDataAccessLevelAllowed(accesslevel : *mut DdqAccessLevel) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DdqGetDiagnosticDataAccessLevelAllowed(&mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn DdqGetDiagnosticRecordAtIndex(hrecord: HDIAGNOSTIC_RECORD, index: u32, record: *mut DIAGNOSTIC_DATA_RECORD) -> windows_core::Result<()> {
    windows_core::link!("diagnosticdataquery.dll" "system" fn DdqGetDiagnosticRecordAtIndex(hrecord : HDIAGNOSTIC_RECORD, index : u32, record : *mut DIAGNOSTIC_DATA_RECORD) -> windows_core::HRESULT);
    unsafe { DdqGetDiagnosticRecordAtIndex(hrecord, index, record as _).ok() }
}
#[inline]
pub unsafe fn DdqGetDiagnosticRecordBinaryDistribution(hsession: HDIAGNOSTIC_DATA_QUERY_SESSION, producernames: &[windows_core::PCWSTR], topnbinaries: u32, binarystats: *mut *mut DIAGNOSTIC_DATA_EVENT_BINARY_STATS, statcount: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("diagnosticdataquery.dll" "system" fn DdqGetDiagnosticRecordBinaryDistribution(hsession : HDIAGNOSTIC_DATA_QUERY_SESSION, producernames : *const windows_core::PCWSTR, producernamecount : u32, topnbinaries : u32, binarystats : *mut *mut DIAGNOSTIC_DATA_EVENT_BINARY_STATS, statcount : *mut u32) -> windows_core::HRESULT);
    unsafe { DdqGetDiagnosticRecordBinaryDistribution(hsession, core::mem::transmute(producernames.as_ptr()), producernames.len().try_into().unwrap(), topnbinaries, binarystats as _, statcount as _).ok() }
}
#[inline]
pub unsafe fn DdqGetDiagnosticRecordCategoryAtIndex(hcategorydescription: HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION, index: u32) -> windows_core::Result<DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION> {
    windows_core::link!("diagnosticdataquery.dll" "system" fn DdqGetDiagnosticRecordCategoryAtIndex(hcategorydescription : HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION, index : u32, categorydescription : *mut DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DdqGetDiagnosticRecordCategoryAtIndex(hcategorydescription, index, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn DdqGetDiagnosticRecordCategoryCount(hcategorydescription: HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION) -> windows_core::Result<u32> {
    windows_core::link!("diagnosticdataquery.dll" "system" fn DdqGetDiagnosticRecordCategoryCount(hcategorydescription : HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION, categorydescriptioncount : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DdqGetDiagnosticRecordCategoryCount(hcategorydescription, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn DdqGetDiagnosticRecordCount(hrecord: HDIAGNOSTIC_RECORD) -> windows_core::Result<u32> {
    windows_core::link!("diagnosticdataquery.dll" "system" fn DdqGetDiagnosticRecordCount(hrecord : HDIAGNOSTIC_RECORD, recordcount : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DdqGetDiagnosticRecordCount(hrecord, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn DdqGetDiagnosticRecordLocaleTagAtIndex(htagdescription: HDIAGNOSTIC_EVENT_TAG_DESCRIPTION, index: u32) -> windows_core::Result<DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION> {
    windows_core::link!("diagnosticdataquery.dll" "system" fn DdqGetDiagnosticRecordLocaleTagAtIndex(htagdescription : HDIAGNOSTIC_EVENT_TAG_DESCRIPTION, index : u32, tagdescription : *mut DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DdqGetDiagnosticRecordLocaleTagAtIndex(htagdescription, index, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn DdqGetDiagnosticRecordLocaleTagCount(htagdescription: HDIAGNOSTIC_EVENT_TAG_DESCRIPTION) -> windows_core::Result<u32> {
    windows_core::link!("diagnosticdataquery.dll" "system" fn DdqGetDiagnosticRecordLocaleTagCount(htagdescription : HDIAGNOSTIC_EVENT_TAG_DESCRIPTION, tagdescriptioncount : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DdqGetDiagnosticRecordLocaleTagCount(htagdescription, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn DdqGetDiagnosticRecordLocaleTags<P1>(hsession: HDIAGNOSTIC_DATA_QUERY_SESSION, locale: P1) -> windows_core::Result<HDIAGNOSTIC_EVENT_TAG_DESCRIPTION>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("diagnosticdataquery.dll" "system" fn DdqGetDiagnosticRecordLocaleTags(hsession : HDIAGNOSTIC_DATA_QUERY_SESSION, locale : windows_core::PCWSTR, htagdescription : *mut HDIAGNOSTIC_EVENT_TAG_DESCRIPTION) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DdqGetDiagnosticRecordLocaleTags(hsession, locale.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn DdqGetDiagnosticRecordPage(hsession: HDIAGNOSTIC_DATA_QUERY_SESSION, searchcriteria: *const DIAGNOSTIC_DATA_SEARCH_CRITERIA, offset: u32, pagerecordcount: u32, baserowid: i64) -> windows_core::Result<HDIAGNOSTIC_RECORD> {
    windows_core::link!("diagnosticdataquery.dll" "system" fn DdqGetDiagnosticRecordPage(hsession : HDIAGNOSTIC_DATA_QUERY_SESSION, searchcriteria : *const DIAGNOSTIC_DATA_SEARCH_CRITERIA, offset : u32, pagerecordcount : u32, baserowid : i64, hrecord : *mut HDIAGNOSTIC_RECORD) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DdqGetDiagnosticRecordPage(hsession, searchcriteria, offset, pagerecordcount, baserowid, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn DdqGetDiagnosticRecordPayload(hsession: HDIAGNOSTIC_DATA_QUERY_SESSION, rowid: i64) -> windows_core::Result<windows_core::PCWSTR> {
    windows_core::link!("diagnosticdataquery.dll" "system" fn DdqGetDiagnosticRecordPayload(hsession : HDIAGNOSTIC_DATA_QUERY_SESSION, rowid : i64, payload : *mut windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DdqGetDiagnosticRecordPayload(hsession, rowid, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn DdqGetDiagnosticRecordProducerAtIndex(hproducerdescription: HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION, index: u32) -> windows_core::Result<DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION> {
    windows_core::link!("diagnosticdataquery.dll" "system" fn DdqGetDiagnosticRecordProducerAtIndex(hproducerdescription : HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION, index : u32, producerdescription : *mut DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DdqGetDiagnosticRecordProducerAtIndex(hproducerdescription, index, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn DdqGetDiagnosticRecordProducerCategories<P1>(hsession: HDIAGNOSTIC_DATA_QUERY_SESSION, producername: P1) -> windows_core::Result<HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("diagnosticdataquery.dll" "system" fn DdqGetDiagnosticRecordProducerCategories(hsession : HDIAGNOSTIC_DATA_QUERY_SESSION, producername : windows_core::PCWSTR, hcategorydescription : *mut HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DdqGetDiagnosticRecordProducerCategories(hsession, producername.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn DdqGetDiagnosticRecordProducerCount(hproducerdescription: HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION) -> windows_core::Result<u32> {
    windows_core::link!("diagnosticdataquery.dll" "system" fn DdqGetDiagnosticRecordProducerCount(hproducerdescription : HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION, producerdescriptioncount : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DdqGetDiagnosticRecordProducerCount(hproducerdescription, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn DdqGetDiagnosticRecordProducers(hsession: HDIAGNOSTIC_DATA_QUERY_SESSION) -> windows_core::Result<HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION> {
    windows_core::link!("diagnosticdataquery.dll" "system" fn DdqGetDiagnosticRecordProducers(hsession : HDIAGNOSTIC_DATA_QUERY_SESSION, hproducerdescription : *mut HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DdqGetDiagnosticRecordProducers(hsession, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn DdqGetDiagnosticRecordStats(hsession: HDIAGNOSTIC_DATA_QUERY_SESSION, searchcriteria: *const DIAGNOSTIC_DATA_SEARCH_CRITERIA, recordcount: *mut u32, minrowid: *mut i64, maxrowid: *mut i64) -> windows_core::Result<()> {
    windows_core::link!("diagnosticdataquery.dll" "system" fn DdqGetDiagnosticRecordStats(hsession : HDIAGNOSTIC_DATA_QUERY_SESSION, searchcriteria : *const DIAGNOSTIC_DATA_SEARCH_CRITERIA, recordcount : *mut u32, minrowid : *mut i64, maxrowid : *mut i64) -> windows_core::HRESULT);
    unsafe { DdqGetDiagnosticRecordStats(hsession, searchcriteria, recordcount as _, minrowid as _, maxrowid as _).ok() }
}
#[inline]
pub unsafe fn DdqGetDiagnosticRecordSummary(hsession: HDIAGNOSTIC_DATA_QUERY_SESSION, producernames: &[windows_core::PCWSTR], generalstats: *mut DIAGNOSTIC_DATA_GENERAL_STATS) -> windows_core::Result<()> {
    windows_core::link!("diagnosticdataquery.dll" "system" fn DdqGetDiagnosticRecordSummary(hsession : HDIAGNOSTIC_DATA_QUERY_SESSION, producernames : *const windows_core::PCWSTR, producernamecount : u32, generalstats : *mut DIAGNOSTIC_DATA_GENERAL_STATS) -> windows_core::HRESULT);
    unsafe { DdqGetDiagnosticRecordSummary(hsession, core::mem::transmute(producernames.as_ptr()), producernames.len().try_into().unwrap(), generalstats as _).ok() }
}
#[inline]
pub unsafe fn DdqGetDiagnosticRecordTagDistribution(hsession: HDIAGNOSTIC_DATA_QUERY_SESSION, producernames: &[windows_core::PCWSTR], tagstats: *mut *mut DIAGNOSTIC_DATA_EVENT_TAG_STATS, statcount: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("diagnosticdataquery.dll" "system" fn DdqGetDiagnosticRecordTagDistribution(hsession : HDIAGNOSTIC_DATA_QUERY_SESSION, producernames : *const windows_core::PCWSTR, producernamecount : u32, tagstats : *mut *mut DIAGNOSTIC_DATA_EVENT_TAG_STATS, statcount : *mut u32) -> windows_core::HRESULT);
    unsafe { DdqGetDiagnosticRecordTagDistribution(hsession, core::mem::transmute(producernames.as_ptr()), producernames.len().try_into().unwrap(), tagstats as _, statcount as _).ok() }
}
#[inline]
pub unsafe fn DdqGetDiagnosticReport(hsession: HDIAGNOSTIC_DATA_QUERY_SESSION, reportstoretype: u32) -> windows_core::Result<HDIAGNOSTIC_REPORT> {
    windows_core::link!("diagnosticdataquery.dll" "system" fn DdqGetDiagnosticReport(hsession : HDIAGNOSTIC_DATA_QUERY_SESSION, reportstoretype : u32, hreport : *mut HDIAGNOSTIC_REPORT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DdqGetDiagnosticReport(hsession, reportstoretype, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn DdqGetDiagnosticReportAtIndex(hreport: HDIAGNOSTIC_REPORT, index: u32, report: *mut DIAGNOSTIC_REPORT_DATA) -> windows_core::Result<()> {
    windows_core::link!("diagnosticdataquery.dll" "system" fn DdqGetDiagnosticReportAtIndex(hreport : HDIAGNOSTIC_REPORT, index : u32, report : *mut DIAGNOSTIC_REPORT_DATA) -> windows_core::HRESULT);
    unsafe { DdqGetDiagnosticReportAtIndex(hreport, index, report as _).ok() }
}
#[inline]
pub unsafe fn DdqGetDiagnosticReportCount(hreport: HDIAGNOSTIC_REPORT) -> windows_core::Result<u32> {
    windows_core::link!("diagnosticdataquery.dll" "system" fn DdqGetDiagnosticReportCount(hreport : HDIAGNOSTIC_REPORT, reportcount : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DdqGetDiagnosticReportCount(hreport, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn DdqGetDiagnosticReportStoreReportCount(hsession: HDIAGNOSTIC_DATA_QUERY_SESSION, reportstoretype: u32) -> windows_core::Result<u32> {
    windows_core::link!("diagnosticdataquery.dll" "system" fn DdqGetDiagnosticReportStoreReportCount(hsession : HDIAGNOSTIC_DATA_QUERY_SESSION, reportstoretype : u32, reportcount : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DdqGetDiagnosticReportStoreReportCount(hsession, reportstoretype, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn DdqGetSessionAccessLevel(hsession: HDIAGNOSTIC_DATA_QUERY_SESSION) -> windows_core::Result<DdqAccessLevel> {
    windows_core::link!("diagnosticdataquery.dll" "system" fn DdqGetSessionAccessLevel(hsession : HDIAGNOSTIC_DATA_QUERY_SESSION, accesslevel : *mut DdqAccessLevel) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DdqGetSessionAccessLevel(hsession, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn DdqGetTranscriptConfiguration(hsession: HDIAGNOSTIC_DATA_QUERY_SESSION) -> windows_core::Result<DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION> {
    windows_core::link!("diagnosticdataquery.dll" "system" fn DdqGetTranscriptConfiguration(hsession : HDIAGNOSTIC_DATA_QUERY_SESSION, currentconfig : *mut DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DdqGetTranscriptConfiguration(hsession, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn DdqIsDiagnosticRecordSampledIn<P3, P5>(hsession: HDIAGNOSTIC_DATA_QUERY_SESSION, providergroup: *const windows_core::GUID, providerid: Option<*const windows_core::GUID>, providername: P3, eventid: Option<*const u32>, eventname: P5, eventversion: Option<*const u32>, eventkeywords: Option<*const u64>) -> windows_core::Result<windows_core::BOOL>
where
    P3: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("diagnosticdataquery.dll" "system" fn DdqIsDiagnosticRecordSampledIn(hsession : HDIAGNOSTIC_DATA_QUERY_SESSION, providergroup : *const windows_core::GUID, providerid : *const windows_core::GUID, providername : windows_core::PCWSTR, eventid : *const u32, eventname : windows_core::PCWSTR, eventversion : *const u32, eventkeywords : *const u64, issampledin : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DdqIsDiagnosticRecordSampledIn(hsession, providergroup, providerid.unwrap_or(core::mem::zeroed()) as _, providername.param().abi(), eventid.unwrap_or(core::mem::zeroed()) as _, eventname.param().abi(), eventversion.unwrap_or(core::mem::zeroed()) as _, eventkeywords.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn DdqSetTranscriptConfiguration(hsession: HDIAGNOSTIC_DATA_QUERY_SESSION, desiredconfig: *const DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION) -> windows_core::Result<()> {
    windows_core::link!("diagnosticdataquery.dll" "system" fn DdqSetTranscriptConfiguration(hsession : HDIAGNOSTIC_DATA_QUERY_SESSION, desiredconfig : *const DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION) -> windows_core::HRESULT);
    unsafe { DdqSetTranscriptConfiguration(hsession, desiredconfig).ok() }
}
pub const AllUserData: DdqAccessLevel = DdqAccessLevel(2i32);
pub const CurrentUserData: DdqAccessLevel = DdqAccessLevel(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DIAGNOSTIC_DATA_EVENT_BINARY_STATS {
    pub moduleName: windows_core::PWSTR,
    pub friendlyModuleName: windows_core::PWSTR,
    pub eventCount: u32,
    pub uploadSizeBytes: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION {
    pub id: i32,
    pub name: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION {
    pub name: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION {
    pub privacyTag: i32,
    pub name: windows_core::PWSTR,
    pub description: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DIAGNOSTIC_DATA_EVENT_TAG_STATS {
    pub privacyTag: i32,
    pub eventCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION {
    pub hoursOfHistoryToKeep: u32,
    pub maxStoreMegabytes: u32,
    pub requestedMaxStoreMegabytes: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DIAGNOSTIC_DATA_GENERAL_STATS {
    pub optInLevel: u32,
    pub transcriptSizeBytes: u64,
    pub oldestEventTimestamp: u64,
    pub totalEventCountLast24Hours: u32,
    pub averageDailyEvents: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIAGNOSTIC_DATA_RECORD {
    pub rowId: i64,
    pub timestamp: u64,
    pub eventKeywords: u64,
    pub fullEventName: windows_core::PWSTR,
    pub providerGroupGuid: windows_core::PWSTR,
    pub producerName: windows_core::PWSTR,
    pub privacyTags: *mut i32,
    pub privacyTagCount: u32,
    pub categoryIds: *mut i32,
    pub categoryIdCount: u32,
    pub isCoreData: windows_core::BOOL,
    pub extra1: windows_core::PWSTR,
    pub extra2: windows_core::PWSTR,
    pub extra3: windows_core::PWSTR,
}
impl Default for DIAGNOSTIC_DATA_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIAGNOSTIC_DATA_SEARCH_CRITERIA {
    pub producerNames: *const windows_core::PCWSTR,
    pub producerNameCount: u32,
    pub textToMatch: windows_core::PCWSTR,
    pub categoryIds: *const i32,
    pub categoryIdCount: u32,
    pub privacyTags: *const i32,
    pub privacyTagCount: u32,
    pub coreDataOnly: windows_core::BOOL,
}
impl Default for DIAGNOSTIC_DATA_SEARCH_CRITERIA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIAGNOSTIC_REPORT_DATA {
    pub signature: DIAGNOSTIC_REPORT_SIGNATURE,
    pub bucketId: windows_core::GUID,
    pub reportId: windows_core::GUID,
    pub creationTime: super::super::Foundation::FILETIME,
    pub sizeInBytes: u64,
    pub cabId: windows_core::PWSTR,
    pub reportStatus: u32,
    pub reportIntegratorId: windows_core::GUID,
    pub fileNames: *mut windows_core::PWSTR,
    pub fileCount: u32,
    pub friendlyEventName: windows_core::PWSTR,
    pub applicationName: windows_core::PWSTR,
    pub applicationPath: windows_core::PWSTR,
    pub description: windows_core::PWSTR,
    pub bucketIdString: windows_core::PWSTR,
    pub legacyBucketId: u64,
    pub reportKey: windows_core::PWSTR,
}
impl Default for DIAGNOSTIC_REPORT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIAGNOSTIC_REPORT_PARAMETER {
    pub name: [u16; 129],
    pub value: [u16; 260],
}
impl Default for DIAGNOSTIC_REPORT_PARAMETER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIAGNOSTIC_REPORT_SIGNATURE {
    pub eventName: [u16; 65],
    pub parameters: [DIAGNOSTIC_REPORT_PARAMETER; 10],
}
impl Default for DIAGNOSTIC_REPORT_SIGNATURE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DdqAccessLevel(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HDIAGNOSTIC_DATA_QUERY_SESSION(pub *mut core::ffi::c_void);
impl HDIAGNOSTIC_DATA_QUERY_SESSION {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HDIAGNOSTIC_DATA_QUERY_SESSION {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_core::link!("diagnosticdataquery.dll" "system" fn DdqCloseSession(hsession : *mut core::ffi::c_void) -> i32);
            unsafe {
                DdqCloseSession(self.0);
            }
        }
    }
}
impl Default for HDIAGNOSTIC_DATA_QUERY_SESSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION(pub *mut core::ffi::c_void);
impl HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_core::link!("diagnosticdataquery.dll" "system" fn DdqFreeDiagnosticRecordProducerCategories(hcategorydescription : *mut core::ffi::c_void) -> i32);
            unsafe {
                DdqFreeDiagnosticRecordProducerCategories(self.0);
            }
        }
    }
}
impl Default for HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION(pub *mut core::ffi::c_void);
impl HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_core::link!("diagnosticdataquery.dll" "system" fn DdqFreeDiagnosticRecordProducers(hproducerdescription : *mut core::ffi::c_void) -> i32);
            unsafe {
                DdqFreeDiagnosticRecordProducers(self.0);
            }
        }
    }
}
impl Default for HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HDIAGNOSTIC_EVENT_TAG_DESCRIPTION(pub *mut core::ffi::c_void);
impl HDIAGNOSTIC_EVENT_TAG_DESCRIPTION {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HDIAGNOSTIC_EVENT_TAG_DESCRIPTION {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_core::link!("diagnosticdataquery.dll" "system" fn DdqFreeDiagnosticRecordLocaleTags(htagdescription : *mut core::ffi::c_void) -> i32);
            unsafe {
                DdqFreeDiagnosticRecordLocaleTags(self.0);
            }
        }
    }
}
impl Default for HDIAGNOSTIC_EVENT_TAG_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HDIAGNOSTIC_RECORD(pub *mut core::ffi::c_void);
impl HDIAGNOSTIC_RECORD {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HDIAGNOSTIC_RECORD {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_core::link!("diagnosticdataquery.dll" "system" fn DdqFreeDiagnosticRecordPage(hrecord : *mut core::ffi::c_void) -> i32);
            unsafe {
                DdqFreeDiagnosticRecordPage(self.0);
            }
        }
    }
}
impl Default for HDIAGNOSTIC_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HDIAGNOSTIC_REPORT(pub *mut core::ffi::c_void);
impl HDIAGNOSTIC_REPORT {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HDIAGNOSTIC_REPORT {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_core::link!("diagnosticdataquery.dll" "system" fn DdqFreeDiagnosticReport(hreport : *mut core::ffi::c_void) -> i32);
            unsafe {
                DdqFreeDiagnosticReport(self.0);
            }
        }
    }
}
impl Default for HDIAGNOSTIC_REPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NoData: DdqAccessLevel = DdqAccessLevel(0i32);
