#[inline]
pub unsafe fn HtmlHelpA<P1>(hwndcaller: Option<super::super::Foundation::HWND>, pszfile: P1, ucommand: HTML_HELP_COMMAND, dwdata: usize) -> super::super::Foundation::HWND
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("hhctrl.ocx" "system" fn HtmlHelpA(hwndcaller : super::super::Foundation:: HWND, pszfile : windows_core::PCSTR, ucommand : u32, dwdata : usize) -> super::super::Foundation:: HWND);
    unsafe { HtmlHelpA(hwndcaller.unwrap_or(core::mem::zeroed()) as _, pszfile.param().abi(), ucommand.0 as _, dwdata) }
}
#[inline]
pub unsafe fn HtmlHelpW<P1>(hwndcaller: Option<super::super::Foundation::HWND>, pszfile: P1, ucommand: HTML_HELP_COMMAND, dwdata: usize) -> super::super::Foundation::HWND
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("hhctrl.ocx" "system" fn HtmlHelpW(hwndcaller : super::super::Foundation:: HWND, pszfile : windows_core::PCWSTR, ucommand : u32, dwdata : usize) -> super::super::Foundation:: HWND);
    unsafe { HtmlHelpW(hwndcaller.unwrap_or(core::mem::zeroed()) as _, pszfile.param().abi(), ucommand.0 as _, dwdata) }
}
pub const CLSID_IITCmdInt: windows_core::GUID = windows_core::GUID::from_u128(0x4662daa2_d393_11d0_9a56_00c04fb68bf7);
pub const CLSID_IITDatabase: windows_core::GUID = windows_core::GUID::from_u128(0x66673452_8c23_11d0_a84e_00aa006c7d01);
pub const CLSID_IITDatabaseLocal: windows_core::GUID = windows_core::GUID::from_u128(0x4662daa9_d393_11d0_9a56_00c04fb68bf7);
pub const CLSID_IITGroupUpdate: windows_core::GUID = windows_core::GUID::from_u128(0x4662daa4_d393_11d0_9a56_00c04fb68bf7);
pub const CLSID_IITIndexBuild: windows_core::GUID = windows_core::GUID::from_u128(0x8fa0d5aa_dedf_11d0_9a61_00c04fb68bf7);
pub const CLSID_IITPropList: windows_core::GUID = windows_core::GUID::from_u128(0x4662daae_d393_11d0_9a56_00c04fb68bf7);
pub const CLSID_IITResultSet: windows_core::GUID = windows_core::GUID::from_u128(0x4662daa7_d393_11d0_9a56_00c04fb68bf7);
pub const CLSID_IITSvMgr: windows_core::GUID = windows_core::GUID::from_u128(0x4662daa3_d393_11d0_9a56_00c04fb68bf7);
pub const CLSID_IITWWFilterBuild: windows_core::GUID = windows_core::GUID::from_u128(0x8fa0d5ab_dedf_11d0_9a61_00c04fb68bf7);
pub const CLSID_IITWordWheel: windows_core::GUID = windows_core::GUID::from_u128(0xd73725c2_8c12_11d0_a84e_00aa006c7d01);
pub const CLSID_IITWordWheelLocal: windows_core::GUID = windows_core::GUID::from_u128(0x4662daa8_d393_11d0_9a56_00c04fb68bf7);
pub const CLSID_IITWordWheelUpdate: windows_core::GUID = windows_core::GUID::from_u128(0x4662daa5_d393_11d0_9a56_00c04fb68bf7);
pub const CLSID_ITEngStemmer: windows_core::GUID = windows_core::GUID::from_u128(0x8fa0d5a8_dedf_11d0_9a61_00c04fb68bf7);
pub const CLSID_ITStdBreaker: windows_core::GUID = windows_core::GUID::from_u128(0x4662daaf_d393_11d0_9a56_00c04fb68bf7);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct COLUMNSTATUS {
    pub cPropCount: i32,
    pub cPropsLoaded: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CProperty {
    pub dwPropID: u32,
    pub cbData: u32,
    pub dwType: u32,
    pub Anonymous: CProperty_0,
    pub fPersist: windows_core::BOOL,
}
impl Default for CProperty {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CProperty_0 {
    pub lpszwData: windows_core::PWSTR,
    pub lpvData: *mut core::ffi::c_void,
    pub dwValue: u32,
}
impl Default for CProperty_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const E_ALL_WILD: windows_core::HRESULT = windows_core::HRESULT(0x80001055_u32 as _);
pub const E_ALREADYINIT: windows_core::HRESULT = windows_core::HRESULT(0x80001083_u32 as _);
pub const E_ALREADYOPEN: windows_core::HRESULT = windows_core::HRESULT(0x80001013_u32 as _);
pub const E_ASSERT: windows_core::HRESULT = windows_core::HRESULT(0x80001006_u32 as _);
pub const E_BADBREAKER: windows_core::HRESULT = windows_core::HRESULT(0x80001053_u32 as _);
pub const E_BADFILE: windows_core::HRESULT = windows_core::HRESULT(0x80001003_u32 as _);
pub const E_BADFILTERSIZE: windows_core::HRESULT = windows_core::HRESULT(0x80001018_u32 as _);
pub const E_BADFORMAT: windows_core::HRESULT = windows_core::HRESULT(0x80001004_u32 as _);
pub const E_BADINDEXFLAGS: windows_core::HRESULT = windows_core::HRESULT(0x80001060_u32 as _);
pub const E_BADPARAM: windows_core::HRESULT = windows_core::HRESULT(0x80001011_u32 as _);
pub const E_BADRANGEOP: windows_core::HRESULT = windows_core::HRESULT(0x8000105D_u32 as _);
pub const E_BADVALUE: windows_core::HRESULT = windows_core::HRESULT(0x80001054_u32 as _);
pub const E_BADVERSION: windows_core::HRESULT = windows_core::HRESULT(0x80001002_u32 as _);
pub const E_CANTFINDDLL: windows_core::HRESULT = windows_core::HRESULT(0x8000100E_u32 as _);
pub const E_DISKFULL: windows_core::HRESULT = windows_core::HRESULT(0x80001038_u32 as _);
pub const E_DUPLICATE: windows_core::HRESULT = windows_core::HRESULT(0x80001001_u32 as _);
pub const E_EXPECTEDTERM: windows_core::HRESULT = windows_core::HRESULT(0x80001057_u32 as _);
pub const E_FILECLOSE: windows_core::HRESULT = windows_core::HRESULT(0x80001031_u32 as _);
pub const E_FILECREATE: windows_core::HRESULT = windows_core::HRESULT(0x80001030_u32 as _);
pub const E_FILEDELETE: windows_core::HRESULT = windows_core::HRESULT(0x80001035_u32 as _);
pub const E_FILEINVALID: windows_core::HRESULT = windows_core::HRESULT(0x80001036_u32 as _);
pub const E_FILENOTFOUND: windows_core::HRESULT = windows_core::HRESULT(0x80001037_u32 as _);
pub const E_FILEREAD: windows_core::HRESULT = windows_core::HRESULT(0x80001032_u32 as _);
pub const E_FILESEEK: windows_core::HRESULT = windows_core::HRESULT(0x80001033_u32 as _);
pub const E_FILEWRITE: windows_core::HRESULT = windows_core::HRESULT(0x80001034_u32 as _);
pub const E_GETLASTERROR: windows_core::HRESULT = windows_core::HRESULT(0x80001010_u32 as _);
pub const E_GROUPIDTOOBIG: windows_core::HRESULT = windows_core::HRESULT(0x8000100A_u32 as _);
pub const E_INTERRUPT: windows_core::HRESULT = windows_core::HRESULT(0x80001007_u32 as _);
pub const E_INVALIDSTATE: windows_core::HRESULT = windows_core::HRESULT(0x80001012_u32 as _);
pub const E_MISSINGPROP: windows_core::HRESULT = windows_core::HRESULT(0x80001080_u32 as _);
pub const E_MISSLPAREN: windows_core::HRESULT = windows_core::HRESULT(0x80001058_u32 as _);
pub const E_MISSQUOTE: windows_core::HRESULT = windows_core::HRESULT(0x8000105A_u32 as _);
pub const E_MISSRPAREN: windows_core::HRESULT = windows_core::HRESULT(0x80001059_u32 as _);
pub const E_NAMETOOLONG: windows_core::HRESULT = windows_core::HRESULT(0x80001020_u32 as _);
pub const E_NOHANDLE: windows_core::HRESULT = windows_core::HRESULT(0x8000100F_u32 as _);
pub const E_NOKEYPROP: windows_core::HRESULT = windows_core::HRESULT(0x80001087_u32 as _);
pub const E_NOMERGEDDATA: windows_core::HRESULT = windows_core::HRESULT(0x8000100C_u32 as _);
pub const E_NOPERMISSION: windows_core::HRESULT = windows_core::HRESULT(0x80001005_u32 as _);
pub const E_NOSTEMMER: windows_core::HRESULT = windows_core::HRESULT(0x80001062_u32 as _);
pub const E_NOTEXIST: windows_core::HRESULT = windows_core::HRESULT(0x80001000_u32 as _);
pub const E_NOTFOUND: windows_core::HRESULT = windows_core::HRESULT(0x8000100D_u32 as _);
pub const E_NOTINIT: windows_core::HRESULT = windows_core::HRESULT(0x80001084_u32 as _);
pub const E_NOTOPEN: windows_core::HRESULT = windows_core::HRESULT(0x80001013_u32 as _);
pub const E_NOTSUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80001008_u32 as _);
pub const E_NULLQUERY: windows_core::HRESULT = windows_core::HRESULT(0x8000105B_u32 as _);
pub const E_OUTOFRANGE: windows_core::HRESULT = windows_core::HRESULT(0x80001009_u32 as _);
pub const E_PROPLISTEMPTY: windows_core::HRESULT = windows_core::HRESULT(0x80001082_u32 as _);
pub const E_PROPLISTNOTEMPTY: windows_core::HRESULT = windows_core::HRESULT(0x80001081_u32 as _);
pub const E_RESULTSETEMPTY: windows_core::HRESULT = windows_core::HRESULT(0x80001085_u32 as _);
pub const E_STOPWORD: windows_core::HRESULT = windows_core::HRESULT(0x8000105C_u32 as _);
pub const E_TOODEEP: windows_core::HRESULT = windows_core::HRESULT(0x80001056_u32 as _);
pub const E_TOOMANYCOLUMNS: windows_core::HRESULT = windows_core::HRESULT(0x80001086_u32 as _);
pub const E_TOOMANYDUPS: windows_core::HRESULT = windows_core::HRESULT(0x80001051_u32 as _);
pub const E_TOOMANYOBJECTS: windows_core::HRESULT = windows_core::HRESULT(0x80001019_u32 as _);
pub const E_TOOMANYTITLES: windows_core::HRESULT = windows_core::HRESULT(0x8000100B_u32 as _);
pub const E_TOOMANYTOPICS: windows_core::HRESULT = windows_core::HRESULT(0x80001050_u32 as _);
pub const E_TREETOOBIG: windows_core::HRESULT = windows_core::HRESULT(0x80001052_u32 as _);
pub const E_UNKNOWN_TRANSPORT: windows_core::HRESULT = windows_core::HRESULT(0x80001016_u32 as _);
pub const E_UNMATCHEDTYPE: windows_core::HRESULT = windows_core::HRESULT(0x8000105E_u32 as _);
pub const E_UNSUPPORTED_TRANSPORT: windows_core::HRESULT = windows_core::HRESULT(0x80001017_u32 as _);
pub const E_WILD_IN_DTYPE: windows_core::HRESULT = windows_core::HRESULT(0x80001061_u32 as _);
pub const E_WORDTOOLONG: windows_core::HRESULT = windows_core::HRESULT(0x8000105F_u32 as _);
pub const HHACT_BACK: i32 = 7i32;
pub const HHACT_CONTRACT: i32 = 6i32;
pub const HHACT_CUSTOMIZE: i32 = 16i32;
pub const HHACT_EXPAND: i32 = 5i32;
pub const HHACT_FORWARD: i32 = 8i32;
pub const HHACT_HIGHLIGHT: i32 = 15i32;
pub const HHACT_HOME: i32 = 11i32;
pub const HHACT_JUMP1: i32 = 17i32;
pub const HHACT_JUMP2: i32 = 18i32;
pub const HHACT_LAST_ENUM: i32 = 23i32;
pub const HHACT_NOTES: i32 = 22i32;
pub const HHACT_OPTIONS: i32 = 13i32;
pub const HHACT_PRINT: i32 = 14i32;
pub const HHACT_REFRESH: i32 = 10i32;
pub const HHACT_STOP: i32 = 9i32;
pub const HHACT_SYNC: i32 = 12i32;
pub const HHACT_TAB_CONTENTS: i32 = 0i32;
pub const HHACT_TAB_FAVORITES: i32 = 4i32;
pub const HHACT_TAB_HISTORY: i32 = 3i32;
pub const HHACT_TAB_INDEX: i32 = 1i32;
pub const HHACT_TAB_SEARCH: i32 = 2i32;
pub const HHACT_TOC_NEXT: i32 = 20i32;
pub const HHACT_TOC_PREV: i32 = 21i32;
pub const HHACT_ZOOM: i32 = 19i32;
#[repr(C)]
#[cfg(feature = "Win32_UI_Controls")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HHNTRACK {
    pub hdr: super::super::UI::Controls::NMHDR,
    pub pszCurUrl: windows_core::PCSTR,
    pub idAction: i32,
    pub phhWinType: *mut HH_WINTYPE,
}
#[cfg(feature = "Win32_UI_Controls")]
impl Default for HHNTRACK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HHN_FIRST: u32 = 4294966436u32;
pub const HHN_LAST: u32 = 4294966417u32;
pub const HHN_NAVCOMPLETE: u32 = 4294966436u32;
#[repr(C)]
#[cfg(feature = "Win32_UI_Controls")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HHN_NOTIFY {
    pub hdr: super::super::UI::Controls::NMHDR,
    pub pszUrl: windows_core::PCSTR,
}
pub const HHN_TRACK: u32 = 4294966435u32;
pub const HHN_WINDOW_CREATE: u32 = 4294966434u32;
pub const HHWIN_BUTTON_BACK: u32 = 4u32;
pub const HHWIN_BUTTON_BROWSE_BCK: u32 = 256u32;
pub const HHWIN_BUTTON_BROWSE_FWD: u32 = 128u32;
pub const HHWIN_BUTTON_CONTENTS: u32 = 1024u32;
pub const HHWIN_BUTTON_EXPAND: u32 = 2u32;
pub const HHWIN_BUTTON_FAVORITES: u32 = 131072u32;
pub const HHWIN_BUTTON_FORWARD: u32 = 8u32;
pub const HHWIN_BUTTON_HISTORY: u32 = 65536u32;
pub const HHWIN_BUTTON_HOME: u32 = 64u32;
pub const HHWIN_BUTTON_INDEX: u32 = 16384u32;
pub const HHWIN_BUTTON_JUMP1: u32 = 262144u32;
pub const HHWIN_BUTTON_JUMP2: u32 = 524288u32;
pub const HHWIN_BUTTON_NOTES: u32 = 512u32;
pub const HHWIN_BUTTON_OPTIONS: u32 = 4096u32;
pub const HHWIN_BUTTON_PRINT: u32 = 8192u32;
pub const HHWIN_BUTTON_REFRESH: u32 = 32u32;
pub const HHWIN_BUTTON_SEARCH: u32 = 32768u32;
pub const HHWIN_BUTTON_STOP: u32 = 16u32;
pub const HHWIN_BUTTON_SYNC: u32 = 2048u32;
pub const HHWIN_BUTTON_TOC_NEXT: u32 = 2097152u32;
pub const HHWIN_BUTTON_TOC_PREV: u32 = 4194304u32;
pub const HHWIN_BUTTON_ZOOM: u32 = 1048576u32;
pub const HHWIN_NAVTAB_BOTTOM: i32 = 2i32;
pub const HHWIN_NAVTAB_LEFT: i32 = 1i32;
pub const HHWIN_NAVTAB_TOP: i32 = 0i32;
pub const HHWIN_NAVTYPE_AUTHOR: i32 = 5i32;
pub const HHWIN_NAVTYPE_CUSTOM_FIRST: i32 = 11i32;
pub const HHWIN_NAVTYPE_FAVORITES: i32 = 3i32;
pub const HHWIN_NAVTYPE_HISTORY: i32 = 4i32;
pub const HHWIN_NAVTYPE_INDEX: i32 = 1i32;
pub const HHWIN_NAVTYPE_SEARCH: i32 = 2i32;
pub const HHWIN_NAVTYPE_TOC: i32 = 0i32;
pub const HHWIN_PARAM_CUR_TAB: u32 = 8192u32;
pub const HHWIN_PARAM_EXPANSION: u32 = 512u32;
pub const HHWIN_PARAM_EXSTYLES: u32 = 8u32;
pub const HHWIN_PARAM_HISTORY_COUNT: u32 = 4096u32;
pub const HHWIN_PARAM_INFOTYPES: u32 = 128u32;
pub const HHWIN_PARAM_NAV_WIDTH: u32 = 32u32;
pub const HHWIN_PARAM_PROPERTIES: u32 = 2u32;
pub const HHWIN_PARAM_RECT: u32 = 16u32;
pub const HHWIN_PARAM_SHOWSTATE: u32 = 64u32;
pub const HHWIN_PARAM_STYLES: u32 = 4u32;
pub const HHWIN_PARAM_TABORDER: u32 = 2048u32;
pub const HHWIN_PARAM_TABPOS: u32 = 1024u32;
pub const HHWIN_PARAM_TB_FLAGS: u32 = 256u32;
pub const HHWIN_PROP_AUTO_SYNC: u32 = 256u32;
pub const HHWIN_PROP_CHANGE_TITLE: u32 = 8192u32;
pub const HHWIN_PROP_MENU: u32 = 65536u32;
pub const HHWIN_PROP_NAV_ONLY_WIN: u32 = 16384u32;
pub const HHWIN_PROP_NODEF_EXSTYLES: u32 = 16u32;
pub const HHWIN_PROP_NODEF_STYLES: u32 = 8u32;
pub const HHWIN_PROP_NOTB_TEXT: u32 = 64u32;
pub const HHWIN_PROP_NOTITLEBAR: u32 = 4u32;
pub const HHWIN_PROP_NO_TOOLBAR: u32 = 32768u32;
pub const HHWIN_PROP_ONTOP: u32 = 2u32;
pub const HHWIN_PROP_POST_QUIT: u32 = 128u32;
pub const HHWIN_PROP_TAB_ADVSEARCH: u32 = 131072u32;
pub const HHWIN_PROP_TAB_AUTOHIDESHOW: u32 = 1u32;
pub const HHWIN_PROP_TAB_CUSTOM1: u32 = 524288u32;
pub const HHWIN_PROP_TAB_CUSTOM2: u32 = 1048576u32;
pub const HHWIN_PROP_TAB_CUSTOM3: u32 = 2097152u32;
pub const HHWIN_PROP_TAB_CUSTOM4: u32 = 4194304u32;
pub const HHWIN_PROP_TAB_CUSTOM5: u32 = 8388608u32;
pub const HHWIN_PROP_TAB_CUSTOM6: u32 = 16777216u32;
pub const HHWIN_PROP_TAB_CUSTOM7: u32 = 33554432u32;
pub const HHWIN_PROP_TAB_CUSTOM8: u32 = 67108864u32;
pub const HHWIN_PROP_TAB_CUSTOM9: u32 = 134217728u32;
pub const HHWIN_PROP_TAB_FAVORITES: u32 = 4096u32;
pub const HHWIN_PROP_TAB_HISTORY: u32 = 2048u32;
pub const HHWIN_PROP_TAB_SEARCH: u32 = 1024u32;
pub const HHWIN_PROP_TRACKING: u32 = 512u32;
pub const HHWIN_PROP_TRI_PANE: u32 = 32u32;
pub const HHWIN_PROP_USER_POS: u32 = 262144u32;
pub const HHWIN_TB_MARGIN: u32 = 268435456u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HH_AKLINK {
    pub cbStruct: i32,
    pub fReserved: windows_core::BOOL,
    pub pszKeywords: *mut i8,
    pub pszUrl: *mut i8,
    pub pszMsgText: *mut i8,
    pub pszMsgTitle: *mut i8,
    pub pszWindow: *mut i8,
    pub fIndexOnFail: windows_core::BOOL,
}
impl Default for HH_AKLINK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HH_ALINK_LOOKUP: HTML_HELP_COMMAND = HTML_HELP_COMMAND(19i32);
pub const HH_CLOSE_ALL: HTML_HELP_COMMAND = HTML_HELP_COMMAND(18i32);
pub const HH_DISPLAY_INDEX: HTML_HELP_COMMAND = HTML_HELP_COMMAND(2i32);
pub const HH_DISPLAY_SEARCH: HTML_HELP_COMMAND = HTML_HELP_COMMAND(3i32);
pub const HH_DISPLAY_TEXT_POPUP: HTML_HELP_COMMAND = HTML_HELP_COMMAND(14i32);
pub const HH_DISPLAY_TOC: HTML_HELP_COMMAND = HTML_HELP_COMMAND(1i32);
pub const HH_DISPLAY_TOPIC: HTML_HELP_COMMAND = HTML_HELP_COMMAND(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HH_ENUM_CAT {
    pub cbStruct: i32,
    pub pszCatName: windows_core::PCSTR,
    pub pszCatDescription: windows_core::PCSTR,
}
pub const HH_ENUM_CATEGORY: HTML_HELP_COMMAND = HTML_HELP_COMMAND(21i32);
pub const HH_ENUM_CATEGORY_IT: HTML_HELP_COMMAND = HTML_HELP_COMMAND(22i32);
pub const HH_ENUM_INFO_TYPE: HTML_HELP_COMMAND = HTML_HELP_COMMAND(7i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HH_ENUM_IT {
    pub cbStruct: i32,
    pub iType: i32,
    pub pszCatName: windows_core::PCSTR,
    pub pszITName: windows_core::PCSTR,
    pub pszITDescription: windows_core::PCSTR,
}
pub const HH_FTS_DEFAULT_PROXIMITY: HTML_HELP_COMMAND = HTML_HELP_COMMAND(-1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HH_FTS_QUERY {
    pub cbStruct: i32,
    pub fUniCodeStrings: windows_core::BOOL,
    pub pszSearchQuery: *mut i8,
    pub iProximity: i32,
    pub fStemmedSearch: windows_core::BOOL,
    pub fTitleOnly: windows_core::BOOL,
    pub fExecute: windows_core::BOOL,
    pub pszWindow: *mut i8,
}
impl Default for HH_FTS_QUERY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HH_GET_LAST_ERROR: HTML_HELP_COMMAND = HTML_HELP_COMMAND(20i32);
pub const HH_GET_WIN_HANDLE: HTML_HELP_COMMAND = HTML_HELP_COMMAND(6i32);
pub const HH_GET_WIN_TYPE: HTML_HELP_COMMAND = HTML_HELP_COMMAND(5i32);
#[repr(C)]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub struct HH_GLOBAL_PROPERTY {
    pub id: HH_GPROPID,
    pub var: super::super::System::Variant::VARIANT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Clone for HH_GLOBAL_PROPERTY {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Default for HH_GLOBAL_PROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HH_GPROPID(pub i32);
pub const HH_GPROPID_CONTENT_LANGUAGE: HH_GPROPID = HH_GPROPID(5i32);
pub const HH_GPROPID_CURRENT_SUBSET: HH_GPROPID = HH_GPROPID(4i32);
pub const HH_GPROPID_SINGLETHREAD: HH_GPROPID = HH_GPROPID(1i32);
pub const HH_GPROPID_TOOLBAR_MARGIN: HH_GPROPID = HH_GPROPID(2i32);
pub const HH_GPROPID_UI_LANGUAGE: HH_GPROPID = HH_GPROPID(3i32);
pub const HH_HELP_CONTEXT: HTML_HELP_COMMAND = HTML_HELP_COMMAND(15i32);
pub const HH_HELP_FINDER: HTML_HELP_COMMAND = HTML_HELP_COMMAND(0i32);
pub const HH_INITIALIZE: HTML_HELP_COMMAND = HTML_HELP_COMMAND(28i32);
pub const HH_KEYWORD_LOOKUP: HTML_HELP_COMMAND = HTML_HELP_COMMAND(13i32);
pub const HH_MAX_TABS: HTML_HELP_COMMAND = HTML_HELP_COMMAND(19i32);
pub const HH_MAX_TABS_CUSTOM: HTML_HELP_COMMAND = HTML_HELP_COMMAND(9i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HH_POPUP {
    pub cbStruct: i32,
    pub hinst: super::super::Foundation::HINSTANCE,
    pub idString: u32,
    pub pszText: *mut i8,
    pub pt: super::super::Foundation::POINT,
    pub clrForeground: super::super::Foundation::COLORREF,
    pub clrBackground: super::super::Foundation::COLORREF,
    pub rcMargins: super::super::Foundation::RECT,
    pub pszFont: *mut i8,
}
impl Default for HH_POPUP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HH_PRETRANSLATEMESSAGE: HTML_HELP_COMMAND = HTML_HELP_COMMAND(253i32);
pub const HH_RESERVED1: HTML_HELP_COMMAND = HTML_HELP_COMMAND(10i32);
pub const HH_RESERVED2: HTML_HELP_COMMAND = HTML_HELP_COMMAND(11i32);
pub const HH_RESERVED3: HTML_HELP_COMMAND = HTML_HELP_COMMAND(12i32);
pub const HH_RESET_IT_FILTER: HTML_HELP_COMMAND = HTML_HELP_COMMAND(23i32);
pub const HH_SAFE_DISPLAY_TOPIC: HTML_HELP_COMMAND = HTML_HELP_COMMAND(32i32);
pub const HH_SET_EXCLUSIVE_FILTER: HTML_HELP_COMMAND = HTML_HELP_COMMAND(25i32);
pub const HH_SET_GLOBAL_PROPERTY: HTML_HELP_COMMAND = HTML_HELP_COMMAND(252i32);
pub const HH_SET_INCLUSIVE_FILTER: HTML_HELP_COMMAND = HTML_HELP_COMMAND(24i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HH_SET_INFOTYPE {
    pub cbStruct: i32,
    pub pszCatName: windows_core::PCSTR,
    pub pszInfoTypeName: windows_core::PCSTR,
}
pub const HH_SET_INFO_TYPE: HTML_HELP_COMMAND = HTML_HELP_COMMAND(8i32);
pub const HH_SET_QUERYSERVICE: HTML_HELP_COMMAND = HTML_HELP_COMMAND(30i32);
pub const HH_SET_WIN_TYPE: HTML_HELP_COMMAND = HTML_HELP_COMMAND(4i32);
pub const HH_SYNC: HTML_HELP_COMMAND = HTML_HELP_COMMAND(9i32);
pub const HH_TAB_AUTHOR: i32 = 5i32;
pub const HH_TAB_CONTENTS: i32 = 0i32;
pub const HH_TAB_CUSTOM_FIRST: i32 = 11i32;
pub const HH_TAB_CUSTOM_LAST: i32 = 19i32;
pub const HH_TAB_FAVORITES: i32 = 3i32;
pub const HH_TAB_HISTORY: i32 = 4i32;
pub const HH_TAB_INDEX: i32 = 1i32;
pub const HH_TAB_SEARCH: i32 = 2i32;
pub const HH_TP_HELP_CONTEXTMENU: HTML_HELP_COMMAND = HTML_HELP_COMMAND(16i32);
pub const HH_TP_HELP_WM_HELP: HTML_HELP_COMMAND = HTML_HELP_COMMAND(17i32);
pub const HH_UNINITIALIZE: HTML_HELP_COMMAND = HTML_HELP_COMMAND(29i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HH_WINTYPE {
    pub cbStruct: i32,
    pub fUniCodeStrings: windows_core::BOOL,
    pub pszType: *mut i8,
    pub fsValidMembers: u32,
    pub fsWinProperties: u32,
    pub pszCaption: *mut i8,
    pub dwStyles: u32,
    pub dwExStyles: u32,
    pub rcWindowPos: super::super::Foundation::RECT,
    pub nShowState: i32,
    pub hwndHelp: super::super::Foundation::HWND,
    pub hwndCaller: super::super::Foundation::HWND,
    pub paInfoTypes: *mut u32,
    pub hwndToolBar: super::super::Foundation::HWND,
    pub hwndNavigation: super::super::Foundation::HWND,
    pub hwndHTML: super::super::Foundation::HWND,
    pub iNavWidth: i32,
    pub rcHTML: super::super::Foundation::RECT,
    pub pszToc: *mut i8,
    pub pszIndex: *mut i8,
    pub pszFile: *mut i8,
    pub pszHome: *mut i8,
    pub fsToolBarFlags: u32,
    pub fNotExpanded: windows_core::BOOL,
    pub curNavType: i32,
    pub tabpos: i32,
    pub idNotify: i32,
    pub tabOrder: [u8; 20],
    pub cHistory: i32,
    pub pszJump1: *mut i8,
    pub pszJump2: *mut i8,
    pub pszUrlJump1: *mut i8,
    pub pszUrlJump2: *mut i8,
    pub rcMinSize: super::super::Foundation::RECT,
    pub cbInfoTypes: i32,
    pub pszCustomTabs: *mut i8,
}
impl Default for HH_WINTYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTML_HELP_COMMAND(pub i32);
pub const IDTB_BACK: u32 = 204u32;
pub const IDTB_BROWSE_BACK: u32 = 212u32;
pub const IDTB_BROWSE_FWD: u32 = 211u32;
pub const IDTB_CONTENTS: u32 = 213u32;
pub const IDTB_CONTRACT: u32 = 201u32;
pub const IDTB_CUSTOMIZE: u32 = 221u32;
pub const IDTB_EXPAND: u32 = 200u32;
pub const IDTB_FAVORITES: u32 = 217u32;
pub const IDTB_FORWARD: u32 = 209u32;
pub const IDTB_HISTORY: u32 = 216u32;
pub const IDTB_HOME: u32 = 205u32;
pub const IDTB_INDEX: u32 = 214u32;
pub const IDTB_JUMP1: u32 = 218u32;
pub const IDTB_JUMP2: u32 = 219u32;
pub const IDTB_NOTES: u32 = 210u32;
pub const IDTB_OPTIONS: u32 = 208u32;
pub const IDTB_PRINT: u32 = 207u32;
pub const IDTB_REFRESH: u32 = 203u32;
pub const IDTB_SEARCH: u32 = 215u32;
pub const IDTB_STOP: u32 = 202u32;
pub const IDTB_SYNC: u32 = 206u32;
pub const IDTB_TOC_NEXT: u32 = 223u32;
pub const IDTB_TOC_PREV: u32 = 224u32;
pub const IDTB_ZOOM: u32 = 222u32;
windows_core::imp::define_interface!(IITDatabase, IITDatabase_Vtbl, 0x8fa0d5a2_dedf_11d0_9a61_00c04fb68bf7);
windows_core::imp::interface_hierarchy!(IITDatabase, windows_core::IUnknown);
impl IITDatabase {
    pub unsafe fn Open<P0, P1>(&self, lpszhost: P0, lpszmoniker: P1, dwflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), lpszhost.param().abi(), lpszmoniker.param().abi(), dwflags).ok() }
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn CreateObject(&self, rclsid: *const windows_core::GUID, pdwobjinstance: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateObject)(windows_core::Interface::as_raw(self), rclsid, pdwobjinstance as _).ok() }
    }
    pub unsafe fn GetObject(&self, dwobjinstance: u32, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetObject)(windows_core::Interface::as_raw(self), dwobjinstance, riid, ppvobj as _).ok() }
    }
    pub unsafe fn GetObjectPersistence<P0>(&self, lpwszobject: P0, dwobjinstance: u32, ppvpersistence: *mut *mut core::ffi::c_void, fstream: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetObjectPersistence)(windows_core::Interface::as_raw(self), lpwszobject.param().abi(), dwobjinstance, ppvpersistence as _, fstream.into()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IITDatabase_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateObject: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut u32) -> windows_core::HRESULT,
    pub GetObject: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetObjectPersistence: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IITDatabase_Impl: windows_core::IUnknownImpl {
    fn Open(&self, lpszhost: &windows_core::PCWSTR, lpszmoniker: &windows_core::PCWSTR, dwflags: u32) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
    fn CreateObject(&self, rclsid: *const windows_core::GUID, pdwobjinstance: *mut u32) -> windows_core::Result<()>;
    fn GetObject(&self, dwobjinstance: u32, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetObjectPersistence(&self, lpwszobject: &windows_core::PCWSTR, dwobjinstance: u32, ppvpersistence: *mut *mut core::ffi::c_void, fstream: windows_core::BOOL) -> windows_core::Result<()>;
}
impl IITDatabase_Vtbl {
    pub const fn new<Identity: IITDatabase_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Open<Identity: IITDatabase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpszhost: windows_core::PCWSTR, lpszmoniker: windows_core::PCWSTR, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IITDatabase_Impl::Open(this, core::mem::transmute(&lpszhost), core::mem::transmute(&lpszmoniker), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn Close<Identity: IITDatabase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IITDatabase_Impl::Close(this).into()
            }
        }
        unsafe extern "system" fn CreateObject<Identity: IITDatabase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, pdwobjinstance: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IITDatabase_Impl::CreateObject(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&pdwobjinstance)).into()
            }
        }
        unsafe extern "system" fn GetObject<Identity: IITDatabase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwobjinstance: u32, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IITDatabase_Impl::GetObject(this, core::mem::transmute_copy(&dwobjinstance), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobj)).into()
            }
        }
        unsafe extern "system" fn GetObjectPersistence<Identity: IITDatabase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpwszobject: windows_core::PCWSTR, dwobjinstance: u32, ppvpersistence: *mut *mut core::ffi::c_void, fstream: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IITDatabase_Impl::GetObjectPersistence(this, core::mem::transmute(&lpwszobject), core::mem::transmute_copy(&dwobjinstance), core::mem::transmute_copy(&ppvpersistence), core::mem::transmute_copy(&fstream)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            CreateObject: CreateObject::<Identity, OFFSET>,
            GetObject: GetObject::<Identity, OFFSET>,
            GetObjectPersistence: GetObjectPersistence::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IITDatabase as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IITDatabase {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IITPropList, IITPropList_Vtbl, 0x1f403bb1_9997_11d0_a850_00aa006c7d01);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IITPropList {
    type Target = super::super::System::Com::IPersistStreamInit;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IITPropList, windows_core::IUnknown, super::super::System::Com::IPersist, super::super::System::Com::IPersistStreamInit);
#[cfg(feature = "Win32_System_Com")]
impl IITPropList {
    pub unsafe fn Set<P1>(&self, propid: u32, lpszwstring: P1, dwoperation: u32) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Set)(windows_core::Interface::as_raw(self), propid, lpszwstring.param().abi(), dwoperation).ok() }
    }
    pub unsafe fn Set2(&self, propid: u32, lpvdata: *mut core::ffi::c_void, cbdata: u32, dwoperation: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Set2)(windows_core::Interface::as_raw(self), propid, lpvdata as _, cbdata, dwoperation).ok() }
    }
    pub unsafe fn Set3(&self, propid: u32, dwdata: u32, dwoperation: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Set3)(windows_core::Interface::as_raw(self), propid, dwdata, dwoperation).ok() }
    }
    pub unsafe fn Add(&self, prop: *mut CProperty) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), prop as _).ok() }
    }
    pub unsafe fn Get(&self, propid: u32, property: *mut CProperty) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Get)(windows_core::Interface::as_raw(self), propid, property as _).ok() }
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SetPersist(&self, fpersist: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPersist)(windows_core::Interface::as_raw(self), fpersist.into()).ok() }
    }
    pub unsafe fn SetPersist2(&self, propid: u32, fpersist: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPersist2)(windows_core::Interface::as_raw(self), propid, fpersist.into()).ok() }
    }
    pub unsafe fn GetFirst(&self, property: *mut CProperty) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetFirst)(windows_core::Interface::as_raw(self), property as _).ok() }
    }
    pub unsafe fn GetNext(&self, property: *mut CProperty) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetNext)(windows_core::Interface::as_raw(self), property as _).ok() }
    }
    pub unsafe fn GetPropCount(&self, cprop: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPropCount)(windows_core::Interface::as_raw(self), cprop as _).ok() }
    }
    pub unsafe fn SaveHeader(&self, lpvdata: *mut core::ffi::c_void, dwhdrsize: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SaveHeader)(windows_core::Interface::as_raw(self), lpvdata as _, dwhdrsize).ok() }
    }
    pub unsafe fn SaveData(&self, lpvheader: *mut core::ffi::c_void, dwhdrsize: u32, lpvdata: *mut core::ffi::c_void, dwbufsize: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SaveData)(windows_core::Interface::as_raw(self), lpvheader as _, dwhdrsize, lpvdata as _, dwbufsize).ok() }
    }
    pub unsafe fn GetHeaderSize(&self, dwhdrsize: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetHeaderSize)(windows_core::Interface::as_raw(self), dwhdrsize as _).ok() }
    }
    pub unsafe fn GetDataSize(&self, lpvheader: *mut core::ffi::c_void, dwhdrsize: u32, dwdatasize: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDataSize)(windows_core::Interface::as_raw(self), lpvheader as _, dwhdrsize, dwdatasize as _).ok() }
    }
    pub unsafe fn SaveDataToStream<P2>(&self, lpvheader: *mut core::ffi::c_void, dwhdrsize: u32, pstream: P2) -> windows_core::Result<()>
    where
        P2: windows_core::Param<super::super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).SaveDataToStream)(windows_core::Interface::as_raw(self), lpvheader as _, dwhdrsize, pstream.param().abi()).ok() }
    }
    pub unsafe fn LoadFromMem(&self, lpvdata: *mut core::ffi::c_void, dwbufsize: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).LoadFromMem)(windows_core::Interface::as_raw(self), lpvdata as _, dwbufsize).ok() }
    }
    pub unsafe fn SaveToMem(&self, lpvdata: *mut core::ffi::c_void, dwbufsize: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SaveToMem)(windows_core::Interface::as_raw(self), lpvdata as _, dwbufsize).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IITPropList_Vtbl {
    pub base__: super::super::System::Com::IPersistStreamInit_Vtbl,
    pub Set: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub Set2: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub Set3: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CProperty) -> windows_core::HRESULT,
    pub Get: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut CProperty) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPersist: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetPersist2: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetFirst: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CProperty) -> windows_core::HRESULT,
    pub GetNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CProperty) -> windows_core::HRESULT,
    pub GetPropCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SaveHeader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SaveData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetHeaderSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetDataSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub SaveDataToStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LoadFromMem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SaveToMem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IITPropList_Impl: super::super::System::Com::IPersistStreamInit_Impl {
    fn Set(&self, propid: u32, lpszwstring: &windows_core::PCWSTR, dwoperation: u32) -> windows_core::Result<()>;
    fn Set2(&self, propid: u32, lpvdata: *mut core::ffi::c_void, cbdata: u32, dwoperation: u32) -> windows_core::Result<()>;
    fn Set3(&self, propid: u32, dwdata: u32, dwoperation: u32) -> windows_core::Result<()>;
    fn Add(&self, prop: *mut CProperty) -> windows_core::Result<()>;
    fn Get(&self, propid: u32, property: *mut CProperty) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn SetPersist(&self, fpersist: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetPersist2(&self, propid: u32, fpersist: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetFirst(&self, property: *mut CProperty) -> windows_core::Result<()>;
    fn GetNext(&self, property: *mut CProperty) -> windows_core::Result<()>;
    fn GetPropCount(&self, cprop: *mut i32) -> windows_core::Result<()>;
    fn SaveHeader(&self, lpvdata: *mut core::ffi::c_void, dwhdrsize: u32) -> windows_core::Result<()>;
    fn SaveData(&self, lpvheader: *mut core::ffi::c_void, dwhdrsize: u32, lpvdata: *mut core::ffi::c_void, dwbufsize: u32) -> windows_core::Result<()>;
    fn GetHeaderSize(&self, dwhdrsize: *mut u32) -> windows_core::Result<()>;
    fn GetDataSize(&self, lpvheader: *mut core::ffi::c_void, dwhdrsize: u32, dwdatasize: *mut u32) -> windows_core::Result<()>;
    fn SaveDataToStream(&self, lpvheader: *mut core::ffi::c_void, dwhdrsize: u32, pstream: windows_core::Ref<super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn LoadFromMem(&self, lpvdata: *mut core::ffi::c_void, dwbufsize: u32) -> windows_core::Result<()>;
    fn SaveToMem(&self, lpvdata: *mut core::ffi::c_void, dwbufsize: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IITPropList_Vtbl {
    pub const fn new<Identity: IITPropList_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Set<Identity: IITPropList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propid: u32, lpszwstring: windows_core::PCWSTR, dwoperation: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IITPropList_Impl::Set(this, core::mem::transmute_copy(&propid), core::mem::transmute(&lpszwstring), core::mem::transmute_copy(&dwoperation)).into()
            }
        }
        unsafe extern "system" fn Set2<Identity: IITPropList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propid: u32, lpvdata: *mut core::ffi::c_void, cbdata: u32, dwoperation: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IITPropList_Impl::Set2(this, core::mem::transmute_copy(&propid), core::mem::transmute_copy(&lpvdata), core::mem::transmute_copy(&cbdata), core::mem::transmute_copy(&dwoperation)).into()
            }
        }
        unsafe extern "system" fn Set3<Identity: IITPropList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propid: u32, dwdata: u32, dwoperation: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IITPropList_Impl::Set3(this, core::mem::transmute_copy(&propid), core::mem::transmute_copy(&dwdata), core::mem::transmute_copy(&dwoperation)).into()
            }
        }
        unsafe extern "system" fn Add<Identity: IITPropList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prop: *mut CProperty) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IITPropList_Impl::Add(this, core::mem::transmute_copy(&prop)).into()
            }
        }
        unsafe extern "system" fn Get<Identity: IITPropList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propid: u32, property: *mut CProperty) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IITPropList_Impl::Get(this, core::mem::transmute_copy(&propid), core::mem::transmute_copy(&property)).into()
            }
        }
        unsafe extern "system" fn Clear<Identity: IITPropList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IITPropList_Impl::Clear(this).into()
            }
        }
        unsafe extern "system" fn SetPersist<Identity: IITPropList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fpersist: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IITPropList_Impl::SetPersist(this, core::mem::transmute_copy(&fpersist)).into()
            }
        }
        unsafe extern "system" fn SetPersist2<Identity: IITPropList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propid: u32, fpersist: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IITPropList_Impl::SetPersist2(this, core::mem::transmute_copy(&propid), core::mem::transmute_copy(&fpersist)).into()
            }
        }
        unsafe extern "system" fn GetFirst<Identity: IITPropList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, property: *mut CProperty) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IITPropList_Impl::GetFirst(this, core::mem::transmute_copy(&property)).into()
            }
        }
        unsafe extern "system" fn GetNext<Identity: IITPropList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, property: *mut CProperty) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IITPropList_Impl::GetNext(this, core::mem::transmute_copy(&property)).into()
            }
        }
        unsafe extern "system" fn GetPropCount<Identity: IITPropList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cprop: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IITPropList_Impl::GetPropCount(this, core::mem::transmute_copy(&cprop)).into()
            }
        }
        unsafe extern "system" fn SaveHeader<Identity: IITPropList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpvdata: *mut core::ffi::c_void, dwhdrsize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IITPropList_Impl::SaveHeader(this, core::mem::transmute_copy(&lpvdata), core::mem::transmute_copy(&dwhdrsize)).into()
            }
        }
        unsafe extern "system" fn SaveData<Identity: IITPropList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpvheader: *mut core::ffi::c_void, dwhdrsize: u32, lpvdata: *mut core::ffi::c_void, dwbufsize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IITPropList_Impl::SaveData(this, core::mem::transmute_copy(&lpvheader), core::mem::transmute_copy(&dwhdrsize), core::mem::transmute_copy(&lpvdata), core::mem::transmute_copy(&dwbufsize)).into()
            }
        }
        unsafe extern "system" fn GetHeaderSize<Identity: IITPropList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwhdrsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IITPropList_Impl::GetHeaderSize(this, core::mem::transmute_copy(&dwhdrsize)).into()
            }
        }
        unsafe extern "system" fn GetDataSize<Identity: IITPropList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpvheader: *mut core::ffi::c_void, dwhdrsize: u32, dwdatasize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IITPropList_Impl::GetDataSize(this, core::mem::transmute_copy(&lpvheader), core::mem::transmute_copy(&dwhdrsize), core::mem::transmute_copy(&dwdatasize)).into()
            }
        }
        unsafe extern "system" fn SaveDataToStream<Identity: IITPropList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpvheader: *mut core::ffi::c_void, dwhdrsize: u32, pstream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IITPropList_Impl::SaveDataToStream(this, core::mem::transmute_copy(&lpvheader), core::mem::transmute_copy(&dwhdrsize), core::mem::transmute_copy(&pstream)).into()
            }
        }
        unsafe extern "system" fn LoadFromMem<Identity: IITPropList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpvdata: *mut core::ffi::c_void, dwbufsize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IITPropList_Impl::LoadFromMem(this, core::mem::transmute_copy(&lpvdata), core::mem::transmute_copy(&dwbufsize)).into()
            }
        }
        unsafe extern "system" fn SaveToMem<Identity: IITPropList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpvdata: *mut core::ffi::c_void, dwbufsize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IITPropList_Impl::SaveToMem(this, core::mem::transmute_copy(&lpvdata), core::mem::transmute_copy(&dwbufsize)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IPersistStreamInit_Vtbl::new::<Identity, OFFSET>(),
            Set: Set::<Identity, OFFSET>,
            Set2: Set2::<Identity, OFFSET>,
            Set3: Set3::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Get: Get::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            SetPersist: SetPersist::<Identity, OFFSET>,
            SetPersist2: SetPersist2::<Identity, OFFSET>,
            GetFirst: GetFirst::<Identity, OFFSET>,
            GetNext: GetNext::<Identity, OFFSET>,
            GetPropCount: GetPropCount::<Identity, OFFSET>,
            SaveHeader: SaveHeader::<Identity, OFFSET>,
            SaveData: SaveData::<Identity, OFFSET>,
            GetHeaderSize: GetHeaderSize::<Identity, OFFSET>,
            GetDataSize: GetDataSize::<Identity, OFFSET>,
            SaveDataToStream: SaveDataToStream::<Identity, OFFSET>,
            LoadFromMem: LoadFromMem::<Identity, OFFSET>,
            SaveToMem: SaveToMem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IITPropList as windows_core::Interface>::IID || iid == &<super::super::System::Com::IPersist as windows_core::Interface>::IID || iid == &<super::super::System::Com::IPersistStreamInit as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IITPropList {}
windows_core::imp::define_interface!(IITResultSet, IITResultSet_Vtbl, 0x3bb91d41_998b_11d0_a850_00aa006c7d01);
windows_core::imp::interface_hierarchy!(IITResultSet, windows_core::IUnknown);
impl IITResultSet {
    pub unsafe fn SetColumnPriority(&self, lcolumnindex: i32, columnpriority: PRIORITY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetColumnPriority)(windows_core::Interface::as_raw(self), lcolumnindex, columnpriority).ok() }
    }
    pub unsafe fn SetColumnHeap(&self, lcolumnindex: i32, lpvheap: *mut core::ffi::c_void, pfncolheapfree: PFNCOLHEAPFREE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetColumnHeap)(windows_core::Interface::as_raw(self), lcolumnindex, lpvheap as _, pfncolheapfree).ok() }
    }
    pub unsafe fn SetKeyProp(&self, propid: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetKeyProp)(windows_core::Interface::as_raw(self), propid).ok() }
    }
    pub unsafe fn Add(&self, propid: u32, dwdefaultdata: u32, priority: PRIORITY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), propid, dwdefaultdata, priority).ok() }
    }
    pub unsafe fn Add2<P1>(&self, propid: u32, lpszwdefault: P1, priority: PRIORITY) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Add2)(windows_core::Interface::as_raw(self), propid, lpszwdefault.param().abi(), priority).ok() }
    }
    pub unsafe fn Add3(&self, propid: u32, lpvdefaultdata: *mut core::ffi::c_void, cbdata: u32, priority: PRIORITY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Add3)(windows_core::Interface::as_raw(self), propid, lpvdefaultdata as _, cbdata, priority).ok() }
    }
    pub unsafe fn Add4(&self, lpvhdr: *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Add4)(windows_core::Interface::as_raw(self), lpvhdr as _).ok() }
    }
    pub unsafe fn Append(&self, lpvhdr: *mut core::ffi::c_void, lpvdata: *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Append)(windows_core::Interface::as_raw(self), lpvhdr as _, lpvdata as _).ok() }
    }
    pub unsafe fn Set(&self, lrowindex: i32, lcolumnindex: i32, lpvdata: *mut core::ffi::c_void, cbdata: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Set)(windows_core::Interface::as_raw(self), lrowindex, lcolumnindex, lpvdata as _, cbdata).ok() }
    }
    pub unsafe fn Set2<P2>(&self, lrowindex: i32, lcolumnindex: i32, lpwstr: P2) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Set2)(windows_core::Interface::as_raw(self), lrowindex, lcolumnindex, lpwstr.param().abi()).ok() }
    }
    pub unsafe fn Set3(&self, lrowindex: i32, lcolumnindex: i32, dwdata: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Set3)(windows_core::Interface::as_raw(self), lrowindex, lcolumnindex, dwdata).ok() }
    }
    pub unsafe fn Set4(&self, lrowindex: i32, lpvhdr: *mut core::ffi::c_void, lpvdata: *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Set4)(windows_core::Interface::as_raw(self), lrowindex, lpvhdr as _, lpvdata as _).ok() }
    }
    pub unsafe fn Copy<P0>(&self, prscopy: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IITResultSet>,
    {
        unsafe { (windows_core::Interface::vtable(self).Copy)(windows_core::Interface::as_raw(self), prscopy.param().abi()).ok() }
    }
    pub unsafe fn AppendRows<P0>(&self, pressrc: P0, lrowsrcfirst: i32, csrcrows: i32, lrowfirstdest: *mut i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IITResultSet>,
    {
        unsafe { (windows_core::Interface::vtable(self).AppendRows)(windows_core::Interface::as_raw(self), pressrc.param().abi(), lrowsrcfirst, csrcrows, lrowfirstdest as _).ok() }
    }
    pub unsafe fn Get(&self, lrowindex: i32, lcolumnindex: i32, prop: *mut CProperty) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Get)(windows_core::Interface::as_raw(self), lrowindex, lcolumnindex, prop as _).ok() }
    }
    pub unsafe fn GetKeyProp(&self, keypropid: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetKeyProp)(windows_core::Interface::as_raw(self), keypropid as _).ok() }
    }
    pub unsafe fn GetColumnPriority(&self, lcolumnindex: i32, columnpriority: *mut PRIORITY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetColumnPriority)(windows_core::Interface::as_raw(self), lcolumnindex, columnpriority as _).ok() }
    }
    pub unsafe fn GetRowCount(&self, lnumberofrows: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetRowCount)(windows_core::Interface::as_raw(self), lnumberofrows as _).ok() }
    }
    pub unsafe fn GetColumnCount(&self, lnumberofcolumns: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetColumnCount)(windows_core::Interface::as_raw(self), lnumberofcolumns as _).ok() }
    }
    pub unsafe fn GetColumn(&self, lcolumnindex: i32, propid: *mut u32, dwtype: *mut u32, lpvdefaultvalue: *mut *mut core::ffi::c_void, cbsize: *mut u32, columnpriority: *mut PRIORITY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetColumn)(windows_core::Interface::as_raw(self), lcolumnindex, propid as _, dwtype as _, lpvdefaultvalue as _, cbsize as _, columnpriority as _).ok() }
    }
    pub unsafe fn GetColumn2(&self, lcolumnindex: i32, propid: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetColumn2)(windows_core::Interface::as_raw(self), lcolumnindex, propid as _).ok() }
    }
    pub unsafe fn GetColumnFromPropID(&self, propid: u32, lcolumnindex: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetColumnFromPropID)(windows_core::Interface::as_raw(self), propid, lcolumnindex as _).ok() }
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn ClearRows(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ClearRows)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Free(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Free)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn IsCompleted(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).IsCompleted)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Cancel(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Pause(&self, fpause: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Pause)(windows_core::Interface::as_raw(self), fpause.into()).ok() }
    }
    pub unsafe fn GetRowStatus(&self, lrowfirst: i32, crows: i32, lprowstatus: *mut ROWSTATUS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetRowStatus)(windows_core::Interface::as_raw(self), lrowfirst, crows, lprowstatus as _).ok() }
    }
    pub unsafe fn GetColumnStatus(&self, lpcolstatus: *mut COLUMNSTATUS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetColumnStatus)(windows_core::Interface::as_raw(self), lpcolstatus as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IITResultSet_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetColumnPriority: unsafe extern "system" fn(*mut core::ffi::c_void, i32, PRIORITY) -> windows_core::HRESULT,
    pub SetColumnHeap: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, PFNCOLHEAPFREE) -> windows_core::HRESULT,
    pub SetKeyProp: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, PRIORITY) -> windows_core::HRESULT,
    pub Add2: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, PRIORITY) -> windows_core::HRESULT,
    pub Add3: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, u32, PRIORITY) -> windows_core::HRESULT,
    pub Add4: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Set: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Set2: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Set3: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, usize) -> windows_core::HRESULT,
    pub Set4: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Copy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AppendRows: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32, *mut i32) -> windows_core::HRESULT,
    pub Get: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut CProperty) -> windows_core::HRESULT,
    pub GetKeyProp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetColumnPriority: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut PRIORITY) -> windows_core::HRESULT,
    pub GetRowCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetColumnCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetColumn: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut u32, *mut u32, *mut *mut core::ffi::c_void, *mut u32, *mut PRIORITY) -> windows_core::HRESULT,
    pub GetColumn2: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut u32) -> windows_core::HRESULT,
    pub GetColumnFromPropID: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut i32) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClearRows: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Free: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsCompleted: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetRowStatus: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut ROWSTATUS) -> windows_core::HRESULT,
    pub GetColumnStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut COLUMNSTATUS) -> windows_core::HRESULT,
}
pub trait IITResultSet_Impl: windows_core::IUnknownImpl {
    fn SetColumnPriority(&self, lcolumnindex: i32, columnpriority: PRIORITY) -> windows_core::Result<()>;
    fn SetColumnHeap(&self, lcolumnindex: i32, lpvheap: *mut core::ffi::c_void, pfncolheapfree: PFNCOLHEAPFREE) -> windows_core::Result<()>;
    fn SetKeyProp(&self, propid: u32) -> windows_core::Result<()>;
    fn Add(&self, propid: u32, dwdefaultdata: u32, priority: PRIORITY) -> windows_core::Result<()>;
    fn Add2(&self, propid: u32, lpszwdefault: &windows_core::PCWSTR, priority: PRIORITY) -> windows_core::Result<()>;
    fn Add3(&self, propid: u32, lpvdefaultdata: *mut core::ffi::c_void, cbdata: u32, priority: PRIORITY) -> windows_core::Result<()>;
    fn Add4(&self, lpvhdr: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Append(&self, lpvhdr: *mut core::ffi::c_void, lpvdata: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Set(&self, lrowindex: i32, lcolumnindex: i32, lpvdata: *mut core::ffi::c_void, cbdata: u32) -> windows_core::Result<()>;
    fn Set2(&self, lrowindex: i32, lcolumnindex: i32, lpwstr: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Set3(&self, lrowindex: i32, lcolumnindex: i32, dwdata: usize) -> windows_core::Result<()>;
    fn Set4(&self, lrowindex: i32, lpvhdr: *mut core::ffi::c_void, lpvdata: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Copy(&self, prscopy: windows_core::Ref<IITResultSet>) -> windows_core::Result<()>;
    fn AppendRows(&self, pressrc: windows_core::Ref<IITResultSet>, lrowsrcfirst: i32, csrcrows: i32, lrowfirstdest: *mut i32) -> windows_core::Result<()>;
    fn Get(&self, lrowindex: i32, lcolumnindex: i32, prop: *mut CProperty) -> windows_core::Result<()>;
    fn GetKeyProp(&self, keypropid: *mut u32) -> windows_core::Result<()>;
    fn GetColumnPriority(&self, lcolumnindex: i32, columnpriority: *mut PRIORITY) -> windows_core::Result<()>;
    fn GetRowCount(&self, lnumberofrows: *mut i32) -> windows_core::Result<()>;
    fn GetColumnCount(&self, lnumberofcolumns: *mut i32) -> windows_core::Result<()>;
    fn GetColumn(&self, lcolumnindex: i32, propid: *mut u32, dwtype: *mut u32, lpvdefaultvalue: *mut *mut core::ffi::c_void, cbsize: *mut u32, columnpriority: *mut PRIORITY) -> windows_core::Result<()>;
    fn GetColumn2(&self, lcolumnindex: i32, propid: *mut u32) -> windows_core::Result<()>;
    fn GetColumnFromPropID(&self, propid: u32, lcolumnindex: *mut i32) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn ClearRows(&self) -> windows_core::Result<()>;
    fn Free(&self) -> windows_core::Result<()>;
    fn IsCompleted(&self) -> windows_core::Result<()>;
    fn Cancel(&self) -> windows_core::Result<()>;
    fn Pause(&self, fpause: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetRowStatus(&self, lrowfirst: i32, crows: i32, lprowstatus: *mut ROWSTATUS) -> windows_core::Result<()>;
    fn GetColumnStatus(&self, lpcolstatus: *mut COLUMNSTATUS) -> windows_core::Result<()>;
}
impl IITResultSet_Vtbl {
    pub const fn new<Identity: IITResultSet_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetColumnPriority<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcolumnindex: i32, columnpriority: PRIORITY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IITResultSet_Impl::SetColumnPriority(this, core::mem::transmute_copy(&lcolumnindex), core::mem::transmute_copy(&columnpriority)).into()
            }
        }
        unsafe extern "system" fn SetColumnHeap<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcolumnindex: i32, lpvheap: *mut core::ffi::c_void, pfncolheapfree: PFNCOLHEAPFREE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IITResultSet_Impl::SetColumnHeap(this, core::mem::transmute_copy(&lcolumnindex), core::mem::transmute_copy(&lpvheap), core::mem::transmute_copy(&pfncolheapfree)).into()
            }
        }
        unsafe extern "system" fn SetKeyProp<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IITResultSet_Impl::SetKeyProp(this, core::mem::transmute_copy(&propid)).into()
            }
        }
        unsafe extern "system" fn Add<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propid: u32, dwdefaultdata: u32, priority: PRIORITY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IITResultSet_Impl::Add(this, core::mem::transmute_copy(&propid), core::mem::transmute_copy(&dwdefaultdata), core::mem::transmute_copy(&priority)).into()
            }
        }
        unsafe extern "system" fn Add2<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propid: u32, lpszwdefault: windows_core::PCWSTR, priority: PRIORITY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IITResultSet_Impl::Add2(this, core::mem::transmute_copy(&propid), core::mem::transmute(&lpszwdefault), core::mem::transmute_copy(&priority)).into()
            }
        }
        unsafe extern "system" fn Add3<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propid: u32, lpvdefaultdata: *mut core::ffi::c_void, cbdata: u32, priority: PRIORITY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IITResultSet_Impl::Add3(this, core::mem::transmute_copy(&propid), core::mem::transmute_copy(&lpvdefaultdata), core::mem::transmute_copy(&cbdata), core::mem::transmute_copy(&priority)).into()
            }
        }
        unsafe extern "system" fn Add4<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpvhdr: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IITResultSet_Impl::Add4(this, core::mem::transmute_copy(&lpvhdr)).into()
            }
        }
        unsafe extern "system" fn Append<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpvhdr: *mut core::ffi::c_void, lpvdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IITResultSet_Impl::Append(this, core::mem::transmute_copy(&lpvhdr), core::mem::transmute_copy(&lpvdata)).into()
            }
        }
        unsafe extern "system" fn Set<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lrowindex: i32, lcolumnindex: i32, lpvdata: *mut core::ffi::c_void, cbdata: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IITResultSet_Impl::Set(this, core::mem::transmute_copy(&lrowindex), core::mem::transmute_copy(&lcolumnindex), core::mem::transmute_copy(&lpvdata), core::mem::transmute_copy(&cbdata)).into()
            }
        }
        unsafe extern "system" fn Set2<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lrowindex: i32, lcolumnindex: i32, lpwstr: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IITResultSet_Impl::Set2(this, core::mem::transmute_copy(&lrowindex), core::mem::transmute_copy(&lcolumnindex), core::mem::transmute(&lpwstr)).into()
            }
        }
        unsafe extern "system" fn Set3<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lrowindex: i32, lcolumnindex: i32, dwdata: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IITResultSet_Impl::Set3(this, core::mem::transmute_copy(&lrowindex), core::mem::transmute_copy(&lcolumnindex), core::mem::transmute_copy(&dwdata)).into()
            }
        }
        unsafe extern "system" fn Set4<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lrowindex: i32, lpvhdr: *mut core::ffi::c_void, lpvdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IITResultSet_Impl::Set4(this, core::mem::transmute_copy(&lrowindex), core::mem::transmute_copy(&lpvhdr), core::mem::transmute_copy(&lpvdata)).into()
            }
        }
        unsafe extern "system" fn Copy<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prscopy: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IITResultSet_Impl::Copy(this, core::mem::transmute_copy(&prscopy)).into()
            }
        }
        unsafe extern "system" fn AppendRows<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pressrc: *mut core::ffi::c_void, lrowsrcfirst: i32, csrcrows: i32, lrowfirstdest: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IITResultSet_Impl::AppendRows(this, core::mem::transmute_copy(&pressrc), core::mem::transmute_copy(&lrowsrcfirst), core::mem::transmute_copy(&csrcrows), core::mem::transmute_copy(&lrowfirstdest)).into()
            }
        }
        unsafe extern "system" fn Get<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lrowindex: i32, lcolumnindex: i32, prop: *mut CProperty) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IITResultSet_Impl::Get(this, core::mem::transmute_copy(&lrowindex), core::mem::transmute_copy(&lcolumnindex), core::mem::transmute_copy(&prop)).into()
            }
        }
        unsafe extern "system" fn GetKeyProp<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, keypropid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IITResultSet_Impl::GetKeyProp(this, core::mem::transmute_copy(&keypropid)).into()
            }
        }
        unsafe extern "system" fn GetColumnPriority<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcolumnindex: i32, columnpriority: *mut PRIORITY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IITResultSet_Impl::GetColumnPriority(this, core::mem::transmute_copy(&lcolumnindex), core::mem::transmute_copy(&columnpriority)).into()
            }
        }
        unsafe extern "system" fn GetRowCount<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnumberofrows: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IITResultSet_Impl::GetRowCount(this, core::mem::transmute_copy(&lnumberofrows)).into()
            }
        }
        unsafe extern "system" fn GetColumnCount<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnumberofcolumns: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IITResultSet_Impl::GetColumnCount(this, core::mem::transmute_copy(&lnumberofcolumns)).into()
            }
        }
        unsafe extern "system" fn GetColumn<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcolumnindex: i32, propid: *mut u32, dwtype: *mut u32, lpvdefaultvalue: *mut *mut core::ffi::c_void, cbsize: *mut u32, columnpriority: *mut PRIORITY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IITResultSet_Impl::GetColumn(this, core::mem::transmute_copy(&lcolumnindex), core::mem::transmute_copy(&propid), core::mem::transmute_copy(&dwtype), core::mem::transmute_copy(&lpvdefaultvalue), core::mem::transmute_copy(&cbsize), core::mem::transmute_copy(&columnpriority)).into()
            }
        }
        unsafe extern "system" fn GetColumn2<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcolumnindex: i32, propid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IITResultSet_Impl::GetColumn2(this, core::mem::transmute_copy(&lcolumnindex), core::mem::transmute_copy(&propid)).into()
            }
        }
        unsafe extern "system" fn GetColumnFromPropID<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propid: u32, lcolumnindex: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IITResultSet_Impl::GetColumnFromPropID(this, core::mem::transmute_copy(&propid), core::mem::transmute_copy(&lcolumnindex)).into()
            }
        }
        unsafe extern "system" fn Clear<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IITResultSet_Impl::Clear(this).into()
            }
        }
        unsafe extern "system" fn ClearRows<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IITResultSet_Impl::ClearRows(this).into()
            }
        }
        unsafe extern "system" fn Free<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IITResultSet_Impl::Free(this).into()
            }
        }
        unsafe extern "system" fn IsCompleted<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IITResultSet_Impl::IsCompleted(this).into()
            }
        }
        unsafe extern "system" fn Cancel<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IITResultSet_Impl::Cancel(this).into()
            }
        }
        unsafe extern "system" fn Pause<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fpause: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IITResultSet_Impl::Pause(this, core::mem::transmute_copy(&fpause)).into()
            }
        }
        unsafe extern "system" fn GetRowStatus<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lrowfirst: i32, crows: i32, lprowstatus: *mut ROWSTATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IITResultSet_Impl::GetRowStatus(this, core::mem::transmute_copy(&lrowfirst), core::mem::transmute_copy(&crows), core::mem::transmute_copy(&lprowstatus)).into()
            }
        }
        unsafe extern "system" fn GetColumnStatus<Identity: IITResultSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpcolstatus: *mut COLUMNSTATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IITResultSet_Impl::GetColumnStatus(this, core::mem::transmute_copy(&lpcolstatus)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetColumnPriority: SetColumnPriority::<Identity, OFFSET>,
            SetColumnHeap: SetColumnHeap::<Identity, OFFSET>,
            SetKeyProp: SetKeyProp::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Add2: Add2::<Identity, OFFSET>,
            Add3: Add3::<Identity, OFFSET>,
            Add4: Add4::<Identity, OFFSET>,
            Append: Append::<Identity, OFFSET>,
            Set: Set::<Identity, OFFSET>,
            Set2: Set2::<Identity, OFFSET>,
            Set3: Set3::<Identity, OFFSET>,
            Set4: Set4::<Identity, OFFSET>,
            Copy: Copy::<Identity, OFFSET>,
            AppendRows: AppendRows::<Identity, OFFSET>,
            Get: Get::<Identity, OFFSET>,
            GetKeyProp: GetKeyProp::<Identity, OFFSET>,
            GetColumnPriority: GetColumnPriority::<Identity, OFFSET>,
            GetRowCount: GetRowCount::<Identity, OFFSET>,
            GetColumnCount: GetColumnCount::<Identity, OFFSET>,
            GetColumn: GetColumn::<Identity, OFFSET>,
            GetColumn2: GetColumn2::<Identity, OFFSET>,
            GetColumnFromPropID: GetColumnFromPropID::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            ClearRows: ClearRows::<Identity, OFFSET>,
            Free: Free::<Identity, OFFSET>,
            IsCompleted: IsCompleted::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
            Pause: Pause::<Identity, OFFSET>,
            GetRowStatus: GetRowStatus::<Identity, OFFSET>,
            GetColumnStatus: GetColumnStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IITResultSet as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IITResultSet {}
pub const IITWBC_BREAK_ACCEPT_WILDCARDS: u32 = 1u32;
pub const IITWBC_BREAK_AND_STEM: u32 = 2u32;
windows_core::imp::define_interface!(IStemSink, IStemSink_Vtbl, 0xfe77c330_7f42_11ce_be57_00aa0051fe20);
windows_core::imp::interface_hierarchy!(IStemSink, windows_core::IUnknown);
impl IStemSink {
    pub unsafe fn PutAltWord<P0>(&self, pwcinbuf: P0, cwc: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).PutAltWord)(windows_core::Interface::as_raw(self), pwcinbuf.param().abi(), cwc).ok() }
    }
    pub unsafe fn PutWord<P0>(&self, pwcinbuf: P0, cwc: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).PutWord)(windows_core::Interface::as_raw(self), pwcinbuf.param().abi(), cwc).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStemSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub PutAltWord: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub PutWord: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
}
pub trait IStemSink_Impl: windows_core::IUnknownImpl {
    fn PutAltWord(&self, pwcinbuf: &windows_core::PCWSTR, cwc: u32) -> windows_core::Result<()>;
    fn PutWord(&self, pwcinbuf: &windows_core::PCWSTR, cwc: u32) -> windows_core::Result<()>;
}
impl IStemSink_Vtbl {
    pub const fn new<Identity: IStemSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PutAltWord<Identity: IStemSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcinbuf: windows_core::PCWSTR, cwc: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStemSink_Impl::PutAltWord(this, core::mem::transmute(&pwcinbuf), core::mem::transmute_copy(&cwc)).into()
            }
        }
        unsafe extern "system" fn PutWord<Identity: IStemSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcinbuf: windows_core::PCWSTR, cwc: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStemSink_Impl::PutWord(this, core::mem::transmute(&pwcinbuf), core::mem::transmute_copy(&cwc)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), PutAltWord: PutAltWord::<Identity, OFFSET>, PutWord: PutWord::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStemSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IStemSink {}
windows_core::imp::define_interface!(IStemmerConfig, IStemmerConfig_Vtbl, 0x8fa0d5a7_dedf_11d0_9a61_00c04fb68bf7);
windows_core::imp::interface_hierarchy!(IStemmerConfig, windows_core::IUnknown);
impl IStemmerConfig {
    pub unsafe fn SetLocaleInfo(&self, dwcodepageid: u32, lcid: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLocaleInfo)(windows_core::Interface::as_raw(self), dwcodepageid, lcid).ok() }
    }
    pub unsafe fn GetLocaleInfo(&self, pdwcodepageid: *mut u32, plcid: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetLocaleInfo)(windows_core::Interface::as_raw(self), pdwcodepageid as _, plcid as _).ok() }
    }
    pub unsafe fn SetControlInfo(&self, grfstemflags: u32, dwreserved: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetControlInfo)(windows_core::Interface::as_raw(self), grfstemflags, dwreserved).ok() }
    }
    pub unsafe fn GetControlInfo(&self, pgrfstemflags: *mut u32, pdwreserved: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetControlInfo)(windows_core::Interface::as_raw(self), pgrfstemflags as _, pdwreserved as _).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LoadExternalStemmerData<P0>(&self, pstream: P0, dwextdatatype: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).LoadExternalStemmerData)(windows_core::Interface::as_raw(self), pstream.param().abi(), dwextdatatype).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStemmerConfig_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetLocaleInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetLocaleInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub SetControlInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetControlInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub LoadExternalStemmerData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    LoadExternalStemmerData: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IStemmerConfig_Impl: windows_core::IUnknownImpl {
    fn SetLocaleInfo(&self, dwcodepageid: u32, lcid: u32) -> windows_core::Result<()>;
    fn GetLocaleInfo(&self, pdwcodepageid: *mut u32, plcid: *mut u32) -> windows_core::Result<()>;
    fn SetControlInfo(&self, grfstemflags: u32, dwreserved: u32) -> windows_core::Result<()>;
    fn GetControlInfo(&self, pgrfstemflags: *mut u32, pdwreserved: *mut u32) -> windows_core::Result<()>;
    fn LoadExternalStemmerData(&self, pstream: windows_core::Ref<super::super::System::Com::IStream>, dwextdatatype: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IStemmerConfig_Vtbl {
    pub const fn new<Identity: IStemmerConfig_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetLocaleInfo<Identity: IStemmerConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcodepageid: u32, lcid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStemmerConfig_Impl::SetLocaleInfo(this, core::mem::transmute_copy(&dwcodepageid), core::mem::transmute_copy(&lcid)).into()
            }
        }
        unsafe extern "system" fn GetLocaleInfo<Identity: IStemmerConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcodepageid: *mut u32, plcid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStemmerConfig_Impl::GetLocaleInfo(this, core::mem::transmute_copy(&pdwcodepageid), core::mem::transmute_copy(&plcid)).into()
            }
        }
        unsafe extern "system" fn SetControlInfo<Identity: IStemmerConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfstemflags: u32, dwreserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStemmerConfig_Impl::SetControlInfo(this, core::mem::transmute_copy(&grfstemflags), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        unsafe extern "system" fn GetControlInfo<Identity: IStemmerConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgrfstemflags: *mut u32, pdwreserved: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStemmerConfig_Impl::GetControlInfo(this, core::mem::transmute_copy(&pgrfstemflags), core::mem::transmute_copy(&pdwreserved)).into()
            }
        }
        unsafe extern "system" fn LoadExternalStemmerData<Identity: IStemmerConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void, dwextdatatype: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStemmerConfig_Impl::LoadExternalStemmerData(this, core::mem::transmute_copy(&pstream), core::mem::transmute_copy(&dwextdatatype)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetLocaleInfo: SetLocaleInfo::<Identity, OFFSET>,
            GetLocaleInfo: GetLocaleInfo::<Identity, OFFSET>,
            SetControlInfo: SetControlInfo::<Identity, OFFSET>,
            GetControlInfo: GetControlInfo::<Identity, OFFSET>,
            LoadExternalStemmerData: LoadExternalStemmerData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStemmerConfig as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IStemmerConfig {}
pub const ITWW_CBKEY_MAX: u32 = 1024u32;
pub const ITWW_OPEN_NOCONNECT: u32 = 1u32;
pub const IT_EXCLUSIVE: i32 = 1i32;
pub const IT_HIDDEN: i32 = 2i32;
pub const IT_INCLUSIVE: i32 = 0i32;
windows_core::imp::define_interface!(IWordBreakerConfig, IWordBreakerConfig_Vtbl, 0x8fa0d5a6_dedf_11d0_9a61_00c04fb68bf7);
windows_core::imp::interface_hierarchy!(IWordBreakerConfig, windows_core::IUnknown);
impl IWordBreakerConfig {
    pub unsafe fn SetLocaleInfo(&self, dwcodepageid: u32, lcid: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLocaleInfo)(windows_core::Interface::as_raw(self), dwcodepageid, lcid).ok() }
    }
    pub unsafe fn GetLocaleInfo(&self, pdwcodepageid: *mut u32, plcid: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetLocaleInfo)(windows_core::Interface::as_raw(self), pdwcodepageid as _, plcid as _).ok() }
    }
    pub unsafe fn SetBreakWordType(&self, dwbreakwordtype: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBreakWordType)(windows_core::Interface::as_raw(self), dwbreakwordtype).ok() }
    }
    pub unsafe fn GetBreakWordType(&self, pdwbreakwordtype: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetBreakWordType)(windows_core::Interface::as_raw(self), pdwbreakwordtype as _).ok() }
    }
    pub unsafe fn SetControlInfo(&self, grfbreakflags: u32, dwreserved: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetControlInfo)(windows_core::Interface::as_raw(self), grfbreakflags, dwreserved).ok() }
    }
    pub unsafe fn GetControlInfo(&self, pgrfbreakflags: *mut u32, pdwreserved: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetControlInfo)(windows_core::Interface::as_raw(self), pgrfbreakflags as _, pdwreserved as _).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LoadExternalBreakerData<P0>(&self, pstream: P0, dwextdatatype: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).LoadExternalBreakerData)(windows_core::Interface::as_raw(self), pstream.param().abi(), dwextdatatype).ok() }
    }
    #[cfg(feature = "Win32_System_Search")]
    pub unsafe fn SetWordStemmer<P1>(&self, rclsid: *const windows_core::GUID, pstemmer: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::super::System::Search::IStemmer>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetWordStemmer)(windows_core::Interface::as_raw(self), rclsid, pstemmer.param().abi()).ok() }
    }
    #[cfg(feature = "Win32_System_Search")]
    pub unsafe fn GetWordStemmer(&self) -> windows_core::Result<super::super::System::Search::IStemmer> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWordStemmer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWordBreakerConfig_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetLocaleInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetLocaleInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub SetBreakWordType: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetBreakWordType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetControlInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetControlInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub LoadExternalBreakerData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    LoadExternalBreakerData: usize,
    #[cfg(feature = "Win32_System_Search")]
    pub SetWordStemmer: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Search"))]
    SetWordStemmer: usize,
    #[cfg(feature = "Win32_System_Search")]
    pub GetWordStemmer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Search"))]
    GetWordStemmer: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Search"))]
pub trait IWordBreakerConfig_Impl: windows_core::IUnknownImpl {
    fn SetLocaleInfo(&self, dwcodepageid: u32, lcid: u32) -> windows_core::Result<()>;
    fn GetLocaleInfo(&self, pdwcodepageid: *mut u32, plcid: *mut u32) -> windows_core::Result<()>;
    fn SetBreakWordType(&self, dwbreakwordtype: u32) -> windows_core::Result<()>;
    fn GetBreakWordType(&self, pdwbreakwordtype: *mut u32) -> windows_core::Result<()>;
    fn SetControlInfo(&self, grfbreakflags: u32, dwreserved: u32) -> windows_core::Result<()>;
    fn GetControlInfo(&self, pgrfbreakflags: *mut u32, pdwreserved: *mut u32) -> windows_core::Result<()>;
    fn LoadExternalBreakerData(&self, pstream: windows_core::Ref<super::super::System::Com::IStream>, dwextdatatype: u32) -> windows_core::Result<()>;
    fn SetWordStemmer(&self, rclsid: *const windows_core::GUID, pstemmer: windows_core::Ref<super::super::System::Search::IStemmer>) -> windows_core::Result<()>;
    fn GetWordStemmer(&self) -> windows_core::Result<super::super::System::Search::IStemmer>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Search"))]
impl IWordBreakerConfig_Vtbl {
    pub const fn new<Identity: IWordBreakerConfig_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetLocaleInfo<Identity: IWordBreakerConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcodepageid: u32, lcid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWordBreakerConfig_Impl::SetLocaleInfo(this, core::mem::transmute_copy(&dwcodepageid), core::mem::transmute_copy(&lcid)).into()
            }
        }
        unsafe extern "system" fn GetLocaleInfo<Identity: IWordBreakerConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcodepageid: *mut u32, plcid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWordBreakerConfig_Impl::GetLocaleInfo(this, core::mem::transmute_copy(&pdwcodepageid), core::mem::transmute_copy(&plcid)).into()
            }
        }
        unsafe extern "system" fn SetBreakWordType<Identity: IWordBreakerConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwbreakwordtype: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWordBreakerConfig_Impl::SetBreakWordType(this, core::mem::transmute_copy(&dwbreakwordtype)).into()
            }
        }
        unsafe extern "system" fn GetBreakWordType<Identity: IWordBreakerConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwbreakwordtype: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWordBreakerConfig_Impl::GetBreakWordType(this, core::mem::transmute_copy(&pdwbreakwordtype)).into()
            }
        }
        unsafe extern "system" fn SetControlInfo<Identity: IWordBreakerConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfbreakflags: u32, dwreserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWordBreakerConfig_Impl::SetControlInfo(this, core::mem::transmute_copy(&grfbreakflags), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        unsafe extern "system" fn GetControlInfo<Identity: IWordBreakerConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgrfbreakflags: *mut u32, pdwreserved: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWordBreakerConfig_Impl::GetControlInfo(this, core::mem::transmute_copy(&pgrfbreakflags), core::mem::transmute_copy(&pdwreserved)).into()
            }
        }
        unsafe extern "system" fn LoadExternalBreakerData<Identity: IWordBreakerConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void, dwextdatatype: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWordBreakerConfig_Impl::LoadExternalBreakerData(this, core::mem::transmute_copy(&pstream), core::mem::transmute_copy(&dwextdatatype)).into()
            }
        }
        unsafe extern "system" fn SetWordStemmer<Identity: IWordBreakerConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, pstemmer: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWordBreakerConfig_Impl::SetWordStemmer(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&pstemmer)).into()
            }
        }
        unsafe extern "system" fn GetWordStemmer<Identity: IWordBreakerConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstemmer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWordBreakerConfig_Impl::GetWordStemmer(this) {
                    Ok(ok__) => {
                        ppstemmer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetLocaleInfo: SetLocaleInfo::<Identity, OFFSET>,
            GetLocaleInfo: GetLocaleInfo::<Identity, OFFSET>,
            SetBreakWordType: SetBreakWordType::<Identity, OFFSET>,
            GetBreakWordType: GetBreakWordType::<Identity, OFFSET>,
            SetControlInfo: SetControlInfo::<Identity, OFFSET>,
            GetControlInfo: GetControlInfo::<Identity, OFFSET>,
            LoadExternalBreakerData: LoadExternalBreakerData::<Identity, OFFSET>,
            SetWordStemmer: SetWordStemmer::<Identity, OFFSET>,
            GetWordStemmer: GetWordStemmer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWordBreakerConfig as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Search"))]
impl windows_core::RuntimeName for IWordBreakerConfig {}
pub const MAX_COLUMNS: u32 = 256u32;
pub type PFNCOLHEAPFREE = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void) -> i32>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PRIORITY(pub i32);
pub const PRIORITY_HIGH: PRIORITY = PRIORITY(2i32);
pub const PRIORITY_LOW: PRIORITY = PRIORITY(0i32);
pub const PRIORITY_NORMAL: PRIORITY = PRIORITY(1i32);
pub const PROP_ADD: u32 = 0u32;
pub const PROP_DELETE: u32 = 1u32;
pub const PROP_UPDATE: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ROWSTATUS {
    pub lRowFirst: i32,
    pub cRows: i32,
    pub cProperties: i32,
    pub cRowsTotal: i32,
}
pub const STDPROP_DISPLAYKEY: u32 = 101u32;
pub const STDPROP_INDEX_BREAK: u32 = 204u32;
pub const STDPROP_INDEX_DTYPE: u32 = 202u32;
pub const STDPROP_INDEX_LENGTH: u32 = 203u32;
pub const STDPROP_INDEX_TERM: u32 = 210u32;
pub const STDPROP_INDEX_TERM_RAW_LENGTH: u32 = 211u32;
pub const STDPROP_INDEX_TEXT: u32 = 200u32;
pub const STDPROP_INDEX_VFLD: u32 = 201u32;
pub const STDPROP_KEY: u32 = 4u32;
pub const STDPROP_SORTKEY: u32 = 100u32;
pub const STDPROP_SORTORDINAL: u32 = 102u32;
pub const STDPROP_TITLE: u32 = 2u32;
pub const STDPROP_UID: u32 = 1u32;
pub const STDPROP_USERDATA: u32 = 3u32;
pub const STDPROP_USERPROP_BASE: u32 = 65536u32;
pub const STDPROP_USERPROP_MAX: u32 = 2147483647u32;
pub const SZ_WWDEST_GLOBAL: windows_core::PCWSTR = windows_core::w!("GLOBAL");
pub const SZ_WWDEST_KEY: windows_core::PCWSTR = windows_core::w!("KEY");
pub const SZ_WWDEST_OCC: windows_core::PCWSTR = windows_core::w!("OCC");
pub const TYPE_POINTER: u32 = 1u32;
pub const TYPE_STRING: u32 = 2u32;
pub const TYPE_VALUE: u32 = 0u32;
