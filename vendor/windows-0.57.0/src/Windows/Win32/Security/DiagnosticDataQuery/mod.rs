#[inline]
pub unsafe fn DdqCancelDiagnosticRecordOperation<P0>(hsession: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::HDIAGNOSTIC_DATA_QUERY_SESSION>,
{
    windows_targets::link!("diagnosticdataquery.dll" "system" fn DdqCancelDiagnosticRecordOperation(hsession : super:: HDIAGNOSTIC_DATA_QUERY_SESSION) -> windows_core::HRESULT);
    DdqCancelDiagnosticRecordOperation(hsession.param().abi()).ok()
}
#[inline]
pub unsafe fn DdqCloseSession<P0>(hsession: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::HDIAGNOSTIC_DATA_QUERY_SESSION>,
{
    windows_targets::link!("diagnosticdataquery.dll" "system" fn DdqCloseSession(hsession : super:: HDIAGNOSTIC_DATA_QUERY_SESSION) -> windows_core::HRESULT);
    DdqCloseSession(hsession.param().abi()).ok()
}
#[inline]
pub unsafe fn DdqCreateSession(accesslevel: DdqAccessLevel) -> windows_core::Result<super::HDIAGNOSTIC_DATA_QUERY_SESSION> {
    windows_targets::link!("diagnosticdataquery.dll" "system" fn DdqCreateSession(accesslevel : DdqAccessLevel, hsession : *mut super:: HDIAGNOSTIC_DATA_QUERY_SESSION) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    DdqCreateSession(accesslevel, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn DdqExtractDiagnosticReport<P0, P1, P2>(hsession: P0, reportstoretype: u32, reportkey: P1, destinationpath: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::HDIAGNOSTIC_DATA_QUERY_SESSION>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("diagnosticdataquery.dll" "system" fn DdqExtractDiagnosticReport(hsession : super:: HDIAGNOSTIC_DATA_QUERY_SESSION, reportstoretype : u32, reportkey : windows_core::PCWSTR, destinationpath : windows_core::PCWSTR) -> windows_core::HRESULT);
    DdqExtractDiagnosticReport(hsession.param().abi(), reportstoretype, reportkey.param().abi(), destinationpath.param().abi()).ok()
}
#[inline]
pub unsafe fn DdqFreeDiagnosticRecordLocaleTags<P0>(htagdescription: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::HDIAGNOSTIC_EVENT_TAG_DESCRIPTION>,
{
    windows_targets::link!("diagnosticdataquery.dll" "system" fn DdqFreeDiagnosticRecordLocaleTags(htagdescription : super:: HDIAGNOSTIC_EVENT_TAG_DESCRIPTION) -> windows_core::HRESULT);
    DdqFreeDiagnosticRecordLocaleTags(htagdescription.param().abi()).ok()
}
#[inline]
pub unsafe fn DdqFreeDiagnosticRecordPage<P0>(hrecord: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::HDIAGNOSTIC_RECORD>,
{
    windows_targets::link!("diagnosticdataquery.dll" "system" fn DdqFreeDiagnosticRecordPage(hrecord : super:: HDIAGNOSTIC_RECORD) -> windows_core::HRESULT);
    DdqFreeDiagnosticRecordPage(hrecord.param().abi()).ok()
}
#[inline]
pub unsafe fn DdqFreeDiagnosticRecordProducerCategories<P0>(hcategorydescription: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION>,
{
    windows_targets::link!("diagnosticdataquery.dll" "system" fn DdqFreeDiagnosticRecordProducerCategories(hcategorydescription : super:: HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION) -> windows_core::HRESULT);
    DdqFreeDiagnosticRecordProducerCategories(hcategorydescription.param().abi()).ok()
}
#[inline]
pub unsafe fn DdqFreeDiagnosticRecordProducers<P0>(hproducerdescription: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION>,
{
    windows_targets::link!("diagnosticdataquery.dll" "system" fn DdqFreeDiagnosticRecordProducers(hproducerdescription : super:: HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION) -> windows_core::HRESULT);
    DdqFreeDiagnosticRecordProducers(hproducerdescription.param().abi()).ok()
}
#[inline]
pub unsafe fn DdqFreeDiagnosticReport<P0>(hreport: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::HDIAGNOSTIC_REPORT>,
{
    windows_targets::link!("diagnosticdataquery.dll" "system" fn DdqFreeDiagnosticReport(hreport : super:: HDIAGNOSTIC_REPORT) -> windows_core::HRESULT);
    DdqFreeDiagnosticReport(hreport.param().abi()).ok()
}
#[inline]
pub unsafe fn DdqGetDiagnosticDataAccessLevelAllowed() -> windows_core::Result<DdqAccessLevel> {
    windows_targets::link!("diagnosticdataquery.dll" "system" fn DdqGetDiagnosticDataAccessLevelAllowed(accesslevel : *mut DdqAccessLevel) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    DdqGetDiagnosticDataAccessLevelAllowed(&mut result__).map(|| result__)
}
#[inline]
pub unsafe fn DdqGetDiagnosticRecordAtIndex<P0>(hrecord: P0, index: u32, record: *mut DIAGNOSTIC_DATA_RECORD) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::HDIAGNOSTIC_RECORD>,
{
    windows_targets::link!("diagnosticdataquery.dll" "system" fn DdqGetDiagnosticRecordAtIndex(hrecord : super:: HDIAGNOSTIC_RECORD, index : u32, record : *mut DIAGNOSTIC_DATA_RECORD) -> windows_core::HRESULT);
    DdqGetDiagnosticRecordAtIndex(hrecord.param().abi(), index, record).ok()
}
#[inline]
pub unsafe fn DdqGetDiagnosticRecordBinaryDistribution<P0>(hsession: P0, producernames: &[windows_core::PCWSTR], topnbinaries: u32, binarystats: *mut *mut DIAGNOSTIC_DATA_EVENT_BINARY_STATS, statcount: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::HDIAGNOSTIC_DATA_QUERY_SESSION>,
{
    windows_targets::link!("diagnosticdataquery.dll" "system" fn DdqGetDiagnosticRecordBinaryDistribution(hsession : super:: HDIAGNOSTIC_DATA_QUERY_SESSION, producernames : *const windows_core::PCWSTR, producernamecount : u32, topnbinaries : u32, binarystats : *mut *mut DIAGNOSTIC_DATA_EVENT_BINARY_STATS, statcount : *mut u32) -> windows_core::HRESULT);
    DdqGetDiagnosticRecordBinaryDistribution(hsession.param().abi(), core::mem::transmute(producernames.as_ptr()), producernames.len().try_into().unwrap(), topnbinaries, binarystats, statcount).ok()
}
#[inline]
pub unsafe fn DdqGetDiagnosticRecordCategoryAtIndex<P0>(hcategorydescription: P0, index: u32) -> windows_core::Result<DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION>
where
    P0: windows_core::Param<super::HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION>,
{
    windows_targets::link!("diagnosticdataquery.dll" "system" fn DdqGetDiagnosticRecordCategoryAtIndex(hcategorydescription : super:: HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION, index : u32, categorydescription : *mut DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    DdqGetDiagnosticRecordCategoryAtIndex(hcategorydescription.param().abi(), index, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn DdqGetDiagnosticRecordCategoryCount<P0>(hcategorydescription: P0) -> windows_core::Result<u32>
where
    P0: windows_core::Param<super::HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION>,
{
    windows_targets::link!("diagnosticdataquery.dll" "system" fn DdqGetDiagnosticRecordCategoryCount(hcategorydescription : super:: HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION, categorydescriptioncount : *mut u32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    DdqGetDiagnosticRecordCategoryCount(hcategorydescription.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn DdqGetDiagnosticRecordCount<P0>(hrecord: P0) -> windows_core::Result<u32>
where
    P0: windows_core::Param<super::HDIAGNOSTIC_RECORD>,
{
    windows_targets::link!("diagnosticdataquery.dll" "system" fn DdqGetDiagnosticRecordCount(hrecord : super:: HDIAGNOSTIC_RECORD, recordcount : *mut u32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    DdqGetDiagnosticRecordCount(hrecord.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn DdqGetDiagnosticRecordLocaleTagAtIndex<P0>(htagdescription: P0, index: u32) -> windows_core::Result<DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION>
where
    P0: windows_core::Param<super::HDIAGNOSTIC_EVENT_TAG_DESCRIPTION>,
{
    windows_targets::link!("diagnosticdataquery.dll" "system" fn DdqGetDiagnosticRecordLocaleTagAtIndex(htagdescription : super:: HDIAGNOSTIC_EVENT_TAG_DESCRIPTION, index : u32, tagdescription : *mut DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    DdqGetDiagnosticRecordLocaleTagAtIndex(htagdescription.param().abi(), index, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn DdqGetDiagnosticRecordLocaleTagCount<P0>(htagdescription: P0) -> windows_core::Result<u32>
where
    P0: windows_core::Param<super::HDIAGNOSTIC_EVENT_TAG_DESCRIPTION>,
{
    windows_targets::link!("diagnosticdataquery.dll" "system" fn DdqGetDiagnosticRecordLocaleTagCount(htagdescription : super:: HDIAGNOSTIC_EVENT_TAG_DESCRIPTION, tagdescriptioncount : *mut u32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    DdqGetDiagnosticRecordLocaleTagCount(htagdescription.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn DdqGetDiagnosticRecordLocaleTags<P0, P1>(hsession: P0, locale: P1) -> windows_core::Result<super::HDIAGNOSTIC_EVENT_TAG_DESCRIPTION>
where
    P0: windows_core::Param<super::HDIAGNOSTIC_DATA_QUERY_SESSION>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("diagnosticdataquery.dll" "system" fn DdqGetDiagnosticRecordLocaleTags(hsession : super:: HDIAGNOSTIC_DATA_QUERY_SESSION, locale : windows_core::PCWSTR, htagdescription : *mut super:: HDIAGNOSTIC_EVENT_TAG_DESCRIPTION) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    DdqGetDiagnosticRecordLocaleTags(hsession.param().abi(), locale.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn DdqGetDiagnosticRecordPage<P0>(hsession: P0, searchcriteria: *const DIAGNOSTIC_DATA_SEARCH_CRITERIA, offset: u32, pagerecordcount: u32, baserowid: i64) -> windows_core::Result<super::HDIAGNOSTIC_RECORD>
where
    P0: windows_core::Param<super::HDIAGNOSTIC_DATA_QUERY_SESSION>,
{
    windows_targets::link!("diagnosticdataquery.dll" "system" fn DdqGetDiagnosticRecordPage(hsession : super:: HDIAGNOSTIC_DATA_QUERY_SESSION, searchcriteria : *const DIAGNOSTIC_DATA_SEARCH_CRITERIA, offset : u32, pagerecordcount : u32, baserowid : i64, hrecord : *mut super:: HDIAGNOSTIC_RECORD) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    DdqGetDiagnosticRecordPage(hsession.param().abi(), searchcriteria, offset, pagerecordcount, baserowid, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn DdqGetDiagnosticRecordPayload<P0>(hsession: P0, rowid: i64) -> windows_core::Result<windows_core::PCWSTR>
where
    P0: windows_core::Param<super::HDIAGNOSTIC_DATA_QUERY_SESSION>,
{
    windows_targets::link!("diagnosticdataquery.dll" "system" fn DdqGetDiagnosticRecordPayload(hsession : super:: HDIAGNOSTIC_DATA_QUERY_SESSION, rowid : i64, payload : *mut windows_core::PCWSTR) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    DdqGetDiagnosticRecordPayload(hsession.param().abi(), rowid, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn DdqGetDiagnosticRecordProducerAtIndex<P0>(hproducerdescription: P0, index: u32) -> windows_core::Result<DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION>
where
    P0: windows_core::Param<super::HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION>,
{
    windows_targets::link!("diagnosticdataquery.dll" "system" fn DdqGetDiagnosticRecordProducerAtIndex(hproducerdescription : super:: HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION, index : u32, producerdescription : *mut DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    DdqGetDiagnosticRecordProducerAtIndex(hproducerdescription.param().abi(), index, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn DdqGetDiagnosticRecordProducerCategories<P0, P1>(hsession: P0, producername: P1) -> windows_core::Result<super::HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION>
where
    P0: windows_core::Param<super::HDIAGNOSTIC_DATA_QUERY_SESSION>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("diagnosticdataquery.dll" "system" fn DdqGetDiagnosticRecordProducerCategories(hsession : super:: HDIAGNOSTIC_DATA_QUERY_SESSION, producername : windows_core::PCWSTR, hcategorydescription : *mut super:: HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    DdqGetDiagnosticRecordProducerCategories(hsession.param().abi(), producername.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn DdqGetDiagnosticRecordProducerCount<P0>(hproducerdescription: P0) -> windows_core::Result<u32>
where
    P0: windows_core::Param<super::HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION>,
{
    windows_targets::link!("diagnosticdataquery.dll" "system" fn DdqGetDiagnosticRecordProducerCount(hproducerdescription : super:: HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION, producerdescriptioncount : *mut u32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    DdqGetDiagnosticRecordProducerCount(hproducerdescription.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn DdqGetDiagnosticRecordProducers<P0>(hsession: P0) -> windows_core::Result<super::HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION>
where
    P0: windows_core::Param<super::HDIAGNOSTIC_DATA_QUERY_SESSION>,
{
    windows_targets::link!("diagnosticdataquery.dll" "system" fn DdqGetDiagnosticRecordProducers(hsession : super:: HDIAGNOSTIC_DATA_QUERY_SESSION, hproducerdescription : *mut super:: HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    DdqGetDiagnosticRecordProducers(hsession.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn DdqGetDiagnosticRecordStats<P0>(hsession: P0, searchcriteria: *const DIAGNOSTIC_DATA_SEARCH_CRITERIA, recordcount: *mut u32, minrowid: *mut i64, maxrowid: *mut i64) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::HDIAGNOSTIC_DATA_QUERY_SESSION>,
{
    windows_targets::link!("diagnosticdataquery.dll" "system" fn DdqGetDiagnosticRecordStats(hsession : super:: HDIAGNOSTIC_DATA_QUERY_SESSION, searchcriteria : *const DIAGNOSTIC_DATA_SEARCH_CRITERIA, recordcount : *mut u32, minrowid : *mut i64, maxrowid : *mut i64) -> windows_core::HRESULT);
    DdqGetDiagnosticRecordStats(hsession.param().abi(), searchcriteria, recordcount, minrowid, maxrowid).ok()
}
#[inline]
pub unsafe fn DdqGetDiagnosticRecordSummary<P0>(hsession: P0, producernames: &[windows_core::PCWSTR], generalstats: *mut DIAGNOSTIC_DATA_GENERAL_STATS) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::HDIAGNOSTIC_DATA_QUERY_SESSION>,
{
    windows_targets::link!("diagnosticdataquery.dll" "system" fn DdqGetDiagnosticRecordSummary(hsession : super:: HDIAGNOSTIC_DATA_QUERY_SESSION, producernames : *const windows_core::PCWSTR, producernamecount : u32, generalstats : *mut DIAGNOSTIC_DATA_GENERAL_STATS) -> windows_core::HRESULT);
    DdqGetDiagnosticRecordSummary(hsession.param().abi(), core::mem::transmute(producernames.as_ptr()), producernames.len().try_into().unwrap(), generalstats).ok()
}
#[inline]
pub unsafe fn DdqGetDiagnosticRecordTagDistribution<P0>(hsession: P0, producernames: &[windows_core::PCWSTR], tagstats: *mut *mut DIAGNOSTIC_DATA_EVENT_TAG_STATS, statcount: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::HDIAGNOSTIC_DATA_QUERY_SESSION>,
{
    windows_targets::link!("diagnosticdataquery.dll" "system" fn DdqGetDiagnosticRecordTagDistribution(hsession : super:: HDIAGNOSTIC_DATA_QUERY_SESSION, producernames : *const windows_core::PCWSTR, producernamecount : u32, tagstats : *mut *mut DIAGNOSTIC_DATA_EVENT_TAG_STATS, statcount : *mut u32) -> windows_core::HRESULT);
    DdqGetDiagnosticRecordTagDistribution(hsession.param().abi(), core::mem::transmute(producernames.as_ptr()), producernames.len().try_into().unwrap(), tagstats, statcount).ok()
}
#[inline]
pub unsafe fn DdqGetDiagnosticReport<P0>(hsession: P0, reportstoretype: u32) -> windows_core::Result<super::HDIAGNOSTIC_REPORT>
where
    P0: windows_core::Param<super::HDIAGNOSTIC_DATA_QUERY_SESSION>,
{
    windows_targets::link!("diagnosticdataquery.dll" "system" fn DdqGetDiagnosticReport(hsession : super:: HDIAGNOSTIC_DATA_QUERY_SESSION, reportstoretype : u32, hreport : *mut super:: HDIAGNOSTIC_REPORT) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    DdqGetDiagnosticReport(hsession.param().abi(), reportstoretype, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn DdqGetDiagnosticReportAtIndex<P0>(hreport: P0, index: u32, report: *mut DIAGNOSTIC_REPORT_DATA) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::HDIAGNOSTIC_REPORT>,
{
    windows_targets::link!("diagnosticdataquery.dll" "system" fn DdqGetDiagnosticReportAtIndex(hreport : super:: HDIAGNOSTIC_REPORT, index : u32, report : *mut DIAGNOSTIC_REPORT_DATA) -> windows_core::HRESULT);
    DdqGetDiagnosticReportAtIndex(hreport.param().abi(), index, report).ok()
}
#[inline]
pub unsafe fn DdqGetDiagnosticReportCount<P0>(hreport: P0) -> windows_core::Result<u32>
where
    P0: windows_core::Param<super::HDIAGNOSTIC_REPORT>,
{
    windows_targets::link!("diagnosticdataquery.dll" "system" fn DdqGetDiagnosticReportCount(hreport : super:: HDIAGNOSTIC_REPORT, reportcount : *mut u32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    DdqGetDiagnosticReportCount(hreport.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn DdqGetDiagnosticReportStoreReportCount<P0>(hsession: P0, reportstoretype: u32) -> windows_core::Result<u32>
where
    P0: windows_core::Param<super::HDIAGNOSTIC_DATA_QUERY_SESSION>,
{
    windows_targets::link!("diagnosticdataquery.dll" "system" fn DdqGetDiagnosticReportStoreReportCount(hsession : super:: HDIAGNOSTIC_DATA_QUERY_SESSION, reportstoretype : u32, reportcount : *mut u32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    DdqGetDiagnosticReportStoreReportCount(hsession.param().abi(), reportstoretype, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn DdqGetSessionAccessLevel<P0>(hsession: P0) -> windows_core::Result<DdqAccessLevel>
where
    P0: windows_core::Param<super::HDIAGNOSTIC_DATA_QUERY_SESSION>,
{
    windows_targets::link!("diagnosticdataquery.dll" "system" fn DdqGetSessionAccessLevel(hsession : super:: HDIAGNOSTIC_DATA_QUERY_SESSION, accesslevel : *mut DdqAccessLevel) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    DdqGetSessionAccessLevel(hsession.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn DdqGetTranscriptConfiguration<P0>(hsession: P0) -> windows_core::Result<DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION>
where
    P0: windows_core::Param<super::HDIAGNOSTIC_DATA_QUERY_SESSION>,
{
    windows_targets::link!("diagnosticdataquery.dll" "system" fn DdqGetTranscriptConfiguration(hsession : super:: HDIAGNOSTIC_DATA_QUERY_SESSION, currentconfig : *mut DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    DdqGetTranscriptConfiguration(hsession.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn DdqIsDiagnosticRecordSampledIn<P0, P1, P2>(hsession: P0, providergroup: *const windows_core::GUID, providerid: Option<*const windows_core::GUID>, providername: P1, eventid: Option<*const u32>, eventname: P2, eventversion: Option<*const u32>, eventkeywords: Option<*const u64>) -> windows_core::Result<super::super::Foundation::BOOL>
where
    P0: windows_core::Param<super::HDIAGNOSTIC_DATA_QUERY_SESSION>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("diagnosticdataquery.dll" "system" fn DdqIsDiagnosticRecordSampledIn(hsession : super:: HDIAGNOSTIC_DATA_QUERY_SESSION, providergroup : *const windows_core::GUID, providerid : *const windows_core::GUID, providername : windows_core::PCWSTR, eventid : *const u32, eventname : windows_core::PCWSTR, eventversion : *const u32, eventkeywords : *const u64, issampledin : *mut super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    DdqIsDiagnosticRecordSampledIn(hsession.param().abi(), providergroup, core::mem::transmute(providerid.unwrap_or(std::ptr::null())), providername.param().abi(), core::mem::transmute(eventid.unwrap_or(std::ptr::null())), eventname.param().abi(), core::mem::transmute(eventversion.unwrap_or(std::ptr::null())), core::mem::transmute(eventkeywords.unwrap_or(std::ptr::null())), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn DdqSetTranscriptConfiguration<P0>(hsession: P0, desiredconfig: *const DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::HDIAGNOSTIC_DATA_QUERY_SESSION>,
{
    windows_targets::link!("diagnosticdataquery.dll" "system" fn DdqSetTranscriptConfiguration(hsession : super:: HDIAGNOSTIC_DATA_QUERY_SESSION, desiredconfig : *const DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION) -> windows_core::HRESULT);
    DdqSetTranscriptConfiguration(hsession.param().abi(), desiredconfig).ok()
}
pub const AllUserData: DdqAccessLevel = DdqAccessLevel(2i32);
pub const CurrentUserData: DdqAccessLevel = DdqAccessLevel(1i32);
pub const NoData: DdqAccessLevel = DdqAccessLevel(0i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DdqAccessLevel(pub i32);
impl windows_core::TypeKind for DdqAccessLevel {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DdqAccessLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DdqAccessLevel").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DIAGNOSTIC_DATA_EVENT_BINARY_STATS {
    pub moduleName: windows_core::PWSTR,
    pub friendlyModuleName: windows_core::PWSTR,
    pub eventCount: u32,
    pub uploadSizeBytes: u64,
}
impl windows_core::TypeKind for DIAGNOSTIC_DATA_EVENT_BINARY_STATS {
    type TypeKind = windows_core::CopyType;
}
impl Default for DIAGNOSTIC_DATA_EVENT_BINARY_STATS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION {
    pub id: i32,
    pub name: windows_core::PWSTR,
}
impl windows_core::TypeKind for DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION {
    type TypeKind = windows_core::CopyType;
}
impl Default for DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION {
    pub name: windows_core::PWSTR,
}
impl windows_core::TypeKind for DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION {
    type TypeKind = windows_core::CopyType;
}
impl Default for DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION {
    pub privacyTag: i32,
    pub name: windows_core::PWSTR,
    pub description: windows_core::PWSTR,
}
impl windows_core::TypeKind for DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION {
    type TypeKind = windows_core::CopyType;
}
impl Default for DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DIAGNOSTIC_DATA_EVENT_TAG_STATS {
    pub privacyTag: i32,
    pub eventCount: u32,
}
impl windows_core::TypeKind for DIAGNOSTIC_DATA_EVENT_TAG_STATS {
    type TypeKind = windows_core::CopyType;
}
impl Default for DIAGNOSTIC_DATA_EVENT_TAG_STATS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION {
    pub hoursOfHistoryToKeep: u32,
    pub maxStoreMegabytes: u32,
    pub requestedMaxStoreMegabytes: u32,
}
impl windows_core::TypeKind for DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIAGNOSTIC_DATA_GENERAL_STATS {
    pub optInLevel: u32,
    pub transcriptSizeBytes: u64,
    pub oldestEventTimestamp: u64,
    pub totalEventCountLast24Hours: u32,
    pub averageDailyEvents: f32,
}
impl windows_core::TypeKind for DIAGNOSTIC_DATA_GENERAL_STATS {
    type TypeKind = windows_core::CopyType;
}
impl Default for DIAGNOSTIC_DATA_GENERAL_STATS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    pub isCoreData: super::super::Foundation::BOOL,
    pub extra1: windows_core::PWSTR,
    pub extra2: windows_core::PWSTR,
    pub extra3: windows_core::PWSTR,
}
impl windows_core::TypeKind for DIAGNOSTIC_DATA_RECORD {
    type TypeKind = windows_core::CopyType;
}
impl Default for DIAGNOSTIC_DATA_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DIAGNOSTIC_DATA_SEARCH_CRITERIA {
    pub producerNames: *const windows_core::PCWSTR,
    pub producerNameCount: u32,
    pub textToMatch: windows_core::PCWSTR,
    pub categoryIds: *const i32,
    pub categoryIdCount: u32,
    pub privacyTags: *const i32,
    pub privacyTagCount: u32,
    pub coreDataOnly: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for DIAGNOSTIC_DATA_SEARCH_CRITERIA {
    type TypeKind = windows_core::CopyType;
}
impl Default for DIAGNOSTIC_DATA_SEARCH_CRITERIA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for DIAGNOSTIC_REPORT_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for DIAGNOSTIC_REPORT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DIAGNOSTIC_REPORT_PARAMETER {
    pub name: [u16; 129],
    pub value: [u16; 260],
}
impl windows_core::TypeKind for DIAGNOSTIC_REPORT_PARAMETER {
    type TypeKind = windows_core::CopyType;
}
impl Default for DIAGNOSTIC_REPORT_PARAMETER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DIAGNOSTIC_REPORT_SIGNATURE {
    pub eventName: [u16; 65],
    pub parameters: [DIAGNOSTIC_REPORT_PARAMETER; 10],
}
impl windows_core::TypeKind for DIAGNOSTIC_REPORT_SIGNATURE {
    type TypeKind = windows_core::CopyType;
}
impl Default for DIAGNOSTIC_REPORT_SIGNATURE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
