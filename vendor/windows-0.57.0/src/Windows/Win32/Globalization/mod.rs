#[inline]
pub unsafe fn AdjustCalendarDate(lpcaldatetime: *mut CALDATETIME, calunit: CALDATETIME_DATEUNIT, amount: i32) -> super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn AdjustCalendarDate(lpcaldatetime : *mut CALDATETIME, calunit : CALDATETIME_DATEUNIT, amount : i32) -> super::Foundation:: BOOL);
    AdjustCalendarDate(lpcaldatetime, calunit, amount)
}
#[inline]
pub unsafe fn CompareStringA(locale: u32, dwcmpflags: u32, lpstring1: &[i8], lpstring2: &[i8]) -> COMPARESTRING_RESULT {
    windows_targets::link!("kernel32.dll" "system" fn CompareStringA(locale : u32, dwcmpflags : u32, lpstring1 : *const i8, cchcount1 : i32, lpstring2 : *const i8, cchcount2 : i32) -> COMPARESTRING_RESULT);
    CompareStringA(locale, dwcmpflags, core::mem::transmute(lpstring1.as_ptr()), lpstring1.len().try_into().unwrap(), core::mem::transmute(lpstring2.as_ptr()), lpstring2.len().try_into().unwrap())
}
#[inline]
pub unsafe fn CompareStringEx<P0, P1>(lplocalename: P0, dwcmpflags: COMPARE_STRING_FLAGS, lpstring1: &[u16], lpstring2: &[u16], lpversioninformation: Option<*const NLSVERSIONINFO>, lpreserved: Option<*const core::ffi::c_void>, lparam: P1) -> COMPARESTRING_RESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::Foundation::LPARAM>,
{
    windows_targets::link!("kernel32.dll" "system" fn CompareStringEx(lplocalename : windows_core::PCWSTR, dwcmpflags : COMPARE_STRING_FLAGS, lpstring1 : windows_core::PCWSTR, cchcount1 : i32, lpstring2 : windows_core::PCWSTR, cchcount2 : i32, lpversioninformation : *const NLSVERSIONINFO, lpreserved : *const core::ffi::c_void, lparam : super::Foundation:: LPARAM) -> COMPARESTRING_RESULT);
    CompareStringEx(lplocalename.param().abi(), dwcmpflags, core::mem::transmute(lpstring1.as_ptr()), lpstring1.len().try_into().unwrap(), core::mem::transmute(lpstring2.as_ptr()), lpstring2.len().try_into().unwrap(), core::mem::transmute(lpversioninformation.unwrap_or(std::ptr::null())), core::mem::transmute(lpreserved.unwrap_or(std::ptr::null())), lparam.param().abi())
}
#[inline]
pub unsafe fn CompareStringOrdinal<P0>(lpstring1: &[u16], lpstring2: &[u16], bignorecase: P0) -> COMPARESTRING_RESULT
where
    P0: windows_core::Param<super::Foundation::BOOL>,
{
    windows_targets::link!("kernel32.dll" "system" fn CompareStringOrdinal(lpstring1 : windows_core::PCWSTR, cchcount1 : i32, lpstring2 : windows_core::PCWSTR, cchcount2 : i32, bignorecase : super::Foundation:: BOOL) -> COMPARESTRING_RESULT);
    CompareStringOrdinal(core::mem::transmute(lpstring1.as_ptr()), lpstring1.len().try_into().unwrap(), core::mem::transmute(lpstring2.as_ptr()), lpstring2.len().try_into().unwrap(), bignorecase.param().abi())
}
#[inline]
pub unsafe fn CompareStringW(locale: u32, dwcmpflags: u32, lpstring1: &[u16], lpstring2: &[u16]) -> COMPARESTRING_RESULT {
    windows_targets::link!("kernel32.dll" "system" fn CompareStringW(locale : u32, dwcmpflags : u32, lpstring1 : windows_core::PCWSTR, cchcount1 : i32, lpstring2 : windows_core::PCWSTR, cchcount2 : i32) -> COMPARESTRING_RESULT);
    CompareStringW(locale, dwcmpflags, core::mem::transmute(lpstring1.as_ptr()), lpstring1.len().try_into().unwrap(), core::mem::transmute(lpstring2.as_ptr()), lpstring2.len().try_into().unwrap())
}
#[inline]
pub unsafe fn ConvertCalDateTimeToSystemTime(lpcaldatetime: *const CALDATETIME, lpsystime: *mut super::Foundation::SYSTEMTIME) -> super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn ConvertCalDateTimeToSystemTime(lpcaldatetime : *const CALDATETIME, lpsystime : *mut super::Foundation:: SYSTEMTIME) -> super::Foundation:: BOOL);
    ConvertCalDateTimeToSystemTime(lpcaldatetime, lpsystime)
}
#[inline]
pub unsafe fn ConvertDefaultLocale(locale: u32) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn ConvertDefaultLocale(locale : u32) -> u32);
    ConvertDefaultLocale(locale)
}
#[inline]
pub unsafe fn ConvertSystemTimeToCalDateTime(lpsystime: *const super::Foundation::SYSTEMTIME, calid: u32, lpcaldatetime: *mut CALDATETIME) -> super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn ConvertSystemTimeToCalDateTime(lpsystime : *const super::Foundation:: SYSTEMTIME, calid : u32, lpcaldatetime : *mut CALDATETIME) -> super::Foundation:: BOOL);
    ConvertSystemTimeToCalDateTime(lpsystime, calid, lpcaldatetime)
}
#[inline]
pub unsafe fn EnumCalendarInfoA(lpcalinfoenumproc: CALINFO_ENUMPROCA, locale: u32, calendar: u32, caltype: u32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn EnumCalendarInfoA(lpcalinfoenumproc : CALINFO_ENUMPROCA, locale : u32, calendar : u32, caltype : u32) -> super::Foundation:: BOOL);
    EnumCalendarInfoA(lpcalinfoenumproc, locale, calendar, caltype).ok()
}
#[inline]
pub unsafe fn EnumCalendarInfoExA(lpcalinfoenumprocex: CALINFO_ENUMPROCEXA, locale: u32, calendar: u32, caltype: u32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn EnumCalendarInfoExA(lpcalinfoenumprocex : CALINFO_ENUMPROCEXA, locale : u32, calendar : u32, caltype : u32) -> super::Foundation:: BOOL);
    EnumCalendarInfoExA(lpcalinfoenumprocex, locale, calendar, caltype).ok()
}
#[inline]
pub unsafe fn EnumCalendarInfoExEx<P0, P1, P2>(pcalinfoenumprocexex: CALINFO_ENUMPROCEXEX, lplocalename: P0, calendar: u32, lpreserved: P1, caltype: u32, lparam: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::Foundation::LPARAM>,
{
    windows_targets::link!("kernel32.dll" "system" fn EnumCalendarInfoExEx(pcalinfoenumprocexex : CALINFO_ENUMPROCEXEX, lplocalename : windows_core::PCWSTR, calendar : u32, lpreserved : windows_core::PCWSTR, caltype : u32, lparam : super::Foundation:: LPARAM) -> super::Foundation:: BOOL);
    EnumCalendarInfoExEx(pcalinfoenumprocexex, lplocalename.param().abi(), calendar, lpreserved.param().abi(), caltype, lparam.param().abi()).ok()
}
#[inline]
pub unsafe fn EnumCalendarInfoExW(lpcalinfoenumprocex: CALINFO_ENUMPROCEXW, locale: u32, calendar: u32, caltype: u32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn EnumCalendarInfoExW(lpcalinfoenumprocex : CALINFO_ENUMPROCEXW, locale : u32, calendar : u32, caltype : u32) -> super::Foundation:: BOOL);
    EnumCalendarInfoExW(lpcalinfoenumprocex, locale, calendar, caltype).ok()
}
#[inline]
pub unsafe fn EnumCalendarInfoW(lpcalinfoenumproc: CALINFO_ENUMPROCW, locale: u32, calendar: u32, caltype: u32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn EnumCalendarInfoW(lpcalinfoenumproc : CALINFO_ENUMPROCW, locale : u32, calendar : u32, caltype : u32) -> super::Foundation:: BOOL);
    EnumCalendarInfoW(lpcalinfoenumproc, locale, calendar, caltype).ok()
}
#[inline]
pub unsafe fn EnumDateFormatsA(lpdatefmtenumproc: DATEFMT_ENUMPROCA, locale: u32, dwflags: u32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn EnumDateFormatsA(lpdatefmtenumproc : DATEFMT_ENUMPROCA, locale : u32, dwflags : u32) -> super::Foundation:: BOOL);
    EnumDateFormatsA(lpdatefmtenumproc, locale, dwflags).ok()
}
#[inline]
pub unsafe fn EnumDateFormatsExA(lpdatefmtenumprocex: DATEFMT_ENUMPROCEXA, locale: u32, dwflags: u32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn EnumDateFormatsExA(lpdatefmtenumprocex : DATEFMT_ENUMPROCEXA, locale : u32, dwflags : u32) -> super::Foundation:: BOOL);
    EnumDateFormatsExA(lpdatefmtenumprocex, locale, dwflags).ok()
}
#[inline]
pub unsafe fn EnumDateFormatsExEx<P0, P1>(lpdatefmtenumprocexex: DATEFMT_ENUMPROCEXEX, lplocalename: P0, dwflags: ENUM_DATE_FORMATS_FLAGS, lparam: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::Foundation::LPARAM>,
{
    windows_targets::link!("kernel32.dll" "system" fn EnumDateFormatsExEx(lpdatefmtenumprocexex : DATEFMT_ENUMPROCEXEX, lplocalename : windows_core::PCWSTR, dwflags : ENUM_DATE_FORMATS_FLAGS, lparam : super::Foundation:: LPARAM) -> super::Foundation:: BOOL);
    EnumDateFormatsExEx(lpdatefmtenumprocexex, lplocalename.param().abi(), dwflags, lparam.param().abi()).ok()
}
#[inline]
pub unsafe fn EnumDateFormatsExW(lpdatefmtenumprocex: DATEFMT_ENUMPROCEXW, locale: u32, dwflags: u32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn EnumDateFormatsExW(lpdatefmtenumprocex : DATEFMT_ENUMPROCEXW, locale : u32, dwflags : u32) -> super::Foundation:: BOOL);
    EnumDateFormatsExW(lpdatefmtenumprocex, locale, dwflags).ok()
}
#[inline]
pub unsafe fn EnumDateFormatsW(lpdatefmtenumproc: DATEFMT_ENUMPROCW, locale: u32, dwflags: u32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn EnumDateFormatsW(lpdatefmtenumproc : DATEFMT_ENUMPROCW, locale : u32, dwflags : u32) -> super::Foundation:: BOOL);
    EnumDateFormatsW(lpdatefmtenumproc, locale, dwflags).ok()
}
#[inline]
pub unsafe fn EnumLanguageGroupLocalesA(lplanggrouplocaleenumproc: LANGGROUPLOCALE_ENUMPROCA, languagegroup: u32, dwflags: u32, lparam: isize) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn EnumLanguageGroupLocalesA(lplanggrouplocaleenumproc : LANGGROUPLOCALE_ENUMPROCA, languagegroup : u32, dwflags : u32, lparam : isize) -> super::Foundation:: BOOL);
    EnumLanguageGroupLocalesA(lplanggrouplocaleenumproc, languagegroup, dwflags, lparam).ok()
}
#[inline]
pub unsafe fn EnumLanguageGroupLocalesW(lplanggrouplocaleenumproc: LANGGROUPLOCALE_ENUMPROCW, languagegroup: u32, dwflags: u32, lparam: isize) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn EnumLanguageGroupLocalesW(lplanggrouplocaleenumproc : LANGGROUPLOCALE_ENUMPROCW, languagegroup : u32, dwflags : u32, lparam : isize) -> super::Foundation:: BOOL);
    EnumLanguageGroupLocalesW(lplanggrouplocaleenumproc, languagegroup, dwflags, lparam).ok()
}
#[inline]
pub unsafe fn EnumSystemCodePagesA(lpcodepageenumproc: CODEPAGE_ENUMPROCA, dwflags: ENUM_SYSTEM_CODE_PAGES_FLAGS) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn EnumSystemCodePagesA(lpcodepageenumproc : CODEPAGE_ENUMPROCA, dwflags : ENUM_SYSTEM_CODE_PAGES_FLAGS) -> super::Foundation:: BOOL);
    EnumSystemCodePagesA(lpcodepageenumproc, dwflags).ok()
}
#[inline]
pub unsafe fn EnumSystemCodePagesW(lpcodepageenumproc: CODEPAGE_ENUMPROCW, dwflags: ENUM_SYSTEM_CODE_PAGES_FLAGS) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn EnumSystemCodePagesW(lpcodepageenumproc : CODEPAGE_ENUMPROCW, dwflags : ENUM_SYSTEM_CODE_PAGES_FLAGS) -> super::Foundation:: BOOL);
    EnumSystemCodePagesW(lpcodepageenumproc, dwflags).ok()
}
#[inline]
pub unsafe fn EnumSystemGeoID(geoclass: u32, parentgeoid: i32, lpgeoenumproc: GEO_ENUMPROC) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn EnumSystemGeoID(geoclass : u32, parentgeoid : i32, lpgeoenumproc : GEO_ENUMPROC) -> super::Foundation:: BOOL);
    EnumSystemGeoID(geoclass, parentgeoid, lpgeoenumproc).ok()
}
#[inline]
pub unsafe fn EnumSystemGeoNames<P0>(geoclass: u32, geoenumproc: GEO_ENUMNAMEPROC, data: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Foundation::LPARAM>,
{
    windows_targets::link!("kernel32.dll" "system" fn EnumSystemGeoNames(geoclass : u32, geoenumproc : GEO_ENUMNAMEPROC, data : super::Foundation:: LPARAM) -> super::Foundation:: BOOL);
    EnumSystemGeoNames(geoclass, geoenumproc, data.param().abi()).ok()
}
#[inline]
pub unsafe fn EnumSystemLanguageGroupsA(lplanguagegroupenumproc: LANGUAGEGROUP_ENUMPROCA, dwflags: ENUM_SYSTEM_LANGUAGE_GROUPS_FLAGS, lparam: isize) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn EnumSystemLanguageGroupsA(lplanguagegroupenumproc : LANGUAGEGROUP_ENUMPROCA, dwflags : ENUM_SYSTEM_LANGUAGE_GROUPS_FLAGS, lparam : isize) -> super::Foundation:: BOOL);
    EnumSystemLanguageGroupsA(lplanguagegroupenumproc, dwflags, lparam).ok()
}
#[inline]
pub unsafe fn EnumSystemLanguageGroupsW(lplanguagegroupenumproc: LANGUAGEGROUP_ENUMPROCW, dwflags: ENUM_SYSTEM_LANGUAGE_GROUPS_FLAGS, lparam: isize) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn EnumSystemLanguageGroupsW(lplanguagegroupenumproc : LANGUAGEGROUP_ENUMPROCW, dwflags : ENUM_SYSTEM_LANGUAGE_GROUPS_FLAGS, lparam : isize) -> super::Foundation:: BOOL);
    EnumSystemLanguageGroupsW(lplanguagegroupenumproc, dwflags, lparam).ok()
}
#[inline]
pub unsafe fn EnumSystemLocalesA(lplocaleenumproc: LOCALE_ENUMPROCA, dwflags: u32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn EnumSystemLocalesA(lplocaleenumproc : LOCALE_ENUMPROCA, dwflags : u32) -> super::Foundation:: BOOL);
    EnumSystemLocalesA(lplocaleenumproc, dwflags).ok()
}
#[inline]
pub unsafe fn EnumSystemLocalesEx<P0>(lplocaleenumprocex: LOCALE_ENUMPROCEX, dwflags: u32, lparam: P0, lpreserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Foundation::LPARAM>,
{
    windows_targets::link!("kernel32.dll" "system" fn EnumSystemLocalesEx(lplocaleenumprocex : LOCALE_ENUMPROCEX, dwflags : u32, lparam : super::Foundation:: LPARAM, lpreserved : *const core::ffi::c_void) -> super::Foundation:: BOOL);
    EnumSystemLocalesEx(lplocaleenumprocex, dwflags, lparam.param().abi(), core::mem::transmute(lpreserved.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn EnumSystemLocalesW(lplocaleenumproc: LOCALE_ENUMPROCW, dwflags: u32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn EnumSystemLocalesW(lplocaleenumproc : LOCALE_ENUMPROCW, dwflags : u32) -> super::Foundation:: BOOL);
    EnumSystemLocalesW(lplocaleenumproc, dwflags).ok()
}
#[inline]
pub unsafe fn EnumTimeFormatsA(lptimefmtenumproc: TIMEFMT_ENUMPROCA, locale: u32, dwflags: TIME_FORMAT_FLAGS) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn EnumTimeFormatsA(lptimefmtenumproc : TIMEFMT_ENUMPROCA, locale : u32, dwflags : TIME_FORMAT_FLAGS) -> super::Foundation:: BOOL);
    EnumTimeFormatsA(lptimefmtenumproc, locale, dwflags).ok()
}
#[inline]
pub unsafe fn EnumTimeFormatsEx<P0, P1>(lptimefmtenumprocex: TIMEFMT_ENUMPROCEX, lplocalename: P0, dwflags: u32, lparam: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::Foundation::LPARAM>,
{
    windows_targets::link!("kernel32.dll" "system" fn EnumTimeFormatsEx(lptimefmtenumprocex : TIMEFMT_ENUMPROCEX, lplocalename : windows_core::PCWSTR, dwflags : u32, lparam : super::Foundation:: LPARAM) -> super::Foundation:: BOOL);
    EnumTimeFormatsEx(lptimefmtenumprocex, lplocalename.param().abi(), dwflags, lparam.param().abi()).ok()
}
#[inline]
pub unsafe fn EnumTimeFormatsW(lptimefmtenumproc: TIMEFMT_ENUMPROCW, locale: u32, dwflags: TIME_FORMAT_FLAGS) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn EnumTimeFormatsW(lptimefmtenumproc : TIMEFMT_ENUMPROCW, locale : u32, dwflags : TIME_FORMAT_FLAGS) -> super::Foundation:: BOOL);
    EnumTimeFormatsW(lptimefmtenumproc, locale, dwflags).ok()
}
#[inline]
pub unsafe fn EnumUILanguagesA(lpuilanguageenumproc: UILANGUAGE_ENUMPROCA, dwflags: u32, lparam: isize) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn EnumUILanguagesA(lpuilanguageenumproc : UILANGUAGE_ENUMPROCA, dwflags : u32, lparam : isize) -> super::Foundation:: BOOL);
    EnumUILanguagesA(lpuilanguageenumproc, dwflags, lparam).ok()
}
#[inline]
pub unsafe fn EnumUILanguagesW(lpuilanguageenumproc: UILANGUAGE_ENUMPROCW, dwflags: u32, lparam: isize) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn EnumUILanguagesW(lpuilanguageenumproc : UILANGUAGE_ENUMPROCW, dwflags : u32, lparam : isize) -> super::Foundation:: BOOL);
    EnumUILanguagesW(lpuilanguageenumproc, dwflags, lparam).ok()
}
#[inline]
pub unsafe fn FindNLSString(locale: u32, dwfindnlsstringflags: u32, lpstringsource: &[u16], lpstringvalue: &[u16], pcchfound: Option<*mut i32>) -> i32 {
    windows_targets::link!("kernel32.dll" "system" fn FindNLSString(locale : u32, dwfindnlsstringflags : u32, lpstringsource : windows_core::PCWSTR, cchsource : i32, lpstringvalue : windows_core::PCWSTR, cchvalue : i32, pcchfound : *mut i32) -> i32);
    FindNLSString(locale, dwfindnlsstringflags, core::mem::transmute(lpstringsource.as_ptr()), lpstringsource.len().try_into().unwrap(), core::mem::transmute(lpstringvalue.as_ptr()), lpstringvalue.len().try_into().unwrap(), core::mem::transmute(pcchfound.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn FindNLSStringEx<P0, P1>(lplocalename: P0, dwfindnlsstringflags: u32, lpstringsource: &[u16], lpstringvalue: &[u16], pcchfound: Option<*mut i32>, lpversioninformation: Option<*const NLSVERSIONINFO>, lpreserved: Option<*const core::ffi::c_void>, sorthandle: P1) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::Foundation::LPARAM>,
{
    windows_targets::link!("kernel32.dll" "system" fn FindNLSStringEx(lplocalename : windows_core::PCWSTR, dwfindnlsstringflags : u32, lpstringsource : windows_core::PCWSTR, cchsource : i32, lpstringvalue : windows_core::PCWSTR, cchvalue : i32, pcchfound : *mut i32, lpversioninformation : *const NLSVERSIONINFO, lpreserved : *const core::ffi::c_void, sorthandle : super::Foundation:: LPARAM) -> i32);
    FindNLSStringEx(lplocalename.param().abi(), dwfindnlsstringflags, core::mem::transmute(lpstringsource.as_ptr()), lpstringsource.len().try_into().unwrap(), core::mem::transmute(lpstringvalue.as_ptr()), lpstringvalue.len().try_into().unwrap(), core::mem::transmute(pcchfound.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpversioninformation.unwrap_or(std::ptr::null())), core::mem::transmute(lpreserved.unwrap_or(std::ptr::null())), sorthandle.param().abi())
}
#[inline]
pub unsafe fn FindStringOrdinal<P0>(dwfindstringordinalflags: u32, lpstringsource: &[u16], lpstringvalue: &[u16], bignorecase: P0) -> i32
where
    P0: windows_core::Param<super::Foundation::BOOL>,
{
    windows_targets::link!("kernel32.dll" "system" fn FindStringOrdinal(dwfindstringordinalflags : u32, lpstringsource : windows_core::PCWSTR, cchsource : i32, lpstringvalue : windows_core::PCWSTR, cchvalue : i32, bignorecase : super::Foundation:: BOOL) -> i32);
    FindStringOrdinal(dwfindstringordinalflags, core::mem::transmute(lpstringsource.as_ptr()), lpstringsource.len().try_into().unwrap(), core::mem::transmute(lpstringvalue.as_ptr()), lpstringvalue.len().try_into().unwrap(), bignorecase.param().abi())
}
#[inline]
pub unsafe fn FoldStringA(dwmapflags: FOLD_STRING_MAP_FLAGS, lpsrcstr: &[u8], lpdeststr: Option<&mut [u8]>) -> i32 {
    windows_targets::link!("kernel32.dll" "system" fn FoldStringA(dwmapflags : FOLD_STRING_MAP_FLAGS, lpsrcstr : windows_core::PCSTR, cchsrc : i32, lpdeststr : windows_core::PSTR, cchdest : i32) -> i32);
    FoldStringA(dwmapflags, core::mem::transmute(lpsrcstr.as_ptr()), lpsrcstr.len().try_into().unwrap(), core::mem::transmute(lpdeststr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpdeststr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn FoldStringW(dwmapflags: FOLD_STRING_MAP_FLAGS, lpsrcstr: &[u16], lpdeststr: Option<&mut [u16]>) -> i32 {
    windows_targets::link!("kernel32.dll" "system" fn FoldStringW(dwmapflags : FOLD_STRING_MAP_FLAGS, lpsrcstr : windows_core::PCWSTR, cchsrc : i32, lpdeststr : windows_core::PWSTR, cchdest : i32) -> i32);
    FoldStringW(dwmapflags, core::mem::transmute(lpsrcstr.as_ptr()), lpsrcstr.len().try_into().unwrap(), core::mem::transmute(lpdeststr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpdeststr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn GetACP() -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn GetACP() -> u32);
    GetACP()
}
#[inline]
pub unsafe fn GetCPInfo(codepage: u32, lpcpinfo: *mut CPINFO) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn GetCPInfo(codepage : u32, lpcpinfo : *mut CPINFO) -> super::Foundation:: BOOL);
    GetCPInfo(codepage, lpcpinfo).ok()
}
#[inline]
pub unsafe fn GetCPInfoExA(codepage: u32, dwflags: u32, lpcpinfoex: *mut CPINFOEXA) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn GetCPInfoExA(codepage : u32, dwflags : u32, lpcpinfoex : *mut CPINFOEXA) -> super::Foundation:: BOOL);
    GetCPInfoExA(codepage, dwflags, lpcpinfoex).ok()
}
#[inline]
pub unsafe fn GetCPInfoExW(codepage: u32, dwflags: u32, lpcpinfoex: *mut CPINFOEXW) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn GetCPInfoExW(codepage : u32, dwflags : u32, lpcpinfoex : *mut CPINFOEXW) -> super::Foundation:: BOOL);
    GetCPInfoExW(codepage, dwflags, lpcpinfoex).ok()
}
#[inline]
pub unsafe fn GetCalendarDateFormatEx<P0, P1>(lpszlocale: P0, dwflags: u32, lpcaldatetime: *const CALDATETIME, lpformat: P1, lpdatestr: windows_core::PWSTR, cchdate: i32) -> super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetCalendarDateFormatEx(lpszlocale : windows_core::PCWSTR, dwflags : u32, lpcaldatetime : *const CALDATETIME, lpformat : windows_core::PCWSTR, lpdatestr : windows_core::PWSTR, cchdate : i32) -> super::Foundation:: BOOL);
    GetCalendarDateFormatEx(lpszlocale.param().abi(), dwflags, lpcaldatetime, lpformat.param().abi(), core::mem::transmute(lpdatestr), cchdate)
}
#[inline]
pub unsafe fn GetCalendarInfoA(locale: u32, calendar: u32, caltype: u32, lpcaldata: Option<&mut [u8]>, lpvalue: Option<*mut u32>) -> i32 {
    windows_targets::link!("kernel32.dll" "system" fn GetCalendarInfoA(locale : u32, calendar : u32, caltype : u32, lpcaldata : windows_core::PSTR, cchdata : i32, lpvalue : *mut u32) -> i32);
    GetCalendarInfoA(locale, calendar, caltype, core::mem::transmute(lpcaldata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpcaldata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lpvalue.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn GetCalendarInfoEx<P0, P1>(lplocalename: P0, calendar: u32, lpreserved: P1, caltype: u32, lpcaldata: Option<&mut [u16]>, lpvalue: Option<*mut u32>) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetCalendarInfoEx(lplocalename : windows_core::PCWSTR, calendar : u32, lpreserved : windows_core::PCWSTR, caltype : u32, lpcaldata : windows_core::PWSTR, cchdata : i32, lpvalue : *mut u32) -> i32);
    GetCalendarInfoEx(lplocalename.param().abi(), calendar, lpreserved.param().abi(), caltype, core::mem::transmute(lpcaldata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpcaldata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lpvalue.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn GetCalendarInfoW(locale: u32, calendar: u32, caltype: u32, lpcaldata: Option<&mut [u16]>, lpvalue: Option<*mut u32>) -> i32 {
    windows_targets::link!("kernel32.dll" "system" fn GetCalendarInfoW(locale : u32, calendar : u32, caltype : u32, lpcaldata : windows_core::PWSTR, cchdata : i32, lpvalue : *mut u32) -> i32);
    GetCalendarInfoW(locale, calendar, caltype, core::mem::transmute(lpcaldata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpcaldata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lpvalue.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn GetCalendarSupportedDateRange(calendar: u32, lpcalmindatetime: *mut CALDATETIME, lpcalmaxdatetime: *mut CALDATETIME) -> super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn GetCalendarSupportedDateRange(calendar : u32, lpcalmindatetime : *mut CALDATETIME, lpcalmaxdatetime : *mut CALDATETIME) -> super::Foundation:: BOOL);
    GetCalendarSupportedDateRange(calendar, lpcalmindatetime, lpcalmaxdatetime)
}
#[inline]
pub unsafe fn GetCurrencyFormatA<P0>(locale: u32, dwflags: u32, lpvalue: P0, lpformat: Option<*const CURRENCYFMTA>, lpcurrencystr: Option<&mut [u8]>) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetCurrencyFormatA(locale : u32, dwflags : u32, lpvalue : windows_core::PCSTR, lpformat : *const CURRENCYFMTA, lpcurrencystr : windows_core::PSTR, cchcurrency : i32) -> i32);
    GetCurrencyFormatA(locale, dwflags, lpvalue.param().abi(), core::mem::transmute(lpformat.unwrap_or(std::ptr::null())), core::mem::transmute(lpcurrencystr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpcurrencystr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn GetCurrencyFormatEx<P0, P1>(lplocalename: P0, dwflags: u32, lpvalue: P1, lpformat: Option<*const CURRENCYFMTW>, lpcurrencystr: Option<&mut [u16]>) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetCurrencyFormatEx(lplocalename : windows_core::PCWSTR, dwflags : u32, lpvalue : windows_core::PCWSTR, lpformat : *const CURRENCYFMTW, lpcurrencystr : windows_core::PWSTR, cchcurrency : i32) -> i32);
    GetCurrencyFormatEx(lplocalename.param().abi(), dwflags, lpvalue.param().abi(), core::mem::transmute(lpformat.unwrap_or(std::ptr::null())), core::mem::transmute(lpcurrencystr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpcurrencystr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn GetCurrencyFormatW<P0>(locale: u32, dwflags: u32, lpvalue: P0, lpformat: Option<*const CURRENCYFMTW>, lpcurrencystr: Option<&mut [u16]>) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetCurrencyFormatW(locale : u32, dwflags : u32, lpvalue : windows_core::PCWSTR, lpformat : *const CURRENCYFMTW, lpcurrencystr : windows_core::PWSTR, cchcurrency : i32) -> i32);
    GetCurrencyFormatW(locale, dwflags, lpvalue.param().abi(), core::mem::transmute(lpformat.unwrap_or(std::ptr::null())), core::mem::transmute(lpcurrencystr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpcurrencystr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn GetDateFormatA<P0>(locale: u32, dwflags: u32, lpdate: Option<*const super::Foundation::SYSTEMTIME>, lpformat: P0, lpdatestr: Option<&mut [u8]>) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetDateFormatA(locale : u32, dwflags : u32, lpdate : *const super::Foundation:: SYSTEMTIME, lpformat : windows_core::PCSTR, lpdatestr : windows_core::PSTR, cchdate : i32) -> i32);
    GetDateFormatA(locale, dwflags, core::mem::transmute(lpdate.unwrap_or(std::ptr::null())), lpformat.param().abi(), core::mem::transmute(lpdatestr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpdatestr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn GetDateFormatEx<P0, P1, P2>(lplocalename: P0, dwflags: ENUM_DATE_FORMATS_FLAGS, lpdate: Option<*const super::Foundation::SYSTEMTIME>, lpformat: P1, lpdatestr: Option<&mut [u16]>, lpcalendar: P2) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetDateFormatEx(lplocalename : windows_core::PCWSTR, dwflags : ENUM_DATE_FORMATS_FLAGS, lpdate : *const super::Foundation:: SYSTEMTIME, lpformat : windows_core::PCWSTR, lpdatestr : windows_core::PWSTR, cchdate : i32, lpcalendar : windows_core::PCWSTR) -> i32);
    GetDateFormatEx(lplocalename.param().abi(), dwflags, core::mem::transmute(lpdate.unwrap_or(std::ptr::null())), lpformat.param().abi(), core::mem::transmute(lpdatestr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpdatestr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), lpcalendar.param().abi())
}
#[inline]
pub unsafe fn GetDateFormatW<P0>(locale: u32, dwflags: u32, lpdate: Option<*const super::Foundation::SYSTEMTIME>, lpformat: P0, lpdatestr: Option<&mut [u16]>) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetDateFormatW(locale : u32, dwflags : u32, lpdate : *const super::Foundation:: SYSTEMTIME, lpformat : windows_core::PCWSTR, lpdatestr : windows_core::PWSTR, cchdate : i32) -> i32);
    GetDateFormatW(locale, dwflags, core::mem::transmute(lpdate.unwrap_or(std::ptr::null())), lpformat.param().abi(), core::mem::transmute(lpdatestr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpdatestr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn GetDistanceOfClosestLanguageInList<P0, P1>(pszlanguage: P0, pszlanguageslist: P1, wchlistdelimiter: u16) -> windows_core::Result<f64>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("bcp47mrm.dll" "system" fn GetDistanceOfClosestLanguageInList(pszlanguage : windows_core::PCWSTR, pszlanguageslist : windows_core::PCWSTR, wchlistdelimiter : u16, pclosestdistance : *mut f64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    GetDistanceOfClosestLanguageInList(pszlanguage.param().abi(), pszlanguageslist.param().abi(), wchlistdelimiter, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn GetDurationFormat<P0>(locale: u32, dwflags: u32, lpduration: Option<*const super::Foundation::SYSTEMTIME>, ullduration: u64, lpformat: P0, lpdurationstr: Option<&mut [u16]>) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetDurationFormat(locale : u32, dwflags : u32, lpduration : *const super::Foundation:: SYSTEMTIME, ullduration : u64, lpformat : windows_core::PCWSTR, lpdurationstr : windows_core::PWSTR, cchduration : i32) -> i32);
    GetDurationFormat(locale, dwflags, core::mem::transmute(lpduration.unwrap_or(std::ptr::null())), ullduration, lpformat.param().abi(), core::mem::transmute(lpdurationstr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpdurationstr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn GetDurationFormatEx<P0, P1>(lplocalename: P0, dwflags: u32, lpduration: Option<*const super::Foundation::SYSTEMTIME>, ullduration: u64, lpformat: P1, lpdurationstr: Option<&mut [u16]>) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetDurationFormatEx(lplocalename : windows_core::PCWSTR, dwflags : u32, lpduration : *const super::Foundation:: SYSTEMTIME, ullduration : u64, lpformat : windows_core::PCWSTR, lpdurationstr : windows_core::PWSTR, cchduration : i32) -> i32);
    GetDurationFormatEx(lplocalename.param().abi(), dwflags, core::mem::transmute(lpduration.unwrap_or(std::ptr::null())), ullduration, lpformat.param().abi(), core::mem::transmute(lpdurationstr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpdurationstr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn GetFileMUIInfo<P0>(dwflags: u32, pcwszfilepath: P0, pfilemuiinfo: Option<*mut FILEMUIINFO>, pcbfilemuiinfo: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetFileMUIInfo(dwflags : u32, pcwszfilepath : windows_core::PCWSTR, pfilemuiinfo : *mut FILEMUIINFO, pcbfilemuiinfo : *mut u32) -> super::Foundation:: BOOL);
    GetFileMUIInfo(dwflags, pcwszfilepath.param().abi(), core::mem::transmute(pfilemuiinfo.unwrap_or(std::ptr::null_mut())), pcbfilemuiinfo).ok()
}
#[inline]
pub unsafe fn GetFileMUIPath<P0>(dwflags: u32, pcwszfilepath: P0, pwszlanguage: windows_core::PWSTR, pcchlanguage: *mut u32, pwszfilemuipath: windows_core::PWSTR, pcchfilemuipath: *mut u32, pululenumerator: *mut u64) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetFileMUIPath(dwflags : u32, pcwszfilepath : windows_core::PCWSTR, pwszlanguage : windows_core::PWSTR, pcchlanguage : *mut u32, pwszfilemuipath : windows_core::PWSTR, pcchfilemuipath : *mut u32, pululenumerator : *mut u64) -> super::Foundation:: BOOL);
    GetFileMUIPath(dwflags, pcwszfilepath.param().abi(), core::mem::transmute(pwszlanguage), pcchlanguage, core::mem::transmute(pwszfilemuipath), pcchfilemuipath, pululenumerator).ok()
}
#[inline]
pub unsafe fn GetGeoInfoA(location: i32, geotype: SYSGEOTYPE, lpgeodata: Option<&mut [u8]>, langid: u16) -> i32 {
    windows_targets::link!("kernel32.dll" "system" fn GetGeoInfoA(location : i32, geotype : SYSGEOTYPE, lpgeodata : windows_core::PSTR, cchdata : i32, langid : u16) -> i32);
    GetGeoInfoA(location, geotype, core::mem::transmute(lpgeodata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpgeodata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), langid)
}
#[inline]
pub unsafe fn GetGeoInfoEx<P0>(location: P0, geotype: SYSGEOTYPE, geodata: Option<&mut [u16]>) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetGeoInfoEx(location : windows_core::PCWSTR, geotype : SYSGEOTYPE, geodata : windows_core::PWSTR, geodatacount : i32) -> i32);
    GetGeoInfoEx(location.param().abi(), geotype, core::mem::transmute(geodata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), geodata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn GetGeoInfoW(location: i32, geotype: SYSGEOTYPE, lpgeodata: Option<&mut [u16]>, langid: u16) -> i32 {
    windows_targets::link!("kernel32.dll" "system" fn GetGeoInfoW(location : i32, geotype : SYSGEOTYPE, lpgeodata : windows_core::PWSTR, cchdata : i32, langid : u16) -> i32);
    GetGeoInfoW(location, geotype, core::mem::transmute(lpgeodata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpgeodata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), langid)
}
#[inline]
pub unsafe fn GetLocaleInfoA(locale: u32, lctype: u32, lplcdata: Option<&mut [u8]>) -> i32 {
    windows_targets::link!("kernel32.dll" "system" fn GetLocaleInfoA(locale : u32, lctype : u32, lplcdata : windows_core::PSTR, cchdata : i32) -> i32);
    GetLocaleInfoA(locale, lctype, core::mem::transmute(lplcdata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lplcdata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn GetLocaleInfoEx<P0>(lplocalename: P0, lctype: u32, lplcdata: Option<&mut [u16]>) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetLocaleInfoEx(lplocalename : windows_core::PCWSTR, lctype : u32, lplcdata : windows_core::PWSTR, cchdata : i32) -> i32);
    GetLocaleInfoEx(lplocalename.param().abi(), lctype, core::mem::transmute(lplcdata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lplcdata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn GetLocaleInfoW(locale: u32, lctype: u32, lplcdata: Option<&mut [u16]>) -> i32 {
    windows_targets::link!("kernel32.dll" "system" fn GetLocaleInfoW(locale : u32, lctype : u32, lplcdata : windows_core::PWSTR, cchdata : i32) -> i32);
    GetLocaleInfoW(locale, lctype, core::mem::transmute(lplcdata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lplcdata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn GetNLSVersion(function: u32, locale: u32, lpversioninformation: *mut NLSVERSIONINFO) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn GetNLSVersion(function : u32, locale : u32, lpversioninformation : *mut NLSVERSIONINFO) -> super::Foundation:: BOOL);
    GetNLSVersion(function, locale, lpversioninformation).ok()
}
#[inline]
pub unsafe fn GetNLSVersionEx<P0>(function: u32, lplocalename: P0, lpversioninformation: *mut NLSVERSIONINFOEX) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetNLSVersionEx(function : u32, lplocalename : windows_core::PCWSTR, lpversioninformation : *mut NLSVERSIONINFOEX) -> super::Foundation:: BOOL);
    GetNLSVersionEx(function, lplocalename.param().abi(), lpversioninformation).ok()
}
#[inline]
pub unsafe fn GetNumberFormatA<P0>(locale: u32, dwflags: u32, lpvalue: P0, lpformat: Option<*const NUMBERFMTA>, lpnumberstr: Option<&mut [u8]>) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetNumberFormatA(locale : u32, dwflags : u32, lpvalue : windows_core::PCSTR, lpformat : *const NUMBERFMTA, lpnumberstr : windows_core::PSTR, cchnumber : i32) -> i32);
    GetNumberFormatA(locale, dwflags, lpvalue.param().abi(), core::mem::transmute(lpformat.unwrap_or(std::ptr::null())), core::mem::transmute(lpnumberstr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpnumberstr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn GetNumberFormatEx<P0, P1>(lplocalename: P0, dwflags: u32, lpvalue: P1, lpformat: Option<*const NUMBERFMTW>, lpnumberstr: Option<&mut [u16]>) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetNumberFormatEx(lplocalename : windows_core::PCWSTR, dwflags : u32, lpvalue : windows_core::PCWSTR, lpformat : *const NUMBERFMTW, lpnumberstr : windows_core::PWSTR, cchnumber : i32) -> i32);
    GetNumberFormatEx(lplocalename.param().abi(), dwflags, lpvalue.param().abi(), core::mem::transmute(lpformat.unwrap_or(std::ptr::null())), core::mem::transmute(lpnumberstr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpnumberstr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn GetNumberFormatW<P0>(locale: u32, dwflags: u32, lpvalue: P0, lpformat: Option<*const NUMBERFMTW>, lpnumberstr: Option<&mut [u16]>) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetNumberFormatW(locale : u32, dwflags : u32, lpvalue : windows_core::PCWSTR, lpformat : *const NUMBERFMTW, lpnumberstr : windows_core::PWSTR, cchnumber : i32) -> i32);
    GetNumberFormatW(locale, dwflags, lpvalue.param().abi(), core::mem::transmute(lpformat.unwrap_or(std::ptr::null())), core::mem::transmute(lpnumberstr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpnumberstr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn GetOEMCP() -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn GetOEMCP() -> u32);
    GetOEMCP()
}
#[inline]
pub unsafe fn GetProcessPreferredUILanguages(dwflags: u32, pulnumlanguages: *mut u32, pwszlanguagesbuffer: windows_core::PWSTR, pcchlanguagesbuffer: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn GetProcessPreferredUILanguages(dwflags : u32, pulnumlanguages : *mut u32, pwszlanguagesbuffer : windows_core::PWSTR, pcchlanguagesbuffer : *mut u32) -> super::Foundation:: BOOL);
    GetProcessPreferredUILanguages(dwflags, pulnumlanguages, core::mem::transmute(pwszlanguagesbuffer), pcchlanguagesbuffer).ok()
}
#[inline]
pub unsafe fn GetStringScripts<P0>(dwflags: u32, lpstring: P0, cchstring: i32, lpscripts: Option<&mut [u16]>) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetStringScripts(dwflags : u32, lpstring : windows_core::PCWSTR, cchstring : i32, lpscripts : windows_core::PWSTR, cchscripts : i32) -> i32);
    GetStringScripts(dwflags, lpstring.param().abi(), cchstring, core::mem::transmute(lpscripts.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpscripts.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn GetStringTypeA(locale: u32, dwinfotype: u32, lpsrcstr: &[u8], lpchartype: *mut u16) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn GetStringTypeA(locale : u32, dwinfotype : u32, lpsrcstr : windows_core::PCSTR, cchsrc : i32, lpchartype : *mut u16) -> super::Foundation:: BOOL);
    GetStringTypeA(locale, dwinfotype, core::mem::transmute(lpsrcstr.as_ptr()), lpsrcstr.len().try_into().unwrap(), lpchartype).ok()
}
#[inline]
pub unsafe fn GetStringTypeExA<P0>(locale: u32, dwinfotype: u32, lpsrcstr: P0, cchsrc: i32, lpchartype: *mut u16) -> super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetStringTypeExA(locale : u32, dwinfotype : u32, lpsrcstr : windows_core::PCSTR, cchsrc : i32, lpchartype : *mut u16) -> super::Foundation:: BOOL);
    GetStringTypeExA(locale, dwinfotype, lpsrcstr.param().abi(), cchsrc, lpchartype)
}
#[inline]
pub unsafe fn GetStringTypeExW<P0>(locale: u32, dwinfotype: u32, lpsrcstr: P0, cchsrc: i32, lpchartype: *mut u16) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetStringTypeExW(locale : u32, dwinfotype : u32, lpsrcstr : windows_core::PCWSTR, cchsrc : i32, lpchartype : *mut u16) -> super::Foundation:: BOOL);
    GetStringTypeExW(locale, dwinfotype, lpsrcstr.param().abi(), cchsrc, lpchartype).ok()
}
#[inline]
pub unsafe fn GetStringTypeW(dwinfotype: u32, lpsrcstr: &[u16], lpchartype: *mut u16) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn GetStringTypeW(dwinfotype : u32, lpsrcstr : windows_core::PCWSTR, cchsrc : i32, lpchartype : *mut u16) -> super::Foundation:: BOOL);
    GetStringTypeW(dwinfotype, core::mem::transmute(lpsrcstr.as_ptr()), lpsrcstr.len().try_into().unwrap(), lpchartype).ok()
}
#[inline]
pub unsafe fn GetSystemDefaultLCID() -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn GetSystemDefaultLCID() -> u32);
    GetSystemDefaultLCID()
}
#[inline]
pub unsafe fn GetSystemDefaultLangID() -> u16 {
    windows_targets::link!("kernel32.dll" "system" fn GetSystemDefaultLangID() -> u16);
    GetSystemDefaultLangID()
}
#[inline]
pub unsafe fn GetSystemDefaultLocaleName(lplocalename: &mut [u16]) -> i32 {
    windows_targets::link!("kernel32.dll" "system" fn GetSystemDefaultLocaleName(lplocalename : windows_core::PWSTR, cchlocalename : i32) -> i32);
    GetSystemDefaultLocaleName(core::mem::transmute(lplocalename.as_ptr()), lplocalename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn GetSystemDefaultUILanguage() -> u16 {
    windows_targets::link!("kernel32.dll" "system" fn GetSystemDefaultUILanguage() -> u16);
    GetSystemDefaultUILanguage()
}
#[inline]
pub unsafe fn GetSystemPreferredUILanguages(dwflags: u32, pulnumlanguages: *mut u32, pwszlanguagesbuffer: windows_core::PWSTR, pcchlanguagesbuffer: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn GetSystemPreferredUILanguages(dwflags : u32, pulnumlanguages : *mut u32, pwszlanguagesbuffer : windows_core::PWSTR, pcchlanguagesbuffer : *mut u32) -> super::Foundation:: BOOL);
    GetSystemPreferredUILanguages(dwflags, pulnumlanguages, core::mem::transmute(pwszlanguagesbuffer), pcchlanguagesbuffer).ok()
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetTextCharset<P0>(hdc: P0) -> i32
where
    P0: windows_core::Param<super::Graphics::Gdi::HDC>,
{
    windows_targets::link!("gdi32.dll" "system" fn GetTextCharset(hdc : super::Graphics::Gdi:: HDC) -> i32);
    GetTextCharset(hdc.param().abi())
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetTextCharsetInfo<P0>(hdc: P0, lpsig: Option<*mut FONTSIGNATURE>, dwflags: u32) -> i32
where
    P0: windows_core::Param<super::Graphics::Gdi::HDC>,
{
    windows_targets::link!("gdi32.dll" "system" fn GetTextCharsetInfo(hdc : super::Graphics::Gdi:: HDC, lpsig : *mut FONTSIGNATURE, dwflags : u32) -> i32);
    GetTextCharsetInfo(hdc.param().abi(), core::mem::transmute(lpsig.unwrap_or(std::ptr::null_mut())), dwflags)
}
#[inline]
pub unsafe fn GetThreadLocale() -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn GetThreadLocale() -> u32);
    GetThreadLocale()
}
#[inline]
pub unsafe fn GetThreadPreferredUILanguages(dwflags: u32, pulnumlanguages: *mut u32, pwszlanguagesbuffer: windows_core::PWSTR, pcchlanguagesbuffer: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn GetThreadPreferredUILanguages(dwflags : u32, pulnumlanguages : *mut u32, pwszlanguagesbuffer : windows_core::PWSTR, pcchlanguagesbuffer : *mut u32) -> super::Foundation:: BOOL);
    GetThreadPreferredUILanguages(dwflags, pulnumlanguages, core::mem::transmute(pwszlanguagesbuffer), pcchlanguagesbuffer).ok()
}
#[inline]
pub unsafe fn GetThreadUILanguage() -> u16 {
    windows_targets::link!("kernel32.dll" "system" fn GetThreadUILanguage() -> u16);
    GetThreadUILanguage()
}
#[inline]
pub unsafe fn GetTimeFormatA<P0>(locale: u32, dwflags: u32, lptime: Option<*const super::Foundation::SYSTEMTIME>, lpformat: P0, lptimestr: Option<&mut [u8]>) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetTimeFormatA(locale : u32, dwflags : u32, lptime : *const super::Foundation:: SYSTEMTIME, lpformat : windows_core::PCSTR, lptimestr : windows_core::PSTR, cchtime : i32) -> i32);
    GetTimeFormatA(locale, dwflags, core::mem::transmute(lptime.unwrap_or(std::ptr::null())), lpformat.param().abi(), core::mem::transmute(lptimestr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lptimestr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn GetTimeFormatEx<P0, P1>(lplocalename: P0, dwflags: TIME_FORMAT_FLAGS, lptime: Option<*const super::Foundation::SYSTEMTIME>, lpformat: P1, lptimestr: Option<&mut [u16]>) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetTimeFormatEx(lplocalename : windows_core::PCWSTR, dwflags : TIME_FORMAT_FLAGS, lptime : *const super::Foundation:: SYSTEMTIME, lpformat : windows_core::PCWSTR, lptimestr : windows_core::PWSTR, cchtime : i32) -> i32);
    GetTimeFormatEx(lplocalename.param().abi(), dwflags, core::mem::transmute(lptime.unwrap_or(std::ptr::null())), lpformat.param().abi(), core::mem::transmute(lptimestr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lptimestr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn GetTimeFormatW<P0>(locale: u32, dwflags: u32, lptime: Option<*const super::Foundation::SYSTEMTIME>, lpformat: P0, lptimestr: Option<&mut [u16]>) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetTimeFormatW(locale : u32, dwflags : u32, lptime : *const super::Foundation:: SYSTEMTIME, lpformat : windows_core::PCWSTR, lptimestr : windows_core::PWSTR, cchtime : i32) -> i32);
    GetTimeFormatW(locale, dwflags, core::mem::transmute(lptime.unwrap_or(std::ptr::null())), lpformat.param().abi(), core::mem::transmute(lptimestr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lptimestr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn GetUILanguageInfo<P0>(dwflags: u32, pwmszlanguage: P0, pwszfallbacklanguages: windows_core::PWSTR, pcchfallbacklanguages: Option<*mut u32>, pattributes: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetUILanguageInfo(dwflags : u32, pwmszlanguage : windows_core::PCWSTR, pwszfallbacklanguages : windows_core::PWSTR, pcchfallbacklanguages : *mut u32, pattributes : *mut u32) -> super::Foundation:: BOOL);
    GetUILanguageInfo(dwflags, pwmszlanguage.param().abi(), core::mem::transmute(pwszfallbacklanguages), core::mem::transmute(pcchfallbacklanguages.unwrap_or(std::ptr::null_mut())), pattributes).ok()
}
#[inline]
pub unsafe fn GetUserDefaultGeoName(geoname: &mut [u16]) -> i32 {
    windows_targets::link!("kernel32.dll" "system" fn GetUserDefaultGeoName(geoname : windows_core::PWSTR, geonamecount : i32) -> i32);
    GetUserDefaultGeoName(core::mem::transmute(geoname.as_ptr()), geoname.len().try_into().unwrap())
}
#[inline]
pub unsafe fn GetUserDefaultLCID() -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn GetUserDefaultLCID() -> u32);
    GetUserDefaultLCID()
}
#[inline]
pub unsafe fn GetUserDefaultLangID() -> u16 {
    windows_targets::link!("kernel32.dll" "system" fn GetUserDefaultLangID() -> u16);
    GetUserDefaultLangID()
}
#[inline]
pub unsafe fn GetUserDefaultLocaleName(lplocalename: &mut [u16]) -> i32 {
    windows_targets::link!("kernel32.dll" "system" fn GetUserDefaultLocaleName(lplocalename : windows_core::PWSTR, cchlocalename : i32) -> i32);
    GetUserDefaultLocaleName(core::mem::transmute(lplocalename.as_ptr()), lplocalename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn GetUserDefaultUILanguage() -> u16 {
    windows_targets::link!("kernel32.dll" "system" fn GetUserDefaultUILanguage() -> u16);
    GetUserDefaultUILanguage()
}
#[inline]
pub unsafe fn GetUserGeoID(geoclass: SYSGEOCLASS) -> i32 {
    windows_targets::link!("kernel32.dll" "system" fn GetUserGeoID(geoclass : SYSGEOCLASS) -> i32);
    GetUserGeoID(geoclass)
}
#[inline]
pub unsafe fn GetUserPreferredUILanguages(dwflags: u32, pulnumlanguages: *mut u32, pwszlanguagesbuffer: windows_core::PWSTR, pcchlanguagesbuffer: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn GetUserPreferredUILanguages(dwflags : u32, pulnumlanguages : *mut u32, pwszlanguagesbuffer : windows_core::PWSTR, pcchlanguagesbuffer : *mut u32) -> super::Foundation:: BOOL);
    GetUserPreferredUILanguages(dwflags, pulnumlanguages, core::mem::transmute(pwszlanguagesbuffer), pcchlanguagesbuffer).ok()
}
#[inline]
pub unsafe fn IdnToAscii(dwflags: u32, lpunicodecharstr: &[u16], lpasciicharstr: Option<&mut [u16]>) -> i32 {
    windows_targets::link!("normaliz.dll" "system" fn IdnToAscii(dwflags : u32, lpunicodecharstr : windows_core::PCWSTR, cchunicodechar : i32, lpasciicharstr : windows_core::PWSTR, cchasciichar : i32) -> i32);
    IdnToAscii(dwflags, core::mem::transmute(lpunicodecharstr.as_ptr()), lpunicodecharstr.len().try_into().unwrap(), core::mem::transmute(lpasciicharstr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpasciicharstr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn IdnToNameprepUnicode(dwflags: u32, lpunicodecharstr: &[u16], lpnameprepcharstr: Option<&mut [u16]>) -> i32 {
    windows_targets::link!("kernel32.dll" "system" fn IdnToNameprepUnicode(dwflags : u32, lpunicodecharstr : windows_core::PCWSTR, cchunicodechar : i32, lpnameprepcharstr : windows_core::PWSTR, cchnameprepchar : i32) -> i32);
    IdnToNameprepUnicode(dwflags, core::mem::transmute(lpunicodecharstr.as_ptr()), lpunicodecharstr.len().try_into().unwrap(), core::mem::transmute(lpnameprepcharstr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpnameprepcharstr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn IdnToUnicode(dwflags: u32, lpasciicharstr: &[u16], lpunicodecharstr: Option<&mut [u16]>) -> i32 {
    windows_targets::link!("normaliz.dll" "system" fn IdnToUnicode(dwflags : u32, lpasciicharstr : windows_core::PCWSTR, cchasciichar : i32, lpunicodecharstr : windows_core::PWSTR, cchunicodechar : i32) -> i32);
    IdnToUnicode(dwflags, core::mem::transmute(lpasciicharstr.as_ptr()), lpasciicharstr.len().try_into().unwrap(), core::mem::transmute(lpunicodecharstr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpunicodecharstr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn IsCalendarLeapYear(calid: u32, year: u32, era: u32) -> super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn IsCalendarLeapYear(calid : u32, year : u32, era : u32) -> super::Foundation:: BOOL);
    IsCalendarLeapYear(calid, year, era)
}
#[inline]
pub unsafe fn IsDBCSLeadByte(testchar: u8) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn IsDBCSLeadByte(testchar : u8) -> super::Foundation:: BOOL);
    IsDBCSLeadByte(testchar).ok()
}
#[inline]
pub unsafe fn IsDBCSLeadByteEx(codepage: u32, testchar: u8) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn IsDBCSLeadByteEx(codepage : u32, testchar : u8) -> super::Foundation:: BOOL);
    IsDBCSLeadByteEx(codepage, testchar).ok()
}
#[inline]
pub unsafe fn IsNLSDefinedString(function: u32, dwflags: u32, lpversioninformation: *const NLSVERSIONINFO, lpstring: &[u16]) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn IsNLSDefinedString(function : u32, dwflags : u32, lpversioninformation : *const NLSVERSIONINFO, lpstring : windows_core::PCWSTR, cchstr : i32) -> super::Foundation:: BOOL);
    IsNLSDefinedString(function, dwflags, lpversioninformation, core::mem::transmute(lpstring.as_ptr()), lpstring.len().try_into().unwrap()).ok()
}
#[inline]
pub unsafe fn IsNormalizedString(normform: NORM_FORM, lpstring: &[u16]) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn IsNormalizedString(normform : NORM_FORM, lpstring : windows_core::PCWSTR, cwlength : i32) -> super::Foundation:: BOOL);
    IsNormalizedString(normform, core::mem::transmute(lpstring.as_ptr()), lpstring.len().try_into().unwrap()).ok()
}
#[inline]
pub unsafe fn IsTextUnicode(lpv: *const core::ffi::c_void, isize: i32, lpiresult: Option<*mut IS_TEXT_UNICODE_RESULT>) -> super::Foundation::BOOL {
    windows_targets::link!("advapi32.dll" "system" fn IsTextUnicode(lpv : *const core::ffi::c_void, isize : i32, lpiresult : *mut IS_TEXT_UNICODE_RESULT) -> super::Foundation:: BOOL);
    IsTextUnicode(lpv, isize, core::mem::transmute(lpiresult.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn IsValidCodePage(codepage: u32) -> super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn IsValidCodePage(codepage : u32) -> super::Foundation:: BOOL);
    IsValidCodePage(codepage)
}
#[inline]
pub unsafe fn IsValidLanguageGroup(languagegroup: u32, dwflags: ENUM_SYSTEM_LANGUAGE_GROUPS_FLAGS) -> super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn IsValidLanguageGroup(languagegroup : u32, dwflags : ENUM_SYSTEM_LANGUAGE_GROUPS_FLAGS) -> super::Foundation:: BOOL);
    IsValidLanguageGroup(languagegroup, dwflags)
}
#[inline]
pub unsafe fn IsValidLocale(locale: u32, dwflags: IS_VALID_LOCALE_FLAGS) -> super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn IsValidLocale(locale : u32, dwflags : IS_VALID_LOCALE_FLAGS) -> super::Foundation:: BOOL);
    IsValidLocale(locale, dwflags)
}
#[inline]
pub unsafe fn IsValidLocaleName<P0>(lplocalename: P0) -> super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn IsValidLocaleName(lplocalename : windows_core::PCWSTR) -> super::Foundation:: BOOL);
    IsValidLocaleName(lplocalename.param().abi())
}
#[inline]
pub unsafe fn IsValidNLSVersion<P0>(function: u32, lplocalename: P0, lpversioninformation: *const NLSVERSIONINFOEX) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn IsValidNLSVersion(function : u32, lplocalename : windows_core::PCWSTR, lpversioninformation : *const NLSVERSIONINFOEX) -> u32);
    IsValidNLSVersion(function, lplocalename.param().abi(), lpversioninformation)
}
#[inline]
pub unsafe fn IsWellFormedTag<P0>(psztag: P0) -> u8
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("bcp47mrm.dll" "system" fn IsWellFormedTag(psztag : windows_core::PCWSTR) -> u8);
    IsWellFormedTag(psztag.param().abi())
}
#[inline]
pub unsafe fn LCIDToLocaleName(locale: u32, lpname: Option<&mut [u16]>, dwflags: u32) -> i32 {
    windows_targets::link!("kernel32.dll" "system" fn LCIDToLocaleName(locale : u32, lpname : windows_core::PWSTR, cchname : i32, dwflags : u32) -> i32);
    LCIDToLocaleName(locale, core::mem::transmute(lpname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), dwflags)
}
#[inline]
pub unsafe fn LCMapStringA(locale: u32, dwmapflags: u32, lpsrcstr: &[u8], lpdeststr: windows_core::PSTR, cchdest: i32) -> i32 {
    windows_targets::link!("kernel32.dll" "system" fn LCMapStringA(locale : u32, dwmapflags : u32, lpsrcstr : windows_core::PCSTR, cchsrc : i32, lpdeststr : windows_core::PSTR, cchdest : i32) -> i32);
    LCMapStringA(locale, dwmapflags, core::mem::transmute(lpsrcstr.as_ptr()), lpsrcstr.len().try_into().unwrap(), core::mem::transmute(lpdeststr), cchdest)
}
#[inline]
pub unsafe fn LCMapStringEx<P0, P1>(lplocalename: P0, dwmapflags: u32, lpsrcstr: &[u16], lpdeststr: Option<&mut [u16]>, lpversioninformation: Option<*const NLSVERSIONINFO>, lpreserved: Option<*const core::ffi::c_void>, sorthandle: P1) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::Foundation::LPARAM>,
{
    windows_targets::link!("kernel32.dll" "system" fn LCMapStringEx(lplocalename : windows_core::PCWSTR, dwmapflags : u32, lpsrcstr : windows_core::PCWSTR, cchsrc : i32, lpdeststr : windows_core::PWSTR, cchdest : i32, lpversioninformation : *const NLSVERSIONINFO, lpreserved : *const core::ffi::c_void, sorthandle : super::Foundation:: LPARAM) -> i32);
    LCMapStringEx(lplocalename.param().abi(), dwmapflags, core::mem::transmute(lpsrcstr.as_ptr()), lpsrcstr.len().try_into().unwrap(), core::mem::transmute(lpdeststr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpdeststr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lpversioninformation.unwrap_or(std::ptr::null())), core::mem::transmute(lpreserved.unwrap_or(std::ptr::null())), sorthandle.param().abi())
}
#[inline]
pub unsafe fn LCMapStringW(locale: u32, dwmapflags: u32, lpsrcstr: &[u16], lpdeststr: windows_core::PWSTR, cchdest: i32) -> i32 {
    windows_targets::link!("kernel32.dll" "system" fn LCMapStringW(locale : u32, dwmapflags : u32, lpsrcstr : windows_core::PCWSTR, cchsrc : i32, lpdeststr : windows_core::PWSTR, cchdest : i32) -> i32);
    LCMapStringW(locale, dwmapflags, core::mem::transmute(lpsrcstr.as_ptr()), lpsrcstr.len().try_into().unwrap(), core::mem::transmute(lpdeststr), cchdest)
}
#[inline]
pub unsafe fn LocaleNameToLCID<P0>(lpname: P0, dwflags: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn LocaleNameToLCID(lpname : windows_core::PCWSTR, dwflags : u32) -> u32);
    LocaleNameToLCID(lpname.param().abi(), dwflags)
}
#[inline]
pub unsafe fn MappingDoAction<P0>(pbag: *mut MAPPING_PROPERTY_BAG, dwrangeindex: u32, pszactionid: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("elscore.dll" "system" fn MappingDoAction(pbag : *mut MAPPING_PROPERTY_BAG, dwrangeindex : u32, pszactionid : windows_core::PCWSTR) -> windows_core::HRESULT);
    MappingDoAction(pbag, dwrangeindex, pszactionid.param().abi()).ok()
}
#[inline]
pub unsafe fn MappingFreePropertyBag(pbag: *const MAPPING_PROPERTY_BAG) -> windows_core::Result<()> {
    windows_targets::link!("elscore.dll" "system" fn MappingFreePropertyBag(pbag : *const MAPPING_PROPERTY_BAG) -> windows_core::HRESULT);
    MappingFreePropertyBag(pbag).ok()
}
#[inline]
pub unsafe fn MappingFreeServices(pserviceinfo: *const MAPPING_SERVICE_INFO) -> windows_core::Result<()> {
    windows_targets::link!("elscore.dll" "system" fn MappingFreeServices(pserviceinfo : *const MAPPING_SERVICE_INFO) -> windows_core::HRESULT);
    MappingFreeServices(pserviceinfo).ok()
}
#[inline]
pub unsafe fn MappingGetServices(poptions: Option<*const MAPPING_ENUM_OPTIONS>, prgservices: *mut *mut MAPPING_SERVICE_INFO, pdwservicescount: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("elscore.dll" "system" fn MappingGetServices(poptions : *const MAPPING_ENUM_OPTIONS, prgservices : *mut *mut MAPPING_SERVICE_INFO, pdwservicescount : *mut u32) -> windows_core::HRESULT);
    MappingGetServices(core::mem::transmute(poptions.unwrap_or(std::ptr::null())), prgservices, pdwservicescount).ok()
}
#[inline]
pub unsafe fn MappingRecognizeText(pserviceinfo: *const MAPPING_SERVICE_INFO, psztext: &[u16], dwindex: u32, poptions: Option<*const MAPPING_OPTIONS>, pbag: *mut MAPPING_PROPERTY_BAG) -> windows_core::Result<()> {
    windows_targets::link!("elscore.dll" "system" fn MappingRecognizeText(pserviceinfo : *const MAPPING_SERVICE_INFO, psztext : windows_core::PCWSTR, dwlength : u32, dwindex : u32, poptions : *const MAPPING_OPTIONS, pbag : *mut MAPPING_PROPERTY_BAG) -> windows_core::HRESULT);
    MappingRecognizeText(pserviceinfo, core::mem::transmute(psztext.as_ptr()), psztext.len().try_into().unwrap(), dwindex, core::mem::transmute(poptions.unwrap_or(std::ptr::null())), pbag).ok()
}
#[inline]
pub unsafe fn MultiByteToWideChar(codepage: u32, dwflags: MULTI_BYTE_TO_WIDE_CHAR_FLAGS, lpmultibytestr: &[u8], lpwidecharstr: Option<&mut [u16]>) -> i32 {
    windows_targets::link!("kernel32.dll" "system" fn MultiByteToWideChar(codepage : u32, dwflags : MULTI_BYTE_TO_WIDE_CHAR_FLAGS, lpmultibytestr : windows_core::PCSTR, cbmultibyte : i32, lpwidecharstr : windows_core::PWSTR, cchwidechar : i32) -> i32);
    MultiByteToWideChar(codepage, dwflags, core::mem::transmute(lpmultibytestr.as_ptr()), lpmultibytestr.len().try_into().unwrap(), core::mem::transmute(lpwidecharstr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpwidecharstr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn NormalizeString(normform: NORM_FORM, lpsrcstring: &[u16], lpdststring: Option<&mut [u16]>) -> i32 {
    windows_targets::link!("kernel32.dll" "system" fn NormalizeString(normform : NORM_FORM, lpsrcstring : windows_core::PCWSTR, cwsrclength : i32, lpdststring : windows_core::PWSTR, cwdstlength : i32) -> i32);
    NormalizeString(normform, core::mem::transmute(lpsrcstring.as_ptr()), lpsrcstring.len().try_into().unwrap(), core::mem::transmute(lpdststring.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpdststring.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn NotifyUILanguageChange<P0, P1>(dwflags: u32, pcwstrnewlanguage: P0, pcwstrpreviouslanguage: P1, dwreserved: u32, pdwstatusrtrn: Option<*mut u32>) -> super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn NotifyUILanguageChange(dwflags : u32, pcwstrnewlanguage : windows_core::PCWSTR, pcwstrpreviouslanguage : windows_core::PCWSTR, dwreserved : u32, pdwstatusrtrn : *mut u32) -> super::Foundation:: BOOL);
    NotifyUILanguageChange(dwflags, pcwstrnewlanguage.param().abi(), pcwstrpreviouslanguage.param().abi(), dwreserved, core::mem::transmute(pdwstatusrtrn.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn ResolveLocaleName<P0>(lpnametoresolve: P0, lplocalename: Option<&mut [u16]>) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn ResolveLocaleName(lpnametoresolve : windows_core::PCWSTR, lplocalename : windows_core::PWSTR, cchlocalename : i32) -> i32);
    ResolveLocaleName(lpnametoresolve.param().abi(), core::mem::transmute(lplocalename.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lplocalename.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn RestoreThreadPreferredUILanguages<P0>(snapshot: P0)
where
    P0: windows_core::Param<HSAVEDUILANGUAGES>,
{
    windows_targets::link!("kernel32.dll" "system" fn RestoreThreadPreferredUILanguages(snapshot : HSAVEDUILANGUAGES));
    RestoreThreadPreferredUILanguages(snapshot.param().abi())
}
#[inline]
pub unsafe fn ScriptApplyDigitSubstitution(psds: *const SCRIPT_DIGITSUBSTITUTE, psc: *mut SCRIPT_CONTROL, pss: *mut SCRIPT_STATE) -> windows_core::Result<()> {
    windows_targets::link!("usp10.dll" "system" fn ScriptApplyDigitSubstitution(psds : *const SCRIPT_DIGITSUBSTITUTE, psc : *mut SCRIPT_CONTROL, pss : *mut SCRIPT_STATE) -> windows_core::HRESULT);
    ScriptApplyDigitSubstitution(psds, psc, pss).ok()
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ScriptApplyLogicalWidth(pidx: *const i32, cchars: i32, cglyphs: i32, pwlogclust: *const u16, psva: *const SCRIPT_VISATTR, piadvance: *const i32, psa: *const SCRIPT_ANALYSIS, pabc: Option<*mut super::Graphics::Gdi::ABC>, pijustify: *mut i32) -> windows_core::Result<()> {
    windows_targets::link!("usp10.dll" "system" fn ScriptApplyLogicalWidth(pidx : *const i32, cchars : i32, cglyphs : i32, pwlogclust : *const u16, psva : *const SCRIPT_VISATTR, piadvance : *const i32, psa : *const SCRIPT_ANALYSIS, pabc : *mut super::Graphics::Gdi:: ABC, pijustify : *mut i32) -> windows_core::HRESULT);
    ScriptApplyLogicalWidth(pidx, cchars, cglyphs, pwlogclust, psva, piadvance, psa, core::mem::transmute(pabc.unwrap_or(std::ptr::null_mut())), pijustify).ok()
}
#[inline]
pub unsafe fn ScriptBreak<P0>(pwcchars: P0, cchars: i32, psa: *const SCRIPT_ANALYSIS, psla: *mut SCRIPT_LOGATTR) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("usp10.dll" "system" fn ScriptBreak(pwcchars : windows_core::PCWSTR, cchars : i32, psa : *const SCRIPT_ANALYSIS, psla : *mut SCRIPT_LOGATTR) -> windows_core::HRESULT);
    ScriptBreak(pwcchars.param().abi(), cchars, psa, psla).ok()
}
#[inline]
pub unsafe fn ScriptCPtoX<P0>(icp: i32, ftrailing: P0, cglyphs: i32, pwlogclust: &[u16], psva: *const SCRIPT_VISATTR, piadvance: *const i32, psa: *const SCRIPT_ANALYSIS, pix: *mut i32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Foundation::BOOL>,
{
    windows_targets::link!("usp10.dll" "system" fn ScriptCPtoX(icp : i32, ftrailing : super::Foundation:: BOOL, cchars : i32, cglyphs : i32, pwlogclust : *const u16, psva : *const SCRIPT_VISATTR, piadvance : *const i32, psa : *const SCRIPT_ANALYSIS, pix : *mut i32) -> windows_core::HRESULT);
    ScriptCPtoX(icp, ftrailing.param().abi(), pwlogclust.len().try_into().unwrap(), cglyphs, core::mem::transmute(pwlogclust.as_ptr()), psva, piadvance, psa, pix).ok()
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ScriptCacheGetHeight<P0>(hdc: P0, psc: *mut *mut core::ffi::c_void, tmheight: *mut i32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Graphics::Gdi::HDC>,
{
    windows_targets::link!("usp10.dll" "system" fn ScriptCacheGetHeight(hdc : super::Graphics::Gdi:: HDC, psc : *mut *mut core::ffi::c_void, tmheight : *mut i32) -> windows_core::HRESULT);
    ScriptCacheGetHeight(hdc.param().abi(), psc, tmheight).ok()
}
#[inline]
pub unsafe fn ScriptFreeCache(psc: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("usp10.dll" "system" fn ScriptFreeCache(psc : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    ScriptFreeCache(psc).ok()
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ScriptGetCMap<P0, P1>(hdc: P0, psc: *mut *mut core::ffi::c_void, pwcinchars: P1, cchars: i32, dwflags: u32, pwoutglyphs: *mut u16) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Graphics::Gdi::HDC>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("usp10.dll" "system" fn ScriptGetCMap(hdc : super::Graphics::Gdi:: HDC, psc : *mut *mut core::ffi::c_void, pwcinchars : windows_core::PCWSTR, cchars : i32, dwflags : u32, pwoutglyphs : *mut u16) -> windows_core::HRESULT);
    ScriptGetCMap(hdc.param().abi(), psc, pwcinchars.param().abi(), cchars, dwflags, pwoutglyphs).ok()
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ScriptGetFontAlternateGlyphs<P0>(hdc: P0, psc: *mut *mut core::ffi::c_void, psa: Option<*const SCRIPT_ANALYSIS>, tagscript: u32, taglangsys: u32, tagfeature: u32, wglyphid: u16, palternateglyphs: &mut [u16], pcalternates: *mut i32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Graphics::Gdi::HDC>,
{
    windows_targets::link!("usp10.dll" "system" fn ScriptGetFontAlternateGlyphs(hdc : super::Graphics::Gdi:: HDC, psc : *mut *mut core::ffi::c_void, psa : *const SCRIPT_ANALYSIS, tagscript : u32, taglangsys : u32, tagfeature : u32, wglyphid : u16, cmaxalternates : i32, palternateglyphs : *mut u16, pcalternates : *mut i32) -> windows_core::HRESULT);
    ScriptGetFontAlternateGlyphs(hdc.param().abi(), psc, core::mem::transmute(psa.unwrap_or(std::ptr::null())), tagscript, taglangsys, tagfeature, wglyphid, palternateglyphs.len().try_into().unwrap(), core::mem::transmute(palternateglyphs.as_ptr()), pcalternates).ok()
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ScriptGetFontFeatureTags<P0>(hdc: P0, psc: *mut *mut core::ffi::c_void, psa: Option<*const SCRIPT_ANALYSIS>, tagscript: u32, taglangsys: u32, pfeaturetags: &mut [u32], pctags: *mut i32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Graphics::Gdi::HDC>,
{
    windows_targets::link!("usp10.dll" "system" fn ScriptGetFontFeatureTags(hdc : super::Graphics::Gdi:: HDC, psc : *mut *mut core::ffi::c_void, psa : *const SCRIPT_ANALYSIS, tagscript : u32, taglangsys : u32, cmaxtags : i32, pfeaturetags : *mut u32, pctags : *mut i32) -> windows_core::HRESULT);
    ScriptGetFontFeatureTags(hdc.param().abi(), psc, core::mem::transmute(psa.unwrap_or(std::ptr::null())), tagscript, taglangsys, pfeaturetags.len().try_into().unwrap(), core::mem::transmute(pfeaturetags.as_ptr()), pctags).ok()
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ScriptGetFontLanguageTags<P0>(hdc: P0, psc: *mut *mut core::ffi::c_void, psa: Option<*const SCRIPT_ANALYSIS>, tagscript: u32, plangsystags: &mut [u32], pctags: *mut i32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Graphics::Gdi::HDC>,
{
    windows_targets::link!("usp10.dll" "system" fn ScriptGetFontLanguageTags(hdc : super::Graphics::Gdi:: HDC, psc : *mut *mut core::ffi::c_void, psa : *const SCRIPT_ANALYSIS, tagscript : u32, cmaxtags : i32, plangsystags : *mut u32, pctags : *mut i32) -> windows_core::HRESULT);
    ScriptGetFontLanguageTags(hdc.param().abi(), psc, core::mem::transmute(psa.unwrap_or(std::ptr::null())), tagscript, plangsystags.len().try_into().unwrap(), core::mem::transmute(plangsystags.as_ptr()), pctags).ok()
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ScriptGetFontProperties<P0>(hdc: P0, psc: *mut *mut core::ffi::c_void, sfp: *mut SCRIPT_FONTPROPERTIES) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Graphics::Gdi::HDC>,
{
    windows_targets::link!("usp10.dll" "system" fn ScriptGetFontProperties(hdc : super::Graphics::Gdi:: HDC, psc : *mut *mut core::ffi::c_void, sfp : *mut SCRIPT_FONTPROPERTIES) -> windows_core::HRESULT);
    ScriptGetFontProperties(hdc.param().abi(), psc, sfp).ok()
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ScriptGetFontScriptTags<P0>(hdc: P0, psc: *mut *mut core::ffi::c_void, psa: Option<*const SCRIPT_ANALYSIS>, pscripttags: &mut [u32], pctags: *mut i32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Graphics::Gdi::HDC>,
{
    windows_targets::link!("usp10.dll" "system" fn ScriptGetFontScriptTags(hdc : super::Graphics::Gdi:: HDC, psc : *mut *mut core::ffi::c_void, psa : *const SCRIPT_ANALYSIS, cmaxtags : i32, pscripttags : *mut u32, pctags : *mut i32) -> windows_core::HRESULT);
    ScriptGetFontScriptTags(hdc.param().abi(), psc, core::mem::transmute(psa.unwrap_or(std::ptr::null())), pscripttags.len().try_into().unwrap(), core::mem::transmute(pscripttags.as_ptr()), pctags).ok()
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ScriptGetGlyphABCWidth<P0>(hdc: P0, psc: *mut *mut core::ffi::c_void, wglyph: u16, pabc: *mut super::Graphics::Gdi::ABC) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Graphics::Gdi::HDC>,
{
    windows_targets::link!("usp10.dll" "system" fn ScriptGetGlyphABCWidth(hdc : super::Graphics::Gdi:: HDC, psc : *mut *mut core::ffi::c_void, wglyph : u16, pabc : *mut super::Graphics::Gdi:: ABC) -> windows_core::HRESULT);
    ScriptGetGlyphABCWidth(hdc.param().abi(), psc, wglyph, pabc).ok()
}
#[inline]
pub unsafe fn ScriptGetLogicalWidths(psa: *const SCRIPT_ANALYSIS, cchars: i32, cglyphs: i32, piglyphwidth: *const i32, pwlogclust: *const u16, psva: *const SCRIPT_VISATTR, pidx: *const i32) -> windows_core::Result<()> {
    windows_targets::link!("usp10.dll" "system" fn ScriptGetLogicalWidths(psa : *const SCRIPT_ANALYSIS, cchars : i32, cglyphs : i32, piglyphwidth : *const i32, pwlogclust : *const u16, psva : *const SCRIPT_VISATTR, pidx : *const i32) -> windows_core::HRESULT);
    ScriptGetLogicalWidths(psa, cchars, cglyphs, piglyphwidth, pwlogclust, psva, pidx).ok()
}
#[inline]
pub unsafe fn ScriptGetProperties(ppsp: *mut *mut *mut SCRIPT_PROPERTIES, pinumscripts: *mut i32) -> windows_core::Result<()> {
    windows_targets::link!("usp10.dll" "system" fn ScriptGetProperties(ppsp : *mut *mut *mut SCRIPT_PROPERTIES, pinumscripts : *mut i32) -> windows_core::HRESULT);
    ScriptGetProperties(ppsp, pinumscripts).ok()
}
#[inline]
pub unsafe fn ScriptIsComplex(pwcinchars: &[u16], dwflags: SCRIPT_IS_COMPLEX_FLAGS) -> windows_core::HRESULT {
    windows_targets::link!("usp10.dll" "system" fn ScriptIsComplex(pwcinchars : windows_core::PCWSTR, cinchars : i32, dwflags : SCRIPT_IS_COMPLEX_FLAGS) -> windows_core::HRESULT);
    ScriptIsComplex(core::mem::transmute(pwcinchars.as_ptr()), pwcinchars.len().try_into().unwrap(), dwflags)
}
#[inline]
pub unsafe fn ScriptItemize(pwcinchars: &[u16], pscontrol: Option<*const SCRIPT_CONTROL>, psstate: Option<*const SCRIPT_STATE>, pitems: &mut [SCRIPT_ITEM], pcitems: *mut i32) -> windows_core::Result<()> {
    windows_targets::link!("usp10.dll" "system" fn ScriptItemize(pwcinchars : windows_core::PCWSTR, cinchars : i32, cmaxitems : i32, pscontrol : *const SCRIPT_CONTROL, psstate : *const SCRIPT_STATE, pitems : *mut SCRIPT_ITEM, pcitems : *mut i32) -> windows_core::HRESULT);
    ScriptItemize(core::mem::transmute(pwcinchars.as_ptr()), pwcinchars.len().try_into().unwrap(), pitems.len().try_into().unwrap(), core::mem::transmute(pscontrol.unwrap_or(std::ptr::null())), core::mem::transmute(psstate.unwrap_or(std::ptr::null())), core::mem::transmute(pitems.as_ptr()), pcitems).ok()
}
#[inline]
pub unsafe fn ScriptItemizeOpenType(pwcinchars: &[u16], cmaxitems: i32, pscontrol: Option<*const SCRIPT_CONTROL>, psstate: Option<*const SCRIPT_STATE>, pitems: *mut SCRIPT_ITEM, pscripttags: *mut u32, pcitems: *mut i32) -> windows_core::Result<()> {
    windows_targets::link!("usp10.dll" "system" fn ScriptItemizeOpenType(pwcinchars : windows_core::PCWSTR, cinchars : i32, cmaxitems : i32, pscontrol : *const SCRIPT_CONTROL, psstate : *const SCRIPT_STATE, pitems : *mut SCRIPT_ITEM, pscripttags : *mut u32, pcitems : *mut i32) -> windows_core::HRESULT);
    ScriptItemizeOpenType(core::mem::transmute(pwcinchars.as_ptr()), pwcinchars.len().try_into().unwrap(), cmaxitems, core::mem::transmute(pscontrol.unwrap_or(std::ptr::null())), core::mem::transmute(psstate.unwrap_or(std::ptr::null())), pitems, pscripttags, pcitems).ok()
}
#[inline]
pub unsafe fn ScriptJustify(psva: *const SCRIPT_VISATTR, piadvance: *const i32, cglyphs: i32, idx: i32, iminkashida: i32, pijustify: *mut i32) -> windows_core::Result<()> {
    windows_targets::link!("usp10.dll" "system" fn ScriptJustify(psva : *const SCRIPT_VISATTR, piadvance : *const i32, cglyphs : i32, idx : i32, iminkashida : i32, pijustify : *mut i32) -> windows_core::HRESULT);
    ScriptJustify(psva, piadvance, cglyphs, idx, iminkashida, pijustify).ok()
}
#[inline]
pub unsafe fn ScriptLayout(cruns: i32, pblevel: *const u8, pivisualtological: Option<*mut i32>, pilogicaltovisual: Option<*mut i32>) -> windows_core::Result<()> {
    windows_targets::link!("usp10.dll" "system" fn ScriptLayout(cruns : i32, pblevel : *const u8, pivisualtological : *mut i32, pilogicaltovisual : *mut i32) -> windows_core::HRESULT);
    ScriptLayout(cruns, pblevel, core::mem::transmute(pivisualtological.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pilogicaltovisual.unwrap_or(std::ptr::null_mut()))).ok()
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ScriptPlace<P0>(hdc: P0, psc: *mut *mut core::ffi::c_void, pwglyphs: *const u16, cglyphs: i32, psva: *const SCRIPT_VISATTR, psa: *mut SCRIPT_ANALYSIS, piadvance: *mut i32, pgoffset: Option<*mut GOFFSET>, pabc: *mut super::Graphics::Gdi::ABC) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Graphics::Gdi::HDC>,
{
    windows_targets::link!("usp10.dll" "system" fn ScriptPlace(hdc : super::Graphics::Gdi:: HDC, psc : *mut *mut core::ffi::c_void, pwglyphs : *const u16, cglyphs : i32, psva : *const SCRIPT_VISATTR, psa : *mut SCRIPT_ANALYSIS, piadvance : *mut i32, pgoffset : *mut GOFFSET, pabc : *mut super::Graphics::Gdi:: ABC) -> windows_core::HRESULT);
    ScriptPlace(hdc.param().abi(), psc, pwglyphs, cglyphs, psva, psa, piadvance, core::mem::transmute(pgoffset.unwrap_or(std::ptr::null_mut())), pabc).ok()
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ScriptPlaceOpenType<P0, P1>(hdc: P0, psc: *mut *mut core::ffi::c_void, psa: *mut SCRIPT_ANALYSIS, tagscript: u32, taglangsys: u32, rcrangechars: Option<*const i32>, rprangeproperties: Option<*const *const TEXTRANGE_PROPERTIES>, cranges: i32, pwcchars: P1, pwlogclust: *const u16, pcharprops: *const SCRIPT_CHARPROP, cchars: i32, pwglyphs: *const u16, pglyphprops: *const SCRIPT_GLYPHPROP, cglyphs: i32, piadvance: *mut i32, pgoffset: *mut GOFFSET, pabc: Option<*mut super::Graphics::Gdi::ABC>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Graphics::Gdi::HDC>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("usp10.dll" "system" fn ScriptPlaceOpenType(hdc : super::Graphics::Gdi:: HDC, psc : *mut *mut core::ffi::c_void, psa : *mut SCRIPT_ANALYSIS, tagscript : u32, taglangsys : u32, rcrangechars : *const i32, rprangeproperties : *const *const TEXTRANGE_PROPERTIES, cranges : i32, pwcchars : windows_core::PCWSTR, pwlogclust : *const u16, pcharprops : *const SCRIPT_CHARPROP, cchars : i32, pwglyphs : *const u16, pglyphprops : *const SCRIPT_GLYPHPROP, cglyphs : i32, piadvance : *mut i32, pgoffset : *mut GOFFSET, pabc : *mut super::Graphics::Gdi:: ABC) -> windows_core::HRESULT);
    ScriptPlaceOpenType(hdc.param().abi(), psc, psa, tagscript, taglangsys, core::mem::transmute(rcrangechars.unwrap_or(std::ptr::null())), core::mem::transmute(rprangeproperties.unwrap_or(std::ptr::null())), cranges, pwcchars.param().abi(), pwlogclust, pcharprops, cchars, pwglyphs, pglyphprops, cglyphs, piadvance, pgoffset, core::mem::transmute(pabc.unwrap_or(std::ptr::null_mut()))).ok()
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ScriptPositionSingleGlyph<P0>(hdc: P0, psc: *mut *mut core::ffi::c_void, psa: Option<*const SCRIPT_ANALYSIS>, tagscript: u32, taglangsys: u32, tagfeature: u32, lparameter: i32, wglyphid: u16, iadvance: i32, goffset: GOFFSET, pioutadvance: *mut i32, poutgoffset: *mut GOFFSET) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Graphics::Gdi::HDC>,
{
    windows_targets::link!("usp10.dll" "system" fn ScriptPositionSingleGlyph(hdc : super::Graphics::Gdi:: HDC, psc : *mut *mut core::ffi::c_void, psa : *const SCRIPT_ANALYSIS, tagscript : u32, taglangsys : u32, tagfeature : u32, lparameter : i32, wglyphid : u16, iadvance : i32, goffset : GOFFSET, pioutadvance : *mut i32, poutgoffset : *mut GOFFSET) -> windows_core::HRESULT);
    ScriptPositionSingleGlyph(hdc.param().abi(), psc, core::mem::transmute(psa.unwrap_or(std::ptr::null())), tagscript, taglangsys, tagfeature, lparameter, wglyphid, iadvance, core::mem::transmute(goffset), pioutadvance, poutgoffset).ok()
}
#[inline]
pub unsafe fn ScriptRecordDigitSubstitution(locale: u32) -> windows_core::Result<SCRIPT_DIGITSUBSTITUTE> {
    windows_targets::link!("usp10.dll" "system" fn ScriptRecordDigitSubstitution(locale : u32, psds : *mut SCRIPT_DIGITSUBSTITUTE) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    ScriptRecordDigitSubstitution(locale, &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ScriptShape<P0, P1>(hdc: P0, psc: *mut *mut core::ffi::c_void, pwcchars: P1, cchars: i32, cmaxglyphs: i32, psa: *mut SCRIPT_ANALYSIS, pwoutglyphs: *mut u16, pwlogclust: *mut u16, psva: *mut SCRIPT_VISATTR, pcglyphs: *mut i32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Graphics::Gdi::HDC>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("usp10.dll" "system" fn ScriptShape(hdc : super::Graphics::Gdi:: HDC, psc : *mut *mut core::ffi::c_void, pwcchars : windows_core::PCWSTR, cchars : i32, cmaxglyphs : i32, psa : *mut SCRIPT_ANALYSIS, pwoutglyphs : *mut u16, pwlogclust : *mut u16, psva : *mut SCRIPT_VISATTR, pcglyphs : *mut i32) -> windows_core::HRESULT);
    ScriptShape(hdc.param().abi(), psc, pwcchars.param().abi(), cchars, cmaxglyphs, psa, pwoutglyphs, pwlogclust, psva, pcglyphs).ok()
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ScriptShapeOpenType<P0, P1>(hdc: P0, psc: *mut *mut core::ffi::c_void, psa: *mut SCRIPT_ANALYSIS, tagscript: u32, taglangsys: u32, rcrangechars: Option<*const i32>, rprangeproperties: Option<*const *const TEXTRANGE_PROPERTIES>, cranges: i32, pwcchars: P1, cchars: i32, cmaxglyphs: i32, pwlogclust: *mut u16, pcharprops: *mut SCRIPT_CHARPROP, pwoutglyphs: *mut u16, poutglyphprops: *mut SCRIPT_GLYPHPROP, pcglyphs: *mut i32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Graphics::Gdi::HDC>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("usp10.dll" "system" fn ScriptShapeOpenType(hdc : super::Graphics::Gdi:: HDC, psc : *mut *mut core::ffi::c_void, psa : *mut SCRIPT_ANALYSIS, tagscript : u32, taglangsys : u32, rcrangechars : *const i32, rprangeproperties : *const *const TEXTRANGE_PROPERTIES, cranges : i32, pwcchars : windows_core::PCWSTR, cchars : i32, cmaxglyphs : i32, pwlogclust : *mut u16, pcharprops : *mut SCRIPT_CHARPROP, pwoutglyphs : *mut u16, poutglyphprops : *mut SCRIPT_GLYPHPROP, pcglyphs : *mut i32) -> windows_core::HRESULT);
    ScriptShapeOpenType(hdc.param().abi(), psc, psa, tagscript, taglangsys, core::mem::transmute(rcrangechars.unwrap_or(std::ptr::null())), core::mem::transmute(rprangeproperties.unwrap_or(std::ptr::null())), cranges, pwcchars.param().abi(), cchars, cmaxglyphs, pwlogclust, pcharprops, pwoutglyphs, poutglyphprops, pcglyphs).ok()
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ScriptStringAnalyse<P0>(hdc: P0, pstring: *const core::ffi::c_void, cglyphs: i32, icharset: i32, dwflags: u32, ireqwidth: i32, pscontrol: Option<*const SCRIPT_CONTROL>, psstate: Option<*const SCRIPT_STATE>, pidx: Option<&[i32]>, ptabdef: Option<*const SCRIPT_TABDEF>, pbinclass: *const u8, pssa: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Graphics::Gdi::HDC>,
{
    windows_targets::link!("usp10.dll" "system" fn ScriptStringAnalyse(hdc : super::Graphics::Gdi:: HDC, pstring : *const core::ffi::c_void, cstring : i32, cglyphs : i32, icharset : i32, dwflags : u32, ireqwidth : i32, pscontrol : *const SCRIPT_CONTROL, psstate : *const SCRIPT_STATE, pidx : *const i32, ptabdef : *const SCRIPT_TABDEF, pbinclass : *const u8, pssa : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    ScriptStringAnalyse(hdc.param().abi(), pstring, pidx.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), cglyphs, icharset, dwflags, ireqwidth, core::mem::transmute(pscontrol.unwrap_or(std::ptr::null())), core::mem::transmute(psstate.unwrap_or(std::ptr::null())), core::mem::transmute(pidx.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), core::mem::transmute(ptabdef.unwrap_or(std::ptr::null())), pbinclass, pssa).ok()
}
#[inline]
pub unsafe fn ScriptStringCPtoX<P0>(ssa: *const core::ffi::c_void, icp: i32, ftrailing: P0) -> windows_core::Result<i32>
where
    P0: windows_core::Param<super::Foundation::BOOL>,
{
    windows_targets::link!("usp10.dll" "system" fn ScriptStringCPtoX(ssa : *const core::ffi::c_void, icp : i32, ftrailing : super::Foundation:: BOOL, px : *mut i32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    ScriptStringCPtoX(ssa, icp, ftrailing.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn ScriptStringFree(pssa: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("usp10.dll" "system" fn ScriptStringFree(pssa : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    ScriptStringFree(pssa).ok()
}
#[inline]
pub unsafe fn ScriptStringGetLogicalWidths(ssa: *const core::ffi::c_void, pidx: *mut i32) -> windows_core::Result<()> {
    windows_targets::link!("usp10.dll" "system" fn ScriptStringGetLogicalWidths(ssa : *const core::ffi::c_void, pidx : *mut i32) -> windows_core::HRESULT);
    ScriptStringGetLogicalWidths(ssa, pidx).ok()
}
#[inline]
pub unsafe fn ScriptStringGetOrder(ssa: *const core::ffi::c_void, puorder: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("usp10.dll" "system" fn ScriptStringGetOrder(ssa : *const core::ffi::c_void, puorder : *mut u32) -> windows_core::HRESULT);
    ScriptStringGetOrder(ssa, puorder).ok()
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ScriptStringOut<P0>(ssa: *const core::ffi::c_void, ix: i32, iy: i32, uoptions: super::Graphics::Gdi::ETO_OPTIONS, prc: Option<*const super::Foundation::RECT>, iminsel: i32, imaxsel: i32, fdisabled: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Foundation::BOOL>,
{
    windows_targets::link!("usp10.dll" "system" fn ScriptStringOut(ssa : *const core::ffi::c_void, ix : i32, iy : i32, uoptions : super::Graphics::Gdi:: ETO_OPTIONS, prc : *const super::Foundation:: RECT, iminsel : i32, imaxsel : i32, fdisabled : super::Foundation:: BOOL) -> windows_core::HRESULT);
    ScriptStringOut(ssa, ix, iy, uoptions, core::mem::transmute(prc.unwrap_or(std::ptr::null())), iminsel, imaxsel, fdisabled.param().abi()).ok()
}
#[inline]
pub unsafe fn ScriptStringValidate(ssa: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("usp10.dll" "system" fn ScriptStringValidate(ssa : *const core::ffi::c_void) -> windows_core::HRESULT);
    ScriptStringValidate(ssa).ok()
}
#[inline]
pub unsafe fn ScriptStringXtoCP(ssa: *const core::ffi::c_void, ix: i32, pich: *mut i32, pitrailing: *mut i32) -> windows_core::Result<()> {
    windows_targets::link!("usp10.dll" "system" fn ScriptStringXtoCP(ssa : *const core::ffi::c_void, ix : i32, pich : *mut i32, pitrailing : *mut i32) -> windows_core::HRESULT);
    ScriptStringXtoCP(ssa, ix, pich, pitrailing).ok()
}
#[inline]
pub unsafe fn ScriptString_pLogAttr(ssa: *const core::ffi::c_void) -> *mut SCRIPT_LOGATTR {
    windows_targets::link!("usp10.dll" "system" fn ScriptString_pLogAttr(ssa : *const core::ffi::c_void) -> *mut SCRIPT_LOGATTR);
    ScriptString_pLogAttr(ssa)
}
#[inline]
pub unsafe fn ScriptString_pSize(ssa: *const core::ffi::c_void) -> *mut super::Foundation::SIZE {
    windows_targets::link!("usp10.dll" "system" fn ScriptString_pSize(ssa : *const core::ffi::c_void) -> *mut super::Foundation:: SIZE);
    ScriptString_pSize(ssa)
}
#[inline]
pub unsafe fn ScriptString_pcOutChars(ssa: *const core::ffi::c_void) -> *mut i32 {
    windows_targets::link!("usp10.dll" "system" fn ScriptString_pcOutChars(ssa : *const core::ffi::c_void) -> *mut i32);
    ScriptString_pcOutChars(ssa)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ScriptSubstituteSingleGlyph<P0>(hdc: P0, psc: *mut *mut core::ffi::c_void, psa: Option<*const SCRIPT_ANALYSIS>, tagscript: u32, taglangsys: u32, tagfeature: u32, lparameter: i32, wglyphid: u16, pwoutglyphid: *mut u16) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Graphics::Gdi::HDC>,
{
    windows_targets::link!("usp10.dll" "system" fn ScriptSubstituteSingleGlyph(hdc : super::Graphics::Gdi:: HDC, psc : *mut *mut core::ffi::c_void, psa : *const SCRIPT_ANALYSIS, tagscript : u32, taglangsys : u32, tagfeature : u32, lparameter : i32, wglyphid : u16, pwoutglyphid : *mut u16) -> windows_core::HRESULT);
    ScriptSubstituteSingleGlyph(hdc.param().abi(), psc, core::mem::transmute(psa.unwrap_or(std::ptr::null())), tagscript, taglangsys, tagfeature, lparameter, wglyphid, pwoutglyphid).ok()
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ScriptTextOut<P0, P1>(hdc: P0, psc: *mut *mut core::ffi::c_void, x: i32, y: i32, fuoptions: u32, lprc: Option<*const super::Foundation::RECT>, psa: *const SCRIPT_ANALYSIS, pwcreserved: P1, ireserved: i32, pwglyphs: *const u16, cglyphs: i32, piadvance: *const i32, pijustify: Option<*const i32>, pgoffset: *const GOFFSET) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Graphics::Gdi::HDC>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("usp10.dll" "system" fn ScriptTextOut(hdc : super::Graphics::Gdi:: HDC, psc : *mut *mut core::ffi::c_void, x : i32, y : i32, fuoptions : u32, lprc : *const super::Foundation:: RECT, psa : *const SCRIPT_ANALYSIS, pwcreserved : windows_core::PCWSTR, ireserved : i32, pwglyphs : *const u16, cglyphs : i32, piadvance : *const i32, pijustify : *const i32, pgoffset : *const GOFFSET) -> windows_core::HRESULT);
    ScriptTextOut(hdc.param().abi(), psc, x, y, fuoptions, core::mem::transmute(lprc.unwrap_or(std::ptr::null())), psa, pwcreserved.param().abi(), ireserved, pwglyphs, cglyphs, piadvance, core::mem::transmute(pijustify.unwrap_or(std::ptr::null())), pgoffset).ok()
}
#[inline]
pub unsafe fn ScriptXtoCP(ix: i32, cglyphs: i32, pwlogclust: &[u16], psva: *const SCRIPT_VISATTR, piadvance: *const i32, psa: *const SCRIPT_ANALYSIS, picp: *mut i32, pitrailing: *mut i32) -> windows_core::Result<()> {
    windows_targets::link!("usp10.dll" "system" fn ScriptXtoCP(ix : i32, cchars : i32, cglyphs : i32, pwlogclust : *const u16, psva : *const SCRIPT_VISATTR, piadvance : *const i32, psa : *const SCRIPT_ANALYSIS, picp : *mut i32, pitrailing : *mut i32) -> windows_core::HRESULT);
    ScriptXtoCP(ix, pwlogclust.len().try_into().unwrap(), cglyphs, core::mem::transmute(pwlogclust.as_ptr()), psva, piadvance, psa, picp, pitrailing).ok()
}
#[inline]
pub unsafe fn SetCalendarInfoA<P0>(locale: u32, calendar: u32, caltype: u32, lpcaldata: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetCalendarInfoA(locale : u32, calendar : u32, caltype : u32, lpcaldata : windows_core::PCSTR) -> super::Foundation:: BOOL);
    SetCalendarInfoA(locale, calendar, caltype, lpcaldata.param().abi()).ok()
}
#[inline]
pub unsafe fn SetCalendarInfoW<P0>(locale: u32, calendar: u32, caltype: u32, lpcaldata: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetCalendarInfoW(locale : u32, calendar : u32, caltype : u32, lpcaldata : windows_core::PCWSTR) -> super::Foundation:: BOOL);
    SetCalendarInfoW(locale, calendar, caltype, lpcaldata.param().abi()).ok()
}
#[inline]
pub unsafe fn SetLocaleInfoA<P0>(locale: u32, lctype: u32, lplcdata: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetLocaleInfoA(locale : u32, lctype : u32, lplcdata : windows_core::PCSTR) -> super::Foundation:: BOOL);
    SetLocaleInfoA(locale, lctype, lplcdata.param().abi()).ok()
}
#[inline]
pub unsafe fn SetLocaleInfoW<P0>(locale: u32, lctype: u32, lplcdata: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetLocaleInfoW(locale : u32, lctype : u32, lplcdata : windows_core::PCWSTR) -> super::Foundation:: BOOL);
    SetLocaleInfoW(locale, lctype, lplcdata.param().abi()).ok()
}
#[inline]
pub unsafe fn SetProcessPreferredUILanguages<P0>(dwflags: u32, pwszlanguagesbuffer: P0, pulnumlanguages: Option<*mut u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetProcessPreferredUILanguages(dwflags : u32, pwszlanguagesbuffer : windows_core::PCWSTR, pulnumlanguages : *mut u32) -> super::Foundation:: BOOL);
    SetProcessPreferredUILanguages(dwflags, pwszlanguagesbuffer.param().abi(), core::mem::transmute(pulnumlanguages.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn SetThreadLocale(locale: u32) -> super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn SetThreadLocale(locale : u32) -> super::Foundation:: BOOL);
    SetThreadLocale(locale)
}
#[inline]
pub unsafe fn SetThreadPreferredUILanguages<P0>(dwflags: u32, pwszlanguagesbuffer: P0, pulnumlanguages: Option<*mut u32>) -> super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetThreadPreferredUILanguages(dwflags : u32, pwszlanguagesbuffer : windows_core::PCWSTR, pulnumlanguages : *mut u32) -> super::Foundation:: BOOL);
    SetThreadPreferredUILanguages(dwflags, pwszlanguagesbuffer.param().abi(), core::mem::transmute(pulnumlanguages.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn SetThreadPreferredUILanguages2<P0>(flags: u32, languages: P0, numlanguagesset: Option<*mut u32>, snapshot: Option<*mut HSAVEDUILANGUAGES>) -> super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetThreadPreferredUILanguages2(flags : u32, languages : windows_core::PCWSTR, numlanguagesset : *mut u32, snapshot : *mut HSAVEDUILANGUAGES) -> super::Foundation:: BOOL);
    SetThreadPreferredUILanguages2(flags, languages.param().abi(), core::mem::transmute(numlanguagesset.unwrap_or(std::ptr::null_mut())), core::mem::transmute(snapshot.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn SetThreadUILanguage(langid: u16) -> u16 {
    windows_targets::link!("kernel32.dll" "system" fn SetThreadUILanguage(langid : u16) -> u16);
    SetThreadUILanguage(langid)
}
#[inline]
pub unsafe fn SetUserGeoID(geoid: i32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn SetUserGeoID(geoid : i32) -> super::Foundation:: BOOL);
    SetUserGeoID(geoid).ok()
}
#[inline]
pub unsafe fn SetUserGeoName<P0>(geoname: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetUserGeoName(geoname : windows_core::PCWSTR) -> super::Foundation:: BOOL);
    SetUserGeoName(geoname.param().abi()).ok()
}
#[inline]
pub unsafe fn TranslateCharsetInfo(lpsrc: *mut u32, lpcs: *mut CHARSETINFO, dwflags: TRANSLATE_CHARSET_INFO_FLAGS) -> windows_core::Result<()> {
    windows_targets::link!("gdi32.dll" "system" fn TranslateCharsetInfo(lpsrc : *mut u32, lpcs : *mut CHARSETINFO, dwflags : TRANSLATE_CHARSET_INFO_FLAGS) -> super::Foundation:: BOOL);
    TranslateCharsetInfo(lpsrc, lpcs, dwflags).ok()
}
#[inline]
pub unsafe fn UCNV_FROM_U_CALLBACK_ESCAPE(context: *const core::ffi::c_void, fromuargs: *mut UConverterFromUnicodeArgs, codeunits: *const u16, length: i32, codepoint: i32, reason: UConverterCallbackReason, err: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn UCNV_FROM_U_CALLBACK_ESCAPE(context : *const core::ffi::c_void, fromuargs : *mut UConverterFromUnicodeArgs, codeunits : *const u16, length : i32, codepoint : i32, reason : UConverterCallbackReason, err : *mut UErrorCode));
    UCNV_FROM_U_CALLBACK_ESCAPE(context, fromuargs, codeunits, length, codepoint, reason, err)
}
#[inline]
pub unsafe fn UCNV_FROM_U_CALLBACK_SKIP(context: *const core::ffi::c_void, fromuargs: *mut UConverterFromUnicodeArgs, codeunits: *const u16, length: i32, codepoint: i32, reason: UConverterCallbackReason, err: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn UCNV_FROM_U_CALLBACK_SKIP(context : *const core::ffi::c_void, fromuargs : *mut UConverterFromUnicodeArgs, codeunits : *const u16, length : i32, codepoint : i32, reason : UConverterCallbackReason, err : *mut UErrorCode));
    UCNV_FROM_U_CALLBACK_SKIP(context, fromuargs, codeunits, length, codepoint, reason, err)
}
#[inline]
pub unsafe fn UCNV_FROM_U_CALLBACK_STOP(context: *const core::ffi::c_void, fromuargs: *mut UConverterFromUnicodeArgs, codeunits: *const u16, length: i32, codepoint: i32, reason: UConverterCallbackReason, err: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn UCNV_FROM_U_CALLBACK_STOP(context : *const core::ffi::c_void, fromuargs : *mut UConverterFromUnicodeArgs, codeunits : *const u16, length : i32, codepoint : i32, reason : UConverterCallbackReason, err : *mut UErrorCode));
    UCNV_FROM_U_CALLBACK_STOP(context, fromuargs, codeunits, length, codepoint, reason, err)
}
#[inline]
pub unsafe fn UCNV_FROM_U_CALLBACK_SUBSTITUTE(context: *const core::ffi::c_void, fromuargs: *mut UConverterFromUnicodeArgs, codeunits: *const u16, length: i32, codepoint: i32, reason: UConverterCallbackReason, err: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn UCNV_FROM_U_CALLBACK_SUBSTITUTE(context : *const core::ffi::c_void, fromuargs : *mut UConverterFromUnicodeArgs, codeunits : *const u16, length : i32, codepoint : i32, reason : UConverterCallbackReason, err : *mut UErrorCode));
    UCNV_FROM_U_CALLBACK_SUBSTITUTE(context, fromuargs, codeunits, length, codepoint, reason, err)
}
#[inline]
pub unsafe fn UCNV_TO_U_CALLBACK_ESCAPE<P0>(context: *const core::ffi::c_void, touargs: *mut UConverterToUnicodeArgs, codeunits: P0, length: i32, reason: UConverterCallbackReason, err: *mut UErrorCode)
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn UCNV_TO_U_CALLBACK_ESCAPE(context : *const core::ffi::c_void, touargs : *mut UConverterToUnicodeArgs, codeunits : windows_core::PCSTR, length : i32, reason : UConverterCallbackReason, err : *mut UErrorCode));
    UCNV_TO_U_CALLBACK_ESCAPE(context, touargs, codeunits.param().abi(), length, reason, err)
}
#[inline]
pub unsafe fn UCNV_TO_U_CALLBACK_SKIP<P0>(context: *const core::ffi::c_void, touargs: *mut UConverterToUnicodeArgs, codeunits: P0, length: i32, reason: UConverterCallbackReason, err: *mut UErrorCode)
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn UCNV_TO_U_CALLBACK_SKIP(context : *const core::ffi::c_void, touargs : *mut UConverterToUnicodeArgs, codeunits : windows_core::PCSTR, length : i32, reason : UConverterCallbackReason, err : *mut UErrorCode));
    UCNV_TO_U_CALLBACK_SKIP(context, touargs, codeunits.param().abi(), length, reason, err)
}
#[inline]
pub unsafe fn UCNV_TO_U_CALLBACK_STOP<P0>(context: *const core::ffi::c_void, touargs: *mut UConverterToUnicodeArgs, codeunits: P0, length: i32, reason: UConverterCallbackReason, err: *mut UErrorCode)
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn UCNV_TO_U_CALLBACK_STOP(context : *const core::ffi::c_void, touargs : *mut UConverterToUnicodeArgs, codeunits : windows_core::PCSTR, length : i32, reason : UConverterCallbackReason, err : *mut UErrorCode));
    UCNV_TO_U_CALLBACK_STOP(context, touargs, codeunits.param().abi(), length, reason, err)
}
#[inline]
pub unsafe fn UCNV_TO_U_CALLBACK_SUBSTITUTE<P0>(context: *const core::ffi::c_void, touargs: *mut UConverterToUnicodeArgs, codeunits: P0, length: i32, reason: UConverterCallbackReason, err: *mut UErrorCode)
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn UCNV_TO_U_CALLBACK_SUBSTITUTE(context : *const core::ffi::c_void, touargs : *mut UConverterToUnicodeArgs, codeunits : windows_core::PCSTR, length : i32, reason : UConverterCallbackReason, err : *mut UErrorCode));
    UCNV_TO_U_CALLBACK_SUBSTITUTE(context, touargs, codeunits.param().abi(), length, reason, err)
}
#[inline]
pub unsafe fn UpdateCalendarDayOfWeek(lpcaldatetime: *mut CALDATETIME) -> super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn UpdateCalendarDayOfWeek(lpcaldatetime : *mut CALDATETIME) -> super::Foundation:: BOOL);
    UpdateCalendarDayOfWeek(lpcaldatetime)
}
#[inline]
pub unsafe fn VerifyScripts<P0, P1>(dwflags: u32, lplocalescripts: P0, cchlocalescripts: i32, lptestscripts: P1, cchtestscripts: i32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn VerifyScripts(dwflags : u32, lplocalescripts : windows_core::PCWSTR, cchlocalescripts : i32, lptestscripts : windows_core::PCWSTR, cchtestscripts : i32) -> super::Foundation:: BOOL);
    VerifyScripts(dwflags, lplocalescripts.param().abi(), cchlocalescripts, lptestscripts.param().abi(), cchtestscripts).ok()
}
#[inline]
pub unsafe fn WideCharToMultiByte<P0>(codepage: u32, dwflags: u32, lpwidecharstr: &[u16], lpmultibytestr: Option<&mut [u8]>, lpdefaultchar: P0, lpuseddefaultchar: Option<*mut super::Foundation::BOOL>) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn WideCharToMultiByte(codepage : u32, dwflags : u32, lpwidecharstr : windows_core::PCWSTR, cchwidechar : i32, lpmultibytestr : windows_core::PSTR, cbmultibyte : i32, lpdefaultchar : windows_core::PCSTR, lpuseddefaultchar : *mut super::Foundation:: BOOL) -> i32);
    WideCharToMultiByte(codepage, dwflags, core::mem::transmute(lpwidecharstr.as_ptr()), lpwidecharstr.len().try_into().unwrap(), core::mem::transmute(lpmultibytestr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpmultibytestr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), lpdefaultchar.param().abi(), core::mem::transmute(lpuseddefaultchar.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn lstrcatA<P0>(lpstring1: windows_core::PSTR, lpstring2: P0) -> windows_core::PSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn lstrcatA(lpstring1 : windows_core::PSTR, lpstring2 : windows_core::PCSTR) -> windows_core::PSTR);
    lstrcatA(core::mem::transmute(lpstring1), lpstring2.param().abi())
}
#[inline]
pub unsafe fn lstrcatW<P0>(lpstring1: windows_core::PWSTR, lpstring2: P0) -> windows_core::PWSTR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn lstrcatW(lpstring1 : windows_core::PWSTR, lpstring2 : windows_core::PCWSTR) -> windows_core::PWSTR);
    lstrcatW(core::mem::transmute(lpstring1), lpstring2.param().abi())
}
#[inline]
pub unsafe fn lstrcmpA<P0, P1>(lpstring1: P0, lpstring2: P1) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn lstrcmpA(lpstring1 : windows_core::PCSTR, lpstring2 : windows_core::PCSTR) -> i32);
    lstrcmpA(lpstring1.param().abi(), lpstring2.param().abi())
}
#[inline]
pub unsafe fn lstrcmpW<P0, P1>(lpstring1: P0, lpstring2: P1) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn lstrcmpW(lpstring1 : windows_core::PCWSTR, lpstring2 : windows_core::PCWSTR) -> i32);
    lstrcmpW(lpstring1.param().abi(), lpstring2.param().abi())
}
#[inline]
pub unsafe fn lstrcmpiA<P0, P1>(lpstring1: P0, lpstring2: P1) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn lstrcmpiA(lpstring1 : windows_core::PCSTR, lpstring2 : windows_core::PCSTR) -> i32);
    lstrcmpiA(lpstring1.param().abi(), lpstring2.param().abi())
}
#[inline]
pub unsafe fn lstrcmpiW<P0, P1>(lpstring1: P0, lpstring2: P1) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn lstrcmpiW(lpstring1 : windows_core::PCWSTR, lpstring2 : windows_core::PCWSTR) -> i32);
    lstrcmpiW(lpstring1.param().abi(), lpstring2.param().abi())
}
#[inline]
pub unsafe fn lstrcpyA<P0>(lpstring1: windows_core::PSTR, lpstring2: P0) -> windows_core::PSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn lstrcpyA(lpstring1 : windows_core::PSTR, lpstring2 : windows_core::PCSTR) -> windows_core::PSTR);
    lstrcpyA(core::mem::transmute(lpstring1), lpstring2.param().abi())
}
#[inline]
pub unsafe fn lstrcpyW<P0>(lpstring1: windows_core::PWSTR, lpstring2: P0) -> windows_core::PWSTR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn lstrcpyW(lpstring1 : windows_core::PWSTR, lpstring2 : windows_core::PCWSTR) -> windows_core::PWSTR);
    lstrcpyW(core::mem::transmute(lpstring1), lpstring2.param().abi())
}
#[inline]
pub unsafe fn lstrcpynA<P0>(lpstring1: &mut [u8], lpstring2: P0) -> windows_core::PSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn lstrcpynA(lpstring1 : windows_core::PSTR, lpstring2 : windows_core::PCSTR, imaxlength : i32) -> windows_core::PSTR);
    lstrcpynA(core::mem::transmute(lpstring1.as_ptr()), lpstring2.param().abi(), lpstring1.len().try_into().unwrap())
}
#[inline]
pub unsafe fn lstrcpynW<P0>(lpstring1: &mut [u16], lpstring2: P0) -> windows_core::PWSTR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn lstrcpynW(lpstring1 : windows_core::PWSTR, lpstring2 : windows_core::PCWSTR, imaxlength : i32) -> windows_core::PWSTR);
    lstrcpynW(core::mem::transmute(lpstring1.as_ptr()), lpstring2.param().abi(), lpstring1.len().try_into().unwrap())
}
#[inline]
pub unsafe fn lstrlenA<P0>(lpstring: P0) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn lstrlenA(lpstring : windows_core::PCSTR) -> i32);
    lstrlenA(lpstring.param().abi())
}
#[inline]
pub unsafe fn lstrlenW<P0>(lpstring: P0) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn lstrlenW(lpstring : windows_core::PCWSTR) -> i32);
    lstrlenW(lpstring.param().abi())
}
#[inline]
pub unsafe fn u_UCharsToChars<P0>(us: *const u16, cs: P0, length: i32)
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn u_UCharsToChars(us : *const u16, cs : windows_core::PCSTR, length : i32));
    u_UCharsToChars(us, cs.param().abi(), length)
}
#[inline]
pub unsafe fn u_austrcpy<P0>(dst: P0, src: *const u16) -> windows_core::PSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn u_austrcpy(dst : windows_core::PCSTR, src : *const u16) -> windows_core::PSTR);
    u_austrcpy(dst.param().abi(), src)
}
#[inline]
pub unsafe fn u_austrncpy<P0>(dst: P0, src: *const u16, n: i32) -> windows_core::PSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn u_austrncpy(dst : windows_core::PCSTR, src : *const u16, n : i32) -> windows_core::PSTR);
    u_austrncpy(dst.param().abi(), src, n)
}
#[inline]
pub unsafe fn u_catclose(catd: *mut UResourceBundle) {
    windows_targets::link!("icu.dll" "cdecl" fn u_catclose(catd : *mut UResourceBundle));
    u_catclose(catd)
}
#[inline]
pub unsafe fn u_catgets(catd: *mut UResourceBundle, set_num: i32, msg_num: i32, s: *const u16, len: *mut i32, ec: *mut UErrorCode) -> *mut u16 {
    windows_targets::link!("icu.dll" "cdecl" fn u_catgets(catd : *mut UResourceBundle, set_num : i32, msg_num : i32, s : *const u16, len : *mut i32, ec : *mut UErrorCode) -> *mut u16);
    u_catgets(catd, set_num, msg_num, s, len, ec)
}
#[inline]
pub unsafe fn u_catopen<P0, P1>(name: P0, locale: P1, ec: *mut UErrorCode) -> *mut UResourceBundle
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn u_catopen(name : windows_core::PCSTR, locale : windows_core::PCSTR, ec : *mut UErrorCode) -> *mut UResourceBundle);
    u_catopen(name.param().abi(), locale.param().abi(), ec)
}
#[inline]
pub unsafe fn u_charAge(c: i32, versionarray: *mut u8) {
    windows_targets::link!("icu.dll" "cdecl" fn u_charAge(c : i32, versionarray : *mut u8));
    u_charAge(c, versionarray)
}
#[inline]
pub unsafe fn u_charDigitValue(c: i32) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn u_charDigitValue(c : i32) -> i32);
    u_charDigitValue(c)
}
#[inline]
pub unsafe fn u_charDirection(c: i32) -> UCharDirection {
    windows_targets::link!("icu.dll" "cdecl" fn u_charDirection(c : i32) -> UCharDirection);
    u_charDirection(c)
}
#[inline]
pub unsafe fn u_charFromName<P0>(namechoice: UCharNameChoice, name: P0, perrorcode: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn u_charFromName(namechoice : UCharNameChoice, name : windows_core::PCSTR, perrorcode : *mut UErrorCode) -> i32);
    u_charFromName(namechoice, name.param().abi(), perrorcode)
}
#[inline]
pub unsafe fn u_charMirror(c: i32) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn u_charMirror(c : i32) -> i32);
    u_charMirror(c)
}
#[inline]
pub unsafe fn u_charName<P0>(code: i32, namechoice: UCharNameChoice, buffer: P0, bufferlength: i32, perrorcode: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn u_charName(code : i32, namechoice : UCharNameChoice, buffer : windows_core::PCSTR, bufferlength : i32, perrorcode : *mut UErrorCode) -> i32);
    u_charName(code, namechoice, buffer.param().abi(), bufferlength, perrorcode)
}
#[inline]
pub unsafe fn u_charType(c: i32) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn u_charType(c : i32) -> i8);
    u_charType(c)
}
#[inline]
pub unsafe fn u_charsToUChars<P0>(cs: P0, us: *mut u16, length: i32)
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn u_charsToUChars(cs : windows_core::PCSTR, us : *mut u16, length : i32));
    u_charsToUChars(cs.param().abi(), us, length)
}
#[inline]
pub unsafe fn u_cleanup() {
    windows_targets::link!("icu.dll" "cdecl" fn u_cleanup());
    u_cleanup()
}
#[inline]
pub unsafe fn u_countChar32(s: *const u16, length: i32) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn u_countChar32(s : *const u16, length : i32) -> i32);
    u_countChar32(s, length)
}
#[inline]
pub unsafe fn u_digit(ch: i32, radix: i8) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn u_digit(ch : i32, radix : i8) -> i32);
    u_digit(ch, radix)
}
#[inline]
pub unsafe fn u_enumCharNames(start: i32, limit: i32, r#fn: *mut UEnumCharNamesFn, context: *mut core::ffi::c_void, namechoice: UCharNameChoice, perrorcode: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn u_enumCharNames(start : i32, limit : i32, r#fn : *mut UEnumCharNamesFn, context : *mut core::ffi::c_void, namechoice : UCharNameChoice, perrorcode : *mut UErrorCode));
    u_enumCharNames(start, limit, r#fn, context, namechoice, perrorcode)
}
#[inline]
pub unsafe fn u_enumCharTypes(enumrange: *mut UCharEnumTypeRange, context: *const core::ffi::c_void) {
    windows_targets::link!("icu.dll" "cdecl" fn u_enumCharTypes(enumrange : *mut UCharEnumTypeRange, context : *const core::ffi::c_void));
    u_enumCharTypes(enumrange, context)
}
#[inline]
pub unsafe fn u_errorName(code: UErrorCode) -> windows_core::PCSTR {
    windows_targets::link!("icu.dll" "cdecl" fn u_errorName(code : UErrorCode) -> windows_core::PCSTR);
    u_errorName(code)
}
#[inline]
pub unsafe fn u_foldCase(c: i32, options: u32) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn u_foldCase(c : i32, options : u32) -> i32);
    u_foldCase(c, options)
}
#[inline]
pub unsafe fn u_forDigit(digit: i32, radix: i8) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn u_forDigit(digit : i32, radix : i8) -> i32);
    u_forDigit(digit, radix)
}
#[inline]
pub unsafe fn u_formatMessage<P0>(locale: P0, pattern: *const u16, patternlength: i32, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn u_formatMessage(locale : windows_core::PCSTR, pattern : *const u16, patternlength : i32, result : *mut u16, resultlength : i32, status : *mut UErrorCode) -> i32);
    u_formatMessage(locale.param().abi(), pattern, patternlength, result, resultlength, status)
}
#[inline]
pub unsafe fn u_formatMessageWithError<P0>(locale: P0, pattern: *const u16, patternlength: i32, result: *mut u16, resultlength: i32, parseerror: *mut UParseError, status: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn u_formatMessageWithError(locale : windows_core::PCSTR, pattern : *const u16, patternlength : i32, result : *mut u16, resultlength : i32, parseerror : *mut UParseError, status : *mut UErrorCode) -> i32);
    u_formatMessageWithError(locale.param().abi(), pattern, patternlength, result, resultlength, parseerror, status)
}
#[inline]
pub unsafe fn u_getBidiPairedBracket(c: i32) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn u_getBidiPairedBracket(c : i32) -> i32);
    u_getBidiPairedBracket(c)
}
#[inline]
pub unsafe fn u_getBinaryPropertySet(property: UProperty, perrorcode: *mut UErrorCode) -> *mut USet {
    windows_targets::link!("icu.dll" "cdecl" fn u_getBinaryPropertySet(property : UProperty, perrorcode : *mut UErrorCode) -> *mut USet);
    u_getBinaryPropertySet(property, perrorcode)
}
#[inline]
pub unsafe fn u_getCombiningClass(c: i32) -> u8 {
    windows_targets::link!("icu.dll" "cdecl" fn u_getCombiningClass(c : i32) -> u8);
    u_getCombiningClass(c)
}
#[inline]
pub unsafe fn u_getDataVersion(dataversionfillin: *mut u8, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn u_getDataVersion(dataversionfillin : *mut u8, status : *mut UErrorCode));
    u_getDataVersion(dataversionfillin, status)
}
#[inline]
pub unsafe fn u_getFC_NFKC_Closure(c: i32, dest: *mut u16, destcapacity: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn u_getFC_NFKC_Closure(c : i32, dest : *mut u16, destcapacity : i32, perrorcode : *mut UErrorCode) -> i32);
    u_getFC_NFKC_Closure(c, dest, destcapacity, perrorcode)
}
#[inline]
pub unsafe fn u_getIntPropertyMap(property: UProperty, perrorcode: *mut UErrorCode) -> *mut UCPMap {
    windows_targets::link!("icu.dll" "cdecl" fn u_getIntPropertyMap(property : UProperty, perrorcode : *mut UErrorCode) -> *mut UCPMap);
    u_getIntPropertyMap(property, perrorcode)
}
#[inline]
pub unsafe fn u_getIntPropertyMaxValue(which: UProperty) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn u_getIntPropertyMaxValue(which : UProperty) -> i32);
    u_getIntPropertyMaxValue(which)
}
#[inline]
pub unsafe fn u_getIntPropertyMinValue(which: UProperty) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn u_getIntPropertyMinValue(which : UProperty) -> i32);
    u_getIntPropertyMinValue(which)
}
#[inline]
pub unsafe fn u_getIntPropertyValue(c: i32, which: UProperty) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn u_getIntPropertyValue(c : i32, which : UProperty) -> i32);
    u_getIntPropertyValue(c, which)
}
#[inline]
pub unsafe fn u_getNumericValue(c: i32) -> f64 {
    windows_targets::link!("icu.dll" "cdecl" fn u_getNumericValue(c : i32) -> f64);
    u_getNumericValue(c)
}
#[inline]
pub unsafe fn u_getPropertyEnum<P0>(alias: P0) -> UProperty
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn u_getPropertyEnum(alias : windows_core::PCSTR) -> UProperty);
    u_getPropertyEnum(alias.param().abi())
}
#[inline]
pub unsafe fn u_getPropertyName(property: UProperty, namechoice: UPropertyNameChoice) -> windows_core::PCSTR {
    windows_targets::link!("icu.dll" "cdecl" fn u_getPropertyName(property : UProperty, namechoice : UPropertyNameChoice) -> windows_core::PCSTR);
    u_getPropertyName(property, namechoice)
}
#[inline]
pub unsafe fn u_getPropertyValueEnum<P0>(property: UProperty, alias: P0) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn u_getPropertyValueEnum(property : UProperty, alias : windows_core::PCSTR) -> i32);
    u_getPropertyValueEnum(property, alias.param().abi())
}
#[inline]
pub unsafe fn u_getPropertyValueName(property: UProperty, value: i32, namechoice: UPropertyNameChoice) -> windows_core::PCSTR {
    windows_targets::link!("icu.dll" "cdecl" fn u_getPropertyValueName(property : UProperty, value : i32, namechoice : UPropertyNameChoice) -> windows_core::PCSTR);
    u_getPropertyValueName(property, value, namechoice)
}
#[inline]
pub unsafe fn u_getUnicodeVersion(versionarray: *mut u8) {
    windows_targets::link!("icu.dll" "cdecl" fn u_getUnicodeVersion(versionarray : *mut u8));
    u_getUnicodeVersion(versionarray)
}
#[inline]
pub unsafe fn u_getVersion(versionarray: *mut u8) {
    windows_targets::link!("icu.dll" "cdecl" fn u_getVersion(versionarray : *mut u8));
    u_getVersion(versionarray)
}
#[inline]
pub unsafe fn u_hasBinaryProperty(c: i32, which: UProperty) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn u_hasBinaryProperty(c : i32, which : UProperty) -> i8);
    u_hasBinaryProperty(c, which)
}
#[inline]
pub unsafe fn u_init(status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn u_init(status : *mut UErrorCode));
    u_init(status)
}
#[inline]
pub unsafe fn u_isIDIgnorable(c: i32) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn u_isIDIgnorable(c : i32) -> i8);
    u_isIDIgnorable(c)
}
#[inline]
pub unsafe fn u_isIDPart(c: i32) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn u_isIDPart(c : i32) -> i8);
    u_isIDPart(c)
}
#[inline]
pub unsafe fn u_isIDStart(c: i32) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn u_isIDStart(c : i32) -> i8);
    u_isIDStart(c)
}
#[inline]
pub unsafe fn u_isISOControl(c: i32) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn u_isISOControl(c : i32) -> i8);
    u_isISOControl(c)
}
#[inline]
pub unsafe fn u_isJavaIDPart(c: i32) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn u_isJavaIDPart(c : i32) -> i8);
    u_isJavaIDPart(c)
}
#[inline]
pub unsafe fn u_isJavaIDStart(c: i32) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn u_isJavaIDStart(c : i32) -> i8);
    u_isJavaIDStart(c)
}
#[inline]
pub unsafe fn u_isJavaSpaceChar(c: i32) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn u_isJavaSpaceChar(c : i32) -> i8);
    u_isJavaSpaceChar(c)
}
#[inline]
pub unsafe fn u_isMirrored(c: i32) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn u_isMirrored(c : i32) -> i8);
    u_isMirrored(c)
}
#[inline]
pub unsafe fn u_isUAlphabetic(c: i32) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn u_isUAlphabetic(c : i32) -> i8);
    u_isUAlphabetic(c)
}
#[inline]
pub unsafe fn u_isULowercase(c: i32) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn u_isULowercase(c : i32) -> i8);
    u_isULowercase(c)
}
#[inline]
pub unsafe fn u_isUUppercase(c: i32) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn u_isUUppercase(c : i32) -> i8);
    u_isUUppercase(c)
}
#[inline]
pub unsafe fn u_isUWhiteSpace(c: i32) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn u_isUWhiteSpace(c : i32) -> i8);
    u_isUWhiteSpace(c)
}
#[inline]
pub unsafe fn u_isWhitespace(c: i32) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn u_isWhitespace(c : i32) -> i8);
    u_isWhitespace(c)
}
#[inline]
pub unsafe fn u_isalnum(c: i32) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn u_isalnum(c : i32) -> i8);
    u_isalnum(c)
}
#[inline]
pub unsafe fn u_isalpha(c: i32) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn u_isalpha(c : i32) -> i8);
    u_isalpha(c)
}
#[inline]
pub unsafe fn u_isbase(c: i32) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn u_isbase(c : i32) -> i8);
    u_isbase(c)
}
#[inline]
pub unsafe fn u_isblank(c: i32) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn u_isblank(c : i32) -> i8);
    u_isblank(c)
}
#[inline]
pub unsafe fn u_iscntrl(c: i32) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn u_iscntrl(c : i32) -> i8);
    u_iscntrl(c)
}
#[inline]
pub unsafe fn u_isdefined(c: i32) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn u_isdefined(c : i32) -> i8);
    u_isdefined(c)
}
#[inline]
pub unsafe fn u_isdigit(c: i32) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn u_isdigit(c : i32) -> i8);
    u_isdigit(c)
}
#[inline]
pub unsafe fn u_isgraph(c: i32) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn u_isgraph(c : i32) -> i8);
    u_isgraph(c)
}
#[inline]
pub unsafe fn u_islower(c: i32) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn u_islower(c : i32) -> i8);
    u_islower(c)
}
#[inline]
pub unsafe fn u_isprint(c: i32) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn u_isprint(c : i32) -> i8);
    u_isprint(c)
}
#[inline]
pub unsafe fn u_ispunct(c: i32) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn u_ispunct(c : i32) -> i8);
    u_ispunct(c)
}
#[inline]
pub unsafe fn u_isspace(c: i32) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn u_isspace(c : i32) -> i8);
    u_isspace(c)
}
#[inline]
pub unsafe fn u_istitle(c: i32) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn u_istitle(c : i32) -> i8);
    u_istitle(c)
}
#[inline]
pub unsafe fn u_isupper(c: i32) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn u_isupper(c : i32) -> i8);
    u_isupper(c)
}
#[inline]
pub unsafe fn u_isxdigit(c: i32) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn u_isxdigit(c : i32) -> i8);
    u_isxdigit(c)
}
#[inline]
pub unsafe fn u_memcasecmp(s1: *const u16, s2: *const u16, length: i32, options: u32) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn u_memcasecmp(s1 : *const u16, s2 : *const u16, length : i32, options : u32) -> i32);
    u_memcasecmp(s1, s2, length, options)
}
#[inline]
pub unsafe fn u_memchr(s: *const u16, c: u16, count: i32) -> *mut u16 {
    windows_targets::link!("icu.dll" "cdecl" fn u_memchr(s : *const u16, c : u16, count : i32) -> *mut u16);
    u_memchr(s, c, count)
}
#[inline]
pub unsafe fn u_memchr32(s: *const u16, c: i32, count: i32) -> *mut u16 {
    windows_targets::link!("icu.dll" "cdecl" fn u_memchr32(s : *const u16, c : i32, count : i32) -> *mut u16);
    u_memchr32(s, c, count)
}
#[inline]
pub unsafe fn u_memcmp(buf1: *const u16, buf2: *const u16, count: i32) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn u_memcmp(buf1 : *const u16, buf2 : *const u16, count : i32) -> i32);
    u_memcmp(buf1, buf2, count)
}
#[inline]
pub unsafe fn u_memcmpCodePointOrder(s1: *const u16, s2: *const u16, count: i32) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn u_memcmpCodePointOrder(s1 : *const u16, s2 : *const u16, count : i32) -> i32);
    u_memcmpCodePointOrder(s1, s2, count)
}
#[inline]
pub unsafe fn u_memcpy(dest: *mut u16, src: *const u16, count: i32) -> *mut u16 {
    windows_targets::link!("icu.dll" "cdecl" fn u_memcpy(dest : *mut u16, src : *const u16, count : i32) -> *mut u16);
    u_memcpy(dest, src, count)
}
#[inline]
pub unsafe fn u_memmove(dest: *mut u16, src: *const u16, count: i32) -> *mut u16 {
    windows_targets::link!("icu.dll" "cdecl" fn u_memmove(dest : *mut u16, src : *const u16, count : i32) -> *mut u16);
    u_memmove(dest, src, count)
}
#[inline]
pub unsafe fn u_memrchr(s: *const u16, c: u16, count: i32) -> *mut u16 {
    windows_targets::link!("icu.dll" "cdecl" fn u_memrchr(s : *const u16, c : u16, count : i32) -> *mut u16);
    u_memrchr(s, c, count)
}
#[inline]
pub unsafe fn u_memrchr32(s: *const u16, c: i32, count: i32) -> *mut u16 {
    windows_targets::link!("icu.dll" "cdecl" fn u_memrchr32(s : *const u16, c : i32, count : i32) -> *mut u16);
    u_memrchr32(s, c, count)
}
#[inline]
pub unsafe fn u_memset(dest: *mut u16, c: u16, count: i32) -> *mut u16 {
    windows_targets::link!("icu.dll" "cdecl" fn u_memset(dest : *mut u16, c : u16, count : i32) -> *mut u16);
    u_memset(dest, c, count)
}
#[inline]
pub unsafe fn u_parseMessage<P0>(locale: P0, pattern: *const u16, patternlength: i32, source: *const u16, sourcelength: i32, status: *mut UErrorCode)
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn u_parseMessage(locale : windows_core::PCSTR, pattern : *const u16, patternlength : i32, source : *const u16, sourcelength : i32, status : *mut UErrorCode));
    u_parseMessage(locale.param().abi(), pattern, patternlength, source, sourcelength, status)
}
#[inline]
pub unsafe fn u_parseMessageWithError<P0>(locale: P0, pattern: *const u16, patternlength: i32, source: *const u16, sourcelength: i32, parseerror: *mut UParseError, status: *mut UErrorCode)
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn u_parseMessageWithError(locale : windows_core::PCSTR, pattern : *const u16, patternlength : i32, source : *const u16, sourcelength : i32, parseerror : *mut UParseError, status : *mut UErrorCode));
    u_parseMessageWithError(locale.param().abi(), pattern, patternlength, source, sourcelength, parseerror, status)
}
#[inline]
pub unsafe fn u_setMemoryFunctions(context: *const core::ffi::c_void, a: *mut UMemAllocFn, r: *mut UMemReallocFn, f: *mut UMemFreeFn, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn u_setMemoryFunctions(context : *const core::ffi::c_void, a : *mut UMemAllocFn, r : *mut UMemReallocFn, f : *mut UMemFreeFn, status : *mut UErrorCode));
    u_setMemoryFunctions(context, a, r, f, status)
}
#[inline]
pub unsafe fn u_shapeArabic(source: *const u16, sourcelength: i32, dest: *mut u16, destsize: i32, options: u32, perrorcode: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn u_shapeArabic(source : *const u16, sourcelength : i32, dest : *mut u16, destsize : i32, options : u32, perrorcode : *mut UErrorCode) -> i32);
    u_shapeArabic(source, sourcelength, dest, destsize, options, perrorcode)
}
#[inline]
pub unsafe fn u_strCaseCompare(s1: *const u16, length1: i32, s2: *const u16, length2: i32, options: u32, perrorcode: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn u_strCaseCompare(s1 : *const u16, length1 : i32, s2 : *const u16, length2 : i32, options : u32, perrorcode : *mut UErrorCode) -> i32);
    u_strCaseCompare(s1, length1, s2, length2, options, perrorcode)
}
#[inline]
pub unsafe fn u_strCompare(s1: *const u16, length1: i32, s2: *const u16, length2: i32, codepointorder: i8) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn u_strCompare(s1 : *const u16, length1 : i32, s2 : *const u16, length2 : i32, codepointorder : i8) -> i32);
    u_strCompare(s1, length1, s2, length2, codepointorder)
}
#[inline]
pub unsafe fn u_strCompareIter(iter1: *mut UCharIterator, iter2: *mut UCharIterator, codepointorder: i8) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn u_strCompareIter(iter1 : *mut UCharIterator, iter2 : *mut UCharIterator, codepointorder : i8) -> i32);
    u_strCompareIter(iter1, iter2, codepointorder)
}
#[inline]
pub unsafe fn u_strFindFirst(s: *const u16, length: i32, substring: *const u16, sublength: i32) -> *mut u16 {
    windows_targets::link!("icu.dll" "cdecl" fn u_strFindFirst(s : *const u16, length : i32, substring : *const u16, sublength : i32) -> *mut u16);
    u_strFindFirst(s, length, substring, sublength)
}
#[inline]
pub unsafe fn u_strFindLast(s: *const u16, length: i32, substring: *const u16, sublength: i32) -> *mut u16 {
    windows_targets::link!("icu.dll" "cdecl" fn u_strFindLast(s : *const u16, length : i32, substring : *const u16, sublength : i32) -> *mut u16);
    u_strFindLast(s, length, substring, sublength)
}
#[inline]
pub unsafe fn u_strFoldCase(dest: *mut u16, destcapacity: i32, src: *const u16, srclength: i32, options: u32, perrorcode: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn u_strFoldCase(dest : *mut u16, destcapacity : i32, src : *const u16, srclength : i32, options : u32, perrorcode : *mut UErrorCode) -> i32);
    u_strFoldCase(dest, destcapacity, src, srclength, options, perrorcode)
}
#[inline]
pub unsafe fn u_strFromJavaModifiedUTF8WithSub<P0>(dest: *mut u16, destcapacity: i32, pdestlength: *mut i32, src: P0, srclength: i32, subchar: i32, pnumsubstitutions: *mut i32, perrorcode: *mut UErrorCode) -> *mut u16
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn u_strFromJavaModifiedUTF8WithSub(dest : *mut u16, destcapacity : i32, pdestlength : *mut i32, src : windows_core::PCSTR, srclength : i32, subchar : i32, pnumsubstitutions : *mut i32, perrorcode : *mut UErrorCode) -> *mut u16);
    u_strFromJavaModifiedUTF8WithSub(dest, destcapacity, pdestlength, src.param().abi(), srclength, subchar, pnumsubstitutions, perrorcode)
}
#[inline]
pub unsafe fn u_strFromUTF32(dest: *mut u16, destcapacity: i32, pdestlength: *mut i32, src: *const i32, srclength: i32, perrorcode: *mut UErrorCode) -> *mut u16 {
    windows_targets::link!("icu.dll" "cdecl" fn u_strFromUTF32(dest : *mut u16, destcapacity : i32, pdestlength : *mut i32, src : *const i32, srclength : i32, perrorcode : *mut UErrorCode) -> *mut u16);
    u_strFromUTF32(dest, destcapacity, pdestlength, src, srclength, perrorcode)
}
#[inline]
pub unsafe fn u_strFromUTF32WithSub(dest: *mut u16, destcapacity: i32, pdestlength: *mut i32, src: *const i32, srclength: i32, subchar: i32, pnumsubstitutions: *mut i32, perrorcode: *mut UErrorCode) -> *mut u16 {
    windows_targets::link!("icu.dll" "cdecl" fn u_strFromUTF32WithSub(dest : *mut u16, destcapacity : i32, pdestlength : *mut i32, src : *const i32, srclength : i32, subchar : i32, pnumsubstitutions : *mut i32, perrorcode : *mut UErrorCode) -> *mut u16);
    u_strFromUTF32WithSub(dest, destcapacity, pdestlength, src, srclength, subchar, pnumsubstitutions, perrorcode)
}
#[inline]
pub unsafe fn u_strFromUTF8<P0>(dest: *mut u16, destcapacity: i32, pdestlength: *mut i32, src: P0, srclength: i32, perrorcode: *mut UErrorCode) -> *mut u16
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn u_strFromUTF8(dest : *mut u16, destcapacity : i32, pdestlength : *mut i32, src : windows_core::PCSTR, srclength : i32, perrorcode : *mut UErrorCode) -> *mut u16);
    u_strFromUTF8(dest, destcapacity, pdestlength, src.param().abi(), srclength, perrorcode)
}
#[inline]
pub unsafe fn u_strFromUTF8Lenient<P0>(dest: *mut u16, destcapacity: i32, pdestlength: *mut i32, src: P0, srclength: i32, perrorcode: *mut UErrorCode) -> *mut u16
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn u_strFromUTF8Lenient(dest : *mut u16, destcapacity : i32, pdestlength : *mut i32, src : windows_core::PCSTR, srclength : i32, perrorcode : *mut UErrorCode) -> *mut u16);
    u_strFromUTF8Lenient(dest, destcapacity, pdestlength, src.param().abi(), srclength, perrorcode)
}
#[inline]
pub unsafe fn u_strFromUTF8WithSub<P0>(dest: *mut u16, destcapacity: i32, pdestlength: *mut i32, src: P0, srclength: i32, subchar: i32, pnumsubstitutions: *mut i32, perrorcode: *mut UErrorCode) -> *mut u16
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn u_strFromUTF8WithSub(dest : *mut u16, destcapacity : i32, pdestlength : *mut i32, src : windows_core::PCSTR, srclength : i32, subchar : i32, pnumsubstitutions : *mut i32, perrorcode : *mut UErrorCode) -> *mut u16);
    u_strFromUTF8WithSub(dest, destcapacity, pdestlength, src.param().abi(), srclength, subchar, pnumsubstitutions, perrorcode)
}
#[inline]
pub unsafe fn u_strFromWCS<P0>(dest: *mut u16, destcapacity: i32, pdestlength: *mut i32, src: P0, srclength: i32, perrorcode: *mut UErrorCode) -> *mut u16
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn u_strFromWCS(dest : *mut u16, destcapacity : i32, pdestlength : *mut i32, src : windows_core::PCWSTR, srclength : i32, perrorcode : *mut UErrorCode) -> *mut u16);
    u_strFromWCS(dest, destcapacity, pdestlength, src.param().abi(), srclength, perrorcode)
}
#[inline]
pub unsafe fn u_strHasMoreChar32Than(s: *const u16, length: i32, number: i32) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn u_strHasMoreChar32Than(s : *const u16, length : i32, number : i32) -> i8);
    u_strHasMoreChar32Than(s, length, number)
}
#[inline]
pub unsafe fn u_strToJavaModifiedUTF8<P0>(dest: P0, destcapacity: i32, pdestlength: *mut i32, src: *const u16, srclength: i32, perrorcode: *mut UErrorCode) -> windows_core::PSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn u_strToJavaModifiedUTF8(dest : windows_core::PCSTR, destcapacity : i32, pdestlength : *mut i32, src : *const u16, srclength : i32, perrorcode : *mut UErrorCode) -> windows_core::PSTR);
    u_strToJavaModifiedUTF8(dest.param().abi(), destcapacity, pdestlength, src, srclength, perrorcode)
}
#[inline]
pub unsafe fn u_strToLower<P0>(dest: *mut u16, destcapacity: i32, src: *const u16, srclength: i32, locale: P0, perrorcode: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn u_strToLower(dest : *mut u16, destcapacity : i32, src : *const u16, srclength : i32, locale : windows_core::PCSTR, perrorcode : *mut UErrorCode) -> i32);
    u_strToLower(dest, destcapacity, src, srclength, locale.param().abi(), perrorcode)
}
#[inline]
pub unsafe fn u_strToTitle<P0>(dest: *mut u16, destcapacity: i32, src: *const u16, srclength: i32, titleiter: *mut UBreakIterator, locale: P0, perrorcode: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn u_strToTitle(dest : *mut u16, destcapacity : i32, src : *const u16, srclength : i32, titleiter : *mut UBreakIterator, locale : windows_core::PCSTR, perrorcode : *mut UErrorCode) -> i32);
    u_strToTitle(dest, destcapacity, src, srclength, titleiter, locale.param().abi(), perrorcode)
}
#[inline]
pub unsafe fn u_strToUTF32(dest: *mut i32, destcapacity: i32, pdestlength: *mut i32, src: *const u16, srclength: i32, perrorcode: *mut UErrorCode) -> *mut i32 {
    windows_targets::link!("icu.dll" "cdecl" fn u_strToUTF32(dest : *mut i32, destcapacity : i32, pdestlength : *mut i32, src : *const u16, srclength : i32, perrorcode : *mut UErrorCode) -> *mut i32);
    u_strToUTF32(dest, destcapacity, pdestlength, src, srclength, perrorcode)
}
#[inline]
pub unsafe fn u_strToUTF32WithSub(dest: *mut i32, destcapacity: i32, pdestlength: *mut i32, src: *const u16, srclength: i32, subchar: i32, pnumsubstitutions: *mut i32, perrorcode: *mut UErrorCode) -> *mut i32 {
    windows_targets::link!("icu.dll" "cdecl" fn u_strToUTF32WithSub(dest : *mut i32, destcapacity : i32, pdestlength : *mut i32, src : *const u16, srclength : i32, subchar : i32, pnumsubstitutions : *mut i32, perrorcode : *mut UErrorCode) -> *mut i32);
    u_strToUTF32WithSub(dest, destcapacity, pdestlength, src, srclength, subchar, pnumsubstitutions, perrorcode)
}
#[inline]
pub unsafe fn u_strToUTF8<P0>(dest: P0, destcapacity: i32, pdestlength: *mut i32, src: *const u16, srclength: i32, perrorcode: *mut UErrorCode) -> windows_core::PSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn u_strToUTF8(dest : windows_core::PCSTR, destcapacity : i32, pdestlength : *mut i32, src : *const u16, srclength : i32, perrorcode : *mut UErrorCode) -> windows_core::PSTR);
    u_strToUTF8(dest.param().abi(), destcapacity, pdestlength, src, srclength, perrorcode)
}
#[inline]
pub unsafe fn u_strToUTF8WithSub<P0>(dest: P0, destcapacity: i32, pdestlength: *mut i32, src: *const u16, srclength: i32, subchar: i32, pnumsubstitutions: *mut i32, perrorcode: *mut UErrorCode) -> windows_core::PSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn u_strToUTF8WithSub(dest : windows_core::PCSTR, destcapacity : i32, pdestlength : *mut i32, src : *const u16, srclength : i32, subchar : i32, pnumsubstitutions : *mut i32, perrorcode : *mut UErrorCode) -> windows_core::PSTR);
    u_strToUTF8WithSub(dest.param().abi(), destcapacity, pdestlength, src, srclength, subchar, pnumsubstitutions, perrorcode)
}
#[inline]
pub unsafe fn u_strToUpper<P0>(dest: *mut u16, destcapacity: i32, src: *const u16, srclength: i32, locale: P0, perrorcode: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn u_strToUpper(dest : *mut u16, destcapacity : i32, src : *const u16, srclength : i32, locale : windows_core::PCSTR, perrorcode : *mut UErrorCode) -> i32);
    u_strToUpper(dest, destcapacity, src, srclength, locale.param().abi(), perrorcode)
}
#[inline]
pub unsafe fn u_strToWCS<P0>(dest: P0, destcapacity: i32, pdestlength: *mut i32, src: *const u16, srclength: i32, perrorcode: *mut UErrorCode) -> windows_core::PWSTR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn u_strToWCS(dest : windows_core::PCWSTR, destcapacity : i32, pdestlength : *mut i32, src : *const u16, srclength : i32, perrorcode : *mut UErrorCode) -> windows_core::PWSTR);
    u_strToWCS(dest.param().abi(), destcapacity, pdestlength, src, srclength, perrorcode)
}
#[inline]
pub unsafe fn u_strcasecmp(s1: *const u16, s2: *const u16, options: u32) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn u_strcasecmp(s1 : *const u16, s2 : *const u16, options : u32) -> i32);
    u_strcasecmp(s1, s2, options)
}
#[inline]
pub unsafe fn u_strcat(dst: *mut u16, src: *const u16) -> *mut u16 {
    windows_targets::link!("icu.dll" "cdecl" fn u_strcat(dst : *mut u16, src : *const u16) -> *mut u16);
    u_strcat(dst, src)
}
#[inline]
pub unsafe fn u_strchr(s: *const u16, c: u16) -> *mut u16 {
    windows_targets::link!("icu.dll" "cdecl" fn u_strchr(s : *const u16, c : u16) -> *mut u16);
    u_strchr(s, c)
}
#[inline]
pub unsafe fn u_strchr32(s: *const u16, c: i32) -> *mut u16 {
    windows_targets::link!("icu.dll" "cdecl" fn u_strchr32(s : *const u16, c : i32) -> *mut u16);
    u_strchr32(s, c)
}
#[inline]
pub unsafe fn u_strcmp(s1: *const u16, s2: *const u16) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn u_strcmp(s1 : *const u16, s2 : *const u16) -> i32);
    u_strcmp(s1, s2)
}
#[inline]
pub unsafe fn u_strcmpCodePointOrder(s1: *const u16, s2: *const u16) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn u_strcmpCodePointOrder(s1 : *const u16, s2 : *const u16) -> i32);
    u_strcmpCodePointOrder(s1, s2)
}
#[inline]
pub unsafe fn u_strcpy(dst: *mut u16, src: *const u16) -> *mut u16 {
    windows_targets::link!("icu.dll" "cdecl" fn u_strcpy(dst : *mut u16, src : *const u16) -> *mut u16);
    u_strcpy(dst, src)
}
#[inline]
pub unsafe fn u_strcspn(string: *const u16, matchset: *const u16) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn u_strcspn(string : *const u16, matchset : *const u16) -> i32);
    u_strcspn(string, matchset)
}
#[inline]
pub unsafe fn u_strlen(s: *const u16) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn u_strlen(s : *const u16) -> i32);
    u_strlen(s)
}
#[inline]
pub unsafe fn u_strncasecmp(s1: *const u16, s2: *const u16, n: i32, options: u32) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn u_strncasecmp(s1 : *const u16, s2 : *const u16, n : i32, options : u32) -> i32);
    u_strncasecmp(s1, s2, n, options)
}
#[inline]
pub unsafe fn u_strncat(dst: *mut u16, src: *const u16, n: i32) -> *mut u16 {
    windows_targets::link!("icu.dll" "cdecl" fn u_strncat(dst : *mut u16, src : *const u16, n : i32) -> *mut u16);
    u_strncat(dst, src, n)
}
#[inline]
pub unsafe fn u_strncmp(ucs1: *const u16, ucs2: *const u16, n: i32) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn u_strncmp(ucs1 : *const u16, ucs2 : *const u16, n : i32) -> i32);
    u_strncmp(ucs1, ucs2, n)
}
#[inline]
pub unsafe fn u_strncmpCodePointOrder(s1: *const u16, s2: *const u16, n: i32) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn u_strncmpCodePointOrder(s1 : *const u16, s2 : *const u16, n : i32) -> i32);
    u_strncmpCodePointOrder(s1, s2, n)
}
#[inline]
pub unsafe fn u_strncpy(dst: *mut u16, src: *const u16, n: i32) -> *mut u16 {
    windows_targets::link!("icu.dll" "cdecl" fn u_strncpy(dst : *mut u16, src : *const u16, n : i32) -> *mut u16);
    u_strncpy(dst, src, n)
}
#[inline]
pub unsafe fn u_strpbrk(string: *const u16, matchset: *const u16) -> *mut u16 {
    windows_targets::link!("icu.dll" "cdecl" fn u_strpbrk(string : *const u16, matchset : *const u16) -> *mut u16);
    u_strpbrk(string, matchset)
}
#[inline]
pub unsafe fn u_strrchr(s: *const u16, c: u16) -> *mut u16 {
    windows_targets::link!("icu.dll" "cdecl" fn u_strrchr(s : *const u16, c : u16) -> *mut u16);
    u_strrchr(s, c)
}
#[inline]
pub unsafe fn u_strrchr32(s: *const u16, c: i32) -> *mut u16 {
    windows_targets::link!("icu.dll" "cdecl" fn u_strrchr32(s : *const u16, c : i32) -> *mut u16);
    u_strrchr32(s, c)
}
#[inline]
pub unsafe fn u_strrstr(s: *const u16, substring: *const u16) -> *mut u16 {
    windows_targets::link!("icu.dll" "cdecl" fn u_strrstr(s : *const u16, substring : *const u16) -> *mut u16);
    u_strrstr(s, substring)
}
#[inline]
pub unsafe fn u_strspn(string: *const u16, matchset: *const u16) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn u_strspn(string : *const u16, matchset : *const u16) -> i32);
    u_strspn(string, matchset)
}
#[inline]
pub unsafe fn u_strstr(s: *const u16, substring: *const u16) -> *mut u16 {
    windows_targets::link!("icu.dll" "cdecl" fn u_strstr(s : *const u16, substring : *const u16) -> *mut u16);
    u_strstr(s, substring)
}
#[inline]
pub unsafe fn u_strtok_r(src: *mut u16, delim: *const u16, savestate: *mut *mut u16) -> *mut u16 {
    windows_targets::link!("icu.dll" "cdecl" fn u_strtok_r(src : *mut u16, delim : *const u16, savestate : *mut *mut u16) -> *mut u16);
    u_strtok_r(src, delim, savestate)
}
#[inline]
pub unsafe fn u_tolower(c: i32) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn u_tolower(c : i32) -> i32);
    u_tolower(c)
}
#[inline]
pub unsafe fn u_totitle(c: i32) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn u_totitle(c : i32) -> i32);
    u_totitle(c)
}
#[inline]
pub unsafe fn u_toupper(c: i32) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn u_toupper(c : i32) -> i32);
    u_toupper(c)
}
#[inline]
pub unsafe fn u_uastrcpy<P0>(dst: *mut u16, src: P0) -> *mut u16
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn u_uastrcpy(dst : *mut u16, src : windows_core::PCSTR) -> *mut u16);
    u_uastrcpy(dst, src.param().abi())
}
#[inline]
pub unsafe fn u_uastrncpy<P0>(dst: *mut u16, src: P0, n: i32) -> *mut u16
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn u_uastrncpy(dst : *mut u16, src : windows_core::PCSTR, n : i32) -> *mut u16);
    u_uastrncpy(dst, src.param().abi(), n)
}
#[inline]
pub unsafe fn u_unescape<P0>(src: P0, dest: *mut u16, destcapacity: i32) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn u_unescape(src : windows_core::PCSTR, dest : *mut u16, destcapacity : i32) -> i32);
    u_unescape(src.param().abi(), dest, destcapacity)
}
#[inline]
pub unsafe fn u_unescapeAt(charat: UNESCAPE_CHAR_AT, offset: *mut i32, length: i32, context: *mut core::ffi::c_void) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn u_unescapeAt(charat : UNESCAPE_CHAR_AT, offset : *mut i32, length : i32, context : *mut core::ffi::c_void) -> i32);
    u_unescapeAt(charat, offset, length, context)
}
#[inline]
pub unsafe fn u_versionFromString<P0>(versionarray: *mut u8, versionstring: P0)
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn u_versionFromString(versionarray : *mut u8, versionstring : windows_core::PCSTR));
    u_versionFromString(versionarray, versionstring.param().abi())
}
#[inline]
pub unsafe fn u_versionFromUString(versionarray: *mut u8, versionstring: *const u16) {
    windows_targets::link!("icu.dll" "cdecl" fn u_versionFromUString(versionarray : *mut u8, versionstring : *const u16));
    u_versionFromUString(versionarray, versionstring)
}
#[inline]
pub unsafe fn u_versionToString<P0>(versionarray: *const u8, versionstring: P0)
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn u_versionToString(versionarray : *const u8, versionstring : windows_core::PCSTR));
    u_versionToString(versionarray, versionstring.param().abi())
}
#[inline]
pub unsafe fn u_vformatMessage<P0>(locale: P0, pattern: *const u16, patternlength: i32, result: *mut u16, resultlength: i32, ap: *mut i8, status: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn u_vformatMessage(locale : windows_core::PCSTR, pattern : *const u16, patternlength : i32, result : *mut u16, resultlength : i32, ap : *mut i8, status : *mut UErrorCode) -> i32);
    u_vformatMessage(locale.param().abi(), pattern, patternlength, result, resultlength, ap, status)
}
#[inline]
pub unsafe fn u_vformatMessageWithError<P0>(locale: P0, pattern: *const u16, patternlength: i32, result: *mut u16, resultlength: i32, parseerror: *mut UParseError, ap: *mut i8, status: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn u_vformatMessageWithError(locale : windows_core::PCSTR, pattern : *const u16, patternlength : i32, result : *mut u16, resultlength : i32, parseerror : *mut UParseError, ap : *mut i8, status : *mut UErrorCode) -> i32);
    u_vformatMessageWithError(locale.param().abi(), pattern, patternlength, result, resultlength, parseerror, ap, status)
}
#[inline]
pub unsafe fn u_vparseMessage<P0>(locale: P0, pattern: *const u16, patternlength: i32, source: *const u16, sourcelength: i32, ap: *mut i8, status: *mut UErrorCode)
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn u_vparseMessage(locale : windows_core::PCSTR, pattern : *const u16, patternlength : i32, source : *const u16, sourcelength : i32, ap : *mut i8, status : *mut UErrorCode));
    u_vparseMessage(locale.param().abi(), pattern, patternlength, source, sourcelength, ap, status)
}
#[inline]
pub unsafe fn u_vparseMessageWithError<P0>(locale: P0, pattern: *const u16, patternlength: i32, source: *const u16, sourcelength: i32, ap: *mut i8, parseerror: *mut UParseError, status: *mut UErrorCode)
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn u_vparseMessageWithError(locale : windows_core::PCSTR, pattern : *const u16, patternlength : i32, source : *const u16, sourcelength : i32, ap : *mut i8, parseerror : *mut UParseError, status : *mut UErrorCode));
    u_vparseMessageWithError(locale.param().abi(), pattern, patternlength, source, sourcelength, ap, parseerror, status)
}
#[inline]
pub unsafe fn ubidi_close(pbidi: *mut UBiDi) {
    windows_targets::link!("icu.dll" "cdecl" fn ubidi_close(pbidi : *mut UBiDi));
    ubidi_close(pbidi)
}
#[inline]
pub unsafe fn ubidi_countParagraphs(pbidi: *mut UBiDi) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ubidi_countParagraphs(pbidi : *mut UBiDi) -> i32);
    ubidi_countParagraphs(pbidi)
}
#[inline]
pub unsafe fn ubidi_countRuns(pbidi: *mut UBiDi, perrorcode: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ubidi_countRuns(pbidi : *mut UBiDi, perrorcode : *mut UErrorCode) -> i32);
    ubidi_countRuns(pbidi, perrorcode)
}
#[inline]
pub unsafe fn ubidi_getBaseDirection(text: *const u16, length: i32) -> UBiDiDirection {
    windows_targets::link!("icu.dll" "cdecl" fn ubidi_getBaseDirection(text : *const u16, length : i32) -> UBiDiDirection);
    ubidi_getBaseDirection(text, length)
}
#[inline]
pub unsafe fn ubidi_getClassCallback(pbidi: *mut UBiDi, r#fn: *mut UBiDiClassCallback, context: *const *const core::ffi::c_void) {
    windows_targets::link!("icu.dll" "cdecl" fn ubidi_getClassCallback(pbidi : *mut UBiDi, r#fn : *mut UBiDiClassCallback, context : *const *const core::ffi::c_void));
    ubidi_getClassCallback(pbidi, r#fn, context)
}
#[inline]
pub unsafe fn ubidi_getCustomizedClass(pbidi: *mut UBiDi, c: i32) -> UCharDirection {
    windows_targets::link!("icu.dll" "cdecl" fn ubidi_getCustomizedClass(pbidi : *mut UBiDi, c : i32) -> UCharDirection);
    ubidi_getCustomizedClass(pbidi, c)
}
#[inline]
pub unsafe fn ubidi_getDirection(pbidi: *const UBiDi) -> UBiDiDirection {
    windows_targets::link!("icu.dll" "cdecl" fn ubidi_getDirection(pbidi : *const UBiDi) -> UBiDiDirection);
    ubidi_getDirection(pbidi)
}
#[inline]
pub unsafe fn ubidi_getLength(pbidi: *const UBiDi) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ubidi_getLength(pbidi : *const UBiDi) -> i32);
    ubidi_getLength(pbidi)
}
#[inline]
pub unsafe fn ubidi_getLevelAt(pbidi: *const UBiDi, charindex: i32) -> u8 {
    windows_targets::link!("icu.dll" "cdecl" fn ubidi_getLevelAt(pbidi : *const UBiDi, charindex : i32) -> u8);
    ubidi_getLevelAt(pbidi, charindex)
}
#[inline]
pub unsafe fn ubidi_getLevels(pbidi: *mut UBiDi, perrorcode: *mut UErrorCode) -> *mut u8 {
    windows_targets::link!("icu.dll" "cdecl" fn ubidi_getLevels(pbidi : *mut UBiDi, perrorcode : *mut UErrorCode) -> *mut u8);
    ubidi_getLevels(pbidi, perrorcode)
}
#[inline]
pub unsafe fn ubidi_getLogicalIndex(pbidi: *mut UBiDi, visualindex: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ubidi_getLogicalIndex(pbidi : *mut UBiDi, visualindex : i32, perrorcode : *mut UErrorCode) -> i32);
    ubidi_getLogicalIndex(pbidi, visualindex, perrorcode)
}
#[inline]
pub unsafe fn ubidi_getLogicalMap(pbidi: *mut UBiDi, indexmap: *mut i32, perrorcode: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn ubidi_getLogicalMap(pbidi : *mut UBiDi, indexmap : *mut i32, perrorcode : *mut UErrorCode));
    ubidi_getLogicalMap(pbidi, indexmap, perrorcode)
}
#[inline]
pub unsafe fn ubidi_getLogicalRun(pbidi: *const UBiDi, logicalposition: i32, plogicallimit: *mut i32, plevel: *mut u8) {
    windows_targets::link!("icu.dll" "cdecl" fn ubidi_getLogicalRun(pbidi : *const UBiDi, logicalposition : i32, plogicallimit : *mut i32, plevel : *mut u8));
    ubidi_getLogicalRun(pbidi, logicalposition, plogicallimit, plevel)
}
#[inline]
pub unsafe fn ubidi_getParaLevel(pbidi: *const UBiDi) -> u8 {
    windows_targets::link!("icu.dll" "cdecl" fn ubidi_getParaLevel(pbidi : *const UBiDi) -> u8);
    ubidi_getParaLevel(pbidi)
}
#[inline]
pub unsafe fn ubidi_getParagraph(pbidi: *const UBiDi, charindex: i32, pparastart: *mut i32, pparalimit: *mut i32, pparalevel: *mut u8, perrorcode: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ubidi_getParagraph(pbidi : *const UBiDi, charindex : i32, pparastart : *mut i32, pparalimit : *mut i32, pparalevel : *mut u8, perrorcode : *mut UErrorCode) -> i32);
    ubidi_getParagraph(pbidi, charindex, pparastart, pparalimit, pparalevel, perrorcode)
}
#[inline]
pub unsafe fn ubidi_getParagraphByIndex(pbidi: *const UBiDi, paraindex: i32, pparastart: *mut i32, pparalimit: *mut i32, pparalevel: *mut u8, perrorcode: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn ubidi_getParagraphByIndex(pbidi : *const UBiDi, paraindex : i32, pparastart : *mut i32, pparalimit : *mut i32, pparalevel : *mut u8, perrorcode : *mut UErrorCode));
    ubidi_getParagraphByIndex(pbidi, paraindex, pparastart, pparalimit, pparalevel, perrorcode)
}
#[inline]
pub unsafe fn ubidi_getProcessedLength(pbidi: *const UBiDi) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ubidi_getProcessedLength(pbidi : *const UBiDi) -> i32);
    ubidi_getProcessedLength(pbidi)
}
#[inline]
pub unsafe fn ubidi_getReorderingMode(pbidi: *mut UBiDi) -> UBiDiReorderingMode {
    windows_targets::link!("icu.dll" "cdecl" fn ubidi_getReorderingMode(pbidi : *mut UBiDi) -> UBiDiReorderingMode);
    ubidi_getReorderingMode(pbidi)
}
#[inline]
pub unsafe fn ubidi_getReorderingOptions(pbidi: *mut UBiDi) -> u32 {
    windows_targets::link!("icu.dll" "cdecl" fn ubidi_getReorderingOptions(pbidi : *mut UBiDi) -> u32);
    ubidi_getReorderingOptions(pbidi)
}
#[inline]
pub unsafe fn ubidi_getResultLength(pbidi: *const UBiDi) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ubidi_getResultLength(pbidi : *const UBiDi) -> i32);
    ubidi_getResultLength(pbidi)
}
#[inline]
pub unsafe fn ubidi_getText(pbidi: *const UBiDi) -> *mut u16 {
    windows_targets::link!("icu.dll" "cdecl" fn ubidi_getText(pbidi : *const UBiDi) -> *mut u16);
    ubidi_getText(pbidi)
}
#[inline]
pub unsafe fn ubidi_getVisualIndex(pbidi: *mut UBiDi, logicalindex: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ubidi_getVisualIndex(pbidi : *mut UBiDi, logicalindex : i32, perrorcode : *mut UErrorCode) -> i32);
    ubidi_getVisualIndex(pbidi, logicalindex, perrorcode)
}
#[inline]
pub unsafe fn ubidi_getVisualMap(pbidi: *mut UBiDi, indexmap: *mut i32, perrorcode: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn ubidi_getVisualMap(pbidi : *mut UBiDi, indexmap : *mut i32, perrorcode : *mut UErrorCode));
    ubidi_getVisualMap(pbidi, indexmap, perrorcode)
}
#[inline]
pub unsafe fn ubidi_getVisualRun(pbidi: *mut UBiDi, runindex: i32, plogicalstart: *mut i32, plength: *mut i32) -> UBiDiDirection {
    windows_targets::link!("icu.dll" "cdecl" fn ubidi_getVisualRun(pbidi : *mut UBiDi, runindex : i32, plogicalstart : *mut i32, plength : *mut i32) -> UBiDiDirection);
    ubidi_getVisualRun(pbidi, runindex, plogicalstart, plength)
}
#[inline]
pub unsafe fn ubidi_invertMap(srcmap: *const i32, destmap: *mut i32, length: i32) {
    windows_targets::link!("icu.dll" "cdecl" fn ubidi_invertMap(srcmap : *const i32, destmap : *mut i32, length : i32));
    ubidi_invertMap(srcmap, destmap, length)
}
#[inline]
pub unsafe fn ubidi_isInverse(pbidi: *mut UBiDi) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn ubidi_isInverse(pbidi : *mut UBiDi) -> i8);
    ubidi_isInverse(pbidi)
}
#[inline]
pub unsafe fn ubidi_isOrderParagraphsLTR(pbidi: *mut UBiDi) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn ubidi_isOrderParagraphsLTR(pbidi : *mut UBiDi) -> i8);
    ubidi_isOrderParagraphsLTR(pbidi)
}
#[inline]
pub unsafe fn ubidi_open() -> *mut UBiDi {
    windows_targets::link!("icu.dll" "cdecl" fn ubidi_open() -> *mut UBiDi);
    ubidi_open()
}
#[inline]
pub unsafe fn ubidi_openSized(maxlength: i32, maxruncount: i32, perrorcode: *mut UErrorCode) -> *mut UBiDi {
    windows_targets::link!("icu.dll" "cdecl" fn ubidi_openSized(maxlength : i32, maxruncount : i32, perrorcode : *mut UErrorCode) -> *mut UBiDi);
    ubidi_openSized(maxlength, maxruncount, perrorcode)
}
#[inline]
pub unsafe fn ubidi_orderParagraphsLTR(pbidi: *mut UBiDi, orderparagraphsltr: i8) {
    windows_targets::link!("icu.dll" "cdecl" fn ubidi_orderParagraphsLTR(pbidi : *mut UBiDi, orderparagraphsltr : i8));
    ubidi_orderParagraphsLTR(pbidi, orderparagraphsltr)
}
#[inline]
pub unsafe fn ubidi_reorderLogical(levels: *const u8, length: i32, indexmap: *mut i32) {
    windows_targets::link!("icu.dll" "cdecl" fn ubidi_reorderLogical(levels : *const u8, length : i32, indexmap : *mut i32));
    ubidi_reorderLogical(levels, length, indexmap)
}
#[inline]
pub unsafe fn ubidi_reorderVisual(levels: *const u8, length: i32, indexmap: *mut i32) {
    windows_targets::link!("icu.dll" "cdecl" fn ubidi_reorderVisual(levels : *const u8, length : i32, indexmap : *mut i32));
    ubidi_reorderVisual(levels, length, indexmap)
}
#[inline]
pub unsafe fn ubidi_setClassCallback(pbidi: *mut UBiDi, newfn: UBiDiClassCallback, newcontext: *const core::ffi::c_void, oldfn: *mut UBiDiClassCallback, oldcontext: *const *const core::ffi::c_void, perrorcode: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn ubidi_setClassCallback(pbidi : *mut UBiDi, newfn : UBiDiClassCallback, newcontext : *const core::ffi::c_void, oldfn : *mut UBiDiClassCallback, oldcontext : *const *const core::ffi::c_void, perrorcode : *mut UErrorCode));
    ubidi_setClassCallback(pbidi, newfn, newcontext, oldfn, oldcontext, perrorcode)
}
#[inline]
pub unsafe fn ubidi_setContext(pbidi: *mut UBiDi, prologue: *const u16, prolength: i32, epilogue: *const u16, epilength: i32, perrorcode: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn ubidi_setContext(pbidi : *mut UBiDi, prologue : *const u16, prolength : i32, epilogue : *const u16, epilength : i32, perrorcode : *mut UErrorCode));
    ubidi_setContext(pbidi, prologue, prolength, epilogue, epilength, perrorcode)
}
#[inline]
pub unsafe fn ubidi_setInverse(pbidi: *mut UBiDi, isinverse: i8) {
    windows_targets::link!("icu.dll" "cdecl" fn ubidi_setInverse(pbidi : *mut UBiDi, isinverse : i8));
    ubidi_setInverse(pbidi, isinverse)
}
#[inline]
pub unsafe fn ubidi_setLine(pparabidi: *const UBiDi, start: i32, limit: i32, plinebidi: *mut UBiDi, perrorcode: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn ubidi_setLine(pparabidi : *const UBiDi, start : i32, limit : i32, plinebidi : *mut UBiDi, perrorcode : *mut UErrorCode));
    ubidi_setLine(pparabidi, start, limit, plinebidi, perrorcode)
}
#[inline]
pub unsafe fn ubidi_setPara(pbidi: *mut UBiDi, text: *const u16, length: i32, paralevel: u8, embeddinglevels: *mut u8, perrorcode: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn ubidi_setPara(pbidi : *mut UBiDi, text : *const u16, length : i32, paralevel : u8, embeddinglevels : *mut u8, perrorcode : *mut UErrorCode));
    ubidi_setPara(pbidi, text, length, paralevel, embeddinglevels, perrorcode)
}
#[inline]
pub unsafe fn ubidi_setReorderingMode(pbidi: *mut UBiDi, reorderingmode: UBiDiReorderingMode) {
    windows_targets::link!("icu.dll" "cdecl" fn ubidi_setReorderingMode(pbidi : *mut UBiDi, reorderingmode : UBiDiReorderingMode));
    ubidi_setReorderingMode(pbidi, reorderingmode)
}
#[inline]
pub unsafe fn ubidi_setReorderingOptions(pbidi: *mut UBiDi, reorderingoptions: u32) {
    windows_targets::link!("icu.dll" "cdecl" fn ubidi_setReorderingOptions(pbidi : *mut UBiDi, reorderingoptions : u32));
    ubidi_setReorderingOptions(pbidi, reorderingoptions)
}
#[inline]
pub unsafe fn ubidi_writeReordered(pbidi: *mut UBiDi, dest: *mut u16, destsize: i32, options: u16, perrorcode: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ubidi_writeReordered(pbidi : *mut UBiDi, dest : *mut u16, destsize : i32, options : u16, perrorcode : *mut UErrorCode) -> i32);
    ubidi_writeReordered(pbidi, dest, destsize, options, perrorcode)
}
#[inline]
pub unsafe fn ubidi_writeReverse(src: *const u16, srclength: i32, dest: *mut u16, destsize: i32, options: u16, perrorcode: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ubidi_writeReverse(src : *const u16, srclength : i32, dest : *mut u16, destsize : i32, options : u16, perrorcode : *mut UErrorCode) -> i32);
    ubidi_writeReverse(src, srclength, dest, destsize, options, perrorcode)
}
#[inline]
pub unsafe fn ubiditransform_close(pbiditransform: *mut UBiDiTransform) {
    windows_targets::link!("icu.dll" "cdecl" fn ubiditransform_close(pbiditransform : *mut UBiDiTransform));
    ubiditransform_close(pbiditransform)
}
#[inline]
pub unsafe fn ubiditransform_open(perrorcode: *mut UErrorCode) -> *mut UBiDiTransform {
    windows_targets::link!("icu.dll" "cdecl" fn ubiditransform_open(perrorcode : *mut UErrorCode) -> *mut UBiDiTransform);
    ubiditransform_open(perrorcode)
}
#[inline]
pub unsafe fn ubiditransform_transform(pbiditransform: *mut UBiDiTransform, src: *const u16, srclength: i32, dest: *mut u16, destsize: i32, inparalevel: u8, inorder: UBiDiOrder, outparalevel: u8, outorder: UBiDiOrder, domirroring: UBiDiMirroring, shapingoptions: u32, perrorcode: *mut UErrorCode) -> u32 {
    windows_targets::link!("icu.dll" "cdecl" fn ubiditransform_transform(pbiditransform : *mut UBiDiTransform, src : *const u16, srclength : i32, dest : *mut u16, destsize : i32, inparalevel : u8, inorder : UBiDiOrder, outparalevel : u8, outorder : UBiDiOrder, domirroring : UBiDiMirroring, shapingoptions : u32, perrorcode : *mut UErrorCode) -> u32);
    ubiditransform_transform(pbiditransform, src, srclength, dest, destsize, inparalevel, inorder, outparalevel, outorder, domirroring, shapingoptions, perrorcode)
}
#[inline]
pub unsafe fn ublock_getCode(c: i32) -> UBlockCode {
    windows_targets::link!("icu.dll" "cdecl" fn ublock_getCode(c : i32) -> UBlockCode);
    ublock_getCode(c)
}
#[inline]
pub unsafe fn ubrk_close(bi: *mut UBreakIterator) {
    windows_targets::link!("icu.dll" "cdecl" fn ubrk_close(bi : *mut UBreakIterator));
    ubrk_close(bi)
}
#[inline]
pub unsafe fn ubrk_countAvailable() -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ubrk_countAvailable() -> i32);
    ubrk_countAvailable()
}
#[inline]
pub unsafe fn ubrk_current(bi: *const UBreakIterator) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ubrk_current(bi : *const UBreakIterator) -> i32);
    ubrk_current(bi)
}
#[inline]
pub unsafe fn ubrk_first(bi: *mut UBreakIterator) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ubrk_first(bi : *mut UBreakIterator) -> i32);
    ubrk_first(bi)
}
#[inline]
pub unsafe fn ubrk_following(bi: *mut UBreakIterator, offset: i32) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ubrk_following(bi : *mut UBreakIterator, offset : i32) -> i32);
    ubrk_following(bi, offset)
}
#[inline]
pub unsafe fn ubrk_getAvailable(index: i32) -> windows_core::PCSTR {
    windows_targets::link!("icu.dll" "cdecl" fn ubrk_getAvailable(index : i32) -> windows_core::PCSTR);
    ubrk_getAvailable(index)
}
#[inline]
pub unsafe fn ubrk_getBinaryRules(bi: *mut UBreakIterator, binaryrules: *mut u8, rulescapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ubrk_getBinaryRules(bi : *mut UBreakIterator, binaryrules : *mut u8, rulescapacity : i32, status : *mut UErrorCode) -> i32);
    ubrk_getBinaryRules(bi, binaryrules, rulescapacity, status)
}
#[inline]
pub unsafe fn ubrk_getLocaleByType(bi: *const UBreakIterator, r#type: ULocDataLocaleType, status: *mut UErrorCode) -> windows_core::PCSTR {
    windows_targets::link!("icu.dll" "cdecl" fn ubrk_getLocaleByType(bi : *const UBreakIterator, r#type : ULocDataLocaleType, status : *mut UErrorCode) -> windows_core::PCSTR);
    ubrk_getLocaleByType(bi, r#type, status)
}
#[inline]
pub unsafe fn ubrk_getRuleStatus(bi: *mut UBreakIterator) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ubrk_getRuleStatus(bi : *mut UBreakIterator) -> i32);
    ubrk_getRuleStatus(bi)
}
#[inline]
pub unsafe fn ubrk_getRuleStatusVec(bi: *mut UBreakIterator, fillinvec: *mut i32, capacity: i32, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ubrk_getRuleStatusVec(bi : *mut UBreakIterator, fillinvec : *mut i32, capacity : i32, status : *mut UErrorCode) -> i32);
    ubrk_getRuleStatusVec(bi, fillinvec, capacity, status)
}
#[inline]
pub unsafe fn ubrk_isBoundary(bi: *mut UBreakIterator, offset: i32) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn ubrk_isBoundary(bi : *mut UBreakIterator, offset : i32) -> i8);
    ubrk_isBoundary(bi, offset)
}
#[inline]
pub unsafe fn ubrk_last(bi: *mut UBreakIterator) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ubrk_last(bi : *mut UBreakIterator) -> i32);
    ubrk_last(bi)
}
#[inline]
pub unsafe fn ubrk_next(bi: *mut UBreakIterator) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ubrk_next(bi : *mut UBreakIterator) -> i32);
    ubrk_next(bi)
}
#[inline]
pub unsafe fn ubrk_open<P0>(r#type: UBreakIteratorType, locale: P0, text: *const u16, textlength: i32, status: *mut UErrorCode) -> *mut UBreakIterator
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ubrk_open(r#type : UBreakIteratorType, locale : windows_core::PCSTR, text : *const u16, textlength : i32, status : *mut UErrorCode) -> *mut UBreakIterator);
    ubrk_open(r#type, locale.param().abi(), text, textlength, status)
}
#[inline]
pub unsafe fn ubrk_openBinaryRules(binaryrules: *const u8, ruleslength: i32, text: *const u16, textlength: i32, status: *mut UErrorCode) -> *mut UBreakIterator {
    windows_targets::link!("icu.dll" "cdecl" fn ubrk_openBinaryRules(binaryrules : *const u8, ruleslength : i32, text : *const u16, textlength : i32, status : *mut UErrorCode) -> *mut UBreakIterator);
    ubrk_openBinaryRules(binaryrules, ruleslength, text, textlength, status)
}
#[inline]
pub unsafe fn ubrk_openRules(rules: *const u16, ruleslength: i32, text: *const u16, textlength: i32, parseerr: *mut UParseError, status: *mut UErrorCode) -> *mut UBreakIterator {
    windows_targets::link!("icu.dll" "cdecl" fn ubrk_openRules(rules : *const u16, ruleslength : i32, text : *const u16, textlength : i32, parseerr : *mut UParseError, status : *mut UErrorCode) -> *mut UBreakIterator);
    ubrk_openRules(rules, ruleslength, text, textlength, parseerr, status)
}
#[inline]
pub unsafe fn ubrk_preceding(bi: *mut UBreakIterator, offset: i32) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ubrk_preceding(bi : *mut UBreakIterator, offset : i32) -> i32);
    ubrk_preceding(bi, offset)
}
#[inline]
pub unsafe fn ubrk_previous(bi: *mut UBreakIterator) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ubrk_previous(bi : *mut UBreakIterator) -> i32);
    ubrk_previous(bi)
}
#[inline]
pub unsafe fn ubrk_refreshUText(bi: *mut UBreakIterator, text: *mut UText, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn ubrk_refreshUText(bi : *mut UBreakIterator, text : *mut UText, status : *mut UErrorCode));
    ubrk_refreshUText(bi, text, status)
}
#[inline]
pub unsafe fn ubrk_safeClone(bi: *const UBreakIterator, stackbuffer: *mut core::ffi::c_void, pbuffersize: *mut i32, status: *mut UErrorCode) -> *mut UBreakIterator {
    windows_targets::link!("icu.dll" "cdecl" fn ubrk_safeClone(bi : *const UBreakIterator, stackbuffer : *mut core::ffi::c_void, pbuffersize : *mut i32, status : *mut UErrorCode) -> *mut UBreakIterator);
    ubrk_safeClone(bi, stackbuffer, pbuffersize, status)
}
#[inline]
pub unsafe fn ubrk_setText(bi: *mut UBreakIterator, text: *const u16, textlength: i32, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn ubrk_setText(bi : *mut UBreakIterator, text : *const u16, textlength : i32, status : *mut UErrorCode));
    ubrk_setText(bi, text, textlength, status)
}
#[inline]
pub unsafe fn ubrk_setUText(bi: *mut UBreakIterator, text: *mut UText, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn ubrk_setUText(bi : *mut UBreakIterator, text : *mut UText, status : *mut UErrorCode));
    ubrk_setUText(bi, text, status)
}
#[inline]
pub unsafe fn ucal_add(cal: *mut *mut core::ffi::c_void, field: UCalendarDateFields, amount: i32, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn ucal_add(cal : *mut *mut core::ffi::c_void, field : UCalendarDateFields, amount : i32, status : *mut UErrorCode));
    ucal_add(cal, field, amount, status)
}
#[inline]
pub unsafe fn ucal_clear(calendar: *mut *mut core::ffi::c_void) {
    windows_targets::link!("icu.dll" "cdecl" fn ucal_clear(calendar : *mut *mut core::ffi::c_void));
    ucal_clear(calendar)
}
#[inline]
pub unsafe fn ucal_clearField(cal: *mut *mut core::ffi::c_void, field: UCalendarDateFields) {
    windows_targets::link!("icu.dll" "cdecl" fn ucal_clearField(cal : *mut *mut core::ffi::c_void, field : UCalendarDateFields));
    ucal_clearField(cal, field)
}
#[inline]
pub unsafe fn ucal_clone(cal: *const *const core::ffi::c_void, status: *mut UErrorCode) -> *mut *mut core::ffi::c_void {
    windows_targets::link!("icu.dll" "cdecl" fn ucal_clone(cal : *const *const core::ffi::c_void, status : *mut UErrorCode) -> *mut *mut core::ffi::c_void);
    ucal_clone(cal, status)
}
#[inline]
pub unsafe fn ucal_close(cal: *mut *mut core::ffi::c_void) {
    windows_targets::link!("icu.dll" "cdecl" fn ucal_close(cal : *mut *mut core::ffi::c_void));
    ucal_close(cal)
}
#[inline]
pub unsafe fn ucal_countAvailable() -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ucal_countAvailable() -> i32);
    ucal_countAvailable()
}
#[inline]
pub unsafe fn ucal_equivalentTo(cal1: *const *const core::ffi::c_void, cal2: *const *const core::ffi::c_void) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn ucal_equivalentTo(cal1 : *const *const core::ffi::c_void, cal2 : *const *const core::ffi::c_void) -> i8);
    ucal_equivalentTo(cal1, cal2)
}
#[inline]
pub unsafe fn ucal_get(cal: *const *const core::ffi::c_void, field: UCalendarDateFields, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ucal_get(cal : *const *const core::ffi::c_void, field : UCalendarDateFields, status : *mut UErrorCode) -> i32);
    ucal_get(cal, field, status)
}
#[inline]
pub unsafe fn ucal_getAttribute(cal: *const *const core::ffi::c_void, attr: UCalendarAttribute) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ucal_getAttribute(cal : *const *const core::ffi::c_void, attr : UCalendarAttribute) -> i32);
    ucal_getAttribute(cal, attr)
}
#[inline]
pub unsafe fn ucal_getAvailable(localeindex: i32) -> windows_core::PCSTR {
    windows_targets::link!("icu.dll" "cdecl" fn ucal_getAvailable(localeindex : i32) -> windows_core::PCSTR);
    ucal_getAvailable(localeindex)
}
#[inline]
pub unsafe fn ucal_getCanonicalTimeZoneID(id: *const u16, len: i32, result: *mut u16, resultcapacity: i32, issystemid: *mut i8, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ucal_getCanonicalTimeZoneID(id : *const u16, len : i32, result : *mut u16, resultcapacity : i32, issystemid : *mut i8, status : *mut UErrorCode) -> i32);
    ucal_getCanonicalTimeZoneID(id, len, result, resultcapacity, issystemid, status)
}
#[inline]
pub unsafe fn ucal_getDSTSavings(zoneid: *const u16, ec: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ucal_getDSTSavings(zoneid : *const u16, ec : *mut UErrorCode) -> i32);
    ucal_getDSTSavings(zoneid, ec)
}
#[inline]
pub unsafe fn ucal_getDayOfWeekType(cal: *const *const core::ffi::c_void, dayofweek: UCalendarDaysOfWeek, status: *mut UErrorCode) -> UCalendarWeekdayType {
    windows_targets::link!("icu.dll" "cdecl" fn ucal_getDayOfWeekType(cal : *const *const core::ffi::c_void, dayofweek : UCalendarDaysOfWeek, status : *mut UErrorCode) -> UCalendarWeekdayType);
    ucal_getDayOfWeekType(cal, dayofweek, status)
}
#[inline]
pub unsafe fn ucal_getDefaultTimeZone(result: *mut u16, resultcapacity: i32, ec: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ucal_getDefaultTimeZone(result : *mut u16, resultcapacity : i32, ec : *mut UErrorCode) -> i32);
    ucal_getDefaultTimeZone(result, resultcapacity, ec)
}
#[inline]
pub unsafe fn ucal_getFieldDifference(cal: *mut *mut core::ffi::c_void, target: f64, field: UCalendarDateFields, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ucal_getFieldDifference(cal : *mut *mut core::ffi::c_void, target : f64, field : UCalendarDateFields, status : *mut UErrorCode) -> i32);
    ucal_getFieldDifference(cal, target, field, status)
}
#[inline]
pub unsafe fn ucal_getGregorianChange(cal: *const *const core::ffi::c_void, perrorcode: *mut UErrorCode) -> f64 {
    windows_targets::link!("icu.dll" "cdecl" fn ucal_getGregorianChange(cal : *const *const core::ffi::c_void, perrorcode : *mut UErrorCode) -> f64);
    ucal_getGregorianChange(cal, perrorcode)
}
#[inline]
pub unsafe fn ucal_getHostTimeZone(result: *mut u16, resultcapacity: i32, ec: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ucal_getHostTimeZone(result : *mut u16, resultcapacity : i32, ec : *mut UErrorCode) -> i32);
    ucal_getHostTimeZone(result, resultcapacity, ec)
}
#[inline]
pub unsafe fn ucal_getKeywordValuesForLocale<P0, P1>(key: P0, locale: P1, commonlyused: i8, status: *mut UErrorCode) -> *mut UEnumeration
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ucal_getKeywordValuesForLocale(key : windows_core::PCSTR, locale : windows_core::PCSTR, commonlyused : i8, status : *mut UErrorCode) -> *mut UEnumeration);
    ucal_getKeywordValuesForLocale(key.param().abi(), locale.param().abi(), commonlyused, status)
}
#[inline]
pub unsafe fn ucal_getLimit(cal: *const *const core::ffi::c_void, field: UCalendarDateFields, r#type: UCalendarLimitType, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ucal_getLimit(cal : *const *const core::ffi::c_void, field : UCalendarDateFields, r#type : UCalendarLimitType, status : *mut UErrorCode) -> i32);
    ucal_getLimit(cal, field, r#type, status)
}
#[inline]
pub unsafe fn ucal_getLocaleByType(cal: *const *const core::ffi::c_void, r#type: ULocDataLocaleType, status: *mut UErrorCode) -> windows_core::PCSTR {
    windows_targets::link!("icu.dll" "cdecl" fn ucal_getLocaleByType(cal : *const *const core::ffi::c_void, r#type : ULocDataLocaleType, status : *mut UErrorCode) -> windows_core::PCSTR);
    ucal_getLocaleByType(cal, r#type, status)
}
#[inline]
pub unsafe fn ucal_getMillis(cal: *const *const core::ffi::c_void, status: *mut UErrorCode) -> f64 {
    windows_targets::link!("icu.dll" "cdecl" fn ucal_getMillis(cal : *const *const core::ffi::c_void, status : *mut UErrorCode) -> f64);
    ucal_getMillis(cal, status)
}
#[inline]
pub unsafe fn ucal_getNow() -> f64 {
    windows_targets::link!("icu.dll" "cdecl" fn ucal_getNow() -> f64);
    ucal_getNow()
}
#[inline]
pub unsafe fn ucal_getTZDataVersion(status: *mut UErrorCode) -> windows_core::PCSTR {
    windows_targets::link!("icu.dll" "cdecl" fn ucal_getTZDataVersion(status : *mut UErrorCode) -> windows_core::PCSTR);
    ucal_getTZDataVersion(status)
}
#[inline]
pub unsafe fn ucal_getTimeZoneDisplayName<P0>(cal: *const *const core::ffi::c_void, r#type: UCalendarDisplayNameType, locale: P0, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ucal_getTimeZoneDisplayName(cal : *const *const core::ffi::c_void, r#type : UCalendarDisplayNameType, locale : windows_core::PCSTR, result : *mut u16, resultlength : i32, status : *mut UErrorCode) -> i32);
    ucal_getTimeZoneDisplayName(cal, r#type, locale.param().abi(), result, resultlength, status)
}
#[inline]
pub unsafe fn ucal_getTimeZoneID(cal: *const *const core::ffi::c_void, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ucal_getTimeZoneID(cal : *const *const core::ffi::c_void, result : *mut u16, resultlength : i32, status : *mut UErrorCode) -> i32);
    ucal_getTimeZoneID(cal, result, resultlength, status)
}
#[inline]
pub unsafe fn ucal_getTimeZoneIDForWindowsID<P0>(winid: *const u16, len: i32, region: P0, id: *mut u16, idcapacity: i32, status: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ucal_getTimeZoneIDForWindowsID(winid : *const u16, len : i32, region : windows_core::PCSTR, id : *mut u16, idcapacity : i32, status : *mut UErrorCode) -> i32);
    ucal_getTimeZoneIDForWindowsID(winid, len, region.param().abi(), id, idcapacity, status)
}
#[inline]
pub unsafe fn ucal_getTimeZoneTransitionDate(cal: *const *const core::ffi::c_void, r#type: UTimeZoneTransitionType, transition: *mut f64, status: *mut UErrorCode) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn ucal_getTimeZoneTransitionDate(cal : *const *const core::ffi::c_void, r#type : UTimeZoneTransitionType, transition : *mut f64, status : *mut UErrorCode) -> i8);
    ucal_getTimeZoneTransitionDate(cal, r#type, transition, status)
}
#[inline]
pub unsafe fn ucal_getType(cal: *const *const core::ffi::c_void, status: *mut UErrorCode) -> windows_core::PCSTR {
    windows_targets::link!("icu.dll" "cdecl" fn ucal_getType(cal : *const *const core::ffi::c_void, status : *mut UErrorCode) -> windows_core::PCSTR);
    ucal_getType(cal, status)
}
#[inline]
pub unsafe fn ucal_getWeekendTransition(cal: *const *const core::ffi::c_void, dayofweek: UCalendarDaysOfWeek, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ucal_getWeekendTransition(cal : *const *const core::ffi::c_void, dayofweek : UCalendarDaysOfWeek, status : *mut UErrorCode) -> i32);
    ucal_getWeekendTransition(cal, dayofweek, status)
}
#[inline]
pub unsafe fn ucal_getWindowsTimeZoneID(id: *const u16, len: i32, winid: *mut u16, winidcapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ucal_getWindowsTimeZoneID(id : *const u16, len : i32, winid : *mut u16, winidcapacity : i32, status : *mut UErrorCode) -> i32);
    ucal_getWindowsTimeZoneID(id, len, winid, winidcapacity, status)
}
#[inline]
pub unsafe fn ucal_inDaylightTime(cal: *const *const core::ffi::c_void, status: *mut UErrorCode) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn ucal_inDaylightTime(cal : *const *const core::ffi::c_void, status : *mut UErrorCode) -> i8);
    ucal_inDaylightTime(cal, status)
}
#[inline]
pub unsafe fn ucal_isSet(cal: *const *const core::ffi::c_void, field: UCalendarDateFields) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn ucal_isSet(cal : *const *const core::ffi::c_void, field : UCalendarDateFields) -> i8);
    ucal_isSet(cal, field)
}
#[inline]
pub unsafe fn ucal_isWeekend(cal: *const *const core::ffi::c_void, date: f64, status: *mut UErrorCode) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn ucal_isWeekend(cal : *const *const core::ffi::c_void, date : f64, status : *mut UErrorCode) -> i8);
    ucal_isWeekend(cal, date, status)
}
#[inline]
pub unsafe fn ucal_open<P0>(zoneid: *const u16, len: i32, locale: P0, r#type: UCalendarType, status: *mut UErrorCode) -> *mut *mut core::ffi::c_void
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ucal_open(zoneid : *const u16, len : i32, locale : windows_core::PCSTR, r#type : UCalendarType, status : *mut UErrorCode) -> *mut *mut core::ffi::c_void);
    ucal_open(zoneid, len, locale.param().abi(), r#type, status)
}
#[inline]
pub unsafe fn ucal_openCountryTimeZones<P0>(country: P0, ec: *mut UErrorCode) -> *mut UEnumeration
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ucal_openCountryTimeZones(country : windows_core::PCSTR, ec : *mut UErrorCode) -> *mut UEnumeration);
    ucal_openCountryTimeZones(country.param().abi(), ec)
}
#[inline]
pub unsafe fn ucal_openTimeZoneIDEnumeration<P0>(zonetype: USystemTimeZoneType, region: P0, rawoffset: *const i32, ec: *mut UErrorCode) -> *mut UEnumeration
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ucal_openTimeZoneIDEnumeration(zonetype : USystemTimeZoneType, region : windows_core::PCSTR, rawoffset : *const i32, ec : *mut UErrorCode) -> *mut UEnumeration);
    ucal_openTimeZoneIDEnumeration(zonetype, region.param().abi(), rawoffset, ec)
}
#[inline]
pub unsafe fn ucal_openTimeZones(ec: *mut UErrorCode) -> *mut UEnumeration {
    windows_targets::link!("icu.dll" "cdecl" fn ucal_openTimeZones(ec : *mut UErrorCode) -> *mut UEnumeration);
    ucal_openTimeZones(ec)
}
#[inline]
pub unsafe fn ucal_roll(cal: *mut *mut core::ffi::c_void, field: UCalendarDateFields, amount: i32, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn ucal_roll(cal : *mut *mut core::ffi::c_void, field : UCalendarDateFields, amount : i32, status : *mut UErrorCode));
    ucal_roll(cal, field, amount, status)
}
#[inline]
pub unsafe fn ucal_set(cal: *mut *mut core::ffi::c_void, field: UCalendarDateFields, value: i32) {
    windows_targets::link!("icu.dll" "cdecl" fn ucal_set(cal : *mut *mut core::ffi::c_void, field : UCalendarDateFields, value : i32));
    ucal_set(cal, field, value)
}
#[inline]
pub unsafe fn ucal_setAttribute(cal: *mut *mut core::ffi::c_void, attr: UCalendarAttribute, newvalue: i32) {
    windows_targets::link!("icu.dll" "cdecl" fn ucal_setAttribute(cal : *mut *mut core::ffi::c_void, attr : UCalendarAttribute, newvalue : i32));
    ucal_setAttribute(cal, attr, newvalue)
}
#[inline]
pub unsafe fn ucal_setDate(cal: *mut *mut core::ffi::c_void, year: i32, month: i32, date: i32, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn ucal_setDate(cal : *mut *mut core::ffi::c_void, year : i32, month : i32, date : i32, status : *mut UErrorCode));
    ucal_setDate(cal, year, month, date, status)
}
#[inline]
pub unsafe fn ucal_setDateTime(cal: *mut *mut core::ffi::c_void, year: i32, month: i32, date: i32, hour: i32, minute: i32, second: i32, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn ucal_setDateTime(cal : *mut *mut core::ffi::c_void, year : i32, month : i32, date : i32, hour : i32, minute : i32, second : i32, status : *mut UErrorCode));
    ucal_setDateTime(cal, year, month, date, hour, minute, second, status)
}
#[inline]
pub unsafe fn ucal_setDefaultTimeZone(zoneid: *const u16, ec: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn ucal_setDefaultTimeZone(zoneid : *const u16, ec : *mut UErrorCode));
    ucal_setDefaultTimeZone(zoneid, ec)
}
#[inline]
pub unsafe fn ucal_setGregorianChange(cal: *mut *mut core::ffi::c_void, date: f64, perrorcode: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn ucal_setGregorianChange(cal : *mut *mut core::ffi::c_void, date : f64, perrorcode : *mut UErrorCode));
    ucal_setGregorianChange(cal, date, perrorcode)
}
#[inline]
pub unsafe fn ucal_setMillis(cal: *mut *mut core::ffi::c_void, datetime: f64, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn ucal_setMillis(cal : *mut *mut core::ffi::c_void, datetime : f64, status : *mut UErrorCode));
    ucal_setMillis(cal, datetime, status)
}
#[inline]
pub unsafe fn ucal_setTimeZone(cal: *mut *mut core::ffi::c_void, zoneid: *const u16, len: i32, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn ucal_setTimeZone(cal : *mut *mut core::ffi::c_void, zoneid : *const u16, len : i32, status : *mut UErrorCode));
    ucal_setTimeZone(cal, zoneid, len, status)
}
#[inline]
pub unsafe fn ucasemap_close(csm: *mut UCaseMap) {
    windows_targets::link!("icu.dll" "cdecl" fn ucasemap_close(csm : *mut UCaseMap));
    ucasemap_close(csm)
}
#[inline]
pub unsafe fn ucasemap_getBreakIterator(csm: *const UCaseMap) -> *mut UBreakIterator {
    windows_targets::link!("icu.dll" "cdecl" fn ucasemap_getBreakIterator(csm : *const UCaseMap) -> *mut UBreakIterator);
    ucasemap_getBreakIterator(csm)
}
#[inline]
pub unsafe fn ucasemap_getLocale(csm: *const UCaseMap) -> windows_core::PCSTR {
    windows_targets::link!("icu.dll" "cdecl" fn ucasemap_getLocale(csm : *const UCaseMap) -> windows_core::PCSTR);
    ucasemap_getLocale(csm)
}
#[inline]
pub unsafe fn ucasemap_getOptions(csm: *const UCaseMap) -> u32 {
    windows_targets::link!("icu.dll" "cdecl" fn ucasemap_getOptions(csm : *const UCaseMap) -> u32);
    ucasemap_getOptions(csm)
}
#[inline]
pub unsafe fn ucasemap_open<P0>(locale: P0, options: u32, perrorcode: *mut UErrorCode) -> *mut UCaseMap
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ucasemap_open(locale : windows_core::PCSTR, options : u32, perrorcode : *mut UErrorCode) -> *mut UCaseMap);
    ucasemap_open(locale.param().abi(), options, perrorcode)
}
#[inline]
pub unsafe fn ucasemap_setBreakIterator(csm: *mut UCaseMap, itertoadopt: *mut UBreakIterator, perrorcode: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn ucasemap_setBreakIterator(csm : *mut UCaseMap, itertoadopt : *mut UBreakIterator, perrorcode : *mut UErrorCode));
    ucasemap_setBreakIterator(csm, itertoadopt, perrorcode)
}
#[inline]
pub unsafe fn ucasemap_setLocale<P0>(csm: *mut UCaseMap, locale: P0, perrorcode: *mut UErrorCode)
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ucasemap_setLocale(csm : *mut UCaseMap, locale : windows_core::PCSTR, perrorcode : *mut UErrorCode));
    ucasemap_setLocale(csm, locale.param().abi(), perrorcode)
}
#[inline]
pub unsafe fn ucasemap_setOptions(csm: *mut UCaseMap, options: u32, perrorcode: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn ucasemap_setOptions(csm : *mut UCaseMap, options : u32, perrorcode : *mut UErrorCode));
    ucasemap_setOptions(csm, options, perrorcode)
}
#[inline]
pub unsafe fn ucasemap_toTitle(csm: *mut UCaseMap, dest: *mut u16, destcapacity: i32, src: *const u16, srclength: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ucasemap_toTitle(csm : *mut UCaseMap, dest : *mut u16, destcapacity : i32, src : *const u16, srclength : i32, perrorcode : *mut UErrorCode) -> i32);
    ucasemap_toTitle(csm, dest, destcapacity, src, srclength, perrorcode)
}
#[inline]
pub unsafe fn ucasemap_utf8FoldCase<P0, P1>(csm: *const UCaseMap, dest: P0, destcapacity: i32, src: P1, srclength: i32, perrorcode: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ucasemap_utf8FoldCase(csm : *const UCaseMap, dest : windows_core::PCSTR, destcapacity : i32, src : windows_core::PCSTR, srclength : i32, perrorcode : *mut UErrorCode) -> i32);
    ucasemap_utf8FoldCase(csm, dest.param().abi(), destcapacity, src.param().abi(), srclength, perrorcode)
}
#[inline]
pub unsafe fn ucasemap_utf8ToLower<P0, P1>(csm: *const UCaseMap, dest: P0, destcapacity: i32, src: P1, srclength: i32, perrorcode: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ucasemap_utf8ToLower(csm : *const UCaseMap, dest : windows_core::PCSTR, destcapacity : i32, src : windows_core::PCSTR, srclength : i32, perrorcode : *mut UErrorCode) -> i32);
    ucasemap_utf8ToLower(csm, dest.param().abi(), destcapacity, src.param().abi(), srclength, perrorcode)
}
#[inline]
pub unsafe fn ucasemap_utf8ToTitle<P0, P1>(csm: *mut UCaseMap, dest: P0, destcapacity: i32, src: P1, srclength: i32, perrorcode: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ucasemap_utf8ToTitle(csm : *mut UCaseMap, dest : windows_core::PCSTR, destcapacity : i32, src : windows_core::PCSTR, srclength : i32, perrorcode : *mut UErrorCode) -> i32);
    ucasemap_utf8ToTitle(csm, dest.param().abi(), destcapacity, src.param().abi(), srclength, perrorcode)
}
#[inline]
pub unsafe fn ucasemap_utf8ToUpper<P0, P1>(csm: *const UCaseMap, dest: P0, destcapacity: i32, src: P1, srclength: i32, perrorcode: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ucasemap_utf8ToUpper(csm : *const UCaseMap, dest : windows_core::PCSTR, destcapacity : i32, src : windows_core::PCSTR, srclength : i32, perrorcode : *mut UErrorCode) -> i32);
    ucasemap_utf8ToUpper(csm, dest.param().abi(), destcapacity, src.param().abi(), srclength, perrorcode)
}
#[inline]
pub unsafe fn ucfpos_close(ucfpos: *mut UConstrainedFieldPosition) {
    windows_targets::link!("icu.dll" "cdecl" fn ucfpos_close(ucfpos : *mut UConstrainedFieldPosition));
    ucfpos_close(ucfpos)
}
#[inline]
pub unsafe fn ucfpos_constrainCategory(ucfpos: *mut UConstrainedFieldPosition, category: i32, ec: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn ucfpos_constrainCategory(ucfpos : *mut UConstrainedFieldPosition, category : i32, ec : *mut UErrorCode));
    ucfpos_constrainCategory(ucfpos, category, ec)
}
#[inline]
pub unsafe fn ucfpos_constrainField(ucfpos: *mut UConstrainedFieldPosition, category: i32, field: i32, ec: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn ucfpos_constrainField(ucfpos : *mut UConstrainedFieldPosition, category : i32, field : i32, ec : *mut UErrorCode));
    ucfpos_constrainField(ucfpos, category, field, ec)
}
#[inline]
pub unsafe fn ucfpos_getCategory(ucfpos: *const UConstrainedFieldPosition, ec: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ucfpos_getCategory(ucfpos : *const UConstrainedFieldPosition, ec : *mut UErrorCode) -> i32);
    ucfpos_getCategory(ucfpos, ec)
}
#[inline]
pub unsafe fn ucfpos_getField(ucfpos: *const UConstrainedFieldPosition, ec: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ucfpos_getField(ucfpos : *const UConstrainedFieldPosition, ec : *mut UErrorCode) -> i32);
    ucfpos_getField(ucfpos, ec)
}
#[inline]
pub unsafe fn ucfpos_getIndexes(ucfpos: *const UConstrainedFieldPosition, pstart: *mut i32, plimit: *mut i32, ec: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn ucfpos_getIndexes(ucfpos : *const UConstrainedFieldPosition, pstart : *mut i32, plimit : *mut i32, ec : *mut UErrorCode));
    ucfpos_getIndexes(ucfpos, pstart, plimit, ec)
}
#[inline]
pub unsafe fn ucfpos_getInt64IterationContext(ucfpos: *const UConstrainedFieldPosition, ec: *mut UErrorCode) -> i64 {
    windows_targets::link!("icu.dll" "cdecl" fn ucfpos_getInt64IterationContext(ucfpos : *const UConstrainedFieldPosition, ec : *mut UErrorCode) -> i64);
    ucfpos_getInt64IterationContext(ucfpos, ec)
}
#[inline]
pub unsafe fn ucfpos_matchesField(ucfpos: *const UConstrainedFieldPosition, category: i32, field: i32, ec: *mut UErrorCode) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn ucfpos_matchesField(ucfpos : *const UConstrainedFieldPosition, category : i32, field : i32, ec : *mut UErrorCode) -> i8);
    ucfpos_matchesField(ucfpos, category, field, ec)
}
#[inline]
pub unsafe fn ucfpos_open(ec: *mut UErrorCode) -> *mut UConstrainedFieldPosition {
    windows_targets::link!("icu.dll" "cdecl" fn ucfpos_open(ec : *mut UErrorCode) -> *mut UConstrainedFieldPosition);
    ucfpos_open(ec)
}
#[inline]
pub unsafe fn ucfpos_reset(ucfpos: *mut UConstrainedFieldPosition, ec: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn ucfpos_reset(ucfpos : *mut UConstrainedFieldPosition, ec : *mut UErrorCode));
    ucfpos_reset(ucfpos, ec)
}
#[inline]
pub unsafe fn ucfpos_setInt64IterationContext(ucfpos: *mut UConstrainedFieldPosition, context: i64, ec: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn ucfpos_setInt64IterationContext(ucfpos : *mut UConstrainedFieldPosition, context : i64, ec : *mut UErrorCode));
    ucfpos_setInt64IterationContext(ucfpos, context, ec)
}
#[inline]
pub unsafe fn ucfpos_setState(ucfpos: *mut UConstrainedFieldPosition, category: i32, field: i32, start: i32, limit: i32, ec: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn ucfpos_setState(ucfpos : *mut UConstrainedFieldPosition, category : i32, field : i32, start : i32, limit : i32, ec : *mut UErrorCode));
    ucfpos_setState(ucfpos, category, field, start, limit, ec)
}
#[inline]
pub unsafe fn ucnv_cbFromUWriteBytes<P0>(args: *mut UConverterFromUnicodeArgs, source: P0, length: i32, offsetindex: i32, err: *mut UErrorCode)
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_cbFromUWriteBytes(args : *mut UConverterFromUnicodeArgs, source : windows_core::PCSTR, length : i32, offsetindex : i32, err : *mut UErrorCode));
    ucnv_cbFromUWriteBytes(args, source.param().abi(), length, offsetindex, err)
}
#[inline]
pub unsafe fn ucnv_cbFromUWriteSub(args: *mut UConverterFromUnicodeArgs, offsetindex: i32, err: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_cbFromUWriteSub(args : *mut UConverterFromUnicodeArgs, offsetindex : i32, err : *mut UErrorCode));
    ucnv_cbFromUWriteSub(args, offsetindex, err)
}
#[inline]
pub unsafe fn ucnv_cbFromUWriteUChars(args: *mut UConverterFromUnicodeArgs, source: *const *const u16, sourcelimit: *const u16, offsetindex: i32, err: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_cbFromUWriteUChars(args : *mut UConverterFromUnicodeArgs, source : *const *const u16, sourcelimit : *const u16, offsetindex : i32, err : *mut UErrorCode));
    ucnv_cbFromUWriteUChars(args, source, sourcelimit, offsetindex, err)
}
#[inline]
pub unsafe fn ucnv_cbToUWriteSub(args: *mut UConverterToUnicodeArgs, offsetindex: i32, err: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_cbToUWriteSub(args : *mut UConverterToUnicodeArgs, offsetindex : i32, err : *mut UErrorCode));
    ucnv_cbToUWriteSub(args, offsetindex, err)
}
#[inline]
pub unsafe fn ucnv_cbToUWriteUChars(args: *mut UConverterToUnicodeArgs, source: *const u16, length: i32, offsetindex: i32, err: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_cbToUWriteUChars(args : *mut UConverterToUnicodeArgs, source : *const u16, length : i32, offsetindex : i32, err : *mut UErrorCode));
    ucnv_cbToUWriteUChars(args, source, length, offsetindex, err)
}
#[inline]
pub unsafe fn ucnv_close(converter: *mut UConverter) {
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_close(converter : *mut UConverter));
    ucnv_close(converter)
}
#[inline]
pub unsafe fn ucnv_compareNames<P0, P1>(name1: P0, name2: P1) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_compareNames(name1 : windows_core::PCSTR, name2 : windows_core::PCSTR) -> i32);
    ucnv_compareNames(name1.param().abi(), name2.param().abi())
}
#[inline]
pub unsafe fn ucnv_convert<P0, P1, P2, P3>(toconvertername: P0, fromconvertername: P1, target: P2, targetcapacity: i32, source: P3, sourcelength: i32, perrorcode: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_convert(toconvertername : windows_core::PCSTR, fromconvertername : windows_core::PCSTR, target : windows_core::PCSTR, targetcapacity : i32, source : windows_core::PCSTR, sourcelength : i32, perrorcode : *mut UErrorCode) -> i32);
    ucnv_convert(toconvertername.param().abi(), fromconvertername.param().abi(), target.param().abi(), targetcapacity, source.param().abi(), sourcelength, perrorcode)
}
#[inline]
pub unsafe fn ucnv_convertEx<P0, P1>(targetcnv: *mut UConverter, sourcecnv: *mut UConverter, target: *mut *mut i8, targetlimit: P0, source: *const *const i8, sourcelimit: P1, pivotstart: *mut u16, pivotsource: *mut *mut u16, pivottarget: *mut *mut u16, pivotlimit: *const u16, reset: i8, flush: i8, perrorcode: *mut UErrorCode)
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_convertEx(targetcnv : *mut UConverter, sourcecnv : *mut UConverter, target : *mut *mut i8, targetlimit : windows_core::PCSTR, source : *const *const i8, sourcelimit : windows_core::PCSTR, pivotstart : *mut u16, pivotsource : *mut *mut u16, pivottarget : *mut *mut u16, pivotlimit : *const u16, reset : i8, flush : i8, perrorcode : *mut UErrorCode));
    ucnv_convertEx(targetcnv, sourcecnv, target, targetlimit.param().abi(), source, sourcelimit.param().abi(), pivotstart, pivotsource, pivottarget, pivotlimit, reset, flush, perrorcode)
}
#[inline]
pub unsafe fn ucnv_countAliases<P0>(alias: P0, perrorcode: *mut UErrorCode) -> u16
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_countAliases(alias : windows_core::PCSTR, perrorcode : *mut UErrorCode) -> u16);
    ucnv_countAliases(alias.param().abi(), perrorcode)
}
#[inline]
pub unsafe fn ucnv_countAvailable() -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_countAvailable() -> i32);
    ucnv_countAvailable()
}
#[inline]
pub unsafe fn ucnv_countStandards() -> u16 {
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_countStandards() -> u16);
    ucnv_countStandards()
}
#[inline]
pub unsafe fn ucnv_detectUnicodeSignature<P0>(source: P0, sourcelength: i32, signaturelength: *mut i32, perrorcode: *mut UErrorCode) -> windows_core::PCSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_detectUnicodeSignature(source : windows_core::PCSTR, sourcelength : i32, signaturelength : *mut i32, perrorcode : *mut UErrorCode) -> windows_core::PCSTR);
    ucnv_detectUnicodeSignature(source.param().abi(), sourcelength, signaturelength, perrorcode)
}
#[inline]
pub unsafe fn ucnv_fixFileSeparator(cnv: *const UConverter, source: *mut u16, sourcelen: i32) {
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_fixFileSeparator(cnv : *const UConverter, source : *mut u16, sourcelen : i32));
    ucnv_fixFileSeparator(cnv, source, sourcelen)
}
#[inline]
pub unsafe fn ucnv_flushCache() -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_flushCache() -> i32);
    ucnv_flushCache()
}
#[inline]
pub unsafe fn ucnv_fromAlgorithmic<P0, P1>(cnv: *mut UConverter, algorithmictype: UConverterType, target: P0, targetcapacity: i32, source: P1, sourcelength: i32, perrorcode: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_fromAlgorithmic(cnv : *mut UConverter, algorithmictype : UConverterType, target : windows_core::PCSTR, targetcapacity : i32, source : windows_core::PCSTR, sourcelength : i32, perrorcode : *mut UErrorCode) -> i32);
    ucnv_fromAlgorithmic(cnv, algorithmictype, target.param().abi(), targetcapacity, source.param().abi(), sourcelength, perrorcode)
}
#[inline]
pub unsafe fn ucnv_fromUChars<P0>(cnv: *mut UConverter, dest: P0, destcapacity: i32, src: *const u16, srclength: i32, perrorcode: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_fromUChars(cnv : *mut UConverter, dest : windows_core::PCSTR, destcapacity : i32, src : *const u16, srclength : i32, perrorcode : *mut UErrorCode) -> i32);
    ucnv_fromUChars(cnv, dest.param().abi(), destcapacity, src, srclength, perrorcode)
}
#[inline]
pub unsafe fn ucnv_fromUCountPending(cnv: *const UConverter, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_fromUCountPending(cnv : *const UConverter, status : *mut UErrorCode) -> i32);
    ucnv_fromUCountPending(cnv, status)
}
#[inline]
pub unsafe fn ucnv_fromUnicode<P0>(converter: *mut UConverter, target: *mut *mut i8, targetlimit: P0, source: *const *const u16, sourcelimit: *const u16, offsets: *mut i32, flush: i8, err: *mut UErrorCode)
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_fromUnicode(converter : *mut UConverter, target : *mut *mut i8, targetlimit : windows_core::PCSTR, source : *const *const u16, sourcelimit : *const u16, offsets : *mut i32, flush : i8, err : *mut UErrorCode));
    ucnv_fromUnicode(converter, target, targetlimit.param().abi(), source, sourcelimit, offsets, flush, err)
}
#[inline]
pub unsafe fn ucnv_getAlias<P0>(alias: P0, n: u16, perrorcode: *mut UErrorCode) -> windows_core::PCSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_getAlias(alias : windows_core::PCSTR, n : u16, perrorcode : *mut UErrorCode) -> windows_core::PCSTR);
    ucnv_getAlias(alias.param().abi(), n, perrorcode)
}
#[inline]
pub unsafe fn ucnv_getAliases<P0>(alias: P0, aliases: *const *const i8, perrorcode: *mut UErrorCode)
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_getAliases(alias : windows_core::PCSTR, aliases : *const *const i8, perrorcode : *mut UErrorCode));
    ucnv_getAliases(alias.param().abi(), aliases, perrorcode)
}
#[inline]
pub unsafe fn ucnv_getAvailableName(n: i32) -> windows_core::PCSTR {
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_getAvailableName(n : i32) -> windows_core::PCSTR);
    ucnv_getAvailableName(n)
}
#[inline]
pub unsafe fn ucnv_getCCSID(converter: *const UConverter, err: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_getCCSID(converter : *const UConverter, err : *mut UErrorCode) -> i32);
    ucnv_getCCSID(converter, err)
}
#[inline]
pub unsafe fn ucnv_getCanonicalName<P0, P1>(alias: P0, standard: P1, perrorcode: *mut UErrorCode) -> windows_core::PCSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_getCanonicalName(alias : windows_core::PCSTR, standard : windows_core::PCSTR, perrorcode : *mut UErrorCode) -> windows_core::PCSTR);
    ucnv_getCanonicalName(alias.param().abi(), standard.param().abi(), perrorcode)
}
#[inline]
pub unsafe fn ucnv_getDefaultName() -> windows_core::PCSTR {
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_getDefaultName() -> windows_core::PCSTR);
    ucnv_getDefaultName()
}
#[inline]
pub unsafe fn ucnv_getDisplayName<P0>(converter: *const UConverter, displaylocale: P0, displayname: *mut u16, displaynamecapacity: i32, err: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_getDisplayName(converter : *const UConverter, displaylocale : windows_core::PCSTR, displayname : *mut u16, displaynamecapacity : i32, err : *mut UErrorCode) -> i32);
    ucnv_getDisplayName(converter, displaylocale.param().abi(), displayname, displaynamecapacity, err)
}
#[inline]
pub unsafe fn ucnv_getFromUCallBack(converter: *const UConverter, action: *mut UConverterFromUCallback, context: *const *const core::ffi::c_void) {
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_getFromUCallBack(converter : *const UConverter, action : *mut UConverterFromUCallback, context : *const *const core::ffi::c_void));
    ucnv_getFromUCallBack(converter, action, context)
}
#[inline]
pub unsafe fn ucnv_getInvalidChars<P0>(converter: *const UConverter, errbytes: P0, len: *mut i8, err: *mut UErrorCode)
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_getInvalidChars(converter : *const UConverter, errbytes : windows_core::PCSTR, len : *mut i8, err : *mut UErrorCode));
    ucnv_getInvalidChars(converter, errbytes.param().abi(), len, err)
}
#[inline]
pub unsafe fn ucnv_getInvalidUChars(converter: *const UConverter, erruchars: *mut u16, len: *mut i8, err: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_getInvalidUChars(converter : *const UConverter, erruchars : *mut u16, len : *mut i8, err : *mut UErrorCode));
    ucnv_getInvalidUChars(converter, erruchars, len, err)
}
#[inline]
pub unsafe fn ucnv_getMaxCharSize(converter: *const UConverter) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_getMaxCharSize(converter : *const UConverter) -> i8);
    ucnv_getMaxCharSize(converter)
}
#[inline]
pub unsafe fn ucnv_getMinCharSize(converter: *const UConverter) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_getMinCharSize(converter : *const UConverter) -> i8);
    ucnv_getMinCharSize(converter)
}
#[inline]
pub unsafe fn ucnv_getName(converter: *const UConverter, err: *mut UErrorCode) -> windows_core::PCSTR {
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_getName(converter : *const UConverter, err : *mut UErrorCode) -> windows_core::PCSTR);
    ucnv_getName(converter, err)
}
#[inline]
pub unsafe fn ucnv_getNextUChar<P0>(converter: *mut UConverter, source: *const *const i8, sourcelimit: P0, err: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_getNextUChar(converter : *mut UConverter, source : *const *const i8, sourcelimit : windows_core::PCSTR, err : *mut UErrorCode) -> i32);
    ucnv_getNextUChar(converter, source, sourcelimit.param().abi(), err)
}
#[inline]
pub unsafe fn ucnv_getPlatform(converter: *const UConverter, err: *mut UErrorCode) -> UConverterPlatform {
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_getPlatform(converter : *const UConverter, err : *mut UErrorCode) -> UConverterPlatform);
    ucnv_getPlatform(converter, err)
}
#[inline]
pub unsafe fn ucnv_getStandard(n: u16, perrorcode: *mut UErrorCode) -> windows_core::PCSTR {
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_getStandard(n : u16, perrorcode : *mut UErrorCode) -> windows_core::PCSTR);
    ucnv_getStandard(n, perrorcode)
}
#[inline]
pub unsafe fn ucnv_getStandardName<P0, P1>(name: P0, standard: P1, perrorcode: *mut UErrorCode) -> windows_core::PCSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_getStandardName(name : windows_core::PCSTR, standard : windows_core::PCSTR, perrorcode : *mut UErrorCode) -> windows_core::PCSTR);
    ucnv_getStandardName(name.param().abi(), standard.param().abi(), perrorcode)
}
#[inline]
pub unsafe fn ucnv_getStarters(converter: *const UConverter, starters: *mut i8, err: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_getStarters(converter : *const UConverter, starters : *mut i8, err : *mut UErrorCode));
    ucnv_getStarters(converter, starters, err)
}
#[inline]
pub unsafe fn ucnv_getSubstChars<P0>(converter: *const UConverter, subchars: P0, len: *mut i8, err: *mut UErrorCode)
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_getSubstChars(converter : *const UConverter, subchars : windows_core::PCSTR, len : *mut i8, err : *mut UErrorCode));
    ucnv_getSubstChars(converter, subchars.param().abi(), len, err)
}
#[inline]
pub unsafe fn ucnv_getToUCallBack(converter: *const UConverter, action: *mut UConverterToUCallback, context: *const *const core::ffi::c_void) {
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_getToUCallBack(converter : *const UConverter, action : *mut UConverterToUCallback, context : *const *const core::ffi::c_void));
    ucnv_getToUCallBack(converter, action, context)
}
#[inline]
pub unsafe fn ucnv_getType(converter: *const UConverter) -> UConverterType {
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_getType(converter : *const UConverter) -> UConverterType);
    ucnv_getType(converter)
}
#[inline]
pub unsafe fn ucnv_getUnicodeSet(cnv: *const UConverter, setfillin: *mut USet, whichset: UConverterUnicodeSet, perrorcode: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_getUnicodeSet(cnv : *const UConverter, setfillin : *mut USet, whichset : UConverterUnicodeSet, perrorcode : *mut UErrorCode));
    ucnv_getUnicodeSet(cnv, setfillin, whichset, perrorcode)
}
#[inline]
pub unsafe fn ucnv_isAmbiguous(cnv: *const UConverter) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_isAmbiguous(cnv : *const UConverter) -> i8);
    ucnv_isAmbiguous(cnv)
}
#[inline]
pub unsafe fn ucnv_isFixedWidth(cnv: *mut UConverter, status: *mut UErrorCode) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_isFixedWidth(cnv : *mut UConverter, status : *mut UErrorCode) -> i8);
    ucnv_isFixedWidth(cnv, status)
}
#[inline]
pub unsafe fn ucnv_open<P0>(convertername: P0, err: *mut UErrorCode) -> *mut UConverter
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_open(convertername : windows_core::PCSTR, err : *mut UErrorCode) -> *mut UConverter);
    ucnv_open(convertername.param().abi(), err)
}
#[inline]
pub unsafe fn ucnv_openAllNames(perrorcode: *mut UErrorCode) -> *mut UEnumeration {
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_openAllNames(perrorcode : *mut UErrorCode) -> *mut UEnumeration);
    ucnv_openAllNames(perrorcode)
}
#[inline]
pub unsafe fn ucnv_openCCSID(codepage: i32, platform: UConverterPlatform, err: *mut UErrorCode) -> *mut UConverter {
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_openCCSID(codepage : i32, platform : UConverterPlatform, err : *mut UErrorCode) -> *mut UConverter);
    ucnv_openCCSID(codepage, platform, err)
}
#[inline]
pub unsafe fn ucnv_openPackage<P0, P1>(packagename: P0, convertername: P1, err: *mut UErrorCode) -> *mut UConverter
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_openPackage(packagename : windows_core::PCSTR, convertername : windows_core::PCSTR, err : *mut UErrorCode) -> *mut UConverter);
    ucnv_openPackage(packagename.param().abi(), convertername.param().abi(), err)
}
#[inline]
pub unsafe fn ucnv_openStandardNames<P0, P1>(convname: P0, standard: P1, perrorcode: *mut UErrorCode) -> *mut UEnumeration
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_openStandardNames(convname : windows_core::PCSTR, standard : windows_core::PCSTR, perrorcode : *mut UErrorCode) -> *mut UEnumeration);
    ucnv_openStandardNames(convname.param().abi(), standard.param().abi(), perrorcode)
}
#[inline]
pub unsafe fn ucnv_openU(name: *const u16, err: *mut UErrorCode) -> *mut UConverter {
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_openU(name : *const u16, err : *mut UErrorCode) -> *mut UConverter);
    ucnv_openU(name, err)
}
#[inline]
pub unsafe fn ucnv_reset(converter: *mut UConverter) {
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_reset(converter : *mut UConverter));
    ucnv_reset(converter)
}
#[inline]
pub unsafe fn ucnv_resetFromUnicode(converter: *mut UConverter) {
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_resetFromUnicode(converter : *mut UConverter));
    ucnv_resetFromUnicode(converter)
}
#[inline]
pub unsafe fn ucnv_resetToUnicode(converter: *mut UConverter) {
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_resetToUnicode(converter : *mut UConverter));
    ucnv_resetToUnicode(converter)
}
#[inline]
pub unsafe fn ucnv_safeClone(cnv: *const UConverter, stackbuffer: *mut core::ffi::c_void, pbuffersize: *mut i32, status: *mut UErrorCode) -> *mut UConverter {
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_safeClone(cnv : *const UConverter, stackbuffer : *mut core::ffi::c_void, pbuffersize : *mut i32, status : *mut UErrorCode) -> *mut UConverter);
    ucnv_safeClone(cnv, stackbuffer, pbuffersize, status)
}
#[inline]
pub unsafe fn ucnv_setDefaultName<P0>(name: P0)
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_setDefaultName(name : windows_core::PCSTR));
    ucnv_setDefaultName(name.param().abi())
}
#[inline]
pub unsafe fn ucnv_setFallback(cnv: *mut UConverter, usesfallback: i8) {
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_setFallback(cnv : *mut UConverter, usesfallback : i8));
    ucnv_setFallback(cnv, usesfallback)
}
#[inline]
pub unsafe fn ucnv_setFromUCallBack(converter: *mut UConverter, newaction: UConverterFromUCallback, newcontext: *const core::ffi::c_void, oldaction: *mut UConverterFromUCallback, oldcontext: *const *const core::ffi::c_void, err: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_setFromUCallBack(converter : *mut UConverter, newaction : UConverterFromUCallback, newcontext : *const core::ffi::c_void, oldaction : *mut UConverterFromUCallback, oldcontext : *const *const core::ffi::c_void, err : *mut UErrorCode));
    ucnv_setFromUCallBack(converter, newaction, newcontext, oldaction, oldcontext, err)
}
#[inline]
pub unsafe fn ucnv_setSubstChars<P0>(converter: *mut UConverter, subchars: P0, len: i8, err: *mut UErrorCode)
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_setSubstChars(converter : *mut UConverter, subchars : windows_core::PCSTR, len : i8, err : *mut UErrorCode));
    ucnv_setSubstChars(converter, subchars.param().abi(), len, err)
}
#[inline]
pub unsafe fn ucnv_setSubstString(cnv: *mut UConverter, s: *const u16, length: i32, err: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_setSubstString(cnv : *mut UConverter, s : *const u16, length : i32, err : *mut UErrorCode));
    ucnv_setSubstString(cnv, s, length, err)
}
#[inline]
pub unsafe fn ucnv_setToUCallBack(converter: *mut UConverter, newaction: UConverterToUCallback, newcontext: *const core::ffi::c_void, oldaction: *mut UConverterToUCallback, oldcontext: *const *const core::ffi::c_void, err: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_setToUCallBack(converter : *mut UConverter, newaction : UConverterToUCallback, newcontext : *const core::ffi::c_void, oldaction : *mut UConverterToUCallback, oldcontext : *const *const core::ffi::c_void, err : *mut UErrorCode));
    ucnv_setToUCallBack(converter, newaction, newcontext, oldaction, oldcontext, err)
}
#[inline]
pub unsafe fn ucnv_toAlgorithmic<P0, P1>(algorithmictype: UConverterType, cnv: *mut UConverter, target: P0, targetcapacity: i32, source: P1, sourcelength: i32, perrorcode: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_toAlgorithmic(algorithmictype : UConverterType, cnv : *mut UConverter, target : windows_core::PCSTR, targetcapacity : i32, source : windows_core::PCSTR, sourcelength : i32, perrorcode : *mut UErrorCode) -> i32);
    ucnv_toAlgorithmic(algorithmictype, cnv, target.param().abi(), targetcapacity, source.param().abi(), sourcelength, perrorcode)
}
#[inline]
pub unsafe fn ucnv_toUChars<P0>(cnv: *mut UConverter, dest: *mut u16, destcapacity: i32, src: P0, srclength: i32, perrorcode: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_toUChars(cnv : *mut UConverter, dest : *mut u16, destcapacity : i32, src : windows_core::PCSTR, srclength : i32, perrorcode : *mut UErrorCode) -> i32);
    ucnv_toUChars(cnv, dest, destcapacity, src.param().abi(), srclength, perrorcode)
}
#[inline]
pub unsafe fn ucnv_toUCountPending(cnv: *const UConverter, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_toUCountPending(cnv : *const UConverter, status : *mut UErrorCode) -> i32);
    ucnv_toUCountPending(cnv, status)
}
#[inline]
pub unsafe fn ucnv_toUnicode<P0>(converter: *mut UConverter, target: *mut *mut u16, targetlimit: *const u16, source: *const *const i8, sourcelimit: P0, offsets: *mut i32, flush: i8, err: *mut UErrorCode)
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_toUnicode(converter : *mut UConverter, target : *mut *mut u16, targetlimit : *const u16, source : *const *const i8, sourcelimit : windows_core::PCSTR, offsets : *mut i32, flush : i8, err : *mut UErrorCode));
    ucnv_toUnicode(converter, target, targetlimit, source, sourcelimit.param().abi(), offsets, flush, err)
}
#[inline]
pub unsafe fn ucnv_usesFallback(cnv: *const UConverter) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn ucnv_usesFallback(cnv : *const UConverter) -> i8);
    ucnv_usesFallback(cnv)
}
#[inline]
pub unsafe fn ucnvsel_close(sel: *mut UConverterSelector) {
    windows_targets::link!("icu.dll" "cdecl" fn ucnvsel_close(sel : *mut UConverterSelector));
    ucnvsel_close(sel)
}
#[inline]
pub unsafe fn ucnvsel_open(converterlist: *const *const i8, converterlistsize: i32, excludedcodepoints: *const USet, whichset: UConverterUnicodeSet, status: *mut UErrorCode) -> *mut UConverterSelector {
    windows_targets::link!("icu.dll" "cdecl" fn ucnvsel_open(converterlist : *const *const i8, converterlistsize : i32, excludedcodepoints : *const USet, whichset : UConverterUnicodeSet, status : *mut UErrorCode) -> *mut UConverterSelector);
    ucnvsel_open(converterlist, converterlistsize, excludedcodepoints, whichset, status)
}
#[inline]
pub unsafe fn ucnvsel_openFromSerialized(buffer: *const core::ffi::c_void, length: i32, status: *mut UErrorCode) -> *mut UConverterSelector {
    windows_targets::link!("icu.dll" "cdecl" fn ucnvsel_openFromSerialized(buffer : *const core::ffi::c_void, length : i32, status : *mut UErrorCode) -> *mut UConverterSelector);
    ucnvsel_openFromSerialized(buffer, length, status)
}
#[inline]
pub unsafe fn ucnvsel_selectForString(sel: *const UConverterSelector, s: *const u16, length: i32, status: *mut UErrorCode) -> *mut UEnumeration {
    windows_targets::link!("icu.dll" "cdecl" fn ucnvsel_selectForString(sel : *const UConverterSelector, s : *const u16, length : i32, status : *mut UErrorCode) -> *mut UEnumeration);
    ucnvsel_selectForString(sel, s, length, status)
}
#[inline]
pub unsafe fn ucnvsel_selectForUTF8<P0>(sel: *const UConverterSelector, s: P0, length: i32, status: *mut UErrorCode) -> *mut UEnumeration
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ucnvsel_selectForUTF8(sel : *const UConverterSelector, s : windows_core::PCSTR, length : i32, status : *mut UErrorCode) -> *mut UEnumeration);
    ucnvsel_selectForUTF8(sel, s.param().abi(), length, status)
}
#[inline]
pub unsafe fn ucnvsel_serialize(sel: *const UConverterSelector, buffer: *mut core::ffi::c_void, buffercapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ucnvsel_serialize(sel : *const UConverterSelector, buffer : *mut core::ffi::c_void, buffercapacity : i32, status : *mut UErrorCode) -> i32);
    ucnvsel_serialize(sel, buffer, buffercapacity, status)
}
#[inline]
pub unsafe fn ucol_cloneBinary(coll: *const UCollator, buffer: *mut u8, capacity: i32, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ucol_cloneBinary(coll : *const UCollator, buffer : *mut u8, capacity : i32, status : *mut UErrorCode) -> i32);
    ucol_cloneBinary(coll, buffer, capacity, status)
}
#[inline]
pub unsafe fn ucol_close(coll: *mut UCollator) {
    windows_targets::link!("icu.dll" "cdecl" fn ucol_close(coll : *mut UCollator));
    ucol_close(coll)
}
#[inline]
pub unsafe fn ucol_closeElements(elems: *mut UCollationElements) {
    windows_targets::link!("icu.dll" "cdecl" fn ucol_closeElements(elems : *mut UCollationElements));
    ucol_closeElements(elems)
}
#[inline]
pub unsafe fn ucol_countAvailable() -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ucol_countAvailable() -> i32);
    ucol_countAvailable()
}
#[inline]
pub unsafe fn ucol_equal(coll: *const UCollator, source: *const u16, sourcelength: i32, target: *const u16, targetlength: i32) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn ucol_equal(coll : *const UCollator, source : *const u16, sourcelength : i32, target : *const u16, targetlength : i32) -> i8);
    ucol_equal(coll, source, sourcelength, target, targetlength)
}
#[inline]
pub unsafe fn ucol_getAttribute(coll: *const UCollator, attr: UColAttribute, status: *mut UErrorCode) -> UColAttributeValue {
    windows_targets::link!("icu.dll" "cdecl" fn ucol_getAttribute(coll : *const UCollator, attr : UColAttribute, status : *mut UErrorCode) -> UColAttributeValue);
    ucol_getAttribute(coll, attr, status)
}
#[inline]
pub unsafe fn ucol_getAvailable(localeindex: i32) -> windows_core::PCSTR {
    windows_targets::link!("icu.dll" "cdecl" fn ucol_getAvailable(localeindex : i32) -> windows_core::PCSTR);
    ucol_getAvailable(localeindex)
}
#[inline]
pub unsafe fn ucol_getBound(source: *const u8, sourcelength: i32, boundtype: UColBoundMode, nooflevels: u32, result: *mut u8, resultlength: i32, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ucol_getBound(source : *const u8, sourcelength : i32, boundtype : UColBoundMode, nooflevels : u32, result : *mut u8, resultlength : i32, status : *mut UErrorCode) -> i32);
    ucol_getBound(source, sourcelength, boundtype, nooflevels, result, resultlength, status)
}
#[inline]
pub unsafe fn ucol_getContractionsAndExpansions(coll: *const UCollator, contractions: *mut USet, expansions: *mut USet, addprefixes: i8, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn ucol_getContractionsAndExpansions(coll : *const UCollator, contractions : *mut USet, expansions : *mut USet, addprefixes : i8, status : *mut UErrorCode));
    ucol_getContractionsAndExpansions(coll, contractions, expansions, addprefixes, status)
}
#[inline]
pub unsafe fn ucol_getDisplayName<P0, P1>(objloc: P0, disploc: P1, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ucol_getDisplayName(objloc : windows_core::PCSTR, disploc : windows_core::PCSTR, result : *mut u16, resultlength : i32, status : *mut UErrorCode) -> i32);
    ucol_getDisplayName(objloc.param().abi(), disploc.param().abi(), result, resultlength, status)
}
#[inline]
pub unsafe fn ucol_getEquivalentReorderCodes(reordercode: i32, dest: *mut i32, destcapacity: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ucol_getEquivalentReorderCodes(reordercode : i32, dest : *mut i32, destcapacity : i32, perrorcode : *mut UErrorCode) -> i32);
    ucol_getEquivalentReorderCodes(reordercode, dest, destcapacity, perrorcode)
}
#[inline]
pub unsafe fn ucol_getFunctionalEquivalent<P0, P1, P2>(result: P0, resultcapacity: i32, keyword: P1, locale: P2, isavailable: *mut i8, status: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ucol_getFunctionalEquivalent(result : windows_core::PCSTR, resultcapacity : i32, keyword : windows_core::PCSTR, locale : windows_core::PCSTR, isavailable : *mut i8, status : *mut UErrorCode) -> i32);
    ucol_getFunctionalEquivalent(result.param().abi(), resultcapacity, keyword.param().abi(), locale.param().abi(), isavailable, status)
}
#[inline]
pub unsafe fn ucol_getKeywordValues<P0>(keyword: P0, status: *mut UErrorCode) -> *mut UEnumeration
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ucol_getKeywordValues(keyword : windows_core::PCSTR, status : *mut UErrorCode) -> *mut UEnumeration);
    ucol_getKeywordValues(keyword.param().abi(), status)
}
#[inline]
pub unsafe fn ucol_getKeywordValuesForLocale<P0, P1>(key: P0, locale: P1, commonlyused: i8, status: *mut UErrorCode) -> *mut UEnumeration
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ucol_getKeywordValuesForLocale(key : windows_core::PCSTR, locale : windows_core::PCSTR, commonlyused : i8, status : *mut UErrorCode) -> *mut UEnumeration);
    ucol_getKeywordValuesForLocale(key.param().abi(), locale.param().abi(), commonlyused, status)
}
#[inline]
pub unsafe fn ucol_getKeywords(status: *mut UErrorCode) -> *mut UEnumeration {
    windows_targets::link!("icu.dll" "cdecl" fn ucol_getKeywords(status : *mut UErrorCode) -> *mut UEnumeration);
    ucol_getKeywords(status)
}
#[inline]
pub unsafe fn ucol_getLocaleByType(coll: *const UCollator, r#type: ULocDataLocaleType, status: *mut UErrorCode) -> windows_core::PCSTR {
    windows_targets::link!("icu.dll" "cdecl" fn ucol_getLocaleByType(coll : *const UCollator, r#type : ULocDataLocaleType, status : *mut UErrorCode) -> windows_core::PCSTR);
    ucol_getLocaleByType(coll, r#type, status)
}
#[inline]
pub unsafe fn ucol_getMaxExpansion(elems: *const UCollationElements, order: i32) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ucol_getMaxExpansion(elems : *const UCollationElements, order : i32) -> i32);
    ucol_getMaxExpansion(elems, order)
}
#[inline]
pub unsafe fn ucol_getMaxVariable(coll: *const UCollator) -> UColReorderCode {
    windows_targets::link!("icu.dll" "cdecl" fn ucol_getMaxVariable(coll : *const UCollator) -> UColReorderCode);
    ucol_getMaxVariable(coll)
}
#[inline]
pub unsafe fn ucol_getOffset(elems: *const UCollationElements) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ucol_getOffset(elems : *const UCollationElements) -> i32);
    ucol_getOffset(elems)
}
#[inline]
pub unsafe fn ucol_getReorderCodes(coll: *const UCollator, dest: *mut i32, destcapacity: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ucol_getReorderCodes(coll : *const UCollator, dest : *mut i32, destcapacity : i32, perrorcode : *mut UErrorCode) -> i32);
    ucol_getReorderCodes(coll, dest, destcapacity, perrorcode)
}
#[inline]
pub unsafe fn ucol_getRules(coll: *const UCollator, length: *mut i32) -> *mut u16 {
    windows_targets::link!("icu.dll" "cdecl" fn ucol_getRules(coll : *const UCollator, length : *mut i32) -> *mut u16);
    ucol_getRules(coll, length)
}
#[inline]
pub unsafe fn ucol_getRulesEx(coll: *const UCollator, delta: UColRuleOption, buffer: *mut u16, bufferlen: i32) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ucol_getRulesEx(coll : *const UCollator, delta : UColRuleOption, buffer : *mut u16, bufferlen : i32) -> i32);
    ucol_getRulesEx(coll, delta, buffer, bufferlen)
}
#[inline]
pub unsafe fn ucol_getSortKey(coll: *const UCollator, source: *const u16, sourcelength: i32, result: *mut u8, resultlength: i32) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ucol_getSortKey(coll : *const UCollator, source : *const u16, sourcelength : i32, result : *mut u8, resultlength : i32) -> i32);
    ucol_getSortKey(coll, source, sourcelength, result, resultlength)
}
#[inline]
pub unsafe fn ucol_getStrength(coll: *const UCollator) -> UColAttributeValue {
    windows_targets::link!("icu.dll" "cdecl" fn ucol_getStrength(coll : *const UCollator) -> UColAttributeValue);
    ucol_getStrength(coll)
}
#[inline]
pub unsafe fn ucol_getTailoredSet(coll: *const UCollator, status: *mut UErrorCode) -> *mut USet {
    windows_targets::link!("icu.dll" "cdecl" fn ucol_getTailoredSet(coll : *const UCollator, status : *mut UErrorCode) -> *mut USet);
    ucol_getTailoredSet(coll, status)
}
#[inline]
pub unsafe fn ucol_getUCAVersion(coll: *const UCollator, info: *mut u8) {
    windows_targets::link!("icu.dll" "cdecl" fn ucol_getUCAVersion(coll : *const UCollator, info : *mut u8));
    ucol_getUCAVersion(coll, info)
}
#[inline]
pub unsafe fn ucol_getVariableTop(coll: *const UCollator, status: *mut UErrorCode) -> u32 {
    windows_targets::link!("icu.dll" "cdecl" fn ucol_getVariableTop(coll : *const UCollator, status : *mut UErrorCode) -> u32);
    ucol_getVariableTop(coll, status)
}
#[inline]
pub unsafe fn ucol_getVersion(coll: *const UCollator, info: *mut u8) {
    windows_targets::link!("icu.dll" "cdecl" fn ucol_getVersion(coll : *const UCollator, info : *mut u8));
    ucol_getVersion(coll, info)
}
#[inline]
pub unsafe fn ucol_greater(coll: *const UCollator, source: *const u16, sourcelength: i32, target: *const u16, targetlength: i32) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn ucol_greater(coll : *const UCollator, source : *const u16, sourcelength : i32, target : *const u16, targetlength : i32) -> i8);
    ucol_greater(coll, source, sourcelength, target, targetlength)
}
#[inline]
pub unsafe fn ucol_greaterOrEqual(coll: *const UCollator, source: *const u16, sourcelength: i32, target: *const u16, targetlength: i32) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn ucol_greaterOrEqual(coll : *const UCollator, source : *const u16, sourcelength : i32, target : *const u16, targetlength : i32) -> i8);
    ucol_greaterOrEqual(coll, source, sourcelength, target, targetlength)
}
#[inline]
pub unsafe fn ucol_keyHashCode(key: *const u8, length: i32) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ucol_keyHashCode(key : *const u8, length : i32) -> i32);
    ucol_keyHashCode(key, length)
}
#[inline]
pub unsafe fn ucol_mergeSortkeys(src1: *const u8, src1length: i32, src2: *const u8, src2length: i32, dest: *mut u8, destcapacity: i32) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ucol_mergeSortkeys(src1 : *const u8, src1length : i32, src2 : *const u8, src2length : i32, dest : *mut u8, destcapacity : i32) -> i32);
    ucol_mergeSortkeys(src1, src1length, src2, src2length, dest, destcapacity)
}
#[inline]
pub unsafe fn ucol_next(elems: *mut UCollationElements, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ucol_next(elems : *mut UCollationElements, status : *mut UErrorCode) -> i32);
    ucol_next(elems, status)
}
#[inline]
pub unsafe fn ucol_nextSortKeyPart(coll: *const UCollator, iter: *mut UCharIterator, state: *mut u32, dest: *mut u8, count: i32, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ucol_nextSortKeyPart(coll : *const UCollator, iter : *mut UCharIterator, state : *mut u32, dest : *mut u8, count : i32, status : *mut UErrorCode) -> i32);
    ucol_nextSortKeyPart(coll, iter, state, dest, count, status)
}
#[inline]
pub unsafe fn ucol_open<P0>(loc: P0, status: *mut UErrorCode) -> *mut UCollator
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ucol_open(loc : windows_core::PCSTR, status : *mut UErrorCode) -> *mut UCollator);
    ucol_open(loc.param().abi(), status)
}
#[inline]
pub unsafe fn ucol_openAvailableLocales(status: *mut UErrorCode) -> *mut UEnumeration {
    windows_targets::link!("icu.dll" "cdecl" fn ucol_openAvailableLocales(status : *mut UErrorCode) -> *mut UEnumeration);
    ucol_openAvailableLocales(status)
}
#[inline]
pub unsafe fn ucol_openBinary(bin: *const u8, length: i32, base: *const UCollator, status: *mut UErrorCode) -> *mut UCollator {
    windows_targets::link!("icu.dll" "cdecl" fn ucol_openBinary(bin : *const u8, length : i32, base : *const UCollator, status : *mut UErrorCode) -> *mut UCollator);
    ucol_openBinary(bin, length, base, status)
}
#[inline]
pub unsafe fn ucol_openElements(coll: *const UCollator, text: *const u16, textlength: i32, status: *mut UErrorCode) -> *mut UCollationElements {
    windows_targets::link!("icu.dll" "cdecl" fn ucol_openElements(coll : *const UCollator, text : *const u16, textlength : i32, status : *mut UErrorCode) -> *mut UCollationElements);
    ucol_openElements(coll, text, textlength, status)
}
#[inline]
pub unsafe fn ucol_openRules(rules: *const u16, ruleslength: i32, normalizationmode: UColAttributeValue, strength: UColAttributeValue, parseerror: *mut UParseError, status: *mut UErrorCode) -> *mut UCollator {
    windows_targets::link!("icu.dll" "cdecl" fn ucol_openRules(rules : *const u16, ruleslength : i32, normalizationmode : UColAttributeValue, strength : UColAttributeValue, parseerror : *mut UParseError, status : *mut UErrorCode) -> *mut UCollator);
    ucol_openRules(rules, ruleslength, normalizationmode, strength, parseerror, status)
}
#[inline]
pub unsafe fn ucol_previous(elems: *mut UCollationElements, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ucol_previous(elems : *mut UCollationElements, status : *mut UErrorCode) -> i32);
    ucol_previous(elems, status)
}
#[inline]
pub unsafe fn ucol_primaryOrder(order: i32) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ucol_primaryOrder(order : i32) -> i32);
    ucol_primaryOrder(order)
}
#[inline]
pub unsafe fn ucol_reset(elems: *mut UCollationElements) {
    windows_targets::link!("icu.dll" "cdecl" fn ucol_reset(elems : *mut UCollationElements));
    ucol_reset(elems)
}
#[inline]
pub unsafe fn ucol_safeClone(coll: *const UCollator, stackbuffer: *mut core::ffi::c_void, pbuffersize: *mut i32, status: *mut UErrorCode) -> *mut UCollator {
    windows_targets::link!("icu.dll" "cdecl" fn ucol_safeClone(coll : *const UCollator, stackbuffer : *mut core::ffi::c_void, pbuffersize : *mut i32, status : *mut UErrorCode) -> *mut UCollator);
    ucol_safeClone(coll, stackbuffer, pbuffersize, status)
}
#[inline]
pub unsafe fn ucol_secondaryOrder(order: i32) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ucol_secondaryOrder(order : i32) -> i32);
    ucol_secondaryOrder(order)
}
#[inline]
pub unsafe fn ucol_setAttribute(coll: *mut UCollator, attr: UColAttribute, value: UColAttributeValue, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn ucol_setAttribute(coll : *mut UCollator, attr : UColAttribute, value : UColAttributeValue, status : *mut UErrorCode));
    ucol_setAttribute(coll, attr, value, status)
}
#[inline]
pub unsafe fn ucol_setMaxVariable(coll: *mut UCollator, group: UColReorderCode, perrorcode: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn ucol_setMaxVariable(coll : *mut UCollator, group : UColReorderCode, perrorcode : *mut UErrorCode));
    ucol_setMaxVariable(coll, group, perrorcode)
}
#[inline]
pub unsafe fn ucol_setOffset(elems: *mut UCollationElements, offset: i32, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn ucol_setOffset(elems : *mut UCollationElements, offset : i32, status : *mut UErrorCode));
    ucol_setOffset(elems, offset, status)
}
#[inline]
pub unsafe fn ucol_setReorderCodes(coll: *mut UCollator, reordercodes: *const i32, reordercodeslength: i32, perrorcode: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn ucol_setReorderCodes(coll : *mut UCollator, reordercodes : *const i32, reordercodeslength : i32, perrorcode : *mut UErrorCode));
    ucol_setReorderCodes(coll, reordercodes, reordercodeslength, perrorcode)
}
#[inline]
pub unsafe fn ucol_setStrength(coll: *mut UCollator, strength: UColAttributeValue) {
    windows_targets::link!("icu.dll" "cdecl" fn ucol_setStrength(coll : *mut UCollator, strength : UColAttributeValue));
    ucol_setStrength(coll, strength)
}
#[inline]
pub unsafe fn ucol_setText(elems: *mut UCollationElements, text: *const u16, textlength: i32, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn ucol_setText(elems : *mut UCollationElements, text : *const u16, textlength : i32, status : *mut UErrorCode));
    ucol_setText(elems, text, textlength, status)
}
#[inline]
pub unsafe fn ucol_strcoll(coll: *const UCollator, source: *const u16, sourcelength: i32, target: *const u16, targetlength: i32) -> UCollationResult {
    windows_targets::link!("icu.dll" "cdecl" fn ucol_strcoll(coll : *const UCollator, source : *const u16, sourcelength : i32, target : *const u16, targetlength : i32) -> UCollationResult);
    ucol_strcoll(coll, source, sourcelength, target, targetlength)
}
#[inline]
pub unsafe fn ucol_strcollIter(coll: *const UCollator, siter: *mut UCharIterator, titer: *mut UCharIterator, status: *mut UErrorCode) -> UCollationResult {
    windows_targets::link!("icu.dll" "cdecl" fn ucol_strcollIter(coll : *const UCollator, siter : *mut UCharIterator, titer : *mut UCharIterator, status : *mut UErrorCode) -> UCollationResult);
    ucol_strcollIter(coll, siter, titer, status)
}
#[inline]
pub unsafe fn ucol_strcollUTF8<P0, P1>(coll: *const UCollator, source: P0, sourcelength: i32, target: P1, targetlength: i32, status: *mut UErrorCode) -> UCollationResult
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ucol_strcollUTF8(coll : *const UCollator, source : windows_core::PCSTR, sourcelength : i32, target : windows_core::PCSTR, targetlength : i32, status : *mut UErrorCode) -> UCollationResult);
    ucol_strcollUTF8(coll, source.param().abi(), sourcelength, target.param().abi(), targetlength, status)
}
#[inline]
pub unsafe fn ucol_tertiaryOrder(order: i32) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ucol_tertiaryOrder(order : i32) -> i32);
    ucol_tertiaryOrder(order)
}
#[inline]
pub unsafe fn ucpmap_get(map: *const UCPMap, c: i32) -> u32 {
    windows_targets::link!("icu.dll" "cdecl" fn ucpmap_get(map : *const UCPMap, c : i32) -> u32);
    ucpmap_get(map, c)
}
#[inline]
pub unsafe fn ucpmap_getRange(map: *const UCPMap, start: i32, option: UCPMapRangeOption, surrogatevalue: u32, filter: *mut UCPMapValueFilter, context: *const core::ffi::c_void, pvalue: *mut u32) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ucpmap_getRange(map : *const UCPMap, start : i32, option : UCPMapRangeOption, surrogatevalue : u32, filter : *mut UCPMapValueFilter, context : *const core::ffi::c_void, pvalue : *mut u32) -> i32);
    ucpmap_getRange(map, start, option, surrogatevalue, filter, context, pvalue)
}
#[inline]
pub unsafe fn ucptrie_close(trie: *mut UCPTrie) {
    windows_targets::link!("icu.dll" "cdecl" fn ucptrie_close(trie : *mut UCPTrie));
    ucptrie_close(trie)
}
#[inline]
pub unsafe fn ucptrie_get(trie: *const UCPTrie, c: i32) -> u32 {
    windows_targets::link!("icu.dll" "cdecl" fn ucptrie_get(trie : *const UCPTrie, c : i32) -> u32);
    ucptrie_get(trie, c)
}
#[inline]
pub unsafe fn ucptrie_getRange(trie: *const UCPTrie, start: i32, option: UCPMapRangeOption, surrogatevalue: u32, filter: *mut UCPMapValueFilter, context: *const core::ffi::c_void, pvalue: *mut u32) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ucptrie_getRange(trie : *const UCPTrie, start : i32, option : UCPMapRangeOption, surrogatevalue : u32, filter : *mut UCPMapValueFilter, context : *const core::ffi::c_void, pvalue : *mut u32) -> i32);
    ucptrie_getRange(trie, start, option, surrogatevalue, filter, context, pvalue)
}
#[inline]
pub unsafe fn ucptrie_getType(trie: *const UCPTrie) -> UCPTrieType {
    windows_targets::link!("icu.dll" "cdecl" fn ucptrie_getType(trie : *const UCPTrie) -> UCPTrieType);
    ucptrie_getType(trie)
}
#[inline]
pub unsafe fn ucptrie_getValueWidth(trie: *const UCPTrie) -> UCPTrieValueWidth {
    windows_targets::link!("icu.dll" "cdecl" fn ucptrie_getValueWidth(trie : *const UCPTrie) -> UCPTrieValueWidth);
    ucptrie_getValueWidth(trie)
}
#[inline]
pub unsafe fn ucptrie_internalSmallIndex(trie: *const UCPTrie, c: i32) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ucptrie_internalSmallIndex(trie : *const UCPTrie, c : i32) -> i32);
    ucptrie_internalSmallIndex(trie, c)
}
#[inline]
pub unsafe fn ucptrie_internalSmallU8Index(trie: *const UCPTrie, lt1: i32, t2: u8, t3: u8) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ucptrie_internalSmallU8Index(trie : *const UCPTrie, lt1 : i32, t2 : u8, t3 : u8) -> i32);
    ucptrie_internalSmallU8Index(trie, lt1, t2, t3)
}
#[inline]
pub unsafe fn ucptrie_internalU8PrevIndex(trie: *const UCPTrie, c: i32, start: *const u8, src: *const u8) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ucptrie_internalU8PrevIndex(trie : *const UCPTrie, c : i32, start : *const u8, src : *const u8) -> i32);
    ucptrie_internalU8PrevIndex(trie, c, start, src)
}
#[inline]
pub unsafe fn ucptrie_openFromBinary(r#type: UCPTrieType, valuewidth: UCPTrieValueWidth, data: *const core::ffi::c_void, length: i32, pactuallength: *mut i32, perrorcode: *mut UErrorCode) -> *mut UCPTrie {
    windows_targets::link!("icu.dll" "cdecl" fn ucptrie_openFromBinary(r#type : UCPTrieType, valuewidth : UCPTrieValueWidth, data : *const core::ffi::c_void, length : i32, pactuallength : *mut i32, perrorcode : *mut UErrorCode) -> *mut UCPTrie);
    ucptrie_openFromBinary(r#type, valuewidth, data, length, pactuallength, perrorcode)
}
#[inline]
pub unsafe fn ucptrie_toBinary(trie: *const UCPTrie, data: *mut core::ffi::c_void, capacity: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ucptrie_toBinary(trie : *const UCPTrie, data : *mut core::ffi::c_void, capacity : i32, perrorcode : *mut UErrorCode) -> i32);
    ucptrie_toBinary(trie, data, capacity, perrorcode)
}
#[inline]
pub unsafe fn ucsdet_close(ucsd: *mut UCharsetDetector) {
    windows_targets::link!("icu.dll" "cdecl" fn ucsdet_close(ucsd : *mut UCharsetDetector));
    ucsdet_close(ucsd)
}
#[inline]
pub unsafe fn ucsdet_detect(ucsd: *mut UCharsetDetector, status: *mut UErrorCode) -> *mut UCharsetMatch {
    windows_targets::link!("icu.dll" "cdecl" fn ucsdet_detect(ucsd : *mut UCharsetDetector, status : *mut UErrorCode) -> *mut UCharsetMatch);
    ucsdet_detect(ucsd, status)
}
#[inline]
pub unsafe fn ucsdet_detectAll(ucsd: *mut UCharsetDetector, matchesfound: *mut i32, status: *mut UErrorCode) -> *mut *mut UCharsetMatch {
    windows_targets::link!("icu.dll" "cdecl" fn ucsdet_detectAll(ucsd : *mut UCharsetDetector, matchesfound : *mut i32, status : *mut UErrorCode) -> *mut *mut UCharsetMatch);
    ucsdet_detectAll(ucsd, matchesfound, status)
}
#[inline]
pub unsafe fn ucsdet_enableInputFilter(ucsd: *mut UCharsetDetector, filter: i8) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn ucsdet_enableInputFilter(ucsd : *mut UCharsetDetector, filter : i8) -> i8);
    ucsdet_enableInputFilter(ucsd, filter)
}
#[inline]
pub unsafe fn ucsdet_getAllDetectableCharsets(ucsd: *const UCharsetDetector, status: *mut UErrorCode) -> *mut UEnumeration {
    windows_targets::link!("icu.dll" "cdecl" fn ucsdet_getAllDetectableCharsets(ucsd : *const UCharsetDetector, status : *mut UErrorCode) -> *mut UEnumeration);
    ucsdet_getAllDetectableCharsets(ucsd, status)
}
#[inline]
pub unsafe fn ucsdet_getConfidence(ucsm: *const UCharsetMatch, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ucsdet_getConfidence(ucsm : *const UCharsetMatch, status : *mut UErrorCode) -> i32);
    ucsdet_getConfidence(ucsm, status)
}
#[inline]
pub unsafe fn ucsdet_getLanguage(ucsm: *const UCharsetMatch, status: *mut UErrorCode) -> windows_core::PCSTR {
    windows_targets::link!("icu.dll" "cdecl" fn ucsdet_getLanguage(ucsm : *const UCharsetMatch, status : *mut UErrorCode) -> windows_core::PCSTR);
    ucsdet_getLanguage(ucsm, status)
}
#[inline]
pub unsafe fn ucsdet_getName(ucsm: *const UCharsetMatch, status: *mut UErrorCode) -> windows_core::PCSTR {
    windows_targets::link!("icu.dll" "cdecl" fn ucsdet_getName(ucsm : *const UCharsetMatch, status : *mut UErrorCode) -> windows_core::PCSTR);
    ucsdet_getName(ucsm, status)
}
#[inline]
pub unsafe fn ucsdet_getUChars(ucsm: *const UCharsetMatch, buf: *mut u16, cap: i32, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ucsdet_getUChars(ucsm : *const UCharsetMatch, buf : *mut u16, cap : i32, status : *mut UErrorCode) -> i32);
    ucsdet_getUChars(ucsm, buf, cap, status)
}
#[inline]
pub unsafe fn ucsdet_isInputFilterEnabled(ucsd: *const UCharsetDetector) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn ucsdet_isInputFilterEnabled(ucsd : *const UCharsetDetector) -> i8);
    ucsdet_isInputFilterEnabled(ucsd)
}
#[inline]
pub unsafe fn ucsdet_open(status: *mut UErrorCode) -> *mut UCharsetDetector {
    windows_targets::link!("icu.dll" "cdecl" fn ucsdet_open(status : *mut UErrorCode) -> *mut UCharsetDetector);
    ucsdet_open(status)
}
#[inline]
pub unsafe fn ucsdet_setDeclaredEncoding<P0>(ucsd: *mut UCharsetDetector, encoding: P0, length: i32, status: *mut UErrorCode)
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ucsdet_setDeclaredEncoding(ucsd : *mut UCharsetDetector, encoding : windows_core::PCSTR, length : i32, status : *mut UErrorCode));
    ucsdet_setDeclaredEncoding(ucsd, encoding.param().abi(), length, status)
}
#[inline]
pub unsafe fn ucsdet_setText<P0>(ucsd: *mut UCharsetDetector, textin: P0, len: i32, status: *mut UErrorCode)
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ucsdet_setText(ucsd : *mut UCharsetDetector, textin : windows_core::PCSTR, len : i32, status : *mut UErrorCode));
    ucsdet_setText(ucsd, textin.param().abi(), len, status)
}
#[inline]
pub unsafe fn ucurr_countCurrencies<P0>(locale: P0, date: f64, ec: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ucurr_countCurrencies(locale : windows_core::PCSTR, date : f64, ec : *mut UErrorCode) -> i32);
    ucurr_countCurrencies(locale.param().abi(), date, ec)
}
#[inline]
pub unsafe fn ucurr_forLocale<P0>(locale: P0, buff: *mut u16, buffcapacity: i32, ec: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ucurr_forLocale(locale : windows_core::PCSTR, buff : *mut u16, buffcapacity : i32, ec : *mut UErrorCode) -> i32);
    ucurr_forLocale(locale.param().abi(), buff, buffcapacity, ec)
}
#[inline]
pub unsafe fn ucurr_forLocaleAndDate<P0>(locale: P0, date: f64, index: i32, buff: *mut u16, buffcapacity: i32, ec: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ucurr_forLocaleAndDate(locale : windows_core::PCSTR, date : f64, index : i32, buff : *mut u16, buffcapacity : i32, ec : *mut UErrorCode) -> i32);
    ucurr_forLocaleAndDate(locale.param().abi(), date, index, buff, buffcapacity, ec)
}
#[inline]
pub unsafe fn ucurr_getDefaultFractionDigits(currency: *const u16, ec: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ucurr_getDefaultFractionDigits(currency : *const u16, ec : *mut UErrorCode) -> i32);
    ucurr_getDefaultFractionDigits(currency, ec)
}
#[inline]
pub unsafe fn ucurr_getDefaultFractionDigitsForUsage(currency: *const u16, usage: UCurrencyUsage, ec: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ucurr_getDefaultFractionDigitsForUsage(currency : *const u16, usage : UCurrencyUsage, ec : *mut UErrorCode) -> i32);
    ucurr_getDefaultFractionDigitsForUsage(currency, usage, ec)
}
#[inline]
pub unsafe fn ucurr_getKeywordValuesForLocale<P0, P1>(key: P0, locale: P1, commonlyused: i8, status: *mut UErrorCode) -> *mut UEnumeration
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ucurr_getKeywordValuesForLocale(key : windows_core::PCSTR, locale : windows_core::PCSTR, commonlyused : i8, status : *mut UErrorCode) -> *mut UEnumeration);
    ucurr_getKeywordValuesForLocale(key.param().abi(), locale.param().abi(), commonlyused, status)
}
#[inline]
pub unsafe fn ucurr_getName<P0>(currency: *const u16, locale: P0, namestyle: UCurrNameStyle, ischoiceformat: *mut i8, len: *mut i32, ec: *mut UErrorCode) -> *mut u16
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ucurr_getName(currency : *const u16, locale : windows_core::PCSTR, namestyle : UCurrNameStyle, ischoiceformat : *mut i8, len : *mut i32, ec : *mut UErrorCode) -> *mut u16);
    ucurr_getName(currency, locale.param().abi(), namestyle, ischoiceformat, len, ec)
}
#[inline]
pub unsafe fn ucurr_getNumericCode(currency: *const u16) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ucurr_getNumericCode(currency : *const u16) -> i32);
    ucurr_getNumericCode(currency)
}
#[inline]
pub unsafe fn ucurr_getPluralName<P0, P1>(currency: *const u16, locale: P0, ischoiceformat: *mut i8, pluralcount: P1, len: *mut i32, ec: *mut UErrorCode) -> *mut u16
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ucurr_getPluralName(currency : *const u16, locale : windows_core::PCSTR, ischoiceformat : *mut i8, pluralcount : windows_core::PCSTR, len : *mut i32, ec : *mut UErrorCode) -> *mut u16);
    ucurr_getPluralName(currency, locale.param().abi(), ischoiceformat, pluralcount.param().abi(), len, ec)
}
#[inline]
pub unsafe fn ucurr_getRoundingIncrement(currency: *const u16, ec: *mut UErrorCode) -> f64 {
    windows_targets::link!("icu.dll" "cdecl" fn ucurr_getRoundingIncrement(currency : *const u16, ec : *mut UErrorCode) -> f64);
    ucurr_getRoundingIncrement(currency, ec)
}
#[inline]
pub unsafe fn ucurr_getRoundingIncrementForUsage(currency: *const u16, usage: UCurrencyUsage, ec: *mut UErrorCode) -> f64 {
    windows_targets::link!("icu.dll" "cdecl" fn ucurr_getRoundingIncrementForUsage(currency : *const u16, usage : UCurrencyUsage, ec : *mut UErrorCode) -> f64);
    ucurr_getRoundingIncrementForUsage(currency, usage, ec)
}
#[inline]
pub unsafe fn ucurr_isAvailable(isocode: *const u16, from: f64, to: f64, errorcode: *mut UErrorCode) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn ucurr_isAvailable(isocode : *const u16, from : f64, to : f64, errorcode : *mut UErrorCode) -> i8);
    ucurr_isAvailable(isocode, from, to, errorcode)
}
#[inline]
pub unsafe fn ucurr_openISOCurrencies(currtype: u32, perrorcode: *mut UErrorCode) -> *mut UEnumeration {
    windows_targets::link!("icu.dll" "cdecl" fn ucurr_openISOCurrencies(currtype : u32, perrorcode : *mut UErrorCode) -> *mut UEnumeration);
    ucurr_openISOCurrencies(currtype, perrorcode)
}
#[inline]
pub unsafe fn ucurr_register<P0>(isocode: *const u16, locale: P0, status: *mut UErrorCode) -> *mut core::ffi::c_void
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ucurr_register(isocode : *const u16, locale : windows_core::PCSTR, status : *mut UErrorCode) -> *mut core::ffi::c_void);
    ucurr_register(isocode, locale.param().abi(), status)
}
#[inline]
pub unsafe fn ucurr_unregister(key: *mut core::ffi::c_void, status: *mut UErrorCode) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn ucurr_unregister(key : *mut core::ffi::c_void, status : *mut UErrorCode) -> i8);
    ucurr_unregister(key, status)
}
#[inline]
pub unsafe fn udat_adoptNumberFormat(fmt: *mut *mut core::ffi::c_void, numberformattoadopt: *mut *mut core::ffi::c_void) {
    windows_targets::link!("icu.dll" "cdecl" fn udat_adoptNumberFormat(fmt : *mut *mut core::ffi::c_void, numberformattoadopt : *mut *mut core::ffi::c_void));
    udat_adoptNumberFormat(fmt, numberformattoadopt)
}
#[inline]
pub unsafe fn udat_adoptNumberFormatForFields(fmt: *mut *mut core::ffi::c_void, fields: *const u16, numberformattoset: *mut *mut core::ffi::c_void, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn udat_adoptNumberFormatForFields(fmt : *mut *mut core::ffi::c_void, fields : *const u16, numberformattoset : *mut *mut core::ffi::c_void, status : *mut UErrorCode));
    udat_adoptNumberFormatForFields(fmt, fields, numberformattoset, status)
}
#[inline]
pub unsafe fn udat_applyPattern(format: *mut *mut core::ffi::c_void, localized: i8, pattern: *const u16, patternlength: i32) {
    windows_targets::link!("icu.dll" "cdecl" fn udat_applyPattern(format : *mut *mut core::ffi::c_void, localized : i8, pattern : *const u16, patternlength : i32));
    udat_applyPattern(format, localized, pattern, patternlength)
}
#[inline]
pub unsafe fn udat_clone(fmt: *const *const core::ffi::c_void, status: *mut UErrorCode) -> *mut *mut core::ffi::c_void {
    windows_targets::link!("icu.dll" "cdecl" fn udat_clone(fmt : *const *const core::ffi::c_void, status : *mut UErrorCode) -> *mut *mut core::ffi::c_void);
    udat_clone(fmt, status)
}
#[inline]
pub unsafe fn udat_close(format: *mut *mut core::ffi::c_void) {
    windows_targets::link!("icu.dll" "cdecl" fn udat_close(format : *mut *mut core::ffi::c_void));
    udat_close(format)
}
#[inline]
pub unsafe fn udat_countAvailable() -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn udat_countAvailable() -> i32);
    udat_countAvailable()
}
#[inline]
pub unsafe fn udat_countSymbols(fmt: *const *const core::ffi::c_void, r#type: UDateFormatSymbolType) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn udat_countSymbols(fmt : *const *const core::ffi::c_void, r#type : UDateFormatSymbolType) -> i32);
    udat_countSymbols(fmt, r#type)
}
#[inline]
pub unsafe fn udat_format(format: *const *const core::ffi::c_void, datetoformat: f64, result: *mut u16, resultlength: i32, position: *mut UFieldPosition, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn udat_format(format : *const *const core::ffi::c_void, datetoformat : f64, result : *mut u16, resultlength : i32, position : *mut UFieldPosition, status : *mut UErrorCode) -> i32);
    udat_format(format, datetoformat, result, resultlength, position, status)
}
#[inline]
pub unsafe fn udat_formatCalendar(format: *const *const core::ffi::c_void, calendar: *mut *mut core::ffi::c_void, result: *mut u16, capacity: i32, position: *mut UFieldPosition, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn udat_formatCalendar(format : *const *const core::ffi::c_void, calendar : *mut *mut core::ffi::c_void, result : *mut u16, capacity : i32, position : *mut UFieldPosition, status : *mut UErrorCode) -> i32);
    udat_formatCalendar(format, calendar, result, capacity, position, status)
}
#[inline]
pub unsafe fn udat_formatCalendarForFields(format: *const *const core::ffi::c_void, calendar: *mut *mut core::ffi::c_void, result: *mut u16, capacity: i32, fpositer: *mut UFieldPositionIterator, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn udat_formatCalendarForFields(format : *const *const core::ffi::c_void, calendar : *mut *mut core::ffi::c_void, result : *mut u16, capacity : i32, fpositer : *mut UFieldPositionIterator, status : *mut UErrorCode) -> i32);
    udat_formatCalendarForFields(format, calendar, result, capacity, fpositer, status)
}
#[inline]
pub unsafe fn udat_formatForFields(format: *const *const core::ffi::c_void, datetoformat: f64, result: *mut u16, resultlength: i32, fpositer: *mut UFieldPositionIterator, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn udat_formatForFields(format : *const *const core::ffi::c_void, datetoformat : f64, result : *mut u16, resultlength : i32, fpositer : *mut UFieldPositionIterator, status : *mut UErrorCode) -> i32);
    udat_formatForFields(format, datetoformat, result, resultlength, fpositer, status)
}
#[inline]
pub unsafe fn udat_get2DigitYearStart(fmt: *const *const core::ffi::c_void, status: *mut UErrorCode) -> f64 {
    windows_targets::link!("icu.dll" "cdecl" fn udat_get2DigitYearStart(fmt : *const *const core::ffi::c_void, status : *mut UErrorCode) -> f64);
    udat_get2DigitYearStart(fmt, status)
}
#[inline]
pub unsafe fn udat_getAvailable(localeindex: i32) -> windows_core::PCSTR {
    windows_targets::link!("icu.dll" "cdecl" fn udat_getAvailable(localeindex : i32) -> windows_core::PCSTR);
    udat_getAvailable(localeindex)
}
#[inline]
pub unsafe fn udat_getBooleanAttribute(fmt: *const *const core::ffi::c_void, attr: UDateFormatBooleanAttribute, status: *mut UErrorCode) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn udat_getBooleanAttribute(fmt : *const *const core::ffi::c_void, attr : UDateFormatBooleanAttribute, status : *mut UErrorCode) -> i8);
    udat_getBooleanAttribute(fmt, attr, status)
}
#[inline]
pub unsafe fn udat_getCalendar(fmt: *const *const core::ffi::c_void) -> *mut *mut core::ffi::c_void {
    windows_targets::link!("icu.dll" "cdecl" fn udat_getCalendar(fmt : *const *const core::ffi::c_void) -> *mut *mut core::ffi::c_void);
    udat_getCalendar(fmt)
}
#[inline]
pub unsafe fn udat_getContext(fmt: *const *const core::ffi::c_void, r#type: UDisplayContextType, status: *mut UErrorCode) -> UDisplayContext {
    windows_targets::link!("icu.dll" "cdecl" fn udat_getContext(fmt : *const *const core::ffi::c_void, r#type : UDisplayContextType, status : *mut UErrorCode) -> UDisplayContext);
    udat_getContext(fmt, r#type, status)
}
#[inline]
pub unsafe fn udat_getLocaleByType(fmt: *const *const core::ffi::c_void, r#type: ULocDataLocaleType, status: *mut UErrorCode) -> windows_core::PCSTR {
    windows_targets::link!("icu.dll" "cdecl" fn udat_getLocaleByType(fmt : *const *const core::ffi::c_void, r#type : ULocDataLocaleType, status : *mut UErrorCode) -> windows_core::PCSTR);
    udat_getLocaleByType(fmt, r#type, status)
}
#[inline]
pub unsafe fn udat_getNumberFormat(fmt: *const *const core::ffi::c_void) -> *mut *mut core::ffi::c_void {
    windows_targets::link!("icu.dll" "cdecl" fn udat_getNumberFormat(fmt : *const *const core::ffi::c_void) -> *mut *mut core::ffi::c_void);
    udat_getNumberFormat(fmt)
}
#[inline]
pub unsafe fn udat_getNumberFormatForField(fmt: *const *const core::ffi::c_void, field: u16) -> *mut *mut core::ffi::c_void {
    windows_targets::link!("icu.dll" "cdecl" fn udat_getNumberFormatForField(fmt : *const *const core::ffi::c_void, field : u16) -> *mut *mut core::ffi::c_void);
    udat_getNumberFormatForField(fmt, field)
}
#[inline]
pub unsafe fn udat_getSymbols(fmt: *const *const core::ffi::c_void, r#type: UDateFormatSymbolType, symbolindex: i32, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn udat_getSymbols(fmt : *const *const core::ffi::c_void, r#type : UDateFormatSymbolType, symbolindex : i32, result : *mut u16, resultlength : i32, status : *mut UErrorCode) -> i32);
    udat_getSymbols(fmt, r#type, symbolindex, result, resultlength, status)
}
#[inline]
pub unsafe fn udat_isLenient(fmt: *const *const core::ffi::c_void) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn udat_isLenient(fmt : *const *const core::ffi::c_void) -> i8);
    udat_isLenient(fmt)
}
#[inline]
pub unsafe fn udat_open<P0>(timestyle: UDateFormatStyle, datestyle: UDateFormatStyle, locale: P0, tzid: *const u16, tzidlength: i32, pattern: *const u16, patternlength: i32, status: *mut UErrorCode) -> *mut *mut core::ffi::c_void
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn udat_open(timestyle : UDateFormatStyle, datestyle : UDateFormatStyle, locale : windows_core::PCSTR, tzid : *const u16, tzidlength : i32, pattern : *const u16, patternlength : i32, status : *mut UErrorCode) -> *mut *mut core::ffi::c_void);
    udat_open(timestyle, datestyle, locale.param().abi(), tzid, tzidlength, pattern, patternlength, status)
}
#[inline]
pub unsafe fn udat_parse(format: *const *const core::ffi::c_void, text: *const u16, textlength: i32, parsepos: *mut i32, status: *mut UErrorCode) -> f64 {
    windows_targets::link!("icu.dll" "cdecl" fn udat_parse(format : *const *const core::ffi::c_void, text : *const u16, textlength : i32, parsepos : *mut i32, status : *mut UErrorCode) -> f64);
    udat_parse(format, text, textlength, parsepos, status)
}
#[inline]
pub unsafe fn udat_parseCalendar(format: *const *const core::ffi::c_void, calendar: *mut *mut core::ffi::c_void, text: *const u16, textlength: i32, parsepos: *mut i32, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn udat_parseCalendar(format : *const *const core::ffi::c_void, calendar : *mut *mut core::ffi::c_void, text : *const u16, textlength : i32, parsepos : *mut i32, status : *mut UErrorCode));
    udat_parseCalendar(format, calendar, text, textlength, parsepos, status)
}
#[inline]
pub unsafe fn udat_set2DigitYearStart(fmt: *mut *mut core::ffi::c_void, d: f64, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn udat_set2DigitYearStart(fmt : *mut *mut core::ffi::c_void, d : f64, status : *mut UErrorCode));
    udat_set2DigitYearStart(fmt, d, status)
}
#[inline]
pub unsafe fn udat_setBooleanAttribute(fmt: *mut *mut core::ffi::c_void, attr: UDateFormatBooleanAttribute, newvalue: i8, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn udat_setBooleanAttribute(fmt : *mut *mut core::ffi::c_void, attr : UDateFormatBooleanAttribute, newvalue : i8, status : *mut UErrorCode));
    udat_setBooleanAttribute(fmt, attr, newvalue, status)
}
#[inline]
pub unsafe fn udat_setCalendar(fmt: *mut *mut core::ffi::c_void, calendartoset: *const *const core::ffi::c_void) {
    windows_targets::link!("icu.dll" "cdecl" fn udat_setCalendar(fmt : *mut *mut core::ffi::c_void, calendartoset : *const *const core::ffi::c_void));
    udat_setCalendar(fmt, calendartoset)
}
#[inline]
pub unsafe fn udat_setContext(fmt: *mut *mut core::ffi::c_void, value: UDisplayContext, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn udat_setContext(fmt : *mut *mut core::ffi::c_void, value : UDisplayContext, status : *mut UErrorCode));
    udat_setContext(fmt, value, status)
}
#[inline]
pub unsafe fn udat_setLenient(fmt: *mut *mut core::ffi::c_void, islenient: i8) {
    windows_targets::link!("icu.dll" "cdecl" fn udat_setLenient(fmt : *mut *mut core::ffi::c_void, islenient : i8));
    udat_setLenient(fmt, islenient)
}
#[inline]
pub unsafe fn udat_setNumberFormat(fmt: *mut *mut core::ffi::c_void, numberformattoset: *const *const core::ffi::c_void) {
    windows_targets::link!("icu.dll" "cdecl" fn udat_setNumberFormat(fmt : *mut *mut core::ffi::c_void, numberformattoset : *const *const core::ffi::c_void));
    udat_setNumberFormat(fmt, numberformattoset)
}
#[inline]
pub unsafe fn udat_setSymbols(format: *mut *mut core::ffi::c_void, r#type: UDateFormatSymbolType, symbolindex: i32, value: *mut u16, valuelength: i32, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn udat_setSymbols(format : *mut *mut core::ffi::c_void, r#type : UDateFormatSymbolType, symbolindex : i32, value : *mut u16, valuelength : i32, status : *mut UErrorCode));
    udat_setSymbols(format, r#type, symbolindex, value, valuelength, status)
}
#[inline]
pub unsafe fn udat_toCalendarDateField(field: UDateFormatField) -> UCalendarDateFields {
    windows_targets::link!("icu.dll" "cdecl" fn udat_toCalendarDateField(field : UDateFormatField) -> UCalendarDateFields);
    udat_toCalendarDateField(field)
}
#[inline]
pub unsafe fn udat_toPattern(fmt: *const *const core::ffi::c_void, localized: i8, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn udat_toPattern(fmt : *const *const core::ffi::c_void, localized : i8, result : *mut u16, resultlength : i32, status : *mut UErrorCode) -> i32);
    udat_toPattern(fmt, localized, result, resultlength, status)
}
#[inline]
pub unsafe fn udatpg_addPattern(dtpg: *mut *mut core::ffi::c_void, pattern: *const u16, patternlength: i32, r#override: i8, conflictingpattern: *mut u16, capacity: i32, plength: *mut i32, perrorcode: *mut UErrorCode) -> UDateTimePatternConflict {
    windows_targets::link!("icu.dll" "cdecl" fn udatpg_addPattern(dtpg : *mut *mut core::ffi::c_void, pattern : *const u16, patternlength : i32, r#override : i8, conflictingpattern : *mut u16, capacity : i32, plength : *mut i32, perrorcode : *mut UErrorCode) -> UDateTimePatternConflict);
    udatpg_addPattern(dtpg, pattern, patternlength, r#override, conflictingpattern, capacity, plength, perrorcode)
}
#[inline]
pub unsafe fn udatpg_clone(dtpg: *const *const core::ffi::c_void, perrorcode: *mut UErrorCode) -> *mut *mut core::ffi::c_void {
    windows_targets::link!("icu.dll" "cdecl" fn udatpg_clone(dtpg : *const *const core::ffi::c_void, perrorcode : *mut UErrorCode) -> *mut *mut core::ffi::c_void);
    udatpg_clone(dtpg, perrorcode)
}
#[inline]
pub unsafe fn udatpg_close(dtpg: *mut *mut core::ffi::c_void) {
    windows_targets::link!("icu.dll" "cdecl" fn udatpg_close(dtpg : *mut *mut core::ffi::c_void));
    udatpg_close(dtpg)
}
#[inline]
pub unsafe fn udatpg_getAppendItemFormat(dtpg: *const *const core::ffi::c_void, field: UDateTimePatternField, plength: *mut i32) -> *mut u16 {
    windows_targets::link!("icu.dll" "cdecl" fn udatpg_getAppendItemFormat(dtpg : *const *const core::ffi::c_void, field : UDateTimePatternField, plength : *mut i32) -> *mut u16);
    udatpg_getAppendItemFormat(dtpg, field, plength)
}
#[inline]
pub unsafe fn udatpg_getAppendItemName(dtpg: *const *const core::ffi::c_void, field: UDateTimePatternField, plength: *mut i32) -> *mut u16 {
    windows_targets::link!("icu.dll" "cdecl" fn udatpg_getAppendItemName(dtpg : *const *const core::ffi::c_void, field : UDateTimePatternField, plength : *mut i32) -> *mut u16);
    udatpg_getAppendItemName(dtpg, field, plength)
}
#[inline]
pub unsafe fn udatpg_getBaseSkeleton(unuseddtpg: *mut *mut core::ffi::c_void, pattern: *const u16, length: i32, baseskeleton: *mut u16, capacity: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn udatpg_getBaseSkeleton(unuseddtpg : *mut *mut core::ffi::c_void, pattern : *const u16, length : i32, baseskeleton : *mut u16, capacity : i32, perrorcode : *mut UErrorCode) -> i32);
    udatpg_getBaseSkeleton(unuseddtpg, pattern, length, baseskeleton, capacity, perrorcode)
}
#[inline]
pub unsafe fn udatpg_getBestPattern(dtpg: *mut *mut core::ffi::c_void, skeleton: *const u16, length: i32, bestpattern: *mut u16, capacity: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn udatpg_getBestPattern(dtpg : *mut *mut core::ffi::c_void, skeleton : *const u16, length : i32, bestpattern : *mut u16, capacity : i32, perrorcode : *mut UErrorCode) -> i32);
    udatpg_getBestPattern(dtpg, skeleton, length, bestpattern, capacity, perrorcode)
}
#[inline]
pub unsafe fn udatpg_getBestPatternWithOptions(dtpg: *mut *mut core::ffi::c_void, skeleton: *const u16, length: i32, options: UDateTimePatternMatchOptions, bestpattern: *mut u16, capacity: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn udatpg_getBestPatternWithOptions(dtpg : *mut *mut core::ffi::c_void, skeleton : *const u16, length : i32, options : UDateTimePatternMatchOptions, bestpattern : *mut u16, capacity : i32, perrorcode : *mut UErrorCode) -> i32);
    udatpg_getBestPatternWithOptions(dtpg, skeleton, length, options, bestpattern, capacity, perrorcode)
}
#[inline]
pub unsafe fn udatpg_getDateTimeFormat(dtpg: *const *const core::ffi::c_void, plength: *mut i32) -> *mut u16 {
    windows_targets::link!("icu.dll" "cdecl" fn udatpg_getDateTimeFormat(dtpg : *const *const core::ffi::c_void, plength : *mut i32) -> *mut u16);
    udatpg_getDateTimeFormat(dtpg, plength)
}
#[inline]
pub unsafe fn udatpg_getDecimal(dtpg: *const *const core::ffi::c_void, plength: *mut i32) -> *mut u16 {
    windows_targets::link!("icu.dll" "cdecl" fn udatpg_getDecimal(dtpg : *const *const core::ffi::c_void, plength : *mut i32) -> *mut u16);
    udatpg_getDecimal(dtpg, plength)
}
#[inline]
pub unsafe fn udatpg_getFieldDisplayName(dtpg: *const *const core::ffi::c_void, field: UDateTimePatternField, width: UDateTimePGDisplayWidth, fieldname: *mut u16, capacity: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn udatpg_getFieldDisplayName(dtpg : *const *const core::ffi::c_void, field : UDateTimePatternField, width : UDateTimePGDisplayWidth, fieldname : *mut u16, capacity : i32, perrorcode : *mut UErrorCode) -> i32);
    udatpg_getFieldDisplayName(dtpg, field, width, fieldname, capacity, perrorcode)
}
#[inline]
pub unsafe fn udatpg_getPatternForSkeleton(dtpg: *const *const core::ffi::c_void, skeleton: *const u16, skeletonlength: i32, plength: *mut i32) -> *mut u16 {
    windows_targets::link!("icu.dll" "cdecl" fn udatpg_getPatternForSkeleton(dtpg : *const *const core::ffi::c_void, skeleton : *const u16, skeletonlength : i32, plength : *mut i32) -> *mut u16);
    udatpg_getPatternForSkeleton(dtpg, skeleton, skeletonlength, plength)
}
#[inline]
pub unsafe fn udatpg_getSkeleton(unuseddtpg: *mut *mut core::ffi::c_void, pattern: *const u16, length: i32, skeleton: *mut u16, capacity: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn udatpg_getSkeleton(unuseddtpg : *mut *mut core::ffi::c_void, pattern : *const u16, length : i32, skeleton : *mut u16, capacity : i32, perrorcode : *mut UErrorCode) -> i32);
    udatpg_getSkeleton(unuseddtpg, pattern, length, skeleton, capacity, perrorcode)
}
#[inline]
pub unsafe fn udatpg_open<P0>(locale: P0, perrorcode: *mut UErrorCode) -> *mut *mut core::ffi::c_void
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn udatpg_open(locale : windows_core::PCSTR, perrorcode : *mut UErrorCode) -> *mut *mut core::ffi::c_void);
    udatpg_open(locale.param().abi(), perrorcode)
}
#[inline]
pub unsafe fn udatpg_openBaseSkeletons(dtpg: *const *const core::ffi::c_void, perrorcode: *mut UErrorCode) -> *mut UEnumeration {
    windows_targets::link!("icu.dll" "cdecl" fn udatpg_openBaseSkeletons(dtpg : *const *const core::ffi::c_void, perrorcode : *mut UErrorCode) -> *mut UEnumeration);
    udatpg_openBaseSkeletons(dtpg, perrorcode)
}
#[inline]
pub unsafe fn udatpg_openEmpty(perrorcode: *mut UErrorCode) -> *mut *mut core::ffi::c_void {
    windows_targets::link!("icu.dll" "cdecl" fn udatpg_openEmpty(perrorcode : *mut UErrorCode) -> *mut *mut core::ffi::c_void);
    udatpg_openEmpty(perrorcode)
}
#[inline]
pub unsafe fn udatpg_openSkeletons(dtpg: *const *const core::ffi::c_void, perrorcode: *mut UErrorCode) -> *mut UEnumeration {
    windows_targets::link!("icu.dll" "cdecl" fn udatpg_openSkeletons(dtpg : *const *const core::ffi::c_void, perrorcode : *mut UErrorCode) -> *mut UEnumeration);
    udatpg_openSkeletons(dtpg, perrorcode)
}
#[inline]
pub unsafe fn udatpg_replaceFieldTypes(dtpg: *mut *mut core::ffi::c_void, pattern: *const u16, patternlength: i32, skeleton: *const u16, skeletonlength: i32, dest: *mut u16, destcapacity: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn udatpg_replaceFieldTypes(dtpg : *mut *mut core::ffi::c_void, pattern : *const u16, patternlength : i32, skeleton : *const u16, skeletonlength : i32, dest : *mut u16, destcapacity : i32, perrorcode : *mut UErrorCode) -> i32);
    udatpg_replaceFieldTypes(dtpg, pattern, patternlength, skeleton, skeletonlength, dest, destcapacity, perrorcode)
}
#[inline]
pub unsafe fn udatpg_replaceFieldTypesWithOptions(dtpg: *mut *mut core::ffi::c_void, pattern: *const u16, patternlength: i32, skeleton: *const u16, skeletonlength: i32, options: UDateTimePatternMatchOptions, dest: *mut u16, destcapacity: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn udatpg_replaceFieldTypesWithOptions(dtpg : *mut *mut core::ffi::c_void, pattern : *const u16, patternlength : i32, skeleton : *const u16, skeletonlength : i32, options : UDateTimePatternMatchOptions, dest : *mut u16, destcapacity : i32, perrorcode : *mut UErrorCode) -> i32);
    udatpg_replaceFieldTypesWithOptions(dtpg, pattern, patternlength, skeleton, skeletonlength, options, dest, destcapacity, perrorcode)
}
#[inline]
pub unsafe fn udatpg_setAppendItemFormat(dtpg: *mut *mut core::ffi::c_void, field: UDateTimePatternField, value: *const u16, length: i32) {
    windows_targets::link!("icu.dll" "cdecl" fn udatpg_setAppendItemFormat(dtpg : *mut *mut core::ffi::c_void, field : UDateTimePatternField, value : *const u16, length : i32));
    udatpg_setAppendItemFormat(dtpg, field, value, length)
}
#[inline]
pub unsafe fn udatpg_setAppendItemName(dtpg: *mut *mut core::ffi::c_void, field: UDateTimePatternField, value: *const u16, length: i32) {
    windows_targets::link!("icu.dll" "cdecl" fn udatpg_setAppendItemName(dtpg : *mut *mut core::ffi::c_void, field : UDateTimePatternField, value : *const u16, length : i32));
    udatpg_setAppendItemName(dtpg, field, value, length)
}
#[inline]
pub unsafe fn udatpg_setDateTimeFormat(dtpg: *const *const core::ffi::c_void, dtformat: *const u16, length: i32) {
    windows_targets::link!("icu.dll" "cdecl" fn udatpg_setDateTimeFormat(dtpg : *const *const core::ffi::c_void, dtformat : *const u16, length : i32));
    udatpg_setDateTimeFormat(dtpg, dtformat, length)
}
#[inline]
pub unsafe fn udatpg_setDecimal(dtpg: *mut *mut core::ffi::c_void, decimal: *const u16, length: i32) {
    windows_targets::link!("icu.dll" "cdecl" fn udatpg_setDecimal(dtpg : *mut *mut core::ffi::c_void, decimal : *const u16, length : i32));
    udatpg_setDecimal(dtpg, decimal, length)
}
#[inline]
pub unsafe fn udtitvfmt_close(formatter: *mut UDateIntervalFormat) {
    windows_targets::link!("icu.dll" "cdecl" fn udtitvfmt_close(formatter : *mut UDateIntervalFormat));
    udtitvfmt_close(formatter)
}
#[inline]
pub unsafe fn udtitvfmt_closeResult(uresult: *mut UFormattedDateInterval) {
    windows_targets::link!("icu.dll" "cdecl" fn udtitvfmt_closeResult(uresult : *mut UFormattedDateInterval));
    udtitvfmt_closeResult(uresult)
}
#[inline]
pub unsafe fn udtitvfmt_format(formatter: *const UDateIntervalFormat, fromdate: f64, todate: f64, result: *mut u16, resultcapacity: i32, position: *mut UFieldPosition, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn udtitvfmt_format(formatter : *const UDateIntervalFormat, fromdate : f64, todate : f64, result : *mut u16, resultcapacity : i32, position : *mut UFieldPosition, status : *mut UErrorCode) -> i32);
    udtitvfmt_format(formatter, fromdate, todate, result, resultcapacity, position, status)
}
#[inline]
pub unsafe fn udtitvfmt_open<P0>(locale: P0, skeleton: *const u16, skeletonlength: i32, tzid: *const u16, tzidlength: i32, status: *mut UErrorCode) -> *mut UDateIntervalFormat
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn udtitvfmt_open(locale : windows_core::PCSTR, skeleton : *const u16, skeletonlength : i32, tzid : *const u16, tzidlength : i32, status : *mut UErrorCode) -> *mut UDateIntervalFormat);
    udtitvfmt_open(locale.param().abi(), skeleton, skeletonlength, tzid, tzidlength, status)
}
#[inline]
pub unsafe fn udtitvfmt_openResult(ec: *mut UErrorCode) -> *mut UFormattedDateInterval {
    windows_targets::link!("icu.dll" "cdecl" fn udtitvfmt_openResult(ec : *mut UErrorCode) -> *mut UFormattedDateInterval);
    udtitvfmt_openResult(ec)
}
#[inline]
pub unsafe fn udtitvfmt_resultAsValue(uresult: *const UFormattedDateInterval, ec: *mut UErrorCode) -> *mut UFormattedValue {
    windows_targets::link!("icu.dll" "cdecl" fn udtitvfmt_resultAsValue(uresult : *const UFormattedDateInterval, ec : *mut UErrorCode) -> *mut UFormattedValue);
    udtitvfmt_resultAsValue(uresult, ec)
}
#[inline]
pub unsafe fn uenum_close(en: *mut UEnumeration) {
    windows_targets::link!("icu.dll" "cdecl" fn uenum_close(en : *mut UEnumeration));
    uenum_close(en)
}
#[inline]
pub unsafe fn uenum_count(en: *mut UEnumeration, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn uenum_count(en : *mut UEnumeration, status : *mut UErrorCode) -> i32);
    uenum_count(en, status)
}
#[inline]
pub unsafe fn uenum_next(en: *mut UEnumeration, resultlength: *mut i32, status: *mut UErrorCode) -> windows_core::PCSTR {
    windows_targets::link!("icu.dll" "cdecl" fn uenum_next(en : *mut UEnumeration, resultlength : *mut i32, status : *mut UErrorCode) -> windows_core::PCSTR);
    uenum_next(en, resultlength, status)
}
#[inline]
pub unsafe fn uenum_openCharStringsEnumeration(strings: *const *const i8, count: i32, ec: *mut UErrorCode) -> *mut UEnumeration {
    windows_targets::link!("icu.dll" "cdecl" fn uenum_openCharStringsEnumeration(strings : *const *const i8, count : i32, ec : *mut UErrorCode) -> *mut UEnumeration);
    uenum_openCharStringsEnumeration(strings, count, ec)
}
#[inline]
pub unsafe fn uenum_openUCharStringsEnumeration(strings: *const *const u16, count: i32, ec: *mut UErrorCode) -> *mut UEnumeration {
    windows_targets::link!("icu.dll" "cdecl" fn uenum_openUCharStringsEnumeration(strings : *const *const u16, count : i32, ec : *mut UErrorCode) -> *mut UEnumeration);
    uenum_openUCharStringsEnumeration(strings, count, ec)
}
#[inline]
pub unsafe fn uenum_reset(en: *mut UEnumeration, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn uenum_reset(en : *mut UEnumeration, status : *mut UErrorCode));
    uenum_reset(en, status)
}
#[inline]
pub unsafe fn uenum_unext(en: *mut UEnumeration, resultlength: *mut i32, status: *mut UErrorCode) -> *mut u16 {
    windows_targets::link!("icu.dll" "cdecl" fn uenum_unext(en : *mut UEnumeration, resultlength : *mut i32, status : *mut UErrorCode) -> *mut u16);
    uenum_unext(en, resultlength, status)
}
#[inline]
pub unsafe fn ufieldpositer_close(fpositer: *mut UFieldPositionIterator) {
    windows_targets::link!("icu.dll" "cdecl" fn ufieldpositer_close(fpositer : *mut UFieldPositionIterator));
    ufieldpositer_close(fpositer)
}
#[inline]
pub unsafe fn ufieldpositer_next(fpositer: *mut UFieldPositionIterator, beginindex: *mut i32, endindex: *mut i32) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ufieldpositer_next(fpositer : *mut UFieldPositionIterator, beginindex : *mut i32, endindex : *mut i32) -> i32);
    ufieldpositer_next(fpositer, beginindex, endindex)
}
#[inline]
pub unsafe fn ufieldpositer_open(status: *mut UErrorCode) -> *mut UFieldPositionIterator {
    windows_targets::link!("icu.dll" "cdecl" fn ufieldpositer_open(status : *mut UErrorCode) -> *mut UFieldPositionIterator);
    ufieldpositer_open(status)
}
#[inline]
pub unsafe fn ufmt_close(fmt: *mut *mut core::ffi::c_void) {
    windows_targets::link!("icu.dll" "cdecl" fn ufmt_close(fmt : *mut *mut core::ffi::c_void));
    ufmt_close(fmt)
}
#[inline]
pub unsafe fn ufmt_getArrayItemByIndex(fmt: *mut *mut core::ffi::c_void, n: i32, status: *mut UErrorCode) -> *mut *mut core::ffi::c_void {
    windows_targets::link!("icu.dll" "cdecl" fn ufmt_getArrayItemByIndex(fmt : *mut *mut core::ffi::c_void, n : i32, status : *mut UErrorCode) -> *mut *mut core::ffi::c_void);
    ufmt_getArrayItemByIndex(fmt, n, status)
}
#[inline]
pub unsafe fn ufmt_getArrayLength(fmt: *const *const core::ffi::c_void, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ufmt_getArrayLength(fmt : *const *const core::ffi::c_void, status : *mut UErrorCode) -> i32);
    ufmt_getArrayLength(fmt, status)
}
#[inline]
pub unsafe fn ufmt_getDate(fmt: *const *const core::ffi::c_void, status: *mut UErrorCode) -> f64 {
    windows_targets::link!("icu.dll" "cdecl" fn ufmt_getDate(fmt : *const *const core::ffi::c_void, status : *mut UErrorCode) -> f64);
    ufmt_getDate(fmt, status)
}
#[inline]
pub unsafe fn ufmt_getDecNumChars(fmt: *mut *mut core::ffi::c_void, len: *mut i32, status: *mut UErrorCode) -> windows_core::PCSTR {
    windows_targets::link!("icu.dll" "cdecl" fn ufmt_getDecNumChars(fmt : *mut *mut core::ffi::c_void, len : *mut i32, status : *mut UErrorCode) -> windows_core::PCSTR);
    ufmt_getDecNumChars(fmt, len, status)
}
#[inline]
pub unsafe fn ufmt_getDouble(fmt: *mut *mut core::ffi::c_void, status: *mut UErrorCode) -> f64 {
    windows_targets::link!("icu.dll" "cdecl" fn ufmt_getDouble(fmt : *mut *mut core::ffi::c_void, status : *mut UErrorCode) -> f64);
    ufmt_getDouble(fmt, status)
}
#[inline]
pub unsafe fn ufmt_getInt64(fmt: *mut *mut core::ffi::c_void, status: *mut UErrorCode) -> i64 {
    windows_targets::link!("icu.dll" "cdecl" fn ufmt_getInt64(fmt : *mut *mut core::ffi::c_void, status : *mut UErrorCode) -> i64);
    ufmt_getInt64(fmt, status)
}
#[inline]
pub unsafe fn ufmt_getLong(fmt: *mut *mut core::ffi::c_void, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ufmt_getLong(fmt : *mut *mut core::ffi::c_void, status : *mut UErrorCode) -> i32);
    ufmt_getLong(fmt, status)
}
#[inline]
pub unsafe fn ufmt_getObject(fmt: *const *const core::ffi::c_void, status: *mut UErrorCode) -> *mut core::ffi::c_void {
    windows_targets::link!("icu.dll" "cdecl" fn ufmt_getObject(fmt : *const *const core::ffi::c_void, status : *mut UErrorCode) -> *mut core::ffi::c_void);
    ufmt_getObject(fmt, status)
}
#[inline]
pub unsafe fn ufmt_getType(fmt: *const *const core::ffi::c_void, status: *mut UErrorCode) -> UFormattableType {
    windows_targets::link!("icu.dll" "cdecl" fn ufmt_getType(fmt : *const *const core::ffi::c_void, status : *mut UErrorCode) -> UFormattableType);
    ufmt_getType(fmt, status)
}
#[inline]
pub unsafe fn ufmt_getUChars(fmt: *mut *mut core::ffi::c_void, len: *mut i32, status: *mut UErrorCode) -> *mut u16 {
    windows_targets::link!("icu.dll" "cdecl" fn ufmt_getUChars(fmt : *mut *mut core::ffi::c_void, len : *mut i32, status : *mut UErrorCode) -> *mut u16);
    ufmt_getUChars(fmt, len, status)
}
#[inline]
pub unsafe fn ufmt_isNumeric(fmt: *const *const core::ffi::c_void) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn ufmt_isNumeric(fmt : *const *const core::ffi::c_void) -> i8);
    ufmt_isNumeric(fmt)
}
#[inline]
pub unsafe fn ufmt_open(status: *mut UErrorCode) -> *mut *mut core::ffi::c_void {
    windows_targets::link!("icu.dll" "cdecl" fn ufmt_open(status : *mut UErrorCode) -> *mut *mut core::ffi::c_void);
    ufmt_open(status)
}
#[inline]
pub unsafe fn ufmtval_getString(ufmtval: *const UFormattedValue, plength: *mut i32, ec: *mut UErrorCode) -> *mut u16 {
    windows_targets::link!("icu.dll" "cdecl" fn ufmtval_getString(ufmtval : *const UFormattedValue, plength : *mut i32, ec : *mut UErrorCode) -> *mut u16);
    ufmtval_getString(ufmtval, plength, ec)
}
#[inline]
pub unsafe fn ufmtval_nextPosition(ufmtval: *const UFormattedValue, ucfpos: *mut UConstrainedFieldPosition, ec: *mut UErrorCode) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn ufmtval_nextPosition(ufmtval : *const UFormattedValue, ucfpos : *mut UConstrainedFieldPosition, ec : *mut UErrorCode) -> i8);
    ufmtval_nextPosition(ufmtval, ucfpos, ec)
}
#[inline]
pub unsafe fn ugender_getInstance<P0>(locale: P0, status: *mut UErrorCode) -> *mut UGenderInfo
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ugender_getInstance(locale : windows_core::PCSTR, status : *mut UErrorCode) -> *mut UGenderInfo);
    ugender_getInstance(locale.param().abi(), status)
}
#[inline]
pub unsafe fn ugender_getListGender(genderinfo: *const UGenderInfo, genders: *const UGender, size: i32, status: *mut UErrorCode) -> UGender {
    windows_targets::link!("icu.dll" "cdecl" fn ugender_getListGender(genderinfo : *const UGenderInfo, genders : *const UGender, size : i32, status : *mut UErrorCode) -> UGender);
    ugender_getListGender(genderinfo, genders, size, status)
}
#[inline]
pub unsafe fn uidna_close(idna: *mut UIDNA) {
    windows_targets::link!("icu.dll" "cdecl" fn uidna_close(idna : *mut UIDNA));
    uidna_close(idna)
}
#[inline]
pub unsafe fn uidna_labelToASCII(idna: *const UIDNA, label: *const u16, length: i32, dest: *mut u16, capacity: i32, pinfo: *mut UIDNAInfo, perrorcode: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn uidna_labelToASCII(idna : *const UIDNA, label : *const u16, length : i32, dest : *mut u16, capacity : i32, pinfo : *mut UIDNAInfo, perrorcode : *mut UErrorCode) -> i32);
    uidna_labelToASCII(idna, label, length, dest, capacity, pinfo, perrorcode)
}
#[inline]
pub unsafe fn uidna_labelToASCII_UTF8<P0, P1>(idna: *const UIDNA, label: P0, length: i32, dest: P1, capacity: i32, pinfo: *mut UIDNAInfo, perrorcode: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uidna_labelToASCII_UTF8(idna : *const UIDNA, label : windows_core::PCSTR, length : i32, dest : windows_core::PCSTR, capacity : i32, pinfo : *mut UIDNAInfo, perrorcode : *mut UErrorCode) -> i32);
    uidna_labelToASCII_UTF8(idna, label.param().abi(), length, dest.param().abi(), capacity, pinfo, perrorcode)
}
#[inline]
pub unsafe fn uidna_labelToUnicode(idna: *const UIDNA, label: *const u16, length: i32, dest: *mut u16, capacity: i32, pinfo: *mut UIDNAInfo, perrorcode: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn uidna_labelToUnicode(idna : *const UIDNA, label : *const u16, length : i32, dest : *mut u16, capacity : i32, pinfo : *mut UIDNAInfo, perrorcode : *mut UErrorCode) -> i32);
    uidna_labelToUnicode(idna, label, length, dest, capacity, pinfo, perrorcode)
}
#[inline]
pub unsafe fn uidna_labelToUnicodeUTF8<P0, P1>(idna: *const UIDNA, label: P0, length: i32, dest: P1, capacity: i32, pinfo: *mut UIDNAInfo, perrorcode: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uidna_labelToUnicodeUTF8(idna : *const UIDNA, label : windows_core::PCSTR, length : i32, dest : windows_core::PCSTR, capacity : i32, pinfo : *mut UIDNAInfo, perrorcode : *mut UErrorCode) -> i32);
    uidna_labelToUnicodeUTF8(idna, label.param().abi(), length, dest.param().abi(), capacity, pinfo, perrorcode)
}
#[inline]
pub unsafe fn uidna_nameToASCII(idna: *const UIDNA, name: *const u16, length: i32, dest: *mut u16, capacity: i32, pinfo: *mut UIDNAInfo, perrorcode: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn uidna_nameToASCII(idna : *const UIDNA, name : *const u16, length : i32, dest : *mut u16, capacity : i32, pinfo : *mut UIDNAInfo, perrorcode : *mut UErrorCode) -> i32);
    uidna_nameToASCII(idna, name, length, dest, capacity, pinfo, perrorcode)
}
#[inline]
pub unsafe fn uidna_nameToASCII_UTF8<P0, P1>(idna: *const UIDNA, name: P0, length: i32, dest: P1, capacity: i32, pinfo: *mut UIDNAInfo, perrorcode: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uidna_nameToASCII_UTF8(idna : *const UIDNA, name : windows_core::PCSTR, length : i32, dest : windows_core::PCSTR, capacity : i32, pinfo : *mut UIDNAInfo, perrorcode : *mut UErrorCode) -> i32);
    uidna_nameToASCII_UTF8(idna, name.param().abi(), length, dest.param().abi(), capacity, pinfo, perrorcode)
}
#[inline]
pub unsafe fn uidna_nameToUnicode(idna: *const UIDNA, name: *const u16, length: i32, dest: *mut u16, capacity: i32, pinfo: *mut UIDNAInfo, perrorcode: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn uidna_nameToUnicode(idna : *const UIDNA, name : *const u16, length : i32, dest : *mut u16, capacity : i32, pinfo : *mut UIDNAInfo, perrorcode : *mut UErrorCode) -> i32);
    uidna_nameToUnicode(idna, name, length, dest, capacity, pinfo, perrorcode)
}
#[inline]
pub unsafe fn uidna_nameToUnicodeUTF8<P0, P1>(idna: *const UIDNA, name: P0, length: i32, dest: P1, capacity: i32, pinfo: *mut UIDNAInfo, perrorcode: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uidna_nameToUnicodeUTF8(idna : *const UIDNA, name : windows_core::PCSTR, length : i32, dest : windows_core::PCSTR, capacity : i32, pinfo : *mut UIDNAInfo, perrorcode : *mut UErrorCode) -> i32);
    uidna_nameToUnicodeUTF8(idna, name.param().abi(), length, dest.param().abi(), capacity, pinfo, perrorcode)
}
#[inline]
pub unsafe fn uidna_openUTS46(options: u32, perrorcode: *mut UErrorCode) -> *mut UIDNA {
    windows_targets::link!("icu.dll" "cdecl" fn uidna_openUTS46(options : u32, perrorcode : *mut UErrorCode) -> *mut UIDNA);
    uidna_openUTS46(options, perrorcode)
}
#[inline]
pub unsafe fn uiter_current32(iter: *mut UCharIterator) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn uiter_current32(iter : *mut UCharIterator) -> i32);
    uiter_current32(iter)
}
#[inline]
pub unsafe fn uiter_getState(iter: *const UCharIterator) -> u32 {
    windows_targets::link!("icu.dll" "cdecl" fn uiter_getState(iter : *const UCharIterator) -> u32);
    uiter_getState(iter)
}
#[inline]
pub unsafe fn uiter_next32(iter: *mut UCharIterator) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn uiter_next32(iter : *mut UCharIterator) -> i32);
    uiter_next32(iter)
}
#[inline]
pub unsafe fn uiter_previous32(iter: *mut UCharIterator) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn uiter_previous32(iter : *mut UCharIterator) -> i32);
    uiter_previous32(iter)
}
#[inline]
pub unsafe fn uiter_setState(iter: *mut UCharIterator, state: u32, perrorcode: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn uiter_setState(iter : *mut UCharIterator, state : u32, perrorcode : *mut UErrorCode));
    uiter_setState(iter, state, perrorcode)
}
#[inline]
pub unsafe fn uiter_setString(iter: *mut UCharIterator, s: *const u16, length: i32) {
    windows_targets::link!("icu.dll" "cdecl" fn uiter_setString(iter : *mut UCharIterator, s : *const u16, length : i32));
    uiter_setString(iter, s, length)
}
#[inline]
pub unsafe fn uiter_setUTF16BE<P0>(iter: *mut UCharIterator, s: P0, length: i32)
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uiter_setUTF16BE(iter : *mut UCharIterator, s : windows_core::PCSTR, length : i32));
    uiter_setUTF16BE(iter, s.param().abi(), length)
}
#[inline]
pub unsafe fn uiter_setUTF8<P0>(iter: *mut UCharIterator, s: P0, length: i32)
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uiter_setUTF8(iter : *mut UCharIterator, s : windows_core::PCSTR, length : i32));
    uiter_setUTF8(iter, s.param().abi(), length)
}
#[inline]
pub unsafe fn uldn_close(ldn: *mut ULocaleDisplayNames) {
    windows_targets::link!("icu.dll" "cdecl" fn uldn_close(ldn : *mut ULocaleDisplayNames));
    uldn_close(ldn)
}
#[inline]
pub unsafe fn uldn_getContext(ldn: *const ULocaleDisplayNames, r#type: UDisplayContextType, perrorcode: *mut UErrorCode) -> UDisplayContext {
    windows_targets::link!("icu.dll" "cdecl" fn uldn_getContext(ldn : *const ULocaleDisplayNames, r#type : UDisplayContextType, perrorcode : *mut UErrorCode) -> UDisplayContext);
    uldn_getContext(ldn, r#type, perrorcode)
}
#[inline]
pub unsafe fn uldn_getDialectHandling(ldn: *const ULocaleDisplayNames) -> UDialectHandling {
    windows_targets::link!("icu.dll" "cdecl" fn uldn_getDialectHandling(ldn : *const ULocaleDisplayNames) -> UDialectHandling);
    uldn_getDialectHandling(ldn)
}
#[inline]
pub unsafe fn uldn_getLocale(ldn: *const ULocaleDisplayNames) -> windows_core::PCSTR {
    windows_targets::link!("icu.dll" "cdecl" fn uldn_getLocale(ldn : *const ULocaleDisplayNames) -> windows_core::PCSTR);
    uldn_getLocale(ldn)
}
#[inline]
pub unsafe fn uldn_keyDisplayName<P0>(ldn: *const ULocaleDisplayNames, key: P0, result: *mut u16, maxresultsize: i32, perrorcode: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uldn_keyDisplayName(ldn : *const ULocaleDisplayNames, key : windows_core::PCSTR, result : *mut u16, maxresultsize : i32, perrorcode : *mut UErrorCode) -> i32);
    uldn_keyDisplayName(ldn, key.param().abi(), result, maxresultsize, perrorcode)
}
#[inline]
pub unsafe fn uldn_keyValueDisplayName<P0, P1>(ldn: *const ULocaleDisplayNames, key: P0, value: P1, result: *mut u16, maxresultsize: i32, perrorcode: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uldn_keyValueDisplayName(ldn : *const ULocaleDisplayNames, key : windows_core::PCSTR, value : windows_core::PCSTR, result : *mut u16, maxresultsize : i32, perrorcode : *mut UErrorCode) -> i32);
    uldn_keyValueDisplayName(ldn, key.param().abi(), value.param().abi(), result, maxresultsize, perrorcode)
}
#[inline]
pub unsafe fn uldn_languageDisplayName<P0>(ldn: *const ULocaleDisplayNames, lang: P0, result: *mut u16, maxresultsize: i32, perrorcode: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uldn_languageDisplayName(ldn : *const ULocaleDisplayNames, lang : windows_core::PCSTR, result : *mut u16, maxresultsize : i32, perrorcode : *mut UErrorCode) -> i32);
    uldn_languageDisplayName(ldn, lang.param().abi(), result, maxresultsize, perrorcode)
}
#[inline]
pub unsafe fn uldn_localeDisplayName<P0>(ldn: *const ULocaleDisplayNames, locale: P0, result: *mut u16, maxresultsize: i32, perrorcode: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uldn_localeDisplayName(ldn : *const ULocaleDisplayNames, locale : windows_core::PCSTR, result : *mut u16, maxresultsize : i32, perrorcode : *mut UErrorCode) -> i32);
    uldn_localeDisplayName(ldn, locale.param().abi(), result, maxresultsize, perrorcode)
}
#[inline]
pub unsafe fn uldn_open<P0>(locale: P0, dialecthandling: UDialectHandling, perrorcode: *mut UErrorCode) -> *mut ULocaleDisplayNames
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uldn_open(locale : windows_core::PCSTR, dialecthandling : UDialectHandling, perrorcode : *mut UErrorCode) -> *mut ULocaleDisplayNames);
    uldn_open(locale.param().abi(), dialecthandling, perrorcode)
}
#[inline]
pub unsafe fn uldn_openForContext<P0>(locale: P0, contexts: *mut UDisplayContext, length: i32, perrorcode: *mut UErrorCode) -> *mut ULocaleDisplayNames
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uldn_openForContext(locale : windows_core::PCSTR, contexts : *mut UDisplayContext, length : i32, perrorcode : *mut UErrorCode) -> *mut ULocaleDisplayNames);
    uldn_openForContext(locale.param().abi(), contexts, length, perrorcode)
}
#[inline]
pub unsafe fn uldn_regionDisplayName<P0>(ldn: *const ULocaleDisplayNames, region: P0, result: *mut u16, maxresultsize: i32, perrorcode: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uldn_regionDisplayName(ldn : *const ULocaleDisplayNames, region : windows_core::PCSTR, result : *mut u16, maxresultsize : i32, perrorcode : *mut UErrorCode) -> i32);
    uldn_regionDisplayName(ldn, region.param().abi(), result, maxresultsize, perrorcode)
}
#[inline]
pub unsafe fn uldn_scriptCodeDisplayName(ldn: *const ULocaleDisplayNames, scriptcode: UScriptCode, result: *mut u16, maxresultsize: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn uldn_scriptCodeDisplayName(ldn : *const ULocaleDisplayNames, scriptcode : UScriptCode, result : *mut u16, maxresultsize : i32, perrorcode : *mut UErrorCode) -> i32);
    uldn_scriptCodeDisplayName(ldn, scriptcode, result, maxresultsize, perrorcode)
}
#[inline]
pub unsafe fn uldn_scriptDisplayName<P0>(ldn: *const ULocaleDisplayNames, script: P0, result: *mut u16, maxresultsize: i32, perrorcode: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uldn_scriptDisplayName(ldn : *const ULocaleDisplayNames, script : windows_core::PCSTR, result : *mut u16, maxresultsize : i32, perrorcode : *mut UErrorCode) -> i32);
    uldn_scriptDisplayName(ldn, script.param().abi(), result, maxresultsize, perrorcode)
}
#[inline]
pub unsafe fn uldn_variantDisplayName<P0>(ldn: *const ULocaleDisplayNames, variant: P0, result: *mut u16, maxresultsize: i32, perrorcode: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uldn_variantDisplayName(ldn : *const ULocaleDisplayNames, variant : windows_core::PCSTR, result : *mut u16, maxresultsize : i32, perrorcode : *mut UErrorCode) -> i32);
    uldn_variantDisplayName(ldn, variant.param().abi(), result, maxresultsize, perrorcode)
}
#[inline]
pub unsafe fn ulistfmt_close(listfmt: *mut UListFormatter) {
    windows_targets::link!("icu.dll" "cdecl" fn ulistfmt_close(listfmt : *mut UListFormatter));
    ulistfmt_close(listfmt)
}
#[inline]
pub unsafe fn ulistfmt_closeResult(uresult: *mut UFormattedList) {
    windows_targets::link!("icu.dll" "cdecl" fn ulistfmt_closeResult(uresult : *mut UFormattedList));
    ulistfmt_closeResult(uresult)
}
#[inline]
pub unsafe fn ulistfmt_format(listfmt: *const UListFormatter, strings: *const *const u16, stringlengths: *const i32, stringcount: i32, result: *mut u16, resultcapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ulistfmt_format(listfmt : *const UListFormatter, strings : *const *const u16, stringlengths : *const i32, stringcount : i32, result : *mut u16, resultcapacity : i32, status : *mut UErrorCode) -> i32);
    ulistfmt_format(listfmt, strings, stringlengths, stringcount, result, resultcapacity, status)
}
#[inline]
pub unsafe fn ulistfmt_formatStringsToResult(listfmt: *const UListFormatter, strings: *const *const u16, stringlengths: *const i32, stringcount: i32, uresult: *mut UFormattedList, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn ulistfmt_formatStringsToResult(listfmt : *const UListFormatter, strings : *const *const u16, stringlengths : *const i32, stringcount : i32, uresult : *mut UFormattedList, status : *mut UErrorCode));
    ulistfmt_formatStringsToResult(listfmt, strings, stringlengths, stringcount, uresult, status)
}
#[inline]
pub unsafe fn ulistfmt_open<P0>(locale: P0, status: *mut UErrorCode) -> *mut UListFormatter
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ulistfmt_open(locale : windows_core::PCSTR, status : *mut UErrorCode) -> *mut UListFormatter);
    ulistfmt_open(locale.param().abi(), status)
}
#[inline]
pub unsafe fn ulistfmt_openForType<P0>(locale: P0, r#type: UListFormatterType, width: UListFormatterWidth, status: *mut UErrorCode) -> *mut UListFormatter
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ulistfmt_openForType(locale : windows_core::PCSTR, r#type : UListFormatterType, width : UListFormatterWidth, status : *mut UErrorCode) -> *mut UListFormatter);
    ulistfmt_openForType(locale.param().abi(), r#type, width, status)
}
#[inline]
pub unsafe fn ulistfmt_openResult(ec: *mut UErrorCode) -> *mut UFormattedList {
    windows_targets::link!("icu.dll" "cdecl" fn ulistfmt_openResult(ec : *mut UErrorCode) -> *mut UFormattedList);
    ulistfmt_openResult(ec)
}
#[inline]
pub unsafe fn ulistfmt_resultAsValue(uresult: *const UFormattedList, ec: *mut UErrorCode) -> *mut UFormattedValue {
    windows_targets::link!("icu.dll" "cdecl" fn ulistfmt_resultAsValue(uresult : *const UFormattedList, ec : *mut UErrorCode) -> *mut UFormattedValue);
    ulistfmt_resultAsValue(uresult, ec)
}
#[inline]
pub unsafe fn uloc_acceptLanguage<P0>(result: P0, resultavailable: i32, outresult: *mut UAcceptResult, acceptlist: *const *const i8, acceptlistcount: i32, availablelocales: *mut UEnumeration, status: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uloc_acceptLanguage(result : windows_core::PCSTR, resultavailable : i32, outresult : *mut UAcceptResult, acceptlist : *const *const i8, acceptlistcount : i32, availablelocales : *mut UEnumeration, status : *mut UErrorCode) -> i32);
    uloc_acceptLanguage(result.param().abi(), resultavailable, outresult, acceptlist, acceptlistcount, availablelocales, status)
}
#[inline]
pub unsafe fn uloc_acceptLanguageFromHTTP<P0, P1>(result: P0, resultavailable: i32, outresult: *mut UAcceptResult, httpacceptlanguage: P1, availablelocales: *mut UEnumeration, status: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uloc_acceptLanguageFromHTTP(result : windows_core::PCSTR, resultavailable : i32, outresult : *mut UAcceptResult, httpacceptlanguage : windows_core::PCSTR, availablelocales : *mut UEnumeration, status : *mut UErrorCode) -> i32);
    uloc_acceptLanguageFromHTTP(result.param().abi(), resultavailable, outresult, httpacceptlanguage.param().abi(), availablelocales, status)
}
#[inline]
pub unsafe fn uloc_addLikelySubtags<P0, P1>(localeid: P0, maximizedlocaleid: P1, maximizedlocaleidcapacity: i32, err: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uloc_addLikelySubtags(localeid : windows_core::PCSTR, maximizedlocaleid : windows_core::PCSTR, maximizedlocaleidcapacity : i32, err : *mut UErrorCode) -> i32);
    uloc_addLikelySubtags(localeid.param().abi(), maximizedlocaleid.param().abi(), maximizedlocaleidcapacity, err)
}
#[inline]
pub unsafe fn uloc_canonicalize<P0, P1>(localeid: P0, name: P1, namecapacity: i32, err: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uloc_canonicalize(localeid : windows_core::PCSTR, name : windows_core::PCSTR, namecapacity : i32, err : *mut UErrorCode) -> i32);
    uloc_canonicalize(localeid.param().abi(), name.param().abi(), namecapacity, err)
}
#[inline]
pub unsafe fn uloc_countAvailable() -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn uloc_countAvailable() -> i32);
    uloc_countAvailable()
}
#[inline]
pub unsafe fn uloc_forLanguageTag<P0, P1>(langtag: P0, localeid: P1, localeidcapacity: i32, parsedlength: *mut i32, err: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uloc_forLanguageTag(langtag : windows_core::PCSTR, localeid : windows_core::PCSTR, localeidcapacity : i32, parsedlength : *mut i32, err : *mut UErrorCode) -> i32);
    uloc_forLanguageTag(langtag.param().abi(), localeid.param().abi(), localeidcapacity, parsedlength, err)
}
#[inline]
pub unsafe fn uloc_getAvailable(n: i32) -> windows_core::PCSTR {
    windows_targets::link!("icu.dll" "cdecl" fn uloc_getAvailable(n : i32) -> windows_core::PCSTR);
    uloc_getAvailable(n)
}
#[inline]
pub unsafe fn uloc_getBaseName<P0, P1>(localeid: P0, name: P1, namecapacity: i32, err: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uloc_getBaseName(localeid : windows_core::PCSTR, name : windows_core::PCSTR, namecapacity : i32, err : *mut UErrorCode) -> i32);
    uloc_getBaseName(localeid.param().abi(), name.param().abi(), namecapacity, err)
}
#[inline]
pub unsafe fn uloc_getCharacterOrientation<P0>(localeid: P0, status: *mut UErrorCode) -> ULayoutType
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uloc_getCharacterOrientation(localeid : windows_core::PCSTR, status : *mut UErrorCode) -> ULayoutType);
    uloc_getCharacterOrientation(localeid.param().abi(), status)
}
#[inline]
pub unsafe fn uloc_getCountry<P0, P1>(localeid: P0, country: P1, countrycapacity: i32, err: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uloc_getCountry(localeid : windows_core::PCSTR, country : windows_core::PCSTR, countrycapacity : i32, err : *mut UErrorCode) -> i32);
    uloc_getCountry(localeid.param().abi(), country.param().abi(), countrycapacity, err)
}
#[inline]
pub unsafe fn uloc_getDefault() -> windows_core::PCSTR {
    windows_targets::link!("icu.dll" "cdecl" fn uloc_getDefault() -> windows_core::PCSTR);
    uloc_getDefault()
}
#[inline]
pub unsafe fn uloc_getDisplayCountry<P0, P1>(locale: P0, displaylocale: P1, country: *mut u16, countrycapacity: i32, status: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uloc_getDisplayCountry(locale : windows_core::PCSTR, displaylocale : windows_core::PCSTR, country : *mut u16, countrycapacity : i32, status : *mut UErrorCode) -> i32);
    uloc_getDisplayCountry(locale.param().abi(), displaylocale.param().abi(), country, countrycapacity, status)
}
#[inline]
pub unsafe fn uloc_getDisplayKeyword<P0, P1>(keyword: P0, displaylocale: P1, dest: *mut u16, destcapacity: i32, status: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uloc_getDisplayKeyword(keyword : windows_core::PCSTR, displaylocale : windows_core::PCSTR, dest : *mut u16, destcapacity : i32, status : *mut UErrorCode) -> i32);
    uloc_getDisplayKeyword(keyword.param().abi(), displaylocale.param().abi(), dest, destcapacity, status)
}
#[inline]
pub unsafe fn uloc_getDisplayKeywordValue<P0, P1, P2>(locale: P0, keyword: P1, displaylocale: P2, dest: *mut u16, destcapacity: i32, status: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uloc_getDisplayKeywordValue(locale : windows_core::PCSTR, keyword : windows_core::PCSTR, displaylocale : windows_core::PCSTR, dest : *mut u16, destcapacity : i32, status : *mut UErrorCode) -> i32);
    uloc_getDisplayKeywordValue(locale.param().abi(), keyword.param().abi(), displaylocale.param().abi(), dest, destcapacity, status)
}
#[inline]
pub unsafe fn uloc_getDisplayLanguage<P0, P1>(locale: P0, displaylocale: P1, language: *mut u16, languagecapacity: i32, status: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uloc_getDisplayLanguage(locale : windows_core::PCSTR, displaylocale : windows_core::PCSTR, language : *mut u16, languagecapacity : i32, status : *mut UErrorCode) -> i32);
    uloc_getDisplayLanguage(locale.param().abi(), displaylocale.param().abi(), language, languagecapacity, status)
}
#[inline]
pub unsafe fn uloc_getDisplayName<P0, P1>(localeid: P0, inlocaleid: P1, result: *mut u16, maxresultsize: i32, err: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uloc_getDisplayName(localeid : windows_core::PCSTR, inlocaleid : windows_core::PCSTR, result : *mut u16, maxresultsize : i32, err : *mut UErrorCode) -> i32);
    uloc_getDisplayName(localeid.param().abi(), inlocaleid.param().abi(), result, maxresultsize, err)
}
#[inline]
pub unsafe fn uloc_getDisplayScript<P0, P1>(locale: P0, displaylocale: P1, script: *mut u16, scriptcapacity: i32, status: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uloc_getDisplayScript(locale : windows_core::PCSTR, displaylocale : windows_core::PCSTR, script : *mut u16, scriptcapacity : i32, status : *mut UErrorCode) -> i32);
    uloc_getDisplayScript(locale.param().abi(), displaylocale.param().abi(), script, scriptcapacity, status)
}
#[inline]
pub unsafe fn uloc_getDisplayVariant<P0, P1>(locale: P0, displaylocale: P1, variant: *mut u16, variantcapacity: i32, status: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uloc_getDisplayVariant(locale : windows_core::PCSTR, displaylocale : windows_core::PCSTR, variant : *mut u16, variantcapacity : i32, status : *mut UErrorCode) -> i32);
    uloc_getDisplayVariant(locale.param().abi(), displaylocale.param().abi(), variant, variantcapacity, status)
}
#[inline]
pub unsafe fn uloc_getISO3Country<P0>(localeid: P0) -> windows_core::PCSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uloc_getISO3Country(localeid : windows_core::PCSTR) -> windows_core::PCSTR);
    uloc_getISO3Country(localeid.param().abi())
}
#[inline]
pub unsafe fn uloc_getISO3Language<P0>(localeid: P0) -> windows_core::PCSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uloc_getISO3Language(localeid : windows_core::PCSTR) -> windows_core::PCSTR);
    uloc_getISO3Language(localeid.param().abi())
}
#[inline]
pub unsafe fn uloc_getISOCountries() -> *mut *mut i8 {
    windows_targets::link!("icu.dll" "cdecl" fn uloc_getISOCountries() -> *mut *mut i8);
    uloc_getISOCountries()
}
#[inline]
pub unsafe fn uloc_getISOLanguages() -> *mut *mut i8 {
    windows_targets::link!("icu.dll" "cdecl" fn uloc_getISOLanguages() -> *mut *mut i8);
    uloc_getISOLanguages()
}
#[inline]
pub unsafe fn uloc_getKeywordValue<P0, P1, P2>(localeid: P0, keywordname: P1, buffer: P2, buffercapacity: i32, status: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uloc_getKeywordValue(localeid : windows_core::PCSTR, keywordname : windows_core::PCSTR, buffer : windows_core::PCSTR, buffercapacity : i32, status : *mut UErrorCode) -> i32);
    uloc_getKeywordValue(localeid.param().abi(), keywordname.param().abi(), buffer.param().abi(), buffercapacity, status)
}
#[inline]
pub unsafe fn uloc_getLCID<P0>(localeid: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uloc_getLCID(localeid : windows_core::PCSTR) -> u32);
    uloc_getLCID(localeid.param().abi())
}
#[inline]
pub unsafe fn uloc_getLanguage<P0, P1>(localeid: P0, language: P1, languagecapacity: i32, err: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uloc_getLanguage(localeid : windows_core::PCSTR, language : windows_core::PCSTR, languagecapacity : i32, err : *mut UErrorCode) -> i32);
    uloc_getLanguage(localeid.param().abi(), language.param().abi(), languagecapacity, err)
}
#[inline]
pub unsafe fn uloc_getLineOrientation<P0>(localeid: P0, status: *mut UErrorCode) -> ULayoutType
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uloc_getLineOrientation(localeid : windows_core::PCSTR, status : *mut UErrorCode) -> ULayoutType);
    uloc_getLineOrientation(localeid.param().abi(), status)
}
#[inline]
pub unsafe fn uloc_getLocaleForLCID<P0>(hostid: u32, locale: P0, localecapacity: i32, status: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uloc_getLocaleForLCID(hostid : u32, locale : windows_core::PCSTR, localecapacity : i32, status : *mut UErrorCode) -> i32);
    uloc_getLocaleForLCID(hostid, locale.param().abi(), localecapacity, status)
}
#[inline]
pub unsafe fn uloc_getName<P0, P1>(localeid: P0, name: P1, namecapacity: i32, err: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uloc_getName(localeid : windows_core::PCSTR, name : windows_core::PCSTR, namecapacity : i32, err : *mut UErrorCode) -> i32);
    uloc_getName(localeid.param().abi(), name.param().abi(), namecapacity, err)
}
#[inline]
pub unsafe fn uloc_getParent<P0, P1>(localeid: P0, parent: P1, parentcapacity: i32, err: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uloc_getParent(localeid : windows_core::PCSTR, parent : windows_core::PCSTR, parentcapacity : i32, err : *mut UErrorCode) -> i32);
    uloc_getParent(localeid.param().abi(), parent.param().abi(), parentcapacity, err)
}
#[inline]
pub unsafe fn uloc_getScript<P0, P1>(localeid: P0, script: P1, scriptcapacity: i32, err: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uloc_getScript(localeid : windows_core::PCSTR, script : windows_core::PCSTR, scriptcapacity : i32, err : *mut UErrorCode) -> i32);
    uloc_getScript(localeid.param().abi(), script.param().abi(), scriptcapacity, err)
}
#[inline]
pub unsafe fn uloc_getVariant<P0, P1>(localeid: P0, variant: P1, variantcapacity: i32, err: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uloc_getVariant(localeid : windows_core::PCSTR, variant : windows_core::PCSTR, variantcapacity : i32, err : *mut UErrorCode) -> i32);
    uloc_getVariant(localeid.param().abi(), variant.param().abi(), variantcapacity, err)
}
#[inline]
pub unsafe fn uloc_isRightToLeft<P0>(locale: P0) -> i8
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uloc_isRightToLeft(locale : windows_core::PCSTR) -> i8);
    uloc_isRightToLeft(locale.param().abi())
}
#[inline]
pub unsafe fn uloc_minimizeSubtags<P0, P1>(localeid: P0, minimizedlocaleid: P1, minimizedlocaleidcapacity: i32, err: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uloc_minimizeSubtags(localeid : windows_core::PCSTR, minimizedlocaleid : windows_core::PCSTR, minimizedlocaleidcapacity : i32, err : *mut UErrorCode) -> i32);
    uloc_minimizeSubtags(localeid.param().abi(), minimizedlocaleid.param().abi(), minimizedlocaleidcapacity, err)
}
#[inline]
pub unsafe fn uloc_openAvailableByType(r#type: ULocAvailableType, status: *mut UErrorCode) -> *mut UEnumeration {
    windows_targets::link!("icu.dll" "cdecl" fn uloc_openAvailableByType(r#type : ULocAvailableType, status : *mut UErrorCode) -> *mut UEnumeration);
    uloc_openAvailableByType(r#type, status)
}
#[inline]
pub unsafe fn uloc_openKeywords<P0>(localeid: P0, status: *mut UErrorCode) -> *mut UEnumeration
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uloc_openKeywords(localeid : windows_core::PCSTR, status : *mut UErrorCode) -> *mut UEnumeration);
    uloc_openKeywords(localeid.param().abi(), status)
}
#[inline]
pub unsafe fn uloc_setDefault<P0>(localeid: P0, status: *mut UErrorCode)
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uloc_setDefault(localeid : windows_core::PCSTR, status : *mut UErrorCode));
    uloc_setDefault(localeid.param().abi(), status)
}
#[inline]
pub unsafe fn uloc_setKeywordValue<P0, P1, P2>(keywordname: P0, keywordvalue: P1, buffer: P2, buffercapacity: i32, status: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uloc_setKeywordValue(keywordname : windows_core::PCSTR, keywordvalue : windows_core::PCSTR, buffer : windows_core::PCSTR, buffercapacity : i32, status : *mut UErrorCode) -> i32);
    uloc_setKeywordValue(keywordname.param().abi(), keywordvalue.param().abi(), buffer.param().abi(), buffercapacity, status)
}
#[inline]
pub unsafe fn uloc_toLanguageTag<P0, P1>(localeid: P0, langtag: P1, langtagcapacity: i32, strict: i8, err: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uloc_toLanguageTag(localeid : windows_core::PCSTR, langtag : windows_core::PCSTR, langtagcapacity : i32, strict : i8, err : *mut UErrorCode) -> i32);
    uloc_toLanguageTag(localeid.param().abi(), langtag.param().abi(), langtagcapacity, strict, err)
}
#[inline]
pub unsafe fn uloc_toLegacyKey<P0>(keyword: P0) -> windows_core::PCSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uloc_toLegacyKey(keyword : windows_core::PCSTR) -> windows_core::PCSTR);
    uloc_toLegacyKey(keyword.param().abi())
}
#[inline]
pub unsafe fn uloc_toLegacyType<P0, P1>(keyword: P0, value: P1) -> windows_core::PCSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uloc_toLegacyType(keyword : windows_core::PCSTR, value : windows_core::PCSTR) -> windows_core::PCSTR);
    uloc_toLegacyType(keyword.param().abi(), value.param().abi())
}
#[inline]
pub unsafe fn uloc_toUnicodeLocaleKey<P0>(keyword: P0) -> windows_core::PCSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uloc_toUnicodeLocaleKey(keyword : windows_core::PCSTR) -> windows_core::PCSTR);
    uloc_toUnicodeLocaleKey(keyword.param().abi())
}
#[inline]
pub unsafe fn uloc_toUnicodeLocaleType<P0, P1>(keyword: P0, value: P1) -> windows_core::PCSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uloc_toUnicodeLocaleType(keyword : windows_core::PCSTR, value : windows_core::PCSTR) -> windows_core::PCSTR);
    uloc_toUnicodeLocaleType(keyword.param().abi(), value.param().abi())
}
#[inline]
pub unsafe fn ulocdata_close(uld: *mut ULocaleData) {
    windows_targets::link!("icu.dll" "cdecl" fn ulocdata_close(uld : *mut ULocaleData));
    ulocdata_close(uld)
}
#[inline]
pub unsafe fn ulocdata_getCLDRVersion(versionarray: *mut u8, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn ulocdata_getCLDRVersion(versionarray : *mut u8, status : *mut UErrorCode));
    ulocdata_getCLDRVersion(versionarray, status)
}
#[inline]
pub unsafe fn ulocdata_getDelimiter(uld: *mut ULocaleData, r#type: ULocaleDataDelimiterType, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ulocdata_getDelimiter(uld : *mut ULocaleData, r#type : ULocaleDataDelimiterType, result : *mut u16, resultlength : i32, status : *mut UErrorCode) -> i32);
    ulocdata_getDelimiter(uld, r#type, result, resultlength, status)
}
#[inline]
pub unsafe fn ulocdata_getExemplarSet(uld: *mut ULocaleData, fillin: *mut USet, options: u32, extype: ULocaleDataExemplarSetType, status: *mut UErrorCode) -> *mut USet {
    windows_targets::link!("icu.dll" "cdecl" fn ulocdata_getExemplarSet(uld : *mut ULocaleData, fillin : *mut USet, options : u32, extype : ULocaleDataExemplarSetType, status : *mut UErrorCode) -> *mut USet);
    ulocdata_getExemplarSet(uld, fillin, options, extype, status)
}
#[inline]
pub unsafe fn ulocdata_getLocaleDisplayPattern(uld: *mut ULocaleData, pattern: *mut u16, patterncapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ulocdata_getLocaleDisplayPattern(uld : *mut ULocaleData, pattern : *mut u16, patterncapacity : i32, status : *mut UErrorCode) -> i32);
    ulocdata_getLocaleDisplayPattern(uld, pattern, patterncapacity, status)
}
#[inline]
pub unsafe fn ulocdata_getLocaleSeparator(uld: *mut ULocaleData, separator: *mut u16, separatorcapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ulocdata_getLocaleSeparator(uld : *mut ULocaleData, separator : *mut u16, separatorcapacity : i32, status : *mut UErrorCode) -> i32);
    ulocdata_getLocaleSeparator(uld, separator, separatorcapacity, status)
}
#[inline]
pub unsafe fn ulocdata_getMeasurementSystem<P0>(localeid: P0, status: *mut UErrorCode) -> UMeasurementSystem
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ulocdata_getMeasurementSystem(localeid : windows_core::PCSTR, status : *mut UErrorCode) -> UMeasurementSystem);
    ulocdata_getMeasurementSystem(localeid.param().abi(), status)
}
#[inline]
pub unsafe fn ulocdata_getNoSubstitute(uld: *mut ULocaleData) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn ulocdata_getNoSubstitute(uld : *mut ULocaleData) -> i8);
    ulocdata_getNoSubstitute(uld)
}
#[inline]
pub unsafe fn ulocdata_getPaperSize<P0>(localeid: P0, height: *mut i32, width: *mut i32, status: *mut UErrorCode)
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ulocdata_getPaperSize(localeid : windows_core::PCSTR, height : *mut i32, width : *mut i32, status : *mut UErrorCode));
    ulocdata_getPaperSize(localeid.param().abi(), height, width, status)
}
#[inline]
pub unsafe fn ulocdata_open<P0>(localeid: P0, status: *mut UErrorCode) -> *mut ULocaleData
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ulocdata_open(localeid : windows_core::PCSTR, status : *mut UErrorCode) -> *mut ULocaleData);
    ulocdata_open(localeid.param().abi(), status)
}
#[inline]
pub unsafe fn ulocdata_setNoSubstitute(uld: *mut ULocaleData, setting: i8) {
    windows_targets::link!("icu.dll" "cdecl" fn ulocdata_setNoSubstitute(uld : *mut ULocaleData, setting : i8));
    ulocdata_setNoSubstitute(uld, setting)
}
#[inline]
pub unsafe fn umsg_applyPattern(fmt: *mut *mut core::ffi::c_void, pattern: *const u16, patternlength: i32, parseerror: *mut UParseError, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn umsg_applyPattern(fmt : *mut *mut core::ffi::c_void, pattern : *const u16, patternlength : i32, parseerror : *mut UParseError, status : *mut UErrorCode));
    umsg_applyPattern(fmt, pattern, patternlength, parseerror, status)
}
#[inline]
pub unsafe fn umsg_autoQuoteApostrophe(pattern: *const u16, patternlength: i32, dest: *mut u16, destcapacity: i32, ec: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn umsg_autoQuoteApostrophe(pattern : *const u16, patternlength : i32, dest : *mut u16, destcapacity : i32, ec : *mut UErrorCode) -> i32);
    umsg_autoQuoteApostrophe(pattern, patternlength, dest, destcapacity, ec)
}
#[inline]
pub unsafe fn umsg_clone(fmt: *const *const core::ffi::c_void, status: *mut UErrorCode) -> *mut core::ffi::c_void {
    windows_targets::link!("icu.dll" "cdecl" fn umsg_clone(fmt : *const *const core::ffi::c_void, status : *mut UErrorCode) -> *mut core::ffi::c_void);
    umsg_clone(fmt, status)
}
#[inline]
pub unsafe fn umsg_close(format: *mut *mut core::ffi::c_void) {
    windows_targets::link!("icu.dll" "cdecl" fn umsg_close(format : *mut *mut core::ffi::c_void));
    umsg_close(format)
}
#[inline]
pub unsafe fn umsg_format(fmt: *const *const core::ffi::c_void, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn umsg_format(fmt : *const *const core::ffi::c_void, result : *mut u16, resultlength : i32, status : *mut UErrorCode) -> i32);
    umsg_format(fmt, result, resultlength, status)
}
#[inline]
pub unsafe fn umsg_getLocale(fmt: *const *const core::ffi::c_void) -> windows_core::PCSTR {
    windows_targets::link!("icu.dll" "cdecl" fn umsg_getLocale(fmt : *const *const core::ffi::c_void) -> windows_core::PCSTR);
    umsg_getLocale(fmt)
}
#[inline]
pub unsafe fn umsg_open<P0>(pattern: *const u16, patternlength: i32, locale: P0, parseerror: *mut UParseError, status: *mut UErrorCode) -> *mut *mut core::ffi::c_void
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn umsg_open(pattern : *const u16, patternlength : i32, locale : windows_core::PCSTR, parseerror : *mut UParseError, status : *mut UErrorCode) -> *mut *mut core::ffi::c_void);
    umsg_open(pattern, patternlength, locale.param().abi(), parseerror, status)
}
#[inline]
pub unsafe fn umsg_parse(fmt: *const *const core::ffi::c_void, source: *const u16, sourcelength: i32, count: *mut i32, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn umsg_parse(fmt : *const *const core::ffi::c_void, source : *const u16, sourcelength : i32, count : *mut i32, status : *mut UErrorCode));
    umsg_parse(fmt, source, sourcelength, count, status)
}
#[inline]
pub unsafe fn umsg_setLocale<P0>(fmt: *mut *mut core::ffi::c_void, locale: P0)
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn umsg_setLocale(fmt : *mut *mut core::ffi::c_void, locale : windows_core::PCSTR));
    umsg_setLocale(fmt, locale.param().abi())
}
#[inline]
pub unsafe fn umsg_toPattern(fmt: *const *const core::ffi::c_void, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn umsg_toPattern(fmt : *const *const core::ffi::c_void, result : *mut u16, resultlength : i32, status : *mut UErrorCode) -> i32);
    umsg_toPattern(fmt, result, resultlength, status)
}
#[inline]
pub unsafe fn umsg_vformat(fmt: *const *const core::ffi::c_void, result: *mut u16, resultlength: i32, ap: *mut i8, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn umsg_vformat(fmt : *const *const core::ffi::c_void, result : *mut u16, resultlength : i32, ap : *mut i8, status : *mut UErrorCode) -> i32);
    umsg_vformat(fmt, result, resultlength, ap, status)
}
#[inline]
pub unsafe fn umsg_vparse(fmt: *const *const core::ffi::c_void, source: *const u16, sourcelength: i32, count: *mut i32, ap: *mut i8, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn umsg_vparse(fmt : *const *const core::ffi::c_void, source : *const u16, sourcelength : i32, count : *mut i32, ap : *mut i8, status : *mut UErrorCode));
    umsg_vparse(fmt, source, sourcelength, count, ap, status)
}
#[inline]
pub unsafe fn umutablecptrie_buildImmutable(trie: *mut UMutableCPTrie, r#type: UCPTrieType, valuewidth: UCPTrieValueWidth, perrorcode: *mut UErrorCode) -> *mut UCPTrie {
    windows_targets::link!("icu.dll" "cdecl" fn umutablecptrie_buildImmutable(trie : *mut UMutableCPTrie, r#type : UCPTrieType, valuewidth : UCPTrieValueWidth, perrorcode : *mut UErrorCode) -> *mut UCPTrie);
    umutablecptrie_buildImmutable(trie, r#type, valuewidth, perrorcode)
}
#[inline]
pub unsafe fn umutablecptrie_clone(other: *const UMutableCPTrie, perrorcode: *mut UErrorCode) -> *mut UMutableCPTrie {
    windows_targets::link!("icu.dll" "cdecl" fn umutablecptrie_clone(other : *const UMutableCPTrie, perrorcode : *mut UErrorCode) -> *mut UMutableCPTrie);
    umutablecptrie_clone(other, perrorcode)
}
#[inline]
pub unsafe fn umutablecptrie_close(trie: *mut UMutableCPTrie) {
    windows_targets::link!("icu.dll" "cdecl" fn umutablecptrie_close(trie : *mut UMutableCPTrie));
    umutablecptrie_close(trie)
}
#[inline]
pub unsafe fn umutablecptrie_fromUCPMap(map: *const UCPMap, perrorcode: *mut UErrorCode) -> *mut UMutableCPTrie {
    windows_targets::link!("icu.dll" "cdecl" fn umutablecptrie_fromUCPMap(map : *const UCPMap, perrorcode : *mut UErrorCode) -> *mut UMutableCPTrie);
    umutablecptrie_fromUCPMap(map, perrorcode)
}
#[inline]
pub unsafe fn umutablecptrie_fromUCPTrie(trie: *const UCPTrie, perrorcode: *mut UErrorCode) -> *mut UMutableCPTrie {
    windows_targets::link!("icu.dll" "cdecl" fn umutablecptrie_fromUCPTrie(trie : *const UCPTrie, perrorcode : *mut UErrorCode) -> *mut UMutableCPTrie);
    umutablecptrie_fromUCPTrie(trie, perrorcode)
}
#[inline]
pub unsafe fn umutablecptrie_get(trie: *const UMutableCPTrie, c: i32) -> u32 {
    windows_targets::link!("icu.dll" "cdecl" fn umutablecptrie_get(trie : *const UMutableCPTrie, c : i32) -> u32);
    umutablecptrie_get(trie, c)
}
#[inline]
pub unsafe fn umutablecptrie_getRange(trie: *const UMutableCPTrie, start: i32, option: UCPMapRangeOption, surrogatevalue: u32, filter: *mut UCPMapValueFilter, context: *const core::ffi::c_void, pvalue: *mut u32) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn umutablecptrie_getRange(trie : *const UMutableCPTrie, start : i32, option : UCPMapRangeOption, surrogatevalue : u32, filter : *mut UCPMapValueFilter, context : *const core::ffi::c_void, pvalue : *mut u32) -> i32);
    umutablecptrie_getRange(trie, start, option, surrogatevalue, filter, context, pvalue)
}
#[inline]
pub unsafe fn umutablecptrie_open(initialvalue: u32, errorvalue: u32, perrorcode: *mut UErrorCode) -> *mut UMutableCPTrie {
    windows_targets::link!("icu.dll" "cdecl" fn umutablecptrie_open(initialvalue : u32, errorvalue : u32, perrorcode : *mut UErrorCode) -> *mut UMutableCPTrie);
    umutablecptrie_open(initialvalue, errorvalue, perrorcode)
}
#[inline]
pub unsafe fn umutablecptrie_set(trie: *mut UMutableCPTrie, c: i32, value: u32, perrorcode: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn umutablecptrie_set(trie : *mut UMutableCPTrie, c : i32, value : u32, perrorcode : *mut UErrorCode));
    umutablecptrie_set(trie, c, value, perrorcode)
}
#[inline]
pub unsafe fn umutablecptrie_setRange(trie: *mut UMutableCPTrie, start: i32, end: i32, value: u32, perrorcode: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn umutablecptrie_setRange(trie : *mut UMutableCPTrie, start : i32, end : i32, value : u32, perrorcode : *mut UErrorCode));
    umutablecptrie_setRange(trie, start, end, value, perrorcode)
}
#[inline]
pub unsafe fn unorm2_append(norm2: *const UNormalizer2, first: *mut u16, firstlength: i32, firstcapacity: i32, second: *const u16, secondlength: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn unorm2_append(norm2 : *const UNormalizer2, first : *mut u16, firstlength : i32, firstcapacity : i32, second : *const u16, secondlength : i32, perrorcode : *mut UErrorCode) -> i32);
    unorm2_append(norm2, first, firstlength, firstcapacity, second, secondlength, perrorcode)
}
#[inline]
pub unsafe fn unorm2_close(norm2: *mut UNormalizer2) {
    windows_targets::link!("icu.dll" "cdecl" fn unorm2_close(norm2 : *mut UNormalizer2));
    unorm2_close(norm2)
}
#[inline]
pub unsafe fn unorm2_composePair(norm2: *const UNormalizer2, a: i32, b: i32) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn unorm2_composePair(norm2 : *const UNormalizer2, a : i32, b : i32) -> i32);
    unorm2_composePair(norm2, a, b)
}
#[inline]
pub unsafe fn unorm2_getCombiningClass(norm2: *const UNormalizer2, c: i32) -> u8 {
    windows_targets::link!("icu.dll" "cdecl" fn unorm2_getCombiningClass(norm2 : *const UNormalizer2, c : i32) -> u8);
    unorm2_getCombiningClass(norm2, c)
}
#[inline]
pub unsafe fn unorm2_getDecomposition(norm2: *const UNormalizer2, c: i32, decomposition: *mut u16, capacity: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn unorm2_getDecomposition(norm2 : *const UNormalizer2, c : i32, decomposition : *mut u16, capacity : i32, perrorcode : *mut UErrorCode) -> i32);
    unorm2_getDecomposition(norm2, c, decomposition, capacity, perrorcode)
}
#[inline]
pub unsafe fn unorm2_getInstance<P0, P1>(packagename: P0, name: P1, mode: UNormalization2Mode, perrorcode: *mut UErrorCode) -> *mut UNormalizer2
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn unorm2_getInstance(packagename : windows_core::PCSTR, name : windows_core::PCSTR, mode : UNormalization2Mode, perrorcode : *mut UErrorCode) -> *mut UNormalizer2);
    unorm2_getInstance(packagename.param().abi(), name.param().abi(), mode, perrorcode)
}
#[inline]
pub unsafe fn unorm2_getNFCInstance(perrorcode: *mut UErrorCode) -> *mut UNormalizer2 {
    windows_targets::link!("icu.dll" "cdecl" fn unorm2_getNFCInstance(perrorcode : *mut UErrorCode) -> *mut UNormalizer2);
    unorm2_getNFCInstance(perrorcode)
}
#[inline]
pub unsafe fn unorm2_getNFDInstance(perrorcode: *mut UErrorCode) -> *mut UNormalizer2 {
    windows_targets::link!("icu.dll" "cdecl" fn unorm2_getNFDInstance(perrorcode : *mut UErrorCode) -> *mut UNormalizer2);
    unorm2_getNFDInstance(perrorcode)
}
#[inline]
pub unsafe fn unorm2_getNFKCCasefoldInstance(perrorcode: *mut UErrorCode) -> *mut UNormalizer2 {
    windows_targets::link!("icu.dll" "cdecl" fn unorm2_getNFKCCasefoldInstance(perrorcode : *mut UErrorCode) -> *mut UNormalizer2);
    unorm2_getNFKCCasefoldInstance(perrorcode)
}
#[inline]
pub unsafe fn unorm2_getNFKCInstance(perrorcode: *mut UErrorCode) -> *mut UNormalizer2 {
    windows_targets::link!("icu.dll" "cdecl" fn unorm2_getNFKCInstance(perrorcode : *mut UErrorCode) -> *mut UNormalizer2);
    unorm2_getNFKCInstance(perrorcode)
}
#[inline]
pub unsafe fn unorm2_getNFKDInstance(perrorcode: *mut UErrorCode) -> *mut UNormalizer2 {
    windows_targets::link!("icu.dll" "cdecl" fn unorm2_getNFKDInstance(perrorcode : *mut UErrorCode) -> *mut UNormalizer2);
    unorm2_getNFKDInstance(perrorcode)
}
#[inline]
pub unsafe fn unorm2_getRawDecomposition(norm2: *const UNormalizer2, c: i32, decomposition: *mut u16, capacity: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn unorm2_getRawDecomposition(norm2 : *const UNormalizer2, c : i32, decomposition : *mut u16, capacity : i32, perrorcode : *mut UErrorCode) -> i32);
    unorm2_getRawDecomposition(norm2, c, decomposition, capacity, perrorcode)
}
#[inline]
pub unsafe fn unorm2_hasBoundaryAfter(norm2: *const UNormalizer2, c: i32) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn unorm2_hasBoundaryAfter(norm2 : *const UNormalizer2, c : i32) -> i8);
    unorm2_hasBoundaryAfter(norm2, c)
}
#[inline]
pub unsafe fn unorm2_hasBoundaryBefore(norm2: *const UNormalizer2, c: i32) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn unorm2_hasBoundaryBefore(norm2 : *const UNormalizer2, c : i32) -> i8);
    unorm2_hasBoundaryBefore(norm2, c)
}
#[inline]
pub unsafe fn unorm2_isInert(norm2: *const UNormalizer2, c: i32) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn unorm2_isInert(norm2 : *const UNormalizer2, c : i32) -> i8);
    unorm2_isInert(norm2, c)
}
#[inline]
pub unsafe fn unorm2_isNormalized(norm2: *const UNormalizer2, s: *const u16, length: i32, perrorcode: *mut UErrorCode) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn unorm2_isNormalized(norm2 : *const UNormalizer2, s : *const u16, length : i32, perrorcode : *mut UErrorCode) -> i8);
    unorm2_isNormalized(norm2, s, length, perrorcode)
}
#[inline]
pub unsafe fn unorm2_normalize(norm2: *const UNormalizer2, src: *const u16, length: i32, dest: *mut u16, capacity: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn unorm2_normalize(norm2 : *const UNormalizer2, src : *const u16, length : i32, dest : *mut u16, capacity : i32, perrorcode : *mut UErrorCode) -> i32);
    unorm2_normalize(norm2, src, length, dest, capacity, perrorcode)
}
#[inline]
pub unsafe fn unorm2_normalizeSecondAndAppend(norm2: *const UNormalizer2, first: *mut u16, firstlength: i32, firstcapacity: i32, second: *const u16, secondlength: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn unorm2_normalizeSecondAndAppend(norm2 : *const UNormalizer2, first : *mut u16, firstlength : i32, firstcapacity : i32, second : *const u16, secondlength : i32, perrorcode : *mut UErrorCode) -> i32);
    unorm2_normalizeSecondAndAppend(norm2, first, firstlength, firstcapacity, second, secondlength, perrorcode)
}
#[inline]
pub unsafe fn unorm2_openFiltered(norm2: *const UNormalizer2, filterset: *const USet, perrorcode: *mut UErrorCode) -> *mut UNormalizer2 {
    windows_targets::link!("icu.dll" "cdecl" fn unorm2_openFiltered(norm2 : *const UNormalizer2, filterset : *const USet, perrorcode : *mut UErrorCode) -> *mut UNormalizer2);
    unorm2_openFiltered(norm2, filterset, perrorcode)
}
#[inline]
pub unsafe fn unorm2_quickCheck(norm2: *const UNormalizer2, s: *const u16, length: i32, perrorcode: *mut UErrorCode) -> UNormalizationCheckResult {
    windows_targets::link!("icu.dll" "cdecl" fn unorm2_quickCheck(norm2 : *const UNormalizer2, s : *const u16, length : i32, perrorcode : *mut UErrorCode) -> UNormalizationCheckResult);
    unorm2_quickCheck(norm2, s, length, perrorcode)
}
#[inline]
pub unsafe fn unorm2_spanQuickCheckYes(norm2: *const UNormalizer2, s: *const u16, length: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn unorm2_spanQuickCheckYes(norm2 : *const UNormalizer2, s : *const u16, length : i32, perrorcode : *mut UErrorCode) -> i32);
    unorm2_spanQuickCheckYes(norm2, s, length, perrorcode)
}
#[inline]
pub unsafe fn unorm_compare(s1: *const u16, length1: i32, s2: *const u16, length2: i32, options: u32, perrorcode: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn unorm_compare(s1 : *const u16, length1 : i32, s2 : *const u16, length2 : i32, options : u32, perrorcode : *mut UErrorCode) -> i32);
    unorm_compare(s1, length1, s2, length2, options, perrorcode)
}
#[inline]
pub unsafe fn unum_applyPattern(format: *mut *mut core::ffi::c_void, localized: i8, pattern: *const u16, patternlength: i32, parseerror: *mut UParseError, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn unum_applyPattern(format : *mut *mut core::ffi::c_void, localized : i8, pattern : *const u16, patternlength : i32, parseerror : *mut UParseError, status : *mut UErrorCode));
    unum_applyPattern(format, localized, pattern, patternlength, parseerror, status)
}
#[inline]
pub unsafe fn unum_clone(fmt: *const *const core::ffi::c_void, status: *mut UErrorCode) -> *mut *mut core::ffi::c_void {
    windows_targets::link!("icu.dll" "cdecl" fn unum_clone(fmt : *const *const core::ffi::c_void, status : *mut UErrorCode) -> *mut *mut core::ffi::c_void);
    unum_clone(fmt, status)
}
#[inline]
pub unsafe fn unum_close(fmt: *mut *mut core::ffi::c_void) {
    windows_targets::link!("icu.dll" "cdecl" fn unum_close(fmt : *mut *mut core::ffi::c_void));
    unum_close(fmt)
}
#[inline]
pub unsafe fn unum_countAvailable() -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn unum_countAvailable() -> i32);
    unum_countAvailable()
}
#[inline]
pub unsafe fn unum_format(fmt: *const *const core::ffi::c_void, number: i32, result: *mut u16, resultlength: i32, pos: *mut UFieldPosition, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn unum_format(fmt : *const *const core::ffi::c_void, number : i32, result : *mut u16, resultlength : i32, pos : *mut UFieldPosition, status : *mut UErrorCode) -> i32);
    unum_format(fmt, number, result, resultlength, pos, status)
}
#[inline]
pub unsafe fn unum_formatDecimal<P0>(fmt: *const *const core::ffi::c_void, number: P0, length: i32, result: *mut u16, resultlength: i32, pos: *mut UFieldPosition, status: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn unum_formatDecimal(fmt : *const *const core::ffi::c_void, number : windows_core::PCSTR, length : i32, result : *mut u16, resultlength : i32, pos : *mut UFieldPosition, status : *mut UErrorCode) -> i32);
    unum_formatDecimal(fmt, number.param().abi(), length, result, resultlength, pos, status)
}
#[inline]
pub unsafe fn unum_formatDouble(fmt: *const *const core::ffi::c_void, number: f64, result: *mut u16, resultlength: i32, pos: *mut UFieldPosition, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn unum_formatDouble(fmt : *const *const core::ffi::c_void, number : f64, result : *mut u16, resultlength : i32, pos : *mut UFieldPosition, status : *mut UErrorCode) -> i32);
    unum_formatDouble(fmt, number, result, resultlength, pos, status)
}
#[inline]
pub unsafe fn unum_formatDoubleCurrency(fmt: *const *const core::ffi::c_void, number: f64, currency: *mut u16, result: *mut u16, resultlength: i32, pos: *mut UFieldPosition, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn unum_formatDoubleCurrency(fmt : *const *const core::ffi::c_void, number : f64, currency : *mut u16, result : *mut u16, resultlength : i32, pos : *mut UFieldPosition, status : *mut UErrorCode) -> i32);
    unum_formatDoubleCurrency(fmt, number, currency, result, resultlength, pos, status)
}
#[inline]
pub unsafe fn unum_formatDoubleForFields(format: *const *const core::ffi::c_void, number: f64, result: *mut u16, resultlength: i32, fpositer: *mut UFieldPositionIterator, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn unum_formatDoubleForFields(format : *const *const core::ffi::c_void, number : f64, result : *mut u16, resultlength : i32, fpositer : *mut UFieldPositionIterator, status : *mut UErrorCode) -> i32);
    unum_formatDoubleForFields(format, number, result, resultlength, fpositer, status)
}
#[inline]
pub unsafe fn unum_formatInt64(fmt: *const *const core::ffi::c_void, number: i64, result: *mut u16, resultlength: i32, pos: *mut UFieldPosition, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn unum_formatInt64(fmt : *const *const core::ffi::c_void, number : i64, result : *mut u16, resultlength : i32, pos : *mut UFieldPosition, status : *mut UErrorCode) -> i32);
    unum_formatInt64(fmt, number, result, resultlength, pos, status)
}
#[inline]
pub unsafe fn unum_formatUFormattable(fmt: *const *const core::ffi::c_void, number: *const *const core::ffi::c_void, result: *mut u16, resultlength: i32, pos: *mut UFieldPosition, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn unum_formatUFormattable(fmt : *const *const core::ffi::c_void, number : *const *const core::ffi::c_void, result : *mut u16, resultlength : i32, pos : *mut UFieldPosition, status : *mut UErrorCode) -> i32);
    unum_formatUFormattable(fmt, number, result, resultlength, pos, status)
}
#[inline]
pub unsafe fn unum_getAttribute(fmt: *const *const core::ffi::c_void, attr: UNumberFormatAttribute) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn unum_getAttribute(fmt : *const *const core::ffi::c_void, attr : UNumberFormatAttribute) -> i32);
    unum_getAttribute(fmt, attr)
}
#[inline]
pub unsafe fn unum_getAvailable(localeindex: i32) -> windows_core::PCSTR {
    windows_targets::link!("icu.dll" "cdecl" fn unum_getAvailable(localeindex : i32) -> windows_core::PCSTR);
    unum_getAvailable(localeindex)
}
#[inline]
pub unsafe fn unum_getContext(fmt: *const *const core::ffi::c_void, r#type: UDisplayContextType, status: *mut UErrorCode) -> UDisplayContext {
    windows_targets::link!("icu.dll" "cdecl" fn unum_getContext(fmt : *const *const core::ffi::c_void, r#type : UDisplayContextType, status : *mut UErrorCode) -> UDisplayContext);
    unum_getContext(fmt, r#type, status)
}
#[inline]
pub unsafe fn unum_getDoubleAttribute(fmt: *const *const core::ffi::c_void, attr: UNumberFormatAttribute) -> f64 {
    windows_targets::link!("icu.dll" "cdecl" fn unum_getDoubleAttribute(fmt : *const *const core::ffi::c_void, attr : UNumberFormatAttribute) -> f64);
    unum_getDoubleAttribute(fmt, attr)
}
#[inline]
pub unsafe fn unum_getLocaleByType(fmt: *const *const core::ffi::c_void, r#type: ULocDataLocaleType, status: *mut UErrorCode) -> windows_core::PCSTR {
    windows_targets::link!("icu.dll" "cdecl" fn unum_getLocaleByType(fmt : *const *const core::ffi::c_void, r#type : ULocDataLocaleType, status : *mut UErrorCode) -> windows_core::PCSTR);
    unum_getLocaleByType(fmt, r#type, status)
}
#[inline]
pub unsafe fn unum_getSymbol(fmt: *const *const core::ffi::c_void, symbol: UNumberFormatSymbol, buffer: *mut u16, size: i32, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn unum_getSymbol(fmt : *const *const core::ffi::c_void, symbol : UNumberFormatSymbol, buffer : *mut u16, size : i32, status : *mut UErrorCode) -> i32);
    unum_getSymbol(fmt, symbol, buffer, size, status)
}
#[inline]
pub unsafe fn unum_getTextAttribute(fmt: *const *const core::ffi::c_void, tag: UNumberFormatTextAttribute, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn unum_getTextAttribute(fmt : *const *const core::ffi::c_void, tag : UNumberFormatTextAttribute, result : *mut u16, resultlength : i32, status : *mut UErrorCode) -> i32);
    unum_getTextAttribute(fmt, tag, result, resultlength, status)
}
#[inline]
pub unsafe fn unum_open<P0>(style: UNumberFormatStyle, pattern: *const u16, patternlength: i32, locale: P0, parseerr: *mut UParseError, status: *mut UErrorCode) -> *mut *mut core::ffi::c_void
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn unum_open(style : UNumberFormatStyle, pattern : *const u16, patternlength : i32, locale : windows_core::PCSTR, parseerr : *mut UParseError, status : *mut UErrorCode) -> *mut *mut core::ffi::c_void);
    unum_open(style, pattern, patternlength, locale.param().abi(), parseerr, status)
}
#[inline]
pub unsafe fn unum_parse(fmt: *const *const core::ffi::c_void, text: *const u16, textlength: i32, parsepos: *mut i32, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn unum_parse(fmt : *const *const core::ffi::c_void, text : *const u16, textlength : i32, parsepos : *mut i32, status : *mut UErrorCode) -> i32);
    unum_parse(fmt, text, textlength, parsepos, status)
}
#[inline]
pub unsafe fn unum_parseDecimal<P0>(fmt: *const *const core::ffi::c_void, text: *const u16, textlength: i32, parsepos: *mut i32, outbuf: P0, outbuflength: i32, status: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn unum_parseDecimal(fmt : *const *const core::ffi::c_void, text : *const u16, textlength : i32, parsepos : *mut i32, outbuf : windows_core::PCSTR, outbuflength : i32, status : *mut UErrorCode) -> i32);
    unum_parseDecimal(fmt, text, textlength, parsepos, outbuf.param().abi(), outbuflength, status)
}
#[inline]
pub unsafe fn unum_parseDouble(fmt: *const *const core::ffi::c_void, text: *const u16, textlength: i32, parsepos: *mut i32, status: *mut UErrorCode) -> f64 {
    windows_targets::link!("icu.dll" "cdecl" fn unum_parseDouble(fmt : *const *const core::ffi::c_void, text : *const u16, textlength : i32, parsepos : *mut i32, status : *mut UErrorCode) -> f64);
    unum_parseDouble(fmt, text, textlength, parsepos, status)
}
#[inline]
pub unsafe fn unum_parseDoubleCurrency(fmt: *const *const core::ffi::c_void, text: *const u16, textlength: i32, parsepos: *mut i32, currency: *mut u16, status: *mut UErrorCode) -> f64 {
    windows_targets::link!("icu.dll" "cdecl" fn unum_parseDoubleCurrency(fmt : *const *const core::ffi::c_void, text : *const u16, textlength : i32, parsepos : *mut i32, currency : *mut u16, status : *mut UErrorCode) -> f64);
    unum_parseDoubleCurrency(fmt, text, textlength, parsepos, currency, status)
}
#[inline]
pub unsafe fn unum_parseInt64(fmt: *const *const core::ffi::c_void, text: *const u16, textlength: i32, parsepos: *mut i32, status: *mut UErrorCode) -> i64 {
    windows_targets::link!("icu.dll" "cdecl" fn unum_parseInt64(fmt : *const *const core::ffi::c_void, text : *const u16, textlength : i32, parsepos : *mut i32, status : *mut UErrorCode) -> i64);
    unum_parseInt64(fmt, text, textlength, parsepos, status)
}
#[inline]
pub unsafe fn unum_parseToUFormattable(fmt: *const *const core::ffi::c_void, result: *mut *mut core::ffi::c_void, text: *const u16, textlength: i32, parsepos: *mut i32, status: *mut UErrorCode) -> *mut *mut core::ffi::c_void {
    windows_targets::link!("icu.dll" "cdecl" fn unum_parseToUFormattable(fmt : *const *const core::ffi::c_void, result : *mut *mut core::ffi::c_void, text : *const u16, textlength : i32, parsepos : *mut i32, status : *mut UErrorCode) -> *mut *mut core::ffi::c_void);
    unum_parseToUFormattable(fmt, result, text, textlength, parsepos, status)
}
#[inline]
pub unsafe fn unum_setAttribute(fmt: *mut *mut core::ffi::c_void, attr: UNumberFormatAttribute, newvalue: i32) {
    windows_targets::link!("icu.dll" "cdecl" fn unum_setAttribute(fmt : *mut *mut core::ffi::c_void, attr : UNumberFormatAttribute, newvalue : i32));
    unum_setAttribute(fmt, attr, newvalue)
}
#[inline]
pub unsafe fn unum_setContext(fmt: *mut *mut core::ffi::c_void, value: UDisplayContext, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn unum_setContext(fmt : *mut *mut core::ffi::c_void, value : UDisplayContext, status : *mut UErrorCode));
    unum_setContext(fmt, value, status)
}
#[inline]
pub unsafe fn unum_setDoubleAttribute(fmt: *mut *mut core::ffi::c_void, attr: UNumberFormatAttribute, newvalue: f64) {
    windows_targets::link!("icu.dll" "cdecl" fn unum_setDoubleAttribute(fmt : *mut *mut core::ffi::c_void, attr : UNumberFormatAttribute, newvalue : f64));
    unum_setDoubleAttribute(fmt, attr, newvalue)
}
#[inline]
pub unsafe fn unum_setSymbol(fmt: *mut *mut core::ffi::c_void, symbol: UNumberFormatSymbol, value: *const u16, length: i32, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn unum_setSymbol(fmt : *mut *mut core::ffi::c_void, symbol : UNumberFormatSymbol, value : *const u16, length : i32, status : *mut UErrorCode));
    unum_setSymbol(fmt, symbol, value, length, status)
}
#[inline]
pub unsafe fn unum_setTextAttribute(fmt: *mut *mut core::ffi::c_void, tag: UNumberFormatTextAttribute, newvalue: *const u16, newvaluelength: i32, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn unum_setTextAttribute(fmt : *mut *mut core::ffi::c_void, tag : UNumberFormatTextAttribute, newvalue : *const u16, newvaluelength : i32, status : *mut UErrorCode));
    unum_setTextAttribute(fmt, tag, newvalue, newvaluelength, status)
}
#[inline]
pub unsafe fn unum_toPattern(fmt: *const *const core::ffi::c_void, ispatternlocalized: i8, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn unum_toPattern(fmt : *const *const core::ffi::c_void, ispatternlocalized : i8, result : *mut u16, resultlength : i32, status : *mut UErrorCode) -> i32);
    unum_toPattern(fmt, ispatternlocalized, result, resultlength, status)
}
#[inline]
pub unsafe fn unumf_close(uformatter: *mut UNumberFormatter) {
    windows_targets::link!("icu.dll" "cdecl" fn unumf_close(uformatter : *mut UNumberFormatter));
    unumf_close(uformatter)
}
#[inline]
pub unsafe fn unumf_closeResult(uresult: *mut UFormattedNumber) {
    windows_targets::link!("icu.dll" "cdecl" fn unumf_closeResult(uresult : *mut UFormattedNumber));
    unumf_closeResult(uresult)
}
#[inline]
pub unsafe fn unumf_formatDecimal<P0>(uformatter: *const UNumberFormatter, value: P0, valuelen: i32, uresult: *mut UFormattedNumber, ec: *mut UErrorCode)
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn unumf_formatDecimal(uformatter : *const UNumberFormatter, value : windows_core::PCSTR, valuelen : i32, uresult : *mut UFormattedNumber, ec : *mut UErrorCode));
    unumf_formatDecimal(uformatter, value.param().abi(), valuelen, uresult, ec)
}
#[inline]
pub unsafe fn unumf_formatDouble(uformatter: *const UNumberFormatter, value: f64, uresult: *mut UFormattedNumber, ec: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn unumf_formatDouble(uformatter : *const UNumberFormatter, value : f64, uresult : *mut UFormattedNumber, ec : *mut UErrorCode));
    unumf_formatDouble(uformatter, value, uresult, ec)
}
#[inline]
pub unsafe fn unumf_formatInt(uformatter: *const UNumberFormatter, value: i64, uresult: *mut UFormattedNumber, ec: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn unumf_formatInt(uformatter : *const UNumberFormatter, value : i64, uresult : *mut UFormattedNumber, ec : *mut UErrorCode));
    unumf_formatInt(uformatter, value, uresult, ec)
}
#[inline]
pub unsafe fn unumf_openForSkeletonAndLocale<P0>(skeleton: *const u16, skeletonlen: i32, locale: P0, ec: *mut UErrorCode) -> *mut UNumberFormatter
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn unumf_openForSkeletonAndLocale(skeleton : *const u16, skeletonlen : i32, locale : windows_core::PCSTR, ec : *mut UErrorCode) -> *mut UNumberFormatter);
    unumf_openForSkeletonAndLocale(skeleton, skeletonlen, locale.param().abi(), ec)
}
#[inline]
pub unsafe fn unumf_openForSkeletonAndLocaleWithError<P0>(skeleton: *const u16, skeletonlen: i32, locale: P0, perror: *mut UParseError, ec: *mut UErrorCode) -> *mut UNumberFormatter
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn unumf_openForSkeletonAndLocaleWithError(skeleton : *const u16, skeletonlen : i32, locale : windows_core::PCSTR, perror : *mut UParseError, ec : *mut UErrorCode) -> *mut UNumberFormatter);
    unumf_openForSkeletonAndLocaleWithError(skeleton, skeletonlen, locale.param().abi(), perror, ec)
}
#[inline]
pub unsafe fn unumf_openResult(ec: *mut UErrorCode) -> *mut UFormattedNumber {
    windows_targets::link!("icu.dll" "cdecl" fn unumf_openResult(ec : *mut UErrorCode) -> *mut UFormattedNumber);
    unumf_openResult(ec)
}
#[inline]
pub unsafe fn unumf_resultAsValue(uresult: *const UFormattedNumber, ec: *mut UErrorCode) -> *mut UFormattedValue {
    windows_targets::link!("icu.dll" "cdecl" fn unumf_resultAsValue(uresult : *const UFormattedNumber, ec : *mut UErrorCode) -> *mut UFormattedValue);
    unumf_resultAsValue(uresult, ec)
}
#[inline]
pub unsafe fn unumf_resultGetAllFieldPositions(uresult: *const UFormattedNumber, ufpositer: *mut UFieldPositionIterator, ec: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn unumf_resultGetAllFieldPositions(uresult : *const UFormattedNumber, ufpositer : *mut UFieldPositionIterator, ec : *mut UErrorCode));
    unumf_resultGetAllFieldPositions(uresult, ufpositer, ec)
}
#[inline]
pub unsafe fn unumf_resultNextFieldPosition(uresult: *const UFormattedNumber, ufpos: *mut UFieldPosition, ec: *mut UErrorCode) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn unumf_resultNextFieldPosition(uresult : *const UFormattedNumber, ufpos : *mut UFieldPosition, ec : *mut UErrorCode) -> i8);
    unumf_resultNextFieldPosition(uresult, ufpos, ec)
}
#[inline]
pub unsafe fn unumf_resultToString(uresult: *const UFormattedNumber, buffer: *mut u16, buffercapacity: i32, ec: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn unumf_resultToString(uresult : *const UFormattedNumber, buffer : *mut u16, buffercapacity : i32, ec : *mut UErrorCode) -> i32);
    unumf_resultToString(uresult, buffer, buffercapacity, ec)
}
#[inline]
pub unsafe fn unumsys_close(unumsys: *mut UNumberingSystem) {
    windows_targets::link!("icu.dll" "cdecl" fn unumsys_close(unumsys : *mut UNumberingSystem));
    unumsys_close(unumsys)
}
#[inline]
pub unsafe fn unumsys_getDescription(unumsys: *const UNumberingSystem, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn unumsys_getDescription(unumsys : *const UNumberingSystem, result : *mut u16, resultlength : i32, status : *mut UErrorCode) -> i32);
    unumsys_getDescription(unumsys, result, resultlength, status)
}
#[inline]
pub unsafe fn unumsys_getName(unumsys: *const UNumberingSystem) -> windows_core::PCSTR {
    windows_targets::link!("icu.dll" "cdecl" fn unumsys_getName(unumsys : *const UNumberingSystem) -> windows_core::PCSTR);
    unumsys_getName(unumsys)
}
#[inline]
pub unsafe fn unumsys_getRadix(unumsys: *const UNumberingSystem) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn unumsys_getRadix(unumsys : *const UNumberingSystem) -> i32);
    unumsys_getRadix(unumsys)
}
#[inline]
pub unsafe fn unumsys_isAlgorithmic(unumsys: *const UNumberingSystem) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn unumsys_isAlgorithmic(unumsys : *const UNumberingSystem) -> i8);
    unumsys_isAlgorithmic(unumsys)
}
#[inline]
pub unsafe fn unumsys_open<P0>(locale: P0, status: *mut UErrorCode) -> *mut UNumberingSystem
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn unumsys_open(locale : windows_core::PCSTR, status : *mut UErrorCode) -> *mut UNumberingSystem);
    unumsys_open(locale.param().abi(), status)
}
#[inline]
pub unsafe fn unumsys_openAvailableNames(status: *mut UErrorCode) -> *mut UEnumeration {
    windows_targets::link!("icu.dll" "cdecl" fn unumsys_openAvailableNames(status : *mut UErrorCode) -> *mut UEnumeration);
    unumsys_openAvailableNames(status)
}
#[inline]
pub unsafe fn unumsys_openByName<P0>(name: P0, status: *mut UErrorCode) -> *mut UNumberingSystem
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn unumsys_openByName(name : windows_core::PCSTR, status : *mut UErrorCode) -> *mut UNumberingSystem);
    unumsys_openByName(name.param().abi(), status)
}
#[inline]
pub unsafe fn uplrules_close(uplrules: *mut UPluralRules) {
    windows_targets::link!("icu.dll" "cdecl" fn uplrules_close(uplrules : *mut UPluralRules));
    uplrules_close(uplrules)
}
#[inline]
pub unsafe fn uplrules_getKeywords(uplrules: *const UPluralRules, status: *mut UErrorCode) -> *mut UEnumeration {
    windows_targets::link!("icu.dll" "cdecl" fn uplrules_getKeywords(uplrules : *const UPluralRules, status : *mut UErrorCode) -> *mut UEnumeration);
    uplrules_getKeywords(uplrules, status)
}
#[inline]
pub unsafe fn uplrules_open<P0>(locale: P0, status: *mut UErrorCode) -> *mut UPluralRules
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uplrules_open(locale : windows_core::PCSTR, status : *mut UErrorCode) -> *mut UPluralRules);
    uplrules_open(locale.param().abi(), status)
}
#[inline]
pub unsafe fn uplrules_openForType<P0>(locale: P0, r#type: UPluralType, status: *mut UErrorCode) -> *mut UPluralRules
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uplrules_openForType(locale : windows_core::PCSTR, r#type : UPluralType, status : *mut UErrorCode) -> *mut UPluralRules);
    uplrules_openForType(locale.param().abi(), r#type, status)
}
#[inline]
pub unsafe fn uplrules_select(uplrules: *const UPluralRules, number: f64, keyword: *mut u16, capacity: i32, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn uplrules_select(uplrules : *const UPluralRules, number : f64, keyword : *mut u16, capacity : i32, status : *mut UErrorCode) -> i32);
    uplrules_select(uplrules, number, keyword, capacity, status)
}
#[inline]
pub unsafe fn uplrules_selectFormatted(uplrules: *const UPluralRules, number: *const UFormattedNumber, keyword: *mut u16, capacity: i32, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn uplrules_selectFormatted(uplrules : *const UPluralRules, number : *const UFormattedNumber, keyword : *mut u16, capacity : i32, status : *mut UErrorCode) -> i32);
    uplrules_selectFormatted(uplrules, number, keyword, capacity, status)
}
#[inline]
pub unsafe fn uregex_appendReplacement(regexp: *mut URegularExpression, replacementtext: *const u16, replacementlength: i32, destbuf: *mut *mut u16, destcapacity: *mut i32, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_appendReplacement(regexp : *mut URegularExpression, replacementtext : *const u16, replacementlength : i32, destbuf : *mut *mut u16, destcapacity : *mut i32, status : *mut UErrorCode) -> i32);
    uregex_appendReplacement(regexp, replacementtext, replacementlength, destbuf, destcapacity, status)
}
#[inline]
pub unsafe fn uregex_appendReplacementUText(regexp: *mut URegularExpression, replacementtext: *mut UText, dest: *mut UText, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_appendReplacementUText(regexp : *mut URegularExpression, replacementtext : *mut UText, dest : *mut UText, status : *mut UErrorCode));
    uregex_appendReplacementUText(regexp, replacementtext, dest, status)
}
#[inline]
pub unsafe fn uregex_appendTail(regexp: *mut URegularExpression, destbuf: *mut *mut u16, destcapacity: *mut i32, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_appendTail(regexp : *mut URegularExpression, destbuf : *mut *mut u16, destcapacity : *mut i32, status : *mut UErrorCode) -> i32);
    uregex_appendTail(regexp, destbuf, destcapacity, status)
}
#[inline]
pub unsafe fn uregex_appendTailUText(regexp: *mut URegularExpression, dest: *mut UText, status: *mut UErrorCode) -> *mut UText {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_appendTailUText(regexp : *mut URegularExpression, dest : *mut UText, status : *mut UErrorCode) -> *mut UText);
    uregex_appendTailUText(regexp, dest, status)
}
#[inline]
pub unsafe fn uregex_clone(regexp: *const URegularExpression, status: *mut UErrorCode) -> *mut URegularExpression {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_clone(regexp : *const URegularExpression, status : *mut UErrorCode) -> *mut URegularExpression);
    uregex_clone(regexp, status)
}
#[inline]
pub unsafe fn uregex_close(regexp: *mut URegularExpression) {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_close(regexp : *mut URegularExpression));
    uregex_close(regexp)
}
#[inline]
pub unsafe fn uregex_end(regexp: *mut URegularExpression, groupnum: i32, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_end(regexp : *mut URegularExpression, groupnum : i32, status : *mut UErrorCode) -> i32);
    uregex_end(regexp, groupnum, status)
}
#[inline]
pub unsafe fn uregex_end64(regexp: *mut URegularExpression, groupnum: i32, status: *mut UErrorCode) -> i64 {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_end64(regexp : *mut URegularExpression, groupnum : i32, status : *mut UErrorCode) -> i64);
    uregex_end64(regexp, groupnum, status)
}
#[inline]
pub unsafe fn uregex_find(regexp: *mut URegularExpression, startindex: i32, status: *mut UErrorCode) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_find(regexp : *mut URegularExpression, startindex : i32, status : *mut UErrorCode) -> i8);
    uregex_find(regexp, startindex, status)
}
#[inline]
pub unsafe fn uregex_find64(regexp: *mut URegularExpression, startindex: i64, status: *mut UErrorCode) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_find64(regexp : *mut URegularExpression, startindex : i64, status : *mut UErrorCode) -> i8);
    uregex_find64(regexp, startindex, status)
}
#[inline]
pub unsafe fn uregex_findNext(regexp: *mut URegularExpression, status: *mut UErrorCode) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_findNext(regexp : *mut URegularExpression, status : *mut UErrorCode) -> i8);
    uregex_findNext(regexp, status)
}
#[inline]
pub unsafe fn uregex_flags(regexp: *const URegularExpression, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_flags(regexp : *const URegularExpression, status : *mut UErrorCode) -> i32);
    uregex_flags(regexp, status)
}
#[inline]
pub unsafe fn uregex_getFindProgressCallback(regexp: *const URegularExpression, callback: *mut URegexFindProgressCallback, context: *const *const core::ffi::c_void, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_getFindProgressCallback(regexp : *const URegularExpression, callback : *mut URegexFindProgressCallback, context : *const *const core::ffi::c_void, status : *mut UErrorCode));
    uregex_getFindProgressCallback(regexp, callback, context, status)
}
#[inline]
pub unsafe fn uregex_getMatchCallback(regexp: *const URegularExpression, callback: *mut URegexMatchCallback, context: *const *const core::ffi::c_void, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_getMatchCallback(regexp : *const URegularExpression, callback : *mut URegexMatchCallback, context : *const *const core::ffi::c_void, status : *mut UErrorCode));
    uregex_getMatchCallback(regexp, callback, context, status)
}
#[inline]
pub unsafe fn uregex_getStackLimit(regexp: *const URegularExpression, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_getStackLimit(regexp : *const URegularExpression, status : *mut UErrorCode) -> i32);
    uregex_getStackLimit(regexp, status)
}
#[inline]
pub unsafe fn uregex_getText(regexp: *mut URegularExpression, textlength: *mut i32, status: *mut UErrorCode) -> *mut u16 {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_getText(regexp : *mut URegularExpression, textlength : *mut i32, status : *mut UErrorCode) -> *mut u16);
    uregex_getText(regexp, textlength, status)
}
#[inline]
pub unsafe fn uregex_getTimeLimit(regexp: *const URegularExpression, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_getTimeLimit(regexp : *const URegularExpression, status : *mut UErrorCode) -> i32);
    uregex_getTimeLimit(regexp, status)
}
#[inline]
pub unsafe fn uregex_getUText(regexp: *mut URegularExpression, dest: *mut UText, status: *mut UErrorCode) -> *mut UText {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_getUText(regexp : *mut URegularExpression, dest : *mut UText, status : *mut UErrorCode) -> *mut UText);
    uregex_getUText(regexp, dest, status)
}
#[inline]
pub unsafe fn uregex_group(regexp: *mut URegularExpression, groupnum: i32, dest: *mut u16, destcapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_group(regexp : *mut URegularExpression, groupnum : i32, dest : *mut u16, destcapacity : i32, status : *mut UErrorCode) -> i32);
    uregex_group(regexp, groupnum, dest, destcapacity, status)
}
#[inline]
pub unsafe fn uregex_groupCount(regexp: *mut URegularExpression, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_groupCount(regexp : *mut URegularExpression, status : *mut UErrorCode) -> i32);
    uregex_groupCount(regexp, status)
}
#[inline]
pub unsafe fn uregex_groupNumberFromCName<P0>(regexp: *mut URegularExpression, groupname: P0, namelength: i32, status: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uregex_groupNumberFromCName(regexp : *mut URegularExpression, groupname : windows_core::PCSTR, namelength : i32, status : *mut UErrorCode) -> i32);
    uregex_groupNumberFromCName(regexp, groupname.param().abi(), namelength, status)
}
#[inline]
pub unsafe fn uregex_groupNumberFromName(regexp: *mut URegularExpression, groupname: *const u16, namelength: i32, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_groupNumberFromName(regexp : *mut URegularExpression, groupname : *const u16, namelength : i32, status : *mut UErrorCode) -> i32);
    uregex_groupNumberFromName(regexp, groupname, namelength, status)
}
#[inline]
pub unsafe fn uregex_groupUText(regexp: *mut URegularExpression, groupnum: i32, dest: *mut UText, grouplength: *mut i64, status: *mut UErrorCode) -> *mut UText {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_groupUText(regexp : *mut URegularExpression, groupnum : i32, dest : *mut UText, grouplength : *mut i64, status : *mut UErrorCode) -> *mut UText);
    uregex_groupUText(regexp, groupnum, dest, grouplength, status)
}
#[inline]
pub unsafe fn uregex_hasAnchoringBounds(regexp: *const URegularExpression, status: *mut UErrorCode) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_hasAnchoringBounds(regexp : *const URegularExpression, status : *mut UErrorCode) -> i8);
    uregex_hasAnchoringBounds(regexp, status)
}
#[inline]
pub unsafe fn uregex_hasTransparentBounds(regexp: *const URegularExpression, status: *mut UErrorCode) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_hasTransparentBounds(regexp : *const URegularExpression, status : *mut UErrorCode) -> i8);
    uregex_hasTransparentBounds(regexp, status)
}
#[inline]
pub unsafe fn uregex_hitEnd(regexp: *const URegularExpression, status: *mut UErrorCode) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_hitEnd(regexp : *const URegularExpression, status : *mut UErrorCode) -> i8);
    uregex_hitEnd(regexp, status)
}
#[inline]
pub unsafe fn uregex_lookingAt(regexp: *mut URegularExpression, startindex: i32, status: *mut UErrorCode) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_lookingAt(regexp : *mut URegularExpression, startindex : i32, status : *mut UErrorCode) -> i8);
    uregex_lookingAt(regexp, startindex, status)
}
#[inline]
pub unsafe fn uregex_lookingAt64(regexp: *mut URegularExpression, startindex: i64, status: *mut UErrorCode) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_lookingAt64(regexp : *mut URegularExpression, startindex : i64, status : *mut UErrorCode) -> i8);
    uregex_lookingAt64(regexp, startindex, status)
}
#[inline]
pub unsafe fn uregex_matches(regexp: *mut URegularExpression, startindex: i32, status: *mut UErrorCode) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_matches(regexp : *mut URegularExpression, startindex : i32, status : *mut UErrorCode) -> i8);
    uregex_matches(regexp, startindex, status)
}
#[inline]
pub unsafe fn uregex_matches64(regexp: *mut URegularExpression, startindex: i64, status: *mut UErrorCode) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_matches64(regexp : *mut URegularExpression, startindex : i64, status : *mut UErrorCode) -> i8);
    uregex_matches64(regexp, startindex, status)
}
#[inline]
pub unsafe fn uregex_open(pattern: *const u16, patternlength: i32, flags: u32, pe: *mut UParseError, status: *mut UErrorCode) -> *mut URegularExpression {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_open(pattern : *const u16, patternlength : i32, flags : u32, pe : *mut UParseError, status : *mut UErrorCode) -> *mut URegularExpression);
    uregex_open(pattern, patternlength, flags, pe, status)
}
#[inline]
pub unsafe fn uregex_openC<P0>(pattern: P0, flags: u32, pe: *mut UParseError, status: *mut UErrorCode) -> *mut URegularExpression
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uregex_openC(pattern : windows_core::PCSTR, flags : u32, pe : *mut UParseError, status : *mut UErrorCode) -> *mut URegularExpression);
    uregex_openC(pattern.param().abi(), flags, pe, status)
}
#[inline]
pub unsafe fn uregex_openUText(pattern: *mut UText, flags: u32, pe: *mut UParseError, status: *mut UErrorCode) -> *mut URegularExpression {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_openUText(pattern : *mut UText, flags : u32, pe : *mut UParseError, status : *mut UErrorCode) -> *mut URegularExpression);
    uregex_openUText(pattern, flags, pe, status)
}
#[inline]
pub unsafe fn uregex_pattern(regexp: *const URegularExpression, patlength: *mut i32, status: *mut UErrorCode) -> *mut u16 {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_pattern(regexp : *const URegularExpression, patlength : *mut i32, status : *mut UErrorCode) -> *mut u16);
    uregex_pattern(regexp, patlength, status)
}
#[inline]
pub unsafe fn uregex_patternUText(regexp: *const URegularExpression, status: *mut UErrorCode) -> *mut UText {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_patternUText(regexp : *const URegularExpression, status : *mut UErrorCode) -> *mut UText);
    uregex_patternUText(regexp, status)
}
#[inline]
pub unsafe fn uregex_refreshUText(regexp: *mut URegularExpression, text: *mut UText, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_refreshUText(regexp : *mut URegularExpression, text : *mut UText, status : *mut UErrorCode));
    uregex_refreshUText(regexp, text, status)
}
#[inline]
pub unsafe fn uregex_regionEnd(regexp: *const URegularExpression, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_regionEnd(regexp : *const URegularExpression, status : *mut UErrorCode) -> i32);
    uregex_regionEnd(regexp, status)
}
#[inline]
pub unsafe fn uregex_regionEnd64(regexp: *const URegularExpression, status: *mut UErrorCode) -> i64 {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_regionEnd64(regexp : *const URegularExpression, status : *mut UErrorCode) -> i64);
    uregex_regionEnd64(regexp, status)
}
#[inline]
pub unsafe fn uregex_regionStart(regexp: *const URegularExpression, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_regionStart(regexp : *const URegularExpression, status : *mut UErrorCode) -> i32);
    uregex_regionStart(regexp, status)
}
#[inline]
pub unsafe fn uregex_regionStart64(regexp: *const URegularExpression, status: *mut UErrorCode) -> i64 {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_regionStart64(regexp : *const URegularExpression, status : *mut UErrorCode) -> i64);
    uregex_regionStart64(regexp, status)
}
#[inline]
pub unsafe fn uregex_replaceAll(regexp: *mut URegularExpression, replacementtext: *const u16, replacementlength: i32, destbuf: *mut u16, destcapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_replaceAll(regexp : *mut URegularExpression, replacementtext : *const u16, replacementlength : i32, destbuf : *mut u16, destcapacity : i32, status : *mut UErrorCode) -> i32);
    uregex_replaceAll(regexp, replacementtext, replacementlength, destbuf, destcapacity, status)
}
#[inline]
pub unsafe fn uregex_replaceAllUText(regexp: *mut URegularExpression, replacement: *mut UText, dest: *mut UText, status: *mut UErrorCode) -> *mut UText {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_replaceAllUText(regexp : *mut URegularExpression, replacement : *mut UText, dest : *mut UText, status : *mut UErrorCode) -> *mut UText);
    uregex_replaceAllUText(regexp, replacement, dest, status)
}
#[inline]
pub unsafe fn uregex_replaceFirst(regexp: *mut URegularExpression, replacementtext: *const u16, replacementlength: i32, destbuf: *mut u16, destcapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_replaceFirst(regexp : *mut URegularExpression, replacementtext : *const u16, replacementlength : i32, destbuf : *mut u16, destcapacity : i32, status : *mut UErrorCode) -> i32);
    uregex_replaceFirst(regexp, replacementtext, replacementlength, destbuf, destcapacity, status)
}
#[inline]
pub unsafe fn uregex_replaceFirstUText(regexp: *mut URegularExpression, replacement: *mut UText, dest: *mut UText, status: *mut UErrorCode) -> *mut UText {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_replaceFirstUText(regexp : *mut URegularExpression, replacement : *mut UText, dest : *mut UText, status : *mut UErrorCode) -> *mut UText);
    uregex_replaceFirstUText(regexp, replacement, dest, status)
}
#[inline]
pub unsafe fn uregex_requireEnd(regexp: *const URegularExpression, status: *mut UErrorCode) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_requireEnd(regexp : *const URegularExpression, status : *mut UErrorCode) -> i8);
    uregex_requireEnd(regexp, status)
}
#[inline]
pub unsafe fn uregex_reset(regexp: *mut URegularExpression, index: i32, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_reset(regexp : *mut URegularExpression, index : i32, status : *mut UErrorCode));
    uregex_reset(regexp, index, status)
}
#[inline]
pub unsafe fn uregex_reset64(regexp: *mut URegularExpression, index: i64, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_reset64(regexp : *mut URegularExpression, index : i64, status : *mut UErrorCode));
    uregex_reset64(regexp, index, status)
}
#[inline]
pub unsafe fn uregex_setFindProgressCallback(regexp: *mut URegularExpression, callback: URegexFindProgressCallback, context: *const core::ffi::c_void, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_setFindProgressCallback(regexp : *mut URegularExpression, callback : URegexFindProgressCallback, context : *const core::ffi::c_void, status : *mut UErrorCode));
    uregex_setFindProgressCallback(regexp, callback, context, status)
}
#[inline]
pub unsafe fn uregex_setMatchCallback(regexp: *mut URegularExpression, callback: URegexMatchCallback, context: *const core::ffi::c_void, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_setMatchCallback(regexp : *mut URegularExpression, callback : URegexMatchCallback, context : *const core::ffi::c_void, status : *mut UErrorCode));
    uregex_setMatchCallback(regexp, callback, context, status)
}
#[inline]
pub unsafe fn uregex_setRegion(regexp: *mut URegularExpression, regionstart: i32, regionlimit: i32, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_setRegion(regexp : *mut URegularExpression, regionstart : i32, regionlimit : i32, status : *mut UErrorCode));
    uregex_setRegion(regexp, regionstart, regionlimit, status)
}
#[inline]
pub unsafe fn uregex_setRegion64(regexp: *mut URegularExpression, regionstart: i64, regionlimit: i64, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_setRegion64(regexp : *mut URegularExpression, regionstart : i64, regionlimit : i64, status : *mut UErrorCode));
    uregex_setRegion64(regexp, regionstart, regionlimit, status)
}
#[inline]
pub unsafe fn uregex_setRegionAndStart(regexp: *mut URegularExpression, regionstart: i64, regionlimit: i64, startindex: i64, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_setRegionAndStart(regexp : *mut URegularExpression, regionstart : i64, regionlimit : i64, startindex : i64, status : *mut UErrorCode));
    uregex_setRegionAndStart(regexp, regionstart, regionlimit, startindex, status)
}
#[inline]
pub unsafe fn uregex_setStackLimit(regexp: *mut URegularExpression, limit: i32, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_setStackLimit(regexp : *mut URegularExpression, limit : i32, status : *mut UErrorCode));
    uregex_setStackLimit(regexp, limit, status)
}
#[inline]
pub unsafe fn uregex_setText(regexp: *mut URegularExpression, text: *const u16, textlength: i32, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_setText(regexp : *mut URegularExpression, text : *const u16, textlength : i32, status : *mut UErrorCode));
    uregex_setText(regexp, text, textlength, status)
}
#[inline]
pub unsafe fn uregex_setTimeLimit(regexp: *mut URegularExpression, limit: i32, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_setTimeLimit(regexp : *mut URegularExpression, limit : i32, status : *mut UErrorCode));
    uregex_setTimeLimit(regexp, limit, status)
}
#[inline]
pub unsafe fn uregex_setUText(regexp: *mut URegularExpression, text: *mut UText, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_setUText(regexp : *mut URegularExpression, text : *mut UText, status : *mut UErrorCode));
    uregex_setUText(regexp, text, status)
}
#[inline]
pub unsafe fn uregex_split(regexp: *mut URegularExpression, destbuf: *mut u16, destcapacity: i32, requiredcapacity: *mut i32, destfields: *mut *mut u16, destfieldscapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_split(regexp : *mut URegularExpression, destbuf : *mut u16, destcapacity : i32, requiredcapacity : *mut i32, destfields : *mut *mut u16, destfieldscapacity : i32, status : *mut UErrorCode) -> i32);
    uregex_split(regexp, destbuf, destcapacity, requiredcapacity, destfields, destfieldscapacity, status)
}
#[inline]
pub unsafe fn uregex_splitUText(regexp: *mut URegularExpression, destfields: *mut *mut UText, destfieldscapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_splitUText(regexp : *mut URegularExpression, destfields : *mut *mut UText, destfieldscapacity : i32, status : *mut UErrorCode) -> i32);
    uregex_splitUText(regexp, destfields, destfieldscapacity, status)
}
#[inline]
pub unsafe fn uregex_start(regexp: *mut URegularExpression, groupnum: i32, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_start(regexp : *mut URegularExpression, groupnum : i32, status : *mut UErrorCode) -> i32);
    uregex_start(regexp, groupnum, status)
}
#[inline]
pub unsafe fn uregex_start64(regexp: *mut URegularExpression, groupnum: i32, status: *mut UErrorCode) -> i64 {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_start64(regexp : *mut URegularExpression, groupnum : i32, status : *mut UErrorCode) -> i64);
    uregex_start64(regexp, groupnum, status)
}
#[inline]
pub unsafe fn uregex_useAnchoringBounds(regexp: *mut URegularExpression, b: i8, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_useAnchoringBounds(regexp : *mut URegularExpression, b : i8, status : *mut UErrorCode));
    uregex_useAnchoringBounds(regexp, b, status)
}
#[inline]
pub unsafe fn uregex_useTransparentBounds(regexp: *mut URegularExpression, b: i8, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn uregex_useTransparentBounds(regexp : *mut URegularExpression, b : i8, status : *mut UErrorCode));
    uregex_useTransparentBounds(regexp, b, status)
}
#[inline]
pub unsafe fn uregion_areEqual(uregion: *const URegion, otherregion: *const URegion) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn uregion_areEqual(uregion : *const URegion, otherregion : *const URegion) -> i8);
    uregion_areEqual(uregion, otherregion)
}
#[inline]
pub unsafe fn uregion_contains(uregion: *const URegion, otherregion: *const URegion) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn uregion_contains(uregion : *const URegion, otherregion : *const URegion) -> i8);
    uregion_contains(uregion, otherregion)
}
#[inline]
pub unsafe fn uregion_getAvailable(r#type: URegionType, status: *mut UErrorCode) -> *mut UEnumeration {
    windows_targets::link!("icu.dll" "cdecl" fn uregion_getAvailable(r#type : URegionType, status : *mut UErrorCode) -> *mut UEnumeration);
    uregion_getAvailable(r#type, status)
}
#[inline]
pub unsafe fn uregion_getContainedRegions(uregion: *const URegion, status: *mut UErrorCode) -> *mut UEnumeration {
    windows_targets::link!("icu.dll" "cdecl" fn uregion_getContainedRegions(uregion : *const URegion, status : *mut UErrorCode) -> *mut UEnumeration);
    uregion_getContainedRegions(uregion, status)
}
#[inline]
pub unsafe fn uregion_getContainedRegionsOfType(uregion: *const URegion, r#type: URegionType, status: *mut UErrorCode) -> *mut UEnumeration {
    windows_targets::link!("icu.dll" "cdecl" fn uregion_getContainedRegionsOfType(uregion : *const URegion, r#type : URegionType, status : *mut UErrorCode) -> *mut UEnumeration);
    uregion_getContainedRegionsOfType(uregion, r#type, status)
}
#[inline]
pub unsafe fn uregion_getContainingRegion(uregion: *const URegion) -> *mut URegion {
    windows_targets::link!("icu.dll" "cdecl" fn uregion_getContainingRegion(uregion : *const URegion) -> *mut URegion);
    uregion_getContainingRegion(uregion)
}
#[inline]
pub unsafe fn uregion_getContainingRegionOfType(uregion: *const URegion, r#type: URegionType) -> *mut URegion {
    windows_targets::link!("icu.dll" "cdecl" fn uregion_getContainingRegionOfType(uregion : *const URegion, r#type : URegionType) -> *mut URegion);
    uregion_getContainingRegionOfType(uregion, r#type)
}
#[inline]
pub unsafe fn uregion_getNumericCode(uregion: *const URegion) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn uregion_getNumericCode(uregion : *const URegion) -> i32);
    uregion_getNumericCode(uregion)
}
#[inline]
pub unsafe fn uregion_getPreferredValues(uregion: *const URegion, status: *mut UErrorCode) -> *mut UEnumeration {
    windows_targets::link!("icu.dll" "cdecl" fn uregion_getPreferredValues(uregion : *const URegion, status : *mut UErrorCode) -> *mut UEnumeration);
    uregion_getPreferredValues(uregion, status)
}
#[inline]
pub unsafe fn uregion_getRegionCode(uregion: *const URegion) -> windows_core::PCSTR {
    windows_targets::link!("icu.dll" "cdecl" fn uregion_getRegionCode(uregion : *const URegion) -> windows_core::PCSTR);
    uregion_getRegionCode(uregion)
}
#[inline]
pub unsafe fn uregion_getRegionFromCode<P0>(regioncode: P0, status: *mut UErrorCode) -> *mut URegion
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uregion_getRegionFromCode(regioncode : windows_core::PCSTR, status : *mut UErrorCode) -> *mut URegion);
    uregion_getRegionFromCode(regioncode.param().abi(), status)
}
#[inline]
pub unsafe fn uregion_getRegionFromNumericCode(code: i32, status: *mut UErrorCode) -> *mut URegion {
    windows_targets::link!("icu.dll" "cdecl" fn uregion_getRegionFromNumericCode(code : i32, status : *mut UErrorCode) -> *mut URegion);
    uregion_getRegionFromNumericCode(code, status)
}
#[inline]
pub unsafe fn uregion_getType(uregion: *const URegion) -> URegionType {
    windows_targets::link!("icu.dll" "cdecl" fn uregion_getType(uregion : *const URegion) -> URegionType);
    uregion_getType(uregion)
}
#[inline]
pub unsafe fn ureldatefmt_close(reldatefmt: *mut URelativeDateTimeFormatter) {
    windows_targets::link!("icu.dll" "cdecl" fn ureldatefmt_close(reldatefmt : *mut URelativeDateTimeFormatter));
    ureldatefmt_close(reldatefmt)
}
#[inline]
pub unsafe fn ureldatefmt_closeResult(ufrdt: *mut UFormattedRelativeDateTime) {
    windows_targets::link!("icu.dll" "cdecl" fn ureldatefmt_closeResult(ufrdt : *mut UFormattedRelativeDateTime));
    ureldatefmt_closeResult(ufrdt)
}
#[inline]
pub unsafe fn ureldatefmt_combineDateAndTime(reldatefmt: *const URelativeDateTimeFormatter, relativedatestring: *const u16, relativedatestringlen: i32, timestring: *const u16, timestringlen: i32, result: *mut u16, resultcapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ureldatefmt_combineDateAndTime(reldatefmt : *const URelativeDateTimeFormatter, relativedatestring : *const u16, relativedatestringlen : i32, timestring : *const u16, timestringlen : i32, result : *mut u16, resultcapacity : i32, status : *mut UErrorCode) -> i32);
    ureldatefmt_combineDateAndTime(reldatefmt, relativedatestring, relativedatestringlen, timestring, timestringlen, result, resultcapacity, status)
}
#[inline]
pub unsafe fn ureldatefmt_format(reldatefmt: *const URelativeDateTimeFormatter, offset: f64, unit: URelativeDateTimeUnit, result: *mut u16, resultcapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ureldatefmt_format(reldatefmt : *const URelativeDateTimeFormatter, offset : f64, unit : URelativeDateTimeUnit, result : *mut u16, resultcapacity : i32, status : *mut UErrorCode) -> i32);
    ureldatefmt_format(reldatefmt, offset, unit, result, resultcapacity, status)
}
#[inline]
pub unsafe fn ureldatefmt_formatNumeric(reldatefmt: *const URelativeDateTimeFormatter, offset: f64, unit: URelativeDateTimeUnit, result: *mut u16, resultcapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ureldatefmt_formatNumeric(reldatefmt : *const URelativeDateTimeFormatter, offset : f64, unit : URelativeDateTimeUnit, result : *mut u16, resultcapacity : i32, status : *mut UErrorCode) -> i32);
    ureldatefmt_formatNumeric(reldatefmt, offset, unit, result, resultcapacity, status)
}
#[inline]
pub unsafe fn ureldatefmt_formatNumericToResult(reldatefmt: *const URelativeDateTimeFormatter, offset: f64, unit: URelativeDateTimeUnit, result: *mut UFormattedRelativeDateTime, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn ureldatefmt_formatNumericToResult(reldatefmt : *const URelativeDateTimeFormatter, offset : f64, unit : URelativeDateTimeUnit, result : *mut UFormattedRelativeDateTime, status : *mut UErrorCode));
    ureldatefmt_formatNumericToResult(reldatefmt, offset, unit, result, status)
}
#[inline]
pub unsafe fn ureldatefmt_formatToResult(reldatefmt: *const URelativeDateTimeFormatter, offset: f64, unit: URelativeDateTimeUnit, result: *mut UFormattedRelativeDateTime, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn ureldatefmt_formatToResult(reldatefmt : *const URelativeDateTimeFormatter, offset : f64, unit : URelativeDateTimeUnit, result : *mut UFormattedRelativeDateTime, status : *mut UErrorCode));
    ureldatefmt_formatToResult(reldatefmt, offset, unit, result, status)
}
#[inline]
pub unsafe fn ureldatefmt_open<P0>(locale: P0, nftoadopt: *mut *mut core::ffi::c_void, width: UDateRelativeDateTimeFormatterStyle, capitalizationcontext: UDisplayContext, status: *mut UErrorCode) -> *mut URelativeDateTimeFormatter
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ureldatefmt_open(locale : windows_core::PCSTR, nftoadopt : *mut *mut core::ffi::c_void, width : UDateRelativeDateTimeFormatterStyle, capitalizationcontext : UDisplayContext, status : *mut UErrorCode) -> *mut URelativeDateTimeFormatter);
    ureldatefmt_open(locale.param().abi(), nftoadopt, width, capitalizationcontext, status)
}
#[inline]
pub unsafe fn ureldatefmt_openResult(ec: *mut UErrorCode) -> *mut UFormattedRelativeDateTime {
    windows_targets::link!("icu.dll" "cdecl" fn ureldatefmt_openResult(ec : *mut UErrorCode) -> *mut UFormattedRelativeDateTime);
    ureldatefmt_openResult(ec)
}
#[inline]
pub unsafe fn ureldatefmt_resultAsValue(ufrdt: *const UFormattedRelativeDateTime, ec: *mut UErrorCode) -> *mut UFormattedValue {
    windows_targets::link!("icu.dll" "cdecl" fn ureldatefmt_resultAsValue(ufrdt : *const UFormattedRelativeDateTime, ec : *mut UErrorCode) -> *mut UFormattedValue);
    ureldatefmt_resultAsValue(ufrdt, ec)
}
#[inline]
pub unsafe fn ures_close(resourcebundle: *mut UResourceBundle) {
    windows_targets::link!("icu.dll" "cdecl" fn ures_close(resourcebundle : *mut UResourceBundle));
    ures_close(resourcebundle)
}
#[inline]
pub unsafe fn ures_getBinary(resourcebundle: *const UResourceBundle, len: *mut i32, status: *mut UErrorCode) -> *mut u8 {
    windows_targets::link!("icu.dll" "cdecl" fn ures_getBinary(resourcebundle : *const UResourceBundle, len : *mut i32, status : *mut UErrorCode) -> *mut u8);
    ures_getBinary(resourcebundle, len, status)
}
#[inline]
pub unsafe fn ures_getByIndex(resourcebundle: *const UResourceBundle, indexr: i32, fillin: *mut UResourceBundle, status: *mut UErrorCode) -> *mut UResourceBundle {
    windows_targets::link!("icu.dll" "cdecl" fn ures_getByIndex(resourcebundle : *const UResourceBundle, indexr : i32, fillin : *mut UResourceBundle, status : *mut UErrorCode) -> *mut UResourceBundle);
    ures_getByIndex(resourcebundle, indexr, fillin, status)
}
#[inline]
pub unsafe fn ures_getByKey<P0>(resourcebundle: *const UResourceBundle, key: P0, fillin: *mut UResourceBundle, status: *mut UErrorCode) -> *mut UResourceBundle
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ures_getByKey(resourcebundle : *const UResourceBundle, key : windows_core::PCSTR, fillin : *mut UResourceBundle, status : *mut UErrorCode) -> *mut UResourceBundle);
    ures_getByKey(resourcebundle, key.param().abi(), fillin, status)
}
#[inline]
pub unsafe fn ures_getInt(resourcebundle: *const UResourceBundle, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ures_getInt(resourcebundle : *const UResourceBundle, status : *mut UErrorCode) -> i32);
    ures_getInt(resourcebundle, status)
}
#[inline]
pub unsafe fn ures_getIntVector(resourcebundle: *const UResourceBundle, len: *mut i32, status: *mut UErrorCode) -> *mut i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ures_getIntVector(resourcebundle : *const UResourceBundle, len : *mut i32, status : *mut UErrorCode) -> *mut i32);
    ures_getIntVector(resourcebundle, len, status)
}
#[inline]
pub unsafe fn ures_getKey(resourcebundle: *const UResourceBundle) -> windows_core::PCSTR {
    windows_targets::link!("icu.dll" "cdecl" fn ures_getKey(resourcebundle : *const UResourceBundle) -> windows_core::PCSTR);
    ures_getKey(resourcebundle)
}
#[inline]
pub unsafe fn ures_getLocaleByType(resourcebundle: *const UResourceBundle, r#type: ULocDataLocaleType, status: *mut UErrorCode) -> windows_core::PCSTR {
    windows_targets::link!("icu.dll" "cdecl" fn ures_getLocaleByType(resourcebundle : *const UResourceBundle, r#type : ULocDataLocaleType, status : *mut UErrorCode) -> windows_core::PCSTR);
    ures_getLocaleByType(resourcebundle, r#type, status)
}
#[inline]
pub unsafe fn ures_getNextResource(resourcebundle: *mut UResourceBundle, fillin: *mut UResourceBundle, status: *mut UErrorCode) -> *mut UResourceBundle {
    windows_targets::link!("icu.dll" "cdecl" fn ures_getNextResource(resourcebundle : *mut UResourceBundle, fillin : *mut UResourceBundle, status : *mut UErrorCode) -> *mut UResourceBundle);
    ures_getNextResource(resourcebundle, fillin, status)
}
#[inline]
pub unsafe fn ures_getNextString(resourcebundle: *mut UResourceBundle, len: *mut i32, key: *const *const i8, status: *mut UErrorCode) -> *mut u16 {
    windows_targets::link!("icu.dll" "cdecl" fn ures_getNextString(resourcebundle : *mut UResourceBundle, len : *mut i32, key : *const *const i8, status : *mut UErrorCode) -> *mut u16);
    ures_getNextString(resourcebundle, len, key, status)
}
#[inline]
pub unsafe fn ures_getSize(resourcebundle: *const UResourceBundle) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn ures_getSize(resourcebundle : *const UResourceBundle) -> i32);
    ures_getSize(resourcebundle)
}
#[inline]
pub unsafe fn ures_getString(resourcebundle: *const UResourceBundle, len: *mut i32, status: *mut UErrorCode) -> *mut u16 {
    windows_targets::link!("icu.dll" "cdecl" fn ures_getString(resourcebundle : *const UResourceBundle, len : *mut i32, status : *mut UErrorCode) -> *mut u16);
    ures_getString(resourcebundle, len, status)
}
#[inline]
pub unsafe fn ures_getStringByIndex(resourcebundle: *const UResourceBundle, indexs: i32, len: *mut i32, status: *mut UErrorCode) -> *mut u16 {
    windows_targets::link!("icu.dll" "cdecl" fn ures_getStringByIndex(resourcebundle : *const UResourceBundle, indexs : i32, len : *mut i32, status : *mut UErrorCode) -> *mut u16);
    ures_getStringByIndex(resourcebundle, indexs, len, status)
}
#[inline]
pub unsafe fn ures_getStringByKey<P0>(resb: *const UResourceBundle, key: P0, len: *mut i32, status: *mut UErrorCode) -> *mut u16
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ures_getStringByKey(resb : *const UResourceBundle, key : windows_core::PCSTR, len : *mut i32, status : *mut UErrorCode) -> *mut u16);
    ures_getStringByKey(resb, key.param().abi(), len, status)
}
#[inline]
pub unsafe fn ures_getType(resourcebundle: *const UResourceBundle) -> UResType {
    windows_targets::link!("icu.dll" "cdecl" fn ures_getType(resourcebundle : *const UResourceBundle) -> UResType);
    ures_getType(resourcebundle)
}
#[inline]
pub unsafe fn ures_getUInt(resourcebundle: *const UResourceBundle, status: *mut UErrorCode) -> u32 {
    windows_targets::link!("icu.dll" "cdecl" fn ures_getUInt(resourcebundle : *const UResourceBundle, status : *mut UErrorCode) -> u32);
    ures_getUInt(resourcebundle, status)
}
#[inline]
pub unsafe fn ures_getUTF8String<P0>(resb: *const UResourceBundle, dest: P0, length: *mut i32, forcecopy: i8, status: *mut UErrorCode) -> windows_core::PCSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ures_getUTF8String(resb : *const UResourceBundle, dest : windows_core::PCSTR, length : *mut i32, forcecopy : i8, status : *mut UErrorCode) -> windows_core::PCSTR);
    ures_getUTF8String(resb, dest.param().abi(), length, forcecopy, status)
}
#[inline]
pub unsafe fn ures_getUTF8StringByIndex<P0>(resb: *const UResourceBundle, stringindex: i32, dest: P0, plength: *mut i32, forcecopy: i8, status: *mut UErrorCode) -> windows_core::PCSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ures_getUTF8StringByIndex(resb : *const UResourceBundle, stringindex : i32, dest : windows_core::PCSTR, plength : *mut i32, forcecopy : i8, status : *mut UErrorCode) -> windows_core::PCSTR);
    ures_getUTF8StringByIndex(resb, stringindex, dest.param().abi(), plength, forcecopy, status)
}
#[inline]
pub unsafe fn ures_getUTF8StringByKey<P0, P1>(resb: *const UResourceBundle, key: P0, dest: P1, plength: *mut i32, forcecopy: i8, status: *mut UErrorCode) -> windows_core::PCSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ures_getUTF8StringByKey(resb : *const UResourceBundle, key : windows_core::PCSTR, dest : windows_core::PCSTR, plength : *mut i32, forcecopy : i8, status : *mut UErrorCode) -> windows_core::PCSTR);
    ures_getUTF8StringByKey(resb, key.param().abi(), dest.param().abi(), plength, forcecopy, status)
}
#[inline]
pub unsafe fn ures_getVersion(resb: *const UResourceBundle, versioninfo: *mut u8) {
    windows_targets::link!("icu.dll" "cdecl" fn ures_getVersion(resb : *const UResourceBundle, versioninfo : *mut u8));
    ures_getVersion(resb, versioninfo)
}
#[inline]
pub unsafe fn ures_hasNext(resourcebundle: *const UResourceBundle) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn ures_hasNext(resourcebundle : *const UResourceBundle) -> i8);
    ures_hasNext(resourcebundle)
}
#[inline]
pub unsafe fn ures_open<P0, P1>(packagename: P0, locale: P1, status: *mut UErrorCode) -> *mut UResourceBundle
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ures_open(packagename : windows_core::PCSTR, locale : windows_core::PCSTR, status : *mut UErrorCode) -> *mut UResourceBundle);
    ures_open(packagename.param().abi(), locale.param().abi(), status)
}
#[inline]
pub unsafe fn ures_openAvailableLocales<P0>(packagename: P0, status: *mut UErrorCode) -> *mut UEnumeration
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ures_openAvailableLocales(packagename : windows_core::PCSTR, status : *mut UErrorCode) -> *mut UEnumeration);
    ures_openAvailableLocales(packagename.param().abi(), status)
}
#[inline]
pub unsafe fn ures_openDirect<P0, P1>(packagename: P0, locale: P1, status: *mut UErrorCode) -> *mut UResourceBundle
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ures_openDirect(packagename : windows_core::PCSTR, locale : windows_core::PCSTR, status : *mut UErrorCode) -> *mut UResourceBundle);
    ures_openDirect(packagename.param().abi(), locale.param().abi(), status)
}
#[inline]
pub unsafe fn ures_openU<P0>(packagename: *const u16, locale: P0, status: *mut UErrorCode) -> *mut UResourceBundle
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn ures_openU(packagename : *const u16, locale : windows_core::PCSTR, status : *mut UErrorCode) -> *mut UResourceBundle);
    ures_openU(packagename, locale.param().abi(), status)
}
#[inline]
pub unsafe fn ures_resetIterator(resourcebundle: *mut UResourceBundle) {
    windows_targets::link!("icu.dll" "cdecl" fn ures_resetIterator(resourcebundle : *mut UResourceBundle));
    ures_resetIterator(resourcebundle)
}
#[inline]
pub unsafe fn uscript_breaksBetweenLetters(script: UScriptCode) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn uscript_breaksBetweenLetters(script : UScriptCode) -> i8);
    uscript_breaksBetweenLetters(script)
}
#[inline]
pub unsafe fn uscript_getCode<P0>(nameorabbrorlocale: P0, fillin: *mut UScriptCode, capacity: i32, err: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uscript_getCode(nameorabbrorlocale : windows_core::PCSTR, fillin : *mut UScriptCode, capacity : i32, err : *mut UErrorCode) -> i32);
    uscript_getCode(nameorabbrorlocale.param().abi(), fillin, capacity, err)
}
#[inline]
pub unsafe fn uscript_getName(scriptcode: UScriptCode) -> windows_core::PCSTR {
    windows_targets::link!("icu.dll" "cdecl" fn uscript_getName(scriptcode : UScriptCode) -> windows_core::PCSTR);
    uscript_getName(scriptcode)
}
#[inline]
pub unsafe fn uscript_getSampleString(script: UScriptCode, dest: *mut u16, capacity: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn uscript_getSampleString(script : UScriptCode, dest : *mut u16, capacity : i32, perrorcode : *mut UErrorCode) -> i32);
    uscript_getSampleString(script, dest, capacity, perrorcode)
}
#[inline]
pub unsafe fn uscript_getScript(codepoint: i32, err: *mut UErrorCode) -> UScriptCode {
    windows_targets::link!("icu.dll" "cdecl" fn uscript_getScript(codepoint : i32, err : *mut UErrorCode) -> UScriptCode);
    uscript_getScript(codepoint, err)
}
#[inline]
pub unsafe fn uscript_getScriptExtensions(c: i32, scripts: *mut UScriptCode, capacity: i32, errorcode: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn uscript_getScriptExtensions(c : i32, scripts : *mut UScriptCode, capacity : i32, errorcode : *mut UErrorCode) -> i32);
    uscript_getScriptExtensions(c, scripts, capacity, errorcode)
}
#[inline]
pub unsafe fn uscript_getShortName(scriptcode: UScriptCode) -> windows_core::PCSTR {
    windows_targets::link!("icu.dll" "cdecl" fn uscript_getShortName(scriptcode : UScriptCode) -> windows_core::PCSTR);
    uscript_getShortName(scriptcode)
}
#[inline]
pub unsafe fn uscript_getUsage(script: UScriptCode) -> UScriptUsage {
    windows_targets::link!("icu.dll" "cdecl" fn uscript_getUsage(script : UScriptCode) -> UScriptUsage);
    uscript_getUsage(script)
}
#[inline]
pub unsafe fn uscript_hasScript(c: i32, sc: UScriptCode) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn uscript_hasScript(c : i32, sc : UScriptCode) -> i8);
    uscript_hasScript(c, sc)
}
#[inline]
pub unsafe fn uscript_isCased(script: UScriptCode) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn uscript_isCased(script : UScriptCode) -> i8);
    uscript_isCased(script)
}
#[inline]
pub unsafe fn uscript_isRightToLeft(script: UScriptCode) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn uscript_isRightToLeft(script : UScriptCode) -> i8);
    uscript_isRightToLeft(script)
}
#[inline]
pub unsafe fn usearch_close(searchiter: *mut UStringSearch) {
    windows_targets::link!("icu.dll" "cdecl" fn usearch_close(searchiter : *mut UStringSearch));
    usearch_close(searchiter)
}
#[inline]
pub unsafe fn usearch_first(strsrch: *mut UStringSearch, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn usearch_first(strsrch : *mut UStringSearch, status : *mut UErrorCode) -> i32);
    usearch_first(strsrch, status)
}
#[inline]
pub unsafe fn usearch_following(strsrch: *mut UStringSearch, position: i32, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn usearch_following(strsrch : *mut UStringSearch, position : i32, status : *mut UErrorCode) -> i32);
    usearch_following(strsrch, position, status)
}
#[inline]
pub unsafe fn usearch_getAttribute(strsrch: *const UStringSearch, attribute: USearchAttribute) -> USearchAttributeValue {
    windows_targets::link!("icu.dll" "cdecl" fn usearch_getAttribute(strsrch : *const UStringSearch, attribute : USearchAttribute) -> USearchAttributeValue);
    usearch_getAttribute(strsrch, attribute)
}
#[inline]
pub unsafe fn usearch_getBreakIterator(strsrch: *const UStringSearch) -> *mut UBreakIterator {
    windows_targets::link!("icu.dll" "cdecl" fn usearch_getBreakIterator(strsrch : *const UStringSearch) -> *mut UBreakIterator);
    usearch_getBreakIterator(strsrch)
}
#[inline]
pub unsafe fn usearch_getCollator(strsrch: *const UStringSearch) -> *mut UCollator {
    windows_targets::link!("icu.dll" "cdecl" fn usearch_getCollator(strsrch : *const UStringSearch) -> *mut UCollator);
    usearch_getCollator(strsrch)
}
#[inline]
pub unsafe fn usearch_getMatchedLength(strsrch: *const UStringSearch) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn usearch_getMatchedLength(strsrch : *const UStringSearch) -> i32);
    usearch_getMatchedLength(strsrch)
}
#[inline]
pub unsafe fn usearch_getMatchedStart(strsrch: *const UStringSearch) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn usearch_getMatchedStart(strsrch : *const UStringSearch) -> i32);
    usearch_getMatchedStart(strsrch)
}
#[inline]
pub unsafe fn usearch_getMatchedText(strsrch: *const UStringSearch, result: *mut u16, resultcapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn usearch_getMatchedText(strsrch : *const UStringSearch, result : *mut u16, resultcapacity : i32, status : *mut UErrorCode) -> i32);
    usearch_getMatchedText(strsrch, result, resultcapacity, status)
}
#[inline]
pub unsafe fn usearch_getOffset(strsrch: *const UStringSearch) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn usearch_getOffset(strsrch : *const UStringSearch) -> i32);
    usearch_getOffset(strsrch)
}
#[inline]
pub unsafe fn usearch_getPattern(strsrch: *const UStringSearch, length: *mut i32) -> *mut u16 {
    windows_targets::link!("icu.dll" "cdecl" fn usearch_getPattern(strsrch : *const UStringSearch, length : *mut i32) -> *mut u16);
    usearch_getPattern(strsrch, length)
}
#[inline]
pub unsafe fn usearch_getText(strsrch: *const UStringSearch, length: *mut i32) -> *mut u16 {
    windows_targets::link!("icu.dll" "cdecl" fn usearch_getText(strsrch : *const UStringSearch, length : *mut i32) -> *mut u16);
    usearch_getText(strsrch, length)
}
#[inline]
pub unsafe fn usearch_last(strsrch: *mut UStringSearch, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn usearch_last(strsrch : *mut UStringSearch, status : *mut UErrorCode) -> i32);
    usearch_last(strsrch, status)
}
#[inline]
pub unsafe fn usearch_next(strsrch: *mut UStringSearch, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn usearch_next(strsrch : *mut UStringSearch, status : *mut UErrorCode) -> i32);
    usearch_next(strsrch, status)
}
#[inline]
pub unsafe fn usearch_open<P0>(pattern: *const u16, patternlength: i32, text: *const u16, textlength: i32, locale: P0, breakiter: *mut UBreakIterator, status: *mut UErrorCode) -> *mut UStringSearch
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn usearch_open(pattern : *const u16, patternlength : i32, text : *const u16, textlength : i32, locale : windows_core::PCSTR, breakiter : *mut UBreakIterator, status : *mut UErrorCode) -> *mut UStringSearch);
    usearch_open(pattern, patternlength, text, textlength, locale.param().abi(), breakiter, status)
}
#[inline]
pub unsafe fn usearch_openFromCollator(pattern: *const u16, patternlength: i32, text: *const u16, textlength: i32, collator: *const UCollator, breakiter: *mut UBreakIterator, status: *mut UErrorCode) -> *mut UStringSearch {
    windows_targets::link!("icu.dll" "cdecl" fn usearch_openFromCollator(pattern : *const u16, patternlength : i32, text : *const u16, textlength : i32, collator : *const UCollator, breakiter : *mut UBreakIterator, status : *mut UErrorCode) -> *mut UStringSearch);
    usearch_openFromCollator(pattern, patternlength, text, textlength, collator, breakiter, status)
}
#[inline]
pub unsafe fn usearch_preceding(strsrch: *mut UStringSearch, position: i32, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn usearch_preceding(strsrch : *mut UStringSearch, position : i32, status : *mut UErrorCode) -> i32);
    usearch_preceding(strsrch, position, status)
}
#[inline]
pub unsafe fn usearch_previous(strsrch: *mut UStringSearch, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn usearch_previous(strsrch : *mut UStringSearch, status : *mut UErrorCode) -> i32);
    usearch_previous(strsrch, status)
}
#[inline]
pub unsafe fn usearch_reset(strsrch: *mut UStringSearch) {
    windows_targets::link!("icu.dll" "cdecl" fn usearch_reset(strsrch : *mut UStringSearch));
    usearch_reset(strsrch)
}
#[inline]
pub unsafe fn usearch_setAttribute(strsrch: *mut UStringSearch, attribute: USearchAttribute, value: USearchAttributeValue, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn usearch_setAttribute(strsrch : *mut UStringSearch, attribute : USearchAttribute, value : USearchAttributeValue, status : *mut UErrorCode));
    usearch_setAttribute(strsrch, attribute, value, status)
}
#[inline]
pub unsafe fn usearch_setBreakIterator(strsrch: *mut UStringSearch, breakiter: *mut UBreakIterator, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn usearch_setBreakIterator(strsrch : *mut UStringSearch, breakiter : *mut UBreakIterator, status : *mut UErrorCode));
    usearch_setBreakIterator(strsrch, breakiter, status)
}
#[inline]
pub unsafe fn usearch_setCollator(strsrch: *mut UStringSearch, collator: *const UCollator, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn usearch_setCollator(strsrch : *mut UStringSearch, collator : *const UCollator, status : *mut UErrorCode));
    usearch_setCollator(strsrch, collator, status)
}
#[inline]
pub unsafe fn usearch_setOffset(strsrch: *mut UStringSearch, position: i32, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn usearch_setOffset(strsrch : *mut UStringSearch, position : i32, status : *mut UErrorCode));
    usearch_setOffset(strsrch, position, status)
}
#[inline]
pub unsafe fn usearch_setPattern(strsrch: *mut UStringSearch, pattern: *const u16, patternlength: i32, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn usearch_setPattern(strsrch : *mut UStringSearch, pattern : *const u16, patternlength : i32, status : *mut UErrorCode));
    usearch_setPattern(strsrch, pattern, patternlength, status)
}
#[inline]
pub unsafe fn usearch_setText(strsrch: *mut UStringSearch, text: *const u16, textlength: i32, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn usearch_setText(strsrch : *mut UStringSearch, text : *const u16, textlength : i32, status : *mut UErrorCode));
    usearch_setText(strsrch, text, textlength, status)
}
#[inline]
pub unsafe fn uset_add(set: *mut USet, c: i32) {
    windows_targets::link!("icu.dll" "cdecl" fn uset_add(set : *mut USet, c : i32));
    uset_add(set, c)
}
#[inline]
pub unsafe fn uset_addAll(set: *mut USet, additionalset: *const USet) {
    windows_targets::link!("icu.dll" "cdecl" fn uset_addAll(set : *mut USet, additionalset : *const USet));
    uset_addAll(set, additionalset)
}
#[inline]
pub unsafe fn uset_addAllCodePoints(set: *mut USet, str: *const u16, strlen: i32) {
    windows_targets::link!("icu.dll" "cdecl" fn uset_addAllCodePoints(set : *mut USet, str : *const u16, strlen : i32));
    uset_addAllCodePoints(set, str, strlen)
}
#[inline]
pub unsafe fn uset_addRange(set: *mut USet, start: i32, end: i32) {
    windows_targets::link!("icu.dll" "cdecl" fn uset_addRange(set : *mut USet, start : i32, end : i32));
    uset_addRange(set, start, end)
}
#[inline]
pub unsafe fn uset_addString(set: *mut USet, str: *const u16, strlen: i32) {
    windows_targets::link!("icu.dll" "cdecl" fn uset_addString(set : *mut USet, str : *const u16, strlen : i32));
    uset_addString(set, str, strlen)
}
#[inline]
pub unsafe fn uset_applyIntPropertyValue(set: *mut USet, prop: UProperty, value: i32, ec: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn uset_applyIntPropertyValue(set : *mut USet, prop : UProperty, value : i32, ec : *mut UErrorCode));
    uset_applyIntPropertyValue(set, prop, value, ec)
}
#[inline]
pub unsafe fn uset_applyPattern(set: *mut USet, pattern: *const u16, patternlength: i32, options: u32, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn uset_applyPattern(set : *mut USet, pattern : *const u16, patternlength : i32, options : u32, status : *mut UErrorCode) -> i32);
    uset_applyPattern(set, pattern, patternlength, options, status)
}
#[inline]
pub unsafe fn uset_applyPropertyAlias(set: *mut USet, prop: *const u16, proplength: i32, value: *const u16, valuelength: i32, ec: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn uset_applyPropertyAlias(set : *mut USet, prop : *const u16, proplength : i32, value : *const u16, valuelength : i32, ec : *mut UErrorCode));
    uset_applyPropertyAlias(set, prop, proplength, value, valuelength, ec)
}
#[inline]
pub unsafe fn uset_charAt(set: *const USet, charindex: i32) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn uset_charAt(set : *const USet, charindex : i32) -> i32);
    uset_charAt(set, charindex)
}
#[inline]
pub unsafe fn uset_clear(set: *mut USet) {
    windows_targets::link!("icu.dll" "cdecl" fn uset_clear(set : *mut USet));
    uset_clear(set)
}
#[inline]
pub unsafe fn uset_clone(set: *const USet) -> *mut USet {
    windows_targets::link!("icu.dll" "cdecl" fn uset_clone(set : *const USet) -> *mut USet);
    uset_clone(set)
}
#[inline]
pub unsafe fn uset_cloneAsThawed(set: *const USet) -> *mut USet {
    windows_targets::link!("icu.dll" "cdecl" fn uset_cloneAsThawed(set : *const USet) -> *mut USet);
    uset_cloneAsThawed(set)
}
#[inline]
pub unsafe fn uset_close(set: *mut USet) {
    windows_targets::link!("icu.dll" "cdecl" fn uset_close(set : *mut USet));
    uset_close(set)
}
#[inline]
pub unsafe fn uset_closeOver(set: *mut USet, attributes: i32) {
    windows_targets::link!("icu.dll" "cdecl" fn uset_closeOver(set : *mut USet, attributes : i32));
    uset_closeOver(set, attributes)
}
#[inline]
pub unsafe fn uset_compact(set: *mut USet) {
    windows_targets::link!("icu.dll" "cdecl" fn uset_compact(set : *mut USet));
    uset_compact(set)
}
#[inline]
pub unsafe fn uset_complement(set: *mut USet) {
    windows_targets::link!("icu.dll" "cdecl" fn uset_complement(set : *mut USet));
    uset_complement(set)
}
#[inline]
pub unsafe fn uset_complementAll(set: *mut USet, complement: *const USet) {
    windows_targets::link!("icu.dll" "cdecl" fn uset_complementAll(set : *mut USet, complement : *const USet));
    uset_complementAll(set, complement)
}
#[inline]
pub unsafe fn uset_contains(set: *const USet, c: i32) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn uset_contains(set : *const USet, c : i32) -> i8);
    uset_contains(set, c)
}
#[inline]
pub unsafe fn uset_containsAll(set1: *const USet, set2: *const USet) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn uset_containsAll(set1 : *const USet, set2 : *const USet) -> i8);
    uset_containsAll(set1, set2)
}
#[inline]
pub unsafe fn uset_containsAllCodePoints(set: *const USet, str: *const u16, strlen: i32) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn uset_containsAllCodePoints(set : *const USet, str : *const u16, strlen : i32) -> i8);
    uset_containsAllCodePoints(set, str, strlen)
}
#[inline]
pub unsafe fn uset_containsNone(set1: *const USet, set2: *const USet) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn uset_containsNone(set1 : *const USet, set2 : *const USet) -> i8);
    uset_containsNone(set1, set2)
}
#[inline]
pub unsafe fn uset_containsRange(set: *const USet, start: i32, end: i32) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn uset_containsRange(set : *const USet, start : i32, end : i32) -> i8);
    uset_containsRange(set, start, end)
}
#[inline]
pub unsafe fn uset_containsSome(set1: *const USet, set2: *const USet) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn uset_containsSome(set1 : *const USet, set2 : *const USet) -> i8);
    uset_containsSome(set1, set2)
}
#[inline]
pub unsafe fn uset_containsString(set: *const USet, str: *const u16, strlen: i32) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn uset_containsString(set : *const USet, str : *const u16, strlen : i32) -> i8);
    uset_containsString(set, str, strlen)
}
#[inline]
pub unsafe fn uset_equals(set1: *const USet, set2: *const USet) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn uset_equals(set1 : *const USet, set2 : *const USet) -> i8);
    uset_equals(set1, set2)
}
#[inline]
pub unsafe fn uset_freeze(set: *mut USet) {
    windows_targets::link!("icu.dll" "cdecl" fn uset_freeze(set : *mut USet));
    uset_freeze(set)
}
#[inline]
pub unsafe fn uset_getItem(set: *const USet, itemindex: i32, start: *mut i32, end: *mut i32, str: *mut u16, strcapacity: i32, ec: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn uset_getItem(set : *const USet, itemindex : i32, start : *mut i32, end : *mut i32, str : *mut u16, strcapacity : i32, ec : *mut UErrorCode) -> i32);
    uset_getItem(set, itemindex, start, end, str, strcapacity, ec)
}
#[inline]
pub unsafe fn uset_getItemCount(set: *const USet) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn uset_getItemCount(set : *const USet) -> i32);
    uset_getItemCount(set)
}
#[inline]
pub unsafe fn uset_getSerializedRange(set: *const USerializedSet, rangeindex: i32, pstart: *mut i32, pend: *mut i32) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn uset_getSerializedRange(set : *const USerializedSet, rangeindex : i32, pstart : *mut i32, pend : *mut i32) -> i8);
    uset_getSerializedRange(set, rangeindex, pstart, pend)
}
#[inline]
pub unsafe fn uset_getSerializedRangeCount(set: *const USerializedSet) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn uset_getSerializedRangeCount(set : *const USerializedSet) -> i32);
    uset_getSerializedRangeCount(set)
}
#[inline]
pub unsafe fn uset_getSerializedSet(fillset: *mut USerializedSet, src: *const u16, srclength: i32) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn uset_getSerializedSet(fillset : *mut USerializedSet, src : *const u16, srclength : i32) -> i8);
    uset_getSerializedSet(fillset, src, srclength)
}
#[inline]
pub unsafe fn uset_indexOf(set: *const USet, c: i32) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn uset_indexOf(set : *const USet, c : i32) -> i32);
    uset_indexOf(set, c)
}
#[inline]
pub unsafe fn uset_isEmpty(set: *const USet) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn uset_isEmpty(set : *const USet) -> i8);
    uset_isEmpty(set)
}
#[inline]
pub unsafe fn uset_isFrozen(set: *const USet) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn uset_isFrozen(set : *const USet) -> i8);
    uset_isFrozen(set)
}
#[inline]
pub unsafe fn uset_open(start: i32, end: i32) -> *mut USet {
    windows_targets::link!("icu.dll" "cdecl" fn uset_open(start : i32, end : i32) -> *mut USet);
    uset_open(start, end)
}
#[inline]
pub unsafe fn uset_openEmpty() -> *mut USet {
    windows_targets::link!("icu.dll" "cdecl" fn uset_openEmpty() -> *mut USet);
    uset_openEmpty()
}
#[inline]
pub unsafe fn uset_openPattern(pattern: *const u16, patternlength: i32, ec: *mut UErrorCode) -> *mut USet {
    windows_targets::link!("icu.dll" "cdecl" fn uset_openPattern(pattern : *const u16, patternlength : i32, ec : *mut UErrorCode) -> *mut USet);
    uset_openPattern(pattern, patternlength, ec)
}
#[inline]
pub unsafe fn uset_openPatternOptions(pattern: *const u16, patternlength: i32, options: u32, ec: *mut UErrorCode) -> *mut USet {
    windows_targets::link!("icu.dll" "cdecl" fn uset_openPatternOptions(pattern : *const u16, patternlength : i32, options : u32, ec : *mut UErrorCode) -> *mut USet);
    uset_openPatternOptions(pattern, patternlength, options, ec)
}
#[inline]
pub unsafe fn uset_remove(set: *mut USet, c: i32) {
    windows_targets::link!("icu.dll" "cdecl" fn uset_remove(set : *mut USet, c : i32));
    uset_remove(set, c)
}
#[inline]
pub unsafe fn uset_removeAll(set: *mut USet, removeset: *const USet) {
    windows_targets::link!("icu.dll" "cdecl" fn uset_removeAll(set : *mut USet, removeset : *const USet));
    uset_removeAll(set, removeset)
}
#[inline]
pub unsafe fn uset_removeAllStrings(set: *mut USet) {
    windows_targets::link!("icu.dll" "cdecl" fn uset_removeAllStrings(set : *mut USet));
    uset_removeAllStrings(set)
}
#[inline]
pub unsafe fn uset_removeRange(set: *mut USet, start: i32, end: i32) {
    windows_targets::link!("icu.dll" "cdecl" fn uset_removeRange(set : *mut USet, start : i32, end : i32));
    uset_removeRange(set, start, end)
}
#[inline]
pub unsafe fn uset_removeString(set: *mut USet, str: *const u16, strlen: i32) {
    windows_targets::link!("icu.dll" "cdecl" fn uset_removeString(set : *mut USet, str : *const u16, strlen : i32));
    uset_removeString(set, str, strlen)
}
#[inline]
pub unsafe fn uset_resemblesPattern(pattern: *const u16, patternlength: i32, pos: i32) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn uset_resemblesPattern(pattern : *const u16, patternlength : i32, pos : i32) -> i8);
    uset_resemblesPattern(pattern, patternlength, pos)
}
#[inline]
pub unsafe fn uset_retain(set: *mut USet, start: i32, end: i32) {
    windows_targets::link!("icu.dll" "cdecl" fn uset_retain(set : *mut USet, start : i32, end : i32));
    uset_retain(set, start, end)
}
#[inline]
pub unsafe fn uset_retainAll(set: *mut USet, retain: *const USet) {
    windows_targets::link!("icu.dll" "cdecl" fn uset_retainAll(set : *mut USet, retain : *const USet));
    uset_retainAll(set, retain)
}
#[inline]
pub unsafe fn uset_serialize(set: *const USet, dest: *mut u16, destcapacity: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn uset_serialize(set : *const USet, dest : *mut u16, destcapacity : i32, perrorcode : *mut UErrorCode) -> i32);
    uset_serialize(set, dest, destcapacity, perrorcode)
}
#[inline]
pub unsafe fn uset_serializedContains(set: *const USerializedSet, c: i32) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn uset_serializedContains(set : *const USerializedSet, c : i32) -> i8);
    uset_serializedContains(set, c)
}
#[inline]
pub unsafe fn uset_set(set: *mut USet, start: i32, end: i32) {
    windows_targets::link!("icu.dll" "cdecl" fn uset_set(set : *mut USet, start : i32, end : i32));
    uset_set(set, start, end)
}
#[inline]
pub unsafe fn uset_setSerializedToOne(fillset: *mut USerializedSet, c: i32) {
    windows_targets::link!("icu.dll" "cdecl" fn uset_setSerializedToOne(fillset : *mut USerializedSet, c : i32));
    uset_setSerializedToOne(fillset, c)
}
#[inline]
pub unsafe fn uset_size(set: *const USet) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn uset_size(set : *const USet) -> i32);
    uset_size(set)
}
#[inline]
pub unsafe fn uset_span(set: *const USet, s: *const u16, length: i32, spancondition: USetSpanCondition) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn uset_span(set : *const USet, s : *const u16, length : i32, spancondition : USetSpanCondition) -> i32);
    uset_span(set, s, length, spancondition)
}
#[inline]
pub unsafe fn uset_spanBack(set: *const USet, s: *const u16, length: i32, spancondition: USetSpanCondition) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn uset_spanBack(set : *const USet, s : *const u16, length : i32, spancondition : USetSpanCondition) -> i32);
    uset_spanBack(set, s, length, spancondition)
}
#[inline]
pub unsafe fn uset_spanBackUTF8<P0>(set: *const USet, s: P0, length: i32, spancondition: USetSpanCondition) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uset_spanBackUTF8(set : *const USet, s : windows_core::PCSTR, length : i32, spancondition : USetSpanCondition) -> i32);
    uset_spanBackUTF8(set, s.param().abi(), length, spancondition)
}
#[inline]
pub unsafe fn uset_spanUTF8<P0>(set: *const USet, s: P0, length: i32, spancondition: USetSpanCondition) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uset_spanUTF8(set : *const USet, s : windows_core::PCSTR, length : i32, spancondition : USetSpanCondition) -> i32);
    uset_spanUTF8(set, s.param().abi(), length, spancondition)
}
#[inline]
pub unsafe fn uset_toPattern(set: *const USet, result: *mut u16, resultcapacity: i32, escapeunprintable: i8, ec: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn uset_toPattern(set : *const USet, result : *mut u16, resultcapacity : i32, escapeunprintable : i8, ec : *mut UErrorCode) -> i32);
    uset_toPattern(set, result, resultcapacity, escapeunprintable, ec)
}
#[inline]
pub unsafe fn uspoof_areConfusable(sc: *const USpoofChecker, id1: *const u16, length1: i32, id2: *const u16, length2: i32, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn uspoof_areConfusable(sc : *const USpoofChecker, id1 : *const u16, length1 : i32, id2 : *const u16, length2 : i32, status : *mut UErrorCode) -> i32);
    uspoof_areConfusable(sc, id1, length1, id2, length2, status)
}
#[inline]
pub unsafe fn uspoof_areConfusableUTF8<P0, P1>(sc: *const USpoofChecker, id1: P0, length1: i32, id2: P1, length2: i32, status: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uspoof_areConfusableUTF8(sc : *const USpoofChecker, id1 : windows_core::PCSTR, length1 : i32, id2 : windows_core::PCSTR, length2 : i32, status : *mut UErrorCode) -> i32);
    uspoof_areConfusableUTF8(sc, id1.param().abi(), length1, id2.param().abi(), length2, status)
}
#[inline]
pub unsafe fn uspoof_check(sc: *const USpoofChecker, id: *const u16, length: i32, position: *mut i32, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn uspoof_check(sc : *const USpoofChecker, id : *const u16, length : i32, position : *mut i32, status : *mut UErrorCode) -> i32);
    uspoof_check(sc, id, length, position, status)
}
#[inline]
pub unsafe fn uspoof_check2(sc: *const USpoofChecker, id: *const u16, length: i32, checkresult: *mut USpoofCheckResult, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn uspoof_check2(sc : *const USpoofChecker, id : *const u16, length : i32, checkresult : *mut USpoofCheckResult, status : *mut UErrorCode) -> i32);
    uspoof_check2(sc, id, length, checkresult, status)
}
#[inline]
pub unsafe fn uspoof_check2UTF8<P0>(sc: *const USpoofChecker, id: P0, length: i32, checkresult: *mut USpoofCheckResult, status: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uspoof_check2UTF8(sc : *const USpoofChecker, id : windows_core::PCSTR, length : i32, checkresult : *mut USpoofCheckResult, status : *mut UErrorCode) -> i32);
    uspoof_check2UTF8(sc, id.param().abi(), length, checkresult, status)
}
#[inline]
pub unsafe fn uspoof_checkUTF8<P0>(sc: *const USpoofChecker, id: P0, length: i32, position: *mut i32, status: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uspoof_checkUTF8(sc : *const USpoofChecker, id : windows_core::PCSTR, length : i32, position : *mut i32, status : *mut UErrorCode) -> i32);
    uspoof_checkUTF8(sc, id.param().abi(), length, position, status)
}
#[inline]
pub unsafe fn uspoof_clone(sc: *const USpoofChecker, status: *mut UErrorCode) -> *mut USpoofChecker {
    windows_targets::link!("icu.dll" "cdecl" fn uspoof_clone(sc : *const USpoofChecker, status : *mut UErrorCode) -> *mut USpoofChecker);
    uspoof_clone(sc, status)
}
#[inline]
pub unsafe fn uspoof_close(sc: *mut USpoofChecker) {
    windows_targets::link!("icu.dll" "cdecl" fn uspoof_close(sc : *mut USpoofChecker));
    uspoof_close(sc)
}
#[inline]
pub unsafe fn uspoof_closeCheckResult(checkresult: *mut USpoofCheckResult) {
    windows_targets::link!("icu.dll" "cdecl" fn uspoof_closeCheckResult(checkresult : *mut USpoofCheckResult));
    uspoof_closeCheckResult(checkresult)
}
#[inline]
pub unsafe fn uspoof_getAllowedChars(sc: *const USpoofChecker, status: *mut UErrorCode) -> *mut USet {
    windows_targets::link!("icu.dll" "cdecl" fn uspoof_getAllowedChars(sc : *const USpoofChecker, status : *mut UErrorCode) -> *mut USet);
    uspoof_getAllowedChars(sc, status)
}
#[inline]
pub unsafe fn uspoof_getAllowedLocales(sc: *mut USpoofChecker, status: *mut UErrorCode) -> windows_core::PCSTR {
    windows_targets::link!("icu.dll" "cdecl" fn uspoof_getAllowedLocales(sc : *mut USpoofChecker, status : *mut UErrorCode) -> windows_core::PCSTR);
    uspoof_getAllowedLocales(sc, status)
}
#[inline]
pub unsafe fn uspoof_getCheckResultChecks(checkresult: *const USpoofCheckResult, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn uspoof_getCheckResultChecks(checkresult : *const USpoofCheckResult, status : *mut UErrorCode) -> i32);
    uspoof_getCheckResultChecks(checkresult, status)
}
#[inline]
pub unsafe fn uspoof_getCheckResultNumerics(checkresult: *const USpoofCheckResult, status: *mut UErrorCode) -> *mut USet {
    windows_targets::link!("icu.dll" "cdecl" fn uspoof_getCheckResultNumerics(checkresult : *const USpoofCheckResult, status : *mut UErrorCode) -> *mut USet);
    uspoof_getCheckResultNumerics(checkresult, status)
}
#[inline]
pub unsafe fn uspoof_getCheckResultRestrictionLevel(checkresult: *const USpoofCheckResult, status: *mut UErrorCode) -> URestrictionLevel {
    windows_targets::link!("icu.dll" "cdecl" fn uspoof_getCheckResultRestrictionLevel(checkresult : *const USpoofCheckResult, status : *mut UErrorCode) -> URestrictionLevel);
    uspoof_getCheckResultRestrictionLevel(checkresult, status)
}
#[inline]
pub unsafe fn uspoof_getChecks(sc: *const USpoofChecker, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn uspoof_getChecks(sc : *const USpoofChecker, status : *mut UErrorCode) -> i32);
    uspoof_getChecks(sc, status)
}
#[inline]
pub unsafe fn uspoof_getInclusionSet(status: *mut UErrorCode) -> *mut USet {
    windows_targets::link!("icu.dll" "cdecl" fn uspoof_getInclusionSet(status : *mut UErrorCode) -> *mut USet);
    uspoof_getInclusionSet(status)
}
#[inline]
pub unsafe fn uspoof_getRecommendedSet(status: *mut UErrorCode) -> *mut USet {
    windows_targets::link!("icu.dll" "cdecl" fn uspoof_getRecommendedSet(status : *mut UErrorCode) -> *mut USet);
    uspoof_getRecommendedSet(status)
}
#[inline]
pub unsafe fn uspoof_getRestrictionLevel(sc: *const USpoofChecker) -> URestrictionLevel {
    windows_targets::link!("icu.dll" "cdecl" fn uspoof_getRestrictionLevel(sc : *const USpoofChecker) -> URestrictionLevel);
    uspoof_getRestrictionLevel(sc)
}
#[inline]
pub unsafe fn uspoof_getSkeleton(sc: *const USpoofChecker, r#type: u32, id: *const u16, length: i32, dest: *mut u16, destcapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn uspoof_getSkeleton(sc : *const USpoofChecker, r#type : u32, id : *const u16, length : i32, dest : *mut u16, destcapacity : i32, status : *mut UErrorCode) -> i32);
    uspoof_getSkeleton(sc, r#type, id, length, dest, destcapacity, status)
}
#[inline]
pub unsafe fn uspoof_getSkeletonUTF8<P0, P1>(sc: *const USpoofChecker, r#type: u32, id: P0, length: i32, dest: P1, destcapacity: i32, status: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uspoof_getSkeletonUTF8(sc : *const USpoofChecker, r#type : u32, id : windows_core::PCSTR, length : i32, dest : windows_core::PCSTR, destcapacity : i32, status : *mut UErrorCode) -> i32);
    uspoof_getSkeletonUTF8(sc, r#type, id.param().abi(), length, dest.param().abi(), destcapacity, status)
}
#[inline]
pub unsafe fn uspoof_open(status: *mut UErrorCode) -> *mut USpoofChecker {
    windows_targets::link!("icu.dll" "cdecl" fn uspoof_open(status : *mut UErrorCode) -> *mut USpoofChecker);
    uspoof_open(status)
}
#[inline]
pub unsafe fn uspoof_openCheckResult(status: *mut UErrorCode) -> *mut USpoofCheckResult {
    windows_targets::link!("icu.dll" "cdecl" fn uspoof_openCheckResult(status : *mut UErrorCode) -> *mut USpoofCheckResult);
    uspoof_openCheckResult(status)
}
#[inline]
pub unsafe fn uspoof_openFromSerialized(data: *const core::ffi::c_void, length: i32, pactuallength: *mut i32, perrorcode: *mut UErrorCode) -> *mut USpoofChecker {
    windows_targets::link!("icu.dll" "cdecl" fn uspoof_openFromSerialized(data : *const core::ffi::c_void, length : i32, pactuallength : *mut i32, perrorcode : *mut UErrorCode) -> *mut USpoofChecker);
    uspoof_openFromSerialized(data, length, pactuallength, perrorcode)
}
#[inline]
pub unsafe fn uspoof_openFromSource<P0, P1>(confusables: P0, confusableslen: i32, confusableswholescript: P1, confusableswholescriptlen: i32, errtype: *mut i32, pe: *mut UParseError, status: *mut UErrorCode) -> *mut USpoofChecker
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uspoof_openFromSource(confusables : windows_core::PCSTR, confusableslen : i32, confusableswholescript : windows_core::PCSTR, confusableswholescriptlen : i32, errtype : *mut i32, pe : *mut UParseError, status : *mut UErrorCode) -> *mut USpoofChecker);
    uspoof_openFromSource(confusables.param().abi(), confusableslen, confusableswholescript.param().abi(), confusableswholescriptlen, errtype, pe, status)
}
#[inline]
pub unsafe fn uspoof_serialize(sc: *mut USpoofChecker, data: *mut core::ffi::c_void, capacity: i32, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn uspoof_serialize(sc : *mut USpoofChecker, data : *mut core::ffi::c_void, capacity : i32, status : *mut UErrorCode) -> i32);
    uspoof_serialize(sc, data, capacity, status)
}
#[inline]
pub unsafe fn uspoof_setAllowedChars(sc: *mut USpoofChecker, chars: *const USet, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn uspoof_setAllowedChars(sc : *mut USpoofChecker, chars : *const USet, status : *mut UErrorCode));
    uspoof_setAllowedChars(sc, chars, status)
}
#[inline]
pub unsafe fn uspoof_setAllowedLocales<P0>(sc: *mut USpoofChecker, localeslist: P0, status: *mut UErrorCode)
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn uspoof_setAllowedLocales(sc : *mut USpoofChecker, localeslist : windows_core::PCSTR, status : *mut UErrorCode));
    uspoof_setAllowedLocales(sc, localeslist.param().abi(), status)
}
#[inline]
pub unsafe fn uspoof_setChecks(sc: *mut USpoofChecker, checks: i32, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn uspoof_setChecks(sc : *mut USpoofChecker, checks : i32, status : *mut UErrorCode));
    uspoof_setChecks(sc, checks, status)
}
#[inline]
pub unsafe fn uspoof_setRestrictionLevel(sc: *mut USpoofChecker, restrictionlevel: URestrictionLevel) {
    windows_targets::link!("icu.dll" "cdecl" fn uspoof_setRestrictionLevel(sc : *mut USpoofChecker, restrictionlevel : URestrictionLevel));
    uspoof_setRestrictionLevel(sc, restrictionlevel)
}
#[inline]
pub unsafe fn usprep_close(profile: *mut UStringPrepProfile) {
    windows_targets::link!("icu.dll" "cdecl" fn usprep_close(profile : *mut UStringPrepProfile));
    usprep_close(profile)
}
#[inline]
pub unsafe fn usprep_open<P0, P1>(path: P0, filename: P1, status: *mut UErrorCode) -> *mut UStringPrepProfile
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn usprep_open(path : windows_core::PCSTR, filename : windows_core::PCSTR, status : *mut UErrorCode) -> *mut UStringPrepProfile);
    usprep_open(path.param().abi(), filename.param().abi(), status)
}
#[inline]
pub unsafe fn usprep_openByType(r#type: UStringPrepProfileType, status: *mut UErrorCode) -> *mut UStringPrepProfile {
    windows_targets::link!("icu.dll" "cdecl" fn usprep_openByType(r#type : UStringPrepProfileType, status : *mut UErrorCode) -> *mut UStringPrepProfile);
    usprep_openByType(r#type, status)
}
#[inline]
pub unsafe fn usprep_prepare(prep: *const UStringPrepProfile, src: *const u16, srclength: i32, dest: *mut u16, destcapacity: i32, options: i32, parseerror: *mut UParseError, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn usprep_prepare(prep : *const UStringPrepProfile, src : *const u16, srclength : i32, dest : *mut u16, destcapacity : i32, options : i32, parseerror : *mut UParseError, status : *mut UErrorCode) -> i32);
    usprep_prepare(prep, src, srclength, dest, destcapacity, options, parseerror, status)
}
#[inline]
pub unsafe fn utext_char32At(ut: *mut UText, nativeindex: i64) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn utext_char32At(ut : *mut UText, nativeindex : i64) -> i32);
    utext_char32At(ut, nativeindex)
}
#[inline]
pub unsafe fn utext_clone(dest: *mut UText, src: *const UText, deep: i8, readonly: i8, status: *mut UErrorCode) -> *mut UText {
    windows_targets::link!("icu.dll" "cdecl" fn utext_clone(dest : *mut UText, src : *const UText, deep : i8, readonly : i8, status : *mut UErrorCode) -> *mut UText);
    utext_clone(dest, src, deep, readonly, status)
}
#[inline]
pub unsafe fn utext_close(ut: *mut UText) -> *mut UText {
    windows_targets::link!("icu.dll" "cdecl" fn utext_close(ut : *mut UText) -> *mut UText);
    utext_close(ut)
}
#[inline]
pub unsafe fn utext_copy(ut: *mut UText, nativestart: i64, nativelimit: i64, destindex: i64, r#move: i8, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn utext_copy(ut : *mut UText, nativestart : i64, nativelimit : i64, destindex : i64, r#move : i8, status : *mut UErrorCode));
    utext_copy(ut, nativestart, nativelimit, destindex, r#move, status)
}
#[inline]
pub unsafe fn utext_current32(ut: *mut UText) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn utext_current32(ut : *mut UText) -> i32);
    utext_current32(ut)
}
#[inline]
pub unsafe fn utext_equals(a: *const UText, b: *const UText) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn utext_equals(a : *const UText, b : *const UText) -> i8);
    utext_equals(a, b)
}
#[inline]
pub unsafe fn utext_extract(ut: *mut UText, nativestart: i64, nativelimit: i64, dest: *mut u16, destcapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn utext_extract(ut : *mut UText, nativestart : i64, nativelimit : i64, dest : *mut u16, destcapacity : i32, status : *mut UErrorCode) -> i32);
    utext_extract(ut, nativestart, nativelimit, dest, destcapacity, status)
}
#[inline]
pub unsafe fn utext_freeze(ut: *mut UText) {
    windows_targets::link!("icu.dll" "cdecl" fn utext_freeze(ut : *mut UText));
    utext_freeze(ut)
}
#[inline]
pub unsafe fn utext_getNativeIndex(ut: *const UText) -> i64 {
    windows_targets::link!("icu.dll" "cdecl" fn utext_getNativeIndex(ut : *const UText) -> i64);
    utext_getNativeIndex(ut)
}
#[inline]
pub unsafe fn utext_getPreviousNativeIndex(ut: *mut UText) -> i64 {
    windows_targets::link!("icu.dll" "cdecl" fn utext_getPreviousNativeIndex(ut : *mut UText) -> i64);
    utext_getPreviousNativeIndex(ut)
}
#[inline]
pub unsafe fn utext_hasMetaData(ut: *const UText) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn utext_hasMetaData(ut : *const UText) -> i8);
    utext_hasMetaData(ut)
}
#[inline]
pub unsafe fn utext_isLengthExpensive(ut: *const UText) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn utext_isLengthExpensive(ut : *const UText) -> i8);
    utext_isLengthExpensive(ut)
}
#[inline]
pub unsafe fn utext_isWritable(ut: *const UText) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn utext_isWritable(ut : *const UText) -> i8);
    utext_isWritable(ut)
}
#[inline]
pub unsafe fn utext_moveIndex32(ut: *mut UText, delta: i32) -> i8 {
    windows_targets::link!("icu.dll" "cdecl" fn utext_moveIndex32(ut : *mut UText, delta : i32) -> i8);
    utext_moveIndex32(ut, delta)
}
#[inline]
pub unsafe fn utext_nativeLength(ut: *mut UText) -> i64 {
    windows_targets::link!("icu.dll" "cdecl" fn utext_nativeLength(ut : *mut UText) -> i64);
    utext_nativeLength(ut)
}
#[inline]
pub unsafe fn utext_next32(ut: *mut UText) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn utext_next32(ut : *mut UText) -> i32);
    utext_next32(ut)
}
#[inline]
pub unsafe fn utext_next32From(ut: *mut UText, nativeindex: i64) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn utext_next32From(ut : *mut UText, nativeindex : i64) -> i32);
    utext_next32From(ut, nativeindex)
}
#[inline]
pub unsafe fn utext_openUChars(ut: *mut UText, s: *const u16, length: i64, status: *mut UErrorCode) -> *mut UText {
    windows_targets::link!("icu.dll" "cdecl" fn utext_openUChars(ut : *mut UText, s : *const u16, length : i64, status : *mut UErrorCode) -> *mut UText);
    utext_openUChars(ut, s, length, status)
}
#[inline]
pub unsafe fn utext_openUTF8<P0>(ut: *mut UText, s: P0, length: i64, status: *mut UErrorCode) -> *mut UText
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn utext_openUTF8(ut : *mut UText, s : windows_core::PCSTR, length : i64, status : *mut UErrorCode) -> *mut UText);
    utext_openUTF8(ut, s.param().abi(), length, status)
}
#[inline]
pub unsafe fn utext_previous32(ut: *mut UText) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn utext_previous32(ut : *mut UText) -> i32);
    utext_previous32(ut)
}
#[inline]
pub unsafe fn utext_previous32From(ut: *mut UText, nativeindex: i64) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn utext_previous32From(ut : *mut UText, nativeindex : i64) -> i32);
    utext_previous32From(ut, nativeindex)
}
#[inline]
pub unsafe fn utext_replace(ut: *mut UText, nativestart: i64, nativelimit: i64, replacementtext: *const u16, replacementlength: i32, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn utext_replace(ut : *mut UText, nativestart : i64, nativelimit : i64, replacementtext : *const u16, replacementlength : i32, status : *mut UErrorCode) -> i32);
    utext_replace(ut, nativestart, nativelimit, replacementtext, replacementlength, status)
}
#[inline]
pub unsafe fn utext_setNativeIndex(ut: *mut UText, nativeindex: i64) {
    windows_targets::link!("icu.dll" "cdecl" fn utext_setNativeIndex(ut : *mut UText, nativeindex : i64));
    utext_setNativeIndex(ut, nativeindex)
}
#[inline]
pub unsafe fn utext_setup(ut: *mut UText, extraspace: i32, status: *mut UErrorCode) -> *mut UText {
    windows_targets::link!("icu.dll" "cdecl" fn utext_setup(ut : *mut UText, extraspace : i32, status : *mut UErrorCode) -> *mut UText);
    utext_setup(ut, extraspace, status)
}
#[inline]
pub unsafe fn utf8_appendCharSafeBody(s: *mut u8, i: i32, length: i32, c: i32, piserror: *mut i8) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn utf8_appendCharSafeBody(s : *mut u8, i : i32, length : i32, c : i32, piserror : *mut i8) -> i32);
    utf8_appendCharSafeBody(s, i, length, c, piserror)
}
#[inline]
pub unsafe fn utf8_back1SafeBody(s: *const u8, start: i32, i: i32) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn utf8_back1SafeBody(s : *const u8, start : i32, i : i32) -> i32);
    utf8_back1SafeBody(s, start, i)
}
#[inline]
pub unsafe fn utf8_nextCharSafeBody(s: *const u8, pi: *mut i32, length: i32, c: i32, strict: i8) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn utf8_nextCharSafeBody(s : *const u8, pi : *mut i32, length : i32, c : i32, strict : i8) -> i32);
    utf8_nextCharSafeBody(s, pi, length, c, strict)
}
#[inline]
pub unsafe fn utf8_prevCharSafeBody(s: *const u8, start: i32, pi: *mut i32, c: i32, strict: i8) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn utf8_prevCharSafeBody(s : *const u8, start : i32, pi : *mut i32, c : i32, strict : i8) -> i32);
    utf8_prevCharSafeBody(s, start, pi, c, strict)
}
#[inline]
pub unsafe fn utmscale_fromInt64(othertime: i64, timescale: UDateTimeScale, status: *mut UErrorCode) -> i64 {
    windows_targets::link!("icu.dll" "cdecl" fn utmscale_fromInt64(othertime : i64, timescale : UDateTimeScale, status : *mut UErrorCode) -> i64);
    utmscale_fromInt64(othertime, timescale, status)
}
#[inline]
pub unsafe fn utmscale_getTimeScaleValue(timescale: UDateTimeScale, value: UTimeScaleValue, status: *mut UErrorCode) -> i64 {
    windows_targets::link!("icu.dll" "cdecl" fn utmscale_getTimeScaleValue(timescale : UDateTimeScale, value : UTimeScaleValue, status : *mut UErrorCode) -> i64);
    utmscale_getTimeScaleValue(timescale, value, status)
}
#[inline]
pub unsafe fn utmscale_toInt64(universaltime: i64, timescale: UDateTimeScale, status: *mut UErrorCode) -> i64 {
    windows_targets::link!("icu.dll" "cdecl" fn utmscale_toInt64(universaltime : i64, timescale : UDateTimeScale, status : *mut UErrorCode) -> i64);
    utmscale_toInt64(universaltime, timescale, status)
}
#[inline]
pub unsafe fn utrace_format<P0, P1>(outbuf: P0, capacity: i32, indent: i32, fmt: P1) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn utrace_format(outbuf : windows_core::PCSTR, capacity : i32, indent : i32, fmt : windows_core::PCSTR) -> i32);
    utrace_format(outbuf.param().abi(), capacity, indent, fmt.param().abi())
}
#[inline]
pub unsafe fn utrace_functionName(fnnumber: i32) -> windows_core::PCSTR {
    windows_targets::link!("icu.dll" "cdecl" fn utrace_functionName(fnnumber : i32) -> windows_core::PCSTR);
    utrace_functionName(fnnumber)
}
#[inline]
pub unsafe fn utrace_getFunctions(context: *const *const core::ffi::c_void, e: *mut UTraceEntry, x: *mut UTraceExit, d: *mut UTraceData) {
    windows_targets::link!("icu.dll" "cdecl" fn utrace_getFunctions(context : *const *const core::ffi::c_void, e : *mut UTraceEntry, x : *mut UTraceExit, d : *mut UTraceData));
    utrace_getFunctions(context, e, x, d)
}
#[inline]
pub unsafe fn utrace_getLevel() -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn utrace_getLevel() -> i32);
    utrace_getLevel()
}
#[inline]
pub unsafe fn utrace_setFunctions(context: *const core::ffi::c_void, e: UTraceEntry, x: UTraceExit, d: UTraceData) {
    windows_targets::link!("icu.dll" "cdecl" fn utrace_setFunctions(context : *const core::ffi::c_void, e : UTraceEntry, x : UTraceExit, d : UTraceData));
    utrace_setFunctions(context, e, x, d)
}
#[inline]
pub unsafe fn utrace_setLevel(tracelevel: i32) {
    windows_targets::link!("icu.dll" "cdecl" fn utrace_setLevel(tracelevel : i32));
    utrace_setLevel(tracelevel)
}
#[inline]
pub unsafe fn utrace_vformat<P0, P1>(outbuf: P0, capacity: i32, indent: i32, fmt: P1, args: *mut i8) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("icu.dll" "cdecl" fn utrace_vformat(outbuf : windows_core::PCSTR, capacity : i32, indent : i32, fmt : windows_core::PCSTR, args : *mut i8) -> i32);
    utrace_vformat(outbuf.param().abi(), capacity, indent, fmt.param().abi(), args)
}
#[inline]
pub unsafe fn utrans_clone(trans: *const *const core::ffi::c_void, status: *mut UErrorCode) -> *mut *mut core::ffi::c_void {
    windows_targets::link!("icu.dll" "cdecl" fn utrans_clone(trans : *const *const core::ffi::c_void, status : *mut UErrorCode) -> *mut *mut core::ffi::c_void);
    utrans_clone(trans, status)
}
#[inline]
pub unsafe fn utrans_close(trans: *mut *mut core::ffi::c_void) {
    windows_targets::link!("icu.dll" "cdecl" fn utrans_close(trans : *mut *mut core::ffi::c_void));
    utrans_close(trans)
}
#[inline]
pub unsafe fn utrans_countAvailableIDs() -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn utrans_countAvailableIDs() -> i32);
    utrans_countAvailableIDs()
}
#[inline]
pub unsafe fn utrans_getSourceSet(trans: *const *const core::ffi::c_void, ignorefilter: i8, fillin: *mut USet, status: *mut UErrorCode) -> *mut USet {
    windows_targets::link!("icu.dll" "cdecl" fn utrans_getSourceSet(trans : *const *const core::ffi::c_void, ignorefilter : i8, fillin : *mut USet, status : *mut UErrorCode) -> *mut USet);
    utrans_getSourceSet(trans, ignorefilter, fillin, status)
}
#[inline]
pub unsafe fn utrans_getUnicodeID(trans: *const *const core::ffi::c_void, resultlength: *mut i32) -> *mut u16 {
    windows_targets::link!("icu.dll" "cdecl" fn utrans_getUnicodeID(trans : *const *const core::ffi::c_void, resultlength : *mut i32) -> *mut u16);
    utrans_getUnicodeID(trans, resultlength)
}
#[inline]
pub unsafe fn utrans_openIDs(perrorcode: *mut UErrorCode) -> *mut UEnumeration {
    windows_targets::link!("icu.dll" "cdecl" fn utrans_openIDs(perrorcode : *mut UErrorCode) -> *mut UEnumeration);
    utrans_openIDs(perrorcode)
}
#[inline]
pub unsafe fn utrans_openInverse(trans: *const *const core::ffi::c_void, status: *mut UErrorCode) -> *mut *mut core::ffi::c_void {
    windows_targets::link!("icu.dll" "cdecl" fn utrans_openInverse(trans : *const *const core::ffi::c_void, status : *mut UErrorCode) -> *mut *mut core::ffi::c_void);
    utrans_openInverse(trans, status)
}
#[inline]
pub unsafe fn utrans_openU(id: *const u16, idlength: i32, dir: UTransDirection, rules: *const u16, ruleslength: i32, parseerror: *mut UParseError, perrorcode: *mut UErrorCode) -> *mut *mut core::ffi::c_void {
    windows_targets::link!("icu.dll" "cdecl" fn utrans_openU(id : *const u16, idlength : i32, dir : UTransDirection, rules : *const u16, ruleslength : i32, parseerror : *mut UParseError, perrorcode : *mut UErrorCode) -> *mut *mut core::ffi::c_void);
    utrans_openU(id, idlength, dir, rules, ruleslength, parseerror, perrorcode)
}
#[inline]
pub unsafe fn utrans_register(adoptedtrans: *mut *mut core::ffi::c_void, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn utrans_register(adoptedtrans : *mut *mut core::ffi::c_void, status : *mut UErrorCode));
    utrans_register(adoptedtrans, status)
}
#[inline]
pub unsafe fn utrans_setFilter(trans: *mut *mut core::ffi::c_void, filterpattern: *const u16, filterpatternlen: i32, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn utrans_setFilter(trans : *mut *mut core::ffi::c_void, filterpattern : *const u16, filterpatternlen : i32, status : *mut UErrorCode));
    utrans_setFilter(trans, filterpattern, filterpatternlen, status)
}
#[inline]
pub unsafe fn utrans_toRules(trans: *const *const core::ffi::c_void, escapeunprintable: i8, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32 {
    windows_targets::link!("icu.dll" "cdecl" fn utrans_toRules(trans : *const *const core::ffi::c_void, escapeunprintable : i8, result : *mut u16, resultlength : i32, status : *mut UErrorCode) -> i32);
    utrans_toRules(trans, escapeunprintable, result, resultlength, status)
}
#[inline]
pub unsafe fn utrans_trans(trans: *const *const core::ffi::c_void, rep: *mut *mut core::ffi::c_void, repfunc: *const UReplaceableCallbacks, start: i32, limit: *mut i32, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn utrans_trans(trans : *const *const core::ffi::c_void, rep : *mut *mut core::ffi::c_void, repfunc : *const UReplaceableCallbacks, start : i32, limit : *mut i32, status : *mut UErrorCode));
    utrans_trans(trans, rep, repfunc, start, limit, status)
}
#[inline]
pub unsafe fn utrans_transIncremental(trans: *const *const core::ffi::c_void, rep: *mut *mut core::ffi::c_void, repfunc: *const UReplaceableCallbacks, pos: *mut UTransPosition, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn utrans_transIncremental(trans : *const *const core::ffi::c_void, rep : *mut *mut core::ffi::c_void, repfunc : *const UReplaceableCallbacks, pos : *mut UTransPosition, status : *mut UErrorCode));
    utrans_transIncremental(trans, rep, repfunc, pos, status)
}
#[inline]
pub unsafe fn utrans_transIncrementalUChars(trans: *const *const core::ffi::c_void, text: *mut u16, textlength: *mut i32, textcapacity: i32, pos: *mut UTransPosition, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn utrans_transIncrementalUChars(trans : *const *const core::ffi::c_void, text : *mut u16, textlength : *mut i32, textcapacity : i32, pos : *mut UTransPosition, status : *mut UErrorCode));
    utrans_transIncrementalUChars(trans, text, textlength, textcapacity, pos, status)
}
#[inline]
pub unsafe fn utrans_transUChars(trans: *const *const core::ffi::c_void, text: *mut u16, textlength: *mut i32, textcapacity: i32, start: i32, limit: *mut i32, status: *mut UErrorCode) {
    windows_targets::link!("icu.dll" "cdecl" fn utrans_transUChars(trans : *const *const core::ffi::c_void, text : *mut u16, textlength : *mut i32, textcapacity : i32, start : i32, limit : *mut i32, status : *mut UErrorCode));
    utrans_transUChars(trans, text, textlength, textcapacity, start, limit, status)
}
#[inline]
pub unsafe fn utrans_unregisterID(id: *const u16, idlength: i32) {
    windows_targets::link!("icu.dll" "cdecl" fn utrans_unregisterID(id : *const u16, idlength : i32));
    utrans_unregisterID(id, idlength)
}
windows_core::imp::define_interface!(IComprehensiveSpellCheckProvider, IComprehensiveSpellCheckProvider_Vtbl, 0x0c58f8de_8e94_479e_9717_70c42c4ad2c3);
impl core::ops::Deref for IComprehensiveSpellCheckProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IComprehensiveSpellCheckProvider, windows_core::IUnknown);
impl IComprehensiveSpellCheckProvider {
    pub unsafe fn ComprehensiveCheck<P0>(&self, text: P0) -> windows_core::Result<IEnumSpellingError>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ComprehensiveCheck)(windows_core::Interface::as_raw(self), text.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IComprehensiveSpellCheckProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ComprehensiveCheck: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumCodePage, IEnumCodePage_Vtbl, 0x275c23e3_3747_11d0_9fea_00aa003f8646);
impl core::ops::Deref for IEnumCodePage {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumCodePage, windows_core::IUnknown);
impl IEnumCodePage {
    pub unsafe fn Clone(&self, ppenum: Option<*const Option<IEnumCodePage>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), core::mem::transmute(ppenum.unwrap_or(std::ptr::null()))).ok()
    }
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut MIMECPINFO, pceltfetched: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, rgelt, core::mem::transmute(pceltfetched.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
}
#[repr(C)]
pub struct IEnumCodePage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut MIMECPINFO, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumRfc1766, IEnumRfc1766_Vtbl, 0x3dc39d1d_c030_11d0_b81b_00c04fc9b31f);
impl core::ops::Deref for IEnumRfc1766 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumRfc1766, windows_core::IUnknown);
impl IEnumRfc1766 {
    pub unsafe fn Clone(&self, ppenum: Option<*const Option<IEnumRfc1766>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), core::mem::transmute(ppenum.unwrap_or(std::ptr::null()))).ok()
    }
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut RFC1766INFO, pceltfetched: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, rgelt, core::mem::transmute(pceltfetched.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
}
#[repr(C)]
pub struct IEnumRfc1766_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut RFC1766INFO, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumScript, IEnumScript_Vtbl, 0xae5f1430_388b_11d2_8380_00c04f8f5da1);
impl core::ops::Deref for IEnumScript {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumScript, windows_core::IUnknown);
impl IEnumScript {
    pub unsafe fn Clone(&self, ppenum: Option<*const Option<IEnumScript>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), core::mem::transmute(ppenum.unwrap_or(std::ptr::null()))).ok()
    }
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut SCRIPTINFO, pceltfetched: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, rgelt, core::mem::transmute(pceltfetched.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
}
#[repr(C)]
pub struct IEnumScript_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut SCRIPTINFO, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumSpellingError, IEnumSpellingError_Vtbl, 0x803e3bd4_2828_4410_8290_418d1d73c762);
impl core::ops::Deref for IEnumSpellingError {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumSpellingError, windows_core::IUnknown);
impl IEnumSpellingError {
    pub unsafe fn Next(&self, value: *mut Option<ISpellingError>) -> windows_core::HRESULT {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), core::mem::transmute(value))
    }
}
#[repr(C)]
pub struct IEnumSpellingError_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMLangCodePages, IMLangCodePages_Vtbl, 0x359f3443_bd4a_11d0_b188_00aa0038c969);
impl core::ops::Deref for IMLangCodePages {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMLangCodePages, windows_core::IUnknown);
impl IMLangCodePages {
    pub unsafe fn GetCharCodePages(&self, chsrc: u16) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCharCodePages)(windows_core::Interface::as_raw(self), chsrc, &mut result__).map(|| result__)
    }
    pub unsafe fn GetStrCodePages(&self, pszsrc: &[u16], dwprioritycodepages: u32, pdwcodepages: Option<*mut u32>, pcchcodepages: Option<*mut i32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetStrCodePages)(windows_core::Interface::as_raw(self), core::mem::transmute(pszsrc.as_ptr()), pszsrc.len().try_into().unwrap(), dwprioritycodepages, core::mem::transmute(pdwcodepages.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pcchcodepages.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CodePageToCodePages(&self, ucodepage: u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CodePageToCodePages)(windows_core::Interface::as_raw(self), ucodepage, &mut result__).map(|| result__)
    }
    pub unsafe fn CodePagesToCodePage(&self, dwcodepages: u32, udefaultcodepage: u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CodePagesToCodePage)(windows_core::Interface::as_raw(self), dwcodepages, udefaultcodepage, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IMLangCodePages_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCharCodePages: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut u32) -> windows_core::HRESULT,
    pub GetStrCodePages: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32, u32, *mut u32, *mut i32) -> windows_core::HRESULT,
    pub CodePageToCodePages: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub CodePagesToCodePage: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMLangConvertCharset, IMLangConvertCharset_Vtbl, 0xd66d6f98_cdaa_11d0_b822_00c04fc9b31f);
impl core::ops::Deref for IMLangConvertCharset {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMLangConvertCharset, windows_core::IUnknown);
impl IMLangConvertCharset {
    pub unsafe fn Initialize(&self, uisrccodepage: u32, uidstcodepage: u32, dwproperty: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), uisrccodepage, uidstcodepage, dwproperty).ok()
    }
    pub unsafe fn GetSourceCodePage(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSourceCodePage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetDestinationCodePage(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDestinationCodePage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetProperty(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn DoConversion(&self, psrcstr: *const u8, pcsrcsize: Option<*mut u32>, pdststr: *mut u8, pcdstsize: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DoConversion)(windows_core::Interface::as_raw(self), psrcstr, core::mem::transmute(pcsrcsize.unwrap_or(std::ptr::null_mut())), pdststr, core::mem::transmute(pcdstsize.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn DoConversionToUnicode<P0>(&self, psrcstr: P0, pcsrcsize: Option<*mut u32>, pdststr: windows_core::PWSTR, pcdstsize: Option<*mut u32>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).DoConversionToUnicode)(windows_core::Interface::as_raw(self), psrcstr.param().abi(), core::mem::transmute(pcsrcsize.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pdststr), core::mem::transmute(pcdstsize.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn DoConversionFromUnicode<P0>(&self, psrcstr: P0, pcsrcsize: Option<*mut u32>, pdststr: windows_core::PSTR, pcdstsize: Option<*mut u32>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).DoConversionFromUnicode)(windows_core::Interface::as_raw(self), psrcstr.param().abi(), core::mem::transmute(pcsrcsize.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pdststr), core::mem::transmute(pcdstsize.unwrap_or(std::ptr::null_mut()))).ok()
    }
}
#[repr(C)]
pub struct IMLangConvertCharset_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32) -> windows_core::HRESULT,
    pub GetSourceCodePage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetDestinationCodePage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub DoConversion: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, *mut u32, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub DoConversionToUnicode: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, *mut u32, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub DoConversionFromUnicode: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u32, windows_core::PSTR, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMLangFontLink, IMLangFontLink_Vtbl, 0x359f3441_bd4a_11d0_b188_00aa0038c969);
impl core::ops::Deref for IMLangFontLink {
    type Target = IMLangCodePages;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMLangFontLink, windows_core::IUnknown, IMLangCodePages);
impl IMLangFontLink {
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetFontCodePages<P0, P1>(&self, hdc: P0, hfont: P1, pdwcodepages: Option<*mut u32>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Graphics::Gdi::HDC>,
        P1: windows_core::Param<super::Graphics::Gdi::HFONT>,
    {
        (windows_core::Interface::vtable(self).GetFontCodePages)(windows_core::Interface::as_raw(self), hdc.param().abi(), hfont.param().abi(), core::mem::transmute(pdwcodepages.unwrap_or(std::ptr::null_mut()))).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn MapFont<P0, P1>(&self, hdc: P0, dwcodepages: u32, hsrcfont: P1, phdestfont: Option<*mut super::Graphics::Gdi::HFONT>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Graphics::Gdi::HDC>,
        P1: windows_core::Param<super::Graphics::Gdi::HFONT>,
    {
        (windows_core::Interface::vtable(self).MapFont)(windows_core::Interface::as_raw(self), hdc.param().abi(), dwcodepages, hsrcfont.param().abi(), core::mem::transmute(phdestfont.unwrap_or(std::ptr::null_mut()))).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn ReleaseFont<P0>(&self, hfont: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Graphics::Gdi::HFONT>,
    {
        (windows_core::Interface::vtable(self).ReleaseFont)(windows_core::Interface::as_raw(self), hfont.param().abi()).ok()
    }
    pub unsafe fn ResetFontMapping(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ResetFontMapping)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IMLangFontLink_Vtbl {
    pub base__: IMLangCodePages_Vtbl,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetFontCodePages: unsafe extern "system" fn(*mut core::ffi::c_void, super::Graphics::Gdi::HDC, super::Graphics::Gdi::HFONT, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetFontCodePages: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub MapFont: unsafe extern "system" fn(*mut core::ffi::c_void, super::Graphics::Gdi::HDC, u32, super::Graphics::Gdi::HFONT, *mut super::Graphics::Gdi::HFONT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    MapFont: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub ReleaseFont: unsafe extern "system" fn(*mut core::ffi::c_void, super::Graphics::Gdi::HFONT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    ReleaseFont: usize,
    pub ResetFontMapping: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMLangFontLink2, IMLangFontLink2_Vtbl, 0xdccfc162_2b38_11d2_b7ec_00c04f8f5d9a);
impl core::ops::Deref for IMLangFontLink2 {
    type Target = IMLangCodePages;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMLangFontLink2, windows_core::IUnknown, IMLangCodePages);
impl IMLangFontLink2 {
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetFontCodePages<P0, P1>(&self, hdc: P0, hfont: P1, pdwcodepages: Option<*mut u32>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Graphics::Gdi::HDC>,
        P1: windows_core::Param<super::Graphics::Gdi::HFONT>,
    {
        (windows_core::Interface::vtable(self).GetFontCodePages)(windows_core::Interface::as_raw(self), hdc.param().abi(), hfont.param().abi(), core::mem::transmute(pdwcodepages.unwrap_or(std::ptr::null_mut()))).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn ReleaseFont<P0>(&self, hfont: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Graphics::Gdi::HFONT>,
    {
        (windows_core::Interface::vtable(self).ReleaseFont)(windows_core::Interface::as_raw(self), hfont.param().abi()).ok()
    }
    pub unsafe fn ResetFontMapping(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ResetFontMapping)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn MapFont<P0>(&self, hdc: P0, dwcodepages: u32, chsrc: u16, pfont: Option<*mut super::Graphics::Gdi::HFONT>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Graphics::Gdi::HDC>,
    {
        (windows_core::Interface::vtable(self).MapFont)(windows_core::Interface::as_raw(self), hdc.param().abi(), dwcodepages, chsrc, core::mem::transmute(pfont.unwrap_or(std::ptr::null_mut()))).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetFontUnicodeRanges<P0>(&self, hdc: P0, puiranges: *const u32, puranges: Option<*mut UNICODERANGE>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Graphics::Gdi::HDC>,
    {
        (windows_core::Interface::vtable(self).GetFontUnicodeRanges)(windows_core::Interface::as_raw(self), hdc.param().abi(), puiranges, core::mem::transmute(puranges.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetScriptFontInfo(&self, sid: u8, dwflags: u32, puifonts: *mut u32, pscriptfont: Option<*mut SCRIPTFONTINFO>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetScriptFontInfo)(windows_core::Interface::as_raw(self), sid, dwflags, puifonts, core::mem::transmute(pscriptfont.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CodePageToScriptID(&self, uicodepage: u32) -> windows_core::Result<u8> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CodePageToScriptID)(windows_core::Interface::as_raw(self), uicodepage, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IMLangFontLink2_Vtbl {
    pub base__: IMLangCodePages_Vtbl,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetFontCodePages: unsafe extern "system" fn(*mut core::ffi::c_void, super::Graphics::Gdi::HDC, super::Graphics::Gdi::HFONT, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetFontCodePages: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub ReleaseFont: unsafe extern "system" fn(*mut core::ffi::c_void, super::Graphics::Gdi::HFONT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    ReleaseFont: usize,
    pub ResetFontMapping: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub MapFont: unsafe extern "system" fn(*mut core::ffi::c_void, super::Graphics::Gdi::HDC, u32, u16, *mut super::Graphics::Gdi::HFONT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    MapFont: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetFontUnicodeRanges: unsafe extern "system" fn(*mut core::ffi::c_void, super::Graphics::Gdi::HDC, *const u32, *mut UNICODERANGE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetFontUnicodeRanges: usize,
    pub GetScriptFontInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u8, u32, *mut u32, *mut SCRIPTFONTINFO) -> windows_core::HRESULT,
    pub CodePageToScriptID: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMLangLineBreakConsole, IMLangLineBreakConsole_Vtbl, 0xf5be2ee1_bfd7_11d0_b188_00aa0038c969);
impl core::ops::Deref for IMLangLineBreakConsole {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMLangLineBreakConsole, windows_core::IUnknown);
impl IMLangLineBreakConsole {
    pub unsafe fn BreakLineML<P0>(&self, psrcmlstr: P0, lsrcpos: i32, lsrclen: i32, cmincolumns: i32, cmaxcolumns: i32, pllinelen: Option<*mut i32>, plskiplen: Option<*mut i32>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMLangString>,
    {
        (windows_core::Interface::vtable(self).BreakLineML)(windows_core::Interface::as_raw(self), psrcmlstr.param().abi(), lsrcpos, lsrclen, cmincolumns, cmaxcolumns, core::mem::transmute(pllinelen.unwrap_or(std::ptr::null_mut())), core::mem::transmute(plskiplen.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn BreakLineW(&self, locale: u32, pszsrc: &[u16], cmaxcolumns: i32, pcchline: Option<*mut i32>, pcchskip: Option<*mut i32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).BreakLineW)(windows_core::Interface::as_raw(self), locale, core::mem::transmute(pszsrc.as_ptr()), pszsrc.len().try_into().unwrap(), cmaxcolumns, core::mem::transmute(pcchline.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pcchskip.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn BreakLineA(&self, locale: u32, ucodepage: u32, pszsrc: &[u8], cmaxcolumns: i32, pcchline: Option<*mut i32>, pcchskip: Option<*mut i32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).BreakLineA)(windows_core::Interface::as_raw(self), locale, ucodepage, core::mem::transmute(pszsrc.as_ptr()), pszsrc.len().try_into().unwrap(), cmaxcolumns, core::mem::transmute(pcchline.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pcchskip.unwrap_or(std::ptr::null_mut()))).ok()
    }
}
#[repr(C)]
pub struct IMLangLineBreakConsole_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub BreakLineML: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32, i32, i32, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub BreakLineW: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, i32, i32, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub BreakLineA: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, windows_core::PCSTR, i32, i32, *mut i32, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMLangString, IMLangString_Vtbl, 0xc04d65ce_b70d_11d0_b188_00aa0038c969);
impl core::ops::Deref for IMLangString {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMLangString, windows_core::IUnknown);
impl IMLangString {
    pub unsafe fn Sync<P0>(&self, fnoaccess: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).Sync)(windows_core::Interface::as_raw(self), fnoaccess.param().abi()).ok()
    }
    pub unsafe fn GetLength(&self, pllen: Option<*mut i32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLength)(windows_core::Interface::as_raw(self), core::mem::transmute(pllen.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetMLStr<P0>(&self, ldestpos: i32, ldestlen: i32, psrcmlstr: P0, lsrcpos: i32, lsrclen: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).SetMLStr)(windows_core::Interface::as_raw(self), ldestpos, ldestlen, psrcmlstr.param().abi(), lsrcpos, lsrclen).ok()
    }
    pub unsafe fn GetMLStr<P0>(&self, lsrcpos: i32, lsrclen: i32, punkouter: P0, dwclscontext: u32, piid: *const windows_core::GUID, ppdestmlstr: *mut Option<windows_core::IUnknown>, pldestpos: Option<*mut i32>, pldestlen: Option<*mut i32>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).GetMLStr)(windows_core::Interface::as_raw(self), lsrcpos, lsrclen, punkouter.param().abi(), dwclscontext, piid, core::mem::transmute(ppdestmlstr), core::mem::transmute(pldestpos.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pldestlen.unwrap_or(std::ptr::null_mut()))).ok()
    }
}
#[repr(C)]
pub struct IMLangString_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Sync: unsafe extern "system" fn(*mut core::ffi::c_void, super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMLStr: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub GetMLStr: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut core::ffi::c_void, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void, *mut i32, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMLangStringAStr, IMLangStringAStr_Vtbl, 0xc04d65d2_b70d_11d0_b188_00aa0038c969);
impl core::ops::Deref for IMLangStringAStr {
    type Target = IMLangString;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMLangStringAStr, windows_core::IUnknown, IMLangString);
impl IMLangStringAStr {
    pub unsafe fn SetAStr(&self, ldestpos: i32, ldestlen: i32, ucodepage: u32, pszsrc: &[u8], pcchactual: Option<*mut i32>, plactuallen: Option<*mut i32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAStr)(windows_core::Interface::as_raw(self), ldestpos, ldestlen, ucodepage, core::mem::transmute(pszsrc.as_ptr()), pszsrc.len().try_into().unwrap(), core::mem::transmute(pcchactual.unwrap_or(std::ptr::null_mut())), core::mem::transmute(plactuallen.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetStrBufA<P0>(&self, ldestpos: i32, ldestlen: i32, ucodepage: u32, psrcbuf: P0, pcchactual: Option<*mut i32>, plactuallen: Option<*mut i32>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMLangStringBufA>,
    {
        (windows_core::Interface::vtable(self).SetStrBufA)(windows_core::Interface::as_raw(self), ldestpos, ldestlen, ucodepage, psrcbuf.param().abi(), core::mem::transmute(pcchactual.unwrap_or(std::ptr::null_mut())), core::mem::transmute(plactuallen.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetAStr(&self, lsrcpos: i32, lsrclen: i32, ucodepagein: u32, pucodepageout: Option<*const u32>, pszdest: Option<&mut [u8]>, pcchactual: Option<*mut i32>, plactuallen: Option<*mut i32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAStr)(windows_core::Interface::as_raw(self), lsrcpos, lsrclen, ucodepagein, core::mem::transmute(pucodepageout.unwrap_or(std::ptr::null())), core::mem::transmute(pszdest.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pszdest.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pcchactual.unwrap_or(std::ptr::null_mut())), core::mem::transmute(plactuallen.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetStrBufA(&self, lsrcpos: i32, lsrcmaxlen: i32, pudestcodepage: Option<*mut u32>, ppdestbuf: *mut Option<IMLangStringBufA>, pldestlen: Option<*mut i32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetStrBufA)(windows_core::Interface::as_raw(self), lsrcpos, lsrcmaxlen, core::mem::transmute(pudestcodepage.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ppdestbuf), core::mem::transmute(pldestlen.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn LockAStr(&self, lsrcpos: i32, lsrclen: i32, lflags: i32, ucodepagein: u32, cchrequest: i32, pucodepageout: Option<*mut u32>, ppszdest: Option<*mut windows_core::PSTR>, pcchdest: Option<*mut i32>, pldestlen: Option<*mut i32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).LockAStr)(windows_core::Interface::as_raw(self), lsrcpos, lsrclen, lflags, ucodepagein, cchrequest, core::mem::transmute(pucodepageout.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ppszdest.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pcchdest.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pldestlen.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn UnlockAStr(&self, pszsrc: &[u8], pcchactual: Option<*mut i32>, plactuallen: Option<*mut i32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UnlockAStr)(windows_core::Interface::as_raw(self), core::mem::transmute(pszsrc.as_ptr()), pszsrc.len().try_into().unwrap(), core::mem::transmute(pcchactual.unwrap_or(std::ptr::null_mut())), core::mem::transmute(plactuallen.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetLocale(&self, ldestpos: i32, ldestlen: i32, locale: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLocale)(windows_core::Interface::as_raw(self), ldestpos, ldestlen, locale).ok()
    }
    pub unsafe fn GetLocale(&self, lsrcpos: i32, lsrcmaxlen: i32, plocale: Option<*mut u32>, pllocalepos: Option<*mut i32>, pllocalelen: Option<*mut i32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLocale)(windows_core::Interface::as_raw(self), lsrcpos, lsrcmaxlen, core::mem::transmute(plocale.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pllocalepos.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pllocalelen.unwrap_or(std::ptr::null_mut()))).ok()
    }
}
#[repr(C)]
pub struct IMLangStringAStr_Vtbl {
    pub base__: IMLangString_Vtbl,
    pub SetAStr: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, u32, windows_core::PCSTR, i32, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub SetStrBufA: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, u32, *mut core::ffi::c_void, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub GetAStr: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, u32, *const u32, windows_core::PSTR, i32, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub GetStrBufA: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut u32, *mut *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub LockAStr: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, u32, i32, *mut u32, *mut windows_core::PSTR, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub UnlockAStr: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, i32, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub SetLocale: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, u32) -> windows_core::HRESULT,
    pub GetLocale: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut u32, *mut i32, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMLangStringBufA, IMLangStringBufA_Vtbl, 0xd24acd23_ba72_11d0_b188_00aa0038c969);
impl core::ops::Deref for IMLangStringBufA {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMLangStringBufA, windows_core::IUnknown);
impl IMLangStringBufA {
    pub unsafe fn GetStatus(&self, plflags: Option<*mut i32>, pcchbuf: Option<*mut i32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), core::mem::transmute(plflags.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pcchbuf.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn LockBuf(&self, cchoffset: i32, cchmaxlock: i32, ppszbuf: *mut *mut i8, pcchbuf: Option<*mut i32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).LockBuf)(windows_core::Interface::as_raw(self), cchoffset, cchmaxlock, ppszbuf, core::mem::transmute(pcchbuf.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn UnlockBuf<P0>(&self, pszbuf: P0, cchoffset: i32, cchwrite: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).UnlockBuf)(windows_core::Interface::as_raw(self), pszbuf.param().abi(), cchoffset, cchwrite).ok()
    }
    pub unsafe fn Insert(&self, cchoffset: i32, cchmaxinsert: i32, pcchactual: Option<*mut i32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Insert)(windows_core::Interface::as_raw(self), cchoffset, cchmaxinsert, core::mem::transmute(pcchactual.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Delete(&self, cchoffset: i32, cchdelete: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), cchoffset, cchdelete).ok()
    }
}
#[repr(C)]
pub struct IMLangStringBufA_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub LockBuf: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut i8, *mut i32) -> windows_core::HRESULT,
    pub UnlockBuf: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, i32, i32) -> windows_core::HRESULT,
    pub Insert: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut i32) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMLangStringBufW, IMLangStringBufW_Vtbl, 0xd24acd21_ba72_11d0_b188_00aa0038c969);
impl core::ops::Deref for IMLangStringBufW {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMLangStringBufW, windows_core::IUnknown);
impl IMLangStringBufW {
    pub unsafe fn GetStatus(&self, plflags: Option<*mut i32>, pcchbuf: Option<*mut i32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), core::mem::transmute(plflags.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pcchbuf.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn LockBuf(&self, cchoffset: i32, cchmaxlock: i32, ppszbuf: *mut *mut u16, pcchbuf: Option<*mut i32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).LockBuf)(windows_core::Interface::as_raw(self), cchoffset, cchmaxlock, ppszbuf, core::mem::transmute(pcchbuf.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn UnlockBuf<P0>(&self, pszbuf: P0, cchoffset: i32, cchwrite: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).UnlockBuf)(windows_core::Interface::as_raw(self), pszbuf.param().abi(), cchoffset, cchwrite).ok()
    }
    pub unsafe fn Insert(&self, cchoffset: i32, cchmaxinsert: i32, pcchactual: Option<*mut i32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Insert)(windows_core::Interface::as_raw(self), cchoffset, cchmaxinsert, core::mem::transmute(pcchactual.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Delete(&self, cchoffset: i32, cchdelete: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), cchoffset, cchdelete).ok()
    }
}
#[repr(C)]
pub struct IMLangStringBufW_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub LockBuf: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut u16, *mut i32) -> windows_core::HRESULT,
    pub UnlockBuf: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32, i32) -> windows_core::HRESULT,
    pub Insert: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut i32) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMLangStringWStr, IMLangStringWStr_Vtbl, 0xc04d65d0_b70d_11d0_b188_00aa0038c969);
impl core::ops::Deref for IMLangStringWStr {
    type Target = IMLangString;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMLangStringWStr, windows_core::IUnknown, IMLangString);
impl IMLangStringWStr {
    pub unsafe fn SetWStr(&self, ldestpos: i32, ldestlen: i32, pszsrc: &[u16], pcchactual: Option<*mut i32>, plactuallen: Option<*mut i32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetWStr)(windows_core::Interface::as_raw(self), ldestpos, ldestlen, core::mem::transmute(pszsrc.as_ptr()), pszsrc.len().try_into().unwrap(), core::mem::transmute(pcchactual.unwrap_or(std::ptr::null_mut())), core::mem::transmute(plactuallen.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetStrBufW<P0>(&self, ldestpos: i32, ldestlen: i32, psrcbuf: P0, pcchactual: Option<*mut i32>, plactuallen: Option<*mut i32>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMLangStringBufW>,
    {
        (windows_core::Interface::vtable(self).SetStrBufW)(windows_core::Interface::as_raw(self), ldestpos, ldestlen, psrcbuf.param().abi(), core::mem::transmute(pcchactual.unwrap_or(std::ptr::null_mut())), core::mem::transmute(plactuallen.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetWStr(&self, lsrcpos: i32, lsrclen: i32, pszdest: Option<&mut [u16]>, pcchactual: Option<*mut i32>, plactuallen: Option<*mut i32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetWStr)(windows_core::Interface::as_raw(self), lsrcpos, lsrclen, core::mem::transmute(pszdest.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pszdest.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pcchactual.unwrap_or(std::ptr::null_mut())), core::mem::transmute(plactuallen.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetStrBufW(&self, lsrcpos: i32, lsrcmaxlen: i32, ppdestbuf: *mut Option<IMLangStringBufW>, pldestlen: Option<*mut i32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetStrBufW)(windows_core::Interface::as_raw(self), lsrcpos, lsrcmaxlen, core::mem::transmute(ppdestbuf), core::mem::transmute(pldestlen.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn LockWStr(&self, lsrcpos: i32, lsrclen: i32, lflags: i32, cchrequest: i32, ppszdest: Option<*mut windows_core::PWSTR>, pcchdest: Option<*mut i32>, pldestlen: Option<*mut i32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).LockWStr)(windows_core::Interface::as_raw(self), lsrcpos, lsrclen, lflags, cchrequest, core::mem::transmute(ppszdest.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pcchdest.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pldestlen.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn UnlockWStr(&self, pszsrc: &[u16], pcchactual: Option<*mut i32>, plactuallen: Option<*mut i32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UnlockWStr)(windows_core::Interface::as_raw(self), core::mem::transmute(pszsrc.as_ptr()), pszsrc.len().try_into().unwrap(), core::mem::transmute(pcchactual.unwrap_or(std::ptr::null_mut())), core::mem::transmute(plactuallen.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetLocale(&self, ldestpos: i32, ldestlen: i32, locale: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLocale)(windows_core::Interface::as_raw(self), ldestpos, ldestlen, locale).ok()
    }
    pub unsafe fn GetLocale(&self, lsrcpos: i32, lsrcmaxlen: i32, plocale: Option<*mut u32>, pllocalepos: Option<*mut i32>, pllocalelen: Option<*mut i32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLocale)(windows_core::Interface::as_raw(self), lsrcpos, lsrcmaxlen, core::mem::transmute(plocale.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pllocalepos.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pllocalelen.unwrap_or(std::ptr::null_mut()))).ok()
    }
}
#[repr(C)]
pub struct IMLangStringWStr_Vtbl {
    pub base__: IMLangString_Vtbl,
    pub SetWStr: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, windows_core::PCWSTR, i32, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub SetStrBufW: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut core::ffi::c_void, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub GetWStr: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, windows_core::PWSTR, i32, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub GetStrBufW: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub LockWStr: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, i32, *mut windows_core::PWSTR, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub UnlockWStr: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub SetLocale: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, u32) -> windows_core::HRESULT,
    pub GetLocale: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut u32, *mut i32, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMultiLanguage, IMultiLanguage_Vtbl, 0x275c23e1_3747_11d0_9fea_00aa003f8646);
impl core::ops::Deref for IMultiLanguage {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMultiLanguage, windows_core::IUnknown);
impl IMultiLanguage {
    pub unsafe fn GetNumberOfCodePageInfo(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNumberOfCodePageInfo)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetCodePageInfo(&self, uicodepage: u32, pcodepageinfo: *mut MIMECPINFO) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCodePageInfo)(windows_core::Interface::as_raw(self), uicodepage, pcodepageinfo).ok()
    }
    pub unsafe fn GetFamilyCodePage(&self, uicodepage: u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFamilyCodePage)(windows_core::Interface::as_raw(self), uicodepage, &mut result__).map(|| result__)
    }
    pub unsafe fn EnumCodePages(&self, grfflags: u32) -> windows_core::Result<IEnumCodePage> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumCodePages)(windows_core::Interface::as_raw(self), grfflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCharsetInfo<P0>(&self, charset: P0, pcharsetinfo: *mut MIMECSETINFO) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).GetCharsetInfo)(windows_core::Interface::as_raw(self), charset.param().abi(), pcharsetinfo).ok()
    }
    pub unsafe fn IsConvertible(&self, dwsrcencoding: u32, dwdstencoding: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).IsConvertible)(windows_core::Interface::as_raw(self), dwsrcencoding, dwdstencoding).ok()
    }
    pub unsafe fn ConvertString(&self, pdwmode: Option<*mut u32>, dwsrcencoding: u32, dwdstencoding: u32, psrcstr: Option<*const u8>, pcsrcsize: Option<*mut u32>, pdststr: Option<*mut u8>, pcdstsize: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ConvertString)(windows_core::Interface::as_raw(self), core::mem::transmute(pdwmode.unwrap_or(std::ptr::null_mut())), dwsrcencoding, dwdstencoding, core::mem::transmute(psrcstr.unwrap_or(std::ptr::null())), core::mem::transmute(pcsrcsize.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pdststr.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pcdstsize.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn ConvertStringToUnicode<P0>(&self, pdwmode: Option<*mut u32>, dwencoding: u32, psrcstr: P0, pcsrcsize: Option<*mut u32>, pdststr: windows_core::PWSTR, pcdstsize: Option<*mut u32>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).ConvertStringToUnicode)(windows_core::Interface::as_raw(self), core::mem::transmute(pdwmode.unwrap_or(std::ptr::null_mut())), dwencoding, psrcstr.param().abi(), core::mem::transmute(pcsrcsize.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pdststr), core::mem::transmute(pcdstsize.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn ConvertStringFromUnicode<P0>(&self, pdwmode: Option<*mut u32>, dwencoding: u32, psrcstr: P0, pcsrcsize: Option<*mut u32>, pdststr: windows_core::PSTR, pcdstsize: Option<*mut u32>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).ConvertStringFromUnicode)(windows_core::Interface::as_raw(self), core::mem::transmute(pdwmode.unwrap_or(std::ptr::null_mut())), dwencoding, psrcstr.param().abi(), core::mem::transmute(pcsrcsize.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pdststr), core::mem::transmute(pcdstsize.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn ConvertStringReset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ConvertStringReset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetRfc1766FromLcid(&self, locale: u32) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRfc1766FromLcid)(windows_core::Interface::as_raw(self), locale, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetLcidFromRfc1766<P0>(&self, plocale: *mut u32, bstrrfc1766: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).GetLcidFromRfc1766)(windows_core::Interface::as_raw(self), plocale, bstrrfc1766.param().abi()).ok()
    }
    pub unsafe fn EnumRfc1766(&self) -> windows_core::Result<IEnumRfc1766> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumRfc1766)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetRfc1766Info(&self, locale: u32, prfc1766info: *mut RFC1766INFO) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRfc1766Info)(windows_core::Interface::as_raw(self), locale, prfc1766info).ok()
    }
    pub unsafe fn CreateConvertCharset(&self, uisrccodepage: u32, uidstcodepage: u32, dwproperty: u32) -> windows_core::Result<IMLangConvertCharset> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateConvertCharset)(windows_core::Interface::as_raw(self), uisrccodepage, uidstcodepage, dwproperty, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IMultiLanguage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetNumberOfCodePageInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetCodePageInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut MIMECPINFO) -> windows_core::HRESULT,
    pub GetFamilyCodePage: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub EnumCodePages: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCharsetInfo: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut MIMECSETINFO) -> windows_core::HRESULT,
    pub IsConvertible: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub ConvertString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, u32, u32, *const u8, *mut u32, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub ConvertStringToUnicode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, u32, windows_core::PCSTR, *mut u32, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub ConvertStringFromUnicode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, u32, windows_core::PCWSTR, *mut u32, windows_core::PSTR, *mut u32) -> windows_core::HRESULT,
    pub ConvertStringReset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRfc1766FromLcid: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetLcidFromRfc1766: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub EnumRfc1766: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRfc1766Info: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut RFC1766INFO) -> windows_core::HRESULT,
    pub CreateConvertCharset: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMultiLanguage2, IMultiLanguage2_Vtbl, 0xdccfc164_2b38_11d2_b7ec_00c04f8f5d9a);
impl core::ops::Deref for IMultiLanguage2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMultiLanguage2, windows_core::IUnknown);
impl IMultiLanguage2 {
    pub unsafe fn GetNumberOfCodePageInfo(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNumberOfCodePageInfo)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetCodePageInfo(&self, uicodepage: u32, langid: u16, pcodepageinfo: *mut MIMECPINFO) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCodePageInfo)(windows_core::Interface::as_raw(self), uicodepage, langid, pcodepageinfo).ok()
    }
    pub unsafe fn GetFamilyCodePage(&self, uicodepage: u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFamilyCodePage)(windows_core::Interface::as_raw(self), uicodepage, &mut result__).map(|| result__)
    }
    pub unsafe fn EnumCodePages(&self, grfflags: u32, langid: u16) -> windows_core::Result<IEnumCodePage> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumCodePages)(windows_core::Interface::as_raw(self), grfflags, langid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCharsetInfo<P0>(&self, charset: P0, pcharsetinfo: *mut MIMECSETINFO) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).GetCharsetInfo)(windows_core::Interface::as_raw(self), charset.param().abi(), pcharsetinfo).ok()
    }
    pub unsafe fn IsConvertible(&self, dwsrcencoding: u32, dwdstencoding: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).IsConvertible)(windows_core::Interface::as_raw(self), dwsrcencoding, dwdstencoding).ok()
    }
    pub unsafe fn ConvertString(&self, pdwmode: Option<*mut u32>, dwsrcencoding: u32, dwdstencoding: u32, psrcstr: Option<*const u8>, pcsrcsize: Option<*mut u32>, pdststr: Option<*mut u8>, pcdstsize: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ConvertString)(windows_core::Interface::as_raw(self), core::mem::transmute(pdwmode.unwrap_or(std::ptr::null_mut())), dwsrcencoding, dwdstencoding, core::mem::transmute(psrcstr.unwrap_or(std::ptr::null())), core::mem::transmute(pcsrcsize.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pdststr.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pcdstsize.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn ConvertStringToUnicode<P0>(&self, pdwmode: Option<*mut u32>, dwencoding: u32, psrcstr: P0, pcsrcsize: Option<*mut u32>, pdststr: windows_core::PWSTR, pcdstsize: Option<*mut u32>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).ConvertStringToUnicode)(windows_core::Interface::as_raw(self), core::mem::transmute(pdwmode.unwrap_or(std::ptr::null_mut())), dwencoding, psrcstr.param().abi(), core::mem::transmute(pcsrcsize.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pdststr), core::mem::transmute(pcdstsize.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn ConvertStringFromUnicode<P0>(&self, pdwmode: Option<*mut u32>, dwencoding: u32, psrcstr: P0, pcsrcsize: Option<*mut u32>, pdststr: windows_core::PSTR, pcdstsize: Option<*mut u32>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).ConvertStringFromUnicode)(windows_core::Interface::as_raw(self), core::mem::transmute(pdwmode.unwrap_or(std::ptr::null_mut())), dwencoding, psrcstr.param().abi(), core::mem::transmute(pcsrcsize.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pdststr), core::mem::transmute(pcdstsize.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn ConvertStringReset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ConvertStringReset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetRfc1766FromLcid(&self, locale: u32) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRfc1766FromLcid)(windows_core::Interface::as_raw(self), locale, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetLcidFromRfc1766<P0>(&self, plocale: *mut u32, bstrrfc1766: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).GetLcidFromRfc1766)(windows_core::Interface::as_raw(self), plocale, bstrrfc1766.param().abi()).ok()
    }
    pub unsafe fn EnumRfc1766(&self, langid: u16) -> windows_core::Result<IEnumRfc1766> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumRfc1766)(windows_core::Interface::as_raw(self), langid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetRfc1766Info(&self, locale: u32, langid: u16, prfc1766info: *mut RFC1766INFO) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRfc1766Info)(windows_core::Interface::as_raw(self), locale, langid, prfc1766info).ok()
    }
    pub unsafe fn CreateConvertCharset(&self, uisrccodepage: u32, uidstcodepage: u32, dwproperty: u32) -> windows_core::Result<IMLangConvertCharset> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateConvertCharset)(windows_core::Interface::as_raw(self), uisrccodepage, uidstcodepage, dwproperty, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ConvertStringInIStream<P0, P1, P2>(&self, pdwmode: Option<*mut u32>, dwflag: u32, lpfallback: P0, dwsrcencoding: u32, dwdstencoding: u32, pstmin: P1, pstmout: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::System::Com::IStream>,
        P2: windows_core::Param<super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).ConvertStringInIStream)(windows_core::Interface::as_raw(self), core::mem::transmute(pdwmode.unwrap_or(std::ptr::null_mut())), dwflag, lpfallback.param().abi(), dwsrcencoding, dwdstencoding, pstmin.param().abi(), pstmout.param().abi()).ok()
    }
    pub unsafe fn ConvertStringToUnicodeEx<P0, P1>(&self, pdwmode: Option<*mut u32>, dwencoding: u32, psrcstr: P0, pcsrcsize: Option<*mut u32>, pdststr: windows_core::PWSTR, pcdstsize: Option<*mut u32>, dwflag: u32, lpfallback: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).ConvertStringToUnicodeEx)(windows_core::Interface::as_raw(self), core::mem::transmute(pdwmode.unwrap_or(std::ptr::null_mut())), dwencoding, psrcstr.param().abi(), core::mem::transmute(pcsrcsize.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pdststr), core::mem::transmute(pcdstsize.unwrap_or(std::ptr::null_mut())), dwflag, lpfallback.param().abi()).ok()
    }
    pub unsafe fn ConvertStringFromUnicodeEx<P0, P1>(&self, pdwmode: Option<*mut u32>, dwencoding: u32, psrcstr: P0, pcsrcsize: Option<*mut u32>, pdststr: windows_core::PSTR, pcdstsize: Option<*mut u32>, dwflag: u32, lpfallback: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).ConvertStringFromUnicodeEx)(windows_core::Interface::as_raw(self), core::mem::transmute(pdwmode.unwrap_or(std::ptr::null_mut())), dwencoding, psrcstr.param().abi(), core::mem::transmute(pcsrcsize.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pdststr), core::mem::transmute(pcdstsize.unwrap_or(std::ptr::null_mut())), dwflag, lpfallback.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DetectCodepageInIStream<P0>(&self, dwflag: u32, dwprefwincodepage: u32, pstmin: P0, lpencoding: *mut DetectEncodingInfo, pnscores: *mut i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).DetectCodepageInIStream)(windows_core::Interface::as_raw(self), dwflag, dwprefwincodepage, pstmin.param().abi(), lpencoding, pnscores).ok()
    }
    pub unsafe fn DetectInputCodepage<P0>(&self, dwflag: u32, dwprefwincodepage: u32, psrcstr: P0, pcsrcsize: *mut i32, lpencoding: *mut DetectEncodingInfo, pnscores: *mut i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).DetectInputCodepage)(windows_core::Interface::as_raw(self), dwflag, dwprefwincodepage, psrcstr.param().abi(), pcsrcsize, lpencoding, pnscores).ok()
    }
    pub unsafe fn ValidateCodePage<P0>(&self, uicodepage: u32, hwnd: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).ValidateCodePage)(windows_core::Interface::as_raw(self), uicodepage, hwnd.param().abi()).ok()
    }
    pub unsafe fn GetCodePageDescription(&self, uicodepage: u32, lcid: u32, lpwidecharstr: &mut [u16]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCodePageDescription)(windows_core::Interface::as_raw(self), uicodepage, lcid, core::mem::transmute(lpwidecharstr.as_ptr()), lpwidecharstr.len().try_into().unwrap()).ok()
    }
    pub unsafe fn IsCodePageInstallable(&self, uicodepage: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).IsCodePageInstallable)(windows_core::Interface::as_raw(self), uicodepage).ok()
    }
    pub unsafe fn SetMimeDBSource(&self, dwsource: MIMECONTF) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMimeDBSource)(windows_core::Interface::as_raw(self), dwsource).ok()
    }
    pub unsafe fn GetNumberOfScripts(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNumberOfScripts)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn EnumScripts(&self, dwflags: u32, langid: u16) -> windows_core::Result<IEnumScript> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumScripts)(windows_core::Interface::as_raw(self), dwflags, langid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ValidateCodePageEx<P0>(&self, uicodepage: u32, hwnd: P0, dwfiodcontrol: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).ValidateCodePageEx)(windows_core::Interface::as_raw(self), uicodepage, hwnd.param().abi(), dwfiodcontrol).ok()
    }
}
#[repr(C)]
pub struct IMultiLanguage2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetNumberOfCodePageInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetCodePageInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u16, *mut MIMECPINFO) -> windows_core::HRESULT,
    pub GetFamilyCodePage: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub EnumCodePages: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCharsetInfo: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut MIMECSETINFO) -> windows_core::HRESULT,
    pub IsConvertible: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub ConvertString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, u32, u32, *const u8, *mut u32, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub ConvertStringToUnicode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, u32, windows_core::PCSTR, *mut u32, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub ConvertStringFromUnicode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, u32, windows_core::PCWSTR, *mut u32, windows_core::PSTR, *mut u32) -> windows_core::HRESULT,
    pub ConvertStringReset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRfc1766FromLcid: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetLcidFromRfc1766: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub EnumRfc1766: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRfc1766Info: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u16, *mut RFC1766INFO) -> windows_core::HRESULT,
    pub CreateConvertCharset: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ConvertStringInIStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, u32, windows_core::PCWSTR, u32, u32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ConvertStringInIStream: usize,
    pub ConvertStringToUnicodeEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, u32, windows_core::PCSTR, *mut u32, windows_core::PWSTR, *mut u32, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub ConvertStringFromUnicodeEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, u32, windows_core::PCWSTR, *mut u32, windows_core::PSTR, *mut u32, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub DetectCodepageInIStream: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut core::ffi::c_void, *mut DetectEncodingInfo, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DetectCodepageInIStream: usize,
    pub DetectInputCodepage: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, windows_core::PCSTR, *mut i32, *mut DetectEncodingInfo, *mut i32) -> windows_core::HRESULT,
    pub ValidateCodePage: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::Foundation::HWND) -> windows_core::HRESULT,
    pub GetCodePageDescription: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, windows_core::PWSTR, i32) -> windows_core::HRESULT,
    pub IsCodePageInstallable: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetMimeDBSource: unsafe extern "system" fn(*mut core::ffi::c_void, MIMECONTF) -> windows_core::HRESULT,
    pub GetNumberOfScripts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub EnumScripts: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ValidateCodePageEx: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::Foundation::HWND, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMultiLanguage3, IMultiLanguage3_Vtbl, 0x4e5868ab_b157_4623_9acc_6a1d9caebe04);
impl core::ops::Deref for IMultiLanguage3 {
    type Target = IMultiLanguage2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMultiLanguage3, windows_core::IUnknown, IMultiLanguage2);
impl IMultiLanguage3 {
    pub unsafe fn DetectOutboundCodePage<P0>(&self, dwflags: u32, lpwidecharstr: &[u16], puipreferredcodepages: Option<&[u32]>, puidetectedcodepages: *mut u32, pndetectedcodepages: *mut u32, lpspecialchar: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).DetectOutboundCodePage)(windows_core::Interface::as_raw(self), dwflags, core::mem::transmute(lpwidecharstr.as_ptr()), lpwidecharstr.len().try_into().unwrap(), core::mem::transmute(puipreferredcodepages.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), puipreferredcodepages.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), puidetectedcodepages, pndetectedcodepages, lpspecialchar.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DetectOutboundCodePageInIStream<P0, P1>(&self, dwflags: u32, pstrin: P0, puipreferredcodepages: Option<&[u32]>, puidetectedcodepages: *mut u32, pndetectedcodepages: *mut u32, lpspecialchar: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::System::Com::IStream>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).DetectOutboundCodePageInIStream)(windows_core::Interface::as_raw(self), dwflags, pstrin.param().abi(), core::mem::transmute(puipreferredcodepages.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), puipreferredcodepages.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), puidetectedcodepages, pndetectedcodepages, lpspecialchar.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IMultiLanguage3_Vtbl {
    pub base__: IMultiLanguage2_Vtbl,
    pub DetectOutboundCodePage: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, u32, *const u32, u32, *mut u32, *mut u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub DetectOutboundCodePageInIStream: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *const u32, u32, *mut u32, *mut u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DetectOutboundCodePageInIStream: usize,
}
windows_core::imp::define_interface!(IOptionDescription, IOptionDescription_Vtbl, 0x432e5f85_35cf_4606_a801_6f70277e1d7a);
impl core::ops::Deref for IOptionDescription {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOptionDescription, windows_core::IUnknown);
impl IOptionDescription {
    pub unsafe fn Id(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Heading(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Heading)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Labels(&self) -> windows_core::Result<super::System::Com::IEnumString> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Labels)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IOptionDescription_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub Heading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Labels: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Labels: usize,
}
windows_core::imp::define_interface!(ISpellCheckProvider, ISpellCheckProvider_Vtbl, 0x73e976e0_8ed4_4eb1_80d7_1be0a16b0c38);
impl core::ops::Deref for ISpellCheckProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpellCheckProvider, windows_core::IUnknown);
impl ISpellCheckProvider {
    pub unsafe fn LanguageTag(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LanguageTag)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Check<P0>(&self, text: P0) -> windows_core::Result<IEnumSpellingError>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Check)(windows_core::Interface::as_raw(self), text.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Suggest<P0>(&self, word: P0) -> windows_core::Result<super::System::Com::IEnumString>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Suggest)(windows_core::Interface::as_raw(self), word.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetOptionValue<P0>(&self, optionid: P0) -> windows_core::Result<u8>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOptionValue)(windows_core::Interface::as_raw(self), optionid.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn SetOptionValue<P0>(&self, optionid: P0, value: u8) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetOptionValue)(windows_core::Interface::as_raw(self), optionid.param().abi(), value).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OptionIds(&self) -> windows_core::Result<super::System::Com::IEnumString> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OptionIds)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Id(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn LocalizedName(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LocalizedName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetOptionDescription<P0>(&self, optionid: P0) -> windows_core::Result<IOptionDescription>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOptionDescription)(windows_core::Interface::as_raw(self), optionid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeWordlist<P0>(&self, wordlisttype: WORDLIST_TYPE, words: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::System::Com::IEnumString>,
    {
        (windows_core::Interface::vtable(self).InitializeWordlist)(windows_core::Interface::as_raw(self), wordlisttype, words.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ISpellCheckProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub LanguageTag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub Check: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Suggest: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Suggest: usize,
    pub GetOptionValue: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u8) -> windows_core::HRESULT,
    pub SetOptionValue: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u8) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub OptionIds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OptionIds: usize,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub LocalizedName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetOptionDescription: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeWordlist: unsafe extern "system" fn(*mut core::ffi::c_void, WORDLIST_TYPE, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeWordlist: usize,
}
windows_core::imp::define_interface!(ISpellCheckProviderFactory, ISpellCheckProviderFactory_Vtbl, 0x9f671e11_77d6_4c92_aefb_615215e3a4be);
impl core::ops::Deref for ISpellCheckProviderFactory {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpellCheckProviderFactory, windows_core::IUnknown);
impl ISpellCheckProviderFactory {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedLanguages(&self) -> windows_core::Result<super::System::Com::IEnumString> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SupportedLanguages)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IsSupported<P0>(&self, languagetag: P0) -> windows_core::Result<super::Foundation::BOOL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsSupported)(windows_core::Interface::as_raw(self), languagetag.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn CreateSpellCheckProvider<P0>(&self, languagetag: P0) -> windows_core::Result<ISpellCheckProvider>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateSpellCheckProvider)(windows_core::Interface::as_raw(self), languagetag.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ISpellCheckProviderFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub SupportedLanguages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupportedLanguages: usize,
    pub IsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CreateSpellCheckProvider: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpellChecker, ISpellChecker_Vtbl, 0xb6fd0b71_e2bc_4653_8d05_f197e412770b);
impl core::ops::Deref for ISpellChecker {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpellChecker, windows_core::IUnknown);
impl ISpellChecker {
    pub unsafe fn LanguageTag(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LanguageTag)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Check<P0>(&self, text: P0) -> windows_core::Result<IEnumSpellingError>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Check)(windows_core::Interface::as_raw(self), text.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Suggest<P0>(&self, word: P0) -> windows_core::Result<super::System::Com::IEnumString>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Suggest)(windows_core::Interface::as_raw(self), word.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Add<P0>(&self, word: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), word.param().abi()).ok()
    }
    pub unsafe fn Ignore<P0>(&self, word: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Ignore)(windows_core::Interface::as_raw(self), word.param().abi()).ok()
    }
    pub unsafe fn AutoCorrect<P0, P1>(&self, from: P0, to: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AutoCorrect)(windows_core::Interface::as_raw(self), from.param().abi(), to.param().abi()).ok()
    }
    pub unsafe fn GetOptionValue<P0>(&self, optionid: P0) -> windows_core::Result<u8>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOptionValue)(windows_core::Interface::as_raw(self), optionid.param().abi(), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OptionIds(&self) -> windows_core::Result<super::System::Com::IEnumString> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OptionIds)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Id(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn LocalizedName(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LocalizedName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn add_SpellCheckerChanged<P0>(&self, handler: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<ISpellCheckerChangedEventHandler>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).add_SpellCheckerChanged)(windows_core::Interface::as_raw(self), handler.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn remove_SpellCheckerChanged(&self, eventcookie: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).remove_SpellCheckerChanged)(windows_core::Interface::as_raw(self), eventcookie).ok()
    }
    pub unsafe fn GetOptionDescription<P0>(&self, optionid: P0) -> windows_core::Result<IOptionDescription>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOptionDescription)(windows_core::Interface::as_raw(self), optionid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ComprehensiveCheck<P0>(&self, text: P0) -> windows_core::Result<IEnumSpellingError>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ComprehensiveCheck)(windows_core::Interface::as_raw(self), text.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ISpellChecker_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub LanguageTag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub Check: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Suggest: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Suggest: usize,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Ignore: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub AutoCorrect: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetOptionValue: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u8) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub OptionIds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OptionIds: usize,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub LocalizedName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub add_SpellCheckerChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub remove_SpellCheckerChanged: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetOptionDescription: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ComprehensiveCheck: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpellChecker2, ISpellChecker2_Vtbl, 0xe7ed1c71_87f7_4378_a840_c9200dacee47);
impl core::ops::Deref for ISpellChecker2 {
    type Target = ISpellChecker;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpellChecker2, windows_core::IUnknown, ISpellChecker);
impl ISpellChecker2 {
    pub unsafe fn Remove<P0>(&self, word: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), word.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ISpellChecker2_Vtbl {
    pub base__: ISpellChecker_Vtbl,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpellCheckerChangedEventHandler, ISpellCheckerChangedEventHandler_Vtbl, 0x0b83a5b0_792f_4eab_9799_acf52c5ed08a);
impl core::ops::Deref for ISpellCheckerChangedEventHandler {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpellCheckerChangedEventHandler, windows_core::IUnknown);
impl ISpellCheckerChangedEventHandler {
    pub unsafe fn Invoke<P0>(&self, sender: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISpellChecker>,
    {
        (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), sender.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ISpellCheckerChangedEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpellCheckerFactory, ISpellCheckerFactory_Vtbl, 0x8e018a9d_2415_4677_bf08_794ea61f94bb);
impl core::ops::Deref for ISpellCheckerFactory {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpellCheckerFactory, windows_core::IUnknown);
impl ISpellCheckerFactory {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedLanguages(&self) -> windows_core::Result<super::System::Com::IEnumString> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SupportedLanguages)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IsSupported<P0>(&self, languagetag: P0) -> windows_core::Result<super::Foundation::BOOL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsSupported)(windows_core::Interface::as_raw(self), languagetag.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn CreateSpellChecker<P0>(&self, languagetag: P0) -> windows_core::Result<ISpellChecker>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateSpellChecker)(windows_core::Interface::as_raw(self), languagetag.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ISpellCheckerFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub SupportedLanguages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupportedLanguages: usize,
    pub IsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CreateSpellChecker: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpellingError, ISpellingError_Vtbl, 0xb7c82d61_fbe8_4b47_9b27_6c0d2e0de0a3);
impl core::ops::Deref for ISpellingError {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpellingError, windows_core::IUnknown);
impl ISpellingError {
    pub unsafe fn StartIndex(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StartIndex)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Length(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Length)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CorrectiveAction(&self) -> windows_core::Result<CORRECTIVE_ACTION> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CorrectiveAction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Replacement(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Replacement)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ISpellingError_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub StartIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Length: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub CorrectiveAction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CORRECTIVE_ACTION) -> windows_core::HRESULT,
    pub Replacement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUserDictionariesRegistrar, IUserDictionariesRegistrar_Vtbl, 0xaa176b85_0e12_4844_8e1a_eef1da77f586);
impl core::ops::Deref for IUserDictionariesRegistrar {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUserDictionariesRegistrar, windows_core::IUnknown);
impl IUserDictionariesRegistrar {
    pub unsafe fn RegisterUserDictionary<P0, P1>(&self, dictionarypath: P0, languagetag: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).RegisterUserDictionary)(windows_core::Interface::as_raw(self), dictionarypath.param().abi(), languagetag.param().abi()).ok()
    }
    pub unsafe fn UnregisterUserDictionary<P0, P1>(&self, dictionarypath: P0, languagetag: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).UnregisterUserDictionary)(windows_core::Interface::as_raw(self), dictionarypath.param().abi(), languagetag.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IUserDictionariesRegistrar_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterUserDictionary: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub UnregisterUserDictionary: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub const ALL_SERVICES: u32 = 0u32;
pub const ALL_SERVICE_TYPES: u32 = 0u32;
pub const C1_ALPHA: u32 = 256u32;
pub const C1_BLANK: u32 = 64u32;
pub const C1_CNTRL: u32 = 32u32;
pub const C1_DEFINED: u32 = 512u32;
pub const C1_DIGIT: u32 = 4u32;
pub const C1_LOWER: u32 = 2u32;
pub const C1_PUNCT: u32 = 16u32;
pub const C1_SPACE: u32 = 8u32;
pub const C1_UPPER: u32 = 1u32;
pub const C1_XDIGIT: u32 = 128u32;
pub const C2_ARABICNUMBER: u32 = 6u32;
pub const C2_BLOCKSEPARATOR: u32 = 8u32;
pub const C2_COMMONSEPARATOR: u32 = 7u32;
pub const C2_EUROPENUMBER: u32 = 3u32;
pub const C2_EUROPESEPARATOR: u32 = 4u32;
pub const C2_EUROPETERMINATOR: u32 = 5u32;
pub const C2_LEFTTORIGHT: u32 = 1u32;
pub const C2_NOTAPPLICABLE: u32 = 0u32;
pub const C2_OTHERNEUTRAL: u32 = 11u32;
pub const C2_RIGHTTOLEFT: u32 = 2u32;
pub const C2_SEGMENTSEPARATOR: u32 = 9u32;
pub const C2_WHITESPACE: u32 = 10u32;
pub const C3_ALPHA: u32 = 32768u32;
pub const C3_DIACRITIC: u32 = 2u32;
pub const C3_FULLWIDTH: u32 = 128u32;
pub const C3_HALFWIDTH: u32 = 64u32;
pub const C3_HIGHSURROGATE: u32 = 2048u32;
pub const C3_HIRAGANA: u32 = 32u32;
pub const C3_IDEOGRAPH: u32 = 256u32;
pub const C3_KASHIDA: u32 = 512u32;
pub const C3_KATAKANA: u32 = 16u32;
pub const C3_LEXICAL: u32 = 1024u32;
pub const C3_LOWSURROGATE: u32 = 4096u32;
pub const C3_NONSPACING: u32 = 1u32;
pub const C3_NOTAPPLICABLE: u32 = 0u32;
pub const C3_SYMBOL: u32 = 8u32;
pub const C3_VOWELMARK: u32 = 4u32;
pub const CAL_GREGORIAN: u32 = 1u32;
pub const CAL_GREGORIAN_ARABIC: u32 = 10u32;
pub const CAL_GREGORIAN_ME_FRENCH: u32 = 9u32;
pub const CAL_GREGORIAN_US: u32 = 2u32;
pub const CAL_GREGORIAN_XLIT_ENGLISH: u32 = 11u32;
pub const CAL_GREGORIAN_XLIT_FRENCH: u32 = 12u32;
pub const CAL_HEBREW: u32 = 8u32;
pub const CAL_HIJRI: u32 = 6u32;
pub const CAL_ICALINTVALUE: u32 = 1u32;
pub const CAL_ITWODIGITYEARMAX: u32 = 48u32;
pub const CAL_IYEAROFFSETRANGE: u32 = 3u32;
pub const CAL_JAPAN: u32 = 3u32;
pub const CAL_KOREA: u32 = 5u32;
pub const CAL_NOUSEROVERRIDE: u32 = 2147483648u32;
pub const CAL_PERSIAN: u32 = 22u32;
pub const CAL_RETURN_GENITIVE_NAMES: u32 = 268435456u32;
pub const CAL_RETURN_NUMBER: u32 = 536870912u32;
pub const CAL_SABBREVDAYNAME1: u32 = 14u32;
pub const CAL_SABBREVDAYNAME2: u32 = 15u32;
pub const CAL_SABBREVDAYNAME3: u32 = 16u32;
pub const CAL_SABBREVDAYNAME4: u32 = 17u32;
pub const CAL_SABBREVDAYNAME5: u32 = 18u32;
pub const CAL_SABBREVDAYNAME6: u32 = 19u32;
pub const CAL_SABBREVDAYNAME7: u32 = 20u32;
pub const CAL_SABBREVERASTRING: u32 = 57u32;
pub const CAL_SABBREVMONTHNAME1: u32 = 34u32;
pub const CAL_SABBREVMONTHNAME10: u32 = 43u32;
pub const CAL_SABBREVMONTHNAME11: u32 = 44u32;
pub const CAL_SABBREVMONTHNAME12: u32 = 45u32;
pub const CAL_SABBREVMONTHNAME13: u32 = 46u32;
pub const CAL_SABBREVMONTHNAME2: u32 = 35u32;
pub const CAL_SABBREVMONTHNAME3: u32 = 36u32;
pub const CAL_SABBREVMONTHNAME4: u32 = 37u32;
pub const CAL_SABBREVMONTHNAME5: u32 = 38u32;
pub const CAL_SABBREVMONTHNAME6: u32 = 39u32;
pub const CAL_SABBREVMONTHNAME7: u32 = 40u32;
pub const CAL_SABBREVMONTHNAME8: u32 = 41u32;
pub const CAL_SABBREVMONTHNAME9: u32 = 42u32;
pub const CAL_SCALNAME: u32 = 2u32;
pub const CAL_SDAYNAME1: u32 = 7u32;
pub const CAL_SDAYNAME2: u32 = 8u32;
pub const CAL_SDAYNAME3: u32 = 9u32;
pub const CAL_SDAYNAME4: u32 = 10u32;
pub const CAL_SDAYNAME5: u32 = 11u32;
pub const CAL_SDAYNAME6: u32 = 12u32;
pub const CAL_SDAYNAME7: u32 = 13u32;
pub const CAL_SENGLISHABBREVERANAME: u32 = 60u32;
pub const CAL_SENGLISHERANAME: u32 = 59u32;
pub const CAL_SERASTRING: u32 = 4u32;
pub const CAL_SJAPANESEERAFIRSTYEAR: u32 = 61u32;
pub const CAL_SLONGDATE: u32 = 6u32;
pub const CAL_SMONTHDAY: u32 = 56u32;
pub const CAL_SMONTHNAME1: u32 = 21u32;
pub const CAL_SMONTHNAME10: u32 = 30u32;
pub const CAL_SMONTHNAME11: u32 = 31u32;
pub const CAL_SMONTHNAME12: u32 = 32u32;
pub const CAL_SMONTHNAME13: u32 = 33u32;
pub const CAL_SMONTHNAME2: u32 = 22u32;
pub const CAL_SMONTHNAME3: u32 = 23u32;
pub const CAL_SMONTHNAME4: u32 = 24u32;
pub const CAL_SMONTHNAME5: u32 = 25u32;
pub const CAL_SMONTHNAME6: u32 = 26u32;
pub const CAL_SMONTHNAME7: u32 = 27u32;
pub const CAL_SMONTHNAME8: u32 = 28u32;
pub const CAL_SMONTHNAME9: u32 = 29u32;
pub const CAL_SRELATIVELONGDATE: u32 = 58u32;
pub const CAL_SSHORTDATE: u32 = 5u32;
pub const CAL_SSHORTESTDAYNAME1: u32 = 49u32;
pub const CAL_SSHORTESTDAYNAME2: u32 = 50u32;
pub const CAL_SSHORTESTDAYNAME3: u32 = 51u32;
pub const CAL_SSHORTESTDAYNAME4: u32 = 52u32;
pub const CAL_SSHORTESTDAYNAME5: u32 = 53u32;
pub const CAL_SSHORTESTDAYNAME6: u32 = 54u32;
pub const CAL_SSHORTESTDAYNAME7: u32 = 55u32;
pub const CAL_SYEARMONTH: u32 = 47u32;
pub const CAL_TAIWAN: u32 = 4u32;
pub const CAL_THAI: u32 = 7u32;
pub const CAL_UMALQURA: u32 = 23u32;
pub const CAL_USE_CP_ACP: u32 = 1073741824u32;
pub const CANITER_SKIP_ZEROES: u32 = 1u32;
pub const COMPARE_STRING: SYSNLS_FUNCTION = SYSNLS_FUNCTION(1i32);
pub const CORRECTIVE_ACTION_DELETE: CORRECTIVE_ACTION = CORRECTIVE_ACTION(3i32);
pub const CORRECTIVE_ACTION_GET_SUGGESTIONS: CORRECTIVE_ACTION = CORRECTIVE_ACTION(1i32);
pub const CORRECTIVE_ACTION_NONE: CORRECTIVE_ACTION = CORRECTIVE_ACTION(0i32);
pub const CORRECTIVE_ACTION_REPLACE: CORRECTIVE_ACTION = CORRECTIVE_ACTION(2i32);
pub const CPIOD_FORCE_PROMPT: i32 = -2147483648i32;
pub const CPIOD_PEEK: i32 = 1073741824i32;
pub const CP_ACP: u32 = 0u32;
pub const CP_INSTALLED: ENUM_SYSTEM_CODE_PAGES_FLAGS = ENUM_SYSTEM_CODE_PAGES_FLAGS(1u32);
pub const CP_MACCP: u32 = 2u32;
pub const CP_OEMCP: u32 = 1u32;
pub const CP_SUPPORTED: ENUM_SYSTEM_CODE_PAGES_FLAGS = ENUM_SYSTEM_CODE_PAGES_FLAGS(2u32);
pub const CP_SYMBOL: u32 = 42u32;
pub const CP_THREAD_ACP: u32 = 3u32;
pub const CP_UTF7: u32 = 65000u32;
pub const CP_UTF8: u32 = 65001u32;
pub const CSTR_EQUAL: COMPARESTRING_RESULT = COMPARESTRING_RESULT(2i32);
pub const CSTR_GREATER_THAN: COMPARESTRING_RESULT = COMPARESTRING_RESULT(3i32);
pub const CSTR_LESS_THAN: COMPARESTRING_RESULT = COMPARESTRING_RESULT(1i32);
pub const CTRY_ALBANIA: u32 = 355u32;
pub const CTRY_ALGERIA: u32 = 213u32;
pub const CTRY_ARGENTINA: u32 = 54u32;
pub const CTRY_ARMENIA: u32 = 374u32;
pub const CTRY_AUSTRALIA: u32 = 61u32;
pub const CTRY_AUSTRIA: u32 = 43u32;
pub const CTRY_AZERBAIJAN: u32 = 994u32;
pub const CTRY_BAHRAIN: u32 = 973u32;
pub const CTRY_BELARUS: u32 = 375u32;
pub const CTRY_BELGIUM: u32 = 32u32;
pub const CTRY_BELIZE: u32 = 501u32;
pub const CTRY_BOLIVIA: u32 = 591u32;
pub const CTRY_BRAZIL: u32 = 55u32;
pub const CTRY_BRUNEI_DARUSSALAM: u32 = 673u32;
pub const CTRY_BULGARIA: u32 = 359u32;
pub const CTRY_CANADA: u32 = 2u32;
pub const CTRY_CARIBBEAN: u32 = 1u32;
pub const CTRY_CHILE: u32 = 56u32;
pub const CTRY_COLOMBIA: u32 = 57u32;
pub const CTRY_COSTA_RICA: u32 = 506u32;
pub const CTRY_CROATIA: u32 = 385u32;
pub const CTRY_CZECH: u32 = 420u32;
pub const CTRY_DEFAULT: u32 = 0u32;
pub const CTRY_DENMARK: u32 = 45u32;
pub const CTRY_DOMINICAN_REPUBLIC: u32 = 1u32;
pub const CTRY_ECUADOR: u32 = 593u32;
pub const CTRY_EGYPT: u32 = 20u32;
pub const CTRY_EL_SALVADOR: u32 = 503u32;
pub const CTRY_ESTONIA: u32 = 372u32;
pub const CTRY_FAEROE_ISLANDS: u32 = 298u32;
pub const CTRY_FINLAND: u32 = 358u32;
pub const CTRY_FRANCE: u32 = 33u32;
pub const CTRY_GEORGIA: u32 = 995u32;
pub const CTRY_GERMANY: u32 = 49u32;
pub const CTRY_GREECE: u32 = 30u32;
pub const CTRY_GUATEMALA: u32 = 502u32;
pub const CTRY_HONDURAS: u32 = 504u32;
pub const CTRY_HONG_KONG: u32 = 852u32;
pub const CTRY_HUNGARY: u32 = 36u32;
pub const CTRY_ICELAND: u32 = 354u32;
pub const CTRY_INDIA: u32 = 91u32;
pub const CTRY_INDONESIA: u32 = 62u32;
pub const CTRY_IRAN: u32 = 981u32;
pub const CTRY_IRAQ: u32 = 964u32;
pub const CTRY_IRELAND: u32 = 353u32;
pub const CTRY_ISRAEL: u32 = 972u32;
pub const CTRY_ITALY: u32 = 39u32;
pub const CTRY_JAMAICA: u32 = 1u32;
pub const CTRY_JAPAN: u32 = 81u32;
pub const CTRY_JORDAN: u32 = 962u32;
pub const CTRY_KAZAKSTAN: u32 = 7u32;
pub const CTRY_KENYA: u32 = 254u32;
pub const CTRY_KUWAIT: u32 = 965u32;
pub const CTRY_KYRGYZSTAN: u32 = 996u32;
pub const CTRY_LATVIA: u32 = 371u32;
pub const CTRY_LEBANON: u32 = 961u32;
pub const CTRY_LIBYA: u32 = 218u32;
pub const CTRY_LIECHTENSTEIN: u32 = 41u32;
pub const CTRY_LITHUANIA: u32 = 370u32;
pub const CTRY_LUXEMBOURG: u32 = 352u32;
pub const CTRY_MACAU: u32 = 853u32;
pub const CTRY_MACEDONIA: u32 = 389u32;
pub const CTRY_MALAYSIA: u32 = 60u32;
pub const CTRY_MALDIVES: u32 = 960u32;
pub const CTRY_MEXICO: u32 = 52u32;
pub const CTRY_MONACO: u32 = 33u32;
pub const CTRY_MONGOLIA: u32 = 976u32;
pub const CTRY_MOROCCO: u32 = 212u32;
pub const CTRY_NETHERLANDS: u32 = 31u32;
pub const CTRY_NEW_ZEALAND: u32 = 64u32;
pub const CTRY_NICARAGUA: u32 = 505u32;
pub const CTRY_NORWAY: u32 = 47u32;
pub const CTRY_OMAN: u32 = 968u32;
pub const CTRY_PAKISTAN: u32 = 92u32;
pub const CTRY_PANAMA: u32 = 507u32;
pub const CTRY_PARAGUAY: u32 = 595u32;
pub const CTRY_PERU: u32 = 51u32;
pub const CTRY_PHILIPPINES: u32 = 63u32;
pub const CTRY_POLAND: u32 = 48u32;
pub const CTRY_PORTUGAL: u32 = 351u32;
pub const CTRY_PRCHINA: u32 = 86u32;
pub const CTRY_PUERTO_RICO: u32 = 1u32;
pub const CTRY_QATAR: u32 = 974u32;
pub const CTRY_ROMANIA: u32 = 40u32;
pub const CTRY_RUSSIA: u32 = 7u32;
pub const CTRY_SAUDI_ARABIA: u32 = 966u32;
pub const CTRY_SERBIA: u32 = 381u32;
pub const CTRY_SINGAPORE: u32 = 65u32;
pub const CTRY_SLOVAK: u32 = 421u32;
pub const CTRY_SLOVENIA: u32 = 386u32;
pub const CTRY_SOUTH_AFRICA: u32 = 27u32;
pub const CTRY_SOUTH_KOREA: u32 = 82u32;
pub const CTRY_SPAIN: u32 = 34u32;
pub const CTRY_SWEDEN: u32 = 46u32;
pub const CTRY_SWITZERLAND: u32 = 41u32;
pub const CTRY_SYRIA: u32 = 963u32;
pub const CTRY_TAIWAN: u32 = 886u32;
pub const CTRY_TATARSTAN: u32 = 7u32;
pub const CTRY_THAILAND: u32 = 66u32;
pub const CTRY_TRINIDAD_Y_TOBAGO: u32 = 1u32;
pub const CTRY_TUNISIA: u32 = 216u32;
pub const CTRY_TURKEY: u32 = 90u32;
pub const CTRY_UAE: u32 = 971u32;
pub const CTRY_UKRAINE: u32 = 380u32;
pub const CTRY_UNITED_KINGDOM: u32 = 44u32;
pub const CTRY_UNITED_STATES: u32 = 1u32;
pub const CTRY_URUGUAY: u32 = 598u32;
pub const CTRY_UZBEKISTAN: u32 = 7u32;
pub const CTRY_VENEZUELA: u32 = 58u32;
pub const CTRY_VIET_NAM: u32 = 84u32;
pub const CTRY_YEMEN: u32 = 967u32;
pub const CTRY_ZIMBABWE: u32 = 263u32;
pub const CT_CTYPE1: u32 = 1u32;
pub const CT_CTYPE2: u32 = 2u32;
pub const CT_CTYPE3: u32 = 4u32;
pub const DATE_AUTOLAYOUT: ENUM_DATE_FORMATS_FLAGS = ENUM_DATE_FORMATS_FLAGS(64u32);
pub const DATE_LONGDATE: ENUM_DATE_FORMATS_FLAGS = ENUM_DATE_FORMATS_FLAGS(2u32);
pub const DATE_LTRREADING: ENUM_DATE_FORMATS_FLAGS = ENUM_DATE_FORMATS_FLAGS(16u32);
pub const DATE_MONTHDAY: ENUM_DATE_FORMATS_FLAGS = ENUM_DATE_FORMATS_FLAGS(128u32);
pub const DATE_RTLREADING: ENUM_DATE_FORMATS_FLAGS = ENUM_DATE_FORMATS_FLAGS(32u32);
pub const DATE_SHORTDATE: ENUM_DATE_FORMATS_FLAGS = ENUM_DATE_FORMATS_FLAGS(1u32);
pub const DATE_USE_ALT_CALENDAR: ENUM_DATE_FORMATS_FLAGS = ENUM_DATE_FORMATS_FLAGS(4u32);
pub const DATE_YEARMONTH: ENUM_DATE_FORMATS_FLAGS = ENUM_DATE_FORMATS_FLAGS(8u32);
pub const DayUnit: CALDATETIME_DATEUNIT = CALDATETIME_DATEUNIT(4i32);
pub const ELS_GUID_LANGUAGE_DETECTION: windows_core::GUID = windows_core::GUID::from_u128(0xcf7e00b1_909b_4d95_a8f4_611f7c377702);
pub const ELS_GUID_SCRIPT_DETECTION: windows_core::GUID = windows_core::GUID::from_u128(0x2d64b439_6caf_4f6b_b688_e5d0f4faa7d7);
pub const ELS_GUID_TRANSLITERATION_BENGALI_TO_LATIN: windows_core::GUID = windows_core::GUID::from_u128(0xf4dfd825_91a4_489f_855e_9ad9bee55727);
pub const ELS_GUID_TRANSLITERATION_CYRILLIC_TO_LATIN: windows_core::GUID = windows_core::GUID::from_u128(0x3dd12a98_5afd_4903_a13f_e17e6c0bfe01);
pub const ELS_GUID_TRANSLITERATION_DEVANAGARI_TO_LATIN: windows_core::GUID = windows_core::GUID::from_u128(0xc4a4dcfe_2661_4d02_9835_f48187109803);
pub const ELS_GUID_TRANSLITERATION_HANGUL_DECOMPOSITION: windows_core::GUID = windows_core::GUID::from_u128(0x4ba2a721_e43d_41b7_b330_536ae1e48863);
pub const ELS_GUID_TRANSLITERATION_HANS_TO_HANT: windows_core::GUID = windows_core::GUID::from_u128(0x3caccdc8_5590_42dc_9a7b_b5a6b5b3b63b);
pub const ELS_GUID_TRANSLITERATION_HANT_TO_HANS: windows_core::GUID = windows_core::GUID::from_u128(0xa3a8333b_f4fc_42f6_a0c4_0462fe7317cb);
pub const ELS_GUID_TRANSLITERATION_MALAYALAM_TO_LATIN: windows_core::GUID = windows_core::GUID::from_u128(0xd8b983b1_f8bf_4a2b_bcd5_5b5ea20613e1);
pub const ENUM_ALL_CALENDARS: u32 = 4294967295u32;
pub const EraUnit: CALDATETIME_DATEUNIT = CALDATETIME_DATEUNIT(0i32);
pub const FIND_ENDSWITH: u32 = 2097152u32;
pub const FIND_FROMEND: u32 = 8388608u32;
pub const FIND_FROMSTART: u32 = 4194304u32;
pub const FIND_STARTSWITH: u32 = 1048576u32;
pub const GEOCLASS_ALL: SYSGEOCLASS = SYSGEOCLASS(0i32);
pub const GEOCLASS_NATION: SYSGEOCLASS = SYSGEOCLASS(16i32);
pub const GEOCLASS_REGION: SYSGEOCLASS = SYSGEOCLASS(14i32);
pub const GEOID_NOT_AVAILABLE: i32 = -1i32;
pub const GEO_CURRENCYCODE: SYSGEOTYPE = SYSGEOTYPE(15i32);
pub const GEO_CURRENCYSYMBOL: SYSGEOTYPE = SYSGEOTYPE(16i32);
pub const GEO_DIALINGCODE: SYSGEOTYPE = SYSGEOTYPE(14i32);
pub const GEO_FRIENDLYNAME: SYSGEOTYPE = SYSGEOTYPE(8i32);
pub const GEO_ID: SYSGEOTYPE = SYSGEOTYPE(18i32);
pub const GEO_ISO2: SYSGEOTYPE = SYSGEOTYPE(4i32);
pub const GEO_ISO3: SYSGEOTYPE = SYSGEOTYPE(5i32);
pub const GEO_ISO_UN_NUMBER: SYSGEOTYPE = SYSGEOTYPE(12i32);
pub const GEO_LATITUDE: SYSGEOTYPE = SYSGEOTYPE(2i32);
pub const GEO_LCID: SYSGEOTYPE = SYSGEOTYPE(7i32);
pub const GEO_LONGITUDE: SYSGEOTYPE = SYSGEOTYPE(3i32);
pub const GEO_NAME: SYSGEOTYPE = SYSGEOTYPE(17i32);
pub const GEO_NATION: SYSGEOTYPE = SYSGEOTYPE(1i32);
pub const GEO_OFFICIALLANGUAGES: SYSGEOTYPE = SYSGEOTYPE(11i32);
pub const GEO_OFFICIALNAME: SYSGEOTYPE = SYSGEOTYPE(9i32);
pub const GEO_PARENT: SYSGEOTYPE = SYSGEOTYPE(13i32);
pub const GEO_RFC1766: SYSGEOTYPE = SYSGEOTYPE(6i32);
pub const GEO_TIMEZONES: SYSGEOTYPE = SYSGEOTYPE(10i32);
pub const GSS_ALLOW_INHERITED_COMMON: u32 = 1u32;
pub const HIGHLEVEL_SERVICE_TYPES: u32 = 1u32;
pub const HIGH_SURROGATE_END: u32 = 56319u32;
pub const HIGH_SURROGATE_START: u32 = 55296u32;
pub const HourUnit: CALDATETIME_DATEUNIT = CALDATETIME_DATEUNIT(5i32);
pub const IDN_ALLOW_UNASSIGNED: u32 = 1u32;
pub const IDN_EMAIL_ADDRESS: u32 = 4u32;
pub const IDN_RAW_PUNYCODE: u32 = 8u32;
pub const IDN_USE_STD3_ASCII_RULES: u32 = 2u32;
pub const IS_TEXT_UNICODE_ASCII16: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(1u32);
pub const IS_TEXT_UNICODE_CONTROLS: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(4u32);
pub const IS_TEXT_UNICODE_ILLEGAL_CHARS: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(256u32);
pub const IS_TEXT_UNICODE_NOT_ASCII_MASK: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(61440u32);
pub const IS_TEXT_UNICODE_NOT_UNICODE_MASK: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(3840u32);
pub const IS_TEXT_UNICODE_NULL_BYTES: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(4096u32);
pub const IS_TEXT_UNICODE_ODD_LENGTH: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(512u32);
pub const IS_TEXT_UNICODE_REVERSE_ASCII16: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(16u32);
pub const IS_TEXT_UNICODE_REVERSE_CONTROLS: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(64u32);
pub const IS_TEXT_UNICODE_REVERSE_MASK: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(240u32);
pub const IS_TEXT_UNICODE_REVERSE_SIGNATURE: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(128u32);
pub const IS_TEXT_UNICODE_REVERSE_STATISTICS: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(32u32);
pub const IS_TEXT_UNICODE_SIGNATURE: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(8u32);
pub const IS_TEXT_UNICODE_STATISTICS: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(2u32);
pub const IS_TEXT_UNICODE_UNICODE_MASK: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(15u32);
pub const LCID_ALTERNATE_SORTS: u32 = 4u32;
pub const LCID_INSTALLED: IS_VALID_LOCALE_FLAGS = IS_VALID_LOCALE_FLAGS(1u32);
pub const LCID_SUPPORTED: IS_VALID_LOCALE_FLAGS = IS_VALID_LOCALE_FLAGS(2u32);
pub const LCMAP_BYTEREV: u32 = 2048u32;
pub const LCMAP_FULLWIDTH: u32 = 8388608u32;
pub const LCMAP_HALFWIDTH: u32 = 4194304u32;
pub const LCMAP_HASH: u32 = 262144u32;
pub const LCMAP_HIRAGANA: u32 = 1048576u32;
pub const LCMAP_KATAKANA: u32 = 2097152u32;
pub const LCMAP_LINGUISTIC_CASING: u32 = 16777216u32;
pub const LCMAP_LOWERCASE: u32 = 256u32;
pub const LCMAP_SIMPLIFIED_CHINESE: u32 = 33554432u32;
pub const LCMAP_SORTHANDLE: u32 = 536870912u32;
pub const LCMAP_SORTKEY: u32 = 1024u32;
pub const LCMAP_TITLECASE: u32 = 768u32;
pub const LCMAP_TRADITIONAL_CHINESE: u32 = 67108864u32;
pub const LCMAP_UPPERCASE: u32 = 512u32;
pub const LGRPID_ARABIC: u32 = 13u32;
pub const LGRPID_ARMENIAN: u32 = 17u32;
pub const LGRPID_BALTIC: u32 = 3u32;
pub const LGRPID_CENTRAL_EUROPE: u32 = 2u32;
pub const LGRPID_CYRILLIC: u32 = 5u32;
pub const LGRPID_GEORGIAN: u32 = 16u32;
pub const LGRPID_GREEK: u32 = 4u32;
pub const LGRPID_HEBREW: u32 = 12u32;
pub const LGRPID_INDIC: u32 = 15u32;
pub const LGRPID_INSTALLED: ENUM_SYSTEM_LANGUAGE_GROUPS_FLAGS = ENUM_SYSTEM_LANGUAGE_GROUPS_FLAGS(1u32);
pub const LGRPID_JAPANESE: u32 = 7u32;
pub const LGRPID_KOREAN: u32 = 8u32;
pub const LGRPID_SIMPLIFIED_CHINESE: u32 = 10u32;
pub const LGRPID_SUPPORTED: ENUM_SYSTEM_LANGUAGE_GROUPS_FLAGS = ENUM_SYSTEM_LANGUAGE_GROUPS_FLAGS(2u32);
pub const LGRPID_THAI: u32 = 11u32;
pub const LGRPID_TRADITIONAL_CHINESE: u32 = 9u32;
pub const LGRPID_TURKIC: u32 = 6u32;
pub const LGRPID_TURKISH: u32 = 6u32;
pub const LGRPID_VIETNAMESE: u32 = 14u32;
pub const LGRPID_WESTERN_EUROPE: u32 = 1u32;
pub const LINGUISTIC_IGNORECASE: COMPARE_STRING_FLAGS = COMPARE_STRING_FLAGS(16u32);
pub const LINGUISTIC_IGNOREDIACRITIC: COMPARE_STRING_FLAGS = COMPARE_STRING_FLAGS(32u32);
pub const LOCALE_ALL: u32 = 0u32;
pub const LOCALE_ALLOW_NEUTRAL_NAMES: u32 = 134217728u32;
pub const LOCALE_ALTERNATE_SORTS: u32 = 4u32;
pub const LOCALE_FONTSIGNATURE: u32 = 88u32;
pub const LOCALE_ICALENDARTYPE: u32 = 4105u32;
pub const LOCALE_ICENTURY: u32 = 36u32;
pub const LOCALE_ICONSTRUCTEDLOCALE: u32 = 125u32;
pub const LOCALE_ICOUNTRY: u32 = 5u32;
pub const LOCALE_ICURRDIGITS: u32 = 25u32;
pub const LOCALE_ICURRENCY: u32 = 27u32;
pub const LOCALE_IDATE: u32 = 33u32;
pub const LOCALE_IDAYLZERO: u32 = 38u32;
pub const LOCALE_IDEFAULTANSICODEPAGE: u32 = 4100u32;
pub const LOCALE_IDEFAULTCODEPAGE: u32 = 11u32;
pub const LOCALE_IDEFAULTCOUNTRY: u32 = 10u32;
pub const LOCALE_IDEFAULTEBCDICCODEPAGE: u32 = 4114u32;
pub const LOCALE_IDEFAULTLANGUAGE: u32 = 9u32;
pub const LOCALE_IDEFAULTMACCODEPAGE: u32 = 4113u32;
pub const LOCALE_IDIALINGCODE: u32 = 5u32;
pub const LOCALE_IDIGITS: u32 = 17u32;
pub const LOCALE_IDIGITSUBSTITUTION: u32 = 4116u32;
pub const LOCALE_IFIRSTDAYOFWEEK: u32 = 4108u32;
pub const LOCALE_IFIRSTWEEKOFYEAR: u32 = 4109u32;
pub const LOCALE_IGEOID: u32 = 91u32;
pub const LOCALE_IINTLCURRDIGITS: u32 = 26u32;
pub const LOCALE_ILANGUAGE: u32 = 1u32;
pub const LOCALE_ILDATE: u32 = 34u32;
pub const LOCALE_ILZERO: u32 = 18u32;
pub const LOCALE_IMEASURE: u32 = 13u32;
pub const LOCALE_IMONLZERO: u32 = 39u32;
pub const LOCALE_INEGATIVEPERCENT: u32 = 116u32;
pub const LOCALE_INEGCURR: u32 = 28u32;
pub const LOCALE_INEGNUMBER: u32 = 4112u32;
pub const LOCALE_INEGSEPBYSPACE: u32 = 87u32;
pub const LOCALE_INEGSIGNPOSN: u32 = 83u32;
pub const LOCALE_INEGSYMPRECEDES: u32 = 86u32;
pub const LOCALE_INEUTRAL: u32 = 113u32;
pub const LOCALE_IOPTIONALCALENDAR: u32 = 4107u32;
pub const LOCALE_IPAPERSIZE: u32 = 4106u32;
pub const LOCALE_IPOSITIVEPERCENT: u32 = 117u32;
pub const LOCALE_IPOSSEPBYSPACE: u32 = 85u32;
pub const LOCALE_IPOSSIGNPOSN: u32 = 82u32;
pub const LOCALE_IPOSSYMPRECEDES: u32 = 84u32;
pub const LOCALE_IREADINGLAYOUT: u32 = 112u32;
pub const LOCALE_ITIME: u32 = 35u32;
pub const LOCALE_ITIMEMARKPOSN: u32 = 4101u32;
pub const LOCALE_ITLZERO: u32 = 37u32;
pub const LOCALE_IUSEUTF8LEGACYACP: u32 = 1638u32;
pub const LOCALE_IUSEUTF8LEGACYOEMCP: u32 = 2457u32;
pub const LOCALE_NAME_INVARIANT: windows_core::PCWSTR = windows_core::w!("");
pub const LOCALE_NAME_SYSTEM_DEFAULT: windows_core::PCWSTR = windows_core::w!("!x-sys-default-locale");
pub const LOCALE_NEUTRALDATA: u32 = 16u32;
pub const LOCALE_NOUSEROVERRIDE: u32 = 2147483648u32;
pub const LOCALE_REPLACEMENT: u32 = 8u32;
pub const LOCALE_RETURN_GENITIVE_NAMES: u32 = 268435456u32;
pub const LOCALE_RETURN_NUMBER: u32 = 536870912u32;
pub const LOCALE_S1159: u32 = 40u32;
pub const LOCALE_S2359: u32 = 41u32;
pub const LOCALE_SABBREVCTRYNAME: u32 = 7u32;
pub const LOCALE_SABBREVDAYNAME1: u32 = 49u32;
pub const LOCALE_SABBREVDAYNAME2: u32 = 50u32;
pub const LOCALE_SABBREVDAYNAME3: u32 = 51u32;
pub const LOCALE_SABBREVDAYNAME4: u32 = 52u32;
pub const LOCALE_SABBREVDAYNAME5: u32 = 53u32;
pub const LOCALE_SABBREVDAYNAME6: u32 = 54u32;
pub const LOCALE_SABBREVDAYNAME7: u32 = 55u32;
pub const LOCALE_SABBREVLANGNAME: u32 = 3u32;
pub const LOCALE_SABBREVMONTHNAME1: u32 = 68u32;
pub const LOCALE_SABBREVMONTHNAME10: u32 = 77u32;
pub const LOCALE_SABBREVMONTHNAME11: u32 = 78u32;
pub const LOCALE_SABBREVMONTHNAME12: u32 = 79u32;
pub const LOCALE_SABBREVMONTHNAME13: u32 = 4111u32;
pub const LOCALE_SABBREVMONTHNAME2: u32 = 69u32;
pub const LOCALE_SABBREVMONTHNAME3: u32 = 70u32;
pub const LOCALE_SABBREVMONTHNAME4: u32 = 71u32;
pub const LOCALE_SABBREVMONTHNAME5: u32 = 72u32;
pub const LOCALE_SABBREVMONTHNAME6: u32 = 73u32;
pub const LOCALE_SABBREVMONTHNAME7: u32 = 74u32;
pub const LOCALE_SABBREVMONTHNAME8: u32 = 75u32;
pub const LOCALE_SABBREVMONTHNAME9: u32 = 76u32;
pub const LOCALE_SAM: u32 = 40u32;
pub const LOCALE_SCONSOLEFALLBACKNAME: u32 = 110u32;
pub const LOCALE_SCOUNTRY: u32 = 6u32;
pub const LOCALE_SCURRENCY: u32 = 20u32;
pub const LOCALE_SDATE: u32 = 29u32;
pub const LOCALE_SDAYNAME1: u32 = 42u32;
pub const LOCALE_SDAYNAME2: u32 = 43u32;
pub const LOCALE_SDAYNAME3: u32 = 44u32;
pub const LOCALE_SDAYNAME4: u32 = 45u32;
pub const LOCALE_SDAYNAME5: u32 = 46u32;
pub const LOCALE_SDAYNAME6: u32 = 47u32;
pub const LOCALE_SDAYNAME7: u32 = 48u32;
pub const LOCALE_SDECIMAL: u32 = 14u32;
pub const LOCALE_SDURATION: u32 = 93u32;
pub const LOCALE_SENGCOUNTRY: u32 = 4098u32;
pub const LOCALE_SENGCURRNAME: u32 = 4103u32;
pub const LOCALE_SENGLANGUAGE: u32 = 4097u32;
pub const LOCALE_SENGLISHCOUNTRYNAME: u32 = 4098u32;
pub const LOCALE_SENGLISHDISPLAYNAME: u32 = 114u32;
pub const LOCALE_SENGLISHLANGUAGENAME: u32 = 4097u32;
pub const LOCALE_SGROUPING: u32 = 16u32;
pub const LOCALE_SINTLSYMBOL: u32 = 21u32;
pub const LOCALE_SISO3166CTRYNAME: u32 = 90u32;
pub const LOCALE_SISO3166CTRYNAME2: u32 = 104u32;
pub const LOCALE_SISO639LANGNAME: u32 = 89u32;
pub const LOCALE_SISO639LANGNAME2: u32 = 103u32;
pub const LOCALE_SKEYBOARDSTOINSTALL: u32 = 94u32;
pub const LOCALE_SLANGDISPLAYNAME: u32 = 111u32;
pub const LOCALE_SLANGUAGE: u32 = 2u32;
pub const LOCALE_SLIST: u32 = 12u32;
pub const LOCALE_SLOCALIZEDCOUNTRYNAME: u32 = 6u32;
pub const LOCALE_SLOCALIZEDDISPLAYNAME: u32 = 2u32;
pub const LOCALE_SLOCALIZEDLANGUAGENAME: u32 = 111u32;
pub const LOCALE_SLONGDATE: u32 = 32u32;
pub const LOCALE_SMONDECIMALSEP: u32 = 22u32;
pub const LOCALE_SMONGROUPING: u32 = 24u32;
pub const LOCALE_SMONTHDAY: u32 = 120u32;
pub const LOCALE_SMONTHNAME1: u32 = 56u32;
pub const LOCALE_SMONTHNAME10: u32 = 65u32;
pub const LOCALE_SMONTHNAME11: u32 = 66u32;
pub const LOCALE_SMONTHNAME12: u32 = 67u32;
pub const LOCALE_SMONTHNAME13: u32 = 4110u32;
pub const LOCALE_SMONTHNAME2: u32 = 57u32;
pub const LOCALE_SMONTHNAME3: u32 = 58u32;
pub const LOCALE_SMONTHNAME4: u32 = 59u32;
pub const LOCALE_SMONTHNAME5: u32 = 60u32;
pub const LOCALE_SMONTHNAME6: u32 = 61u32;
pub const LOCALE_SMONTHNAME7: u32 = 62u32;
pub const LOCALE_SMONTHNAME8: u32 = 63u32;
pub const LOCALE_SMONTHNAME9: u32 = 64u32;
pub const LOCALE_SMONTHOUSANDSEP: u32 = 23u32;
pub const LOCALE_SNAME: u32 = 92u32;
pub const LOCALE_SNAN: u32 = 105u32;
pub const LOCALE_SNATIVECOUNTRYNAME: u32 = 8u32;
pub const LOCALE_SNATIVECTRYNAME: u32 = 8u32;
pub const LOCALE_SNATIVECURRNAME: u32 = 4104u32;
pub const LOCALE_SNATIVEDIGITS: u32 = 19u32;
pub const LOCALE_SNATIVEDISPLAYNAME: u32 = 115u32;
pub const LOCALE_SNATIVELANGNAME: u32 = 4u32;
pub const LOCALE_SNATIVELANGUAGENAME: u32 = 4u32;
pub const LOCALE_SNEGATIVESIGN: u32 = 81u32;
pub const LOCALE_SNEGINFINITY: u32 = 107u32;
pub const LOCALE_SOPENTYPELANGUAGETAG: u32 = 122u32;
pub const LOCALE_SPARENT: u32 = 109u32;
pub const LOCALE_SPECIFICDATA: u32 = 32u32;
pub const LOCALE_SPERCENT: u32 = 118u32;
pub const LOCALE_SPERMILLE: u32 = 119u32;
pub const LOCALE_SPM: u32 = 41u32;
pub const LOCALE_SPOSINFINITY: u32 = 106u32;
pub const LOCALE_SPOSITIVESIGN: u32 = 80u32;
pub const LOCALE_SRELATIVELONGDATE: u32 = 124u32;
pub const LOCALE_SSCRIPTS: u32 = 108u32;
pub const LOCALE_SSHORTDATE: u32 = 31u32;
pub const LOCALE_SSHORTESTAM: u32 = 126u32;
pub const LOCALE_SSHORTESTDAYNAME1: u32 = 96u32;
pub const LOCALE_SSHORTESTDAYNAME2: u32 = 97u32;
pub const LOCALE_SSHORTESTDAYNAME3: u32 = 98u32;
pub const LOCALE_SSHORTESTDAYNAME4: u32 = 99u32;
pub const LOCALE_SSHORTESTDAYNAME5: u32 = 100u32;
pub const LOCALE_SSHORTESTDAYNAME6: u32 = 101u32;
pub const LOCALE_SSHORTESTDAYNAME7: u32 = 102u32;
pub const LOCALE_SSHORTESTPM: u32 = 127u32;
pub const LOCALE_SSHORTTIME: u32 = 121u32;
pub const LOCALE_SSORTLOCALE: u32 = 123u32;
pub const LOCALE_SSORTNAME: u32 = 4115u32;
pub const LOCALE_STHOUSAND: u32 = 15u32;
pub const LOCALE_STIME: u32 = 30u32;
pub const LOCALE_STIMEFORMAT: u32 = 4099u32;
pub const LOCALE_SUPPLEMENTAL: u32 = 2u32;
pub const LOCALE_SYEARMONTH: u32 = 4102u32;
pub const LOCALE_USE_CP_ACP: u32 = 1073741824u32;
pub const LOCALE_WINDOWS: u32 = 1u32;
pub const LOWLEVEL_SERVICE_TYPES: u32 = 2u32;
pub const LOW_SURROGATE_END: u32 = 57343u32;
pub const LOW_SURROGATE_START: u32 = 56320u32;
pub const MAP_COMPOSITE: FOLD_STRING_MAP_FLAGS = FOLD_STRING_MAP_FLAGS(64u32);
pub const MAP_EXPAND_LIGATURES: FOLD_STRING_MAP_FLAGS = FOLD_STRING_MAP_FLAGS(8192u32);
pub const MAP_FOLDCZONE: FOLD_STRING_MAP_FLAGS = FOLD_STRING_MAP_FLAGS(16u32);
pub const MAP_FOLDDIGITS: FOLD_STRING_MAP_FLAGS = FOLD_STRING_MAP_FLAGS(128u32);
pub const MAP_PRECOMPOSED: FOLD_STRING_MAP_FLAGS = FOLD_STRING_MAP_FLAGS(32u32);
pub const MAX_DEFAULTCHAR: u32 = 2u32;
pub const MAX_LEADBYTES: u32 = 12u32;
pub const MAX_LOCALE_NAME: u32 = 32u32;
pub const MAX_MIMECP_NAME: u32 = 64u32;
pub const MAX_MIMECSET_NAME: u32 = 50u32;
pub const MAX_MIMEFACE_NAME: u32 = 32u32;
pub const MAX_RFC1766_NAME: u32 = 6u32;
pub const MAX_SCRIPT_NAME: u32 = 48u32;
pub const MB_COMPOSITE: MULTI_BYTE_TO_WIDE_CHAR_FLAGS = MULTI_BYTE_TO_WIDE_CHAR_FLAGS(2u32);
pub const MB_ERR_INVALID_CHARS: MULTI_BYTE_TO_WIDE_CHAR_FLAGS = MULTI_BYTE_TO_WIDE_CHAR_FLAGS(8u32);
pub const MB_PRECOMPOSED: MULTI_BYTE_TO_WIDE_CHAR_FLAGS = MULTI_BYTE_TO_WIDE_CHAR_FLAGS(1u32);
pub const MB_USEGLYPHCHARS: MULTI_BYTE_TO_WIDE_CHAR_FLAGS = MULTI_BYTE_TO_WIDE_CHAR_FLAGS(4u32);
pub const MIMECONTF_BROWSER: MIMECONTF = MIMECONTF(2i32);
pub const MIMECONTF_EXPORT: MIMECONTF = MIMECONTF(1024i32);
pub const MIMECONTF_IMPORT: MIMECONTF = MIMECONTF(8i32);
pub const MIMECONTF_MAILNEWS: MIMECONTF = MIMECONTF(1i32);
pub const MIMECONTF_MIME_IE4: MIMECONTF = MIMECONTF(268435456i32);
pub const MIMECONTF_MIME_LATEST: MIMECONTF = MIMECONTF(536870912i32);
pub const MIMECONTF_MIME_REGISTRY: MIMECONTF = MIMECONTF(1073741824i32);
pub const MIMECONTF_MINIMAL: MIMECONTF = MIMECONTF(4i32);
pub const MIMECONTF_PRIVCONVERTER: MIMECONTF = MIMECONTF(65536i32);
pub const MIMECONTF_SAVABLE_BROWSER: MIMECONTF = MIMECONTF(512i32);
pub const MIMECONTF_SAVABLE_MAILNEWS: MIMECONTF = MIMECONTF(256i32);
pub const MIMECONTF_VALID: MIMECONTF = MIMECONTF(131072i32);
pub const MIMECONTF_VALID_NLS: MIMECONTF = MIMECONTF(262144i32);
pub const MIN_SPELLING_NTDDI: u32 = 100794368u32;
pub const MLCONVCHARF_AUTODETECT: MLCONVCHAR = MLCONVCHAR(1i32);
pub const MLCONVCHARF_DETECTJPN: MLCONVCHAR = MLCONVCHAR(32i32);
pub const MLCONVCHARF_ENTITIZE: MLCONVCHAR = MLCONVCHAR(2i32);
pub const MLCONVCHARF_NAME_ENTITIZE: MLCONVCHAR = MLCONVCHAR(4i32);
pub const MLCONVCHARF_NCR_ENTITIZE: MLCONVCHAR = MLCONVCHAR(2i32);
pub const MLCONVCHARF_NOBESTFITCHARS: MLCONVCHAR = MLCONVCHAR(16i32);
pub const MLCONVCHARF_USEDEFCHAR: MLCONVCHAR = MLCONVCHAR(8i32);
pub const MLDETECTCP_7BIT: MLDETECTCP = MLDETECTCP(1i32);
pub const MLDETECTCP_8BIT: MLDETECTCP = MLDETECTCP(2i32);
pub const MLDETECTCP_DBCS: MLDETECTCP = MLDETECTCP(4i32);
pub const MLDETECTCP_HTML: MLDETECTCP = MLDETECTCP(8i32);
pub const MLDETECTCP_NONE: MLDETECTCP = MLDETECTCP(0i32);
pub const MLDETECTCP_NUMBER: MLDETECTCP = MLDETECTCP(16i32);
pub const MLDETECTF_BROWSER: MLCP = MLCP(2i32);
pub const MLDETECTF_EURO_UTF8: MLCP = MLCP(128i32);
pub const MLDETECTF_FILTER_SPECIALCHAR: MLCP = MLCP(64i32);
pub const MLDETECTF_MAILNEWS: MLCP = MLCP(1i32);
pub const MLDETECTF_PREFERRED_ONLY: MLCP = MLCP(32i32);
pub const MLDETECTF_PRESERVE_ORDER: MLCP = MLCP(16i32);
pub const MLDETECTF_VALID: MLCP = MLCP(4i32);
pub const MLDETECTF_VALID_NLS: MLCP = MLCP(8i32);
pub const MLSTR_READ: MLSTR_FLAGS = MLSTR_FLAGS(1i32);
pub const MLSTR_WRITE: MLSTR_FLAGS = MLSTR_FLAGS(2i32);
pub const MUI_COMPLEX_SCRIPT_FILTER: u32 = 512u32;
pub const MUI_CONSOLE_FILTER: u32 = 256u32;
pub const MUI_FILEINFO_VERSION: u32 = 1u32;
pub const MUI_FILETYPE_LANGUAGE_NEUTRAL_MAIN: u32 = 2u32;
pub const MUI_FILETYPE_LANGUAGE_NEUTRAL_MUI: u32 = 4u32;
pub const MUI_FILETYPE_NOT_LANGUAGE_NEUTRAL: u32 = 1u32;
pub const MUI_FORMAT_INF_COMPAT: u32 = 2u32;
pub const MUI_FORMAT_REG_COMPAT: u32 = 1u32;
pub const MUI_FULL_LANGUAGE: u32 = 1u32;
pub const MUI_IMMUTABLE_LOOKUP: u32 = 16u32;
pub const MUI_LANGUAGE_EXACT: u32 = 16u32;
pub const MUI_LANGUAGE_ID: u32 = 4u32;
pub const MUI_LANGUAGE_INSTALLED: u32 = 32u32;
pub const MUI_LANGUAGE_LICENSED: u32 = 64u32;
pub const MUI_LANGUAGE_NAME: u32 = 8u32;
pub const MUI_LANG_NEUTRAL_PE_FILE: u32 = 256u32;
pub const MUI_LIP_LANGUAGE: u32 = 4u32;
pub const MUI_MACHINE_LANGUAGE_SETTINGS: u32 = 1024u32;
pub const MUI_MERGE_SYSTEM_FALLBACK: u32 = 16u32;
pub const MUI_MERGE_USER_FALLBACK: u32 = 32u32;
pub const MUI_NON_LANG_NEUTRAL_FILE: u32 = 512u32;
pub const MUI_PARTIAL_LANGUAGE: u32 = 2u32;
pub const MUI_QUERY_CHECKSUM: u32 = 2u32;
pub const MUI_QUERY_LANGUAGE_NAME: u32 = 4u32;
pub const MUI_QUERY_RESOURCE_TYPES: u32 = 8u32;
pub const MUI_QUERY_TYPE: u32 = 1u32;
pub const MUI_RESET_FILTERS: u32 = 1u32;
pub const MUI_SKIP_STRING_CACHE: u32 = 8u32;
pub const MUI_THREAD_LANGUAGES: u32 = 64u32;
pub const MUI_USER_PREFERRED_UI_LANGUAGES: u32 = 16u32;
pub const MUI_USE_INSTALLED_LANGUAGES: u32 = 32u32;
pub const MUI_USE_SEARCH_ALL_LANGUAGES: u32 = 64u32;
pub const MUI_VERIFY_FILE_EXISTS: u32 = 4u32;
pub const MinuteUnit: CALDATETIME_DATEUNIT = CALDATETIME_DATEUNIT(6i32);
pub const MonthUnit: CALDATETIME_DATEUNIT = CALDATETIME_DATEUNIT(2i32);
pub const NLS_CP_CPINFO: u32 = 268435456u32;
pub const NLS_CP_MBTOWC: u32 = 1073741824u32;
pub const NLS_CP_WCTOMB: u32 = 2147483648u32;
pub const NORM_IGNORECASE: COMPARE_STRING_FLAGS = COMPARE_STRING_FLAGS(1u32);
pub const NORM_IGNOREKANATYPE: COMPARE_STRING_FLAGS = COMPARE_STRING_FLAGS(65536u32);
pub const NORM_IGNORENONSPACE: COMPARE_STRING_FLAGS = COMPARE_STRING_FLAGS(2u32);
pub const NORM_IGNORESYMBOLS: COMPARE_STRING_FLAGS = COMPARE_STRING_FLAGS(4u32);
pub const NORM_IGNOREWIDTH: COMPARE_STRING_FLAGS = COMPARE_STRING_FLAGS(131072u32);
pub const NORM_LINGUISTIC_CASING: COMPARE_STRING_FLAGS = COMPARE_STRING_FLAGS(134217728u32);
pub const NUMSYS_NAME_CAPACITY: u32 = 8u32;
pub const NormalizationC: NORM_FORM = NORM_FORM(1i32);
pub const NormalizationD: NORM_FORM = NORM_FORM(2i32);
pub const NormalizationKC: NORM_FORM = NORM_FORM(5i32);
pub const NormalizationKD: NORM_FORM = NORM_FORM(6i32);
pub const NormalizationOther: NORM_FORM = NORM_FORM(0i32);
pub const OFFLINE_SERVICES: u32 = 2u32;
pub const ONLINE_SERVICES: u32 = 1u32;
pub const SCRIPTCONTF_FIXED_FONT: SCRIPTFONTCONTF = SCRIPTFONTCONTF(1i32);
pub const SCRIPTCONTF_PROPORTIONAL_FONT: SCRIPTFONTCONTF = SCRIPTFONTCONTF(2i32);
pub const SCRIPTCONTF_SCRIPT_HIDE: SCRIPTFONTCONTF = SCRIPTFONTCONTF(131072i32);
pub const SCRIPTCONTF_SCRIPT_SYSTEM: SCRIPTFONTCONTF = SCRIPTFONTCONTF(262144i32);
pub const SCRIPTCONTF_SCRIPT_USER: SCRIPTFONTCONTF = SCRIPTFONTCONTF(65536i32);
pub const SCRIPT_DIGITSUBSTITUTE_CONTEXT: u32 = 0u32;
pub const SCRIPT_DIGITSUBSTITUTE_NATIONAL: u32 = 2u32;
pub const SCRIPT_DIGITSUBSTITUTE_NONE: u32 = 1u32;
pub const SCRIPT_DIGITSUBSTITUTE_TRADITIONAL: u32 = 3u32;
pub const SCRIPT_JUSTIFY_ARABIC_ALEF: SCRIPT_JUSTIFY = SCRIPT_JUSTIFY(9i32);
pub const SCRIPT_JUSTIFY_ARABIC_BA: SCRIPT_JUSTIFY = SCRIPT_JUSTIFY(12i32);
pub const SCRIPT_JUSTIFY_ARABIC_BARA: SCRIPT_JUSTIFY = SCRIPT_JUSTIFY(13i32);
pub const SCRIPT_JUSTIFY_ARABIC_BLANK: SCRIPT_JUSTIFY = SCRIPT_JUSTIFY(1i32);
pub const SCRIPT_JUSTIFY_ARABIC_HA: SCRIPT_JUSTIFY = SCRIPT_JUSTIFY(10i32);
pub const SCRIPT_JUSTIFY_ARABIC_KASHIDA: SCRIPT_JUSTIFY = SCRIPT_JUSTIFY(8i32);
pub const SCRIPT_JUSTIFY_ARABIC_NORMAL: SCRIPT_JUSTIFY = SCRIPT_JUSTIFY(7i32);
pub const SCRIPT_JUSTIFY_ARABIC_RA: SCRIPT_JUSTIFY = SCRIPT_JUSTIFY(11i32);
pub const SCRIPT_JUSTIFY_ARABIC_SEEN: SCRIPT_JUSTIFY = SCRIPT_JUSTIFY(14i32);
pub const SCRIPT_JUSTIFY_ARABIC_SEEN_M: SCRIPT_JUSTIFY = SCRIPT_JUSTIFY(15i32);
pub const SCRIPT_JUSTIFY_BLANK: SCRIPT_JUSTIFY = SCRIPT_JUSTIFY(4i32);
pub const SCRIPT_JUSTIFY_CHARACTER: SCRIPT_JUSTIFY = SCRIPT_JUSTIFY(2i32);
pub const SCRIPT_JUSTIFY_NONE: SCRIPT_JUSTIFY = SCRIPT_JUSTIFY(0i32);
pub const SCRIPT_JUSTIFY_RESERVED1: SCRIPT_JUSTIFY = SCRIPT_JUSTIFY(3i32);
pub const SCRIPT_JUSTIFY_RESERVED2: SCRIPT_JUSTIFY = SCRIPT_JUSTIFY(5i32);
pub const SCRIPT_JUSTIFY_RESERVED3: SCRIPT_JUSTIFY = SCRIPT_JUSTIFY(6i32);
pub const SCRIPT_TAG_UNKNOWN: u32 = 0u32;
pub const SCRIPT_UNDEFINED: u32 = 0u32;
pub const SGCM_RTL: u32 = 1u32;
pub const SIC_ASCIIDIGIT: SCRIPT_IS_COMPLEX_FLAGS = SCRIPT_IS_COMPLEX_FLAGS(2u32);
pub const SIC_COMPLEX: SCRIPT_IS_COMPLEX_FLAGS = SCRIPT_IS_COMPLEX_FLAGS(1u32);
pub const SIC_NEUTRAL: SCRIPT_IS_COMPLEX_FLAGS = SCRIPT_IS_COMPLEX_FLAGS(4u32);
pub const SORTING_PARADIGM_ICU: u32 = 16777216u32;
pub const SORTING_PARADIGM_NLS: u32 = 0u32;
pub const SORT_DIGITSASNUMBERS: COMPARE_STRING_FLAGS = COMPARE_STRING_FLAGS(8u32);
pub const SORT_STRINGSORT: COMPARE_STRING_FLAGS = COMPARE_STRING_FLAGS(4096u32);
pub const SSA_BREAK: u32 = 64u32;
pub const SSA_CLIP: u32 = 4u32;
pub const SSA_DONTGLYPH: u32 = 1073741824u32;
pub const SSA_DZWG: u32 = 16u32;
pub const SSA_FALLBACK: u32 = 32u32;
pub const SSA_FIT: u32 = 8u32;
pub const SSA_FULLMEASURE: u32 = 67108864u32;
pub const SSA_GCP: u32 = 512u32;
pub const SSA_GLYPHS: u32 = 128u32;
pub const SSA_HIDEHOTKEY: u32 = 8192u32;
pub const SSA_HOTKEY: u32 = 1024u32;
pub const SSA_HOTKEYONLY: u32 = 9216u32;
pub const SSA_LAYOUTRTL: u32 = 536870912u32;
pub const SSA_LINK: u32 = 4096u32;
pub const SSA_LPKANSIFALLBACK: u32 = 134217728u32;
pub const SSA_METAFILE: u32 = 2048u32;
pub const SSA_NOKASHIDA: u32 = 2147483648u32;
pub const SSA_PASSWORD: u32 = 1u32;
pub const SSA_PIDX: u32 = 268435456u32;
pub const SSA_RTL: u32 = 256u32;
pub const SSA_TAB: u32 = 2u32;
pub const SecondUnit: CALDATETIME_DATEUNIT = CALDATETIME_DATEUNIT(7i32);
pub const TCI_SRCCHARSET: TRANSLATE_CHARSET_INFO_FLAGS = TRANSLATE_CHARSET_INFO_FLAGS(1u32);
pub const TCI_SRCCODEPAGE: TRANSLATE_CHARSET_INFO_FLAGS = TRANSLATE_CHARSET_INFO_FLAGS(2u32);
pub const TCI_SRCFONTSIG: TRANSLATE_CHARSET_INFO_FLAGS = TRANSLATE_CHARSET_INFO_FLAGS(3u32);
pub const TCI_SRCLOCALE: TRANSLATE_CHARSET_INFO_FLAGS = TRANSLATE_CHARSET_INFO_FLAGS(4096u32);
pub const TIME_FORCE24HOURFORMAT: TIME_FORMAT_FLAGS = TIME_FORMAT_FLAGS(8u32);
pub const TIME_NOMINUTESORSECONDS: TIME_FORMAT_FLAGS = TIME_FORMAT_FLAGS(1u32);
pub const TIME_NOSECONDS: TIME_FORMAT_FLAGS = TIME_FORMAT_FLAGS(2u32);
pub const TIME_NOTIMEMARKER: TIME_FORMAT_FLAGS = TIME_FORMAT_FLAGS(4u32);
pub const TickUnit: CALDATETIME_DATEUNIT = CALDATETIME_DATEUNIT(8i32);
pub const U16_MAX_LENGTH: u32 = 2u32;
pub const U8_LEAD3_T1_BITS: windows_core::PCSTR = windows_core::s!(" 000000000000\u{10}00");
pub const U8_LEAD4_T1_BITS: windows_core::PCSTR = windows_core::s!("\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{1e}\u{f}\u{f}\u{f}\u{0}\u{0}\u{0}\u{0}");
pub const U8_MAX_LENGTH: u32 = 4u32;
pub const UBIDI_DEFAULT_LTR: u32 = 254u32;
pub const UBIDI_DEFAULT_RTL: u32 = 255u32;
pub const UBIDI_DO_MIRRORING: u32 = 2u32;
pub const UBIDI_INSERT_LRM_FOR_NUMERIC: u32 = 4u32;
pub const UBIDI_KEEP_BASE_COMBINING: u32 = 1u32;
pub const UBIDI_LEVEL_OVERRIDE: u32 = 128u32;
pub const UBIDI_LOGICAL: UBiDiOrder = UBiDiOrder(0i32);
pub const UBIDI_LTR: UBiDiDirection = UBiDiDirection(0i32);
pub const UBIDI_MAP_NOWHERE: i32 = -1i32;
pub const UBIDI_MAX_EXPLICIT_LEVEL: u32 = 125u32;
pub const UBIDI_MIRRORING_OFF: UBiDiMirroring = UBiDiMirroring(0i32);
pub const UBIDI_MIRRORING_ON: UBiDiMirroring = UBiDiMirroring(1i32);
pub const UBIDI_MIXED: UBiDiDirection = UBiDiDirection(2i32);
pub const UBIDI_NEUTRAL: UBiDiDirection = UBiDiDirection(3i32);
pub const UBIDI_OPTION_DEFAULT: UBiDiReorderingOption = UBiDiReorderingOption(0i32);
pub const UBIDI_OPTION_INSERT_MARKS: UBiDiReorderingOption = UBiDiReorderingOption(1i32);
pub const UBIDI_OPTION_REMOVE_CONTROLS: UBiDiReorderingOption = UBiDiReorderingOption(2i32);
pub const UBIDI_OPTION_STREAMING: UBiDiReorderingOption = UBiDiReorderingOption(4i32);
pub const UBIDI_OUTPUT_REVERSE: u32 = 16u32;
pub const UBIDI_REMOVE_BIDI_CONTROLS: u32 = 8u32;
pub const UBIDI_REORDER_DEFAULT: UBiDiReorderingMode = UBiDiReorderingMode(0i32);
pub const UBIDI_REORDER_GROUP_NUMBERS_WITH_R: UBiDiReorderingMode = UBiDiReorderingMode(2i32);
pub const UBIDI_REORDER_INVERSE_FOR_NUMBERS_SPECIAL: UBiDiReorderingMode = UBiDiReorderingMode(6i32);
pub const UBIDI_REORDER_INVERSE_LIKE_DIRECT: UBiDiReorderingMode = UBiDiReorderingMode(5i32);
pub const UBIDI_REORDER_INVERSE_NUMBERS_AS_L: UBiDiReorderingMode = UBiDiReorderingMode(4i32);
pub const UBIDI_REORDER_NUMBERS_SPECIAL: UBiDiReorderingMode = UBiDiReorderingMode(1i32);
pub const UBIDI_REORDER_RUNS_ONLY: UBiDiReorderingMode = UBiDiReorderingMode(3i32);
pub const UBIDI_RTL: UBiDiDirection = UBiDiDirection(1i32);
pub const UBIDI_VISUAL: UBiDiOrder = UBiDiOrder(1i32);
pub const UBLOCK_ADLAM: UBlockCode = UBlockCode(263i32);
pub const UBLOCK_AEGEAN_NUMBERS: UBlockCode = UBlockCode(119i32);
pub const UBLOCK_AHOM: UBlockCode = UBlockCode(253i32);
pub const UBLOCK_ALCHEMICAL_SYMBOLS: UBlockCode = UBlockCode(208i32);
pub const UBLOCK_ALPHABETIC_PRESENTATION_FORMS: UBlockCode = UBlockCode(80i32);
pub const UBLOCK_ANATOLIAN_HIEROGLYPHS: UBlockCode = UBlockCode(254i32);
pub const UBLOCK_ANCIENT_GREEK_MUSICAL_NOTATION: UBlockCode = UBlockCode(126i32);
pub const UBLOCK_ANCIENT_GREEK_NUMBERS: UBlockCode = UBlockCode(127i32);
pub const UBLOCK_ANCIENT_SYMBOLS: UBlockCode = UBlockCode(165i32);
pub const UBLOCK_ARABIC: UBlockCode = UBlockCode(12i32);
pub const UBLOCK_ARABIC_EXTENDED_A: UBlockCode = UBlockCode(210i32);
pub const UBLOCK_ARABIC_MATHEMATICAL_ALPHABETIC_SYMBOLS: UBlockCode = UBlockCode(211i32);
pub const UBLOCK_ARABIC_PRESENTATION_FORMS_A: UBlockCode = UBlockCode(81i32);
pub const UBLOCK_ARABIC_PRESENTATION_FORMS_B: UBlockCode = UBlockCode(85i32);
pub const UBLOCK_ARABIC_SUPPLEMENT: UBlockCode = UBlockCode(128i32);
pub const UBLOCK_ARMENIAN: UBlockCode = UBlockCode(10i32);
pub const UBLOCK_ARROWS: UBlockCode = UBlockCode(46i32);
pub const UBLOCK_AVESTAN: UBlockCode = UBlockCode(188i32);
pub const UBLOCK_BALINESE: UBlockCode = UBlockCode(147i32);
pub const UBLOCK_BAMUM: UBlockCode = UBlockCode(177i32);
pub const UBLOCK_BAMUM_SUPPLEMENT: UBlockCode = UBlockCode(202i32);
pub const UBLOCK_BASIC_LATIN: UBlockCode = UBlockCode(1i32);
pub const UBLOCK_BASSA_VAH: UBlockCode = UBlockCode(221i32);
pub const UBLOCK_BATAK: UBlockCode = UBlockCode(199i32);
pub const UBLOCK_BENGALI: UBlockCode = UBlockCode(16i32);
pub const UBLOCK_BHAIKSUKI: UBlockCode = UBlockCode(264i32);
pub const UBLOCK_BLOCK_ELEMENTS: UBlockCode = UBlockCode(53i32);
pub const UBLOCK_BOPOMOFO: UBlockCode = UBlockCode(64i32);
pub const UBLOCK_BOPOMOFO_EXTENDED: UBlockCode = UBlockCode(67i32);
pub const UBLOCK_BOX_DRAWING: UBlockCode = UBlockCode(52i32);
pub const UBLOCK_BRAHMI: UBlockCode = UBlockCode(201i32);
pub const UBLOCK_BRAILLE_PATTERNS: UBlockCode = UBlockCode(57i32);
pub const UBLOCK_BUGINESE: UBlockCode = UBlockCode(129i32);
pub const UBLOCK_BUHID: UBlockCode = UBlockCode(100i32);
pub const UBLOCK_BYZANTINE_MUSICAL_SYMBOLS: UBlockCode = UBlockCode(91i32);
pub const UBLOCK_CARIAN: UBlockCode = UBlockCode(168i32);
pub const UBLOCK_CAUCASIAN_ALBANIAN: UBlockCode = UBlockCode(222i32);
pub const UBLOCK_CHAKMA: UBlockCode = UBlockCode(212i32);
pub const UBLOCK_CHAM: UBlockCode = UBlockCode(164i32);
pub const UBLOCK_CHEROKEE: UBlockCode = UBlockCode(32i32);
pub const UBLOCK_CHEROKEE_SUPPLEMENT: UBlockCode = UBlockCode(255i32);
pub const UBLOCK_CHESS_SYMBOLS: UBlockCode = UBlockCode(281i32);
pub const UBLOCK_CHORASMIAN: UBlockCode = UBlockCode(301i32);
pub const UBLOCK_CJK_COMPATIBILITY: UBlockCode = UBlockCode(69i32);
pub const UBLOCK_CJK_COMPATIBILITY_FORMS: UBlockCode = UBlockCode(83i32);
pub const UBLOCK_CJK_COMPATIBILITY_IDEOGRAPHS: UBlockCode = UBlockCode(79i32);
pub const UBLOCK_CJK_COMPATIBILITY_IDEOGRAPHS_SUPPLEMENT: UBlockCode = UBlockCode(95i32);
pub const UBLOCK_CJK_RADICALS_SUPPLEMENT: UBlockCode = UBlockCode(58i32);
pub const UBLOCK_CJK_STROKES: UBlockCode = UBlockCode(130i32);
pub const UBLOCK_CJK_SYMBOLS_AND_PUNCTUATION: UBlockCode = UBlockCode(61i32);
pub const UBLOCK_CJK_UNIFIED_IDEOGRAPHS: UBlockCode = UBlockCode(71i32);
pub const UBLOCK_CJK_UNIFIED_IDEOGRAPHS_EXTENSION_A: UBlockCode = UBlockCode(70i32);
pub const UBLOCK_CJK_UNIFIED_IDEOGRAPHS_EXTENSION_B: UBlockCode = UBlockCode(94i32);
pub const UBLOCK_CJK_UNIFIED_IDEOGRAPHS_EXTENSION_C: UBlockCode = UBlockCode(197i32);
pub const UBLOCK_CJK_UNIFIED_IDEOGRAPHS_EXTENSION_D: UBlockCode = UBlockCode(209i32);
pub const UBLOCK_CJK_UNIFIED_IDEOGRAPHS_EXTENSION_E: UBlockCode = UBlockCode(256i32);
pub const UBLOCK_CJK_UNIFIED_IDEOGRAPHS_EXTENSION_F: UBlockCode = UBlockCode(274i32);
pub const UBLOCK_CJK_UNIFIED_IDEOGRAPHS_EXTENSION_G: UBlockCode = UBlockCode(302i32);
pub const UBLOCK_COMBINING_DIACRITICAL_MARKS: UBlockCode = UBlockCode(7i32);
pub const UBLOCK_COMBINING_DIACRITICAL_MARKS_EXTENDED: UBlockCode = UBlockCode(224i32);
pub const UBLOCK_COMBINING_DIACRITICAL_MARKS_SUPPLEMENT: UBlockCode = UBlockCode(131i32);
pub const UBLOCK_COMBINING_HALF_MARKS: UBlockCode = UBlockCode(82i32);
pub const UBLOCK_COMBINING_MARKS_FOR_SYMBOLS: UBlockCode = UBlockCode(43i32);
pub const UBLOCK_COMMON_INDIC_NUMBER_FORMS: UBlockCode = UBlockCode(178i32);
pub const UBLOCK_CONTROL_PICTURES: UBlockCode = UBlockCode(49i32);
pub const UBLOCK_COPTIC: UBlockCode = UBlockCode(132i32);
pub const UBLOCK_COPTIC_EPACT_NUMBERS: UBlockCode = UBlockCode(223i32);
pub const UBLOCK_COUNTING_ROD_NUMERALS: UBlockCode = UBlockCode(154i32);
pub const UBLOCK_CUNEIFORM: UBlockCode = UBlockCode(152i32);
pub const UBLOCK_CUNEIFORM_NUMBERS_AND_PUNCTUATION: UBlockCode = UBlockCode(153i32);
pub const UBLOCK_CURRENCY_SYMBOLS: UBlockCode = UBlockCode(42i32);
pub const UBLOCK_CYPRIOT_SYLLABARY: UBlockCode = UBlockCode(123i32);
pub const UBLOCK_CYRILLIC: UBlockCode = UBlockCode(9i32);
pub const UBLOCK_CYRILLIC_EXTENDED_A: UBlockCode = UBlockCode(158i32);
pub const UBLOCK_CYRILLIC_EXTENDED_B: UBlockCode = UBlockCode(160i32);
pub const UBLOCK_CYRILLIC_EXTENDED_C: UBlockCode = UBlockCode(265i32);
pub const UBLOCK_CYRILLIC_SUPPLEMENT: UBlockCode = UBlockCode(97i32);
pub const UBLOCK_CYRILLIC_SUPPLEMENTARY: UBlockCode = UBlockCode(97i32);
pub const UBLOCK_DESERET: UBlockCode = UBlockCode(90i32);
pub const UBLOCK_DEVANAGARI: UBlockCode = UBlockCode(15i32);
pub const UBLOCK_DEVANAGARI_EXTENDED: UBlockCode = UBlockCode(179i32);
pub const UBLOCK_DINGBATS: UBlockCode = UBlockCode(56i32);
pub const UBLOCK_DIVES_AKURU: UBlockCode = UBlockCode(303i32);
pub const UBLOCK_DOGRA: UBlockCode = UBlockCode(282i32);
pub const UBLOCK_DOMINO_TILES: UBlockCode = UBlockCode(171i32);
pub const UBLOCK_DUPLOYAN: UBlockCode = UBlockCode(225i32);
pub const UBLOCK_EARLY_DYNASTIC_CUNEIFORM: UBlockCode = UBlockCode(257i32);
pub const UBLOCK_EGYPTIAN_HIEROGLYPHS: UBlockCode = UBlockCode(194i32);
pub const UBLOCK_EGYPTIAN_HIEROGLYPH_FORMAT_CONTROLS: UBlockCode = UBlockCode(292i32);
pub const UBLOCK_ELBASAN: UBlockCode = UBlockCode(226i32);
pub const UBLOCK_ELYMAIC: UBlockCode = UBlockCode(293i32);
pub const UBLOCK_EMOTICONS: UBlockCode = UBlockCode(206i32);
pub const UBLOCK_ENCLOSED_ALPHANUMERICS: UBlockCode = UBlockCode(51i32);
pub const UBLOCK_ENCLOSED_ALPHANUMERIC_SUPPLEMENT: UBlockCode = UBlockCode(195i32);
pub const UBLOCK_ENCLOSED_CJK_LETTERS_AND_MONTHS: UBlockCode = UBlockCode(68i32);
pub const UBLOCK_ENCLOSED_IDEOGRAPHIC_SUPPLEMENT: UBlockCode = UBlockCode(196i32);
pub const UBLOCK_ETHIOPIC: UBlockCode = UBlockCode(31i32);
pub const UBLOCK_ETHIOPIC_EXTENDED: UBlockCode = UBlockCode(133i32);
pub const UBLOCK_ETHIOPIC_EXTENDED_A: UBlockCode = UBlockCode(200i32);
pub const UBLOCK_ETHIOPIC_SUPPLEMENT: UBlockCode = UBlockCode(134i32);
pub const UBLOCK_GENERAL_PUNCTUATION: UBlockCode = UBlockCode(40i32);
pub const UBLOCK_GEOMETRIC_SHAPES: UBlockCode = UBlockCode(54i32);
pub const UBLOCK_GEOMETRIC_SHAPES_EXTENDED: UBlockCode = UBlockCode(227i32);
pub const UBLOCK_GEORGIAN: UBlockCode = UBlockCode(29i32);
pub const UBLOCK_GEORGIAN_EXTENDED: UBlockCode = UBlockCode(283i32);
pub const UBLOCK_GEORGIAN_SUPPLEMENT: UBlockCode = UBlockCode(135i32);
pub const UBLOCK_GLAGOLITIC: UBlockCode = UBlockCode(136i32);
pub const UBLOCK_GLAGOLITIC_SUPPLEMENT: UBlockCode = UBlockCode(266i32);
pub const UBLOCK_GOTHIC: UBlockCode = UBlockCode(89i32);
pub const UBLOCK_GRANTHA: UBlockCode = UBlockCode(228i32);
pub const UBLOCK_GREEK: UBlockCode = UBlockCode(8i32);
pub const UBLOCK_GREEK_EXTENDED: UBlockCode = UBlockCode(39i32);
pub const UBLOCK_GUJARATI: UBlockCode = UBlockCode(18i32);
pub const UBLOCK_GUNJALA_GONDI: UBlockCode = UBlockCode(284i32);
pub const UBLOCK_GURMUKHI: UBlockCode = UBlockCode(17i32);
pub const UBLOCK_HALFWIDTH_AND_FULLWIDTH_FORMS: UBlockCode = UBlockCode(87i32);
pub const UBLOCK_HANGUL_COMPATIBILITY_JAMO: UBlockCode = UBlockCode(65i32);
pub const UBLOCK_HANGUL_JAMO: UBlockCode = UBlockCode(30i32);
pub const UBLOCK_HANGUL_JAMO_EXTENDED_A: UBlockCode = UBlockCode(180i32);
pub const UBLOCK_HANGUL_JAMO_EXTENDED_B: UBlockCode = UBlockCode(185i32);
pub const UBLOCK_HANGUL_SYLLABLES: UBlockCode = UBlockCode(74i32);
pub const UBLOCK_HANIFI_ROHINGYA: UBlockCode = UBlockCode(285i32);
pub const UBLOCK_HANUNOO: UBlockCode = UBlockCode(99i32);
pub const UBLOCK_HATRAN: UBlockCode = UBlockCode(258i32);
pub const UBLOCK_HEBREW: UBlockCode = UBlockCode(11i32);
pub const UBLOCK_HIGH_PRIVATE_USE_SURROGATES: UBlockCode = UBlockCode(76i32);
pub const UBLOCK_HIGH_SURROGATES: UBlockCode = UBlockCode(75i32);
pub const UBLOCK_HIRAGANA: UBlockCode = UBlockCode(62i32);
pub const UBLOCK_IDEOGRAPHIC_DESCRIPTION_CHARACTERS: UBlockCode = UBlockCode(60i32);
pub const UBLOCK_IDEOGRAPHIC_SYMBOLS_AND_PUNCTUATION: UBlockCode = UBlockCode(267i32);
pub const UBLOCK_IMPERIAL_ARAMAIC: UBlockCode = UBlockCode(186i32);
pub const UBLOCK_INDIC_SIYAQ_NUMBERS: UBlockCode = UBlockCode(286i32);
pub const UBLOCK_INSCRIPTIONAL_PAHLAVI: UBlockCode = UBlockCode(190i32);
pub const UBLOCK_INSCRIPTIONAL_PARTHIAN: UBlockCode = UBlockCode(189i32);
pub const UBLOCK_INVALID_CODE: UBlockCode = UBlockCode(-1i32);
pub const UBLOCK_IPA_EXTENSIONS: UBlockCode = UBlockCode(5i32);
pub const UBLOCK_JAVANESE: UBlockCode = UBlockCode(181i32);
pub const UBLOCK_KAITHI: UBlockCode = UBlockCode(193i32);
pub const UBLOCK_KANA_EXTENDED_A: UBlockCode = UBlockCode(275i32);
pub const UBLOCK_KANA_SUPPLEMENT: UBlockCode = UBlockCode(203i32);
pub const UBLOCK_KANBUN: UBlockCode = UBlockCode(66i32);
pub const UBLOCK_KANGXI_RADICALS: UBlockCode = UBlockCode(59i32);
pub const UBLOCK_KANNADA: UBlockCode = UBlockCode(22i32);
pub const UBLOCK_KATAKANA: UBlockCode = UBlockCode(63i32);
pub const UBLOCK_KATAKANA_PHONETIC_EXTENSIONS: UBlockCode = UBlockCode(107i32);
pub const UBLOCK_KAYAH_LI: UBlockCode = UBlockCode(162i32);
pub const UBLOCK_KHAROSHTHI: UBlockCode = UBlockCode(137i32);
pub const UBLOCK_KHITAN_SMALL_SCRIPT: UBlockCode = UBlockCode(304i32);
pub const UBLOCK_KHMER: UBlockCode = UBlockCode(36i32);
pub const UBLOCK_KHMER_SYMBOLS: UBlockCode = UBlockCode(113i32);
pub const UBLOCK_KHOJKI: UBlockCode = UBlockCode(229i32);
pub const UBLOCK_KHUDAWADI: UBlockCode = UBlockCode(230i32);
pub const UBLOCK_LAO: UBlockCode = UBlockCode(26i32);
pub const UBLOCK_LATIN_1_SUPPLEMENT: UBlockCode = UBlockCode(2i32);
pub const UBLOCK_LATIN_EXTENDED_A: UBlockCode = UBlockCode(3i32);
pub const UBLOCK_LATIN_EXTENDED_ADDITIONAL: UBlockCode = UBlockCode(38i32);
pub const UBLOCK_LATIN_EXTENDED_B: UBlockCode = UBlockCode(4i32);
pub const UBLOCK_LATIN_EXTENDED_C: UBlockCode = UBlockCode(148i32);
pub const UBLOCK_LATIN_EXTENDED_D: UBlockCode = UBlockCode(149i32);
pub const UBLOCK_LATIN_EXTENDED_E: UBlockCode = UBlockCode(231i32);
pub const UBLOCK_LEPCHA: UBlockCode = UBlockCode(156i32);
pub const UBLOCK_LETTERLIKE_SYMBOLS: UBlockCode = UBlockCode(44i32);
pub const UBLOCK_LIMBU: UBlockCode = UBlockCode(111i32);
pub const UBLOCK_LINEAR_A: UBlockCode = UBlockCode(232i32);
pub const UBLOCK_LINEAR_B_IDEOGRAMS: UBlockCode = UBlockCode(118i32);
pub const UBLOCK_LINEAR_B_SYLLABARY: UBlockCode = UBlockCode(117i32);
pub const UBLOCK_LISU: UBlockCode = UBlockCode(176i32);
pub const UBLOCK_LISU_SUPPLEMENT: UBlockCode = UBlockCode(305i32);
pub const UBLOCK_LOW_SURROGATES: UBlockCode = UBlockCode(77i32);
pub const UBLOCK_LYCIAN: UBlockCode = UBlockCode(167i32);
pub const UBLOCK_LYDIAN: UBlockCode = UBlockCode(169i32);
pub const UBLOCK_MAHAJANI: UBlockCode = UBlockCode(233i32);
pub const UBLOCK_MAHJONG_TILES: UBlockCode = UBlockCode(170i32);
pub const UBLOCK_MAKASAR: UBlockCode = UBlockCode(287i32);
pub const UBLOCK_MALAYALAM: UBlockCode = UBlockCode(23i32);
pub const UBLOCK_MANDAIC: UBlockCode = UBlockCode(198i32);
pub const UBLOCK_MANICHAEAN: UBlockCode = UBlockCode(234i32);
pub const UBLOCK_MARCHEN: UBlockCode = UBlockCode(268i32);
pub const UBLOCK_MASARAM_GONDI: UBlockCode = UBlockCode(276i32);
pub const UBLOCK_MATHEMATICAL_ALPHANUMERIC_SYMBOLS: UBlockCode = UBlockCode(93i32);
pub const UBLOCK_MATHEMATICAL_OPERATORS: UBlockCode = UBlockCode(47i32);
pub const UBLOCK_MAYAN_NUMERALS: UBlockCode = UBlockCode(288i32);
pub const UBLOCK_MEDEFAIDRIN: UBlockCode = UBlockCode(289i32);
pub const UBLOCK_MEETEI_MAYEK: UBlockCode = UBlockCode(184i32);
pub const UBLOCK_MEETEI_MAYEK_EXTENSIONS: UBlockCode = UBlockCode(213i32);
pub const UBLOCK_MENDE_KIKAKUI: UBlockCode = UBlockCode(235i32);
pub const UBLOCK_MEROITIC_CURSIVE: UBlockCode = UBlockCode(214i32);
pub const UBLOCK_MEROITIC_HIEROGLYPHS: UBlockCode = UBlockCode(215i32);
pub const UBLOCK_MIAO: UBlockCode = UBlockCode(216i32);
pub const UBLOCK_MISCELLANEOUS_MATHEMATICAL_SYMBOLS_A: UBlockCode = UBlockCode(102i32);
pub const UBLOCK_MISCELLANEOUS_MATHEMATICAL_SYMBOLS_B: UBlockCode = UBlockCode(105i32);
pub const UBLOCK_MISCELLANEOUS_SYMBOLS: UBlockCode = UBlockCode(55i32);
pub const UBLOCK_MISCELLANEOUS_SYMBOLS_AND_ARROWS: UBlockCode = UBlockCode(115i32);
pub const UBLOCK_MISCELLANEOUS_SYMBOLS_AND_PICTOGRAPHS: UBlockCode = UBlockCode(205i32);
pub const UBLOCK_MISCELLANEOUS_TECHNICAL: UBlockCode = UBlockCode(48i32);
pub const UBLOCK_MODI: UBlockCode = UBlockCode(236i32);
pub const UBLOCK_MODIFIER_TONE_LETTERS: UBlockCode = UBlockCode(138i32);
pub const UBLOCK_MONGOLIAN: UBlockCode = UBlockCode(37i32);
pub const UBLOCK_MONGOLIAN_SUPPLEMENT: UBlockCode = UBlockCode(269i32);
pub const UBLOCK_MRO: UBlockCode = UBlockCode(237i32);
pub const UBLOCK_MULTANI: UBlockCode = UBlockCode(259i32);
pub const UBLOCK_MUSICAL_SYMBOLS: UBlockCode = UBlockCode(92i32);
pub const UBLOCK_MYANMAR: UBlockCode = UBlockCode(28i32);
pub const UBLOCK_MYANMAR_EXTENDED_A: UBlockCode = UBlockCode(182i32);
pub const UBLOCK_MYANMAR_EXTENDED_B: UBlockCode = UBlockCode(238i32);
pub const UBLOCK_NABATAEAN: UBlockCode = UBlockCode(239i32);
pub const UBLOCK_NANDINAGARI: UBlockCode = UBlockCode(294i32);
pub const UBLOCK_NEWA: UBlockCode = UBlockCode(270i32);
pub const UBLOCK_NEW_TAI_LUE: UBlockCode = UBlockCode(139i32);
pub const UBLOCK_NKO: UBlockCode = UBlockCode(146i32);
pub const UBLOCK_NO_BLOCK: UBlockCode = UBlockCode(0i32);
pub const UBLOCK_NUMBER_FORMS: UBlockCode = UBlockCode(45i32);
pub const UBLOCK_NUSHU: UBlockCode = UBlockCode(277i32);
pub const UBLOCK_NYIAKENG_PUACHUE_HMONG: UBlockCode = UBlockCode(295i32);
pub const UBLOCK_OGHAM: UBlockCode = UBlockCode(34i32);
pub const UBLOCK_OLD_HUNGARIAN: UBlockCode = UBlockCode(260i32);
pub const UBLOCK_OLD_ITALIC: UBlockCode = UBlockCode(88i32);
pub const UBLOCK_OLD_NORTH_ARABIAN: UBlockCode = UBlockCode(240i32);
pub const UBLOCK_OLD_PERMIC: UBlockCode = UBlockCode(241i32);
pub const UBLOCK_OLD_PERSIAN: UBlockCode = UBlockCode(140i32);
pub const UBLOCK_OLD_SOGDIAN: UBlockCode = UBlockCode(290i32);
pub const UBLOCK_OLD_SOUTH_ARABIAN: UBlockCode = UBlockCode(187i32);
pub const UBLOCK_OLD_TURKIC: UBlockCode = UBlockCode(191i32);
pub const UBLOCK_OL_CHIKI: UBlockCode = UBlockCode(157i32);
pub const UBLOCK_OPTICAL_CHARACTER_RECOGNITION: UBlockCode = UBlockCode(50i32);
pub const UBLOCK_ORIYA: UBlockCode = UBlockCode(19i32);
pub const UBLOCK_ORNAMENTAL_DINGBATS: UBlockCode = UBlockCode(242i32);
pub const UBLOCK_OSAGE: UBlockCode = UBlockCode(271i32);
pub const UBLOCK_OSMANYA: UBlockCode = UBlockCode(122i32);
pub const UBLOCK_OTTOMAN_SIYAQ_NUMBERS: UBlockCode = UBlockCode(296i32);
pub const UBLOCK_PAHAWH_HMONG: UBlockCode = UBlockCode(243i32);
pub const UBLOCK_PALMYRENE: UBlockCode = UBlockCode(244i32);
pub const UBLOCK_PAU_CIN_HAU: UBlockCode = UBlockCode(245i32);
pub const UBLOCK_PHAGS_PA: UBlockCode = UBlockCode(150i32);
pub const UBLOCK_PHAISTOS_DISC: UBlockCode = UBlockCode(166i32);
pub const UBLOCK_PHOENICIAN: UBlockCode = UBlockCode(151i32);
pub const UBLOCK_PHONETIC_EXTENSIONS: UBlockCode = UBlockCode(114i32);
pub const UBLOCK_PHONETIC_EXTENSIONS_SUPPLEMENT: UBlockCode = UBlockCode(141i32);
pub const UBLOCK_PLAYING_CARDS: UBlockCode = UBlockCode(204i32);
pub const UBLOCK_PRIVATE_USE: UBlockCode = UBlockCode(78i32);
pub const UBLOCK_PRIVATE_USE_AREA: UBlockCode = UBlockCode(78i32);
pub const UBLOCK_PSALTER_PAHLAVI: UBlockCode = UBlockCode(246i32);
pub const UBLOCK_REJANG: UBlockCode = UBlockCode(163i32);
pub const UBLOCK_RUMI_NUMERAL_SYMBOLS: UBlockCode = UBlockCode(192i32);
pub const UBLOCK_RUNIC: UBlockCode = UBlockCode(35i32);
pub const UBLOCK_SAMARITAN: UBlockCode = UBlockCode(172i32);
pub const UBLOCK_SAURASHTRA: UBlockCode = UBlockCode(161i32);
pub const UBLOCK_SHARADA: UBlockCode = UBlockCode(217i32);
pub const UBLOCK_SHAVIAN: UBlockCode = UBlockCode(121i32);
pub const UBLOCK_SHORTHAND_FORMAT_CONTROLS: UBlockCode = UBlockCode(247i32);
pub const UBLOCK_SIDDHAM: UBlockCode = UBlockCode(248i32);
pub const UBLOCK_SINHALA: UBlockCode = UBlockCode(24i32);
pub const UBLOCK_SINHALA_ARCHAIC_NUMBERS: UBlockCode = UBlockCode(249i32);
pub const UBLOCK_SMALL_FORM_VARIANTS: UBlockCode = UBlockCode(84i32);
pub const UBLOCK_SMALL_KANA_EXTENSION: UBlockCode = UBlockCode(297i32);
pub const UBLOCK_SOGDIAN: UBlockCode = UBlockCode(291i32);
pub const UBLOCK_SORA_SOMPENG: UBlockCode = UBlockCode(218i32);
pub const UBLOCK_SOYOMBO: UBlockCode = UBlockCode(278i32);
pub const UBLOCK_SPACING_MODIFIER_LETTERS: UBlockCode = UBlockCode(6i32);
pub const UBLOCK_SPECIALS: UBlockCode = UBlockCode(86i32);
pub const UBLOCK_SUNDANESE: UBlockCode = UBlockCode(155i32);
pub const UBLOCK_SUNDANESE_SUPPLEMENT: UBlockCode = UBlockCode(219i32);
pub const UBLOCK_SUPERSCRIPTS_AND_SUBSCRIPTS: UBlockCode = UBlockCode(41i32);
pub const UBLOCK_SUPPLEMENTAL_ARROWS_A: UBlockCode = UBlockCode(103i32);
pub const UBLOCK_SUPPLEMENTAL_ARROWS_B: UBlockCode = UBlockCode(104i32);
pub const UBLOCK_SUPPLEMENTAL_ARROWS_C: UBlockCode = UBlockCode(250i32);
pub const UBLOCK_SUPPLEMENTAL_MATHEMATICAL_OPERATORS: UBlockCode = UBlockCode(106i32);
pub const UBLOCK_SUPPLEMENTAL_PUNCTUATION: UBlockCode = UBlockCode(142i32);
pub const UBLOCK_SUPPLEMENTAL_SYMBOLS_AND_PICTOGRAPHS: UBlockCode = UBlockCode(261i32);
pub const UBLOCK_SUPPLEMENTARY_PRIVATE_USE_AREA_A: UBlockCode = UBlockCode(109i32);
pub const UBLOCK_SUPPLEMENTARY_PRIVATE_USE_AREA_B: UBlockCode = UBlockCode(110i32);
pub const UBLOCK_SUTTON_SIGNWRITING: UBlockCode = UBlockCode(262i32);
pub const UBLOCK_SYLOTI_NAGRI: UBlockCode = UBlockCode(143i32);
pub const UBLOCK_SYMBOLS_AND_PICTOGRAPHS_EXTENDED_A: UBlockCode = UBlockCode(298i32);
pub const UBLOCK_SYMBOLS_FOR_LEGACY_COMPUTING: UBlockCode = UBlockCode(306i32);
pub const UBLOCK_SYRIAC: UBlockCode = UBlockCode(13i32);
pub const UBLOCK_SYRIAC_SUPPLEMENT: UBlockCode = UBlockCode(279i32);
pub const UBLOCK_TAGALOG: UBlockCode = UBlockCode(98i32);
pub const UBLOCK_TAGBANWA: UBlockCode = UBlockCode(101i32);
pub const UBLOCK_TAGS: UBlockCode = UBlockCode(96i32);
pub const UBLOCK_TAI_LE: UBlockCode = UBlockCode(112i32);
pub const UBLOCK_TAI_THAM: UBlockCode = UBlockCode(174i32);
pub const UBLOCK_TAI_VIET: UBlockCode = UBlockCode(183i32);
pub const UBLOCK_TAI_XUAN_JING_SYMBOLS: UBlockCode = UBlockCode(124i32);
pub const UBLOCK_TAKRI: UBlockCode = UBlockCode(220i32);
pub const UBLOCK_TAMIL: UBlockCode = UBlockCode(20i32);
pub const UBLOCK_TAMIL_SUPPLEMENT: UBlockCode = UBlockCode(299i32);
pub const UBLOCK_TANGUT: UBlockCode = UBlockCode(272i32);
pub const UBLOCK_TANGUT_COMPONENTS: UBlockCode = UBlockCode(273i32);
pub const UBLOCK_TANGUT_SUPPLEMENT: UBlockCode = UBlockCode(307i32);
pub const UBLOCK_TELUGU: UBlockCode = UBlockCode(21i32);
pub const UBLOCK_THAANA: UBlockCode = UBlockCode(14i32);
pub const UBLOCK_THAI: UBlockCode = UBlockCode(25i32);
pub const UBLOCK_TIBETAN: UBlockCode = UBlockCode(27i32);
pub const UBLOCK_TIFINAGH: UBlockCode = UBlockCode(144i32);
pub const UBLOCK_TIRHUTA: UBlockCode = UBlockCode(251i32);
pub const UBLOCK_TRANSPORT_AND_MAP_SYMBOLS: UBlockCode = UBlockCode(207i32);
pub const UBLOCK_UGARITIC: UBlockCode = UBlockCode(120i32);
pub const UBLOCK_UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS: UBlockCode = UBlockCode(33i32);
pub const UBLOCK_UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS_EXTENDED: UBlockCode = UBlockCode(173i32);
pub const UBLOCK_VAI: UBlockCode = UBlockCode(159i32);
pub const UBLOCK_VARIATION_SELECTORS: UBlockCode = UBlockCode(108i32);
pub const UBLOCK_VARIATION_SELECTORS_SUPPLEMENT: UBlockCode = UBlockCode(125i32);
pub const UBLOCK_VEDIC_EXTENSIONS: UBlockCode = UBlockCode(175i32);
pub const UBLOCK_VERTICAL_FORMS: UBlockCode = UBlockCode(145i32);
pub const UBLOCK_WANCHO: UBlockCode = UBlockCode(300i32);
pub const UBLOCK_WARANG_CITI: UBlockCode = UBlockCode(252i32);
pub const UBLOCK_YEZIDI: UBlockCode = UBlockCode(308i32);
pub const UBLOCK_YIJING_HEXAGRAM_SYMBOLS: UBlockCode = UBlockCode(116i32);
pub const UBLOCK_YI_RADICALS: UBlockCode = UBlockCode(73i32);
pub const UBLOCK_YI_SYLLABLES: UBlockCode = UBlockCode(72i32);
pub const UBLOCK_ZANABAZAR_SQUARE: UBlockCode = UBlockCode(280i32);
pub const UBRK_CHARACTER: UBreakIteratorType = UBreakIteratorType(0i32);
pub const UBRK_LINE: UBreakIteratorType = UBreakIteratorType(2i32);
pub const UBRK_LINE_HARD: ULineBreakTag = ULineBreakTag(100i32);
pub const UBRK_LINE_HARD_LIMIT: ULineBreakTag = ULineBreakTag(200i32);
pub const UBRK_LINE_SOFT: ULineBreakTag = ULineBreakTag(0i32);
pub const UBRK_LINE_SOFT_LIMIT: ULineBreakTag = ULineBreakTag(100i32);
pub const UBRK_SENTENCE: UBreakIteratorType = UBreakIteratorType(3i32);
pub const UBRK_SENTENCE_SEP: USentenceBreakTag = USentenceBreakTag(100i32);
pub const UBRK_SENTENCE_SEP_LIMIT: USentenceBreakTag = USentenceBreakTag(200i32);
pub const UBRK_SENTENCE_TERM: USentenceBreakTag = USentenceBreakTag(0i32);
pub const UBRK_SENTENCE_TERM_LIMIT: USentenceBreakTag = USentenceBreakTag(100i32);
pub const UBRK_WORD: UBreakIteratorType = UBreakIteratorType(1i32);
pub const UBRK_WORD_IDEO: UWordBreak = UWordBreak(400i32);
pub const UBRK_WORD_IDEO_LIMIT: UWordBreak = UWordBreak(500i32);
pub const UBRK_WORD_KANA: UWordBreak = UWordBreak(300i32);
pub const UBRK_WORD_KANA_LIMIT: UWordBreak = UWordBreak(400i32);
pub const UBRK_WORD_LETTER: UWordBreak = UWordBreak(200i32);
pub const UBRK_WORD_LETTER_LIMIT: UWordBreak = UWordBreak(300i32);
pub const UBRK_WORD_NONE: UWordBreak = UWordBreak(0i32);
pub const UBRK_WORD_NONE_LIMIT: UWordBreak = UWordBreak(100i32);
pub const UBRK_WORD_NUMBER: UWordBreak = UWordBreak(100i32);
pub const UBRK_WORD_NUMBER_LIMIT: UWordBreak = UWordBreak(200i32);
pub const UCAL_ACTUAL_MAXIMUM: UCalendarLimitType = UCalendarLimitType(5i32);
pub const UCAL_ACTUAL_MINIMUM: UCalendarLimitType = UCalendarLimitType(4i32);
pub const UCAL_AM: UCalendarAMPMs = UCalendarAMPMs(0i32);
pub const UCAL_AM_PM: UCalendarDateFields = UCalendarDateFields(9i32);
pub const UCAL_APRIL: UCalendarMonths = UCalendarMonths(3i32);
pub const UCAL_AUGUST: UCalendarMonths = UCalendarMonths(7i32);
pub const UCAL_DATE: UCalendarDateFields = UCalendarDateFields(5i32);
pub const UCAL_DAY_OF_MONTH: UCalendarDateFields = UCalendarDateFields(5i32);
pub const UCAL_DAY_OF_WEEK: UCalendarDateFields = UCalendarDateFields(7i32);
pub const UCAL_DAY_OF_WEEK_IN_MONTH: UCalendarDateFields = UCalendarDateFields(8i32);
pub const UCAL_DAY_OF_YEAR: UCalendarDateFields = UCalendarDateFields(6i32);
pub const UCAL_DECEMBER: UCalendarMonths = UCalendarMonths(11i32);
pub const UCAL_DEFAULT: UCalendarType = UCalendarType(0i32);
pub const UCAL_DOW_LOCAL: UCalendarDateFields = UCalendarDateFields(18i32);
pub const UCAL_DST: UCalendarDisplayNameType = UCalendarDisplayNameType(2i32);
pub const UCAL_DST_OFFSET: UCalendarDateFields = UCalendarDateFields(16i32);
pub const UCAL_ERA: UCalendarDateFields = UCalendarDateFields(0i32);
pub const UCAL_EXTENDED_YEAR: UCalendarDateFields = UCalendarDateFields(19i32);
pub const UCAL_FEBRUARY: UCalendarMonths = UCalendarMonths(1i32);
pub const UCAL_FIELD_COUNT: UCalendarDateFields = UCalendarDateFields(23i32);
pub const UCAL_FIRST_DAY_OF_WEEK: UCalendarAttribute = UCalendarAttribute(1i32);
pub const UCAL_FRIDAY: UCalendarDaysOfWeek = UCalendarDaysOfWeek(6i32);
pub const UCAL_GREATEST_MINIMUM: UCalendarLimitType = UCalendarLimitType(2i32);
pub const UCAL_GREGORIAN: UCalendarType = UCalendarType(1i32);
pub const UCAL_HOUR: UCalendarDateFields = UCalendarDateFields(10i32);
pub const UCAL_HOUR_OF_DAY: UCalendarDateFields = UCalendarDateFields(11i32);
pub const UCAL_IS_LEAP_MONTH: UCalendarDateFields = UCalendarDateFields(22i32);
pub const UCAL_JANUARY: UCalendarMonths = UCalendarMonths(0i32);
pub const UCAL_JULIAN_DAY: UCalendarDateFields = UCalendarDateFields(20i32);
pub const UCAL_JULY: UCalendarMonths = UCalendarMonths(6i32);
pub const UCAL_JUNE: UCalendarMonths = UCalendarMonths(5i32);
pub const UCAL_LEAST_MAXIMUM: UCalendarLimitType = UCalendarLimitType(3i32);
pub const UCAL_LENIENT: UCalendarAttribute = UCalendarAttribute(0i32);
pub const UCAL_MARCH: UCalendarMonths = UCalendarMonths(2i32);
pub const UCAL_MAXIMUM: UCalendarLimitType = UCalendarLimitType(1i32);
pub const UCAL_MAY: UCalendarMonths = UCalendarMonths(4i32);
pub const UCAL_MILLISECOND: UCalendarDateFields = UCalendarDateFields(14i32);
pub const UCAL_MILLISECONDS_IN_DAY: UCalendarDateFields = UCalendarDateFields(21i32);
pub const UCAL_MINIMAL_DAYS_IN_FIRST_WEEK: UCalendarAttribute = UCalendarAttribute(2i32);
pub const UCAL_MINIMUM: UCalendarLimitType = UCalendarLimitType(0i32);
pub const UCAL_MINUTE: UCalendarDateFields = UCalendarDateFields(12i32);
pub const UCAL_MONDAY: UCalendarDaysOfWeek = UCalendarDaysOfWeek(2i32);
pub const UCAL_MONTH: UCalendarDateFields = UCalendarDateFields(2i32);
pub const UCAL_NOVEMBER: UCalendarMonths = UCalendarMonths(10i32);
pub const UCAL_OCTOBER: UCalendarMonths = UCalendarMonths(9i32);
pub const UCAL_PM: UCalendarAMPMs = UCalendarAMPMs(1i32);
pub const UCAL_REPEATED_WALL_TIME: UCalendarAttribute = UCalendarAttribute(3i32);
pub const UCAL_SATURDAY: UCalendarDaysOfWeek = UCalendarDaysOfWeek(7i32);
pub const UCAL_SECOND: UCalendarDateFields = UCalendarDateFields(13i32);
pub const UCAL_SEPTEMBER: UCalendarMonths = UCalendarMonths(8i32);
pub const UCAL_SHORT_DST: UCalendarDisplayNameType = UCalendarDisplayNameType(3i32);
pub const UCAL_SHORT_STANDARD: UCalendarDisplayNameType = UCalendarDisplayNameType(1i32);
pub const UCAL_SKIPPED_WALL_TIME: UCalendarAttribute = UCalendarAttribute(4i32);
pub const UCAL_STANDARD: UCalendarDisplayNameType = UCalendarDisplayNameType(0i32);
pub const UCAL_SUNDAY: UCalendarDaysOfWeek = UCalendarDaysOfWeek(1i32);
pub const UCAL_THURSDAY: UCalendarDaysOfWeek = UCalendarDaysOfWeek(5i32);
pub const UCAL_TRADITIONAL: UCalendarType = UCalendarType(0i32);
pub const UCAL_TUESDAY: UCalendarDaysOfWeek = UCalendarDaysOfWeek(3i32);
pub const UCAL_TZ_TRANSITION_NEXT: UTimeZoneTransitionType = UTimeZoneTransitionType(0i32);
pub const UCAL_TZ_TRANSITION_NEXT_INCLUSIVE: UTimeZoneTransitionType = UTimeZoneTransitionType(1i32);
pub const UCAL_TZ_TRANSITION_PREVIOUS: UTimeZoneTransitionType = UTimeZoneTransitionType(2i32);
pub const UCAL_TZ_TRANSITION_PREVIOUS_INCLUSIVE: UTimeZoneTransitionType = UTimeZoneTransitionType(3i32);
pub const UCAL_UNDECIMBER: UCalendarMonths = UCalendarMonths(12i32);
pub const UCAL_UNKNOWN_ZONE_ID: windows_core::PCSTR = windows_core::s!("Etc/Unknown");
pub const UCAL_WALLTIME_FIRST: UCalendarWallTimeOption = UCalendarWallTimeOption(1i32);
pub const UCAL_WALLTIME_LAST: UCalendarWallTimeOption = UCalendarWallTimeOption(0i32);
pub const UCAL_WALLTIME_NEXT_VALID: UCalendarWallTimeOption = UCalendarWallTimeOption(2i32);
pub const UCAL_WEDNESDAY: UCalendarDaysOfWeek = UCalendarDaysOfWeek(4i32);
pub const UCAL_WEEKDAY: UCalendarWeekdayType = UCalendarWeekdayType(0i32);
pub const UCAL_WEEKEND: UCalendarWeekdayType = UCalendarWeekdayType(1i32);
pub const UCAL_WEEKEND_CEASE: UCalendarWeekdayType = UCalendarWeekdayType(3i32);
pub const UCAL_WEEKEND_ONSET: UCalendarWeekdayType = UCalendarWeekdayType(2i32);
pub const UCAL_WEEK_OF_MONTH: UCalendarDateFields = UCalendarDateFields(4i32);
pub const UCAL_WEEK_OF_YEAR: UCalendarDateFields = UCalendarDateFields(3i32);
pub const UCAL_YEAR: UCalendarDateFields = UCalendarDateFields(1i32);
pub const UCAL_YEAR_WOY: UCalendarDateFields = UCalendarDateFields(17i32);
pub const UCAL_ZONE_OFFSET: UCalendarDateFields = UCalendarDateFields(15i32);
pub const UCAL_ZONE_TYPE_ANY: USystemTimeZoneType = USystemTimeZoneType(0i32);
pub const UCAL_ZONE_TYPE_CANONICAL: USystemTimeZoneType = USystemTimeZoneType(1i32);
pub const UCAL_ZONE_TYPE_CANONICAL_LOCATION: USystemTimeZoneType = USystemTimeZoneType(2i32);
pub const UCHAR_AGE: UProperty = UProperty(16384i32);
pub const UCHAR_ALPHABETIC: UProperty = UProperty(0i32);
pub const UCHAR_ASCII_HEX_DIGIT: UProperty = UProperty(1i32);
pub const UCHAR_BIDI_CLASS: UProperty = UProperty(4096i32);
pub const UCHAR_BIDI_CONTROL: UProperty = UProperty(2i32);
pub const UCHAR_BIDI_MIRRORED: UProperty = UProperty(3i32);
pub const UCHAR_BIDI_MIRRORING_GLYPH: UProperty = UProperty(16385i32);
pub const UCHAR_BIDI_PAIRED_BRACKET: UProperty = UProperty(16397i32);
pub const UCHAR_BIDI_PAIRED_BRACKET_TYPE: UProperty = UProperty(4117i32);
pub const UCHAR_BINARY_START: UProperty = UProperty(0i32);
pub const UCHAR_BLOCK: UProperty = UProperty(4097i32);
pub const UCHAR_CANONICAL_COMBINING_CLASS: UProperty = UProperty(4098i32);
pub const UCHAR_CASED: UProperty = UProperty(49i32);
pub const UCHAR_CASE_FOLDING: UProperty = UProperty(16386i32);
pub const UCHAR_CASE_IGNORABLE: UProperty = UProperty(50i32);
pub const UCHAR_CASE_SENSITIVE: UProperty = UProperty(34i32);
pub const UCHAR_CHANGES_WHEN_CASEFOLDED: UProperty = UProperty(54i32);
pub const UCHAR_CHANGES_WHEN_CASEMAPPED: UProperty = UProperty(55i32);
pub const UCHAR_CHANGES_WHEN_LOWERCASED: UProperty = UProperty(51i32);
pub const UCHAR_CHANGES_WHEN_NFKC_CASEFOLDED: UProperty = UProperty(56i32);
pub const UCHAR_CHANGES_WHEN_TITLECASED: UProperty = UProperty(53i32);
pub const UCHAR_CHANGES_WHEN_UPPERCASED: UProperty = UProperty(52i32);
pub const UCHAR_DASH: UProperty = UProperty(4i32);
pub const UCHAR_DECOMPOSITION_TYPE: UProperty = UProperty(4099i32);
pub const UCHAR_DEFAULT_IGNORABLE_CODE_POINT: UProperty = UProperty(5i32);
pub const UCHAR_DEPRECATED: UProperty = UProperty(6i32);
pub const UCHAR_DIACRITIC: UProperty = UProperty(7i32);
pub const UCHAR_DOUBLE_START: UProperty = UProperty(12288i32);
pub const UCHAR_EAST_ASIAN_WIDTH: UProperty = UProperty(4100i32);
pub const UCHAR_EMOJI: UProperty = UProperty(57i32);
pub const UCHAR_EMOJI_COMPONENT: UProperty = UProperty(61i32);
pub const UCHAR_EMOJI_MODIFIER: UProperty = UProperty(59i32);
pub const UCHAR_EMOJI_MODIFIER_BASE: UProperty = UProperty(60i32);
pub const UCHAR_EMOJI_PRESENTATION: UProperty = UProperty(58i32);
pub const UCHAR_EXTENDED_PICTOGRAPHIC: UProperty = UProperty(64i32);
pub const UCHAR_EXTENDER: UProperty = UProperty(8i32);
pub const UCHAR_FULL_COMPOSITION_EXCLUSION: UProperty = UProperty(9i32);
pub const UCHAR_GENERAL_CATEGORY: UProperty = UProperty(4101i32);
pub const UCHAR_GENERAL_CATEGORY_MASK: UProperty = UProperty(8192i32);
pub const UCHAR_GRAPHEME_BASE: UProperty = UProperty(10i32);
pub const UCHAR_GRAPHEME_CLUSTER_BREAK: UProperty = UProperty(4114i32);
pub const UCHAR_GRAPHEME_EXTEND: UProperty = UProperty(11i32);
pub const UCHAR_GRAPHEME_LINK: UProperty = UProperty(12i32);
pub const UCHAR_HANGUL_SYLLABLE_TYPE: UProperty = UProperty(4107i32);
pub const UCHAR_HEX_DIGIT: UProperty = UProperty(13i32);
pub const UCHAR_HYPHEN: UProperty = UProperty(14i32);
pub const UCHAR_IDEOGRAPHIC: UProperty = UProperty(17i32);
pub const UCHAR_IDS_BINARY_OPERATOR: UProperty = UProperty(18i32);
pub const UCHAR_IDS_TRINARY_OPERATOR: UProperty = UProperty(19i32);
pub const UCHAR_ID_CONTINUE: UProperty = UProperty(15i32);
pub const UCHAR_ID_START: UProperty = UProperty(16i32);
pub const UCHAR_INDIC_POSITIONAL_CATEGORY: UProperty = UProperty(4118i32);
pub const UCHAR_INDIC_SYLLABIC_CATEGORY: UProperty = UProperty(4119i32);
pub const UCHAR_INT_START: UProperty = UProperty(4096i32);
pub const UCHAR_INVALID_CODE: UProperty = UProperty(-1i32);
pub const UCHAR_JOINING_GROUP: UProperty = UProperty(4102i32);
pub const UCHAR_JOINING_TYPE: UProperty = UProperty(4103i32);
pub const UCHAR_JOIN_CONTROL: UProperty = UProperty(20i32);
pub const UCHAR_LEAD_CANONICAL_COMBINING_CLASS: UProperty = UProperty(4112i32);
pub const UCHAR_LINE_BREAK: UProperty = UProperty(4104i32);
pub const UCHAR_LOGICAL_ORDER_EXCEPTION: UProperty = UProperty(21i32);
pub const UCHAR_LOWERCASE: UProperty = UProperty(22i32);
pub const UCHAR_LOWERCASE_MAPPING: UProperty = UProperty(16388i32);
pub const UCHAR_MASK_START: UProperty = UProperty(8192i32);
pub const UCHAR_MATH: UProperty = UProperty(23i32);
pub const UCHAR_MAX_VALUE: u32 = 1114111u32;
pub const UCHAR_MIN_VALUE: u32 = 0u32;
pub const UCHAR_NAME: UProperty = UProperty(16389i32);
pub const UCHAR_NFC_INERT: UProperty = UProperty(39i32);
pub const UCHAR_NFC_QUICK_CHECK: UProperty = UProperty(4110i32);
pub const UCHAR_NFD_INERT: UProperty = UProperty(37i32);
pub const UCHAR_NFD_QUICK_CHECK: UProperty = UProperty(4108i32);
pub const UCHAR_NFKC_INERT: UProperty = UProperty(40i32);
pub const UCHAR_NFKC_QUICK_CHECK: UProperty = UProperty(4111i32);
pub const UCHAR_NFKD_INERT: UProperty = UProperty(38i32);
pub const UCHAR_NFKD_QUICK_CHECK: UProperty = UProperty(4109i32);
pub const UCHAR_NONCHARACTER_CODE_POINT: UProperty = UProperty(24i32);
pub const UCHAR_NUMERIC_TYPE: UProperty = UProperty(4105i32);
pub const UCHAR_NUMERIC_VALUE: UProperty = UProperty(12288i32);
pub const UCHAR_OTHER_PROPERTY_START: UProperty = UProperty(28672i32);
pub const UCHAR_PATTERN_SYNTAX: UProperty = UProperty(42i32);
pub const UCHAR_PATTERN_WHITE_SPACE: UProperty = UProperty(43i32);
pub const UCHAR_POSIX_ALNUM: UProperty = UProperty(44i32);
pub const UCHAR_POSIX_BLANK: UProperty = UProperty(45i32);
pub const UCHAR_POSIX_GRAPH: UProperty = UProperty(46i32);
pub const UCHAR_POSIX_PRINT: UProperty = UProperty(47i32);
pub const UCHAR_POSIX_XDIGIT: UProperty = UProperty(48i32);
pub const UCHAR_PREPENDED_CONCATENATION_MARK: UProperty = UProperty(63i32);
pub const UCHAR_QUOTATION_MARK: UProperty = UProperty(25i32);
pub const UCHAR_RADICAL: UProperty = UProperty(26i32);
pub const UCHAR_REGIONAL_INDICATOR: UProperty = UProperty(62i32);
pub const UCHAR_SCRIPT: UProperty = UProperty(4106i32);
pub const UCHAR_SCRIPT_EXTENSIONS: UProperty = UProperty(28672i32);
pub const UCHAR_SEGMENT_STARTER: UProperty = UProperty(41i32);
pub const UCHAR_SENTENCE_BREAK: UProperty = UProperty(4115i32);
pub const UCHAR_SIMPLE_CASE_FOLDING: UProperty = UProperty(16390i32);
pub const UCHAR_SIMPLE_LOWERCASE_MAPPING: UProperty = UProperty(16391i32);
pub const UCHAR_SIMPLE_TITLECASE_MAPPING: UProperty = UProperty(16392i32);
pub const UCHAR_SIMPLE_UPPERCASE_MAPPING: UProperty = UProperty(16393i32);
pub const UCHAR_SOFT_DOTTED: UProperty = UProperty(27i32);
pub const UCHAR_STRING_START: UProperty = UProperty(16384i32);
pub const UCHAR_S_TERM: UProperty = UProperty(35i32);
pub const UCHAR_TERMINAL_PUNCTUATION: UProperty = UProperty(28i32);
pub const UCHAR_TITLECASE_MAPPING: UProperty = UProperty(16394i32);
pub const UCHAR_TRAIL_CANONICAL_COMBINING_CLASS: UProperty = UProperty(4113i32);
pub const UCHAR_UNIFIED_IDEOGRAPH: UProperty = UProperty(29i32);
pub const UCHAR_UPPERCASE: UProperty = UProperty(30i32);
pub const UCHAR_UPPERCASE_MAPPING: UProperty = UProperty(16396i32);
pub const UCHAR_VARIATION_SELECTOR: UProperty = UProperty(36i32);
pub const UCHAR_VERTICAL_ORIENTATION: UProperty = UProperty(4120i32);
pub const UCHAR_WHITE_SPACE: UProperty = UProperty(31i32);
pub const UCHAR_WORD_BREAK: UProperty = UProperty(4116i32);
pub const UCHAR_XID_CONTINUE: UProperty = UProperty(32i32);
pub const UCHAR_XID_START: UProperty = UProperty(33i32);
pub const UCLN_NO_AUTO_CLEANUP: u32 = 1u32;
pub const UCNV_BOCU1: UConverterType = UConverterType(28i32);
pub const UCNV_CESU8: UConverterType = UConverterType(31i32);
pub const UCNV_CLONE: UConverterCallbackReason = UConverterCallbackReason(5i32);
pub const UCNV_CLOSE: UConverterCallbackReason = UConverterCallbackReason(4i32);
pub const UCNV_COMPOUND_TEXT: UConverterType = UConverterType(33i32);
pub const UCNV_DBCS: UConverterType = UConverterType(1i32);
pub const UCNV_EBCDIC_STATEFUL: UConverterType = UConverterType(9i32);
pub const UCNV_ESCAPE_C: windows_core::PCSTR = windows_core::s!("C");
pub const UCNV_ESCAPE_CSS2: windows_core::PCSTR = windows_core::s!("S");
pub const UCNV_ESCAPE_JAVA: windows_core::PCSTR = windows_core::s!("J");
pub const UCNV_ESCAPE_UNICODE: windows_core::PCSTR = windows_core::s!("U");
pub const UCNV_ESCAPE_XML_DEC: windows_core::PCSTR = windows_core::s!("D");
pub const UCNV_ESCAPE_XML_HEX: windows_core::PCSTR = windows_core::s!("X");
pub const UCNV_HZ: UConverterType = UConverterType(23i32);
pub const UCNV_IBM: UConverterPlatform = UConverterPlatform(0i32);
pub const UCNV_ILLEGAL: UConverterCallbackReason = UConverterCallbackReason(1i32);
pub const UCNV_IMAP_MAILBOX: UConverterType = UConverterType(32i32);
pub const UCNV_IRREGULAR: UConverterCallbackReason = UConverterCallbackReason(2i32);
pub const UCNV_ISCII: UConverterType = UConverterType(25i32);
pub const UCNV_ISO_2022: UConverterType = UConverterType(10i32);
pub const UCNV_LATIN_1: UConverterType = UConverterType(3i32);
pub const UCNV_LMBCS_1: UConverterType = UConverterType(11i32);
pub const UCNV_LMBCS_11: UConverterType = UConverterType(18i32);
pub const UCNV_LMBCS_16: UConverterType = UConverterType(19i32);
pub const UCNV_LMBCS_17: UConverterType = UConverterType(20i32);
pub const UCNV_LMBCS_18: UConverterType = UConverterType(21i32);
pub const UCNV_LMBCS_19: UConverterType = UConverterType(22i32);
pub const UCNV_LMBCS_2: UConverterType = UConverterType(12i32);
pub const UCNV_LMBCS_3: UConverterType = UConverterType(13i32);
pub const UCNV_LMBCS_4: UConverterType = UConverterType(14i32);
pub const UCNV_LMBCS_5: UConverterType = UConverterType(15i32);
pub const UCNV_LMBCS_6: UConverterType = UConverterType(16i32);
pub const UCNV_LMBCS_8: UConverterType = UConverterType(17i32);
pub const UCNV_LMBCS_LAST: UConverterType = UConverterType(22i32);
pub const UCNV_LOCALE_OPTION_STRING: windows_core::PCSTR = windows_core::s!(",locale=");
pub const UCNV_MAX_CONVERTER_NAME_LENGTH: u32 = 60u32;
pub const UCNV_MBCS: UConverterType = UConverterType(2i32);
pub const UCNV_NUMBER_OF_SUPPORTED_CONVERTER_TYPES: UConverterType = UConverterType(34i32);
pub const UCNV_OPTION_SEP_STRING: windows_core::PCSTR = windows_core::s!(",");
pub const UCNV_RESET: UConverterCallbackReason = UConverterCallbackReason(3i32);
pub const UCNV_ROUNDTRIP_AND_FALLBACK_SET: UConverterUnicodeSet = UConverterUnicodeSet(1i32);
pub const UCNV_ROUNDTRIP_SET: UConverterUnicodeSet = UConverterUnicodeSet(0i32);
pub const UCNV_SBCS: UConverterType = UConverterType(0i32);
pub const UCNV_SCSU: UConverterType = UConverterType(24i32);
pub const UCNV_SI: u32 = 15u32;
pub const UCNV_SKIP_STOP_ON_ILLEGAL: windows_core::PCSTR = windows_core::s!("i");
pub const UCNV_SO: u32 = 14u32;
pub const UCNV_SUB_STOP_ON_ILLEGAL: windows_core::PCSTR = windows_core::s!("i");
pub const UCNV_SWAP_LFNL_OPTION_STRING: windows_core::PCSTR = windows_core::s!(",swaplfnl");
pub const UCNV_UNASSIGNED: UConverterCallbackReason = UConverterCallbackReason(0i32);
pub const UCNV_UNKNOWN: UConverterPlatform = UConverterPlatform(-1i32);
pub const UCNV_UNSUPPORTED_CONVERTER: UConverterType = UConverterType(-1i32);
pub const UCNV_US_ASCII: UConverterType = UConverterType(26i32);
pub const UCNV_UTF16: UConverterType = UConverterType(29i32);
pub const UCNV_UTF16_BigEndian: UConverterType = UConverterType(5i32);
pub const UCNV_UTF16_LittleEndian: UConverterType = UConverterType(6i32);
pub const UCNV_UTF32: UConverterType = UConverterType(30i32);
pub const UCNV_UTF32_BigEndian: UConverterType = UConverterType(7i32);
pub const UCNV_UTF32_LittleEndian: UConverterType = UConverterType(8i32);
pub const UCNV_UTF7: UConverterType = UConverterType(27i32);
pub const UCNV_UTF8: UConverterType = UConverterType(4i32);
pub const UCNV_VALUE_SEP_STRING: windows_core::PCSTR = windows_core::s!("=");
pub const UCNV_VERSION_OPTION_STRING: windows_core::PCSTR = windows_core::s!(",version=");
pub const UCOL_ALTERNATE_HANDLING: UColAttribute = UColAttribute(1i32);
pub const UCOL_ATTRIBUTE_COUNT: UColAttribute = UColAttribute(8i32);
pub const UCOL_BOUND_LOWER: UColBoundMode = UColBoundMode(0i32);
pub const UCOL_BOUND_UPPER: UColBoundMode = UColBoundMode(1i32);
pub const UCOL_BOUND_UPPER_LONG: UColBoundMode = UColBoundMode(2i32);
pub const UCOL_CASE_FIRST: UColAttribute = UColAttribute(2i32);
pub const UCOL_CASE_LEVEL: UColAttribute = UColAttribute(3i32);
pub const UCOL_CE_STRENGTH_LIMIT: UColAttributeValue = UColAttributeValue(3i32);
pub const UCOL_DECOMPOSITION_MODE: UColAttribute = UColAttribute(4i32);
pub const UCOL_DEFAULT: UColAttributeValue = UColAttributeValue(-1i32);
pub const UCOL_DEFAULT_STRENGTH: UColAttributeValue = UColAttributeValue(2i32);
pub const UCOL_EQUAL: UCollationResult = UCollationResult(0i32);
pub const UCOL_FRENCH_COLLATION: UColAttribute = UColAttribute(0i32);
pub const UCOL_FULL_RULES: UColRuleOption = UColRuleOption(1i32);
pub const UCOL_GREATER: UCollationResult = UCollationResult(1i32);
pub const UCOL_IDENTICAL: UColAttributeValue = UColAttributeValue(15i32);
pub const UCOL_LESS: UCollationResult = UCollationResult(-1i32);
pub const UCOL_LOWER_FIRST: UColAttributeValue = UColAttributeValue(24i32);
pub const UCOL_NON_IGNORABLE: UColAttributeValue = UColAttributeValue(21i32);
pub const UCOL_NORMALIZATION_MODE: UColAttribute = UColAttribute(4i32);
pub const UCOL_NUMERIC_COLLATION: UColAttribute = UColAttribute(7i32);
pub const UCOL_OFF: UColAttributeValue = UColAttributeValue(16i32);
pub const UCOL_ON: UColAttributeValue = UColAttributeValue(17i32);
pub const UCOL_PRIMARY: UColAttributeValue = UColAttributeValue(0i32);
pub const UCOL_QUATERNARY: UColAttributeValue = UColAttributeValue(3i32);
pub const UCOL_REORDER_CODE_CURRENCY: UColReorderCode = UColReorderCode(4099i32);
pub const UCOL_REORDER_CODE_DEFAULT: UColReorderCode = UColReorderCode(-1i32);
pub const UCOL_REORDER_CODE_DIGIT: UColReorderCode = UColReorderCode(4100i32);
pub const UCOL_REORDER_CODE_FIRST: UColReorderCode = UColReorderCode(4096i32);
pub const UCOL_REORDER_CODE_NONE: UColReorderCode = UColReorderCode(103i32);
pub const UCOL_REORDER_CODE_OTHERS: UColReorderCode = UColReorderCode(103i32);
pub const UCOL_REORDER_CODE_PUNCTUATION: UColReorderCode = UColReorderCode(4097i32);
pub const UCOL_REORDER_CODE_SPACE: UColReorderCode = UColReorderCode(4096i32);
pub const UCOL_REORDER_CODE_SYMBOL: UColReorderCode = UColReorderCode(4098i32);
pub const UCOL_SECONDARY: UColAttributeValue = UColAttributeValue(1i32);
pub const UCOL_SHIFTED: UColAttributeValue = UColAttributeValue(20i32);
pub const UCOL_STRENGTH: UColAttribute = UColAttribute(5i32);
pub const UCOL_STRENGTH_LIMIT: UColAttributeValue = UColAttributeValue(16i32);
pub const UCOL_TAILORING_ONLY: UColRuleOption = UColRuleOption(0i32);
pub const UCOL_TERTIARY: UColAttributeValue = UColAttributeValue(2i32);
pub const UCOL_UPPER_FIRST: UColAttributeValue = UColAttributeValue(25i32);
pub const UCONFIG_ENABLE_PLUGINS: u32 = 0u32;
pub const UCONFIG_FORMAT_FASTPATHS_49: u32 = 1u32;
pub const UCONFIG_HAVE_PARSEALLINPUT: u32 = 1u32;
pub const UCONFIG_NO_BREAK_ITERATION: u32 = 1u32;
pub const UCONFIG_NO_COLLATION: u32 = 1u32;
pub const UCONFIG_NO_CONVERSION: u32 = 0u32;
pub const UCONFIG_NO_FILE_IO: u32 = 0u32;
pub const UCONFIG_NO_FILTERED_BREAK_ITERATION: u32 = 0u32;
pub const UCONFIG_NO_FORMATTING: u32 = 1u32;
pub const UCONFIG_NO_IDNA: u32 = 1u32;
pub const UCONFIG_NO_LEGACY_CONVERSION: u32 = 1u32;
pub const UCONFIG_NO_NORMALIZATION: u32 = 0u32;
pub const UCONFIG_NO_REGULAR_EXPRESSIONS: u32 = 1u32;
pub const UCONFIG_NO_SERVICE: u32 = 0u32;
pub const UCONFIG_NO_TRANSLITERATION: u32 = 1u32;
pub const UCONFIG_ONLY_COLLATION: u32 = 0u32;
pub const UCONFIG_ONLY_HTML_CONVERSION: u32 = 0u32;
pub const UCPMAP_RANGE_FIXED_ALL_SURROGATES: UCPMapRangeOption = UCPMapRangeOption(2i32);
pub const UCPMAP_RANGE_FIXED_LEAD_SURROGATES: UCPMapRangeOption = UCPMapRangeOption(1i32);
pub const UCPMAP_RANGE_NORMAL: UCPMapRangeOption = UCPMapRangeOption(0i32);
pub const UCPTRIE_ERROR_VALUE_NEG_DATA_OFFSET: i32 = 1i32;
pub const UCPTRIE_FAST_DATA_BLOCK_LENGTH: i32 = 64i32;
pub const UCPTRIE_FAST_DATA_MASK: i32 = 63i32;
pub const UCPTRIE_FAST_SHIFT: i32 = 6i32;
pub const UCPTRIE_HIGH_VALUE_NEG_DATA_OFFSET: i32 = 2i32;
pub const UCPTRIE_SMALL_MAX: i32 = 4095i32;
pub const UCPTRIE_TYPE_ANY: UCPTrieType = UCPTrieType(-1i32);
pub const UCPTRIE_TYPE_FAST: UCPTrieType = UCPTrieType(0i32);
pub const UCPTRIE_TYPE_SMALL: UCPTrieType = UCPTrieType(1i32);
pub const UCPTRIE_VALUE_BITS_16: UCPTrieValueWidth = UCPTrieValueWidth(0i32);
pub const UCPTRIE_VALUE_BITS_32: UCPTrieValueWidth = UCPTrieValueWidth(1i32);
pub const UCPTRIE_VALUE_BITS_8: UCPTrieValueWidth = UCPTrieValueWidth(2i32);
pub const UCPTRIE_VALUE_BITS_ANY: UCPTrieValueWidth = UCPTrieValueWidth(-1i32);
pub const UCURR_ALL: UCurrCurrencyType = UCurrCurrencyType(2147483647i32);
pub const UCURR_COMMON: UCurrCurrencyType = UCurrCurrencyType(1i32);
pub const UCURR_DEPRECATED: UCurrCurrencyType = UCurrCurrencyType(4i32);
pub const UCURR_LONG_NAME: UCurrNameStyle = UCurrNameStyle(1i32);
pub const UCURR_NARROW_SYMBOL_NAME: UCurrNameStyle = UCurrNameStyle(2i32);
pub const UCURR_NON_DEPRECATED: UCurrCurrencyType = UCurrCurrencyType(8i32);
pub const UCURR_SYMBOL_NAME: UCurrNameStyle = UCurrNameStyle(0i32);
pub const UCURR_UNCOMMON: UCurrCurrencyType = UCurrCurrencyType(2i32);
pub const UCURR_USAGE_CASH: UCurrencyUsage = UCurrencyUsage(1i32);
pub const UCURR_USAGE_STANDARD: UCurrencyUsage = UCurrencyUsage(0i32);
pub const UDATPG_ABBREVIATED: UDateTimePGDisplayWidth = UDateTimePGDisplayWidth(1i32);
pub const UDATPG_BASE_CONFLICT: UDateTimePatternConflict = UDateTimePatternConflict(1i32);
pub const UDATPG_CONFLICT: UDateTimePatternConflict = UDateTimePatternConflict(2i32);
pub const UDATPG_DAYPERIOD_FIELD: UDateTimePatternField = UDateTimePatternField(10i32);
pub const UDATPG_DAY_FIELD: UDateTimePatternField = UDateTimePatternField(9i32);
pub const UDATPG_DAY_OF_WEEK_IN_MONTH_FIELD: UDateTimePatternField = UDateTimePatternField(8i32);
pub const UDATPG_DAY_OF_YEAR_FIELD: UDateTimePatternField = UDateTimePatternField(7i32);
pub const UDATPG_ERA_FIELD: UDateTimePatternField = UDateTimePatternField(0i32);
pub const UDATPG_FIELD_COUNT: UDateTimePatternField = UDateTimePatternField(16i32);
pub const UDATPG_FRACTIONAL_SECOND_FIELD: UDateTimePatternField = UDateTimePatternField(14i32);
pub const UDATPG_HOUR_FIELD: UDateTimePatternField = UDateTimePatternField(11i32);
pub const UDATPG_MATCH_ALL_FIELDS_LENGTH: UDateTimePatternMatchOptions = UDateTimePatternMatchOptions(65535i32);
pub const UDATPG_MATCH_HOUR_FIELD_LENGTH: UDateTimePatternMatchOptions = UDateTimePatternMatchOptions(2048i32);
pub const UDATPG_MATCH_NO_OPTIONS: UDateTimePatternMatchOptions = UDateTimePatternMatchOptions(0i32);
pub const UDATPG_MINUTE_FIELD: UDateTimePatternField = UDateTimePatternField(12i32);
pub const UDATPG_MONTH_FIELD: UDateTimePatternField = UDateTimePatternField(3i32);
pub const UDATPG_NARROW: UDateTimePGDisplayWidth = UDateTimePGDisplayWidth(2i32);
pub const UDATPG_NO_CONFLICT: UDateTimePatternConflict = UDateTimePatternConflict(0i32);
pub const UDATPG_QUARTER_FIELD: UDateTimePatternField = UDateTimePatternField(2i32);
pub const UDATPG_SECOND_FIELD: UDateTimePatternField = UDateTimePatternField(13i32);
pub const UDATPG_WEEKDAY_FIELD: UDateTimePatternField = UDateTimePatternField(6i32);
pub const UDATPG_WEEK_OF_MONTH_FIELD: UDateTimePatternField = UDateTimePatternField(5i32);
pub const UDATPG_WEEK_OF_YEAR_FIELD: UDateTimePatternField = UDateTimePatternField(4i32);
pub const UDATPG_WIDE: UDateTimePGDisplayWidth = UDateTimePGDisplayWidth(0i32);
pub const UDATPG_YEAR_FIELD: UDateTimePatternField = UDateTimePatternField(1i32);
pub const UDATPG_ZONE_FIELD: UDateTimePatternField = UDateTimePatternField(15i32);
pub const UDAT_ABBR_GENERIC_TZ: windows_core::PCSTR = windows_core::s!("v");
pub const UDAT_ABBR_MONTH: windows_core::PCSTR = windows_core::s!("MMM");
pub const UDAT_ABBR_MONTH_DAY: windows_core::PCSTR = windows_core::s!("MMMd");
pub const UDAT_ABBR_MONTH_WEEKDAY_DAY: windows_core::PCSTR = windows_core::s!("MMMEd");
pub const UDAT_ABBR_QUARTER: windows_core::PCSTR = windows_core::s!("QQQ");
pub const UDAT_ABBR_SPECIFIC_TZ: windows_core::PCSTR = windows_core::s!("z");
pub const UDAT_ABBR_UTC_TZ: windows_core::PCSTR = windows_core::s!("ZZZZ");
pub const UDAT_ABBR_WEEKDAY: windows_core::PCSTR = windows_core::s!("E");
pub const UDAT_ABSOLUTE_DAY: UDateAbsoluteUnit = UDateAbsoluteUnit(7i32);
pub const UDAT_ABSOLUTE_FRIDAY: UDateAbsoluteUnit = UDateAbsoluteUnit(5i32);
pub const UDAT_ABSOLUTE_MONDAY: UDateAbsoluteUnit = UDateAbsoluteUnit(1i32);
pub const UDAT_ABSOLUTE_MONTH: UDateAbsoluteUnit = UDateAbsoluteUnit(9i32);
pub const UDAT_ABSOLUTE_NOW: UDateAbsoluteUnit = UDateAbsoluteUnit(11i32);
pub const UDAT_ABSOLUTE_SATURDAY: UDateAbsoluteUnit = UDateAbsoluteUnit(6i32);
pub const UDAT_ABSOLUTE_SUNDAY: UDateAbsoluteUnit = UDateAbsoluteUnit(0i32);
pub const UDAT_ABSOLUTE_THURSDAY: UDateAbsoluteUnit = UDateAbsoluteUnit(4i32);
pub const UDAT_ABSOLUTE_TUESDAY: UDateAbsoluteUnit = UDateAbsoluteUnit(2i32);
pub const UDAT_ABSOLUTE_UNIT_COUNT: UDateAbsoluteUnit = UDateAbsoluteUnit(12i32);
pub const UDAT_ABSOLUTE_WEDNESDAY: UDateAbsoluteUnit = UDateAbsoluteUnit(3i32);
pub const UDAT_ABSOLUTE_WEEK: UDateAbsoluteUnit = UDateAbsoluteUnit(8i32);
pub const UDAT_ABSOLUTE_YEAR: UDateAbsoluteUnit = UDateAbsoluteUnit(10i32);
pub const UDAT_AM_PMS: UDateFormatSymbolType = UDateFormatSymbolType(5i32);
pub const UDAT_AM_PM_FIELD: UDateFormatField = UDateFormatField(14i32);
pub const UDAT_AM_PM_MIDNIGHT_NOON_FIELD: UDateFormatField = UDateFormatField(35i32);
pub const UDAT_BOOLEAN_ATTRIBUTE_COUNT: UDateFormatBooleanAttribute = UDateFormatBooleanAttribute(4i32);
pub const UDAT_CYCLIC_YEARS_ABBREVIATED: UDateFormatSymbolType = UDateFormatSymbolType(23i32);
pub const UDAT_CYCLIC_YEARS_NARROW: UDateFormatSymbolType = UDateFormatSymbolType(24i32);
pub const UDAT_CYCLIC_YEARS_WIDE: UDateFormatSymbolType = UDateFormatSymbolType(22i32);
pub const UDAT_DATE_FIELD: UDateFormatField = UDateFormatField(3i32);
pub const UDAT_DAY: windows_core::PCSTR = windows_core::s!("d");
pub const UDAT_DAY_OF_WEEK_FIELD: UDateFormatField = UDateFormatField(9i32);
pub const UDAT_DAY_OF_WEEK_IN_MONTH_FIELD: UDateFormatField = UDateFormatField(11i32);
pub const UDAT_DAY_OF_YEAR_FIELD: UDateFormatField = UDateFormatField(10i32);
pub const UDAT_DEFAULT: UDateFormatStyle = UDateFormatStyle(2i32);
pub const UDAT_DIRECTION_COUNT: UDateDirection = UDateDirection(6i32);
pub const UDAT_DIRECTION_LAST: UDateDirection = UDateDirection(1i32);
pub const UDAT_DIRECTION_LAST_2: UDateDirection = UDateDirection(0i32);
pub const UDAT_DIRECTION_NEXT: UDateDirection = UDateDirection(3i32);
pub const UDAT_DIRECTION_NEXT_2: UDateDirection = UDateDirection(4i32);
pub const UDAT_DIRECTION_PLAIN: UDateDirection = UDateDirection(5i32);
pub const UDAT_DIRECTION_THIS: UDateDirection = UDateDirection(2i32);
pub const UDAT_DOW_LOCAL_FIELD: UDateFormatField = UDateFormatField(19i32);
pub const UDAT_ERAS: UDateFormatSymbolType = UDateFormatSymbolType(0i32);
pub const UDAT_ERA_FIELD: UDateFormatField = UDateFormatField(0i32);
pub const UDAT_ERA_NAMES: UDateFormatSymbolType = UDateFormatSymbolType(7i32);
pub const UDAT_EXTENDED_YEAR_FIELD: UDateFormatField = UDateFormatField(20i32);
pub const UDAT_FLEXIBLE_DAY_PERIOD_FIELD: UDateFormatField = UDateFormatField(36i32);
pub const UDAT_FRACTIONAL_SECOND_FIELD: UDateFormatField = UDateFormatField(8i32);
pub const UDAT_FULL: UDateFormatStyle = UDateFormatStyle(0i32);
pub const UDAT_FULL_RELATIVE: UDateFormatStyle = UDateFormatStyle(128i32);
pub const UDAT_GENERIC_TZ: windows_core::PCSTR = windows_core::s!("vvvv");
pub const UDAT_HOUR: windows_core::PCSTR = windows_core::s!("j");
pub const UDAT_HOUR0_FIELD: UDateFormatField = UDateFormatField(16i32);
pub const UDAT_HOUR1_FIELD: UDateFormatField = UDateFormatField(15i32);
pub const UDAT_HOUR24: windows_core::PCSTR = windows_core::s!("H");
pub const UDAT_HOUR24_MINUTE: windows_core::PCSTR = windows_core::s!("Hm");
pub const UDAT_HOUR24_MINUTE_SECOND: windows_core::PCSTR = windows_core::s!("Hms");
pub const UDAT_HOUR_MINUTE: windows_core::PCSTR = windows_core::s!("jm");
pub const UDAT_HOUR_MINUTE_SECOND: windows_core::PCSTR = windows_core::s!("jms");
pub const UDAT_HOUR_OF_DAY0_FIELD: UDateFormatField = UDateFormatField(5i32);
pub const UDAT_HOUR_OF_DAY1_FIELD: UDateFormatField = UDateFormatField(4i32);
pub const UDAT_JULIAN_DAY_FIELD: UDateFormatField = UDateFormatField(21i32);
pub const UDAT_LOCALIZED_CHARS: UDateFormatSymbolType = UDateFormatSymbolType(6i32);
pub const UDAT_LOCATION_TZ: windows_core::PCSTR = windows_core::s!("VVVV");
pub const UDAT_LONG: UDateFormatStyle = UDateFormatStyle(1i32);
pub const UDAT_LONG_RELATIVE: UDateFormatStyle = UDateFormatStyle(129i32);
pub const UDAT_MEDIUM: UDateFormatStyle = UDateFormatStyle(2i32);
pub const UDAT_MEDIUM_RELATIVE: UDateFormatStyle = UDateFormatStyle(130i32);
pub const UDAT_MILLISECONDS_IN_DAY_FIELD: UDateFormatField = UDateFormatField(22i32);
pub const UDAT_MINUTE: windows_core::PCSTR = windows_core::s!("m");
pub const UDAT_MINUTE_FIELD: UDateFormatField = UDateFormatField(6i32);
pub const UDAT_MINUTE_SECOND: windows_core::PCSTR = windows_core::s!("ms");
pub const UDAT_MONTH: windows_core::PCSTR = windows_core::s!("MMMM");
pub const UDAT_MONTHS: UDateFormatSymbolType = UDateFormatSymbolType(1i32);
pub const UDAT_MONTH_DAY: windows_core::PCSTR = windows_core::s!("MMMMd");
pub const UDAT_MONTH_FIELD: UDateFormatField = UDateFormatField(2i32);
pub const UDAT_MONTH_WEEKDAY_DAY: windows_core::PCSTR = windows_core::s!("MMMMEEEEd");
pub const UDAT_NARROW_MONTHS: UDateFormatSymbolType = UDateFormatSymbolType(8i32);
pub const UDAT_NARROW_WEEKDAYS: UDateFormatSymbolType = UDateFormatSymbolType(9i32);
pub const UDAT_NONE: UDateFormatStyle = UDateFormatStyle(-1i32);
pub const UDAT_NUM_MONTH: windows_core::PCSTR = windows_core::s!("M");
pub const UDAT_NUM_MONTH_DAY: windows_core::PCSTR = windows_core::s!("Md");
pub const UDAT_NUM_MONTH_WEEKDAY_DAY: windows_core::PCSTR = windows_core::s!("MEd");
pub const UDAT_PARSE_ALLOW_NUMERIC: UDateFormatBooleanAttribute = UDateFormatBooleanAttribute(1i32);
pub const UDAT_PARSE_ALLOW_WHITESPACE: UDateFormatBooleanAttribute = UDateFormatBooleanAttribute(0i32);
pub const UDAT_PARSE_MULTIPLE_PATTERNS_FOR_MATCH: UDateFormatBooleanAttribute = UDateFormatBooleanAttribute(3i32);
pub const UDAT_PARSE_PARTIAL_LITERAL_MATCH: UDateFormatBooleanAttribute = UDateFormatBooleanAttribute(2i32);
pub const UDAT_PATTERN: UDateFormatStyle = UDateFormatStyle(-2i32);
pub const UDAT_QUARTER: windows_core::PCSTR = windows_core::s!("QQQQ");
pub const UDAT_QUARTERS: UDateFormatSymbolType = UDateFormatSymbolType(16i32);
pub const UDAT_QUARTER_FIELD: UDateFormatField = UDateFormatField(27i32);
pub const UDAT_RELATIVE: UDateFormatStyle = UDateFormatStyle(128i32);
pub const UDAT_RELATIVE_DAYS: UDateRelativeUnit = UDateRelativeUnit(3i32);
pub const UDAT_RELATIVE_HOURS: UDateRelativeUnit = UDateRelativeUnit(2i32);
pub const UDAT_RELATIVE_MINUTES: UDateRelativeUnit = UDateRelativeUnit(1i32);
pub const UDAT_RELATIVE_MONTHS: UDateRelativeUnit = UDateRelativeUnit(5i32);
pub const UDAT_RELATIVE_SECONDS: UDateRelativeUnit = UDateRelativeUnit(0i32);
pub const UDAT_RELATIVE_UNIT_COUNT: UDateRelativeUnit = UDateRelativeUnit(7i32);
pub const UDAT_RELATIVE_WEEKS: UDateRelativeUnit = UDateRelativeUnit(4i32);
pub const UDAT_RELATIVE_YEARS: UDateRelativeUnit = UDateRelativeUnit(6i32);
pub const UDAT_REL_LITERAL_FIELD: URelativeDateTimeFormatterField = URelativeDateTimeFormatterField(0i32);
pub const UDAT_REL_NUMERIC_FIELD: URelativeDateTimeFormatterField = URelativeDateTimeFormatterField(1i32);
pub const UDAT_REL_UNIT_DAY: URelativeDateTimeUnit = URelativeDateTimeUnit(4i32);
pub const UDAT_REL_UNIT_FRIDAY: URelativeDateTimeUnit = URelativeDateTimeUnit(13i32);
pub const UDAT_REL_UNIT_HOUR: URelativeDateTimeUnit = URelativeDateTimeUnit(5i32);
pub const UDAT_REL_UNIT_MINUTE: URelativeDateTimeUnit = URelativeDateTimeUnit(6i32);
pub const UDAT_REL_UNIT_MONDAY: URelativeDateTimeUnit = URelativeDateTimeUnit(9i32);
pub const UDAT_REL_UNIT_MONTH: URelativeDateTimeUnit = URelativeDateTimeUnit(2i32);
pub const UDAT_REL_UNIT_QUARTER: URelativeDateTimeUnit = URelativeDateTimeUnit(1i32);
pub const UDAT_REL_UNIT_SATURDAY: URelativeDateTimeUnit = URelativeDateTimeUnit(14i32);
pub const UDAT_REL_UNIT_SECOND: URelativeDateTimeUnit = URelativeDateTimeUnit(7i32);
pub const UDAT_REL_UNIT_SUNDAY: URelativeDateTimeUnit = URelativeDateTimeUnit(8i32);
pub const UDAT_REL_UNIT_THURSDAY: URelativeDateTimeUnit = URelativeDateTimeUnit(12i32);
pub const UDAT_REL_UNIT_TUESDAY: URelativeDateTimeUnit = URelativeDateTimeUnit(10i32);
pub const UDAT_REL_UNIT_WEDNESDAY: URelativeDateTimeUnit = URelativeDateTimeUnit(11i32);
pub const UDAT_REL_UNIT_WEEK: URelativeDateTimeUnit = URelativeDateTimeUnit(3i32);
pub const UDAT_REL_UNIT_YEAR: URelativeDateTimeUnit = URelativeDateTimeUnit(0i32);
pub const UDAT_SECOND: windows_core::PCSTR = windows_core::s!("s");
pub const UDAT_SECOND_FIELD: UDateFormatField = UDateFormatField(7i32);
pub const UDAT_SHORT: UDateFormatStyle = UDateFormatStyle(3i32);
pub const UDAT_SHORTER_WEEKDAYS: UDateFormatSymbolType = UDateFormatSymbolType(20i32);
pub const UDAT_SHORT_MONTHS: UDateFormatSymbolType = UDateFormatSymbolType(2i32);
pub const UDAT_SHORT_QUARTERS: UDateFormatSymbolType = UDateFormatSymbolType(17i32);
pub const UDAT_SHORT_RELATIVE: UDateFormatStyle = UDateFormatStyle(131i32);
pub const UDAT_SHORT_WEEKDAYS: UDateFormatSymbolType = UDateFormatSymbolType(4i32);
pub const UDAT_SPECIFIC_TZ: windows_core::PCSTR = windows_core::s!("zzzz");
pub const UDAT_STANDALONE_DAY_FIELD: UDateFormatField = UDateFormatField(25i32);
pub const UDAT_STANDALONE_MONTHS: UDateFormatSymbolType = UDateFormatSymbolType(10i32);
pub const UDAT_STANDALONE_MONTH_FIELD: UDateFormatField = UDateFormatField(26i32);
pub const UDAT_STANDALONE_NARROW_MONTHS: UDateFormatSymbolType = UDateFormatSymbolType(12i32);
pub const UDAT_STANDALONE_NARROW_WEEKDAYS: UDateFormatSymbolType = UDateFormatSymbolType(15i32);
pub const UDAT_STANDALONE_QUARTERS: UDateFormatSymbolType = UDateFormatSymbolType(18i32);
pub const UDAT_STANDALONE_QUARTER_FIELD: UDateFormatField = UDateFormatField(28i32);
pub const UDAT_STANDALONE_SHORTER_WEEKDAYS: UDateFormatSymbolType = UDateFormatSymbolType(21i32);
pub const UDAT_STANDALONE_SHORT_MONTHS: UDateFormatSymbolType = UDateFormatSymbolType(11i32);
pub const UDAT_STANDALONE_SHORT_QUARTERS: UDateFormatSymbolType = UDateFormatSymbolType(19i32);
pub const UDAT_STANDALONE_SHORT_WEEKDAYS: UDateFormatSymbolType = UDateFormatSymbolType(14i32);
pub const UDAT_STANDALONE_WEEKDAYS: UDateFormatSymbolType = UDateFormatSymbolType(13i32);
pub const UDAT_STYLE_LONG: UDateRelativeDateTimeFormatterStyle = UDateRelativeDateTimeFormatterStyle(0i32);
pub const UDAT_STYLE_NARROW: UDateRelativeDateTimeFormatterStyle = UDateRelativeDateTimeFormatterStyle(2i32);
pub const UDAT_STYLE_SHORT: UDateRelativeDateTimeFormatterStyle = UDateRelativeDateTimeFormatterStyle(1i32);
pub const UDAT_TIMEZONE_FIELD: UDateFormatField = UDateFormatField(17i32);
pub const UDAT_TIMEZONE_GENERIC_FIELD: UDateFormatField = UDateFormatField(24i32);
pub const UDAT_TIMEZONE_ISO_FIELD: UDateFormatField = UDateFormatField(32i32);
pub const UDAT_TIMEZONE_ISO_LOCAL_FIELD: UDateFormatField = UDateFormatField(33i32);
pub const UDAT_TIMEZONE_LOCALIZED_GMT_OFFSET_FIELD: UDateFormatField = UDateFormatField(31i32);
pub const UDAT_TIMEZONE_RFC_FIELD: UDateFormatField = UDateFormatField(23i32);
pub const UDAT_TIMEZONE_SPECIAL_FIELD: UDateFormatField = UDateFormatField(29i32);
pub const UDAT_WEEKDAY: windows_core::PCSTR = windows_core::s!("EEEE");
pub const UDAT_WEEKDAYS: UDateFormatSymbolType = UDateFormatSymbolType(3i32);
pub const UDAT_WEEK_OF_MONTH_FIELD: UDateFormatField = UDateFormatField(13i32);
pub const UDAT_WEEK_OF_YEAR_FIELD: UDateFormatField = UDateFormatField(12i32);
pub const UDAT_YEAR: windows_core::PCSTR = windows_core::s!("y");
pub const UDAT_YEAR_ABBR_MONTH: windows_core::PCSTR = windows_core::s!("yMMM");
pub const UDAT_YEAR_ABBR_MONTH_DAY: windows_core::PCSTR = windows_core::s!("yMMMd");
pub const UDAT_YEAR_ABBR_MONTH_WEEKDAY_DAY: windows_core::PCSTR = windows_core::s!("yMMMEd");
pub const UDAT_YEAR_ABBR_QUARTER: windows_core::PCSTR = windows_core::s!("yQQQ");
pub const UDAT_YEAR_FIELD: UDateFormatField = UDateFormatField(1i32);
pub const UDAT_YEAR_MONTH: windows_core::PCSTR = windows_core::s!("yMMMM");
pub const UDAT_YEAR_MONTH_DAY: windows_core::PCSTR = windows_core::s!("yMMMMd");
pub const UDAT_YEAR_MONTH_WEEKDAY_DAY: windows_core::PCSTR = windows_core::s!("yMMMMEEEEd");
pub const UDAT_YEAR_NAME_FIELD: UDateFormatField = UDateFormatField(30i32);
pub const UDAT_YEAR_NUM_MONTH: windows_core::PCSTR = windows_core::s!("yM");
pub const UDAT_YEAR_NUM_MONTH_DAY: windows_core::PCSTR = windows_core::s!("yMd");
pub const UDAT_YEAR_NUM_MONTH_WEEKDAY_DAY: windows_core::PCSTR = windows_core::s!("yMEd");
pub const UDAT_YEAR_QUARTER: windows_core::PCSTR = windows_core::s!("yQQQQ");
pub const UDAT_YEAR_WOY_FIELD: UDateFormatField = UDateFormatField(18i32);
pub const UDAT_ZODIAC_NAMES_ABBREVIATED: UDateFormatSymbolType = UDateFormatSymbolType(26i32);
pub const UDAT_ZODIAC_NAMES_NARROW: UDateFormatSymbolType = UDateFormatSymbolType(27i32);
pub const UDAT_ZODIAC_NAMES_WIDE: UDateFormatSymbolType = UDateFormatSymbolType(25i32);
pub const UDISPCTX_CAPITALIZATION_FOR_BEGINNING_OF_SENTENCE: UDisplayContext = UDisplayContext(258i32);
pub const UDISPCTX_CAPITALIZATION_FOR_MIDDLE_OF_SENTENCE: UDisplayContext = UDisplayContext(257i32);
pub const UDISPCTX_CAPITALIZATION_FOR_STANDALONE: UDisplayContext = UDisplayContext(260i32);
pub const UDISPCTX_CAPITALIZATION_FOR_UI_LIST_OR_MENU: UDisplayContext = UDisplayContext(259i32);
pub const UDISPCTX_CAPITALIZATION_NONE: UDisplayContext = UDisplayContext(256i32);
pub const UDISPCTX_DIALECT_NAMES: UDisplayContext = UDisplayContext(1i32);
pub const UDISPCTX_LENGTH_FULL: UDisplayContext = UDisplayContext(512i32);
pub const UDISPCTX_LENGTH_SHORT: UDisplayContext = UDisplayContext(513i32);
pub const UDISPCTX_NO_SUBSTITUTE: UDisplayContext = UDisplayContext(769i32);
pub const UDISPCTX_STANDARD_NAMES: UDisplayContext = UDisplayContext(0i32);
pub const UDISPCTX_SUBSTITUTE: UDisplayContext = UDisplayContext(768i32);
pub const UDISPCTX_TYPE_CAPITALIZATION: UDisplayContextType = UDisplayContextType(1i32);
pub const UDISPCTX_TYPE_DIALECT_HANDLING: UDisplayContextType = UDisplayContextType(0i32);
pub const UDISPCTX_TYPE_DISPLAY_LENGTH: UDisplayContextType = UDisplayContextType(2i32);
pub const UDISPCTX_TYPE_SUBSTITUTE_HANDLING: UDisplayContextType = UDisplayContextType(3i32);
pub const UDTS_DB2_TIME: UDateTimeScale = UDateTimeScale(8i32);
pub const UDTS_DOTNET_DATE_TIME: UDateTimeScale = UDateTimeScale(4i32);
pub const UDTS_EXCEL_TIME: UDateTimeScale = UDateTimeScale(7i32);
pub const UDTS_ICU4C_TIME: UDateTimeScale = UDateTimeScale(2i32);
pub const UDTS_JAVA_TIME: UDateTimeScale = UDateTimeScale(0i32);
pub const UDTS_MAC_OLD_TIME: UDateTimeScale = UDateTimeScale(5i32);
pub const UDTS_MAC_TIME: UDateTimeScale = UDateTimeScale(6i32);
pub const UDTS_UNIX_MICROSECONDS_TIME: UDateTimeScale = UDateTimeScale(9i32);
pub const UDTS_UNIX_TIME: UDateTimeScale = UDateTimeScale(1i32);
pub const UDTS_WINDOWS_FILE_TIME: UDateTimeScale = UDateTimeScale(3i32);
pub const UFIELD_CATEGORY_DATE: UFieldCategory = UFieldCategory(1i32);
pub const UFIELD_CATEGORY_DATE_INTERVAL: UFieldCategory = UFieldCategory(5i32);
pub const UFIELD_CATEGORY_DATE_INTERVAL_SPAN: UFieldCategory = UFieldCategory(4101i32);
pub const UFIELD_CATEGORY_LIST: UFieldCategory = UFieldCategory(3i32);
pub const UFIELD_CATEGORY_LIST_SPAN: UFieldCategory = UFieldCategory(4099i32);
pub const UFIELD_CATEGORY_NUMBER: UFieldCategory = UFieldCategory(2i32);
pub const UFIELD_CATEGORY_RELATIVE_DATETIME: UFieldCategory = UFieldCategory(4i32);
pub const UFIELD_CATEGORY_UNDEFINED: UFieldCategory = UFieldCategory(0i32);
pub const UFMT_ARRAY: UFormattableType = UFormattableType(4i32);
pub const UFMT_DATE: UFormattableType = UFormattableType(0i32);
pub const UFMT_DOUBLE: UFormattableType = UFormattableType(1i32);
pub const UFMT_INT64: UFormattableType = UFormattableType(5i32);
pub const UFMT_LONG: UFormattableType = UFormattableType(2i32);
pub const UFMT_OBJECT: UFormattableType = UFormattableType(6i32);
pub const UFMT_STRING: UFormattableType = UFormattableType(3i32);
pub const UGENDER_FEMALE: UGender = UGender(1i32);
pub const UGENDER_MALE: UGender = UGender(0i32);
pub const UGENDER_OTHER: UGender = UGender(2i32);
pub const UIDNA_CHECK_BIDI: i32 = 4i32;
pub const UIDNA_CHECK_CONTEXTJ: i32 = 8i32;
pub const UIDNA_CHECK_CONTEXTO: i32 = 64i32;
pub const UIDNA_DEFAULT: i32 = 0i32;
pub const UIDNA_ERROR_BIDI: i32 = 2048i32;
pub const UIDNA_ERROR_CONTEXTJ: i32 = 4096i32;
pub const UIDNA_ERROR_CONTEXTO_DIGITS: i32 = 16384i32;
pub const UIDNA_ERROR_CONTEXTO_PUNCTUATION: i32 = 8192i32;
pub const UIDNA_ERROR_DISALLOWED: i32 = 128i32;
pub const UIDNA_ERROR_DOMAIN_NAME_TOO_LONG: i32 = 4i32;
pub const UIDNA_ERROR_EMPTY_LABEL: i32 = 1i32;
pub const UIDNA_ERROR_HYPHEN_3_4: i32 = 32i32;
pub const UIDNA_ERROR_INVALID_ACE_LABEL: i32 = 1024i32;
pub const UIDNA_ERROR_LABEL_HAS_DOT: i32 = 512i32;
pub const UIDNA_ERROR_LABEL_TOO_LONG: i32 = 2i32;
pub const UIDNA_ERROR_LEADING_COMBINING_MARK: i32 = 64i32;
pub const UIDNA_ERROR_LEADING_HYPHEN: i32 = 8i32;
pub const UIDNA_ERROR_PUNYCODE: i32 = 256i32;
pub const UIDNA_ERROR_TRAILING_HYPHEN: i32 = 16i32;
pub const UIDNA_NONTRANSITIONAL_TO_ASCII: i32 = 16i32;
pub const UIDNA_NONTRANSITIONAL_TO_UNICODE: i32 = 32i32;
pub const UIDNA_USE_STD3_RULES: i32 = 2i32;
pub const UITER_CURRENT: UCharIteratorOrigin = UCharIteratorOrigin(1i32);
pub const UITER_LENGTH: UCharIteratorOrigin = UCharIteratorOrigin(4i32);
pub const UITER_LIMIT: UCharIteratorOrigin = UCharIteratorOrigin(2i32);
pub const UITER_START: UCharIteratorOrigin = UCharIteratorOrigin(0i32);
pub const UITER_UNKNOWN_INDEX: i32 = -2i32;
pub const UITER_ZERO: UCharIteratorOrigin = UCharIteratorOrigin(3i32);
pub const ULDN_DIALECT_NAMES: UDialectHandling = UDialectHandling(1i32);
pub const ULDN_STANDARD_NAMES: UDialectHandling = UDialectHandling(0i32);
pub const ULISTFMT_ELEMENT_FIELD: UListFormatterField = UListFormatterField(1i32);
pub const ULISTFMT_LITERAL_FIELD: UListFormatterField = UListFormatterField(0i32);
pub const ULISTFMT_TYPE_AND: UListFormatterType = UListFormatterType(0i32);
pub const ULISTFMT_TYPE_OR: UListFormatterType = UListFormatterType(1i32);
pub const ULISTFMT_TYPE_UNITS: UListFormatterType = UListFormatterType(2i32);
pub const ULISTFMT_WIDTH_NARROW: UListFormatterWidth = UListFormatterWidth(2i32);
pub const ULISTFMT_WIDTH_SHORT: UListFormatterWidth = UListFormatterWidth(1i32);
pub const ULISTFMT_WIDTH_WIDE: UListFormatterWidth = UListFormatterWidth(0i32);
pub const ULOCDATA_ALT_QUOTATION_END: ULocaleDataDelimiterType = ULocaleDataDelimiterType(3i32);
pub const ULOCDATA_ALT_QUOTATION_START: ULocaleDataDelimiterType = ULocaleDataDelimiterType(2i32);
pub const ULOCDATA_ES_AUXILIARY: ULocaleDataExemplarSetType = ULocaleDataExemplarSetType(1i32);
pub const ULOCDATA_ES_INDEX: ULocaleDataExemplarSetType = ULocaleDataExemplarSetType(2i32);
pub const ULOCDATA_ES_PUNCTUATION: ULocaleDataExemplarSetType = ULocaleDataExemplarSetType(3i32);
pub const ULOCDATA_ES_STANDARD: ULocaleDataExemplarSetType = ULocaleDataExemplarSetType(0i32);
pub const ULOCDATA_QUOTATION_END: ULocaleDataDelimiterType = ULocaleDataDelimiterType(1i32);
pub const ULOCDATA_QUOTATION_START: ULocaleDataDelimiterType = ULocaleDataDelimiterType(0i32);
pub const ULOC_ACCEPT_FAILED: UAcceptResult = UAcceptResult(0i32);
pub const ULOC_ACCEPT_FALLBACK: UAcceptResult = UAcceptResult(2i32);
pub const ULOC_ACCEPT_VALID: UAcceptResult = UAcceptResult(1i32);
pub const ULOC_ACTUAL_LOCALE: ULocDataLocaleType = ULocDataLocaleType(0i32);
pub const ULOC_AVAILABLE_DEFAULT: ULocAvailableType = ULocAvailableType(0i32);
pub const ULOC_AVAILABLE_ONLY_LEGACY_ALIASES: ULocAvailableType = ULocAvailableType(1i32);
pub const ULOC_AVAILABLE_WITH_LEGACY_ALIASES: ULocAvailableType = ULocAvailableType(2i32);
pub const ULOC_CANADA: windows_core::PCSTR = windows_core::s!("en_CA");
pub const ULOC_CANADA_FRENCH: windows_core::PCSTR = windows_core::s!("fr_CA");
pub const ULOC_CHINA: windows_core::PCSTR = windows_core::s!("zh_CN");
pub const ULOC_CHINESE: windows_core::PCSTR = windows_core::s!("zh");
pub const ULOC_COUNTRY_CAPACITY: u32 = 4u32;
pub const ULOC_ENGLISH: windows_core::PCSTR = windows_core::s!("en");
pub const ULOC_FRANCE: windows_core::PCSTR = windows_core::s!("fr_FR");
pub const ULOC_FRENCH: windows_core::PCSTR = windows_core::s!("fr");
pub const ULOC_FULLNAME_CAPACITY: u32 = 157u32;
pub const ULOC_GERMAN: windows_core::PCSTR = windows_core::s!("de");
pub const ULOC_GERMANY: windows_core::PCSTR = windows_core::s!("de_DE");
pub const ULOC_ITALIAN: windows_core::PCSTR = windows_core::s!("it");
pub const ULOC_ITALY: windows_core::PCSTR = windows_core::s!("it_IT");
pub const ULOC_JAPAN: windows_core::PCSTR = windows_core::s!("ja_JP");
pub const ULOC_JAPANESE: windows_core::PCSTR = windows_core::s!("ja");
pub const ULOC_KEYWORDS_CAPACITY: u32 = 96u32;
pub const ULOC_KEYWORD_AND_VALUES_CAPACITY: u32 = 100u32;
pub const ULOC_KEYWORD_ASSIGN_UNICODE: u32 = 61u32;
pub const ULOC_KEYWORD_ITEM_SEPARATOR_UNICODE: u32 = 59u32;
pub const ULOC_KEYWORD_SEPARATOR_UNICODE: u32 = 64u32;
pub const ULOC_KOREA: windows_core::PCSTR = windows_core::s!("ko_KR");
pub const ULOC_KOREAN: windows_core::PCSTR = windows_core::s!("ko");
pub const ULOC_LANG_CAPACITY: u32 = 12u32;
pub const ULOC_LAYOUT_BTT: ULayoutType = ULayoutType(3i32);
pub const ULOC_LAYOUT_LTR: ULayoutType = ULayoutType(0i32);
pub const ULOC_LAYOUT_RTL: ULayoutType = ULayoutType(1i32);
pub const ULOC_LAYOUT_TTB: ULayoutType = ULayoutType(2i32);
pub const ULOC_LAYOUT_UNKNOWN: ULayoutType = ULayoutType(4i32);
pub const ULOC_PRC: windows_core::PCSTR = windows_core::s!("zh_CN");
pub const ULOC_SCRIPT_CAPACITY: u32 = 6u32;
pub const ULOC_SIMPLIFIED_CHINESE: windows_core::PCSTR = windows_core::s!("zh_CN");
pub const ULOC_TAIWAN: windows_core::PCSTR = windows_core::s!("zh_TW");
pub const ULOC_TRADITIONAL_CHINESE: windows_core::PCSTR = windows_core::s!("zh_TW");
pub const ULOC_UK: windows_core::PCSTR = windows_core::s!("en_GB");
pub const ULOC_US: windows_core::PCSTR = windows_core::s!("en_US");
pub const ULOC_VALID_LOCALE: ULocDataLocaleType = ULocDataLocaleType(1i32);
pub const UMEASFMT_WIDTH_COUNT: UMeasureFormatWidth = UMeasureFormatWidth(4i32);
pub const UMEASFMT_WIDTH_NARROW: UMeasureFormatWidth = UMeasureFormatWidth(2i32);
pub const UMEASFMT_WIDTH_NUMERIC: UMeasureFormatWidth = UMeasureFormatWidth(3i32);
pub const UMEASFMT_WIDTH_SHORT: UMeasureFormatWidth = UMeasureFormatWidth(1i32);
pub const UMEASFMT_WIDTH_WIDE: UMeasureFormatWidth = UMeasureFormatWidth(0i32);
pub const UMSGPAT_APOS_DOUBLE_OPTIONAL: UMessagePatternApostropheMode = UMessagePatternApostropheMode(0i32);
pub const UMSGPAT_APOS_DOUBLE_REQUIRED: UMessagePatternApostropheMode = UMessagePatternApostropheMode(1i32);
pub const UMSGPAT_ARG_NAME_NOT_NUMBER: i32 = -1i32;
pub const UMSGPAT_ARG_NAME_NOT_VALID: i32 = -2i32;
pub const UMSGPAT_ARG_TYPE_CHOICE: UMessagePatternArgType = UMessagePatternArgType(2i32);
pub const UMSGPAT_ARG_TYPE_NONE: UMessagePatternArgType = UMessagePatternArgType(0i32);
pub const UMSGPAT_ARG_TYPE_PLURAL: UMessagePatternArgType = UMessagePatternArgType(3i32);
pub const UMSGPAT_ARG_TYPE_SELECT: UMessagePatternArgType = UMessagePatternArgType(4i32);
pub const UMSGPAT_ARG_TYPE_SELECTORDINAL: UMessagePatternArgType = UMessagePatternArgType(5i32);
pub const UMSGPAT_ARG_TYPE_SIMPLE: UMessagePatternArgType = UMessagePatternArgType(1i32);
pub const UMSGPAT_PART_TYPE_ARG_DOUBLE: UMessagePatternPartType = UMessagePatternPartType(13i32);
pub const UMSGPAT_PART_TYPE_ARG_INT: UMessagePatternPartType = UMessagePatternPartType(12i32);
pub const UMSGPAT_PART_TYPE_ARG_LIMIT: UMessagePatternPartType = UMessagePatternPartType(6i32);
pub const UMSGPAT_PART_TYPE_ARG_NAME: UMessagePatternPartType = UMessagePatternPartType(8i32);
pub const UMSGPAT_PART_TYPE_ARG_NUMBER: UMessagePatternPartType = UMessagePatternPartType(7i32);
pub const UMSGPAT_PART_TYPE_ARG_SELECTOR: UMessagePatternPartType = UMessagePatternPartType(11i32);
pub const UMSGPAT_PART_TYPE_ARG_START: UMessagePatternPartType = UMessagePatternPartType(5i32);
pub const UMSGPAT_PART_TYPE_ARG_STYLE: UMessagePatternPartType = UMessagePatternPartType(10i32);
pub const UMSGPAT_PART_TYPE_ARG_TYPE: UMessagePatternPartType = UMessagePatternPartType(9i32);
pub const UMSGPAT_PART_TYPE_INSERT_CHAR: UMessagePatternPartType = UMessagePatternPartType(3i32);
pub const UMSGPAT_PART_TYPE_MSG_LIMIT: UMessagePatternPartType = UMessagePatternPartType(1i32);
pub const UMSGPAT_PART_TYPE_MSG_START: UMessagePatternPartType = UMessagePatternPartType(0i32);
pub const UMSGPAT_PART_TYPE_REPLACE_NUMBER: UMessagePatternPartType = UMessagePatternPartType(4i32);
pub const UMSGPAT_PART_TYPE_SKIP_SYNTAX: UMessagePatternPartType = UMessagePatternPartType(2i32);
pub const UMS_SI: UMeasurementSystem = UMeasurementSystem(0i32);
pub const UMS_UK: UMeasurementSystem = UMeasurementSystem(2i32);
pub const UMS_US: UMeasurementSystem = UMeasurementSystem(1i32);
pub const UNISCRIBE_OPENTYPE: u32 = 256u32;
pub const UNORM2_COMPOSE: UNormalization2Mode = UNormalization2Mode(0i32);
pub const UNORM2_COMPOSE_CONTIGUOUS: UNormalization2Mode = UNormalization2Mode(3i32);
pub const UNORM2_DECOMPOSE: UNormalization2Mode = UNormalization2Mode(1i32);
pub const UNORM2_FCD: UNormalization2Mode = UNormalization2Mode(2i32);
pub const UNORM_DEFAULT: UNormalizationMode = UNormalizationMode(4i32);
pub const UNORM_FCD: UNormalizationMode = UNormalizationMode(6i32);
pub const UNORM_INPUT_IS_FCD: u32 = 131072u32;
pub const UNORM_MAYBE: UNormalizationCheckResult = UNormalizationCheckResult(2i32);
pub const UNORM_MODE_COUNT: UNormalizationMode = UNormalizationMode(7i32);
pub const UNORM_NFC: UNormalizationMode = UNormalizationMode(4i32);
pub const UNORM_NFD: UNormalizationMode = UNormalizationMode(2i32);
pub const UNORM_NFKC: UNormalizationMode = UNormalizationMode(5i32);
pub const UNORM_NFKD: UNormalizationMode = UNormalizationMode(3i32);
pub const UNORM_NO: UNormalizationCheckResult = UNormalizationCheckResult(0i32);
pub const UNORM_NONE: UNormalizationMode = UNormalizationMode(1i32);
pub const UNORM_YES: UNormalizationCheckResult = UNormalizationCheckResult(1i32);
pub const UNUM_CASH_CURRENCY: UNumberFormatStyle = UNumberFormatStyle(13i32);
pub const UNUM_COMPACT_FIELD: UNumberFormatFields = UNumberFormatFields(12i32);
pub const UNUM_CURRENCY: UNumberFormatStyle = UNumberFormatStyle(2i32);
pub const UNUM_CURRENCY_ACCOUNTING: UNumberFormatStyle = UNumberFormatStyle(12i32);
pub const UNUM_CURRENCY_CODE: UNumberFormatTextAttribute = UNumberFormatTextAttribute(5i32);
pub const UNUM_CURRENCY_FIELD: UNumberFormatFields = UNumberFormatFields(7i32);
pub const UNUM_CURRENCY_INSERT: UCurrencySpacing = UCurrencySpacing(2i32);
pub const UNUM_CURRENCY_ISO: UNumberFormatStyle = UNumberFormatStyle(10i32);
pub const UNUM_CURRENCY_MATCH: UCurrencySpacing = UCurrencySpacing(0i32);
pub const UNUM_CURRENCY_PLURAL: UNumberFormatStyle = UNumberFormatStyle(11i32);
pub const UNUM_CURRENCY_SPACING_COUNT: UCurrencySpacing = UCurrencySpacing(3i32);
pub const UNUM_CURRENCY_STANDARD: UNumberFormatStyle = UNumberFormatStyle(16i32);
pub const UNUM_CURRENCY_SURROUNDING_MATCH: UCurrencySpacing = UCurrencySpacing(1i32);
pub const UNUM_CURRENCY_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(8i32);
pub const UNUM_CURRENCY_USAGE: UNumberFormatAttribute = UNumberFormatAttribute(23i32);
pub const UNUM_DECIMAL: UNumberFormatStyle = UNumberFormatStyle(1i32);
pub const UNUM_DECIMAL_ALWAYS_SHOWN: UNumberFormatAttribute = UNumberFormatAttribute(2i32);
pub const UNUM_DECIMAL_COMPACT_LONG: UNumberFormatStyle = UNumberFormatStyle(15i32);
pub const UNUM_DECIMAL_COMPACT_SHORT: UNumberFormatStyle = UNumberFormatStyle(14i32);
pub const UNUM_DECIMAL_SEPARATOR_ALWAYS: UNumberDecimalSeparatorDisplay = UNumberDecimalSeparatorDisplay(1i32);
pub const UNUM_DECIMAL_SEPARATOR_AUTO: UNumberDecimalSeparatorDisplay = UNumberDecimalSeparatorDisplay(0i32);
pub const UNUM_DECIMAL_SEPARATOR_COUNT: UNumberDecimalSeparatorDisplay = UNumberDecimalSeparatorDisplay(2i32);
pub const UNUM_DECIMAL_SEPARATOR_FIELD: UNumberFormatFields = UNumberFormatFields(2i32);
pub const UNUM_DECIMAL_SEPARATOR_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(0i32);
pub const UNUM_DEFAULT: UNumberFormatStyle = UNumberFormatStyle(1i32);
pub const UNUM_DEFAULT_RULESET: UNumberFormatTextAttribute = UNumberFormatTextAttribute(6i32);
pub const UNUM_DIGIT_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(5i32);
pub const UNUM_DURATION: UNumberFormatStyle = UNumberFormatStyle(7i32);
pub const UNUM_EIGHT_DIGIT_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(25i32);
pub const UNUM_EXPONENTIAL_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(11i32);
pub const UNUM_EXPONENT_FIELD: UNumberFormatFields = UNumberFormatFields(5i32);
pub const UNUM_EXPONENT_MULTIPLICATION_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(27i32);
pub const UNUM_EXPONENT_SIGN_FIELD: UNumberFormatFields = UNumberFormatFields(4i32);
pub const UNUM_EXPONENT_SYMBOL_FIELD: UNumberFormatFields = UNumberFormatFields(3i32);
pub const UNUM_FIVE_DIGIT_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(22i32);
pub const UNUM_FORMAT_ATTRIBUTE_VALUE_HIDDEN: UNumberFormatAttributeValue = UNumberFormatAttributeValue(0i32);
pub const UNUM_FORMAT_FAIL_IF_MORE_THAN_MAX_DIGITS: UNumberFormatAttribute = UNumberFormatAttribute(4096i32);
pub const UNUM_FORMAT_WIDTH: UNumberFormatAttribute = UNumberFormatAttribute(13i32);
pub const UNUM_FOUR_DIGIT_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(21i32);
pub const UNUM_FRACTION_DIGITS: UNumberFormatAttribute = UNumberFormatAttribute(8i32);
pub const UNUM_FRACTION_FIELD: UNumberFormatFields = UNumberFormatFields(1i32);
pub const UNUM_GROUPING_AUTO: UNumberGroupingStrategy = UNumberGroupingStrategy(2i32);
pub const UNUM_GROUPING_MIN2: UNumberGroupingStrategy = UNumberGroupingStrategy(1i32);
pub const UNUM_GROUPING_OFF: UNumberGroupingStrategy = UNumberGroupingStrategy(0i32);
pub const UNUM_GROUPING_ON_ALIGNED: UNumberGroupingStrategy = UNumberGroupingStrategy(3i32);
pub const UNUM_GROUPING_SEPARATOR_FIELD: UNumberFormatFields = UNumberFormatFields(6i32);
pub const UNUM_GROUPING_SEPARATOR_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(1i32);
pub const UNUM_GROUPING_SIZE: UNumberFormatAttribute = UNumberFormatAttribute(10i32);
pub const UNUM_GROUPING_THOUSANDS: UNumberGroupingStrategy = UNumberGroupingStrategy(4i32);
pub const UNUM_GROUPING_USED: UNumberFormatAttribute = UNumberFormatAttribute(1i32);
pub const UNUM_IDENTITY_FALLBACK_APPROXIMATELY: UNumberRangeIdentityFallback = UNumberRangeIdentityFallback(2i32);
pub const UNUM_IDENTITY_FALLBACK_APPROXIMATELY_OR_SINGLE_VALUE: UNumberRangeIdentityFallback = UNumberRangeIdentityFallback(1i32);
pub const UNUM_IDENTITY_FALLBACK_RANGE: UNumberRangeIdentityFallback = UNumberRangeIdentityFallback(3i32);
pub const UNUM_IDENTITY_FALLBACK_SINGLE_VALUE: UNumberRangeIdentityFallback = UNumberRangeIdentityFallback(0i32);
pub const UNUM_IDENTITY_RESULT_EQUAL_AFTER_ROUNDING: UNumberRangeIdentityResult = UNumberRangeIdentityResult(1i32);
pub const UNUM_IDENTITY_RESULT_EQUAL_BEFORE_ROUNDING: UNumberRangeIdentityResult = UNumberRangeIdentityResult(0i32);
pub const UNUM_IDENTITY_RESULT_NOT_EQUAL: UNumberRangeIdentityResult = UNumberRangeIdentityResult(2i32);
pub const UNUM_IGNORE: UNumberFormatStyle = UNumberFormatStyle(0i32);
pub const UNUM_INFINITY_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(14i32);
pub const UNUM_INTEGER_DIGITS: UNumberFormatAttribute = UNumberFormatAttribute(5i32);
pub const UNUM_INTEGER_FIELD: UNumberFormatFields = UNumberFormatFields(0i32);
pub const UNUM_INTL_CURRENCY_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(9i32);
pub const UNUM_LENIENT_PARSE: UNumberFormatAttribute = UNumberFormatAttribute(19i32);
pub const UNUM_LONG: UNumberCompactStyle = UNumberCompactStyle(1i32);
pub const UNUM_MAX_FRACTION_DIGITS: UNumberFormatAttribute = UNumberFormatAttribute(6i32);
pub const UNUM_MAX_INTEGER_DIGITS: UNumberFormatAttribute = UNumberFormatAttribute(3i32);
pub const UNUM_MAX_SIGNIFICANT_DIGITS: UNumberFormatAttribute = UNumberFormatAttribute(18i32);
pub const UNUM_MEASURE_UNIT_FIELD: UNumberFormatFields = UNumberFormatFields(11i32);
pub const UNUM_MINIMUM_GROUPING_DIGITS: UNumberFormatAttribute = UNumberFormatAttribute(22i32);
pub const UNUM_MINUS_SIGN_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(6i32);
pub const UNUM_MIN_FRACTION_DIGITS: UNumberFormatAttribute = UNumberFormatAttribute(7i32);
pub const UNUM_MIN_INTEGER_DIGITS: UNumberFormatAttribute = UNumberFormatAttribute(4i32);
pub const UNUM_MIN_SIGNIFICANT_DIGITS: UNumberFormatAttribute = UNumberFormatAttribute(17i32);
pub const UNUM_MONETARY_GROUPING_SEPARATOR_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(17i32);
pub const UNUM_MONETARY_SEPARATOR_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(10i32);
pub const UNUM_MULTIPLIER: UNumberFormatAttribute = UNumberFormatAttribute(9i32);
pub const UNUM_NAN_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(15i32);
pub const UNUM_NEGATIVE_PREFIX: UNumberFormatTextAttribute = UNumberFormatTextAttribute(2i32);
pub const UNUM_NEGATIVE_SUFFIX: UNumberFormatTextAttribute = UNumberFormatTextAttribute(3i32);
pub const UNUM_NINE_DIGIT_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(26i32);
pub const UNUM_NUMBERING_SYSTEM: UNumberFormatStyle = UNumberFormatStyle(8i32);
pub const UNUM_ONE_DIGIT_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(18i32);
pub const UNUM_ORDINAL: UNumberFormatStyle = UNumberFormatStyle(6i32);
pub const UNUM_PADDING_CHARACTER: UNumberFormatTextAttribute = UNumberFormatTextAttribute(4i32);
pub const UNUM_PADDING_POSITION: UNumberFormatAttribute = UNumberFormatAttribute(14i32);
pub const UNUM_PAD_AFTER_PREFIX: UNumberFormatPadPosition = UNumberFormatPadPosition(1i32);
pub const UNUM_PAD_AFTER_SUFFIX: UNumberFormatPadPosition = UNumberFormatPadPosition(3i32);
pub const UNUM_PAD_BEFORE_PREFIX: UNumberFormatPadPosition = UNumberFormatPadPosition(0i32);
pub const UNUM_PAD_BEFORE_SUFFIX: UNumberFormatPadPosition = UNumberFormatPadPosition(2i32);
pub const UNUM_PAD_ESCAPE_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(13i32);
pub const UNUM_PARSE_ALL_INPUT: UNumberFormatAttribute = UNumberFormatAttribute(20i32);
pub const UNUM_PARSE_CASE_SENSITIVE: UNumberFormatAttribute = UNumberFormatAttribute(4099i32);
pub const UNUM_PARSE_DECIMAL_MARK_REQUIRED: UNumberFormatAttribute = UNumberFormatAttribute(4098i32);
pub const UNUM_PARSE_INT_ONLY: UNumberFormatAttribute = UNumberFormatAttribute(0i32);
pub const UNUM_PARSE_NO_EXPONENT: UNumberFormatAttribute = UNumberFormatAttribute(4097i32);
pub const UNUM_PATTERN_DECIMAL: UNumberFormatStyle = UNumberFormatStyle(0i32);
pub const UNUM_PATTERN_RULEBASED: UNumberFormatStyle = UNumberFormatStyle(9i32);
pub const UNUM_PATTERN_SEPARATOR_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(2i32);
pub const UNUM_PERCENT: UNumberFormatStyle = UNumberFormatStyle(3i32);
pub const UNUM_PERCENT_FIELD: UNumberFormatFields = UNumberFormatFields(8i32);
pub const UNUM_PERCENT_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(3i32);
pub const UNUM_PERMILL_FIELD: UNumberFormatFields = UNumberFormatFields(9i32);
pub const UNUM_PERMILL_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(12i32);
pub const UNUM_PLUS_SIGN_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(7i32);
pub const UNUM_POSITIVE_PREFIX: UNumberFormatTextAttribute = UNumberFormatTextAttribute(0i32);
pub const UNUM_POSITIVE_SUFFIX: UNumberFormatTextAttribute = UNumberFormatTextAttribute(1i32);
pub const UNUM_PUBLIC_RULESETS: UNumberFormatTextAttribute = UNumberFormatTextAttribute(7i32);
pub const UNUM_RANGE_COLLAPSE_ALL: UNumberRangeCollapse = UNumberRangeCollapse(3i32);
pub const UNUM_RANGE_COLLAPSE_AUTO: UNumberRangeCollapse = UNumberRangeCollapse(0i32);
pub const UNUM_RANGE_COLLAPSE_NONE: UNumberRangeCollapse = UNumberRangeCollapse(1i32);
pub const UNUM_RANGE_COLLAPSE_UNIT: UNumberRangeCollapse = UNumberRangeCollapse(2i32);
pub const UNUM_ROUNDING_INCREMENT: UNumberFormatAttribute = UNumberFormatAttribute(12i32);
pub const UNUM_ROUNDING_MODE: UNumberFormatAttribute = UNumberFormatAttribute(11i32);
pub const UNUM_ROUND_CEILING: UNumberFormatRoundingMode = UNumberFormatRoundingMode(0i32);
pub const UNUM_ROUND_DOWN: UNumberFormatRoundingMode = UNumberFormatRoundingMode(2i32);
pub const UNUM_ROUND_FLOOR: UNumberFormatRoundingMode = UNumberFormatRoundingMode(1i32);
pub const UNUM_ROUND_HALFDOWN: UNumberFormatRoundingMode = UNumberFormatRoundingMode(5i32);
pub const UNUM_ROUND_HALFEVEN: UNumberFormatRoundingMode = UNumberFormatRoundingMode(4i32);
pub const UNUM_ROUND_HALFUP: UNumberFormatRoundingMode = UNumberFormatRoundingMode(6i32);
pub const UNUM_ROUND_UNNECESSARY: UNumberFormatRoundingMode = UNumberFormatRoundingMode(7i32);
pub const UNUM_ROUND_UP: UNumberFormatRoundingMode = UNumberFormatRoundingMode(3i32);
pub const UNUM_SCALE: UNumberFormatAttribute = UNumberFormatAttribute(21i32);
pub const UNUM_SCIENTIFIC: UNumberFormatStyle = UNumberFormatStyle(4i32);
pub const UNUM_SECONDARY_GROUPING_SIZE: UNumberFormatAttribute = UNumberFormatAttribute(15i32);
pub const UNUM_SEVEN_DIGIT_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(24i32);
pub const UNUM_SHORT: UNumberCompactStyle = UNumberCompactStyle(0i32);
pub const UNUM_SIGNIFICANT_DIGITS_USED: UNumberFormatAttribute = UNumberFormatAttribute(16i32);
pub const UNUM_SIGNIFICANT_DIGIT_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(16i32);
pub const UNUM_SIGN_ACCOUNTING: UNumberSignDisplay = UNumberSignDisplay(3i32);
pub const UNUM_SIGN_ACCOUNTING_ALWAYS: UNumberSignDisplay = UNumberSignDisplay(4i32);
pub const UNUM_SIGN_ACCOUNTING_EXCEPT_ZERO: UNumberSignDisplay = UNumberSignDisplay(6i32);
pub const UNUM_SIGN_ALWAYS: UNumberSignDisplay = UNumberSignDisplay(1i32);
pub const UNUM_SIGN_ALWAYS_SHOWN: UNumberFormatAttribute = UNumberFormatAttribute(4100i32);
pub const UNUM_SIGN_AUTO: UNumberSignDisplay = UNumberSignDisplay(0i32);
pub const UNUM_SIGN_COUNT: UNumberSignDisplay = UNumberSignDisplay(7i32);
pub const UNUM_SIGN_EXCEPT_ZERO: UNumberSignDisplay = UNumberSignDisplay(5i32);
pub const UNUM_SIGN_FIELD: UNumberFormatFields = UNumberFormatFields(10i32);
pub const UNUM_SIGN_NEVER: UNumberSignDisplay = UNumberSignDisplay(2i32);
pub const UNUM_SIX_DIGIT_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(23i32);
pub const UNUM_SPELLOUT: UNumberFormatStyle = UNumberFormatStyle(5i32);
pub const UNUM_THREE_DIGIT_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(20i32);
pub const UNUM_TWO_DIGIT_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(19i32);
pub const UNUM_UNIT_WIDTH_COUNT: UNumberUnitWidth = UNumberUnitWidth(5i32);
pub const UNUM_UNIT_WIDTH_FULL_NAME: UNumberUnitWidth = UNumberUnitWidth(2i32);
pub const UNUM_UNIT_WIDTH_HIDDEN: UNumberUnitWidth = UNumberUnitWidth(4i32);
pub const UNUM_UNIT_WIDTH_ISO_CODE: UNumberUnitWidth = UNumberUnitWidth(3i32);
pub const UNUM_UNIT_WIDTH_NARROW: UNumberUnitWidth = UNumberUnitWidth(0i32);
pub const UNUM_UNIT_WIDTH_SHORT: UNumberUnitWidth = UNumberUnitWidth(1i32);
pub const UNUM_ZERO_DIGIT_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(4i32);
pub const UPLURAL_TYPE_CARDINAL: UPluralType = UPluralType(0i32);
pub const UPLURAL_TYPE_ORDINAL: UPluralType = UPluralType(1i32);
pub const UREGEX_CASE_INSENSITIVE: URegexpFlag = URegexpFlag(2i32);
pub const UREGEX_COMMENTS: URegexpFlag = URegexpFlag(4i32);
pub const UREGEX_DOTALL: URegexpFlag = URegexpFlag(32i32);
pub const UREGEX_ERROR_ON_UNKNOWN_ESCAPES: URegexpFlag = URegexpFlag(512i32);
pub const UREGEX_LITERAL: URegexpFlag = URegexpFlag(16i32);
pub const UREGEX_MULTILINE: URegexpFlag = URegexpFlag(8i32);
pub const UREGEX_UNIX_LINES: URegexpFlag = URegexpFlag(1i32);
pub const UREGEX_UWORD: URegexpFlag = URegexpFlag(256i32);
pub const URES_ALIAS: UResType = UResType(3i32);
pub const URES_ARRAY: UResType = UResType(8i32);
pub const URES_BINARY: UResType = UResType(1i32);
pub const URES_INT: UResType = UResType(7i32);
pub const URES_INT_VECTOR: UResType = UResType(14i32);
pub const URES_NONE: UResType = UResType(-1i32);
pub const URES_STRING: UResType = UResType(0i32);
pub const URES_TABLE: UResType = UResType(2i32);
pub const URGN_CONTINENT: URegionType = URegionType(3i32);
pub const URGN_DEPRECATED: URegionType = URegionType(6i32);
pub const URGN_GROUPING: URegionType = URegionType(5i32);
pub const URGN_SUBCONTINENT: URegionType = URegionType(4i32);
pub const URGN_TERRITORY: URegionType = URegionType(1i32);
pub const URGN_UNKNOWN: URegionType = URegionType(0i32);
pub const URGN_WORLD: URegionType = URegionType(2i32);
pub const USCRIPT_ADLAM: UScriptCode = UScriptCode(167i32);
pub const USCRIPT_AFAKA: UScriptCode = UScriptCode(147i32);
pub const USCRIPT_AHOM: UScriptCode = UScriptCode(161i32);
pub const USCRIPT_ANATOLIAN_HIEROGLYPHS: UScriptCode = UScriptCode(156i32);
pub const USCRIPT_ARABIC: UScriptCode = UScriptCode(2i32);
pub const USCRIPT_ARMENIAN: UScriptCode = UScriptCode(3i32);
pub const USCRIPT_AVESTAN: UScriptCode = UScriptCode(117i32);
pub const USCRIPT_BALINESE: UScriptCode = UScriptCode(62i32);
pub const USCRIPT_BAMUM: UScriptCode = UScriptCode(130i32);
pub const USCRIPT_BASSA_VAH: UScriptCode = UScriptCode(134i32);
pub const USCRIPT_BATAK: UScriptCode = UScriptCode(63i32);
pub const USCRIPT_BENGALI: UScriptCode = UScriptCode(4i32);
pub const USCRIPT_BHAIKSUKI: UScriptCode = UScriptCode(168i32);
pub const USCRIPT_BLISSYMBOLS: UScriptCode = UScriptCode(64i32);
pub const USCRIPT_BOOK_PAHLAVI: UScriptCode = UScriptCode(124i32);
pub const USCRIPT_BOPOMOFO: UScriptCode = UScriptCode(5i32);
pub const USCRIPT_BRAHMI: UScriptCode = UScriptCode(65i32);
pub const USCRIPT_BRAILLE: UScriptCode = UScriptCode(46i32);
pub const USCRIPT_BUGINESE: UScriptCode = UScriptCode(55i32);
pub const USCRIPT_BUHID: UScriptCode = UScriptCode(44i32);
pub const USCRIPT_CANADIAN_ABORIGINAL: UScriptCode = UScriptCode(40i32);
pub const USCRIPT_CARIAN: UScriptCode = UScriptCode(104i32);
pub const USCRIPT_CAUCASIAN_ALBANIAN: UScriptCode = UScriptCode(159i32);
pub const USCRIPT_CHAKMA: UScriptCode = UScriptCode(118i32);
pub const USCRIPT_CHAM: UScriptCode = UScriptCode(66i32);
pub const USCRIPT_CHEROKEE: UScriptCode = UScriptCode(6i32);
pub const USCRIPT_CHORASMIAN: UScriptCode = UScriptCode(189i32);
pub const USCRIPT_CIRTH: UScriptCode = UScriptCode(67i32);
pub const USCRIPT_COMMON: UScriptCode = UScriptCode(0i32);
pub const USCRIPT_COPTIC: UScriptCode = UScriptCode(7i32);
pub const USCRIPT_CUNEIFORM: UScriptCode = UScriptCode(101i32);
pub const USCRIPT_CYPRIOT: UScriptCode = UScriptCode(47i32);
pub const USCRIPT_CYRILLIC: UScriptCode = UScriptCode(8i32);
pub const USCRIPT_DEMOTIC_EGYPTIAN: UScriptCode = UScriptCode(69i32);
pub const USCRIPT_DESERET: UScriptCode = UScriptCode(9i32);
pub const USCRIPT_DEVANAGARI: UScriptCode = UScriptCode(10i32);
pub const USCRIPT_DIVES_AKURU: UScriptCode = UScriptCode(190i32);
pub const USCRIPT_DOGRA: UScriptCode = UScriptCode(178i32);
pub const USCRIPT_DUPLOYAN: UScriptCode = UScriptCode(135i32);
pub const USCRIPT_EASTERN_SYRIAC: UScriptCode = UScriptCode(97i32);
pub const USCRIPT_EGYPTIAN_HIEROGLYPHS: UScriptCode = UScriptCode(71i32);
pub const USCRIPT_ELBASAN: UScriptCode = UScriptCode(136i32);
pub const USCRIPT_ELYMAIC: UScriptCode = UScriptCode(185i32);
pub const USCRIPT_ESTRANGELO_SYRIAC: UScriptCode = UScriptCode(95i32);
pub const USCRIPT_ETHIOPIC: UScriptCode = UScriptCode(11i32);
pub const USCRIPT_GEORGIAN: UScriptCode = UScriptCode(12i32);
pub const USCRIPT_GLAGOLITIC: UScriptCode = UScriptCode(56i32);
pub const USCRIPT_GOTHIC: UScriptCode = UScriptCode(13i32);
pub const USCRIPT_GRANTHA: UScriptCode = UScriptCode(137i32);
pub const USCRIPT_GREEK: UScriptCode = UScriptCode(14i32);
pub const USCRIPT_GUJARATI: UScriptCode = UScriptCode(15i32);
pub const USCRIPT_GUNJALA_GONDI: UScriptCode = UScriptCode(179i32);
pub const USCRIPT_GURMUKHI: UScriptCode = UScriptCode(16i32);
pub const USCRIPT_HAN: UScriptCode = UScriptCode(17i32);
pub const USCRIPT_HANGUL: UScriptCode = UScriptCode(18i32);
pub const USCRIPT_HANIFI_ROHINGYA: UScriptCode = UScriptCode(182i32);
pub const USCRIPT_HANUNOO: UScriptCode = UScriptCode(43i32);
pub const USCRIPT_HAN_WITH_BOPOMOFO: UScriptCode = UScriptCode(172i32);
pub const USCRIPT_HARAPPAN_INDUS: UScriptCode = UScriptCode(77i32);
pub const USCRIPT_HATRAN: UScriptCode = UScriptCode(162i32);
pub const USCRIPT_HEBREW: UScriptCode = UScriptCode(19i32);
pub const USCRIPT_HIERATIC_EGYPTIAN: UScriptCode = UScriptCode(70i32);
pub const USCRIPT_HIRAGANA: UScriptCode = UScriptCode(20i32);
pub const USCRIPT_IMPERIAL_ARAMAIC: UScriptCode = UScriptCode(116i32);
pub const USCRIPT_INHERITED: UScriptCode = UScriptCode(1i32);
pub const USCRIPT_INSCRIPTIONAL_PAHLAVI: UScriptCode = UScriptCode(122i32);
pub const USCRIPT_INSCRIPTIONAL_PARTHIAN: UScriptCode = UScriptCode(125i32);
pub const USCRIPT_INVALID_CODE: UScriptCode = UScriptCode(-1i32);
pub const USCRIPT_JAMO: UScriptCode = UScriptCode(173i32);
pub const USCRIPT_JAPANESE: UScriptCode = UScriptCode(105i32);
pub const USCRIPT_JAVANESE: UScriptCode = UScriptCode(78i32);
pub const USCRIPT_JURCHEN: UScriptCode = UScriptCode(148i32);
pub const USCRIPT_KAITHI: UScriptCode = UScriptCode(120i32);
pub const USCRIPT_KANNADA: UScriptCode = UScriptCode(21i32);
pub const USCRIPT_KATAKANA: UScriptCode = UScriptCode(22i32);
pub const USCRIPT_KATAKANA_OR_HIRAGANA: UScriptCode = UScriptCode(54i32);
pub const USCRIPT_KAYAH_LI: UScriptCode = UScriptCode(79i32);
pub const USCRIPT_KHAROSHTHI: UScriptCode = UScriptCode(57i32);
pub const USCRIPT_KHITAN_SMALL_SCRIPT: UScriptCode = UScriptCode(191i32);
pub const USCRIPT_KHMER: UScriptCode = UScriptCode(23i32);
pub const USCRIPT_KHOJKI: UScriptCode = UScriptCode(157i32);
pub const USCRIPT_KHUDAWADI: UScriptCode = UScriptCode(145i32);
pub const USCRIPT_KHUTSURI: UScriptCode = UScriptCode(72i32);
pub const USCRIPT_KOREAN: UScriptCode = UScriptCode(119i32);
pub const USCRIPT_KPELLE: UScriptCode = UScriptCode(138i32);
pub const USCRIPT_LANNA: UScriptCode = UScriptCode(106i32);
pub const USCRIPT_LAO: UScriptCode = UScriptCode(24i32);
pub const USCRIPT_LATIN: UScriptCode = UScriptCode(25i32);
pub const USCRIPT_LATIN_FRAKTUR: UScriptCode = UScriptCode(80i32);
pub const USCRIPT_LATIN_GAELIC: UScriptCode = UScriptCode(81i32);
pub const USCRIPT_LEPCHA: UScriptCode = UScriptCode(82i32);
pub const USCRIPT_LIMBU: UScriptCode = UScriptCode(48i32);
pub const USCRIPT_LINEAR_A: UScriptCode = UScriptCode(83i32);
pub const USCRIPT_LINEAR_B: UScriptCode = UScriptCode(49i32);
pub const USCRIPT_LISU: UScriptCode = UScriptCode(131i32);
pub const USCRIPT_LOMA: UScriptCode = UScriptCode(139i32);
pub const USCRIPT_LYCIAN: UScriptCode = UScriptCode(107i32);
pub const USCRIPT_LYDIAN: UScriptCode = UScriptCode(108i32);
pub const USCRIPT_MAHAJANI: UScriptCode = UScriptCode(160i32);
pub const USCRIPT_MAKASAR: UScriptCode = UScriptCode(180i32);
pub const USCRIPT_MALAYALAM: UScriptCode = UScriptCode(26i32);
pub const USCRIPT_MANDAEAN: UScriptCode = UScriptCode(84i32);
pub const USCRIPT_MANDAIC: UScriptCode = UScriptCode(84i32);
pub const USCRIPT_MANICHAEAN: UScriptCode = UScriptCode(121i32);
pub const USCRIPT_MARCHEN: UScriptCode = UScriptCode(169i32);
pub const USCRIPT_MASARAM_GONDI: UScriptCode = UScriptCode(175i32);
pub const USCRIPT_MATHEMATICAL_NOTATION: UScriptCode = UScriptCode(128i32);
pub const USCRIPT_MAYAN_HIEROGLYPHS: UScriptCode = UScriptCode(85i32);
pub const USCRIPT_MEDEFAIDRIN: UScriptCode = UScriptCode(181i32);
pub const USCRIPT_MEITEI_MAYEK: UScriptCode = UScriptCode(115i32);
pub const USCRIPT_MENDE: UScriptCode = UScriptCode(140i32);
pub const USCRIPT_MEROITIC: UScriptCode = UScriptCode(86i32);
pub const USCRIPT_MEROITIC_CURSIVE: UScriptCode = UScriptCode(141i32);
pub const USCRIPT_MEROITIC_HIEROGLYPHS: UScriptCode = UScriptCode(86i32);
pub const USCRIPT_MIAO: UScriptCode = UScriptCode(92i32);
pub const USCRIPT_MODI: UScriptCode = UScriptCode(163i32);
pub const USCRIPT_MONGOLIAN: UScriptCode = UScriptCode(27i32);
pub const USCRIPT_MOON: UScriptCode = UScriptCode(114i32);
pub const USCRIPT_MRO: UScriptCode = UScriptCode(149i32);
pub const USCRIPT_MULTANI: UScriptCode = UScriptCode(164i32);
pub const USCRIPT_MYANMAR: UScriptCode = UScriptCode(28i32);
pub const USCRIPT_NABATAEAN: UScriptCode = UScriptCode(143i32);
pub const USCRIPT_NAKHI_GEBA: UScriptCode = UScriptCode(132i32);
pub const USCRIPT_NANDINAGARI: UScriptCode = UScriptCode(187i32);
pub const USCRIPT_NEWA: UScriptCode = UScriptCode(170i32);
pub const USCRIPT_NEW_TAI_LUE: UScriptCode = UScriptCode(59i32);
pub const USCRIPT_NKO: UScriptCode = UScriptCode(87i32);
pub const USCRIPT_NUSHU: UScriptCode = UScriptCode(150i32);
pub const USCRIPT_NYIAKENG_PUACHUE_HMONG: UScriptCode = UScriptCode(186i32);
pub const USCRIPT_OGHAM: UScriptCode = UScriptCode(29i32);
pub const USCRIPT_OLD_CHURCH_SLAVONIC_CYRILLIC: UScriptCode = UScriptCode(68i32);
pub const USCRIPT_OLD_HUNGARIAN: UScriptCode = UScriptCode(76i32);
pub const USCRIPT_OLD_ITALIC: UScriptCode = UScriptCode(30i32);
pub const USCRIPT_OLD_NORTH_ARABIAN: UScriptCode = UScriptCode(142i32);
pub const USCRIPT_OLD_PERMIC: UScriptCode = UScriptCode(89i32);
pub const USCRIPT_OLD_PERSIAN: UScriptCode = UScriptCode(61i32);
pub const USCRIPT_OLD_SOGDIAN: UScriptCode = UScriptCode(184i32);
pub const USCRIPT_OLD_SOUTH_ARABIAN: UScriptCode = UScriptCode(133i32);
pub const USCRIPT_OL_CHIKI: UScriptCode = UScriptCode(109i32);
pub const USCRIPT_ORIYA: UScriptCode = UScriptCode(31i32);
pub const USCRIPT_ORKHON: UScriptCode = UScriptCode(88i32);
pub const USCRIPT_OSAGE: UScriptCode = UScriptCode(171i32);
pub const USCRIPT_OSMANYA: UScriptCode = UScriptCode(50i32);
pub const USCRIPT_PAHAWH_HMONG: UScriptCode = UScriptCode(75i32);
pub const USCRIPT_PALMYRENE: UScriptCode = UScriptCode(144i32);
pub const USCRIPT_PAU_CIN_HAU: UScriptCode = UScriptCode(165i32);
pub const USCRIPT_PHAGS_PA: UScriptCode = UScriptCode(90i32);
pub const USCRIPT_PHOENICIAN: UScriptCode = UScriptCode(91i32);
pub const USCRIPT_PHONETIC_POLLARD: UScriptCode = UScriptCode(92i32);
pub const USCRIPT_PSALTER_PAHLAVI: UScriptCode = UScriptCode(123i32);
pub const USCRIPT_REJANG: UScriptCode = UScriptCode(110i32);
pub const USCRIPT_RONGORONGO: UScriptCode = UScriptCode(93i32);
pub const USCRIPT_RUNIC: UScriptCode = UScriptCode(32i32);
pub const USCRIPT_SAMARITAN: UScriptCode = UScriptCode(126i32);
pub const USCRIPT_SARATI: UScriptCode = UScriptCode(94i32);
pub const USCRIPT_SAURASHTRA: UScriptCode = UScriptCode(111i32);
pub const USCRIPT_SHARADA: UScriptCode = UScriptCode(151i32);
pub const USCRIPT_SHAVIAN: UScriptCode = UScriptCode(51i32);
pub const USCRIPT_SIDDHAM: UScriptCode = UScriptCode(166i32);
pub const USCRIPT_SIGN_WRITING: UScriptCode = UScriptCode(112i32);
pub const USCRIPT_SIMPLIFIED_HAN: UScriptCode = UScriptCode(73i32);
pub const USCRIPT_SINDHI: UScriptCode = UScriptCode(145i32);
pub const USCRIPT_SINHALA: UScriptCode = UScriptCode(33i32);
pub const USCRIPT_SOGDIAN: UScriptCode = UScriptCode(183i32);
pub const USCRIPT_SORA_SOMPENG: UScriptCode = UScriptCode(152i32);
pub const USCRIPT_SOYOMBO: UScriptCode = UScriptCode(176i32);
pub const USCRIPT_SUNDANESE: UScriptCode = UScriptCode(113i32);
pub const USCRIPT_SYLOTI_NAGRI: UScriptCode = UScriptCode(58i32);
pub const USCRIPT_SYMBOLS: UScriptCode = UScriptCode(129i32);
pub const USCRIPT_SYMBOLS_EMOJI: UScriptCode = UScriptCode(174i32);
pub const USCRIPT_SYRIAC: UScriptCode = UScriptCode(34i32);
pub const USCRIPT_TAGALOG: UScriptCode = UScriptCode(42i32);
pub const USCRIPT_TAGBANWA: UScriptCode = UScriptCode(45i32);
pub const USCRIPT_TAI_LE: UScriptCode = UScriptCode(52i32);
pub const USCRIPT_TAI_VIET: UScriptCode = UScriptCode(127i32);
pub const USCRIPT_TAKRI: UScriptCode = UScriptCode(153i32);
pub const USCRIPT_TAMIL: UScriptCode = UScriptCode(35i32);
pub const USCRIPT_TANGUT: UScriptCode = UScriptCode(154i32);
pub const USCRIPT_TELUGU: UScriptCode = UScriptCode(36i32);
pub const USCRIPT_TENGWAR: UScriptCode = UScriptCode(98i32);
pub const USCRIPT_THAANA: UScriptCode = UScriptCode(37i32);
pub const USCRIPT_THAI: UScriptCode = UScriptCode(38i32);
pub const USCRIPT_TIBETAN: UScriptCode = UScriptCode(39i32);
pub const USCRIPT_TIFINAGH: UScriptCode = UScriptCode(60i32);
pub const USCRIPT_TIRHUTA: UScriptCode = UScriptCode(158i32);
pub const USCRIPT_TRADITIONAL_HAN: UScriptCode = UScriptCode(74i32);
pub const USCRIPT_UCAS: UScriptCode = UScriptCode(40i32);
pub const USCRIPT_UGARITIC: UScriptCode = UScriptCode(53i32);
pub const USCRIPT_UNKNOWN: UScriptCode = UScriptCode(103i32);
pub const USCRIPT_UNWRITTEN_LANGUAGES: UScriptCode = UScriptCode(102i32);
pub const USCRIPT_USAGE_ASPIRATIONAL: UScriptUsage = UScriptUsage(4i32);
pub const USCRIPT_USAGE_EXCLUDED: UScriptUsage = UScriptUsage(2i32);
pub const USCRIPT_USAGE_LIMITED_USE: UScriptUsage = UScriptUsage(3i32);
pub const USCRIPT_USAGE_NOT_ENCODED: UScriptUsage = UScriptUsage(0i32);
pub const USCRIPT_USAGE_RECOMMENDED: UScriptUsage = UScriptUsage(5i32);
pub const USCRIPT_USAGE_UNKNOWN: UScriptUsage = UScriptUsage(1i32);
pub const USCRIPT_VAI: UScriptCode = UScriptCode(99i32);
pub const USCRIPT_VISIBLE_SPEECH: UScriptCode = UScriptCode(100i32);
pub const USCRIPT_WANCHO: UScriptCode = UScriptCode(188i32);
pub const USCRIPT_WARANG_CITI: UScriptCode = UScriptCode(146i32);
pub const USCRIPT_WESTERN_SYRIAC: UScriptCode = UScriptCode(96i32);
pub const USCRIPT_WOLEAI: UScriptCode = UScriptCode(155i32);
pub const USCRIPT_YEZIDI: UScriptCode = UScriptCode(192i32);
pub const USCRIPT_YI: UScriptCode = UScriptCode(41i32);
pub const USCRIPT_ZANABAZAR_SQUARE: UScriptCode = UScriptCode(177i32);
pub const USEARCH_ANY_BASE_WEIGHT_IS_WILDCARD: USearchAttributeValue = USearchAttributeValue(4i32);
pub const USEARCH_DEFAULT: USearchAttributeValue = USearchAttributeValue(-1i32);
pub const USEARCH_DONE: i32 = -1i32;
pub const USEARCH_ELEMENT_COMPARISON: USearchAttribute = USearchAttribute(2i32);
pub const USEARCH_OFF: USearchAttributeValue = USearchAttributeValue(0i32);
pub const USEARCH_ON: USearchAttributeValue = USearchAttributeValue(1i32);
pub const USEARCH_OVERLAP: USearchAttribute = USearchAttribute(0i32);
pub const USEARCH_PATTERN_BASE_WEIGHT_IS_WILDCARD: USearchAttributeValue = USearchAttributeValue(3i32);
pub const USEARCH_STANDARD_ELEMENT_COMPARISON: USearchAttributeValue = USearchAttributeValue(2i32);
pub const USET_ADD_CASE_MAPPINGS: i32 = 4i32;
pub const USET_CASE_INSENSITIVE: i32 = 2i32;
pub const USET_IGNORE_SPACE: i32 = 1i32;
pub const USET_SERIALIZED_STATIC_ARRAY_CAPACITY: i32 = 8i32;
pub const USET_SPAN_CONTAINED: USetSpanCondition = USetSpanCondition(1i32);
pub const USET_SPAN_NOT_CONTAINED: USetSpanCondition = USetSpanCondition(0i32);
pub const USET_SPAN_SIMPLE: USetSpanCondition = USetSpanCondition(2i32);
pub const USPOOF_ALL_CHECKS: USpoofChecks = USpoofChecks(65535i32);
pub const USPOOF_ASCII: URestrictionLevel = URestrictionLevel(268435456i32);
pub const USPOOF_AUX_INFO: USpoofChecks = USpoofChecks(1073741824i32);
pub const USPOOF_CHAR_LIMIT: USpoofChecks = USpoofChecks(64i32);
pub const USPOOF_CONFUSABLE: USpoofChecks = USpoofChecks(7i32);
pub const USPOOF_HIDDEN_OVERLAY: USpoofChecks = USpoofChecks(256i32);
pub const USPOOF_HIGHLY_RESTRICTIVE: URestrictionLevel = URestrictionLevel(805306368i32);
pub const USPOOF_INVISIBLE: USpoofChecks = USpoofChecks(32i32);
pub const USPOOF_MINIMALLY_RESTRICTIVE: URestrictionLevel = URestrictionLevel(1342177280i32);
pub const USPOOF_MIXED_NUMBERS: USpoofChecks = USpoofChecks(128i32);
pub const USPOOF_MIXED_SCRIPT_CONFUSABLE: USpoofChecks = USpoofChecks(2i32);
pub const USPOOF_MODERATELY_RESTRICTIVE: URestrictionLevel = URestrictionLevel(1073741824i32);
pub const USPOOF_RESTRICTION_LEVEL: USpoofChecks = USpoofChecks(16i32);
pub const USPOOF_RESTRICTION_LEVEL_MASK: URestrictionLevel = URestrictionLevel(2130706432i32);
pub const USPOOF_SINGLE_SCRIPT_CONFUSABLE: USpoofChecks = USpoofChecks(1i32);
pub const USPOOF_SINGLE_SCRIPT_RESTRICTIVE: URestrictionLevel = URestrictionLevel(536870912i32);
pub const USPOOF_UNRESTRICTIVE: URestrictionLevel = URestrictionLevel(1610612736i32);
pub const USPOOF_WHOLE_SCRIPT_CONFUSABLE: USpoofChecks = USpoofChecks(4i32);
pub const USPREP_ALLOW_UNASSIGNED: u32 = 1u32;
pub const USPREP_DEFAULT: u32 = 0u32;
pub const USPREP_RFC3491_NAMEPREP: UStringPrepProfileType = UStringPrepProfileType(0i32);
pub const USPREP_RFC3530_NFS4_CIS_PREP: UStringPrepProfileType = UStringPrepProfileType(3i32);
pub const USPREP_RFC3530_NFS4_CS_PREP: UStringPrepProfileType = UStringPrepProfileType(1i32);
pub const USPREP_RFC3530_NFS4_CS_PREP_CI: UStringPrepProfileType = UStringPrepProfileType(2i32);
pub const USPREP_RFC3530_NFS4_MIXED_PREP_PREFIX: UStringPrepProfileType = UStringPrepProfileType(4i32);
pub const USPREP_RFC3530_NFS4_MIXED_PREP_SUFFIX: UStringPrepProfileType = UStringPrepProfileType(5i32);
pub const USPREP_RFC3722_ISCSI: UStringPrepProfileType = UStringPrepProfileType(6i32);
pub const USPREP_RFC3920_NODEPREP: UStringPrepProfileType = UStringPrepProfileType(7i32);
pub const USPREP_RFC3920_RESOURCEPREP: UStringPrepProfileType = UStringPrepProfileType(8i32);
pub const USPREP_RFC4011_MIB: UStringPrepProfileType = UStringPrepProfileType(9i32);
pub const USPREP_RFC4013_SASLPREP: UStringPrepProfileType = UStringPrepProfileType(10i32);
pub const USPREP_RFC4505_TRACE: UStringPrepProfileType = UStringPrepProfileType(11i32);
pub const USPREP_RFC4518_LDAP: UStringPrepProfileType = UStringPrepProfileType(12i32);
pub const USPREP_RFC4518_LDAP_CI: UStringPrepProfileType = UStringPrepProfileType(13i32);
pub const USP_E_SCRIPT_NOT_IN_FONT: windows_core::HRESULT = windows_core::HRESULT(0x80040200_u32 as _);
pub const USTRINGTRIE_BUILD_FAST: UStringTrieBuildOption = UStringTrieBuildOption(0i32);
pub const USTRINGTRIE_BUILD_SMALL: UStringTrieBuildOption = UStringTrieBuildOption(1i32);
pub const USTRINGTRIE_FINAL_VALUE: UStringTrieResult = UStringTrieResult(2i32);
pub const USTRINGTRIE_INTERMEDIATE_VALUE: UStringTrieResult = UStringTrieResult(3i32);
pub const USTRINGTRIE_NO_MATCH: UStringTrieResult = UStringTrieResult(0i32);
pub const USTRINGTRIE_NO_VALUE: UStringTrieResult = UStringTrieResult(1i32);
pub const UTEXT_MAGIC: i32 = 878368812i32;
pub const UTEXT_PROVIDER_HAS_META_DATA: i32 = 4i32;
pub const UTEXT_PROVIDER_LENGTH_IS_EXPENSIVE: i32 = 1i32;
pub const UTEXT_PROVIDER_OWNS_TEXT: i32 = 5i32;
pub const UTEXT_PROVIDER_STABLE_CHUNKS: i32 = 2i32;
pub const UTEXT_PROVIDER_WRITABLE: i32 = 3i32;
pub const UTF16_MAX_CHAR_LENGTH: u32 = 2u32;
pub const UTF32_MAX_CHAR_LENGTH: u32 = 1u32;
pub const UTF8_ERROR_VALUE_1: u32 = 21u32;
pub const UTF8_ERROR_VALUE_2: u32 = 159u32;
pub const UTF8_MAX_CHAR_LENGTH: u32 = 4u32;
pub const UTF_ERROR_VALUE: u32 = 65535u32;
pub const UTF_MAX_CHAR_LENGTH: u32 = 2u32;
pub const UTF_SIZE: u32 = 16u32;
pub const UTRACE_COLLATION_START: UTraceFunctionNumber = UTraceFunctionNumber(8192i32);
pub const UTRACE_CONVERSION_START: UTraceFunctionNumber = UTraceFunctionNumber(4096i32);
pub const UTRACE_ERROR: UTraceLevel = UTraceLevel(0i32);
pub const UTRACE_FUNCTION_START: UTraceFunctionNumber = UTraceFunctionNumber(0i32);
pub const UTRACE_INFO: UTraceLevel = UTraceLevel(7i32);
pub const UTRACE_OFF: UTraceLevel = UTraceLevel(-1i32);
pub const UTRACE_OPEN_CLOSE: UTraceLevel = UTraceLevel(5i32);
pub const UTRACE_UCNV_CLONE: UTraceFunctionNumber = UTraceFunctionNumber(4099i32);
pub const UTRACE_UCNV_CLOSE: UTraceFunctionNumber = UTraceFunctionNumber(4100i32);
pub const UTRACE_UCNV_FLUSH_CACHE: UTraceFunctionNumber = UTraceFunctionNumber(4101i32);
pub const UTRACE_UCNV_LOAD: UTraceFunctionNumber = UTraceFunctionNumber(4102i32);
pub const UTRACE_UCNV_OPEN: UTraceFunctionNumber = UTraceFunctionNumber(4096i32);
pub const UTRACE_UCNV_OPEN_ALGORITHMIC: UTraceFunctionNumber = UTraceFunctionNumber(4098i32);
pub const UTRACE_UCNV_OPEN_PACKAGE: UTraceFunctionNumber = UTraceFunctionNumber(4097i32);
pub const UTRACE_UCNV_UNLOAD: UTraceFunctionNumber = UTraceFunctionNumber(4103i32);
pub const UTRACE_UCOL_CLOSE: UTraceFunctionNumber = UTraceFunctionNumber(8193i32);
pub const UTRACE_UCOL_GETLOCALE: UTraceFunctionNumber = UTraceFunctionNumber(8196i32);
pub const UTRACE_UCOL_GET_SORTKEY: UTraceFunctionNumber = UTraceFunctionNumber(8195i32);
pub const UTRACE_UCOL_NEXTSORTKEYPART: UTraceFunctionNumber = UTraceFunctionNumber(8197i32);
pub const UTRACE_UCOL_OPEN: UTraceFunctionNumber = UTraceFunctionNumber(8192i32);
pub const UTRACE_UCOL_OPEN_FROM_SHORT_STRING: UTraceFunctionNumber = UTraceFunctionNumber(8199i32);
pub const UTRACE_UCOL_STRCOLL: UTraceFunctionNumber = UTraceFunctionNumber(8194i32);
pub const UTRACE_UCOL_STRCOLLITER: UTraceFunctionNumber = UTraceFunctionNumber(8198i32);
pub const UTRACE_UCOL_STRCOLLUTF8: UTraceFunctionNumber = UTraceFunctionNumber(8200i32);
pub const UTRACE_UDATA_BUNDLE: UTraceFunctionNumber = UTraceFunctionNumber(12289i32);
pub const UTRACE_UDATA_DATA_FILE: UTraceFunctionNumber = UTraceFunctionNumber(12290i32);
pub const UTRACE_UDATA_RESOURCE: UTraceFunctionNumber = UTraceFunctionNumber(12288i32);
pub const UTRACE_UDATA_RES_FILE: UTraceFunctionNumber = UTraceFunctionNumber(12291i32);
pub const UTRACE_UDATA_START: UTraceFunctionNumber = UTraceFunctionNumber(12288i32);
pub const UTRACE_U_CLEANUP: UTraceFunctionNumber = UTraceFunctionNumber(1i32);
pub const UTRACE_U_INIT: UTraceFunctionNumber = UTraceFunctionNumber(0i32);
pub const UTRACE_VERBOSE: UTraceLevel = UTraceLevel(9i32);
pub const UTRACE_WARNING: UTraceLevel = UTraceLevel(3i32);
pub const UTRANS_FORWARD: UTransDirection = UTransDirection(0i32);
pub const UTRANS_REVERSE: UTransDirection = UTransDirection(1i32);
pub const UTSV_EPOCH_OFFSET_VALUE: UTimeScaleValue = UTimeScaleValue(1i32);
pub const UTSV_FROM_MAX_VALUE: UTimeScaleValue = UTimeScaleValue(3i32);
pub const UTSV_FROM_MIN_VALUE: UTimeScaleValue = UTimeScaleValue(2i32);
pub const UTSV_TO_MAX_VALUE: UTimeScaleValue = UTimeScaleValue(5i32);
pub const UTSV_TO_MIN_VALUE: UTimeScaleValue = UTimeScaleValue(4i32);
pub const UTSV_UNITS_VALUE: UTimeScaleValue = UTimeScaleValue(0i32);
pub const UTZFMT_PARSE_OPTION_ALL_STYLES: UTimeZoneFormatParseOption = UTimeZoneFormatParseOption(1i32);
pub const UTZFMT_PARSE_OPTION_NONE: UTimeZoneFormatParseOption = UTimeZoneFormatParseOption(0i32);
pub const UTZFMT_PARSE_OPTION_TZ_DATABASE_ABBREVIATIONS: UTimeZoneFormatParseOption = UTimeZoneFormatParseOption(2i32);
pub const UTZFMT_PAT_COUNT: UTimeZoneFormatGMTOffsetPatternType = UTimeZoneFormatGMTOffsetPatternType(6i32);
pub const UTZFMT_PAT_NEGATIVE_H: UTimeZoneFormatGMTOffsetPatternType = UTimeZoneFormatGMTOffsetPatternType(5i32);
pub const UTZFMT_PAT_NEGATIVE_HM: UTimeZoneFormatGMTOffsetPatternType = UTimeZoneFormatGMTOffsetPatternType(2i32);
pub const UTZFMT_PAT_NEGATIVE_HMS: UTimeZoneFormatGMTOffsetPatternType = UTimeZoneFormatGMTOffsetPatternType(3i32);
pub const UTZFMT_PAT_POSITIVE_H: UTimeZoneFormatGMTOffsetPatternType = UTimeZoneFormatGMTOffsetPatternType(4i32);
pub const UTZFMT_PAT_POSITIVE_HM: UTimeZoneFormatGMTOffsetPatternType = UTimeZoneFormatGMTOffsetPatternType(0i32);
pub const UTZFMT_PAT_POSITIVE_HMS: UTimeZoneFormatGMTOffsetPatternType = UTimeZoneFormatGMTOffsetPatternType(1i32);
pub const UTZFMT_STYLE_EXEMPLAR_LOCATION: UTimeZoneFormatStyle = UTimeZoneFormatStyle(19i32);
pub const UTZFMT_STYLE_GENERIC_LOCATION: UTimeZoneFormatStyle = UTimeZoneFormatStyle(0i32);
pub const UTZFMT_STYLE_GENERIC_LONG: UTimeZoneFormatStyle = UTimeZoneFormatStyle(1i32);
pub const UTZFMT_STYLE_GENERIC_SHORT: UTimeZoneFormatStyle = UTimeZoneFormatStyle(2i32);
pub const UTZFMT_STYLE_ISO_BASIC_FIXED: UTimeZoneFormatStyle = UTimeZoneFormatStyle(9i32);
pub const UTZFMT_STYLE_ISO_BASIC_FULL: UTimeZoneFormatStyle = UTimeZoneFormatStyle(11i32);
pub const UTZFMT_STYLE_ISO_BASIC_LOCAL_FIXED: UTimeZoneFormatStyle = UTimeZoneFormatStyle(10i32);
pub const UTZFMT_STYLE_ISO_BASIC_LOCAL_FULL: UTimeZoneFormatStyle = UTimeZoneFormatStyle(12i32);
pub const UTZFMT_STYLE_ISO_BASIC_LOCAL_SHORT: UTimeZoneFormatStyle = UTimeZoneFormatStyle(8i32);
pub const UTZFMT_STYLE_ISO_BASIC_SHORT: UTimeZoneFormatStyle = UTimeZoneFormatStyle(7i32);
pub const UTZFMT_STYLE_ISO_EXTENDED_FIXED: UTimeZoneFormatStyle = UTimeZoneFormatStyle(13i32);
pub const UTZFMT_STYLE_ISO_EXTENDED_FULL: UTimeZoneFormatStyle = UTimeZoneFormatStyle(15i32);
pub const UTZFMT_STYLE_ISO_EXTENDED_LOCAL_FIXED: UTimeZoneFormatStyle = UTimeZoneFormatStyle(14i32);
pub const UTZFMT_STYLE_ISO_EXTENDED_LOCAL_FULL: UTimeZoneFormatStyle = UTimeZoneFormatStyle(16i32);
pub const UTZFMT_STYLE_LOCALIZED_GMT: UTimeZoneFormatStyle = UTimeZoneFormatStyle(5i32);
pub const UTZFMT_STYLE_LOCALIZED_GMT_SHORT: UTimeZoneFormatStyle = UTimeZoneFormatStyle(6i32);
pub const UTZFMT_STYLE_SPECIFIC_LONG: UTimeZoneFormatStyle = UTimeZoneFormatStyle(3i32);
pub const UTZFMT_STYLE_SPECIFIC_SHORT: UTimeZoneFormatStyle = UTimeZoneFormatStyle(4i32);
pub const UTZFMT_STYLE_ZONE_ID: UTimeZoneFormatStyle = UTimeZoneFormatStyle(17i32);
pub const UTZFMT_STYLE_ZONE_ID_SHORT: UTimeZoneFormatStyle = UTimeZoneFormatStyle(18i32);
pub const UTZFMT_TIME_TYPE_DAYLIGHT: UTimeZoneFormatTimeType = UTimeZoneFormatTimeType(2i32);
pub const UTZFMT_TIME_TYPE_STANDARD: UTimeZoneFormatTimeType = UTimeZoneFormatTimeType(1i32);
pub const UTZFMT_TIME_TYPE_UNKNOWN: UTimeZoneFormatTimeType = UTimeZoneFormatTimeType(0i32);
pub const UTZNM_EXEMPLAR_LOCATION: UTimeZoneNameType = UTimeZoneNameType(64i32);
pub const UTZNM_LONG_DAYLIGHT: UTimeZoneNameType = UTimeZoneNameType(4i32);
pub const UTZNM_LONG_GENERIC: UTimeZoneNameType = UTimeZoneNameType(1i32);
pub const UTZNM_LONG_STANDARD: UTimeZoneNameType = UTimeZoneNameType(2i32);
pub const UTZNM_SHORT_DAYLIGHT: UTimeZoneNameType = UTimeZoneNameType(32i32);
pub const UTZNM_SHORT_GENERIC: UTimeZoneNameType = UTimeZoneNameType(8i32);
pub const UTZNM_SHORT_STANDARD: UTimeZoneNameType = UTimeZoneNameType(16i32);
pub const UTZNM_UNKNOWN: UTimeZoneNameType = UTimeZoneNameType(0i32);
pub const U_ALPHAINDEX_INFLOW: UAlphabeticIndexLabelType = UAlphabeticIndexLabelType(2i32);
pub const U_ALPHAINDEX_NORMAL: UAlphabeticIndexLabelType = UAlphabeticIndexLabelType(0i32);
pub const U_ALPHAINDEX_OVERFLOW: UAlphabeticIndexLabelType = UAlphabeticIndexLabelType(3i32);
pub const U_ALPHAINDEX_UNDERFLOW: UAlphabeticIndexLabelType = UAlphabeticIndexLabelType(1i32);
pub const U_AMBIGUOUS_ALIAS_WARNING: UErrorCode = UErrorCode(-122i32);
pub const U_ARABIC_NUMBER: UCharDirection = UCharDirection(5i32);
pub const U_ARGUMENT_TYPE_MISMATCH: UErrorCode = UErrorCode(65804i32);
pub const U_ASCII_FAMILY: u32 = 0u32;
pub const U_BAD_VARIABLE_DEFINITION: UErrorCode = UErrorCode(65536i32);
pub const U_BLOCK_SEPARATOR: UCharDirection = UCharDirection(7i32);
pub const U_BOUNDARY_NEUTRAL: UCharDirection = UCharDirection(18i32);
pub const U_BPT_CLOSE: UBidiPairedBracketType = UBidiPairedBracketType(2i32);
pub const U_BPT_NONE: UBidiPairedBracketType = UBidiPairedBracketType(0i32);
pub const U_BPT_OPEN: UBidiPairedBracketType = UBidiPairedBracketType(1i32);
pub const U_BRK_ASSIGN_ERROR: UErrorCode = UErrorCode(66053i32);
pub const U_BRK_ERROR_START: UErrorCode = UErrorCode(66048i32);
pub const U_BRK_HEX_DIGITS_EXPECTED: UErrorCode = UErrorCode(66049i32);
pub const U_BRK_INIT_ERROR: UErrorCode = UErrorCode(66058i32);
pub const U_BRK_INTERNAL_ERROR: UErrorCode = UErrorCode(66048i32);
pub const U_BRK_MALFORMED_RULE_TAG: UErrorCode = UErrorCode(66061i32);
pub const U_BRK_MISMATCHED_PAREN: UErrorCode = UErrorCode(66055i32);
pub const U_BRK_NEW_LINE_IN_QUOTED_STRING: UErrorCode = UErrorCode(66056i32);
pub const U_BRK_RULE_EMPTY_SET: UErrorCode = UErrorCode(66059i32);
pub const U_BRK_RULE_SYNTAX: UErrorCode = UErrorCode(66051i32);
pub const U_BRK_SEMICOLON_EXPECTED: UErrorCode = UErrorCode(66050i32);
pub const U_BRK_UNCLOSED_SET: UErrorCode = UErrorCode(66052i32);
pub const U_BRK_UNDEFINED_VARIABLE: UErrorCode = UErrorCode(66057i32);
pub const U_BRK_UNRECOGNIZED_OPTION: UErrorCode = UErrorCode(66060i32);
pub const U_BRK_VARIABLE_REDFINITION: UErrorCode = UErrorCode(66054i32);
pub const U_BUFFER_OVERFLOW_ERROR: UErrorCode = UErrorCode(15i32);
pub const U_CE_NOT_FOUND_ERROR: UErrorCode = UErrorCode(21i32);
pub const U_CHAR16_IS_TYPEDEF: u32 = 1u32;
pub const U_CHARSET_FAMILY: u32 = 1u32;
pub const U_CHARSET_IS_UTF8: u32 = 1u32;
pub const U_CHAR_CATEGORY_COUNT: UCharCategory = UCharCategory(30i32);
pub const U_CHAR_NAME_ALIAS: UCharNameChoice = UCharNameChoice(3i32);
pub const U_CHECK_DYLOAD: u32 = 1u32;
pub const U_COLLATOR_VERSION_MISMATCH: UErrorCode = UErrorCode(28i32);
pub const U_COMBINED_IMPLEMENTATION: u32 = 1u32;
pub const U_COMBINING_SPACING_MARK: UCharCategory = UCharCategory(8i32);
pub const U_COMMON_NUMBER_SEPARATOR: UCharDirection = UCharDirection(6i32);
pub const U_COMPARE_CODE_POINT_ORDER: u32 = 32768u32;
pub const U_COMPARE_IGNORE_CASE: u32 = 65536u32;
pub const U_CONNECTOR_PUNCTUATION: UCharCategory = UCharCategory(22i32);
pub const U_CONTROL_CHAR: UCharCategory = UCharCategory(15i32);
pub const U_COPYRIGHT_STRING_LENGTH: u32 = 128u32;
pub const U_CPLUSPLUS_VERSION: u32 = 0u32;
pub const U_CURRENCY_SYMBOL: UCharCategory = UCharCategory(25i32);
pub const U_DASH_PUNCTUATION: UCharCategory = UCharCategory(19i32);
pub const U_DEBUG: u32 = 1u32;
pub const U_DECIMAL_DIGIT_NUMBER: UCharCategory = UCharCategory(9i32);
pub const U_DECIMAL_NUMBER_SYNTAX_ERROR: UErrorCode = UErrorCode(65808i32);
pub const U_DEFAULT_KEYWORD_MISSING: UErrorCode = UErrorCode(65807i32);
pub const U_DEFAULT_SHOW_DRAFT: u32 = 0u32;
pub const U_DEFINE_FALSE_AND_TRUE: u32 = 1u32;
pub const U_DIFFERENT_UCA_VERSION: UErrorCode = UErrorCode(-121i32);
pub const U_DIR_NON_SPACING_MARK: UCharDirection = UCharDirection(17i32);
pub const U_DISABLE_RENAMING: u32 = 1u32;
pub const U_DT_CANONICAL: UDecompositionType = UDecompositionType(1i32);
pub const U_DT_CIRCLE: UDecompositionType = UDecompositionType(3i32);
pub const U_DT_COMPAT: UDecompositionType = UDecompositionType(2i32);
pub const U_DT_FINAL: UDecompositionType = UDecompositionType(4i32);
pub const U_DT_FONT: UDecompositionType = UDecompositionType(5i32);
pub const U_DT_FRACTION: UDecompositionType = UDecompositionType(6i32);
pub const U_DT_INITIAL: UDecompositionType = UDecompositionType(7i32);
pub const U_DT_ISOLATED: UDecompositionType = UDecompositionType(8i32);
pub const U_DT_MEDIAL: UDecompositionType = UDecompositionType(9i32);
pub const U_DT_NARROW: UDecompositionType = UDecompositionType(10i32);
pub const U_DT_NOBREAK: UDecompositionType = UDecompositionType(11i32);
pub const U_DT_NONE: UDecompositionType = UDecompositionType(0i32);
pub const U_DT_SMALL: UDecompositionType = UDecompositionType(12i32);
pub const U_DT_SQUARE: UDecompositionType = UDecompositionType(13i32);
pub const U_DT_SUB: UDecompositionType = UDecompositionType(14i32);
pub const U_DT_SUPER: UDecompositionType = UDecompositionType(15i32);
pub const U_DT_VERTICAL: UDecompositionType = UDecompositionType(16i32);
pub const U_DT_WIDE: UDecompositionType = UDecompositionType(17i32);
pub const U_DUPLICATE_KEYWORD: UErrorCode = UErrorCode(65805i32);
pub const U_EA_AMBIGUOUS: UEastAsianWidth = UEastAsianWidth(1i32);
pub const U_EA_FULLWIDTH: UEastAsianWidth = UEastAsianWidth(3i32);
pub const U_EA_HALFWIDTH: UEastAsianWidth = UEastAsianWidth(2i32);
pub const U_EA_NARROW: UEastAsianWidth = UEastAsianWidth(4i32);
pub const U_EA_NEUTRAL: UEastAsianWidth = UEastAsianWidth(0i32);
pub const U_EA_WIDE: UEastAsianWidth = UEastAsianWidth(5i32);
pub const U_EBCDIC_FAMILY: u32 = 1u32;
pub const U_EDITS_NO_RESET: u32 = 8192u32;
pub const U_ENABLE_DYLOAD: u32 = 1u32;
pub const U_ENABLE_TRACING: u32 = 0u32;
pub const U_ENCLOSING_MARK: UCharCategory = UCharCategory(7i32);
pub const U_END_PUNCTUATION: UCharCategory = UCharCategory(21i32);
pub const U_ENUM_OUT_OF_SYNC_ERROR: UErrorCode = UErrorCode(25i32);
pub const U_ERROR_WARNING_START: UErrorCode = UErrorCode(-128i32);
pub const U_EUROPEAN_NUMBER: UCharDirection = UCharDirection(2i32);
pub const U_EUROPEAN_NUMBER_SEPARATOR: UCharDirection = UCharDirection(3i32);
pub const U_EUROPEAN_NUMBER_TERMINATOR: UCharDirection = UCharDirection(4i32);
pub const U_EXTENDED_CHAR_NAME: UCharNameChoice = UCharNameChoice(2i32);
pub const U_FILE_ACCESS_ERROR: UErrorCode = UErrorCode(4i32);
pub const U_FINAL_PUNCTUATION: UCharCategory = UCharCategory(29i32);
pub const U_FIRST_STRONG_ISOLATE: UCharDirection = UCharDirection(19i32);
pub const U_FMT_PARSE_ERROR_START: UErrorCode = UErrorCode(65792i32);
pub const U_FOLD_CASE_DEFAULT: u32 = 0u32;
pub const U_FOLD_CASE_EXCLUDE_SPECIAL_I: u32 = 1u32;
pub const U_FORMAT_CHAR: UCharCategory = UCharCategory(16i32);
pub const U_FORMAT_INEXACT_ERROR: UErrorCode = UErrorCode(65809i32);
pub const U_GCB_CONTROL: UGraphemeClusterBreak = UGraphemeClusterBreak(1i32);
pub const U_GCB_CR: UGraphemeClusterBreak = UGraphemeClusterBreak(2i32);
pub const U_GCB_EXTEND: UGraphemeClusterBreak = UGraphemeClusterBreak(3i32);
pub const U_GCB_E_BASE: UGraphemeClusterBreak = UGraphemeClusterBreak(13i32);
pub const U_GCB_E_BASE_GAZ: UGraphemeClusterBreak = UGraphemeClusterBreak(14i32);
pub const U_GCB_E_MODIFIER: UGraphemeClusterBreak = UGraphemeClusterBreak(15i32);
pub const U_GCB_GLUE_AFTER_ZWJ: UGraphemeClusterBreak = UGraphemeClusterBreak(16i32);
pub const U_GCB_L: UGraphemeClusterBreak = UGraphemeClusterBreak(4i32);
pub const U_GCB_LF: UGraphemeClusterBreak = UGraphemeClusterBreak(5i32);
pub const U_GCB_LV: UGraphemeClusterBreak = UGraphemeClusterBreak(6i32);
pub const U_GCB_LVT: UGraphemeClusterBreak = UGraphemeClusterBreak(7i32);
pub const U_GCB_OTHER: UGraphemeClusterBreak = UGraphemeClusterBreak(0i32);
pub const U_GCB_PREPEND: UGraphemeClusterBreak = UGraphemeClusterBreak(11i32);
pub const U_GCB_REGIONAL_INDICATOR: UGraphemeClusterBreak = UGraphemeClusterBreak(12i32);
pub const U_GCB_SPACING_MARK: UGraphemeClusterBreak = UGraphemeClusterBreak(10i32);
pub const U_GCB_T: UGraphemeClusterBreak = UGraphemeClusterBreak(8i32);
pub const U_GCB_V: UGraphemeClusterBreak = UGraphemeClusterBreak(9i32);
pub const U_GCB_ZWJ: UGraphemeClusterBreak = UGraphemeClusterBreak(17i32);
pub const U_GCC_MAJOR_MINOR: u32 = 0u32;
pub const U_GENERAL_OTHER_TYPES: UCharCategory = UCharCategory(0i32);
pub const U_HAVE_CHAR16_T: u32 = 1u32;
pub const U_HAVE_DEBUG_LOCATION_NEW: u32 = 1u32;
pub const U_HAVE_INTTYPES_H: u32 = 1u32;
pub const U_HAVE_LIB_SUFFIX: u32 = 1u32;
pub const U_HAVE_PLACEMENT_NEW: u32 = 0u32;
pub const U_HAVE_RBNF: u32 = 0u32;
pub const U_HAVE_RVALUE_REFERENCES: u32 = 1u32;
pub const U_HAVE_STDINT_H: u32 = 1u32;
pub const U_HAVE_STD_STRING: u32 = 0u32;
pub const U_HAVE_WCHAR_H: u32 = 0u32;
pub const U_HAVE_WCSCPY: u32 = 0u32;
pub const U_HIDE_DEPRECATED_API: u32 = 1u32;
pub const U_HIDE_DRAFT_API: u32 = 1u32;
pub const U_HIDE_INTERNAL_API: u32 = 1u32;
pub const U_HIDE_OBSOLETE_API: u32 = 1u32;
pub const U_HIDE_OBSOLETE_UTF_OLD_H: u32 = 0u32;
pub const U_HST_LEADING_JAMO: UHangulSyllableType = UHangulSyllableType(1i32);
pub const U_HST_LVT_SYLLABLE: UHangulSyllableType = UHangulSyllableType(5i32);
pub const U_HST_LV_SYLLABLE: UHangulSyllableType = UHangulSyllableType(4i32);
pub const U_HST_NOT_APPLICABLE: UHangulSyllableType = UHangulSyllableType(0i32);
pub const U_HST_TRAILING_JAMO: UHangulSyllableType = UHangulSyllableType(3i32);
pub const U_HST_VOWEL_JAMO: UHangulSyllableType = UHangulSyllableType(2i32);
pub const U_ICUDATA_TYPE_LETTER: windows_core::PCSTR = windows_core::s!("e");
pub const U_ICU_DATA_KEY: windows_core::PCSTR = windows_core::s!("DataVersion");
pub const U_ICU_VERSION_BUNDLE: windows_core::PCSTR = windows_core::s!("icuver");
pub const U_IDNA_ACE_PREFIX_ERROR: UErrorCode = UErrorCode(66564i32);
pub const U_IDNA_CHECK_BIDI_ERROR: UErrorCode = UErrorCode(66562i32);
pub const U_IDNA_DOMAIN_NAME_TOO_LONG_ERROR: UErrorCode = UErrorCode(66568i32);
pub const U_IDNA_ERROR_START: UErrorCode = UErrorCode(66560i32);
pub const U_IDNA_LABEL_TOO_LONG_ERROR: UErrorCode = UErrorCode(66566i32);
pub const U_IDNA_PROHIBITED_ERROR: UErrorCode = UErrorCode(66560i32);
pub const U_IDNA_STD3_ASCII_RULES_ERROR: UErrorCode = UErrorCode(66563i32);
pub const U_IDNA_UNASSIGNED_ERROR: UErrorCode = UErrorCode(66561i32);
pub const U_IDNA_VERIFICATION_ERROR: UErrorCode = UErrorCode(66565i32);
pub const U_IDNA_ZERO_LENGTH_LABEL_ERROR: UErrorCode = UErrorCode(66567i32);
pub const U_ILLEGAL_ARGUMENT_ERROR: UErrorCode = UErrorCode(1i32);
pub const U_ILLEGAL_CHARACTER: UErrorCode = UErrorCode(65567i32);
pub const U_ILLEGAL_CHAR_FOUND: UErrorCode = UErrorCode(12i32);
pub const U_ILLEGAL_CHAR_IN_SEGMENT: UErrorCode = UErrorCode(65564i32);
pub const U_ILLEGAL_ESCAPE_SEQUENCE: UErrorCode = UErrorCode(18i32);
pub const U_ILLEGAL_PAD_POSITION: UErrorCode = UErrorCode(65800i32);
pub const U_INDEX_OUTOFBOUNDS_ERROR: UErrorCode = UErrorCode(8i32);
pub const U_INITIAL_PUNCTUATION: UCharCategory = UCharCategory(28i32);
pub const U_INPC_BOTTOM: UIndicPositionalCategory = UIndicPositionalCategory(1i32);
pub const U_INPC_BOTTOM_AND_LEFT: UIndicPositionalCategory = UIndicPositionalCategory(2i32);
pub const U_INPC_BOTTOM_AND_RIGHT: UIndicPositionalCategory = UIndicPositionalCategory(3i32);
pub const U_INPC_LEFT: UIndicPositionalCategory = UIndicPositionalCategory(4i32);
pub const U_INPC_LEFT_AND_RIGHT: UIndicPositionalCategory = UIndicPositionalCategory(5i32);
pub const U_INPC_NA: UIndicPositionalCategory = UIndicPositionalCategory(0i32);
pub const U_INPC_OVERSTRUCK: UIndicPositionalCategory = UIndicPositionalCategory(6i32);
pub const U_INPC_RIGHT: UIndicPositionalCategory = UIndicPositionalCategory(7i32);
pub const U_INPC_TOP: UIndicPositionalCategory = UIndicPositionalCategory(8i32);
pub const U_INPC_TOP_AND_BOTTOM: UIndicPositionalCategory = UIndicPositionalCategory(9i32);
pub const U_INPC_TOP_AND_BOTTOM_AND_LEFT: UIndicPositionalCategory = UIndicPositionalCategory(15i32);
pub const U_INPC_TOP_AND_BOTTOM_AND_RIGHT: UIndicPositionalCategory = UIndicPositionalCategory(10i32);
pub const U_INPC_TOP_AND_LEFT: UIndicPositionalCategory = UIndicPositionalCategory(11i32);
pub const U_INPC_TOP_AND_LEFT_AND_RIGHT: UIndicPositionalCategory = UIndicPositionalCategory(12i32);
pub const U_INPC_TOP_AND_RIGHT: UIndicPositionalCategory = UIndicPositionalCategory(13i32);
pub const U_INPC_VISUAL_ORDER_LEFT: UIndicPositionalCategory = UIndicPositionalCategory(14i32);
pub const U_INSC_AVAGRAHA: UIndicSyllabicCategory = UIndicSyllabicCategory(1i32);
pub const U_INSC_BINDU: UIndicSyllabicCategory = UIndicSyllabicCategory(2i32);
pub const U_INSC_BRAHMI_JOINING_NUMBER: UIndicSyllabicCategory = UIndicSyllabicCategory(3i32);
pub const U_INSC_CANTILLATION_MARK: UIndicSyllabicCategory = UIndicSyllabicCategory(4i32);
pub const U_INSC_CONSONANT: UIndicSyllabicCategory = UIndicSyllabicCategory(5i32);
pub const U_INSC_CONSONANT_DEAD: UIndicSyllabicCategory = UIndicSyllabicCategory(6i32);
pub const U_INSC_CONSONANT_FINAL: UIndicSyllabicCategory = UIndicSyllabicCategory(7i32);
pub const U_INSC_CONSONANT_HEAD_LETTER: UIndicSyllabicCategory = UIndicSyllabicCategory(8i32);
pub const U_INSC_CONSONANT_INITIAL_POSTFIXED: UIndicSyllabicCategory = UIndicSyllabicCategory(9i32);
pub const U_INSC_CONSONANT_KILLER: UIndicSyllabicCategory = UIndicSyllabicCategory(10i32);
pub const U_INSC_CONSONANT_MEDIAL: UIndicSyllabicCategory = UIndicSyllabicCategory(11i32);
pub const U_INSC_CONSONANT_PLACEHOLDER: UIndicSyllabicCategory = UIndicSyllabicCategory(12i32);
pub const U_INSC_CONSONANT_PRECEDING_REPHA: UIndicSyllabicCategory = UIndicSyllabicCategory(13i32);
pub const U_INSC_CONSONANT_PREFIXED: UIndicSyllabicCategory = UIndicSyllabicCategory(14i32);
pub const U_INSC_CONSONANT_SUBJOINED: UIndicSyllabicCategory = UIndicSyllabicCategory(15i32);
pub const U_INSC_CONSONANT_SUCCEEDING_REPHA: UIndicSyllabicCategory = UIndicSyllabicCategory(16i32);
pub const U_INSC_CONSONANT_WITH_STACKER: UIndicSyllabicCategory = UIndicSyllabicCategory(17i32);
pub const U_INSC_GEMINATION_MARK: UIndicSyllabicCategory = UIndicSyllabicCategory(18i32);
pub const U_INSC_INVISIBLE_STACKER: UIndicSyllabicCategory = UIndicSyllabicCategory(19i32);
pub const U_INSC_JOINER: UIndicSyllabicCategory = UIndicSyllabicCategory(20i32);
pub const U_INSC_MODIFYING_LETTER: UIndicSyllabicCategory = UIndicSyllabicCategory(21i32);
pub const U_INSC_NON_JOINER: UIndicSyllabicCategory = UIndicSyllabicCategory(22i32);
pub const U_INSC_NUKTA: UIndicSyllabicCategory = UIndicSyllabicCategory(23i32);
pub const U_INSC_NUMBER: UIndicSyllabicCategory = UIndicSyllabicCategory(24i32);
pub const U_INSC_NUMBER_JOINER: UIndicSyllabicCategory = UIndicSyllabicCategory(25i32);
pub const U_INSC_OTHER: UIndicSyllabicCategory = UIndicSyllabicCategory(0i32);
pub const U_INSC_PURE_KILLER: UIndicSyllabicCategory = UIndicSyllabicCategory(26i32);
pub const U_INSC_REGISTER_SHIFTER: UIndicSyllabicCategory = UIndicSyllabicCategory(27i32);
pub const U_INSC_SYLLABLE_MODIFIER: UIndicSyllabicCategory = UIndicSyllabicCategory(28i32);
pub const U_INSC_TONE_LETTER: UIndicSyllabicCategory = UIndicSyllabicCategory(29i32);
pub const U_INSC_TONE_MARK: UIndicSyllabicCategory = UIndicSyllabicCategory(30i32);
pub const U_INSC_VIRAMA: UIndicSyllabicCategory = UIndicSyllabicCategory(31i32);
pub const U_INSC_VISARGA: UIndicSyllabicCategory = UIndicSyllabicCategory(32i32);
pub const U_INSC_VOWEL: UIndicSyllabicCategory = UIndicSyllabicCategory(33i32);
pub const U_INSC_VOWEL_DEPENDENT: UIndicSyllabicCategory = UIndicSyllabicCategory(34i32);
pub const U_INSC_VOWEL_INDEPENDENT: UIndicSyllabicCategory = UIndicSyllabicCategory(35i32);
pub const U_INTERNAL_PROGRAM_ERROR: UErrorCode = UErrorCode(5i32);
pub const U_INTERNAL_TRANSLITERATOR_ERROR: UErrorCode = UErrorCode(65568i32);
pub const U_INVALID_CHAR_FOUND: UErrorCode = UErrorCode(10i32);
pub const U_INVALID_FORMAT_ERROR: UErrorCode = UErrorCode(3i32);
pub const U_INVALID_FUNCTION: UErrorCode = UErrorCode(65570i32);
pub const U_INVALID_ID: UErrorCode = UErrorCode(65569i32);
pub const U_INVALID_PROPERTY_PATTERN: UErrorCode = UErrorCode(65561i32);
pub const U_INVALID_RBT_SYNTAX: UErrorCode = UErrorCode(65560i32);
pub const U_INVALID_STATE_ERROR: UErrorCode = UErrorCode(27i32);
pub const U_INVALID_TABLE_FILE: UErrorCode = UErrorCode(14i32);
pub const U_INVALID_TABLE_FORMAT: UErrorCode = UErrorCode(13i32);
pub const U_INVARIANT_CONVERSION_ERROR: UErrorCode = UErrorCode(26i32);
pub const U_IOSTREAM_SOURCE: u32 = 199711u32;
pub const U_IS_BIG_ENDIAN: u32 = 0u32;
pub const U_JG_AFRICAN_FEH: UJoiningGroup = UJoiningGroup(86i32);
pub const U_JG_AFRICAN_NOON: UJoiningGroup = UJoiningGroup(87i32);
pub const U_JG_AFRICAN_QAF: UJoiningGroup = UJoiningGroup(88i32);
pub const U_JG_AIN: UJoiningGroup = UJoiningGroup(1i32);
pub const U_JG_ALAPH: UJoiningGroup = UJoiningGroup(2i32);
pub const U_JG_ALEF: UJoiningGroup = UJoiningGroup(3i32);
pub const U_JG_BEH: UJoiningGroup = UJoiningGroup(4i32);
pub const U_JG_BETH: UJoiningGroup = UJoiningGroup(5i32);
pub const U_JG_BURUSHASKI_YEH_BARREE: UJoiningGroup = UJoiningGroup(54i32);
pub const U_JG_DAL: UJoiningGroup = UJoiningGroup(6i32);
pub const U_JG_DALATH_RISH: UJoiningGroup = UJoiningGroup(7i32);
pub const U_JG_E: UJoiningGroup = UJoiningGroup(8i32);
pub const U_JG_FARSI_YEH: UJoiningGroup = UJoiningGroup(55i32);
pub const U_JG_FE: UJoiningGroup = UJoiningGroup(51i32);
pub const U_JG_FEH: UJoiningGroup = UJoiningGroup(9i32);
pub const U_JG_FINAL_SEMKATH: UJoiningGroup = UJoiningGroup(10i32);
pub const U_JG_GAF: UJoiningGroup = UJoiningGroup(11i32);
pub const U_JG_GAMAL: UJoiningGroup = UJoiningGroup(12i32);
pub const U_JG_HAH: UJoiningGroup = UJoiningGroup(13i32);
pub const U_JG_HAMZA_ON_HEH_GOAL: UJoiningGroup = UJoiningGroup(14i32);
pub const U_JG_HANIFI_ROHINGYA_KINNA_YA: UJoiningGroup = UJoiningGroup(100i32);
pub const U_JG_HANIFI_ROHINGYA_PA: UJoiningGroup = UJoiningGroup(101i32);
pub const U_JG_HE: UJoiningGroup = UJoiningGroup(15i32);
pub const U_JG_HEH: UJoiningGroup = UJoiningGroup(16i32);
pub const U_JG_HEH_GOAL: UJoiningGroup = UJoiningGroup(17i32);
pub const U_JG_HETH: UJoiningGroup = UJoiningGroup(18i32);
pub const U_JG_KAF: UJoiningGroup = UJoiningGroup(19i32);
pub const U_JG_KAPH: UJoiningGroup = UJoiningGroup(20i32);
pub const U_JG_KHAPH: UJoiningGroup = UJoiningGroup(52i32);
pub const U_JG_KNOTTED_HEH: UJoiningGroup = UJoiningGroup(21i32);
pub const U_JG_LAM: UJoiningGroup = UJoiningGroup(22i32);
pub const U_JG_LAMADH: UJoiningGroup = UJoiningGroup(23i32);
pub const U_JG_MALAYALAM_BHA: UJoiningGroup = UJoiningGroup(89i32);
pub const U_JG_MALAYALAM_JA: UJoiningGroup = UJoiningGroup(90i32);
pub const U_JG_MALAYALAM_LLA: UJoiningGroup = UJoiningGroup(91i32);
pub const U_JG_MALAYALAM_LLLA: UJoiningGroup = UJoiningGroup(92i32);
pub const U_JG_MALAYALAM_NGA: UJoiningGroup = UJoiningGroup(93i32);
pub const U_JG_MALAYALAM_NNA: UJoiningGroup = UJoiningGroup(94i32);
pub const U_JG_MALAYALAM_NNNA: UJoiningGroup = UJoiningGroup(95i32);
pub const U_JG_MALAYALAM_NYA: UJoiningGroup = UJoiningGroup(96i32);
pub const U_JG_MALAYALAM_RA: UJoiningGroup = UJoiningGroup(97i32);
pub const U_JG_MALAYALAM_SSA: UJoiningGroup = UJoiningGroup(98i32);
pub const U_JG_MALAYALAM_TTA: UJoiningGroup = UJoiningGroup(99i32);
pub const U_JG_MANICHAEAN_ALEPH: UJoiningGroup = UJoiningGroup(58i32);
pub const U_JG_MANICHAEAN_AYIN: UJoiningGroup = UJoiningGroup(59i32);
pub const U_JG_MANICHAEAN_BETH: UJoiningGroup = UJoiningGroup(60i32);
pub const U_JG_MANICHAEAN_DALETH: UJoiningGroup = UJoiningGroup(61i32);
pub const U_JG_MANICHAEAN_DHAMEDH: UJoiningGroup = UJoiningGroup(62i32);
pub const U_JG_MANICHAEAN_FIVE: UJoiningGroup = UJoiningGroup(63i32);
pub const U_JG_MANICHAEAN_GIMEL: UJoiningGroup = UJoiningGroup(64i32);
pub const U_JG_MANICHAEAN_HETH: UJoiningGroup = UJoiningGroup(65i32);
pub const U_JG_MANICHAEAN_HUNDRED: UJoiningGroup = UJoiningGroup(66i32);
pub const U_JG_MANICHAEAN_KAPH: UJoiningGroup = UJoiningGroup(67i32);
pub const U_JG_MANICHAEAN_LAMEDH: UJoiningGroup = UJoiningGroup(68i32);
pub const U_JG_MANICHAEAN_MEM: UJoiningGroup = UJoiningGroup(69i32);
pub const U_JG_MANICHAEAN_NUN: UJoiningGroup = UJoiningGroup(70i32);
pub const U_JG_MANICHAEAN_ONE: UJoiningGroup = UJoiningGroup(71i32);
pub const U_JG_MANICHAEAN_PE: UJoiningGroup = UJoiningGroup(72i32);
pub const U_JG_MANICHAEAN_QOPH: UJoiningGroup = UJoiningGroup(73i32);
pub const U_JG_MANICHAEAN_RESH: UJoiningGroup = UJoiningGroup(74i32);
pub const U_JG_MANICHAEAN_SADHE: UJoiningGroup = UJoiningGroup(75i32);
pub const U_JG_MANICHAEAN_SAMEKH: UJoiningGroup = UJoiningGroup(76i32);
pub const U_JG_MANICHAEAN_TAW: UJoiningGroup = UJoiningGroup(77i32);
pub const U_JG_MANICHAEAN_TEN: UJoiningGroup = UJoiningGroup(78i32);
pub const U_JG_MANICHAEAN_TETH: UJoiningGroup = UJoiningGroup(79i32);
pub const U_JG_MANICHAEAN_THAMEDH: UJoiningGroup = UJoiningGroup(80i32);
pub const U_JG_MANICHAEAN_TWENTY: UJoiningGroup = UJoiningGroup(81i32);
pub const U_JG_MANICHAEAN_WAW: UJoiningGroup = UJoiningGroup(82i32);
pub const U_JG_MANICHAEAN_YODH: UJoiningGroup = UJoiningGroup(83i32);
pub const U_JG_MANICHAEAN_ZAYIN: UJoiningGroup = UJoiningGroup(84i32);
pub const U_JG_MEEM: UJoiningGroup = UJoiningGroup(24i32);
pub const U_JG_MIM: UJoiningGroup = UJoiningGroup(25i32);
pub const U_JG_NOON: UJoiningGroup = UJoiningGroup(26i32);
pub const U_JG_NO_JOINING_GROUP: UJoiningGroup = UJoiningGroup(0i32);
pub const U_JG_NUN: UJoiningGroup = UJoiningGroup(27i32);
pub const U_JG_NYA: UJoiningGroup = UJoiningGroup(56i32);
pub const U_JG_PE: UJoiningGroup = UJoiningGroup(28i32);
pub const U_JG_QAF: UJoiningGroup = UJoiningGroup(29i32);
pub const U_JG_QAPH: UJoiningGroup = UJoiningGroup(30i32);
pub const U_JG_REH: UJoiningGroup = UJoiningGroup(31i32);
pub const U_JG_REVERSED_PE: UJoiningGroup = UJoiningGroup(32i32);
pub const U_JG_ROHINGYA_YEH: UJoiningGroup = UJoiningGroup(57i32);
pub const U_JG_SAD: UJoiningGroup = UJoiningGroup(33i32);
pub const U_JG_SADHE: UJoiningGroup = UJoiningGroup(34i32);
pub const U_JG_SEEN: UJoiningGroup = UJoiningGroup(35i32);
pub const U_JG_SEMKATH: UJoiningGroup = UJoiningGroup(36i32);
pub const U_JG_SHIN: UJoiningGroup = UJoiningGroup(37i32);
pub const U_JG_STRAIGHT_WAW: UJoiningGroup = UJoiningGroup(85i32);
pub const U_JG_SWASH_KAF: UJoiningGroup = UJoiningGroup(38i32);
pub const U_JG_SYRIAC_WAW: UJoiningGroup = UJoiningGroup(39i32);
pub const U_JG_TAH: UJoiningGroup = UJoiningGroup(40i32);
pub const U_JG_TAW: UJoiningGroup = UJoiningGroup(41i32);
pub const U_JG_TEH_MARBUTA: UJoiningGroup = UJoiningGroup(42i32);
pub const U_JG_TEH_MARBUTA_GOAL: UJoiningGroup = UJoiningGroup(14i32);
pub const U_JG_TETH: UJoiningGroup = UJoiningGroup(43i32);
pub const U_JG_WAW: UJoiningGroup = UJoiningGroup(44i32);
pub const U_JG_YEH: UJoiningGroup = UJoiningGroup(45i32);
pub const U_JG_YEH_BARREE: UJoiningGroup = UJoiningGroup(46i32);
pub const U_JG_YEH_WITH_TAIL: UJoiningGroup = UJoiningGroup(47i32);
pub const U_JG_YUDH: UJoiningGroup = UJoiningGroup(48i32);
pub const U_JG_YUDH_HE: UJoiningGroup = UJoiningGroup(49i32);
pub const U_JG_ZAIN: UJoiningGroup = UJoiningGroup(50i32);
pub const U_JG_ZHAIN: UJoiningGroup = UJoiningGroup(53i32);
pub const U_JT_DUAL_JOINING: UJoiningType = UJoiningType(2i32);
pub const U_JT_JOIN_CAUSING: UJoiningType = UJoiningType(1i32);
pub const U_JT_LEFT_JOINING: UJoiningType = UJoiningType(3i32);
pub const U_JT_NON_JOINING: UJoiningType = UJoiningType(0i32);
pub const U_JT_RIGHT_JOINING: UJoiningType = UJoiningType(4i32);
pub const U_JT_TRANSPARENT: UJoiningType = UJoiningType(5i32);
pub const U_LB_ALPHABETIC: ULineBreak = ULineBreak(2i32);
pub const U_LB_AMBIGUOUS: ULineBreak = ULineBreak(1i32);
pub const U_LB_BREAK_AFTER: ULineBreak = ULineBreak(4i32);
pub const U_LB_BREAK_BEFORE: ULineBreak = ULineBreak(5i32);
pub const U_LB_BREAK_BOTH: ULineBreak = ULineBreak(3i32);
pub const U_LB_BREAK_SYMBOLS: ULineBreak = ULineBreak(27i32);
pub const U_LB_CARRIAGE_RETURN: ULineBreak = ULineBreak(10i32);
pub const U_LB_CLOSE_PARENTHESIS: ULineBreak = ULineBreak(36i32);
pub const U_LB_CLOSE_PUNCTUATION: ULineBreak = ULineBreak(8i32);
pub const U_LB_COMBINING_MARK: ULineBreak = ULineBreak(9i32);
pub const U_LB_COMPLEX_CONTEXT: ULineBreak = ULineBreak(24i32);
pub const U_LB_CONDITIONAL_JAPANESE_STARTER: ULineBreak = ULineBreak(37i32);
pub const U_LB_CONTINGENT_BREAK: ULineBreak = ULineBreak(7i32);
pub const U_LB_EXCLAMATION: ULineBreak = ULineBreak(11i32);
pub const U_LB_E_BASE: ULineBreak = ULineBreak(40i32);
pub const U_LB_E_MODIFIER: ULineBreak = ULineBreak(41i32);
pub const U_LB_GLUE: ULineBreak = ULineBreak(12i32);
pub const U_LB_H2: ULineBreak = ULineBreak(31i32);
pub const U_LB_H3: ULineBreak = ULineBreak(32i32);
pub const U_LB_HEBREW_LETTER: ULineBreak = ULineBreak(38i32);
pub const U_LB_HYPHEN: ULineBreak = ULineBreak(13i32);
pub const U_LB_IDEOGRAPHIC: ULineBreak = ULineBreak(14i32);
pub const U_LB_INFIX_NUMERIC: ULineBreak = ULineBreak(16i32);
pub const U_LB_INSEPARABLE: ULineBreak = ULineBreak(15i32);
pub const U_LB_INSEPERABLE: ULineBreak = ULineBreak(15i32);
pub const U_LB_JL: ULineBreak = ULineBreak(33i32);
pub const U_LB_JT: ULineBreak = ULineBreak(34i32);
pub const U_LB_JV: ULineBreak = ULineBreak(35i32);
pub const U_LB_LINE_FEED: ULineBreak = ULineBreak(17i32);
pub const U_LB_MANDATORY_BREAK: ULineBreak = ULineBreak(6i32);
pub const U_LB_NEXT_LINE: ULineBreak = ULineBreak(29i32);
pub const U_LB_NONSTARTER: ULineBreak = ULineBreak(18i32);
pub const U_LB_NUMERIC: ULineBreak = ULineBreak(19i32);
pub const U_LB_OPEN_PUNCTUATION: ULineBreak = ULineBreak(20i32);
pub const U_LB_POSTFIX_NUMERIC: ULineBreak = ULineBreak(21i32);
pub const U_LB_PREFIX_NUMERIC: ULineBreak = ULineBreak(22i32);
pub const U_LB_QUOTATION: ULineBreak = ULineBreak(23i32);
pub const U_LB_REGIONAL_INDICATOR: ULineBreak = ULineBreak(39i32);
pub const U_LB_SPACE: ULineBreak = ULineBreak(26i32);
pub const U_LB_SURROGATE: ULineBreak = ULineBreak(25i32);
pub const U_LB_UNKNOWN: ULineBreak = ULineBreak(0i32);
pub const U_LB_WORD_JOINER: ULineBreak = ULineBreak(30i32);
pub const U_LB_ZWJ: ULineBreak = ULineBreak(42i32);
pub const U_LB_ZWSPACE: ULineBreak = ULineBreak(28i32);
pub const U_LEFT_TO_RIGHT: UCharDirection = UCharDirection(0i32);
pub const U_LEFT_TO_RIGHT_EMBEDDING: UCharDirection = UCharDirection(11i32);
pub const U_LEFT_TO_RIGHT_ISOLATE: UCharDirection = UCharDirection(20i32);
pub const U_LEFT_TO_RIGHT_OVERRIDE: UCharDirection = UCharDirection(12i32);
pub const U_LETTER_NUMBER: UCharCategory = UCharCategory(10i32);
pub const U_LIB_SUFFIX_C_NAME_STRING: windows_core::PCSTR = windows_core::s!("");
pub const U_LINE_SEPARATOR: UCharCategory = UCharCategory(13i32);
pub const U_LONG_PROPERTY_NAME: UPropertyNameChoice = UPropertyNameChoice(1i32);
pub const U_LOWERCASE_LETTER: UCharCategory = UCharCategory(2i32);
pub const U_MALFORMED_EXPONENTIAL_PATTERN: UErrorCode = UErrorCode(65795i32);
pub const U_MALFORMED_PRAGMA: UErrorCode = UErrorCode(65562i32);
pub const U_MALFORMED_RULE: UErrorCode = UErrorCode(65537i32);
pub const U_MALFORMED_SET: UErrorCode = UErrorCode(65538i32);
pub const U_MALFORMED_SYMBOL_REFERENCE: UErrorCode = UErrorCode(65539i32);
pub const U_MALFORMED_UNICODE_ESCAPE: UErrorCode = UErrorCode(65540i32);
pub const U_MALFORMED_VARIABLE_DEFINITION: UErrorCode = UErrorCode(65541i32);
pub const U_MALFORMED_VARIABLE_REFERENCE: UErrorCode = UErrorCode(65542i32);
pub const U_MATH_SYMBOL: UCharCategory = UCharCategory(24i32);
pub const U_MAX_VERSION_LENGTH: u32 = 4u32;
pub const U_MAX_VERSION_STRING_LENGTH: u32 = 20u32;
pub const U_MEMORY_ALLOCATION_ERROR: UErrorCode = UErrorCode(7i32);
pub const U_MESSAGE_PARSE_ERROR: UErrorCode = UErrorCode(6i32);
pub const U_MILLIS_PER_DAY: u32 = 86400000u32;
pub const U_MILLIS_PER_HOUR: u32 = 3600000u32;
pub const U_MILLIS_PER_MINUTE: u32 = 60000u32;
pub const U_MILLIS_PER_SECOND: u32 = 1000u32;
pub const U_MISMATCHED_SEGMENT_DELIMITERS: UErrorCode = UErrorCode(65543i32);
pub const U_MISPLACED_ANCHOR_START: UErrorCode = UErrorCode(65544i32);
pub const U_MISPLACED_COMPOUND_FILTER: UErrorCode = UErrorCode(65558i32);
pub const U_MISPLACED_CURSOR_OFFSET: UErrorCode = UErrorCode(65545i32);
pub const U_MISPLACED_QUANTIFIER: UErrorCode = UErrorCode(65546i32);
pub const U_MISSING_OPERATOR: UErrorCode = UErrorCode(65547i32);
pub const U_MISSING_RESOURCE_ERROR: UErrorCode = UErrorCode(2i32);
pub const U_MISSING_SEGMENT_CLOSE: UErrorCode = UErrorCode(65548i32);
pub const U_MODIFIER_LETTER: UCharCategory = UCharCategory(4i32);
pub const U_MODIFIER_SYMBOL: UCharCategory = UCharCategory(26i32);
pub const U_MULTIPLE_ANTE_CONTEXTS: UErrorCode = UErrorCode(65549i32);
pub const U_MULTIPLE_COMPOUND_FILTERS: UErrorCode = UErrorCode(65559i32);
pub const U_MULTIPLE_CURSORS: UErrorCode = UErrorCode(65550i32);
pub const U_MULTIPLE_DECIMAL_SEPARATORS: UErrorCode = UErrorCode(65793i32);
pub const U_MULTIPLE_DECIMAL_SEPERATORS: UErrorCode = UErrorCode(65793i32);
pub const U_MULTIPLE_EXPONENTIAL_SYMBOLS: UErrorCode = UErrorCode(65794i32);
pub const U_MULTIPLE_PAD_SPECIFIERS: UErrorCode = UErrorCode(65798i32);
pub const U_MULTIPLE_PERCENT_SYMBOLS: UErrorCode = UErrorCode(65796i32);
pub const U_MULTIPLE_PERMILL_SYMBOLS: UErrorCode = UErrorCode(65797i32);
pub const U_MULTIPLE_POST_CONTEXTS: UErrorCode = UErrorCode(65551i32);
pub const U_NON_SPACING_MARK: UCharCategory = UCharCategory(6i32);
pub const U_NO_DEFAULT_INCLUDE_UTF_HEADERS: u32 = 1u32;
pub const U_NO_SPACE_AVAILABLE: UErrorCode = UErrorCode(20i32);
pub const U_NO_WRITE_PERMISSION: UErrorCode = UErrorCode(30i32);
pub const U_NT_DECIMAL: UNumericType = UNumericType(1i32);
pub const U_NT_DIGIT: UNumericType = UNumericType(2i32);
pub const U_NT_NONE: UNumericType = UNumericType(0i32);
pub const U_NT_NUMERIC: UNumericType = UNumericType(3i32);
pub const U_NUMBER_ARG_OUTOFBOUNDS_ERROR: UErrorCode = UErrorCode(65810i32);
pub const U_NUMBER_SKELETON_SYNTAX_ERROR: UErrorCode = UErrorCode(65811i32);
pub const U_OMIT_UNCHANGED_TEXT: u32 = 16384u32;
pub const U_OTHER_LETTER: UCharCategory = UCharCategory(5i32);
pub const U_OTHER_NEUTRAL: UCharDirection = UCharDirection(10i32);
pub const U_OTHER_NUMBER: UCharCategory = UCharCategory(11i32);
pub const U_OTHER_PUNCTUATION: UCharCategory = UCharCategory(23i32);
pub const U_OTHER_SYMBOL: UCharCategory = UCharCategory(27i32);
pub const U_OVERRIDE_CXX_ALLOCATION: u32 = 1u32;
pub const U_PARAGRAPH_SEPARATOR: UCharCategory = UCharCategory(14i32);
pub const U_PARSE_CONTEXT_LEN: i32 = 16i32;
pub const U_PARSE_ERROR: UErrorCode = UErrorCode(9i32);
pub const U_PARSE_ERROR_START: UErrorCode = UErrorCode(65536i32);
pub const U_PATTERN_SYNTAX_ERROR: UErrorCode = UErrorCode(65799i32);
pub const U_PF_AIX: u32 = 3100u32;
pub const U_PF_ANDROID: u32 = 4050u32;
pub const U_PF_BROWSER_NATIVE_CLIENT: u32 = 4020u32;
pub const U_PF_BSD: u32 = 3000u32;
pub const U_PF_CYGWIN: u32 = 1900u32;
pub const U_PF_DARWIN: u32 = 3500u32;
pub const U_PF_EMSCRIPTEN: u32 = 5010u32;
pub const U_PF_FUCHSIA: u32 = 4100u32;
pub const U_PF_HPUX: u32 = 2100u32;
pub const U_PF_IPHONE: u32 = 3550u32;
pub const U_PF_IRIX: u32 = 3200u32;
pub const U_PF_LINUX: u32 = 4000u32;
pub const U_PF_MINGW: u32 = 1800u32;
pub const U_PF_OS390: u32 = 9000u32;
pub const U_PF_OS400: u32 = 9400u32;
pub const U_PF_QNX: u32 = 3700u32;
pub const U_PF_SOLARIS: u32 = 2600u32;
pub const U_PF_UNKNOWN: u32 = 0u32;
pub const U_PF_WINDOWS: u32 = 1000u32;
pub const U_PLATFORM: u32 = 1800u32;
pub const U_PLATFORM_HAS_WIN32_API: u32 = 1u32;
pub const U_PLATFORM_HAS_WINUWP_API: u32 = 0u32;
pub const U_PLATFORM_IMPLEMENTS_POSIX: u32 = 0u32;
pub const U_PLATFORM_IS_DARWIN_BASED: u32 = 1u32;
pub const U_PLATFORM_IS_LINUX_BASED: u32 = 1u32;
pub const U_PLATFORM_USES_ONLY_WIN32_API: u32 = 1u32;
pub const U_PLUGIN_CHANGED_LEVEL_WARNING: UErrorCode = UErrorCode(-120i32);
pub const U_PLUGIN_DIDNT_SET_LEVEL: UErrorCode = UErrorCode(66817i32);
pub const U_PLUGIN_ERROR_START: UErrorCode = UErrorCode(66816i32);
pub const U_PLUGIN_TOO_HIGH: UErrorCode = UErrorCode(66816i32);
pub const U_POP_DIRECTIONAL_FORMAT: UCharDirection = UCharDirection(16i32);
pub const U_POP_DIRECTIONAL_ISOLATE: UCharDirection = UCharDirection(22i32);
pub const U_PRIMARY_TOO_LONG_ERROR: UErrorCode = UErrorCode(22i32);
pub const U_PRIVATE_USE_CHAR: UCharCategory = UCharCategory(17i32);
pub const U_REGEX_BAD_ESCAPE_SEQUENCE: UErrorCode = UErrorCode(66307i32);
pub const U_REGEX_BAD_INTERVAL: UErrorCode = UErrorCode(66312i32);
pub const U_REGEX_ERROR_START: UErrorCode = UErrorCode(66304i32);
pub const U_REGEX_INTERNAL_ERROR: UErrorCode = UErrorCode(66304i32);
pub const U_REGEX_INVALID_BACK_REF: UErrorCode = UErrorCode(66314i32);
pub const U_REGEX_INVALID_CAPTURE_GROUP_NAME: UErrorCode = UErrorCode(66325i32);
pub const U_REGEX_INVALID_FLAG: UErrorCode = UErrorCode(66315i32);
pub const U_REGEX_INVALID_RANGE: UErrorCode = UErrorCode(66320i32);
pub const U_REGEX_INVALID_STATE: UErrorCode = UErrorCode(66306i32);
pub const U_REGEX_LOOK_BEHIND_LIMIT: UErrorCode = UErrorCode(66316i32);
pub const U_REGEX_MAX_LT_MIN: UErrorCode = UErrorCode(66313i32);
pub const U_REGEX_MISMATCHED_PAREN: UErrorCode = UErrorCode(66310i32);
pub const U_REGEX_MISSING_CLOSE_BRACKET: UErrorCode = UErrorCode(66319i32);
pub const U_REGEX_NUMBER_TOO_BIG: UErrorCode = UErrorCode(66311i32);
pub const U_REGEX_PATTERN_TOO_BIG: UErrorCode = UErrorCode(66324i32);
pub const U_REGEX_PROPERTY_SYNTAX: UErrorCode = UErrorCode(66308i32);
pub const U_REGEX_RULE_SYNTAX: UErrorCode = UErrorCode(66305i32);
pub const U_REGEX_SET_CONTAINS_STRING: UErrorCode = UErrorCode(66317i32);
pub const U_REGEX_STACK_OVERFLOW: UErrorCode = UErrorCode(66321i32);
pub const U_REGEX_STOPPED_BY_CALLER: UErrorCode = UErrorCode(66323i32);
pub const U_REGEX_TIME_OUT: UErrorCode = UErrorCode(66322i32);
pub const U_REGEX_UNIMPLEMENTED: UErrorCode = UErrorCode(66309i32);
pub const U_RESOURCE_TYPE_MISMATCH: UErrorCode = UErrorCode(17i32);
pub const U_RIGHT_TO_LEFT: UCharDirection = UCharDirection(1i32);
pub const U_RIGHT_TO_LEFT_ARABIC: UCharDirection = UCharDirection(13i32);
pub const U_RIGHT_TO_LEFT_EMBEDDING: UCharDirection = UCharDirection(14i32);
pub const U_RIGHT_TO_LEFT_ISOLATE: UCharDirection = UCharDirection(21i32);
pub const U_RIGHT_TO_LEFT_OVERRIDE: UCharDirection = UCharDirection(15i32);
pub const U_RULE_MASK_ERROR: UErrorCode = UErrorCode(65557i32);
pub const U_SAFECLONE_ALLOCATED_WARNING: UErrorCode = UErrorCode(-126i32);
pub const U_SB_ATERM: USentenceBreak = USentenceBreak(1i32);
pub const U_SB_CLOSE: USentenceBreak = USentenceBreak(2i32);
pub const U_SB_CR: USentenceBreak = USentenceBreak(11i32);
pub const U_SB_EXTEND: USentenceBreak = USentenceBreak(12i32);
pub const U_SB_FORMAT: USentenceBreak = USentenceBreak(3i32);
pub const U_SB_LF: USentenceBreak = USentenceBreak(13i32);
pub const U_SB_LOWER: USentenceBreak = USentenceBreak(4i32);
pub const U_SB_NUMERIC: USentenceBreak = USentenceBreak(5i32);
pub const U_SB_OLETTER: USentenceBreak = USentenceBreak(6i32);
pub const U_SB_OTHER: USentenceBreak = USentenceBreak(0i32);
pub const U_SB_SCONTINUE: USentenceBreak = USentenceBreak(14i32);
pub const U_SB_SEP: USentenceBreak = USentenceBreak(7i32);
pub const U_SB_SP: USentenceBreak = USentenceBreak(8i32);
pub const U_SB_STERM: USentenceBreak = USentenceBreak(9i32);
pub const U_SB_UPPER: USentenceBreak = USentenceBreak(10i32);
pub const U_SEGMENT_SEPARATOR: UCharDirection = UCharDirection(8i32);
pub const U_SENTINEL: i32 = -1i32;
pub const U_SHAPE_AGGREGATE_TASHKEEL: u32 = 16384u32;
pub const U_SHAPE_AGGREGATE_TASHKEEL_MASK: u32 = 16384u32;
pub const U_SHAPE_AGGREGATE_TASHKEEL_NOOP: u32 = 0u32;
pub const U_SHAPE_DIGITS_ALEN2AN_INIT_AL: u32 = 128u32;
pub const U_SHAPE_DIGITS_ALEN2AN_INIT_LR: u32 = 96u32;
pub const U_SHAPE_DIGITS_AN2EN: u32 = 64u32;
pub const U_SHAPE_DIGITS_EN2AN: u32 = 32u32;
pub const U_SHAPE_DIGITS_MASK: u32 = 224u32;
pub const U_SHAPE_DIGITS_NOOP: u32 = 0u32;
pub const U_SHAPE_DIGITS_RESERVED: u32 = 160u32;
pub const U_SHAPE_DIGIT_TYPE_AN: u32 = 0u32;
pub const U_SHAPE_DIGIT_TYPE_AN_EXTENDED: u32 = 256u32;
pub const U_SHAPE_DIGIT_TYPE_MASK: u32 = 768u32;
pub const U_SHAPE_DIGIT_TYPE_RESERVED: u32 = 512u32;
pub const U_SHAPE_LAMALEF_AUTO: u32 = 65536u32;
pub const U_SHAPE_LAMALEF_BEGIN: u32 = 3u32;
pub const U_SHAPE_LAMALEF_END: u32 = 2u32;
pub const U_SHAPE_LAMALEF_MASK: u32 = 65539u32;
pub const U_SHAPE_LAMALEF_NEAR: u32 = 1u32;
pub const U_SHAPE_LAMALEF_RESIZE: u32 = 0u32;
pub const U_SHAPE_LENGTH_FIXED_SPACES_AT_BEGINNING: u32 = 3u32;
pub const U_SHAPE_LENGTH_FIXED_SPACES_AT_END: u32 = 2u32;
pub const U_SHAPE_LENGTH_FIXED_SPACES_NEAR: u32 = 1u32;
pub const U_SHAPE_LENGTH_GROW_SHRINK: u32 = 0u32;
pub const U_SHAPE_LENGTH_MASK: u32 = 65539u32;
pub const U_SHAPE_LETTERS_MASK: u32 = 24u32;
pub const U_SHAPE_LETTERS_NOOP: u32 = 0u32;
pub const U_SHAPE_LETTERS_SHAPE: u32 = 8u32;
pub const U_SHAPE_LETTERS_SHAPE_TASHKEEL_ISOLATED: u32 = 24u32;
pub const U_SHAPE_LETTERS_UNSHAPE: u32 = 16u32;
pub const U_SHAPE_PRESERVE_PRESENTATION: u32 = 32768u32;
pub const U_SHAPE_PRESERVE_PRESENTATION_MASK: u32 = 32768u32;
pub const U_SHAPE_PRESERVE_PRESENTATION_NOOP: u32 = 0u32;
pub const U_SHAPE_SEEN_MASK: u32 = 7340032u32;
pub const U_SHAPE_SEEN_TWOCELL_NEAR: u32 = 2097152u32;
pub const U_SHAPE_SPACES_RELATIVE_TO_TEXT_BEGIN_END: u32 = 67108864u32;
pub const U_SHAPE_SPACES_RELATIVE_TO_TEXT_MASK: u32 = 67108864u32;
pub const U_SHAPE_TAIL_NEW_UNICODE: u32 = 134217728u32;
pub const U_SHAPE_TAIL_TYPE_MASK: u32 = 134217728u32;
pub const U_SHAPE_TASHKEEL_BEGIN: u32 = 262144u32;
pub const U_SHAPE_TASHKEEL_END: u32 = 393216u32;
pub const U_SHAPE_TASHKEEL_MASK: u32 = 917504u32;
pub const U_SHAPE_TASHKEEL_REPLACE_BY_TATWEEL: u32 = 786432u32;
pub const U_SHAPE_TASHKEEL_RESIZE: u32 = 524288u32;
pub const U_SHAPE_TEXT_DIRECTION_LOGICAL: u32 = 0u32;
pub const U_SHAPE_TEXT_DIRECTION_MASK: u32 = 4u32;
pub const U_SHAPE_TEXT_DIRECTION_VISUAL_LTR: u32 = 4u32;
pub const U_SHAPE_TEXT_DIRECTION_VISUAL_RTL: u32 = 0u32;
pub const U_SHAPE_YEHHAMZA_MASK: u32 = 58720256u32;
pub const U_SHAPE_YEHHAMZA_TWOCELL_NEAR: u32 = 16777216u32;
pub const U_SHORT_PROPERTY_NAME: UPropertyNameChoice = UPropertyNameChoice(0i32);
pub const U_SHOW_CPLUSPLUS_API: u32 = 0u32;
pub const U_SIZEOF_UCHAR: u32 = 2u32;
pub const U_SIZEOF_WCHAR_T: u32 = 1u32;
pub const U_SORT_KEY_TOO_SHORT_WARNING: UErrorCode = UErrorCode(-123i32);
pub const U_SPACE_SEPARATOR: UCharCategory = UCharCategory(12i32);
pub const U_START_PUNCTUATION: UCharCategory = UCharCategory(20i32);
pub const U_STATE_OLD_WARNING: UErrorCode = UErrorCode(-125i32);
pub const U_STATE_TOO_OLD_ERROR: UErrorCode = UErrorCode(23i32);
pub const U_STRINGPREP_CHECK_BIDI_ERROR: UErrorCode = UErrorCode(66562i32);
pub const U_STRINGPREP_PROHIBITED_ERROR: UErrorCode = UErrorCode(66560i32);
pub const U_STRINGPREP_UNASSIGNED_ERROR: UErrorCode = UErrorCode(66561i32);
pub const U_STRING_NOT_TERMINATED_WARNING: UErrorCode = UErrorCode(-124i32);
pub const U_SURROGATE: UCharCategory = UCharCategory(18i32);
pub const U_TITLECASE_ADJUST_TO_CASED: u32 = 1024u32;
pub const U_TITLECASE_LETTER: UCharCategory = UCharCategory(3i32);
pub const U_TITLECASE_NO_BREAK_ADJUSTMENT: u32 = 512u32;
pub const U_TITLECASE_NO_LOWERCASE: u32 = 256u32;
pub const U_TITLECASE_SENTENCES: u32 = 64u32;
pub const U_TITLECASE_WHOLE_STRING: u32 = 32u32;
pub const U_TOO_MANY_ALIASES_ERROR: UErrorCode = UErrorCode(24i32);
pub const U_TRAILING_BACKSLASH: UErrorCode = UErrorCode(65552i32);
pub const U_TRUNCATED_CHAR_FOUND: UErrorCode = UErrorCode(11i32);
pub const U_UNASSIGNED: UCharCategory = UCharCategory(0i32);
pub const U_UNCLOSED_SEGMENT: UErrorCode = UErrorCode(65563i32);
pub const U_UNDEFINED_KEYWORD: UErrorCode = UErrorCode(65806i32);
pub const U_UNDEFINED_SEGMENT_REFERENCE: UErrorCode = UErrorCode(65553i32);
pub const U_UNDEFINED_VARIABLE: UErrorCode = UErrorCode(65554i32);
pub const U_UNEXPECTED_TOKEN: UErrorCode = UErrorCode(65792i32);
pub const U_UNICODE_CHAR_NAME: UCharNameChoice = UCharNameChoice(0i32);
pub const U_UNICODE_VERSION: windows_core::PCSTR = windows_core::s!("8.0");
pub const U_UNMATCHED_BRACES: UErrorCode = UErrorCode(65801i32);
pub const U_UNQUOTED_SPECIAL: UErrorCode = UErrorCode(65555i32);
pub const U_UNSUPPORTED_ATTRIBUTE: UErrorCode = UErrorCode(65803i32);
pub const U_UNSUPPORTED_ERROR: UErrorCode = UErrorCode(16i32);
pub const U_UNSUPPORTED_ESCAPE_SEQUENCE: UErrorCode = UErrorCode(19i32);
pub const U_UNSUPPORTED_PROPERTY: UErrorCode = UErrorCode(65802i32);
pub const U_UNTERMINATED_QUOTE: UErrorCode = UErrorCode(65556i32);
pub const U_UPPERCASE_LETTER: UCharCategory = UCharCategory(1i32);
pub const U_USELESS_COLLATOR_ERROR: UErrorCode = UErrorCode(29i32);
pub const U_USING_DEFAULT_WARNING: UErrorCode = UErrorCode(-127i32);
pub const U_USING_FALLBACK_WARNING: UErrorCode = UErrorCode(-128i32);
pub const U_USING_ICU_NAMESPACE: u32 = 1u32;
pub const U_VARIABLE_RANGE_EXHAUSTED: UErrorCode = UErrorCode(65565i32);
pub const U_VARIABLE_RANGE_OVERLAP: UErrorCode = UErrorCode(65566i32);
pub const U_VO_ROTATED: UVerticalOrientation = UVerticalOrientation(0i32);
pub const U_VO_TRANSFORMED_ROTATED: UVerticalOrientation = UVerticalOrientation(1i32);
pub const U_VO_TRANSFORMED_UPRIGHT: UVerticalOrientation = UVerticalOrientation(2i32);
pub const U_VO_UPRIGHT: UVerticalOrientation = UVerticalOrientation(3i32);
pub const U_WB_ALETTER: UWordBreakValues = UWordBreakValues(1i32);
pub const U_WB_CR: UWordBreakValues = UWordBreakValues(8i32);
pub const U_WB_DOUBLE_QUOTE: UWordBreakValues = UWordBreakValues(16i32);
pub const U_WB_EXTEND: UWordBreakValues = UWordBreakValues(9i32);
pub const U_WB_EXTENDNUMLET: UWordBreakValues = UWordBreakValues(7i32);
pub const U_WB_E_BASE: UWordBreakValues = UWordBreakValues(17i32);
pub const U_WB_E_BASE_GAZ: UWordBreakValues = UWordBreakValues(18i32);
pub const U_WB_E_MODIFIER: UWordBreakValues = UWordBreakValues(19i32);
pub const U_WB_FORMAT: UWordBreakValues = UWordBreakValues(2i32);
pub const U_WB_GLUE_AFTER_ZWJ: UWordBreakValues = UWordBreakValues(20i32);
pub const U_WB_HEBREW_LETTER: UWordBreakValues = UWordBreakValues(14i32);
pub const U_WB_KATAKANA: UWordBreakValues = UWordBreakValues(3i32);
pub const U_WB_LF: UWordBreakValues = UWordBreakValues(10i32);
pub const U_WB_MIDLETTER: UWordBreakValues = UWordBreakValues(4i32);
pub const U_WB_MIDNUM: UWordBreakValues = UWordBreakValues(5i32);
pub const U_WB_MIDNUMLET: UWordBreakValues = UWordBreakValues(11i32);
pub const U_WB_NEWLINE: UWordBreakValues = UWordBreakValues(12i32);
pub const U_WB_NUMERIC: UWordBreakValues = UWordBreakValues(6i32);
pub const U_WB_OTHER: UWordBreakValues = UWordBreakValues(0i32);
pub const U_WB_REGIONAL_INDICATOR: UWordBreakValues = UWordBreakValues(13i32);
pub const U_WB_SINGLE_QUOTE: UWordBreakValues = UWordBreakValues(15i32);
pub const U_WB_WSEGSPACE: UWordBreakValues = UWordBreakValues(22i32);
pub const U_WB_ZWJ: UWordBreakValues = UWordBreakValues(21i32);
pub const U_WHITE_SPACE_NEUTRAL: UCharDirection = UCharDirection(9i32);
pub const U_ZERO_ERROR: UErrorCode = UErrorCode(0i32);
pub const VS_ALLOW_LATIN: u32 = 1u32;
pub const WC_COMPOSITECHECK: u32 = 512u32;
pub const WC_DEFAULTCHAR: u32 = 64u32;
pub const WC_DISCARDNS: u32 = 16u32;
pub const WC_ERR_INVALID_CHARS: u32 = 128u32;
pub const WC_NO_BEST_FIT_CHARS: u32 = 1024u32;
pub const WC_SEPCHARS: u32 = 32u32;
pub const WORDLIST_TYPE_ADD: WORDLIST_TYPE = WORDLIST_TYPE(1i32);
pub const WORDLIST_TYPE_AUTOCORRECT: WORDLIST_TYPE = WORDLIST_TYPE(3i32);
pub const WORDLIST_TYPE_EXCLUDE: WORDLIST_TYPE = WORDLIST_TYPE(2i32);
pub const WORDLIST_TYPE_IGNORE: WORDLIST_TYPE = WORDLIST_TYPE(0i32);
pub const WeekUnit: CALDATETIME_DATEUNIT = CALDATETIME_DATEUNIT(3i32);
pub const YearUnit: CALDATETIME_DATEUNIT = CALDATETIME_DATEUNIT(1i32);
pub const sidArabic: SCRIPTCONTF = SCRIPTCONTF(9i32);
pub const sidArmenian: SCRIPTCONTF = SCRIPTCONTF(7i32);
pub const sidAsciiLatin: SCRIPTCONTF = SCRIPTCONTF(3i32);
pub const sidAsciiSym: SCRIPTCONTF = SCRIPTCONTF(2i32);
pub const sidBengali: SCRIPTCONTF = SCRIPTCONTF(11i32);
pub const sidBopomofo: SCRIPTCONTF = SCRIPTCONTF(25i32);
pub const sidBraille: SCRIPTCONTF = SCRIPTCONTF(31i32);
pub const sidBurmese: SCRIPTCONTF = SCRIPTCONTF(36i32);
pub const sidCanSyllabic: SCRIPTCONTF = SCRIPTCONTF(28i32);
pub const sidCherokee: SCRIPTCONTF = SCRIPTCONTF(29i32);
pub const sidCyrillic: SCRIPTCONTF = SCRIPTCONTF(6i32);
pub const sidDefault: SCRIPTCONTF = SCRIPTCONTF(0i32);
pub const sidDevanagari: SCRIPTCONTF = SCRIPTCONTF(10i32);
pub const sidEthiopic: SCRIPTCONTF = SCRIPTCONTF(27i32);
pub const sidFEFirst: SCRIPTCONTF = SCRIPTCONTF(23i32);
pub const sidFELast: SCRIPTCONTF = SCRIPTCONTF(26i32);
pub const sidGeorgian: SCRIPTCONTF = SCRIPTCONTF(22i32);
pub const sidGreek: SCRIPTCONTF = SCRIPTCONTF(5i32);
pub const sidGujarati: SCRIPTCONTF = SCRIPTCONTF(13i32);
pub const sidGurmukhi: SCRIPTCONTF = SCRIPTCONTF(12i32);
pub const sidHan: SCRIPTCONTF = SCRIPTCONTF(26i32);
pub const sidHangul: SCRIPTCONTF = SCRIPTCONTF(23i32);
pub const sidHebrew: SCRIPTCONTF = SCRIPTCONTF(8i32);
pub const sidKana: SCRIPTCONTF = SCRIPTCONTF(24i32);
pub const sidKannada: SCRIPTCONTF = SCRIPTCONTF(17i32);
pub const sidKhmer: SCRIPTCONTF = SCRIPTCONTF(37i32);
pub const sidLao: SCRIPTCONTF = SCRIPTCONTF(20i32);
pub const sidLatin: SCRIPTCONTF = SCRIPTCONTF(4i32);
pub const sidLim: SCRIPTCONTF = SCRIPTCONTF(41i32);
pub const sidMalayalam: SCRIPTCONTF = SCRIPTCONTF(18i32);
pub const sidMerge: SCRIPTCONTF = SCRIPTCONTF(1i32);
pub const sidMongolian: SCRIPTCONTF = SCRIPTCONTF(39i32);
pub const sidOgham: SCRIPTCONTF = SCRIPTCONTF(33i32);
pub const sidOriya: SCRIPTCONTF = SCRIPTCONTF(14i32);
pub const sidRunic: SCRIPTCONTF = SCRIPTCONTF(32i32);
pub const sidSinhala: SCRIPTCONTF = SCRIPTCONTF(34i32);
pub const sidSyriac: SCRIPTCONTF = SCRIPTCONTF(35i32);
pub const sidTamil: SCRIPTCONTF = SCRIPTCONTF(15i32);
pub const sidTelugu: SCRIPTCONTF = SCRIPTCONTF(16i32);
pub const sidThaana: SCRIPTCONTF = SCRIPTCONTF(38i32);
pub const sidThai: SCRIPTCONTF = SCRIPTCONTF(19i32);
pub const sidTibetan: SCRIPTCONTF = SCRIPTCONTF(21i32);
pub const sidUserDefined: SCRIPTCONTF = SCRIPTCONTF(40i32);
pub const sidYi: SCRIPTCONTF = SCRIPTCONTF(30i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CALDATETIME_DATEUNIT(pub i32);
impl windows_core::TypeKind for CALDATETIME_DATEUNIT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CALDATETIME_DATEUNIT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CALDATETIME_DATEUNIT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COMPARESTRING_RESULT(pub i32);
impl windows_core::TypeKind for COMPARESTRING_RESULT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COMPARESTRING_RESULT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COMPARESTRING_RESULT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COMPARE_STRING_FLAGS(pub u32);
impl windows_core::TypeKind for COMPARE_STRING_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COMPARE_STRING_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COMPARE_STRING_FLAGS").field(&self.0).finish()
    }
}
impl COMPARE_STRING_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for COMPARE_STRING_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for COMPARE_STRING_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for COMPARE_STRING_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for COMPARE_STRING_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for COMPARE_STRING_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CORRECTIVE_ACTION(pub i32);
impl windows_core::TypeKind for CORRECTIVE_ACTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CORRECTIVE_ACTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CORRECTIVE_ACTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ENUM_DATE_FORMATS_FLAGS(pub u32);
impl windows_core::TypeKind for ENUM_DATE_FORMATS_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ENUM_DATE_FORMATS_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ENUM_DATE_FORMATS_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ENUM_SYSTEM_CODE_PAGES_FLAGS(pub u32);
impl windows_core::TypeKind for ENUM_SYSTEM_CODE_PAGES_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ENUM_SYSTEM_CODE_PAGES_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ENUM_SYSTEM_CODE_PAGES_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ENUM_SYSTEM_LANGUAGE_GROUPS_FLAGS(pub u32);
impl windows_core::TypeKind for ENUM_SYSTEM_LANGUAGE_GROUPS_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ENUM_SYSTEM_LANGUAGE_GROUPS_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ENUM_SYSTEM_LANGUAGE_GROUPS_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FOLD_STRING_MAP_FLAGS(pub u32);
impl windows_core::TypeKind for FOLD_STRING_MAP_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FOLD_STRING_MAP_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FOLD_STRING_MAP_FLAGS").field(&self.0).finish()
    }
}
impl FOLD_STRING_MAP_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for FOLD_STRING_MAP_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for FOLD_STRING_MAP_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for FOLD_STRING_MAP_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for FOLD_STRING_MAP_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for FOLD_STRING_MAP_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IS_TEXT_UNICODE_RESULT(pub u32);
impl windows_core::TypeKind for IS_TEXT_UNICODE_RESULT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for IS_TEXT_UNICODE_RESULT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IS_TEXT_UNICODE_RESULT").field(&self.0).finish()
    }
}
impl IS_TEXT_UNICODE_RESULT {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for IS_TEXT_UNICODE_RESULT {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for IS_TEXT_UNICODE_RESULT {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for IS_TEXT_UNICODE_RESULT {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for IS_TEXT_UNICODE_RESULT {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for IS_TEXT_UNICODE_RESULT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IS_VALID_LOCALE_FLAGS(pub u32);
impl windows_core::TypeKind for IS_VALID_LOCALE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for IS_VALID_LOCALE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IS_VALID_LOCALE_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MIMECONTF(pub i32);
impl windows_core::TypeKind for MIMECONTF {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MIMECONTF {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MIMECONTF").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MLCONVCHAR(pub i32);
impl windows_core::TypeKind for MLCONVCHAR {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MLCONVCHAR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MLCONVCHAR").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MLCP(pub i32);
impl windows_core::TypeKind for MLCP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MLCP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MLCP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MLDETECTCP(pub i32);
impl windows_core::TypeKind for MLDETECTCP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MLDETECTCP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MLDETECTCP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MLSTR_FLAGS(pub i32);
impl windows_core::TypeKind for MLSTR_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MLSTR_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MLSTR_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MULTI_BYTE_TO_WIDE_CHAR_FLAGS(pub u32);
impl windows_core::TypeKind for MULTI_BYTE_TO_WIDE_CHAR_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MULTI_BYTE_TO_WIDE_CHAR_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MULTI_BYTE_TO_WIDE_CHAR_FLAGS").field(&self.0).finish()
    }
}
impl MULTI_BYTE_TO_WIDE_CHAR_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for MULTI_BYTE_TO_WIDE_CHAR_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for MULTI_BYTE_TO_WIDE_CHAR_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for MULTI_BYTE_TO_WIDE_CHAR_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for MULTI_BYTE_TO_WIDE_CHAR_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for MULTI_BYTE_TO_WIDE_CHAR_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NORM_FORM(pub i32);
impl windows_core::TypeKind for NORM_FORM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NORM_FORM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NORM_FORM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SCRIPTCONTF(pub i32);
impl windows_core::TypeKind for SCRIPTCONTF {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SCRIPTCONTF {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SCRIPTCONTF").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SCRIPTFONTCONTF(pub i32);
impl windows_core::TypeKind for SCRIPTFONTCONTF {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SCRIPTFONTCONTF {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SCRIPTFONTCONTF").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SCRIPT_IS_COMPLEX_FLAGS(pub u32);
impl windows_core::TypeKind for SCRIPT_IS_COMPLEX_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SCRIPT_IS_COMPLEX_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SCRIPT_IS_COMPLEX_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SCRIPT_JUSTIFY(pub i32);
impl windows_core::TypeKind for SCRIPT_JUSTIFY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SCRIPT_JUSTIFY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SCRIPT_JUSTIFY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SYSGEOCLASS(pub i32);
impl windows_core::TypeKind for SYSGEOCLASS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SYSGEOCLASS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SYSGEOCLASS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SYSGEOTYPE(pub i32);
impl windows_core::TypeKind for SYSGEOTYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SYSGEOTYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SYSGEOTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SYSNLS_FUNCTION(pub i32);
impl windows_core::TypeKind for SYSNLS_FUNCTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SYSNLS_FUNCTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SYSNLS_FUNCTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TIME_FORMAT_FLAGS(pub u32);
impl windows_core::TypeKind for TIME_FORMAT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TIME_FORMAT_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TIME_FORMAT_FLAGS").field(&self.0).finish()
    }
}
impl TIME_FORMAT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for TIME_FORMAT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for TIME_FORMAT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for TIME_FORMAT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for TIME_FORMAT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for TIME_FORMAT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TRANSLATE_CHARSET_INFO_FLAGS(pub u32);
impl windows_core::TypeKind for TRANSLATE_CHARSET_INFO_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TRANSLATE_CHARSET_INFO_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TRANSLATE_CHARSET_INFO_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UAcceptResult(pub i32);
impl windows_core::TypeKind for UAcceptResult {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UAcceptResult {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UAcceptResult").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UAlphabeticIndexLabelType(pub i32);
impl windows_core::TypeKind for UAlphabeticIndexLabelType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UAlphabeticIndexLabelType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UAlphabeticIndexLabelType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UBiDiDirection(pub i32);
impl windows_core::TypeKind for UBiDiDirection {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UBiDiDirection {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UBiDiDirection").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UBiDiMirroring(pub i32);
impl windows_core::TypeKind for UBiDiMirroring {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UBiDiMirroring {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UBiDiMirroring").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UBiDiOrder(pub i32);
impl windows_core::TypeKind for UBiDiOrder {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UBiDiOrder {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UBiDiOrder").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UBiDiReorderingMode(pub i32);
impl windows_core::TypeKind for UBiDiReorderingMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UBiDiReorderingMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UBiDiReorderingMode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UBiDiReorderingOption(pub i32);
impl windows_core::TypeKind for UBiDiReorderingOption {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UBiDiReorderingOption {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UBiDiReorderingOption").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UBidiPairedBracketType(pub i32);
impl windows_core::TypeKind for UBidiPairedBracketType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UBidiPairedBracketType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UBidiPairedBracketType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UBlockCode(pub i32);
impl windows_core::TypeKind for UBlockCode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UBlockCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UBlockCode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UBreakIteratorType(pub i32);
impl windows_core::TypeKind for UBreakIteratorType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UBreakIteratorType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UBreakIteratorType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UCPMapRangeOption(pub i32);
impl windows_core::TypeKind for UCPMapRangeOption {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UCPMapRangeOption {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UCPMapRangeOption").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UCPTrieType(pub i32);
impl windows_core::TypeKind for UCPTrieType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UCPTrieType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UCPTrieType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UCPTrieValueWidth(pub i32);
impl windows_core::TypeKind for UCPTrieValueWidth {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UCPTrieValueWidth {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UCPTrieValueWidth").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UCalendarAMPMs(pub i32);
impl windows_core::TypeKind for UCalendarAMPMs {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UCalendarAMPMs {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UCalendarAMPMs").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UCalendarAttribute(pub i32);
impl windows_core::TypeKind for UCalendarAttribute {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UCalendarAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UCalendarAttribute").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UCalendarDateFields(pub i32);
impl windows_core::TypeKind for UCalendarDateFields {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UCalendarDateFields {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UCalendarDateFields").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UCalendarDaysOfWeek(pub i32);
impl windows_core::TypeKind for UCalendarDaysOfWeek {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UCalendarDaysOfWeek {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UCalendarDaysOfWeek").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UCalendarDisplayNameType(pub i32);
impl windows_core::TypeKind for UCalendarDisplayNameType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UCalendarDisplayNameType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UCalendarDisplayNameType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UCalendarLimitType(pub i32);
impl windows_core::TypeKind for UCalendarLimitType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UCalendarLimitType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UCalendarLimitType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UCalendarMonths(pub i32);
impl windows_core::TypeKind for UCalendarMonths {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UCalendarMonths {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UCalendarMonths").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UCalendarType(pub i32);
impl windows_core::TypeKind for UCalendarType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UCalendarType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UCalendarType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UCalendarWallTimeOption(pub i32);
impl windows_core::TypeKind for UCalendarWallTimeOption {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UCalendarWallTimeOption {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UCalendarWallTimeOption").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UCalendarWeekdayType(pub i32);
impl windows_core::TypeKind for UCalendarWeekdayType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UCalendarWeekdayType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UCalendarWeekdayType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UCharCategory(pub i32);
impl windows_core::TypeKind for UCharCategory {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UCharCategory {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UCharCategory").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UCharDirection(pub i32);
impl windows_core::TypeKind for UCharDirection {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UCharDirection {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UCharDirection").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UCharIteratorOrigin(pub i32);
impl windows_core::TypeKind for UCharIteratorOrigin {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UCharIteratorOrigin {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UCharIteratorOrigin").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UCharNameChoice(pub i32);
impl windows_core::TypeKind for UCharNameChoice {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UCharNameChoice {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UCharNameChoice").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UColAttribute(pub i32);
impl windows_core::TypeKind for UColAttribute {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UColAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UColAttribute").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UColAttributeValue(pub i32);
impl windows_core::TypeKind for UColAttributeValue {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UColAttributeValue {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UColAttributeValue").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UColBoundMode(pub i32);
impl windows_core::TypeKind for UColBoundMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UColBoundMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UColBoundMode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UColReorderCode(pub i32);
impl windows_core::TypeKind for UColReorderCode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UColReorderCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UColReorderCode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UColRuleOption(pub i32);
impl windows_core::TypeKind for UColRuleOption {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UColRuleOption {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UColRuleOption").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UCollationResult(pub i32);
impl windows_core::TypeKind for UCollationResult {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UCollationResult {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UCollationResult").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UConverterCallbackReason(pub i32);
impl windows_core::TypeKind for UConverterCallbackReason {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UConverterCallbackReason {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UConverterCallbackReason").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UConverterPlatform(pub i32);
impl windows_core::TypeKind for UConverterPlatform {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UConverterPlatform {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UConverterPlatform").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UConverterType(pub i32);
impl windows_core::TypeKind for UConverterType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UConverterType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UConverterType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UConverterUnicodeSet(pub i32);
impl windows_core::TypeKind for UConverterUnicodeSet {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UConverterUnicodeSet {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UConverterUnicodeSet").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UCurrCurrencyType(pub i32);
impl windows_core::TypeKind for UCurrCurrencyType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UCurrCurrencyType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UCurrCurrencyType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UCurrNameStyle(pub i32);
impl windows_core::TypeKind for UCurrNameStyle {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UCurrNameStyle {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UCurrNameStyle").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UCurrencySpacing(pub i32);
impl windows_core::TypeKind for UCurrencySpacing {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UCurrencySpacing {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UCurrencySpacing").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UCurrencyUsage(pub i32);
impl windows_core::TypeKind for UCurrencyUsage {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UCurrencyUsage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UCurrencyUsage").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UDateAbsoluteUnit(pub i32);
impl windows_core::TypeKind for UDateAbsoluteUnit {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UDateAbsoluteUnit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UDateAbsoluteUnit").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UDateDirection(pub i32);
impl windows_core::TypeKind for UDateDirection {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UDateDirection {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UDateDirection").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UDateFormatBooleanAttribute(pub i32);
impl windows_core::TypeKind for UDateFormatBooleanAttribute {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UDateFormatBooleanAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UDateFormatBooleanAttribute").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UDateFormatField(pub i32);
impl windows_core::TypeKind for UDateFormatField {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UDateFormatField {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UDateFormatField").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UDateFormatStyle(pub i32);
impl windows_core::TypeKind for UDateFormatStyle {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UDateFormatStyle {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UDateFormatStyle").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UDateFormatSymbolType(pub i32);
impl windows_core::TypeKind for UDateFormatSymbolType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UDateFormatSymbolType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UDateFormatSymbolType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UDateRelativeDateTimeFormatterStyle(pub i32);
impl windows_core::TypeKind for UDateRelativeDateTimeFormatterStyle {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UDateRelativeDateTimeFormatterStyle {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UDateRelativeDateTimeFormatterStyle").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UDateRelativeUnit(pub i32);
impl windows_core::TypeKind for UDateRelativeUnit {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UDateRelativeUnit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UDateRelativeUnit").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UDateTimePGDisplayWidth(pub i32);
impl windows_core::TypeKind for UDateTimePGDisplayWidth {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UDateTimePGDisplayWidth {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UDateTimePGDisplayWidth").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UDateTimePatternConflict(pub i32);
impl windows_core::TypeKind for UDateTimePatternConflict {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UDateTimePatternConflict {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UDateTimePatternConflict").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UDateTimePatternField(pub i32);
impl windows_core::TypeKind for UDateTimePatternField {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UDateTimePatternField {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UDateTimePatternField").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UDateTimePatternMatchOptions(pub i32);
impl windows_core::TypeKind for UDateTimePatternMatchOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UDateTimePatternMatchOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UDateTimePatternMatchOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UDateTimeScale(pub i32);
impl windows_core::TypeKind for UDateTimeScale {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UDateTimeScale {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UDateTimeScale").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UDecompositionType(pub i32);
impl windows_core::TypeKind for UDecompositionType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UDecompositionType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UDecompositionType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UDialectHandling(pub i32);
impl windows_core::TypeKind for UDialectHandling {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UDialectHandling {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UDialectHandling").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UDisplayContext(pub i32);
impl windows_core::TypeKind for UDisplayContext {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UDisplayContext {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UDisplayContext").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UDisplayContextType(pub i32);
impl windows_core::TypeKind for UDisplayContextType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UDisplayContextType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UDisplayContextType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UEastAsianWidth(pub i32);
impl windows_core::TypeKind for UEastAsianWidth {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UEastAsianWidth {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UEastAsianWidth").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UErrorCode(pub i32);
impl windows_core::TypeKind for UErrorCode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UErrorCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UErrorCode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UFieldCategory(pub i32);
impl windows_core::TypeKind for UFieldCategory {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UFieldCategory {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UFieldCategory").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UFormattableType(pub i32);
impl windows_core::TypeKind for UFormattableType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UFormattableType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UFormattableType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UGender(pub i32);
impl windows_core::TypeKind for UGender {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UGender {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UGender").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UGraphemeClusterBreak(pub i32);
impl windows_core::TypeKind for UGraphemeClusterBreak {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UGraphemeClusterBreak {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UGraphemeClusterBreak").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UHangulSyllableType(pub i32);
impl windows_core::TypeKind for UHangulSyllableType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UHangulSyllableType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UHangulSyllableType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UIndicPositionalCategory(pub i32);
impl windows_core::TypeKind for UIndicPositionalCategory {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UIndicPositionalCategory {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UIndicPositionalCategory").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UIndicSyllabicCategory(pub i32);
impl windows_core::TypeKind for UIndicSyllabicCategory {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UIndicSyllabicCategory {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UIndicSyllabicCategory").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UJoiningGroup(pub i32);
impl windows_core::TypeKind for UJoiningGroup {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UJoiningGroup {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UJoiningGroup").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UJoiningType(pub i32);
impl windows_core::TypeKind for UJoiningType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UJoiningType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UJoiningType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ULayoutType(pub i32);
impl windows_core::TypeKind for ULayoutType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ULayoutType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ULayoutType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ULineBreak(pub i32);
impl windows_core::TypeKind for ULineBreak {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ULineBreak {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ULineBreak").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ULineBreakTag(pub i32);
impl windows_core::TypeKind for ULineBreakTag {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ULineBreakTag {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ULineBreakTag").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UListFormatterField(pub i32);
impl windows_core::TypeKind for UListFormatterField {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UListFormatterField {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UListFormatterField").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UListFormatterType(pub i32);
impl windows_core::TypeKind for UListFormatterType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UListFormatterType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UListFormatterType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UListFormatterWidth(pub i32);
impl windows_core::TypeKind for UListFormatterWidth {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UListFormatterWidth {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UListFormatterWidth").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ULocAvailableType(pub i32);
impl windows_core::TypeKind for ULocAvailableType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ULocAvailableType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ULocAvailableType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ULocDataLocaleType(pub i32);
impl windows_core::TypeKind for ULocDataLocaleType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ULocDataLocaleType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ULocDataLocaleType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ULocaleDataDelimiterType(pub i32);
impl windows_core::TypeKind for ULocaleDataDelimiterType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ULocaleDataDelimiterType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ULocaleDataDelimiterType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ULocaleDataExemplarSetType(pub i32);
impl windows_core::TypeKind for ULocaleDataExemplarSetType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ULocaleDataExemplarSetType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ULocaleDataExemplarSetType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UMeasureFormatWidth(pub i32);
impl windows_core::TypeKind for UMeasureFormatWidth {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UMeasureFormatWidth {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UMeasureFormatWidth").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UMeasurementSystem(pub i32);
impl windows_core::TypeKind for UMeasurementSystem {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UMeasurementSystem {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UMeasurementSystem").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UMessagePatternApostropheMode(pub i32);
impl windows_core::TypeKind for UMessagePatternApostropheMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UMessagePatternApostropheMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UMessagePatternApostropheMode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UMessagePatternArgType(pub i32);
impl windows_core::TypeKind for UMessagePatternArgType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UMessagePatternArgType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UMessagePatternArgType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UMessagePatternPartType(pub i32);
impl windows_core::TypeKind for UMessagePatternPartType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UMessagePatternPartType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UMessagePatternPartType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UNormalization2Mode(pub i32);
impl windows_core::TypeKind for UNormalization2Mode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UNormalization2Mode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UNormalization2Mode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UNormalizationCheckResult(pub i32);
impl windows_core::TypeKind for UNormalizationCheckResult {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UNormalizationCheckResult {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UNormalizationCheckResult").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UNormalizationMode(pub i32);
impl windows_core::TypeKind for UNormalizationMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UNormalizationMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UNormalizationMode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UNumberCompactStyle(pub i32);
impl windows_core::TypeKind for UNumberCompactStyle {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UNumberCompactStyle {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UNumberCompactStyle").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UNumberDecimalSeparatorDisplay(pub i32);
impl windows_core::TypeKind for UNumberDecimalSeparatorDisplay {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UNumberDecimalSeparatorDisplay {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UNumberDecimalSeparatorDisplay").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UNumberFormatAttribute(pub i32);
impl windows_core::TypeKind for UNumberFormatAttribute {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UNumberFormatAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UNumberFormatAttribute").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UNumberFormatAttributeValue(pub i32);
impl windows_core::TypeKind for UNumberFormatAttributeValue {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UNumberFormatAttributeValue {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UNumberFormatAttributeValue").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UNumberFormatFields(pub i32);
impl windows_core::TypeKind for UNumberFormatFields {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UNumberFormatFields {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UNumberFormatFields").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UNumberFormatPadPosition(pub i32);
impl windows_core::TypeKind for UNumberFormatPadPosition {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UNumberFormatPadPosition {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UNumberFormatPadPosition").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UNumberFormatRoundingMode(pub i32);
impl windows_core::TypeKind for UNumberFormatRoundingMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UNumberFormatRoundingMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UNumberFormatRoundingMode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UNumberFormatStyle(pub i32);
impl windows_core::TypeKind for UNumberFormatStyle {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UNumberFormatStyle {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UNumberFormatStyle").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UNumberFormatSymbol(pub i32);
impl windows_core::TypeKind for UNumberFormatSymbol {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UNumberFormatSymbol {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UNumberFormatSymbol").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UNumberFormatTextAttribute(pub i32);
impl windows_core::TypeKind for UNumberFormatTextAttribute {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UNumberFormatTextAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UNumberFormatTextAttribute").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UNumberGroupingStrategy(pub i32);
impl windows_core::TypeKind for UNumberGroupingStrategy {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UNumberGroupingStrategy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UNumberGroupingStrategy").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UNumberRangeCollapse(pub i32);
impl windows_core::TypeKind for UNumberRangeCollapse {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UNumberRangeCollapse {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UNumberRangeCollapse").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UNumberRangeIdentityFallback(pub i32);
impl windows_core::TypeKind for UNumberRangeIdentityFallback {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UNumberRangeIdentityFallback {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UNumberRangeIdentityFallback").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UNumberRangeIdentityResult(pub i32);
impl windows_core::TypeKind for UNumberRangeIdentityResult {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UNumberRangeIdentityResult {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UNumberRangeIdentityResult").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UNumberSignDisplay(pub i32);
impl windows_core::TypeKind for UNumberSignDisplay {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UNumberSignDisplay {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UNumberSignDisplay").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UNumberUnitWidth(pub i32);
impl windows_core::TypeKind for UNumberUnitWidth {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UNumberUnitWidth {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UNumberUnitWidth").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UNumericType(pub i32);
impl windows_core::TypeKind for UNumericType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UNumericType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UNumericType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UPluralType(pub i32);
impl windows_core::TypeKind for UPluralType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UPluralType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UPluralType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UProperty(pub i32);
impl windows_core::TypeKind for UProperty {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UProperty {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UProperty").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UPropertyNameChoice(pub i32);
impl windows_core::TypeKind for UPropertyNameChoice {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UPropertyNameChoice {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UPropertyNameChoice").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct URegexpFlag(pub i32);
impl windows_core::TypeKind for URegexpFlag {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for URegexpFlag {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("URegexpFlag").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct URegionType(pub i32);
impl windows_core::TypeKind for URegionType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for URegionType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("URegionType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct URelativeDateTimeFormatterField(pub i32);
impl windows_core::TypeKind for URelativeDateTimeFormatterField {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for URelativeDateTimeFormatterField {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("URelativeDateTimeFormatterField").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct URelativeDateTimeUnit(pub i32);
impl windows_core::TypeKind for URelativeDateTimeUnit {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for URelativeDateTimeUnit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("URelativeDateTimeUnit").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UResType(pub i32);
impl windows_core::TypeKind for UResType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UResType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UResType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct URestrictionLevel(pub i32);
impl windows_core::TypeKind for URestrictionLevel {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for URestrictionLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("URestrictionLevel").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UScriptCode(pub i32);
impl windows_core::TypeKind for UScriptCode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UScriptCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UScriptCode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UScriptUsage(pub i32);
impl windows_core::TypeKind for UScriptUsage {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UScriptUsage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UScriptUsage").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct USearchAttribute(pub i32);
impl windows_core::TypeKind for USearchAttribute {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for USearchAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("USearchAttribute").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct USearchAttributeValue(pub i32);
impl windows_core::TypeKind for USearchAttributeValue {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for USearchAttributeValue {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("USearchAttributeValue").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct USentenceBreak(pub i32);
impl windows_core::TypeKind for USentenceBreak {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for USentenceBreak {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("USentenceBreak").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct USentenceBreakTag(pub i32);
impl windows_core::TypeKind for USentenceBreakTag {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for USentenceBreakTag {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("USentenceBreakTag").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct USetSpanCondition(pub i32);
impl windows_core::TypeKind for USetSpanCondition {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for USetSpanCondition {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("USetSpanCondition").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct USpoofChecks(pub i32);
impl windows_core::TypeKind for USpoofChecks {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for USpoofChecks {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("USpoofChecks").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UStringPrepProfileType(pub i32);
impl windows_core::TypeKind for UStringPrepProfileType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UStringPrepProfileType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UStringPrepProfileType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UStringTrieBuildOption(pub i32);
impl windows_core::TypeKind for UStringTrieBuildOption {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UStringTrieBuildOption {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UStringTrieBuildOption").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UStringTrieResult(pub i32);
impl windows_core::TypeKind for UStringTrieResult {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UStringTrieResult {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UStringTrieResult").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct USystemTimeZoneType(pub i32);
impl windows_core::TypeKind for USystemTimeZoneType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for USystemTimeZoneType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("USystemTimeZoneType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UTimeScaleValue(pub i32);
impl windows_core::TypeKind for UTimeScaleValue {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UTimeScaleValue {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UTimeScaleValue").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UTimeZoneFormatGMTOffsetPatternType(pub i32);
impl windows_core::TypeKind for UTimeZoneFormatGMTOffsetPatternType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UTimeZoneFormatGMTOffsetPatternType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UTimeZoneFormatGMTOffsetPatternType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UTimeZoneFormatParseOption(pub i32);
impl windows_core::TypeKind for UTimeZoneFormatParseOption {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UTimeZoneFormatParseOption {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UTimeZoneFormatParseOption").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UTimeZoneFormatStyle(pub i32);
impl windows_core::TypeKind for UTimeZoneFormatStyle {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UTimeZoneFormatStyle {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UTimeZoneFormatStyle").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UTimeZoneFormatTimeType(pub i32);
impl windows_core::TypeKind for UTimeZoneFormatTimeType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UTimeZoneFormatTimeType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UTimeZoneFormatTimeType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UTimeZoneNameType(pub i32);
impl windows_core::TypeKind for UTimeZoneNameType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UTimeZoneNameType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UTimeZoneNameType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UTimeZoneTransitionType(pub i32);
impl windows_core::TypeKind for UTimeZoneTransitionType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UTimeZoneTransitionType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UTimeZoneTransitionType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UTraceFunctionNumber(pub i32);
impl windows_core::TypeKind for UTraceFunctionNumber {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UTraceFunctionNumber {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UTraceFunctionNumber").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UTraceLevel(pub i32);
impl windows_core::TypeKind for UTraceLevel {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UTraceLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UTraceLevel").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UTransDirection(pub i32);
impl windows_core::TypeKind for UTransDirection {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UTransDirection {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UTransDirection").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UVerticalOrientation(pub i32);
impl windows_core::TypeKind for UVerticalOrientation {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UVerticalOrientation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UVerticalOrientation").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UWordBreak(pub i32);
impl windows_core::TypeKind for UWordBreak {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UWordBreak {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UWordBreak").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UWordBreakValues(pub i32);
impl windows_core::TypeKind for UWordBreakValues {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UWordBreakValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UWordBreakValues").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WORDLIST_TYPE(pub i32);
impl windows_core::TypeKind for WORDLIST_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WORDLIST_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WORDLIST_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CALDATETIME {
    pub CalId: u32,
    pub Era: u32,
    pub Year: u32,
    pub Month: u32,
    pub Day: u32,
    pub DayOfWeek: u32,
    pub Hour: u32,
    pub Minute: u32,
    pub Second: u32,
    pub Tick: u32,
}
impl windows_core::TypeKind for CALDATETIME {
    type TypeKind = windows_core::CopyType;
}
impl Default for CALDATETIME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CHARSETINFO {
    pub ciCharset: u32,
    pub ciACP: u32,
    pub fs: FONTSIGNATURE,
}
impl windows_core::TypeKind for CHARSETINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for CHARSETINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMLangConvertCharset: windows_core::GUID = windows_core::GUID::from_u128(0xd66d6f99_cdaa_11d0_b822_00c04fc9b31f);
pub const CMLangString: windows_core::GUID = windows_core::GUID::from_u128(0xc04d65cf_b70d_11d0_b188_00aa0038c969);
pub const CMultiLanguage: windows_core::GUID = windows_core::GUID::from_u128(0x275c23e2_3747_11d0_9fea_00aa003f8646);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CPINFO {
    pub MaxCharSize: u32,
    pub DefaultChar: [u8; 2],
    pub LeadByte: [u8; 12],
}
impl windows_core::TypeKind for CPINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for CPINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CPINFOEXA {
    pub MaxCharSize: u32,
    pub DefaultChar: [u8; 2],
    pub LeadByte: [u8; 12],
    pub UnicodeDefaultChar: u16,
    pub CodePage: u32,
    pub CodePageName: [i8; 260],
}
impl windows_core::TypeKind for CPINFOEXA {
    type TypeKind = windows_core::CopyType;
}
impl Default for CPINFOEXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CPINFOEXW {
    pub MaxCharSize: u32,
    pub DefaultChar: [u8; 2],
    pub LeadByte: [u8; 12],
    pub UnicodeDefaultChar: u16,
    pub CodePage: u32,
    pub CodePageName: [u16; 260],
}
impl windows_core::TypeKind for CPINFOEXW {
    type TypeKind = windows_core::CopyType;
}
impl Default for CPINFOEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CURRENCYFMTA {
    pub NumDigits: u32,
    pub LeadingZero: u32,
    pub Grouping: u32,
    pub lpDecimalSep: windows_core::PSTR,
    pub lpThousandSep: windows_core::PSTR,
    pub NegativeOrder: u32,
    pub PositiveOrder: u32,
    pub lpCurrencySymbol: windows_core::PSTR,
}
impl windows_core::TypeKind for CURRENCYFMTA {
    type TypeKind = windows_core::CopyType;
}
impl Default for CURRENCYFMTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CURRENCYFMTW {
    pub NumDigits: u32,
    pub LeadingZero: u32,
    pub Grouping: u32,
    pub lpDecimalSep: windows_core::PWSTR,
    pub lpThousandSep: windows_core::PWSTR,
    pub NegativeOrder: u32,
    pub PositiveOrder: u32,
    pub lpCurrencySymbol: windows_core::PWSTR,
}
impl windows_core::TypeKind for CURRENCYFMTW {
    type TypeKind = windows_core::CopyType;
}
impl Default for CURRENCYFMTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DetectEncodingInfo {
    pub nLangID: u32,
    pub nCodePage: u32,
    pub nDocPercent: i32,
    pub nConfidence: i32,
}
impl windows_core::TypeKind for DetectEncodingInfo {
    type TypeKind = windows_core::CopyType;
}
impl Default for DetectEncodingInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ENUMTEXTMETRICA {
    pub etmNewTextMetricEx: NEWTEXTMETRICEXA,
    pub etmAxesList: super::Graphics::Gdi::AXESLISTA,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::TypeKind for ENUMTEXTMETRICA {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for ENUMTEXTMETRICA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ENUMTEXTMETRICW {
    pub etmNewTextMetricEx: NEWTEXTMETRICEXW,
    pub etmAxesList: super::Graphics::Gdi::AXESLISTW,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::TypeKind for ENUMTEXTMETRICW {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for ENUMTEXTMETRICW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILEMUIINFO {
    pub dwSize: u32,
    pub dwVersion: u32,
    pub dwFileType: u32,
    pub pChecksum: [u8; 16],
    pub pServiceChecksum: [u8; 16],
    pub dwLanguageNameOffset: u32,
    pub dwTypeIDMainSize: u32,
    pub dwTypeIDMainOffset: u32,
    pub dwTypeNameMainOffset: u32,
    pub dwTypeIDMUISize: u32,
    pub dwTypeIDMUIOffset: u32,
    pub dwTypeNameMUIOffset: u32,
    pub abBuffer: [u8; 8],
}
impl windows_core::TypeKind for FILEMUIINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILEMUIINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FONTSIGNATURE {
    pub fsUsb: [u32; 4],
    pub fsCsb: [u32; 2],
}
impl windows_core::TypeKind for FONTSIGNATURE {
    type TypeKind = windows_core::CopyType;
}
impl Default for FONTSIGNATURE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GOFFSET {
    pub du: i32,
    pub dv: i32,
}
impl windows_core::TypeKind for GOFFSET {
    type TypeKind = windows_core::CopyType;
}
impl Default for GOFFSET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HIMC(pub isize);
impl HIMC {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl Default for HIMC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HIMC {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HIMCC(pub isize);
impl HIMCC {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl Default for HIMCC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HIMCC {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HSAVEDUILANGUAGES(pub isize);
impl HSAVEDUILANGUAGES {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl Default for HSAVEDUILANGUAGES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HSAVEDUILANGUAGES {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LOCALESIGNATURE {
    pub lsUsb: [u32; 4],
    pub lsCsbDefault: [u32; 2],
    pub lsCsbSupported: [u32; 2],
}
impl windows_core::TypeKind for LOCALESIGNATURE {
    type TypeKind = windows_core::CopyType;
}
impl Default for LOCALESIGNATURE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MAPPING_DATA_RANGE {
    pub dwStartIndex: u32,
    pub dwEndIndex: u32,
    pub pszDescription: windows_core::PWSTR,
    pub dwDescriptionLength: u32,
    pub pData: *mut core::ffi::c_void,
    pub dwDataSize: u32,
    pub pszContentType: windows_core::PWSTR,
    pub prgActionIds: *mut windows_core::PWSTR,
    pub dwActionsCount: u32,
    pub prgActionDisplayNames: *mut windows_core::PWSTR,
}
impl windows_core::TypeKind for MAPPING_DATA_RANGE {
    type TypeKind = windows_core::CopyType;
}
impl Default for MAPPING_DATA_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MAPPING_ENUM_OPTIONS {
    pub Size: usize,
    pub pszCategory: windows_core::PWSTR,
    pub pszInputLanguage: windows_core::PWSTR,
    pub pszOutputLanguage: windows_core::PWSTR,
    pub pszInputScript: windows_core::PWSTR,
    pub pszOutputScript: windows_core::PWSTR,
    pub pszInputContentType: windows_core::PWSTR,
    pub pszOutputContentType: windows_core::PWSTR,
    pub pGuid: *mut windows_core::GUID,
    pub _bitfield: u32,
}
impl windows_core::TypeKind for MAPPING_ENUM_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl Default for MAPPING_ENUM_OPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct MAPPING_OPTIONS {
    pub Size: usize,
    pub pszInputLanguage: windows_core::PWSTR,
    pub pszOutputLanguage: windows_core::PWSTR,
    pub pszInputScript: windows_core::PWSTR,
    pub pszOutputScript: windows_core::PWSTR,
    pub pszInputContentType: windows_core::PWSTR,
    pub pszOutputContentType: windows_core::PWSTR,
    pub pszUILanguage: windows_core::PWSTR,
    pub pfnRecognizeCallback: PFN_MAPPINGCALLBACKPROC,
    pub pRecognizeCallerData: *mut core::ffi::c_void,
    pub dwRecognizeCallerDataSize: u32,
    pub pfnActionCallback: PFN_MAPPINGCALLBACKPROC,
    pub pActionCallerData: *mut core::ffi::c_void,
    pub dwActionCallerDataSize: u32,
    pub dwServiceFlag: u32,
    pub _bitfield: u32,
}
impl windows_core::TypeKind for MAPPING_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl Default for MAPPING_OPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MAPPING_PROPERTY_BAG {
    pub Size: usize,
    pub prgResultRanges: *mut MAPPING_DATA_RANGE,
    pub dwRangesCount: u32,
    pub pServiceData: *mut core::ffi::c_void,
    pub dwServiceDataSize: u32,
    pub pCallerData: *mut core::ffi::c_void,
    pub dwCallerDataSize: u32,
    pub pContext: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for MAPPING_PROPERTY_BAG {
    type TypeKind = windows_core::CopyType;
}
impl Default for MAPPING_PROPERTY_BAG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MAPPING_SERVICE_INFO {
    pub Size: usize,
    pub pszCopyright: windows_core::PWSTR,
    pub wMajorVersion: u16,
    pub wMinorVersion: u16,
    pub wBuildVersion: u16,
    pub wStepVersion: u16,
    pub dwInputContentTypesCount: u32,
    pub prgInputContentTypes: *mut windows_core::PWSTR,
    pub dwOutputContentTypesCount: u32,
    pub prgOutputContentTypes: *mut windows_core::PWSTR,
    pub dwInputLanguagesCount: u32,
    pub prgInputLanguages: *mut windows_core::PWSTR,
    pub dwOutputLanguagesCount: u32,
    pub prgOutputLanguages: *mut windows_core::PWSTR,
    pub dwInputScriptsCount: u32,
    pub prgInputScripts: *mut windows_core::PWSTR,
    pub dwOutputScriptsCount: u32,
    pub prgOutputScripts: *mut windows_core::PWSTR,
    pub guid: windows_core::GUID,
    pub pszCategory: windows_core::PWSTR,
    pub pszDescription: windows_core::PWSTR,
    pub dwPrivateDataSize: u32,
    pub pPrivateData: *mut core::ffi::c_void,
    pub pContext: *mut core::ffi::c_void,
    pub _bitfield: u32,
}
impl windows_core::TypeKind for MAPPING_SERVICE_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for MAPPING_SERVICE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MIMECPINFO {
    pub dwFlags: u32,
    pub uiCodePage: u32,
    pub uiFamilyCodePage: u32,
    pub wszDescription: [u16; 64],
    pub wszWebCharset: [u16; 50],
    pub wszHeaderCharset: [u16; 50],
    pub wszBodyCharset: [u16; 50],
    pub wszFixedWidthFont: [u16; 32],
    pub wszProportionalFont: [u16; 32],
    pub bGDICharset: u8,
}
impl windows_core::TypeKind for MIMECPINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for MIMECPINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MIMECSETINFO {
    pub uiCodePage: u32,
    pub uiInternetEncoding: u32,
    pub wszCharset: [u16; 50],
}
impl windows_core::TypeKind for MIMECSETINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for MIMECSETINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NEWTEXTMETRICEXA {
    pub ntmTm: super::Graphics::Gdi::NEWTEXTMETRICA,
    pub ntmFontSig: FONTSIGNATURE,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::TypeKind for NEWTEXTMETRICEXA {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for NEWTEXTMETRICEXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NEWTEXTMETRICEXW {
    pub ntmTm: super::Graphics::Gdi::NEWTEXTMETRICW,
    pub ntmFontSig: FONTSIGNATURE,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::TypeKind for NEWTEXTMETRICEXW {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for NEWTEXTMETRICEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NLSVERSIONINFO {
    pub dwNLSVersionInfoSize: u32,
    pub dwNLSVersion: u32,
    pub dwDefinedVersion: u32,
    pub dwEffectiveId: u32,
    pub guidCustomVersion: windows_core::GUID,
}
impl windows_core::TypeKind for NLSVERSIONINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for NLSVERSIONINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NLSVERSIONINFOEX {
    pub dwNLSVersionInfoSize: u32,
    pub dwNLSVersion: u32,
    pub dwDefinedVersion: u32,
    pub dwEffectiveId: u32,
    pub guidCustomVersion: windows_core::GUID,
}
impl windows_core::TypeKind for NLSVERSIONINFOEX {
    type TypeKind = windows_core::CopyType;
}
impl Default for NLSVERSIONINFOEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NUMBERFMTA {
    pub NumDigits: u32,
    pub LeadingZero: u32,
    pub Grouping: u32,
    pub lpDecimalSep: windows_core::PSTR,
    pub lpThousandSep: windows_core::PSTR,
    pub NegativeOrder: u32,
}
impl windows_core::TypeKind for NUMBERFMTA {
    type TypeKind = windows_core::CopyType;
}
impl Default for NUMBERFMTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NUMBERFMTW {
    pub NumDigits: u32,
    pub LeadingZero: u32,
    pub Grouping: u32,
    pub lpDecimalSep: windows_core::PWSTR,
    pub lpThousandSep: windows_core::PWSTR,
    pub NegativeOrder: u32,
}
impl windows_core::TypeKind for NUMBERFMTW {
    type TypeKind = windows_core::CopyType;
}
impl Default for NUMBERFMTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OPENTYPE_FEATURE_RECORD {
    pub tagFeature: u32,
    pub lParameter: i32,
}
impl windows_core::TypeKind for OPENTYPE_FEATURE_RECORD {
    type TypeKind = windows_core::CopyType;
}
impl Default for OPENTYPE_FEATURE_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RFC1766INFO {
    pub lcid: u32,
    pub wszRfc1766: [u16; 6],
    pub wszLocaleName: [u16; 32],
}
impl windows_core::TypeKind for RFC1766INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for RFC1766INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCRIPTFONTINFO {
    pub scripts: i64,
    pub wszFont: [u16; 32],
}
impl windows_core::TypeKind for SCRIPTFONTINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for SCRIPTFONTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCRIPTINFO {
    pub ScriptId: u8,
    pub uiCodePage: u32,
    pub wszDescription: [u16; 48],
    pub wszFixedWidthFont: [u16; 32],
    pub wszProportionalFont: [u16; 32],
}
impl windows_core::TypeKind for SCRIPTINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for SCRIPTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCRIPT_ANALYSIS {
    pub _bitfield: u16,
    pub s: SCRIPT_STATE,
}
impl windows_core::TypeKind for SCRIPT_ANALYSIS {
    type TypeKind = windows_core::CopyType;
}
impl Default for SCRIPT_ANALYSIS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCRIPT_CHARPROP {
    pub _bitfield: u16,
}
impl windows_core::TypeKind for SCRIPT_CHARPROP {
    type TypeKind = windows_core::CopyType;
}
impl Default for SCRIPT_CHARPROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCRIPT_CONTROL {
    pub _bitfield: u32,
}
impl windows_core::TypeKind for SCRIPT_CONTROL {
    type TypeKind = windows_core::CopyType;
}
impl Default for SCRIPT_CONTROL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCRIPT_DIGITSUBSTITUTE {
    pub _bitfield1: u32,
    pub _bitfield2: u32,
    pub dwReserved: u32,
}
impl windows_core::TypeKind for SCRIPT_DIGITSUBSTITUTE {
    type TypeKind = windows_core::CopyType;
}
impl Default for SCRIPT_DIGITSUBSTITUTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCRIPT_FONTPROPERTIES {
    pub cBytes: i32,
    pub wgBlank: u16,
    pub wgDefault: u16,
    pub wgInvalid: u16,
    pub wgKashida: u16,
    pub iKashidaWidth: i32,
}
impl windows_core::TypeKind for SCRIPT_FONTPROPERTIES {
    type TypeKind = windows_core::CopyType;
}
impl Default for SCRIPT_FONTPROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCRIPT_GLYPHPROP {
    pub sva: SCRIPT_VISATTR,
    pub reserved: u16,
}
impl windows_core::TypeKind for SCRIPT_GLYPHPROP {
    type TypeKind = windows_core::CopyType;
}
impl Default for SCRIPT_GLYPHPROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCRIPT_ITEM {
    pub iCharPos: i32,
    pub a: SCRIPT_ANALYSIS,
}
impl windows_core::TypeKind for SCRIPT_ITEM {
    type TypeKind = windows_core::CopyType;
}
impl Default for SCRIPT_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCRIPT_LOGATTR {
    pub _bitfield: u8,
}
impl windows_core::TypeKind for SCRIPT_LOGATTR {
    type TypeKind = windows_core::CopyType;
}
impl Default for SCRIPT_LOGATTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCRIPT_PROPERTIES {
    pub _bitfield1: u32,
    pub _bitfield2: u32,
}
impl windows_core::TypeKind for SCRIPT_PROPERTIES {
    type TypeKind = windows_core::CopyType;
}
impl Default for SCRIPT_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCRIPT_STATE {
    pub _bitfield: u16,
}
impl windows_core::TypeKind for SCRIPT_STATE {
    type TypeKind = windows_core::CopyType;
}
impl Default for SCRIPT_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCRIPT_TABDEF {
    pub cTabStops: i32,
    pub iScale: i32,
    pub pTabStops: *mut i32,
    pub iTabOrigin: i32,
}
impl windows_core::TypeKind for SCRIPT_TABDEF {
    type TypeKind = windows_core::CopyType;
}
impl Default for SCRIPT_TABDEF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCRIPT_VISATTR {
    pub _bitfield: u16,
}
impl windows_core::TypeKind for SCRIPT_VISATTR {
    type TypeKind = windows_core::CopyType;
}
impl Default for SCRIPT_VISATTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SpellCheckerFactory: windows_core::GUID = windows_core::GUID::from_u128(0x7ab36653_1796_484b_bdfa_e74f1db7c1dc);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TEXTRANGE_PROPERTIES {
    pub potfRecords: *mut OPENTYPE_FEATURE_RECORD,
    pub cotfRecords: i32,
}
impl windows_core::TypeKind for TEXTRANGE_PROPERTIES {
    type TypeKind = windows_core::CopyType;
}
impl Default for TEXTRANGE_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UBiDi(pub isize);
impl Default for UBiDi {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for UBiDi {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UBiDiTransform(pub isize);
impl Default for UBiDiTransform {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for UBiDiTransform {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UBreakIterator(pub isize);
impl Default for UBreakIterator {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for UBreakIterator {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UCPMap(pub isize);
impl Default for UCPMap {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for UCPMap {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UCPTrie {
    pub index: *const u16,
    pub data: UCPTrieData,
    pub indexLength: i32,
    pub dataLength: i32,
    pub highStart: i32,
    pub shifted12HighStart: u16,
    pub r#type: i8,
    pub valueWidth: i8,
    pub reserved32: u32,
    pub reserved16: u16,
    pub index3NullOffset: u16,
    pub dataNullOffset: i32,
    pub nullValue: u32,
}
impl windows_core::TypeKind for UCPTrie {
    type TypeKind = windows_core::CopyType;
}
impl Default for UCPTrie {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union UCPTrieData {
    pub ptr0: *const core::ffi::c_void,
    pub ptr16: *const u16,
    pub ptr32: *const u32,
    pub ptr8: *const u8,
}
impl windows_core::TypeKind for UCPTrieData {
    type TypeKind = windows_core::CopyType;
}
impl Default for UCPTrieData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UCaseMap(pub isize);
impl Default for UCaseMap {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for UCaseMap {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct UCharIterator {
    pub context: *const core::ffi::c_void,
    pub length: i32,
    pub start: i32,
    pub index: i32,
    pub limit: i32,
    pub reservedField: i32,
    pub getIndex: UCharIteratorGetIndex,
    pub r#move: UCharIteratorMove,
    pub hasNext: UCharIteratorHasNext,
    pub hasPrevious: UCharIteratorHasPrevious,
    pub current: UCharIteratorCurrent,
    pub next: UCharIteratorNext,
    pub previous: UCharIteratorPrevious,
    pub reservedFn: UCharIteratorReserved,
    pub getState: UCharIteratorGetState,
    pub setState: UCharIteratorSetState,
}
impl windows_core::TypeKind for UCharIterator {
    type TypeKind = windows_core::CopyType;
}
impl Default for UCharIterator {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UCharsetDetector(pub isize);
impl Default for UCharsetDetector {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for UCharsetDetector {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UCharsetMatch(pub isize);
impl Default for UCharsetMatch {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for UCharsetMatch {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UCollationElements(pub isize);
impl Default for UCollationElements {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for UCollationElements {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UCollator(pub isize);
impl Default for UCollator {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for UCollator {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UConstrainedFieldPosition(pub isize);
impl Default for UConstrainedFieldPosition {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for UConstrainedFieldPosition {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UConverter(pub isize);
impl Default for UConverter {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for UConverter {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UConverterFromUnicodeArgs {
    pub size: u16,
    pub flush: i8,
    pub converter: *mut UConverter,
    pub source: *const u16,
    pub sourceLimit: *const u16,
    pub target: windows_core::PSTR,
    pub targetLimit: windows_core::PCSTR,
    pub offsets: *mut i32,
}
impl windows_core::TypeKind for UConverterFromUnicodeArgs {
    type TypeKind = windows_core::CopyType;
}
impl Default for UConverterFromUnicodeArgs {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UConverterSelector(pub isize);
impl Default for UConverterSelector {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for UConverterSelector {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UConverterToUnicodeArgs {
    pub size: u16,
    pub flush: i8,
    pub converter: *mut UConverter,
    pub source: windows_core::PCSTR,
    pub sourceLimit: windows_core::PCSTR,
    pub target: *mut u16,
    pub targetLimit: *const u16,
    pub offsets: *mut i32,
}
impl windows_core::TypeKind for UConverterToUnicodeArgs {
    type TypeKind = windows_core::CopyType;
}
impl Default for UConverterToUnicodeArgs {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UDateFormatSymbols(pub isize);
impl Default for UDateFormatSymbols {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for UDateFormatSymbols {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UDateIntervalFormat(pub isize);
impl Default for UDateIntervalFormat {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for UDateIntervalFormat {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UEnumeration(pub isize);
impl Default for UEnumeration {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for UEnumeration {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UFieldPosition {
    pub field: i32,
    pub beginIndex: i32,
    pub endIndex: i32,
}
impl windows_core::TypeKind for UFieldPosition {
    type TypeKind = windows_core::CopyType;
}
impl Default for UFieldPosition {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UFieldPositionIterator(pub isize);
impl Default for UFieldPositionIterator {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for UFieldPositionIterator {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UFormattedDateInterval(pub isize);
impl Default for UFormattedDateInterval {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for UFormattedDateInterval {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UFormattedList(pub isize);
impl Default for UFormattedList {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for UFormattedList {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UFormattedNumber(pub isize);
impl Default for UFormattedNumber {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for UFormattedNumber {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UFormattedNumberRange(pub isize);
impl Default for UFormattedNumberRange {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for UFormattedNumberRange {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UFormattedRelativeDateTime(pub isize);
impl Default for UFormattedRelativeDateTime {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for UFormattedRelativeDateTime {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UFormattedValue(pub isize);
impl Default for UFormattedValue {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for UFormattedValue {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UGenderInfo(pub isize);
impl Default for UGenderInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for UGenderInfo {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UHashtable(pub isize);
impl Default for UHashtable {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for UHashtable {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UIDNA(pub isize);
impl Default for UIDNA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for UIDNA {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UIDNAInfo {
    pub size: i16,
    pub isTransitionalDifferent: i8,
    pub reservedB3: i8,
    pub errors: u32,
    pub reservedI2: i32,
    pub reservedI3: i32,
}
impl windows_core::TypeKind for UIDNAInfo {
    type TypeKind = windows_core::CopyType;
}
impl Default for UIDNAInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UListFormatter(pub isize);
impl Default for UListFormatter {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for UListFormatter {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ULocaleData(pub isize);
impl Default for ULocaleData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for ULocaleData {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ULocaleDisplayNames(pub isize);
impl Default for ULocaleDisplayNames {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for ULocaleDisplayNames {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UMutableCPTrie(pub isize);
impl Default for UMutableCPTrie {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for UMutableCPTrie {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UNICODERANGE {
    pub wcFrom: u16,
    pub wcTo: u16,
}
impl windows_core::TypeKind for UNICODERANGE {
    type TypeKind = windows_core::CopyType;
}
impl Default for UNICODERANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UNormalizer2(pub isize);
impl Default for UNormalizer2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for UNormalizer2 {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UNumberFormatter(pub isize);
impl Default for UNumberFormatter {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for UNumberFormatter {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UNumberingSystem(pub isize);
impl Default for UNumberingSystem {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for UNumberingSystem {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UParseError {
    pub line: i32,
    pub offset: i32,
    pub preContext: [u16; 16],
    pub postContext: [u16; 16],
}
impl windows_core::TypeKind for UParseError {
    type TypeKind = windows_core::CopyType;
}
impl Default for UParseError {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UPluralRules(pub isize);
impl Default for UPluralRules {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for UPluralRules {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct URegion(pub isize);
impl Default for URegion {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for URegion {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct URegularExpression(pub isize);
impl Default for URegularExpression {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for URegularExpression {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct URelativeDateTimeFormatter(pub isize);
impl Default for URelativeDateTimeFormatter {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for URelativeDateTimeFormatter {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UReplaceableCallbacks {
    pub length: isize,
    pub charAt: isize,
    pub char32At: isize,
    pub replace: isize,
    pub extract: isize,
    pub copy: isize,
}
impl windows_core::TypeKind for UReplaceableCallbacks {
    type TypeKind = windows_core::CopyType;
}
impl Default for UReplaceableCallbacks {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UResourceBundle(pub isize);
impl Default for UResourceBundle {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for UResourceBundle {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct USearch(pub isize);
impl Default for USearch {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for USearch {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USerializedSet {
    pub array: *const u16,
    pub bmpLength: i32,
    pub length: i32,
    pub staticArray: [u16; 8],
}
impl windows_core::TypeKind for USerializedSet {
    type TypeKind = windows_core::CopyType;
}
impl Default for USerializedSet {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct USet(pub isize);
impl Default for USet {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for USet {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct USpoofCheckResult(pub isize);
impl Default for USpoofCheckResult {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for USpoofCheckResult {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct USpoofChecker(pub isize);
impl Default for USpoofChecker {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for USpoofChecker {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UStringPrepProfile(pub isize);
impl Default for UStringPrepProfile {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for UStringPrepProfile {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UStringSearch(pub isize);
impl Default for UStringSearch {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for UStringSearch {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UText {
    pub magic: u32,
    pub flags: i32,
    pub providerProperties: i32,
    pub sizeOfStruct: i32,
    pub chunkNativeLimit: i64,
    pub extraSize: i32,
    pub nativeIndexingLimit: i32,
    pub chunkNativeStart: i64,
    pub chunkOffset: i32,
    pub chunkLength: i32,
    pub chunkContents: *const u16,
    pub pFuncs: *const UTextFuncs,
    pub pExtra: *mut core::ffi::c_void,
    pub context: *const core::ffi::c_void,
    pub p: *const core::ffi::c_void,
    pub q: *const core::ffi::c_void,
    pub r: *const core::ffi::c_void,
    pub privP: *mut core::ffi::c_void,
    pub a: i64,
    pub b: i32,
    pub c: i32,
    pub privA: i64,
    pub privB: i32,
    pub privC: i32,
}
impl windows_core::TypeKind for UText {
    type TypeKind = windows_core::CopyType;
}
impl Default for UText {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct UTextFuncs {
    pub tableSize: i32,
    pub reserved1: i32,
    pub reserved2: i32,
    pub reserved3: i32,
    pub clone: UTextClone,
    pub nativeLength: UTextNativeLength,
    pub access: UTextAccess,
    pub extract: UTextExtract,
    pub replace: UTextReplace,
    pub copy: UTextCopy,
    pub mapOffsetToNative: UTextMapOffsetToNative,
    pub mapNativeIndexToUTF16: UTextMapNativeIndexToUTF16,
    pub close: UTextClose,
    pub spare1: UTextClose,
    pub spare2: UTextClose,
    pub spare3: UTextClose,
}
impl windows_core::TypeKind for UTextFuncs {
    type TypeKind = windows_core::CopyType;
}
impl Default for UTextFuncs {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UTransPosition {
    pub contextStart: i32,
    pub contextLimit: i32,
    pub start: i32,
    pub limit: i32,
}
impl windows_core::TypeKind for UTransPosition {
    type TypeKind = windows_core::CopyType;
}
impl Default for UTransPosition {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CALINFO_ENUMPROCA = Option<unsafe extern "system" fn(param0: windows_core::PCSTR) -> super::Foundation::BOOL>;
pub type CALINFO_ENUMPROCEXA = Option<unsafe extern "system" fn(param0: windows_core::PCSTR, param1: u32) -> super::Foundation::BOOL>;
pub type CALINFO_ENUMPROCEXEX = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: u32, param2: windows_core::PCWSTR, param3: super::Foundation::LPARAM) -> super::Foundation::BOOL>;
pub type CALINFO_ENUMPROCEXW = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: u32) -> super::Foundation::BOOL>;
pub type CALINFO_ENUMPROCW = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR) -> super::Foundation::BOOL>;
pub type CODEPAGE_ENUMPROCA = Option<unsafe extern "system" fn(param0: windows_core::PCSTR) -> super::Foundation::BOOL>;
pub type CODEPAGE_ENUMPROCW = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR) -> super::Foundation::BOOL>;
pub type DATEFMT_ENUMPROCA = Option<unsafe extern "system" fn(param0: windows_core::PCSTR) -> super::Foundation::BOOL>;
pub type DATEFMT_ENUMPROCEXA = Option<unsafe extern "system" fn(param0: windows_core::PCSTR, param1: u32) -> super::Foundation::BOOL>;
pub type DATEFMT_ENUMPROCEXEX = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: u32, param2: super::Foundation::LPARAM) -> super::Foundation::BOOL>;
pub type DATEFMT_ENUMPROCEXW = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: u32) -> super::Foundation::BOOL>;
pub type DATEFMT_ENUMPROCW = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR) -> super::Foundation::BOOL>;
pub type GEO_ENUMNAMEPROC = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: super::Foundation::LPARAM) -> super::Foundation::BOOL>;
pub type GEO_ENUMPROC = Option<unsafe extern "system" fn(param0: i32) -> super::Foundation::BOOL>;
pub type LANGGROUPLOCALE_ENUMPROCA = Option<unsafe extern "system" fn(param0: u32, param1: u32, param2: windows_core::PCSTR, param3: isize) -> super::Foundation::BOOL>;
pub type LANGGROUPLOCALE_ENUMPROCW = Option<unsafe extern "system" fn(param0: u32, param1: u32, param2: windows_core::PCWSTR, param3: isize) -> super::Foundation::BOOL>;
pub type LANGUAGEGROUP_ENUMPROCA = Option<unsafe extern "system" fn(param0: u32, param1: windows_core::PCSTR, param2: windows_core::PCSTR, param3: u32, param4: isize) -> super::Foundation::BOOL>;
pub type LANGUAGEGROUP_ENUMPROCW = Option<unsafe extern "system" fn(param0: u32, param1: windows_core::PCWSTR, param2: windows_core::PCWSTR, param3: u32, param4: isize) -> super::Foundation::BOOL>;
pub type LOCALE_ENUMPROCA = Option<unsafe extern "system" fn(param0: windows_core::PCSTR) -> super::Foundation::BOOL>;
pub type LOCALE_ENUMPROCEX = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: u32, param2: super::Foundation::LPARAM) -> super::Foundation::BOOL>;
pub type LOCALE_ENUMPROCW = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR) -> super::Foundation::BOOL>;
pub type PFN_MAPPINGCALLBACKPROC = Option<unsafe extern "system" fn(pbag: *mut MAPPING_PROPERTY_BAG, data: *mut core::ffi::c_void, dwdatasize: u32, result: windows_core::HRESULT)>;
pub type TIMEFMT_ENUMPROCA = Option<unsafe extern "system" fn(param0: windows_core::PCSTR) -> super::Foundation::BOOL>;
pub type TIMEFMT_ENUMPROCEX = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: super::Foundation::LPARAM) -> super::Foundation::BOOL>;
pub type TIMEFMT_ENUMPROCW = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR) -> super::Foundation::BOOL>;
pub type UBiDiClassCallback = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, c: i32) -> UCharDirection>;
pub type UCPMapValueFilter = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, value: u32) -> u32>;
pub type UCharEnumTypeRange = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, start: i32, limit: i32, r#type: UCharCategory) -> i8>;
pub type UCharIteratorCurrent = Option<unsafe extern "system" fn(iter: *mut UCharIterator) -> i32>;
pub type UCharIteratorGetIndex = Option<unsafe extern "system" fn(iter: *mut UCharIterator, origin: UCharIteratorOrigin) -> i32>;
pub type UCharIteratorGetState = Option<unsafe extern "system" fn(iter: *const UCharIterator) -> u32>;
pub type UCharIteratorHasNext = Option<unsafe extern "system" fn(iter: *mut UCharIterator) -> i8>;
pub type UCharIteratorHasPrevious = Option<unsafe extern "system" fn(iter: *mut UCharIterator) -> i8>;
pub type UCharIteratorMove = Option<unsafe extern "system" fn(iter: *mut UCharIterator, delta: i32, origin: UCharIteratorOrigin) -> i32>;
pub type UCharIteratorNext = Option<unsafe extern "system" fn(iter: *mut UCharIterator) -> i32>;
pub type UCharIteratorPrevious = Option<unsafe extern "system" fn(iter: *mut UCharIterator) -> i32>;
pub type UCharIteratorReserved = Option<unsafe extern "system" fn(iter: *mut UCharIterator, something: i32) -> i32>;
pub type UCharIteratorSetState = Option<unsafe extern "system" fn(iter: *mut UCharIterator, state: u32, perrorcode: *mut UErrorCode)>;
pub type UConverterFromUCallback = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, args: *mut UConverterFromUnicodeArgs, codeunits: *const u16, length: i32, codepoint: i32, reason: UConverterCallbackReason, perrorcode: *mut UErrorCode)>;
pub type UConverterToUCallback = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, args: *mut UConverterToUnicodeArgs, codeunits: windows_core::PCSTR, length: i32, reason: UConverterCallbackReason, perrorcode: *mut UErrorCode)>;
pub type UEnumCharNamesFn = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, code: i32, namechoice: UCharNameChoice, name: windows_core::PCSTR, length: i32) -> i8>;
pub type UILANGUAGE_ENUMPROCA = Option<unsafe extern "system" fn(param0: windows_core::PCSTR, param1: isize) -> super::Foundation::BOOL>;
pub type UILANGUAGE_ENUMPROCW = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: isize) -> super::Foundation::BOOL>;
pub type UMemAllocFn = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, size: usize) -> *mut core::ffi::c_void>;
pub type UMemFreeFn = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, mem: *mut core::ffi::c_void)>;
pub type UMemReallocFn = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, mem: *mut core::ffi::c_void, size: usize) -> *mut core::ffi::c_void>;
pub type UNESCAPE_CHAR_AT = Option<unsafe extern "system" fn(offset: i32, context: *mut core::ffi::c_void) -> u16>;
pub type URegexFindProgressCallback = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, matchindex: i64) -> i8>;
pub type URegexMatchCallback = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, steps: i32) -> i8>;
pub type UStringCaseMapper = Option<unsafe extern "system" fn(csm: *const UCaseMap, dest: *mut u16, destcapacity: i32, src: *const u16, srclength: i32, perrorcode: *mut UErrorCode) -> i32>;
pub type UTextAccess = Option<unsafe extern "system" fn(ut: *mut UText, nativeindex: i64, forward: i8) -> i8>;
pub type UTextClone = Option<unsafe extern "system" fn(dest: *mut UText, src: *const UText, deep: i8, status: *mut UErrorCode) -> *mut UText>;
pub type UTextClose = Option<unsafe extern "system" fn(ut: *mut UText)>;
pub type UTextCopy = Option<unsafe extern "system" fn(ut: *mut UText, nativestart: i64, nativelimit: i64, nativedest: i64, r#move: i8, status: *mut UErrorCode)>;
pub type UTextExtract = Option<unsafe extern "system" fn(ut: *mut UText, nativestart: i64, nativelimit: i64, dest: *mut u16, destcapacity: i32, status: *mut UErrorCode) -> i32>;
pub type UTextMapNativeIndexToUTF16 = Option<unsafe extern "system" fn(ut: *const UText, nativeindex: i64) -> i32>;
pub type UTextMapOffsetToNative = Option<unsafe extern "system" fn(ut: *const UText) -> i64>;
pub type UTextNativeLength = Option<unsafe extern "system" fn(ut: *mut UText) -> i64>;
pub type UTextReplace = Option<unsafe extern "system" fn(ut: *mut UText, nativestart: i64, nativelimit: i64, replacementtext: *const u16, replacmentlength: i32, status: *mut UErrorCode) -> i32>;
pub type UTraceData = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, fnnumber: i32, level: i32, fmt: windows_core::PCSTR, args: *mut i8)>;
pub type UTraceEntry = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, fnnumber: i32)>;
pub type UTraceExit = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, fnnumber: i32, fmt: windows_core::PCSTR, args: *mut i8)>;
#[cfg(feature = "implement")]
core::include!("impl.rs");
