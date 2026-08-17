// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Procedure declarations, constant definitions, and macros for the NLS component.
use ctypes::c_int;
use shared::basetsd::LONG_PTR;
use shared::guiddef::GUID;
use shared::minwindef::{
    BOOL, BYTE, DWORD, INT, LPARAM, LPDWORD, LPINT, LPVOID, LPWORD, MAX_PATH, PDWORD, PULONG,
    UINT,
};
use um::minwinbase::SYSTEMTIME;
use um::winnt::{
    CHAR, LANGID, LCID, LONG, LPCSTR, LPCWCH, LPCWSTR, LPSTR, LPWSTR, PCNZCH, PCNZWCH, PCWSTR,
    PCZZWSTR, PULONGLONG, PWSTR, PZZWSTR, ULONGLONG, WCHAR,
};
pub const MAX_LEADBYTES: usize = 12;
pub const MAX_DEFAULTCHAR: usize = 2;
pub const MB_PRECOMPOSED: DWORD = 0x00000001;
pub const MB_COMPOSITE: DWORD = 0x00000002;
pub const MB_USEGLYPHCHARS: DWORD = 0x00000004;
pub const MB_ERR_INVALID_CHARS: DWORD = 0x00000008;
pub const WC_COMPOSITECHECK: DWORD = 0x00000200;
pub const WC_DISCARDNS: DWORD = 0x00000010;
pub const WC_SEPCHARS: DWORD = 0x00000020;
pub const WC_DEFAULTCHAR: DWORD = 0x00000040;
pub const WC_ERR_INVALID_CHARS: DWORD = 0x00000080;
pub const WC_NO_BEST_FIT_CHARS: DWORD = 0x00000400;
pub const CP_ACP: DWORD = 0;
pub const CP_OEMCP: DWORD = 1;
pub const CP_MACCP: DWORD = 2;
pub const CP_THREAD_ACP: DWORD = 3;
pub const CP_SYMBOL: DWORD = 42;
pub const CP_UTF7: DWORD = 65000;
pub const CP_UTF8: DWORD = 65001;
pub type LGRPID = DWORD;
pub type LCTYPE = DWORD;
pub type CALTYPE = DWORD;
pub type CALID = DWORD;
STRUCT!{struct CPINFO {
    MaxCharSize: UINT,
    DefaultChar: [BYTE; MAX_DEFAULTCHAR],
    LeadByte: [BYTE; MAX_LEADBYTES],
}}
pub type LPCPINFO = *mut CPINFO;
STRUCT!{struct CPINFOEXA {
    MaxCharSize: UINT,
    DefaultChar: [BYTE; MAX_DEFAULTCHAR],
    LeadByte: [BYTE; MAX_LEADBYTES],
    UnicodeDefaultChar: WCHAR,
    CodePage: UINT,
    CodePageName: [CHAR; MAX_PATH],
}}
pub type LPCPINFOEXA = *mut CPINFOEXA;
STRUCT!{struct CPINFOEXW {
    MaxCharSize: UINT,
    DefaultChar: [BYTE; MAX_DEFAULTCHAR],
    LeadByte: [BYTE; MAX_LEADBYTES],
    UnicodeDefaultChar: WCHAR,
    CodePage: UINT,
    CodePageName: [WCHAR; MAX_PATH],
}}
pub type LPCPINFOEXW = *mut CPINFOEXW;
STRUCT!{struct NUMBERFMTA {
    NumDigits: UINT,
    LeadingZero: UINT,
    Grouping: UINT,
    lpDecimalSep: LPSTR,
    lpThousandSep: LPSTR,
    NegativeOrder: UINT,
}}
pub type LPNUMBERFMTA = *mut NUMBERFMTA;
STRUCT!{struct NUMBERFMTW {
    NumDigits: UINT,
    LeadingZero: UINT,
    Grouping: UINT,
    lpDecimalSep: LPWSTR,
    lpThousandSep: LPWSTR,
    NegativeOrder: UINT,
}}
pub type LPNUMBERFMTW = *mut NUMBERFMTW;
STRUCT!{struct CURRENCYFMTA {
    NumDigits: UINT,
    LeadingZero: UINT,
    Grouping: UINT,
    lpDecimalSep: LPSTR,
    lpThousandSep: LPSTR,
    NegativeOrder: UINT,
    PositiveOrder: UINT,
    lpCurrencySymbol: LPSTR,
}}
pub type LPCURRENCYFMTA = *mut CURRENCYFMTA;
STRUCT!{struct CURRENCYFMTW {
    NumDigits: UINT,
    LeadingZero: UINT,
    Grouping: UINT,
    lpDecimalSep: LPWSTR,
    lpThousandSep: LPWSTR,
    NegativeOrder: UINT,
    PositiveOrder: UINT,
    lpCurrencySymbol: LPWSTR,
}}
pub type LPCURRENCYFMTW = *mut CURRENCYFMTW;
pub type NLS_FUNCTION = DWORD;
STRUCT!{struct NLSVERSIONINFO {
    dwNLSVersionInfoSize: DWORD,
    dwNLSVersion: DWORD,
    dwDefinedVersion: DWORD,
    dwEffectiveId: DWORD,
    guidCustomVersion: GUID,
}}
pub type LPNLSVERSIONINFO = *mut NLSVERSIONINFO;
STRUCT!{struct NLSVERSIONINFOEX {
    dwNLSVersionInfoSize: DWORD,
    dwNLSVersion: DWORD,
    dwDefinedVersion: DWORD,
    dwEffectiveId: DWORD,
    guidCustomVersion: GUID,
}}
pub type LPNLSVERSIONINFOEX = *mut NLSVERSIONINFOEX;
pub type GEOID = LONG;
pub type GEOTYPE = DWORD;
pub type GEOCLASS = DWORD;
ENUM!{enum NORM_FORM {
    NormalizationOther = 0,
    NormalizationC = 0x1,
    NormalizationD = 0x2,
    NormalizationKC = 0x5,
    NormalizationKD = 0x6,
}}
FN!{stdcall LANGUAGEGROUP_ENUMPROCA(
    LGRPID,
    LPSTR,
    LPSTR,
    DWORD,
    LONG_PTR,
) -> BOOL}
FN!{stdcall LANGGROUPLOCALE_ENUMPROCA(
    LGRPID,
    LCID,
    LPSTR,
    LONG_PTR,
) -> BOOL}
FN!{stdcall UILANGUAGE_ENUMPROCA(
    LPSTR,
    LONG_PTR,
) -> BOOL}
FN!{stdcall CODEPAGE_ENUMPROCA(
    LPSTR,
) -> BOOL}
FN!{stdcall DATEFMT_ENUMPROCA(
    LPSTR,
) -> BOOL}
FN!{stdcall DATEFMT_ENUMPROCEXA(
    LPSTR,
    CALID,
) -> BOOL}
FN!{stdcall TIMEFMT_ENUMPROCA(
    LPSTR,
) -> BOOL}
FN!{stdcall CALINFO_ENUMPROCA(
    LPSTR,
) -> BOOL}
FN!{stdcall CALINFO_ENUMPROCEXA(
    LPSTR,
    CALID,
) -> BOOL}
FN!{stdcall LOCALE_ENUMPROCA(
    LPSTR,
) -> BOOL}
FN!{stdcall LOCALE_ENUMPROCW(
    LPWSTR,
) -> BOOL}
FN!{stdcall LANGUAGEGROUP_ENUMPROCW(
    LGRPID,
    LPWSTR,
    LPWSTR,
    DWORD,
    LONG_PTR,
) -> BOOL}
FN!{stdcall LANGGROUPLOCALE_ENUMPROCW(
    LGRPID,
    LCID,
    LPWSTR,
    LONG_PTR,
) -> BOOL}
FN!{stdcall UILANGUAGE_ENUMPROCW(
    LPWSTR,
    LONG_PTR,
) -> BOOL}
FN!{stdcall CODEPAGE_ENUMPROCW(
    LPWSTR,
) -> BOOL}
FN!{stdcall DATEFMT_ENUMPROCW(
    LPWSTR,
) -> BOOL}
FN!{stdcall DATEFMT_ENUMPROCEXW(
    LPWSTR,
    CALID,
) -> BOOL}
FN!{stdcall TIMEFMT_ENUMPROCW(
    LPWSTR,
) -> BOOL}
FN!{stdcall CALINFO_ENUMPROCW(
    LPWSTR,
) -> BOOL}
FN!{stdcall CALINFO_ENUMPROCEXW(
    LPWSTR,
    CALID,
) -> BOOL}
FN!{stdcall GEO_ENUMPROC(
    GEOID,
) -> BOOL}
STRUCT!{struct FILEMUIINFO {
    dwSize: DWORD,
    dwVersion: DWORD,
    dwFileType: DWORD,
    pChecksum: [BYTE; 16],
    pServiceChecksum: [BYTE; 16],
    dwLanguageNameOffset: DWORD,
    dwTypeIDMainSize: DWORD,
    dwTypeIDMainOffset: DWORD,
    dwTypeNameMainOffset: DWORD,
    dwTypeIDMUISize: DWORD,
    dwTypeIDMUIOffset: DWORD,
    dwTypeNameMUIOffset: DWORD,
    abBuffer: [BYTE; 8],
}}
pub type PFILEMUIINFO = *mut FILEMUIINFO;
FN!{stdcall CALINFO_ENUMPROCEXEX(
    LPWSTR,
    CALID,
    LPWSTR,
    LPARAM,
) -> BOOL}
FN!{stdcall DATEFMT_ENUMPROCEXEX(
    LPWSTR,
    CALID,
    LPARAM,
) -> BOOL}
FN!{stdcall TIMEFMT_ENUMPROCEX(
    LPWSTR,
    LPARAM,
) -> BOOL}
FN!{stdcall LOCALE_ENUMPROCEX(
    LPWSTR,
    DWORD,
    LPARAM,
) -> BOOL}
extern "system" {
    pub fn CompareStringA(
        Locale: LCID,
        dwCmpFlags: DWORD,
        lpString1: PCNZCH,
        cchCount1: c_int,
        lpString2: PCNZCH,
        cchCount2: c_int,
    ) -> c_int;
    pub fn CompareStringEx(
        lpLocaleName: LPCWSTR,
        dwCmpFlags: DWORD,
        lpString1: LPCWCH,
        cchCount1: c_int,
        lpString2: LPCWCH,
        cchCount2: c_int,
        lpVersionInformation: LPNLSVERSIONINFO,
        lpReserved: LPVOID,
        lParam: LPARAM,
    ) -> c_int;
    pub fn CompareStringW(
        Locale: LCID,
        dwCmpFlags: DWORD,
        lpString1: PCNZWCH,
        cchCount1: c_int,
        lpString2: PCNZWCH,
        cchCount2: c_int,
    ) -> c_int;
    pub fn ConvertDefaultLocale(Locale: LCID) -> LCID;
    pub fn EnumCalendarInfoA(
        lpCalInfoEnumProc: CALINFO_ENUMPROCA,
        Locale: LCID,
        Calendar: CALID,
        CalType: CALTYPE,
    ) -> BOOL;
    pub fn EnumCalendarInfoExA(
        lpCalInfoEnumProcEx: CALINFO_ENUMPROCEXA,
        Locale: LCID,
        Calendar: CALID,
        CalType: CALTYPE,
    ) -> BOOL;
    pub fn EnumCalendarInfoExEx(
        pCalInfoEnumProcExEx: CALINFO_ENUMPROCEXEX,
        lpLocaleName: LPCWSTR,
        Calendar: CALID,
        lpReserved: LPCWSTR,
        CalType: CALTYPE,
        lParam: LPARAM,
    ) -> BOOL;
    pub fn EnumCalendarInfoExW(
        lpCalInfoEnumProcEx: CALINFO_ENUMPROCEXW,
        Locale: LCID,
        Calendar: CALID,
        CalType: CALTYPE,
    ) -> BOOL;
    pub fn EnumCalendarInfoW(
        lpCalInfoEnumProc: CALINFO_ENUMPROCW,
        Locale: LCID,
        Calendar: CALID,
        CalType: CALTYPE,
    ) -> BOOL;
    pub fn EnumDateFormatsA(
        lpDateFmtEnumProc: DATEFMT_ENUMPROCA,
        Locale: LCID,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn EnumDateFormatsExA(
        lpDateFmtEnumProcEx: DATEFMT_ENUMPROCEXA,
        Locale: LCID,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn EnumDateFormatsExEx(
        lpDateFmtEnumProcExEx: DATEFMT_ENUMPROCEXEX,
        lpLocaleName: LPCWSTR,
        dwFlags: DWORD,
        lParam: LPARAM,
    ) -> BOOL;
    pub fn EnumDateFormatsExW(
        lpDateFmtEnumProcEx: DATEFMT_ENUMPROCEXW,
        Locale: LCID,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn EnumDateFormatsW(
        lpDateFmtEnumProc: DATEFMT_ENUMPROCW,
        Locale: LCID,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn EnumLanguageGroupLocalesA(
        lpLangGroupLocaleEnumProc: LANGGROUPLOCALE_ENUMPROCA,
        LanguageGroup: LGRPID,
        dwFlags: DWORD,
        lParam: LONG_PTR,
    ) -> BOOL;
    pub fn EnumLanguageGroupLocalesW(
        lpLangGroupLocaleEnumProc: LANGGROUPLOCALE_ENUMPROCW,
        LanguageGroup: LGRPID,
        dwFlags: DWORD,
        lParam: LONG_PTR,
    ) -> BOOL;
    pub fn EnumSystemCodePagesA(
        lpCodePageEnumProc: CODEPAGE_ENUMPROCA,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn EnumSystemCodePagesW(
        lpCodePageEnumProc: CODEPAGE_ENUMPROCW,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn EnumSystemGeoID(
        GeoClass: GEOCLASS,
        ParentGeoId: GEOID,
        lpGeoEnumProc: GEO_ENUMPROC,
    ) -> BOOL;
    pub fn EnumSystemLanguageGroupsA(
        lpLanguageGroupEnumProc: LANGUAGEGROUP_ENUMPROCA,
        dwFlags: DWORD,
        lParam: LONG_PTR,
    ) -> BOOL;
    pub fn EnumSystemLanguageGroupsW(
        lpLanguageGroupEnumProc: LANGUAGEGROUP_ENUMPROCW,
        dwFlags: DWORD,
        lParam: LONG_PTR,
    ) -> BOOL;
    pub fn EnumSystemLocalesA(
        lpLocaleEnumProc: LOCALE_ENUMPROCA,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn EnumSystemLocalesEx(
        lpLocaleEnumProcEx: LOCALE_ENUMPROCEX,
        dwFlags: DWORD,
        lParam: LPARAM,
        lpReserved: LPVOID,
    ) -> BOOL;
    pub fn EnumSystemLocalesW(
        lpLocaleEnumProc: LOCALE_ENUMPROCW,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn EnumTimeFormatsA(
        lpTimeFmtEnumProc: TIMEFMT_ENUMPROCA,
        Locale: LCID,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn EnumTimeFormatsEx(
        lpTimeFmtEnumProcEx: TIMEFMT_ENUMPROCEX,
        lpLocaleName: LPCWSTR,
        dwFlags: DWORD,
        lParam: LPARAM,
    ) -> BOOL;
    pub fn EnumTimeFormatsW(
        lpTimeFmtEnumProc: TIMEFMT_ENUMPROCW,
        Locale: LCID,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn EnumUILanguagesA(
        lpUILanguageEnumProc: UILANGUAGE_ENUMPROCA,
        dwFlags: DWORD,
        lParam: LONG_PTR,
    ) -> BOOL;
    pub fn EnumUILanguagesW(
        lpUILanguageEnumProc: UILANGUAGE_ENUMPROCW,
        dwFlags: DWORD,
        lParam: LONG_PTR,
    ) -> BOOL;
    pub fn FindNLSString(
        Locale: LCID,
        dwFindNLSStringFlags: DWORD,
        lpStringSource: LPCWSTR,
        cchSource: c_int,
        lpStringValue: LPCWSTR,
        cchValue: c_int,
        pcchFound: LPINT,
    ) -> c_int;
    pub fn FindNLSStringEx(
        lpLocaleName: LPCWSTR,
        dwFindNLSStringFlags: DWORD,
        lpStringSource: LPCWSTR,
        cchSource: c_int,
        lpStringValue: LPCWSTR,
        cchValue: c_int,
        pcchFound: LPINT,
        lpVersionInformation: LPNLSVERSIONINFO,
        lpReserved: LPVOID,
        sortHandle: LPARAM,
    ) -> c_int;
    pub fn FoldStringA(
        dwMapFlags: DWORD,
        lpSrcStr: LPCSTR,
        cchSrc: c_int,
        lpDestStr: LPSTR,
        cchDest: c_int,
    ) -> c_int;
    pub fn GetACP() -> UINT;
    pub fn GetCPInfo(
        CodePage: UINT,
        lpCPInfo: LPCPINFO,
    ) -> BOOL;
    pub fn GetCPInfoExA(
        CodePage: UINT,
        dwFlags: DWORD,
        lpCPInfoEx: LPCPINFOEXA,
    ) -> BOOL;
    pub fn GetCPInfoExW(
        CodePage: UINT,
        dwFlags: DWORD,
        lpCPInfoEx: LPCPINFOEXW,
    ) -> BOOL;
    pub fn GetCalendarInfoA(
        Locale: LCID,
        Calendar: CALID,
        CalType: CALTYPE,
        lpCalData: LPSTR,
        cchData: c_int,
        lpValue: LPDWORD,
    ) -> c_int;
    pub fn GetCalendarInfoEx(
        lpLocaleName: LPCWSTR,
        Calendar: CALID,
        lpReserved: LPCWSTR,
        CalType: CALTYPE,
        lpCalData: LPWSTR,
        cchData: c_int,
        lpValue: LPDWORD,
    ) -> c_int;
    pub fn GetCalendarInfoW(
        Locale: LCID,
        Calendar: CALID,
        CalType: CALTYPE,
        lpCalData: LPWSTR,
        cchData: c_int,
        lpValue: LPDWORD,
    ) -> c_int;
    pub fn GetCurrencyFormatA(
        Locale: LCID,
        dwFlags: DWORD,
        lpValue: LPCSTR,
        lpFormat: *const CURRENCYFMTA,
        lpCurrencyStr: LPSTR,
        cchCurrency: c_int,
    ) -> c_int;
    pub fn GetCurrencyFormatEx(
        lpLocaleName: LPCWSTR,
        dwFlags: DWORD,
        lpValue: LPCWSTR,
        lpFormat: *const CURRENCYFMTW,
        lpCurrencyStr: LPWSTR,
        cchCurrency: c_int,
    ) -> c_int;
    pub fn GetCurrencyFormatW(
        Locale: LCID,
        dwFlags: DWORD,
        lpValue: LPCWSTR,
        lpFormat: *const CURRENCYFMTW,
        lpCurrencyStr: LPWSTR,
        cchCurrency: c_int,
    ) -> c_int;
    pub fn GetDurationFormat(
        Locale: LCID,
        dwFlags: DWORD,
        lpDuration: *const SYSTEMTIME,
        ullDuration: ULONGLONG,
        lpFormat: LPCWSTR,
        lpDurationStr: LPWSTR,
        cchDuration: c_int,
    ) -> c_int;
    pub fn GetDurationFormatEx(
        lpLocaleName: LPCWSTR,
        dwFlags: DWORD,
        lpDuration: *const SYSTEMTIME,
        ullDuration: ULONGLONG,
        lpFormat: LPCWSTR,
        lpDurationStr: LPWSTR,
        cchDuration: c_int,
    ) -> c_int;
    pub fn GetFileMUIInfo(
        dwFlags: DWORD,
        pcwszFilePath: PCWSTR,
        pFileMUIInfo: PFILEMUIINFO,
        pcbFileMUIInfo: *mut DWORD,
    ) -> BOOL;
    pub fn GetFileMUIPath(
        dwFlags: DWORD,
        pcwszFilePath: PCWSTR,
        pwszLanguage: PWSTR,
        pcchLanguage: PULONG,
        pwszFileMUIPath: PWSTR,
        pcchFileMUIPath: PULONG,
        pululEnumerator: PULONGLONG,
    ) -> BOOL;
    pub fn GetGeoInfoA(
        Location: GEOID,
        GeoType: GEOTYPE,
        lpGeoData: LPSTR,
        cchData: c_int,
        LangId: LANGID,
    ) -> c_int;
    pub fn GetGeoInfoW(
        Location: GEOID,
        GeoType: GEOTYPE,
        lpGeoData: LPWSTR,
        cchData: c_int,
        LangId: LANGID,
    ) -> c_int;
    pub fn GetLocaleInfoA(
        Locale: LCID,
        LCType: LCTYPE,
        lpLCData: LPSTR,
        cchData: c_int,
    ) -> c_int;
    pub fn GetLocaleInfoEx(
        lpLocaleName: LPCWSTR,
        LCType: LCTYPE,
        lpLCData: LPWSTR,
        cchData: c_int,
    ) -> c_int;
    pub fn GetLocaleInfoW(
        Locale: LCID,
        LCType: LCTYPE,
        lpLCData: LPWSTR,
        cchData: c_int,
    ) -> c_int;
    pub fn GetNLSVersion(
        Function: NLS_FUNCTION,
        Locale: LCID,
        lpVersionInformation: LPNLSVERSIONINFO,
    ) -> BOOL;
    pub fn GetNLSVersionEx(
        function: NLS_FUNCTION,
        lpLocaleName: LPCWSTR,
        lpVersionInformation: LPNLSVERSIONINFOEX,
    ) -> BOOL;
    pub fn GetNumberFormatA(
        Locale: LCID,
        dwFlags: DWORD,
        lpValue: LPCSTR,
        lpFormat: *const NUMBERFMTA,
        lpNumberStr: LPSTR,
        cchNumber: c_int,
    ) -> c_int;
    pub fn GetNumberFormatEx(
        lpLocaleName: LPCWSTR,
        dwFlags: DWORD,
        lpValue: LPCWSTR,
        lpFormat: *const NUMBERFMTW,
        lpNumberStr: LPWSTR,
        cchNumber: c_int,
    ) -> c_int;
    pub fn GetNumberFormatW(
        Locale: LCID,
        dwFlags: DWORD,
        lpValue: LPCWSTR,
        lpFormat: *const NUMBERFMTW,
        lpNumberStr: LPWSTR,
        cchNumber: c_int,
    ) -> c_int;
    pub fn GetOEMCP() -> UINT;
    pub fn GetProcessPreferredUILanguages(
        dwFlags: DWORD,
        pulNumLanguages: PULONG,
        pwszLanguagesBuffer: PZZWSTR,
        pcchLanguagesBuffer: PULONG,
    ) -> BOOL;
    pub fn GetStringScripts(
        dwFlags: DWORD,
        lpString: LPCWSTR,
        cchString: c_int,
        lpScripts: LPWSTR,
        cchScripts: c_int,
    ) -> c_int;
    pub fn GetStringTypeA(
        Locale: LCID,
        dwInfoType: DWORD,
        lpSrcStr: LPCSTR,
        cchSrc: c_int,
        lpCharType: LPWORD,
    ) -> BOOL;
    pub fn GetStringTypeExA(
        Locale: LCID,
        dwInfoType: DWORD,
        lpSrcStr: LPCSTR,
        cchSrc: c_int,
        lpCharType: LPWORD,
    ) -> BOOL;
    pub fn GetStringTypeW(
        dwInfoType: DWORD,
        lpSrcStr: LPCWCH,
        cchSrc: c_int,
        lpCharType: LPWORD,
    ) -> BOOL;
    pub fn GetSystemDefaultLCID() -> LCID;
    pub fn GetSystemDefaultLangID() -> LANGID;
    pub fn GetSystemDefaultLocaleName(
        lpLocaleName: LPWSTR,
        cchLocaleName: c_int,
    ) -> c_int;
    pub fn GetSystemDefaultUILanguage() -> LANGID;
    pub fn GetSystemPreferredUILanguages(
        dwFlags: DWORD,
        pulNumLanguages: PULONG,
        pwszLanguagesBuffer: PZZWSTR,
        pcchLanguagesBuffer: PULONG,
    ) -> BOOL;
    pub fn GetThreadLocale() -> LCID;
    pub fn GetThreadPreferredUILanguages(
        dwFlags: DWORD,
        pulNumLanguages: PULONG,
        pwszLanguagesBuffer: PZZWSTR,
        pcchLanguagesBuffer: PULONG,
    ) -> BOOL;
    pub fn GetThreadUILanguage() -> LANGID;
    pub fn GetUILanguageInfo(
        dwFlags: DWORD,
        pwmszLanguage: PCZZWSTR,
        pwszFallbackLanguages: PZZWSTR,
        pcchFallbackLanguages: PDWORD,
        pAttributes: PDWORD,
    ) -> BOOL;
    pub fn GetUserDefaultLCID() -> LCID;
    pub fn GetUserDefaultLangID() -> LANGID;
    pub fn GetUserDefaultLocaleName(
        lpLocaleName: LPWSTR,
        cchLocaleName: c_int,
    ) -> c_int;
    pub fn GetUserDefaultUILanguage() -> LANGID;
    pub fn GetUserGeoID(GeoClass: GEOCLASS) -> GEOID;
    pub fn GetUserPreferredUILanguages(
        dwFlags: DWORD,
        pulNumLanguages: PULONG,
        pwszLanguagesBuffer: PZZWSTR,
        pcchLanguagesBuffer: PULONG,
    ) -> BOOL;
    pub fn IsDBCSLeadByte(
        TestChar: BYTE,
    ) -> BOOL;
    pub fn IsDBCSLeadByteEx(
        CodePage: UINT,
        TestChar: BYTE,
    ) -> BOOL;
    pub fn IsNLSDefinedString(
        Function: NLS_FUNCTION,
        dwFlags: DWORD,
        lpVersionInformation: LPNLSVERSIONINFO,
        lpString: LPCWSTR,
        cchStr: INT,
    ) -> BOOL;
    pub fn IsNormalizedString(
        NormForm: NORM_FORM,
        lpString: LPCWSTR,
        cwLength: c_int,
    ) -> BOOL;
    pub fn IsValidCodePage(
        CodePage: UINT,
    ) -> BOOL;
    pub fn IsValidLanguageGroup(
        LanguageGroup: LGRPID,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn IsValidLocale(
        Locale: LCID,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn IsValidLocaleName(
        lpLocaleName: LPCWSTR,
    ) -> BOOL;
    pub fn IsValidNLSVersion(
        function: NLS_FUNCTION,
        lpLocaleName: LPCWSTR,
        lpVersionInformation: LPNLSVERSIONINFOEX,
    ) -> BOOL;
    pub fn LCIDToLocaleName(
        Locale: LCID,
        lpName: LPWSTR,
        cchName: c_int,
        dwFlags: DWORD,
    ) -> c_int;
    pub fn LCMapStringA(
        Locale: LCID,
        dwMapFlags: DWORD,
        lpSrcStr: LPCSTR,
        cchSrc: c_int,
        lpDestStr: LPSTR,
        cchDest: c_int,
    ) -> c_int;
    pub fn LCMapStringEx(
        lpLocaleName: LPCWSTR,
        dwMapFlags: DWORD,
        lpSrcStr: LPCWSTR,
        cchSrc: c_int,
        lpDestStr: LPWSTR,
        cchDest: c_int,
        lpVersionInformation: LPNLSVERSIONINFO,
        lpReserved: LPVOID,
        sortHandle: LPARAM,
    ) -> c_int;
    pub fn LCMapStringW(
        Locale: LCID,
        dwMapFlags: DWORD,
        lpSrcStr: LPCWSTR,
        cchSrc: c_int,
        lpDestStr: LPWSTR,
        cchDest: c_int,
    ) -> c_int;
    pub fn LocaleNameToLCID(
        lpName: LPCWSTR,
        dwFlags: DWORD,
    ) -> LCID;
    pub fn NormalizeString(
        NormForm: NORM_FORM,
        lpSrcString: LPCWSTR,
        cwSrcLength: c_int,
        lpDstString: LPWSTR,
        cwDstLength: c_int,
    ) -> c_int;
    pub fn NotifyUILanguageChange(
        dwFlags: DWORD,
        pcwstrNewLanguage: PCWSTR,
        pcwstrPreviousLanguage: PCWSTR,
        dwReserved: DWORD,
        pdwStatusRtrn: PDWORD,
    ) -> BOOL;
    pub fn ResolveLocaleName(
        lpNameToResolve: LPCWSTR,
        lpLocaleName: LPWSTR,
        cchLocaleName: c_int,
    ) -> c_int;
    pub fn SetCalendarInfoA(
        Locale: LCID,
        Calendar: CALID,
        CalType: CALTYPE,
        lpCalData: LPCSTR,
    ) -> BOOL;
    pub fn SetCalendarInfoW(
        Locale: LCID,
        Calendar: CALID,
        CalType: CALTYPE,
        lpCalData: LPCWSTR,
    ) -> BOOL;
    pub fn SetLocaleInfoA(
        Locale: LCID,
        LCType: LCTYPE,
        lpLCData: LPCSTR,
    ) -> BOOL;
    pub fn SetLocaleInfoW(
        Locale: LCID,
        LCType: LCTYPE,
        lpLCData: LPCWSTR,
    ) -> BOOL;
    pub fn SetProcessPreferredUILanguages(
        dwFlags: DWORD,
        pwszLanguagesBuffer: PCZZWSTR,
        pulNumLanguages: PULONG,
    ) -> BOOL;
    pub fn SetThreadLocale(Locale: LCID) -> BOOL;
    pub fn SetThreadPreferredUILanguages(
        dwFlags: DWORD,
        pwszLanguagesBuffer: PCZZWSTR,
        pulNumLanguages: PULONG,
    ) -> BOOL;
    pub fn SetThreadUILanguage(LangId: LANGID) -> LANGID;
    pub fn SetUserGeoID(GeoId: GEOID) -> BOOL;
    pub fn VerifyScripts(
        dwFlags: DWORD,
        lpLocaleScripts: LPCWSTR,
        cchLocaleScripts: c_int,
        lpTestScripts: LPCWSTR,
        cchTestScripts: c_int,
    ) -> BOOL;
}
