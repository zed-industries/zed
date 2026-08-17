#[inline]
pub unsafe fn AdjustCalendarDate(lpcaldatetime: *mut CALDATETIME, calunit: CALDATETIME_DATEUNIT, amount: i32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn AdjustCalendarDate(lpcaldatetime : *mut CALDATETIME, calunit : CALDATETIME_DATEUNIT, amount : i32) -> windows_core::BOOL);
    unsafe { AdjustCalendarDate(lpcaldatetime as _, calunit, amount) }
}
#[inline]
pub unsafe fn CompareStringA(locale: u32, dwcmpflags: u32, lpstring1: &[i8], lpstring2: &[i8]) -> COMPARESTRING_RESULT {
    windows_core::link!("kernel32.dll" "system" fn CompareStringA(locale : u32, dwcmpflags : u32, lpstring1 : *const i8, cchcount1 : i32, lpstring2 : *const i8, cchcount2 : i32) -> COMPARESTRING_RESULT);
    unsafe { CompareStringA(locale, dwcmpflags, core::mem::transmute(lpstring1.as_ptr()), lpstring1.len().try_into().unwrap(), core::mem::transmute(lpstring2.as_ptr()), lpstring2.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn CompareStringEx<P0>(lplocalename: P0, dwcmpflags: COMPARE_STRING_FLAGS, lpstring1: &[u16], lpstring2: &[u16], lpversioninformation: Option<*const NLSVERSIONINFO>, lpreserved: Option<*const core::ffi::c_void>, lparam: Option<super::Foundation::LPARAM>) -> COMPARESTRING_RESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CompareStringEx(lplocalename : windows_core::PCWSTR, dwcmpflags : COMPARE_STRING_FLAGS, lpstring1 : windows_core::PCWSTR, cchcount1 : i32, lpstring2 : windows_core::PCWSTR, cchcount2 : i32, lpversioninformation : *const NLSVERSIONINFO, lpreserved : *const core::ffi::c_void, lparam : super::Foundation:: LPARAM) -> COMPARESTRING_RESULT);
    unsafe { CompareStringEx(lplocalename.param().abi(), dwcmpflags, core::mem::transmute(lpstring1.as_ptr()), lpstring1.len().try_into().unwrap(), core::mem::transmute(lpstring2.as_ptr()), lpstring2.len().try_into().unwrap(), lpversioninformation.unwrap_or(core::mem::zeroed()) as _, lpreserved.unwrap_or(core::mem::zeroed()) as _, lparam.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CompareStringOrdinal(lpstring1: &[u16], lpstring2: &[u16], bignorecase: bool) -> COMPARESTRING_RESULT {
    windows_core::link!("kernel32.dll" "system" fn CompareStringOrdinal(lpstring1 : windows_core::PCWSTR, cchcount1 : i32, lpstring2 : windows_core::PCWSTR, cchcount2 : i32, bignorecase : windows_core::BOOL) -> COMPARESTRING_RESULT);
    unsafe { CompareStringOrdinal(core::mem::transmute(lpstring1.as_ptr()), lpstring1.len().try_into().unwrap(), core::mem::transmute(lpstring2.as_ptr()), lpstring2.len().try_into().unwrap(), bignorecase.into()) }
}
#[inline]
pub unsafe fn CompareStringW(locale: u32, dwcmpflags: u32, lpstring1: &[u16], lpstring2: &[u16]) -> COMPARESTRING_RESULT {
    windows_core::link!("kernel32.dll" "system" fn CompareStringW(locale : u32, dwcmpflags : u32, lpstring1 : windows_core::PCWSTR, cchcount1 : i32, lpstring2 : windows_core::PCWSTR, cchcount2 : i32) -> COMPARESTRING_RESULT);
    unsafe { CompareStringW(locale, dwcmpflags, core::mem::transmute(lpstring1.as_ptr()), lpstring1.len().try_into().unwrap(), core::mem::transmute(lpstring2.as_ptr()), lpstring2.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn ConvertCalDateTimeToSystemTime(lpcaldatetime: *const CALDATETIME, lpsystime: *mut super::Foundation::SYSTEMTIME) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn ConvertCalDateTimeToSystemTime(lpcaldatetime : *const CALDATETIME, lpsystime : *mut super::Foundation:: SYSTEMTIME) -> windows_core::BOOL);
    unsafe { ConvertCalDateTimeToSystemTime(lpcaldatetime, lpsystime as _) }
}
#[inline]
pub unsafe fn ConvertDefaultLocale(locale: u32) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn ConvertDefaultLocale(locale : u32) -> u32);
    unsafe { ConvertDefaultLocale(locale) }
}
#[inline]
pub unsafe fn ConvertSystemTimeToCalDateTime(lpsystime: *const super::Foundation::SYSTEMTIME, calid: u32, lpcaldatetime: *mut CALDATETIME) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn ConvertSystemTimeToCalDateTime(lpsystime : *const super::Foundation:: SYSTEMTIME, calid : u32, lpcaldatetime : *mut CALDATETIME) -> windows_core::BOOL);
    unsafe { ConvertSystemTimeToCalDateTime(lpsystime, calid, lpcaldatetime as _) }
}
#[inline]
pub unsafe fn EnumCalendarInfoA(lpcalinfoenumproc: CALINFO_ENUMPROCA, locale: u32, calendar: u32, caltype: u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn EnumCalendarInfoA(lpcalinfoenumproc : CALINFO_ENUMPROCA, locale : u32, calendar : u32, caltype : u32) -> windows_core::BOOL);
    unsafe { EnumCalendarInfoA(lpcalinfoenumproc, locale, calendar, caltype).ok() }
}
#[inline]
pub unsafe fn EnumCalendarInfoExA(lpcalinfoenumprocex: CALINFO_ENUMPROCEXA, locale: u32, calendar: u32, caltype: u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn EnumCalendarInfoExA(lpcalinfoenumprocex : CALINFO_ENUMPROCEXA, locale : u32, calendar : u32, caltype : u32) -> windows_core::BOOL);
    unsafe { EnumCalendarInfoExA(lpcalinfoenumprocex, locale, calendar, caltype).ok() }
}
#[inline]
pub unsafe fn EnumCalendarInfoExEx<P1, P3>(pcalinfoenumprocexex: CALINFO_ENUMPROCEXEX, lplocalename: P1, calendar: u32, lpreserved: P3, caltype: u32, lparam: super::Foundation::LPARAM) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn EnumCalendarInfoExEx(pcalinfoenumprocexex : CALINFO_ENUMPROCEXEX, lplocalename : windows_core::PCWSTR, calendar : u32, lpreserved : windows_core::PCWSTR, caltype : u32, lparam : super::Foundation:: LPARAM) -> windows_core::BOOL);
    unsafe { EnumCalendarInfoExEx(pcalinfoenumprocexex, lplocalename.param().abi(), calendar, lpreserved.param().abi(), caltype, lparam).ok() }
}
#[inline]
pub unsafe fn EnumCalendarInfoExW(lpcalinfoenumprocex: CALINFO_ENUMPROCEXW, locale: u32, calendar: u32, caltype: u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn EnumCalendarInfoExW(lpcalinfoenumprocex : CALINFO_ENUMPROCEXW, locale : u32, calendar : u32, caltype : u32) -> windows_core::BOOL);
    unsafe { EnumCalendarInfoExW(lpcalinfoenumprocex, locale, calendar, caltype).ok() }
}
#[inline]
pub unsafe fn EnumCalendarInfoW(lpcalinfoenumproc: CALINFO_ENUMPROCW, locale: u32, calendar: u32, caltype: u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn EnumCalendarInfoW(lpcalinfoenumproc : CALINFO_ENUMPROCW, locale : u32, calendar : u32, caltype : u32) -> windows_core::BOOL);
    unsafe { EnumCalendarInfoW(lpcalinfoenumproc, locale, calendar, caltype).ok() }
}
#[inline]
pub unsafe fn EnumDateFormatsA(lpdatefmtenumproc: DATEFMT_ENUMPROCA, locale: u32, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn EnumDateFormatsA(lpdatefmtenumproc : DATEFMT_ENUMPROCA, locale : u32, dwflags : u32) -> windows_core::BOOL);
    unsafe { EnumDateFormatsA(lpdatefmtenumproc, locale, dwflags).ok() }
}
#[inline]
pub unsafe fn EnumDateFormatsExA(lpdatefmtenumprocex: DATEFMT_ENUMPROCEXA, locale: u32, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn EnumDateFormatsExA(lpdatefmtenumprocex : DATEFMT_ENUMPROCEXA, locale : u32, dwflags : u32) -> windows_core::BOOL);
    unsafe { EnumDateFormatsExA(lpdatefmtenumprocex, locale, dwflags).ok() }
}
#[inline]
pub unsafe fn EnumDateFormatsExEx<P1>(lpdatefmtenumprocexex: DATEFMT_ENUMPROCEXEX, lplocalename: P1, dwflags: ENUM_DATE_FORMATS_FLAGS, lparam: super::Foundation::LPARAM) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn EnumDateFormatsExEx(lpdatefmtenumprocexex : DATEFMT_ENUMPROCEXEX, lplocalename : windows_core::PCWSTR, dwflags : ENUM_DATE_FORMATS_FLAGS, lparam : super::Foundation:: LPARAM) -> windows_core::BOOL);
    unsafe { EnumDateFormatsExEx(lpdatefmtenumprocexex, lplocalename.param().abi(), dwflags, lparam).ok() }
}
#[inline]
pub unsafe fn EnumDateFormatsExW(lpdatefmtenumprocex: DATEFMT_ENUMPROCEXW, locale: u32, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn EnumDateFormatsExW(lpdatefmtenumprocex : DATEFMT_ENUMPROCEXW, locale : u32, dwflags : u32) -> windows_core::BOOL);
    unsafe { EnumDateFormatsExW(lpdatefmtenumprocex, locale, dwflags).ok() }
}
#[inline]
pub unsafe fn EnumDateFormatsW(lpdatefmtenumproc: DATEFMT_ENUMPROCW, locale: u32, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn EnumDateFormatsW(lpdatefmtenumproc : DATEFMT_ENUMPROCW, locale : u32, dwflags : u32) -> windows_core::BOOL);
    unsafe { EnumDateFormatsW(lpdatefmtenumproc, locale, dwflags).ok() }
}
#[inline]
pub unsafe fn EnumLanguageGroupLocalesA(lplanggrouplocaleenumproc: LANGGROUPLOCALE_ENUMPROCA, languagegroup: u32, dwflags: u32, lparam: isize) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn EnumLanguageGroupLocalesA(lplanggrouplocaleenumproc : LANGGROUPLOCALE_ENUMPROCA, languagegroup : u32, dwflags : u32, lparam : isize) -> windows_core::BOOL);
    unsafe { EnumLanguageGroupLocalesA(lplanggrouplocaleenumproc, languagegroup, dwflags, lparam).ok() }
}
#[inline]
pub unsafe fn EnumLanguageGroupLocalesW(lplanggrouplocaleenumproc: LANGGROUPLOCALE_ENUMPROCW, languagegroup: u32, dwflags: u32, lparam: isize) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn EnumLanguageGroupLocalesW(lplanggrouplocaleenumproc : LANGGROUPLOCALE_ENUMPROCW, languagegroup : u32, dwflags : u32, lparam : isize) -> windows_core::BOOL);
    unsafe { EnumLanguageGroupLocalesW(lplanggrouplocaleenumproc, languagegroup, dwflags, lparam).ok() }
}
#[inline]
pub unsafe fn EnumSystemCodePagesA(lpcodepageenumproc: CODEPAGE_ENUMPROCA, dwflags: ENUM_SYSTEM_CODE_PAGES_FLAGS) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn EnumSystemCodePagesA(lpcodepageenumproc : CODEPAGE_ENUMPROCA, dwflags : ENUM_SYSTEM_CODE_PAGES_FLAGS) -> windows_core::BOOL);
    unsafe { EnumSystemCodePagesA(lpcodepageenumproc, dwflags).ok() }
}
#[inline]
pub unsafe fn EnumSystemCodePagesW(lpcodepageenumproc: CODEPAGE_ENUMPROCW, dwflags: ENUM_SYSTEM_CODE_PAGES_FLAGS) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn EnumSystemCodePagesW(lpcodepageenumproc : CODEPAGE_ENUMPROCW, dwflags : ENUM_SYSTEM_CODE_PAGES_FLAGS) -> windows_core::BOOL);
    unsafe { EnumSystemCodePagesW(lpcodepageenumproc, dwflags).ok() }
}
#[inline]
pub unsafe fn EnumSystemGeoID(geoclass: u32, parentgeoid: i32, lpgeoenumproc: GEO_ENUMPROC) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn EnumSystemGeoID(geoclass : u32, parentgeoid : i32, lpgeoenumproc : GEO_ENUMPROC) -> windows_core::BOOL);
    unsafe { EnumSystemGeoID(geoclass, parentgeoid, lpgeoenumproc).ok() }
}
#[inline]
pub unsafe fn EnumSystemGeoNames(geoclass: u32, geoenumproc: GEO_ENUMNAMEPROC, data: super::Foundation::LPARAM) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn EnumSystemGeoNames(geoclass : u32, geoenumproc : GEO_ENUMNAMEPROC, data : super::Foundation:: LPARAM) -> windows_core::BOOL);
    unsafe { EnumSystemGeoNames(geoclass, geoenumproc, data).ok() }
}
#[inline]
pub unsafe fn EnumSystemLanguageGroupsA(lplanguagegroupenumproc: LANGUAGEGROUP_ENUMPROCA, dwflags: ENUM_SYSTEM_LANGUAGE_GROUPS_FLAGS, lparam: isize) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn EnumSystemLanguageGroupsA(lplanguagegroupenumproc : LANGUAGEGROUP_ENUMPROCA, dwflags : ENUM_SYSTEM_LANGUAGE_GROUPS_FLAGS, lparam : isize) -> windows_core::BOOL);
    unsafe { EnumSystemLanguageGroupsA(lplanguagegroupenumproc, dwflags, lparam).ok() }
}
#[inline]
pub unsafe fn EnumSystemLanguageGroupsW(lplanguagegroupenumproc: LANGUAGEGROUP_ENUMPROCW, dwflags: ENUM_SYSTEM_LANGUAGE_GROUPS_FLAGS, lparam: isize) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn EnumSystemLanguageGroupsW(lplanguagegroupenumproc : LANGUAGEGROUP_ENUMPROCW, dwflags : ENUM_SYSTEM_LANGUAGE_GROUPS_FLAGS, lparam : isize) -> windows_core::BOOL);
    unsafe { EnumSystemLanguageGroupsW(lplanguagegroupenumproc, dwflags, lparam).ok() }
}
#[inline]
pub unsafe fn EnumSystemLocalesA(lplocaleenumproc: LOCALE_ENUMPROCA, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn EnumSystemLocalesA(lplocaleenumproc : LOCALE_ENUMPROCA, dwflags : u32) -> windows_core::BOOL);
    unsafe { EnumSystemLocalesA(lplocaleenumproc, dwflags).ok() }
}
#[inline]
pub unsafe fn EnumSystemLocalesEx(lplocaleenumprocex: LOCALE_ENUMPROCEX, dwflags: u32, lparam: super::Foundation::LPARAM, lpreserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn EnumSystemLocalesEx(lplocaleenumprocex : LOCALE_ENUMPROCEX, dwflags : u32, lparam : super::Foundation:: LPARAM, lpreserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { EnumSystemLocalesEx(lplocaleenumprocex, dwflags, lparam, lpreserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn EnumSystemLocalesW(lplocaleenumproc: LOCALE_ENUMPROCW, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn EnumSystemLocalesW(lplocaleenumproc : LOCALE_ENUMPROCW, dwflags : u32) -> windows_core::BOOL);
    unsafe { EnumSystemLocalesW(lplocaleenumproc, dwflags).ok() }
}
#[inline]
pub unsafe fn EnumTimeFormatsA(lptimefmtenumproc: TIMEFMT_ENUMPROCA, locale: u32, dwflags: TIME_FORMAT_FLAGS) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn EnumTimeFormatsA(lptimefmtenumproc : TIMEFMT_ENUMPROCA, locale : u32, dwflags : TIME_FORMAT_FLAGS) -> windows_core::BOOL);
    unsafe { EnumTimeFormatsA(lptimefmtenumproc, locale, dwflags).ok() }
}
#[inline]
pub unsafe fn EnumTimeFormatsEx<P1>(lptimefmtenumprocex: TIMEFMT_ENUMPROCEX, lplocalename: P1, dwflags: u32, lparam: super::Foundation::LPARAM) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn EnumTimeFormatsEx(lptimefmtenumprocex : TIMEFMT_ENUMPROCEX, lplocalename : windows_core::PCWSTR, dwflags : u32, lparam : super::Foundation:: LPARAM) -> windows_core::BOOL);
    unsafe { EnumTimeFormatsEx(lptimefmtenumprocex, lplocalename.param().abi(), dwflags, lparam).ok() }
}
#[inline]
pub unsafe fn EnumTimeFormatsW(lptimefmtenumproc: TIMEFMT_ENUMPROCW, locale: u32, dwflags: TIME_FORMAT_FLAGS) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn EnumTimeFormatsW(lptimefmtenumproc : TIMEFMT_ENUMPROCW, locale : u32, dwflags : TIME_FORMAT_FLAGS) -> windows_core::BOOL);
    unsafe { EnumTimeFormatsW(lptimefmtenumproc, locale, dwflags).ok() }
}
#[inline]
pub unsafe fn EnumUILanguagesA(lpuilanguageenumproc: UILANGUAGE_ENUMPROCA, dwflags: u32, lparam: isize) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn EnumUILanguagesA(lpuilanguageenumproc : UILANGUAGE_ENUMPROCA, dwflags : u32, lparam : isize) -> windows_core::BOOL);
    unsafe { EnumUILanguagesA(lpuilanguageenumproc, dwflags, lparam).ok() }
}
#[inline]
pub unsafe fn EnumUILanguagesW(lpuilanguageenumproc: UILANGUAGE_ENUMPROCW, dwflags: u32, lparam: isize) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn EnumUILanguagesW(lpuilanguageenumproc : UILANGUAGE_ENUMPROCW, dwflags : u32, lparam : isize) -> windows_core::BOOL);
    unsafe { EnumUILanguagesW(lpuilanguageenumproc, dwflags, lparam).ok() }
}
#[inline]
pub unsafe fn FindNLSString(locale: u32, dwfindnlsstringflags: u32, lpstringsource: &[u16], lpstringvalue: &[u16], pcchfound: Option<*mut i32>) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn FindNLSString(locale : u32, dwfindnlsstringflags : u32, lpstringsource : windows_core::PCWSTR, cchsource : i32, lpstringvalue : windows_core::PCWSTR, cchvalue : i32, pcchfound : *mut i32) -> i32);
    unsafe { FindNLSString(locale, dwfindnlsstringflags, core::mem::transmute(lpstringsource.as_ptr()), lpstringsource.len().try_into().unwrap(), core::mem::transmute(lpstringvalue.as_ptr()), lpstringvalue.len().try_into().unwrap(), pcchfound.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn FindNLSStringEx<P0>(lplocalename: P0, dwfindnlsstringflags: u32, lpstringsource: &[u16], lpstringvalue: &[u16], pcchfound: Option<*mut i32>, lpversioninformation: Option<*const NLSVERSIONINFO>, lpreserved: Option<*const core::ffi::c_void>, sorthandle: super::Foundation::LPARAM) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn FindNLSStringEx(lplocalename : windows_core::PCWSTR, dwfindnlsstringflags : u32, lpstringsource : windows_core::PCWSTR, cchsource : i32, lpstringvalue : windows_core::PCWSTR, cchvalue : i32, pcchfound : *mut i32, lpversioninformation : *const NLSVERSIONINFO, lpreserved : *const core::ffi::c_void, sorthandle : super::Foundation:: LPARAM) -> i32);
    unsafe { FindNLSStringEx(lplocalename.param().abi(), dwfindnlsstringflags, core::mem::transmute(lpstringsource.as_ptr()), lpstringsource.len().try_into().unwrap(), core::mem::transmute(lpstringvalue.as_ptr()), lpstringvalue.len().try_into().unwrap(), pcchfound.unwrap_or(core::mem::zeroed()) as _, lpversioninformation.unwrap_or(core::mem::zeroed()) as _, lpreserved.unwrap_or(core::mem::zeroed()) as _, sorthandle) }
}
#[inline]
pub unsafe fn FindStringOrdinal(dwfindstringordinalflags: u32, lpstringsource: &[u16], lpstringvalue: &[u16], bignorecase: bool) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn FindStringOrdinal(dwfindstringordinalflags : u32, lpstringsource : windows_core::PCWSTR, cchsource : i32, lpstringvalue : windows_core::PCWSTR, cchvalue : i32, bignorecase : windows_core::BOOL) -> i32);
    unsafe { FindStringOrdinal(dwfindstringordinalflags, core::mem::transmute(lpstringsource.as_ptr()), lpstringsource.len().try_into().unwrap(), core::mem::transmute(lpstringvalue.as_ptr()), lpstringvalue.len().try_into().unwrap(), bignorecase.into()) }
}
#[inline]
pub unsafe fn FoldStringA(dwmapflags: FOLD_STRING_MAP_FLAGS, lpsrcstr: &[u8], lpdeststr: Option<&mut [u8]>) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn FoldStringA(dwmapflags : FOLD_STRING_MAP_FLAGS, lpsrcstr : windows_core::PCSTR, cchsrc : i32, lpdeststr : windows_core::PSTR, cchdest : i32) -> i32);
    unsafe { FoldStringA(dwmapflags, core::mem::transmute(lpsrcstr.as_ptr()), lpsrcstr.len().try_into().unwrap(), core::mem::transmute(lpdeststr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpdeststr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn FoldStringW(dwmapflags: FOLD_STRING_MAP_FLAGS, lpsrcstr: &[u16], lpdeststr: Option<&mut [u16]>) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn FoldStringW(dwmapflags : FOLD_STRING_MAP_FLAGS, lpsrcstr : windows_core::PCWSTR, cchsrc : i32, lpdeststr : windows_core::PWSTR, cchdest : i32) -> i32);
    unsafe { FoldStringW(dwmapflags, core::mem::transmute(lpsrcstr.as_ptr()), lpsrcstr.len().try_into().unwrap(), core::mem::transmute(lpdeststr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpdeststr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn GetACP() -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetACP() -> u32);
    unsafe { GetACP() }
}
#[inline]
pub unsafe fn GetCPInfo(codepage: u32, lpcpinfo: *mut CPINFO) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn GetCPInfo(codepage : u32, lpcpinfo : *mut CPINFO) -> windows_core::BOOL);
    unsafe { GetCPInfo(codepage, lpcpinfo as _).ok() }
}
#[inline]
pub unsafe fn GetCPInfoExA(codepage: u32, dwflags: u32, lpcpinfoex: *mut CPINFOEXA) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn GetCPInfoExA(codepage : u32, dwflags : u32, lpcpinfoex : *mut CPINFOEXA) -> windows_core::BOOL);
    unsafe { GetCPInfoExA(codepage, dwflags, lpcpinfoex as _).ok() }
}
#[inline]
pub unsafe fn GetCPInfoExW(codepage: u32, dwflags: u32, lpcpinfoex: *mut CPINFOEXW) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn GetCPInfoExW(codepage : u32, dwflags : u32, lpcpinfoex : *mut CPINFOEXW) -> windows_core::BOOL);
    unsafe { GetCPInfoExW(codepage, dwflags, lpcpinfoex as _).ok() }
}
#[inline]
pub unsafe fn GetCalendarDateFormatEx<P0, P3>(lpszlocale: P0, dwflags: u32, lpcaldatetime: *const CALDATETIME, lpformat: P3, lpdatestr: windows_core::PWSTR, cchdate: i32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetCalendarDateFormatEx(lpszlocale : windows_core::PCWSTR, dwflags : u32, lpcaldatetime : *const CALDATETIME, lpformat : windows_core::PCWSTR, lpdatestr : windows_core::PWSTR, cchdate : i32) -> windows_core::BOOL);
    unsafe { GetCalendarDateFormatEx(lpszlocale.param().abi(), dwflags, lpcaldatetime, lpformat.param().abi(), core::mem::transmute(lpdatestr), cchdate) }
}
#[inline]
pub unsafe fn GetCalendarInfoA(locale: u32, calendar: u32, caltype: u32, lpcaldata: Option<&mut [u8]>, lpvalue: Option<*mut u32>) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn GetCalendarInfoA(locale : u32, calendar : u32, caltype : u32, lpcaldata : windows_core::PSTR, cchdata : i32, lpvalue : *mut u32) -> i32);
    unsafe { GetCalendarInfoA(locale, calendar, caltype, core::mem::transmute(lpcaldata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpcaldata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), lpvalue.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetCalendarInfoEx<P0, P2>(lplocalename: P0, calendar: u32, lpreserved: P2, caltype: u32, lpcaldata: Option<&mut [u16]>, lpvalue: Option<*mut u32>) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetCalendarInfoEx(lplocalename : windows_core::PCWSTR, calendar : u32, lpreserved : windows_core::PCWSTR, caltype : u32, lpcaldata : windows_core::PWSTR, cchdata : i32, lpvalue : *mut u32) -> i32);
    unsafe { GetCalendarInfoEx(lplocalename.param().abi(), calendar, lpreserved.param().abi(), caltype, core::mem::transmute(lpcaldata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpcaldata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), lpvalue.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetCalendarInfoW(locale: u32, calendar: u32, caltype: u32, lpcaldata: Option<&mut [u16]>, lpvalue: Option<*mut u32>) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn GetCalendarInfoW(locale : u32, calendar : u32, caltype : u32, lpcaldata : windows_core::PWSTR, cchdata : i32, lpvalue : *mut u32) -> i32);
    unsafe { GetCalendarInfoW(locale, calendar, caltype, core::mem::transmute(lpcaldata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpcaldata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), lpvalue.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetCalendarSupportedDateRange(calendar: u32, lpcalmindatetime: *mut CALDATETIME, lpcalmaxdatetime: *mut CALDATETIME) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetCalendarSupportedDateRange(calendar : u32, lpcalmindatetime : *mut CALDATETIME, lpcalmaxdatetime : *mut CALDATETIME) -> windows_core::BOOL);
    unsafe { GetCalendarSupportedDateRange(calendar, lpcalmindatetime as _, lpcalmaxdatetime as _) }
}
#[inline]
pub unsafe fn GetCurrencyFormatA<P2>(locale: u32, dwflags: u32, lpvalue: P2, lpformat: Option<*const CURRENCYFMTA>, lpcurrencystr: Option<&mut [u8]>) -> i32
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetCurrencyFormatA(locale : u32, dwflags : u32, lpvalue : windows_core::PCSTR, lpformat : *const CURRENCYFMTA, lpcurrencystr : windows_core::PSTR, cchcurrency : i32) -> i32);
    unsafe { GetCurrencyFormatA(locale, dwflags, lpvalue.param().abi(), lpformat.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(lpcurrencystr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpcurrencystr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn GetCurrencyFormatEx<P0, P2>(lplocalename: P0, dwflags: u32, lpvalue: P2, lpformat: Option<*const CURRENCYFMTW>, lpcurrencystr: Option<&mut [u16]>) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetCurrencyFormatEx(lplocalename : windows_core::PCWSTR, dwflags : u32, lpvalue : windows_core::PCWSTR, lpformat : *const CURRENCYFMTW, lpcurrencystr : windows_core::PWSTR, cchcurrency : i32) -> i32);
    unsafe { GetCurrencyFormatEx(lplocalename.param().abi(), dwflags, lpvalue.param().abi(), lpformat.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(lpcurrencystr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpcurrencystr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn GetCurrencyFormatW<P2>(locale: u32, dwflags: u32, lpvalue: P2, lpformat: Option<*const CURRENCYFMTW>, lpcurrencystr: Option<&mut [u16]>) -> i32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetCurrencyFormatW(locale : u32, dwflags : u32, lpvalue : windows_core::PCWSTR, lpformat : *const CURRENCYFMTW, lpcurrencystr : windows_core::PWSTR, cchcurrency : i32) -> i32);
    unsafe { GetCurrencyFormatW(locale, dwflags, lpvalue.param().abi(), lpformat.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(lpcurrencystr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpcurrencystr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn GetDateFormatA<P3>(locale: u32, dwflags: u32, lpdate: Option<*const super::Foundation::SYSTEMTIME>, lpformat: P3, lpdatestr: Option<&mut [u8]>) -> i32
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetDateFormatA(locale : u32, dwflags : u32, lpdate : *const super::Foundation:: SYSTEMTIME, lpformat : windows_core::PCSTR, lpdatestr : windows_core::PSTR, cchdate : i32) -> i32);
    unsafe { GetDateFormatA(locale, dwflags, lpdate.unwrap_or(core::mem::zeroed()) as _, lpformat.param().abi(), core::mem::transmute(lpdatestr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpdatestr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn GetDateFormatEx<P0, P3, P6>(lplocalename: P0, dwflags: ENUM_DATE_FORMATS_FLAGS, lpdate: Option<*const super::Foundation::SYSTEMTIME>, lpformat: P3, lpdatestr: Option<&mut [u16]>, lpcalendar: P6) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P6: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetDateFormatEx(lplocalename : windows_core::PCWSTR, dwflags : ENUM_DATE_FORMATS_FLAGS, lpdate : *const super::Foundation:: SYSTEMTIME, lpformat : windows_core::PCWSTR, lpdatestr : windows_core::PWSTR, cchdate : i32, lpcalendar : windows_core::PCWSTR) -> i32);
    unsafe { GetDateFormatEx(lplocalename.param().abi(), dwflags, lpdate.unwrap_or(core::mem::zeroed()) as _, lpformat.param().abi(), core::mem::transmute(lpdatestr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpdatestr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), lpcalendar.param().abi()) }
}
#[inline]
pub unsafe fn GetDateFormatW<P3>(locale: u32, dwflags: u32, lpdate: Option<*const super::Foundation::SYSTEMTIME>, lpformat: P3, lpdatestr: Option<&mut [u16]>) -> i32
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetDateFormatW(locale : u32, dwflags : u32, lpdate : *const super::Foundation:: SYSTEMTIME, lpformat : windows_core::PCWSTR, lpdatestr : windows_core::PWSTR, cchdate : i32) -> i32);
    unsafe { GetDateFormatW(locale, dwflags, lpdate.unwrap_or(core::mem::zeroed()) as _, lpformat.param().abi(), core::mem::transmute(lpdatestr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpdatestr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn GetDistanceOfClosestLanguageInList<P0, P1>(pszlanguage: P0, pszlanguageslist: P1, wchlistdelimiter: u16) -> windows_core::Result<f64>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcp47mrm.dll" "system" fn GetDistanceOfClosestLanguageInList(pszlanguage : windows_core::PCWSTR, pszlanguageslist : windows_core::PCWSTR, wchlistdelimiter : u16, pclosestdistance : *mut f64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetDistanceOfClosestLanguageInList(pszlanguage.param().abi(), pszlanguageslist.param().abi(), wchlistdelimiter, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn GetDurationFormat<P4>(locale: u32, dwflags: u32, lpduration: Option<*const super::Foundation::SYSTEMTIME>, ullduration: u64, lpformat: P4, lpdurationstr: Option<&mut [u16]>) -> i32
where
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetDurationFormat(locale : u32, dwflags : u32, lpduration : *const super::Foundation:: SYSTEMTIME, ullduration : u64, lpformat : windows_core::PCWSTR, lpdurationstr : windows_core::PWSTR, cchduration : i32) -> i32);
    unsafe { GetDurationFormat(locale, dwflags, lpduration.unwrap_or(core::mem::zeroed()) as _, ullduration, lpformat.param().abi(), core::mem::transmute(lpdurationstr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpdurationstr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn GetDurationFormatEx<P0, P4>(lplocalename: P0, dwflags: u32, lpduration: Option<*const super::Foundation::SYSTEMTIME>, ullduration: u64, lpformat: P4, lpdurationstr: Option<&mut [u16]>) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetDurationFormatEx(lplocalename : windows_core::PCWSTR, dwflags : u32, lpduration : *const super::Foundation:: SYSTEMTIME, ullduration : u64, lpformat : windows_core::PCWSTR, lpdurationstr : windows_core::PWSTR, cchduration : i32) -> i32);
    unsafe { GetDurationFormatEx(lplocalename.param().abi(), dwflags, lpduration.unwrap_or(core::mem::zeroed()) as _, ullduration, lpformat.param().abi(), core::mem::transmute(lpdurationstr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpdurationstr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn GetFileMUIInfo<P1>(dwflags: u32, pcwszfilepath: P1, pfilemuiinfo: Option<*mut FILEMUIINFO>, pcbfilemuiinfo: *mut u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetFileMUIInfo(dwflags : u32, pcwszfilepath : windows_core::PCWSTR, pfilemuiinfo : *mut FILEMUIINFO, pcbfilemuiinfo : *mut u32) -> windows_core::BOOL);
    unsafe { GetFileMUIInfo(dwflags, pcwszfilepath.param().abi(), pfilemuiinfo.unwrap_or(core::mem::zeroed()) as _, pcbfilemuiinfo as _).ok() }
}
#[inline]
pub unsafe fn GetFileMUIPath<P1>(dwflags: u32, pcwszfilepath: P1, pwszlanguage: Option<windows_core::PWSTR>, pcchlanguage: *mut u32, pwszfilemuipath: Option<windows_core::PWSTR>, pcchfilemuipath: *mut u32, pululenumerator: *mut u64) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetFileMUIPath(dwflags : u32, pcwszfilepath : windows_core::PCWSTR, pwszlanguage : windows_core::PWSTR, pcchlanguage : *mut u32, pwszfilemuipath : windows_core::PWSTR, pcchfilemuipath : *mut u32, pululenumerator : *mut u64) -> windows_core::BOOL);
    unsafe { GetFileMUIPath(dwflags, pcwszfilepath.param().abi(), pwszlanguage.unwrap_or(core::mem::zeroed()) as _, pcchlanguage as _, pwszfilemuipath.unwrap_or(core::mem::zeroed()) as _, pcchfilemuipath as _, pululenumerator as _).ok() }
}
#[inline]
pub unsafe fn GetGeoInfoA(location: i32, geotype: SYSGEOTYPE, lpgeodata: Option<&mut [u8]>, langid: u16) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn GetGeoInfoA(location : i32, geotype : SYSGEOTYPE, lpgeodata : windows_core::PSTR, cchdata : i32, langid : u16) -> i32);
    unsafe { GetGeoInfoA(location, geotype, core::mem::transmute(lpgeodata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpgeodata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), langid) }
}
#[inline]
pub unsafe fn GetGeoInfoEx<P0>(location: P0, geotype: SYSGEOTYPE, geodata: Option<&mut [u16]>) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetGeoInfoEx(location : windows_core::PCWSTR, geotype : SYSGEOTYPE, geodata : windows_core::PWSTR, geodatacount : i32) -> i32);
    unsafe { GetGeoInfoEx(location.param().abi(), geotype, core::mem::transmute(geodata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), geodata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn GetGeoInfoW(location: i32, geotype: SYSGEOTYPE, lpgeodata: Option<&mut [u16]>, langid: u16) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn GetGeoInfoW(location : i32, geotype : SYSGEOTYPE, lpgeodata : windows_core::PWSTR, cchdata : i32, langid : u16) -> i32);
    unsafe { GetGeoInfoW(location, geotype, core::mem::transmute(lpgeodata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpgeodata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), langid) }
}
#[inline]
pub unsafe fn GetLocaleInfoA(locale: u32, lctype: u32, lplcdata: Option<&mut [u8]>) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn GetLocaleInfoA(locale : u32, lctype : u32, lplcdata : windows_core::PSTR, cchdata : i32) -> i32);
    unsafe { GetLocaleInfoA(locale, lctype, core::mem::transmute(lplcdata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lplcdata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn GetLocaleInfoEx<P0>(lplocalename: P0, lctype: u32, lplcdata: Option<&mut [u16]>) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetLocaleInfoEx(lplocalename : windows_core::PCWSTR, lctype : u32, lplcdata : windows_core::PWSTR, cchdata : i32) -> i32);
    unsafe { GetLocaleInfoEx(lplocalename.param().abi(), lctype, core::mem::transmute(lplcdata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lplcdata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn GetLocaleInfoW(locale: u32, lctype: u32, lplcdata: Option<&mut [u16]>) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn GetLocaleInfoW(locale : u32, lctype : u32, lplcdata : windows_core::PWSTR, cchdata : i32) -> i32);
    unsafe { GetLocaleInfoW(locale, lctype, core::mem::transmute(lplcdata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lplcdata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn GetNLSVersion(function: u32, locale: u32, lpversioninformation: *mut NLSVERSIONINFO) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn GetNLSVersion(function : u32, locale : u32, lpversioninformation : *mut NLSVERSIONINFO) -> windows_core::BOOL);
    unsafe { GetNLSVersion(function, locale, lpversioninformation as _).ok() }
}
#[inline]
pub unsafe fn GetNLSVersionEx<P1>(function: u32, lplocalename: P1, lpversioninformation: *mut NLSVERSIONINFOEX) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetNLSVersionEx(function : u32, lplocalename : windows_core::PCWSTR, lpversioninformation : *mut NLSVERSIONINFOEX) -> windows_core::BOOL);
    unsafe { GetNLSVersionEx(function, lplocalename.param().abi(), lpversioninformation as _).ok() }
}
#[inline]
pub unsafe fn GetNumberFormatA<P2>(locale: u32, dwflags: u32, lpvalue: P2, lpformat: Option<*const NUMBERFMTA>, lpnumberstr: Option<&mut [u8]>) -> i32
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetNumberFormatA(locale : u32, dwflags : u32, lpvalue : windows_core::PCSTR, lpformat : *const NUMBERFMTA, lpnumberstr : windows_core::PSTR, cchnumber : i32) -> i32);
    unsafe { GetNumberFormatA(locale, dwflags, lpvalue.param().abi(), lpformat.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(lpnumberstr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpnumberstr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn GetNumberFormatEx<P0, P2>(lplocalename: P0, dwflags: u32, lpvalue: P2, lpformat: Option<*const NUMBERFMTW>, lpnumberstr: Option<&mut [u16]>) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetNumberFormatEx(lplocalename : windows_core::PCWSTR, dwflags : u32, lpvalue : windows_core::PCWSTR, lpformat : *const NUMBERFMTW, lpnumberstr : windows_core::PWSTR, cchnumber : i32) -> i32);
    unsafe { GetNumberFormatEx(lplocalename.param().abi(), dwflags, lpvalue.param().abi(), lpformat.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(lpnumberstr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpnumberstr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn GetNumberFormatW<P2>(locale: u32, dwflags: u32, lpvalue: P2, lpformat: Option<*const NUMBERFMTW>, lpnumberstr: Option<&mut [u16]>) -> i32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetNumberFormatW(locale : u32, dwflags : u32, lpvalue : windows_core::PCWSTR, lpformat : *const NUMBERFMTW, lpnumberstr : windows_core::PWSTR, cchnumber : i32) -> i32);
    unsafe { GetNumberFormatW(locale, dwflags, lpvalue.param().abi(), lpformat.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(lpnumberstr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpnumberstr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn GetOEMCP() -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetOEMCP() -> u32);
    unsafe { GetOEMCP() }
}
#[inline]
pub unsafe fn GetProcessPreferredUILanguages(dwflags: u32, pulnumlanguages: *mut u32, pwszlanguagesbuffer: Option<windows_core::PWSTR>, pcchlanguagesbuffer: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn GetProcessPreferredUILanguages(dwflags : u32, pulnumlanguages : *mut u32, pwszlanguagesbuffer : windows_core::PWSTR, pcchlanguagesbuffer : *mut u32) -> windows_core::BOOL);
    unsafe { GetProcessPreferredUILanguages(dwflags, pulnumlanguages as _, pwszlanguagesbuffer.unwrap_or(core::mem::zeroed()) as _, pcchlanguagesbuffer as _).ok() }
}
#[inline]
pub unsafe fn GetStringScripts<P1>(dwflags: u32, lpstring: P1, cchstring: i32, lpscripts: Option<&mut [u16]>) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetStringScripts(dwflags : u32, lpstring : windows_core::PCWSTR, cchstring : i32, lpscripts : windows_core::PWSTR, cchscripts : i32) -> i32);
    unsafe { GetStringScripts(dwflags, lpstring.param().abi(), cchstring, core::mem::transmute(lpscripts.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpscripts.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn GetStringTypeA(locale: u32, dwinfotype: u32, lpsrcstr: &[u8], lpchartype: *mut u16) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn GetStringTypeA(locale : u32, dwinfotype : u32, lpsrcstr : windows_core::PCSTR, cchsrc : i32, lpchartype : *mut u16) -> windows_core::BOOL);
    unsafe { GetStringTypeA(locale, dwinfotype, core::mem::transmute(lpsrcstr.as_ptr()), lpsrcstr.len().try_into().unwrap(), lpchartype as _).ok() }
}
#[inline]
pub unsafe fn GetStringTypeExA<P2>(locale: u32, dwinfotype: u32, lpsrcstr: P2, cchsrc: i32, lpchartype: *mut u16) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetStringTypeExA(locale : u32, dwinfotype : u32, lpsrcstr : windows_core::PCSTR, cchsrc : i32, lpchartype : *mut u16) -> windows_core::BOOL);
    unsafe { GetStringTypeExA(locale, dwinfotype, lpsrcstr.param().abi(), cchsrc, lpchartype as _) }
}
#[inline]
pub unsafe fn GetStringTypeExW<P2>(locale: u32, dwinfotype: u32, lpsrcstr: P2, cchsrc: i32, lpchartype: *mut u16) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetStringTypeExW(locale : u32, dwinfotype : u32, lpsrcstr : windows_core::PCWSTR, cchsrc : i32, lpchartype : *mut u16) -> windows_core::BOOL);
    unsafe { GetStringTypeExW(locale, dwinfotype, lpsrcstr.param().abi(), cchsrc, lpchartype as _).ok() }
}
#[inline]
pub unsafe fn GetStringTypeW(dwinfotype: u32, lpsrcstr: &[u16], lpchartype: *mut u16) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn GetStringTypeW(dwinfotype : u32, lpsrcstr : windows_core::PCWSTR, cchsrc : i32, lpchartype : *mut u16) -> windows_core::BOOL);
    unsafe { GetStringTypeW(dwinfotype, core::mem::transmute(lpsrcstr.as_ptr()), lpsrcstr.len().try_into().unwrap(), lpchartype as _).ok() }
}
#[inline]
pub unsafe fn GetSystemDefaultLCID() -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetSystemDefaultLCID() -> u32);
    unsafe { GetSystemDefaultLCID() }
}
#[inline]
pub unsafe fn GetSystemDefaultLangID() -> u16 {
    windows_core::link!("kernel32.dll" "system" fn GetSystemDefaultLangID() -> u16);
    unsafe { GetSystemDefaultLangID() }
}
#[inline]
pub unsafe fn GetSystemDefaultLocaleName(lplocalename: &mut [u16]) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn GetSystemDefaultLocaleName(lplocalename : windows_core::PWSTR, cchlocalename : i32) -> i32);
    unsafe { GetSystemDefaultLocaleName(core::mem::transmute(lplocalename.as_ptr()), lplocalename.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn GetSystemDefaultUILanguage() -> u16 {
    windows_core::link!("kernel32.dll" "system" fn GetSystemDefaultUILanguage() -> u16);
    unsafe { GetSystemDefaultUILanguage() }
}
#[inline]
pub unsafe fn GetSystemPreferredUILanguages(dwflags: u32, pulnumlanguages: *mut u32, pwszlanguagesbuffer: Option<windows_core::PWSTR>, pcchlanguagesbuffer: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn GetSystemPreferredUILanguages(dwflags : u32, pulnumlanguages : *mut u32, pwszlanguagesbuffer : windows_core::PWSTR, pcchlanguagesbuffer : *mut u32) -> windows_core::BOOL);
    unsafe { GetSystemPreferredUILanguages(dwflags, pulnumlanguages as _, pwszlanguagesbuffer.unwrap_or(core::mem::zeroed()) as _, pcchlanguagesbuffer as _).ok() }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetTextCharset(hdc: super::Graphics::Gdi::HDC) -> i32 {
    windows_core::link!("gdi32.dll" "system" fn GetTextCharset(hdc : super::Graphics::Gdi:: HDC) -> i32);
    unsafe { GetTextCharset(hdc) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetTextCharsetInfo(hdc: super::Graphics::Gdi::HDC, lpsig: Option<*mut FONTSIGNATURE>, dwflags: u32) -> i32 {
    windows_core::link!("gdi32.dll" "system" fn GetTextCharsetInfo(hdc : super::Graphics::Gdi:: HDC, lpsig : *mut FONTSIGNATURE, dwflags : u32) -> i32);
    unsafe { GetTextCharsetInfo(hdc, lpsig.unwrap_or(core::mem::zeroed()) as _, dwflags) }
}
#[inline]
pub unsafe fn GetThreadLocale() -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetThreadLocale() -> u32);
    unsafe { GetThreadLocale() }
}
#[inline]
pub unsafe fn GetThreadPreferredUILanguages(dwflags: u32, pulnumlanguages: *mut u32, pwszlanguagesbuffer: Option<windows_core::PWSTR>, pcchlanguagesbuffer: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn GetThreadPreferredUILanguages(dwflags : u32, pulnumlanguages : *mut u32, pwszlanguagesbuffer : windows_core::PWSTR, pcchlanguagesbuffer : *mut u32) -> windows_core::BOOL);
    unsafe { GetThreadPreferredUILanguages(dwflags, pulnumlanguages as _, pwszlanguagesbuffer.unwrap_or(core::mem::zeroed()) as _, pcchlanguagesbuffer as _).ok() }
}
#[inline]
pub unsafe fn GetThreadUILanguage() -> u16 {
    windows_core::link!("kernel32.dll" "system" fn GetThreadUILanguage() -> u16);
    unsafe { GetThreadUILanguage() }
}
#[inline]
pub unsafe fn GetTimeFormatA<P3>(locale: u32, dwflags: u32, lptime: Option<*const super::Foundation::SYSTEMTIME>, lpformat: P3, lptimestr: Option<&mut [u8]>) -> i32
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetTimeFormatA(locale : u32, dwflags : u32, lptime : *const super::Foundation:: SYSTEMTIME, lpformat : windows_core::PCSTR, lptimestr : windows_core::PSTR, cchtime : i32) -> i32);
    unsafe { GetTimeFormatA(locale, dwflags, lptime.unwrap_or(core::mem::zeroed()) as _, lpformat.param().abi(), core::mem::transmute(lptimestr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lptimestr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn GetTimeFormatEx<P0, P3>(lplocalename: P0, dwflags: TIME_FORMAT_FLAGS, lptime: Option<*const super::Foundation::SYSTEMTIME>, lpformat: P3, lptimestr: Option<&mut [u16]>) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetTimeFormatEx(lplocalename : windows_core::PCWSTR, dwflags : TIME_FORMAT_FLAGS, lptime : *const super::Foundation:: SYSTEMTIME, lpformat : windows_core::PCWSTR, lptimestr : windows_core::PWSTR, cchtime : i32) -> i32);
    unsafe { GetTimeFormatEx(lplocalename.param().abi(), dwflags, lptime.unwrap_or(core::mem::zeroed()) as _, lpformat.param().abi(), core::mem::transmute(lptimestr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lptimestr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn GetTimeFormatW<P3>(locale: u32, dwflags: u32, lptime: Option<*const super::Foundation::SYSTEMTIME>, lpformat: P3, lptimestr: Option<&mut [u16]>) -> i32
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetTimeFormatW(locale : u32, dwflags : u32, lptime : *const super::Foundation:: SYSTEMTIME, lpformat : windows_core::PCWSTR, lptimestr : windows_core::PWSTR, cchtime : i32) -> i32);
    unsafe { GetTimeFormatW(locale, dwflags, lptime.unwrap_or(core::mem::zeroed()) as _, lpformat.param().abi(), core::mem::transmute(lptimestr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lptimestr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn GetUILanguageInfo<P1>(dwflags: u32, pwmszlanguage: P1, pwszfallbacklanguages: Option<windows_core::PWSTR>, pcchfallbacklanguages: Option<*mut u32>, pattributes: *mut u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetUILanguageInfo(dwflags : u32, pwmszlanguage : windows_core::PCWSTR, pwszfallbacklanguages : windows_core::PWSTR, pcchfallbacklanguages : *mut u32, pattributes : *mut u32) -> windows_core::BOOL);
    unsafe { GetUILanguageInfo(dwflags, pwmszlanguage.param().abi(), pwszfallbacklanguages.unwrap_or(core::mem::zeroed()) as _, pcchfallbacklanguages.unwrap_or(core::mem::zeroed()) as _, pattributes as _).ok() }
}
#[inline]
pub unsafe fn GetUserDefaultGeoName(geoname: &mut [u16]) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn GetUserDefaultGeoName(geoname : windows_core::PWSTR, geonamecount : i32) -> i32);
    unsafe { GetUserDefaultGeoName(core::mem::transmute(geoname.as_ptr()), geoname.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn GetUserDefaultLCID() -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetUserDefaultLCID() -> u32);
    unsafe { GetUserDefaultLCID() }
}
#[inline]
pub unsafe fn GetUserDefaultLangID() -> u16 {
    windows_core::link!("kernel32.dll" "system" fn GetUserDefaultLangID() -> u16);
    unsafe { GetUserDefaultLangID() }
}
#[inline]
pub unsafe fn GetUserDefaultLocaleName(lplocalename: &mut [u16]) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn GetUserDefaultLocaleName(lplocalename : windows_core::PWSTR, cchlocalename : i32) -> i32);
    unsafe { GetUserDefaultLocaleName(core::mem::transmute(lplocalename.as_ptr()), lplocalename.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn GetUserDefaultUILanguage() -> u16 {
    windows_core::link!("kernel32.dll" "system" fn GetUserDefaultUILanguage() -> u16);
    unsafe { GetUserDefaultUILanguage() }
}
#[inline]
pub unsafe fn GetUserGeoID(geoclass: SYSGEOCLASS) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn GetUserGeoID(geoclass : SYSGEOCLASS) -> i32);
    unsafe { GetUserGeoID(geoclass) }
}
#[inline]
pub unsafe fn GetUserPreferredUILanguages(dwflags: u32, pulnumlanguages: *mut u32, pwszlanguagesbuffer: Option<windows_core::PWSTR>, pcchlanguagesbuffer: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn GetUserPreferredUILanguages(dwflags : u32, pulnumlanguages : *mut u32, pwszlanguagesbuffer : windows_core::PWSTR, pcchlanguagesbuffer : *mut u32) -> windows_core::BOOL);
    unsafe { GetUserPreferredUILanguages(dwflags, pulnumlanguages as _, pwszlanguagesbuffer.unwrap_or(core::mem::zeroed()) as _, pcchlanguagesbuffer as _).ok() }
}
#[inline]
pub unsafe fn IdnToAscii(dwflags: u32, lpunicodecharstr: &[u16], lpasciicharstr: Option<&mut [u16]>) -> i32 {
    windows_core::link!("normaliz.dll" "system" fn IdnToAscii(dwflags : u32, lpunicodecharstr : windows_core::PCWSTR, cchunicodechar : i32, lpasciicharstr : windows_core::PWSTR, cchasciichar : i32) -> i32);
    unsafe { IdnToAscii(dwflags, core::mem::transmute(lpunicodecharstr.as_ptr()), lpunicodecharstr.len().try_into().unwrap(), core::mem::transmute(lpasciicharstr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpasciicharstr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn IdnToNameprepUnicode(dwflags: u32, lpunicodecharstr: &[u16], lpnameprepcharstr: Option<&mut [u16]>) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn IdnToNameprepUnicode(dwflags : u32, lpunicodecharstr : windows_core::PCWSTR, cchunicodechar : i32, lpnameprepcharstr : windows_core::PWSTR, cchnameprepchar : i32) -> i32);
    unsafe { IdnToNameprepUnicode(dwflags, core::mem::transmute(lpunicodecharstr.as_ptr()), lpunicodecharstr.len().try_into().unwrap(), core::mem::transmute(lpnameprepcharstr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpnameprepcharstr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn IdnToUnicode(dwflags: u32, lpasciicharstr: &[u16], lpunicodecharstr: Option<&mut [u16]>) -> i32 {
    windows_core::link!("normaliz.dll" "system" fn IdnToUnicode(dwflags : u32, lpasciicharstr : windows_core::PCWSTR, cchasciichar : i32, lpunicodecharstr : windows_core::PWSTR, cchunicodechar : i32) -> i32);
    unsafe { IdnToUnicode(dwflags, core::mem::transmute(lpasciicharstr.as_ptr()), lpasciicharstr.len().try_into().unwrap(), core::mem::transmute(lpunicodecharstr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpunicodecharstr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn IsCalendarLeapYear(calid: u32, year: u32, era: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn IsCalendarLeapYear(calid : u32, year : u32, era : u32) -> windows_core::BOOL);
    unsafe { IsCalendarLeapYear(calid, year, era) }
}
#[inline]
pub unsafe fn IsDBCSLeadByte(testchar: u8) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn IsDBCSLeadByte(testchar : u8) -> windows_core::BOOL);
    unsafe { IsDBCSLeadByte(testchar).ok() }
}
#[inline]
pub unsafe fn IsDBCSLeadByteEx(codepage: u32, testchar: u8) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn IsDBCSLeadByteEx(codepage : u32, testchar : u8) -> windows_core::BOOL);
    unsafe { IsDBCSLeadByteEx(codepage, testchar).ok() }
}
#[inline]
pub unsafe fn IsNLSDefinedString(function: u32, dwflags: u32, lpversioninformation: *const NLSVERSIONINFO, lpstring: &[u16]) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn IsNLSDefinedString(function : u32, dwflags : u32, lpversioninformation : *const NLSVERSIONINFO, lpstring : windows_core::PCWSTR, cchstr : i32) -> windows_core::BOOL);
    unsafe { IsNLSDefinedString(function, dwflags, lpversioninformation, core::mem::transmute(lpstring.as_ptr()), lpstring.len().try_into().unwrap()).ok() }
}
#[inline]
pub unsafe fn IsNormalizedString(normform: NORM_FORM, lpstring: &[u16]) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn IsNormalizedString(normform : NORM_FORM, lpstring : windows_core::PCWSTR, cwlength : i32) -> windows_core::BOOL);
    unsafe { IsNormalizedString(normform, core::mem::transmute(lpstring.as_ptr()), lpstring.len().try_into().unwrap()).ok() }
}
#[inline]
pub unsafe fn IsTextUnicode(lpv: *const core::ffi::c_void, isize: i32, lpiresult: Option<*mut IS_TEXT_UNICODE_RESULT>) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn IsTextUnicode(lpv : *const core::ffi::c_void, isize : i32, lpiresult : *mut IS_TEXT_UNICODE_RESULT) -> windows_core::BOOL);
    unsafe { IsTextUnicode(lpv, isize, lpiresult.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn IsValidCodePage(codepage: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn IsValidCodePage(codepage : u32) -> windows_core::BOOL);
    unsafe { IsValidCodePage(codepage) }
}
#[inline]
pub unsafe fn IsValidLanguageGroup(languagegroup: u32, dwflags: ENUM_SYSTEM_LANGUAGE_GROUPS_FLAGS) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn IsValidLanguageGroup(languagegroup : u32, dwflags : ENUM_SYSTEM_LANGUAGE_GROUPS_FLAGS) -> windows_core::BOOL);
    unsafe { IsValidLanguageGroup(languagegroup, dwflags) }
}
#[inline]
pub unsafe fn IsValidLocale(locale: u32, dwflags: IS_VALID_LOCALE_FLAGS) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn IsValidLocale(locale : u32, dwflags : IS_VALID_LOCALE_FLAGS) -> windows_core::BOOL);
    unsafe { IsValidLocale(locale, dwflags) }
}
#[inline]
pub unsafe fn IsValidLocaleName<P0>(lplocalename: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn IsValidLocaleName(lplocalename : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { IsValidLocaleName(lplocalename.param().abi()) }
}
#[inline]
pub unsafe fn IsValidNLSVersion<P1>(function: u32, lplocalename: P1, lpversioninformation: *const NLSVERSIONINFOEX) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn IsValidNLSVersion(function : u32, lplocalename : windows_core::PCWSTR, lpversioninformation : *const NLSVERSIONINFOEX) -> u32);
    unsafe { IsValidNLSVersion(function, lplocalename.param().abi(), lpversioninformation) }
}
#[inline]
pub unsafe fn IsWellFormedTag<P0>(psztag: P0) -> u8
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcp47mrm.dll" "system" fn IsWellFormedTag(psztag : windows_core::PCWSTR) -> u8);
    unsafe { IsWellFormedTag(psztag.param().abi()) }
}
#[inline]
pub unsafe fn LCIDToLocaleName(locale: u32, lpname: Option<&mut [u16]>, dwflags: u32) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn LCIDToLocaleName(locale : u32, lpname : windows_core::PWSTR, cchname : i32, dwflags : u32) -> i32);
    unsafe { LCIDToLocaleName(locale, core::mem::transmute(lpname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), dwflags) }
}
#[inline]
pub unsafe fn LCMapStringA(locale: u32, dwmapflags: u32, lpsrcstr: &[u8], lpdeststr: Option<windows_core::PSTR>, cchdest: i32) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn LCMapStringA(locale : u32, dwmapflags : u32, lpsrcstr : windows_core::PCSTR, cchsrc : i32, lpdeststr : windows_core::PSTR, cchdest : i32) -> i32);
    unsafe { LCMapStringA(locale, dwmapflags, core::mem::transmute(lpsrcstr.as_ptr()), lpsrcstr.len().try_into().unwrap(), lpdeststr.unwrap_or(core::mem::zeroed()) as _, cchdest) }
}
#[inline]
pub unsafe fn LCMapStringEx<P0>(lplocalename: P0, dwmapflags: u32, lpsrcstr: &[u16], lpdeststr: Option<&mut [u16]>, lpversioninformation: Option<*const NLSVERSIONINFO>, lpreserved: Option<*const core::ffi::c_void>, sorthandle: super::Foundation::LPARAM) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn LCMapStringEx(lplocalename : windows_core::PCWSTR, dwmapflags : u32, lpsrcstr : windows_core::PCWSTR, cchsrc : i32, lpdeststr : windows_core::PWSTR, cchdest : i32, lpversioninformation : *const NLSVERSIONINFO, lpreserved : *const core::ffi::c_void, sorthandle : super::Foundation:: LPARAM) -> i32);
    unsafe { LCMapStringEx(lplocalename.param().abi(), dwmapflags, core::mem::transmute(lpsrcstr.as_ptr()), lpsrcstr.len().try_into().unwrap(), core::mem::transmute(lpdeststr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpdeststr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), lpversioninformation.unwrap_or(core::mem::zeroed()) as _, lpreserved.unwrap_or(core::mem::zeroed()) as _, sorthandle) }
}
#[inline]
pub unsafe fn LCMapStringW(locale: u32, dwmapflags: u32, lpsrcstr: &[u16], lpdeststr: Option<windows_core::PWSTR>, cchdest: i32) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn LCMapStringW(locale : u32, dwmapflags : u32, lpsrcstr : windows_core::PCWSTR, cchsrc : i32, lpdeststr : windows_core::PWSTR, cchdest : i32) -> i32);
    unsafe { LCMapStringW(locale, dwmapflags, core::mem::transmute(lpsrcstr.as_ptr()), lpsrcstr.len().try_into().unwrap(), lpdeststr.unwrap_or(core::mem::zeroed()) as _, cchdest) }
}
#[inline]
pub unsafe fn LocaleNameToLCID<P0>(lpname: P0, dwflags: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn LocaleNameToLCID(lpname : windows_core::PCWSTR, dwflags : u32) -> u32);
    unsafe { LocaleNameToLCID(lpname.param().abi(), dwflags) }
}
#[inline]
pub unsafe fn MappingDoAction<P2>(pbag: *mut MAPPING_PROPERTY_BAG, dwrangeindex: u32, pszactionid: P2) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("elscore.dll" "system" fn MappingDoAction(pbag : *mut MAPPING_PROPERTY_BAG, dwrangeindex : u32, pszactionid : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { MappingDoAction(pbag as _, dwrangeindex, pszactionid.param().abi()).ok() }
}
#[inline]
pub unsafe fn MappingFreePropertyBag(pbag: *const MAPPING_PROPERTY_BAG) -> windows_core::Result<()> {
    windows_core::link!("elscore.dll" "system" fn MappingFreePropertyBag(pbag : *const MAPPING_PROPERTY_BAG) -> windows_core::HRESULT);
    unsafe { MappingFreePropertyBag(pbag).ok() }
}
#[inline]
pub unsafe fn MappingFreeServices(pserviceinfo: *const MAPPING_SERVICE_INFO) -> windows_core::Result<()> {
    windows_core::link!("elscore.dll" "system" fn MappingFreeServices(pserviceinfo : *const MAPPING_SERVICE_INFO) -> windows_core::HRESULT);
    unsafe { MappingFreeServices(pserviceinfo).ok() }
}
#[inline]
pub unsafe fn MappingGetServices(poptions: Option<*const MAPPING_ENUM_OPTIONS>, prgservices: *mut *mut MAPPING_SERVICE_INFO, pdwservicescount: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("elscore.dll" "system" fn MappingGetServices(poptions : *const MAPPING_ENUM_OPTIONS, prgservices : *mut *mut MAPPING_SERVICE_INFO, pdwservicescount : *mut u32) -> windows_core::HRESULT);
    unsafe { MappingGetServices(poptions.unwrap_or(core::mem::zeroed()) as _, prgservices as _, pdwservicescount as _).ok() }
}
#[inline]
pub unsafe fn MappingRecognizeText(pserviceinfo: *const MAPPING_SERVICE_INFO, psztext: &[u16], dwindex: u32, poptions: Option<*const MAPPING_OPTIONS>, pbag: *mut MAPPING_PROPERTY_BAG) -> windows_core::Result<()> {
    windows_core::link!("elscore.dll" "system" fn MappingRecognizeText(pserviceinfo : *const MAPPING_SERVICE_INFO, psztext : windows_core::PCWSTR, dwlength : u32, dwindex : u32, poptions : *const MAPPING_OPTIONS, pbag : *mut MAPPING_PROPERTY_BAG) -> windows_core::HRESULT);
    unsafe { MappingRecognizeText(pserviceinfo, core::mem::transmute(psztext.as_ptr()), psztext.len().try_into().unwrap(), dwindex, poptions.unwrap_or(core::mem::zeroed()) as _, pbag as _).ok() }
}
#[inline]
pub unsafe fn MultiByteToWideChar(codepage: u32, dwflags: MULTI_BYTE_TO_WIDE_CHAR_FLAGS, lpmultibytestr: &[u8], lpwidecharstr: Option<&mut [u16]>) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn MultiByteToWideChar(codepage : u32, dwflags : MULTI_BYTE_TO_WIDE_CHAR_FLAGS, lpmultibytestr : windows_core::PCSTR, cbmultibyte : i32, lpwidecharstr : windows_core::PWSTR, cchwidechar : i32) -> i32);
    unsafe { MultiByteToWideChar(codepage, dwflags, core::mem::transmute(lpmultibytestr.as_ptr()), lpmultibytestr.len().try_into().unwrap(), core::mem::transmute(lpwidecharstr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpwidecharstr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn NormalizeString(normform: NORM_FORM, lpsrcstring: &[u16], lpdststring: Option<&mut [u16]>) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn NormalizeString(normform : NORM_FORM, lpsrcstring : windows_core::PCWSTR, cwsrclength : i32, lpdststring : windows_core::PWSTR, cwdstlength : i32) -> i32);
    unsafe { NormalizeString(normform, core::mem::transmute(lpsrcstring.as_ptr()), lpsrcstring.len().try_into().unwrap(), core::mem::transmute(lpdststring.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpdststring.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn NotifyUILanguageChange<P1, P2>(dwflags: u32, pcwstrnewlanguage: P1, pcwstrpreviouslanguage: P2, dwreserved: Option<u32>, pdwstatusrtrn: Option<*mut u32>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn NotifyUILanguageChange(dwflags : u32, pcwstrnewlanguage : windows_core::PCWSTR, pcwstrpreviouslanguage : windows_core::PCWSTR, dwreserved : u32, pdwstatusrtrn : *mut u32) -> windows_core::BOOL);
    unsafe { NotifyUILanguageChange(dwflags, pcwstrnewlanguage.param().abi(), pcwstrpreviouslanguage.param().abi(), dwreserved.unwrap_or(core::mem::zeroed()) as _, pdwstatusrtrn.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ResolveLocaleName<P0>(lpnametoresolve: P0, lplocalename: Option<&mut [u16]>) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn ResolveLocaleName(lpnametoresolve : windows_core::PCWSTR, lplocalename : windows_core::PWSTR, cchlocalename : i32) -> i32);
    unsafe { ResolveLocaleName(lpnametoresolve.param().abi(), core::mem::transmute(lplocalename.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lplocalename.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn RestoreThreadPreferredUILanguages(snapshot: HSAVEDUILANGUAGES) {
    windows_core::link!("kernel32.dll" "system" fn RestoreThreadPreferredUILanguages(snapshot : HSAVEDUILANGUAGES));
    unsafe { RestoreThreadPreferredUILanguages(snapshot) }
}
#[inline]
pub unsafe fn ScriptApplyDigitSubstitution(psds: *const SCRIPT_DIGITSUBSTITUTE, psc: *mut SCRIPT_CONTROL, pss: *mut SCRIPT_STATE) -> windows_core::Result<()> {
    windows_core::link!("usp10.dll" "system" fn ScriptApplyDigitSubstitution(psds : *const SCRIPT_DIGITSUBSTITUTE, psc : *mut SCRIPT_CONTROL, pss : *mut SCRIPT_STATE) -> windows_core::HRESULT);
    unsafe { ScriptApplyDigitSubstitution(psds, psc as _, pss as _).ok() }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ScriptApplyLogicalWidth(pidx: *const i32, cchars: i32, cglyphs: i32, pwlogclust: *const u16, psva: *const SCRIPT_VISATTR, piadvance: *const i32, psa: *const SCRIPT_ANALYSIS, pabc: Option<*mut super::Graphics::Gdi::ABC>, pijustify: *mut i32) -> windows_core::Result<()> {
    windows_core::link!("usp10.dll" "system" fn ScriptApplyLogicalWidth(pidx : *const i32, cchars : i32, cglyphs : i32, pwlogclust : *const u16, psva : *const SCRIPT_VISATTR, piadvance : *const i32, psa : *const SCRIPT_ANALYSIS, pabc : *mut super::Graphics::Gdi:: ABC, pijustify : *mut i32) -> windows_core::HRESULT);
    unsafe { ScriptApplyLogicalWidth(pidx, cchars, cglyphs, pwlogclust, psva, piadvance, psa, pabc.unwrap_or(core::mem::zeroed()) as _, pijustify as _).ok() }
}
#[inline]
pub unsafe fn ScriptBreak<P0>(pwcchars: P0, cchars: i32, psa: *const SCRIPT_ANALYSIS, psla: *mut SCRIPT_LOGATTR) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("usp10.dll" "system" fn ScriptBreak(pwcchars : windows_core::PCWSTR, cchars : i32, psa : *const SCRIPT_ANALYSIS, psla : *mut SCRIPT_LOGATTR) -> windows_core::HRESULT);
    unsafe { ScriptBreak(pwcchars.param().abi(), cchars, psa, psla as _).ok() }
}
#[inline]
pub unsafe fn ScriptCPtoX(icp: i32, ftrailing: bool, cglyphs: i32, pwlogclust: &[u16], psva: *const SCRIPT_VISATTR, piadvance: *const i32, psa: *const SCRIPT_ANALYSIS, pix: *mut i32) -> windows_core::Result<()> {
    windows_core::link!("usp10.dll" "system" fn ScriptCPtoX(icp : i32, ftrailing : windows_core::BOOL, cchars : i32, cglyphs : i32, pwlogclust : *const u16, psva : *const SCRIPT_VISATTR, piadvance : *const i32, psa : *const SCRIPT_ANALYSIS, pix : *mut i32) -> windows_core::HRESULT);
    unsafe { ScriptCPtoX(icp, ftrailing.into(), pwlogclust.len().try_into().unwrap(), cglyphs, core::mem::transmute(pwlogclust.as_ptr()), psva, piadvance, psa, pix as _).ok() }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ScriptCacheGetHeight(hdc: super::Graphics::Gdi::HDC, psc: *mut *mut core::ffi::c_void, tmheight: *mut i32) -> windows_core::Result<()> {
    windows_core::link!("usp10.dll" "system" fn ScriptCacheGetHeight(hdc : super::Graphics::Gdi:: HDC, psc : *mut *mut core::ffi::c_void, tmheight : *mut i32) -> windows_core::HRESULT);
    unsafe { ScriptCacheGetHeight(hdc, psc as _, tmheight as _).ok() }
}
#[inline]
pub unsafe fn ScriptFreeCache(psc: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_core::link!("usp10.dll" "system" fn ScriptFreeCache(psc : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { ScriptFreeCache(psc as _).ok() }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ScriptGetCMap<P2>(hdc: super::Graphics::Gdi::HDC, psc: *mut *mut core::ffi::c_void, pwcinchars: P2, cchars: i32, dwflags: u32, pwoutglyphs: *mut u16) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("usp10.dll" "system" fn ScriptGetCMap(hdc : super::Graphics::Gdi:: HDC, psc : *mut *mut core::ffi::c_void, pwcinchars : windows_core::PCWSTR, cchars : i32, dwflags : u32, pwoutglyphs : *mut u16) -> windows_core::HRESULT);
    unsafe { ScriptGetCMap(hdc, psc as _, pwcinchars.param().abi(), cchars, dwflags, pwoutglyphs as _).ok() }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ScriptGetFontAlternateGlyphs(hdc: Option<super::Graphics::Gdi::HDC>, psc: *mut *mut core::ffi::c_void, psa: Option<*const SCRIPT_ANALYSIS>, tagscript: u32, taglangsys: u32, tagfeature: u32, wglyphid: u16, palternateglyphs: &mut [u16], pcalternates: *mut i32) -> windows_core::Result<()> {
    windows_core::link!("usp10.dll" "system" fn ScriptGetFontAlternateGlyphs(hdc : super::Graphics::Gdi:: HDC, psc : *mut *mut core::ffi::c_void, psa : *const SCRIPT_ANALYSIS, tagscript : u32, taglangsys : u32, tagfeature : u32, wglyphid : u16, cmaxalternates : i32, palternateglyphs : *mut u16, pcalternates : *mut i32) -> windows_core::HRESULT);
    unsafe { ScriptGetFontAlternateGlyphs(hdc.unwrap_or(core::mem::zeroed()) as _, psc as _, psa.unwrap_or(core::mem::zeroed()) as _, tagscript, taglangsys, tagfeature, wglyphid, palternateglyphs.len().try_into().unwrap(), core::mem::transmute(palternateglyphs.as_ptr()), pcalternates as _).ok() }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ScriptGetFontFeatureTags(hdc: Option<super::Graphics::Gdi::HDC>, psc: *mut *mut core::ffi::c_void, psa: Option<*const SCRIPT_ANALYSIS>, tagscript: u32, taglangsys: u32, pfeaturetags: &mut [u32], pctags: *mut i32) -> windows_core::Result<()> {
    windows_core::link!("usp10.dll" "system" fn ScriptGetFontFeatureTags(hdc : super::Graphics::Gdi:: HDC, psc : *mut *mut core::ffi::c_void, psa : *const SCRIPT_ANALYSIS, tagscript : u32, taglangsys : u32, cmaxtags : i32, pfeaturetags : *mut u32, pctags : *mut i32) -> windows_core::HRESULT);
    unsafe { ScriptGetFontFeatureTags(hdc.unwrap_or(core::mem::zeroed()) as _, psc as _, psa.unwrap_or(core::mem::zeroed()) as _, tagscript, taglangsys, pfeaturetags.len().try_into().unwrap(), core::mem::transmute(pfeaturetags.as_ptr()), pctags as _).ok() }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ScriptGetFontLanguageTags(hdc: Option<super::Graphics::Gdi::HDC>, psc: *mut *mut core::ffi::c_void, psa: Option<*const SCRIPT_ANALYSIS>, tagscript: u32, plangsystags: &mut [u32], pctags: *mut i32) -> windows_core::Result<()> {
    windows_core::link!("usp10.dll" "system" fn ScriptGetFontLanguageTags(hdc : super::Graphics::Gdi:: HDC, psc : *mut *mut core::ffi::c_void, psa : *const SCRIPT_ANALYSIS, tagscript : u32, cmaxtags : i32, plangsystags : *mut u32, pctags : *mut i32) -> windows_core::HRESULT);
    unsafe { ScriptGetFontLanguageTags(hdc.unwrap_or(core::mem::zeroed()) as _, psc as _, psa.unwrap_or(core::mem::zeroed()) as _, tagscript, plangsystags.len().try_into().unwrap(), core::mem::transmute(plangsystags.as_ptr()), pctags as _).ok() }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ScriptGetFontProperties(hdc: super::Graphics::Gdi::HDC, psc: *mut *mut core::ffi::c_void, sfp: *mut SCRIPT_FONTPROPERTIES) -> windows_core::Result<()> {
    windows_core::link!("usp10.dll" "system" fn ScriptGetFontProperties(hdc : super::Graphics::Gdi:: HDC, psc : *mut *mut core::ffi::c_void, sfp : *mut SCRIPT_FONTPROPERTIES) -> windows_core::HRESULT);
    unsafe { ScriptGetFontProperties(hdc, psc as _, sfp as _).ok() }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ScriptGetFontScriptTags(hdc: Option<super::Graphics::Gdi::HDC>, psc: *mut *mut core::ffi::c_void, psa: Option<*const SCRIPT_ANALYSIS>, pscripttags: &mut [u32], pctags: *mut i32) -> windows_core::Result<()> {
    windows_core::link!("usp10.dll" "system" fn ScriptGetFontScriptTags(hdc : super::Graphics::Gdi:: HDC, psc : *mut *mut core::ffi::c_void, psa : *const SCRIPT_ANALYSIS, cmaxtags : i32, pscripttags : *mut u32, pctags : *mut i32) -> windows_core::HRESULT);
    unsafe { ScriptGetFontScriptTags(hdc.unwrap_or(core::mem::zeroed()) as _, psc as _, psa.unwrap_or(core::mem::zeroed()) as _, pscripttags.len().try_into().unwrap(), core::mem::transmute(pscripttags.as_ptr()), pctags as _).ok() }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ScriptGetGlyphABCWidth(hdc: super::Graphics::Gdi::HDC, psc: *mut *mut core::ffi::c_void, wglyph: u16, pabc: *mut super::Graphics::Gdi::ABC) -> windows_core::Result<()> {
    windows_core::link!("usp10.dll" "system" fn ScriptGetGlyphABCWidth(hdc : super::Graphics::Gdi:: HDC, psc : *mut *mut core::ffi::c_void, wglyph : u16, pabc : *mut super::Graphics::Gdi:: ABC) -> windows_core::HRESULT);
    unsafe { ScriptGetGlyphABCWidth(hdc, psc as _, wglyph, pabc as _).ok() }
}
#[inline]
pub unsafe fn ScriptGetLogicalWidths(psa: *const SCRIPT_ANALYSIS, cchars: i32, cglyphs: i32, piglyphwidth: *const i32, pwlogclust: *const u16, psva: *const SCRIPT_VISATTR, pidx: *const i32) -> windows_core::Result<()> {
    windows_core::link!("usp10.dll" "system" fn ScriptGetLogicalWidths(psa : *const SCRIPT_ANALYSIS, cchars : i32, cglyphs : i32, piglyphwidth : *const i32, pwlogclust : *const u16, psva : *const SCRIPT_VISATTR, pidx : *const i32) -> windows_core::HRESULT);
    unsafe { ScriptGetLogicalWidths(psa, cchars, cglyphs, piglyphwidth, pwlogclust, psva, pidx).ok() }
}
#[inline]
pub unsafe fn ScriptGetProperties(ppsp: *mut *mut *mut SCRIPT_PROPERTIES, pinumscripts: *mut i32) -> windows_core::Result<()> {
    windows_core::link!("usp10.dll" "system" fn ScriptGetProperties(ppsp : *mut *mut *mut SCRIPT_PROPERTIES, pinumscripts : *mut i32) -> windows_core::HRESULT);
    unsafe { ScriptGetProperties(ppsp as _, pinumscripts as _).ok() }
}
#[inline]
pub unsafe fn ScriptIsComplex(pwcinchars: &[u16], dwflags: SCRIPT_IS_COMPLEX_FLAGS) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptIsComplex(pwcinchars : windows_core::PCWSTR, cinchars : i32, dwflags : SCRIPT_IS_COMPLEX_FLAGS) -> windows_core::HRESULT);
    unsafe { ScriptIsComplex(core::mem::transmute(pwcinchars.as_ptr()), pwcinchars.len().try_into().unwrap(), dwflags) }
}
#[inline]
pub unsafe fn ScriptItemize(pwcinchars: &[u16], pscontrol: Option<*const SCRIPT_CONTROL>, psstate: Option<*const SCRIPT_STATE>, pitems: &mut [SCRIPT_ITEM], pcitems: *mut i32) -> windows_core::Result<()> {
    windows_core::link!("usp10.dll" "system" fn ScriptItemize(pwcinchars : windows_core::PCWSTR, cinchars : i32, cmaxitems : i32, pscontrol : *const SCRIPT_CONTROL, psstate : *const SCRIPT_STATE, pitems : *mut SCRIPT_ITEM, pcitems : *mut i32) -> windows_core::HRESULT);
    unsafe { ScriptItemize(core::mem::transmute(pwcinchars.as_ptr()), pwcinchars.len().try_into().unwrap(), pitems.len().try_into().unwrap(), pscontrol.unwrap_or(core::mem::zeroed()) as _, psstate.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pitems.as_ptr()), pcitems as _).ok() }
}
#[inline]
pub unsafe fn ScriptItemizeOpenType(pwcinchars: &[u16], cmaxitems: i32, pscontrol: Option<*const SCRIPT_CONTROL>, psstate: Option<*const SCRIPT_STATE>, pitems: *mut SCRIPT_ITEM, pscripttags: *mut u32, pcitems: *mut i32) -> windows_core::Result<()> {
    windows_core::link!("usp10.dll" "system" fn ScriptItemizeOpenType(pwcinchars : windows_core::PCWSTR, cinchars : i32, cmaxitems : i32, pscontrol : *const SCRIPT_CONTROL, psstate : *const SCRIPT_STATE, pitems : *mut SCRIPT_ITEM, pscripttags : *mut u32, pcitems : *mut i32) -> windows_core::HRESULT);
    unsafe { ScriptItemizeOpenType(core::mem::transmute(pwcinchars.as_ptr()), pwcinchars.len().try_into().unwrap(), cmaxitems, pscontrol.unwrap_or(core::mem::zeroed()) as _, psstate.unwrap_or(core::mem::zeroed()) as _, pitems as _, pscripttags as _, pcitems as _).ok() }
}
#[inline]
pub unsafe fn ScriptJustify(psva: *const SCRIPT_VISATTR, piadvance: *const i32, cglyphs: i32, idx: i32, iminkashida: i32, pijustify: *mut i32) -> windows_core::Result<()> {
    windows_core::link!("usp10.dll" "system" fn ScriptJustify(psva : *const SCRIPT_VISATTR, piadvance : *const i32, cglyphs : i32, idx : i32, iminkashida : i32, pijustify : *mut i32) -> windows_core::HRESULT);
    unsafe { ScriptJustify(psva, piadvance, cglyphs, idx, iminkashida, pijustify as _).ok() }
}
#[inline]
pub unsafe fn ScriptLayout(cruns: i32, pblevel: *const u8, pivisualtological: Option<*mut i32>, pilogicaltovisual: Option<*mut i32>) -> windows_core::Result<()> {
    windows_core::link!("usp10.dll" "system" fn ScriptLayout(cruns : i32, pblevel : *const u8, pivisualtological : *mut i32, pilogicaltovisual : *mut i32) -> windows_core::HRESULT);
    unsafe { ScriptLayout(cruns, pblevel, pivisualtological.unwrap_or(core::mem::zeroed()) as _, pilogicaltovisual.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ScriptPlace(hdc: super::Graphics::Gdi::HDC, psc: *mut *mut core::ffi::c_void, pwglyphs: *const u16, cglyphs: i32, psva: *const SCRIPT_VISATTR, psa: *mut SCRIPT_ANALYSIS, piadvance: *mut i32, pgoffset: Option<*mut GOFFSET>, pabc: *mut super::Graphics::Gdi::ABC) -> windows_core::Result<()> {
    windows_core::link!("usp10.dll" "system" fn ScriptPlace(hdc : super::Graphics::Gdi:: HDC, psc : *mut *mut core::ffi::c_void, pwglyphs : *const u16, cglyphs : i32, psva : *const SCRIPT_VISATTR, psa : *mut SCRIPT_ANALYSIS, piadvance : *mut i32, pgoffset : *mut GOFFSET, pabc : *mut super::Graphics::Gdi:: ABC) -> windows_core::HRESULT);
    unsafe { ScriptPlace(hdc, psc as _, pwglyphs, cglyphs, psva, psa as _, piadvance as _, pgoffset.unwrap_or(core::mem::zeroed()) as _, pabc as _).ok() }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ScriptPlaceOpenType<P8>(hdc: Option<super::Graphics::Gdi::HDC>, psc: *mut *mut core::ffi::c_void, psa: *mut SCRIPT_ANALYSIS, tagscript: u32, taglangsys: u32, rcrangechars: Option<*const i32>, rprangeproperties: Option<*const *const TEXTRANGE_PROPERTIES>, cranges: i32, pwcchars: P8, pwlogclust: *const u16, pcharprops: *const SCRIPT_CHARPROP, cchars: i32, pwglyphs: *const u16, pglyphprops: *const SCRIPT_GLYPHPROP, cglyphs: i32, piadvance: *mut i32, pgoffset: *mut GOFFSET, pabc: Option<*mut super::Graphics::Gdi::ABC>) -> windows_core::Result<()>
where
    P8: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("usp10.dll" "system" fn ScriptPlaceOpenType(hdc : super::Graphics::Gdi:: HDC, psc : *mut *mut core::ffi::c_void, psa : *mut SCRIPT_ANALYSIS, tagscript : u32, taglangsys : u32, rcrangechars : *const i32, rprangeproperties : *const *const TEXTRANGE_PROPERTIES, cranges : i32, pwcchars : windows_core::PCWSTR, pwlogclust : *const u16, pcharprops : *const SCRIPT_CHARPROP, cchars : i32, pwglyphs : *const u16, pglyphprops : *const SCRIPT_GLYPHPROP, cglyphs : i32, piadvance : *mut i32, pgoffset : *mut GOFFSET, pabc : *mut super::Graphics::Gdi:: ABC) -> windows_core::HRESULT);
    unsafe { ScriptPlaceOpenType(hdc.unwrap_or(core::mem::zeroed()) as _, psc as _, psa as _, tagscript, taglangsys, rcrangechars.unwrap_or(core::mem::zeroed()) as _, rprangeproperties.unwrap_or(core::mem::zeroed()) as _, cranges, pwcchars.param().abi(), pwlogclust, pcharprops, cchars, pwglyphs, pglyphprops, cglyphs, piadvance as _, pgoffset as _, pabc.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ScriptPositionSingleGlyph(hdc: Option<super::Graphics::Gdi::HDC>, psc: *mut *mut core::ffi::c_void, psa: Option<*const SCRIPT_ANALYSIS>, tagscript: u32, taglangsys: u32, tagfeature: u32, lparameter: i32, wglyphid: u16, iadvance: i32, goffset: GOFFSET, pioutadvance: *mut i32, poutgoffset: *mut GOFFSET) -> windows_core::Result<()> {
    windows_core::link!("usp10.dll" "system" fn ScriptPositionSingleGlyph(hdc : super::Graphics::Gdi:: HDC, psc : *mut *mut core::ffi::c_void, psa : *const SCRIPT_ANALYSIS, tagscript : u32, taglangsys : u32, tagfeature : u32, lparameter : i32, wglyphid : u16, iadvance : i32, goffset : GOFFSET, pioutadvance : *mut i32, poutgoffset : *mut GOFFSET) -> windows_core::HRESULT);
    unsafe { ScriptPositionSingleGlyph(hdc.unwrap_or(core::mem::zeroed()) as _, psc as _, psa.unwrap_or(core::mem::zeroed()) as _, tagscript, taglangsys, tagfeature, lparameter, wglyphid, iadvance, core::mem::transmute(goffset), pioutadvance as _, poutgoffset as _).ok() }
}
#[inline]
pub unsafe fn ScriptRecordDigitSubstitution(locale: u32) -> windows_core::Result<SCRIPT_DIGITSUBSTITUTE> {
    windows_core::link!("usp10.dll" "system" fn ScriptRecordDigitSubstitution(locale : u32, psds : *mut SCRIPT_DIGITSUBSTITUTE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        ScriptRecordDigitSubstitution(locale, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ScriptShape<P2>(hdc: super::Graphics::Gdi::HDC, psc: *mut *mut core::ffi::c_void, pwcchars: P2, cchars: i32, cmaxglyphs: i32, psa: *mut SCRIPT_ANALYSIS, pwoutglyphs: *mut u16, pwlogclust: *mut u16, psva: *mut SCRIPT_VISATTR, pcglyphs: *mut i32) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("usp10.dll" "system" fn ScriptShape(hdc : super::Graphics::Gdi:: HDC, psc : *mut *mut core::ffi::c_void, pwcchars : windows_core::PCWSTR, cchars : i32, cmaxglyphs : i32, psa : *mut SCRIPT_ANALYSIS, pwoutglyphs : *mut u16, pwlogclust : *mut u16, psva : *mut SCRIPT_VISATTR, pcglyphs : *mut i32) -> windows_core::HRESULT);
    unsafe { ScriptShape(hdc, psc as _, pwcchars.param().abi(), cchars, cmaxglyphs, psa as _, pwoutglyphs as _, pwlogclust as _, psva as _, pcglyphs as _).ok() }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ScriptShapeOpenType<P8>(hdc: Option<super::Graphics::Gdi::HDC>, psc: *mut *mut core::ffi::c_void, psa: *mut SCRIPT_ANALYSIS, tagscript: u32, taglangsys: u32, rcrangechars: Option<*const i32>, rprangeproperties: Option<*const *const TEXTRANGE_PROPERTIES>, cranges: i32, pwcchars: P8, cchars: i32, cmaxglyphs: i32, pwlogclust: *mut u16, pcharprops: *mut SCRIPT_CHARPROP, pwoutglyphs: *mut u16, poutglyphprops: *mut SCRIPT_GLYPHPROP, pcglyphs: *mut i32) -> windows_core::Result<()>
where
    P8: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("usp10.dll" "system" fn ScriptShapeOpenType(hdc : super::Graphics::Gdi:: HDC, psc : *mut *mut core::ffi::c_void, psa : *mut SCRIPT_ANALYSIS, tagscript : u32, taglangsys : u32, rcrangechars : *const i32, rprangeproperties : *const *const TEXTRANGE_PROPERTIES, cranges : i32, pwcchars : windows_core::PCWSTR, cchars : i32, cmaxglyphs : i32, pwlogclust : *mut u16, pcharprops : *mut SCRIPT_CHARPROP, pwoutglyphs : *mut u16, poutglyphprops : *mut SCRIPT_GLYPHPROP, pcglyphs : *mut i32) -> windows_core::HRESULT);
    unsafe { ScriptShapeOpenType(hdc.unwrap_or(core::mem::zeroed()) as _, psc as _, psa as _, tagscript, taglangsys, rcrangechars.unwrap_or(core::mem::zeroed()) as _, rprangeproperties.unwrap_or(core::mem::zeroed()) as _, cranges, pwcchars.param().abi(), cchars, cmaxglyphs, pwlogclust as _, pcharprops as _, pwoutglyphs as _, poutglyphprops as _, pcglyphs as _).ok() }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ScriptStringAnalyse(hdc: super::Graphics::Gdi::HDC, pstring: *const core::ffi::c_void, cstring: i32, cglyphs: i32, icharset: i32, dwflags: u32, ireqwidth: i32, pscontrol: Option<*const SCRIPT_CONTROL>, psstate: Option<*const SCRIPT_STATE>, pidx: Option<*const i32>, ptabdef: Option<*const SCRIPT_TABDEF>, pbinclass: *const u8, pssa: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_core::link!("usp10.dll" "system" fn ScriptStringAnalyse(hdc : super::Graphics::Gdi:: HDC, pstring : *const core::ffi::c_void, cstring : i32, cglyphs : i32, icharset : i32, dwflags : u32, ireqwidth : i32, pscontrol : *const SCRIPT_CONTROL, psstate : *const SCRIPT_STATE, pidx : *const i32, ptabdef : *const SCRIPT_TABDEF, pbinclass : *const u8, pssa : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { ScriptStringAnalyse(hdc, pstring, cstring, cglyphs, icharset, dwflags, ireqwidth, pscontrol.unwrap_or(core::mem::zeroed()) as _, psstate.unwrap_or(core::mem::zeroed()) as _, pidx.unwrap_or(core::mem::zeroed()) as _, ptabdef.unwrap_or(core::mem::zeroed()) as _, pbinclass, pssa as _).ok() }
}
#[inline]
pub unsafe fn ScriptStringCPtoX(ssa: *const core::ffi::c_void, icp: i32, ftrailing: bool) -> windows_core::Result<i32> {
    windows_core::link!("usp10.dll" "system" fn ScriptStringCPtoX(ssa : *const core::ffi::c_void, icp : i32, ftrailing : windows_core::BOOL, px : *mut i32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        ScriptStringCPtoX(ssa, icp, ftrailing.into(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn ScriptStringFree(pssa: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_core::link!("usp10.dll" "system" fn ScriptStringFree(pssa : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { ScriptStringFree(pssa as _).ok() }
}
#[inline]
pub unsafe fn ScriptStringGetLogicalWidths(ssa: *const core::ffi::c_void, pidx: *mut i32) -> windows_core::Result<()> {
    windows_core::link!("usp10.dll" "system" fn ScriptStringGetLogicalWidths(ssa : *const core::ffi::c_void, pidx : *mut i32) -> windows_core::HRESULT);
    unsafe { ScriptStringGetLogicalWidths(ssa, pidx as _).ok() }
}
#[inline]
pub unsafe fn ScriptStringGetOrder(ssa: *const core::ffi::c_void, puorder: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("usp10.dll" "system" fn ScriptStringGetOrder(ssa : *const core::ffi::c_void, puorder : *mut u32) -> windows_core::HRESULT);
    unsafe { ScriptStringGetOrder(ssa, puorder as _).ok() }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ScriptStringOut(ssa: *const core::ffi::c_void, ix: i32, iy: i32, uoptions: super::Graphics::Gdi::ETO_OPTIONS, prc: Option<*const super::Foundation::RECT>, iminsel: i32, imaxsel: i32, fdisabled: bool) -> windows_core::Result<()> {
    windows_core::link!("usp10.dll" "system" fn ScriptStringOut(ssa : *const core::ffi::c_void, ix : i32, iy : i32, uoptions : super::Graphics::Gdi:: ETO_OPTIONS, prc : *const super::Foundation:: RECT, iminsel : i32, imaxsel : i32, fdisabled : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { ScriptStringOut(ssa, ix, iy, uoptions, prc.unwrap_or(core::mem::zeroed()) as _, iminsel, imaxsel, fdisabled.into()).ok() }
}
#[inline]
pub unsafe fn ScriptStringValidate(ssa: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_core::link!("usp10.dll" "system" fn ScriptStringValidate(ssa : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { ScriptStringValidate(ssa).ok() }
}
#[inline]
pub unsafe fn ScriptStringXtoCP(ssa: *const core::ffi::c_void, ix: i32, pich: *mut i32, pitrailing: *mut i32) -> windows_core::Result<()> {
    windows_core::link!("usp10.dll" "system" fn ScriptStringXtoCP(ssa : *const core::ffi::c_void, ix : i32, pich : *mut i32, pitrailing : *mut i32) -> windows_core::HRESULT);
    unsafe { ScriptStringXtoCP(ssa, ix, pich as _, pitrailing as _).ok() }
}
#[inline]
pub unsafe fn ScriptString_pLogAttr(ssa: *const core::ffi::c_void) -> *mut SCRIPT_LOGATTR {
    windows_core::link!("usp10.dll" "system" fn ScriptString_pLogAttr(ssa : *const core::ffi::c_void) -> *mut SCRIPT_LOGATTR);
    unsafe { ScriptString_pLogAttr(ssa) }
}
#[inline]
pub unsafe fn ScriptString_pSize(ssa: *const core::ffi::c_void) -> *mut super::Foundation::SIZE {
    windows_core::link!("usp10.dll" "system" fn ScriptString_pSize(ssa : *const core::ffi::c_void) -> *mut super::Foundation:: SIZE);
    unsafe { ScriptString_pSize(ssa) }
}
#[inline]
pub unsafe fn ScriptString_pcOutChars(ssa: *const core::ffi::c_void) -> *mut i32 {
    windows_core::link!("usp10.dll" "system" fn ScriptString_pcOutChars(ssa : *const core::ffi::c_void) -> *mut i32);
    unsafe { ScriptString_pcOutChars(ssa) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ScriptSubstituteSingleGlyph(hdc: Option<super::Graphics::Gdi::HDC>, psc: *mut *mut core::ffi::c_void, psa: Option<*const SCRIPT_ANALYSIS>, tagscript: u32, taglangsys: u32, tagfeature: u32, lparameter: i32, wglyphid: u16, pwoutglyphid: *mut u16) -> windows_core::Result<()> {
    windows_core::link!("usp10.dll" "system" fn ScriptSubstituteSingleGlyph(hdc : super::Graphics::Gdi:: HDC, psc : *mut *mut core::ffi::c_void, psa : *const SCRIPT_ANALYSIS, tagscript : u32, taglangsys : u32, tagfeature : u32, lparameter : i32, wglyphid : u16, pwoutglyphid : *mut u16) -> windows_core::HRESULT);
    unsafe { ScriptSubstituteSingleGlyph(hdc.unwrap_or(core::mem::zeroed()) as _, psc as _, psa.unwrap_or(core::mem::zeroed()) as _, tagscript, taglangsys, tagfeature, lparameter, wglyphid, pwoutglyphid as _).ok() }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ScriptTextOut<P7>(hdc: super::Graphics::Gdi::HDC, psc: *mut *mut core::ffi::c_void, x: i32, y: i32, fuoptions: u32, lprc: Option<*const super::Foundation::RECT>, psa: *const SCRIPT_ANALYSIS, pwcreserved: P7, ireserved: Option<i32>, pwglyphs: *const u16, cglyphs: i32, piadvance: *const i32, pijustify: Option<*const i32>, pgoffset: *const GOFFSET) -> windows_core::Result<()>
where
    P7: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("usp10.dll" "system" fn ScriptTextOut(hdc : super::Graphics::Gdi:: HDC, psc : *mut *mut core::ffi::c_void, x : i32, y : i32, fuoptions : u32, lprc : *const super::Foundation:: RECT, psa : *const SCRIPT_ANALYSIS, pwcreserved : windows_core::PCWSTR, ireserved : i32, pwglyphs : *const u16, cglyphs : i32, piadvance : *const i32, pijustify : *const i32, pgoffset : *const GOFFSET) -> windows_core::HRESULT);
    unsafe { ScriptTextOut(hdc, psc as _, x, y, fuoptions, lprc.unwrap_or(core::mem::zeroed()) as _, psa, pwcreserved.param().abi(), ireserved.unwrap_or(core::mem::zeroed()) as _, pwglyphs, cglyphs, piadvance, pijustify.unwrap_or(core::mem::zeroed()) as _, pgoffset).ok() }
}
#[inline]
pub unsafe fn ScriptXtoCP(ix: i32, cglyphs: i32, pwlogclust: &[u16], psva: *const SCRIPT_VISATTR, piadvance: *const i32, psa: *const SCRIPT_ANALYSIS, picp: *mut i32, pitrailing: *mut i32) -> windows_core::Result<()> {
    windows_core::link!("usp10.dll" "system" fn ScriptXtoCP(ix : i32, cchars : i32, cglyphs : i32, pwlogclust : *const u16, psva : *const SCRIPT_VISATTR, piadvance : *const i32, psa : *const SCRIPT_ANALYSIS, picp : *mut i32, pitrailing : *mut i32) -> windows_core::HRESULT);
    unsafe { ScriptXtoCP(ix, pwlogclust.len().try_into().unwrap(), cglyphs, core::mem::transmute(pwlogclust.as_ptr()), psva, piadvance, psa, picp as _, pitrailing as _).ok() }
}
#[inline]
pub unsafe fn SetCalendarInfoA<P3>(locale: u32, calendar: u32, caltype: u32, lpcaldata: P3) -> windows_core::Result<()>
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetCalendarInfoA(locale : u32, calendar : u32, caltype : u32, lpcaldata : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { SetCalendarInfoA(locale, calendar, caltype, lpcaldata.param().abi()).ok() }
}
#[inline]
pub unsafe fn SetCalendarInfoW<P3>(locale: u32, calendar: u32, caltype: u32, lpcaldata: P3) -> windows_core::Result<()>
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetCalendarInfoW(locale : u32, calendar : u32, caltype : u32, lpcaldata : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SetCalendarInfoW(locale, calendar, caltype, lpcaldata.param().abi()).ok() }
}
#[inline]
pub unsafe fn SetLocaleInfoA<P2>(locale: u32, lctype: u32, lplcdata: P2) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetLocaleInfoA(locale : u32, lctype : u32, lplcdata : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { SetLocaleInfoA(locale, lctype, lplcdata.param().abi()).ok() }
}
#[inline]
pub unsafe fn SetLocaleInfoW<P2>(locale: u32, lctype: u32, lplcdata: P2) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetLocaleInfoW(locale : u32, lctype : u32, lplcdata : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SetLocaleInfoW(locale, lctype, lplcdata.param().abi()).ok() }
}
#[inline]
pub unsafe fn SetProcessPreferredUILanguages<P1>(dwflags: u32, pwszlanguagesbuffer: P1, pulnumlanguages: Option<*mut u32>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetProcessPreferredUILanguages(dwflags : u32, pwszlanguagesbuffer : windows_core::PCWSTR, pulnumlanguages : *mut u32) -> windows_core::BOOL);
    unsafe { SetProcessPreferredUILanguages(dwflags, pwszlanguagesbuffer.param().abi(), pulnumlanguages.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetThreadLocale(locale: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetThreadLocale(locale : u32) -> windows_core::BOOL);
    unsafe { SetThreadLocale(locale) }
}
#[inline]
pub unsafe fn SetThreadPreferredUILanguages<P1>(dwflags: u32, pwszlanguagesbuffer: P1, pulnumlanguages: Option<*mut u32>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetThreadPreferredUILanguages(dwflags : u32, pwszlanguagesbuffer : windows_core::PCWSTR, pulnumlanguages : *mut u32) -> windows_core::BOOL);
    unsafe { SetThreadPreferredUILanguages(dwflags, pwszlanguagesbuffer.param().abi(), pulnumlanguages.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetThreadPreferredUILanguages2<P1>(flags: u32, languages: P1, numlanguagesset: Option<*mut u32>, snapshot: Option<*mut HSAVEDUILANGUAGES>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetThreadPreferredUILanguages2(flags : u32, languages : windows_core::PCWSTR, numlanguagesset : *mut u32, snapshot : *mut HSAVEDUILANGUAGES) -> windows_core::BOOL);
    unsafe { SetThreadPreferredUILanguages2(flags, languages.param().abi(), numlanguagesset.unwrap_or(core::mem::zeroed()) as _, snapshot.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetThreadUILanguage(langid: u16) -> u16 {
    windows_core::link!("kernel32.dll" "system" fn SetThreadUILanguage(langid : u16) -> u16);
    unsafe { SetThreadUILanguage(langid) }
}
#[inline]
pub unsafe fn SetUserGeoID(geoid: i32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn SetUserGeoID(geoid : i32) -> windows_core::BOOL);
    unsafe { SetUserGeoID(geoid).ok() }
}
#[inline]
pub unsafe fn SetUserGeoName<P0>(geoname: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetUserGeoName(geoname : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SetUserGeoName(geoname.param().abi()).ok() }
}
#[inline]
pub unsafe fn TranslateCharsetInfo(lpsrc: *mut u32, lpcs: *mut CHARSETINFO, dwflags: TRANSLATE_CHARSET_INFO_FLAGS) -> windows_core::Result<()> {
    windows_core::link!("gdi32.dll" "system" fn TranslateCharsetInfo(lpsrc : *mut u32, lpcs : *mut CHARSETINFO, dwflags : TRANSLATE_CHARSET_INFO_FLAGS) -> windows_core::BOOL);
    unsafe { TranslateCharsetInfo(lpsrc as _, lpcs as _, dwflags).ok() }
}
#[inline]
pub unsafe fn UCNV_FROM_U_CALLBACK_ESCAPE(context: *const core::ffi::c_void, fromuargs: *mut UConverterFromUnicodeArgs, codeunits: *const u16, length: i32, codepoint: i32, reason: UConverterCallbackReason, err: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn UCNV_FROM_U_CALLBACK_ESCAPE(context : *const core::ffi::c_void, fromuargs : *mut UConverterFromUnicodeArgs, codeunits : *const u16, length : i32, codepoint : i32, reason : UConverterCallbackReason, err : *mut UErrorCode));
    unsafe { UCNV_FROM_U_CALLBACK_ESCAPE(context, fromuargs as _, codeunits, length, codepoint, reason, err as _) }
}
#[inline]
pub unsafe fn UCNV_FROM_U_CALLBACK_SKIP(context: *const core::ffi::c_void, fromuargs: *mut UConverterFromUnicodeArgs, codeunits: *const u16, length: i32, codepoint: i32, reason: UConverterCallbackReason, err: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn UCNV_FROM_U_CALLBACK_SKIP(context : *const core::ffi::c_void, fromuargs : *mut UConverterFromUnicodeArgs, codeunits : *const u16, length : i32, codepoint : i32, reason : UConverterCallbackReason, err : *mut UErrorCode));
    unsafe { UCNV_FROM_U_CALLBACK_SKIP(context, fromuargs as _, codeunits, length, codepoint, reason, err as _) }
}
#[inline]
pub unsafe fn UCNV_FROM_U_CALLBACK_STOP(context: *const core::ffi::c_void, fromuargs: *mut UConverterFromUnicodeArgs, codeunits: *const u16, length: i32, codepoint: i32, reason: UConverterCallbackReason, err: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn UCNV_FROM_U_CALLBACK_STOP(context : *const core::ffi::c_void, fromuargs : *mut UConverterFromUnicodeArgs, codeunits : *const u16, length : i32, codepoint : i32, reason : UConverterCallbackReason, err : *mut UErrorCode));
    unsafe { UCNV_FROM_U_CALLBACK_STOP(context, fromuargs as _, codeunits, length, codepoint, reason, err as _) }
}
#[inline]
pub unsafe fn UCNV_FROM_U_CALLBACK_SUBSTITUTE(context: *const core::ffi::c_void, fromuargs: *mut UConverterFromUnicodeArgs, codeunits: *const u16, length: i32, codepoint: i32, reason: UConverterCallbackReason, err: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn UCNV_FROM_U_CALLBACK_SUBSTITUTE(context : *const core::ffi::c_void, fromuargs : *mut UConverterFromUnicodeArgs, codeunits : *const u16, length : i32, codepoint : i32, reason : UConverterCallbackReason, err : *mut UErrorCode));
    unsafe { UCNV_FROM_U_CALLBACK_SUBSTITUTE(context, fromuargs as _, codeunits, length, codepoint, reason, err as _) }
}
#[inline]
pub unsafe fn UCNV_TO_U_CALLBACK_ESCAPE<P2>(context: *const core::ffi::c_void, touargs: *mut UConverterToUnicodeArgs, codeunits: P2, length: i32, reason: UConverterCallbackReason, err: *mut UErrorCode)
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn UCNV_TO_U_CALLBACK_ESCAPE(context : *const core::ffi::c_void, touargs : *mut UConverterToUnicodeArgs, codeunits : windows_core::PCSTR, length : i32, reason : UConverterCallbackReason, err : *mut UErrorCode));
    unsafe { UCNV_TO_U_CALLBACK_ESCAPE(context, touargs as _, codeunits.param().abi(), length, reason, err as _) }
}
#[inline]
pub unsafe fn UCNV_TO_U_CALLBACK_SKIP<P2>(context: *const core::ffi::c_void, touargs: *mut UConverterToUnicodeArgs, codeunits: P2, length: i32, reason: UConverterCallbackReason, err: *mut UErrorCode)
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn UCNV_TO_U_CALLBACK_SKIP(context : *const core::ffi::c_void, touargs : *mut UConverterToUnicodeArgs, codeunits : windows_core::PCSTR, length : i32, reason : UConverterCallbackReason, err : *mut UErrorCode));
    unsafe { UCNV_TO_U_CALLBACK_SKIP(context, touargs as _, codeunits.param().abi(), length, reason, err as _) }
}
#[inline]
pub unsafe fn UCNV_TO_U_CALLBACK_STOP<P2>(context: *const core::ffi::c_void, touargs: *mut UConverterToUnicodeArgs, codeunits: P2, length: i32, reason: UConverterCallbackReason, err: *mut UErrorCode)
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn UCNV_TO_U_CALLBACK_STOP(context : *const core::ffi::c_void, touargs : *mut UConverterToUnicodeArgs, codeunits : windows_core::PCSTR, length : i32, reason : UConverterCallbackReason, err : *mut UErrorCode));
    unsafe { UCNV_TO_U_CALLBACK_STOP(context, touargs as _, codeunits.param().abi(), length, reason, err as _) }
}
#[inline]
pub unsafe fn UCNV_TO_U_CALLBACK_SUBSTITUTE<P2>(context: *const core::ffi::c_void, touargs: *mut UConverterToUnicodeArgs, codeunits: P2, length: i32, reason: UConverterCallbackReason, err: *mut UErrorCode)
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn UCNV_TO_U_CALLBACK_SUBSTITUTE(context : *const core::ffi::c_void, touargs : *mut UConverterToUnicodeArgs, codeunits : windows_core::PCSTR, length : i32, reason : UConverterCallbackReason, err : *mut UErrorCode));
    unsafe { UCNV_TO_U_CALLBACK_SUBSTITUTE(context, touargs as _, codeunits.param().abi(), length, reason, err as _) }
}
#[inline]
pub unsafe fn UpdateCalendarDayOfWeek(lpcaldatetime: *mut CALDATETIME) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn UpdateCalendarDayOfWeek(lpcaldatetime : *mut CALDATETIME) -> windows_core::BOOL);
    unsafe { UpdateCalendarDayOfWeek(lpcaldatetime as _) }
}
#[inline]
pub unsafe fn VerifyScripts<P1, P3>(dwflags: u32, lplocalescripts: P1, cchlocalescripts: i32, lptestscripts: P3, cchtestscripts: i32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn VerifyScripts(dwflags : u32, lplocalescripts : windows_core::PCWSTR, cchlocalescripts : i32, lptestscripts : windows_core::PCWSTR, cchtestscripts : i32) -> windows_core::BOOL);
    unsafe { VerifyScripts(dwflags, lplocalescripts.param().abi(), cchlocalescripts, lptestscripts.param().abi(), cchtestscripts).ok() }
}
#[inline]
pub unsafe fn WideCharToMultiByte<P6>(codepage: u32, dwflags: u32, lpwidecharstr: &[u16], lpmultibytestr: Option<&mut [u8]>, lpdefaultchar: P6, lpuseddefaultchar: Option<*mut windows_core::BOOL>) -> i32
where
    P6: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn WideCharToMultiByte(codepage : u32, dwflags : u32, lpwidecharstr : windows_core::PCWSTR, cchwidechar : i32, lpmultibytestr : windows_core::PSTR, cbmultibyte : i32, lpdefaultchar : windows_core::PCSTR, lpuseddefaultchar : *mut windows_core::BOOL) -> i32);
    unsafe { WideCharToMultiByte(codepage, dwflags, core::mem::transmute(lpwidecharstr.as_ptr()), lpwidecharstr.len().try_into().unwrap(), core::mem::transmute(lpmultibytestr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpmultibytestr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), lpdefaultchar.param().abi(), lpuseddefaultchar.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn lstrcatA<P1>(lpstring1: windows_core::PSTR, lpstring2: P1) -> windows_core::PSTR
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn lstrcatA(lpstring1 : windows_core::PSTR, lpstring2 : windows_core::PCSTR) -> windows_core::PSTR);
    unsafe { lstrcatA(core::mem::transmute(lpstring1), lpstring2.param().abi()) }
}
#[inline]
pub unsafe fn lstrcatW<P1>(lpstring1: windows_core::PWSTR, lpstring2: P1) -> windows_core::PWSTR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn lstrcatW(lpstring1 : windows_core::PWSTR, lpstring2 : windows_core::PCWSTR) -> windows_core::PWSTR);
    unsafe { lstrcatW(core::mem::transmute(lpstring1), lpstring2.param().abi()) }
}
#[inline]
pub unsafe fn lstrcmpA<P0, P1>(lpstring1: P0, lpstring2: P1) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn lstrcmpA(lpstring1 : windows_core::PCSTR, lpstring2 : windows_core::PCSTR) -> i32);
    unsafe { lstrcmpA(lpstring1.param().abi(), lpstring2.param().abi()) }
}
#[inline]
pub unsafe fn lstrcmpW<P0, P1>(lpstring1: P0, lpstring2: P1) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn lstrcmpW(lpstring1 : windows_core::PCWSTR, lpstring2 : windows_core::PCWSTR) -> i32);
    unsafe { lstrcmpW(lpstring1.param().abi(), lpstring2.param().abi()) }
}
#[inline]
pub unsafe fn lstrcmpiA<P0, P1>(lpstring1: P0, lpstring2: P1) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn lstrcmpiA(lpstring1 : windows_core::PCSTR, lpstring2 : windows_core::PCSTR) -> i32);
    unsafe { lstrcmpiA(lpstring1.param().abi(), lpstring2.param().abi()) }
}
#[inline]
pub unsafe fn lstrcmpiW<P0, P1>(lpstring1: P0, lpstring2: P1) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn lstrcmpiW(lpstring1 : windows_core::PCWSTR, lpstring2 : windows_core::PCWSTR) -> i32);
    unsafe { lstrcmpiW(lpstring1.param().abi(), lpstring2.param().abi()) }
}
#[inline]
pub unsafe fn lstrcpyA<P1>(lpstring1: windows_core::PSTR, lpstring2: P1) -> windows_core::PSTR
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn lstrcpyA(lpstring1 : windows_core::PSTR, lpstring2 : windows_core::PCSTR) -> windows_core::PSTR);
    unsafe { lstrcpyA(core::mem::transmute(lpstring1), lpstring2.param().abi()) }
}
#[inline]
pub unsafe fn lstrcpyW<P1>(lpstring1: windows_core::PWSTR, lpstring2: P1) -> windows_core::PWSTR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn lstrcpyW(lpstring1 : windows_core::PWSTR, lpstring2 : windows_core::PCWSTR) -> windows_core::PWSTR);
    unsafe { lstrcpyW(core::mem::transmute(lpstring1), lpstring2.param().abi()) }
}
#[inline]
pub unsafe fn lstrcpynA<P1>(lpstring1: &mut [u8], lpstring2: P1) -> windows_core::PSTR
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn lstrcpynA(lpstring1 : windows_core::PSTR, lpstring2 : windows_core::PCSTR, imaxlength : i32) -> windows_core::PSTR);
    unsafe { lstrcpynA(core::mem::transmute(lpstring1.as_ptr()), lpstring2.param().abi(), lpstring1.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn lstrcpynW<P1>(lpstring1: &mut [u16], lpstring2: P1) -> windows_core::PWSTR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn lstrcpynW(lpstring1 : windows_core::PWSTR, lpstring2 : windows_core::PCWSTR, imaxlength : i32) -> windows_core::PWSTR);
    unsafe { lstrcpynW(core::mem::transmute(lpstring1.as_ptr()), lpstring2.param().abi(), lpstring1.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn lstrlenA<P0>(lpstring: P0) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn lstrlenA(lpstring : windows_core::PCSTR) -> i32);
    unsafe { lstrlenA(lpstring.param().abi()) }
}
#[inline]
pub unsafe fn lstrlenW<P0>(lpstring: P0) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn lstrlenW(lpstring : windows_core::PCWSTR) -> i32);
    unsafe { lstrlenW(lpstring.param().abi()) }
}
#[inline]
pub unsafe fn u_UCharsToChars<P1>(us: *const u16, cs: P1, length: i32)
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn u_UCharsToChars(us : *const u16, cs : windows_core::PCSTR, length : i32));
    unsafe { u_UCharsToChars(us, cs.param().abi(), length) }
}
#[inline]
pub unsafe fn u_austrcpy<P0>(dst: P0, src: *const u16) -> windows_core::PSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn u_austrcpy(dst : windows_core::PCSTR, src : *const u16) -> windows_core::PSTR);
    unsafe { u_austrcpy(dst.param().abi(), src) }
}
#[inline]
pub unsafe fn u_austrncpy<P0>(dst: P0, src: *const u16, n: i32) -> windows_core::PSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn u_austrncpy(dst : windows_core::PCSTR, src : *const u16, n : i32) -> windows_core::PSTR);
    unsafe { u_austrncpy(dst.param().abi(), src, n) }
}
#[inline]
pub unsafe fn u_catclose(catd: *mut UResourceBundle) {
    windows_core::link!("icuuc.dll" "C" fn u_catclose(catd : *mut UResourceBundle));
    unsafe { u_catclose(catd as _) }
}
#[inline]
pub unsafe fn u_catgets(catd: *mut UResourceBundle, set_num: i32, msg_num: i32, s: *const u16, len: *mut i32, ec: *mut UErrorCode) -> *mut u16 {
    windows_core::link!("icuuc.dll" "C" fn u_catgets(catd : *mut UResourceBundle, set_num : i32, msg_num : i32, s : *const u16, len : *mut i32, ec : *mut UErrorCode) -> *mut u16);
    unsafe { u_catgets(catd as _, set_num, msg_num, s, len as _, ec as _) }
}
#[inline]
pub unsafe fn u_catopen<P0, P1>(name: P0, locale: P1, ec: *mut UErrorCode) -> *mut UResourceBundle
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn u_catopen(name : windows_core::PCSTR, locale : windows_core::PCSTR, ec : *mut UErrorCode) -> *mut UResourceBundle);
    unsafe { u_catopen(name.param().abi(), locale.param().abi(), ec as _) }
}
#[inline]
pub unsafe fn u_charAge(c: i32, versionarray: *mut u8) {
    windows_core::link!("icuuc.dll" "C" fn u_charAge(c : i32, versionarray : *mut u8));
    unsafe { u_charAge(c, versionarray as _) }
}
#[inline]
pub unsafe fn u_charDigitValue(c: i32) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_charDigitValue(c : i32) -> i32);
    unsafe { u_charDigitValue(c) }
}
#[inline]
pub unsafe fn u_charDirection(c: i32) -> UCharDirection {
    windows_core::link!("icuuc.dll" "C" fn u_charDirection(c : i32) -> UCharDirection);
    unsafe { u_charDirection(c) }
}
#[inline]
pub unsafe fn u_charFromName<P1>(namechoice: UCharNameChoice, name: P1, perrorcode: *mut UErrorCode) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn u_charFromName(namechoice : UCharNameChoice, name : windows_core::PCSTR, perrorcode : *mut UErrorCode) -> i32);
    unsafe { u_charFromName(namechoice, name.param().abi(), perrorcode as _) }
}
#[inline]
pub unsafe fn u_charMirror(c: i32) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_charMirror(c : i32) -> i32);
    unsafe { u_charMirror(c) }
}
#[inline]
pub unsafe fn u_charName<P2>(code: i32, namechoice: UCharNameChoice, buffer: P2, bufferlength: i32, perrorcode: *mut UErrorCode) -> i32
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn u_charName(code : i32, namechoice : UCharNameChoice, buffer : windows_core::PCSTR, bufferlength : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { u_charName(code, namechoice, buffer.param().abi(), bufferlength, perrorcode as _) }
}
#[inline]
pub unsafe fn u_charType(c: i32) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn u_charType(c : i32) -> i8);
    unsafe { u_charType(c) }
}
#[inline]
pub unsafe fn u_charsToUChars<P0>(cs: P0, us: *mut u16, length: i32)
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn u_charsToUChars(cs : windows_core::PCSTR, us : *mut u16, length : i32));
    unsafe { u_charsToUChars(cs.param().abi(), us as _, length) }
}
#[inline]
pub unsafe fn u_cleanup() {
    windows_core::link!("icuuc.dll" "C" fn u_cleanup());
    unsafe { u_cleanup() }
}
#[inline]
pub unsafe fn u_countChar32(s: *const u16, length: i32) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_countChar32(s : *const u16, length : i32) -> i32);
    unsafe { u_countChar32(s, length) }
}
#[inline]
pub unsafe fn u_digit(ch: i32, radix: i8) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_digit(ch : i32, radix : i8) -> i32);
    unsafe { u_digit(ch, radix) }
}
#[inline]
pub unsafe fn u_enumCharNames(start: i32, limit: i32, r#fn: *mut UEnumCharNamesFn, context: *mut core::ffi::c_void, namechoice: UCharNameChoice, perrorcode: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn u_enumCharNames(start : i32, limit : i32, r#fn : *mut UEnumCharNamesFn, context : *mut core::ffi::c_void, namechoice : UCharNameChoice, perrorcode : *mut UErrorCode));
    unsafe { u_enumCharNames(start, limit, r#fn as _, context as _, namechoice, perrorcode as _) }
}
#[inline]
pub unsafe fn u_enumCharTypes(enumrange: *mut UCharEnumTypeRange, context: *const core::ffi::c_void) {
    windows_core::link!("icuuc.dll" "C" fn u_enumCharTypes(enumrange : *mut UCharEnumTypeRange, context : *const core::ffi::c_void));
    unsafe { u_enumCharTypes(enumrange as _, context) }
}
#[inline]
pub unsafe fn u_errorName(code: UErrorCode) -> windows_core::PCSTR {
    windows_core::link!("icuuc.dll" "C" fn u_errorName(code : UErrorCode) -> windows_core::PCSTR);
    unsafe { u_errorName(code) }
}
#[inline]
pub unsafe fn u_foldCase(c: i32, options: u32) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_foldCase(c : i32, options : u32) -> i32);
    unsafe { u_foldCase(c, options) }
}
#[inline]
pub unsafe fn u_forDigit(digit: i32, radix: i8) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_forDigit(digit : i32, radix : i8) -> i32);
    unsafe { u_forDigit(digit, radix) }
}
#[inline]
pub unsafe fn u_formatMessage<P0>(locale: P0, pattern: *const u16, patternlength: i32, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuin.dll" "C" fn u_formatMessage(locale : windows_core::PCSTR, pattern : *const u16, patternlength : i32, result : *mut u16, resultlength : i32, status : *mut UErrorCode) -> i32);
    unsafe { u_formatMessage(locale.param().abi(), pattern, patternlength, result as _, resultlength, status as _) }
}
#[inline]
pub unsafe fn u_formatMessageWithError<P0>(locale: P0, pattern: *const u16, patternlength: i32, result: *mut u16, resultlength: i32, parseerror: *mut UParseError, status: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuin.dll" "C" fn u_formatMessageWithError(locale : windows_core::PCSTR, pattern : *const u16, patternlength : i32, result : *mut u16, resultlength : i32, parseerror : *mut UParseError, status : *mut UErrorCode) -> i32);
    unsafe { u_formatMessageWithError(locale.param().abi(), pattern, patternlength, result as _, resultlength, parseerror as _, status as _) }
}
#[inline]
pub unsafe fn u_getBidiPairedBracket(c: i32) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_getBidiPairedBracket(c : i32) -> i32);
    unsafe { u_getBidiPairedBracket(c) }
}
#[inline]
pub unsafe fn u_getBinaryPropertySet(property: UProperty, perrorcode: *mut UErrorCode) -> *mut USet {
    windows_core::link!("icu.dll" "C" fn u_getBinaryPropertySet(property : UProperty, perrorcode : *mut UErrorCode) -> *mut USet);
    unsafe { u_getBinaryPropertySet(property, perrorcode as _) }
}
#[inline]
pub unsafe fn u_getCombiningClass(c: i32) -> u8 {
    windows_core::link!("icuuc.dll" "C" fn u_getCombiningClass(c : i32) -> u8);
    unsafe { u_getCombiningClass(c) }
}
#[inline]
pub unsafe fn u_getDataVersion(dataversionfillin: *mut u8, status: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn u_getDataVersion(dataversionfillin : *mut u8, status : *mut UErrorCode));
    unsafe { u_getDataVersion(dataversionfillin as _, status as _) }
}
#[inline]
pub unsafe fn u_getFC_NFKC_Closure(c: i32, dest: *mut u16, destcapacity: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_getFC_NFKC_Closure(c : i32, dest : *mut u16, destcapacity : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { u_getFC_NFKC_Closure(c, dest as _, destcapacity, perrorcode as _) }
}
#[inline]
pub unsafe fn u_getIntPropertyMap(property: UProperty, perrorcode: *mut UErrorCode) -> *mut UCPMap {
    windows_core::link!("icu.dll" "C" fn u_getIntPropertyMap(property : UProperty, perrorcode : *mut UErrorCode) -> *mut UCPMap);
    unsafe { u_getIntPropertyMap(property, perrorcode as _) }
}
#[inline]
pub unsafe fn u_getIntPropertyMaxValue(which: UProperty) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_getIntPropertyMaxValue(which : UProperty) -> i32);
    unsafe { u_getIntPropertyMaxValue(which) }
}
#[inline]
pub unsafe fn u_getIntPropertyMinValue(which: UProperty) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_getIntPropertyMinValue(which : UProperty) -> i32);
    unsafe { u_getIntPropertyMinValue(which) }
}
#[inline]
pub unsafe fn u_getIntPropertyValue(c: i32, which: UProperty) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_getIntPropertyValue(c : i32, which : UProperty) -> i32);
    unsafe { u_getIntPropertyValue(c, which) }
}
#[inline]
pub unsafe fn u_getNumericValue(c: i32) -> f64 {
    windows_core::link!("icuuc.dll" "C" fn u_getNumericValue(c : i32) -> f64);
    unsafe { u_getNumericValue(c) }
}
#[inline]
pub unsafe fn u_getPropertyEnum<P0>(alias: P0) -> UProperty
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn u_getPropertyEnum(alias : windows_core::PCSTR) -> UProperty);
    unsafe { u_getPropertyEnum(alias.param().abi()) }
}
#[inline]
pub unsafe fn u_getPropertyName(property: UProperty, namechoice: UPropertyNameChoice) -> windows_core::PCSTR {
    windows_core::link!("icuuc.dll" "C" fn u_getPropertyName(property : UProperty, namechoice : UPropertyNameChoice) -> windows_core::PCSTR);
    unsafe { u_getPropertyName(property, namechoice) }
}
#[inline]
pub unsafe fn u_getPropertyValueEnum<P1>(property: UProperty, alias: P1) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn u_getPropertyValueEnum(property : UProperty, alias : windows_core::PCSTR) -> i32);
    unsafe { u_getPropertyValueEnum(property, alias.param().abi()) }
}
#[inline]
pub unsafe fn u_getPropertyValueName(property: UProperty, value: i32, namechoice: UPropertyNameChoice) -> windows_core::PCSTR {
    windows_core::link!("icuuc.dll" "C" fn u_getPropertyValueName(property : UProperty, value : i32, namechoice : UPropertyNameChoice) -> windows_core::PCSTR);
    unsafe { u_getPropertyValueName(property, value, namechoice) }
}
#[inline]
pub unsafe fn u_getUnicodeVersion(versionarray: *mut u8) {
    windows_core::link!("icuuc.dll" "C" fn u_getUnicodeVersion(versionarray : *mut u8));
    unsafe { u_getUnicodeVersion(versionarray as _) }
}
#[inline]
pub unsafe fn u_getVersion(versionarray: *mut u8) {
    windows_core::link!("icuuc.dll" "C" fn u_getVersion(versionarray : *mut u8));
    unsafe { u_getVersion(versionarray as _) }
}
#[inline]
pub unsafe fn u_hasBinaryProperty(c: i32, which: UProperty) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn u_hasBinaryProperty(c : i32, which : UProperty) -> i8);
    unsafe { u_hasBinaryProperty(c, which) }
}
#[inline]
pub unsafe fn u_init(status: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn u_init(status : *mut UErrorCode));
    unsafe { u_init(status as _) }
}
#[inline]
pub unsafe fn u_isIDIgnorable(c: i32) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn u_isIDIgnorable(c : i32) -> i8);
    unsafe { u_isIDIgnorable(c) }
}
#[inline]
pub unsafe fn u_isIDPart(c: i32) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn u_isIDPart(c : i32) -> i8);
    unsafe { u_isIDPart(c) }
}
#[inline]
pub unsafe fn u_isIDStart(c: i32) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn u_isIDStart(c : i32) -> i8);
    unsafe { u_isIDStart(c) }
}
#[inline]
pub unsafe fn u_isISOControl(c: i32) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn u_isISOControl(c : i32) -> i8);
    unsafe { u_isISOControl(c) }
}
#[inline]
pub unsafe fn u_isJavaIDPart(c: i32) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn u_isJavaIDPart(c : i32) -> i8);
    unsafe { u_isJavaIDPart(c) }
}
#[inline]
pub unsafe fn u_isJavaIDStart(c: i32) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn u_isJavaIDStart(c : i32) -> i8);
    unsafe { u_isJavaIDStart(c) }
}
#[inline]
pub unsafe fn u_isJavaSpaceChar(c: i32) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn u_isJavaSpaceChar(c : i32) -> i8);
    unsafe { u_isJavaSpaceChar(c) }
}
#[inline]
pub unsafe fn u_isMirrored(c: i32) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn u_isMirrored(c : i32) -> i8);
    unsafe { u_isMirrored(c) }
}
#[inline]
pub unsafe fn u_isUAlphabetic(c: i32) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn u_isUAlphabetic(c : i32) -> i8);
    unsafe { u_isUAlphabetic(c) }
}
#[inline]
pub unsafe fn u_isULowercase(c: i32) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn u_isULowercase(c : i32) -> i8);
    unsafe { u_isULowercase(c) }
}
#[inline]
pub unsafe fn u_isUUppercase(c: i32) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn u_isUUppercase(c : i32) -> i8);
    unsafe { u_isUUppercase(c) }
}
#[inline]
pub unsafe fn u_isUWhiteSpace(c: i32) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn u_isUWhiteSpace(c : i32) -> i8);
    unsafe { u_isUWhiteSpace(c) }
}
#[inline]
pub unsafe fn u_isWhitespace(c: i32) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn u_isWhitespace(c : i32) -> i8);
    unsafe { u_isWhitespace(c) }
}
#[inline]
pub unsafe fn u_isalnum(c: i32) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn u_isalnum(c : i32) -> i8);
    unsafe { u_isalnum(c) }
}
#[inline]
pub unsafe fn u_isalpha(c: i32) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn u_isalpha(c : i32) -> i8);
    unsafe { u_isalpha(c) }
}
#[inline]
pub unsafe fn u_isbase(c: i32) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn u_isbase(c : i32) -> i8);
    unsafe { u_isbase(c) }
}
#[inline]
pub unsafe fn u_isblank(c: i32) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn u_isblank(c : i32) -> i8);
    unsafe { u_isblank(c) }
}
#[inline]
pub unsafe fn u_iscntrl(c: i32) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn u_iscntrl(c : i32) -> i8);
    unsafe { u_iscntrl(c) }
}
#[inline]
pub unsafe fn u_isdefined(c: i32) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn u_isdefined(c : i32) -> i8);
    unsafe { u_isdefined(c) }
}
#[inline]
pub unsafe fn u_isdigit(c: i32) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn u_isdigit(c : i32) -> i8);
    unsafe { u_isdigit(c) }
}
#[inline]
pub unsafe fn u_isgraph(c: i32) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn u_isgraph(c : i32) -> i8);
    unsafe { u_isgraph(c) }
}
#[inline]
pub unsafe fn u_islower(c: i32) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn u_islower(c : i32) -> i8);
    unsafe { u_islower(c) }
}
#[inline]
pub unsafe fn u_isprint(c: i32) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn u_isprint(c : i32) -> i8);
    unsafe { u_isprint(c) }
}
#[inline]
pub unsafe fn u_ispunct(c: i32) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn u_ispunct(c : i32) -> i8);
    unsafe { u_ispunct(c) }
}
#[inline]
pub unsafe fn u_isspace(c: i32) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn u_isspace(c : i32) -> i8);
    unsafe { u_isspace(c) }
}
#[inline]
pub unsafe fn u_istitle(c: i32) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn u_istitle(c : i32) -> i8);
    unsafe { u_istitle(c) }
}
#[inline]
pub unsafe fn u_isupper(c: i32) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn u_isupper(c : i32) -> i8);
    unsafe { u_isupper(c) }
}
#[inline]
pub unsafe fn u_isxdigit(c: i32) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn u_isxdigit(c : i32) -> i8);
    unsafe { u_isxdigit(c) }
}
#[inline]
pub unsafe fn u_memcasecmp(s1: *const u16, s2: *const u16, length: i32, options: u32) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_memcasecmp(s1 : *const u16, s2 : *const u16, length : i32, options : u32) -> i32);
    unsafe { u_memcasecmp(s1, s2, length, options) }
}
#[inline]
pub unsafe fn u_memchr(s: *const u16, c: u16, count: i32) -> *mut u16 {
    windows_core::link!("icuuc.dll" "C" fn u_memchr(s : *const u16, c : u16, count : i32) -> *mut u16);
    unsafe { u_memchr(s, c, count) }
}
#[inline]
pub unsafe fn u_memchr32(s: *const u16, c: i32, count: i32) -> *mut u16 {
    windows_core::link!("icuuc.dll" "C" fn u_memchr32(s : *const u16, c : i32, count : i32) -> *mut u16);
    unsafe { u_memchr32(s, c, count) }
}
#[inline]
pub unsafe fn u_memcmp(buf1: *const u16, buf2: *const u16, count: i32) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_memcmp(buf1 : *const u16, buf2 : *const u16, count : i32) -> i32);
    unsafe { u_memcmp(buf1, buf2, count) }
}
#[inline]
pub unsafe fn u_memcmpCodePointOrder(s1: *const u16, s2: *const u16, count: i32) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_memcmpCodePointOrder(s1 : *const u16, s2 : *const u16, count : i32) -> i32);
    unsafe { u_memcmpCodePointOrder(s1, s2, count) }
}
#[inline]
pub unsafe fn u_memcpy(dest: *mut u16, src: *const u16, count: i32) -> *mut u16 {
    windows_core::link!("icuuc.dll" "C" fn u_memcpy(dest : *mut u16, src : *const u16, count : i32) -> *mut u16);
    unsafe { u_memcpy(dest as _, src, count) }
}
#[inline]
pub unsafe fn u_memmove(dest: *mut u16, src: *const u16, count: i32) -> *mut u16 {
    windows_core::link!("icuuc.dll" "C" fn u_memmove(dest : *mut u16, src : *const u16, count : i32) -> *mut u16);
    unsafe { u_memmove(dest as _, src, count) }
}
#[inline]
pub unsafe fn u_memrchr(s: *const u16, c: u16, count: i32) -> *mut u16 {
    windows_core::link!("icuuc.dll" "C" fn u_memrchr(s : *const u16, c : u16, count : i32) -> *mut u16);
    unsafe { u_memrchr(s, c, count) }
}
#[inline]
pub unsafe fn u_memrchr32(s: *const u16, c: i32, count: i32) -> *mut u16 {
    windows_core::link!("icuuc.dll" "C" fn u_memrchr32(s : *const u16, c : i32, count : i32) -> *mut u16);
    unsafe { u_memrchr32(s, c, count) }
}
#[inline]
pub unsafe fn u_memset(dest: *mut u16, c: u16, count: i32) -> *mut u16 {
    windows_core::link!("icuuc.dll" "C" fn u_memset(dest : *mut u16, c : u16, count : i32) -> *mut u16);
    unsafe { u_memset(dest as _, c, count) }
}
#[inline]
pub unsafe fn u_parseMessage<P0>(locale: P0, pattern: *const u16, patternlength: i32, source: *const u16, sourcelength: i32, status: *mut UErrorCode)
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuin.dll" "C" fn u_parseMessage(locale : windows_core::PCSTR, pattern : *const u16, patternlength : i32, source : *const u16, sourcelength : i32, status : *mut UErrorCode));
    unsafe { u_parseMessage(locale.param().abi(), pattern, patternlength, source, sourcelength, status as _) }
}
#[inline]
pub unsafe fn u_parseMessageWithError<P0>(locale: P0, pattern: *const u16, patternlength: i32, source: *const u16, sourcelength: i32, parseerror: *mut UParseError, status: *mut UErrorCode)
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuin.dll" "C" fn u_parseMessageWithError(locale : windows_core::PCSTR, pattern : *const u16, patternlength : i32, source : *const u16, sourcelength : i32, parseerror : *mut UParseError, status : *mut UErrorCode));
    unsafe { u_parseMessageWithError(locale.param().abi(), pattern, patternlength, source, sourcelength, parseerror as _, status as _) }
}
#[inline]
pub unsafe fn u_setMemoryFunctions(context: *const core::ffi::c_void, a: *mut UMemAllocFn, r: *mut UMemReallocFn, f: *mut UMemFreeFn, status: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn u_setMemoryFunctions(context : *const core::ffi::c_void, a : *mut UMemAllocFn, r : *mut UMemReallocFn, f : *mut UMemFreeFn, status : *mut UErrorCode));
    unsafe { u_setMemoryFunctions(context, a as _, r as _, f as _, status as _) }
}
#[inline]
pub unsafe fn u_shapeArabic(source: *const u16, sourcelength: i32, dest: *mut u16, destsize: i32, options: u32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_shapeArabic(source : *const u16, sourcelength : i32, dest : *mut u16, destsize : i32, options : u32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { u_shapeArabic(source, sourcelength, dest as _, destsize, options, perrorcode as _) }
}
#[inline]
pub unsafe fn u_strCaseCompare(s1: *const u16, length1: i32, s2: *const u16, length2: i32, options: u32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_strCaseCompare(s1 : *const u16, length1 : i32, s2 : *const u16, length2 : i32, options : u32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { u_strCaseCompare(s1, length1, s2, length2, options, perrorcode as _) }
}
#[inline]
pub unsafe fn u_strCompare(s1: *const u16, length1: i32, s2: *const u16, length2: i32, codepointorder: i8) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_strCompare(s1 : *const u16, length1 : i32, s2 : *const u16, length2 : i32, codepointorder : i8) -> i32);
    unsafe { u_strCompare(s1, length1, s2, length2, codepointorder) }
}
#[inline]
pub unsafe fn u_strCompareIter(iter1: *mut UCharIterator, iter2: *mut UCharIterator, codepointorder: i8) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_strCompareIter(iter1 : *mut UCharIterator, iter2 : *mut UCharIterator, codepointorder : i8) -> i32);
    unsafe { u_strCompareIter(iter1 as _, iter2 as _, codepointorder) }
}
#[inline]
pub unsafe fn u_strFindFirst(s: *const u16, length: i32, substring: *const u16, sublength: i32) -> *mut u16 {
    windows_core::link!("icuuc.dll" "C" fn u_strFindFirst(s : *const u16, length : i32, substring : *const u16, sublength : i32) -> *mut u16);
    unsafe { u_strFindFirst(s, length, substring, sublength) }
}
#[inline]
pub unsafe fn u_strFindLast(s: *const u16, length: i32, substring: *const u16, sublength: i32) -> *mut u16 {
    windows_core::link!("icuuc.dll" "C" fn u_strFindLast(s : *const u16, length : i32, substring : *const u16, sublength : i32) -> *mut u16);
    unsafe { u_strFindLast(s, length, substring, sublength) }
}
#[inline]
pub unsafe fn u_strFoldCase(dest: *mut u16, destcapacity: i32, src: *const u16, srclength: i32, options: u32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_strFoldCase(dest : *mut u16, destcapacity : i32, src : *const u16, srclength : i32, options : u32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { u_strFoldCase(dest as _, destcapacity, src, srclength, options, perrorcode as _) }
}
#[inline]
pub unsafe fn u_strFromJavaModifiedUTF8WithSub<P3>(dest: *mut u16, destcapacity: i32, pdestlength: *mut i32, src: P3, srclength: i32, subchar: i32, pnumsubstitutions: *mut i32, perrorcode: *mut UErrorCode) -> *mut u16
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn u_strFromJavaModifiedUTF8WithSub(dest : *mut u16, destcapacity : i32, pdestlength : *mut i32, src : windows_core::PCSTR, srclength : i32, subchar : i32, pnumsubstitutions : *mut i32, perrorcode : *mut UErrorCode) -> *mut u16);
    unsafe { u_strFromJavaModifiedUTF8WithSub(dest as _, destcapacity, pdestlength as _, src.param().abi(), srclength, subchar, pnumsubstitutions as _, perrorcode as _) }
}
#[inline]
pub unsafe fn u_strFromUTF32(dest: *mut u16, destcapacity: i32, pdestlength: *mut i32, src: *const i32, srclength: i32, perrorcode: *mut UErrorCode) -> *mut u16 {
    windows_core::link!("icuuc.dll" "C" fn u_strFromUTF32(dest : *mut u16, destcapacity : i32, pdestlength : *mut i32, src : *const i32, srclength : i32, perrorcode : *mut UErrorCode) -> *mut u16);
    unsafe { u_strFromUTF32(dest as _, destcapacity, pdestlength as _, src, srclength, perrorcode as _) }
}
#[inline]
pub unsafe fn u_strFromUTF32WithSub(dest: *mut u16, destcapacity: i32, pdestlength: *mut i32, src: *const i32, srclength: i32, subchar: i32, pnumsubstitutions: *mut i32, perrorcode: *mut UErrorCode) -> *mut u16 {
    windows_core::link!("icuuc.dll" "C" fn u_strFromUTF32WithSub(dest : *mut u16, destcapacity : i32, pdestlength : *mut i32, src : *const i32, srclength : i32, subchar : i32, pnumsubstitutions : *mut i32, perrorcode : *mut UErrorCode) -> *mut u16);
    unsafe { u_strFromUTF32WithSub(dest as _, destcapacity, pdestlength as _, src, srclength, subchar, pnumsubstitutions as _, perrorcode as _) }
}
#[inline]
pub unsafe fn u_strFromUTF8<P3>(dest: *mut u16, destcapacity: i32, pdestlength: *mut i32, src: P3, srclength: i32, perrorcode: *mut UErrorCode) -> *mut u16
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn u_strFromUTF8(dest : *mut u16, destcapacity : i32, pdestlength : *mut i32, src : windows_core::PCSTR, srclength : i32, perrorcode : *mut UErrorCode) -> *mut u16);
    unsafe { u_strFromUTF8(dest as _, destcapacity, pdestlength as _, src.param().abi(), srclength, perrorcode as _) }
}
#[inline]
pub unsafe fn u_strFromUTF8Lenient<P3>(dest: *mut u16, destcapacity: i32, pdestlength: *mut i32, src: P3, srclength: i32, perrorcode: *mut UErrorCode) -> *mut u16
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn u_strFromUTF8Lenient(dest : *mut u16, destcapacity : i32, pdestlength : *mut i32, src : windows_core::PCSTR, srclength : i32, perrorcode : *mut UErrorCode) -> *mut u16);
    unsafe { u_strFromUTF8Lenient(dest as _, destcapacity, pdestlength as _, src.param().abi(), srclength, perrorcode as _) }
}
#[inline]
pub unsafe fn u_strFromUTF8WithSub<P3>(dest: *mut u16, destcapacity: i32, pdestlength: *mut i32, src: P3, srclength: i32, subchar: i32, pnumsubstitutions: *mut i32, perrorcode: *mut UErrorCode) -> *mut u16
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn u_strFromUTF8WithSub(dest : *mut u16, destcapacity : i32, pdestlength : *mut i32, src : windows_core::PCSTR, srclength : i32, subchar : i32, pnumsubstitutions : *mut i32, perrorcode : *mut UErrorCode) -> *mut u16);
    unsafe { u_strFromUTF8WithSub(dest as _, destcapacity, pdestlength as _, src.param().abi(), srclength, subchar, pnumsubstitutions as _, perrorcode as _) }
}
#[inline]
pub unsafe fn u_strFromWCS<P3>(dest: *mut u16, destcapacity: i32, pdestlength: *mut i32, src: P3, srclength: i32, perrorcode: *mut UErrorCode) -> *mut u16
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn u_strFromWCS(dest : *mut u16, destcapacity : i32, pdestlength : *mut i32, src : windows_core::PCWSTR, srclength : i32, perrorcode : *mut UErrorCode) -> *mut u16);
    unsafe { u_strFromWCS(dest as _, destcapacity, pdestlength as _, src.param().abi(), srclength, perrorcode as _) }
}
#[inline]
pub unsafe fn u_strHasMoreChar32Than(s: *const u16, length: i32, number: i32) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn u_strHasMoreChar32Than(s : *const u16, length : i32, number : i32) -> i8);
    unsafe { u_strHasMoreChar32Than(s, length, number) }
}
#[inline]
pub unsafe fn u_strToJavaModifiedUTF8<P0>(dest: P0, destcapacity: i32, pdestlength: *mut i32, src: *const u16, srclength: i32, perrorcode: *mut UErrorCode) -> windows_core::PSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn u_strToJavaModifiedUTF8(dest : windows_core::PCSTR, destcapacity : i32, pdestlength : *mut i32, src : *const u16, srclength : i32, perrorcode : *mut UErrorCode) -> windows_core::PSTR);
    unsafe { u_strToJavaModifiedUTF8(dest.param().abi(), destcapacity, pdestlength as _, src, srclength, perrorcode as _) }
}
#[inline]
pub unsafe fn u_strToLower<P4>(dest: *mut u16, destcapacity: i32, src: *const u16, srclength: i32, locale: P4, perrorcode: *mut UErrorCode) -> i32
where
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn u_strToLower(dest : *mut u16, destcapacity : i32, src : *const u16, srclength : i32, locale : windows_core::PCSTR, perrorcode : *mut UErrorCode) -> i32);
    unsafe { u_strToLower(dest as _, destcapacity, src, srclength, locale.param().abi(), perrorcode as _) }
}
#[inline]
pub unsafe fn u_strToTitle<P5>(dest: *mut u16, destcapacity: i32, src: *const u16, srclength: i32, titleiter: *mut UBreakIterator, locale: P5, perrorcode: *mut UErrorCode) -> i32
where
    P5: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn u_strToTitle(dest : *mut u16, destcapacity : i32, src : *const u16, srclength : i32, titleiter : *mut UBreakIterator, locale : windows_core::PCSTR, perrorcode : *mut UErrorCode) -> i32);
    unsafe { u_strToTitle(dest as _, destcapacity, src, srclength, titleiter as _, locale.param().abi(), perrorcode as _) }
}
#[inline]
pub unsafe fn u_strToUTF32(dest: *mut i32, destcapacity: i32, pdestlength: *mut i32, src: *const u16, srclength: i32, perrorcode: *mut UErrorCode) -> *mut i32 {
    windows_core::link!("icuuc.dll" "C" fn u_strToUTF32(dest : *mut i32, destcapacity : i32, pdestlength : *mut i32, src : *const u16, srclength : i32, perrorcode : *mut UErrorCode) -> *mut i32);
    unsafe { u_strToUTF32(dest as _, destcapacity, pdestlength as _, src, srclength, perrorcode as _) }
}
#[inline]
pub unsafe fn u_strToUTF32WithSub(dest: *mut i32, destcapacity: i32, pdestlength: *mut i32, src: *const u16, srclength: i32, subchar: i32, pnumsubstitutions: *mut i32, perrorcode: *mut UErrorCode) -> *mut i32 {
    windows_core::link!("icuuc.dll" "C" fn u_strToUTF32WithSub(dest : *mut i32, destcapacity : i32, pdestlength : *mut i32, src : *const u16, srclength : i32, subchar : i32, pnumsubstitutions : *mut i32, perrorcode : *mut UErrorCode) -> *mut i32);
    unsafe { u_strToUTF32WithSub(dest as _, destcapacity, pdestlength as _, src, srclength, subchar, pnumsubstitutions as _, perrorcode as _) }
}
#[inline]
pub unsafe fn u_strToUTF8<P0>(dest: P0, destcapacity: i32, pdestlength: *mut i32, src: *const u16, srclength: i32, perrorcode: *mut UErrorCode) -> windows_core::PSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn u_strToUTF8(dest : windows_core::PCSTR, destcapacity : i32, pdestlength : *mut i32, src : *const u16, srclength : i32, perrorcode : *mut UErrorCode) -> windows_core::PSTR);
    unsafe { u_strToUTF8(dest.param().abi(), destcapacity, pdestlength as _, src, srclength, perrorcode as _) }
}
#[inline]
pub unsafe fn u_strToUTF8WithSub<P0>(dest: P0, destcapacity: i32, pdestlength: *mut i32, src: *const u16, srclength: i32, subchar: i32, pnumsubstitutions: *mut i32, perrorcode: *mut UErrorCode) -> windows_core::PSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn u_strToUTF8WithSub(dest : windows_core::PCSTR, destcapacity : i32, pdestlength : *mut i32, src : *const u16, srclength : i32, subchar : i32, pnumsubstitutions : *mut i32, perrorcode : *mut UErrorCode) -> windows_core::PSTR);
    unsafe { u_strToUTF8WithSub(dest.param().abi(), destcapacity, pdestlength as _, src, srclength, subchar, pnumsubstitutions as _, perrorcode as _) }
}
#[inline]
pub unsafe fn u_strToUpper<P4>(dest: *mut u16, destcapacity: i32, src: *const u16, srclength: i32, locale: P4, perrorcode: *mut UErrorCode) -> i32
where
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn u_strToUpper(dest : *mut u16, destcapacity : i32, src : *const u16, srclength : i32, locale : windows_core::PCSTR, perrorcode : *mut UErrorCode) -> i32);
    unsafe { u_strToUpper(dest as _, destcapacity, src, srclength, locale.param().abi(), perrorcode as _) }
}
#[inline]
pub unsafe fn u_strToWCS<P0>(dest: P0, destcapacity: i32, pdestlength: *mut i32, src: *const u16, srclength: i32, perrorcode: *mut UErrorCode) -> windows_core::PWSTR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn u_strToWCS(dest : windows_core::PCWSTR, destcapacity : i32, pdestlength : *mut i32, src : *const u16, srclength : i32, perrorcode : *mut UErrorCode) -> windows_core::PWSTR);
    unsafe { u_strToWCS(dest.param().abi(), destcapacity, pdestlength as _, src, srclength, perrorcode as _) }
}
#[inline]
pub unsafe fn u_strcasecmp(s1: *const u16, s2: *const u16, options: u32) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_strcasecmp(s1 : *const u16, s2 : *const u16, options : u32) -> i32);
    unsafe { u_strcasecmp(s1, s2, options) }
}
#[inline]
pub unsafe fn u_strcat(dst: *mut u16, src: *const u16) -> *mut u16 {
    windows_core::link!("icuuc.dll" "C" fn u_strcat(dst : *mut u16, src : *const u16) -> *mut u16);
    unsafe { u_strcat(dst as _, src) }
}
#[inline]
pub unsafe fn u_strchr(s: *const u16, c: u16) -> *mut u16 {
    windows_core::link!("icuuc.dll" "C" fn u_strchr(s : *const u16, c : u16) -> *mut u16);
    unsafe { u_strchr(s, c) }
}
#[inline]
pub unsafe fn u_strchr32(s: *const u16, c: i32) -> *mut u16 {
    windows_core::link!("icuuc.dll" "C" fn u_strchr32(s : *const u16, c : i32) -> *mut u16);
    unsafe { u_strchr32(s, c) }
}
#[inline]
pub unsafe fn u_strcmp(s1: *const u16, s2: *const u16) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_strcmp(s1 : *const u16, s2 : *const u16) -> i32);
    unsafe { u_strcmp(s1, s2) }
}
#[inline]
pub unsafe fn u_strcmpCodePointOrder(s1: *const u16, s2: *const u16) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_strcmpCodePointOrder(s1 : *const u16, s2 : *const u16) -> i32);
    unsafe { u_strcmpCodePointOrder(s1, s2) }
}
#[inline]
pub unsafe fn u_strcpy(dst: *mut u16, src: *const u16) -> *mut u16 {
    windows_core::link!("icuuc.dll" "C" fn u_strcpy(dst : *mut u16, src : *const u16) -> *mut u16);
    unsafe { u_strcpy(dst as _, src) }
}
#[inline]
pub unsafe fn u_strcspn(string: *const u16, matchset: *const u16) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_strcspn(string : *const u16, matchset : *const u16) -> i32);
    unsafe { u_strcspn(string, matchset) }
}
#[inline]
pub unsafe fn u_strlen(s: *const u16) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_strlen(s : *const u16) -> i32);
    unsafe { u_strlen(s) }
}
#[inline]
pub unsafe fn u_strncasecmp(s1: *const u16, s2: *const u16, n: i32, options: u32) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_strncasecmp(s1 : *const u16, s2 : *const u16, n : i32, options : u32) -> i32);
    unsafe { u_strncasecmp(s1, s2, n, options) }
}
#[inline]
pub unsafe fn u_strncat(dst: *mut u16, src: *const u16, n: i32) -> *mut u16 {
    windows_core::link!("icuuc.dll" "C" fn u_strncat(dst : *mut u16, src : *const u16, n : i32) -> *mut u16);
    unsafe { u_strncat(dst as _, src, n) }
}
#[inline]
pub unsafe fn u_strncmp(ucs1: *const u16, ucs2: *const u16, n: i32) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_strncmp(ucs1 : *const u16, ucs2 : *const u16, n : i32) -> i32);
    unsafe { u_strncmp(ucs1, ucs2, n) }
}
#[inline]
pub unsafe fn u_strncmpCodePointOrder(s1: *const u16, s2: *const u16, n: i32) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_strncmpCodePointOrder(s1 : *const u16, s2 : *const u16, n : i32) -> i32);
    unsafe { u_strncmpCodePointOrder(s1, s2, n) }
}
#[inline]
pub unsafe fn u_strncpy(dst: *mut u16, src: *const u16, n: i32) -> *mut u16 {
    windows_core::link!("icuuc.dll" "C" fn u_strncpy(dst : *mut u16, src : *const u16, n : i32) -> *mut u16);
    unsafe { u_strncpy(dst as _, src, n) }
}
#[inline]
pub unsafe fn u_strpbrk(string: *const u16, matchset: *const u16) -> *mut u16 {
    windows_core::link!("icuuc.dll" "C" fn u_strpbrk(string : *const u16, matchset : *const u16) -> *mut u16);
    unsafe { u_strpbrk(string, matchset) }
}
#[inline]
pub unsafe fn u_strrchr(s: *const u16, c: u16) -> *mut u16 {
    windows_core::link!("icuuc.dll" "C" fn u_strrchr(s : *const u16, c : u16) -> *mut u16);
    unsafe { u_strrchr(s, c) }
}
#[inline]
pub unsafe fn u_strrchr32(s: *const u16, c: i32) -> *mut u16 {
    windows_core::link!("icuuc.dll" "C" fn u_strrchr32(s : *const u16, c : i32) -> *mut u16);
    unsafe { u_strrchr32(s, c) }
}
#[inline]
pub unsafe fn u_strrstr(s: *const u16, substring: *const u16) -> *mut u16 {
    windows_core::link!("icuuc.dll" "C" fn u_strrstr(s : *const u16, substring : *const u16) -> *mut u16);
    unsafe { u_strrstr(s, substring) }
}
#[inline]
pub unsafe fn u_strspn(string: *const u16, matchset: *const u16) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_strspn(string : *const u16, matchset : *const u16) -> i32);
    unsafe { u_strspn(string, matchset) }
}
#[inline]
pub unsafe fn u_strstr(s: *const u16, substring: *const u16) -> *mut u16 {
    windows_core::link!("icuuc.dll" "C" fn u_strstr(s : *const u16, substring : *const u16) -> *mut u16);
    unsafe { u_strstr(s, substring) }
}
#[inline]
pub unsafe fn u_strtok_r(src: *mut u16, delim: *const u16, savestate: *mut *mut u16) -> *mut u16 {
    windows_core::link!("icuuc.dll" "C" fn u_strtok_r(src : *mut u16, delim : *const u16, savestate : *mut *mut u16) -> *mut u16);
    unsafe { u_strtok_r(src as _, delim, savestate as _) }
}
#[inline]
pub unsafe fn u_tolower(c: i32) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_tolower(c : i32) -> i32);
    unsafe { u_tolower(c) }
}
#[inline]
pub unsafe fn u_totitle(c: i32) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_totitle(c : i32) -> i32);
    unsafe { u_totitle(c) }
}
#[inline]
pub unsafe fn u_toupper(c: i32) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_toupper(c : i32) -> i32);
    unsafe { u_toupper(c) }
}
#[inline]
pub unsafe fn u_uastrcpy<P1>(dst: *mut u16, src: P1) -> *mut u16
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn u_uastrcpy(dst : *mut u16, src : windows_core::PCSTR) -> *mut u16);
    unsafe { u_uastrcpy(dst as _, src.param().abi()) }
}
#[inline]
pub unsafe fn u_uastrncpy<P1>(dst: *mut u16, src: P1, n: i32) -> *mut u16
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn u_uastrncpy(dst : *mut u16, src : windows_core::PCSTR, n : i32) -> *mut u16);
    unsafe { u_uastrncpy(dst as _, src.param().abi(), n) }
}
#[inline]
pub unsafe fn u_unescape<P0>(src: P0, dest: *mut u16, destcapacity: i32) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn u_unescape(src : windows_core::PCSTR, dest : *mut u16, destcapacity : i32) -> i32);
    unsafe { u_unescape(src.param().abi(), dest as _, destcapacity) }
}
#[inline]
pub unsafe fn u_unescapeAt(charat: UNESCAPE_CHAR_AT, offset: *mut i32, length: i32, context: *mut core::ffi::c_void) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn u_unescapeAt(charat : UNESCAPE_CHAR_AT, offset : *mut i32, length : i32, context : *mut core::ffi::c_void) -> i32);
    unsafe { u_unescapeAt(charat, offset as _, length, context as _) }
}
#[inline]
pub unsafe fn u_versionFromString<P1>(versionarray: *mut u8, versionstring: P1)
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn u_versionFromString(versionarray : *mut u8, versionstring : windows_core::PCSTR));
    unsafe { u_versionFromString(versionarray as _, versionstring.param().abi()) }
}
#[inline]
pub unsafe fn u_versionFromUString(versionarray: *mut u8, versionstring: *const u16) {
    windows_core::link!("icuuc.dll" "C" fn u_versionFromUString(versionarray : *mut u8, versionstring : *const u16));
    unsafe { u_versionFromUString(versionarray as _, versionstring) }
}
#[inline]
pub unsafe fn u_versionToString<P1>(versionarray: *const u8, versionstring: P1)
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn u_versionToString(versionarray : *const u8, versionstring : windows_core::PCSTR));
    unsafe { u_versionToString(versionarray, versionstring.param().abi()) }
}
#[inline]
pub unsafe fn u_vformatMessage<P0>(locale: P0, pattern: *const u16, patternlength: i32, result: *mut u16, resultlength: i32, ap: *mut i8, status: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuin.dll" "C" fn u_vformatMessage(locale : windows_core::PCSTR, pattern : *const u16, patternlength : i32, result : *mut u16, resultlength : i32, ap : *mut i8, status : *mut UErrorCode) -> i32);
    unsafe { u_vformatMessage(locale.param().abi(), pattern, patternlength, result as _, resultlength, ap as _, status as _) }
}
#[inline]
pub unsafe fn u_vformatMessageWithError<P0>(locale: P0, pattern: *const u16, patternlength: i32, result: *mut u16, resultlength: i32, parseerror: *mut UParseError, ap: *mut i8, status: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuin.dll" "C" fn u_vformatMessageWithError(locale : windows_core::PCSTR, pattern : *const u16, patternlength : i32, result : *mut u16, resultlength : i32, parseerror : *mut UParseError, ap : *mut i8, status : *mut UErrorCode) -> i32);
    unsafe { u_vformatMessageWithError(locale.param().abi(), pattern, patternlength, result as _, resultlength, parseerror as _, ap as _, status as _) }
}
#[inline]
pub unsafe fn u_vparseMessage<P0>(locale: P0, pattern: *const u16, patternlength: i32, source: *const u16, sourcelength: i32, ap: *mut i8, status: *mut UErrorCode)
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuin.dll" "C" fn u_vparseMessage(locale : windows_core::PCSTR, pattern : *const u16, patternlength : i32, source : *const u16, sourcelength : i32, ap : *mut i8, status : *mut UErrorCode));
    unsafe { u_vparseMessage(locale.param().abi(), pattern, patternlength, source, sourcelength, ap as _, status as _) }
}
#[inline]
pub unsafe fn u_vparseMessageWithError<P0>(locale: P0, pattern: *const u16, patternlength: i32, source: *const u16, sourcelength: i32, ap: *mut i8, parseerror: *mut UParseError, status: *mut UErrorCode)
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuin.dll" "C" fn u_vparseMessageWithError(locale : windows_core::PCSTR, pattern : *const u16, patternlength : i32, source : *const u16, sourcelength : i32, ap : *mut i8, parseerror : *mut UParseError, status : *mut UErrorCode));
    unsafe { u_vparseMessageWithError(locale.param().abi(), pattern, patternlength, source, sourcelength, ap as _, parseerror as _, status as _) }
}
#[inline]
pub unsafe fn ubidi_close(pbidi: *mut UBiDi) {
    windows_core::link!("icuuc.dll" "C" fn ubidi_close(pbidi : *mut UBiDi));
    unsafe { ubidi_close(pbidi as _) }
}
#[inline]
pub unsafe fn ubidi_countParagraphs(pbidi: *mut UBiDi) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ubidi_countParagraphs(pbidi : *mut UBiDi) -> i32);
    unsafe { ubidi_countParagraphs(pbidi as _) }
}
#[inline]
pub unsafe fn ubidi_countRuns(pbidi: *mut UBiDi, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ubidi_countRuns(pbidi : *mut UBiDi, perrorcode : *mut UErrorCode) -> i32);
    unsafe { ubidi_countRuns(pbidi as _, perrorcode as _) }
}
#[inline]
pub unsafe fn ubidi_getBaseDirection(text: *const u16, length: i32) -> UBiDiDirection {
    windows_core::link!("icuuc.dll" "C" fn ubidi_getBaseDirection(text : *const u16, length : i32) -> UBiDiDirection);
    unsafe { ubidi_getBaseDirection(text, length) }
}
#[inline]
pub unsafe fn ubidi_getClassCallback(pbidi: *mut UBiDi, r#fn: *mut UBiDiClassCallback, context: *const *const core::ffi::c_void) {
    windows_core::link!("icuuc.dll" "C" fn ubidi_getClassCallback(pbidi : *mut UBiDi, r#fn : *mut UBiDiClassCallback, context : *const *const core::ffi::c_void));
    unsafe { ubidi_getClassCallback(pbidi as _, r#fn as _, context) }
}
#[inline]
pub unsafe fn ubidi_getCustomizedClass(pbidi: *mut UBiDi, c: i32) -> UCharDirection {
    windows_core::link!("icuuc.dll" "C" fn ubidi_getCustomizedClass(pbidi : *mut UBiDi, c : i32) -> UCharDirection);
    unsafe { ubidi_getCustomizedClass(pbidi as _, c) }
}
#[inline]
pub unsafe fn ubidi_getDirection(pbidi: *const UBiDi) -> UBiDiDirection {
    windows_core::link!("icuuc.dll" "C" fn ubidi_getDirection(pbidi : *const UBiDi) -> UBiDiDirection);
    unsafe { ubidi_getDirection(pbidi) }
}
#[inline]
pub unsafe fn ubidi_getLength(pbidi: *const UBiDi) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ubidi_getLength(pbidi : *const UBiDi) -> i32);
    unsafe { ubidi_getLength(pbidi) }
}
#[inline]
pub unsafe fn ubidi_getLevelAt(pbidi: *const UBiDi, charindex: i32) -> u8 {
    windows_core::link!("icuuc.dll" "C" fn ubidi_getLevelAt(pbidi : *const UBiDi, charindex : i32) -> u8);
    unsafe { ubidi_getLevelAt(pbidi, charindex) }
}
#[inline]
pub unsafe fn ubidi_getLevels(pbidi: *mut UBiDi, perrorcode: *mut UErrorCode) -> *mut u8 {
    windows_core::link!("icuuc.dll" "C" fn ubidi_getLevels(pbidi : *mut UBiDi, perrorcode : *mut UErrorCode) -> *mut u8);
    unsafe { ubidi_getLevels(pbidi as _, perrorcode as _) }
}
#[inline]
pub unsafe fn ubidi_getLogicalIndex(pbidi: *mut UBiDi, visualindex: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ubidi_getLogicalIndex(pbidi : *mut UBiDi, visualindex : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { ubidi_getLogicalIndex(pbidi as _, visualindex, perrorcode as _) }
}
#[inline]
pub unsafe fn ubidi_getLogicalMap(pbidi: *mut UBiDi, indexmap: *mut i32, perrorcode: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn ubidi_getLogicalMap(pbidi : *mut UBiDi, indexmap : *mut i32, perrorcode : *mut UErrorCode));
    unsafe { ubidi_getLogicalMap(pbidi as _, indexmap as _, perrorcode as _) }
}
#[inline]
pub unsafe fn ubidi_getLogicalRun(pbidi: *const UBiDi, logicalposition: i32, plogicallimit: *mut i32, plevel: *mut u8) {
    windows_core::link!("icuuc.dll" "C" fn ubidi_getLogicalRun(pbidi : *const UBiDi, logicalposition : i32, plogicallimit : *mut i32, plevel : *mut u8));
    unsafe { ubidi_getLogicalRun(pbidi, logicalposition, plogicallimit as _, plevel as _) }
}
#[inline]
pub unsafe fn ubidi_getParaLevel(pbidi: *const UBiDi) -> u8 {
    windows_core::link!("icuuc.dll" "C" fn ubidi_getParaLevel(pbidi : *const UBiDi) -> u8);
    unsafe { ubidi_getParaLevel(pbidi) }
}
#[inline]
pub unsafe fn ubidi_getParagraph(pbidi: *const UBiDi, charindex: i32, pparastart: *mut i32, pparalimit: *mut i32, pparalevel: *mut u8, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ubidi_getParagraph(pbidi : *const UBiDi, charindex : i32, pparastart : *mut i32, pparalimit : *mut i32, pparalevel : *mut u8, perrorcode : *mut UErrorCode) -> i32);
    unsafe { ubidi_getParagraph(pbidi, charindex, pparastart as _, pparalimit as _, pparalevel as _, perrorcode as _) }
}
#[inline]
pub unsafe fn ubidi_getParagraphByIndex(pbidi: *const UBiDi, paraindex: i32, pparastart: *mut i32, pparalimit: *mut i32, pparalevel: *mut u8, perrorcode: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn ubidi_getParagraphByIndex(pbidi : *const UBiDi, paraindex : i32, pparastart : *mut i32, pparalimit : *mut i32, pparalevel : *mut u8, perrorcode : *mut UErrorCode));
    unsafe { ubidi_getParagraphByIndex(pbidi, paraindex, pparastart as _, pparalimit as _, pparalevel as _, perrorcode as _) }
}
#[inline]
pub unsafe fn ubidi_getProcessedLength(pbidi: *const UBiDi) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ubidi_getProcessedLength(pbidi : *const UBiDi) -> i32);
    unsafe { ubidi_getProcessedLength(pbidi) }
}
#[inline]
pub unsafe fn ubidi_getReorderingMode(pbidi: *mut UBiDi) -> UBiDiReorderingMode {
    windows_core::link!("icuuc.dll" "C" fn ubidi_getReorderingMode(pbidi : *mut UBiDi) -> UBiDiReorderingMode);
    unsafe { ubidi_getReorderingMode(pbidi as _) }
}
#[inline]
pub unsafe fn ubidi_getReorderingOptions(pbidi: *mut UBiDi) -> u32 {
    windows_core::link!("icuuc.dll" "C" fn ubidi_getReorderingOptions(pbidi : *mut UBiDi) -> u32);
    unsafe { ubidi_getReorderingOptions(pbidi as _) }
}
#[inline]
pub unsafe fn ubidi_getResultLength(pbidi: *const UBiDi) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ubidi_getResultLength(pbidi : *const UBiDi) -> i32);
    unsafe { ubidi_getResultLength(pbidi) }
}
#[inline]
pub unsafe fn ubidi_getText(pbidi: *const UBiDi) -> *mut u16 {
    windows_core::link!("icuuc.dll" "C" fn ubidi_getText(pbidi : *const UBiDi) -> *mut u16);
    unsafe { ubidi_getText(pbidi) }
}
#[inline]
pub unsafe fn ubidi_getVisualIndex(pbidi: *mut UBiDi, logicalindex: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ubidi_getVisualIndex(pbidi : *mut UBiDi, logicalindex : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { ubidi_getVisualIndex(pbidi as _, logicalindex, perrorcode as _) }
}
#[inline]
pub unsafe fn ubidi_getVisualMap(pbidi: *mut UBiDi, indexmap: *mut i32, perrorcode: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn ubidi_getVisualMap(pbidi : *mut UBiDi, indexmap : *mut i32, perrorcode : *mut UErrorCode));
    unsafe { ubidi_getVisualMap(pbidi as _, indexmap as _, perrorcode as _) }
}
#[inline]
pub unsafe fn ubidi_getVisualRun(pbidi: *mut UBiDi, runindex: i32, plogicalstart: *mut i32, plength: *mut i32) -> UBiDiDirection {
    windows_core::link!("icuuc.dll" "C" fn ubidi_getVisualRun(pbidi : *mut UBiDi, runindex : i32, plogicalstart : *mut i32, plength : *mut i32) -> UBiDiDirection);
    unsafe { ubidi_getVisualRun(pbidi as _, runindex, plogicalstart as _, plength as _) }
}
#[inline]
pub unsafe fn ubidi_invertMap(srcmap: *const i32, destmap: *mut i32, length: i32) {
    windows_core::link!("icuuc.dll" "C" fn ubidi_invertMap(srcmap : *const i32, destmap : *mut i32, length : i32));
    unsafe { ubidi_invertMap(srcmap, destmap as _, length) }
}
#[inline]
pub unsafe fn ubidi_isInverse(pbidi: *mut UBiDi) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn ubidi_isInverse(pbidi : *mut UBiDi) -> i8);
    unsafe { ubidi_isInverse(pbidi as _) }
}
#[inline]
pub unsafe fn ubidi_isOrderParagraphsLTR(pbidi: *mut UBiDi) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn ubidi_isOrderParagraphsLTR(pbidi : *mut UBiDi) -> i8);
    unsafe { ubidi_isOrderParagraphsLTR(pbidi as _) }
}
#[inline]
pub unsafe fn ubidi_open() -> *mut UBiDi {
    windows_core::link!("icuuc.dll" "C" fn ubidi_open() -> *mut UBiDi);
    unsafe { ubidi_open() }
}
#[inline]
pub unsafe fn ubidi_openSized(maxlength: i32, maxruncount: i32, perrorcode: *mut UErrorCode) -> *mut UBiDi {
    windows_core::link!("icuuc.dll" "C" fn ubidi_openSized(maxlength : i32, maxruncount : i32, perrorcode : *mut UErrorCode) -> *mut UBiDi);
    unsafe { ubidi_openSized(maxlength, maxruncount, perrorcode as _) }
}
#[inline]
pub unsafe fn ubidi_orderParagraphsLTR(pbidi: *mut UBiDi, orderparagraphsltr: i8) {
    windows_core::link!("icuuc.dll" "C" fn ubidi_orderParagraphsLTR(pbidi : *mut UBiDi, orderparagraphsltr : i8));
    unsafe { ubidi_orderParagraphsLTR(pbidi as _, orderparagraphsltr) }
}
#[inline]
pub unsafe fn ubidi_reorderLogical(levels: *const u8, length: i32, indexmap: *mut i32) {
    windows_core::link!("icuuc.dll" "C" fn ubidi_reorderLogical(levels : *const u8, length : i32, indexmap : *mut i32));
    unsafe { ubidi_reorderLogical(levels, length, indexmap as _) }
}
#[inline]
pub unsafe fn ubidi_reorderVisual(levels: *const u8, length: i32, indexmap: *mut i32) {
    windows_core::link!("icuuc.dll" "C" fn ubidi_reorderVisual(levels : *const u8, length : i32, indexmap : *mut i32));
    unsafe { ubidi_reorderVisual(levels, length, indexmap as _) }
}
#[inline]
pub unsafe fn ubidi_setClassCallback(pbidi: *mut UBiDi, newfn: UBiDiClassCallback, newcontext: *const core::ffi::c_void, oldfn: *mut UBiDiClassCallback, oldcontext: *const *const core::ffi::c_void, perrorcode: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn ubidi_setClassCallback(pbidi : *mut UBiDi, newfn : UBiDiClassCallback, newcontext : *const core::ffi::c_void, oldfn : *mut UBiDiClassCallback, oldcontext : *const *const core::ffi::c_void, perrorcode : *mut UErrorCode));
    unsafe { ubidi_setClassCallback(pbidi as _, newfn, newcontext, oldfn as _, oldcontext, perrorcode as _) }
}
#[inline]
pub unsafe fn ubidi_setContext(pbidi: *mut UBiDi, prologue: *const u16, prolength: i32, epilogue: *const u16, epilength: i32, perrorcode: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn ubidi_setContext(pbidi : *mut UBiDi, prologue : *const u16, prolength : i32, epilogue : *const u16, epilength : i32, perrorcode : *mut UErrorCode));
    unsafe { ubidi_setContext(pbidi as _, prologue, prolength, epilogue, epilength, perrorcode as _) }
}
#[inline]
pub unsafe fn ubidi_setInverse(pbidi: *mut UBiDi, isinverse: i8) {
    windows_core::link!("icuuc.dll" "C" fn ubidi_setInverse(pbidi : *mut UBiDi, isinverse : i8));
    unsafe { ubidi_setInverse(pbidi as _, isinverse) }
}
#[inline]
pub unsafe fn ubidi_setLine(pparabidi: *const UBiDi, start: i32, limit: i32, plinebidi: *mut UBiDi, perrorcode: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn ubidi_setLine(pparabidi : *const UBiDi, start : i32, limit : i32, plinebidi : *mut UBiDi, perrorcode : *mut UErrorCode));
    unsafe { ubidi_setLine(pparabidi, start, limit, plinebidi as _, perrorcode as _) }
}
#[inline]
pub unsafe fn ubidi_setPara(pbidi: *mut UBiDi, text: *const u16, length: i32, paralevel: u8, embeddinglevels: *mut u8, perrorcode: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn ubidi_setPara(pbidi : *mut UBiDi, text : *const u16, length : i32, paralevel : u8, embeddinglevels : *mut u8, perrorcode : *mut UErrorCode));
    unsafe { ubidi_setPara(pbidi as _, text, length, paralevel, embeddinglevels as _, perrorcode as _) }
}
#[inline]
pub unsafe fn ubidi_setReorderingMode(pbidi: *mut UBiDi, reorderingmode: UBiDiReorderingMode) {
    windows_core::link!("icuuc.dll" "C" fn ubidi_setReorderingMode(pbidi : *mut UBiDi, reorderingmode : UBiDiReorderingMode));
    unsafe { ubidi_setReorderingMode(pbidi as _, reorderingmode) }
}
#[inline]
pub unsafe fn ubidi_setReorderingOptions(pbidi: *mut UBiDi, reorderingoptions: u32) {
    windows_core::link!("icuuc.dll" "C" fn ubidi_setReorderingOptions(pbidi : *mut UBiDi, reorderingoptions : u32));
    unsafe { ubidi_setReorderingOptions(pbidi as _, reorderingoptions) }
}
#[inline]
pub unsafe fn ubidi_writeReordered(pbidi: *mut UBiDi, dest: *mut u16, destsize: i32, options: u16, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ubidi_writeReordered(pbidi : *mut UBiDi, dest : *mut u16, destsize : i32, options : u16, perrorcode : *mut UErrorCode) -> i32);
    unsafe { ubidi_writeReordered(pbidi as _, dest as _, destsize, options, perrorcode as _) }
}
#[inline]
pub unsafe fn ubidi_writeReverse(src: *const u16, srclength: i32, dest: *mut u16, destsize: i32, options: u16, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ubidi_writeReverse(src : *const u16, srclength : i32, dest : *mut u16, destsize : i32, options : u16, perrorcode : *mut UErrorCode) -> i32);
    unsafe { ubidi_writeReverse(src, srclength, dest as _, destsize, options, perrorcode as _) }
}
#[inline]
pub unsafe fn ubiditransform_close(pbiditransform: *mut UBiDiTransform) {
    windows_core::link!("icuuc.dll" "C" fn ubiditransform_close(pbiditransform : *mut UBiDiTransform));
    unsafe { ubiditransform_close(pbiditransform as _) }
}
#[inline]
pub unsafe fn ubiditransform_open(perrorcode: *mut UErrorCode) -> *mut UBiDiTransform {
    windows_core::link!("icuuc.dll" "C" fn ubiditransform_open(perrorcode : *mut UErrorCode) -> *mut UBiDiTransform);
    unsafe { ubiditransform_open(perrorcode as _) }
}
#[inline]
pub unsafe fn ubiditransform_transform(pbiditransform: *mut UBiDiTransform, src: *const u16, srclength: i32, dest: *mut u16, destsize: i32, inparalevel: u8, inorder: UBiDiOrder, outparalevel: u8, outorder: UBiDiOrder, domirroring: UBiDiMirroring, shapingoptions: u32, perrorcode: *mut UErrorCode) -> u32 {
    windows_core::link!("icuuc.dll" "C" fn ubiditransform_transform(pbiditransform : *mut UBiDiTransform, src : *const u16, srclength : i32, dest : *mut u16, destsize : i32, inparalevel : u8, inorder : UBiDiOrder, outparalevel : u8, outorder : UBiDiOrder, domirroring : UBiDiMirroring, shapingoptions : u32, perrorcode : *mut UErrorCode) -> u32);
    unsafe { ubiditransform_transform(pbiditransform as _, src, srclength, dest as _, destsize, inparalevel, inorder, outparalevel, outorder, domirroring, shapingoptions, perrorcode as _) }
}
#[inline]
pub unsafe fn ublock_getCode(c: i32) -> UBlockCode {
    windows_core::link!("icuuc.dll" "C" fn ublock_getCode(c : i32) -> UBlockCode);
    unsafe { ublock_getCode(c) }
}
#[inline]
pub unsafe fn ubrk_close(bi: *mut UBreakIterator) {
    windows_core::link!("icuuc.dll" "C" fn ubrk_close(bi : *mut UBreakIterator));
    unsafe { ubrk_close(bi as _) }
}
#[inline]
pub unsafe fn ubrk_countAvailable() -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ubrk_countAvailable() -> i32);
    unsafe { ubrk_countAvailable() }
}
#[inline]
pub unsafe fn ubrk_current(bi: *const UBreakIterator) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ubrk_current(bi : *const UBreakIterator) -> i32);
    unsafe { ubrk_current(bi) }
}
#[inline]
pub unsafe fn ubrk_first(bi: *mut UBreakIterator) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ubrk_first(bi : *mut UBreakIterator) -> i32);
    unsafe { ubrk_first(bi as _) }
}
#[inline]
pub unsafe fn ubrk_following(bi: *mut UBreakIterator, offset: i32) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ubrk_following(bi : *mut UBreakIterator, offset : i32) -> i32);
    unsafe { ubrk_following(bi as _, offset) }
}
#[inline]
pub unsafe fn ubrk_getAvailable(index: i32) -> windows_core::PCSTR {
    windows_core::link!("icuuc.dll" "C" fn ubrk_getAvailable(index : i32) -> windows_core::PCSTR);
    unsafe { ubrk_getAvailable(index) }
}
#[inline]
pub unsafe fn ubrk_getBinaryRules(bi: *mut UBreakIterator, binaryrules: *mut u8, rulescapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ubrk_getBinaryRules(bi : *mut UBreakIterator, binaryrules : *mut u8, rulescapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { ubrk_getBinaryRules(bi as _, binaryrules as _, rulescapacity, status as _) }
}
#[inline]
pub unsafe fn ubrk_getLocaleByType(bi: *const UBreakIterator, r#type: ULocDataLocaleType, status: *mut UErrorCode) -> windows_core::PCSTR {
    windows_core::link!("icuuc.dll" "C" fn ubrk_getLocaleByType(bi : *const UBreakIterator, r#type : ULocDataLocaleType, status : *mut UErrorCode) -> windows_core::PCSTR);
    unsafe { ubrk_getLocaleByType(bi, r#type, status as _) }
}
#[inline]
pub unsafe fn ubrk_getRuleStatus(bi: *mut UBreakIterator) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ubrk_getRuleStatus(bi : *mut UBreakIterator) -> i32);
    unsafe { ubrk_getRuleStatus(bi as _) }
}
#[inline]
pub unsafe fn ubrk_getRuleStatusVec(bi: *mut UBreakIterator, fillinvec: *mut i32, capacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ubrk_getRuleStatusVec(bi : *mut UBreakIterator, fillinvec : *mut i32, capacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { ubrk_getRuleStatusVec(bi as _, fillinvec as _, capacity, status as _) }
}
#[inline]
pub unsafe fn ubrk_isBoundary(bi: *mut UBreakIterator, offset: i32) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn ubrk_isBoundary(bi : *mut UBreakIterator, offset : i32) -> i8);
    unsafe { ubrk_isBoundary(bi as _, offset) }
}
#[inline]
pub unsafe fn ubrk_last(bi: *mut UBreakIterator) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ubrk_last(bi : *mut UBreakIterator) -> i32);
    unsafe { ubrk_last(bi as _) }
}
#[inline]
pub unsafe fn ubrk_next(bi: *mut UBreakIterator) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ubrk_next(bi : *mut UBreakIterator) -> i32);
    unsafe { ubrk_next(bi as _) }
}
#[inline]
pub unsafe fn ubrk_open<P1>(r#type: UBreakIteratorType, locale: P1, text: *const u16, textlength: i32, status: *mut UErrorCode) -> *mut UBreakIterator
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn ubrk_open(r#type : UBreakIteratorType, locale : windows_core::PCSTR, text : *const u16, textlength : i32, status : *mut UErrorCode) -> *mut UBreakIterator);
    unsafe { ubrk_open(r#type, locale.param().abi(), text, textlength, status as _) }
}
#[inline]
pub unsafe fn ubrk_openBinaryRules(binaryrules: *const u8, ruleslength: i32, text: *const u16, textlength: i32, status: *mut UErrorCode) -> *mut UBreakIterator {
    windows_core::link!("icuuc.dll" "C" fn ubrk_openBinaryRules(binaryrules : *const u8, ruleslength : i32, text : *const u16, textlength : i32, status : *mut UErrorCode) -> *mut UBreakIterator);
    unsafe { ubrk_openBinaryRules(binaryrules, ruleslength, text, textlength, status as _) }
}
#[inline]
pub unsafe fn ubrk_openRules(rules: *const u16, ruleslength: i32, text: *const u16, textlength: i32, parseerr: *mut UParseError, status: *mut UErrorCode) -> *mut UBreakIterator {
    windows_core::link!("icuuc.dll" "C" fn ubrk_openRules(rules : *const u16, ruleslength : i32, text : *const u16, textlength : i32, parseerr : *mut UParseError, status : *mut UErrorCode) -> *mut UBreakIterator);
    unsafe { ubrk_openRules(rules, ruleslength, text, textlength, parseerr as _, status as _) }
}
#[inline]
pub unsafe fn ubrk_preceding(bi: *mut UBreakIterator, offset: i32) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ubrk_preceding(bi : *mut UBreakIterator, offset : i32) -> i32);
    unsafe { ubrk_preceding(bi as _, offset) }
}
#[inline]
pub unsafe fn ubrk_previous(bi: *mut UBreakIterator) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ubrk_previous(bi : *mut UBreakIterator) -> i32);
    unsafe { ubrk_previous(bi as _) }
}
#[inline]
pub unsafe fn ubrk_refreshUText(bi: *mut UBreakIterator, text: *mut UText, status: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn ubrk_refreshUText(bi : *mut UBreakIterator, text : *mut UText, status : *mut UErrorCode));
    unsafe { ubrk_refreshUText(bi as _, text as _, status as _) }
}
#[inline]
pub unsafe fn ubrk_safeClone(bi: *const UBreakIterator, stackbuffer: *mut core::ffi::c_void, pbuffersize: *mut i32, status: *mut UErrorCode) -> *mut UBreakIterator {
    windows_core::link!("icuuc.dll" "C" fn ubrk_safeClone(bi : *const UBreakIterator, stackbuffer : *mut core::ffi::c_void, pbuffersize : *mut i32, status : *mut UErrorCode) -> *mut UBreakIterator);
    unsafe { ubrk_safeClone(bi, stackbuffer as _, pbuffersize as _, status as _) }
}
#[inline]
pub unsafe fn ubrk_setText(bi: *mut UBreakIterator, text: *const u16, textlength: i32, status: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn ubrk_setText(bi : *mut UBreakIterator, text : *const u16, textlength : i32, status : *mut UErrorCode));
    unsafe { ubrk_setText(bi as _, text, textlength, status as _) }
}
#[inline]
pub unsafe fn ubrk_setUText(bi: *mut UBreakIterator, text: *mut UText, status: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn ubrk_setUText(bi : *mut UBreakIterator, text : *mut UText, status : *mut UErrorCode));
    unsafe { ubrk_setUText(bi as _, text as _, status as _) }
}
#[inline]
pub unsafe fn ucal_add(cal: *mut *mut core::ffi::c_void, field: UCalendarDateFields, amount: i32, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn ucal_add(cal : *mut *mut core::ffi::c_void, field : UCalendarDateFields, amount : i32, status : *mut UErrorCode));
    unsafe { ucal_add(cal as _, field, amount, status as _) }
}
#[inline]
pub unsafe fn ucal_clear(calendar: *mut *mut core::ffi::c_void) {
    windows_core::link!("icuin.dll" "C" fn ucal_clear(calendar : *mut *mut core::ffi::c_void));
    unsafe { ucal_clear(calendar as _) }
}
#[inline]
pub unsafe fn ucal_clearField(cal: *mut *mut core::ffi::c_void, field: UCalendarDateFields) {
    windows_core::link!("icuin.dll" "C" fn ucal_clearField(cal : *mut *mut core::ffi::c_void, field : UCalendarDateFields));
    unsafe { ucal_clearField(cal as _, field) }
}
#[inline]
pub unsafe fn ucal_clone(cal: *const *const core::ffi::c_void, status: *mut UErrorCode) -> *mut *mut core::ffi::c_void {
    windows_core::link!("icuin.dll" "C" fn ucal_clone(cal : *const *const core::ffi::c_void, status : *mut UErrorCode) -> *mut *mut core::ffi::c_void);
    unsafe { ucal_clone(cal, status as _) }
}
#[inline]
pub unsafe fn ucal_close(cal: *mut *mut core::ffi::c_void) {
    windows_core::link!("icuin.dll" "C" fn ucal_close(cal : *mut *mut core::ffi::c_void));
    unsafe { ucal_close(cal as _) }
}
#[inline]
pub unsafe fn ucal_countAvailable() -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucal_countAvailable() -> i32);
    unsafe { ucal_countAvailable() }
}
#[inline]
pub unsafe fn ucal_equivalentTo(cal1: *const *const core::ffi::c_void, cal2: *const *const core::ffi::c_void) -> i8 {
    windows_core::link!("icuin.dll" "C" fn ucal_equivalentTo(cal1 : *const *const core::ffi::c_void, cal2 : *const *const core::ffi::c_void) -> i8);
    unsafe { ucal_equivalentTo(cal1, cal2) }
}
#[inline]
pub unsafe fn ucal_get(cal: *const *const core::ffi::c_void, field: UCalendarDateFields, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucal_get(cal : *const *const core::ffi::c_void, field : UCalendarDateFields, status : *mut UErrorCode) -> i32);
    unsafe { ucal_get(cal, field, status as _) }
}
#[inline]
pub unsafe fn ucal_getAttribute(cal: *const *const core::ffi::c_void, attr: UCalendarAttribute) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucal_getAttribute(cal : *const *const core::ffi::c_void, attr : UCalendarAttribute) -> i32);
    unsafe { ucal_getAttribute(cal, attr) }
}
#[inline]
pub unsafe fn ucal_getAvailable(localeindex: i32) -> windows_core::PCSTR {
    windows_core::link!("icuin.dll" "C" fn ucal_getAvailable(localeindex : i32) -> windows_core::PCSTR);
    unsafe { ucal_getAvailable(localeindex) }
}
#[inline]
pub unsafe fn ucal_getCanonicalTimeZoneID(id: *const u16, len: i32, result: *mut u16, resultcapacity: i32, issystemid: *mut i8, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucal_getCanonicalTimeZoneID(id : *const u16, len : i32, result : *mut u16, resultcapacity : i32, issystemid : *mut i8, status : *mut UErrorCode) -> i32);
    unsafe { ucal_getCanonicalTimeZoneID(id, len, result as _, resultcapacity, issystemid as _, status as _) }
}
#[inline]
pub unsafe fn ucal_getDSTSavings(zoneid: *const u16, ec: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucal_getDSTSavings(zoneid : *const u16, ec : *mut UErrorCode) -> i32);
    unsafe { ucal_getDSTSavings(zoneid, ec as _) }
}
#[inline]
pub unsafe fn ucal_getDayOfWeekType(cal: *const *const core::ffi::c_void, dayofweek: UCalendarDaysOfWeek, status: *mut UErrorCode) -> UCalendarWeekdayType {
    windows_core::link!("icuin.dll" "C" fn ucal_getDayOfWeekType(cal : *const *const core::ffi::c_void, dayofweek : UCalendarDaysOfWeek, status : *mut UErrorCode) -> UCalendarWeekdayType);
    unsafe { ucal_getDayOfWeekType(cal, dayofweek, status as _) }
}
#[inline]
pub unsafe fn ucal_getDefaultTimeZone(result: *mut u16, resultcapacity: i32, ec: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucal_getDefaultTimeZone(result : *mut u16, resultcapacity : i32, ec : *mut UErrorCode) -> i32);
    unsafe { ucal_getDefaultTimeZone(result as _, resultcapacity, ec as _) }
}
#[inline]
pub unsafe fn ucal_getFieldDifference(cal: *mut *mut core::ffi::c_void, target: f64, field: UCalendarDateFields, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucal_getFieldDifference(cal : *mut *mut core::ffi::c_void, target : f64, field : UCalendarDateFields, status : *mut UErrorCode) -> i32);
    unsafe { ucal_getFieldDifference(cal as _, target, field, status as _) }
}
#[inline]
pub unsafe fn ucal_getGregorianChange(cal: *const *const core::ffi::c_void, perrorcode: *mut UErrorCode) -> f64 {
    windows_core::link!("icuin.dll" "C" fn ucal_getGregorianChange(cal : *const *const core::ffi::c_void, perrorcode : *mut UErrorCode) -> f64);
    unsafe { ucal_getGregorianChange(cal, perrorcode as _) }
}
#[inline]
pub unsafe fn ucal_getHostTimeZone(result: *mut u16, resultcapacity: i32, ec: *mut UErrorCode) -> i32 {
    windows_core::link!("icu.dll" "C" fn ucal_getHostTimeZone(result : *mut u16, resultcapacity : i32, ec : *mut UErrorCode) -> i32);
    unsafe { ucal_getHostTimeZone(result as _, resultcapacity, ec as _) }
}
#[inline]
pub unsafe fn ucal_getKeywordValuesForLocale<P0, P1>(key: P0, locale: P1, commonlyused: i8, status: *mut UErrorCode) -> *mut UEnumeration
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuin.dll" "C" fn ucal_getKeywordValuesForLocale(key : windows_core::PCSTR, locale : windows_core::PCSTR, commonlyused : i8, status : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { ucal_getKeywordValuesForLocale(key.param().abi(), locale.param().abi(), commonlyused, status as _) }
}
#[inline]
pub unsafe fn ucal_getLimit(cal: *const *const core::ffi::c_void, field: UCalendarDateFields, r#type: UCalendarLimitType, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucal_getLimit(cal : *const *const core::ffi::c_void, field : UCalendarDateFields, r#type : UCalendarLimitType, status : *mut UErrorCode) -> i32);
    unsafe { ucal_getLimit(cal, field, r#type, status as _) }
}
#[inline]
pub unsafe fn ucal_getLocaleByType(cal: *const *const core::ffi::c_void, r#type: ULocDataLocaleType, status: *mut UErrorCode) -> windows_core::PCSTR {
    windows_core::link!("icuin.dll" "C" fn ucal_getLocaleByType(cal : *const *const core::ffi::c_void, r#type : ULocDataLocaleType, status : *mut UErrorCode) -> windows_core::PCSTR);
    unsafe { ucal_getLocaleByType(cal, r#type, status as _) }
}
#[inline]
pub unsafe fn ucal_getMillis(cal: *const *const core::ffi::c_void, status: *mut UErrorCode) -> f64 {
    windows_core::link!("icuin.dll" "C" fn ucal_getMillis(cal : *const *const core::ffi::c_void, status : *mut UErrorCode) -> f64);
    unsafe { ucal_getMillis(cal, status as _) }
}
#[inline]
pub unsafe fn ucal_getNow() -> f64 {
    windows_core::link!("icuin.dll" "C" fn ucal_getNow() -> f64);
    unsafe { ucal_getNow() }
}
#[inline]
pub unsafe fn ucal_getTZDataVersion(status: *mut UErrorCode) -> windows_core::PCSTR {
    windows_core::link!("icuin.dll" "C" fn ucal_getTZDataVersion(status : *mut UErrorCode) -> windows_core::PCSTR);
    unsafe { ucal_getTZDataVersion(status as _) }
}
#[inline]
pub unsafe fn ucal_getTimeZoneDisplayName<P2>(cal: *const *const core::ffi::c_void, r#type: UCalendarDisplayNameType, locale: P2, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuin.dll" "C" fn ucal_getTimeZoneDisplayName(cal : *const *const core::ffi::c_void, r#type : UCalendarDisplayNameType, locale : windows_core::PCSTR, result : *mut u16, resultlength : i32, status : *mut UErrorCode) -> i32);
    unsafe { ucal_getTimeZoneDisplayName(cal, r#type, locale.param().abi(), result as _, resultlength, status as _) }
}
#[inline]
pub unsafe fn ucal_getTimeZoneID(cal: *const *const core::ffi::c_void, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucal_getTimeZoneID(cal : *const *const core::ffi::c_void, result : *mut u16, resultlength : i32, status : *mut UErrorCode) -> i32);
    unsafe { ucal_getTimeZoneID(cal, result as _, resultlength, status as _) }
}
#[inline]
pub unsafe fn ucal_getTimeZoneIDForWindowsID<P2>(winid: *const u16, len: i32, region: P2, id: *mut u16, idcapacity: i32, status: *mut UErrorCode) -> i32
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuin.dll" "C" fn ucal_getTimeZoneIDForWindowsID(winid : *const u16, len : i32, region : windows_core::PCSTR, id : *mut u16, idcapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { ucal_getTimeZoneIDForWindowsID(winid, len, region.param().abi(), id as _, idcapacity, status as _) }
}
#[inline]
pub unsafe fn ucal_getTimeZoneTransitionDate(cal: *const *const core::ffi::c_void, r#type: UTimeZoneTransitionType, transition: *mut f64, status: *mut UErrorCode) -> i8 {
    windows_core::link!("icuin.dll" "C" fn ucal_getTimeZoneTransitionDate(cal : *const *const core::ffi::c_void, r#type : UTimeZoneTransitionType, transition : *mut f64, status : *mut UErrorCode) -> i8);
    unsafe { ucal_getTimeZoneTransitionDate(cal, r#type, transition as _, status as _) }
}
#[inline]
pub unsafe fn ucal_getType(cal: *const *const core::ffi::c_void, status: *mut UErrorCode) -> windows_core::PCSTR {
    windows_core::link!("icuin.dll" "C" fn ucal_getType(cal : *const *const core::ffi::c_void, status : *mut UErrorCode) -> windows_core::PCSTR);
    unsafe { ucal_getType(cal, status as _) }
}
#[inline]
pub unsafe fn ucal_getWeekendTransition(cal: *const *const core::ffi::c_void, dayofweek: UCalendarDaysOfWeek, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucal_getWeekendTransition(cal : *const *const core::ffi::c_void, dayofweek : UCalendarDaysOfWeek, status : *mut UErrorCode) -> i32);
    unsafe { ucal_getWeekendTransition(cal, dayofweek, status as _) }
}
#[inline]
pub unsafe fn ucal_getWindowsTimeZoneID(id: *const u16, len: i32, winid: *mut u16, winidcapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucal_getWindowsTimeZoneID(id : *const u16, len : i32, winid : *mut u16, winidcapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { ucal_getWindowsTimeZoneID(id, len, winid as _, winidcapacity, status as _) }
}
#[inline]
pub unsafe fn ucal_inDaylightTime(cal: *const *const core::ffi::c_void, status: *mut UErrorCode) -> i8 {
    windows_core::link!("icuin.dll" "C" fn ucal_inDaylightTime(cal : *const *const core::ffi::c_void, status : *mut UErrorCode) -> i8);
    unsafe { ucal_inDaylightTime(cal, status as _) }
}
#[inline]
pub unsafe fn ucal_isSet(cal: *const *const core::ffi::c_void, field: UCalendarDateFields) -> i8 {
    windows_core::link!("icuin.dll" "C" fn ucal_isSet(cal : *const *const core::ffi::c_void, field : UCalendarDateFields) -> i8);
    unsafe { ucal_isSet(cal, field) }
}
#[inline]
pub unsafe fn ucal_isWeekend(cal: *const *const core::ffi::c_void, date: f64, status: *mut UErrorCode) -> i8 {
    windows_core::link!("icuin.dll" "C" fn ucal_isWeekend(cal : *const *const core::ffi::c_void, date : f64, status : *mut UErrorCode) -> i8);
    unsafe { ucal_isWeekend(cal, date, status as _) }
}
#[inline]
pub unsafe fn ucal_open<P2>(zoneid: *const u16, len: i32, locale: P2, r#type: UCalendarType, status: *mut UErrorCode) -> *mut *mut core::ffi::c_void
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuin.dll" "C" fn ucal_open(zoneid : *const u16, len : i32, locale : windows_core::PCSTR, r#type : UCalendarType, status : *mut UErrorCode) -> *mut *mut core::ffi::c_void);
    unsafe { ucal_open(zoneid, len, locale.param().abi(), r#type, status as _) }
}
#[inline]
pub unsafe fn ucal_openCountryTimeZones<P0>(country: P0, ec: *mut UErrorCode) -> *mut UEnumeration
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuin.dll" "C" fn ucal_openCountryTimeZones(country : windows_core::PCSTR, ec : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { ucal_openCountryTimeZones(country.param().abi(), ec as _) }
}
#[inline]
pub unsafe fn ucal_openTimeZoneIDEnumeration<P1>(zonetype: USystemTimeZoneType, region: P1, rawoffset: *const i32, ec: *mut UErrorCode) -> *mut UEnumeration
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuin.dll" "C" fn ucal_openTimeZoneIDEnumeration(zonetype : USystemTimeZoneType, region : windows_core::PCSTR, rawoffset : *const i32, ec : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { ucal_openTimeZoneIDEnumeration(zonetype, region.param().abi(), rawoffset, ec as _) }
}
#[inline]
pub unsafe fn ucal_openTimeZones(ec: *mut UErrorCode) -> *mut UEnumeration {
    windows_core::link!("icuin.dll" "C" fn ucal_openTimeZones(ec : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { ucal_openTimeZones(ec as _) }
}
#[inline]
pub unsafe fn ucal_roll(cal: *mut *mut core::ffi::c_void, field: UCalendarDateFields, amount: i32, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn ucal_roll(cal : *mut *mut core::ffi::c_void, field : UCalendarDateFields, amount : i32, status : *mut UErrorCode));
    unsafe { ucal_roll(cal as _, field, amount, status as _) }
}
#[inline]
pub unsafe fn ucal_set(cal: *mut *mut core::ffi::c_void, field: UCalendarDateFields, value: i32) {
    windows_core::link!("icuin.dll" "C" fn ucal_set(cal : *mut *mut core::ffi::c_void, field : UCalendarDateFields, value : i32));
    unsafe { ucal_set(cal as _, field, value) }
}
#[inline]
pub unsafe fn ucal_setAttribute(cal: *mut *mut core::ffi::c_void, attr: UCalendarAttribute, newvalue: i32) {
    windows_core::link!("icuin.dll" "C" fn ucal_setAttribute(cal : *mut *mut core::ffi::c_void, attr : UCalendarAttribute, newvalue : i32));
    unsafe { ucal_setAttribute(cal as _, attr, newvalue) }
}
#[inline]
pub unsafe fn ucal_setDate(cal: *mut *mut core::ffi::c_void, year: i32, month: i32, date: i32, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn ucal_setDate(cal : *mut *mut core::ffi::c_void, year : i32, month : i32, date : i32, status : *mut UErrorCode));
    unsafe { ucal_setDate(cal as _, year, month, date, status as _) }
}
#[inline]
pub unsafe fn ucal_setDateTime(cal: *mut *mut core::ffi::c_void, year: i32, month: i32, date: i32, hour: i32, minute: i32, second: i32, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn ucal_setDateTime(cal : *mut *mut core::ffi::c_void, year : i32, month : i32, date : i32, hour : i32, minute : i32, second : i32, status : *mut UErrorCode));
    unsafe { ucal_setDateTime(cal as _, year, month, date, hour, minute, second, status as _) }
}
#[inline]
pub unsafe fn ucal_setDefaultTimeZone(zoneid: *const u16, ec: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn ucal_setDefaultTimeZone(zoneid : *const u16, ec : *mut UErrorCode));
    unsafe { ucal_setDefaultTimeZone(zoneid, ec as _) }
}
#[inline]
pub unsafe fn ucal_setGregorianChange(cal: *mut *mut core::ffi::c_void, date: f64, perrorcode: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn ucal_setGregorianChange(cal : *mut *mut core::ffi::c_void, date : f64, perrorcode : *mut UErrorCode));
    unsafe { ucal_setGregorianChange(cal as _, date, perrorcode as _) }
}
#[inline]
pub unsafe fn ucal_setMillis(cal: *mut *mut core::ffi::c_void, datetime: f64, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn ucal_setMillis(cal : *mut *mut core::ffi::c_void, datetime : f64, status : *mut UErrorCode));
    unsafe { ucal_setMillis(cal as _, datetime, status as _) }
}
#[inline]
pub unsafe fn ucal_setTimeZone(cal: *mut *mut core::ffi::c_void, zoneid: *const u16, len: i32, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn ucal_setTimeZone(cal : *mut *mut core::ffi::c_void, zoneid : *const u16, len : i32, status : *mut UErrorCode));
    unsafe { ucal_setTimeZone(cal as _, zoneid, len, status as _) }
}
#[inline]
pub unsafe fn ucasemap_close(csm: *mut UCaseMap) {
    windows_core::link!("icuuc.dll" "C" fn ucasemap_close(csm : *mut UCaseMap));
    unsafe { ucasemap_close(csm as _) }
}
#[inline]
pub unsafe fn ucasemap_getBreakIterator(csm: *const UCaseMap) -> *mut UBreakIterator {
    windows_core::link!("icuuc.dll" "C" fn ucasemap_getBreakIterator(csm : *const UCaseMap) -> *mut UBreakIterator);
    unsafe { ucasemap_getBreakIterator(csm) }
}
#[inline]
pub unsafe fn ucasemap_getLocale(csm: *const UCaseMap) -> windows_core::PCSTR {
    windows_core::link!("icuuc.dll" "C" fn ucasemap_getLocale(csm : *const UCaseMap) -> windows_core::PCSTR);
    unsafe { ucasemap_getLocale(csm) }
}
#[inline]
pub unsafe fn ucasemap_getOptions(csm: *const UCaseMap) -> u32 {
    windows_core::link!("icuuc.dll" "C" fn ucasemap_getOptions(csm : *const UCaseMap) -> u32);
    unsafe { ucasemap_getOptions(csm) }
}
#[inline]
pub unsafe fn ucasemap_open<P0>(locale: P0, options: u32, perrorcode: *mut UErrorCode) -> *mut UCaseMap
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn ucasemap_open(locale : windows_core::PCSTR, options : u32, perrorcode : *mut UErrorCode) -> *mut UCaseMap);
    unsafe { ucasemap_open(locale.param().abi(), options, perrorcode as _) }
}
#[inline]
pub unsafe fn ucasemap_setBreakIterator(csm: *mut UCaseMap, itertoadopt: *mut UBreakIterator, perrorcode: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn ucasemap_setBreakIterator(csm : *mut UCaseMap, itertoadopt : *mut UBreakIterator, perrorcode : *mut UErrorCode));
    unsafe { ucasemap_setBreakIterator(csm as _, itertoadopt as _, perrorcode as _) }
}
#[inline]
pub unsafe fn ucasemap_setLocale<P1>(csm: *mut UCaseMap, locale: P1, perrorcode: *mut UErrorCode)
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn ucasemap_setLocale(csm : *mut UCaseMap, locale : windows_core::PCSTR, perrorcode : *mut UErrorCode));
    unsafe { ucasemap_setLocale(csm as _, locale.param().abi(), perrorcode as _) }
}
#[inline]
pub unsafe fn ucasemap_setOptions(csm: *mut UCaseMap, options: u32, perrorcode: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn ucasemap_setOptions(csm : *mut UCaseMap, options : u32, perrorcode : *mut UErrorCode));
    unsafe { ucasemap_setOptions(csm as _, options, perrorcode as _) }
}
#[inline]
pub unsafe fn ucasemap_toTitle(csm: *mut UCaseMap, dest: *mut u16, destcapacity: i32, src: *const u16, srclength: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ucasemap_toTitle(csm : *mut UCaseMap, dest : *mut u16, destcapacity : i32, src : *const u16, srclength : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { ucasemap_toTitle(csm as _, dest as _, destcapacity, src, srclength, perrorcode as _) }
}
#[inline]
pub unsafe fn ucasemap_utf8FoldCase<P1, P3>(csm: *const UCaseMap, dest: P1, destcapacity: i32, src: P3, srclength: i32, perrorcode: *mut UErrorCode) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn ucasemap_utf8FoldCase(csm : *const UCaseMap, dest : windows_core::PCSTR, destcapacity : i32, src : windows_core::PCSTR, srclength : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { ucasemap_utf8FoldCase(csm, dest.param().abi(), destcapacity, src.param().abi(), srclength, perrorcode as _) }
}
#[inline]
pub unsafe fn ucasemap_utf8ToLower<P1, P3>(csm: *const UCaseMap, dest: P1, destcapacity: i32, src: P3, srclength: i32, perrorcode: *mut UErrorCode) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn ucasemap_utf8ToLower(csm : *const UCaseMap, dest : windows_core::PCSTR, destcapacity : i32, src : windows_core::PCSTR, srclength : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { ucasemap_utf8ToLower(csm, dest.param().abi(), destcapacity, src.param().abi(), srclength, perrorcode as _) }
}
#[inline]
pub unsafe fn ucasemap_utf8ToTitle<P1, P3>(csm: *mut UCaseMap, dest: P1, destcapacity: i32, src: P3, srclength: i32, perrorcode: *mut UErrorCode) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn ucasemap_utf8ToTitle(csm : *mut UCaseMap, dest : windows_core::PCSTR, destcapacity : i32, src : windows_core::PCSTR, srclength : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { ucasemap_utf8ToTitle(csm as _, dest.param().abi(), destcapacity, src.param().abi(), srclength, perrorcode as _) }
}
#[inline]
pub unsafe fn ucasemap_utf8ToUpper<P1, P3>(csm: *const UCaseMap, dest: P1, destcapacity: i32, src: P3, srclength: i32, perrorcode: *mut UErrorCode) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn ucasemap_utf8ToUpper(csm : *const UCaseMap, dest : windows_core::PCSTR, destcapacity : i32, src : windows_core::PCSTR, srclength : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { ucasemap_utf8ToUpper(csm, dest.param().abi(), destcapacity, src.param().abi(), srclength, perrorcode as _) }
}
#[inline]
pub unsafe fn ucfpos_close(ucfpos: *mut UConstrainedFieldPosition) {
    windows_core::link!("icu.dll" "C" fn ucfpos_close(ucfpos : *mut UConstrainedFieldPosition));
    unsafe { ucfpos_close(ucfpos as _) }
}
#[inline]
pub unsafe fn ucfpos_constrainCategory(ucfpos: *mut UConstrainedFieldPosition, category: i32, ec: *mut UErrorCode) {
    windows_core::link!("icu.dll" "C" fn ucfpos_constrainCategory(ucfpos : *mut UConstrainedFieldPosition, category : i32, ec : *mut UErrorCode));
    unsafe { ucfpos_constrainCategory(ucfpos as _, category, ec as _) }
}
#[inline]
pub unsafe fn ucfpos_constrainField(ucfpos: *mut UConstrainedFieldPosition, category: i32, field: i32, ec: *mut UErrorCode) {
    windows_core::link!("icu.dll" "C" fn ucfpos_constrainField(ucfpos : *mut UConstrainedFieldPosition, category : i32, field : i32, ec : *mut UErrorCode));
    unsafe { ucfpos_constrainField(ucfpos as _, category, field, ec as _) }
}
#[inline]
pub unsafe fn ucfpos_getCategory(ucfpos: *const UConstrainedFieldPosition, ec: *mut UErrorCode) -> i32 {
    windows_core::link!("icu.dll" "C" fn ucfpos_getCategory(ucfpos : *const UConstrainedFieldPosition, ec : *mut UErrorCode) -> i32);
    unsafe { ucfpos_getCategory(ucfpos, ec as _) }
}
#[inline]
pub unsafe fn ucfpos_getField(ucfpos: *const UConstrainedFieldPosition, ec: *mut UErrorCode) -> i32 {
    windows_core::link!("icu.dll" "C" fn ucfpos_getField(ucfpos : *const UConstrainedFieldPosition, ec : *mut UErrorCode) -> i32);
    unsafe { ucfpos_getField(ucfpos, ec as _) }
}
#[inline]
pub unsafe fn ucfpos_getIndexes(ucfpos: *const UConstrainedFieldPosition, pstart: *mut i32, plimit: *mut i32, ec: *mut UErrorCode) {
    windows_core::link!("icu.dll" "C" fn ucfpos_getIndexes(ucfpos : *const UConstrainedFieldPosition, pstart : *mut i32, plimit : *mut i32, ec : *mut UErrorCode));
    unsafe { ucfpos_getIndexes(ucfpos, pstart as _, plimit as _, ec as _) }
}
#[inline]
pub unsafe fn ucfpos_getInt64IterationContext(ucfpos: *const UConstrainedFieldPosition, ec: *mut UErrorCode) -> i64 {
    windows_core::link!("icu.dll" "C" fn ucfpos_getInt64IterationContext(ucfpos : *const UConstrainedFieldPosition, ec : *mut UErrorCode) -> i64);
    unsafe { ucfpos_getInt64IterationContext(ucfpos, ec as _) }
}
#[inline]
pub unsafe fn ucfpos_matchesField(ucfpos: *const UConstrainedFieldPosition, category: i32, field: i32, ec: *mut UErrorCode) -> i8 {
    windows_core::link!("icu.dll" "C" fn ucfpos_matchesField(ucfpos : *const UConstrainedFieldPosition, category : i32, field : i32, ec : *mut UErrorCode) -> i8);
    unsafe { ucfpos_matchesField(ucfpos, category, field, ec as _) }
}
#[inline]
pub unsafe fn ucfpos_open(ec: *mut UErrorCode) -> *mut UConstrainedFieldPosition {
    windows_core::link!("icu.dll" "C" fn ucfpos_open(ec : *mut UErrorCode) -> *mut UConstrainedFieldPosition);
    unsafe { ucfpos_open(ec as _) }
}
#[inline]
pub unsafe fn ucfpos_reset(ucfpos: *mut UConstrainedFieldPosition, ec: *mut UErrorCode) {
    windows_core::link!("icu.dll" "C" fn ucfpos_reset(ucfpos : *mut UConstrainedFieldPosition, ec : *mut UErrorCode));
    unsafe { ucfpos_reset(ucfpos as _, ec as _) }
}
#[inline]
pub unsafe fn ucfpos_setInt64IterationContext(ucfpos: *mut UConstrainedFieldPosition, context: i64, ec: *mut UErrorCode) {
    windows_core::link!("icu.dll" "C" fn ucfpos_setInt64IterationContext(ucfpos : *mut UConstrainedFieldPosition, context : i64, ec : *mut UErrorCode));
    unsafe { ucfpos_setInt64IterationContext(ucfpos as _, context, ec as _) }
}
#[inline]
pub unsafe fn ucfpos_setState(ucfpos: *mut UConstrainedFieldPosition, category: i32, field: i32, start: i32, limit: i32, ec: *mut UErrorCode) {
    windows_core::link!("icu.dll" "C" fn ucfpos_setState(ucfpos : *mut UConstrainedFieldPosition, category : i32, field : i32, start : i32, limit : i32, ec : *mut UErrorCode));
    unsafe { ucfpos_setState(ucfpos as _, category, field, start, limit, ec as _) }
}
#[inline]
pub unsafe fn ucnv_cbFromUWriteBytes<P1>(args: *mut UConverterFromUnicodeArgs, source: P1, length: i32, offsetindex: i32, err: *mut UErrorCode)
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn ucnv_cbFromUWriteBytes(args : *mut UConverterFromUnicodeArgs, source : windows_core::PCSTR, length : i32, offsetindex : i32, err : *mut UErrorCode));
    unsafe { ucnv_cbFromUWriteBytes(args as _, source.param().abi(), length, offsetindex, err as _) }
}
#[inline]
pub unsafe fn ucnv_cbFromUWriteSub(args: *mut UConverterFromUnicodeArgs, offsetindex: i32, err: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn ucnv_cbFromUWriteSub(args : *mut UConverterFromUnicodeArgs, offsetindex : i32, err : *mut UErrorCode));
    unsafe { ucnv_cbFromUWriteSub(args as _, offsetindex, err as _) }
}
#[inline]
pub unsafe fn ucnv_cbFromUWriteUChars(args: *mut UConverterFromUnicodeArgs, source: *const *const u16, sourcelimit: *const u16, offsetindex: i32, err: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn ucnv_cbFromUWriteUChars(args : *mut UConverterFromUnicodeArgs, source : *const *const u16, sourcelimit : *const u16, offsetindex : i32, err : *mut UErrorCode));
    unsafe { ucnv_cbFromUWriteUChars(args as _, source, sourcelimit, offsetindex, err as _) }
}
#[inline]
pub unsafe fn ucnv_cbToUWriteSub(args: *mut UConverterToUnicodeArgs, offsetindex: i32, err: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn ucnv_cbToUWriteSub(args : *mut UConverterToUnicodeArgs, offsetindex : i32, err : *mut UErrorCode));
    unsafe { ucnv_cbToUWriteSub(args as _, offsetindex, err as _) }
}
#[inline]
pub unsafe fn ucnv_cbToUWriteUChars(args: *mut UConverterToUnicodeArgs, source: *const u16, length: i32, offsetindex: i32, err: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn ucnv_cbToUWriteUChars(args : *mut UConverterToUnicodeArgs, source : *const u16, length : i32, offsetindex : i32, err : *mut UErrorCode));
    unsafe { ucnv_cbToUWriteUChars(args as _, source, length, offsetindex, err as _) }
}
#[inline]
pub unsafe fn ucnv_close(converter: *mut UConverter) {
    windows_core::link!("icuuc.dll" "C" fn ucnv_close(converter : *mut UConverter));
    unsafe { ucnv_close(converter as _) }
}
#[inline]
pub unsafe fn ucnv_compareNames<P0, P1>(name1: P0, name2: P1) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn ucnv_compareNames(name1 : windows_core::PCSTR, name2 : windows_core::PCSTR) -> i32);
    unsafe { ucnv_compareNames(name1.param().abi(), name2.param().abi()) }
}
#[inline]
pub unsafe fn ucnv_convert<P0, P1, P2, P4>(toconvertername: P0, fromconvertername: P1, target: P2, targetcapacity: i32, source: P4, sourcelength: i32, perrorcode: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn ucnv_convert(toconvertername : windows_core::PCSTR, fromconvertername : windows_core::PCSTR, target : windows_core::PCSTR, targetcapacity : i32, source : windows_core::PCSTR, sourcelength : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { ucnv_convert(toconvertername.param().abi(), fromconvertername.param().abi(), target.param().abi(), targetcapacity, source.param().abi(), sourcelength, perrorcode as _) }
}
#[inline]
pub unsafe fn ucnv_convertEx<P3, P5>(targetcnv: *mut UConverter, sourcecnv: *mut UConverter, target: *mut *mut i8, targetlimit: P3, source: *const *const i8, sourcelimit: P5, pivotstart: *mut u16, pivotsource: *mut *mut u16, pivottarget: *mut *mut u16, pivotlimit: *const u16, reset: i8, flush: i8, perrorcode: *mut UErrorCode)
where
    P3: windows_core::Param<windows_core::PCSTR>,
    P5: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn ucnv_convertEx(targetcnv : *mut UConverter, sourcecnv : *mut UConverter, target : *mut *mut i8, targetlimit : windows_core::PCSTR, source : *const *const i8, sourcelimit : windows_core::PCSTR, pivotstart : *mut u16, pivotsource : *mut *mut u16, pivottarget : *mut *mut u16, pivotlimit : *const u16, reset : i8, flush : i8, perrorcode : *mut UErrorCode));
    unsafe { ucnv_convertEx(targetcnv as _, sourcecnv as _, target as _, targetlimit.param().abi(), source, sourcelimit.param().abi(), pivotstart as _, pivotsource as _, pivottarget as _, pivotlimit, reset, flush, perrorcode as _) }
}
#[inline]
pub unsafe fn ucnv_countAliases<P0>(alias: P0, perrorcode: *mut UErrorCode) -> u16
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn ucnv_countAliases(alias : windows_core::PCSTR, perrorcode : *mut UErrorCode) -> u16);
    unsafe { ucnv_countAliases(alias.param().abi(), perrorcode as _) }
}
#[inline]
pub unsafe fn ucnv_countAvailable() -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ucnv_countAvailable() -> i32);
    unsafe { ucnv_countAvailable() }
}
#[inline]
pub unsafe fn ucnv_countStandards() -> u16 {
    windows_core::link!("icuuc.dll" "C" fn ucnv_countStandards() -> u16);
    unsafe { ucnv_countStandards() }
}
#[inline]
pub unsafe fn ucnv_detectUnicodeSignature<P0>(source: P0, sourcelength: i32, signaturelength: *mut i32, perrorcode: *mut UErrorCode) -> windows_core::PCSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn ucnv_detectUnicodeSignature(source : windows_core::PCSTR, sourcelength : i32, signaturelength : *mut i32, perrorcode : *mut UErrorCode) -> windows_core::PCSTR);
    unsafe { ucnv_detectUnicodeSignature(source.param().abi(), sourcelength, signaturelength as _, perrorcode as _) }
}
#[inline]
pub unsafe fn ucnv_fixFileSeparator(cnv: *const UConverter, source: *mut u16, sourcelen: i32) {
    windows_core::link!("icuuc.dll" "C" fn ucnv_fixFileSeparator(cnv : *const UConverter, source : *mut u16, sourcelen : i32));
    unsafe { ucnv_fixFileSeparator(cnv, source as _, sourcelen) }
}
#[inline]
pub unsafe fn ucnv_flushCache() -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ucnv_flushCache() -> i32);
    unsafe { ucnv_flushCache() }
}
#[inline]
pub unsafe fn ucnv_fromAlgorithmic<P2, P4>(cnv: *mut UConverter, algorithmictype: UConverterType, target: P2, targetcapacity: i32, source: P4, sourcelength: i32, perrorcode: *mut UErrorCode) -> i32
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn ucnv_fromAlgorithmic(cnv : *mut UConverter, algorithmictype : UConverterType, target : windows_core::PCSTR, targetcapacity : i32, source : windows_core::PCSTR, sourcelength : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { ucnv_fromAlgorithmic(cnv as _, algorithmictype, target.param().abi(), targetcapacity, source.param().abi(), sourcelength, perrorcode as _) }
}
#[inline]
pub unsafe fn ucnv_fromUChars<P1>(cnv: *mut UConverter, dest: P1, destcapacity: i32, src: *const u16, srclength: i32, perrorcode: *mut UErrorCode) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn ucnv_fromUChars(cnv : *mut UConverter, dest : windows_core::PCSTR, destcapacity : i32, src : *const u16, srclength : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { ucnv_fromUChars(cnv as _, dest.param().abi(), destcapacity, src, srclength, perrorcode as _) }
}
#[inline]
pub unsafe fn ucnv_fromUCountPending(cnv: *const UConverter, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ucnv_fromUCountPending(cnv : *const UConverter, status : *mut UErrorCode) -> i32);
    unsafe { ucnv_fromUCountPending(cnv, status as _) }
}
#[inline]
pub unsafe fn ucnv_fromUnicode<P2>(converter: *mut UConverter, target: *mut *mut i8, targetlimit: P2, source: *const *const u16, sourcelimit: *const u16, offsets: *mut i32, flush: i8, err: *mut UErrorCode)
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn ucnv_fromUnicode(converter : *mut UConverter, target : *mut *mut i8, targetlimit : windows_core::PCSTR, source : *const *const u16, sourcelimit : *const u16, offsets : *mut i32, flush : i8, err : *mut UErrorCode));
    unsafe { ucnv_fromUnicode(converter as _, target as _, targetlimit.param().abi(), source, sourcelimit, offsets as _, flush, err as _) }
}
#[inline]
pub unsafe fn ucnv_getAlias<P0>(alias: P0, n: u16, perrorcode: *mut UErrorCode) -> windows_core::PCSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn ucnv_getAlias(alias : windows_core::PCSTR, n : u16, perrorcode : *mut UErrorCode) -> windows_core::PCSTR);
    unsafe { ucnv_getAlias(alias.param().abi(), n, perrorcode as _) }
}
#[inline]
pub unsafe fn ucnv_getAliases<P0>(alias: P0, aliases: *const *const i8, perrorcode: *mut UErrorCode)
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn ucnv_getAliases(alias : windows_core::PCSTR, aliases : *const *const i8, perrorcode : *mut UErrorCode));
    unsafe { ucnv_getAliases(alias.param().abi(), aliases, perrorcode as _) }
}
#[inline]
pub unsafe fn ucnv_getAvailableName(n: i32) -> windows_core::PCSTR {
    windows_core::link!("icuuc.dll" "C" fn ucnv_getAvailableName(n : i32) -> windows_core::PCSTR);
    unsafe { ucnv_getAvailableName(n) }
}
#[inline]
pub unsafe fn ucnv_getCCSID(converter: *const UConverter, err: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ucnv_getCCSID(converter : *const UConverter, err : *mut UErrorCode) -> i32);
    unsafe { ucnv_getCCSID(converter, err as _) }
}
#[inline]
pub unsafe fn ucnv_getCanonicalName<P0, P1>(alias: P0, standard: P1, perrorcode: *mut UErrorCode) -> windows_core::PCSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn ucnv_getCanonicalName(alias : windows_core::PCSTR, standard : windows_core::PCSTR, perrorcode : *mut UErrorCode) -> windows_core::PCSTR);
    unsafe { ucnv_getCanonicalName(alias.param().abi(), standard.param().abi(), perrorcode as _) }
}
#[inline]
pub unsafe fn ucnv_getDefaultName() -> windows_core::PCSTR {
    windows_core::link!("icuuc.dll" "C" fn ucnv_getDefaultName() -> windows_core::PCSTR);
    unsafe { ucnv_getDefaultName() }
}
#[inline]
pub unsafe fn ucnv_getDisplayName<P1>(converter: *const UConverter, displaylocale: P1, displayname: *mut u16, displaynamecapacity: i32, err: *mut UErrorCode) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn ucnv_getDisplayName(converter : *const UConverter, displaylocale : windows_core::PCSTR, displayname : *mut u16, displaynamecapacity : i32, err : *mut UErrorCode) -> i32);
    unsafe { ucnv_getDisplayName(converter, displaylocale.param().abi(), displayname as _, displaynamecapacity, err as _) }
}
#[inline]
pub unsafe fn ucnv_getFromUCallBack(converter: *const UConverter, action: *mut UConverterFromUCallback, context: *const *const core::ffi::c_void) {
    windows_core::link!("icuuc.dll" "C" fn ucnv_getFromUCallBack(converter : *const UConverter, action : *mut UConverterFromUCallback, context : *const *const core::ffi::c_void));
    unsafe { ucnv_getFromUCallBack(converter, action as _, context) }
}
#[inline]
pub unsafe fn ucnv_getInvalidChars<P1>(converter: *const UConverter, errbytes: P1, len: *mut i8, err: *mut UErrorCode)
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn ucnv_getInvalidChars(converter : *const UConverter, errbytes : windows_core::PCSTR, len : *mut i8, err : *mut UErrorCode));
    unsafe { ucnv_getInvalidChars(converter, errbytes.param().abi(), len as _, err as _) }
}
#[inline]
pub unsafe fn ucnv_getInvalidUChars(converter: *const UConverter, erruchars: *mut u16, len: *mut i8, err: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn ucnv_getInvalidUChars(converter : *const UConverter, erruchars : *mut u16, len : *mut i8, err : *mut UErrorCode));
    unsafe { ucnv_getInvalidUChars(converter, erruchars as _, len as _, err as _) }
}
#[inline]
pub unsafe fn ucnv_getMaxCharSize(converter: *const UConverter) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn ucnv_getMaxCharSize(converter : *const UConverter) -> i8);
    unsafe { ucnv_getMaxCharSize(converter) }
}
#[inline]
pub unsafe fn ucnv_getMinCharSize(converter: *const UConverter) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn ucnv_getMinCharSize(converter : *const UConverter) -> i8);
    unsafe { ucnv_getMinCharSize(converter) }
}
#[inline]
pub unsafe fn ucnv_getName(converter: *const UConverter, err: *mut UErrorCode) -> windows_core::PCSTR {
    windows_core::link!("icuuc.dll" "C" fn ucnv_getName(converter : *const UConverter, err : *mut UErrorCode) -> windows_core::PCSTR);
    unsafe { ucnv_getName(converter, err as _) }
}
#[inline]
pub unsafe fn ucnv_getNextUChar<P2>(converter: *mut UConverter, source: *const *const i8, sourcelimit: P2, err: *mut UErrorCode) -> i32
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn ucnv_getNextUChar(converter : *mut UConverter, source : *const *const i8, sourcelimit : windows_core::PCSTR, err : *mut UErrorCode) -> i32);
    unsafe { ucnv_getNextUChar(converter as _, source, sourcelimit.param().abi(), err as _) }
}
#[inline]
pub unsafe fn ucnv_getPlatform(converter: *const UConverter, err: *mut UErrorCode) -> UConverterPlatform {
    windows_core::link!("icuuc.dll" "C" fn ucnv_getPlatform(converter : *const UConverter, err : *mut UErrorCode) -> UConverterPlatform);
    unsafe { ucnv_getPlatform(converter, err as _) }
}
#[inline]
pub unsafe fn ucnv_getStandard(n: u16, perrorcode: *mut UErrorCode) -> windows_core::PCSTR {
    windows_core::link!("icuuc.dll" "C" fn ucnv_getStandard(n : u16, perrorcode : *mut UErrorCode) -> windows_core::PCSTR);
    unsafe { ucnv_getStandard(n, perrorcode as _) }
}
#[inline]
pub unsafe fn ucnv_getStandardName<P0, P1>(name: P0, standard: P1, perrorcode: *mut UErrorCode) -> windows_core::PCSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn ucnv_getStandardName(name : windows_core::PCSTR, standard : windows_core::PCSTR, perrorcode : *mut UErrorCode) -> windows_core::PCSTR);
    unsafe { ucnv_getStandardName(name.param().abi(), standard.param().abi(), perrorcode as _) }
}
#[inline]
pub unsafe fn ucnv_getStarters(converter: *const UConverter, starters: *mut i8, err: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn ucnv_getStarters(converter : *const UConverter, starters : *mut i8, err : *mut UErrorCode));
    unsafe { ucnv_getStarters(converter, starters as _, err as _) }
}
#[inline]
pub unsafe fn ucnv_getSubstChars<P1>(converter: *const UConverter, subchars: P1, len: *mut i8, err: *mut UErrorCode)
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn ucnv_getSubstChars(converter : *const UConverter, subchars : windows_core::PCSTR, len : *mut i8, err : *mut UErrorCode));
    unsafe { ucnv_getSubstChars(converter, subchars.param().abi(), len as _, err as _) }
}
#[inline]
pub unsafe fn ucnv_getToUCallBack(converter: *const UConverter, action: *mut UConverterToUCallback, context: *const *const core::ffi::c_void) {
    windows_core::link!("icuuc.dll" "C" fn ucnv_getToUCallBack(converter : *const UConverter, action : *mut UConverterToUCallback, context : *const *const core::ffi::c_void));
    unsafe { ucnv_getToUCallBack(converter, action as _, context) }
}
#[inline]
pub unsafe fn ucnv_getType(converter: *const UConverter) -> UConverterType {
    windows_core::link!("icuuc.dll" "C" fn ucnv_getType(converter : *const UConverter) -> UConverterType);
    unsafe { ucnv_getType(converter) }
}
#[inline]
pub unsafe fn ucnv_getUnicodeSet(cnv: *const UConverter, setfillin: *mut USet, whichset: UConverterUnicodeSet, perrorcode: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn ucnv_getUnicodeSet(cnv : *const UConverter, setfillin : *mut USet, whichset : UConverterUnicodeSet, perrorcode : *mut UErrorCode));
    unsafe { ucnv_getUnicodeSet(cnv, setfillin as _, whichset, perrorcode as _) }
}
#[inline]
pub unsafe fn ucnv_isAmbiguous(cnv: *const UConverter) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn ucnv_isAmbiguous(cnv : *const UConverter) -> i8);
    unsafe { ucnv_isAmbiguous(cnv) }
}
#[inline]
pub unsafe fn ucnv_isFixedWidth(cnv: *mut UConverter, status: *mut UErrorCode) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn ucnv_isFixedWidth(cnv : *mut UConverter, status : *mut UErrorCode) -> i8);
    unsafe { ucnv_isFixedWidth(cnv as _, status as _) }
}
#[inline]
pub unsafe fn ucnv_open<P0>(convertername: P0, err: *mut UErrorCode) -> *mut UConverter
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn ucnv_open(convertername : windows_core::PCSTR, err : *mut UErrorCode) -> *mut UConverter);
    unsafe { ucnv_open(convertername.param().abi(), err as _) }
}
#[inline]
pub unsafe fn ucnv_openAllNames(perrorcode: *mut UErrorCode) -> *mut UEnumeration {
    windows_core::link!("icuuc.dll" "C" fn ucnv_openAllNames(perrorcode : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { ucnv_openAllNames(perrorcode as _) }
}
#[inline]
pub unsafe fn ucnv_openCCSID(codepage: i32, platform: UConverterPlatform, err: *mut UErrorCode) -> *mut UConverter {
    windows_core::link!("icuuc.dll" "C" fn ucnv_openCCSID(codepage : i32, platform : UConverterPlatform, err : *mut UErrorCode) -> *mut UConverter);
    unsafe { ucnv_openCCSID(codepage, platform, err as _) }
}
#[inline]
pub unsafe fn ucnv_openPackage<P0, P1>(packagename: P0, convertername: P1, err: *mut UErrorCode) -> *mut UConverter
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn ucnv_openPackage(packagename : windows_core::PCSTR, convertername : windows_core::PCSTR, err : *mut UErrorCode) -> *mut UConverter);
    unsafe { ucnv_openPackage(packagename.param().abi(), convertername.param().abi(), err as _) }
}
#[inline]
pub unsafe fn ucnv_openStandardNames<P0, P1>(convname: P0, standard: P1, perrorcode: *mut UErrorCode) -> *mut UEnumeration
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn ucnv_openStandardNames(convname : windows_core::PCSTR, standard : windows_core::PCSTR, perrorcode : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { ucnv_openStandardNames(convname.param().abi(), standard.param().abi(), perrorcode as _) }
}
#[inline]
pub unsafe fn ucnv_openU(name: *const u16, err: *mut UErrorCode) -> *mut UConverter {
    windows_core::link!("icuuc.dll" "C" fn ucnv_openU(name : *const u16, err : *mut UErrorCode) -> *mut UConverter);
    unsafe { ucnv_openU(name, err as _) }
}
#[inline]
pub unsafe fn ucnv_reset(converter: *mut UConverter) {
    windows_core::link!("icuuc.dll" "C" fn ucnv_reset(converter : *mut UConverter));
    unsafe { ucnv_reset(converter as _) }
}
#[inline]
pub unsafe fn ucnv_resetFromUnicode(converter: *mut UConverter) {
    windows_core::link!("icuuc.dll" "C" fn ucnv_resetFromUnicode(converter : *mut UConverter));
    unsafe { ucnv_resetFromUnicode(converter as _) }
}
#[inline]
pub unsafe fn ucnv_resetToUnicode(converter: *mut UConverter) {
    windows_core::link!("icuuc.dll" "C" fn ucnv_resetToUnicode(converter : *mut UConverter));
    unsafe { ucnv_resetToUnicode(converter as _) }
}
#[inline]
pub unsafe fn ucnv_safeClone(cnv: *const UConverter, stackbuffer: *mut core::ffi::c_void, pbuffersize: *mut i32, status: *mut UErrorCode) -> *mut UConverter {
    windows_core::link!("icuuc.dll" "C" fn ucnv_safeClone(cnv : *const UConverter, stackbuffer : *mut core::ffi::c_void, pbuffersize : *mut i32, status : *mut UErrorCode) -> *mut UConverter);
    unsafe { ucnv_safeClone(cnv, stackbuffer as _, pbuffersize as _, status as _) }
}
#[inline]
pub unsafe fn ucnv_setDefaultName<P0>(name: P0)
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn ucnv_setDefaultName(name : windows_core::PCSTR));
    unsafe { ucnv_setDefaultName(name.param().abi()) }
}
#[inline]
pub unsafe fn ucnv_setFallback(cnv: *mut UConverter, usesfallback: i8) {
    windows_core::link!("icuuc.dll" "C" fn ucnv_setFallback(cnv : *mut UConverter, usesfallback : i8));
    unsafe { ucnv_setFallback(cnv as _, usesfallback) }
}
#[inline]
pub unsafe fn ucnv_setFromUCallBack(converter: *mut UConverter, newaction: UConverterFromUCallback, newcontext: *const core::ffi::c_void, oldaction: *mut UConverterFromUCallback, oldcontext: *const *const core::ffi::c_void, err: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn ucnv_setFromUCallBack(converter : *mut UConverter, newaction : UConverterFromUCallback, newcontext : *const core::ffi::c_void, oldaction : *mut UConverterFromUCallback, oldcontext : *const *const core::ffi::c_void, err : *mut UErrorCode));
    unsafe { ucnv_setFromUCallBack(converter as _, newaction, newcontext, oldaction as _, oldcontext, err as _) }
}
#[inline]
pub unsafe fn ucnv_setSubstChars<P1>(converter: *mut UConverter, subchars: P1, len: i8, err: *mut UErrorCode)
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn ucnv_setSubstChars(converter : *mut UConverter, subchars : windows_core::PCSTR, len : i8, err : *mut UErrorCode));
    unsafe { ucnv_setSubstChars(converter as _, subchars.param().abi(), len, err as _) }
}
#[inline]
pub unsafe fn ucnv_setSubstString(cnv: *mut UConverter, s: *const u16, length: i32, err: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn ucnv_setSubstString(cnv : *mut UConverter, s : *const u16, length : i32, err : *mut UErrorCode));
    unsafe { ucnv_setSubstString(cnv as _, s, length, err as _) }
}
#[inline]
pub unsafe fn ucnv_setToUCallBack(converter: *mut UConverter, newaction: UConverterToUCallback, newcontext: *const core::ffi::c_void, oldaction: *mut UConverterToUCallback, oldcontext: *const *const core::ffi::c_void, err: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn ucnv_setToUCallBack(converter : *mut UConverter, newaction : UConverterToUCallback, newcontext : *const core::ffi::c_void, oldaction : *mut UConverterToUCallback, oldcontext : *const *const core::ffi::c_void, err : *mut UErrorCode));
    unsafe { ucnv_setToUCallBack(converter as _, newaction, newcontext, oldaction as _, oldcontext, err as _) }
}
#[inline]
pub unsafe fn ucnv_toAlgorithmic<P2, P4>(algorithmictype: UConverterType, cnv: *mut UConverter, target: P2, targetcapacity: i32, source: P4, sourcelength: i32, perrorcode: *mut UErrorCode) -> i32
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn ucnv_toAlgorithmic(algorithmictype : UConverterType, cnv : *mut UConverter, target : windows_core::PCSTR, targetcapacity : i32, source : windows_core::PCSTR, sourcelength : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { ucnv_toAlgorithmic(algorithmictype, cnv as _, target.param().abi(), targetcapacity, source.param().abi(), sourcelength, perrorcode as _) }
}
#[inline]
pub unsafe fn ucnv_toUChars<P3>(cnv: *mut UConverter, dest: *mut u16, destcapacity: i32, src: P3, srclength: i32, perrorcode: *mut UErrorCode) -> i32
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn ucnv_toUChars(cnv : *mut UConverter, dest : *mut u16, destcapacity : i32, src : windows_core::PCSTR, srclength : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { ucnv_toUChars(cnv as _, dest as _, destcapacity, src.param().abi(), srclength, perrorcode as _) }
}
#[inline]
pub unsafe fn ucnv_toUCountPending(cnv: *const UConverter, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ucnv_toUCountPending(cnv : *const UConverter, status : *mut UErrorCode) -> i32);
    unsafe { ucnv_toUCountPending(cnv, status as _) }
}
#[inline]
pub unsafe fn ucnv_toUnicode<P4>(converter: *mut UConverter, target: *mut *mut u16, targetlimit: *const u16, source: *const *const i8, sourcelimit: P4, offsets: *mut i32, flush: i8, err: *mut UErrorCode)
where
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn ucnv_toUnicode(converter : *mut UConverter, target : *mut *mut u16, targetlimit : *const u16, source : *const *const i8, sourcelimit : windows_core::PCSTR, offsets : *mut i32, flush : i8, err : *mut UErrorCode));
    unsafe { ucnv_toUnicode(converter as _, target as _, targetlimit, source, sourcelimit.param().abi(), offsets as _, flush, err as _) }
}
#[inline]
pub unsafe fn ucnv_usesFallback(cnv: *const UConverter) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn ucnv_usesFallback(cnv : *const UConverter) -> i8);
    unsafe { ucnv_usesFallback(cnv) }
}
#[inline]
pub unsafe fn ucnvsel_close(sel: *mut UConverterSelector) {
    windows_core::link!("icuuc.dll" "C" fn ucnvsel_close(sel : *mut UConverterSelector));
    unsafe { ucnvsel_close(sel as _) }
}
#[inline]
pub unsafe fn ucnvsel_open(converterlist: *const *const i8, converterlistsize: i32, excludedcodepoints: *const USet, whichset: UConverterUnicodeSet, status: *mut UErrorCode) -> *mut UConverterSelector {
    windows_core::link!("icuuc.dll" "C" fn ucnvsel_open(converterlist : *const *const i8, converterlistsize : i32, excludedcodepoints : *const USet, whichset : UConverterUnicodeSet, status : *mut UErrorCode) -> *mut UConverterSelector);
    unsafe { ucnvsel_open(converterlist, converterlistsize, excludedcodepoints, whichset, status as _) }
}
#[inline]
pub unsafe fn ucnvsel_openFromSerialized(buffer: *const core::ffi::c_void, length: i32, status: *mut UErrorCode) -> *mut UConverterSelector {
    windows_core::link!("icuuc.dll" "C" fn ucnvsel_openFromSerialized(buffer : *const core::ffi::c_void, length : i32, status : *mut UErrorCode) -> *mut UConverterSelector);
    unsafe { ucnvsel_openFromSerialized(buffer, length, status as _) }
}
#[inline]
pub unsafe fn ucnvsel_selectForString(sel: *const UConverterSelector, s: *const u16, length: i32, status: *mut UErrorCode) -> *mut UEnumeration {
    windows_core::link!("icuuc.dll" "C" fn ucnvsel_selectForString(sel : *const UConverterSelector, s : *const u16, length : i32, status : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { ucnvsel_selectForString(sel, s, length, status as _) }
}
#[inline]
pub unsafe fn ucnvsel_selectForUTF8<P1>(sel: *const UConverterSelector, s: P1, length: i32, status: *mut UErrorCode) -> *mut UEnumeration
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn ucnvsel_selectForUTF8(sel : *const UConverterSelector, s : windows_core::PCSTR, length : i32, status : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { ucnvsel_selectForUTF8(sel, s.param().abi(), length, status as _) }
}
#[inline]
pub unsafe fn ucnvsel_serialize(sel: *const UConverterSelector, buffer: *mut core::ffi::c_void, buffercapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ucnvsel_serialize(sel : *const UConverterSelector, buffer : *mut core::ffi::c_void, buffercapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { ucnvsel_serialize(sel, buffer as _, buffercapacity, status as _) }
}
#[inline]
pub unsafe fn ucol_cloneBinary(coll: *const UCollator, buffer: *mut u8, capacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucol_cloneBinary(coll : *const UCollator, buffer : *mut u8, capacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { ucol_cloneBinary(coll, buffer as _, capacity, status as _) }
}
#[inline]
pub unsafe fn ucol_close(coll: *mut UCollator) {
    windows_core::link!("icuin.dll" "C" fn ucol_close(coll : *mut UCollator));
    unsafe { ucol_close(coll as _) }
}
#[inline]
pub unsafe fn ucol_closeElements(elems: *mut UCollationElements) {
    windows_core::link!("icuin.dll" "C" fn ucol_closeElements(elems : *mut UCollationElements));
    unsafe { ucol_closeElements(elems as _) }
}
#[inline]
pub unsafe fn ucol_countAvailable() -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucol_countAvailable() -> i32);
    unsafe { ucol_countAvailable() }
}
#[inline]
pub unsafe fn ucol_equal(coll: *const UCollator, source: *const u16, sourcelength: i32, target: *const u16, targetlength: i32) -> i8 {
    windows_core::link!("icuin.dll" "C" fn ucol_equal(coll : *const UCollator, source : *const u16, sourcelength : i32, target : *const u16, targetlength : i32) -> i8);
    unsafe { ucol_equal(coll, source, sourcelength, target, targetlength) }
}
#[inline]
pub unsafe fn ucol_getAttribute(coll: *const UCollator, attr: UColAttribute, status: *mut UErrorCode) -> UColAttributeValue {
    windows_core::link!("icuin.dll" "C" fn ucol_getAttribute(coll : *const UCollator, attr : UColAttribute, status : *mut UErrorCode) -> UColAttributeValue);
    unsafe { ucol_getAttribute(coll, attr, status as _) }
}
#[inline]
pub unsafe fn ucol_getAvailable(localeindex: i32) -> windows_core::PCSTR {
    windows_core::link!("icuin.dll" "C" fn ucol_getAvailable(localeindex : i32) -> windows_core::PCSTR);
    unsafe { ucol_getAvailable(localeindex) }
}
#[inline]
pub unsafe fn ucol_getBound(source: *const u8, sourcelength: i32, boundtype: UColBoundMode, nooflevels: u32, result: *mut u8, resultlength: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucol_getBound(source : *const u8, sourcelength : i32, boundtype : UColBoundMode, nooflevels : u32, result : *mut u8, resultlength : i32, status : *mut UErrorCode) -> i32);
    unsafe { ucol_getBound(source, sourcelength, boundtype, nooflevels, result as _, resultlength, status as _) }
}
#[inline]
pub unsafe fn ucol_getContractionsAndExpansions(coll: *const UCollator, contractions: *mut USet, expansions: *mut USet, addprefixes: i8, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn ucol_getContractionsAndExpansions(coll : *const UCollator, contractions : *mut USet, expansions : *mut USet, addprefixes : i8, status : *mut UErrorCode));
    unsafe { ucol_getContractionsAndExpansions(coll, contractions as _, expansions as _, addprefixes, status as _) }
}
#[inline]
pub unsafe fn ucol_getDisplayName<P0, P1>(objloc: P0, disploc: P1, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuin.dll" "C" fn ucol_getDisplayName(objloc : windows_core::PCSTR, disploc : windows_core::PCSTR, result : *mut u16, resultlength : i32, status : *mut UErrorCode) -> i32);
    unsafe { ucol_getDisplayName(objloc.param().abi(), disploc.param().abi(), result as _, resultlength, status as _) }
}
#[inline]
pub unsafe fn ucol_getEquivalentReorderCodes(reordercode: i32, dest: *mut i32, destcapacity: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucol_getEquivalentReorderCodes(reordercode : i32, dest : *mut i32, destcapacity : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { ucol_getEquivalentReorderCodes(reordercode, dest as _, destcapacity, perrorcode as _) }
}
#[inline]
pub unsafe fn ucol_getFunctionalEquivalent<P0, P2, P3>(result: P0, resultcapacity: i32, keyword: P2, locale: P3, isavailable: *mut i8, status: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuin.dll" "C" fn ucol_getFunctionalEquivalent(result : windows_core::PCSTR, resultcapacity : i32, keyword : windows_core::PCSTR, locale : windows_core::PCSTR, isavailable : *mut i8, status : *mut UErrorCode) -> i32);
    unsafe { ucol_getFunctionalEquivalent(result.param().abi(), resultcapacity, keyword.param().abi(), locale.param().abi(), isavailable as _, status as _) }
}
#[inline]
pub unsafe fn ucol_getKeywordValues<P0>(keyword: P0, status: *mut UErrorCode) -> *mut UEnumeration
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuin.dll" "C" fn ucol_getKeywordValues(keyword : windows_core::PCSTR, status : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { ucol_getKeywordValues(keyword.param().abi(), status as _) }
}
#[inline]
pub unsafe fn ucol_getKeywordValuesForLocale<P0, P1>(key: P0, locale: P1, commonlyused: i8, status: *mut UErrorCode) -> *mut UEnumeration
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuin.dll" "C" fn ucol_getKeywordValuesForLocale(key : windows_core::PCSTR, locale : windows_core::PCSTR, commonlyused : i8, status : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { ucol_getKeywordValuesForLocale(key.param().abi(), locale.param().abi(), commonlyused, status as _) }
}
#[inline]
pub unsafe fn ucol_getKeywords(status: *mut UErrorCode) -> *mut UEnumeration {
    windows_core::link!("icuin.dll" "C" fn ucol_getKeywords(status : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { ucol_getKeywords(status as _) }
}
#[inline]
pub unsafe fn ucol_getLocaleByType(coll: *const UCollator, r#type: ULocDataLocaleType, status: *mut UErrorCode) -> windows_core::PCSTR {
    windows_core::link!("icuin.dll" "C" fn ucol_getLocaleByType(coll : *const UCollator, r#type : ULocDataLocaleType, status : *mut UErrorCode) -> windows_core::PCSTR);
    unsafe { ucol_getLocaleByType(coll, r#type, status as _) }
}
#[inline]
pub unsafe fn ucol_getMaxExpansion(elems: *const UCollationElements, order: i32) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucol_getMaxExpansion(elems : *const UCollationElements, order : i32) -> i32);
    unsafe { ucol_getMaxExpansion(elems, order) }
}
#[inline]
pub unsafe fn ucol_getMaxVariable(coll: *const UCollator) -> UColReorderCode {
    windows_core::link!("icuin.dll" "C" fn ucol_getMaxVariable(coll : *const UCollator) -> UColReorderCode);
    unsafe { ucol_getMaxVariable(coll) }
}
#[inline]
pub unsafe fn ucol_getOffset(elems: *const UCollationElements) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucol_getOffset(elems : *const UCollationElements) -> i32);
    unsafe { ucol_getOffset(elems) }
}
#[inline]
pub unsafe fn ucol_getReorderCodes(coll: *const UCollator, dest: *mut i32, destcapacity: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucol_getReorderCodes(coll : *const UCollator, dest : *mut i32, destcapacity : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { ucol_getReorderCodes(coll, dest as _, destcapacity, perrorcode as _) }
}
#[inline]
pub unsafe fn ucol_getRules(coll: *const UCollator, length: *mut i32) -> *mut u16 {
    windows_core::link!("icuin.dll" "C" fn ucol_getRules(coll : *const UCollator, length : *mut i32) -> *mut u16);
    unsafe { ucol_getRules(coll, length as _) }
}
#[inline]
pub unsafe fn ucol_getRulesEx(coll: *const UCollator, delta: UColRuleOption, buffer: *mut u16, bufferlen: i32) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucol_getRulesEx(coll : *const UCollator, delta : UColRuleOption, buffer : *mut u16, bufferlen : i32) -> i32);
    unsafe { ucol_getRulesEx(coll, delta, buffer as _, bufferlen) }
}
#[inline]
pub unsafe fn ucol_getSortKey(coll: *const UCollator, source: *const u16, sourcelength: i32, result: *mut u8, resultlength: i32) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucol_getSortKey(coll : *const UCollator, source : *const u16, sourcelength : i32, result : *mut u8, resultlength : i32) -> i32);
    unsafe { ucol_getSortKey(coll, source, sourcelength, result as _, resultlength) }
}
#[inline]
pub unsafe fn ucol_getStrength(coll: *const UCollator) -> UColAttributeValue {
    windows_core::link!("icuin.dll" "C" fn ucol_getStrength(coll : *const UCollator) -> UColAttributeValue);
    unsafe { ucol_getStrength(coll) }
}
#[inline]
pub unsafe fn ucol_getTailoredSet(coll: *const UCollator, status: *mut UErrorCode) -> *mut USet {
    windows_core::link!("icuin.dll" "C" fn ucol_getTailoredSet(coll : *const UCollator, status : *mut UErrorCode) -> *mut USet);
    unsafe { ucol_getTailoredSet(coll, status as _) }
}
#[inline]
pub unsafe fn ucol_getUCAVersion(coll: *const UCollator, info: *mut u8) {
    windows_core::link!("icuin.dll" "C" fn ucol_getUCAVersion(coll : *const UCollator, info : *mut u8));
    unsafe { ucol_getUCAVersion(coll, info as _) }
}
#[inline]
pub unsafe fn ucol_getVariableTop(coll: *const UCollator, status: *mut UErrorCode) -> u32 {
    windows_core::link!("icuin.dll" "C" fn ucol_getVariableTop(coll : *const UCollator, status : *mut UErrorCode) -> u32);
    unsafe { ucol_getVariableTop(coll, status as _) }
}
#[inline]
pub unsafe fn ucol_getVersion(coll: *const UCollator, info: *mut u8) {
    windows_core::link!("icuin.dll" "C" fn ucol_getVersion(coll : *const UCollator, info : *mut u8));
    unsafe { ucol_getVersion(coll, info as _) }
}
#[inline]
pub unsafe fn ucol_greater(coll: *const UCollator, source: *const u16, sourcelength: i32, target: *const u16, targetlength: i32) -> i8 {
    windows_core::link!("icuin.dll" "C" fn ucol_greater(coll : *const UCollator, source : *const u16, sourcelength : i32, target : *const u16, targetlength : i32) -> i8);
    unsafe { ucol_greater(coll, source, sourcelength, target, targetlength) }
}
#[inline]
pub unsafe fn ucol_greaterOrEqual(coll: *const UCollator, source: *const u16, sourcelength: i32, target: *const u16, targetlength: i32) -> i8 {
    windows_core::link!("icuin.dll" "C" fn ucol_greaterOrEqual(coll : *const UCollator, source : *const u16, sourcelength : i32, target : *const u16, targetlength : i32) -> i8);
    unsafe { ucol_greaterOrEqual(coll, source, sourcelength, target, targetlength) }
}
#[inline]
pub unsafe fn ucol_keyHashCode(key: *const u8, length: i32) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucol_keyHashCode(key : *const u8, length : i32) -> i32);
    unsafe { ucol_keyHashCode(key, length) }
}
#[inline]
pub unsafe fn ucol_mergeSortkeys(src1: *const u8, src1length: i32, src2: *const u8, src2length: i32, dest: *mut u8, destcapacity: i32) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucol_mergeSortkeys(src1 : *const u8, src1length : i32, src2 : *const u8, src2length : i32, dest : *mut u8, destcapacity : i32) -> i32);
    unsafe { ucol_mergeSortkeys(src1, src1length, src2, src2length, dest as _, destcapacity) }
}
#[inline]
pub unsafe fn ucol_next(elems: *mut UCollationElements, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucol_next(elems : *mut UCollationElements, status : *mut UErrorCode) -> i32);
    unsafe { ucol_next(elems as _, status as _) }
}
#[inline]
pub unsafe fn ucol_nextSortKeyPart(coll: *const UCollator, iter: *mut UCharIterator, state: *mut u32, dest: *mut u8, count: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucol_nextSortKeyPart(coll : *const UCollator, iter : *mut UCharIterator, state : *mut u32, dest : *mut u8, count : i32, status : *mut UErrorCode) -> i32);
    unsafe { ucol_nextSortKeyPart(coll, iter as _, state as _, dest as _, count, status as _) }
}
#[inline]
pub unsafe fn ucol_open<P0>(loc: P0, status: *mut UErrorCode) -> *mut UCollator
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuin.dll" "C" fn ucol_open(loc : windows_core::PCSTR, status : *mut UErrorCode) -> *mut UCollator);
    unsafe { ucol_open(loc.param().abi(), status as _) }
}
#[inline]
pub unsafe fn ucol_openAvailableLocales(status: *mut UErrorCode) -> *mut UEnumeration {
    windows_core::link!("icuin.dll" "C" fn ucol_openAvailableLocales(status : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { ucol_openAvailableLocales(status as _) }
}
#[inline]
pub unsafe fn ucol_openBinary(bin: *const u8, length: i32, base: *const UCollator, status: *mut UErrorCode) -> *mut UCollator {
    windows_core::link!("icuin.dll" "C" fn ucol_openBinary(bin : *const u8, length : i32, base : *const UCollator, status : *mut UErrorCode) -> *mut UCollator);
    unsafe { ucol_openBinary(bin, length, base, status as _) }
}
#[inline]
pub unsafe fn ucol_openElements(coll: *const UCollator, text: *const u16, textlength: i32, status: *mut UErrorCode) -> *mut UCollationElements {
    windows_core::link!("icuin.dll" "C" fn ucol_openElements(coll : *const UCollator, text : *const u16, textlength : i32, status : *mut UErrorCode) -> *mut UCollationElements);
    unsafe { ucol_openElements(coll, text, textlength, status as _) }
}
#[inline]
pub unsafe fn ucol_openRules(rules: *const u16, ruleslength: i32, normalizationmode: UColAttributeValue, strength: UColAttributeValue, parseerror: *mut UParseError, status: *mut UErrorCode) -> *mut UCollator {
    windows_core::link!("icuin.dll" "C" fn ucol_openRules(rules : *const u16, ruleslength : i32, normalizationmode : UColAttributeValue, strength : UColAttributeValue, parseerror : *mut UParseError, status : *mut UErrorCode) -> *mut UCollator);
    unsafe { ucol_openRules(rules, ruleslength, normalizationmode, strength, parseerror as _, status as _) }
}
#[inline]
pub unsafe fn ucol_previous(elems: *mut UCollationElements, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucol_previous(elems : *mut UCollationElements, status : *mut UErrorCode) -> i32);
    unsafe { ucol_previous(elems as _, status as _) }
}
#[inline]
pub unsafe fn ucol_primaryOrder(order: i32) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucol_primaryOrder(order : i32) -> i32);
    unsafe { ucol_primaryOrder(order) }
}
#[inline]
pub unsafe fn ucol_reset(elems: *mut UCollationElements) {
    windows_core::link!("icuin.dll" "C" fn ucol_reset(elems : *mut UCollationElements));
    unsafe { ucol_reset(elems as _) }
}
#[inline]
pub unsafe fn ucol_safeClone(coll: *const UCollator, stackbuffer: *mut core::ffi::c_void, pbuffersize: *mut i32, status: *mut UErrorCode) -> *mut UCollator {
    windows_core::link!("icuin.dll" "C" fn ucol_safeClone(coll : *const UCollator, stackbuffer : *mut core::ffi::c_void, pbuffersize : *mut i32, status : *mut UErrorCode) -> *mut UCollator);
    unsafe { ucol_safeClone(coll, stackbuffer as _, pbuffersize as _, status as _) }
}
#[inline]
pub unsafe fn ucol_secondaryOrder(order: i32) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucol_secondaryOrder(order : i32) -> i32);
    unsafe { ucol_secondaryOrder(order) }
}
#[inline]
pub unsafe fn ucol_setAttribute(coll: *mut UCollator, attr: UColAttribute, value: UColAttributeValue, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn ucol_setAttribute(coll : *mut UCollator, attr : UColAttribute, value : UColAttributeValue, status : *mut UErrorCode));
    unsafe { ucol_setAttribute(coll as _, attr, value, status as _) }
}
#[inline]
pub unsafe fn ucol_setMaxVariable(coll: *mut UCollator, group: UColReorderCode, perrorcode: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn ucol_setMaxVariable(coll : *mut UCollator, group : UColReorderCode, perrorcode : *mut UErrorCode));
    unsafe { ucol_setMaxVariable(coll as _, group, perrorcode as _) }
}
#[inline]
pub unsafe fn ucol_setOffset(elems: *mut UCollationElements, offset: i32, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn ucol_setOffset(elems : *mut UCollationElements, offset : i32, status : *mut UErrorCode));
    unsafe { ucol_setOffset(elems as _, offset, status as _) }
}
#[inline]
pub unsafe fn ucol_setReorderCodes(coll: *mut UCollator, reordercodes: *const i32, reordercodeslength: i32, perrorcode: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn ucol_setReorderCodes(coll : *mut UCollator, reordercodes : *const i32, reordercodeslength : i32, perrorcode : *mut UErrorCode));
    unsafe { ucol_setReorderCodes(coll as _, reordercodes, reordercodeslength, perrorcode as _) }
}
#[inline]
pub unsafe fn ucol_setStrength(coll: *mut UCollator, strength: UColAttributeValue) {
    windows_core::link!("icuin.dll" "C" fn ucol_setStrength(coll : *mut UCollator, strength : UColAttributeValue));
    unsafe { ucol_setStrength(coll as _, strength) }
}
#[inline]
pub unsafe fn ucol_setText(elems: *mut UCollationElements, text: *const u16, textlength: i32, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn ucol_setText(elems : *mut UCollationElements, text : *const u16, textlength : i32, status : *mut UErrorCode));
    unsafe { ucol_setText(elems as _, text, textlength, status as _) }
}
#[inline]
pub unsafe fn ucol_strcoll(coll: *const UCollator, source: *const u16, sourcelength: i32, target: *const u16, targetlength: i32) -> UCollationResult {
    windows_core::link!("icuin.dll" "C" fn ucol_strcoll(coll : *const UCollator, source : *const u16, sourcelength : i32, target : *const u16, targetlength : i32) -> UCollationResult);
    unsafe { ucol_strcoll(coll, source, sourcelength, target, targetlength) }
}
#[inline]
pub unsafe fn ucol_strcollIter(coll: *const UCollator, siter: *mut UCharIterator, titer: *mut UCharIterator, status: *mut UErrorCode) -> UCollationResult {
    windows_core::link!("icuin.dll" "C" fn ucol_strcollIter(coll : *const UCollator, siter : *mut UCharIterator, titer : *mut UCharIterator, status : *mut UErrorCode) -> UCollationResult);
    unsafe { ucol_strcollIter(coll, siter as _, titer as _, status as _) }
}
#[inline]
pub unsafe fn ucol_strcollUTF8<P1, P3>(coll: *const UCollator, source: P1, sourcelength: i32, target: P3, targetlength: i32, status: *mut UErrorCode) -> UCollationResult
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuin.dll" "C" fn ucol_strcollUTF8(coll : *const UCollator, source : windows_core::PCSTR, sourcelength : i32, target : windows_core::PCSTR, targetlength : i32, status : *mut UErrorCode) -> UCollationResult);
    unsafe { ucol_strcollUTF8(coll, source.param().abi(), sourcelength, target.param().abi(), targetlength, status as _) }
}
#[inline]
pub unsafe fn ucol_tertiaryOrder(order: i32) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucol_tertiaryOrder(order : i32) -> i32);
    unsafe { ucol_tertiaryOrder(order) }
}
#[inline]
pub unsafe fn ucpmap_get(map: *const UCPMap, c: i32) -> u32 {
    windows_core::link!("icu.dll" "C" fn ucpmap_get(map : *const UCPMap, c : i32) -> u32);
    unsafe { ucpmap_get(map, c) }
}
#[inline]
pub unsafe fn ucpmap_getRange(map: *const UCPMap, start: i32, option: UCPMapRangeOption, surrogatevalue: u32, filter: *mut UCPMapValueFilter, context: *const core::ffi::c_void, pvalue: *mut u32) -> i32 {
    windows_core::link!("icu.dll" "C" fn ucpmap_getRange(map : *const UCPMap, start : i32, option : UCPMapRangeOption, surrogatevalue : u32, filter : *mut UCPMapValueFilter, context : *const core::ffi::c_void, pvalue : *mut u32) -> i32);
    unsafe { ucpmap_getRange(map, start, option, surrogatevalue, filter as _, context, pvalue as _) }
}
#[inline]
pub unsafe fn ucptrie_close(trie: *mut UCPTrie) {
    windows_core::link!("icu.dll" "C" fn ucptrie_close(trie : *mut UCPTrie));
    unsafe { ucptrie_close(trie as _) }
}
#[inline]
pub unsafe fn ucptrie_get(trie: *const UCPTrie, c: i32) -> u32 {
    windows_core::link!("icu.dll" "C" fn ucptrie_get(trie : *const UCPTrie, c : i32) -> u32);
    unsafe { ucptrie_get(trie, c) }
}
#[inline]
pub unsafe fn ucptrie_getRange(trie: *const UCPTrie, start: i32, option: UCPMapRangeOption, surrogatevalue: u32, filter: *mut UCPMapValueFilter, context: *const core::ffi::c_void, pvalue: *mut u32) -> i32 {
    windows_core::link!("icu.dll" "C" fn ucptrie_getRange(trie : *const UCPTrie, start : i32, option : UCPMapRangeOption, surrogatevalue : u32, filter : *mut UCPMapValueFilter, context : *const core::ffi::c_void, pvalue : *mut u32) -> i32);
    unsafe { ucptrie_getRange(trie, start, option, surrogatevalue, filter as _, context, pvalue as _) }
}
#[inline]
pub unsafe fn ucptrie_getType(trie: *const UCPTrie) -> UCPTrieType {
    windows_core::link!("icu.dll" "C" fn ucptrie_getType(trie : *const UCPTrie) -> UCPTrieType);
    unsafe { ucptrie_getType(trie) }
}
#[inline]
pub unsafe fn ucptrie_getValueWidth(trie: *const UCPTrie) -> UCPTrieValueWidth {
    windows_core::link!("icu.dll" "C" fn ucptrie_getValueWidth(trie : *const UCPTrie) -> UCPTrieValueWidth);
    unsafe { ucptrie_getValueWidth(trie) }
}
#[inline]
pub unsafe fn ucptrie_internalSmallIndex(trie: *const UCPTrie, c: i32) -> i32 {
    windows_core::link!("icu.dll" "C" fn ucptrie_internalSmallIndex(trie : *const UCPTrie, c : i32) -> i32);
    unsafe { ucptrie_internalSmallIndex(trie, c) }
}
#[inline]
pub unsafe fn ucptrie_internalSmallU8Index(trie: *const UCPTrie, lt1: i32, t2: u8, t3: u8) -> i32 {
    windows_core::link!("icu.dll" "C" fn ucptrie_internalSmallU8Index(trie : *const UCPTrie, lt1 : i32, t2 : u8, t3 : u8) -> i32);
    unsafe { ucptrie_internalSmallU8Index(trie, lt1, t2, t3) }
}
#[inline]
pub unsafe fn ucptrie_internalU8PrevIndex(trie: *const UCPTrie, c: i32, start: *const u8, src: *const u8) -> i32 {
    windows_core::link!("icu.dll" "C" fn ucptrie_internalU8PrevIndex(trie : *const UCPTrie, c : i32, start : *const u8, src : *const u8) -> i32);
    unsafe { ucptrie_internalU8PrevIndex(trie, c, start, src) }
}
#[inline]
pub unsafe fn ucptrie_openFromBinary(r#type: UCPTrieType, valuewidth: UCPTrieValueWidth, data: *const core::ffi::c_void, length: i32, pactuallength: *mut i32, perrorcode: *mut UErrorCode) -> *mut UCPTrie {
    windows_core::link!("icu.dll" "C" fn ucptrie_openFromBinary(r#type : UCPTrieType, valuewidth : UCPTrieValueWidth, data : *const core::ffi::c_void, length : i32, pactuallength : *mut i32, perrorcode : *mut UErrorCode) -> *mut UCPTrie);
    unsafe { ucptrie_openFromBinary(r#type, valuewidth, data, length, pactuallength as _, perrorcode as _) }
}
#[inline]
pub unsafe fn ucptrie_toBinary(trie: *const UCPTrie, data: *mut core::ffi::c_void, capacity: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icu.dll" "C" fn ucptrie_toBinary(trie : *const UCPTrie, data : *mut core::ffi::c_void, capacity : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { ucptrie_toBinary(trie, data as _, capacity, perrorcode as _) }
}
#[inline]
pub unsafe fn ucsdet_close(ucsd: *mut UCharsetDetector) {
    windows_core::link!("icuin.dll" "C" fn ucsdet_close(ucsd : *mut UCharsetDetector));
    unsafe { ucsdet_close(ucsd as _) }
}
#[inline]
pub unsafe fn ucsdet_detect(ucsd: *mut UCharsetDetector, status: *mut UErrorCode) -> *mut UCharsetMatch {
    windows_core::link!("icuin.dll" "C" fn ucsdet_detect(ucsd : *mut UCharsetDetector, status : *mut UErrorCode) -> *mut UCharsetMatch);
    unsafe { ucsdet_detect(ucsd as _, status as _) }
}
#[inline]
pub unsafe fn ucsdet_detectAll(ucsd: *mut UCharsetDetector, matchesfound: *mut i32, status: *mut UErrorCode) -> *mut *mut UCharsetMatch {
    windows_core::link!("icuin.dll" "C" fn ucsdet_detectAll(ucsd : *mut UCharsetDetector, matchesfound : *mut i32, status : *mut UErrorCode) -> *mut *mut UCharsetMatch);
    unsafe { ucsdet_detectAll(ucsd as _, matchesfound as _, status as _) }
}
#[inline]
pub unsafe fn ucsdet_enableInputFilter(ucsd: *mut UCharsetDetector, filter: i8) -> i8 {
    windows_core::link!("icuin.dll" "C" fn ucsdet_enableInputFilter(ucsd : *mut UCharsetDetector, filter : i8) -> i8);
    unsafe { ucsdet_enableInputFilter(ucsd as _, filter) }
}
#[inline]
pub unsafe fn ucsdet_getAllDetectableCharsets(ucsd: *const UCharsetDetector, status: *mut UErrorCode) -> *mut UEnumeration {
    windows_core::link!("icuin.dll" "C" fn ucsdet_getAllDetectableCharsets(ucsd : *const UCharsetDetector, status : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { ucsdet_getAllDetectableCharsets(ucsd, status as _) }
}
#[inline]
pub unsafe fn ucsdet_getConfidence(ucsm: *const UCharsetMatch, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucsdet_getConfidence(ucsm : *const UCharsetMatch, status : *mut UErrorCode) -> i32);
    unsafe { ucsdet_getConfidence(ucsm, status as _) }
}
#[inline]
pub unsafe fn ucsdet_getLanguage(ucsm: *const UCharsetMatch, status: *mut UErrorCode) -> windows_core::PCSTR {
    windows_core::link!("icuin.dll" "C" fn ucsdet_getLanguage(ucsm : *const UCharsetMatch, status : *mut UErrorCode) -> windows_core::PCSTR);
    unsafe { ucsdet_getLanguage(ucsm, status as _) }
}
#[inline]
pub unsafe fn ucsdet_getName(ucsm: *const UCharsetMatch, status: *mut UErrorCode) -> windows_core::PCSTR {
    windows_core::link!("icuin.dll" "C" fn ucsdet_getName(ucsm : *const UCharsetMatch, status : *mut UErrorCode) -> windows_core::PCSTR);
    unsafe { ucsdet_getName(ucsm, status as _) }
}
#[inline]
pub unsafe fn ucsdet_getUChars(ucsm: *const UCharsetMatch, buf: *mut u16, cap: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ucsdet_getUChars(ucsm : *const UCharsetMatch, buf : *mut u16, cap : i32, status : *mut UErrorCode) -> i32);
    unsafe { ucsdet_getUChars(ucsm, buf as _, cap, status as _) }
}
#[inline]
pub unsafe fn ucsdet_isInputFilterEnabled(ucsd: *const UCharsetDetector) -> i8 {
    windows_core::link!("icuin.dll" "C" fn ucsdet_isInputFilterEnabled(ucsd : *const UCharsetDetector) -> i8);
    unsafe { ucsdet_isInputFilterEnabled(ucsd) }
}
#[inline]
pub unsafe fn ucsdet_open(status: *mut UErrorCode) -> *mut UCharsetDetector {
    windows_core::link!("icuin.dll" "C" fn ucsdet_open(status : *mut UErrorCode) -> *mut UCharsetDetector);
    unsafe { ucsdet_open(status as _) }
}
#[inline]
pub unsafe fn ucsdet_setDeclaredEncoding<P1>(ucsd: *mut UCharsetDetector, encoding: P1, length: i32, status: *mut UErrorCode)
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuin.dll" "C" fn ucsdet_setDeclaredEncoding(ucsd : *mut UCharsetDetector, encoding : windows_core::PCSTR, length : i32, status : *mut UErrorCode));
    unsafe { ucsdet_setDeclaredEncoding(ucsd as _, encoding.param().abi(), length, status as _) }
}
#[inline]
pub unsafe fn ucsdet_setText<P1>(ucsd: *mut UCharsetDetector, textin: P1, len: i32, status: *mut UErrorCode)
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuin.dll" "C" fn ucsdet_setText(ucsd : *mut UCharsetDetector, textin : windows_core::PCSTR, len : i32, status : *mut UErrorCode));
    unsafe { ucsdet_setText(ucsd as _, textin.param().abi(), len, status as _) }
}
#[inline]
pub unsafe fn ucurr_countCurrencies<P0>(locale: P0, date: f64, ec: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn ucurr_countCurrencies(locale : windows_core::PCSTR, date : f64, ec : *mut UErrorCode) -> i32);
    unsafe { ucurr_countCurrencies(locale.param().abi(), date, ec as _) }
}
#[inline]
pub unsafe fn ucurr_forLocale<P0>(locale: P0, buff: *mut u16, buffcapacity: i32, ec: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn ucurr_forLocale(locale : windows_core::PCSTR, buff : *mut u16, buffcapacity : i32, ec : *mut UErrorCode) -> i32);
    unsafe { ucurr_forLocale(locale.param().abi(), buff as _, buffcapacity, ec as _) }
}
#[inline]
pub unsafe fn ucurr_forLocaleAndDate<P0>(locale: P0, date: f64, index: i32, buff: *mut u16, buffcapacity: i32, ec: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn ucurr_forLocaleAndDate(locale : windows_core::PCSTR, date : f64, index : i32, buff : *mut u16, buffcapacity : i32, ec : *mut UErrorCode) -> i32);
    unsafe { ucurr_forLocaleAndDate(locale.param().abi(), date, index, buff as _, buffcapacity, ec as _) }
}
#[inline]
pub unsafe fn ucurr_getDefaultFractionDigits(currency: *const u16, ec: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ucurr_getDefaultFractionDigits(currency : *const u16, ec : *mut UErrorCode) -> i32);
    unsafe { ucurr_getDefaultFractionDigits(currency, ec as _) }
}
#[inline]
pub unsafe fn ucurr_getDefaultFractionDigitsForUsage(currency: *const u16, usage: UCurrencyUsage, ec: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ucurr_getDefaultFractionDigitsForUsage(currency : *const u16, usage : UCurrencyUsage, ec : *mut UErrorCode) -> i32);
    unsafe { ucurr_getDefaultFractionDigitsForUsage(currency, usage, ec as _) }
}
#[inline]
pub unsafe fn ucurr_getKeywordValuesForLocale<P0, P1>(key: P0, locale: P1, commonlyused: i8, status: *mut UErrorCode) -> *mut UEnumeration
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn ucurr_getKeywordValuesForLocale(key : windows_core::PCSTR, locale : windows_core::PCSTR, commonlyused : i8, status : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { ucurr_getKeywordValuesForLocale(key.param().abi(), locale.param().abi(), commonlyused, status as _) }
}
#[inline]
pub unsafe fn ucurr_getName<P1>(currency: *const u16, locale: P1, namestyle: UCurrNameStyle, ischoiceformat: *mut i8, len: *mut i32, ec: *mut UErrorCode) -> *mut u16
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn ucurr_getName(currency : *const u16, locale : windows_core::PCSTR, namestyle : UCurrNameStyle, ischoiceformat : *mut i8, len : *mut i32, ec : *mut UErrorCode) -> *mut u16);
    unsafe { ucurr_getName(currency, locale.param().abi(), namestyle, ischoiceformat as _, len as _, ec as _) }
}
#[inline]
pub unsafe fn ucurr_getNumericCode(currency: *const u16) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ucurr_getNumericCode(currency : *const u16) -> i32);
    unsafe { ucurr_getNumericCode(currency) }
}
#[inline]
pub unsafe fn ucurr_getPluralName<P1, P3>(currency: *const u16, locale: P1, ischoiceformat: *mut i8, pluralcount: P3, len: *mut i32, ec: *mut UErrorCode) -> *mut u16
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn ucurr_getPluralName(currency : *const u16, locale : windows_core::PCSTR, ischoiceformat : *mut i8, pluralcount : windows_core::PCSTR, len : *mut i32, ec : *mut UErrorCode) -> *mut u16);
    unsafe { ucurr_getPluralName(currency, locale.param().abi(), ischoiceformat as _, pluralcount.param().abi(), len as _, ec as _) }
}
#[inline]
pub unsafe fn ucurr_getRoundingIncrement(currency: *const u16, ec: *mut UErrorCode) -> f64 {
    windows_core::link!("icuuc.dll" "C" fn ucurr_getRoundingIncrement(currency : *const u16, ec : *mut UErrorCode) -> f64);
    unsafe { ucurr_getRoundingIncrement(currency, ec as _) }
}
#[inline]
pub unsafe fn ucurr_getRoundingIncrementForUsage(currency: *const u16, usage: UCurrencyUsage, ec: *mut UErrorCode) -> f64 {
    windows_core::link!("icuuc.dll" "C" fn ucurr_getRoundingIncrementForUsage(currency : *const u16, usage : UCurrencyUsage, ec : *mut UErrorCode) -> f64);
    unsafe { ucurr_getRoundingIncrementForUsage(currency, usage, ec as _) }
}
#[inline]
pub unsafe fn ucurr_isAvailable(isocode: *const u16, from: f64, to: f64, errorcode: *mut UErrorCode) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn ucurr_isAvailable(isocode : *const u16, from : f64, to : f64, errorcode : *mut UErrorCode) -> i8);
    unsafe { ucurr_isAvailable(isocode, from, to, errorcode as _) }
}
#[inline]
pub unsafe fn ucurr_openISOCurrencies(currtype: u32, perrorcode: *mut UErrorCode) -> *mut UEnumeration {
    windows_core::link!("icuuc.dll" "C" fn ucurr_openISOCurrencies(currtype : u32, perrorcode : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { ucurr_openISOCurrencies(currtype, perrorcode as _) }
}
#[inline]
pub unsafe fn ucurr_register<P1>(isocode: *const u16, locale: P1, status: *mut UErrorCode) -> *mut core::ffi::c_void
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn ucurr_register(isocode : *const u16, locale : windows_core::PCSTR, status : *mut UErrorCode) -> *mut core::ffi::c_void);
    unsafe { ucurr_register(isocode, locale.param().abi(), status as _) }
}
#[inline]
pub unsafe fn ucurr_unregister(key: *mut core::ffi::c_void, status: *mut UErrorCode) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn ucurr_unregister(key : *mut core::ffi::c_void, status : *mut UErrorCode) -> i8);
    unsafe { ucurr_unregister(key as _, status as _) }
}
#[inline]
pub unsafe fn udat_adoptNumberFormat(fmt: *mut *mut core::ffi::c_void, numberformattoadopt: *mut *mut core::ffi::c_void) {
    windows_core::link!("icuin.dll" "C" fn udat_adoptNumberFormat(fmt : *mut *mut core::ffi::c_void, numberformattoadopt : *mut *mut core::ffi::c_void));
    unsafe { udat_adoptNumberFormat(fmt as _, numberformattoadopt as _) }
}
#[inline]
pub unsafe fn udat_adoptNumberFormatForFields(fmt: *mut *mut core::ffi::c_void, fields: *const u16, numberformattoset: *mut *mut core::ffi::c_void, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn udat_adoptNumberFormatForFields(fmt : *mut *mut core::ffi::c_void, fields : *const u16, numberformattoset : *mut *mut core::ffi::c_void, status : *mut UErrorCode));
    unsafe { udat_adoptNumberFormatForFields(fmt as _, fields, numberformattoset as _, status as _) }
}
#[inline]
pub unsafe fn udat_applyPattern(format: *mut *mut core::ffi::c_void, localized: i8, pattern: *const u16, patternlength: i32) {
    windows_core::link!("icuin.dll" "C" fn udat_applyPattern(format : *mut *mut core::ffi::c_void, localized : i8, pattern : *const u16, patternlength : i32));
    unsafe { udat_applyPattern(format as _, localized, pattern, patternlength) }
}
#[inline]
pub unsafe fn udat_clone(fmt: *const *const core::ffi::c_void, status: *mut UErrorCode) -> *mut *mut core::ffi::c_void {
    windows_core::link!("icuin.dll" "C" fn udat_clone(fmt : *const *const core::ffi::c_void, status : *mut UErrorCode) -> *mut *mut core::ffi::c_void);
    unsafe { udat_clone(fmt, status as _) }
}
#[inline]
pub unsafe fn udat_close(format: *mut *mut core::ffi::c_void) {
    windows_core::link!("icuin.dll" "C" fn udat_close(format : *mut *mut core::ffi::c_void));
    unsafe { udat_close(format as _) }
}
#[inline]
pub unsafe fn udat_countAvailable() -> i32 {
    windows_core::link!("icuin.dll" "C" fn udat_countAvailable() -> i32);
    unsafe { udat_countAvailable() }
}
#[inline]
pub unsafe fn udat_countSymbols(fmt: *const *const core::ffi::c_void, r#type: UDateFormatSymbolType) -> i32 {
    windows_core::link!("icuin.dll" "C" fn udat_countSymbols(fmt : *const *const core::ffi::c_void, r#type : UDateFormatSymbolType) -> i32);
    unsafe { udat_countSymbols(fmt, r#type) }
}
#[inline]
pub unsafe fn udat_format(format: *const *const core::ffi::c_void, datetoformat: f64, result: *mut u16, resultlength: i32, position: *mut UFieldPosition, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn udat_format(format : *const *const core::ffi::c_void, datetoformat : f64, result : *mut u16, resultlength : i32, position : *mut UFieldPosition, status : *mut UErrorCode) -> i32);
    unsafe { udat_format(format, datetoformat, result as _, resultlength, position as _, status as _) }
}
#[inline]
pub unsafe fn udat_formatCalendar(format: *const *const core::ffi::c_void, calendar: *mut *mut core::ffi::c_void, result: *mut u16, capacity: i32, position: *mut UFieldPosition, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn udat_formatCalendar(format : *const *const core::ffi::c_void, calendar : *mut *mut core::ffi::c_void, result : *mut u16, capacity : i32, position : *mut UFieldPosition, status : *mut UErrorCode) -> i32);
    unsafe { udat_formatCalendar(format, calendar as _, result as _, capacity, position as _, status as _) }
}
#[inline]
pub unsafe fn udat_formatCalendarForFields(format: *const *const core::ffi::c_void, calendar: *mut *mut core::ffi::c_void, result: *mut u16, capacity: i32, fpositer: *mut UFieldPositionIterator, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn udat_formatCalendarForFields(format : *const *const core::ffi::c_void, calendar : *mut *mut core::ffi::c_void, result : *mut u16, capacity : i32, fpositer : *mut UFieldPositionIterator, status : *mut UErrorCode) -> i32);
    unsafe { udat_formatCalendarForFields(format, calendar as _, result as _, capacity, fpositer as _, status as _) }
}
#[inline]
pub unsafe fn udat_formatForFields(format: *const *const core::ffi::c_void, datetoformat: f64, result: *mut u16, resultlength: i32, fpositer: *mut UFieldPositionIterator, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn udat_formatForFields(format : *const *const core::ffi::c_void, datetoformat : f64, result : *mut u16, resultlength : i32, fpositer : *mut UFieldPositionIterator, status : *mut UErrorCode) -> i32);
    unsafe { udat_formatForFields(format, datetoformat, result as _, resultlength, fpositer as _, status as _) }
}
#[inline]
pub unsafe fn udat_get2DigitYearStart(fmt: *const *const core::ffi::c_void, status: *mut UErrorCode) -> f64 {
    windows_core::link!("icuin.dll" "C" fn udat_get2DigitYearStart(fmt : *const *const core::ffi::c_void, status : *mut UErrorCode) -> f64);
    unsafe { udat_get2DigitYearStart(fmt, status as _) }
}
#[inline]
pub unsafe fn udat_getAvailable(localeindex: i32) -> windows_core::PCSTR {
    windows_core::link!("icuin.dll" "C" fn udat_getAvailable(localeindex : i32) -> windows_core::PCSTR);
    unsafe { udat_getAvailable(localeindex) }
}
#[inline]
pub unsafe fn udat_getBooleanAttribute(fmt: *const *const core::ffi::c_void, attr: UDateFormatBooleanAttribute, status: *mut UErrorCode) -> i8 {
    windows_core::link!("icuin.dll" "C" fn udat_getBooleanAttribute(fmt : *const *const core::ffi::c_void, attr : UDateFormatBooleanAttribute, status : *mut UErrorCode) -> i8);
    unsafe { udat_getBooleanAttribute(fmt, attr, status as _) }
}
#[inline]
pub unsafe fn udat_getCalendar(fmt: *const *const core::ffi::c_void) -> *mut *mut core::ffi::c_void {
    windows_core::link!("icuin.dll" "C" fn udat_getCalendar(fmt : *const *const core::ffi::c_void) -> *mut *mut core::ffi::c_void);
    unsafe { udat_getCalendar(fmt) }
}
#[inline]
pub unsafe fn udat_getContext(fmt: *const *const core::ffi::c_void, r#type: UDisplayContextType, status: *mut UErrorCode) -> UDisplayContext {
    windows_core::link!("icuin.dll" "C" fn udat_getContext(fmt : *const *const core::ffi::c_void, r#type : UDisplayContextType, status : *mut UErrorCode) -> UDisplayContext);
    unsafe { udat_getContext(fmt, r#type, status as _) }
}
#[inline]
pub unsafe fn udat_getLocaleByType(fmt: *const *const core::ffi::c_void, r#type: ULocDataLocaleType, status: *mut UErrorCode) -> windows_core::PCSTR {
    windows_core::link!("icuin.dll" "C" fn udat_getLocaleByType(fmt : *const *const core::ffi::c_void, r#type : ULocDataLocaleType, status : *mut UErrorCode) -> windows_core::PCSTR);
    unsafe { udat_getLocaleByType(fmt, r#type, status as _) }
}
#[inline]
pub unsafe fn udat_getNumberFormat(fmt: *const *const core::ffi::c_void) -> *mut *mut core::ffi::c_void {
    windows_core::link!("icuin.dll" "C" fn udat_getNumberFormat(fmt : *const *const core::ffi::c_void) -> *mut *mut core::ffi::c_void);
    unsafe { udat_getNumberFormat(fmt) }
}
#[inline]
pub unsafe fn udat_getNumberFormatForField(fmt: *const *const core::ffi::c_void, field: u16) -> *mut *mut core::ffi::c_void {
    windows_core::link!("icuin.dll" "C" fn udat_getNumberFormatForField(fmt : *const *const core::ffi::c_void, field : u16) -> *mut *mut core::ffi::c_void);
    unsafe { udat_getNumberFormatForField(fmt, field) }
}
#[inline]
pub unsafe fn udat_getSymbols(fmt: *const *const core::ffi::c_void, r#type: UDateFormatSymbolType, symbolindex: i32, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn udat_getSymbols(fmt : *const *const core::ffi::c_void, r#type : UDateFormatSymbolType, symbolindex : i32, result : *mut u16, resultlength : i32, status : *mut UErrorCode) -> i32);
    unsafe { udat_getSymbols(fmt, r#type, symbolindex, result as _, resultlength, status as _) }
}
#[inline]
pub unsafe fn udat_isLenient(fmt: *const *const core::ffi::c_void) -> i8 {
    windows_core::link!("icuin.dll" "C" fn udat_isLenient(fmt : *const *const core::ffi::c_void) -> i8);
    unsafe { udat_isLenient(fmt) }
}
#[inline]
pub unsafe fn udat_open<P2>(timestyle: UDateFormatStyle, datestyle: UDateFormatStyle, locale: P2, tzid: *const u16, tzidlength: i32, pattern: *const u16, patternlength: i32, status: *mut UErrorCode) -> *mut *mut core::ffi::c_void
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuin.dll" "C" fn udat_open(timestyle : UDateFormatStyle, datestyle : UDateFormatStyle, locale : windows_core::PCSTR, tzid : *const u16, tzidlength : i32, pattern : *const u16, patternlength : i32, status : *mut UErrorCode) -> *mut *mut core::ffi::c_void);
    unsafe { udat_open(timestyle, datestyle, locale.param().abi(), tzid, tzidlength, pattern, patternlength, status as _) }
}
#[inline]
pub unsafe fn udat_parse(format: *const *const core::ffi::c_void, text: *const u16, textlength: i32, parsepos: *mut i32, status: *mut UErrorCode) -> f64 {
    windows_core::link!("icuin.dll" "C" fn udat_parse(format : *const *const core::ffi::c_void, text : *const u16, textlength : i32, parsepos : *mut i32, status : *mut UErrorCode) -> f64);
    unsafe { udat_parse(format, text, textlength, parsepos as _, status as _) }
}
#[inline]
pub unsafe fn udat_parseCalendar(format: *const *const core::ffi::c_void, calendar: *mut *mut core::ffi::c_void, text: *const u16, textlength: i32, parsepos: *mut i32, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn udat_parseCalendar(format : *const *const core::ffi::c_void, calendar : *mut *mut core::ffi::c_void, text : *const u16, textlength : i32, parsepos : *mut i32, status : *mut UErrorCode));
    unsafe { udat_parseCalendar(format, calendar as _, text, textlength, parsepos as _, status as _) }
}
#[inline]
pub unsafe fn udat_set2DigitYearStart(fmt: *mut *mut core::ffi::c_void, d: f64, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn udat_set2DigitYearStart(fmt : *mut *mut core::ffi::c_void, d : f64, status : *mut UErrorCode));
    unsafe { udat_set2DigitYearStart(fmt as _, d, status as _) }
}
#[inline]
pub unsafe fn udat_setBooleanAttribute(fmt: *mut *mut core::ffi::c_void, attr: UDateFormatBooleanAttribute, newvalue: i8, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn udat_setBooleanAttribute(fmt : *mut *mut core::ffi::c_void, attr : UDateFormatBooleanAttribute, newvalue : i8, status : *mut UErrorCode));
    unsafe { udat_setBooleanAttribute(fmt as _, attr, newvalue, status as _) }
}
#[inline]
pub unsafe fn udat_setCalendar(fmt: *mut *mut core::ffi::c_void, calendartoset: *const *const core::ffi::c_void) {
    windows_core::link!("icuin.dll" "C" fn udat_setCalendar(fmt : *mut *mut core::ffi::c_void, calendartoset : *const *const core::ffi::c_void));
    unsafe { udat_setCalendar(fmt as _, calendartoset) }
}
#[inline]
pub unsafe fn udat_setContext(fmt: *mut *mut core::ffi::c_void, value: UDisplayContext, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn udat_setContext(fmt : *mut *mut core::ffi::c_void, value : UDisplayContext, status : *mut UErrorCode));
    unsafe { udat_setContext(fmt as _, value, status as _) }
}
#[inline]
pub unsafe fn udat_setLenient(fmt: *mut *mut core::ffi::c_void, islenient: i8) {
    windows_core::link!("icuin.dll" "C" fn udat_setLenient(fmt : *mut *mut core::ffi::c_void, islenient : i8));
    unsafe { udat_setLenient(fmt as _, islenient) }
}
#[inline]
pub unsafe fn udat_setNumberFormat(fmt: *mut *mut core::ffi::c_void, numberformattoset: *const *const core::ffi::c_void) {
    windows_core::link!("icuin.dll" "C" fn udat_setNumberFormat(fmt : *mut *mut core::ffi::c_void, numberformattoset : *const *const core::ffi::c_void));
    unsafe { udat_setNumberFormat(fmt as _, numberformattoset) }
}
#[inline]
pub unsafe fn udat_setSymbols(format: *mut *mut core::ffi::c_void, r#type: UDateFormatSymbolType, symbolindex: i32, value: *mut u16, valuelength: i32, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn udat_setSymbols(format : *mut *mut core::ffi::c_void, r#type : UDateFormatSymbolType, symbolindex : i32, value : *mut u16, valuelength : i32, status : *mut UErrorCode));
    unsafe { udat_setSymbols(format as _, r#type, symbolindex, value as _, valuelength, status as _) }
}
#[inline]
pub unsafe fn udat_toCalendarDateField(field: UDateFormatField) -> UCalendarDateFields {
    windows_core::link!("icuin.dll" "C" fn udat_toCalendarDateField(field : UDateFormatField) -> UCalendarDateFields);
    unsafe { udat_toCalendarDateField(field) }
}
#[inline]
pub unsafe fn udat_toPattern(fmt: *const *const core::ffi::c_void, localized: i8, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn udat_toPattern(fmt : *const *const core::ffi::c_void, localized : i8, result : *mut u16, resultlength : i32, status : *mut UErrorCode) -> i32);
    unsafe { udat_toPattern(fmt, localized, result as _, resultlength, status as _) }
}
#[inline]
pub unsafe fn udatpg_addPattern(dtpg: *mut *mut core::ffi::c_void, pattern: *const u16, patternlength: i32, r#override: i8, conflictingpattern: *mut u16, capacity: i32, plength: *mut i32, perrorcode: *mut UErrorCode) -> UDateTimePatternConflict {
    windows_core::link!("icuin.dll" "C" fn udatpg_addPattern(dtpg : *mut *mut core::ffi::c_void, pattern : *const u16, patternlength : i32, r#override : i8, conflictingpattern : *mut u16, capacity : i32, plength : *mut i32, perrorcode : *mut UErrorCode) -> UDateTimePatternConflict);
    unsafe { udatpg_addPattern(dtpg as _, pattern, patternlength, r#override, conflictingpattern as _, capacity, plength as _, perrorcode as _) }
}
#[inline]
pub unsafe fn udatpg_clone(dtpg: *const *const core::ffi::c_void, perrorcode: *mut UErrorCode) -> *mut *mut core::ffi::c_void {
    windows_core::link!("icuin.dll" "C" fn udatpg_clone(dtpg : *const *const core::ffi::c_void, perrorcode : *mut UErrorCode) -> *mut *mut core::ffi::c_void);
    unsafe { udatpg_clone(dtpg, perrorcode as _) }
}
#[inline]
pub unsafe fn udatpg_close(dtpg: *mut *mut core::ffi::c_void) {
    windows_core::link!("icuin.dll" "C" fn udatpg_close(dtpg : *mut *mut core::ffi::c_void));
    unsafe { udatpg_close(dtpg as _) }
}
#[inline]
pub unsafe fn udatpg_getAppendItemFormat(dtpg: *const *const core::ffi::c_void, field: UDateTimePatternField, plength: *mut i32) -> *mut u16 {
    windows_core::link!("icuin.dll" "C" fn udatpg_getAppendItemFormat(dtpg : *const *const core::ffi::c_void, field : UDateTimePatternField, plength : *mut i32) -> *mut u16);
    unsafe { udatpg_getAppendItemFormat(dtpg, field, plength as _) }
}
#[inline]
pub unsafe fn udatpg_getAppendItemName(dtpg: *const *const core::ffi::c_void, field: UDateTimePatternField, plength: *mut i32) -> *mut u16 {
    windows_core::link!("icuin.dll" "C" fn udatpg_getAppendItemName(dtpg : *const *const core::ffi::c_void, field : UDateTimePatternField, plength : *mut i32) -> *mut u16);
    unsafe { udatpg_getAppendItemName(dtpg, field, plength as _) }
}
#[inline]
pub unsafe fn udatpg_getBaseSkeleton(unuseddtpg: *mut *mut core::ffi::c_void, pattern: *const u16, length: i32, baseskeleton: *mut u16, capacity: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn udatpg_getBaseSkeleton(unuseddtpg : *mut *mut core::ffi::c_void, pattern : *const u16, length : i32, baseskeleton : *mut u16, capacity : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { udatpg_getBaseSkeleton(unuseddtpg as _, pattern, length, baseskeleton as _, capacity, perrorcode as _) }
}
#[inline]
pub unsafe fn udatpg_getBestPattern(dtpg: *mut *mut core::ffi::c_void, skeleton: *const u16, length: i32, bestpattern: *mut u16, capacity: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn udatpg_getBestPattern(dtpg : *mut *mut core::ffi::c_void, skeleton : *const u16, length : i32, bestpattern : *mut u16, capacity : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { udatpg_getBestPattern(dtpg as _, skeleton, length, bestpattern as _, capacity, perrorcode as _) }
}
#[inline]
pub unsafe fn udatpg_getBestPatternWithOptions(dtpg: *mut *mut core::ffi::c_void, skeleton: *const u16, length: i32, options: UDateTimePatternMatchOptions, bestpattern: *mut u16, capacity: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn udatpg_getBestPatternWithOptions(dtpg : *mut *mut core::ffi::c_void, skeleton : *const u16, length : i32, options : UDateTimePatternMatchOptions, bestpattern : *mut u16, capacity : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { udatpg_getBestPatternWithOptions(dtpg as _, skeleton, length, options, bestpattern as _, capacity, perrorcode as _) }
}
#[inline]
pub unsafe fn udatpg_getDateTimeFormat(dtpg: *const *const core::ffi::c_void, plength: *mut i32) -> *mut u16 {
    windows_core::link!("icuin.dll" "C" fn udatpg_getDateTimeFormat(dtpg : *const *const core::ffi::c_void, plength : *mut i32) -> *mut u16);
    unsafe { udatpg_getDateTimeFormat(dtpg, plength as _) }
}
#[inline]
pub unsafe fn udatpg_getDecimal(dtpg: *const *const core::ffi::c_void, plength: *mut i32) -> *mut u16 {
    windows_core::link!("icuin.dll" "C" fn udatpg_getDecimal(dtpg : *const *const core::ffi::c_void, plength : *mut i32) -> *mut u16);
    unsafe { udatpg_getDecimal(dtpg, plength as _) }
}
#[inline]
pub unsafe fn udatpg_getFieldDisplayName(dtpg: *const *const core::ffi::c_void, field: UDateTimePatternField, width: UDateTimePGDisplayWidth, fieldname: *mut u16, capacity: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icu.dll" "C" fn udatpg_getFieldDisplayName(dtpg : *const *const core::ffi::c_void, field : UDateTimePatternField, width : UDateTimePGDisplayWidth, fieldname : *mut u16, capacity : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { udatpg_getFieldDisplayName(dtpg, field, width, fieldname as _, capacity, perrorcode as _) }
}
#[inline]
pub unsafe fn udatpg_getPatternForSkeleton(dtpg: *const *const core::ffi::c_void, skeleton: *const u16, skeletonlength: i32, plength: *mut i32) -> *mut u16 {
    windows_core::link!("icuin.dll" "C" fn udatpg_getPatternForSkeleton(dtpg : *const *const core::ffi::c_void, skeleton : *const u16, skeletonlength : i32, plength : *mut i32) -> *mut u16);
    unsafe { udatpg_getPatternForSkeleton(dtpg, skeleton, skeletonlength, plength as _) }
}
#[inline]
pub unsafe fn udatpg_getSkeleton(unuseddtpg: *mut *mut core::ffi::c_void, pattern: *const u16, length: i32, skeleton: *mut u16, capacity: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn udatpg_getSkeleton(unuseddtpg : *mut *mut core::ffi::c_void, pattern : *const u16, length : i32, skeleton : *mut u16, capacity : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { udatpg_getSkeleton(unuseddtpg as _, pattern, length, skeleton as _, capacity, perrorcode as _) }
}
#[inline]
pub unsafe fn udatpg_open<P0>(locale: P0, perrorcode: *mut UErrorCode) -> *mut *mut core::ffi::c_void
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuin.dll" "C" fn udatpg_open(locale : windows_core::PCSTR, perrorcode : *mut UErrorCode) -> *mut *mut core::ffi::c_void);
    unsafe { udatpg_open(locale.param().abi(), perrorcode as _) }
}
#[inline]
pub unsafe fn udatpg_openBaseSkeletons(dtpg: *const *const core::ffi::c_void, perrorcode: *mut UErrorCode) -> *mut UEnumeration {
    windows_core::link!("icuin.dll" "C" fn udatpg_openBaseSkeletons(dtpg : *const *const core::ffi::c_void, perrorcode : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { udatpg_openBaseSkeletons(dtpg, perrorcode as _) }
}
#[inline]
pub unsafe fn udatpg_openEmpty(perrorcode: *mut UErrorCode) -> *mut *mut core::ffi::c_void {
    windows_core::link!("icuin.dll" "C" fn udatpg_openEmpty(perrorcode : *mut UErrorCode) -> *mut *mut core::ffi::c_void);
    unsafe { udatpg_openEmpty(perrorcode as _) }
}
#[inline]
pub unsafe fn udatpg_openSkeletons(dtpg: *const *const core::ffi::c_void, perrorcode: *mut UErrorCode) -> *mut UEnumeration {
    windows_core::link!("icuin.dll" "C" fn udatpg_openSkeletons(dtpg : *const *const core::ffi::c_void, perrorcode : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { udatpg_openSkeletons(dtpg, perrorcode as _) }
}
#[inline]
pub unsafe fn udatpg_replaceFieldTypes(dtpg: *mut *mut core::ffi::c_void, pattern: *const u16, patternlength: i32, skeleton: *const u16, skeletonlength: i32, dest: *mut u16, destcapacity: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn udatpg_replaceFieldTypes(dtpg : *mut *mut core::ffi::c_void, pattern : *const u16, patternlength : i32, skeleton : *const u16, skeletonlength : i32, dest : *mut u16, destcapacity : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { udatpg_replaceFieldTypes(dtpg as _, pattern, patternlength, skeleton, skeletonlength, dest as _, destcapacity, perrorcode as _) }
}
#[inline]
pub unsafe fn udatpg_replaceFieldTypesWithOptions(dtpg: *mut *mut core::ffi::c_void, pattern: *const u16, patternlength: i32, skeleton: *const u16, skeletonlength: i32, options: UDateTimePatternMatchOptions, dest: *mut u16, destcapacity: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn udatpg_replaceFieldTypesWithOptions(dtpg : *mut *mut core::ffi::c_void, pattern : *const u16, patternlength : i32, skeleton : *const u16, skeletonlength : i32, options : UDateTimePatternMatchOptions, dest : *mut u16, destcapacity : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { udatpg_replaceFieldTypesWithOptions(dtpg as _, pattern, patternlength, skeleton, skeletonlength, options, dest as _, destcapacity, perrorcode as _) }
}
#[inline]
pub unsafe fn udatpg_setAppendItemFormat(dtpg: *mut *mut core::ffi::c_void, field: UDateTimePatternField, value: *const u16, length: i32) {
    windows_core::link!("icuin.dll" "C" fn udatpg_setAppendItemFormat(dtpg : *mut *mut core::ffi::c_void, field : UDateTimePatternField, value : *const u16, length : i32));
    unsafe { udatpg_setAppendItemFormat(dtpg as _, field, value, length) }
}
#[inline]
pub unsafe fn udatpg_setAppendItemName(dtpg: *mut *mut core::ffi::c_void, field: UDateTimePatternField, value: *const u16, length: i32) {
    windows_core::link!("icuin.dll" "C" fn udatpg_setAppendItemName(dtpg : *mut *mut core::ffi::c_void, field : UDateTimePatternField, value : *const u16, length : i32));
    unsafe { udatpg_setAppendItemName(dtpg as _, field, value, length) }
}
#[inline]
pub unsafe fn udatpg_setDateTimeFormat(dtpg: *const *const core::ffi::c_void, dtformat: *const u16, length: i32) {
    windows_core::link!("icuin.dll" "C" fn udatpg_setDateTimeFormat(dtpg : *const *const core::ffi::c_void, dtformat : *const u16, length : i32));
    unsafe { udatpg_setDateTimeFormat(dtpg, dtformat, length) }
}
#[inline]
pub unsafe fn udatpg_setDecimal(dtpg: *mut *mut core::ffi::c_void, decimal: *const u16, length: i32) {
    windows_core::link!("icuin.dll" "C" fn udatpg_setDecimal(dtpg : *mut *mut core::ffi::c_void, decimal : *const u16, length : i32));
    unsafe { udatpg_setDecimal(dtpg as _, decimal, length) }
}
#[inline]
pub unsafe fn udtitvfmt_close(formatter: *mut UDateIntervalFormat) {
    windows_core::link!("icuin.dll" "C" fn udtitvfmt_close(formatter : *mut UDateIntervalFormat));
    unsafe { udtitvfmt_close(formatter as _) }
}
#[inline]
pub unsafe fn udtitvfmt_closeResult(uresult: *mut UFormattedDateInterval) {
    windows_core::link!("icu.dll" "C" fn udtitvfmt_closeResult(uresult : *mut UFormattedDateInterval));
    unsafe { udtitvfmt_closeResult(uresult as _) }
}
#[inline]
pub unsafe fn udtitvfmt_format(formatter: *const UDateIntervalFormat, fromdate: f64, todate: f64, result: *mut u16, resultcapacity: i32, position: *mut UFieldPosition, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn udtitvfmt_format(formatter : *const UDateIntervalFormat, fromdate : f64, todate : f64, result : *mut u16, resultcapacity : i32, position : *mut UFieldPosition, status : *mut UErrorCode) -> i32);
    unsafe { udtitvfmt_format(formatter, fromdate, todate, result as _, resultcapacity, position as _, status as _) }
}
#[inline]
pub unsafe fn udtitvfmt_open<P0>(locale: P0, skeleton: *const u16, skeletonlength: i32, tzid: *const u16, tzidlength: i32, status: *mut UErrorCode) -> *mut UDateIntervalFormat
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuin.dll" "C" fn udtitvfmt_open(locale : windows_core::PCSTR, skeleton : *const u16, skeletonlength : i32, tzid : *const u16, tzidlength : i32, status : *mut UErrorCode) -> *mut UDateIntervalFormat);
    unsafe { udtitvfmt_open(locale.param().abi(), skeleton, skeletonlength, tzid, tzidlength, status as _) }
}
#[inline]
pub unsafe fn udtitvfmt_openResult(ec: *mut UErrorCode) -> *mut UFormattedDateInterval {
    windows_core::link!("icu.dll" "C" fn udtitvfmt_openResult(ec : *mut UErrorCode) -> *mut UFormattedDateInterval);
    unsafe { udtitvfmt_openResult(ec as _) }
}
#[inline]
pub unsafe fn udtitvfmt_resultAsValue(uresult: *const UFormattedDateInterval, ec: *mut UErrorCode) -> *mut UFormattedValue {
    windows_core::link!("icu.dll" "C" fn udtitvfmt_resultAsValue(uresult : *const UFormattedDateInterval, ec : *mut UErrorCode) -> *mut UFormattedValue);
    unsafe { udtitvfmt_resultAsValue(uresult, ec as _) }
}
#[inline]
pub unsafe fn uenum_close(en: *mut UEnumeration) {
    windows_core::link!("icuuc.dll" "C" fn uenum_close(en : *mut UEnumeration));
    unsafe { uenum_close(en as _) }
}
#[inline]
pub unsafe fn uenum_count(en: *mut UEnumeration, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uenum_count(en : *mut UEnumeration, status : *mut UErrorCode) -> i32);
    unsafe { uenum_count(en as _, status as _) }
}
#[inline]
pub unsafe fn uenum_next(en: *mut UEnumeration, resultlength: *mut i32, status: *mut UErrorCode) -> windows_core::PCSTR {
    windows_core::link!("icuuc.dll" "C" fn uenum_next(en : *mut UEnumeration, resultlength : *mut i32, status : *mut UErrorCode) -> windows_core::PCSTR);
    unsafe { uenum_next(en as _, resultlength as _, status as _) }
}
#[inline]
pub unsafe fn uenum_openCharStringsEnumeration(strings: *const *const i8, count: i32, ec: *mut UErrorCode) -> *mut UEnumeration {
    windows_core::link!("icuuc.dll" "C" fn uenum_openCharStringsEnumeration(strings : *const *const i8, count : i32, ec : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { uenum_openCharStringsEnumeration(strings, count, ec as _) }
}
#[inline]
pub unsafe fn uenum_openUCharStringsEnumeration(strings: *const *const u16, count: i32, ec: *mut UErrorCode) -> *mut UEnumeration {
    windows_core::link!("icuuc.dll" "C" fn uenum_openUCharStringsEnumeration(strings : *const *const u16, count : i32, ec : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { uenum_openUCharStringsEnumeration(strings, count, ec as _) }
}
#[inline]
pub unsafe fn uenum_reset(en: *mut UEnumeration, status: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn uenum_reset(en : *mut UEnumeration, status : *mut UErrorCode));
    unsafe { uenum_reset(en as _, status as _) }
}
#[inline]
pub unsafe fn uenum_unext(en: *mut UEnumeration, resultlength: *mut i32, status: *mut UErrorCode) -> *mut u16 {
    windows_core::link!("icuuc.dll" "C" fn uenum_unext(en : *mut UEnumeration, resultlength : *mut i32, status : *mut UErrorCode) -> *mut u16);
    unsafe { uenum_unext(en as _, resultlength as _, status as _) }
}
#[inline]
pub unsafe fn ufieldpositer_close(fpositer: *mut UFieldPositionIterator) {
    windows_core::link!("icuin.dll" "C" fn ufieldpositer_close(fpositer : *mut UFieldPositionIterator));
    unsafe { ufieldpositer_close(fpositer as _) }
}
#[inline]
pub unsafe fn ufieldpositer_next(fpositer: *mut UFieldPositionIterator, beginindex: *mut i32, endindex: *mut i32) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ufieldpositer_next(fpositer : *mut UFieldPositionIterator, beginindex : *mut i32, endindex : *mut i32) -> i32);
    unsafe { ufieldpositer_next(fpositer as _, beginindex as _, endindex as _) }
}
#[inline]
pub unsafe fn ufieldpositer_open(status: *mut UErrorCode) -> *mut UFieldPositionIterator {
    windows_core::link!("icuin.dll" "C" fn ufieldpositer_open(status : *mut UErrorCode) -> *mut UFieldPositionIterator);
    unsafe { ufieldpositer_open(status as _) }
}
#[inline]
pub unsafe fn ufmt_close(fmt: *mut *mut core::ffi::c_void) {
    windows_core::link!("icuin.dll" "C" fn ufmt_close(fmt : *mut *mut core::ffi::c_void));
    unsafe { ufmt_close(fmt as _) }
}
#[inline]
pub unsafe fn ufmt_getArrayItemByIndex(fmt: *mut *mut core::ffi::c_void, n: i32, status: *mut UErrorCode) -> *mut *mut core::ffi::c_void {
    windows_core::link!("icuin.dll" "C" fn ufmt_getArrayItemByIndex(fmt : *mut *mut core::ffi::c_void, n : i32, status : *mut UErrorCode) -> *mut *mut core::ffi::c_void);
    unsafe { ufmt_getArrayItemByIndex(fmt as _, n, status as _) }
}
#[inline]
pub unsafe fn ufmt_getArrayLength(fmt: *const *const core::ffi::c_void, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ufmt_getArrayLength(fmt : *const *const core::ffi::c_void, status : *mut UErrorCode) -> i32);
    unsafe { ufmt_getArrayLength(fmt, status as _) }
}
#[inline]
pub unsafe fn ufmt_getDate(fmt: *const *const core::ffi::c_void, status: *mut UErrorCode) -> f64 {
    windows_core::link!("icuin.dll" "C" fn ufmt_getDate(fmt : *const *const core::ffi::c_void, status : *mut UErrorCode) -> f64);
    unsafe { ufmt_getDate(fmt, status as _) }
}
#[inline]
pub unsafe fn ufmt_getDecNumChars(fmt: *mut *mut core::ffi::c_void, len: *mut i32, status: *mut UErrorCode) -> windows_core::PCSTR {
    windows_core::link!("icuin.dll" "C" fn ufmt_getDecNumChars(fmt : *mut *mut core::ffi::c_void, len : *mut i32, status : *mut UErrorCode) -> windows_core::PCSTR);
    unsafe { ufmt_getDecNumChars(fmt as _, len as _, status as _) }
}
#[inline]
pub unsafe fn ufmt_getDouble(fmt: *mut *mut core::ffi::c_void, status: *mut UErrorCode) -> f64 {
    windows_core::link!("icuin.dll" "C" fn ufmt_getDouble(fmt : *mut *mut core::ffi::c_void, status : *mut UErrorCode) -> f64);
    unsafe { ufmt_getDouble(fmt as _, status as _) }
}
#[inline]
pub unsafe fn ufmt_getInt64(fmt: *mut *mut core::ffi::c_void, status: *mut UErrorCode) -> i64 {
    windows_core::link!("icuin.dll" "C" fn ufmt_getInt64(fmt : *mut *mut core::ffi::c_void, status : *mut UErrorCode) -> i64);
    unsafe { ufmt_getInt64(fmt as _, status as _) }
}
#[inline]
pub unsafe fn ufmt_getLong(fmt: *mut *mut core::ffi::c_void, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ufmt_getLong(fmt : *mut *mut core::ffi::c_void, status : *mut UErrorCode) -> i32);
    unsafe { ufmt_getLong(fmt as _, status as _) }
}
#[inline]
pub unsafe fn ufmt_getObject(fmt: *const *const core::ffi::c_void, status: *mut UErrorCode) -> *mut core::ffi::c_void {
    windows_core::link!("icuin.dll" "C" fn ufmt_getObject(fmt : *const *const core::ffi::c_void, status : *mut UErrorCode) -> *mut core::ffi::c_void);
    unsafe { ufmt_getObject(fmt, status as _) }
}
#[inline]
pub unsafe fn ufmt_getType(fmt: *const *const core::ffi::c_void, status: *mut UErrorCode) -> UFormattableType {
    windows_core::link!("icuin.dll" "C" fn ufmt_getType(fmt : *const *const core::ffi::c_void, status : *mut UErrorCode) -> UFormattableType);
    unsafe { ufmt_getType(fmt, status as _) }
}
#[inline]
pub unsafe fn ufmt_getUChars(fmt: *mut *mut core::ffi::c_void, len: *mut i32, status: *mut UErrorCode) -> *mut u16 {
    windows_core::link!("icuin.dll" "C" fn ufmt_getUChars(fmt : *mut *mut core::ffi::c_void, len : *mut i32, status : *mut UErrorCode) -> *mut u16);
    unsafe { ufmt_getUChars(fmt as _, len as _, status as _) }
}
#[inline]
pub unsafe fn ufmt_isNumeric(fmt: *const *const core::ffi::c_void) -> i8 {
    windows_core::link!("icuin.dll" "C" fn ufmt_isNumeric(fmt : *const *const core::ffi::c_void) -> i8);
    unsafe { ufmt_isNumeric(fmt) }
}
#[inline]
pub unsafe fn ufmt_open(status: *mut UErrorCode) -> *mut *mut core::ffi::c_void {
    windows_core::link!("icuin.dll" "C" fn ufmt_open(status : *mut UErrorCode) -> *mut *mut core::ffi::c_void);
    unsafe { ufmt_open(status as _) }
}
#[inline]
pub unsafe fn ufmtval_getString(ufmtval: *const UFormattedValue, plength: *mut i32, ec: *mut UErrorCode) -> *mut u16 {
    windows_core::link!("icu.dll" "C" fn ufmtval_getString(ufmtval : *const UFormattedValue, plength : *mut i32, ec : *mut UErrorCode) -> *mut u16);
    unsafe { ufmtval_getString(ufmtval, plength as _, ec as _) }
}
#[inline]
pub unsafe fn ufmtval_nextPosition(ufmtval: *const UFormattedValue, ucfpos: *mut UConstrainedFieldPosition, ec: *mut UErrorCode) -> i8 {
    windows_core::link!("icu.dll" "C" fn ufmtval_nextPosition(ufmtval : *const UFormattedValue, ucfpos : *mut UConstrainedFieldPosition, ec : *mut UErrorCode) -> i8);
    unsafe { ufmtval_nextPosition(ufmtval, ucfpos as _, ec as _) }
}
#[inline]
pub unsafe fn ugender_getInstance<P0>(locale: P0, status: *mut UErrorCode) -> *mut UGenderInfo
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuin.dll" "C" fn ugender_getInstance(locale : windows_core::PCSTR, status : *mut UErrorCode) -> *mut UGenderInfo);
    unsafe { ugender_getInstance(locale.param().abi(), status as _) }
}
#[inline]
pub unsafe fn ugender_getListGender(genderinfo: *const UGenderInfo, genders: *const UGender, size: i32, status: *mut UErrorCode) -> UGender {
    windows_core::link!("icuin.dll" "C" fn ugender_getListGender(genderinfo : *const UGenderInfo, genders : *const UGender, size : i32, status : *mut UErrorCode) -> UGender);
    unsafe { ugender_getListGender(genderinfo, genders, size, status as _) }
}
#[inline]
pub unsafe fn uidna_close(idna: *mut UIDNA) {
    windows_core::link!("icuuc.dll" "C" fn uidna_close(idna : *mut UIDNA));
    unsafe { uidna_close(idna as _) }
}
#[inline]
pub unsafe fn uidna_labelToASCII(idna: *const UIDNA, label: *const u16, length: i32, dest: *mut u16, capacity: i32, pinfo: *mut UIDNAInfo, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uidna_labelToASCII(idna : *const UIDNA, label : *const u16, length : i32, dest : *mut u16, capacity : i32, pinfo : *mut UIDNAInfo, perrorcode : *mut UErrorCode) -> i32);
    unsafe { uidna_labelToASCII(idna, label, length, dest as _, capacity, pinfo as _, perrorcode as _) }
}
#[inline]
pub unsafe fn uidna_labelToASCII_UTF8<P1, P3>(idna: *const UIDNA, label: P1, length: i32, dest: P3, capacity: i32, pinfo: *mut UIDNAInfo, perrorcode: *mut UErrorCode) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uidna_labelToASCII_UTF8(idna : *const UIDNA, label : windows_core::PCSTR, length : i32, dest : windows_core::PCSTR, capacity : i32, pinfo : *mut UIDNAInfo, perrorcode : *mut UErrorCode) -> i32);
    unsafe { uidna_labelToASCII_UTF8(idna, label.param().abi(), length, dest.param().abi(), capacity, pinfo as _, perrorcode as _) }
}
#[inline]
pub unsafe fn uidna_labelToUnicode(idna: *const UIDNA, label: *const u16, length: i32, dest: *mut u16, capacity: i32, pinfo: *mut UIDNAInfo, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uidna_labelToUnicode(idna : *const UIDNA, label : *const u16, length : i32, dest : *mut u16, capacity : i32, pinfo : *mut UIDNAInfo, perrorcode : *mut UErrorCode) -> i32);
    unsafe { uidna_labelToUnicode(idna, label, length, dest as _, capacity, pinfo as _, perrorcode as _) }
}
#[inline]
pub unsafe fn uidna_labelToUnicodeUTF8<P1, P3>(idna: *const UIDNA, label: P1, length: i32, dest: P3, capacity: i32, pinfo: *mut UIDNAInfo, perrorcode: *mut UErrorCode) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uidna_labelToUnicodeUTF8(idna : *const UIDNA, label : windows_core::PCSTR, length : i32, dest : windows_core::PCSTR, capacity : i32, pinfo : *mut UIDNAInfo, perrorcode : *mut UErrorCode) -> i32);
    unsafe { uidna_labelToUnicodeUTF8(idna, label.param().abi(), length, dest.param().abi(), capacity, pinfo as _, perrorcode as _) }
}
#[inline]
pub unsafe fn uidna_nameToASCII(idna: *const UIDNA, name: *const u16, length: i32, dest: *mut u16, capacity: i32, pinfo: *mut UIDNAInfo, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uidna_nameToASCII(idna : *const UIDNA, name : *const u16, length : i32, dest : *mut u16, capacity : i32, pinfo : *mut UIDNAInfo, perrorcode : *mut UErrorCode) -> i32);
    unsafe { uidna_nameToASCII(idna, name, length, dest as _, capacity, pinfo as _, perrorcode as _) }
}
#[inline]
pub unsafe fn uidna_nameToASCII_UTF8<P1, P3>(idna: *const UIDNA, name: P1, length: i32, dest: P3, capacity: i32, pinfo: *mut UIDNAInfo, perrorcode: *mut UErrorCode) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uidna_nameToASCII_UTF8(idna : *const UIDNA, name : windows_core::PCSTR, length : i32, dest : windows_core::PCSTR, capacity : i32, pinfo : *mut UIDNAInfo, perrorcode : *mut UErrorCode) -> i32);
    unsafe { uidna_nameToASCII_UTF8(idna, name.param().abi(), length, dest.param().abi(), capacity, pinfo as _, perrorcode as _) }
}
#[inline]
pub unsafe fn uidna_nameToUnicode(idna: *const UIDNA, name: *const u16, length: i32, dest: *mut u16, capacity: i32, pinfo: *mut UIDNAInfo, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uidna_nameToUnicode(idna : *const UIDNA, name : *const u16, length : i32, dest : *mut u16, capacity : i32, pinfo : *mut UIDNAInfo, perrorcode : *mut UErrorCode) -> i32);
    unsafe { uidna_nameToUnicode(idna, name, length, dest as _, capacity, pinfo as _, perrorcode as _) }
}
#[inline]
pub unsafe fn uidna_nameToUnicodeUTF8<P1, P3>(idna: *const UIDNA, name: P1, length: i32, dest: P3, capacity: i32, pinfo: *mut UIDNAInfo, perrorcode: *mut UErrorCode) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uidna_nameToUnicodeUTF8(idna : *const UIDNA, name : windows_core::PCSTR, length : i32, dest : windows_core::PCSTR, capacity : i32, pinfo : *mut UIDNAInfo, perrorcode : *mut UErrorCode) -> i32);
    unsafe { uidna_nameToUnicodeUTF8(idna, name.param().abi(), length, dest.param().abi(), capacity, pinfo as _, perrorcode as _) }
}
#[inline]
pub unsafe fn uidna_openUTS46(options: u32, perrorcode: *mut UErrorCode) -> *mut UIDNA {
    windows_core::link!("icuuc.dll" "C" fn uidna_openUTS46(options : u32, perrorcode : *mut UErrorCode) -> *mut UIDNA);
    unsafe { uidna_openUTS46(options, perrorcode as _) }
}
#[inline]
pub unsafe fn uiter_current32(iter: *mut UCharIterator) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uiter_current32(iter : *mut UCharIterator) -> i32);
    unsafe { uiter_current32(iter as _) }
}
#[inline]
pub unsafe fn uiter_getState(iter: *const UCharIterator) -> u32 {
    windows_core::link!("icuuc.dll" "C" fn uiter_getState(iter : *const UCharIterator) -> u32);
    unsafe { uiter_getState(iter) }
}
#[inline]
pub unsafe fn uiter_next32(iter: *mut UCharIterator) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uiter_next32(iter : *mut UCharIterator) -> i32);
    unsafe { uiter_next32(iter as _) }
}
#[inline]
pub unsafe fn uiter_previous32(iter: *mut UCharIterator) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uiter_previous32(iter : *mut UCharIterator) -> i32);
    unsafe { uiter_previous32(iter as _) }
}
#[inline]
pub unsafe fn uiter_setState(iter: *mut UCharIterator, state: u32, perrorcode: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn uiter_setState(iter : *mut UCharIterator, state : u32, perrorcode : *mut UErrorCode));
    unsafe { uiter_setState(iter as _, state, perrorcode as _) }
}
#[inline]
pub unsafe fn uiter_setString(iter: *mut UCharIterator, s: *const u16, length: i32) {
    windows_core::link!("icuuc.dll" "C" fn uiter_setString(iter : *mut UCharIterator, s : *const u16, length : i32));
    unsafe { uiter_setString(iter as _, s, length) }
}
#[inline]
pub unsafe fn uiter_setUTF16BE<P1>(iter: *mut UCharIterator, s: P1, length: i32)
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uiter_setUTF16BE(iter : *mut UCharIterator, s : windows_core::PCSTR, length : i32));
    unsafe { uiter_setUTF16BE(iter as _, s.param().abi(), length) }
}
#[inline]
pub unsafe fn uiter_setUTF8<P1>(iter: *mut UCharIterator, s: P1, length: i32)
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uiter_setUTF8(iter : *mut UCharIterator, s : windows_core::PCSTR, length : i32));
    unsafe { uiter_setUTF8(iter as _, s.param().abi(), length) }
}
#[inline]
pub unsafe fn uldn_close(ldn: *mut ULocaleDisplayNames) {
    windows_core::link!("icuuc.dll" "C" fn uldn_close(ldn : *mut ULocaleDisplayNames));
    unsafe { uldn_close(ldn as _) }
}
#[inline]
pub unsafe fn uldn_getContext(ldn: *const ULocaleDisplayNames, r#type: UDisplayContextType, perrorcode: *mut UErrorCode) -> UDisplayContext {
    windows_core::link!("icuuc.dll" "C" fn uldn_getContext(ldn : *const ULocaleDisplayNames, r#type : UDisplayContextType, perrorcode : *mut UErrorCode) -> UDisplayContext);
    unsafe { uldn_getContext(ldn, r#type, perrorcode as _) }
}
#[inline]
pub unsafe fn uldn_getDialectHandling(ldn: *const ULocaleDisplayNames) -> UDialectHandling {
    windows_core::link!("icuuc.dll" "C" fn uldn_getDialectHandling(ldn : *const ULocaleDisplayNames) -> UDialectHandling);
    unsafe { uldn_getDialectHandling(ldn) }
}
#[inline]
pub unsafe fn uldn_getLocale(ldn: *const ULocaleDisplayNames) -> windows_core::PCSTR {
    windows_core::link!("icuuc.dll" "C" fn uldn_getLocale(ldn : *const ULocaleDisplayNames) -> windows_core::PCSTR);
    unsafe { uldn_getLocale(ldn) }
}
#[inline]
pub unsafe fn uldn_keyDisplayName<P1>(ldn: *const ULocaleDisplayNames, key: P1, result: *mut u16, maxresultsize: i32, perrorcode: *mut UErrorCode) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uldn_keyDisplayName(ldn : *const ULocaleDisplayNames, key : windows_core::PCSTR, result : *mut u16, maxresultsize : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { uldn_keyDisplayName(ldn, key.param().abi(), result as _, maxresultsize, perrorcode as _) }
}
#[inline]
pub unsafe fn uldn_keyValueDisplayName<P1, P2>(ldn: *const ULocaleDisplayNames, key: P1, value: P2, result: *mut u16, maxresultsize: i32, perrorcode: *mut UErrorCode) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uldn_keyValueDisplayName(ldn : *const ULocaleDisplayNames, key : windows_core::PCSTR, value : windows_core::PCSTR, result : *mut u16, maxresultsize : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { uldn_keyValueDisplayName(ldn, key.param().abi(), value.param().abi(), result as _, maxresultsize, perrorcode as _) }
}
#[inline]
pub unsafe fn uldn_languageDisplayName<P1>(ldn: *const ULocaleDisplayNames, lang: P1, result: *mut u16, maxresultsize: i32, perrorcode: *mut UErrorCode) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uldn_languageDisplayName(ldn : *const ULocaleDisplayNames, lang : windows_core::PCSTR, result : *mut u16, maxresultsize : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { uldn_languageDisplayName(ldn, lang.param().abi(), result as _, maxresultsize, perrorcode as _) }
}
#[inline]
pub unsafe fn uldn_localeDisplayName<P1>(ldn: *const ULocaleDisplayNames, locale: P1, result: *mut u16, maxresultsize: i32, perrorcode: *mut UErrorCode) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uldn_localeDisplayName(ldn : *const ULocaleDisplayNames, locale : windows_core::PCSTR, result : *mut u16, maxresultsize : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { uldn_localeDisplayName(ldn, locale.param().abi(), result as _, maxresultsize, perrorcode as _) }
}
#[inline]
pub unsafe fn uldn_open<P0>(locale: P0, dialecthandling: UDialectHandling, perrorcode: *mut UErrorCode) -> *mut ULocaleDisplayNames
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uldn_open(locale : windows_core::PCSTR, dialecthandling : UDialectHandling, perrorcode : *mut UErrorCode) -> *mut ULocaleDisplayNames);
    unsafe { uldn_open(locale.param().abi(), dialecthandling, perrorcode as _) }
}
#[inline]
pub unsafe fn uldn_openForContext<P0>(locale: P0, contexts: *mut UDisplayContext, length: i32, perrorcode: *mut UErrorCode) -> *mut ULocaleDisplayNames
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uldn_openForContext(locale : windows_core::PCSTR, contexts : *mut UDisplayContext, length : i32, perrorcode : *mut UErrorCode) -> *mut ULocaleDisplayNames);
    unsafe { uldn_openForContext(locale.param().abi(), contexts as _, length, perrorcode as _) }
}
#[inline]
pub unsafe fn uldn_regionDisplayName<P1>(ldn: *const ULocaleDisplayNames, region: P1, result: *mut u16, maxresultsize: i32, perrorcode: *mut UErrorCode) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uldn_regionDisplayName(ldn : *const ULocaleDisplayNames, region : windows_core::PCSTR, result : *mut u16, maxresultsize : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { uldn_regionDisplayName(ldn, region.param().abi(), result as _, maxresultsize, perrorcode as _) }
}
#[inline]
pub unsafe fn uldn_scriptCodeDisplayName(ldn: *const ULocaleDisplayNames, scriptcode: UScriptCode, result: *mut u16, maxresultsize: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uldn_scriptCodeDisplayName(ldn : *const ULocaleDisplayNames, scriptcode : UScriptCode, result : *mut u16, maxresultsize : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { uldn_scriptCodeDisplayName(ldn, scriptcode, result as _, maxresultsize, perrorcode as _) }
}
#[inline]
pub unsafe fn uldn_scriptDisplayName<P1>(ldn: *const ULocaleDisplayNames, script: P1, result: *mut u16, maxresultsize: i32, perrorcode: *mut UErrorCode) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uldn_scriptDisplayName(ldn : *const ULocaleDisplayNames, script : windows_core::PCSTR, result : *mut u16, maxresultsize : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { uldn_scriptDisplayName(ldn, script.param().abi(), result as _, maxresultsize, perrorcode as _) }
}
#[inline]
pub unsafe fn uldn_variantDisplayName<P1>(ldn: *const ULocaleDisplayNames, variant: P1, result: *mut u16, maxresultsize: i32, perrorcode: *mut UErrorCode) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uldn_variantDisplayName(ldn : *const ULocaleDisplayNames, variant : windows_core::PCSTR, result : *mut u16, maxresultsize : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { uldn_variantDisplayName(ldn, variant.param().abi(), result as _, maxresultsize, perrorcode as _) }
}
#[inline]
pub unsafe fn ulistfmt_close(listfmt: *mut UListFormatter) {
    windows_core::link!("icuuc.dll" "C" fn ulistfmt_close(listfmt : *mut UListFormatter));
    unsafe { ulistfmt_close(listfmt as _) }
}
#[inline]
pub unsafe fn ulistfmt_closeResult(uresult: *mut UFormattedList) {
    windows_core::link!("icu.dll" "C" fn ulistfmt_closeResult(uresult : *mut UFormattedList));
    unsafe { ulistfmt_closeResult(uresult as _) }
}
#[inline]
pub unsafe fn ulistfmt_format(listfmt: *const UListFormatter, strings: *const *const u16, stringlengths: *const i32, stringcount: i32, result: *mut u16, resultcapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ulistfmt_format(listfmt : *const UListFormatter, strings : *const *const u16, stringlengths : *const i32, stringcount : i32, result : *mut u16, resultcapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { ulistfmt_format(listfmt, strings, stringlengths, stringcount, result as _, resultcapacity, status as _) }
}
#[inline]
pub unsafe fn ulistfmt_formatStringsToResult(listfmt: *const UListFormatter, strings: *const *const u16, stringlengths: *const i32, stringcount: i32, uresult: *mut UFormattedList, status: *mut UErrorCode) {
    windows_core::link!("icu.dll" "C" fn ulistfmt_formatStringsToResult(listfmt : *const UListFormatter, strings : *const *const u16, stringlengths : *const i32, stringcount : i32, uresult : *mut UFormattedList, status : *mut UErrorCode));
    unsafe { ulistfmt_formatStringsToResult(listfmt, strings, stringlengths, stringcount, uresult as _, status as _) }
}
#[inline]
pub unsafe fn ulistfmt_open<P0>(locale: P0, status: *mut UErrorCode) -> *mut UListFormatter
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn ulistfmt_open(locale : windows_core::PCSTR, status : *mut UErrorCode) -> *mut UListFormatter);
    unsafe { ulistfmt_open(locale.param().abi(), status as _) }
}
#[inline]
pub unsafe fn ulistfmt_openForType<P0>(locale: P0, r#type: UListFormatterType, width: UListFormatterWidth, status: *mut UErrorCode) -> *mut UListFormatter
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icu.dll" "C" fn ulistfmt_openForType(locale : windows_core::PCSTR, r#type : UListFormatterType, width : UListFormatterWidth, status : *mut UErrorCode) -> *mut UListFormatter);
    unsafe { ulistfmt_openForType(locale.param().abi(), r#type, width, status as _) }
}
#[inline]
pub unsafe fn ulistfmt_openResult(ec: *mut UErrorCode) -> *mut UFormattedList {
    windows_core::link!("icu.dll" "C" fn ulistfmt_openResult(ec : *mut UErrorCode) -> *mut UFormattedList);
    unsafe { ulistfmt_openResult(ec as _) }
}
#[inline]
pub unsafe fn ulistfmt_resultAsValue(uresult: *const UFormattedList, ec: *mut UErrorCode) -> *mut UFormattedValue {
    windows_core::link!("icu.dll" "C" fn ulistfmt_resultAsValue(uresult : *const UFormattedList, ec : *mut UErrorCode) -> *mut UFormattedValue);
    unsafe { ulistfmt_resultAsValue(uresult, ec as _) }
}
#[inline]
pub unsafe fn uloc_acceptLanguage<P0>(result: P0, resultavailable: i32, outresult: *mut UAcceptResult, acceptlist: *const *const i8, acceptlistcount: i32, availablelocales: *mut UEnumeration, status: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uloc_acceptLanguage(result : windows_core::PCSTR, resultavailable : i32, outresult : *mut UAcceptResult, acceptlist : *const *const i8, acceptlistcount : i32, availablelocales : *mut UEnumeration, status : *mut UErrorCode) -> i32);
    unsafe { uloc_acceptLanguage(result.param().abi(), resultavailable, outresult as _, acceptlist, acceptlistcount, availablelocales as _, status as _) }
}
#[inline]
pub unsafe fn uloc_acceptLanguageFromHTTP<P0, P3>(result: P0, resultavailable: i32, outresult: *mut UAcceptResult, httpacceptlanguage: P3, availablelocales: *mut UEnumeration, status: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uloc_acceptLanguageFromHTTP(result : windows_core::PCSTR, resultavailable : i32, outresult : *mut UAcceptResult, httpacceptlanguage : windows_core::PCSTR, availablelocales : *mut UEnumeration, status : *mut UErrorCode) -> i32);
    unsafe { uloc_acceptLanguageFromHTTP(result.param().abi(), resultavailable, outresult as _, httpacceptlanguage.param().abi(), availablelocales as _, status as _) }
}
#[inline]
pub unsafe fn uloc_addLikelySubtags<P0, P1>(localeid: P0, maximizedlocaleid: P1, maximizedlocaleidcapacity: i32, err: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uloc_addLikelySubtags(localeid : windows_core::PCSTR, maximizedlocaleid : windows_core::PCSTR, maximizedlocaleidcapacity : i32, err : *mut UErrorCode) -> i32);
    unsafe { uloc_addLikelySubtags(localeid.param().abi(), maximizedlocaleid.param().abi(), maximizedlocaleidcapacity, err as _) }
}
#[inline]
pub unsafe fn uloc_canonicalize<P0, P1>(localeid: P0, name: P1, namecapacity: i32, err: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uloc_canonicalize(localeid : windows_core::PCSTR, name : windows_core::PCSTR, namecapacity : i32, err : *mut UErrorCode) -> i32);
    unsafe { uloc_canonicalize(localeid.param().abi(), name.param().abi(), namecapacity, err as _) }
}
#[inline]
pub unsafe fn uloc_countAvailable() -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uloc_countAvailable() -> i32);
    unsafe { uloc_countAvailable() }
}
#[inline]
pub unsafe fn uloc_forLanguageTag<P0, P1>(langtag: P0, localeid: P1, localeidcapacity: i32, parsedlength: *mut i32, err: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uloc_forLanguageTag(langtag : windows_core::PCSTR, localeid : windows_core::PCSTR, localeidcapacity : i32, parsedlength : *mut i32, err : *mut UErrorCode) -> i32);
    unsafe { uloc_forLanguageTag(langtag.param().abi(), localeid.param().abi(), localeidcapacity, parsedlength as _, err as _) }
}
#[inline]
pub unsafe fn uloc_getAvailable(n: i32) -> windows_core::PCSTR {
    windows_core::link!("icuuc.dll" "C" fn uloc_getAvailable(n : i32) -> windows_core::PCSTR);
    unsafe { uloc_getAvailable(n) }
}
#[inline]
pub unsafe fn uloc_getBaseName<P0, P1>(localeid: P0, name: P1, namecapacity: i32, err: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uloc_getBaseName(localeid : windows_core::PCSTR, name : windows_core::PCSTR, namecapacity : i32, err : *mut UErrorCode) -> i32);
    unsafe { uloc_getBaseName(localeid.param().abi(), name.param().abi(), namecapacity, err as _) }
}
#[inline]
pub unsafe fn uloc_getCharacterOrientation<P0>(localeid: P0, status: *mut UErrorCode) -> ULayoutType
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uloc_getCharacterOrientation(localeid : windows_core::PCSTR, status : *mut UErrorCode) -> ULayoutType);
    unsafe { uloc_getCharacterOrientation(localeid.param().abi(), status as _) }
}
#[inline]
pub unsafe fn uloc_getCountry<P0, P1>(localeid: P0, country: P1, countrycapacity: i32, err: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uloc_getCountry(localeid : windows_core::PCSTR, country : windows_core::PCSTR, countrycapacity : i32, err : *mut UErrorCode) -> i32);
    unsafe { uloc_getCountry(localeid.param().abi(), country.param().abi(), countrycapacity, err as _) }
}
#[inline]
pub unsafe fn uloc_getDefault() -> windows_core::PCSTR {
    windows_core::link!("icuuc.dll" "C" fn uloc_getDefault() -> windows_core::PCSTR);
    unsafe { uloc_getDefault() }
}
#[inline]
pub unsafe fn uloc_getDisplayCountry<P0, P1>(locale: P0, displaylocale: P1, country: *mut u16, countrycapacity: i32, status: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uloc_getDisplayCountry(locale : windows_core::PCSTR, displaylocale : windows_core::PCSTR, country : *mut u16, countrycapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { uloc_getDisplayCountry(locale.param().abi(), displaylocale.param().abi(), country as _, countrycapacity, status as _) }
}
#[inline]
pub unsafe fn uloc_getDisplayKeyword<P0, P1>(keyword: P0, displaylocale: P1, dest: *mut u16, destcapacity: i32, status: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uloc_getDisplayKeyword(keyword : windows_core::PCSTR, displaylocale : windows_core::PCSTR, dest : *mut u16, destcapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { uloc_getDisplayKeyword(keyword.param().abi(), displaylocale.param().abi(), dest as _, destcapacity, status as _) }
}
#[inline]
pub unsafe fn uloc_getDisplayKeywordValue<P0, P1, P2>(locale: P0, keyword: P1, displaylocale: P2, dest: *mut u16, destcapacity: i32, status: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uloc_getDisplayKeywordValue(locale : windows_core::PCSTR, keyword : windows_core::PCSTR, displaylocale : windows_core::PCSTR, dest : *mut u16, destcapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { uloc_getDisplayKeywordValue(locale.param().abi(), keyword.param().abi(), displaylocale.param().abi(), dest as _, destcapacity, status as _) }
}
#[inline]
pub unsafe fn uloc_getDisplayLanguage<P0, P1>(locale: P0, displaylocale: P1, language: *mut u16, languagecapacity: i32, status: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uloc_getDisplayLanguage(locale : windows_core::PCSTR, displaylocale : windows_core::PCSTR, language : *mut u16, languagecapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { uloc_getDisplayLanguage(locale.param().abi(), displaylocale.param().abi(), language as _, languagecapacity, status as _) }
}
#[inline]
pub unsafe fn uloc_getDisplayName<P0, P1>(localeid: P0, inlocaleid: P1, result: *mut u16, maxresultsize: i32, err: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uloc_getDisplayName(localeid : windows_core::PCSTR, inlocaleid : windows_core::PCSTR, result : *mut u16, maxresultsize : i32, err : *mut UErrorCode) -> i32);
    unsafe { uloc_getDisplayName(localeid.param().abi(), inlocaleid.param().abi(), result as _, maxresultsize, err as _) }
}
#[inline]
pub unsafe fn uloc_getDisplayScript<P0, P1>(locale: P0, displaylocale: P1, script: *mut u16, scriptcapacity: i32, status: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uloc_getDisplayScript(locale : windows_core::PCSTR, displaylocale : windows_core::PCSTR, script : *mut u16, scriptcapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { uloc_getDisplayScript(locale.param().abi(), displaylocale.param().abi(), script as _, scriptcapacity, status as _) }
}
#[inline]
pub unsafe fn uloc_getDisplayVariant<P0, P1>(locale: P0, displaylocale: P1, variant: *mut u16, variantcapacity: i32, status: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uloc_getDisplayVariant(locale : windows_core::PCSTR, displaylocale : windows_core::PCSTR, variant : *mut u16, variantcapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { uloc_getDisplayVariant(locale.param().abi(), displaylocale.param().abi(), variant as _, variantcapacity, status as _) }
}
#[inline]
pub unsafe fn uloc_getISO3Country<P0>(localeid: P0) -> windows_core::PCSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uloc_getISO3Country(localeid : windows_core::PCSTR) -> windows_core::PCSTR);
    unsafe { uloc_getISO3Country(localeid.param().abi()) }
}
#[inline]
pub unsafe fn uloc_getISO3Language<P0>(localeid: P0) -> windows_core::PCSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uloc_getISO3Language(localeid : windows_core::PCSTR) -> windows_core::PCSTR);
    unsafe { uloc_getISO3Language(localeid.param().abi()) }
}
#[inline]
pub unsafe fn uloc_getISOCountries() -> *mut *mut i8 {
    windows_core::link!("icuuc.dll" "C" fn uloc_getISOCountries() -> *mut *mut i8);
    unsafe { uloc_getISOCountries() }
}
#[inline]
pub unsafe fn uloc_getISOLanguages() -> *mut *mut i8 {
    windows_core::link!("icuuc.dll" "C" fn uloc_getISOLanguages() -> *mut *mut i8);
    unsafe { uloc_getISOLanguages() }
}
#[inline]
pub unsafe fn uloc_getKeywordValue<P0, P1, P2>(localeid: P0, keywordname: P1, buffer: P2, buffercapacity: i32, status: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uloc_getKeywordValue(localeid : windows_core::PCSTR, keywordname : windows_core::PCSTR, buffer : windows_core::PCSTR, buffercapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { uloc_getKeywordValue(localeid.param().abi(), keywordname.param().abi(), buffer.param().abi(), buffercapacity, status as _) }
}
#[inline]
pub unsafe fn uloc_getLCID<P0>(localeid: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uloc_getLCID(localeid : windows_core::PCSTR) -> u32);
    unsafe { uloc_getLCID(localeid.param().abi()) }
}
#[inline]
pub unsafe fn uloc_getLanguage<P0, P1>(localeid: P0, language: P1, languagecapacity: i32, err: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uloc_getLanguage(localeid : windows_core::PCSTR, language : windows_core::PCSTR, languagecapacity : i32, err : *mut UErrorCode) -> i32);
    unsafe { uloc_getLanguage(localeid.param().abi(), language.param().abi(), languagecapacity, err as _) }
}
#[inline]
pub unsafe fn uloc_getLineOrientation<P0>(localeid: P0, status: *mut UErrorCode) -> ULayoutType
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uloc_getLineOrientation(localeid : windows_core::PCSTR, status : *mut UErrorCode) -> ULayoutType);
    unsafe { uloc_getLineOrientation(localeid.param().abi(), status as _) }
}
#[inline]
pub unsafe fn uloc_getLocaleForLCID<P1>(hostid: u32, locale: P1, localecapacity: i32, status: *mut UErrorCode) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uloc_getLocaleForLCID(hostid : u32, locale : windows_core::PCSTR, localecapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { uloc_getLocaleForLCID(hostid, locale.param().abi(), localecapacity, status as _) }
}
#[inline]
pub unsafe fn uloc_getName<P0, P1>(localeid: P0, name: P1, namecapacity: i32, err: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uloc_getName(localeid : windows_core::PCSTR, name : windows_core::PCSTR, namecapacity : i32, err : *mut UErrorCode) -> i32);
    unsafe { uloc_getName(localeid.param().abi(), name.param().abi(), namecapacity, err as _) }
}
#[inline]
pub unsafe fn uloc_getParent<P0, P1>(localeid: P0, parent: P1, parentcapacity: i32, err: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uloc_getParent(localeid : windows_core::PCSTR, parent : windows_core::PCSTR, parentcapacity : i32, err : *mut UErrorCode) -> i32);
    unsafe { uloc_getParent(localeid.param().abi(), parent.param().abi(), parentcapacity, err as _) }
}
#[inline]
pub unsafe fn uloc_getScript<P0, P1>(localeid: P0, script: P1, scriptcapacity: i32, err: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uloc_getScript(localeid : windows_core::PCSTR, script : windows_core::PCSTR, scriptcapacity : i32, err : *mut UErrorCode) -> i32);
    unsafe { uloc_getScript(localeid.param().abi(), script.param().abi(), scriptcapacity, err as _) }
}
#[inline]
pub unsafe fn uloc_getVariant<P0, P1>(localeid: P0, variant: P1, variantcapacity: i32, err: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uloc_getVariant(localeid : windows_core::PCSTR, variant : windows_core::PCSTR, variantcapacity : i32, err : *mut UErrorCode) -> i32);
    unsafe { uloc_getVariant(localeid.param().abi(), variant.param().abi(), variantcapacity, err as _) }
}
#[inline]
pub unsafe fn uloc_isRightToLeft<P0>(locale: P0) -> i8
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uloc_isRightToLeft(locale : windows_core::PCSTR) -> i8);
    unsafe { uloc_isRightToLeft(locale.param().abi()) }
}
#[inline]
pub unsafe fn uloc_minimizeSubtags<P0, P1>(localeid: P0, minimizedlocaleid: P1, minimizedlocaleidcapacity: i32, err: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uloc_minimizeSubtags(localeid : windows_core::PCSTR, minimizedlocaleid : windows_core::PCSTR, minimizedlocaleidcapacity : i32, err : *mut UErrorCode) -> i32);
    unsafe { uloc_minimizeSubtags(localeid.param().abi(), minimizedlocaleid.param().abi(), minimizedlocaleidcapacity, err as _) }
}
#[inline]
pub unsafe fn uloc_openAvailableByType(r#type: ULocAvailableType, status: *mut UErrorCode) -> *mut UEnumeration {
    windows_core::link!("icu.dll" "C" fn uloc_openAvailableByType(r#type : ULocAvailableType, status : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { uloc_openAvailableByType(r#type, status as _) }
}
#[inline]
pub unsafe fn uloc_openKeywords<P0>(localeid: P0, status: *mut UErrorCode) -> *mut UEnumeration
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uloc_openKeywords(localeid : windows_core::PCSTR, status : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { uloc_openKeywords(localeid.param().abi(), status as _) }
}
#[inline]
pub unsafe fn uloc_setDefault<P0>(localeid: P0, status: *mut UErrorCode)
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uloc_setDefault(localeid : windows_core::PCSTR, status : *mut UErrorCode));
    unsafe { uloc_setDefault(localeid.param().abi(), status as _) }
}
#[inline]
pub unsafe fn uloc_setKeywordValue<P0, P1, P2>(keywordname: P0, keywordvalue: P1, buffer: P2, buffercapacity: i32, status: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uloc_setKeywordValue(keywordname : windows_core::PCSTR, keywordvalue : windows_core::PCSTR, buffer : windows_core::PCSTR, buffercapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { uloc_setKeywordValue(keywordname.param().abi(), keywordvalue.param().abi(), buffer.param().abi(), buffercapacity, status as _) }
}
#[inline]
pub unsafe fn uloc_toLanguageTag<P0, P1>(localeid: P0, langtag: P1, langtagcapacity: i32, strict: i8, err: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uloc_toLanguageTag(localeid : windows_core::PCSTR, langtag : windows_core::PCSTR, langtagcapacity : i32, strict : i8, err : *mut UErrorCode) -> i32);
    unsafe { uloc_toLanguageTag(localeid.param().abi(), langtag.param().abi(), langtagcapacity, strict, err as _) }
}
#[inline]
pub unsafe fn uloc_toLegacyKey<P0>(keyword: P0) -> windows_core::PCSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uloc_toLegacyKey(keyword : windows_core::PCSTR) -> windows_core::PCSTR);
    unsafe { uloc_toLegacyKey(keyword.param().abi()) }
}
#[inline]
pub unsafe fn uloc_toLegacyType<P0, P1>(keyword: P0, value: P1) -> windows_core::PCSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uloc_toLegacyType(keyword : windows_core::PCSTR, value : windows_core::PCSTR) -> windows_core::PCSTR);
    unsafe { uloc_toLegacyType(keyword.param().abi(), value.param().abi()) }
}
#[inline]
pub unsafe fn uloc_toUnicodeLocaleKey<P0>(keyword: P0) -> windows_core::PCSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uloc_toUnicodeLocaleKey(keyword : windows_core::PCSTR) -> windows_core::PCSTR);
    unsafe { uloc_toUnicodeLocaleKey(keyword.param().abi()) }
}
#[inline]
pub unsafe fn uloc_toUnicodeLocaleType<P0, P1>(keyword: P0, value: P1) -> windows_core::PCSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uloc_toUnicodeLocaleType(keyword : windows_core::PCSTR, value : windows_core::PCSTR) -> windows_core::PCSTR);
    unsafe { uloc_toUnicodeLocaleType(keyword.param().abi(), value.param().abi()) }
}
#[inline]
pub unsafe fn ulocdata_close(uld: *mut ULocaleData) {
    windows_core::link!("icuin.dll" "C" fn ulocdata_close(uld : *mut ULocaleData));
    unsafe { ulocdata_close(uld as _) }
}
#[inline]
pub unsafe fn ulocdata_getCLDRVersion(versionarray: *mut u8, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn ulocdata_getCLDRVersion(versionarray : *mut u8, status : *mut UErrorCode));
    unsafe { ulocdata_getCLDRVersion(versionarray as _, status as _) }
}
#[inline]
pub unsafe fn ulocdata_getDelimiter(uld: *mut ULocaleData, r#type: ULocaleDataDelimiterType, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ulocdata_getDelimiter(uld : *mut ULocaleData, r#type : ULocaleDataDelimiterType, result : *mut u16, resultlength : i32, status : *mut UErrorCode) -> i32);
    unsafe { ulocdata_getDelimiter(uld as _, r#type, result as _, resultlength, status as _) }
}
#[inline]
pub unsafe fn ulocdata_getExemplarSet(uld: *mut ULocaleData, fillin: *mut USet, options: u32, extype: ULocaleDataExemplarSetType, status: *mut UErrorCode) -> *mut USet {
    windows_core::link!("icuin.dll" "C" fn ulocdata_getExemplarSet(uld : *mut ULocaleData, fillin : *mut USet, options : u32, extype : ULocaleDataExemplarSetType, status : *mut UErrorCode) -> *mut USet);
    unsafe { ulocdata_getExemplarSet(uld as _, fillin as _, options, extype, status as _) }
}
#[inline]
pub unsafe fn ulocdata_getLocaleDisplayPattern(uld: *mut ULocaleData, pattern: *mut u16, patterncapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ulocdata_getLocaleDisplayPattern(uld : *mut ULocaleData, pattern : *mut u16, patterncapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { ulocdata_getLocaleDisplayPattern(uld as _, pattern as _, patterncapacity, status as _) }
}
#[inline]
pub unsafe fn ulocdata_getLocaleSeparator(uld: *mut ULocaleData, separator: *mut u16, separatorcapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ulocdata_getLocaleSeparator(uld : *mut ULocaleData, separator : *mut u16, separatorcapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { ulocdata_getLocaleSeparator(uld as _, separator as _, separatorcapacity, status as _) }
}
#[inline]
pub unsafe fn ulocdata_getMeasurementSystem<P0>(localeid: P0, status: *mut UErrorCode) -> UMeasurementSystem
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuin.dll" "C" fn ulocdata_getMeasurementSystem(localeid : windows_core::PCSTR, status : *mut UErrorCode) -> UMeasurementSystem);
    unsafe { ulocdata_getMeasurementSystem(localeid.param().abi(), status as _) }
}
#[inline]
pub unsafe fn ulocdata_getNoSubstitute(uld: *mut ULocaleData) -> i8 {
    windows_core::link!("icuin.dll" "C" fn ulocdata_getNoSubstitute(uld : *mut ULocaleData) -> i8);
    unsafe { ulocdata_getNoSubstitute(uld as _) }
}
#[inline]
pub unsafe fn ulocdata_getPaperSize<P0>(localeid: P0, height: *mut i32, width: *mut i32, status: *mut UErrorCode)
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuin.dll" "C" fn ulocdata_getPaperSize(localeid : windows_core::PCSTR, height : *mut i32, width : *mut i32, status : *mut UErrorCode));
    unsafe { ulocdata_getPaperSize(localeid.param().abi(), height as _, width as _, status as _) }
}
#[inline]
pub unsafe fn ulocdata_open<P0>(localeid: P0, status: *mut UErrorCode) -> *mut ULocaleData
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuin.dll" "C" fn ulocdata_open(localeid : windows_core::PCSTR, status : *mut UErrorCode) -> *mut ULocaleData);
    unsafe { ulocdata_open(localeid.param().abi(), status as _) }
}
#[inline]
pub unsafe fn ulocdata_setNoSubstitute(uld: *mut ULocaleData, setting: i8) {
    windows_core::link!("icuin.dll" "C" fn ulocdata_setNoSubstitute(uld : *mut ULocaleData, setting : i8));
    unsafe { ulocdata_setNoSubstitute(uld as _, setting) }
}
#[inline]
pub unsafe fn umsg_applyPattern(fmt: *mut *mut core::ffi::c_void, pattern: *const u16, patternlength: i32, parseerror: *mut UParseError, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn umsg_applyPattern(fmt : *mut *mut core::ffi::c_void, pattern : *const u16, patternlength : i32, parseerror : *mut UParseError, status : *mut UErrorCode));
    unsafe { umsg_applyPattern(fmt as _, pattern, patternlength, parseerror as _, status as _) }
}
#[inline]
pub unsafe fn umsg_autoQuoteApostrophe(pattern: *const u16, patternlength: i32, dest: *mut u16, destcapacity: i32, ec: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn umsg_autoQuoteApostrophe(pattern : *const u16, patternlength : i32, dest : *mut u16, destcapacity : i32, ec : *mut UErrorCode) -> i32);
    unsafe { umsg_autoQuoteApostrophe(pattern, patternlength, dest as _, destcapacity, ec as _) }
}
#[inline]
pub unsafe fn umsg_clone(fmt: *const *const core::ffi::c_void, status: *mut UErrorCode) -> *mut core::ffi::c_void {
    windows_core::link!("icuin.dll" "C" fn umsg_clone(fmt : *const *const core::ffi::c_void, status : *mut UErrorCode) -> *mut core::ffi::c_void);
    unsafe { umsg_clone(fmt, status as _) }
}
#[inline]
pub unsafe fn umsg_close(format: *mut *mut core::ffi::c_void) {
    windows_core::link!("icuin.dll" "C" fn umsg_close(format : *mut *mut core::ffi::c_void));
    unsafe { umsg_close(format as _) }
}
#[inline]
pub unsafe fn umsg_format(fmt: *const *const core::ffi::c_void, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn umsg_format(fmt : *const *const core::ffi::c_void, result : *mut u16, resultlength : i32, status : *mut UErrorCode) -> i32);
    unsafe { umsg_format(fmt, result as _, resultlength, status as _) }
}
#[inline]
pub unsafe fn umsg_getLocale(fmt: *const *const core::ffi::c_void) -> windows_core::PCSTR {
    windows_core::link!("icuin.dll" "C" fn umsg_getLocale(fmt : *const *const core::ffi::c_void) -> windows_core::PCSTR);
    unsafe { umsg_getLocale(fmt) }
}
#[inline]
pub unsafe fn umsg_open<P2>(pattern: *const u16, patternlength: i32, locale: P2, parseerror: *mut UParseError, status: *mut UErrorCode) -> *mut *mut core::ffi::c_void
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuin.dll" "C" fn umsg_open(pattern : *const u16, patternlength : i32, locale : windows_core::PCSTR, parseerror : *mut UParseError, status : *mut UErrorCode) -> *mut *mut core::ffi::c_void);
    unsafe { umsg_open(pattern, patternlength, locale.param().abi(), parseerror as _, status as _) }
}
#[inline]
pub unsafe fn umsg_parse(fmt: *const *const core::ffi::c_void, source: *const u16, sourcelength: i32, count: *mut i32, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn umsg_parse(fmt : *const *const core::ffi::c_void, source : *const u16, sourcelength : i32, count : *mut i32, status : *mut UErrorCode));
    unsafe { umsg_parse(fmt, source, sourcelength, count as _, status as _) }
}
#[inline]
pub unsafe fn umsg_setLocale<P1>(fmt: *mut *mut core::ffi::c_void, locale: P1)
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuin.dll" "C" fn umsg_setLocale(fmt : *mut *mut core::ffi::c_void, locale : windows_core::PCSTR));
    unsafe { umsg_setLocale(fmt as _, locale.param().abi()) }
}
#[inline]
pub unsafe fn umsg_toPattern(fmt: *const *const core::ffi::c_void, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn umsg_toPattern(fmt : *const *const core::ffi::c_void, result : *mut u16, resultlength : i32, status : *mut UErrorCode) -> i32);
    unsafe { umsg_toPattern(fmt, result as _, resultlength, status as _) }
}
#[inline]
pub unsafe fn umsg_vformat(fmt: *const *const core::ffi::c_void, result: *mut u16, resultlength: i32, ap: *mut i8, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn umsg_vformat(fmt : *const *const core::ffi::c_void, result : *mut u16, resultlength : i32, ap : *mut i8, status : *mut UErrorCode) -> i32);
    unsafe { umsg_vformat(fmt, result as _, resultlength, ap as _, status as _) }
}
#[inline]
pub unsafe fn umsg_vparse(fmt: *const *const core::ffi::c_void, source: *const u16, sourcelength: i32, count: *mut i32, ap: *mut i8, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn umsg_vparse(fmt : *const *const core::ffi::c_void, source : *const u16, sourcelength : i32, count : *mut i32, ap : *mut i8, status : *mut UErrorCode));
    unsafe { umsg_vparse(fmt, source, sourcelength, count as _, ap as _, status as _) }
}
#[inline]
pub unsafe fn umutablecptrie_buildImmutable(trie: *mut UMutableCPTrie, r#type: UCPTrieType, valuewidth: UCPTrieValueWidth, perrorcode: *mut UErrorCode) -> *mut UCPTrie {
    windows_core::link!("icu.dll" "C" fn umutablecptrie_buildImmutable(trie : *mut UMutableCPTrie, r#type : UCPTrieType, valuewidth : UCPTrieValueWidth, perrorcode : *mut UErrorCode) -> *mut UCPTrie);
    unsafe { umutablecptrie_buildImmutable(trie as _, r#type, valuewidth, perrorcode as _) }
}
#[inline]
pub unsafe fn umutablecptrie_clone(other: *const UMutableCPTrie, perrorcode: *mut UErrorCode) -> *mut UMutableCPTrie {
    windows_core::link!("icu.dll" "C" fn umutablecptrie_clone(other : *const UMutableCPTrie, perrorcode : *mut UErrorCode) -> *mut UMutableCPTrie);
    unsafe { umutablecptrie_clone(other, perrorcode as _) }
}
#[inline]
pub unsafe fn umutablecptrie_close(trie: *mut UMutableCPTrie) {
    windows_core::link!("icu.dll" "C" fn umutablecptrie_close(trie : *mut UMutableCPTrie));
    unsafe { umutablecptrie_close(trie as _) }
}
#[inline]
pub unsafe fn umutablecptrie_fromUCPMap(map: *const UCPMap, perrorcode: *mut UErrorCode) -> *mut UMutableCPTrie {
    windows_core::link!("icu.dll" "C" fn umutablecptrie_fromUCPMap(map : *const UCPMap, perrorcode : *mut UErrorCode) -> *mut UMutableCPTrie);
    unsafe { umutablecptrie_fromUCPMap(map, perrorcode as _) }
}
#[inline]
pub unsafe fn umutablecptrie_fromUCPTrie(trie: *const UCPTrie, perrorcode: *mut UErrorCode) -> *mut UMutableCPTrie {
    windows_core::link!("icu.dll" "C" fn umutablecptrie_fromUCPTrie(trie : *const UCPTrie, perrorcode : *mut UErrorCode) -> *mut UMutableCPTrie);
    unsafe { umutablecptrie_fromUCPTrie(trie, perrorcode as _) }
}
#[inline]
pub unsafe fn umutablecptrie_get(trie: *const UMutableCPTrie, c: i32) -> u32 {
    windows_core::link!("icu.dll" "C" fn umutablecptrie_get(trie : *const UMutableCPTrie, c : i32) -> u32);
    unsafe { umutablecptrie_get(trie, c) }
}
#[inline]
pub unsafe fn umutablecptrie_getRange(trie: *const UMutableCPTrie, start: i32, option: UCPMapRangeOption, surrogatevalue: u32, filter: *mut UCPMapValueFilter, context: *const core::ffi::c_void, pvalue: *mut u32) -> i32 {
    windows_core::link!("icu.dll" "C" fn umutablecptrie_getRange(trie : *const UMutableCPTrie, start : i32, option : UCPMapRangeOption, surrogatevalue : u32, filter : *mut UCPMapValueFilter, context : *const core::ffi::c_void, pvalue : *mut u32) -> i32);
    unsafe { umutablecptrie_getRange(trie, start, option, surrogatevalue, filter as _, context, pvalue as _) }
}
#[inline]
pub unsafe fn umutablecptrie_open(initialvalue: u32, errorvalue: u32, perrorcode: *mut UErrorCode) -> *mut UMutableCPTrie {
    windows_core::link!("icu.dll" "C" fn umutablecptrie_open(initialvalue : u32, errorvalue : u32, perrorcode : *mut UErrorCode) -> *mut UMutableCPTrie);
    unsafe { umutablecptrie_open(initialvalue, errorvalue, perrorcode as _) }
}
#[inline]
pub unsafe fn umutablecptrie_set(trie: *mut UMutableCPTrie, c: i32, value: u32, perrorcode: *mut UErrorCode) {
    windows_core::link!("icu.dll" "C" fn umutablecptrie_set(trie : *mut UMutableCPTrie, c : i32, value : u32, perrorcode : *mut UErrorCode));
    unsafe { umutablecptrie_set(trie as _, c, value, perrorcode as _) }
}
#[inline]
pub unsafe fn umutablecptrie_setRange(trie: *mut UMutableCPTrie, start: i32, end: i32, value: u32, perrorcode: *mut UErrorCode) {
    windows_core::link!("icu.dll" "C" fn umutablecptrie_setRange(trie : *mut UMutableCPTrie, start : i32, end : i32, value : u32, perrorcode : *mut UErrorCode));
    unsafe { umutablecptrie_setRange(trie as _, start, end, value, perrorcode as _) }
}
#[inline]
pub unsafe fn unorm2_append(norm2: *const UNormalizer2, first: *mut u16, firstlength: i32, firstcapacity: i32, second: *const u16, secondlength: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn unorm2_append(norm2 : *const UNormalizer2, first : *mut u16, firstlength : i32, firstcapacity : i32, second : *const u16, secondlength : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { unorm2_append(norm2, first as _, firstlength, firstcapacity, second, secondlength, perrorcode as _) }
}
#[inline]
pub unsafe fn unorm2_close(norm2: *mut UNormalizer2) {
    windows_core::link!("icuuc.dll" "C" fn unorm2_close(norm2 : *mut UNormalizer2));
    unsafe { unorm2_close(norm2 as _) }
}
#[inline]
pub unsafe fn unorm2_composePair(norm2: *const UNormalizer2, a: i32, b: i32) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn unorm2_composePair(norm2 : *const UNormalizer2, a : i32, b : i32) -> i32);
    unsafe { unorm2_composePair(norm2, a, b) }
}
#[inline]
pub unsafe fn unorm2_getCombiningClass(norm2: *const UNormalizer2, c: i32) -> u8 {
    windows_core::link!("icuuc.dll" "C" fn unorm2_getCombiningClass(norm2 : *const UNormalizer2, c : i32) -> u8);
    unsafe { unorm2_getCombiningClass(norm2, c) }
}
#[inline]
pub unsafe fn unorm2_getDecomposition(norm2: *const UNormalizer2, c: i32, decomposition: *mut u16, capacity: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn unorm2_getDecomposition(norm2 : *const UNormalizer2, c : i32, decomposition : *mut u16, capacity : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { unorm2_getDecomposition(norm2, c, decomposition as _, capacity, perrorcode as _) }
}
#[inline]
pub unsafe fn unorm2_getInstance<P0, P1>(packagename: P0, name: P1, mode: UNormalization2Mode, perrorcode: *mut UErrorCode) -> *mut UNormalizer2
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn unorm2_getInstance(packagename : windows_core::PCSTR, name : windows_core::PCSTR, mode : UNormalization2Mode, perrorcode : *mut UErrorCode) -> *mut UNormalizer2);
    unsafe { unorm2_getInstance(packagename.param().abi(), name.param().abi(), mode, perrorcode as _) }
}
#[inline]
pub unsafe fn unorm2_getNFCInstance(perrorcode: *mut UErrorCode) -> *mut UNormalizer2 {
    windows_core::link!("icuuc.dll" "C" fn unorm2_getNFCInstance(perrorcode : *mut UErrorCode) -> *mut UNormalizer2);
    unsafe { unorm2_getNFCInstance(perrorcode as _) }
}
#[inline]
pub unsafe fn unorm2_getNFDInstance(perrorcode: *mut UErrorCode) -> *mut UNormalizer2 {
    windows_core::link!("icuuc.dll" "C" fn unorm2_getNFDInstance(perrorcode : *mut UErrorCode) -> *mut UNormalizer2);
    unsafe { unorm2_getNFDInstance(perrorcode as _) }
}
#[inline]
pub unsafe fn unorm2_getNFKCCasefoldInstance(perrorcode: *mut UErrorCode) -> *mut UNormalizer2 {
    windows_core::link!("icuuc.dll" "C" fn unorm2_getNFKCCasefoldInstance(perrorcode : *mut UErrorCode) -> *mut UNormalizer2);
    unsafe { unorm2_getNFKCCasefoldInstance(perrorcode as _) }
}
#[inline]
pub unsafe fn unorm2_getNFKCInstance(perrorcode: *mut UErrorCode) -> *mut UNormalizer2 {
    windows_core::link!("icuuc.dll" "C" fn unorm2_getNFKCInstance(perrorcode : *mut UErrorCode) -> *mut UNormalizer2);
    unsafe { unorm2_getNFKCInstance(perrorcode as _) }
}
#[inline]
pub unsafe fn unorm2_getNFKDInstance(perrorcode: *mut UErrorCode) -> *mut UNormalizer2 {
    windows_core::link!("icuuc.dll" "C" fn unorm2_getNFKDInstance(perrorcode : *mut UErrorCode) -> *mut UNormalizer2);
    unsafe { unorm2_getNFKDInstance(perrorcode as _) }
}
#[inline]
pub unsafe fn unorm2_getRawDecomposition(norm2: *const UNormalizer2, c: i32, decomposition: *mut u16, capacity: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn unorm2_getRawDecomposition(norm2 : *const UNormalizer2, c : i32, decomposition : *mut u16, capacity : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { unorm2_getRawDecomposition(norm2, c, decomposition as _, capacity, perrorcode as _) }
}
#[inline]
pub unsafe fn unorm2_hasBoundaryAfter(norm2: *const UNormalizer2, c: i32) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn unorm2_hasBoundaryAfter(norm2 : *const UNormalizer2, c : i32) -> i8);
    unsafe { unorm2_hasBoundaryAfter(norm2, c) }
}
#[inline]
pub unsafe fn unorm2_hasBoundaryBefore(norm2: *const UNormalizer2, c: i32) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn unorm2_hasBoundaryBefore(norm2 : *const UNormalizer2, c : i32) -> i8);
    unsafe { unorm2_hasBoundaryBefore(norm2, c) }
}
#[inline]
pub unsafe fn unorm2_isInert(norm2: *const UNormalizer2, c: i32) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn unorm2_isInert(norm2 : *const UNormalizer2, c : i32) -> i8);
    unsafe { unorm2_isInert(norm2, c) }
}
#[inline]
pub unsafe fn unorm2_isNormalized(norm2: *const UNormalizer2, s: *const u16, length: i32, perrorcode: *mut UErrorCode) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn unorm2_isNormalized(norm2 : *const UNormalizer2, s : *const u16, length : i32, perrorcode : *mut UErrorCode) -> i8);
    unsafe { unorm2_isNormalized(norm2, s, length, perrorcode as _) }
}
#[inline]
pub unsafe fn unorm2_normalize(norm2: *const UNormalizer2, src: *const u16, length: i32, dest: *mut u16, capacity: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn unorm2_normalize(norm2 : *const UNormalizer2, src : *const u16, length : i32, dest : *mut u16, capacity : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { unorm2_normalize(norm2, src, length, dest as _, capacity, perrorcode as _) }
}
#[inline]
pub unsafe fn unorm2_normalizeSecondAndAppend(norm2: *const UNormalizer2, first: *mut u16, firstlength: i32, firstcapacity: i32, second: *const u16, secondlength: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn unorm2_normalizeSecondAndAppend(norm2 : *const UNormalizer2, first : *mut u16, firstlength : i32, firstcapacity : i32, second : *const u16, secondlength : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { unorm2_normalizeSecondAndAppend(norm2, first as _, firstlength, firstcapacity, second, secondlength, perrorcode as _) }
}
#[inline]
pub unsafe fn unorm2_openFiltered(norm2: *const UNormalizer2, filterset: *const USet, perrorcode: *mut UErrorCode) -> *mut UNormalizer2 {
    windows_core::link!("icuuc.dll" "C" fn unorm2_openFiltered(norm2 : *const UNormalizer2, filterset : *const USet, perrorcode : *mut UErrorCode) -> *mut UNormalizer2);
    unsafe { unorm2_openFiltered(norm2, filterset, perrorcode as _) }
}
#[inline]
pub unsafe fn unorm2_quickCheck(norm2: *const UNormalizer2, s: *const u16, length: i32, perrorcode: *mut UErrorCode) -> UNormalizationCheckResult {
    windows_core::link!("icuuc.dll" "C" fn unorm2_quickCheck(norm2 : *const UNormalizer2, s : *const u16, length : i32, perrorcode : *mut UErrorCode) -> UNormalizationCheckResult);
    unsafe { unorm2_quickCheck(norm2, s, length, perrorcode as _) }
}
#[inline]
pub unsafe fn unorm2_spanQuickCheckYes(norm2: *const UNormalizer2, s: *const u16, length: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn unorm2_spanQuickCheckYes(norm2 : *const UNormalizer2, s : *const u16, length : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { unorm2_spanQuickCheckYes(norm2, s, length, perrorcode as _) }
}
#[inline]
pub unsafe fn unorm_compare(s1: *const u16, length1: i32, s2: *const u16, length2: i32, options: u32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn unorm_compare(s1 : *const u16, length1 : i32, s2 : *const u16, length2 : i32, options : u32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { unorm_compare(s1, length1, s2, length2, options, perrorcode as _) }
}
#[inline]
pub unsafe fn unum_applyPattern(format: *mut *mut core::ffi::c_void, localized: i8, pattern: *const u16, patternlength: i32, parseerror: *mut UParseError, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn unum_applyPattern(format : *mut *mut core::ffi::c_void, localized : i8, pattern : *const u16, patternlength : i32, parseerror : *mut UParseError, status : *mut UErrorCode));
    unsafe { unum_applyPattern(format as _, localized, pattern, patternlength, parseerror as _, status as _) }
}
#[inline]
pub unsafe fn unum_clone(fmt: *const *const core::ffi::c_void, status: *mut UErrorCode) -> *mut *mut core::ffi::c_void {
    windows_core::link!("icuin.dll" "C" fn unum_clone(fmt : *const *const core::ffi::c_void, status : *mut UErrorCode) -> *mut *mut core::ffi::c_void);
    unsafe { unum_clone(fmt, status as _) }
}
#[inline]
pub unsafe fn unum_close(fmt: *mut *mut core::ffi::c_void) {
    windows_core::link!("icuin.dll" "C" fn unum_close(fmt : *mut *mut core::ffi::c_void));
    unsafe { unum_close(fmt as _) }
}
#[inline]
pub unsafe fn unum_countAvailable() -> i32 {
    windows_core::link!("icuin.dll" "C" fn unum_countAvailable() -> i32);
    unsafe { unum_countAvailable() }
}
#[inline]
pub unsafe fn unum_format(fmt: *const *const core::ffi::c_void, number: i32, result: *mut u16, resultlength: i32, pos: *mut UFieldPosition, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn unum_format(fmt : *const *const core::ffi::c_void, number : i32, result : *mut u16, resultlength : i32, pos : *mut UFieldPosition, status : *mut UErrorCode) -> i32);
    unsafe { unum_format(fmt, number, result as _, resultlength, pos as _, status as _) }
}
#[inline]
pub unsafe fn unum_formatDecimal<P1>(fmt: *const *const core::ffi::c_void, number: P1, length: i32, result: *mut u16, resultlength: i32, pos: *mut UFieldPosition, status: *mut UErrorCode) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuin.dll" "C" fn unum_formatDecimal(fmt : *const *const core::ffi::c_void, number : windows_core::PCSTR, length : i32, result : *mut u16, resultlength : i32, pos : *mut UFieldPosition, status : *mut UErrorCode) -> i32);
    unsafe { unum_formatDecimal(fmt, number.param().abi(), length, result as _, resultlength, pos as _, status as _) }
}
#[inline]
pub unsafe fn unum_formatDouble(fmt: *const *const core::ffi::c_void, number: f64, result: *mut u16, resultlength: i32, pos: *mut UFieldPosition, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn unum_formatDouble(fmt : *const *const core::ffi::c_void, number : f64, result : *mut u16, resultlength : i32, pos : *mut UFieldPosition, status : *mut UErrorCode) -> i32);
    unsafe { unum_formatDouble(fmt, number, result as _, resultlength, pos as _, status as _) }
}
#[inline]
pub unsafe fn unum_formatDoubleCurrency(fmt: *const *const core::ffi::c_void, number: f64, currency: *mut u16, result: *mut u16, resultlength: i32, pos: *mut UFieldPosition, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn unum_formatDoubleCurrency(fmt : *const *const core::ffi::c_void, number : f64, currency : *mut u16, result : *mut u16, resultlength : i32, pos : *mut UFieldPosition, status : *mut UErrorCode) -> i32);
    unsafe { unum_formatDoubleCurrency(fmt, number, currency as _, result as _, resultlength, pos as _, status as _) }
}
#[inline]
pub unsafe fn unum_formatDoubleForFields(format: *const *const core::ffi::c_void, number: f64, result: *mut u16, resultlength: i32, fpositer: *mut UFieldPositionIterator, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn unum_formatDoubleForFields(format : *const *const core::ffi::c_void, number : f64, result : *mut u16, resultlength : i32, fpositer : *mut UFieldPositionIterator, status : *mut UErrorCode) -> i32);
    unsafe { unum_formatDoubleForFields(format, number, result as _, resultlength, fpositer as _, status as _) }
}
#[inline]
pub unsafe fn unum_formatInt64(fmt: *const *const core::ffi::c_void, number: i64, result: *mut u16, resultlength: i32, pos: *mut UFieldPosition, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn unum_formatInt64(fmt : *const *const core::ffi::c_void, number : i64, result : *mut u16, resultlength : i32, pos : *mut UFieldPosition, status : *mut UErrorCode) -> i32);
    unsafe { unum_formatInt64(fmt, number, result as _, resultlength, pos as _, status as _) }
}
#[inline]
pub unsafe fn unum_formatUFormattable(fmt: *const *const core::ffi::c_void, number: *const *const core::ffi::c_void, result: *mut u16, resultlength: i32, pos: *mut UFieldPosition, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn unum_formatUFormattable(fmt : *const *const core::ffi::c_void, number : *const *const core::ffi::c_void, result : *mut u16, resultlength : i32, pos : *mut UFieldPosition, status : *mut UErrorCode) -> i32);
    unsafe { unum_formatUFormattable(fmt, number, result as _, resultlength, pos as _, status as _) }
}
#[inline]
pub unsafe fn unum_getAttribute(fmt: *const *const core::ffi::c_void, attr: UNumberFormatAttribute) -> i32 {
    windows_core::link!("icuin.dll" "C" fn unum_getAttribute(fmt : *const *const core::ffi::c_void, attr : UNumberFormatAttribute) -> i32);
    unsafe { unum_getAttribute(fmt, attr) }
}
#[inline]
pub unsafe fn unum_getAvailable(localeindex: i32) -> windows_core::PCSTR {
    windows_core::link!("icuin.dll" "C" fn unum_getAvailable(localeindex : i32) -> windows_core::PCSTR);
    unsafe { unum_getAvailable(localeindex) }
}
#[inline]
pub unsafe fn unum_getContext(fmt: *const *const core::ffi::c_void, r#type: UDisplayContextType, status: *mut UErrorCode) -> UDisplayContext {
    windows_core::link!("icuin.dll" "C" fn unum_getContext(fmt : *const *const core::ffi::c_void, r#type : UDisplayContextType, status : *mut UErrorCode) -> UDisplayContext);
    unsafe { unum_getContext(fmt, r#type, status as _) }
}
#[inline]
pub unsafe fn unum_getDoubleAttribute(fmt: *const *const core::ffi::c_void, attr: UNumberFormatAttribute) -> f64 {
    windows_core::link!("icuin.dll" "C" fn unum_getDoubleAttribute(fmt : *const *const core::ffi::c_void, attr : UNumberFormatAttribute) -> f64);
    unsafe { unum_getDoubleAttribute(fmt, attr) }
}
#[inline]
pub unsafe fn unum_getLocaleByType(fmt: *const *const core::ffi::c_void, r#type: ULocDataLocaleType, status: *mut UErrorCode) -> windows_core::PCSTR {
    windows_core::link!("icuin.dll" "C" fn unum_getLocaleByType(fmt : *const *const core::ffi::c_void, r#type : ULocDataLocaleType, status : *mut UErrorCode) -> windows_core::PCSTR);
    unsafe { unum_getLocaleByType(fmt, r#type, status as _) }
}
#[inline]
pub unsafe fn unum_getSymbol(fmt: *const *const core::ffi::c_void, symbol: UNumberFormatSymbol, buffer: *mut u16, size: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn unum_getSymbol(fmt : *const *const core::ffi::c_void, symbol : UNumberFormatSymbol, buffer : *mut u16, size : i32, status : *mut UErrorCode) -> i32);
    unsafe { unum_getSymbol(fmt, symbol, buffer as _, size, status as _) }
}
#[inline]
pub unsafe fn unum_getTextAttribute(fmt: *const *const core::ffi::c_void, tag: UNumberFormatTextAttribute, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn unum_getTextAttribute(fmt : *const *const core::ffi::c_void, tag : UNumberFormatTextAttribute, result : *mut u16, resultlength : i32, status : *mut UErrorCode) -> i32);
    unsafe { unum_getTextAttribute(fmt, tag, result as _, resultlength, status as _) }
}
#[inline]
pub unsafe fn unum_open<P3>(style: UNumberFormatStyle, pattern: *const u16, patternlength: i32, locale: P3, parseerr: *mut UParseError, status: *mut UErrorCode) -> *mut *mut core::ffi::c_void
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuin.dll" "C" fn unum_open(style : UNumberFormatStyle, pattern : *const u16, patternlength : i32, locale : windows_core::PCSTR, parseerr : *mut UParseError, status : *mut UErrorCode) -> *mut *mut core::ffi::c_void);
    unsafe { unum_open(style, pattern, patternlength, locale.param().abi(), parseerr as _, status as _) }
}
#[inline]
pub unsafe fn unum_parse(fmt: *const *const core::ffi::c_void, text: *const u16, textlength: i32, parsepos: *mut i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn unum_parse(fmt : *const *const core::ffi::c_void, text : *const u16, textlength : i32, parsepos : *mut i32, status : *mut UErrorCode) -> i32);
    unsafe { unum_parse(fmt, text, textlength, parsepos as _, status as _) }
}
#[inline]
pub unsafe fn unum_parseDecimal<P4>(fmt: *const *const core::ffi::c_void, text: *const u16, textlength: i32, parsepos: *mut i32, outbuf: P4, outbuflength: i32, status: *mut UErrorCode) -> i32
where
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuin.dll" "C" fn unum_parseDecimal(fmt : *const *const core::ffi::c_void, text : *const u16, textlength : i32, parsepos : *mut i32, outbuf : windows_core::PCSTR, outbuflength : i32, status : *mut UErrorCode) -> i32);
    unsafe { unum_parseDecimal(fmt, text, textlength, parsepos as _, outbuf.param().abi(), outbuflength, status as _) }
}
#[inline]
pub unsafe fn unum_parseDouble(fmt: *const *const core::ffi::c_void, text: *const u16, textlength: i32, parsepos: *mut i32, status: *mut UErrorCode) -> f64 {
    windows_core::link!("icuin.dll" "C" fn unum_parseDouble(fmt : *const *const core::ffi::c_void, text : *const u16, textlength : i32, parsepos : *mut i32, status : *mut UErrorCode) -> f64);
    unsafe { unum_parseDouble(fmt, text, textlength, parsepos as _, status as _) }
}
#[inline]
pub unsafe fn unum_parseDoubleCurrency(fmt: *const *const core::ffi::c_void, text: *const u16, textlength: i32, parsepos: *mut i32, currency: *mut u16, status: *mut UErrorCode) -> f64 {
    windows_core::link!("icuin.dll" "C" fn unum_parseDoubleCurrency(fmt : *const *const core::ffi::c_void, text : *const u16, textlength : i32, parsepos : *mut i32, currency : *mut u16, status : *mut UErrorCode) -> f64);
    unsafe { unum_parseDoubleCurrency(fmt, text, textlength, parsepos as _, currency as _, status as _) }
}
#[inline]
pub unsafe fn unum_parseInt64(fmt: *const *const core::ffi::c_void, text: *const u16, textlength: i32, parsepos: *mut i32, status: *mut UErrorCode) -> i64 {
    windows_core::link!("icuin.dll" "C" fn unum_parseInt64(fmt : *const *const core::ffi::c_void, text : *const u16, textlength : i32, parsepos : *mut i32, status : *mut UErrorCode) -> i64);
    unsafe { unum_parseInt64(fmt, text, textlength, parsepos as _, status as _) }
}
#[inline]
pub unsafe fn unum_parseToUFormattable(fmt: *const *const core::ffi::c_void, result: *mut *mut core::ffi::c_void, text: *const u16, textlength: i32, parsepos: *mut i32, status: *mut UErrorCode) -> *mut *mut core::ffi::c_void {
    windows_core::link!("icuin.dll" "C" fn unum_parseToUFormattable(fmt : *const *const core::ffi::c_void, result : *mut *mut core::ffi::c_void, text : *const u16, textlength : i32, parsepos : *mut i32, status : *mut UErrorCode) -> *mut *mut core::ffi::c_void);
    unsafe { unum_parseToUFormattable(fmt, result as _, text, textlength, parsepos as _, status as _) }
}
#[inline]
pub unsafe fn unum_setAttribute(fmt: *mut *mut core::ffi::c_void, attr: UNumberFormatAttribute, newvalue: i32) {
    windows_core::link!("icuin.dll" "C" fn unum_setAttribute(fmt : *mut *mut core::ffi::c_void, attr : UNumberFormatAttribute, newvalue : i32));
    unsafe { unum_setAttribute(fmt as _, attr, newvalue) }
}
#[inline]
pub unsafe fn unum_setContext(fmt: *mut *mut core::ffi::c_void, value: UDisplayContext, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn unum_setContext(fmt : *mut *mut core::ffi::c_void, value : UDisplayContext, status : *mut UErrorCode));
    unsafe { unum_setContext(fmt as _, value, status as _) }
}
#[inline]
pub unsafe fn unum_setDoubleAttribute(fmt: *mut *mut core::ffi::c_void, attr: UNumberFormatAttribute, newvalue: f64) {
    windows_core::link!("icuin.dll" "C" fn unum_setDoubleAttribute(fmt : *mut *mut core::ffi::c_void, attr : UNumberFormatAttribute, newvalue : f64));
    unsafe { unum_setDoubleAttribute(fmt as _, attr, newvalue) }
}
#[inline]
pub unsafe fn unum_setSymbol(fmt: *mut *mut core::ffi::c_void, symbol: UNumberFormatSymbol, value: *const u16, length: i32, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn unum_setSymbol(fmt : *mut *mut core::ffi::c_void, symbol : UNumberFormatSymbol, value : *const u16, length : i32, status : *mut UErrorCode));
    unsafe { unum_setSymbol(fmt as _, symbol, value, length, status as _) }
}
#[inline]
pub unsafe fn unum_setTextAttribute(fmt: *mut *mut core::ffi::c_void, tag: UNumberFormatTextAttribute, newvalue: *const u16, newvaluelength: i32, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn unum_setTextAttribute(fmt : *mut *mut core::ffi::c_void, tag : UNumberFormatTextAttribute, newvalue : *const u16, newvaluelength : i32, status : *mut UErrorCode));
    unsafe { unum_setTextAttribute(fmt as _, tag, newvalue, newvaluelength, status as _) }
}
#[inline]
pub unsafe fn unum_toPattern(fmt: *const *const core::ffi::c_void, ispatternlocalized: i8, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn unum_toPattern(fmt : *const *const core::ffi::c_void, ispatternlocalized : i8, result : *mut u16, resultlength : i32, status : *mut UErrorCode) -> i32);
    unsafe { unum_toPattern(fmt, ispatternlocalized, result as _, resultlength, status as _) }
}
#[inline]
pub unsafe fn unumf_close(uformatter: *mut UNumberFormatter) {
    windows_core::link!("icu.dll" "C" fn unumf_close(uformatter : *mut UNumberFormatter));
    unsafe { unumf_close(uformatter as _) }
}
#[inline]
pub unsafe fn unumf_closeResult(uresult: *mut UFormattedNumber) {
    windows_core::link!("icu.dll" "C" fn unumf_closeResult(uresult : *mut UFormattedNumber));
    unsafe { unumf_closeResult(uresult as _) }
}
#[inline]
pub unsafe fn unumf_formatDecimal<P1>(uformatter: *const UNumberFormatter, value: P1, valuelen: i32, uresult: *mut UFormattedNumber, ec: *mut UErrorCode)
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icu.dll" "C" fn unumf_formatDecimal(uformatter : *const UNumberFormatter, value : windows_core::PCSTR, valuelen : i32, uresult : *mut UFormattedNumber, ec : *mut UErrorCode));
    unsafe { unumf_formatDecimal(uformatter, value.param().abi(), valuelen, uresult as _, ec as _) }
}
#[inline]
pub unsafe fn unumf_formatDouble(uformatter: *const UNumberFormatter, value: f64, uresult: *mut UFormattedNumber, ec: *mut UErrorCode) {
    windows_core::link!("icu.dll" "C" fn unumf_formatDouble(uformatter : *const UNumberFormatter, value : f64, uresult : *mut UFormattedNumber, ec : *mut UErrorCode));
    unsafe { unumf_formatDouble(uformatter, value, uresult as _, ec as _) }
}
#[inline]
pub unsafe fn unumf_formatInt(uformatter: *const UNumberFormatter, value: i64, uresult: *mut UFormattedNumber, ec: *mut UErrorCode) {
    windows_core::link!("icu.dll" "C" fn unumf_formatInt(uformatter : *const UNumberFormatter, value : i64, uresult : *mut UFormattedNumber, ec : *mut UErrorCode));
    unsafe { unumf_formatInt(uformatter, value, uresult as _, ec as _) }
}
#[inline]
pub unsafe fn unumf_openForSkeletonAndLocale<P2>(skeleton: *const u16, skeletonlen: i32, locale: P2, ec: *mut UErrorCode) -> *mut UNumberFormatter
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icu.dll" "C" fn unumf_openForSkeletonAndLocale(skeleton : *const u16, skeletonlen : i32, locale : windows_core::PCSTR, ec : *mut UErrorCode) -> *mut UNumberFormatter);
    unsafe { unumf_openForSkeletonAndLocale(skeleton, skeletonlen, locale.param().abi(), ec as _) }
}
#[inline]
pub unsafe fn unumf_openForSkeletonAndLocaleWithError<P2>(skeleton: *const u16, skeletonlen: i32, locale: P2, perror: *mut UParseError, ec: *mut UErrorCode) -> *mut UNumberFormatter
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icu.dll" "C" fn unumf_openForSkeletonAndLocaleWithError(skeleton : *const u16, skeletonlen : i32, locale : windows_core::PCSTR, perror : *mut UParseError, ec : *mut UErrorCode) -> *mut UNumberFormatter);
    unsafe { unumf_openForSkeletonAndLocaleWithError(skeleton, skeletonlen, locale.param().abi(), perror as _, ec as _) }
}
#[inline]
pub unsafe fn unumf_openResult(ec: *mut UErrorCode) -> *mut UFormattedNumber {
    windows_core::link!("icu.dll" "C" fn unumf_openResult(ec : *mut UErrorCode) -> *mut UFormattedNumber);
    unsafe { unumf_openResult(ec as _) }
}
#[inline]
pub unsafe fn unumf_resultAsValue(uresult: *const UFormattedNumber, ec: *mut UErrorCode) -> *mut UFormattedValue {
    windows_core::link!("icu.dll" "C" fn unumf_resultAsValue(uresult : *const UFormattedNumber, ec : *mut UErrorCode) -> *mut UFormattedValue);
    unsafe { unumf_resultAsValue(uresult, ec as _) }
}
#[inline]
pub unsafe fn unumf_resultGetAllFieldPositions(uresult: *const UFormattedNumber, ufpositer: *mut UFieldPositionIterator, ec: *mut UErrorCode) {
    windows_core::link!("icu.dll" "C" fn unumf_resultGetAllFieldPositions(uresult : *const UFormattedNumber, ufpositer : *mut UFieldPositionIterator, ec : *mut UErrorCode));
    unsafe { unumf_resultGetAllFieldPositions(uresult, ufpositer as _, ec as _) }
}
#[inline]
pub unsafe fn unumf_resultNextFieldPosition(uresult: *const UFormattedNumber, ufpos: *mut UFieldPosition, ec: *mut UErrorCode) -> i8 {
    windows_core::link!("icu.dll" "C" fn unumf_resultNextFieldPosition(uresult : *const UFormattedNumber, ufpos : *mut UFieldPosition, ec : *mut UErrorCode) -> i8);
    unsafe { unumf_resultNextFieldPosition(uresult, ufpos as _, ec as _) }
}
#[inline]
pub unsafe fn unumf_resultToString(uresult: *const UFormattedNumber, buffer: *mut u16, buffercapacity: i32, ec: *mut UErrorCode) -> i32 {
    windows_core::link!("icu.dll" "C" fn unumf_resultToString(uresult : *const UFormattedNumber, buffer : *mut u16, buffercapacity : i32, ec : *mut UErrorCode) -> i32);
    unsafe { unumf_resultToString(uresult, buffer as _, buffercapacity, ec as _) }
}
#[inline]
pub unsafe fn unumsys_close(unumsys: *mut UNumberingSystem) {
    windows_core::link!("icuin.dll" "C" fn unumsys_close(unumsys : *mut UNumberingSystem));
    unsafe { unumsys_close(unumsys as _) }
}
#[inline]
pub unsafe fn unumsys_getDescription(unumsys: *const UNumberingSystem, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn unumsys_getDescription(unumsys : *const UNumberingSystem, result : *mut u16, resultlength : i32, status : *mut UErrorCode) -> i32);
    unsafe { unumsys_getDescription(unumsys, result as _, resultlength, status as _) }
}
#[inline]
pub unsafe fn unumsys_getName(unumsys: *const UNumberingSystem) -> windows_core::PCSTR {
    windows_core::link!("icuin.dll" "C" fn unumsys_getName(unumsys : *const UNumberingSystem) -> windows_core::PCSTR);
    unsafe { unumsys_getName(unumsys) }
}
#[inline]
pub unsafe fn unumsys_getRadix(unumsys: *const UNumberingSystem) -> i32 {
    windows_core::link!("icuin.dll" "C" fn unumsys_getRadix(unumsys : *const UNumberingSystem) -> i32);
    unsafe { unumsys_getRadix(unumsys) }
}
#[inline]
pub unsafe fn unumsys_isAlgorithmic(unumsys: *const UNumberingSystem) -> i8 {
    windows_core::link!("icuin.dll" "C" fn unumsys_isAlgorithmic(unumsys : *const UNumberingSystem) -> i8);
    unsafe { unumsys_isAlgorithmic(unumsys) }
}
#[inline]
pub unsafe fn unumsys_open<P0>(locale: P0, status: *mut UErrorCode) -> *mut UNumberingSystem
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuin.dll" "C" fn unumsys_open(locale : windows_core::PCSTR, status : *mut UErrorCode) -> *mut UNumberingSystem);
    unsafe { unumsys_open(locale.param().abi(), status as _) }
}
#[inline]
pub unsafe fn unumsys_openAvailableNames(status: *mut UErrorCode) -> *mut UEnumeration {
    windows_core::link!("icuin.dll" "C" fn unumsys_openAvailableNames(status : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { unumsys_openAvailableNames(status as _) }
}
#[inline]
pub unsafe fn unumsys_openByName<P0>(name: P0, status: *mut UErrorCode) -> *mut UNumberingSystem
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuin.dll" "C" fn unumsys_openByName(name : windows_core::PCSTR, status : *mut UErrorCode) -> *mut UNumberingSystem);
    unsafe { unumsys_openByName(name.param().abi(), status as _) }
}
#[inline]
pub unsafe fn uplrules_close(uplrules: *mut UPluralRules) {
    windows_core::link!("icuin.dll" "C" fn uplrules_close(uplrules : *mut UPluralRules));
    unsafe { uplrules_close(uplrules as _) }
}
#[inline]
pub unsafe fn uplrules_getKeywords(uplrules: *const UPluralRules, status: *mut UErrorCode) -> *mut UEnumeration {
    windows_core::link!("icuin.dll" "C" fn uplrules_getKeywords(uplrules : *const UPluralRules, status : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { uplrules_getKeywords(uplrules, status as _) }
}
#[inline]
pub unsafe fn uplrules_open<P0>(locale: P0, status: *mut UErrorCode) -> *mut UPluralRules
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuin.dll" "C" fn uplrules_open(locale : windows_core::PCSTR, status : *mut UErrorCode) -> *mut UPluralRules);
    unsafe { uplrules_open(locale.param().abi(), status as _) }
}
#[inline]
pub unsafe fn uplrules_openForType<P0>(locale: P0, r#type: UPluralType, status: *mut UErrorCode) -> *mut UPluralRules
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuin.dll" "C" fn uplrules_openForType(locale : windows_core::PCSTR, r#type : UPluralType, status : *mut UErrorCode) -> *mut UPluralRules);
    unsafe { uplrules_openForType(locale.param().abi(), r#type, status as _) }
}
#[inline]
pub unsafe fn uplrules_select(uplrules: *const UPluralRules, number: f64, keyword: *mut u16, capacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uplrules_select(uplrules : *const UPluralRules, number : f64, keyword : *mut u16, capacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { uplrules_select(uplrules, number, keyword as _, capacity, status as _) }
}
#[inline]
pub unsafe fn uplrules_selectFormatted(uplrules: *const UPluralRules, number: *const UFormattedNumber, keyword: *mut u16, capacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icu.dll" "C" fn uplrules_selectFormatted(uplrules : *const UPluralRules, number : *const UFormattedNumber, keyword : *mut u16, capacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { uplrules_selectFormatted(uplrules, number, keyword as _, capacity, status as _) }
}
#[inline]
pub unsafe fn uregex_appendReplacement(regexp: *mut URegularExpression, replacementtext: *const u16, replacementlength: i32, destbuf: *mut *mut u16, destcapacity: *mut i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uregex_appendReplacement(regexp : *mut URegularExpression, replacementtext : *const u16, replacementlength : i32, destbuf : *mut *mut u16, destcapacity : *mut i32, status : *mut UErrorCode) -> i32);
    unsafe { uregex_appendReplacement(regexp as _, replacementtext, replacementlength, destbuf as _, destcapacity as _, status as _) }
}
#[inline]
pub unsafe fn uregex_appendReplacementUText(regexp: *mut URegularExpression, replacementtext: *mut UText, dest: *mut UText, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn uregex_appendReplacementUText(regexp : *mut URegularExpression, replacementtext : *mut UText, dest : *mut UText, status : *mut UErrorCode));
    unsafe { uregex_appendReplacementUText(regexp as _, replacementtext as _, dest as _, status as _) }
}
#[inline]
pub unsafe fn uregex_appendTail(regexp: *mut URegularExpression, destbuf: *mut *mut u16, destcapacity: *mut i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uregex_appendTail(regexp : *mut URegularExpression, destbuf : *mut *mut u16, destcapacity : *mut i32, status : *mut UErrorCode) -> i32);
    unsafe { uregex_appendTail(regexp as _, destbuf as _, destcapacity as _, status as _) }
}
#[inline]
pub unsafe fn uregex_appendTailUText(regexp: *mut URegularExpression, dest: *mut UText, status: *mut UErrorCode) -> *mut UText {
    windows_core::link!("icuin.dll" "C" fn uregex_appendTailUText(regexp : *mut URegularExpression, dest : *mut UText, status : *mut UErrorCode) -> *mut UText);
    unsafe { uregex_appendTailUText(regexp as _, dest as _, status as _) }
}
#[inline]
pub unsafe fn uregex_clone(regexp: *const URegularExpression, status: *mut UErrorCode) -> *mut URegularExpression {
    windows_core::link!("icuin.dll" "C" fn uregex_clone(regexp : *const URegularExpression, status : *mut UErrorCode) -> *mut URegularExpression);
    unsafe { uregex_clone(regexp, status as _) }
}
#[inline]
pub unsafe fn uregex_close(regexp: *mut URegularExpression) {
    windows_core::link!("icuin.dll" "C" fn uregex_close(regexp : *mut URegularExpression));
    unsafe { uregex_close(regexp as _) }
}
#[inline]
pub unsafe fn uregex_end(regexp: *mut URegularExpression, groupnum: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uregex_end(regexp : *mut URegularExpression, groupnum : i32, status : *mut UErrorCode) -> i32);
    unsafe { uregex_end(regexp as _, groupnum, status as _) }
}
#[inline]
pub unsafe fn uregex_end64(regexp: *mut URegularExpression, groupnum: i32, status: *mut UErrorCode) -> i64 {
    windows_core::link!("icuin.dll" "C" fn uregex_end64(regexp : *mut URegularExpression, groupnum : i32, status : *mut UErrorCode) -> i64);
    unsafe { uregex_end64(regexp as _, groupnum, status as _) }
}
#[inline]
pub unsafe fn uregex_find(regexp: *mut URegularExpression, startindex: i32, status: *mut UErrorCode) -> i8 {
    windows_core::link!("icuin.dll" "C" fn uregex_find(regexp : *mut URegularExpression, startindex : i32, status : *mut UErrorCode) -> i8);
    unsafe { uregex_find(regexp as _, startindex, status as _) }
}
#[inline]
pub unsafe fn uregex_find64(regexp: *mut URegularExpression, startindex: i64, status: *mut UErrorCode) -> i8 {
    windows_core::link!("icuin.dll" "C" fn uregex_find64(regexp : *mut URegularExpression, startindex : i64, status : *mut UErrorCode) -> i8);
    unsafe { uregex_find64(regexp as _, startindex, status as _) }
}
#[inline]
pub unsafe fn uregex_findNext(regexp: *mut URegularExpression, status: *mut UErrorCode) -> i8 {
    windows_core::link!("icuin.dll" "C" fn uregex_findNext(regexp : *mut URegularExpression, status : *mut UErrorCode) -> i8);
    unsafe { uregex_findNext(regexp as _, status as _) }
}
#[inline]
pub unsafe fn uregex_flags(regexp: *const URegularExpression, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uregex_flags(regexp : *const URegularExpression, status : *mut UErrorCode) -> i32);
    unsafe { uregex_flags(regexp, status as _) }
}
#[inline]
pub unsafe fn uregex_getFindProgressCallback(regexp: *const URegularExpression, callback: *mut URegexFindProgressCallback, context: *const *const core::ffi::c_void, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn uregex_getFindProgressCallback(regexp : *const URegularExpression, callback : *mut URegexFindProgressCallback, context : *const *const core::ffi::c_void, status : *mut UErrorCode));
    unsafe { uregex_getFindProgressCallback(regexp, callback as _, context, status as _) }
}
#[inline]
pub unsafe fn uregex_getMatchCallback(regexp: *const URegularExpression, callback: *mut URegexMatchCallback, context: *const *const core::ffi::c_void, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn uregex_getMatchCallback(regexp : *const URegularExpression, callback : *mut URegexMatchCallback, context : *const *const core::ffi::c_void, status : *mut UErrorCode));
    unsafe { uregex_getMatchCallback(regexp, callback as _, context, status as _) }
}
#[inline]
pub unsafe fn uregex_getStackLimit(regexp: *const URegularExpression, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uregex_getStackLimit(regexp : *const URegularExpression, status : *mut UErrorCode) -> i32);
    unsafe { uregex_getStackLimit(regexp, status as _) }
}
#[inline]
pub unsafe fn uregex_getText(regexp: *mut URegularExpression, textlength: *mut i32, status: *mut UErrorCode) -> *mut u16 {
    windows_core::link!("icuin.dll" "C" fn uregex_getText(regexp : *mut URegularExpression, textlength : *mut i32, status : *mut UErrorCode) -> *mut u16);
    unsafe { uregex_getText(regexp as _, textlength as _, status as _) }
}
#[inline]
pub unsafe fn uregex_getTimeLimit(regexp: *const URegularExpression, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uregex_getTimeLimit(regexp : *const URegularExpression, status : *mut UErrorCode) -> i32);
    unsafe { uregex_getTimeLimit(regexp, status as _) }
}
#[inline]
pub unsafe fn uregex_getUText(regexp: *mut URegularExpression, dest: *mut UText, status: *mut UErrorCode) -> *mut UText {
    windows_core::link!("icuin.dll" "C" fn uregex_getUText(regexp : *mut URegularExpression, dest : *mut UText, status : *mut UErrorCode) -> *mut UText);
    unsafe { uregex_getUText(regexp as _, dest as _, status as _) }
}
#[inline]
pub unsafe fn uregex_group(regexp: *mut URegularExpression, groupnum: i32, dest: *mut u16, destcapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uregex_group(regexp : *mut URegularExpression, groupnum : i32, dest : *mut u16, destcapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { uregex_group(regexp as _, groupnum, dest as _, destcapacity, status as _) }
}
#[inline]
pub unsafe fn uregex_groupCount(regexp: *mut URegularExpression, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uregex_groupCount(regexp : *mut URegularExpression, status : *mut UErrorCode) -> i32);
    unsafe { uregex_groupCount(regexp as _, status as _) }
}
#[inline]
pub unsafe fn uregex_groupNumberFromCName<P1>(regexp: *mut URegularExpression, groupname: P1, namelength: i32, status: *mut UErrorCode) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuin.dll" "C" fn uregex_groupNumberFromCName(regexp : *mut URegularExpression, groupname : windows_core::PCSTR, namelength : i32, status : *mut UErrorCode) -> i32);
    unsafe { uregex_groupNumberFromCName(regexp as _, groupname.param().abi(), namelength, status as _) }
}
#[inline]
pub unsafe fn uregex_groupNumberFromName(regexp: *mut URegularExpression, groupname: *const u16, namelength: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uregex_groupNumberFromName(regexp : *mut URegularExpression, groupname : *const u16, namelength : i32, status : *mut UErrorCode) -> i32);
    unsafe { uregex_groupNumberFromName(regexp as _, groupname, namelength, status as _) }
}
#[inline]
pub unsafe fn uregex_groupUText(regexp: *mut URegularExpression, groupnum: i32, dest: *mut UText, grouplength: *mut i64, status: *mut UErrorCode) -> *mut UText {
    windows_core::link!("icuin.dll" "C" fn uregex_groupUText(regexp : *mut URegularExpression, groupnum : i32, dest : *mut UText, grouplength : *mut i64, status : *mut UErrorCode) -> *mut UText);
    unsafe { uregex_groupUText(regexp as _, groupnum, dest as _, grouplength as _, status as _) }
}
#[inline]
pub unsafe fn uregex_hasAnchoringBounds(regexp: *const URegularExpression, status: *mut UErrorCode) -> i8 {
    windows_core::link!("icuin.dll" "C" fn uregex_hasAnchoringBounds(regexp : *const URegularExpression, status : *mut UErrorCode) -> i8);
    unsafe { uregex_hasAnchoringBounds(regexp, status as _) }
}
#[inline]
pub unsafe fn uregex_hasTransparentBounds(regexp: *const URegularExpression, status: *mut UErrorCode) -> i8 {
    windows_core::link!("icuin.dll" "C" fn uregex_hasTransparentBounds(regexp : *const URegularExpression, status : *mut UErrorCode) -> i8);
    unsafe { uregex_hasTransparentBounds(regexp, status as _) }
}
#[inline]
pub unsafe fn uregex_hitEnd(regexp: *const URegularExpression, status: *mut UErrorCode) -> i8 {
    windows_core::link!("icuin.dll" "C" fn uregex_hitEnd(regexp : *const URegularExpression, status : *mut UErrorCode) -> i8);
    unsafe { uregex_hitEnd(regexp, status as _) }
}
#[inline]
pub unsafe fn uregex_lookingAt(regexp: *mut URegularExpression, startindex: i32, status: *mut UErrorCode) -> i8 {
    windows_core::link!("icuin.dll" "C" fn uregex_lookingAt(regexp : *mut URegularExpression, startindex : i32, status : *mut UErrorCode) -> i8);
    unsafe { uregex_lookingAt(regexp as _, startindex, status as _) }
}
#[inline]
pub unsafe fn uregex_lookingAt64(regexp: *mut URegularExpression, startindex: i64, status: *mut UErrorCode) -> i8 {
    windows_core::link!("icuin.dll" "C" fn uregex_lookingAt64(regexp : *mut URegularExpression, startindex : i64, status : *mut UErrorCode) -> i8);
    unsafe { uregex_lookingAt64(regexp as _, startindex, status as _) }
}
#[inline]
pub unsafe fn uregex_matches(regexp: *mut URegularExpression, startindex: i32, status: *mut UErrorCode) -> i8 {
    windows_core::link!("icuin.dll" "C" fn uregex_matches(regexp : *mut URegularExpression, startindex : i32, status : *mut UErrorCode) -> i8);
    unsafe { uregex_matches(regexp as _, startindex, status as _) }
}
#[inline]
pub unsafe fn uregex_matches64(regexp: *mut URegularExpression, startindex: i64, status: *mut UErrorCode) -> i8 {
    windows_core::link!("icuin.dll" "C" fn uregex_matches64(regexp : *mut URegularExpression, startindex : i64, status : *mut UErrorCode) -> i8);
    unsafe { uregex_matches64(regexp as _, startindex, status as _) }
}
#[inline]
pub unsafe fn uregex_open(pattern: *const u16, patternlength: i32, flags: u32, pe: *mut UParseError, status: *mut UErrorCode) -> *mut URegularExpression {
    windows_core::link!("icuin.dll" "C" fn uregex_open(pattern : *const u16, patternlength : i32, flags : u32, pe : *mut UParseError, status : *mut UErrorCode) -> *mut URegularExpression);
    unsafe { uregex_open(pattern, patternlength, flags, pe as _, status as _) }
}
#[inline]
pub unsafe fn uregex_openC<P0>(pattern: P0, flags: u32, pe: *mut UParseError, status: *mut UErrorCode) -> *mut URegularExpression
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuin.dll" "C" fn uregex_openC(pattern : windows_core::PCSTR, flags : u32, pe : *mut UParseError, status : *mut UErrorCode) -> *mut URegularExpression);
    unsafe { uregex_openC(pattern.param().abi(), flags, pe as _, status as _) }
}
#[inline]
pub unsafe fn uregex_openUText(pattern: *mut UText, flags: u32, pe: *mut UParseError, status: *mut UErrorCode) -> *mut URegularExpression {
    windows_core::link!("icuin.dll" "C" fn uregex_openUText(pattern : *mut UText, flags : u32, pe : *mut UParseError, status : *mut UErrorCode) -> *mut URegularExpression);
    unsafe { uregex_openUText(pattern as _, flags, pe as _, status as _) }
}
#[inline]
pub unsafe fn uregex_pattern(regexp: *const URegularExpression, patlength: *mut i32, status: *mut UErrorCode) -> *mut u16 {
    windows_core::link!("icuin.dll" "C" fn uregex_pattern(regexp : *const URegularExpression, patlength : *mut i32, status : *mut UErrorCode) -> *mut u16);
    unsafe { uregex_pattern(regexp, patlength as _, status as _) }
}
#[inline]
pub unsafe fn uregex_patternUText(regexp: *const URegularExpression, status: *mut UErrorCode) -> *mut UText {
    windows_core::link!("icuin.dll" "C" fn uregex_patternUText(regexp : *const URegularExpression, status : *mut UErrorCode) -> *mut UText);
    unsafe { uregex_patternUText(regexp, status as _) }
}
#[inline]
pub unsafe fn uregex_refreshUText(regexp: *mut URegularExpression, text: *mut UText, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn uregex_refreshUText(regexp : *mut URegularExpression, text : *mut UText, status : *mut UErrorCode));
    unsafe { uregex_refreshUText(regexp as _, text as _, status as _) }
}
#[inline]
pub unsafe fn uregex_regionEnd(regexp: *const URegularExpression, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uregex_regionEnd(regexp : *const URegularExpression, status : *mut UErrorCode) -> i32);
    unsafe { uregex_regionEnd(regexp, status as _) }
}
#[inline]
pub unsafe fn uregex_regionEnd64(regexp: *const URegularExpression, status: *mut UErrorCode) -> i64 {
    windows_core::link!("icuin.dll" "C" fn uregex_regionEnd64(regexp : *const URegularExpression, status : *mut UErrorCode) -> i64);
    unsafe { uregex_regionEnd64(regexp, status as _) }
}
#[inline]
pub unsafe fn uregex_regionStart(regexp: *const URegularExpression, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uregex_regionStart(regexp : *const URegularExpression, status : *mut UErrorCode) -> i32);
    unsafe { uregex_regionStart(regexp, status as _) }
}
#[inline]
pub unsafe fn uregex_regionStart64(regexp: *const URegularExpression, status: *mut UErrorCode) -> i64 {
    windows_core::link!("icuin.dll" "C" fn uregex_regionStart64(regexp : *const URegularExpression, status : *mut UErrorCode) -> i64);
    unsafe { uregex_regionStart64(regexp, status as _) }
}
#[inline]
pub unsafe fn uregex_replaceAll(regexp: *mut URegularExpression, replacementtext: *const u16, replacementlength: i32, destbuf: *mut u16, destcapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uregex_replaceAll(regexp : *mut URegularExpression, replacementtext : *const u16, replacementlength : i32, destbuf : *mut u16, destcapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { uregex_replaceAll(regexp as _, replacementtext, replacementlength, destbuf as _, destcapacity, status as _) }
}
#[inline]
pub unsafe fn uregex_replaceAllUText(regexp: *mut URegularExpression, replacement: *mut UText, dest: *mut UText, status: *mut UErrorCode) -> *mut UText {
    windows_core::link!("icuin.dll" "C" fn uregex_replaceAllUText(regexp : *mut URegularExpression, replacement : *mut UText, dest : *mut UText, status : *mut UErrorCode) -> *mut UText);
    unsafe { uregex_replaceAllUText(regexp as _, replacement as _, dest as _, status as _) }
}
#[inline]
pub unsafe fn uregex_replaceFirst(regexp: *mut URegularExpression, replacementtext: *const u16, replacementlength: i32, destbuf: *mut u16, destcapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uregex_replaceFirst(regexp : *mut URegularExpression, replacementtext : *const u16, replacementlength : i32, destbuf : *mut u16, destcapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { uregex_replaceFirst(regexp as _, replacementtext, replacementlength, destbuf as _, destcapacity, status as _) }
}
#[inline]
pub unsafe fn uregex_replaceFirstUText(regexp: *mut URegularExpression, replacement: *mut UText, dest: *mut UText, status: *mut UErrorCode) -> *mut UText {
    windows_core::link!("icuin.dll" "C" fn uregex_replaceFirstUText(regexp : *mut URegularExpression, replacement : *mut UText, dest : *mut UText, status : *mut UErrorCode) -> *mut UText);
    unsafe { uregex_replaceFirstUText(regexp as _, replacement as _, dest as _, status as _) }
}
#[inline]
pub unsafe fn uregex_requireEnd(regexp: *const URegularExpression, status: *mut UErrorCode) -> i8 {
    windows_core::link!("icuin.dll" "C" fn uregex_requireEnd(regexp : *const URegularExpression, status : *mut UErrorCode) -> i8);
    unsafe { uregex_requireEnd(regexp, status as _) }
}
#[inline]
pub unsafe fn uregex_reset(regexp: *mut URegularExpression, index: i32, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn uregex_reset(regexp : *mut URegularExpression, index : i32, status : *mut UErrorCode));
    unsafe { uregex_reset(regexp as _, index, status as _) }
}
#[inline]
pub unsafe fn uregex_reset64(regexp: *mut URegularExpression, index: i64, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn uregex_reset64(regexp : *mut URegularExpression, index : i64, status : *mut UErrorCode));
    unsafe { uregex_reset64(regexp as _, index, status as _) }
}
#[inline]
pub unsafe fn uregex_setFindProgressCallback(regexp: *mut URegularExpression, callback: URegexFindProgressCallback, context: *const core::ffi::c_void, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn uregex_setFindProgressCallback(regexp : *mut URegularExpression, callback : URegexFindProgressCallback, context : *const core::ffi::c_void, status : *mut UErrorCode));
    unsafe { uregex_setFindProgressCallback(regexp as _, callback, context, status as _) }
}
#[inline]
pub unsafe fn uregex_setMatchCallback(regexp: *mut URegularExpression, callback: URegexMatchCallback, context: *const core::ffi::c_void, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn uregex_setMatchCallback(regexp : *mut URegularExpression, callback : URegexMatchCallback, context : *const core::ffi::c_void, status : *mut UErrorCode));
    unsafe { uregex_setMatchCallback(regexp as _, callback, context, status as _) }
}
#[inline]
pub unsafe fn uregex_setRegion(regexp: *mut URegularExpression, regionstart: i32, regionlimit: i32, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn uregex_setRegion(regexp : *mut URegularExpression, regionstart : i32, regionlimit : i32, status : *mut UErrorCode));
    unsafe { uregex_setRegion(regexp as _, regionstart, regionlimit, status as _) }
}
#[inline]
pub unsafe fn uregex_setRegion64(regexp: *mut URegularExpression, regionstart: i64, regionlimit: i64, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn uregex_setRegion64(regexp : *mut URegularExpression, regionstart : i64, regionlimit : i64, status : *mut UErrorCode));
    unsafe { uregex_setRegion64(regexp as _, regionstart, regionlimit, status as _) }
}
#[inline]
pub unsafe fn uregex_setRegionAndStart(regexp: *mut URegularExpression, regionstart: i64, regionlimit: i64, startindex: i64, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn uregex_setRegionAndStart(regexp : *mut URegularExpression, regionstart : i64, regionlimit : i64, startindex : i64, status : *mut UErrorCode));
    unsafe { uregex_setRegionAndStart(regexp as _, regionstart, regionlimit, startindex, status as _) }
}
#[inline]
pub unsafe fn uregex_setStackLimit(regexp: *mut URegularExpression, limit: i32, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn uregex_setStackLimit(regexp : *mut URegularExpression, limit : i32, status : *mut UErrorCode));
    unsafe { uregex_setStackLimit(regexp as _, limit, status as _) }
}
#[inline]
pub unsafe fn uregex_setText(regexp: *mut URegularExpression, text: *const u16, textlength: i32, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn uregex_setText(regexp : *mut URegularExpression, text : *const u16, textlength : i32, status : *mut UErrorCode));
    unsafe { uregex_setText(regexp as _, text, textlength, status as _) }
}
#[inline]
pub unsafe fn uregex_setTimeLimit(regexp: *mut URegularExpression, limit: i32, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn uregex_setTimeLimit(regexp : *mut URegularExpression, limit : i32, status : *mut UErrorCode));
    unsafe { uregex_setTimeLimit(regexp as _, limit, status as _) }
}
#[inline]
pub unsafe fn uregex_setUText(regexp: *mut URegularExpression, text: *mut UText, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn uregex_setUText(regexp : *mut URegularExpression, text : *mut UText, status : *mut UErrorCode));
    unsafe { uregex_setUText(regexp as _, text as _, status as _) }
}
#[inline]
pub unsafe fn uregex_split(regexp: *mut URegularExpression, destbuf: *mut u16, destcapacity: i32, requiredcapacity: *mut i32, destfields: *mut *mut u16, destfieldscapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uregex_split(regexp : *mut URegularExpression, destbuf : *mut u16, destcapacity : i32, requiredcapacity : *mut i32, destfields : *mut *mut u16, destfieldscapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { uregex_split(regexp as _, destbuf as _, destcapacity, requiredcapacity as _, destfields as _, destfieldscapacity, status as _) }
}
#[inline]
pub unsafe fn uregex_splitUText(regexp: *mut URegularExpression, destfields: *mut *mut UText, destfieldscapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uregex_splitUText(regexp : *mut URegularExpression, destfields : *mut *mut UText, destfieldscapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { uregex_splitUText(regexp as _, destfields as _, destfieldscapacity, status as _) }
}
#[inline]
pub unsafe fn uregex_start(regexp: *mut URegularExpression, groupnum: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uregex_start(regexp : *mut URegularExpression, groupnum : i32, status : *mut UErrorCode) -> i32);
    unsafe { uregex_start(regexp as _, groupnum, status as _) }
}
#[inline]
pub unsafe fn uregex_start64(regexp: *mut URegularExpression, groupnum: i32, status: *mut UErrorCode) -> i64 {
    windows_core::link!("icuin.dll" "C" fn uregex_start64(regexp : *mut URegularExpression, groupnum : i32, status : *mut UErrorCode) -> i64);
    unsafe { uregex_start64(regexp as _, groupnum, status as _) }
}
#[inline]
pub unsafe fn uregex_useAnchoringBounds(regexp: *mut URegularExpression, b: i8, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn uregex_useAnchoringBounds(regexp : *mut URegularExpression, b : i8, status : *mut UErrorCode));
    unsafe { uregex_useAnchoringBounds(regexp as _, b, status as _) }
}
#[inline]
pub unsafe fn uregex_useTransparentBounds(regexp: *mut URegularExpression, b: i8, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn uregex_useTransparentBounds(regexp : *mut URegularExpression, b : i8, status : *mut UErrorCode));
    unsafe { uregex_useTransparentBounds(regexp as _, b, status as _) }
}
#[inline]
pub unsafe fn uregion_areEqual(uregion: *const URegion, otherregion: *const URegion) -> i8 {
    windows_core::link!("icuin.dll" "C" fn uregion_areEqual(uregion : *const URegion, otherregion : *const URegion) -> i8);
    unsafe { uregion_areEqual(uregion, otherregion) }
}
#[inline]
pub unsafe fn uregion_contains(uregion: *const URegion, otherregion: *const URegion) -> i8 {
    windows_core::link!("icuin.dll" "C" fn uregion_contains(uregion : *const URegion, otherregion : *const URegion) -> i8);
    unsafe { uregion_contains(uregion, otherregion) }
}
#[inline]
pub unsafe fn uregion_getAvailable(r#type: URegionType, status: *mut UErrorCode) -> *mut UEnumeration {
    windows_core::link!("icuin.dll" "C" fn uregion_getAvailable(r#type : URegionType, status : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { uregion_getAvailable(r#type, status as _) }
}
#[inline]
pub unsafe fn uregion_getContainedRegions(uregion: *const URegion, status: *mut UErrorCode) -> *mut UEnumeration {
    windows_core::link!("icuin.dll" "C" fn uregion_getContainedRegions(uregion : *const URegion, status : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { uregion_getContainedRegions(uregion, status as _) }
}
#[inline]
pub unsafe fn uregion_getContainedRegionsOfType(uregion: *const URegion, r#type: URegionType, status: *mut UErrorCode) -> *mut UEnumeration {
    windows_core::link!("icuin.dll" "C" fn uregion_getContainedRegionsOfType(uregion : *const URegion, r#type : URegionType, status : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { uregion_getContainedRegionsOfType(uregion, r#type, status as _) }
}
#[inline]
pub unsafe fn uregion_getContainingRegion(uregion: *const URegion) -> *mut URegion {
    windows_core::link!("icuin.dll" "C" fn uregion_getContainingRegion(uregion : *const URegion) -> *mut URegion);
    unsafe { uregion_getContainingRegion(uregion) }
}
#[inline]
pub unsafe fn uregion_getContainingRegionOfType(uregion: *const URegion, r#type: URegionType) -> *mut URegion {
    windows_core::link!("icuin.dll" "C" fn uregion_getContainingRegionOfType(uregion : *const URegion, r#type : URegionType) -> *mut URegion);
    unsafe { uregion_getContainingRegionOfType(uregion, r#type) }
}
#[inline]
pub unsafe fn uregion_getNumericCode(uregion: *const URegion) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uregion_getNumericCode(uregion : *const URegion) -> i32);
    unsafe { uregion_getNumericCode(uregion) }
}
#[inline]
pub unsafe fn uregion_getPreferredValues(uregion: *const URegion, status: *mut UErrorCode) -> *mut UEnumeration {
    windows_core::link!("icuin.dll" "C" fn uregion_getPreferredValues(uregion : *const URegion, status : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { uregion_getPreferredValues(uregion, status as _) }
}
#[inline]
pub unsafe fn uregion_getRegionCode(uregion: *const URegion) -> windows_core::PCSTR {
    windows_core::link!("icuin.dll" "C" fn uregion_getRegionCode(uregion : *const URegion) -> windows_core::PCSTR);
    unsafe { uregion_getRegionCode(uregion) }
}
#[inline]
pub unsafe fn uregion_getRegionFromCode<P0>(regioncode: P0, status: *mut UErrorCode) -> *mut URegion
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuin.dll" "C" fn uregion_getRegionFromCode(regioncode : windows_core::PCSTR, status : *mut UErrorCode) -> *mut URegion);
    unsafe { uregion_getRegionFromCode(regioncode.param().abi(), status as _) }
}
#[inline]
pub unsafe fn uregion_getRegionFromNumericCode(code: i32, status: *mut UErrorCode) -> *mut URegion {
    windows_core::link!("icuin.dll" "C" fn uregion_getRegionFromNumericCode(code : i32, status : *mut UErrorCode) -> *mut URegion);
    unsafe { uregion_getRegionFromNumericCode(code, status as _) }
}
#[inline]
pub unsafe fn uregion_getType(uregion: *const URegion) -> URegionType {
    windows_core::link!("icuin.dll" "C" fn uregion_getType(uregion : *const URegion) -> URegionType);
    unsafe { uregion_getType(uregion) }
}
#[inline]
pub unsafe fn ureldatefmt_close(reldatefmt: *mut URelativeDateTimeFormatter) {
    windows_core::link!("icuin.dll" "C" fn ureldatefmt_close(reldatefmt : *mut URelativeDateTimeFormatter));
    unsafe { ureldatefmt_close(reldatefmt as _) }
}
#[inline]
pub unsafe fn ureldatefmt_closeResult(ufrdt: *mut UFormattedRelativeDateTime) {
    windows_core::link!("icu.dll" "C" fn ureldatefmt_closeResult(ufrdt : *mut UFormattedRelativeDateTime));
    unsafe { ureldatefmt_closeResult(ufrdt as _) }
}
#[inline]
pub unsafe fn ureldatefmt_combineDateAndTime(reldatefmt: *const URelativeDateTimeFormatter, relativedatestring: *const u16, relativedatestringlen: i32, timestring: *const u16, timestringlen: i32, result: *mut u16, resultcapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ureldatefmt_combineDateAndTime(reldatefmt : *const URelativeDateTimeFormatter, relativedatestring : *const u16, relativedatestringlen : i32, timestring : *const u16, timestringlen : i32, result : *mut u16, resultcapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { ureldatefmt_combineDateAndTime(reldatefmt, relativedatestring, relativedatestringlen, timestring, timestringlen, result as _, resultcapacity, status as _) }
}
#[inline]
pub unsafe fn ureldatefmt_format(reldatefmt: *const URelativeDateTimeFormatter, offset: f64, unit: URelativeDateTimeUnit, result: *mut u16, resultcapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ureldatefmt_format(reldatefmt : *const URelativeDateTimeFormatter, offset : f64, unit : URelativeDateTimeUnit, result : *mut u16, resultcapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { ureldatefmt_format(reldatefmt, offset, unit, result as _, resultcapacity, status as _) }
}
#[inline]
pub unsafe fn ureldatefmt_formatNumeric(reldatefmt: *const URelativeDateTimeFormatter, offset: f64, unit: URelativeDateTimeUnit, result: *mut u16, resultcapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn ureldatefmt_formatNumeric(reldatefmt : *const URelativeDateTimeFormatter, offset : f64, unit : URelativeDateTimeUnit, result : *mut u16, resultcapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { ureldatefmt_formatNumeric(reldatefmt, offset, unit, result as _, resultcapacity, status as _) }
}
#[inline]
pub unsafe fn ureldatefmt_formatNumericToResult(reldatefmt: *const URelativeDateTimeFormatter, offset: f64, unit: URelativeDateTimeUnit, result: *mut UFormattedRelativeDateTime, status: *mut UErrorCode) {
    windows_core::link!("icu.dll" "C" fn ureldatefmt_formatNumericToResult(reldatefmt : *const URelativeDateTimeFormatter, offset : f64, unit : URelativeDateTimeUnit, result : *mut UFormattedRelativeDateTime, status : *mut UErrorCode));
    unsafe { ureldatefmt_formatNumericToResult(reldatefmt, offset, unit, result as _, status as _) }
}
#[inline]
pub unsafe fn ureldatefmt_formatToResult(reldatefmt: *const URelativeDateTimeFormatter, offset: f64, unit: URelativeDateTimeUnit, result: *mut UFormattedRelativeDateTime, status: *mut UErrorCode) {
    windows_core::link!("icu.dll" "C" fn ureldatefmt_formatToResult(reldatefmt : *const URelativeDateTimeFormatter, offset : f64, unit : URelativeDateTimeUnit, result : *mut UFormattedRelativeDateTime, status : *mut UErrorCode));
    unsafe { ureldatefmt_formatToResult(reldatefmt, offset, unit, result as _, status as _) }
}
#[inline]
pub unsafe fn ureldatefmt_open<P0>(locale: P0, nftoadopt: *mut *mut core::ffi::c_void, width: UDateRelativeDateTimeFormatterStyle, capitalizationcontext: UDisplayContext, status: *mut UErrorCode) -> *mut URelativeDateTimeFormatter
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuin.dll" "C" fn ureldatefmt_open(locale : windows_core::PCSTR, nftoadopt : *mut *mut core::ffi::c_void, width : UDateRelativeDateTimeFormatterStyle, capitalizationcontext : UDisplayContext, status : *mut UErrorCode) -> *mut URelativeDateTimeFormatter);
    unsafe { ureldatefmt_open(locale.param().abi(), nftoadopt as _, width, capitalizationcontext, status as _) }
}
#[inline]
pub unsafe fn ureldatefmt_openResult(ec: *mut UErrorCode) -> *mut UFormattedRelativeDateTime {
    windows_core::link!("icu.dll" "C" fn ureldatefmt_openResult(ec : *mut UErrorCode) -> *mut UFormattedRelativeDateTime);
    unsafe { ureldatefmt_openResult(ec as _) }
}
#[inline]
pub unsafe fn ureldatefmt_resultAsValue(ufrdt: *const UFormattedRelativeDateTime, ec: *mut UErrorCode) -> *mut UFormattedValue {
    windows_core::link!("icu.dll" "C" fn ureldatefmt_resultAsValue(ufrdt : *const UFormattedRelativeDateTime, ec : *mut UErrorCode) -> *mut UFormattedValue);
    unsafe { ureldatefmt_resultAsValue(ufrdt, ec as _) }
}
#[inline]
pub unsafe fn ures_close(resourcebundle: *mut UResourceBundle) {
    windows_core::link!("icuuc.dll" "C" fn ures_close(resourcebundle : *mut UResourceBundle));
    unsafe { ures_close(resourcebundle as _) }
}
#[inline]
pub unsafe fn ures_getBinary(resourcebundle: *const UResourceBundle, len: *mut i32, status: *mut UErrorCode) -> *mut u8 {
    windows_core::link!("icuuc.dll" "C" fn ures_getBinary(resourcebundle : *const UResourceBundle, len : *mut i32, status : *mut UErrorCode) -> *mut u8);
    unsafe { ures_getBinary(resourcebundle, len as _, status as _) }
}
#[inline]
pub unsafe fn ures_getByIndex(resourcebundle: *const UResourceBundle, indexr: i32, fillin: *mut UResourceBundle, status: *mut UErrorCode) -> *mut UResourceBundle {
    windows_core::link!("icuuc.dll" "C" fn ures_getByIndex(resourcebundle : *const UResourceBundle, indexr : i32, fillin : *mut UResourceBundle, status : *mut UErrorCode) -> *mut UResourceBundle);
    unsafe { ures_getByIndex(resourcebundle, indexr, fillin as _, status as _) }
}
#[inline]
pub unsafe fn ures_getByKey<P1>(resourcebundle: *const UResourceBundle, key: P1, fillin: *mut UResourceBundle, status: *mut UErrorCode) -> *mut UResourceBundle
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn ures_getByKey(resourcebundle : *const UResourceBundle, key : windows_core::PCSTR, fillin : *mut UResourceBundle, status : *mut UErrorCode) -> *mut UResourceBundle);
    unsafe { ures_getByKey(resourcebundle, key.param().abi(), fillin as _, status as _) }
}
#[inline]
pub unsafe fn ures_getInt(resourcebundle: *const UResourceBundle, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ures_getInt(resourcebundle : *const UResourceBundle, status : *mut UErrorCode) -> i32);
    unsafe { ures_getInt(resourcebundle, status as _) }
}
#[inline]
pub unsafe fn ures_getIntVector(resourcebundle: *const UResourceBundle, len: *mut i32, status: *mut UErrorCode) -> *mut i32 {
    windows_core::link!("icuuc.dll" "C" fn ures_getIntVector(resourcebundle : *const UResourceBundle, len : *mut i32, status : *mut UErrorCode) -> *mut i32);
    unsafe { ures_getIntVector(resourcebundle, len as _, status as _) }
}
#[inline]
pub unsafe fn ures_getKey(resourcebundle: *const UResourceBundle) -> windows_core::PCSTR {
    windows_core::link!("icuuc.dll" "C" fn ures_getKey(resourcebundle : *const UResourceBundle) -> windows_core::PCSTR);
    unsafe { ures_getKey(resourcebundle) }
}
#[inline]
pub unsafe fn ures_getLocaleByType(resourcebundle: *const UResourceBundle, r#type: ULocDataLocaleType, status: *mut UErrorCode) -> windows_core::PCSTR {
    windows_core::link!("icuuc.dll" "C" fn ures_getLocaleByType(resourcebundle : *const UResourceBundle, r#type : ULocDataLocaleType, status : *mut UErrorCode) -> windows_core::PCSTR);
    unsafe { ures_getLocaleByType(resourcebundle, r#type, status as _) }
}
#[inline]
pub unsafe fn ures_getNextResource(resourcebundle: *mut UResourceBundle, fillin: *mut UResourceBundle, status: *mut UErrorCode) -> *mut UResourceBundle {
    windows_core::link!("icuuc.dll" "C" fn ures_getNextResource(resourcebundle : *mut UResourceBundle, fillin : *mut UResourceBundle, status : *mut UErrorCode) -> *mut UResourceBundle);
    unsafe { ures_getNextResource(resourcebundle as _, fillin as _, status as _) }
}
#[inline]
pub unsafe fn ures_getNextString(resourcebundle: *mut UResourceBundle, len: *mut i32, key: *const *const i8, status: *mut UErrorCode) -> *mut u16 {
    windows_core::link!("icuuc.dll" "C" fn ures_getNextString(resourcebundle : *mut UResourceBundle, len : *mut i32, key : *const *const i8, status : *mut UErrorCode) -> *mut u16);
    unsafe { ures_getNextString(resourcebundle as _, len as _, key, status as _) }
}
#[inline]
pub unsafe fn ures_getSize(resourcebundle: *const UResourceBundle) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn ures_getSize(resourcebundle : *const UResourceBundle) -> i32);
    unsafe { ures_getSize(resourcebundle) }
}
#[inline]
pub unsafe fn ures_getString(resourcebundle: *const UResourceBundle, len: *mut i32, status: *mut UErrorCode) -> *mut u16 {
    windows_core::link!("icuuc.dll" "C" fn ures_getString(resourcebundle : *const UResourceBundle, len : *mut i32, status : *mut UErrorCode) -> *mut u16);
    unsafe { ures_getString(resourcebundle, len as _, status as _) }
}
#[inline]
pub unsafe fn ures_getStringByIndex(resourcebundle: *const UResourceBundle, indexs: i32, len: *mut i32, status: *mut UErrorCode) -> *mut u16 {
    windows_core::link!("icuuc.dll" "C" fn ures_getStringByIndex(resourcebundle : *const UResourceBundle, indexs : i32, len : *mut i32, status : *mut UErrorCode) -> *mut u16);
    unsafe { ures_getStringByIndex(resourcebundle, indexs, len as _, status as _) }
}
#[inline]
pub unsafe fn ures_getStringByKey<P1>(resb: *const UResourceBundle, key: P1, len: *mut i32, status: *mut UErrorCode) -> *mut u16
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn ures_getStringByKey(resb : *const UResourceBundle, key : windows_core::PCSTR, len : *mut i32, status : *mut UErrorCode) -> *mut u16);
    unsafe { ures_getStringByKey(resb, key.param().abi(), len as _, status as _) }
}
#[inline]
pub unsafe fn ures_getType(resourcebundle: *const UResourceBundle) -> UResType {
    windows_core::link!("icuuc.dll" "C" fn ures_getType(resourcebundle : *const UResourceBundle) -> UResType);
    unsafe { ures_getType(resourcebundle) }
}
#[inline]
pub unsafe fn ures_getUInt(resourcebundle: *const UResourceBundle, status: *mut UErrorCode) -> u32 {
    windows_core::link!("icuuc.dll" "C" fn ures_getUInt(resourcebundle : *const UResourceBundle, status : *mut UErrorCode) -> u32);
    unsafe { ures_getUInt(resourcebundle, status as _) }
}
#[inline]
pub unsafe fn ures_getUTF8String<P1>(resb: *const UResourceBundle, dest: P1, length: *mut i32, forcecopy: i8, status: *mut UErrorCode) -> windows_core::PCSTR
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn ures_getUTF8String(resb : *const UResourceBundle, dest : windows_core::PCSTR, length : *mut i32, forcecopy : i8, status : *mut UErrorCode) -> windows_core::PCSTR);
    unsafe { ures_getUTF8String(resb, dest.param().abi(), length as _, forcecopy, status as _) }
}
#[inline]
pub unsafe fn ures_getUTF8StringByIndex<P2>(resb: *const UResourceBundle, stringindex: i32, dest: P2, plength: *mut i32, forcecopy: i8, status: *mut UErrorCode) -> windows_core::PCSTR
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn ures_getUTF8StringByIndex(resb : *const UResourceBundle, stringindex : i32, dest : windows_core::PCSTR, plength : *mut i32, forcecopy : i8, status : *mut UErrorCode) -> windows_core::PCSTR);
    unsafe { ures_getUTF8StringByIndex(resb, stringindex, dest.param().abi(), plength as _, forcecopy, status as _) }
}
#[inline]
pub unsafe fn ures_getUTF8StringByKey<P1, P2>(resb: *const UResourceBundle, key: P1, dest: P2, plength: *mut i32, forcecopy: i8, status: *mut UErrorCode) -> windows_core::PCSTR
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn ures_getUTF8StringByKey(resb : *const UResourceBundle, key : windows_core::PCSTR, dest : windows_core::PCSTR, plength : *mut i32, forcecopy : i8, status : *mut UErrorCode) -> windows_core::PCSTR);
    unsafe { ures_getUTF8StringByKey(resb, key.param().abi(), dest.param().abi(), plength as _, forcecopy, status as _) }
}
#[inline]
pub unsafe fn ures_getVersion(resb: *const UResourceBundle, versioninfo: *mut u8) {
    windows_core::link!("icuuc.dll" "C" fn ures_getVersion(resb : *const UResourceBundle, versioninfo : *mut u8));
    unsafe { ures_getVersion(resb, versioninfo as _) }
}
#[inline]
pub unsafe fn ures_hasNext(resourcebundle: *const UResourceBundle) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn ures_hasNext(resourcebundle : *const UResourceBundle) -> i8);
    unsafe { ures_hasNext(resourcebundle) }
}
#[inline]
pub unsafe fn ures_open<P0, P1>(packagename: P0, locale: P1, status: *mut UErrorCode) -> *mut UResourceBundle
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn ures_open(packagename : windows_core::PCSTR, locale : windows_core::PCSTR, status : *mut UErrorCode) -> *mut UResourceBundle);
    unsafe { ures_open(packagename.param().abi(), locale.param().abi(), status as _) }
}
#[inline]
pub unsafe fn ures_openAvailableLocales<P0>(packagename: P0, status: *mut UErrorCode) -> *mut UEnumeration
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn ures_openAvailableLocales(packagename : windows_core::PCSTR, status : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { ures_openAvailableLocales(packagename.param().abi(), status as _) }
}
#[inline]
pub unsafe fn ures_openDirect<P0, P1>(packagename: P0, locale: P1, status: *mut UErrorCode) -> *mut UResourceBundle
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn ures_openDirect(packagename : windows_core::PCSTR, locale : windows_core::PCSTR, status : *mut UErrorCode) -> *mut UResourceBundle);
    unsafe { ures_openDirect(packagename.param().abi(), locale.param().abi(), status as _) }
}
#[inline]
pub unsafe fn ures_openU<P1>(packagename: *const u16, locale: P1, status: *mut UErrorCode) -> *mut UResourceBundle
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn ures_openU(packagename : *const u16, locale : windows_core::PCSTR, status : *mut UErrorCode) -> *mut UResourceBundle);
    unsafe { ures_openU(packagename, locale.param().abi(), status as _) }
}
#[inline]
pub unsafe fn ures_resetIterator(resourcebundle: *mut UResourceBundle) {
    windows_core::link!("icuuc.dll" "C" fn ures_resetIterator(resourcebundle : *mut UResourceBundle));
    unsafe { ures_resetIterator(resourcebundle as _) }
}
#[inline]
pub unsafe fn uscript_breaksBetweenLetters(script: UScriptCode) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn uscript_breaksBetweenLetters(script : UScriptCode) -> i8);
    unsafe { uscript_breaksBetweenLetters(script) }
}
#[inline]
pub unsafe fn uscript_getCode<P0>(nameorabbrorlocale: P0, fillin: *mut UScriptCode, capacity: i32, err: *mut UErrorCode) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uscript_getCode(nameorabbrorlocale : windows_core::PCSTR, fillin : *mut UScriptCode, capacity : i32, err : *mut UErrorCode) -> i32);
    unsafe { uscript_getCode(nameorabbrorlocale.param().abi(), fillin as _, capacity, err as _) }
}
#[inline]
pub unsafe fn uscript_getName(scriptcode: UScriptCode) -> windows_core::PCSTR {
    windows_core::link!("icuuc.dll" "C" fn uscript_getName(scriptcode : UScriptCode) -> windows_core::PCSTR);
    unsafe { uscript_getName(scriptcode) }
}
#[inline]
pub unsafe fn uscript_getSampleString(script: UScriptCode, dest: *mut u16, capacity: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uscript_getSampleString(script : UScriptCode, dest : *mut u16, capacity : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { uscript_getSampleString(script, dest as _, capacity, perrorcode as _) }
}
#[inline]
pub unsafe fn uscript_getScript(codepoint: i32, err: *mut UErrorCode) -> UScriptCode {
    windows_core::link!("icuuc.dll" "C" fn uscript_getScript(codepoint : i32, err : *mut UErrorCode) -> UScriptCode);
    unsafe { uscript_getScript(codepoint, err as _) }
}
#[inline]
pub unsafe fn uscript_getScriptExtensions(c: i32, scripts: *mut UScriptCode, capacity: i32, errorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uscript_getScriptExtensions(c : i32, scripts : *mut UScriptCode, capacity : i32, errorcode : *mut UErrorCode) -> i32);
    unsafe { uscript_getScriptExtensions(c, scripts as _, capacity, errorcode as _) }
}
#[inline]
pub unsafe fn uscript_getShortName(scriptcode: UScriptCode) -> windows_core::PCSTR {
    windows_core::link!("icuuc.dll" "C" fn uscript_getShortName(scriptcode : UScriptCode) -> windows_core::PCSTR);
    unsafe { uscript_getShortName(scriptcode) }
}
#[inline]
pub unsafe fn uscript_getUsage(script: UScriptCode) -> UScriptUsage {
    windows_core::link!("icuuc.dll" "C" fn uscript_getUsage(script : UScriptCode) -> UScriptUsage);
    unsafe { uscript_getUsage(script) }
}
#[inline]
pub unsafe fn uscript_hasScript(c: i32, sc: UScriptCode) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn uscript_hasScript(c : i32, sc : UScriptCode) -> i8);
    unsafe { uscript_hasScript(c, sc) }
}
#[inline]
pub unsafe fn uscript_isCased(script: UScriptCode) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn uscript_isCased(script : UScriptCode) -> i8);
    unsafe { uscript_isCased(script) }
}
#[inline]
pub unsafe fn uscript_isRightToLeft(script: UScriptCode) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn uscript_isRightToLeft(script : UScriptCode) -> i8);
    unsafe { uscript_isRightToLeft(script) }
}
#[inline]
pub unsafe fn usearch_close(searchiter: *mut UStringSearch) {
    windows_core::link!("icuin.dll" "C" fn usearch_close(searchiter : *mut UStringSearch));
    unsafe { usearch_close(searchiter as _) }
}
#[inline]
pub unsafe fn usearch_first(strsrch: *mut UStringSearch, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn usearch_first(strsrch : *mut UStringSearch, status : *mut UErrorCode) -> i32);
    unsafe { usearch_first(strsrch as _, status as _) }
}
#[inline]
pub unsafe fn usearch_following(strsrch: *mut UStringSearch, position: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn usearch_following(strsrch : *mut UStringSearch, position : i32, status : *mut UErrorCode) -> i32);
    unsafe { usearch_following(strsrch as _, position, status as _) }
}
#[inline]
pub unsafe fn usearch_getAttribute(strsrch: *const UStringSearch, attribute: USearchAttribute) -> USearchAttributeValue {
    windows_core::link!("icuin.dll" "C" fn usearch_getAttribute(strsrch : *const UStringSearch, attribute : USearchAttribute) -> USearchAttributeValue);
    unsafe { usearch_getAttribute(strsrch, attribute) }
}
#[inline]
pub unsafe fn usearch_getBreakIterator(strsrch: *const UStringSearch) -> *mut UBreakIterator {
    windows_core::link!("icuin.dll" "C" fn usearch_getBreakIterator(strsrch : *const UStringSearch) -> *mut UBreakIterator);
    unsafe { usearch_getBreakIterator(strsrch) }
}
#[inline]
pub unsafe fn usearch_getCollator(strsrch: *const UStringSearch) -> *mut UCollator {
    windows_core::link!("icuin.dll" "C" fn usearch_getCollator(strsrch : *const UStringSearch) -> *mut UCollator);
    unsafe { usearch_getCollator(strsrch) }
}
#[inline]
pub unsafe fn usearch_getMatchedLength(strsrch: *const UStringSearch) -> i32 {
    windows_core::link!("icuin.dll" "C" fn usearch_getMatchedLength(strsrch : *const UStringSearch) -> i32);
    unsafe { usearch_getMatchedLength(strsrch) }
}
#[inline]
pub unsafe fn usearch_getMatchedStart(strsrch: *const UStringSearch) -> i32 {
    windows_core::link!("icuin.dll" "C" fn usearch_getMatchedStart(strsrch : *const UStringSearch) -> i32);
    unsafe { usearch_getMatchedStart(strsrch) }
}
#[inline]
pub unsafe fn usearch_getMatchedText(strsrch: *const UStringSearch, result: *mut u16, resultcapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn usearch_getMatchedText(strsrch : *const UStringSearch, result : *mut u16, resultcapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { usearch_getMatchedText(strsrch, result as _, resultcapacity, status as _) }
}
#[inline]
pub unsafe fn usearch_getOffset(strsrch: *const UStringSearch) -> i32 {
    windows_core::link!("icuin.dll" "C" fn usearch_getOffset(strsrch : *const UStringSearch) -> i32);
    unsafe { usearch_getOffset(strsrch) }
}
#[inline]
pub unsafe fn usearch_getPattern(strsrch: *const UStringSearch, length: *mut i32) -> *mut u16 {
    windows_core::link!("icuin.dll" "C" fn usearch_getPattern(strsrch : *const UStringSearch, length : *mut i32) -> *mut u16);
    unsafe { usearch_getPattern(strsrch, length as _) }
}
#[inline]
pub unsafe fn usearch_getText(strsrch: *const UStringSearch, length: *mut i32) -> *mut u16 {
    windows_core::link!("icuin.dll" "C" fn usearch_getText(strsrch : *const UStringSearch, length : *mut i32) -> *mut u16);
    unsafe { usearch_getText(strsrch, length as _) }
}
#[inline]
pub unsafe fn usearch_last(strsrch: *mut UStringSearch, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn usearch_last(strsrch : *mut UStringSearch, status : *mut UErrorCode) -> i32);
    unsafe { usearch_last(strsrch as _, status as _) }
}
#[inline]
pub unsafe fn usearch_next(strsrch: *mut UStringSearch, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn usearch_next(strsrch : *mut UStringSearch, status : *mut UErrorCode) -> i32);
    unsafe { usearch_next(strsrch as _, status as _) }
}
#[inline]
pub unsafe fn usearch_open<P4>(pattern: *const u16, patternlength: i32, text: *const u16, textlength: i32, locale: P4, breakiter: *mut UBreakIterator, status: *mut UErrorCode) -> *mut UStringSearch
where
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuin.dll" "C" fn usearch_open(pattern : *const u16, patternlength : i32, text : *const u16, textlength : i32, locale : windows_core::PCSTR, breakiter : *mut UBreakIterator, status : *mut UErrorCode) -> *mut UStringSearch);
    unsafe { usearch_open(pattern, patternlength, text, textlength, locale.param().abi(), breakiter as _, status as _) }
}
#[inline]
pub unsafe fn usearch_openFromCollator(pattern: *const u16, patternlength: i32, text: *const u16, textlength: i32, collator: *const UCollator, breakiter: *mut UBreakIterator, status: *mut UErrorCode) -> *mut UStringSearch {
    windows_core::link!("icuin.dll" "C" fn usearch_openFromCollator(pattern : *const u16, patternlength : i32, text : *const u16, textlength : i32, collator : *const UCollator, breakiter : *mut UBreakIterator, status : *mut UErrorCode) -> *mut UStringSearch);
    unsafe { usearch_openFromCollator(pattern, patternlength, text, textlength, collator, breakiter as _, status as _) }
}
#[inline]
pub unsafe fn usearch_preceding(strsrch: *mut UStringSearch, position: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn usearch_preceding(strsrch : *mut UStringSearch, position : i32, status : *mut UErrorCode) -> i32);
    unsafe { usearch_preceding(strsrch as _, position, status as _) }
}
#[inline]
pub unsafe fn usearch_previous(strsrch: *mut UStringSearch, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn usearch_previous(strsrch : *mut UStringSearch, status : *mut UErrorCode) -> i32);
    unsafe { usearch_previous(strsrch as _, status as _) }
}
#[inline]
pub unsafe fn usearch_reset(strsrch: *mut UStringSearch) {
    windows_core::link!("icuin.dll" "C" fn usearch_reset(strsrch : *mut UStringSearch));
    unsafe { usearch_reset(strsrch as _) }
}
#[inline]
pub unsafe fn usearch_setAttribute(strsrch: *mut UStringSearch, attribute: USearchAttribute, value: USearchAttributeValue, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn usearch_setAttribute(strsrch : *mut UStringSearch, attribute : USearchAttribute, value : USearchAttributeValue, status : *mut UErrorCode));
    unsafe { usearch_setAttribute(strsrch as _, attribute, value, status as _) }
}
#[inline]
pub unsafe fn usearch_setBreakIterator(strsrch: *mut UStringSearch, breakiter: *mut UBreakIterator, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn usearch_setBreakIterator(strsrch : *mut UStringSearch, breakiter : *mut UBreakIterator, status : *mut UErrorCode));
    unsafe { usearch_setBreakIterator(strsrch as _, breakiter as _, status as _) }
}
#[inline]
pub unsafe fn usearch_setCollator(strsrch: *mut UStringSearch, collator: *const UCollator, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn usearch_setCollator(strsrch : *mut UStringSearch, collator : *const UCollator, status : *mut UErrorCode));
    unsafe { usearch_setCollator(strsrch as _, collator, status as _) }
}
#[inline]
pub unsafe fn usearch_setOffset(strsrch: *mut UStringSearch, position: i32, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn usearch_setOffset(strsrch : *mut UStringSearch, position : i32, status : *mut UErrorCode));
    unsafe { usearch_setOffset(strsrch as _, position, status as _) }
}
#[inline]
pub unsafe fn usearch_setPattern(strsrch: *mut UStringSearch, pattern: *const u16, patternlength: i32, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn usearch_setPattern(strsrch : *mut UStringSearch, pattern : *const u16, patternlength : i32, status : *mut UErrorCode));
    unsafe { usearch_setPattern(strsrch as _, pattern, patternlength, status as _) }
}
#[inline]
pub unsafe fn usearch_setText(strsrch: *mut UStringSearch, text: *const u16, textlength: i32, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn usearch_setText(strsrch : *mut UStringSearch, text : *const u16, textlength : i32, status : *mut UErrorCode));
    unsafe { usearch_setText(strsrch as _, text, textlength, status as _) }
}
#[inline]
pub unsafe fn uset_add(set: *mut USet, c: i32) {
    windows_core::link!("icuuc.dll" "C" fn uset_add(set : *mut USet, c : i32));
    unsafe { uset_add(set as _, c) }
}
#[inline]
pub unsafe fn uset_addAll(set: *mut USet, additionalset: *const USet) {
    windows_core::link!("icuuc.dll" "C" fn uset_addAll(set : *mut USet, additionalset : *const USet));
    unsafe { uset_addAll(set as _, additionalset) }
}
#[inline]
pub unsafe fn uset_addAllCodePoints(set: *mut USet, str: *const u16, strlen: i32) {
    windows_core::link!("icuuc.dll" "C" fn uset_addAllCodePoints(set : *mut USet, str : *const u16, strlen : i32));
    unsafe { uset_addAllCodePoints(set as _, str, strlen) }
}
#[inline]
pub unsafe fn uset_addRange(set: *mut USet, start: i32, end: i32) {
    windows_core::link!("icuuc.dll" "C" fn uset_addRange(set : *mut USet, start : i32, end : i32));
    unsafe { uset_addRange(set as _, start, end) }
}
#[inline]
pub unsafe fn uset_addString(set: *mut USet, str: *const u16, strlen: i32) {
    windows_core::link!("icuuc.dll" "C" fn uset_addString(set : *mut USet, str : *const u16, strlen : i32));
    unsafe { uset_addString(set as _, str, strlen) }
}
#[inline]
pub unsafe fn uset_applyIntPropertyValue(set: *mut USet, prop: UProperty, value: i32, ec: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn uset_applyIntPropertyValue(set : *mut USet, prop : UProperty, value : i32, ec : *mut UErrorCode));
    unsafe { uset_applyIntPropertyValue(set as _, prop, value, ec as _) }
}
#[inline]
pub unsafe fn uset_applyPattern(set: *mut USet, pattern: *const u16, patternlength: i32, options: u32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uset_applyPattern(set : *mut USet, pattern : *const u16, patternlength : i32, options : u32, status : *mut UErrorCode) -> i32);
    unsafe { uset_applyPattern(set as _, pattern, patternlength, options, status as _) }
}
#[inline]
pub unsafe fn uset_applyPropertyAlias(set: *mut USet, prop: *const u16, proplength: i32, value: *const u16, valuelength: i32, ec: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn uset_applyPropertyAlias(set : *mut USet, prop : *const u16, proplength : i32, value : *const u16, valuelength : i32, ec : *mut UErrorCode));
    unsafe { uset_applyPropertyAlias(set as _, prop, proplength, value, valuelength, ec as _) }
}
#[inline]
pub unsafe fn uset_charAt(set: *const USet, charindex: i32) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uset_charAt(set : *const USet, charindex : i32) -> i32);
    unsafe { uset_charAt(set, charindex) }
}
#[inline]
pub unsafe fn uset_clear(set: *mut USet) {
    windows_core::link!("icuuc.dll" "C" fn uset_clear(set : *mut USet));
    unsafe { uset_clear(set as _) }
}
#[inline]
pub unsafe fn uset_clone(set: *const USet) -> *mut USet {
    windows_core::link!("icuuc.dll" "C" fn uset_clone(set : *const USet) -> *mut USet);
    unsafe { uset_clone(set) }
}
#[inline]
pub unsafe fn uset_cloneAsThawed(set: *const USet) -> *mut USet {
    windows_core::link!("icuuc.dll" "C" fn uset_cloneAsThawed(set : *const USet) -> *mut USet);
    unsafe { uset_cloneAsThawed(set) }
}
#[inline]
pub unsafe fn uset_close(set: *mut USet) {
    windows_core::link!("icuuc.dll" "C" fn uset_close(set : *mut USet));
    unsafe { uset_close(set as _) }
}
#[inline]
pub unsafe fn uset_closeOver(set: *mut USet, attributes: i32) {
    windows_core::link!("icuuc.dll" "C" fn uset_closeOver(set : *mut USet, attributes : i32));
    unsafe { uset_closeOver(set as _, attributes) }
}
#[inline]
pub unsafe fn uset_compact(set: *mut USet) {
    windows_core::link!("icuuc.dll" "C" fn uset_compact(set : *mut USet));
    unsafe { uset_compact(set as _) }
}
#[inline]
pub unsafe fn uset_complement(set: *mut USet) {
    windows_core::link!("icuuc.dll" "C" fn uset_complement(set : *mut USet));
    unsafe { uset_complement(set as _) }
}
#[inline]
pub unsafe fn uset_complementAll(set: *mut USet, complement: *const USet) {
    windows_core::link!("icuuc.dll" "C" fn uset_complementAll(set : *mut USet, complement : *const USet));
    unsafe { uset_complementAll(set as _, complement) }
}
#[inline]
pub unsafe fn uset_contains(set: *const USet, c: i32) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn uset_contains(set : *const USet, c : i32) -> i8);
    unsafe { uset_contains(set, c) }
}
#[inline]
pub unsafe fn uset_containsAll(set1: *const USet, set2: *const USet) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn uset_containsAll(set1 : *const USet, set2 : *const USet) -> i8);
    unsafe { uset_containsAll(set1, set2) }
}
#[inline]
pub unsafe fn uset_containsAllCodePoints(set: *const USet, str: *const u16, strlen: i32) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn uset_containsAllCodePoints(set : *const USet, str : *const u16, strlen : i32) -> i8);
    unsafe { uset_containsAllCodePoints(set, str, strlen) }
}
#[inline]
pub unsafe fn uset_containsNone(set1: *const USet, set2: *const USet) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn uset_containsNone(set1 : *const USet, set2 : *const USet) -> i8);
    unsafe { uset_containsNone(set1, set2) }
}
#[inline]
pub unsafe fn uset_containsRange(set: *const USet, start: i32, end: i32) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn uset_containsRange(set : *const USet, start : i32, end : i32) -> i8);
    unsafe { uset_containsRange(set, start, end) }
}
#[inline]
pub unsafe fn uset_containsSome(set1: *const USet, set2: *const USet) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn uset_containsSome(set1 : *const USet, set2 : *const USet) -> i8);
    unsafe { uset_containsSome(set1, set2) }
}
#[inline]
pub unsafe fn uset_containsString(set: *const USet, str: *const u16, strlen: i32) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn uset_containsString(set : *const USet, str : *const u16, strlen : i32) -> i8);
    unsafe { uset_containsString(set, str, strlen) }
}
#[inline]
pub unsafe fn uset_equals(set1: *const USet, set2: *const USet) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn uset_equals(set1 : *const USet, set2 : *const USet) -> i8);
    unsafe { uset_equals(set1, set2) }
}
#[inline]
pub unsafe fn uset_freeze(set: *mut USet) {
    windows_core::link!("icuuc.dll" "C" fn uset_freeze(set : *mut USet));
    unsafe { uset_freeze(set as _) }
}
#[inline]
pub unsafe fn uset_getItem(set: *const USet, itemindex: i32, start: *mut i32, end: *mut i32, str: *mut u16, strcapacity: i32, ec: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uset_getItem(set : *const USet, itemindex : i32, start : *mut i32, end : *mut i32, str : *mut u16, strcapacity : i32, ec : *mut UErrorCode) -> i32);
    unsafe { uset_getItem(set, itemindex, start as _, end as _, str as _, strcapacity, ec as _) }
}
#[inline]
pub unsafe fn uset_getItemCount(set: *const USet) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uset_getItemCount(set : *const USet) -> i32);
    unsafe { uset_getItemCount(set) }
}
#[inline]
pub unsafe fn uset_getSerializedRange(set: *const USerializedSet, rangeindex: i32, pstart: *mut i32, pend: *mut i32) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn uset_getSerializedRange(set : *const USerializedSet, rangeindex : i32, pstart : *mut i32, pend : *mut i32) -> i8);
    unsafe { uset_getSerializedRange(set, rangeindex, pstart as _, pend as _) }
}
#[inline]
pub unsafe fn uset_getSerializedRangeCount(set: *const USerializedSet) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uset_getSerializedRangeCount(set : *const USerializedSet) -> i32);
    unsafe { uset_getSerializedRangeCount(set) }
}
#[inline]
pub unsafe fn uset_getSerializedSet(fillset: *mut USerializedSet, src: *const u16, srclength: i32) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn uset_getSerializedSet(fillset : *mut USerializedSet, src : *const u16, srclength : i32) -> i8);
    unsafe { uset_getSerializedSet(fillset as _, src, srclength) }
}
#[inline]
pub unsafe fn uset_indexOf(set: *const USet, c: i32) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uset_indexOf(set : *const USet, c : i32) -> i32);
    unsafe { uset_indexOf(set, c) }
}
#[inline]
pub unsafe fn uset_isEmpty(set: *const USet) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn uset_isEmpty(set : *const USet) -> i8);
    unsafe { uset_isEmpty(set) }
}
#[inline]
pub unsafe fn uset_isFrozen(set: *const USet) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn uset_isFrozen(set : *const USet) -> i8);
    unsafe { uset_isFrozen(set) }
}
#[inline]
pub unsafe fn uset_open(start: i32, end: i32) -> *mut USet {
    windows_core::link!("icuuc.dll" "C" fn uset_open(start : i32, end : i32) -> *mut USet);
    unsafe { uset_open(start, end) }
}
#[inline]
pub unsafe fn uset_openEmpty() -> *mut USet {
    windows_core::link!("icuuc.dll" "C" fn uset_openEmpty() -> *mut USet);
    unsafe { uset_openEmpty() }
}
#[inline]
pub unsafe fn uset_openPattern(pattern: *const u16, patternlength: i32, ec: *mut UErrorCode) -> *mut USet {
    windows_core::link!("icuuc.dll" "C" fn uset_openPattern(pattern : *const u16, patternlength : i32, ec : *mut UErrorCode) -> *mut USet);
    unsafe { uset_openPattern(pattern, patternlength, ec as _) }
}
#[inline]
pub unsafe fn uset_openPatternOptions(pattern: *const u16, patternlength: i32, options: u32, ec: *mut UErrorCode) -> *mut USet {
    windows_core::link!("icuuc.dll" "C" fn uset_openPatternOptions(pattern : *const u16, patternlength : i32, options : u32, ec : *mut UErrorCode) -> *mut USet);
    unsafe { uset_openPatternOptions(pattern, patternlength, options, ec as _) }
}
#[inline]
pub unsafe fn uset_remove(set: *mut USet, c: i32) {
    windows_core::link!("icuuc.dll" "C" fn uset_remove(set : *mut USet, c : i32));
    unsafe { uset_remove(set as _, c) }
}
#[inline]
pub unsafe fn uset_removeAll(set: *mut USet, removeset: *const USet) {
    windows_core::link!("icuuc.dll" "C" fn uset_removeAll(set : *mut USet, removeset : *const USet));
    unsafe { uset_removeAll(set as _, removeset) }
}
#[inline]
pub unsafe fn uset_removeAllStrings(set: *mut USet) {
    windows_core::link!("icuuc.dll" "C" fn uset_removeAllStrings(set : *mut USet));
    unsafe { uset_removeAllStrings(set as _) }
}
#[inline]
pub unsafe fn uset_removeRange(set: *mut USet, start: i32, end: i32) {
    windows_core::link!("icuuc.dll" "C" fn uset_removeRange(set : *mut USet, start : i32, end : i32));
    unsafe { uset_removeRange(set as _, start, end) }
}
#[inline]
pub unsafe fn uset_removeString(set: *mut USet, str: *const u16, strlen: i32) {
    windows_core::link!("icuuc.dll" "C" fn uset_removeString(set : *mut USet, str : *const u16, strlen : i32));
    unsafe { uset_removeString(set as _, str, strlen) }
}
#[inline]
pub unsafe fn uset_resemblesPattern(pattern: *const u16, patternlength: i32, pos: i32) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn uset_resemblesPattern(pattern : *const u16, patternlength : i32, pos : i32) -> i8);
    unsafe { uset_resemblesPattern(pattern, patternlength, pos) }
}
#[inline]
pub unsafe fn uset_retain(set: *mut USet, start: i32, end: i32) {
    windows_core::link!("icuuc.dll" "C" fn uset_retain(set : *mut USet, start : i32, end : i32));
    unsafe { uset_retain(set as _, start, end) }
}
#[inline]
pub unsafe fn uset_retainAll(set: *mut USet, retain: *const USet) {
    windows_core::link!("icuuc.dll" "C" fn uset_retainAll(set : *mut USet, retain : *const USet));
    unsafe { uset_retainAll(set as _, retain) }
}
#[inline]
pub unsafe fn uset_serialize(set: *const USet, dest: *mut u16, destcapacity: i32, perrorcode: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uset_serialize(set : *const USet, dest : *mut u16, destcapacity : i32, perrorcode : *mut UErrorCode) -> i32);
    unsafe { uset_serialize(set, dest as _, destcapacity, perrorcode as _) }
}
#[inline]
pub unsafe fn uset_serializedContains(set: *const USerializedSet, c: i32) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn uset_serializedContains(set : *const USerializedSet, c : i32) -> i8);
    unsafe { uset_serializedContains(set, c) }
}
#[inline]
pub unsafe fn uset_set(set: *mut USet, start: i32, end: i32) {
    windows_core::link!("icuuc.dll" "C" fn uset_set(set : *mut USet, start : i32, end : i32));
    unsafe { uset_set(set as _, start, end) }
}
#[inline]
pub unsafe fn uset_setSerializedToOne(fillset: *mut USerializedSet, c: i32) {
    windows_core::link!("icuuc.dll" "C" fn uset_setSerializedToOne(fillset : *mut USerializedSet, c : i32));
    unsafe { uset_setSerializedToOne(fillset as _, c) }
}
#[inline]
pub unsafe fn uset_size(set: *const USet) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uset_size(set : *const USet) -> i32);
    unsafe { uset_size(set) }
}
#[inline]
pub unsafe fn uset_span(set: *const USet, s: *const u16, length: i32, spancondition: USetSpanCondition) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uset_span(set : *const USet, s : *const u16, length : i32, spancondition : USetSpanCondition) -> i32);
    unsafe { uset_span(set, s, length, spancondition) }
}
#[inline]
pub unsafe fn uset_spanBack(set: *const USet, s: *const u16, length: i32, spancondition: USetSpanCondition) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uset_spanBack(set : *const USet, s : *const u16, length : i32, spancondition : USetSpanCondition) -> i32);
    unsafe { uset_spanBack(set, s, length, spancondition) }
}
#[inline]
pub unsafe fn uset_spanBackUTF8<P1>(set: *const USet, s: P1, length: i32, spancondition: USetSpanCondition) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uset_spanBackUTF8(set : *const USet, s : windows_core::PCSTR, length : i32, spancondition : USetSpanCondition) -> i32);
    unsafe { uset_spanBackUTF8(set, s.param().abi(), length, spancondition) }
}
#[inline]
pub unsafe fn uset_spanUTF8<P1>(set: *const USet, s: P1, length: i32, spancondition: USetSpanCondition) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn uset_spanUTF8(set : *const USet, s : windows_core::PCSTR, length : i32, spancondition : USetSpanCondition) -> i32);
    unsafe { uset_spanUTF8(set, s.param().abi(), length, spancondition) }
}
#[inline]
pub unsafe fn uset_toPattern(set: *const USet, result: *mut u16, resultcapacity: i32, escapeunprintable: i8, ec: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn uset_toPattern(set : *const USet, result : *mut u16, resultcapacity : i32, escapeunprintable : i8, ec : *mut UErrorCode) -> i32);
    unsafe { uset_toPattern(set, result as _, resultcapacity, escapeunprintable, ec as _) }
}
#[inline]
pub unsafe fn uspoof_areConfusable(sc: *const USpoofChecker, id1: *const u16, length1: i32, id2: *const u16, length2: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uspoof_areConfusable(sc : *const USpoofChecker, id1 : *const u16, length1 : i32, id2 : *const u16, length2 : i32, status : *mut UErrorCode) -> i32);
    unsafe { uspoof_areConfusable(sc, id1, length1, id2, length2, status as _) }
}
#[inline]
pub unsafe fn uspoof_areConfusableUTF8<P1, P3>(sc: *const USpoofChecker, id1: P1, length1: i32, id2: P3, length2: i32, status: *mut UErrorCode) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuin.dll" "C" fn uspoof_areConfusableUTF8(sc : *const USpoofChecker, id1 : windows_core::PCSTR, length1 : i32, id2 : windows_core::PCSTR, length2 : i32, status : *mut UErrorCode) -> i32);
    unsafe { uspoof_areConfusableUTF8(sc, id1.param().abi(), length1, id2.param().abi(), length2, status as _) }
}
#[inline]
pub unsafe fn uspoof_check(sc: *const USpoofChecker, id: *const u16, length: i32, position: *mut i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uspoof_check(sc : *const USpoofChecker, id : *const u16, length : i32, position : *mut i32, status : *mut UErrorCode) -> i32);
    unsafe { uspoof_check(sc, id, length, position as _, status as _) }
}
#[inline]
pub unsafe fn uspoof_check2(sc: *const USpoofChecker, id: *const u16, length: i32, checkresult: *mut USpoofCheckResult, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uspoof_check2(sc : *const USpoofChecker, id : *const u16, length : i32, checkresult : *mut USpoofCheckResult, status : *mut UErrorCode) -> i32);
    unsafe { uspoof_check2(sc, id, length, checkresult as _, status as _) }
}
#[inline]
pub unsafe fn uspoof_check2UTF8<P1>(sc: *const USpoofChecker, id: P1, length: i32, checkresult: *mut USpoofCheckResult, status: *mut UErrorCode) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuin.dll" "C" fn uspoof_check2UTF8(sc : *const USpoofChecker, id : windows_core::PCSTR, length : i32, checkresult : *mut USpoofCheckResult, status : *mut UErrorCode) -> i32);
    unsafe { uspoof_check2UTF8(sc, id.param().abi(), length, checkresult as _, status as _) }
}
#[inline]
pub unsafe fn uspoof_checkUTF8<P1>(sc: *const USpoofChecker, id: P1, length: i32, position: *mut i32, status: *mut UErrorCode) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuin.dll" "C" fn uspoof_checkUTF8(sc : *const USpoofChecker, id : windows_core::PCSTR, length : i32, position : *mut i32, status : *mut UErrorCode) -> i32);
    unsafe { uspoof_checkUTF8(sc, id.param().abi(), length, position as _, status as _) }
}
#[inline]
pub unsafe fn uspoof_clone(sc: *const USpoofChecker, status: *mut UErrorCode) -> *mut USpoofChecker {
    windows_core::link!("icuin.dll" "C" fn uspoof_clone(sc : *const USpoofChecker, status : *mut UErrorCode) -> *mut USpoofChecker);
    unsafe { uspoof_clone(sc, status as _) }
}
#[inline]
pub unsafe fn uspoof_close(sc: *mut USpoofChecker) {
    windows_core::link!("icuin.dll" "C" fn uspoof_close(sc : *mut USpoofChecker));
    unsafe { uspoof_close(sc as _) }
}
#[inline]
pub unsafe fn uspoof_closeCheckResult(checkresult: *mut USpoofCheckResult) {
    windows_core::link!("icuin.dll" "C" fn uspoof_closeCheckResult(checkresult : *mut USpoofCheckResult));
    unsafe { uspoof_closeCheckResult(checkresult as _) }
}
#[inline]
pub unsafe fn uspoof_getAllowedChars(sc: *const USpoofChecker, status: *mut UErrorCode) -> *mut USet {
    windows_core::link!("icuin.dll" "C" fn uspoof_getAllowedChars(sc : *const USpoofChecker, status : *mut UErrorCode) -> *mut USet);
    unsafe { uspoof_getAllowedChars(sc, status as _) }
}
#[inline]
pub unsafe fn uspoof_getAllowedLocales(sc: *mut USpoofChecker, status: *mut UErrorCode) -> windows_core::PCSTR {
    windows_core::link!("icuin.dll" "C" fn uspoof_getAllowedLocales(sc : *mut USpoofChecker, status : *mut UErrorCode) -> windows_core::PCSTR);
    unsafe { uspoof_getAllowedLocales(sc as _, status as _) }
}
#[inline]
pub unsafe fn uspoof_getCheckResultChecks(checkresult: *const USpoofCheckResult, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uspoof_getCheckResultChecks(checkresult : *const USpoofCheckResult, status : *mut UErrorCode) -> i32);
    unsafe { uspoof_getCheckResultChecks(checkresult, status as _) }
}
#[inline]
pub unsafe fn uspoof_getCheckResultNumerics(checkresult: *const USpoofCheckResult, status: *mut UErrorCode) -> *mut USet {
    windows_core::link!("icuin.dll" "C" fn uspoof_getCheckResultNumerics(checkresult : *const USpoofCheckResult, status : *mut UErrorCode) -> *mut USet);
    unsafe { uspoof_getCheckResultNumerics(checkresult, status as _) }
}
#[inline]
pub unsafe fn uspoof_getCheckResultRestrictionLevel(checkresult: *const USpoofCheckResult, status: *mut UErrorCode) -> URestrictionLevel {
    windows_core::link!("icuin.dll" "C" fn uspoof_getCheckResultRestrictionLevel(checkresult : *const USpoofCheckResult, status : *mut UErrorCode) -> URestrictionLevel);
    unsafe { uspoof_getCheckResultRestrictionLevel(checkresult, status as _) }
}
#[inline]
pub unsafe fn uspoof_getChecks(sc: *const USpoofChecker, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uspoof_getChecks(sc : *const USpoofChecker, status : *mut UErrorCode) -> i32);
    unsafe { uspoof_getChecks(sc, status as _) }
}
#[inline]
pub unsafe fn uspoof_getInclusionSet(status: *mut UErrorCode) -> *mut USet {
    windows_core::link!("icuin.dll" "C" fn uspoof_getInclusionSet(status : *mut UErrorCode) -> *mut USet);
    unsafe { uspoof_getInclusionSet(status as _) }
}
#[inline]
pub unsafe fn uspoof_getRecommendedSet(status: *mut UErrorCode) -> *mut USet {
    windows_core::link!("icuin.dll" "C" fn uspoof_getRecommendedSet(status : *mut UErrorCode) -> *mut USet);
    unsafe { uspoof_getRecommendedSet(status as _) }
}
#[inline]
pub unsafe fn uspoof_getRestrictionLevel(sc: *const USpoofChecker) -> URestrictionLevel {
    windows_core::link!("icuin.dll" "C" fn uspoof_getRestrictionLevel(sc : *const USpoofChecker) -> URestrictionLevel);
    unsafe { uspoof_getRestrictionLevel(sc) }
}
#[inline]
pub unsafe fn uspoof_getSkeleton(sc: *const USpoofChecker, r#type: u32, id: *const u16, length: i32, dest: *mut u16, destcapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uspoof_getSkeleton(sc : *const USpoofChecker, r#type : u32, id : *const u16, length : i32, dest : *mut u16, destcapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { uspoof_getSkeleton(sc, r#type, id, length, dest as _, destcapacity, status as _) }
}
#[inline]
pub unsafe fn uspoof_getSkeletonUTF8<P2, P4>(sc: *const USpoofChecker, r#type: u32, id: P2, length: i32, dest: P4, destcapacity: i32, status: *mut UErrorCode) -> i32
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuin.dll" "C" fn uspoof_getSkeletonUTF8(sc : *const USpoofChecker, r#type : u32, id : windows_core::PCSTR, length : i32, dest : windows_core::PCSTR, destcapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { uspoof_getSkeletonUTF8(sc, r#type, id.param().abi(), length, dest.param().abi(), destcapacity, status as _) }
}
#[inline]
pub unsafe fn uspoof_open(status: *mut UErrorCode) -> *mut USpoofChecker {
    windows_core::link!("icuin.dll" "C" fn uspoof_open(status : *mut UErrorCode) -> *mut USpoofChecker);
    unsafe { uspoof_open(status as _) }
}
#[inline]
pub unsafe fn uspoof_openCheckResult(status: *mut UErrorCode) -> *mut USpoofCheckResult {
    windows_core::link!("icuin.dll" "C" fn uspoof_openCheckResult(status : *mut UErrorCode) -> *mut USpoofCheckResult);
    unsafe { uspoof_openCheckResult(status as _) }
}
#[inline]
pub unsafe fn uspoof_openFromSerialized(data: *const core::ffi::c_void, length: i32, pactuallength: *mut i32, perrorcode: *mut UErrorCode) -> *mut USpoofChecker {
    windows_core::link!("icuin.dll" "C" fn uspoof_openFromSerialized(data : *const core::ffi::c_void, length : i32, pactuallength : *mut i32, perrorcode : *mut UErrorCode) -> *mut USpoofChecker);
    unsafe { uspoof_openFromSerialized(data, length, pactuallength as _, perrorcode as _) }
}
#[inline]
pub unsafe fn uspoof_openFromSource<P0, P2>(confusables: P0, confusableslen: i32, confusableswholescript: P2, confusableswholescriptlen: i32, errtype: *mut i32, pe: *mut UParseError, status: *mut UErrorCode) -> *mut USpoofChecker
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuin.dll" "C" fn uspoof_openFromSource(confusables : windows_core::PCSTR, confusableslen : i32, confusableswholescript : windows_core::PCSTR, confusableswholescriptlen : i32, errtype : *mut i32, pe : *mut UParseError, status : *mut UErrorCode) -> *mut USpoofChecker);
    unsafe { uspoof_openFromSource(confusables.param().abi(), confusableslen, confusableswholescript.param().abi(), confusableswholescriptlen, errtype as _, pe as _, status as _) }
}
#[inline]
pub unsafe fn uspoof_serialize(sc: *mut USpoofChecker, data: *mut core::ffi::c_void, capacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn uspoof_serialize(sc : *mut USpoofChecker, data : *mut core::ffi::c_void, capacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { uspoof_serialize(sc as _, data as _, capacity, status as _) }
}
#[inline]
pub unsafe fn uspoof_setAllowedChars(sc: *mut USpoofChecker, chars: *const USet, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn uspoof_setAllowedChars(sc : *mut USpoofChecker, chars : *const USet, status : *mut UErrorCode));
    unsafe { uspoof_setAllowedChars(sc as _, chars, status as _) }
}
#[inline]
pub unsafe fn uspoof_setAllowedLocales<P1>(sc: *mut USpoofChecker, localeslist: P1, status: *mut UErrorCode)
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuin.dll" "C" fn uspoof_setAllowedLocales(sc : *mut USpoofChecker, localeslist : windows_core::PCSTR, status : *mut UErrorCode));
    unsafe { uspoof_setAllowedLocales(sc as _, localeslist.param().abi(), status as _) }
}
#[inline]
pub unsafe fn uspoof_setChecks(sc: *mut USpoofChecker, checks: i32, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn uspoof_setChecks(sc : *mut USpoofChecker, checks : i32, status : *mut UErrorCode));
    unsafe { uspoof_setChecks(sc as _, checks, status as _) }
}
#[inline]
pub unsafe fn uspoof_setRestrictionLevel(sc: *mut USpoofChecker, restrictionlevel: URestrictionLevel) {
    windows_core::link!("icuin.dll" "C" fn uspoof_setRestrictionLevel(sc : *mut USpoofChecker, restrictionlevel : URestrictionLevel));
    unsafe { uspoof_setRestrictionLevel(sc as _, restrictionlevel) }
}
#[inline]
pub unsafe fn usprep_close(profile: *mut UStringPrepProfile) {
    windows_core::link!("icuuc.dll" "C" fn usprep_close(profile : *mut UStringPrepProfile));
    unsafe { usprep_close(profile as _) }
}
#[inline]
pub unsafe fn usprep_open<P0, P1>(path: P0, filename: P1, status: *mut UErrorCode) -> *mut UStringPrepProfile
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn usprep_open(path : windows_core::PCSTR, filename : windows_core::PCSTR, status : *mut UErrorCode) -> *mut UStringPrepProfile);
    unsafe { usprep_open(path.param().abi(), filename.param().abi(), status as _) }
}
#[inline]
pub unsafe fn usprep_openByType(r#type: UStringPrepProfileType, status: *mut UErrorCode) -> *mut UStringPrepProfile {
    windows_core::link!("icuuc.dll" "C" fn usprep_openByType(r#type : UStringPrepProfileType, status : *mut UErrorCode) -> *mut UStringPrepProfile);
    unsafe { usprep_openByType(r#type, status as _) }
}
#[inline]
pub unsafe fn usprep_prepare(prep: *const UStringPrepProfile, src: *const u16, srclength: i32, dest: *mut u16, destcapacity: i32, options: i32, parseerror: *mut UParseError, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn usprep_prepare(prep : *const UStringPrepProfile, src : *const u16, srclength : i32, dest : *mut u16, destcapacity : i32, options : i32, parseerror : *mut UParseError, status : *mut UErrorCode) -> i32);
    unsafe { usprep_prepare(prep, src, srclength, dest as _, destcapacity, options, parseerror as _, status as _) }
}
#[inline]
pub unsafe fn utext_char32At(ut: *mut UText, nativeindex: i64) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn utext_char32At(ut : *mut UText, nativeindex : i64) -> i32);
    unsafe { utext_char32At(ut as _, nativeindex) }
}
#[inline]
pub unsafe fn utext_clone(dest: *mut UText, src: *const UText, deep: i8, readonly: i8, status: *mut UErrorCode) -> *mut UText {
    windows_core::link!("icuuc.dll" "C" fn utext_clone(dest : *mut UText, src : *const UText, deep : i8, readonly : i8, status : *mut UErrorCode) -> *mut UText);
    unsafe { utext_clone(dest as _, src, deep, readonly, status as _) }
}
#[inline]
pub unsafe fn utext_close(ut: *mut UText) -> *mut UText {
    windows_core::link!("icuuc.dll" "C" fn utext_close(ut : *mut UText) -> *mut UText);
    unsafe { utext_close(ut as _) }
}
#[inline]
pub unsafe fn utext_copy(ut: *mut UText, nativestart: i64, nativelimit: i64, destindex: i64, r#move: i8, status: *mut UErrorCode) {
    windows_core::link!("icuuc.dll" "C" fn utext_copy(ut : *mut UText, nativestart : i64, nativelimit : i64, destindex : i64, r#move : i8, status : *mut UErrorCode));
    unsafe { utext_copy(ut as _, nativestart, nativelimit, destindex, r#move, status as _) }
}
#[inline]
pub unsafe fn utext_current32(ut: *mut UText) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn utext_current32(ut : *mut UText) -> i32);
    unsafe { utext_current32(ut as _) }
}
#[inline]
pub unsafe fn utext_equals(a: *const UText, b: *const UText) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn utext_equals(a : *const UText, b : *const UText) -> i8);
    unsafe { utext_equals(a, b) }
}
#[inline]
pub unsafe fn utext_extract(ut: *mut UText, nativestart: i64, nativelimit: i64, dest: *mut u16, destcapacity: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn utext_extract(ut : *mut UText, nativestart : i64, nativelimit : i64, dest : *mut u16, destcapacity : i32, status : *mut UErrorCode) -> i32);
    unsafe { utext_extract(ut as _, nativestart, nativelimit, dest as _, destcapacity, status as _) }
}
#[inline]
pub unsafe fn utext_freeze(ut: *mut UText) {
    windows_core::link!("icuuc.dll" "C" fn utext_freeze(ut : *mut UText));
    unsafe { utext_freeze(ut as _) }
}
#[inline]
pub unsafe fn utext_getNativeIndex(ut: *const UText) -> i64 {
    windows_core::link!("icuuc.dll" "C" fn utext_getNativeIndex(ut : *const UText) -> i64);
    unsafe { utext_getNativeIndex(ut) }
}
#[inline]
pub unsafe fn utext_getPreviousNativeIndex(ut: *mut UText) -> i64 {
    windows_core::link!("icuuc.dll" "C" fn utext_getPreviousNativeIndex(ut : *mut UText) -> i64);
    unsafe { utext_getPreviousNativeIndex(ut as _) }
}
#[inline]
pub unsafe fn utext_hasMetaData(ut: *const UText) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn utext_hasMetaData(ut : *const UText) -> i8);
    unsafe { utext_hasMetaData(ut) }
}
#[inline]
pub unsafe fn utext_isLengthExpensive(ut: *const UText) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn utext_isLengthExpensive(ut : *const UText) -> i8);
    unsafe { utext_isLengthExpensive(ut) }
}
#[inline]
pub unsafe fn utext_isWritable(ut: *const UText) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn utext_isWritable(ut : *const UText) -> i8);
    unsafe { utext_isWritable(ut) }
}
#[inline]
pub unsafe fn utext_moveIndex32(ut: *mut UText, delta: i32) -> i8 {
    windows_core::link!("icuuc.dll" "C" fn utext_moveIndex32(ut : *mut UText, delta : i32) -> i8);
    unsafe { utext_moveIndex32(ut as _, delta) }
}
#[inline]
pub unsafe fn utext_nativeLength(ut: *mut UText) -> i64 {
    windows_core::link!("icuuc.dll" "C" fn utext_nativeLength(ut : *mut UText) -> i64);
    unsafe { utext_nativeLength(ut as _) }
}
#[inline]
pub unsafe fn utext_next32(ut: *mut UText) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn utext_next32(ut : *mut UText) -> i32);
    unsafe { utext_next32(ut as _) }
}
#[inline]
pub unsafe fn utext_next32From(ut: *mut UText, nativeindex: i64) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn utext_next32From(ut : *mut UText, nativeindex : i64) -> i32);
    unsafe { utext_next32From(ut as _, nativeindex) }
}
#[inline]
pub unsafe fn utext_openUChars(ut: *mut UText, s: *const u16, length: i64, status: *mut UErrorCode) -> *mut UText {
    windows_core::link!("icuuc.dll" "C" fn utext_openUChars(ut : *mut UText, s : *const u16, length : i64, status : *mut UErrorCode) -> *mut UText);
    unsafe { utext_openUChars(ut as _, s, length, status as _) }
}
#[inline]
pub unsafe fn utext_openUTF8<P1>(ut: *mut UText, s: P1, length: i64, status: *mut UErrorCode) -> *mut UText
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn utext_openUTF8(ut : *mut UText, s : windows_core::PCSTR, length : i64, status : *mut UErrorCode) -> *mut UText);
    unsafe { utext_openUTF8(ut as _, s.param().abi(), length, status as _) }
}
#[inline]
pub unsafe fn utext_previous32(ut: *mut UText) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn utext_previous32(ut : *mut UText) -> i32);
    unsafe { utext_previous32(ut as _) }
}
#[inline]
pub unsafe fn utext_previous32From(ut: *mut UText, nativeindex: i64) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn utext_previous32From(ut : *mut UText, nativeindex : i64) -> i32);
    unsafe { utext_previous32From(ut as _, nativeindex) }
}
#[inline]
pub unsafe fn utext_replace(ut: *mut UText, nativestart: i64, nativelimit: i64, replacementtext: *const u16, replacementlength: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn utext_replace(ut : *mut UText, nativestart : i64, nativelimit : i64, replacementtext : *const u16, replacementlength : i32, status : *mut UErrorCode) -> i32);
    unsafe { utext_replace(ut as _, nativestart, nativelimit, replacementtext, replacementlength, status as _) }
}
#[inline]
pub unsafe fn utext_setNativeIndex(ut: *mut UText, nativeindex: i64) {
    windows_core::link!("icuuc.dll" "C" fn utext_setNativeIndex(ut : *mut UText, nativeindex : i64));
    unsafe { utext_setNativeIndex(ut as _, nativeindex) }
}
#[inline]
pub unsafe fn utext_setup(ut: *mut UText, extraspace: i32, status: *mut UErrorCode) -> *mut UText {
    windows_core::link!("icuuc.dll" "C" fn utext_setup(ut : *mut UText, extraspace : i32, status : *mut UErrorCode) -> *mut UText);
    unsafe { utext_setup(ut as _, extraspace, status as _) }
}
#[inline]
pub unsafe fn utf8_appendCharSafeBody(s: *mut u8, i: i32, length: i32, c: i32, piserror: *mut i8) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn utf8_appendCharSafeBody(s : *mut u8, i : i32, length : i32, c : i32, piserror : *mut i8) -> i32);
    unsafe { utf8_appendCharSafeBody(s as _, i, length, c, piserror as _) }
}
#[inline]
pub unsafe fn utf8_back1SafeBody(s: *const u8, start: i32, i: i32) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn utf8_back1SafeBody(s : *const u8, start : i32, i : i32) -> i32);
    unsafe { utf8_back1SafeBody(s, start, i) }
}
#[inline]
pub unsafe fn utf8_nextCharSafeBody(s: *const u8, pi: *mut i32, length: i32, c: i32, strict: i8) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn utf8_nextCharSafeBody(s : *const u8, pi : *mut i32, length : i32, c : i32, strict : i8) -> i32);
    unsafe { utf8_nextCharSafeBody(s, pi as _, length, c, strict) }
}
#[inline]
pub unsafe fn utf8_prevCharSafeBody(s: *const u8, start: i32, pi: *mut i32, c: i32, strict: i8) -> i32 {
    windows_core::link!("icuuc.dll" "C" fn utf8_prevCharSafeBody(s : *const u8, start : i32, pi : *mut i32, c : i32, strict : i8) -> i32);
    unsafe { utf8_prevCharSafeBody(s, start, pi as _, c, strict) }
}
#[inline]
pub unsafe fn utmscale_fromInt64(othertime: i64, timescale: UDateTimeScale, status: *mut UErrorCode) -> i64 {
    windows_core::link!("icuin.dll" "C" fn utmscale_fromInt64(othertime : i64, timescale : UDateTimeScale, status : *mut UErrorCode) -> i64);
    unsafe { utmscale_fromInt64(othertime, timescale, status as _) }
}
#[inline]
pub unsafe fn utmscale_getTimeScaleValue(timescale: UDateTimeScale, value: UTimeScaleValue, status: *mut UErrorCode) -> i64 {
    windows_core::link!("icuin.dll" "C" fn utmscale_getTimeScaleValue(timescale : UDateTimeScale, value : UTimeScaleValue, status : *mut UErrorCode) -> i64);
    unsafe { utmscale_getTimeScaleValue(timescale, value, status as _) }
}
#[inline]
pub unsafe fn utmscale_toInt64(universaltime: i64, timescale: UDateTimeScale, status: *mut UErrorCode) -> i64 {
    windows_core::link!("icuin.dll" "C" fn utmscale_toInt64(universaltime : i64, timescale : UDateTimeScale, status : *mut UErrorCode) -> i64);
    unsafe { utmscale_toInt64(universaltime, timescale, status as _) }
}
#[inline]
pub unsafe fn utrace_format<P0, P3>(outbuf: P0, capacity: i32, indent: i32, fmt: P3) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn utrace_format(outbuf : windows_core::PCSTR, capacity : i32, indent : i32, fmt : windows_core::PCSTR) -> i32);
    unsafe { utrace_format(outbuf.param().abi(), capacity, indent, fmt.param().abi()) }
}
#[inline]
pub unsafe fn utrace_functionName(fnnumber: i32) -> windows_core::PCSTR {
    windows_core::link!("icuuc.dll" "C" fn utrace_functionName(fnnumber : i32) -> windows_core::PCSTR);
    unsafe { utrace_functionName(fnnumber) }
}
#[inline]
pub unsafe fn utrace_getFunctions(context: *const *const core::ffi::c_void, e: *mut UTraceEntry, x: *mut UTraceExit, d: *mut UTraceData) {
    windows_core::link!("icuuc.dll" "C" fn utrace_getFunctions(context : *const *const core::ffi::c_void, e : *mut UTraceEntry, x : *mut UTraceExit, d : *mut UTraceData));
    unsafe { utrace_getFunctions(context, e as _, x as _, d as _) }
}
#[inline]
pub unsafe fn utrace_getLevel() -> i32 {
    windows_core::link!("icuuc.dll" "C" fn utrace_getLevel() -> i32);
    unsafe { utrace_getLevel() }
}
#[inline]
pub unsafe fn utrace_setFunctions(context: *const core::ffi::c_void, e: UTraceEntry, x: UTraceExit, d: UTraceData) {
    windows_core::link!("icuuc.dll" "C" fn utrace_setFunctions(context : *const core::ffi::c_void, e : UTraceEntry, x : UTraceExit, d : UTraceData));
    unsafe { utrace_setFunctions(context, e, x, d) }
}
#[inline]
pub unsafe fn utrace_setLevel(tracelevel: i32) {
    windows_core::link!("icuuc.dll" "C" fn utrace_setLevel(tracelevel : i32));
    unsafe { utrace_setLevel(tracelevel) }
}
#[inline]
pub unsafe fn utrace_vformat<P0, P3>(outbuf: P0, capacity: i32, indent: i32, fmt: P3, args: *mut i8) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("icuuc.dll" "C" fn utrace_vformat(outbuf : windows_core::PCSTR, capacity : i32, indent : i32, fmt : windows_core::PCSTR, args : *mut i8) -> i32);
    unsafe { utrace_vformat(outbuf.param().abi(), capacity, indent, fmt.param().abi(), args as _) }
}
#[inline]
pub unsafe fn utrans_clone(trans: *const *const core::ffi::c_void, status: *mut UErrorCode) -> *mut *mut core::ffi::c_void {
    windows_core::link!("icuin.dll" "C" fn utrans_clone(trans : *const *const core::ffi::c_void, status : *mut UErrorCode) -> *mut *mut core::ffi::c_void);
    unsafe { utrans_clone(trans, status as _) }
}
#[inline]
pub unsafe fn utrans_close(trans: *mut *mut core::ffi::c_void) {
    windows_core::link!("icuin.dll" "C" fn utrans_close(trans : *mut *mut core::ffi::c_void));
    unsafe { utrans_close(trans as _) }
}
#[inline]
pub unsafe fn utrans_countAvailableIDs() -> i32 {
    windows_core::link!("icuin.dll" "C" fn utrans_countAvailableIDs() -> i32);
    unsafe { utrans_countAvailableIDs() }
}
#[inline]
pub unsafe fn utrans_getSourceSet(trans: *const *const core::ffi::c_void, ignorefilter: i8, fillin: *mut USet, status: *mut UErrorCode) -> *mut USet {
    windows_core::link!("icuin.dll" "C" fn utrans_getSourceSet(trans : *const *const core::ffi::c_void, ignorefilter : i8, fillin : *mut USet, status : *mut UErrorCode) -> *mut USet);
    unsafe { utrans_getSourceSet(trans, ignorefilter, fillin as _, status as _) }
}
#[inline]
pub unsafe fn utrans_getUnicodeID(trans: *const *const core::ffi::c_void, resultlength: *mut i32) -> *mut u16 {
    windows_core::link!("icuin.dll" "C" fn utrans_getUnicodeID(trans : *const *const core::ffi::c_void, resultlength : *mut i32) -> *mut u16);
    unsafe { utrans_getUnicodeID(trans, resultlength as _) }
}
#[inline]
pub unsafe fn utrans_openIDs(perrorcode: *mut UErrorCode) -> *mut UEnumeration {
    windows_core::link!("icuin.dll" "C" fn utrans_openIDs(perrorcode : *mut UErrorCode) -> *mut UEnumeration);
    unsafe { utrans_openIDs(perrorcode as _) }
}
#[inline]
pub unsafe fn utrans_openInverse(trans: *const *const core::ffi::c_void, status: *mut UErrorCode) -> *mut *mut core::ffi::c_void {
    windows_core::link!("icuin.dll" "C" fn utrans_openInverse(trans : *const *const core::ffi::c_void, status : *mut UErrorCode) -> *mut *mut core::ffi::c_void);
    unsafe { utrans_openInverse(trans, status as _) }
}
#[inline]
pub unsafe fn utrans_openU(id: *const u16, idlength: i32, dir: UTransDirection, rules: *const u16, ruleslength: i32, parseerror: *mut UParseError, perrorcode: *mut UErrorCode) -> *mut *mut core::ffi::c_void {
    windows_core::link!("icuin.dll" "C" fn utrans_openU(id : *const u16, idlength : i32, dir : UTransDirection, rules : *const u16, ruleslength : i32, parseerror : *mut UParseError, perrorcode : *mut UErrorCode) -> *mut *mut core::ffi::c_void);
    unsafe { utrans_openU(id, idlength, dir, rules, ruleslength, parseerror as _, perrorcode as _) }
}
#[inline]
pub unsafe fn utrans_register(adoptedtrans: *mut *mut core::ffi::c_void, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn utrans_register(adoptedtrans : *mut *mut core::ffi::c_void, status : *mut UErrorCode));
    unsafe { utrans_register(adoptedtrans as _, status as _) }
}
#[inline]
pub unsafe fn utrans_setFilter(trans: *mut *mut core::ffi::c_void, filterpattern: *const u16, filterpatternlen: i32, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn utrans_setFilter(trans : *mut *mut core::ffi::c_void, filterpattern : *const u16, filterpatternlen : i32, status : *mut UErrorCode));
    unsafe { utrans_setFilter(trans as _, filterpattern, filterpatternlen, status as _) }
}
#[inline]
pub unsafe fn utrans_toRules(trans: *const *const core::ffi::c_void, escapeunprintable: i8, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32 {
    windows_core::link!("icuin.dll" "C" fn utrans_toRules(trans : *const *const core::ffi::c_void, escapeunprintable : i8, result : *mut u16, resultlength : i32, status : *mut UErrorCode) -> i32);
    unsafe { utrans_toRules(trans, escapeunprintable, result as _, resultlength, status as _) }
}
#[inline]
pub unsafe fn utrans_trans(trans: *const *const core::ffi::c_void, rep: *mut *mut core::ffi::c_void, repfunc: *const UReplaceableCallbacks, start: i32, limit: *mut i32, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn utrans_trans(trans : *const *const core::ffi::c_void, rep : *mut *mut core::ffi::c_void, repfunc : *const UReplaceableCallbacks, start : i32, limit : *mut i32, status : *mut UErrorCode));
    unsafe { utrans_trans(trans, rep as _, repfunc, start, limit as _, status as _) }
}
#[inline]
pub unsafe fn utrans_transIncremental(trans: *const *const core::ffi::c_void, rep: *mut *mut core::ffi::c_void, repfunc: *const UReplaceableCallbacks, pos: *mut UTransPosition, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn utrans_transIncremental(trans : *const *const core::ffi::c_void, rep : *mut *mut core::ffi::c_void, repfunc : *const UReplaceableCallbacks, pos : *mut UTransPosition, status : *mut UErrorCode));
    unsafe { utrans_transIncremental(trans, rep as _, repfunc, pos as _, status as _) }
}
#[inline]
pub unsafe fn utrans_transIncrementalUChars(trans: *const *const core::ffi::c_void, text: *mut u16, textlength: *mut i32, textcapacity: i32, pos: *mut UTransPosition, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn utrans_transIncrementalUChars(trans : *const *const core::ffi::c_void, text : *mut u16, textlength : *mut i32, textcapacity : i32, pos : *mut UTransPosition, status : *mut UErrorCode));
    unsafe { utrans_transIncrementalUChars(trans, text as _, textlength as _, textcapacity, pos as _, status as _) }
}
#[inline]
pub unsafe fn utrans_transUChars(trans: *const *const core::ffi::c_void, text: *mut u16, textlength: *mut i32, textcapacity: i32, start: i32, limit: *mut i32, status: *mut UErrorCode) {
    windows_core::link!("icuin.dll" "C" fn utrans_transUChars(trans : *const *const core::ffi::c_void, text : *mut u16, textlength : *mut i32, textcapacity : i32, start : i32, limit : *mut i32, status : *mut UErrorCode));
    unsafe { utrans_transUChars(trans, text as _, textlength as _, textcapacity, start, limit as _, status as _) }
}
#[inline]
pub unsafe fn utrans_unregisterID(id: *const u16, idlength: i32) {
    windows_core::link!("icuin.dll" "C" fn utrans_unregisterID(id : *const u16, idlength : i32));
    unsafe { utrans_unregisterID(id, idlength) }
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CALDATETIME_DATEUNIT(pub i32);
pub type CALINFO_ENUMPROCA = Option<unsafe extern "system" fn(param0: windows_core::PCSTR) -> windows_core::BOOL>;
pub type CALINFO_ENUMPROCEXA = Option<unsafe extern "system" fn(param0: windows_core::PCSTR, param1: u32) -> windows_core::BOOL>;
pub type CALINFO_ENUMPROCEXEX = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: u32, param2: windows_core::PCWSTR, param3: super::Foundation::LPARAM) -> windows_core::BOOL>;
pub type CALINFO_ENUMPROCEXW = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: u32) -> windows_core::BOOL>;
pub type CALINFO_ENUMPROCW = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR) -> windows_core::BOOL>;
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CHARSETINFO {
    pub ciCharset: u32,
    pub ciACP: u32,
    pub fs: FONTSIGNATURE,
}
pub const CMLangConvertCharset: windows_core::GUID = windows_core::GUID::from_u128(0xd66d6f99_cdaa_11d0_b822_00c04fc9b31f);
pub const CMLangString: windows_core::GUID = windows_core::GUID::from_u128(0xc04d65cf_b70d_11d0_b188_00aa0038c969);
pub const CMultiLanguage: windows_core::GUID = windows_core::GUID::from_u128(0x275c23e2_3747_11d0_9fea_00aa003f8646);
pub type CODEPAGE_ENUMPROCA = Option<unsafe extern "system" fn(param0: windows_core::PCSTR) -> windows_core::BOOL>;
pub type CODEPAGE_ENUMPROCW = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR) -> windows_core::BOOL>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COMPARESTRING_RESULT(pub i32);
pub const COMPARE_STRING: SYSNLS_FUNCTION = SYSNLS_FUNCTION(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COMPARE_STRING_FLAGS(pub u32);
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CORRECTIVE_ACTION(pub i32);
pub const CORRECTIVE_ACTION_DELETE: CORRECTIVE_ACTION = CORRECTIVE_ACTION(3i32);
pub const CORRECTIVE_ACTION_GET_SUGGESTIONS: CORRECTIVE_ACTION = CORRECTIVE_ACTION(1i32);
pub const CORRECTIVE_ACTION_NONE: CORRECTIVE_ACTION = CORRECTIVE_ACTION(0i32);
pub const CORRECTIVE_ACTION_REPLACE: CORRECTIVE_ACTION = CORRECTIVE_ACTION(2i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CPINFO {
    pub MaxCharSize: u32,
    pub DefaultChar: [u8; 2],
    pub LeadByte: [u8; 12],
}
impl Default for CPINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CPINFOEXA {
    pub MaxCharSize: u32,
    pub DefaultChar: [u8; 2],
    pub LeadByte: [u8; 12],
    pub UnicodeDefaultChar: u16,
    pub CodePage: u32,
    pub CodePageName: [i8; 260],
}
impl Default for CPINFOEXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CPINFOEXW {
    pub MaxCharSize: u32,
    pub DefaultChar: [u8; 2],
    pub LeadByte: [u8; 12],
    pub UnicodeDefaultChar: u16,
    pub CodePage: u32,
    pub CodePageName: [u16; 260],
}
impl Default for CPINFOEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
pub type DATEFMT_ENUMPROCA = Option<unsafe extern "system" fn(param0: windows_core::PCSTR) -> windows_core::BOOL>;
pub type DATEFMT_ENUMPROCEXA = Option<unsafe extern "system" fn(param0: windows_core::PCSTR, param1: u32) -> windows_core::BOOL>;
pub type DATEFMT_ENUMPROCEXEX = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: u32, param2: super::Foundation::LPARAM) -> windows_core::BOOL>;
pub type DATEFMT_ENUMPROCEXW = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: u32) -> windows_core::BOOL>;
pub type DATEFMT_ENUMPROCW = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR) -> windows_core::BOOL>;
pub const DATE_AUTOLAYOUT: ENUM_DATE_FORMATS_FLAGS = ENUM_DATE_FORMATS_FLAGS(64u32);
pub const DATE_LONGDATE: ENUM_DATE_FORMATS_FLAGS = ENUM_DATE_FORMATS_FLAGS(2u32);
pub const DATE_LTRREADING: ENUM_DATE_FORMATS_FLAGS = ENUM_DATE_FORMATS_FLAGS(16u32);
pub const DATE_MONTHDAY: ENUM_DATE_FORMATS_FLAGS = ENUM_DATE_FORMATS_FLAGS(128u32);
pub const DATE_RTLREADING: ENUM_DATE_FORMATS_FLAGS = ENUM_DATE_FORMATS_FLAGS(32u32);
pub const DATE_SHORTDATE: ENUM_DATE_FORMATS_FLAGS = ENUM_DATE_FORMATS_FLAGS(1u32);
pub const DATE_USE_ALT_CALENDAR: ENUM_DATE_FORMATS_FLAGS = ENUM_DATE_FORMATS_FLAGS(4u32);
pub const DATE_YEARMONTH: ENUM_DATE_FORMATS_FLAGS = ENUM_DATE_FORMATS_FLAGS(8u32);
pub const DayUnit: CALDATETIME_DATEUNIT = CALDATETIME_DATEUNIT(4i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DetectEncodingInfo {
    pub nLangID: u32,
    pub nCodePage: u32,
    pub nDocPercent: i32,
    pub nConfidence: i32,
}
pub const ELS_GUID_LANGUAGE_DETECTION: windows_core::GUID = windows_core::GUID::from_u128(0xcf7e00b1_909b_4d95_a8f4_611f7c377702);
pub const ELS_GUID_SCRIPT_DETECTION: windows_core::GUID = windows_core::GUID::from_u128(0x2d64b439_6caf_4f6b_b688_e5d0f4faa7d7);
pub const ELS_GUID_TRANSLITERATION_BENGALI_TO_LATIN: windows_core::GUID = windows_core::GUID::from_u128(0xf4dfd825_91a4_489f_855e_9ad9bee55727);
pub const ELS_GUID_TRANSLITERATION_CYRILLIC_TO_LATIN: windows_core::GUID = windows_core::GUID::from_u128(0x3dd12a98_5afd_4903_a13f_e17e6c0bfe01);
pub const ELS_GUID_TRANSLITERATION_DEVANAGARI_TO_LATIN: windows_core::GUID = windows_core::GUID::from_u128(0xc4a4dcfe_2661_4d02_9835_f48187109803);
pub const ELS_GUID_TRANSLITERATION_HANGUL_DECOMPOSITION: windows_core::GUID = windows_core::GUID::from_u128(0x4ba2a721_e43d_41b7_b330_536ae1e48863);
pub const ELS_GUID_TRANSLITERATION_HANS_TO_HANT: windows_core::GUID = windows_core::GUID::from_u128(0x3caccdc8_5590_42dc_9a7b_b5a6b5b3b63b);
pub const ELS_GUID_TRANSLITERATION_HANT_TO_HANS: windows_core::GUID = windows_core::GUID::from_u128(0xa3a8333b_f4fc_42f6_a0c4_0462fe7317cb);
pub const ELS_GUID_TRANSLITERATION_MALAYALAM_TO_LATIN: windows_core::GUID = windows_core::GUID::from_u128(0xd8b983b1_f8bf_4a2b_bcd5_5b5ea20613e1);
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ENUMTEXTMETRICA {
    pub etmNewTextMetricEx: NEWTEXTMETRICEXA,
    pub etmAxesList: super::Graphics::Gdi::AXESLISTA,
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ENUMTEXTMETRICW {
    pub etmNewTextMetricEx: NEWTEXTMETRICEXW,
    pub etmAxesList: super::Graphics::Gdi::AXESLISTW,
}
pub const ENUM_ALL_CALENDARS: u32 = 4294967295u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ENUM_DATE_FORMATS_FLAGS(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ENUM_SYSTEM_CODE_PAGES_FLAGS(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ENUM_SYSTEM_LANGUAGE_GROUPS_FLAGS(pub u32);
pub const EraUnit: CALDATETIME_DATEUNIT = CALDATETIME_DATEUNIT(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Default for FILEMUIINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FIND_ENDSWITH: u32 = 2097152u32;
pub const FIND_FROMEND: u32 = 8388608u32;
pub const FIND_FROMSTART: u32 = 4194304u32;
pub const FIND_STARTSWITH: u32 = 1048576u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FOLD_STRING_MAP_FLAGS(pub u32);
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
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FONTSIGNATURE {
    pub fsUsb: [u32; 4],
    pub fsCsb: [u32; 2],
}
impl Default for FONTSIGNATURE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const GEOCLASS_ALL: SYSGEOCLASS = SYSGEOCLASS(0i32);
pub const GEOCLASS_NATION: SYSGEOCLASS = SYSGEOCLASS(16i32);
pub const GEOCLASS_REGION: SYSGEOCLASS = SYSGEOCLASS(14i32);
pub const GEOID_NOT_AVAILABLE: i32 = -1i32;
pub const GEO_CURRENCYCODE: SYSGEOTYPE = SYSGEOTYPE(15i32);
pub const GEO_CURRENCYSYMBOL: SYSGEOTYPE = SYSGEOTYPE(16i32);
pub const GEO_DIALINGCODE: SYSGEOTYPE = SYSGEOTYPE(14i32);
pub type GEO_ENUMNAMEPROC = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: super::Foundation::LPARAM) -> windows_core::BOOL>;
pub type GEO_ENUMPROC = Option<unsafe extern "system" fn(param0: i32) -> windows_core::BOOL>;
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GOFFSET {
    pub du: i32,
    pub dv: i32,
}
pub const GSS_ALLOW_INHERITED_COMMON: u32 = 1u32;
pub const HIGHLEVEL_SERVICE_TYPES: u32 = 1u32;
pub const HIGH_SURROGATE_END: u32 = 56319u32;
pub const HIGH_SURROGATE_START: u32 = 55296u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HSAVEDUILANGUAGES(pub *mut core::ffi::c_void);
impl HSAVEDUILANGUAGES {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl Default for HSAVEDUILANGUAGES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HourUnit: CALDATETIME_DATEUNIT = CALDATETIME_DATEUNIT(5i32);
windows_core::imp::define_interface!(IComprehensiveSpellCheckProvider, IComprehensiveSpellCheckProvider_Vtbl, 0x0c58f8de_8e94_479e_9717_70c42c4ad2c3);
windows_core::imp::interface_hierarchy!(IComprehensiveSpellCheckProvider, windows_core::IUnknown);
impl IComprehensiveSpellCheckProvider {
    pub unsafe fn ComprehensiveCheck<P0>(&self, text: P0) -> windows_core::Result<IEnumSpellingError>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ComprehensiveCheck)(windows_core::Interface::as_raw(self), text.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComprehensiveSpellCheckProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ComprehensiveCheck: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IComprehensiveSpellCheckProvider_Impl: windows_core::IUnknownImpl {
    fn ComprehensiveCheck(&self, text: &windows_core::PCWSTR) -> windows_core::Result<IEnumSpellingError>;
}
impl IComprehensiveSpellCheckProvider_Vtbl {
    pub const fn new<Identity: IComprehensiveSpellCheckProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ComprehensiveCheck<Identity: IComprehensiveSpellCheckProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, text: windows_core::PCWSTR, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IComprehensiveSpellCheckProvider_Impl::ComprehensiveCheck(this, core::mem::transmute(&text)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ComprehensiveCheck: ComprehensiveCheck::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComprehensiveSpellCheckProvider as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IComprehensiveSpellCheckProvider {}
pub const IDN_ALLOW_UNASSIGNED: u32 = 1u32;
pub const IDN_EMAIL_ADDRESS: u32 = 4u32;
pub const IDN_RAW_PUNYCODE: u32 = 8u32;
pub const IDN_USE_STD3_ASCII_RULES: u32 = 2u32;
windows_core::imp::define_interface!(IEnumCodePage, IEnumCodePage_Vtbl, 0x275c23e3_3747_11d0_9fea_00aa003f8646);
windows_core::imp::interface_hierarchy!(IEnumCodePage, windows_core::IUnknown);
impl IEnumCodePage {
    pub unsafe fn Clone(&self, ppenum: Option<*const Option<IEnumCodePage>>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), ppenum.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut MIMECPINFO, pceltfetched: Option<*mut u32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, rgelt as _, pceltfetched.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumCodePage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut MIMECPINFO, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IEnumCodePage_Impl: windows_core::IUnknownImpl {
    fn Clone(&self, ppenum: *const Option<IEnumCodePage>) -> windows_core::Result<()>;
    fn Next(&self, celt: u32, rgelt: *mut MIMECPINFO, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
}
impl IEnumCodePage_Vtbl {
    pub const fn new<Identity: IEnumCodePage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Clone<Identity: IEnumCodePage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *const *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumCodePage_Impl::Clone(this, core::mem::transmute_copy(&ppenum)).into()
            }
        }
        unsafe extern "system" fn Next<Identity: IEnumCodePage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut MIMECPINFO, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumCodePage_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumCodePage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumCodePage_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumCodePage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumCodePage_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumCodePage as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumCodePage {}
windows_core::imp::define_interface!(IEnumRfc1766, IEnumRfc1766_Vtbl, 0x3dc39d1d_c030_11d0_b81b_00c04fc9b31f);
windows_core::imp::interface_hierarchy!(IEnumRfc1766, windows_core::IUnknown);
impl IEnumRfc1766 {
    pub unsafe fn Clone(&self, ppenum: Option<*const Option<IEnumRfc1766>>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), ppenum.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut RFC1766INFO, pceltfetched: Option<*mut u32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, rgelt as _, pceltfetched.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumRfc1766_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut RFC1766INFO, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IEnumRfc1766_Impl: windows_core::IUnknownImpl {
    fn Clone(&self, ppenum: *const Option<IEnumRfc1766>) -> windows_core::Result<()>;
    fn Next(&self, celt: u32, rgelt: *mut RFC1766INFO, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
}
impl IEnumRfc1766_Vtbl {
    pub const fn new<Identity: IEnumRfc1766_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Clone<Identity: IEnumRfc1766_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *const *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumRfc1766_Impl::Clone(this, core::mem::transmute_copy(&ppenum)).into()
            }
        }
        unsafe extern "system" fn Next<Identity: IEnumRfc1766_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut RFC1766INFO, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumRfc1766_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumRfc1766_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumRfc1766_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumRfc1766_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumRfc1766_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumRfc1766 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumRfc1766 {}
windows_core::imp::define_interface!(IEnumScript, IEnumScript_Vtbl, 0xae5f1430_388b_11d2_8380_00c04f8f5da1);
windows_core::imp::interface_hierarchy!(IEnumScript, windows_core::IUnknown);
impl IEnumScript {
    pub unsafe fn Clone(&self, ppenum: Option<*const Option<IEnumScript>>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), ppenum.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut SCRIPTINFO, pceltfetched: Option<*mut u32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, rgelt as _, pceltfetched.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumScript_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut SCRIPTINFO, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IEnumScript_Impl: windows_core::IUnknownImpl {
    fn Clone(&self, ppenum: *const Option<IEnumScript>) -> windows_core::Result<()>;
    fn Next(&self, celt: u32, rgelt: *mut SCRIPTINFO, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
}
impl IEnumScript_Vtbl {
    pub const fn new<Identity: IEnumScript_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Clone<Identity: IEnumScript_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *const *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumScript_Impl::Clone(this, core::mem::transmute_copy(&ppenum)).into()
            }
        }
        unsafe extern "system" fn Next<Identity: IEnumScript_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut SCRIPTINFO, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumScript_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumScript_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumScript_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumScript_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumScript_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumScript as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumScript {}
windows_core::imp::define_interface!(IEnumSpellingError, IEnumSpellingError_Vtbl, 0x803e3bd4_2828_4410_8290_418d1d73c762);
windows_core::imp::interface_hierarchy!(IEnumSpellingError, windows_core::IUnknown);
impl IEnumSpellingError {
    pub unsafe fn Next(&self, value: *mut Option<ISpellingError>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), core::mem::transmute(value)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSpellingError_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumSpellingError_Impl: windows_core::IUnknownImpl {
    fn Next(&self, value: windows_core::OutRef<ISpellingError>) -> windows_core::HRESULT;
}
impl IEnumSpellingError_Vtbl {
    pub const fn new<Identity: IEnumSpellingError_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumSpellingError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumSpellingError_Impl::Next(this, core::mem::transmute_copy(&value))
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Next: Next::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumSpellingError as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumSpellingError {}
windows_core::imp::define_interface!(IMLangCodePages, IMLangCodePages_Vtbl, 0x359f3443_bd4a_11d0_b188_00aa0038c969);
windows_core::imp::interface_hierarchy!(IMLangCodePages, windows_core::IUnknown);
impl IMLangCodePages {
    pub unsafe fn GetCharCodePages(&self, chsrc: u16) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCharCodePages)(windows_core::Interface::as_raw(self), chsrc, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetStrCodePages(&self, pszsrc: &[u16], dwprioritycodepages: u32, pdwcodepages: Option<*mut u32>, pcchcodepages: Option<*mut i32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetStrCodePages)(windows_core::Interface::as_raw(self), core::mem::transmute(pszsrc.as_ptr()), pszsrc.len().try_into().unwrap(), dwprioritycodepages, pdwcodepages.unwrap_or(core::mem::zeroed()) as _, pcchcodepages.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn CodePageToCodePages(&self, ucodepage: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CodePageToCodePages)(windows_core::Interface::as_raw(self), ucodepage, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CodePagesToCodePage(&self, dwcodepages: u32, udefaultcodepage: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CodePagesToCodePage)(windows_core::Interface::as_raw(self), dwcodepages, udefaultcodepage, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLangCodePages_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCharCodePages: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut u32) -> windows_core::HRESULT,
    pub GetStrCodePages: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32, u32, *mut u32, *mut i32) -> windows_core::HRESULT,
    pub CodePageToCodePages: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub CodePagesToCodePage: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut u32) -> windows_core::HRESULT,
}
pub trait IMLangCodePages_Impl: windows_core::IUnknownImpl {
    fn GetCharCodePages(&self, chsrc: u16) -> windows_core::Result<u32>;
    fn GetStrCodePages(&self, pszsrc: &windows_core::PCWSTR, cchsrc: i32, dwprioritycodepages: u32, pdwcodepages: *mut u32, pcchcodepages: *mut i32) -> windows_core::Result<()>;
    fn CodePageToCodePages(&self, ucodepage: u32) -> windows_core::Result<u32>;
    fn CodePagesToCodePage(&self, dwcodepages: u32, udefaultcodepage: u32) -> windows_core::Result<u32>;
}
impl IMLangCodePages_Vtbl {
    pub const fn new<Identity: IMLangCodePages_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCharCodePages<Identity: IMLangCodePages_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, chsrc: u16, pdwcodepages: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMLangCodePages_Impl::GetCharCodePages(this, core::mem::transmute_copy(&chsrc)) {
                    Ok(ok__) => {
                        pdwcodepages.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStrCodePages<Identity: IMLangCodePages_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszsrc: windows_core::PCWSTR, cchsrc: i32, dwprioritycodepages: u32, pdwcodepages: *mut u32, pcchcodepages: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangCodePages_Impl::GetStrCodePages(this, core::mem::transmute(&pszsrc), core::mem::transmute_copy(&cchsrc), core::mem::transmute_copy(&dwprioritycodepages), core::mem::transmute_copy(&pdwcodepages), core::mem::transmute_copy(&pcchcodepages)).into()
            }
        }
        unsafe extern "system" fn CodePageToCodePages<Identity: IMLangCodePages_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ucodepage: u32, pdwcodepages: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMLangCodePages_Impl::CodePageToCodePages(this, core::mem::transmute_copy(&ucodepage)) {
                    Ok(ok__) => {
                        pdwcodepages.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CodePagesToCodePage<Identity: IMLangCodePages_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcodepages: u32, udefaultcodepage: u32, pucodepage: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMLangCodePages_Impl::CodePagesToCodePage(this, core::mem::transmute_copy(&dwcodepages), core::mem::transmute_copy(&udefaultcodepage)) {
                    Ok(ok__) => {
                        pucodepage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCharCodePages: GetCharCodePages::<Identity, OFFSET>,
            GetStrCodePages: GetStrCodePages::<Identity, OFFSET>,
            CodePageToCodePages: CodePageToCodePages::<Identity, OFFSET>,
            CodePagesToCodePage: CodePagesToCodePage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLangCodePages as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMLangCodePages {}
windows_core::imp::define_interface!(IMLangConvertCharset, IMLangConvertCharset_Vtbl, 0xd66d6f98_cdaa_11d0_b822_00c04fc9b31f);
windows_core::imp::interface_hierarchy!(IMLangConvertCharset, windows_core::IUnknown);
impl IMLangConvertCharset {
    pub unsafe fn Initialize(&self, uisrccodepage: u32, uidstcodepage: u32, dwproperty: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), uisrccodepage, uidstcodepage, dwproperty).ok() }
    }
    pub unsafe fn GetSourceCodePage(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSourceCodePage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDestinationCodePage(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDestinationCodePage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetProperty(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DoConversion(&self, psrcstr: *const u8, pcsrcsize: Option<*mut u32>, pdststr: *mut u8, pcdstsize: Option<*mut u32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DoConversion)(windows_core::Interface::as_raw(self), psrcstr, pcsrcsize.unwrap_or(core::mem::zeroed()) as _, pdststr as _, pcdstsize.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn DoConversionToUnicode<P0>(&self, psrcstr: P0, pcsrcsize: Option<*mut u32>, pdststr: windows_core::PWSTR, pcdstsize: Option<*mut u32>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).DoConversionToUnicode)(windows_core::Interface::as_raw(self), psrcstr.param().abi(), pcsrcsize.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pdststr), pcdstsize.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn DoConversionFromUnicode<P0>(&self, psrcstr: P0, pcsrcsize: Option<*mut u32>, pdststr: windows_core::PSTR, pcdstsize: Option<*mut u32>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).DoConversionFromUnicode)(windows_core::Interface::as_raw(self), psrcstr.param().abi(), pcsrcsize.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pdststr), pcdstsize.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
pub trait IMLangConvertCharset_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, uisrccodepage: u32, uidstcodepage: u32, dwproperty: u32) -> windows_core::Result<()>;
    fn GetSourceCodePage(&self) -> windows_core::Result<u32>;
    fn GetDestinationCodePage(&self) -> windows_core::Result<u32>;
    fn GetProperty(&self) -> windows_core::Result<u32>;
    fn DoConversion(&self, psrcstr: *const u8, pcsrcsize: *mut u32, pdststr: *mut u8, pcdstsize: *mut u32) -> windows_core::Result<()>;
    fn DoConversionToUnicode(&self, psrcstr: &windows_core::PCSTR, pcsrcsize: *mut u32, pdststr: windows_core::PWSTR, pcdstsize: *mut u32) -> windows_core::Result<()>;
    fn DoConversionFromUnicode(&self, psrcstr: &windows_core::PCWSTR, pcsrcsize: *mut u32, pdststr: windows_core::PSTR, pcdstsize: *mut u32) -> windows_core::Result<()>;
}
impl IMLangConvertCharset_Vtbl {
    pub const fn new<Identity: IMLangConvertCharset_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IMLangConvertCharset_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uisrccodepage: u32, uidstcodepage: u32, dwproperty: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangConvertCharset_Impl::Initialize(this, core::mem::transmute_copy(&uisrccodepage), core::mem::transmute_copy(&uidstcodepage), core::mem::transmute_copy(&dwproperty)).into()
            }
        }
        unsafe extern "system" fn GetSourceCodePage<Identity: IMLangConvertCharset_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puisrccodepage: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMLangConvertCharset_Impl::GetSourceCodePage(this) {
                    Ok(ok__) => {
                        puisrccodepage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDestinationCodePage<Identity: IMLangConvertCharset_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puidstcodepage: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMLangConvertCharset_Impl::GetDestinationCodePage(this) {
                    Ok(ok__) => {
                        puidstcodepage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProperty<Identity: IMLangConvertCharset_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwproperty: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMLangConvertCharset_Impl::GetProperty(this) {
                    Ok(ok__) => {
                        pdwproperty.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DoConversion<Identity: IMLangConvertCharset_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psrcstr: *const u8, pcsrcsize: *mut u32, pdststr: *mut u8, pcdstsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangConvertCharset_Impl::DoConversion(this, core::mem::transmute_copy(&psrcstr), core::mem::transmute_copy(&pcsrcsize), core::mem::transmute_copy(&pdststr), core::mem::transmute_copy(&pcdstsize)).into()
            }
        }
        unsafe extern "system" fn DoConversionToUnicode<Identity: IMLangConvertCharset_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psrcstr: windows_core::PCSTR, pcsrcsize: *mut u32, pdststr: windows_core::PWSTR, pcdstsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangConvertCharset_Impl::DoConversionToUnicode(this, core::mem::transmute(&psrcstr), core::mem::transmute_copy(&pcsrcsize), core::mem::transmute_copy(&pdststr), core::mem::transmute_copy(&pcdstsize)).into()
            }
        }
        unsafe extern "system" fn DoConversionFromUnicode<Identity: IMLangConvertCharset_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psrcstr: windows_core::PCWSTR, pcsrcsize: *mut u32, pdststr: windows_core::PSTR, pcdstsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangConvertCharset_Impl::DoConversionFromUnicode(this, core::mem::transmute(&psrcstr), core::mem::transmute_copy(&pcsrcsize), core::mem::transmute_copy(&pdststr), core::mem::transmute_copy(&pcdstsize)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            GetSourceCodePage: GetSourceCodePage::<Identity, OFFSET>,
            GetDestinationCodePage: GetDestinationCodePage::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            DoConversion: DoConversion::<Identity, OFFSET>,
            DoConversionToUnicode: DoConversionToUnicode::<Identity, OFFSET>,
            DoConversionFromUnicode: DoConversionFromUnicode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLangConvertCharset as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMLangConvertCharset {}
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
    pub unsafe fn GetFontCodePages(&self, hdc: super::Graphics::Gdi::HDC, hfont: super::Graphics::Gdi::HFONT, pdwcodepages: Option<*mut u32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetFontCodePages)(windows_core::Interface::as_raw(self), hdc, hfont, pdwcodepages.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn MapFont(&self, hdc: super::Graphics::Gdi::HDC, dwcodepages: u32, hsrcfont: super::Graphics::Gdi::HFONT, phdestfont: Option<*mut super::Graphics::Gdi::HFONT>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).MapFont)(windows_core::Interface::as_raw(self), hdc, dwcodepages, hsrcfont, phdestfont.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn ReleaseFont(&self, hfont: super::Graphics::Gdi::HFONT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReleaseFont)(windows_core::Interface::as_raw(self), hfont).ok() }
    }
    pub unsafe fn ResetFontMapping(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ResetFontMapping)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IMLangFontLink_Impl: IMLangCodePages_Impl {
    fn GetFontCodePages(&self, hdc: super::Graphics::Gdi::HDC, hfont: super::Graphics::Gdi::HFONT, pdwcodepages: *mut u32) -> windows_core::Result<()>;
    fn MapFont(&self, hdc: super::Graphics::Gdi::HDC, dwcodepages: u32, hsrcfont: super::Graphics::Gdi::HFONT, phdestfont: *mut super::Graphics::Gdi::HFONT) -> windows_core::Result<()>;
    fn ReleaseFont(&self, hfont: super::Graphics::Gdi::HFONT) -> windows_core::Result<()>;
    fn ResetFontMapping(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IMLangFontLink_Vtbl {
    pub const fn new<Identity: IMLangFontLink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFontCodePages<Identity: IMLangFontLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdc: super::Graphics::Gdi::HDC, hfont: super::Graphics::Gdi::HFONT, pdwcodepages: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangFontLink_Impl::GetFontCodePages(this, core::mem::transmute_copy(&hdc), core::mem::transmute_copy(&hfont), core::mem::transmute_copy(&pdwcodepages)).into()
            }
        }
        unsafe extern "system" fn MapFont<Identity: IMLangFontLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdc: super::Graphics::Gdi::HDC, dwcodepages: u32, hsrcfont: super::Graphics::Gdi::HFONT, phdestfont: *mut super::Graphics::Gdi::HFONT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangFontLink_Impl::MapFont(this, core::mem::transmute_copy(&hdc), core::mem::transmute_copy(&dwcodepages), core::mem::transmute_copy(&hsrcfont), core::mem::transmute_copy(&phdestfont)).into()
            }
        }
        unsafe extern "system" fn ReleaseFont<Identity: IMLangFontLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hfont: super::Graphics::Gdi::HFONT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangFontLink_Impl::ReleaseFont(this, core::mem::transmute_copy(&hfont)).into()
            }
        }
        unsafe extern "system" fn ResetFontMapping<Identity: IMLangFontLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangFontLink_Impl::ResetFontMapping(this).into()
            }
        }
        Self {
            base__: IMLangCodePages_Vtbl::new::<Identity, OFFSET>(),
            GetFontCodePages: GetFontCodePages::<Identity, OFFSET>,
            MapFont: MapFont::<Identity, OFFSET>,
            ReleaseFont: ReleaseFont::<Identity, OFFSET>,
            ResetFontMapping: ResetFontMapping::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLangFontLink as windows_core::Interface>::IID || iid == &<IMLangCodePages as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IMLangFontLink {}
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
    pub unsafe fn GetFontCodePages(&self, hdc: super::Graphics::Gdi::HDC, hfont: super::Graphics::Gdi::HFONT, pdwcodepages: Option<*mut u32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetFontCodePages)(windows_core::Interface::as_raw(self), hdc, hfont, pdwcodepages.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn ReleaseFont(&self, hfont: super::Graphics::Gdi::HFONT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReleaseFont)(windows_core::Interface::as_raw(self), hfont).ok() }
    }
    pub unsafe fn ResetFontMapping(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ResetFontMapping)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn MapFont(&self, hdc: super::Graphics::Gdi::HDC, dwcodepages: u32, chsrc: u16, pfont: Option<*mut super::Graphics::Gdi::HFONT>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).MapFont)(windows_core::Interface::as_raw(self), hdc, dwcodepages, chsrc, pfont.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetFontUnicodeRanges(&self, hdc: super::Graphics::Gdi::HDC, puiranges: *const u32, puranges: Option<*mut UNICODERANGE>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetFontUnicodeRanges)(windows_core::Interface::as_raw(self), hdc, puiranges, puranges.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn GetScriptFontInfo(&self, sid: u8, dwflags: u32, puifonts: *mut u32, pscriptfont: Option<*mut SCRIPTFONTINFO>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetScriptFontInfo)(windows_core::Interface::as_raw(self), sid, dwflags, puifonts as _, pscriptfont.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn CodePageToScriptID(&self, uicodepage: u32) -> windows_core::Result<u8> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CodePageToScriptID)(windows_core::Interface::as_raw(self), uicodepage, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IMLangFontLink2_Impl: IMLangCodePages_Impl {
    fn GetFontCodePages(&self, hdc: super::Graphics::Gdi::HDC, hfont: super::Graphics::Gdi::HFONT, pdwcodepages: *mut u32) -> windows_core::Result<()>;
    fn ReleaseFont(&self, hfont: super::Graphics::Gdi::HFONT) -> windows_core::Result<()>;
    fn ResetFontMapping(&self) -> windows_core::Result<()>;
    fn MapFont(&self, hdc: super::Graphics::Gdi::HDC, dwcodepages: u32, chsrc: u16, pfont: *mut super::Graphics::Gdi::HFONT) -> windows_core::Result<()>;
    fn GetFontUnicodeRanges(&self, hdc: super::Graphics::Gdi::HDC, puiranges: *const u32, puranges: *mut UNICODERANGE) -> windows_core::Result<()>;
    fn GetScriptFontInfo(&self, sid: u8, dwflags: u32, puifonts: *mut u32, pscriptfont: *mut SCRIPTFONTINFO) -> windows_core::Result<()>;
    fn CodePageToScriptID(&self, uicodepage: u32) -> windows_core::Result<u8>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IMLangFontLink2_Vtbl {
    pub const fn new<Identity: IMLangFontLink2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFontCodePages<Identity: IMLangFontLink2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdc: super::Graphics::Gdi::HDC, hfont: super::Graphics::Gdi::HFONT, pdwcodepages: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangFontLink2_Impl::GetFontCodePages(this, core::mem::transmute_copy(&hdc), core::mem::transmute_copy(&hfont), core::mem::transmute_copy(&pdwcodepages)).into()
            }
        }
        unsafe extern "system" fn ReleaseFont<Identity: IMLangFontLink2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hfont: super::Graphics::Gdi::HFONT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangFontLink2_Impl::ReleaseFont(this, core::mem::transmute_copy(&hfont)).into()
            }
        }
        unsafe extern "system" fn ResetFontMapping<Identity: IMLangFontLink2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangFontLink2_Impl::ResetFontMapping(this).into()
            }
        }
        unsafe extern "system" fn MapFont<Identity: IMLangFontLink2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdc: super::Graphics::Gdi::HDC, dwcodepages: u32, chsrc: u16, pfont: *mut super::Graphics::Gdi::HFONT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangFontLink2_Impl::MapFont(this, core::mem::transmute_copy(&hdc), core::mem::transmute_copy(&dwcodepages), core::mem::transmute_copy(&chsrc), core::mem::transmute_copy(&pfont)).into()
            }
        }
        unsafe extern "system" fn GetFontUnicodeRanges<Identity: IMLangFontLink2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdc: super::Graphics::Gdi::HDC, puiranges: *const u32, puranges: *mut UNICODERANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangFontLink2_Impl::GetFontUnicodeRanges(this, core::mem::transmute_copy(&hdc), core::mem::transmute_copy(&puiranges), core::mem::transmute_copy(&puranges)).into()
            }
        }
        unsafe extern "system" fn GetScriptFontInfo<Identity: IMLangFontLink2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sid: u8, dwflags: u32, puifonts: *mut u32, pscriptfont: *mut SCRIPTFONTINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangFontLink2_Impl::GetScriptFontInfo(this, core::mem::transmute_copy(&sid), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&puifonts), core::mem::transmute_copy(&pscriptfont)).into()
            }
        }
        unsafe extern "system" fn CodePageToScriptID<Identity: IMLangFontLink2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uicodepage: u32, psid: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMLangFontLink2_Impl::CodePageToScriptID(this, core::mem::transmute_copy(&uicodepage)) {
                    Ok(ok__) => {
                        psid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IMLangCodePages_Vtbl::new::<Identity, OFFSET>(),
            GetFontCodePages: GetFontCodePages::<Identity, OFFSET>,
            ReleaseFont: ReleaseFont::<Identity, OFFSET>,
            ResetFontMapping: ResetFontMapping::<Identity, OFFSET>,
            MapFont: MapFont::<Identity, OFFSET>,
            GetFontUnicodeRanges: GetFontUnicodeRanges::<Identity, OFFSET>,
            GetScriptFontInfo: GetScriptFontInfo::<Identity, OFFSET>,
            CodePageToScriptID: CodePageToScriptID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLangFontLink2 as windows_core::Interface>::IID || iid == &<IMLangCodePages as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IMLangFontLink2 {}
windows_core::imp::define_interface!(IMLangLineBreakConsole, IMLangLineBreakConsole_Vtbl, 0xf5be2ee1_bfd7_11d0_b188_00aa0038c969);
windows_core::imp::interface_hierarchy!(IMLangLineBreakConsole, windows_core::IUnknown);
impl IMLangLineBreakConsole {
    pub unsafe fn BreakLineML<P0>(&self, psrcmlstr: P0, lsrcpos: i32, lsrclen: i32, cmincolumns: i32, cmaxcolumns: i32, pllinelen: Option<*mut i32>, plskiplen: Option<*mut i32>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMLangString>,
    {
        unsafe { (windows_core::Interface::vtable(self).BreakLineML)(windows_core::Interface::as_raw(self), psrcmlstr.param().abi(), lsrcpos, lsrclen, cmincolumns, cmaxcolumns, pllinelen.unwrap_or(core::mem::zeroed()) as _, plskiplen.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn BreakLineW(&self, locale: u32, pszsrc: &[u16], cmaxcolumns: i32, pcchline: Option<*mut i32>, pcchskip: Option<*mut i32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BreakLineW)(windows_core::Interface::as_raw(self), locale, core::mem::transmute(pszsrc.as_ptr()), pszsrc.len().try_into().unwrap(), cmaxcolumns, pcchline.unwrap_or(core::mem::zeroed()) as _, pcchskip.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn BreakLineA(&self, locale: u32, ucodepage: u32, pszsrc: &[u8], cmaxcolumns: i32, pcchline: Option<*mut i32>, pcchskip: Option<*mut i32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BreakLineA)(windows_core::Interface::as_raw(self), locale, ucodepage, core::mem::transmute(pszsrc.as_ptr()), pszsrc.len().try_into().unwrap(), cmaxcolumns, pcchline.unwrap_or(core::mem::zeroed()) as _, pcchskip.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLangLineBreakConsole_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub BreakLineML: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32, i32, i32, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub BreakLineW: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, i32, i32, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub BreakLineA: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, windows_core::PCSTR, i32, i32, *mut i32, *mut i32) -> windows_core::HRESULT,
}
pub trait IMLangLineBreakConsole_Impl: windows_core::IUnknownImpl {
    fn BreakLineML(&self, psrcmlstr: windows_core::Ref<IMLangString>, lsrcpos: i32, lsrclen: i32, cmincolumns: i32, cmaxcolumns: i32, pllinelen: *mut i32, plskiplen: *mut i32) -> windows_core::Result<()>;
    fn BreakLineW(&self, locale: u32, pszsrc: &windows_core::PCWSTR, cchsrc: i32, cmaxcolumns: i32, pcchline: *mut i32, pcchskip: *mut i32) -> windows_core::Result<()>;
    fn BreakLineA(&self, locale: u32, ucodepage: u32, pszsrc: &windows_core::PCSTR, cchsrc: i32, cmaxcolumns: i32, pcchline: *mut i32, pcchskip: *mut i32) -> windows_core::Result<()>;
}
impl IMLangLineBreakConsole_Vtbl {
    pub const fn new<Identity: IMLangLineBreakConsole_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BreakLineML<Identity: IMLangLineBreakConsole_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psrcmlstr: *mut core::ffi::c_void, lsrcpos: i32, lsrclen: i32, cmincolumns: i32, cmaxcolumns: i32, pllinelen: *mut i32, plskiplen: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangLineBreakConsole_Impl::BreakLineML(this, core::mem::transmute_copy(&psrcmlstr), core::mem::transmute_copy(&lsrcpos), core::mem::transmute_copy(&lsrclen), core::mem::transmute_copy(&cmincolumns), core::mem::transmute_copy(&cmaxcolumns), core::mem::transmute_copy(&pllinelen), core::mem::transmute_copy(&plskiplen)).into()
            }
        }
        unsafe extern "system" fn BreakLineW<Identity: IMLangLineBreakConsole_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, locale: u32, pszsrc: windows_core::PCWSTR, cchsrc: i32, cmaxcolumns: i32, pcchline: *mut i32, pcchskip: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangLineBreakConsole_Impl::BreakLineW(this, core::mem::transmute_copy(&locale), core::mem::transmute(&pszsrc), core::mem::transmute_copy(&cchsrc), core::mem::transmute_copy(&cmaxcolumns), core::mem::transmute_copy(&pcchline), core::mem::transmute_copy(&pcchskip)).into()
            }
        }
        unsafe extern "system" fn BreakLineA<Identity: IMLangLineBreakConsole_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, locale: u32, ucodepage: u32, pszsrc: windows_core::PCSTR, cchsrc: i32, cmaxcolumns: i32, pcchline: *mut i32, pcchskip: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangLineBreakConsole_Impl::BreakLineA(this, core::mem::transmute_copy(&locale), core::mem::transmute_copy(&ucodepage), core::mem::transmute(&pszsrc), core::mem::transmute_copy(&cchsrc), core::mem::transmute_copy(&cmaxcolumns), core::mem::transmute_copy(&pcchline), core::mem::transmute_copy(&pcchskip)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BreakLineML: BreakLineML::<Identity, OFFSET>,
            BreakLineW: BreakLineW::<Identity, OFFSET>,
            BreakLineA: BreakLineA::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLangLineBreakConsole as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMLangLineBreakConsole {}
windows_core::imp::define_interface!(IMLangString, IMLangString_Vtbl, 0xc04d65ce_b70d_11d0_b188_00aa0038c969);
windows_core::imp::interface_hierarchy!(IMLangString, windows_core::IUnknown);
impl IMLangString {
    pub unsafe fn Sync(&self, fnoaccess: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Sync)(windows_core::Interface::as_raw(self), fnoaccess.into()).ok() }
    }
    pub unsafe fn GetLength(&self, pllen: Option<*mut i32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetLength)(windows_core::Interface::as_raw(self), pllen.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn SetMLStr<P2>(&self, ldestpos: i32, ldestlen: i32, psrcmlstr: P2, lsrcpos: i32, lsrclen: i32) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetMLStr)(windows_core::Interface::as_raw(self), ldestpos, ldestlen, psrcmlstr.param().abi(), lsrcpos, lsrclen).ok() }
    }
    pub unsafe fn GetMLStr<P2>(&self, lsrcpos: i32, lsrclen: i32, punkouter: P2, dwclscontext: u32, piid: *const windows_core::GUID, ppdestmlstr: *mut Option<windows_core::IUnknown>, pldestpos: Option<*mut i32>, pldestlen: Option<*mut i32>) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetMLStr)(windows_core::Interface::as_raw(self), lsrcpos, lsrclen, punkouter.param().abi(), dwclscontext, piid, core::mem::transmute(ppdestmlstr), pldestpos.unwrap_or(core::mem::zeroed()) as _, pldestlen.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLangString_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Sync: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMLStr: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub GetMLStr: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut core::ffi::c_void, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void, *mut i32, *mut i32) -> windows_core::HRESULT,
}
pub trait IMLangString_Impl: windows_core::IUnknownImpl {
    fn Sync(&self, fnoaccess: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetLength(&self, pllen: *mut i32) -> windows_core::Result<()>;
    fn SetMLStr(&self, ldestpos: i32, ldestlen: i32, psrcmlstr: windows_core::Ref<windows_core::IUnknown>, lsrcpos: i32, lsrclen: i32) -> windows_core::Result<()>;
    fn GetMLStr(&self, lsrcpos: i32, lsrclen: i32, punkouter: windows_core::Ref<windows_core::IUnknown>, dwclscontext: u32, piid: *const windows_core::GUID, ppdestmlstr: windows_core::OutRef<windows_core::IUnknown>, pldestpos: *mut i32, pldestlen: *mut i32) -> windows_core::Result<()>;
}
impl IMLangString_Vtbl {
    pub const fn new<Identity: IMLangString_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Sync<Identity: IMLangString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fnoaccess: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangString_Impl::Sync(this, core::mem::transmute_copy(&fnoaccess)).into()
            }
        }
        unsafe extern "system" fn GetLength<Identity: IMLangString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pllen: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangString_Impl::GetLength(this, core::mem::transmute_copy(&pllen)).into()
            }
        }
        unsafe extern "system" fn SetMLStr<Identity: IMLangString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ldestpos: i32, ldestlen: i32, psrcmlstr: *mut core::ffi::c_void, lsrcpos: i32, lsrclen: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangString_Impl::SetMLStr(this, core::mem::transmute_copy(&ldestpos), core::mem::transmute_copy(&ldestlen), core::mem::transmute_copy(&psrcmlstr), core::mem::transmute_copy(&lsrcpos), core::mem::transmute_copy(&lsrclen)).into()
            }
        }
        unsafe extern "system" fn GetMLStr<Identity: IMLangString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lsrcpos: i32, lsrclen: i32, punkouter: *mut core::ffi::c_void, dwclscontext: u32, piid: *const windows_core::GUID, ppdestmlstr: *mut *mut core::ffi::c_void, pldestpos: *mut i32, pldestlen: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangString_Impl::GetMLStr(this, core::mem::transmute_copy(&lsrcpos), core::mem::transmute_copy(&lsrclen), core::mem::transmute_copy(&punkouter), core::mem::transmute_copy(&dwclscontext), core::mem::transmute_copy(&piid), core::mem::transmute_copy(&ppdestmlstr), core::mem::transmute_copy(&pldestpos), core::mem::transmute_copy(&pldestlen)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Sync: Sync::<Identity, OFFSET>,
            GetLength: GetLength::<Identity, OFFSET>,
            SetMLStr: SetMLStr::<Identity, OFFSET>,
            GetMLStr: GetMLStr::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLangString as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMLangString {}
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
        unsafe { (windows_core::Interface::vtable(self).SetAStr)(windows_core::Interface::as_raw(self), ldestpos, ldestlen, ucodepage, core::mem::transmute(pszsrc.as_ptr()), pszsrc.len().try_into().unwrap(), pcchactual.unwrap_or(core::mem::zeroed()) as _, plactuallen.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn SetStrBufA<P3>(&self, ldestpos: i32, ldestlen: i32, ucodepage: u32, psrcbuf: P3, pcchactual: Option<*mut i32>, plactuallen: Option<*mut i32>) -> windows_core::Result<()>
    where
        P3: windows_core::Param<IMLangStringBufA>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetStrBufA)(windows_core::Interface::as_raw(self), ldestpos, ldestlen, ucodepage, psrcbuf.param().abi(), pcchactual.unwrap_or(core::mem::zeroed()) as _, plactuallen.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn GetAStr(&self, lsrcpos: i32, lsrclen: i32, ucodepagein: u32, pucodepageout: Option<*const u32>, pszdest: Option<&mut [u8]>, pcchactual: Option<*mut i32>, plactuallen: Option<*mut i32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetAStr)(windows_core::Interface::as_raw(self), lsrcpos, lsrclen, ucodepagein, pucodepageout.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pszdest.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pszdest.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcchactual.unwrap_or(core::mem::zeroed()) as _, plactuallen.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn GetStrBufA(&self, lsrcpos: i32, lsrcmaxlen: i32, pudestcodepage: Option<*mut u32>, ppdestbuf: *mut Option<IMLangStringBufA>, pldestlen: Option<*mut i32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetStrBufA)(windows_core::Interface::as_raw(self), lsrcpos, lsrcmaxlen, pudestcodepage.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(ppdestbuf), pldestlen.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn LockAStr(&self, lsrcpos: i32, lsrclen: i32, lflags: i32, ucodepagein: u32, cchrequest: i32, pucodepageout: Option<*mut u32>, ppszdest: Option<*mut windows_core::PSTR>, pcchdest: Option<*mut i32>, pldestlen: Option<*mut i32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).LockAStr)(windows_core::Interface::as_raw(self), lsrcpos, lsrclen, lflags, ucodepagein, cchrequest, pucodepageout.unwrap_or(core::mem::zeroed()) as _, ppszdest.unwrap_or(core::mem::zeroed()) as _, pcchdest.unwrap_or(core::mem::zeroed()) as _, pldestlen.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn UnlockAStr(&self, pszsrc: &[u8], pcchactual: Option<*mut i32>, plactuallen: Option<*mut i32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UnlockAStr)(windows_core::Interface::as_raw(self), core::mem::transmute(pszsrc.as_ptr()), pszsrc.len().try_into().unwrap(), pcchactual.unwrap_or(core::mem::zeroed()) as _, plactuallen.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn SetLocale(&self, ldestpos: i32, ldestlen: i32, locale: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLocale)(windows_core::Interface::as_raw(self), ldestpos, ldestlen, locale).ok() }
    }
    pub unsafe fn GetLocale(&self, lsrcpos: i32, lsrcmaxlen: i32, plocale: Option<*mut u32>, pllocalepos: Option<*mut i32>, pllocalelen: Option<*mut i32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetLocale)(windows_core::Interface::as_raw(self), lsrcpos, lsrcmaxlen, plocale.unwrap_or(core::mem::zeroed()) as _, pllocalepos.unwrap_or(core::mem::zeroed()) as _, pllocalelen.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
pub trait IMLangStringAStr_Impl: IMLangString_Impl {
    fn SetAStr(&self, ldestpos: i32, ldestlen: i32, ucodepage: u32, pszsrc: &windows_core::PCSTR, cchsrc: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> windows_core::Result<()>;
    fn SetStrBufA(&self, ldestpos: i32, ldestlen: i32, ucodepage: u32, psrcbuf: windows_core::Ref<IMLangStringBufA>, pcchactual: *mut i32, plactuallen: *mut i32) -> windows_core::Result<()>;
    fn GetAStr(&self, lsrcpos: i32, lsrclen: i32, ucodepagein: u32, pucodepageout: *const u32, pszdest: windows_core::PSTR, cchdest: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> windows_core::Result<()>;
    fn GetStrBufA(&self, lsrcpos: i32, lsrcmaxlen: i32, pudestcodepage: *mut u32, ppdestbuf: windows_core::OutRef<IMLangStringBufA>, pldestlen: *mut i32) -> windows_core::Result<()>;
    fn LockAStr(&self, lsrcpos: i32, lsrclen: i32, lflags: i32, ucodepagein: u32, cchrequest: i32, pucodepageout: *mut u32, ppszdest: *mut windows_core::PSTR, pcchdest: *mut i32, pldestlen: *mut i32) -> windows_core::Result<()>;
    fn UnlockAStr(&self, pszsrc: &windows_core::PCSTR, cchsrc: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> windows_core::Result<()>;
    fn SetLocale(&self, ldestpos: i32, ldestlen: i32, locale: u32) -> windows_core::Result<()>;
    fn GetLocale(&self, lsrcpos: i32, lsrcmaxlen: i32, plocale: *mut u32, pllocalepos: *mut i32, pllocalelen: *mut i32) -> windows_core::Result<()>;
}
impl IMLangStringAStr_Vtbl {
    pub const fn new<Identity: IMLangStringAStr_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetAStr<Identity: IMLangStringAStr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ldestpos: i32, ldestlen: i32, ucodepage: u32, pszsrc: windows_core::PCSTR, cchsrc: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangStringAStr_Impl::SetAStr(this, core::mem::transmute_copy(&ldestpos), core::mem::transmute_copy(&ldestlen), core::mem::transmute_copy(&ucodepage), core::mem::transmute(&pszsrc), core::mem::transmute_copy(&cchsrc), core::mem::transmute_copy(&pcchactual), core::mem::transmute_copy(&plactuallen)).into()
            }
        }
        unsafe extern "system" fn SetStrBufA<Identity: IMLangStringAStr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ldestpos: i32, ldestlen: i32, ucodepage: u32, psrcbuf: *mut core::ffi::c_void, pcchactual: *mut i32, plactuallen: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangStringAStr_Impl::SetStrBufA(this, core::mem::transmute_copy(&ldestpos), core::mem::transmute_copy(&ldestlen), core::mem::transmute_copy(&ucodepage), core::mem::transmute_copy(&psrcbuf), core::mem::transmute_copy(&pcchactual), core::mem::transmute_copy(&plactuallen)).into()
            }
        }
        unsafe extern "system" fn GetAStr<Identity: IMLangStringAStr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lsrcpos: i32, lsrclen: i32, ucodepagein: u32, pucodepageout: *const u32, pszdest: windows_core::PSTR, cchdest: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangStringAStr_Impl::GetAStr(this, core::mem::transmute_copy(&lsrcpos), core::mem::transmute_copy(&lsrclen), core::mem::transmute_copy(&ucodepagein), core::mem::transmute_copy(&pucodepageout), core::mem::transmute_copy(&pszdest), core::mem::transmute_copy(&cchdest), core::mem::transmute_copy(&pcchactual), core::mem::transmute_copy(&plactuallen)).into()
            }
        }
        unsafe extern "system" fn GetStrBufA<Identity: IMLangStringAStr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lsrcpos: i32, lsrcmaxlen: i32, pudestcodepage: *mut u32, ppdestbuf: *mut *mut core::ffi::c_void, pldestlen: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangStringAStr_Impl::GetStrBufA(this, core::mem::transmute_copy(&lsrcpos), core::mem::transmute_copy(&lsrcmaxlen), core::mem::transmute_copy(&pudestcodepage), core::mem::transmute_copy(&ppdestbuf), core::mem::transmute_copy(&pldestlen)).into()
            }
        }
        unsafe extern "system" fn LockAStr<Identity: IMLangStringAStr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lsrcpos: i32, lsrclen: i32, lflags: i32, ucodepagein: u32, cchrequest: i32, pucodepageout: *mut u32, ppszdest: *mut windows_core::PSTR, pcchdest: *mut i32, pldestlen: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangStringAStr_Impl::LockAStr(this, core::mem::transmute_copy(&lsrcpos), core::mem::transmute_copy(&lsrclen), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&ucodepagein), core::mem::transmute_copy(&cchrequest), core::mem::transmute_copy(&pucodepageout), core::mem::transmute_copy(&ppszdest), core::mem::transmute_copy(&pcchdest), core::mem::transmute_copy(&pldestlen)).into()
            }
        }
        unsafe extern "system" fn UnlockAStr<Identity: IMLangStringAStr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszsrc: windows_core::PCSTR, cchsrc: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangStringAStr_Impl::UnlockAStr(this, core::mem::transmute(&pszsrc), core::mem::transmute_copy(&cchsrc), core::mem::transmute_copy(&pcchactual), core::mem::transmute_copy(&plactuallen)).into()
            }
        }
        unsafe extern "system" fn SetLocale<Identity: IMLangStringAStr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ldestpos: i32, ldestlen: i32, locale: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangStringAStr_Impl::SetLocale(this, core::mem::transmute_copy(&ldestpos), core::mem::transmute_copy(&ldestlen), core::mem::transmute_copy(&locale)).into()
            }
        }
        unsafe extern "system" fn GetLocale<Identity: IMLangStringAStr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lsrcpos: i32, lsrcmaxlen: i32, plocale: *mut u32, pllocalepos: *mut i32, pllocalelen: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangStringAStr_Impl::GetLocale(this, core::mem::transmute_copy(&lsrcpos), core::mem::transmute_copy(&lsrcmaxlen), core::mem::transmute_copy(&plocale), core::mem::transmute_copy(&pllocalepos), core::mem::transmute_copy(&pllocalelen)).into()
            }
        }
        Self {
            base__: IMLangString_Vtbl::new::<Identity, OFFSET>(),
            SetAStr: SetAStr::<Identity, OFFSET>,
            SetStrBufA: SetStrBufA::<Identity, OFFSET>,
            GetAStr: GetAStr::<Identity, OFFSET>,
            GetStrBufA: GetStrBufA::<Identity, OFFSET>,
            LockAStr: LockAStr::<Identity, OFFSET>,
            UnlockAStr: UnlockAStr::<Identity, OFFSET>,
            SetLocale: SetLocale::<Identity, OFFSET>,
            GetLocale: GetLocale::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLangStringAStr as windows_core::Interface>::IID || iid == &<IMLangString as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMLangStringAStr {}
windows_core::imp::define_interface!(IMLangStringBufA, IMLangStringBufA_Vtbl, 0xd24acd23_ba72_11d0_b188_00aa0038c969);
windows_core::imp::interface_hierarchy!(IMLangStringBufA, windows_core::IUnknown);
impl IMLangStringBufA {
    pub unsafe fn GetStatus(&self, plflags: Option<*mut i32>, pcchbuf: Option<*mut i32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), plflags.unwrap_or(core::mem::zeroed()) as _, pcchbuf.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn LockBuf(&self, cchoffset: i32, cchmaxlock: i32, ppszbuf: *mut *mut i8, pcchbuf: Option<*mut i32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).LockBuf)(windows_core::Interface::as_raw(self), cchoffset, cchmaxlock, ppszbuf as _, pcchbuf.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn UnlockBuf<P0>(&self, pszbuf: P0, cchoffset: i32, cchwrite: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnlockBuf)(windows_core::Interface::as_raw(self), pszbuf.param().abi(), cchoffset, cchwrite).ok() }
    }
    pub unsafe fn Insert(&self, cchoffset: i32, cchmaxinsert: i32, pcchactual: Option<*mut i32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Insert)(windows_core::Interface::as_raw(self), cchoffset, cchmaxinsert, pcchactual.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn Delete(&self, cchoffset: i32, cchdelete: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), cchoffset, cchdelete).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLangStringBufA_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub LockBuf: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut i8, *mut i32) -> windows_core::HRESULT,
    pub UnlockBuf: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, i32, i32) -> windows_core::HRESULT,
    pub Insert: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut i32) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
}
pub trait IMLangStringBufA_Impl: windows_core::IUnknownImpl {
    fn GetStatus(&self, plflags: *mut i32, pcchbuf: *mut i32) -> windows_core::Result<()>;
    fn LockBuf(&self, cchoffset: i32, cchmaxlock: i32, ppszbuf: *mut *mut i8, pcchbuf: *mut i32) -> windows_core::Result<()>;
    fn UnlockBuf(&self, pszbuf: &windows_core::PCSTR, cchoffset: i32, cchwrite: i32) -> windows_core::Result<()>;
    fn Insert(&self, cchoffset: i32, cchmaxinsert: i32, pcchactual: *mut i32) -> windows_core::Result<()>;
    fn Delete(&self, cchoffset: i32, cchdelete: i32) -> windows_core::Result<()>;
}
impl IMLangStringBufA_Vtbl {
    pub const fn new<Identity: IMLangStringBufA_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStatus<Identity: IMLangStringBufA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plflags: *mut i32, pcchbuf: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangStringBufA_Impl::GetStatus(this, core::mem::transmute_copy(&plflags), core::mem::transmute_copy(&pcchbuf)).into()
            }
        }
        unsafe extern "system" fn LockBuf<Identity: IMLangStringBufA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchoffset: i32, cchmaxlock: i32, ppszbuf: *mut *mut i8, pcchbuf: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangStringBufA_Impl::LockBuf(this, core::mem::transmute_copy(&cchoffset), core::mem::transmute_copy(&cchmaxlock), core::mem::transmute_copy(&ppszbuf), core::mem::transmute_copy(&pcchbuf)).into()
            }
        }
        unsafe extern "system" fn UnlockBuf<Identity: IMLangStringBufA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszbuf: windows_core::PCSTR, cchoffset: i32, cchwrite: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangStringBufA_Impl::UnlockBuf(this, core::mem::transmute(&pszbuf), core::mem::transmute_copy(&cchoffset), core::mem::transmute_copy(&cchwrite)).into()
            }
        }
        unsafe extern "system" fn Insert<Identity: IMLangStringBufA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchoffset: i32, cchmaxinsert: i32, pcchactual: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangStringBufA_Impl::Insert(this, core::mem::transmute_copy(&cchoffset), core::mem::transmute_copy(&cchmaxinsert), core::mem::transmute_copy(&pcchactual)).into()
            }
        }
        unsafe extern "system" fn Delete<Identity: IMLangStringBufA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchoffset: i32, cchdelete: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangStringBufA_Impl::Delete(this, core::mem::transmute_copy(&cchoffset), core::mem::transmute_copy(&cchdelete)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStatus: GetStatus::<Identity, OFFSET>,
            LockBuf: LockBuf::<Identity, OFFSET>,
            UnlockBuf: UnlockBuf::<Identity, OFFSET>,
            Insert: Insert::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLangStringBufA as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMLangStringBufA {}
windows_core::imp::define_interface!(IMLangStringBufW, IMLangStringBufW_Vtbl, 0xd24acd21_ba72_11d0_b188_00aa0038c969);
windows_core::imp::interface_hierarchy!(IMLangStringBufW, windows_core::IUnknown);
impl IMLangStringBufW {
    pub unsafe fn GetStatus(&self, plflags: Option<*mut i32>, pcchbuf: Option<*mut i32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), plflags.unwrap_or(core::mem::zeroed()) as _, pcchbuf.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn LockBuf(&self, cchoffset: i32, cchmaxlock: i32, ppszbuf: *mut *mut u16, pcchbuf: Option<*mut i32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).LockBuf)(windows_core::Interface::as_raw(self), cchoffset, cchmaxlock, ppszbuf as _, pcchbuf.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn UnlockBuf<P0>(&self, pszbuf: P0, cchoffset: i32, cchwrite: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnlockBuf)(windows_core::Interface::as_raw(self), pszbuf.param().abi(), cchoffset, cchwrite).ok() }
    }
    pub unsafe fn Insert(&self, cchoffset: i32, cchmaxinsert: i32, pcchactual: Option<*mut i32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Insert)(windows_core::Interface::as_raw(self), cchoffset, cchmaxinsert, pcchactual.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn Delete(&self, cchoffset: i32, cchdelete: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), cchoffset, cchdelete).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLangStringBufW_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub LockBuf: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut u16, *mut i32) -> windows_core::HRESULT,
    pub UnlockBuf: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32, i32) -> windows_core::HRESULT,
    pub Insert: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut i32) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
}
pub trait IMLangStringBufW_Impl: windows_core::IUnknownImpl {
    fn GetStatus(&self, plflags: *mut i32, pcchbuf: *mut i32) -> windows_core::Result<()>;
    fn LockBuf(&self, cchoffset: i32, cchmaxlock: i32, ppszbuf: *mut *mut u16, pcchbuf: *mut i32) -> windows_core::Result<()>;
    fn UnlockBuf(&self, pszbuf: &windows_core::PCWSTR, cchoffset: i32, cchwrite: i32) -> windows_core::Result<()>;
    fn Insert(&self, cchoffset: i32, cchmaxinsert: i32, pcchactual: *mut i32) -> windows_core::Result<()>;
    fn Delete(&self, cchoffset: i32, cchdelete: i32) -> windows_core::Result<()>;
}
impl IMLangStringBufW_Vtbl {
    pub const fn new<Identity: IMLangStringBufW_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStatus<Identity: IMLangStringBufW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plflags: *mut i32, pcchbuf: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangStringBufW_Impl::GetStatus(this, core::mem::transmute_copy(&plflags), core::mem::transmute_copy(&pcchbuf)).into()
            }
        }
        unsafe extern "system" fn LockBuf<Identity: IMLangStringBufW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchoffset: i32, cchmaxlock: i32, ppszbuf: *mut *mut u16, pcchbuf: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangStringBufW_Impl::LockBuf(this, core::mem::transmute_copy(&cchoffset), core::mem::transmute_copy(&cchmaxlock), core::mem::transmute_copy(&ppszbuf), core::mem::transmute_copy(&pcchbuf)).into()
            }
        }
        unsafe extern "system" fn UnlockBuf<Identity: IMLangStringBufW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszbuf: windows_core::PCWSTR, cchoffset: i32, cchwrite: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangStringBufW_Impl::UnlockBuf(this, core::mem::transmute(&pszbuf), core::mem::transmute_copy(&cchoffset), core::mem::transmute_copy(&cchwrite)).into()
            }
        }
        unsafe extern "system" fn Insert<Identity: IMLangStringBufW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchoffset: i32, cchmaxinsert: i32, pcchactual: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangStringBufW_Impl::Insert(this, core::mem::transmute_copy(&cchoffset), core::mem::transmute_copy(&cchmaxinsert), core::mem::transmute_copy(&pcchactual)).into()
            }
        }
        unsafe extern "system" fn Delete<Identity: IMLangStringBufW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchoffset: i32, cchdelete: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangStringBufW_Impl::Delete(this, core::mem::transmute_copy(&cchoffset), core::mem::transmute_copy(&cchdelete)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStatus: GetStatus::<Identity, OFFSET>,
            LockBuf: LockBuf::<Identity, OFFSET>,
            UnlockBuf: UnlockBuf::<Identity, OFFSET>,
            Insert: Insert::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLangStringBufW as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMLangStringBufW {}
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
        unsafe { (windows_core::Interface::vtable(self).SetWStr)(windows_core::Interface::as_raw(self), ldestpos, ldestlen, core::mem::transmute(pszsrc.as_ptr()), pszsrc.len().try_into().unwrap(), pcchactual.unwrap_or(core::mem::zeroed()) as _, plactuallen.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn SetStrBufW<P2>(&self, ldestpos: i32, ldestlen: i32, psrcbuf: P2, pcchactual: Option<*mut i32>, plactuallen: Option<*mut i32>) -> windows_core::Result<()>
    where
        P2: windows_core::Param<IMLangStringBufW>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetStrBufW)(windows_core::Interface::as_raw(self), ldestpos, ldestlen, psrcbuf.param().abi(), pcchactual.unwrap_or(core::mem::zeroed()) as _, plactuallen.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn GetWStr(&self, lsrcpos: i32, lsrclen: i32, pszdest: Option<&mut [u16]>, pcchactual: Option<*mut i32>, plactuallen: Option<*mut i32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetWStr)(windows_core::Interface::as_raw(self), lsrcpos, lsrclen, core::mem::transmute(pszdest.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pszdest.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcchactual.unwrap_or(core::mem::zeroed()) as _, plactuallen.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn GetStrBufW(&self, lsrcpos: i32, lsrcmaxlen: i32, ppdestbuf: *mut Option<IMLangStringBufW>, pldestlen: Option<*mut i32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetStrBufW)(windows_core::Interface::as_raw(self), lsrcpos, lsrcmaxlen, core::mem::transmute(ppdestbuf), pldestlen.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn LockWStr(&self, lsrcpos: i32, lsrclen: i32, lflags: i32, cchrequest: i32, ppszdest: Option<*mut windows_core::PWSTR>, pcchdest: Option<*mut i32>, pldestlen: Option<*mut i32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).LockWStr)(windows_core::Interface::as_raw(self), lsrcpos, lsrclen, lflags, cchrequest, ppszdest.unwrap_or(core::mem::zeroed()) as _, pcchdest.unwrap_or(core::mem::zeroed()) as _, pldestlen.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn UnlockWStr(&self, pszsrc: &[u16], pcchactual: Option<*mut i32>, plactuallen: Option<*mut i32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UnlockWStr)(windows_core::Interface::as_raw(self), core::mem::transmute(pszsrc.as_ptr()), pszsrc.len().try_into().unwrap(), pcchactual.unwrap_or(core::mem::zeroed()) as _, plactuallen.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn SetLocale(&self, ldestpos: i32, ldestlen: i32, locale: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLocale)(windows_core::Interface::as_raw(self), ldestpos, ldestlen, locale).ok() }
    }
    pub unsafe fn GetLocale(&self, lsrcpos: i32, lsrcmaxlen: i32, plocale: Option<*mut u32>, pllocalepos: Option<*mut i32>, pllocalelen: Option<*mut i32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetLocale)(windows_core::Interface::as_raw(self), lsrcpos, lsrcmaxlen, plocale.unwrap_or(core::mem::zeroed()) as _, pllocalepos.unwrap_or(core::mem::zeroed()) as _, pllocalelen.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
pub trait IMLangStringWStr_Impl: IMLangString_Impl {
    fn SetWStr(&self, ldestpos: i32, ldestlen: i32, pszsrc: &windows_core::PCWSTR, cchsrc: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> windows_core::Result<()>;
    fn SetStrBufW(&self, ldestpos: i32, ldestlen: i32, psrcbuf: windows_core::Ref<IMLangStringBufW>, pcchactual: *mut i32, plactuallen: *mut i32) -> windows_core::Result<()>;
    fn GetWStr(&self, lsrcpos: i32, lsrclen: i32, pszdest: windows_core::PWSTR, cchdest: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> windows_core::Result<()>;
    fn GetStrBufW(&self, lsrcpos: i32, lsrcmaxlen: i32, ppdestbuf: windows_core::OutRef<IMLangStringBufW>, pldestlen: *mut i32) -> windows_core::Result<()>;
    fn LockWStr(&self, lsrcpos: i32, lsrclen: i32, lflags: i32, cchrequest: i32, ppszdest: *mut windows_core::PWSTR, pcchdest: *mut i32, pldestlen: *mut i32) -> windows_core::Result<()>;
    fn UnlockWStr(&self, pszsrc: &windows_core::PCWSTR, cchsrc: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> windows_core::Result<()>;
    fn SetLocale(&self, ldestpos: i32, ldestlen: i32, locale: u32) -> windows_core::Result<()>;
    fn GetLocale(&self, lsrcpos: i32, lsrcmaxlen: i32, plocale: *mut u32, pllocalepos: *mut i32, pllocalelen: *mut i32) -> windows_core::Result<()>;
}
impl IMLangStringWStr_Vtbl {
    pub const fn new<Identity: IMLangStringWStr_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetWStr<Identity: IMLangStringWStr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ldestpos: i32, ldestlen: i32, pszsrc: windows_core::PCWSTR, cchsrc: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangStringWStr_Impl::SetWStr(this, core::mem::transmute_copy(&ldestpos), core::mem::transmute_copy(&ldestlen), core::mem::transmute(&pszsrc), core::mem::transmute_copy(&cchsrc), core::mem::transmute_copy(&pcchactual), core::mem::transmute_copy(&plactuallen)).into()
            }
        }
        unsafe extern "system" fn SetStrBufW<Identity: IMLangStringWStr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ldestpos: i32, ldestlen: i32, psrcbuf: *mut core::ffi::c_void, pcchactual: *mut i32, plactuallen: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangStringWStr_Impl::SetStrBufW(this, core::mem::transmute_copy(&ldestpos), core::mem::transmute_copy(&ldestlen), core::mem::transmute_copy(&psrcbuf), core::mem::transmute_copy(&pcchactual), core::mem::transmute_copy(&plactuallen)).into()
            }
        }
        unsafe extern "system" fn GetWStr<Identity: IMLangStringWStr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lsrcpos: i32, lsrclen: i32, pszdest: windows_core::PWSTR, cchdest: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangStringWStr_Impl::GetWStr(this, core::mem::transmute_copy(&lsrcpos), core::mem::transmute_copy(&lsrclen), core::mem::transmute_copy(&pszdest), core::mem::transmute_copy(&cchdest), core::mem::transmute_copy(&pcchactual), core::mem::transmute_copy(&plactuallen)).into()
            }
        }
        unsafe extern "system" fn GetStrBufW<Identity: IMLangStringWStr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lsrcpos: i32, lsrcmaxlen: i32, ppdestbuf: *mut *mut core::ffi::c_void, pldestlen: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangStringWStr_Impl::GetStrBufW(this, core::mem::transmute_copy(&lsrcpos), core::mem::transmute_copy(&lsrcmaxlen), core::mem::transmute_copy(&ppdestbuf), core::mem::transmute_copy(&pldestlen)).into()
            }
        }
        unsafe extern "system" fn LockWStr<Identity: IMLangStringWStr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lsrcpos: i32, lsrclen: i32, lflags: i32, cchrequest: i32, ppszdest: *mut windows_core::PWSTR, pcchdest: *mut i32, pldestlen: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangStringWStr_Impl::LockWStr(this, core::mem::transmute_copy(&lsrcpos), core::mem::transmute_copy(&lsrclen), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&cchrequest), core::mem::transmute_copy(&ppszdest), core::mem::transmute_copy(&pcchdest), core::mem::transmute_copy(&pldestlen)).into()
            }
        }
        unsafe extern "system" fn UnlockWStr<Identity: IMLangStringWStr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszsrc: windows_core::PCWSTR, cchsrc: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangStringWStr_Impl::UnlockWStr(this, core::mem::transmute(&pszsrc), core::mem::transmute_copy(&cchsrc), core::mem::transmute_copy(&pcchactual), core::mem::transmute_copy(&plactuallen)).into()
            }
        }
        unsafe extern "system" fn SetLocale<Identity: IMLangStringWStr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ldestpos: i32, ldestlen: i32, locale: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangStringWStr_Impl::SetLocale(this, core::mem::transmute_copy(&ldestpos), core::mem::transmute_copy(&ldestlen), core::mem::transmute_copy(&locale)).into()
            }
        }
        unsafe extern "system" fn GetLocale<Identity: IMLangStringWStr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lsrcpos: i32, lsrcmaxlen: i32, plocale: *mut u32, pllocalepos: *mut i32, pllocalelen: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangStringWStr_Impl::GetLocale(this, core::mem::transmute_copy(&lsrcpos), core::mem::transmute_copy(&lsrcmaxlen), core::mem::transmute_copy(&plocale), core::mem::transmute_copy(&pllocalepos), core::mem::transmute_copy(&pllocalelen)).into()
            }
        }
        Self {
            base__: IMLangString_Vtbl::new::<Identity, OFFSET>(),
            SetWStr: SetWStr::<Identity, OFFSET>,
            SetStrBufW: SetStrBufW::<Identity, OFFSET>,
            GetWStr: GetWStr::<Identity, OFFSET>,
            GetStrBufW: GetStrBufW::<Identity, OFFSET>,
            LockWStr: LockWStr::<Identity, OFFSET>,
            UnlockWStr: UnlockWStr::<Identity, OFFSET>,
            SetLocale: SetLocale::<Identity, OFFSET>,
            GetLocale: GetLocale::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLangStringWStr as windows_core::Interface>::IID || iid == &<IMLangString as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMLangStringWStr {}
windows_core::imp::define_interface!(IMultiLanguage, IMultiLanguage_Vtbl, 0x275c23e1_3747_11d0_9fea_00aa003f8646);
windows_core::imp::interface_hierarchy!(IMultiLanguage, windows_core::IUnknown);
impl IMultiLanguage {
    pub unsafe fn GetNumberOfCodePageInfo(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNumberOfCodePageInfo)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCodePageInfo(&self, uicodepage: u32, pcodepageinfo: *mut MIMECPINFO) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCodePageInfo)(windows_core::Interface::as_raw(self), uicodepage, pcodepageinfo as _).ok() }
    }
    pub unsafe fn GetFamilyCodePage(&self, uicodepage: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFamilyCodePage)(windows_core::Interface::as_raw(self), uicodepage, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnumCodePages(&self, grfflags: u32) -> windows_core::Result<IEnumCodePage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumCodePages)(windows_core::Interface::as_raw(self), grfflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCharsetInfo(&self, charset: &windows_core::BSTR, pcharsetinfo: *mut MIMECSETINFO) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCharsetInfo)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(charset), pcharsetinfo as _).ok() }
    }
    pub unsafe fn IsConvertible(&self, dwsrcencoding: u32, dwdstencoding: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).IsConvertible)(windows_core::Interface::as_raw(self), dwsrcencoding, dwdstencoding).ok() }
    }
    pub unsafe fn ConvertString(&self, pdwmode: Option<*mut u32>, dwsrcencoding: u32, dwdstencoding: u32, psrcstr: Option<*const u8>, pcsrcsize: Option<*mut u32>, pdststr: Option<*mut u8>, pcdstsize: Option<*mut u32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ConvertString)(windows_core::Interface::as_raw(self), pdwmode.unwrap_or(core::mem::zeroed()) as _, dwsrcencoding, dwdstencoding, psrcstr.unwrap_or(core::mem::zeroed()) as _, pcsrcsize.unwrap_or(core::mem::zeroed()) as _, pdststr.unwrap_or(core::mem::zeroed()) as _, pcdstsize.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn ConvertStringToUnicode<P2>(&self, pdwmode: Option<*mut u32>, dwencoding: u32, psrcstr: P2, pcsrcsize: Option<*mut u32>, pdststr: Option<windows_core::PWSTR>, pcdstsize: Option<*mut u32>) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ConvertStringToUnicode)(windows_core::Interface::as_raw(self), pdwmode.unwrap_or(core::mem::zeroed()) as _, dwencoding, psrcstr.param().abi(), pcsrcsize.unwrap_or(core::mem::zeroed()) as _, pdststr.unwrap_or(core::mem::zeroed()) as _, pcdstsize.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn ConvertStringFromUnicode<P2>(&self, pdwmode: Option<*mut u32>, dwencoding: u32, psrcstr: P2, pcsrcsize: Option<*mut u32>, pdststr: Option<windows_core::PSTR>, pcdstsize: Option<*mut u32>) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ConvertStringFromUnicode)(windows_core::Interface::as_raw(self), pdwmode.unwrap_or(core::mem::zeroed()) as _, dwencoding, psrcstr.param().abi(), pcsrcsize.unwrap_or(core::mem::zeroed()) as _, pdststr.unwrap_or(core::mem::zeroed()) as _, pcdstsize.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn ConvertStringReset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ConvertStringReset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetRfc1766FromLcid(&self, locale: u32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRfc1766FromLcid)(windows_core::Interface::as_raw(self), locale, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetLcidFromRfc1766(&self, plocale: *mut u32, bstrrfc1766: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetLcidFromRfc1766)(windows_core::Interface::as_raw(self), plocale as _, core::mem::transmute_copy(bstrrfc1766)).ok() }
    }
    pub unsafe fn EnumRfc1766(&self) -> windows_core::Result<IEnumRfc1766> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumRfc1766)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetRfc1766Info(&self, locale: u32, prfc1766info: *mut RFC1766INFO) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetRfc1766Info)(windows_core::Interface::as_raw(self), locale, prfc1766info as _).ok() }
    }
    pub unsafe fn CreateConvertCharset(&self, uisrccodepage: u32, uidstcodepage: u32, dwproperty: u32) -> windows_core::Result<IMLangConvertCharset> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateConvertCharset)(windows_core::Interface::as_raw(self), uisrccodepage, uidstcodepage, dwproperty, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultiLanguage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetNumberOfCodePageInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetCodePageInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut MIMECPINFO) -> windows_core::HRESULT,
    pub GetFamilyCodePage: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub EnumCodePages: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCharsetInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut MIMECSETINFO) -> windows_core::HRESULT,
    pub IsConvertible: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub ConvertString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, u32, u32, *const u8, *mut u32, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub ConvertStringToUnicode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, u32, windows_core::PCSTR, *mut u32, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub ConvertStringFromUnicode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, u32, windows_core::PCWSTR, *mut u32, windows_core::PSTR, *mut u32) -> windows_core::HRESULT,
    pub ConvertStringReset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRfc1766FromLcid: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLcidFromRfc1766: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumRfc1766: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRfc1766Info: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut RFC1766INFO) -> windows_core::HRESULT,
    pub CreateConvertCharset: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMultiLanguage_Impl: windows_core::IUnknownImpl {
    fn GetNumberOfCodePageInfo(&self) -> windows_core::Result<u32>;
    fn GetCodePageInfo(&self, uicodepage: u32, pcodepageinfo: *mut MIMECPINFO) -> windows_core::Result<()>;
    fn GetFamilyCodePage(&self, uicodepage: u32) -> windows_core::Result<u32>;
    fn EnumCodePages(&self, grfflags: u32) -> windows_core::Result<IEnumCodePage>;
    fn GetCharsetInfo(&self, charset: &windows_core::BSTR, pcharsetinfo: *mut MIMECSETINFO) -> windows_core::Result<()>;
    fn IsConvertible(&self, dwsrcencoding: u32, dwdstencoding: u32) -> windows_core::Result<()>;
    fn ConvertString(&self, pdwmode: *mut u32, dwsrcencoding: u32, dwdstencoding: u32, psrcstr: *const u8, pcsrcsize: *mut u32, pdststr: *mut u8, pcdstsize: *mut u32) -> windows_core::Result<()>;
    fn ConvertStringToUnicode(&self, pdwmode: *mut u32, dwencoding: u32, psrcstr: &windows_core::PCSTR, pcsrcsize: *mut u32, pdststr: windows_core::PWSTR, pcdstsize: *mut u32) -> windows_core::Result<()>;
    fn ConvertStringFromUnicode(&self, pdwmode: *mut u32, dwencoding: u32, psrcstr: &windows_core::PCWSTR, pcsrcsize: *mut u32, pdststr: windows_core::PSTR, pcdstsize: *mut u32) -> windows_core::Result<()>;
    fn ConvertStringReset(&self) -> windows_core::Result<()>;
    fn GetRfc1766FromLcid(&self, locale: u32) -> windows_core::Result<windows_core::BSTR>;
    fn GetLcidFromRfc1766(&self, plocale: *mut u32, bstrrfc1766: &windows_core::BSTR) -> windows_core::Result<()>;
    fn EnumRfc1766(&self) -> windows_core::Result<IEnumRfc1766>;
    fn GetRfc1766Info(&self, locale: u32, prfc1766info: *mut RFC1766INFO) -> windows_core::Result<()>;
    fn CreateConvertCharset(&self, uisrccodepage: u32, uidstcodepage: u32, dwproperty: u32) -> windows_core::Result<IMLangConvertCharset>;
}
impl IMultiLanguage_Vtbl {
    pub const fn new<Identity: IMultiLanguage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetNumberOfCodePageInfo<Identity: IMultiLanguage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pccodepage: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultiLanguage_Impl::GetNumberOfCodePageInfo(this) {
                    Ok(ok__) => {
                        pccodepage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCodePageInfo<Identity: IMultiLanguage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uicodepage: u32, pcodepageinfo: *mut MIMECPINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage_Impl::GetCodePageInfo(this, core::mem::transmute_copy(&uicodepage), core::mem::transmute_copy(&pcodepageinfo)).into()
            }
        }
        unsafe extern "system" fn GetFamilyCodePage<Identity: IMultiLanguage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uicodepage: u32, puifamilycodepage: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultiLanguage_Impl::GetFamilyCodePage(this, core::mem::transmute_copy(&uicodepage)) {
                    Ok(ok__) => {
                        puifamilycodepage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumCodePages<Identity: IMultiLanguage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfflags: u32, ppenumcodepage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultiLanguage_Impl::EnumCodePages(this, core::mem::transmute_copy(&grfflags)) {
                    Ok(ok__) => {
                        ppenumcodepage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCharsetInfo<Identity: IMultiLanguage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, charset: *mut core::ffi::c_void, pcharsetinfo: *mut MIMECSETINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage_Impl::GetCharsetInfo(this, core::mem::transmute(&charset), core::mem::transmute_copy(&pcharsetinfo)).into()
            }
        }
        unsafe extern "system" fn IsConvertible<Identity: IMultiLanguage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsrcencoding: u32, dwdstencoding: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage_Impl::IsConvertible(this, core::mem::transmute_copy(&dwsrcencoding), core::mem::transmute_copy(&dwdstencoding)).into()
            }
        }
        unsafe extern "system" fn ConvertString<Identity: IMultiLanguage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmode: *mut u32, dwsrcencoding: u32, dwdstencoding: u32, psrcstr: *const u8, pcsrcsize: *mut u32, pdststr: *mut u8, pcdstsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage_Impl::ConvertString(this, core::mem::transmute_copy(&pdwmode), core::mem::transmute_copy(&dwsrcencoding), core::mem::transmute_copy(&dwdstencoding), core::mem::transmute_copy(&psrcstr), core::mem::transmute_copy(&pcsrcsize), core::mem::transmute_copy(&pdststr), core::mem::transmute_copy(&pcdstsize)).into()
            }
        }
        unsafe extern "system" fn ConvertStringToUnicode<Identity: IMultiLanguage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmode: *mut u32, dwencoding: u32, psrcstr: windows_core::PCSTR, pcsrcsize: *mut u32, pdststr: windows_core::PWSTR, pcdstsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage_Impl::ConvertStringToUnicode(this, core::mem::transmute_copy(&pdwmode), core::mem::transmute_copy(&dwencoding), core::mem::transmute(&psrcstr), core::mem::transmute_copy(&pcsrcsize), core::mem::transmute_copy(&pdststr), core::mem::transmute_copy(&pcdstsize)).into()
            }
        }
        unsafe extern "system" fn ConvertStringFromUnicode<Identity: IMultiLanguage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmode: *mut u32, dwencoding: u32, psrcstr: windows_core::PCWSTR, pcsrcsize: *mut u32, pdststr: windows_core::PSTR, pcdstsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage_Impl::ConvertStringFromUnicode(this, core::mem::transmute_copy(&pdwmode), core::mem::transmute_copy(&dwencoding), core::mem::transmute(&psrcstr), core::mem::transmute_copy(&pcsrcsize), core::mem::transmute_copy(&pdststr), core::mem::transmute_copy(&pcdstsize)).into()
            }
        }
        unsafe extern "system" fn ConvertStringReset<Identity: IMultiLanguage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage_Impl::ConvertStringReset(this).into()
            }
        }
        unsafe extern "system" fn GetRfc1766FromLcid<Identity: IMultiLanguage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, locale: u32, pbstrrfc1766: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultiLanguage_Impl::GetRfc1766FromLcid(this, core::mem::transmute_copy(&locale)) {
                    Ok(ok__) => {
                        pbstrrfc1766.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLcidFromRfc1766<Identity: IMultiLanguage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plocale: *mut u32, bstrrfc1766: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage_Impl::GetLcidFromRfc1766(this, core::mem::transmute_copy(&plocale), core::mem::transmute(&bstrrfc1766)).into()
            }
        }
        unsafe extern "system" fn EnumRfc1766<Identity: IMultiLanguage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumrfc1766: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultiLanguage_Impl::EnumRfc1766(this) {
                    Ok(ok__) => {
                        ppenumrfc1766.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRfc1766Info<Identity: IMultiLanguage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, locale: u32, prfc1766info: *mut RFC1766INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage_Impl::GetRfc1766Info(this, core::mem::transmute_copy(&locale), core::mem::transmute_copy(&prfc1766info)).into()
            }
        }
        unsafe extern "system" fn CreateConvertCharset<Identity: IMultiLanguage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uisrccodepage: u32, uidstcodepage: u32, dwproperty: u32, ppmlangconvertcharset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultiLanguage_Impl::CreateConvertCharset(this, core::mem::transmute_copy(&uisrccodepage), core::mem::transmute_copy(&uidstcodepage), core::mem::transmute_copy(&dwproperty)) {
                    Ok(ok__) => {
                        ppmlangconvertcharset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetNumberOfCodePageInfo: GetNumberOfCodePageInfo::<Identity, OFFSET>,
            GetCodePageInfo: GetCodePageInfo::<Identity, OFFSET>,
            GetFamilyCodePage: GetFamilyCodePage::<Identity, OFFSET>,
            EnumCodePages: EnumCodePages::<Identity, OFFSET>,
            GetCharsetInfo: GetCharsetInfo::<Identity, OFFSET>,
            IsConvertible: IsConvertible::<Identity, OFFSET>,
            ConvertString: ConvertString::<Identity, OFFSET>,
            ConvertStringToUnicode: ConvertStringToUnicode::<Identity, OFFSET>,
            ConvertStringFromUnicode: ConvertStringFromUnicode::<Identity, OFFSET>,
            ConvertStringReset: ConvertStringReset::<Identity, OFFSET>,
            GetRfc1766FromLcid: GetRfc1766FromLcid::<Identity, OFFSET>,
            GetLcidFromRfc1766: GetLcidFromRfc1766::<Identity, OFFSET>,
            EnumRfc1766: EnumRfc1766::<Identity, OFFSET>,
            GetRfc1766Info: GetRfc1766Info::<Identity, OFFSET>,
            CreateConvertCharset: CreateConvertCharset::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMultiLanguage as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMultiLanguage {}
windows_core::imp::define_interface!(IMultiLanguage2, IMultiLanguage2_Vtbl, 0xdccfc164_2b38_11d2_b7ec_00c04f8f5d9a);
windows_core::imp::interface_hierarchy!(IMultiLanguage2, windows_core::IUnknown);
impl IMultiLanguage2 {
    pub unsafe fn GetNumberOfCodePageInfo(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNumberOfCodePageInfo)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCodePageInfo(&self, uicodepage: u32, langid: u16, pcodepageinfo: *mut MIMECPINFO) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCodePageInfo)(windows_core::Interface::as_raw(self), uicodepage, langid, pcodepageinfo as _).ok() }
    }
    pub unsafe fn GetFamilyCodePage(&self, uicodepage: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFamilyCodePage)(windows_core::Interface::as_raw(self), uicodepage, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnumCodePages(&self, grfflags: u32, langid: u16) -> windows_core::Result<IEnumCodePage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumCodePages)(windows_core::Interface::as_raw(self), grfflags, langid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCharsetInfo(&self, charset: &windows_core::BSTR, pcharsetinfo: *mut MIMECSETINFO) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCharsetInfo)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(charset), pcharsetinfo as _).ok() }
    }
    pub unsafe fn IsConvertible(&self, dwsrcencoding: u32, dwdstencoding: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).IsConvertible)(windows_core::Interface::as_raw(self), dwsrcencoding, dwdstencoding).ok() }
    }
    pub unsafe fn ConvertString(&self, pdwmode: Option<*mut u32>, dwsrcencoding: u32, dwdstencoding: u32, psrcstr: Option<*const u8>, pcsrcsize: Option<*mut u32>, pdststr: Option<*mut u8>, pcdstsize: Option<*mut u32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ConvertString)(windows_core::Interface::as_raw(self), pdwmode.unwrap_or(core::mem::zeroed()) as _, dwsrcencoding, dwdstencoding, psrcstr.unwrap_or(core::mem::zeroed()) as _, pcsrcsize.unwrap_or(core::mem::zeroed()) as _, pdststr.unwrap_or(core::mem::zeroed()) as _, pcdstsize.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn ConvertStringToUnicode<P2>(&self, pdwmode: Option<*mut u32>, dwencoding: u32, psrcstr: P2, pcsrcsize: Option<*mut u32>, pdststr: Option<windows_core::PWSTR>, pcdstsize: Option<*mut u32>) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ConvertStringToUnicode)(windows_core::Interface::as_raw(self), pdwmode.unwrap_or(core::mem::zeroed()) as _, dwencoding, psrcstr.param().abi(), pcsrcsize.unwrap_or(core::mem::zeroed()) as _, pdststr.unwrap_or(core::mem::zeroed()) as _, pcdstsize.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn ConvertStringFromUnicode<P2>(&self, pdwmode: Option<*mut u32>, dwencoding: u32, psrcstr: P2, pcsrcsize: Option<*mut u32>, pdststr: Option<windows_core::PSTR>, pcdstsize: Option<*mut u32>) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ConvertStringFromUnicode)(windows_core::Interface::as_raw(self), pdwmode.unwrap_or(core::mem::zeroed()) as _, dwencoding, psrcstr.param().abi(), pcsrcsize.unwrap_or(core::mem::zeroed()) as _, pdststr.unwrap_or(core::mem::zeroed()) as _, pcdstsize.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn ConvertStringReset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ConvertStringReset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetRfc1766FromLcid(&self, locale: u32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRfc1766FromLcid)(windows_core::Interface::as_raw(self), locale, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetLcidFromRfc1766(&self, plocale: *mut u32, bstrrfc1766: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetLcidFromRfc1766)(windows_core::Interface::as_raw(self), plocale as _, core::mem::transmute_copy(bstrrfc1766)).ok() }
    }
    pub unsafe fn EnumRfc1766(&self, langid: u16) -> windows_core::Result<IEnumRfc1766> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumRfc1766)(windows_core::Interface::as_raw(self), langid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetRfc1766Info(&self, locale: u32, langid: u16, prfc1766info: *mut RFC1766INFO) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetRfc1766Info)(windows_core::Interface::as_raw(self), locale, langid, prfc1766info as _).ok() }
    }
    pub unsafe fn CreateConvertCharset(&self, uisrccodepage: u32, uidstcodepage: u32, dwproperty: u32) -> windows_core::Result<IMLangConvertCharset> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateConvertCharset)(windows_core::Interface::as_raw(self), uisrccodepage, uidstcodepage, dwproperty, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ConvertStringInIStream<P2, P5, P6>(&self, pdwmode: Option<*mut u32>, dwflag: u32, lpfallback: P2, dwsrcencoding: u32, dwdstencoding: u32, pstmin: P5, pstmout: P6) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<super::System::Com::IStream>,
        P6: windows_core::Param<super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).ConvertStringInIStream)(windows_core::Interface::as_raw(self), pdwmode.unwrap_or(core::mem::zeroed()) as _, dwflag, lpfallback.param().abi(), dwsrcencoding, dwdstencoding, pstmin.param().abi(), pstmout.param().abi()).ok() }
    }
    pub unsafe fn ConvertStringToUnicodeEx<P2, P7>(&self, pdwmode: Option<*mut u32>, dwencoding: u32, psrcstr: P2, pcsrcsize: Option<*mut u32>, pdststr: windows_core::PWSTR, pcdstsize: Option<*mut u32>, dwflag: u32, lpfallback: P7) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCSTR>,
        P7: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ConvertStringToUnicodeEx)(windows_core::Interface::as_raw(self), pdwmode.unwrap_or(core::mem::zeroed()) as _, dwencoding, psrcstr.param().abi(), pcsrcsize.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pdststr), pcdstsize.unwrap_or(core::mem::zeroed()) as _, dwflag, lpfallback.param().abi()).ok() }
    }
    pub unsafe fn ConvertStringFromUnicodeEx<P2, P7>(&self, pdwmode: Option<*mut u32>, dwencoding: u32, psrcstr: P2, pcsrcsize: Option<*mut u32>, pdststr: windows_core::PSTR, pcdstsize: Option<*mut u32>, dwflag: u32, lpfallback: P7) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P7: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ConvertStringFromUnicodeEx)(windows_core::Interface::as_raw(self), pdwmode.unwrap_or(core::mem::zeroed()) as _, dwencoding, psrcstr.param().abi(), pcsrcsize.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pdststr), pcdstsize.unwrap_or(core::mem::zeroed()) as _, dwflag, lpfallback.param().abi()).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DetectCodepageInIStream<P2>(&self, dwflag: u32, dwprefwincodepage: u32, pstmin: P2, lpencoding: *mut DetectEncodingInfo, pnscores: *mut i32) -> windows_core::Result<()>
    where
        P2: windows_core::Param<super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).DetectCodepageInIStream)(windows_core::Interface::as_raw(self), dwflag, dwprefwincodepage, pstmin.param().abi(), lpencoding as _, pnscores as _).ok() }
    }
    pub unsafe fn DetectInputCodepage<P2>(&self, dwflag: u32, dwprefwincodepage: u32, psrcstr: P2, pcsrcsize: *mut i32, lpencoding: *mut DetectEncodingInfo, pnscores: *mut i32) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).DetectInputCodepage)(windows_core::Interface::as_raw(self), dwflag, dwprefwincodepage, psrcstr.param().abi(), pcsrcsize as _, lpencoding as _, pnscores as _).ok() }
    }
    pub unsafe fn ValidateCodePage(&self, uicodepage: u32, hwnd: super::Foundation::HWND) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ValidateCodePage)(windows_core::Interface::as_raw(self), uicodepage, hwnd).ok() }
    }
    pub unsafe fn GetCodePageDescription(&self, uicodepage: u32, lcid: u32, lpwidecharstr: &mut [u16]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCodePageDescription)(windows_core::Interface::as_raw(self), uicodepage, lcid, core::mem::transmute(lpwidecharstr.as_ptr()), lpwidecharstr.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn IsCodePageInstallable(&self, uicodepage: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).IsCodePageInstallable)(windows_core::Interface::as_raw(self), uicodepage).ok() }
    }
    pub unsafe fn SetMimeDBSource(&self, dwsource: MIMECONTF) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMimeDBSource)(windows_core::Interface::as_raw(self), dwsource).ok() }
    }
    pub unsafe fn GetNumberOfScripts(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNumberOfScripts)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnumScripts(&self, dwflags: u32, langid: u16) -> windows_core::Result<IEnumScript> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumScripts)(windows_core::Interface::as_raw(self), dwflags, langid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ValidateCodePageEx(&self, uicodepage: u32, hwnd: super::Foundation::HWND, dwfiodcontrol: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ValidateCodePageEx)(windows_core::Interface::as_raw(self), uicodepage, hwnd, dwfiodcontrol).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultiLanguage2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetNumberOfCodePageInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetCodePageInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u16, *mut MIMECPINFO) -> windows_core::HRESULT,
    pub GetFamilyCodePage: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub EnumCodePages: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCharsetInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut MIMECSETINFO) -> windows_core::HRESULT,
    pub IsConvertible: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub ConvertString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, u32, u32, *const u8, *mut u32, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub ConvertStringToUnicode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, u32, windows_core::PCSTR, *mut u32, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub ConvertStringFromUnicode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, u32, windows_core::PCWSTR, *mut u32, windows_core::PSTR, *mut u32) -> windows_core::HRESULT,
    pub ConvertStringReset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRfc1766FromLcid: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLcidFromRfc1766: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
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
#[cfg(feature = "Win32_System_Com")]
pub trait IMultiLanguage2_Impl: windows_core::IUnknownImpl {
    fn GetNumberOfCodePageInfo(&self) -> windows_core::Result<u32>;
    fn GetCodePageInfo(&self, uicodepage: u32, langid: u16, pcodepageinfo: *mut MIMECPINFO) -> windows_core::Result<()>;
    fn GetFamilyCodePage(&self, uicodepage: u32) -> windows_core::Result<u32>;
    fn EnumCodePages(&self, grfflags: u32, langid: u16) -> windows_core::Result<IEnumCodePage>;
    fn GetCharsetInfo(&self, charset: &windows_core::BSTR, pcharsetinfo: *mut MIMECSETINFO) -> windows_core::Result<()>;
    fn IsConvertible(&self, dwsrcencoding: u32, dwdstencoding: u32) -> windows_core::Result<()>;
    fn ConvertString(&self, pdwmode: *mut u32, dwsrcencoding: u32, dwdstencoding: u32, psrcstr: *const u8, pcsrcsize: *mut u32, pdststr: *mut u8, pcdstsize: *mut u32) -> windows_core::Result<()>;
    fn ConvertStringToUnicode(&self, pdwmode: *mut u32, dwencoding: u32, psrcstr: &windows_core::PCSTR, pcsrcsize: *mut u32, pdststr: windows_core::PWSTR, pcdstsize: *mut u32) -> windows_core::Result<()>;
    fn ConvertStringFromUnicode(&self, pdwmode: *mut u32, dwencoding: u32, psrcstr: &windows_core::PCWSTR, pcsrcsize: *mut u32, pdststr: windows_core::PSTR, pcdstsize: *mut u32) -> windows_core::Result<()>;
    fn ConvertStringReset(&self) -> windows_core::Result<()>;
    fn GetRfc1766FromLcid(&self, locale: u32) -> windows_core::Result<windows_core::BSTR>;
    fn GetLcidFromRfc1766(&self, plocale: *mut u32, bstrrfc1766: &windows_core::BSTR) -> windows_core::Result<()>;
    fn EnumRfc1766(&self, langid: u16) -> windows_core::Result<IEnumRfc1766>;
    fn GetRfc1766Info(&self, locale: u32, langid: u16, prfc1766info: *mut RFC1766INFO) -> windows_core::Result<()>;
    fn CreateConvertCharset(&self, uisrccodepage: u32, uidstcodepage: u32, dwproperty: u32) -> windows_core::Result<IMLangConvertCharset>;
    fn ConvertStringInIStream(&self, pdwmode: *mut u32, dwflag: u32, lpfallback: &windows_core::PCWSTR, dwsrcencoding: u32, dwdstencoding: u32, pstmin: windows_core::Ref<super::System::Com::IStream>, pstmout: windows_core::Ref<super::System::Com::IStream>) -> windows_core::Result<()>;
    fn ConvertStringToUnicodeEx(&self, pdwmode: *mut u32, dwencoding: u32, psrcstr: &windows_core::PCSTR, pcsrcsize: *mut u32, pdststr: windows_core::PWSTR, pcdstsize: *mut u32, dwflag: u32, lpfallback: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn ConvertStringFromUnicodeEx(&self, pdwmode: *mut u32, dwencoding: u32, psrcstr: &windows_core::PCWSTR, pcsrcsize: *mut u32, pdststr: windows_core::PSTR, pcdstsize: *mut u32, dwflag: u32, lpfallback: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn DetectCodepageInIStream(&self, dwflag: u32, dwprefwincodepage: u32, pstmin: windows_core::Ref<super::System::Com::IStream>, lpencoding: *mut DetectEncodingInfo, pnscores: *mut i32) -> windows_core::Result<()>;
    fn DetectInputCodepage(&self, dwflag: u32, dwprefwincodepage: u32, psrcstr: &windows_core::PCSTR, pcsrcsize: *mut i32, lpencoding: *mut DetectEncodingInfo, pnscores: *mut i32) -> windows_core::Result<()>;
    fn ValidateCodePage(&self, uicodepage: u32, hwnd: super::Foundation::HWND) -> windows_core::Result<()>;
    fn GetCodePageDescription(&self, uicodepage: u32, lcid: u32, lpwidecharstr: windows_core::PWSTR, cchwidechar: i32) -> windows_core::Result<()>;
    fn IsCodePageInstallable(&self, uicodepage: u32) -> windows_core::Result<()>;
    fn SetMimeDBSource(&self, dwsource: MIMECONTF) -> windows_core::Result<()>;
    fn GetNumberOfScripts(&self) -> windows_core::Result<u32>;
    fn EnumScripts(&self, dwflags: u32, langid: u16) -> windows_core::Result<IEnumScript>;
    fn ValidateCodePageEx(&self, uicodepage: u32, hwnd: super::Foundation::HWND, dwfiodcontrol: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IMultiLanguage2_Vtbl {
    pub const fn new<Identity: IMultiLanguage2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetNumberOfCodePageInfo<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pccodepage: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultiLanguage2_Impl::GetNumberOfCodePageInfo(this) {
                    Ok(ok__) => {
                        pccodepage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCodePageInfo<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uicodepage: u32, langid: u16, pcodepageinfo: *mut MIMECPINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage2_Impl::GetCodePageInfo(this, core::mem::transmute_copy(&uicodepage), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&pcodepageinfo)).into()
            }
        }
        unsafe extern "system" fn GetFamilyCodePage<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uicodepage: u32, puifamilycodepage: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultiLanguage2_Impl::GetFamilyCodePage(this, core::mem::transmute_copy(&uicodepage)) {
                    Ok(ok__) => {
                        puifamilycodepage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumCodePages<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfflags: u32, langid: u16, ppenumcodepage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultiLanguage2_Impl::EnumCodePages(this, core::mem::transmute_copy(&grfflags), core::mem::transmute_copy(&langid)) {
                    Ok(ok__) => {
                        ppenumcodepage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCharsetInfo<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, charset: *mut core::ffi::c_void, pcharsetinfo: *mut MIMECSETINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage2_Impl::GetCharsetInfo(this, core::mem::transmute(&charset), core::mem::transmute_copy(&pcharsetinfo)).into()
            }
        }
        unsafe extern "system" fn IsConvertible<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsrcencoding: u32, dwdstencoding: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage2_Impl::IsConvertible(this, core::mem::transmute_copy(&dwsrcencoding), core::mem::transmute_copy(&dwdstencoding)).into()
            }
        }
        unsafe extern "system" fn ConvertString<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmode: *mut u32, dwsrcencoding: u32, dwdstencoding: u32, psrcstr: *const u8, pcsrcsize: *mut u32, pdststr: *mut u8, pcdstsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage2_Impl::ConvertString(this, core::mem::transmute_copy(&pdwmode), core::mem::transmute_copy(&dwsrcencoding), core::mem::transmute_copy(&dwdstencoding), core::mem::transmute_copy(&psrcstr), core::mem::transmute_copy(&pcsrcsize), core::mem::transmute_copy(&pdststr), core::mem::transmute_copy(&pcdstsize)).into()
            }
        }
        unsafe extern "system" fn ConvertStringToUnicode<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmode: *mut u32, dwencoding: u32, psrcstr: windows_core::PCSTR, pcsrcsize: *mut u32, pdststr: windows_core::PWSTR, pcdstsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage2_Impl::ConvertStringToUnicode(this, core::mem::transmute_copy(&pdwmode), core::mem::transmute_copy(&dwencoding), core::mem::transmute(&psrcstr), core::mem::transmute_copy(&pcsrcsize), core::mem::transmute_copy(&pdststr), core::mem::transmute_copy(&pcdstsize)).into()
            }
        }
        unsafe extern "system" fn ConvertStringFromUnicode<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmode: *mut u32, dwencoding: u32, psrcstr: windows_core::PCWSTR, pcsrcsize: *mut u32, pdststr: windows_core::PSTR, pcdstsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage2_Impl::ConvertStringFromUnicode(this, core::mem::transmute_copy(&pdwmode), core::mem::transmute_copy(&dwencoding), core::mem::transmute(&psrcstr), core::mem::transmute_copy(&pcsrcsize), core::mem::transmute_copy(&pdststr), core::mem::transmute_copy(&pcdstsize)).into()
            }
        }
        unsafe extern "system" fn ConvertStringReset<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage2_Impl::ConvertStringReset(this).into()
            }
        }
        unsafe extern "system" fn GetRfc1766FromLcid<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, locale: u32, pbstrrfc1766: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultiLanguage2_Impl::GetRfc1766FromLcid(this, core::mem::transmute_copy(&locale)) {
                    Ok(ok__) => {
                        pbstrrfc1766.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLcidFromRfc1766<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plocale: *mut u32, bstrrfc1766: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage2_Impl::GetLcidFromRfc1766(this, core::mem::transmute_copy(&plocale), core::mem::transmute(&bstrrfc1766)).into()
            }
        }
        unsafe extern "system" fn EnumRfc1766<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, langid: u16, ppenumrfc1766: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultiLanguage2_Impl::EnumRfc1766(this, core::mem::transmute_copy(&langid)) {
                    Ok(ok__) => {
                        ppenumrfc1766.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRfc1766Info<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, locale: u32, langid: u16, prfc1766info: *mut RFC1766INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage2_Impl::GetRfc1766Info(this, core::mem::transmute_copy(&locale), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&prfc1766info)).into()
            }
        }
        unsafe extern "system" fn CreateConvertCharset<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uisrccodepage: u32, uidstcodepage: u32, dwproperty: u32, ppmlangconvertcharset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultiLanguage2_Impl::CreateConvertCharset(this, core::mem::transmute_copy(&uisrccodepage), core::mem::transmute_copy(&uidstcodepage), core::mem::transmute_copy(&dwproperty)) {
                    Ok(ok__) => {
                        ppmlangconvertcharset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ConvertStringInIStream<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmode: *mut u32, dwflag: u32, lpfallback: windows_core::PCWSTR, dwsrcencoding: u32, dwdstencoding: u32, pstmin: *mut core::ffi::c_void, pstmout: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage2_Impl::ConvertStringInIStream(this, core::mem::transmute_copy(&pdwmode), core::mem::transmute_copy(&dwflag), core::mem::transmute(&lpfallback), core::mem::transmute_copy(&dwsrcencoding), core::mem::transmute_copy(&dwdstencoding), core::mem::transmute_copy(&pstmin), core::mem::transmute_copy(&pstmout)).into()
            }
        }
        unsafe extern "system" fn ConvertStringToUnicodeEx<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmode: *mut u32, dwencoding: u32, psrcstr: windows_core::PCSTR, pcsrcsize: *mut u32, pdststr: windows_core::PWSTR, pcdstsize: *mut u32, dwflag: u32, lpfallback: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage2_Impl::ConvertStringToUnicodeEx(this, core::mem::transmute_copy(&pdwmode), core::mem::transmute_copy(&dwencoding), core::mem::transmute(&psrcstr), core::mem::transmute_copy(&pcsrcsize), core::mem::transmute_copy(&pdststr), core::mem::transmute_copy(&pcdstsize), core::mem::transmute_copy(&dwflag), core::mem::transmute(&lpfallback)).into()
            }
        }
        unsafe extern "system" fn ConvertStringFromUnicodeEx<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmode: *mut u32, dwencoding: u32, psrcstr: windows_core::PCWSTR, pcsrcsize: *mut u32, pdststr: windows_core::PSTR, pcdstsize: *mut u32, dwflag: u32, lpfallback: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage2_Impl::ConvertStringFromUnicodeEx(this, core::mem::transmute_copy(&pdwmode), core::mem::transmute_copy(&dwencoding), core::mem::transmute(&psrcstr), core::mem::transmute_copy(&pcsrcsize), core::mem::transmute_copy(&pdststr), core::mem::transmute_copy(&pcdstsize), core::mem::transmute_copy(&dwflag), core::mem::transmute(&lpfallback)).into()
            }
        }
        unsafe extern "system" fn DetectCodepageInIStream<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflag: u32, dwprefwincodepage: u32, pstmin: *mut core::ffi::c_void, lpencoding: *mut DetectEncodingInfo, pnscores: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage2_Impl::DetectCodepageInIStream(this, core::mem::transmute_copy(&dwflag), core::mem::transmute_copy(&dwprefwincodepage), core::mem::transmute_copy(&pstmin), core::mem::transmute_copy(&lpencoding), core::mem::transmute_copy(&pnscores)).into()
            }
        }
        unsafe extern "system" fn DetectInputCodepage<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflag: u32, dwprefwincodepage: u32, psrcstr: windows_core::PCSTR, pcsrcsize: *mut i32, lpencoding: *mut DetectEncodingInfo, pnscores: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage2_Impl::DetectInputCodepage(this, core::mem::transmute_copy(&dwflag), core::mem::transmute_copy(&dwprefwincodepage), core::mem::transmute(&psrcstr), core::mem::transmute_copy(&pcsrcsize), core::mem::transmute_copy(&lpencoding), core::mem::transmute_copy(&pnscores)).into()
            }
        }
        unsafe extern "system" fn ValidateCodePage<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uicodepage: u32, hwnd: super::Foundation::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage2_Impl::ValidateCodePage(this, core::mem::transmute_copy(&uicodepage), core::mem::transmute_copy(&hwnd)).into()
            }
        }
        unsafe extern "system" fn GetCodePageDescription<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uicodepage: u32, lcid: u32, lpwidecharstr: windows_core::PWSTR, cchwidechar: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage2_Impl::GetCodePageDescription(this, core::mem::transmute_copy(&uicodepage), core::mem::transmute_copy(&lcid), core::mem::transmute_copy(&lpwidecharstr), core::mem::transmute_copy(&cchwidechar)).into()
            }
        }
        unsafe extern "system" fn IsCodePageInstallable<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uicodepage: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage2_Impl::IsCodePageInstallable(this, core::mem::transmute_copy(&uicodepage)).into()
            }
        }
        unsafe extern "system" fn SetMimeDBSource<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsource: MIMECONTF) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage2_Impl::SetMimeDBSource(this, core::mem::transmute_copy(&dwsource)).into()
            }
        }
        unsafe extern "system" fn GetNumberOfScripts<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnscripts: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultiLanguage2_Impl::GetNumberOfScripts(this) {
                    Ok(ok__) => {
                        pnscripts.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumScripts<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, langid: u16, ppenumscript: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultiLanguage2_Impl::EnumScripts(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&langid)) {
                    Ok(ok__) => {
                        ppenumscript.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ValidateCodePageEx<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uicodepage: u32, hwnd: super::Foundation::HWND, dwfiodcontrol: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage2_Impl::ValidateCodePageEx(this, core::mem::transmute_copy(&uicodepage), core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&dwfiodcontrol)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetNumberOfCodePageInfo: GetNumberOfCodePageInfo::<Identity, OFFSET>,
            GetCodePageInfo: GetCodePageInfo::<Identity, OFFSET>,
            GetFamilyCodePage: GetFamilyCodePage::<Identity, OFFSET>,
            EnumCodePages: EnumCodePages::<Identity, OFFSET>,
            GetCharsetInfo: GetCharsetInfo::<Identity, OFFSET>,
            IsConvertible: IsConvertible::<Identity, OFFSET>,
            ConvertString: ConvertString::<Identity, OFFSET>,
            ConvertStringToUnicode: ConvertStringToUnicode::<Identity, OFFSET>,
            ConvertStringFromUnicode: ConvertStringFromUnicode::<Identity, OFFSET>,
            ConvertStringReset: ConvertStringReset::<Identity, OFFSET>,
            GetRfc1766FromLcid: GetRfc1766FromLcid::<Identity, OFFSET>,
            GetLcidFromRfc1766: GetLcidFromRfc1766::<Identity, OFFSET>,
            EnumRfc1766: EnumRfc1766::<Identity, OFFSET>,
            GetRfc1766Info: GetRfc1766Info::<Identity, OFFSET>,
            CreateConvertCharset: CreateConvertCharset::<Identity, OFFSET>,
            ConvertStringInIStream: ConvertStringInIStream::<Identity, OFFSET>,
            ConvertStringToUnicodeEx: ConvertStringToUnicodeEx::<Identity, OFFSET>,
            ConvertStringFromUnicodeEx: ConvertStringFromUnicodeEx::<Identity, OFFSET>,
            DetectCodepageInIStream: DetectCodepageInIStream::<Identity, OFFSET>,
            DetectInputCodepage: DetectInputCodepage::<Identity, OFFSET>,
            ValidateCodePage: ValidateCodePage::<Identity, OFFSET>,
            GetCodePageDescription: GetCodePageDescription::<Identity, OFFSET>,
            IsCodePageInstallable: IsCodePageInstallable::<Identity, OFFSET>,
            SetMimeDBSource: SetMimeDBSource::<Identity, OFFSET>,
            GetNumberOfScripts: GetNumberOfScripts::<Identity, OFFSET>,
            EnumScripts: EnumScripts::<Identity, OFFSET>,
            ValidateCodePageEx: ValidateCodePageEx::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMultiLanguage2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMultiLanguage2 {}
windows_core::imp::define_interface!(IMultiLanguage3, IMultiLanguage3_Vtbl, 0x4e5868ab_b157_4623_9acc_6a1d9caebe04);
impl core::ops::Deref for IMultiLanguage3 {
    type Target = IMultiLanguage2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMultiLanguage3, windows_core::IUnknown, IMultiLanguage2);
impl IMultiLanguage3 {
    pub unsafe fn DetectOutboundCodePage<P7>(&self, dwflags: u32, lpwidecharstr: &[u16], puipreferredcodepages: Option<&[u32]>, puidetectedcodepages: *mut u32, pndetectedcodepages: *mut u32, lpspecialchar: P7) -> windows_core::Result<()>
    where
        P7: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).DetectOutboundCodePage)(windows_core::Interface::as_raw(self), dwflags, core::mem::transmute(lpwidecharstr.as_ptr()), lpwidecharstr.len().try_into().unwrap(), core::mem::transmute(puipreferredcodepages.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), puipreferredcodepages.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), puidetectedcodepages as _, pndetectedcodepages as _, lpspecialchar.param().abi()).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DetectOutboundCodePageInIStream<P1, P6>(&self, dwflags: u32, pstrin: P1, puipreferredcodepages: Option<&[u32]>, puidetectedcodepages: *mut u32, pndetectedcodepages: *mut u32, lpspecialchar: P6) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::System::Com::IStream>,
        P6: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).DetectOutboundCodePageInIStream)(windows_core::Interface::as_raw(self), dwflags, pstrin.param().abi(), core::mem::transmute(puipreferredcodepages.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), puipreferredcodepages.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), puidetectedcodepages as _, pndetectedcodepages as _, lpspecialchar.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultiLanguage3_Vtbl {
    pub base__: IMultiLanguage2_Vtbl,
    pub DetectOutboundCodePage: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, u32, *const u32, u32, *mut u32, *mut u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub DetectOutboundCodePageInIStream: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *const u32, u32, *mut u32, *mut u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DetectOutboundCodePageInIStream: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMultiLanguage3_Impl: IMultiLanguage2_Impl {
    fn DetectOutboundCodePage(&self, dwflags: u32, lpwidecharstr: &windows_core::PCWSTR, cchwidechar: u32, puipreferredcodepages: *const u32, npreferredcodepages: u32, puidetectedcodepages: *mut u32, pndetectedcodepages: *mut u32, lpspecialchar: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn DetectOutboundCodePageInIStream(&self, dwflags: u32, pstrin: windows_core::Ref<super::System::Com::IStream>, puipreferredcodepages: *const u32, npreferredcodepages: u32, puidetectedcodepages: *mut u32, pndetectedcodepages: *mut u32, lpspecialchar: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IMultiLanguage3_Vtbl {
    pub const fn new<Identity: IMultiLanguage3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DetectOutboundCodePage<Identity: IMultiLanguage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, lpwidecharstr: windows_core::PCWSTR, cchwidechar: u32, puipreferredcodepages: *const u32, npreferredcodepages: u32, puidetectedcodepages: *mut u32, pndetectedcodepages: *mut u32, lpspecialchar: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage3_Impl::DetectOutboundCodePage(this, core::mem::transmute_copy(&dwflags), core::mem::transmute(&lpwidecharstr), core::mem::transmute_copy(&cchwidechar), core::mem::transmute_copy(&puipreferredcodepages), core::mem::transmute_copy(&npreferredcodepages), core::mem::transmute_copy(&puidetectedcodepages), core::mem::transmute_copy(&pndetectedcodepages), core::mem::transmute(&lpspecialchar)).into()
            }
        }
        unsafe extern "system" fn DetectOutboundCodePageInIStream<Identity: IMultiLanguage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pstrin: *mut core::ffi::c_void, puipreferredcodepages: *const u32, npreferredcodepages: u32, puidetectedcodepages: *mut u32, pndetectedcodepages: *mut u32, lpspecialchar: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage3_Impl::DetectOutboundCodePageInIStream(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pstrin), core::mem::transmute_copy(&puipreferredcodepages), core::mem::transmute_copy(&npreferredcodepages), core::mem::transmute_copy(&puidetectedcodepages), core::mem::transmute_copy(&pndetectedcodepages), core::mem::transmute(&lpspecialchar)).into()
            }
        }
        Self {
            base__: IMultiLanguage2_Vtbl::new::<Identity, OFFSET>(),
            DetectOutboundCodePage: DetectOutboundCodePage::<Identity, OFFSET>,
            DetectOutboundCodePageInIStream: DetectOutboundCodePageInIStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMultiLanguage3 as windows_core::Interface>::IID || iid == &<IMultiLanguage2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMultiLanguage3 {}
windows_core::imp::define_interface!(IOptionDescription, IOptionDescription_Vtbl, 0x432e5f85_35cf_4606_a801_6f70277e1d7a);
windows_core::imp::interface_hierarchy!(IOptionDescription, windows_core::IUnknown);
impl IOptionDescription {
    pub unsafe fn Id(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Heading(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Heading)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Labels(&self) -> windows_core::Result<super::System::Com::IEnumString> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Labels)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(feature = "Win32_System_Com")]
pub trait IOptionDescription_Impl: windows_core::IUnknownImpl {
    fn Id(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn Heading(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn Description(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn Labels(&self) -> windows_core::Result<super::System::Com::IEnumString>;
}
#[cfg(feature = "Win32_System_Com")]
impl IOptionDescription_Vtbl {
    pub const fn new<Identity: IOptionDescription_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Id<Identity: IOptionDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOptionDescription_Impl::Id(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Heading<Identity: IOptionDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOptionDescription_Impl::Heading(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Description<Identity: IOptionDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOptionDescription_Impl::Description(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Labels<Identity: IOptionDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOptionDescription_Impl::Labels(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Id: Id::<Identity, OFFSET>,
            Heading: Heading::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            Labels: Labels::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOptionDescription as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IOptionDescription {}
pub const IS_TEXT_UNICODE_ASCII16: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(1u32);
pub const IS_TEXT_UNICODE_CONTROLS: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(4u32);
pub const IS_TEXT_UNICODE_ILLEGAL_CHARS: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(256u32);
pub const IS_TEXT_UNICODE_NOT_ASCII_MASK: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(61440u32);
pub const IS_TEXT_UNICODE_NOT_UNICODE_MASK: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(3840u32);
pub const IS_TEXT_UNICODE_NULL_BYTES: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(4096u32);
pub const IS_TEXT_UNICODE_ODD_LENGTH: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(512u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IS_TEXT_UNICODE_RESULT(pub u32);
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
pub const IS_TEXT_UNICODE_REVERSE_ASCII16: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(16u32);
pub const IS_TEXT_UNICODE_REVERSE_CONTROLS: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(64u32);
pub const IS_TEXT_UNICODE_REVERSE_MASK: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(240u32);
pub const IS_TEXT_UNICODE_REVERSE_SIGNATURE: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(128u32);
pub const IS_TEXT_UNICODE_REVERSE_STATISTICS: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(32u32);
pub const IS_TEXT_UNICODE_SIGNATURE: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(8u32);
pub const IS_TEXT_UNICODE_STATISTICS: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(2u32);
pub const IS_TEXT_UNICODE_UNICODE_MASK: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(15u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IS_VALID_LOCALE_FLAGS(pub u32);
windows_core::imp::define_interface!(ISpellCheckProvider, ISpellCheckProvider_Vtbl, 0x73e976e0_8ed4_4eb1_80d7_1be0a16b0c38);
windows_core::imp::interface_hierarchy!(ISpellCheckProvider, windows_core::IUnknown);
impl ISpellCheckProvider {
    pub unsafe fn LanguageTag(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LanguageTag)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Check<P0>(&self, text: P0) -> windows_core::Result<IEnumSpellingError>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Check)(windows_core::Interface::as_raw(self), text.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Suggest<P0>(&self, word: P0) -> windows_core::Result<super::System::Com::IEnumString>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Suggest)(windows_core::Interface::as_raw(self), word.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetOptionValue<P0>(&self, optionid: P0) -> windows_core::Result<u8>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOptionValue)(windows_core::Interface::as_raw(self), optionid.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetOptionValue<P0>(&self, optionid: P0, value: u8) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetOptionValue)(windows_core::Interface::as_raw(self), optionid.param().abi(), value).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OptionIds(&self) -> windows_core::Result<super::System::Com::IEnumString> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OptionIds)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Id(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn LocalizedName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LocalizedName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetOptionDescription<P0>(&self, optionid: P0) -> windows_core::Result<IOptionDescription>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOptionDescription)(windows_core::Interface::as_raw(self), optionid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeWordlist<P1>(&self, wordlisttype: WORDLIST_TYPE, words: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::System::Com::IEnumString>,
    {
        unsafe { (windows_core::Interface::vtable(self).InitializeWordlist)(windows_core::Interface::as_raw(self), wordlisttype, words.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(feature = "Win32_System_Com")]
pub trait ISpellCheckProvider_Impl: windows_core::IUnknownImpl {
    fn LanguageTag(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn Check(&self, text: &windows_core::PCWSTR) -> windows_core::Result<IEnumSpellingError>;
    fn Suggest(&self, word: &windows_core::PCWSTR) -> windows_core::Result<super::System::Com::IEnumString>;
    fn GetOptionValue(&self, optionid: &windows_core::PCWSTR) -> windows_core::Result<u8>;
    fn SetOptionValue(&self, optionid: &windows_core::PCWSTR, value: u8) -> windows_core::Result<()>;
    fn OptionIds(&self) -> windows_core::Result<super::System::Com::IEnumString>;
    fn Id(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn LocalizedName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetOptionDescription(&self, optionid: &windows_core::PCWSTR) -> windows_core::Result<IOptionDescription>;
    fn InitializeWordlist(&self, wordlisttype: WORDLIST_TYPE, words: windows_core::Ref<super::System::Com::IEnumString>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ISpellCheckProvider_Vtbl {
    pub const fn new<Identity: ISpellCheckProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LanguageTag<Identity: ISpellCheckProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellCheckProvider_Impl::LanguageTag(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Check<Identity: ISpellCheckProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, text: windows_core::PCWSTR, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellCheckProvider_Impl::Check(this, core::mem::transmute(&text)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Suggest<Identity: ISpellCheckProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, word: windows_core::PCWSTR, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellCheckProvider_Impl::Suggest(this, core::mem::transmute(&word)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOptionValue<Identity: ISpellCheckProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, optionid: windows_core::PCWSTR, value: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellCheckProvider_Impl::GetOptionValue(this, core::mem::transmute(&optionid)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOptionValue<Identity: ISpellCheckProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, optionid: windows_core::PCWSTR, value: u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpellCheckProvider_Impl::SetOptionValue(this, core::mem::transmute(&optionid), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn OptionIds<Identity: ISpellCheckProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellCheckProvider_Impl::OptionIds(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Id<Identity: ISpellCheckProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellCheckProvider_Impl::Id(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LocalizedName<Identity: ISpellCheckProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellCheckProvider_Impl::LocalizedName(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOptionDescription<Identity: ISpellCheckProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, optionid: windows_core::PCWSTR, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellCheckProvider_Impl::GetOptionDescription(this, core::mem::transmute(&optionid)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InitializeWordlist<Identity: ISpellCheckProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wordlisttype: WORDLIST_TYPE, words: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpellCheckProvider_Impl::InitializeWordlist(this, core::mem::transmute_copy(&wordlisttype), core::mem::transmute_copy(&words)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            LanguageTag: LanguageTag::<Identity, OFFSET>,
            Check: Check::<Identity, OFFSET>,
            Suggest: Suggest::<Identity, OFFSET>,
            GetOptionValue: GetOptionValue::<Identity, OFFSET>,
            SetOptionValue: SetOptionValue::<Identity, OFFSET>,
            OptionIds: OptionIds::<Identity, OFFSET>,
            Id: Id::<Identity, OFFSET>,
            LocalizedName: LocalizedName::<Identity, OFFSET>,
            GetOptionDescription: GetOptionDescription::<Identity, OFFSET>,
            InitializeWordlist: InitializeWordlist::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpellCheckProvider as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISpellCheckProvider {}
windows_core::imp::define_interface!(ISpellCheckProviderFactory, ISpellCheckProviderFactory_Vtbl, 0x9f671e11_77d6_4c92_aefb_615215e3a4be);
windows_core::imp::interface_hierarchy!(ISpellCheckProviderFactory, windows_core::IUnknown);
impl ISpellCheckProviderFactory {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedLanguages(&self) -> windows_core::Result<super::System::Com::IEnumString> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SupportedLanguages)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn IsSupported<P0>(&self, languagetag: P0) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsSupported)(windows_core::Interface::as_raw(self), languagetag.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CreateSpellCheckProvider<P0>(&self, languagetag: P0) -> windows_core::Result<ISpellCheckProvider>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSpellCheckProvider)(windows_core::Interface::as_raw(self), languagetag.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpellCheckProviderFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub SupportedLanguages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupportedLanguages: usize,
    pub IsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CreateSpellCheckProvider: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpellCheckProviderFactory_Impl: windows_core::IUnknownImpl {
    fn SupportedLanguages(&self) -> windows_core::Result<super::System::Com::IEnumString>;
    fn IsSupported(&self, languagetag: &windows_core::PCWSTR) -> windows_core::Result<windows_core::BOOL>;
    fn CreateSpellCheckProvider(&self, languagetag: &windows_core::PCWSTR) -> windows_core::Result<ISpellCheckProvider>;
}
#[cfg(feature = "Win32_System_Com")]
impl ISpellCheckProviderFactory_Vtbl {
    pub const fn new<Identity: ISpellCheckProviderFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SupportedLanguages<Identity: ISpellCheckProviderFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellCheckProviderFactory_Impl::SupportedLanguages(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsSupported<Identity: ISpellCheckProviderFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, languagetag: windows_core::PCWSTR, value: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellCheckProviderFactory_Impl::IsSupported(this, core::mem::transmute(&languagetag)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSpellCheckProvider<Identity: ISpellCheckProviderFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, languagetag: windows_core::PCWSTR, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellCheckProviderFactory_Impl::CreateSpellCheckProvider(this, core::mem::transmute(&languagetag)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SupportedLanguages: SupportedLanguages::<Identity, OFFSET>,
            IsSupported: IsSupported::<Identity, OFFSET>,
            CreateSpellCheckProvider: CreateSpellCheckProvider::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpellCheckProviderFactory as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISpellCheckProviderFactory {}
windows_core::imp::define_interface!(ISpellChecker, ISpellChecker_Vtbl, 0xb6fd0b71_e2bc_4653_8d05_f197e412770b);
windows_core::imp::interface_hierarchy!(ISpellChecker, windows_core::IUnknown);
impl ISpellChecker {
    pub unsafe fn LanguageTag(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LanguageTag)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Check<P0>(&self, text: P0) -> windows_core::Result<IEnumSpellingError>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Check)(windows_core::Interface::as_raw(self), text.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Suggest<P0>(&self, word: P0) -> windows_core::Result<super::System::Com::IEnumString>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Suggest)(windows_core::Interface::as_raw(self), word.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Add<P0>(&self, word: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), word.param().abi()).ok() }
    }
    pub unsafe fn Ignore<P0>(&self, word: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Ignore)(windows_core::Interface::as_raw(self), word.param().abi()).ok() }
    }
    pub unsafe fn AutoCorrect<P0, P1>(&self, from: P0, to: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AutoCorrect)(windows_core::Interface::as_raw(self), from.param().abi(), to.param().abi()).ok() }
    }
    pub unsafe fn GetOptionValue<P0>(&self, optionid: P0) -> windows_core::Result<u8>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOptionValue)(windows_core::Interface::as_raw(self), optionid.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OptionIds(&self) -> windows_core::Result<super::System::Com::IEnumString> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OptionIds)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Id(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn LocalizedName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LocalizedName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn add_SpellCheckerChanged<P0>(&self, handler: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<ISpellCheckerChangedEventHandler>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).add_SpellCheckerChanged)(windows_core::Interface::as_raw(self), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn remove_SpellCheckerChanged(&self, eventcookie: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).remove_SpellCheckerChanged)(windows_core::Interface::as_raw(self), eventcookie).ok() }
    }
    pub unsafe fn GetOptionDescription<P0>(&self, optionid: P0) -> windows_core::Result<IOptionDescription>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOptionDescription)(windows_core::Interface::as_raw(self), optionid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ComprehensiveCheck<P0>(&self, text: P0) -> windows_core::Result<IEnumSpellingError>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ComprehensiveCheck)(windows_core::Interface::as_raw(self), text.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(feature = "Win32_System_Com")]
pub trait ISpellChecker_Impl: windows_core::IUnknownImpl {
    fn LanguageTag(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn Check(&self, text: &windows_core::PCWSTR) -> windows_core::Result<IEnumSpellingError>;
    fn Suggest(&self, word: &windows_core::PCWSTR) -> windows_core::Result<super::System::Com::IEnumString>;
    fn Add(&self, word: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Ignore(&self, word: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn AutoCorrect(&self, from: &windows_core::PCWSTR, to: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetOptionValue(&self, optionid: &windows_core::PCWSTR) -> windows_core::Result<u8>;
    fn OptionIds(&self) -> windows_core::Result<super::System::Com::IEnumString>;
    fn Id(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn LocalizedName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn add_SpellCheckerChanged(&self, handler: windows_core::Ref<ISpellCheckerChangedEventHandler>) -> windows_core::Result<u32>;
    fn remove_SpellCheckerChanged(&self, eventcookie: u32) -> windows_core::Result<()>;
    fn GetOptionDescription(&self, optionid: &windows_core::PCWSTR) -> windows_core::Result<IOptionDescription>;
    fn ComprehensiveCheck(&self, text: &windows_core::PCWSTR) -> windows_core::Result<IEnumSpellingError>;
}
#[cfg(feature = "Win32_System_Com")]
impl ISpellChecker_Vtbl {
    pub const fn new<Identity: ISpellChecker_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LanguageTag<Identity: ISpellChecker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellChecker_Impl::LanguageTag(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Check<Identity: ISpellChecker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, text: windows_core::PCWSTR, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellChecker_Impl::Check(this, core::mem::transmute(&text)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Suggest<Identity: ISpellChecker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, word: windows_core::PCWSTR, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellChecker_Impl::Suggest(this, core::mem::transmute(&word)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: ISpellChecker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, word: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpellChecker_Impl::Add(this, core::mem::transmute(&word)).into()
            }
        }
        unsafe extern "system" fn Ignore<Identity: ISpellChecker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, word: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpellChecker_Impl::Ignore(this, core::mem::transmute(&word)).into()
            }
        }
        unsafe extern "system" fn AutoCorrect<Identity: ISpellChecker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, from: windows_core::PCWSTR, to: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpellChecker_Impl::AutoCorrect(this, core::mem::transmute(&from), core::mem::transmute(&to)).into()
            }
        }
        unsafe extern "system" fn GetOptionValue<Identity: ISpellChecker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, optionid: windows_core::PCWSTR, value: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellChecker_Impl::GetOptionValue(this, core::mem::transmute(&optionid)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OptionIds<Identity: ISpellChecker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellChecker_Impl::OptionIds(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Id<Identity: ISpellChecker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellChecker_Impl::Id(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LocalizedName<Identity: ISpellChecker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellChecker_Impl::LocalizedName(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn add_SpellCheckerChanged<Identity: ISpellChecker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, eventcookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellChecker_Impl::add_SpellCheckerChanged(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        eventcookie.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn remove_SpellCheckerChanged<Identity: ISpellChecker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventcookie: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpellChecker_Impl::remove_SpellCheckerChanged(this, core::mem::transmute_copy(&eventcookie)).into()
            }
        }
        unsafe extern "system" fn GetOptionDescription<Identity: ISpellChecker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, optionid: windows_core::PCWSTR, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellChecker_Impl::GetOptionDescription(this, core::mem::transmute(&optionid)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ComprehensiveCheck<Identity: ISpellChecker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, text: windows_core::PCWSTR, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellChecker_Impl::ComprehensiveCheck(this, core::mem::transmute(&text)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            LanguageTag: LanguageTag::<Identity, OFFSET>,
            Check: Check::<Identity, OFFSET>,
            Suggest: Suggest::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Ignore: Ignore::<Identity, OFFSET>,
            AutoCorrect: AutoCorrect::<Identity, OFFSET>,
            GetOptionValue: GetOptionValue::<Identity, OFFSET>,
            OptionIds: OptionIds::<Identity, OFFSET>,
            Id: Id::<Identity, OFFSET>,
            LocalizedName: LocalizedName::<Identity, OFFSET>,
            add_SpellCheckerChanged: add_SpellCheckerChanged::<Identity, OFFSET>,
            remove_SpellCheckerChanged: remove_SpellCheckerChanged::<Identity, OFFSET>,
            GetOptionDescription: GetOptionDescription::<Identity, OFFSET>,
            ComprehensiveCheck: ComprehensiveCheck::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpellChecker as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISpellChecker {}
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
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), word.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpellChecker2_Vtbl {
    pub base__: ISpellChecker_Vtbl,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpellChecker2_Impl: ISpellChecker_Impl {
    fn Remove(&self, word: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ISpellChecker2_Vtbl {
    pub const fn new<Identity: ISpellChecker2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Remove<Identity: ISpellChecker2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, word: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpellChecker2_Impl::Remove(this, core::mem::transmute(&word)).into()
            }
        }
        Self { base__: ISpellChecker_Vtbl::new::<Identity, OFFSET>(), Remove: Remove::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpellChecker2 as windows_core::Interface>::IID || iid == &<ISpellChecker as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISpellChecker2 {}
windows_core::imp::define_interface!(ISpellCheckerChangedEventHandler, ISpellCheckerChangedEventHandler_Vtbl, 0x0b83a5b0_792f_4eab_9799_acf52c5ed08a);
windows_core::imp::interface_hierarchy!(ISpellCheckerChangedEventHandler, windows_core::IUnknown);
impl ISpellCheckerChangedEventHandler {
    pub unsafe fn Invoke<P0>(&self, sender: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISpellChecker>,
    {
        unsafe { (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), sender.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpellCheckerChangedEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISpellCheckerChangedEventHandler_Impl: windows_core::IUnknownImpl {
    fn Invoke(&self, sender: windows_core::Ref<ISpellChecker>) -> windows_core::Result<()>;
}
impl ISpellCheckerChangedEventHandler_Vtbl {
    pub const fn new<Identity: ISpellCheckerChangedEventHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Invoke<Identity: ISpellCheckerChangedEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpellCheckerChangedEventHandler_Impl::Invoke(this, core::mem::transmute_copy(&sender)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Invoke: Invoke::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpellCheckerChangedEventHandler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISpellCheckerChangedEventHandler {}
windows_core::imp::define_interface!(ISpellCheckerFactory, ISpellCheckerFactory_Vtbl, 0x8e018a9d_2415_4677_bf08_794ea61f94bb);
windows_core::imp::interface_hierarchy!(ISpellCheckerFactory, windows_core::IUnknown);
impl ISpellCheckerFactory {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedLanguages(&self) -> windows_core::Result<super::System::Com::IEnumString> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SupportedLanguages)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn IsSupported<P0>(&self, languagetag: P0) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsSupported)(windows_core::Interface::as_raw(self), languagetag.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CreateSpellChecker<P0>(&self, languagetag: P0) -> windows_core::Result<ISpellChecker>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSpellChecker)(windows_core::Interface::as_raw(self), languagetag.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpellCheckerFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub SupportedLanguages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupportedLanguages: usize,
    pub IsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CreateSpellChecker: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpellCheckerFactory_Impl: windows_core::IUnknownImpl {
    fn SupportedLanguages(&self) -> windows_core::Result<super::System::Com::IEnumString>;
    fn IsSupported(&self, languagetag: &windows_core::PCWSTR) -> windows_core::Result<windows_core::BOOL>;
    fn CreateSpellChecker(&self, languagetag: &windows_core::PCWSTR) -> windows_core::Result<ISpellChecker>;
}
#[cfg(feature = "Win32_System_Com")]
impl ISpellCheckerFactory_Vtbl {
    pub const fn new<Identity: ISpellCheckerFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SupportedLanguages<Identity: ISpellCheckerFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellCheckerFactory_Impl::SupportedLanguages(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsSupported<Identity: ISpellCheckerFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, languagetag: windows_core::PCWSTR, value: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellCheckerFactory_Impl::IsSupported(this, core::mem::transmute(&languagetag)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSpellChecker<Identity: ISpellCheckerFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, languagetag: windows_core::PCWSTR, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellCheckerFactory_Impl::CreateSpellChecker(this, core::mem::transmute(&languagetag)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SupportedLanguages: SupportedLanguages::<Identity, OFFSET>,
            IsSupported: IsSupported::<Identity, OFFSET>,
            CreateSpellChecker: CreateSpellChecker::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpellCheckerFactory as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISpellCheckerFactory {}
windows_core::imp::define_interface!(ISpellingError, ISpellingError_Vtbl, 0xb7c82d61_fbe8_4b47_9b27_6c0d2e0de0a3);
windows_core::imp::interface_hierarchy!(ISpellingError, windows_core::IUnknown);
impl ISpellingError {
    pub unsafe fn StartIndex(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartIndex)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Length(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Length)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CorrectiveAction(&self) -> windows_core::Result<CORRECTIVE_ACTION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CorrectiveAction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Replacement(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Replacement)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpellingError_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub StartIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Length: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub CorrectiveAction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CORRECTIVE_ACTION) -> windows_core::HRESULT,
    pub Replacement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait ISpellingError_Impl: windows_core::IUnknownImpl {
    fn StartIndex(&self) -> windows_core::Result<u32>;
    fn Length(&self) -> windows_core::Result<u32>;
    fn CorrectiveAction(&self) -> windows_core::Result<CORRECTIVE_ACTION>;
    fn Replacement(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl ISpellingError_Vtbl {
    pub const fn new<Identity: ISpellingError_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn StartIndex<Identity: ISpellingError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellingError_Impl::StartIndex(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Length<Identity: ISpellingError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellingError_Impl::Length(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CorrectiveAction<Identity: ISpellingError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut CORRECTIVE_ACTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellingError_Impl::CorrectiveAction(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Replacement<Identity: ISpellingError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellingError_Impl::Replacement(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StartIndex: StartIndex::<Identity, OFFSET>,
            Length: Length::<Identity, OFFSET>,
            CorrectiveAction: CorrectiveAction::<Identity, OFFSET>,
            Replacement: Replacement::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpellingError as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISpellingError {}
windows_core::imp::define_interface!(IUserDictionariesRegistrar, IUserDictionariesRegistrar_Vtbl, 0xaa176b85_0e12_4844_8e1a_eef1da77f586);
windows_core::imp::interface_hierarchy!(IUserDictionariesRegistrar, windows_core::IUnknown);
impl IUserDictionariesRegistrar {
    pub unsafe fn RegisterUserDictionary<P0, P1>(&self, dictionarypath: P0, languagetag: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterUserDictionary)(windows_core::Interface::as_raw(self), dictionarypath.param().abi(), languagetag.param().abi()).ok() }
    }
    pub unsafe fn UnregisterUserDictionary<P0, P1>(&self, dictionarypath: P0, languagetag: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnregisterUserDictionary)(windows_core::Interface::as_raw(self), dictionarypath.param().abi(), languagetag.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDictionariesRegistrar_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterUserDictionary: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub UnregisterUserDictionary: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IUserDictionariesRegistrar_Impl: windows_core::IUnknownImpl {
    fn RegisterUserDictionary(&self, dictionarypath: &windows_core::PCWSTR, languagetag: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn UnregisterUserDictionary(&self, dictionarypath: &windows_core::PCWSTR, languagetag: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IUserDictionariesRegistrar_Vtbl {
    pub const fn new<Identity: IUserDictionariesRegistrar_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterUserDictionary<Identity: IUserDictionariesRegistrar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dictionarypath: windows_core::PCWSTR, languagetag: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUserDictionariesRegistrar_Impl::RegisterUserDictionary(this, core::mem::transmute(&dictionarypath), core::mem::transmute(&languagetag)).into()
            }
        }
        unsafe extern "system" fn UnregisterUserDictionary<Identity: IUserDictionariesRegistrar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dictionarypath: windows_core::PCWSTR, languagetag: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUserDictionariesRegistrar_Impl::UnregisterUserDictionary(this, core::mem::transmute(&dictionarypath), core::mem::transmute(&languagetag)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterUserDictionary: RegisterUserDictionary::<Identity, OFFSET>,
            UnregisterUserDictionary: UnregisterUserDictionary::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUserDictionariesRegistrar as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUserDictionariesRegistrar {}
pub type LANGGROUPLOCALE_ENUMPROCA = Option<unsafe extern "system" fn(param0: u32, param1: u32, param2: windows_core::PCSTR, param3: isize) -> windows_core::BOOL>;
pub type LANGGROUPLOCALE_ENUMPROCW = Option<unsafe extern "system" fn(param0: u32, param1: u32, param2: windows_core::PCWSTR, param3: isize) -> windows_core::BOOL>;
pub type LANGUAGEGROUP_ENUMPROCA = Option<unsafe extern "system" fn(param0: u32, param1: windows_core::PCSTR, param2: windows_core::PCSTR, param3: u32, param4: isize) -> windows_core::BOOL>;
pub type LANGUAGEGROUP_ENUMPROCW = Option<unsafe extern "system" fn(param0: u32, param1: windows_core::PCWSTR, param2: windows_core::PCWSTR, param3: u32, param4: isize) -> windows_core::BOOL>;
pub const LANG_SYSTEM_DEFAULT: i32 = 2048i32;
pub const LANG_USER_DEFAULT: i32 = 1024i32;
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
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LOCALESIGNATURE {
    pub lsUsb: [u32; 4],
    pub lsCsbDefault: [u32; 2],
    pub lsCsbSupported: [u32; 2],
}
impl Default for LOCALESIGNATURE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const LOCALE_ALL: u32 = 0u32;
pub const LOCALE_ALLOW_NEUTRAL_NAMES: u32 = 134217728u32;
pub const LOCALE_ALTERNATE_SORTS: u32 = 4u32;
pub type LOCALE_ENUMPROCA = Option<unsafe extern "system" fn(param0: windows_core::PCSTR) -> windows_core::BOOL>;
pub type LOCALE_ENUMPROCEX = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: u32, param2: super::Foundation::LPARAM) -> windows_core::BOOL>;
pub type LOCALE_ENUMPROCW = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR) -> windows_core::BOOL>;
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
pub const LOCALE_SYSTEM_DEFAULT: u32 = 2048u32;
pub const LOCALE_USER_DEFAULT: u32 = 1024u32;
pub const LOCALE_USE_CP_ACP: u32 = 1073741824u32;
pub const LOCALE_WINDOWS: u32 = 1u32;
pub const LOWLEVEL_SERVICE_TYPES: u32 = 2u32;
pub const LOW_SURROGATE_END: u32 = 57343u32;
pub const LOW_SURROGATE_START: u32 = 56320u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Default for MAPPING_DATA_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Default for MAPPING_OPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Default for MAPPING_PROPERTY_BAG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Default for MAPPING_SERVICE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MIMECONTF(pub i32);
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
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Default for MIMECPINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIMECSETINFO {
    pub uiCodePage: u32,
    pub uiInternetEncoding: u32,
    pub wszCharset: [u16; 50],
}
impl Default for MIMECSETINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MIN_SPELLING_NTDDI: u32 = 100794368u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MLCONVCHAR(pub i32);
pub const MLCONVCHARF_AUTODETECT: MLCONVCHAR = MLCONVCHAR(1i32);
pub const MLCONVCHARF_DETECTJPN: MLCONVCHAR = MLCONVCHAR(32i32);
pub const MLCONVCHARF_ENTITIZE: MLCONVCHAR = MLCONVCHAR(2i32);
pub const MLCONVCHARF_NAME_ENTITIZE: MLCONVCHAR = MLCONVCHAR(4i32);
pub const MLCONVCHARF_NCR_ENTITIZE: MLCONVCHAR = MLCONVCHAR(2i32);
pub const MLCONVCHARF_NOBESTFITCHARS: MLCONVCHAR = MLCONVCHAR(16i32);
pub const MLCONVCHARF_USEDEFCHAR: MLCONVCHAR = MLCONVCHAR(8i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MLCP(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MLDETECTCP(pub i32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MLSTR_FLAGS(pub i32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MULTI_BYTE_TO_WIDE_CHAR_FLAGS(pub u32);
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
pub const MinuteUnit: CALDATETIME_DATEUNIT = CALDATETIME_DATEUNIT(6i32);
pub const MonthUnit: CALDATETIME_DATEUNIT = CALDATETIME_DATEUNIT(2i32);
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NEWTEXTMETRICEXA {
    pub ntmTm: super::Graphics::Gdi::NEWTEXTMETRICA,
    pub ntmFontSig: FONTSIGNATURE,
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NEWTEXTMETRICEXW {
    pub ntmTm: super::Graphics::Gdi::NEWTEXTMETRICW,
    pub ntmFontSig: FONTSIGNATURE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NLSVERSIONINFO {
    pub dwNLSVersionInfoSize: u32,
    pub dwNLSVersion: u32,
    pub dwDefinedVersion: u32,
    pub dwEffectiveId: u32,
    pub guidCustomVersion: windows_core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NLSVERSIONINFOEX {
    pub dwNLSVersionInfoSize: u32,
    pub dwNLSVersion: u32,
    pub dwDefinedVersion: u32,
    pub dwEffectiveId: u32,
    pub guidCustomVersion: windows_core::GUID,
}
pub const NLS_CP_CPINFO: u32 = 268435456u32;
pub const NLS_CP_MBTOWC: u32 = 1073741824u32;
pub const NLS_CP_WCTOMB: u32 = 2147483648u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NORM_FORM(pub i32);
pub const NORM_IGNORECASE: COMPARE_STRING_FLAGS = COMPARE_STRING_FLAGS(1u32);
pub const NORM_IGNOREKANATYPE: COMPARE_STRING_FLAGS = COMPARE_STRING_FLAGS(65536u32);
pub const NORM_IGNORENONSPACE: COMPARE_STRING_FLAGS = COMPARE_STRING_FLAGS(2u32);
pub const NORM_IGNORESYMBOLS: COMPARE_STRING_FLAGS = COMPARE_STRING_FLAGS(4u32);
pub const NORM_IGNOREWIDTH: COMPARE_STRING_FLAGS = COMPARE_STRING_FLAGS(131072u32);
pub const NORM_LINGUISTIC_CASING: COMPARE_STRING_FLAGS = COMPARE_STRING_FLAGS(134217728u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NUMBERFMTA {
    pub NumDigits: u32,
    pub LeadingZero: u32,
    pub Grouping: u32,
    pub lpDecimalSep: windows_core::PSTR,
    pub lpThousandSep: windows_core::PSTR,
    pub NegativeOrder: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NUMBERFMTW {
    pub NumDigits: u32,
    pub LeadingZero: u32,
    pub Grouping: u32,
    pub lpDecimalSep: windows_core::PWSTR,
    pub lpThousandSep: windows_core::PWSTR,
    pub NegativeOrder: u32,
}
pub const NUMSYS_NAME_CAPACITY: u32 = 8u32;
pub const NormalizationC: NORM_FORM = NORM_FORM(1i32);
pub const NormalizationD: NORM_FORM = NORM_FORM(2i32);
pub const NormalizationKC: NORM_FORM = NORM_FORM(5i32);
pub const NormalizationKD: NORM_FORM = NORM_FORM(6i32);
pub const NormalizationOther: NORM_FORM = NORM_FORM(0i32);
pub const OFFLINE_SERVICES: u32 = 2u32;
pub const ONLINE_SERVICES: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OPENTYPE_FEATURE_RECORD {
    pub tagFeature: u32,
    pub lParameter: i32,
}
pub type PFN_MAPPINGCALLBACKPROC = Option<unsafe extern "system" fn(pbag: *mut MAPPING_PROPERTY_BAG, data: *mut core::ffi::c_void, dwdatasize: u32, result: windows_core::HRESULT)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RFC1766INFO {
    pub lcid: u32,
    pub wszRfc1766: [u16; 6],
    pub wszLocaleName: [u16; 32],
}
impl Default for RFC1766INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCRIPTCONTF(pub i32);
pub const SCRIPTCONTF_FIXED_FONT: SCRIPTFONTCONTF = SCRIPTFONTCONTF(1i32);
pub const SCRIPTCONTF_PROPORTIONAL_FONT: SCRIPTFONTCONTF = SCRIPTFONTCONTF(2i32);
pub const SCRIPTCONTF_SCRIPT_HIDE: SCRIPTFONTCONTF = SCRIPTFONTCONTF(131072i32);
pub const SCRIPTCONTF_SCRIPT_SYSTEM: SCRIPTFONTCONTF = SCRIPTFONTCONTF(262144i32);
pub const SCRIPTCONTF_SCRIPT_USER: SCRIPTFONTCONTF = SCRIPTFONTCONTF(65536i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCRIPTFONTCONTF(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCRIPTFONTINFO {
    pub scripts: i64,
    pub wszFont: [u16; 32],
}
impl Default for SCRIPTFONTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCRIPTINFO {
    pub ScriptId: u8,
    pub uiCodePage: u32,
    pub wszDescription: [u16; 48],
    pub wszFixedWidthFont: [u16; 32],
    pub wszProportionalFont: [u16; 32],
}
impl Default for SCRIPTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCRIPT_ANALYSIS {
    pub _bitfield: u16,
    pub s: SCRIPT_STATE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCRIPT_CHARPROP {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCRIPT_CONTROL {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCRIPT_DIGITSUBSTITUTE {
    pub _bitfield1: u32,
    pub _bitfield2: u32,
    pub dwReserved: u32,
}
pub const SCRIPT_DIGITSUBSTITUTE_CONTEXT: u32 = 0u32;
pub const SCRIPT_DIGITSUBSTITUTE_NATIONAL: u32 = 2u32;
pub const SCRIPT_DIGITSUBSTITUTE_NONE: u32 = 1u32;
pub const SCRIPT_DIGITSUBSTITUTE_TRADITIONAL: u32 = 3u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCRIPT_FONTPROPERTIES {
    pub cBytes: i32,
    pub wgBlank: u16,
    pub wgDefault: u16,
    pub wgInvalid: u16,
    pub wgKashida: u16,
    pub iKashidaWidth: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCRIPT_GLYPHPROP {
    pub sva: SCRIPT_VISATTR,
    pub reserved: u16,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCRIPT_IS_COMPLEX_FLAGS(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCRIPT_ITEM {
    pub iCharPos: i32,
    pub a: SCRIPT_ANALYSIS,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCRIPT_JUSTIFY(pub i32);
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCRIPT_LOGATTR {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCRIPT_PROPERTIES {
    pub _bitfield1: u32,
    pub _bitfield2: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCRIPT_STATE {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCRIPT_TABDEF {
    pub cTabStops: i32,
    pub iScale: i32,
    pub pTabStops: *mut i32,
    pub iTabOrigin: i32,
}
impl Default for SCRIPT_TABDEF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SCRIPT_TAG_UNKNOWN: u32 = 0u32;
pub const SCRIPT_UNDEFINED: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCRIPT_VISATTR {
    pub _bitfield: u16,
}
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SYSGEOCLASS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SYSGEOTYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SYSNLS_FUNCTION(pub i32);
pub const SecondUnit: CALDATETIME_DATEUNIT = CALDATETIME_DATEUNIT(7i32);
pub const SpellCheckerFactory: windows_core::GUID = windows_core::GUID::from_u128(0x7ab36653_1796_484b_bdfa_e74f1db7c1dc);
pub const TCI_SRCCHARSET: TRANSLATE_CHARSET_INFO_FLAGS = TRANSLATE_CHARSET_INFO_FLAGS(1u32);
pub const TCI_SRCCODEPAGE: TRANSLATE_CHARSET_INFO_FLAGS = TRANSLATE_CHARSET_INFO_FLAGS(2u32);
pub const TCI_SRCFONTSIG: TRANSLATE_CHARSET_INFO_FLAGS = TRANSLATE_CHARSET_INFO_FLAGS(3u32);
pub const TCI_SRCLOCALE: TRANSLATE_CHARSET_INFO_FLAGS = TRANSLATE_CHARSET_INFO_FLAGS(4096u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TEXTRANGE_PROPERTIES {
    pub potfRecords: *mut OPENTYPE_FEATURE_RECORD,
    pub cotfRecords: i32,
}
impl Default for TEXTRANGE_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type TIMEFMT_ENUMPROCA = Option<unsafe extern "system" fn(param0: windows_core::PCSTR) -> windows_core::BOOL>;
pub type TIMEFMT_ENUMPROCEX = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: super::Foundation::LPARAM) -> windows_core::BOOL>;
pub type TIMEFMT_ENUMPROCW = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR) -> windows_core::BOOL>;
pub const TIME_FORCE24HOURFORMAT: TIME_FORMAT_FLAGS = TIME_FORMAT_FLAGS(8u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TIME_FORMAT_FLAGS(pub u32);
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
pub const TIME_NOMINUTESORSECONDS: TIME_FORMAT_FLAGS = TIME_FORMAT_FLAGS(1u32);
pub const TIME_NOSECONDS: TIME_FORMAT_FLAGS = TIME_FORMAT_FLAGS(2u32);
pub const TIME_NOTIMEMARKER: TIME_FORMAT_FLAGS = TIME_FORMAT_FLAGS(4u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRANSLATE_CHARSET_INFO_FLAGS(pub u32);
pub const TickUnit: CALDATETIME_DATEUNIT = CALDATETIME_DATEUNIT(8i32);
pub const U16_MAX_LENGTH: u32 = 2u32;
pub const U8_LEAD3_T1_BITS: windows_core::PCSTR = windows_core::s!(" 000000000000\u{10}00");
pub const U8_LEAD4_T1_BITS: windows_core::PCSTR = windows_core::s!("\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{1e}\u{f}\u{f}\u{f}\u{0}\u{0}\u{0}\u{0}");
pub const U8_MAX_LENGTH: u32 = 4u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UAcceptResult(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UAlphabeticIndexLabelType(pub i32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UBiDi(pub isize);
pub type UBiDiClassCallback = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, c: i32) -> UCharDirection>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UBiDiDirection(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UBiDiMirroring(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UBiDiOrder(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UBiDiReorderingMode(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UBiDiReorderingOption(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UBiDiTransform(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UBidiPairedBracketType(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UBlockCode(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UBreakIterator(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UBreakIteratorType(pub i32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UCPMap(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UCPMapRangeOption(pub i32);
pub type UCPMapValueFilter = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, value: u32) -> u32>;
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
impl Default for UCPTrieData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UCPTrieType(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UCPTrieValueWidth(pub i32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UCalendarAMPMs(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UCalendarAttribute(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UCalendarDateFields(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UCalendarDaysOfWeek(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UCalendarDisplayNameType(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UCalendarLimitType(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UCalendarMonths(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UCalendarType(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UCalendarWallTimeOption(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UCalendarWeekdayType(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UCaseMap(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UCharCategory(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UCharDirection(pub i32);
pub type UCharEnumTypeRange = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, start: i32, limit: i32, r#type: UCharCategory) -> i8>;
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
impl Default for UCharIterator {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type UCharIteratorCurrent = Option<unsafe extern "system" fn(iter: *mut UCharIterator) -> i32>;
pub type UCharIteratorGetIndex = Option<unsafe extern "system" fn(iter: *mut UCharIterator, origin: UCharIteratorOrigin) -> i32>;
pub type UCharIteratorGetState = Option<unsafe extern "system" fn(iter: *const UCharIterator) -> u32>;
pub type UCharIteratorHasNext = Option<unsafe extern "system" fn(iter: *mut UCharIterator) -> i8>;
pub type UCharIteratorHasPrevious = Option<unsafe extern "system" fn(iter: *mut UCharIterator) -> i8>;
pub type UCharIteratorMove = Option<unsafe extern "system" fn(iter: *mut UCharIterator, delta: i32, origin: UCharIteratorOrigin) -> i32>;
pub type UCharIteratorNext = Option<unsafe extern "system" fn(iter: *mut UCharIterator) -> i32>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UCharIteratorOrigin(pub i32);
pub type UCharIteratorPrevious = Option<unsafe extern "system" fn(iter: *mut UCharIterator) -> i32>;
pub type UCharIteratorReserved = Option<unsafe extern "system" fn(iter: *mut UCharIterator, something: i32) -> i32>;
pub type UCharIteratorSetState = Option<unsafe extern "system" fn(iter: *mut UCharIterator, state: u32, perrorcode: *mut UErrorCode)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UCharNameChoice(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UCharsetDetector(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UCharsetMatch(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UColAttribute(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UColAttributeValue(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UColBoundMode(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UColReorderCode(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UColRuleOption(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UCollationElements(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UCollationResult(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UCollator(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UConstrainedFieldPosition(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UConverter(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UConverterCallbackReason(pub i32);
pub type UConverterFromUCallback = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, args: *mut UConverterFromUnicodeArgs, codeunits: *const u16, length: i32, codepoint: i32, reason: UConverterCallbackReason, perrorcode: *mut UErrorCode)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Default for UConverterFromUnicodeArgs {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UConverterPlatform(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UConverterSelector(pub isize);
pub type UConverterToUCallback = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, args: *mut UConverterToUnicodeArgs, codeunits: windows_core::PCSTR, length: i32, reason: UConverterCallbackReason, perrorcode: *mut UErrorCode)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Default for UConverterToUnicodeArgs {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UConverterType(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UConverterUnicodeSet(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UCurrCurrencyType(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UCurrNameStyle(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UCurrencySpacing(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UCurrencyUsage(pub i32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UDateAbsoluteUnit(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UDateDirection(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UDateFormatBooleanAttribute(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UDateFormatField(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UDateFormatStyle(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UDateFormatSymbolType(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UDateFormatSymbols(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UDateIntervalFormat(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UDateRelativeDateTimeFormatterStyle(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UDateRelativeUnit(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UDateTimePGDisplayWidth(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UDateTimePatternConflict(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UDateTimePatternField(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UDateTimePatternMatchOptions(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UDateTimeScale(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UDecompositionType(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UDialectHandling(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UDisplayContext(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UDisplayContextType(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UEastAsianWidth(pub i32);
pub type UEnumCharNamesFn = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, code: i32, namechoice: UCharNameChoice, name: windows_core::PCSTR, length: i32) -> i8>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UEnumeration(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UErrorCode(pub i32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UFieldCategory(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UFieldPosition {
    pub field: i32,
    pub beginIndex: i32,
    pub endIndex: i32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UFieldPositionIterator(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UFormattableType(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UFormattedDateInterval(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UFormattedList(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UFormattedNumber(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UFormattedNumberRange(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UFormattedRelativeDateTime(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UFormattedValue(pub isize);
pub const UGENDER_FEMALE: UGender = UGender(1i32);
pub const UGENDER_MALE: UGender = UGender(0i32);
pub const UGENDER_OTHER: UGender = UGender(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UGender(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UGenderInfo(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UGraphemeClusterBreak(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UHangulSyllableType(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UHashtable(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UIDNA(pub isize);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UIDNAInfo {
    pub size: i16,
    pub isTransitionalDifferent: i8,
    pub reservedB3: i8,
    pub errors: u32,
    pub reservedI2: i32,
    pub reservedI3: i32,
}
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
pub type UILANGUAGE_ENUMPROCA = Option<unsafe extern "system" fn(param0: windows_core::PCSTR, param1: isize) -> windows_core::BOOL>;
pub type UILANGUAGE_ENUMPROCW = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: isize) -> windows_core::BOOL>;
pub const UITER_CURRENT: UCharIteratorOrigin = UCharIteratorOrigin(1i32);
pub const UITER_LENGTH: UCharIteratorOrigin = UCharIteratorOrigin(4i32);
pub const UITER_LIMIT: UCharIteratorOrigin = UCharIteratorOrigin(2i32);
pub const UITER_START: UCharIteratorOrigin = UCharIteratorOrigin(0i32);
pub const UITER_UNKNOWN_INDEX: i32 = -2i32;
pub const UITER_ZERO: UCharIteratorOrigin = UCharIteratorOrigin(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UIndicPositionalCategory(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UIndicSyllabicCategory(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UJoiningGroup(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UJoiningType(pub i32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ULayoutType(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ULineBreak(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ULineBreakTag(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UListFormatter(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UListFormatterField(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UListFormatterType(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UListFormatterWidth(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ULocAvailableType(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ULocDataLocaleType(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ULocaleData(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ULocaleDataDelimiterType(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ULocaleDataExemplarSetType(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ULocaleDisplayNames(pub isize);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UMeasureFormatWidth(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UMeasurementSystem(pub i32);
pub type UMemAllocFn = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, size: usize) -> *mut core::ffi::c_void>;
pub type UMemFreeFn = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, mem: *mut core::ffi::c_void)>;
pub type UMemReallocFn = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, mem: *mut core::ffi::c_void, size: usize) -> *mut core::ffi::c_void>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UMessagePatternApostropheMode(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UMessagePatternArgType(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UMessagePatternPartType(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UMutableCPTrie(pub isize);
pub type UNESCAPE_CHAR_AT = Option<unsafe extern "system" fn(offset: i32, context: *mut core::ffi::c_void) -> u16>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UNICODERANGE {
    pub wcFrom: u16,
    pub wcTo: u16,
}
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UNormalization2Mode(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UNormalizationCheckResult(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UNormalizationMode(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UNormalizer2(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UNumberCompactStyle(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UNumberDecimalSeparatorDisplay(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UNumberFormatAttribute(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UNumberFormatAttributeValue(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UNumberFormatFields(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UNumberFormatPadPosition(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UNumberFormatRoundingMode(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UNumberFormatStyle(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UNumberFormatSymbol(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UNumberFormatTextAttribute(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UNumberFormatter(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UNumberGroupingStrategy(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UNumberRangeCollapse(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UNumberRangeIdentityFallback(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UNumberRangeIdentityResult(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UNumberSignDisplay(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UNumberUnitWidth(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UNumberingSystem(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UNumericType(pub i32);
pub const UPLURAL_TYPE_CARDINAL: UPluralType = UPluralType(0i32);
pub const UPLURAL_TYPE_ORDINAL: UPluralType = UPluralType(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UParseError {
    pub line: i32,
    pub offset: i32,
    pub preContext: [u16; 16],
    pub postContext: [u16; 16],
}
impl Default for UParseError {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UPluralRules(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UPluralType(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UProperty(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UPropertyNameChoice(pub i32);
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
pub type URegexFindProgressCallback = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, matchindex: i64) -> i8>;
pub type URegexMatchCallback = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, steps: i32) -> i8>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct URegexpFlag(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct URegion(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct URegionType(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct URegularExpression(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct URelativeDateTimeFormatter(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct URelativeDateTimeFormatterField(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct URelativeDateTimeUnit(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UReplaceableCallbacks {
    pub length: isize,
    pub charAt: isize,
    pub char32At: isize,
    pub replace: isize,
    pub extract: isize,
    pub copy: isize,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UResType(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UResourceBundle(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct URestrictionLevel(pub i32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UScriptCode(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UScriptUsage(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct USearch(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USearchAttribute(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USearchAttributeValue(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USentenceBreak(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USentenceBreakTag(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct USerializedSet {
    pub array: *const u16,
    pub bmpLength: i32,
    pub length: i32,
    pub staticArray: [u16; 8],
}
impl Default for USerializedSet {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct USet(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USetSpanCondition(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct USpoofCheckResult(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct USpoofChecker(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USpoofChecks(pub i32);
pub type UStringCaseMapper = Option<unsafe extern "system" fn(csm: *const UCaseMap, dest: *mut u16, destcapacity: i32, src: *const u16, srclength: i32, perrorcode: *mut UErrorCode) -> i32>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UStringPrepProfile(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UStringPrepProfileType(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UStringSearch(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UStringTrieBuildOption(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UStringTrieResult(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USystemTimeZoneType(pub i32);
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
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Default for UText {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type UTextAccess = Option<unsafe extern "system" fn(ut: *mut UText, nativeindex: i64, forward: i8) -> i8>;
pub type UTextClone = Option<unsafe extern "system" fn(dest: *mut UText, src: *const UText, deep: i8, status: *mut UErrorCode) -> *mut UText>;
pub type UTextClose = Option<unsafe extern "system" fn(ut: *mut UText)>;
pub type UTextCopy = Option<unsafe extern "system" fn(ut: *mut UText, nativestart: i64, nativelimit: i64, nativedest: i64, r#move: i8, status: *mut UErrorCode)>;
pub type UTextExtract = Option<unsafe extern "system" fn(ut: *mut UText, nativestart: i64, nativelimit: i64, dest: *mut u16, destcapacity: i32, status: *mut UErrorCode) -> i32>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
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
pub type UTextMapNativeIndexToUTF16 = Option<unsafe extern "system" fn(ut: *const UText, nativeindex: i64) -> i32>;
pub type UTextMapOffsetToNative = Option<unsafe extern "system" fn(ut: *const UText) -> i64>;
pub type UTextNativeLength = Option<unsafe extern "system" fn(ut: *mut UText) -> i64>;
pub type UTextReplace = Option<unsafe extern "system" fn(ut: *mut UText, nativestart: i64, nativelimit: i64, replacementtext: *const u16, replacmentlength: i32, status: *mut UErrorCode) -> i32>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UTimeScaleValue(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UTimeZoneFormatGMTOffsetPatternType(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UTimeZoneFormatParseOption(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UTimeZoneFormatStyle(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UTimeZoneFormatTimeType(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UTimeZoneNameType(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UTimeZoneTransitionType(pub i32);
pub type UTraceData = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, fnnumber: i32, level: i32, fmt: windows_core::PCSTR, args: *mut i8)>;
pub type UTraceEntry = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, fnnumber: i32)>;
pub type UTraceExit = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, fnnumber: i32, fmt: windows_core::PCSTR, args: *mut i8)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UTraceFunctionNumber(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UTraceLevel(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UTransDirection(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UTransPosition {
    pub contextStart: i32,
    pub contextLimit: i32,
    pub start: i32,
    pub limit: i32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UVerticalOrientation(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UWordBreak(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UWordBreakValues(pub i32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WORDLIST_TYPE(pub i32);
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
