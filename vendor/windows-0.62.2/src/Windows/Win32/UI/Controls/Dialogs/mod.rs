#[inline]
pub unsafe fn ChooseColorA(param0: *mut CHOOSECOLORA) -> windows_core::BOOL {
    windows_core::link!("comdlg32.dll" "system" fn ChooseColorA(param0 : *mut CHOOSECOLORA) -> windows_core::BOOL);
    unsafe { ChooseColorA(param0 as _) }
}
#[inline]
pub unsafe fn ChooseColorW(param0: *mut CHOOSECOLORW) -> windows_core::BOOL {
    windows_core::link!("comdlg32.dll" "system" fn ChooseColorW(param0 : *mut CHOOSECOLORW) -> windows_core::BOOL);
    unsafe { ChooseColorW(param0 as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ChooseFontA(param0: *mut CHOOSEFONTA) -> windows_core::BOOL {
    windows_core::link!("comdlg32.dll" "system" fn ChooseFontA(param0 : *mut CHOOSEFONTA) -> windows_core::BOOL);
    unsafe { ChooseFontA(param0 as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ChooseFontW(param0: *mut CHOOSEFONTW) -> windows_core::BOOL {
    windows_core::link!("comdlg32.dll" "system" fn ChooseFontW(param0 : *mut CHOOSEFONTW) -> windows_core::BOOL);
    unsafe { ChooseFontW(param0 as _) }
}
#[inline]
pub unsafe fn CommDlgExtendedError() -> COMMON_DLG_ERRORS {
    windows_core::link!("comdlg32.dll" "system" fn CommDlgExtendedError() -> COMMON_DLG_ERRORS);
    unsafe { CommDlgExtendedError() }
}
#[inline]
pub unsafe fn FindTextA(param0: *mut FINDREPLACEA) -> super::super::super::Foundation::HWND {
    windows_core::link!("comdlg32.dll" "system" fn FindTextA(param0 : *mut FINDREPLACEA) -> super::super::super::Foundation:: HWND);
    unsafe { FindTextA(param0 as _) }
}
#[inline]
pub unsafe fn FindTextW(param0: *mut FINDREPLACEW) -> super::super::super::Foundation::HWND {
    windows_core::link!("comdlg32.dll" "system" fn FindTextW(param0 : *mut FINDREPLACEW) -> super::super::super::Foundation:: HWND);
    unsafe { FindTextW(param0 as _) }
}
#[inline]
pub unsafe fn GetFileTitleA<P0>(param0: P0, buf: &mut [u8]) -> i16
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("comdlg32.dll" "system" fn GetFileTitleA(param0 : windows_core::PCSTR, buf : windows_core::PSTR, cchsize : u16) -> i16);
    unsafe { GetFileTitleA(param0.param().abi(), core::mem::transmute(buf.as_ptr()), buf.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn GetFileTitleW<P0>(param0: P0, buf: &mut [u16]) -> i16
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("comdlg32.dll" "system" fn GetFileTitleW(param0 : windows_core::PCWSTR, buf : windows_core::PWSTR, cchsize : u16) -> i16);
    unsafe { GetFileTitleW(param0.param().abi(), core::mem::transmute(buf.as_ptr()), buf.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn GetOpenFileNameA(param0: *mut OPENFILENAMEA) -> windows_core::BOOL {
    windows_core::link!("comdlg32.dll" "system" fn GetOpenFileNameA(param0 : *mut OPENFILENAMEA) -> windows_core::BOOL);
    unsafe { GetOpenFileNameA(param0 as _) }
}
#[inline]
pub unsafe fn GetOpenFileNameW(param0: *mut OPENFILENAMEW) -> windows_core::BOOL {
    windows_core::link!("comdlg32.dll" "system" fn GetOpenFileNameW(param0 : *mut OPENFILENAMEW) -> windows_core::BOOL);
    unsafe { GetOpenFileNameW(param0 as _) }
}
#[inline]
pub unsafe fn GetSaveFileNameA(param0: *mut OPENFILENAMEA) -> windows_core::BOOL {
    windows_core::link!("comdlg32.dll" "system" fn GetSaveFileNameA(param0 : *mut OPENFILENAMEA) -> windows_core::BOOL);
    unsafe { GetSaveFileNameA(param0 as _) }
}
#[inline]
pub unsafe fn GetSaveFileNameW(param0: *mut OPENFILENAMEW) -> windows_core::BOOL {
    windows_core::link!("comdlg32.dll" "system" fn GetSaveFileNameW(param0 : *mut OPENFILENAMEW) -> windows_core::BOOL);
    unsafe { GetSaveFileNameW(param0 as _) }
}
#[inline]
pub unsafe fn PageSetupDlgA(param0: *mut PAGESETUPDLGA) -> windows_core::BOOL {
    windows_core::link!("comdlg32.dll" "system" fn PageSetupDlgA(param0 : *mut PAGESETUPDLGA) -> windows_core::BOOL);
    unsafe { PageSetupDlgA(param0 as _) }
}
#[inline]
pub unsafe fn PageSetupDlgW(param0: *mut PAGESETUPDLGW) -> windows_core::BOOL {
    windows_core::link!("comdlg32.dll" "system" fn PageSetupDlgW(param0 : *mut PAGESETUPDLGW) -> windows_core::BOOL);
    unsafe { PageSetupDlgW(param0 as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn PrintDlgA(ppd: *mut PRINTDLGA) -> windows_core::BOOL {
    windows_core::link!("comdlg32.dll" "system" fn PrintDlgA(ppd : *mut PRINTDLGA) -> windows_core::BOOL);
    unsafe { PrintDlgA(ppd as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn PrintDlgExA(ppd: *mut PRINTDLGEXA) -> windows_core::Result<()> {
    windows_core::link!("comdlg32.dll" "system" fn PrintDlgExA(ppd : *mut PRINTDLGEXA) -> windows_core::HRESULT);
    unsafe { PrintDlgExA(core::mem::transmute(ppd)).ok() }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn PrintDlgExW(ppd: *mut PRINTDLGEXW) -> windows_core::Result<()> {
    windows_core::link!("comdlg32.dll" "system" fn PrintDlgExW(ppd : *mut PRINTDLGEXW) -> windows_core::HRESULT);
    unsafe { PrintDlgExW(core::mem::transmute(ppd)).ok() }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn PrintDlgW(ppd: *mut PRINTDLGW) -> windows_core::BOOL {
    windows_core::link!("comdlg32.dll" "system" fn PrintDlgW(ppd : *mut PRINTDLGW) -> windows_core::BOOL);
    unsafe { PrintDlgW(ppd as _) }
}
#[inline]
pub unsafe fn ReplaceTextA(param0: *mut FINDREPLACEA) -> super::super::super::Foundation::HWND {
    windows_core::link!("comdlg32.dll" "system" fn ReplaceTextA(param0 : *mut FINDREPLACEA) -> super::super::super::Foundation:: HWND);
    unsafe { ReplaceTextA(param0 as _) }
}
#[inline]
pub unsafe fn ReplaceTextW(param0: *mut FINDREPLACEW) -> super::super::super::Foundation::HWND {
    windows_core::link!("comdlg32.dll" "system" fn ReplaceTextW(param0 : *mut FINDREPLACEW) -> super::super::super::Foundation:: HWND);
    unsafe { ReplaceTextW(param0 as _) }
}
pub const BOLD_FONTTYPE: CHOOSEFONT_FONT_TYPE = CHOOSEFONT_FONT_TYPE(256u16);
pub const CCERR_CHOOSECOLORCODES: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(20480u32);
pub const CC_ANYCOLOR: CHOOSECOLOR_FLAGS = CHOOSECOLOR_FLAGS(256u32);
pub const CC_ENABLEHOOK: CHOOSECOLOR_FLAGS = CHOOSECOLOR_FLAGS(16u32);
pub const CC_ENABLETEMPLATE: CHOOSECOLOR_FLAGS = CHOOSECOLOR_FLAGS(32u32);
pub const CC_ENABLETEMPLATEHANDLE: CHOOSECOLOR_FLAGS = CHOOSECOLOR_FLAGS(64u32);
pub const CC_FULLOPEN: CHOOSECOLOR_FLAGS = CHOOSECOLOR_FLAGS(2u32);
pub const CC_PREVENTFULLOPEN: CHOOSECOLOR_FLAGS = CHOOSECOLOR_FLAGS(4u32);
pub const CC_RGBINIT: CHOOSECOLOR_FLAGS = CHOOSECOLOR_FLAGS(1u32);
pub const CC_SHOWHELP: CHOOSECOLOR_FLAGS = CHOOSECOLOR_FLAGS(8u32);
pub const CC_SOLIDCOLOR: CHOOSECOLOR_FLAGS = CHOOSECOLOR_FLAGS(128u32);
pub const CDERR_DIALOGFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(65535u32);
pub const CDERR_FINDRESFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(6u32);
pub const CDERR_GENERALCODES: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(0u32);
pub const CDERR_INITIALIZATION: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(2u32);
pub const CDERR_LOADRESFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(7u32);
pub const CDERR_LOADSTRFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(5u32);
pub const CDERR_LOCKRESFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(8u32);
pub const CDERR_MEMALLOCFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(9u32);
pub const CDERR_MEMLOCKFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(10u32);
pub const CDERR_NOHINSTANCE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4u32);
pub const CDERR_NOHOOK: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(11u32);
pub const CDERR_NOTEMPLATE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(3u32);
pub const CDERR_REGISTERMSGFAIL: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(12u32);
pub const CDERR_STRUCTSIZE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(1u32);
pub const CDM_FIRST: u32 = 1124u32;
pub const CDM_GETFILEPATH: u32 = 1125u32;
pub const CDM_GETFOLDERIDLIST: u32 = 1127u32;
pub const CDM_GETFOLDERPATH: u32 = 1126u32;
pub const CDM_GETSPEC: u32 = 1124u32;
pub const CDM_HIDECONTROL: u32 = 1129u32;
pub const CDM_LAST: u32 = 1224u32;
pub const CDM_SETCONTROLTEXT: u32 = 1128u32;
pub const CDM_SETDEFEXT: u32 = 1130u32;
pub const CDN_FILEOK: u32 = 4294966690u32;
pub const CDN_FOLDERCHANGE: u32 = 4294966693u32;
pub const CDN_HELP: u32 = 4294966691u32;
pub const CDN_INCLUDEITEM: u32 = 4294966688u32;
pub const CDN_INITDONE: u32 = 4294966695u32;
pub const CDN_SELCHANGE: u32 = 4294966694u32;
pub const CDN_SHAREVIOLATION: u32 = 4294966692u32;
pub const CDN_TYPECHANGE: u32 = 4294966689u32;
pub const CD_LBSELADD: u32 = 2u32;
pub const CD_LBSELCHANGE: u32 = 0u32;
pub const CD_LBSELNOITEMS: i32 = -1i32;
pub const CD_LBSELSUB: u32 = 1u32;
pub const CFERR_CHOOSEFONTCODES: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(8192u32);
pub const CFERR_MAXLESSTHANMIN: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(8194u32);
pub const CFERR_NOFONTS: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(8193u32);
pub const CF_ANSIONLY: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(1024u32);
pub const CF_APPLY: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(512u32);
pub const CF_BOTH: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(3u32);
pub const CF_EFFECTS: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(256u32);
pub const CF_ENABLEHOOK: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(8u32);
pub const CF_ENABLETEMPLATE: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(16u32);
pub const CF_ENABLETEMPLATEHANDLE: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(32u32);
pub const CF_FIXEDPITCHONLY: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(16384u32);
pub const CF_FORCEFONTEXIST: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(65536u32);
pub const CF_INACTIVEFONTS: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(33554432u32);
pub const CF_INITTOLOGFONTSTRUCT: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(64u32);
pub const CF_LIMITSIZE: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(8192u32);
pub const CF_NOFACESEL: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(524288u32);
pub const CF_NOOEMFONTS: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(2048u32);
pub const CF_NOSCRIPTSEL: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(8388608u32);
pub const CF_NOSIMULATIONS: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(4096u32);
pub const CF_NOSIZESEL: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(2097152u32);
pub const CF_NOSTYLESEL: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(1048576u32);
pub const CF_NOVECTORFONTS: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(2048u32);
pub const CF_NOVERTFONTS: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(16777216u32);
pub const CF_PRINTERFONTS: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(2u32);
pub const CF_SCALABLEONLY: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(131072u32);
pub const CF_SCREENFONTS: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(1u32);
pub const CF_SCRIPTSONLY: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(1024u32);
pub const CF_SELECTSCRIPT: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(4194304u32);
pub const CF_SHOWHELP: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(4u32);
pub const CF_TTONLY: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(262144u32);
pub const CF_USESTYLE: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(128u32);
pub const CF_WYSIWYG: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(32768u32);
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct CHOOSECOLORA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HWND,
    pub rgbResult: super::super::super::Foundation::COLORREF,
    pub lpCustColors: *mut super::super::super::Foundation::COLORREF,
    pub Flags: CHOOSECOLOR_FLAGS,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPCCHOOKPROC,
    pub lpTemplateName: windows_core::PCSTR,
}
#[cfg(target_arch = "x86")]
impl Default for CHOOSECOLORA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug)]
pub struct CHOOSECOLORA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HWND,
    pub rgbResult: super::super::super::Foundation::COLORREF,
    pub lpCustColors: *mut super::super::super::Foundation::COLORREF,
    pub Flags: CHOOSECOLOR_FLAGS,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPCCHOOKPROC,
    pub lpTemplateName: windows_core::PCSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for CHOOSECOLORA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct CHOOSECOLORW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HWND,
    pub rgbResult: super::super::super::Foundation::COLORREF,
    pub lpCustColors: *mut super::super::super::Foundation::COLORREF,
    pub Flags: CHOOSECOLOR_FLAGS,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPCCHOOKPROC,
    pub lpTemplateName: windows_core::PCWSTR,
}
#[cfg(target_arch = "x86")]
impl Default for CHOOSECOLORW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug)]
pub struct CHOOSECOLORW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HWND,
    pub rgbResult: super::super::super::Foundation::COLORREF,
    pub lpCustColors: *mut super::super::super::Foundation::COLORREF,
    pub Flags: CHOOSECOLOR_FLAGS,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPCCHOOKPROC,
    pub lpTemplateName: windows_core::PCWSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for CHOOSECOLORW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CHOOSECOLOR_FLAGS(pub u32);
impl CHOOSECOLOR_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CHOOSECOLOR_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CHOOSECOLOR_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CHOOSECOLOR_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CHOOSECOLOR_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CHOOSECOLOR_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy)]
pub struct CHOOSEFONTA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub lpLogFont: *mut super::super::super::Graphics::Gdi::LOGFONTA,
    pub iPointSize: i32,
    pub Flags: CHOOSEFONT_FLAGS,
    pub rgbColors: super::super::super::Foundation::COLORREF,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPCFHOOKPROC,
    pub lpTemplateName: windows_core::PCSTR,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpszStyle: windows_core::PSTR,
    pub nFontType: CHOOSEFONT_FONT_TYPE,
    pub ___MISSING_ALIGNMENT__: u16,
    pub nSizeMin: i32,
    pub nSizeMax: i32,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for CHOOSEFONTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug)]
pub struct CHOOSEFONTA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub lpLogFont: *mut super::super::super::Graphics::Gdi::LOGFONTA,
    pub iPointSize: i32,
    pub Flags: CHOOSEFONT_FLAGS,
    pub rgbColors: super::super::super::Foundation::COLORREF,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPCFHOOKPROC,
    pub lpTemplateName: windows_core::PCSTR,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpszStyle: windows_core::PSTR,
    pub nFontType: CHOOSEFONT_FONT_TYPE,
    pub ___MISSING_ALIGNMENT__: u16,
    pub nSizeMin: i32,
    pub nSizeMax: i32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for CHOOSEFONTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy)]
pub struct CHOOSEFONTW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub lpLogFont: *mut super::super::super::Graphics::Gdi::LOGFONTW,
    pub iPointSize: i32,
    pub Flags: CHOOSEFONT_FLAGS,
    pub rgbColors: super::super::super::Foundation::COLORREF,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPCFHOOKPROC,
    pub lpTemplateName: windows_core::PCWSTR,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpszStyle: windows_core::PWSTR,
    pub nFontType: CHOOSEFONT_FONT_TYPE,
    pub ___MISSING_ALIGNMENT__: u16,
    pub nSizeMin: i32,
    pub nSizeMax: i32,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for CHOOSEFONTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug)]
pub struct CHOOSEFONTW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub lpLogFont: *mut super::super::super::Graphics::Gdi::LOGFONTW,
    pub iPointSize: i32,
    pub Flags: CHOOSEFONT_FLAGS,
    pub rgbColors: super::super::super::Foundation::COLORREF,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPCFHOOKPROC,
    pub lpTemplateName: windows_core::PCWSTR,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpszStyle: windows_core::PWSTR,
    pub nFontType: CHOOSEFONT_FONT_TYPE,
    pub ___MISSING_ALIGNMENT__: u16,
    pub nSizeMin: i32,
    pub nSizeMax: i32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for CHOOSEFONTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CHOOSEFONT_FLAGS(pub u32);
impl CHOOSEFONT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CHOOSEFONT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CHOOSEFONT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CHOOSEFONT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CHOOSEFONT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CHOOSEFONT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CHOOSEFONT_FONT_TYPE(pub u16);
impl CHOOSEFONT_FONT_TYPE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CHOOSEFONT_FONT_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CHOOSEFONT_FONT_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CHOOSEFONT_FONT_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CHOOSEFONT_FONT_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CHOOSEFONT_FONT_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const COLOROKSTRING: windows_core::PCWSTR = windows_core::w!("commdlg_ColorOK");
pub const COLOROKSTRINGA: windows_core::PCSTR = windows_core::s!("commdlg_ColorOK");
pub const COLOROKSTRINGW: windows_core::PCWSTR = windows_core::w!("commdlg_ColorOK");
pub const COLOR_ADD: u32 = 712u32;
pub const COLOR_BLUE: u32 = 708u32;
pub const COLOR_BLUEACCEL: u32 = 728u32;
pub const COLOR_BOX1: u32 = 720u32;
pub const COLOR_CURRENT: u32 = 709u32;
pub const COLOR_CUSTOM1: u32 = 721u32;
pub const COLOR_ELEMENT: u32 = 716u32;
pub const COLOR_GREEN: u32 = 707u32;
pub const COLOR_GREENACCEL: u32 = 727u32;
pub const COLOR_HUE: u32 = 703u32;
pub const COLOR_HUEACCEL: u32 = 723u32;
pub const COLOR_HUESCROLL: u32 = 700u32;
pub const COLOR_LUM: u32 = 705u32;
pub const COLOR_LUMACCEL: u32 = 725u32;
pub const COLOR_LUMSCROLL: u32 = 702u32;
pub const COLOR_MIX: u32 = 719u32;
pub const COLOR_PALETTE: u32 = 718u32;
pub const COLOR_RAINBOW: u32 = 710u32;
pub const COLOR_RED: u32 = 706u32;
pub const COLOR_REDACCEL: u32 = 726u32;
pub const COLOR_SAMPLES: u32 = 717u32;
pub const COLOR_SAT: u32 = 704u32;
pub const COLOR_SATACCEL: u32 = 724u32;
pub const COLOR_SATSCROLL: u32 = 701u32;
pub const COLOR_SAVE: u32 = 711u32;
pub const COLOR_SCHEMES: u32 = 715u32;
pub const COLOR_SOLID: u32 = 713u32;
pub const COLOR_SOLID_LEFT: u32 = 730u32;
pub const COLOR_SOLID_RIGHT: u32 = 731u32;
pub const COLOR_TUNE: u32 = 714u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COMMON_DLG_ERRORS(pub u32);
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct DEVNAMES {
    pub wDriverOffset: u16,
    pub wDeviceOffset: u16,
    pub wOutputOffset: u16,
    pub wDefault: u16,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DEVNAMES {
    pub wDriverOffset: u16,
    pub wDeviceOffset: u16,
    pub wOutputOffset: u16,
    pub wDefault: u16,
}
pub const DLG_COLOR: u32 = 10u32;
pub const DN_DEFAULTPRN: u32 = 1u32;
pub const FILEOKSTRING: windows_core::PCWSTR = windows_core::w!("commdlg_FileNameOK");
pub const FILEOKSTRINGA: windows_core::PCSTR = windows_core::s!("commdlg_FileNameOK");
pub const FILEOKSTRINGW: windows_core::PCWSTR = windows_core::w!("commdlg_FileNameOK");
pub const FINDMSGSTRING: windows_core::PCWSTR = windows_core::w!("commdlg_FindReplace");
pub const FINDMSGSTRINGA: windows_core::PCSTR = windows_core::s!("commdlg_FindReplace");
pub const FINDMSGSTRINGW: windows_core::PCWSTR = windows_core::w!("commdlg_FindReplace");
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct FINDREPLACEA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub Flags: FINDREPLACE_FLAGS,
    pub lpstrFindWhat: windows_core::PSTR,
    pub lpstrReplaceWith: windows_core::PSTR,
    pub wFindWhatLen: u16,
    pub wReplaceWithLen: u16,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPFRHOOKPROC,
    pub lpTemplateName: windows_core::PCSTR,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct FINDREPLACEA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub Flags: FINDREPLACE_FLAGS,
    pub lpstrFindWhat: windows_core::PSTR,
    pub lpstrReplaceWith: windows_core::PSTR,
    pub wFindWhatLen: u16,
    pub wReplaceWithLen: u16,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPFRHOOKPROC,
    pub lpTemplateName: windows_core::PCSTR,
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct FINDREPLACEW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub Flags: FINDREPLACE_FLAGS,
    pub lpstrFindWhat: windows_core::PWSTR,
    pub lpstrReplaceWith: windows_core::PWSTR,
    pub wFindWhatLen: u16,
    pub wReplaceWithLen: u16,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPFRHOOKPROC,
    pub lpTemplateName: windows_core::PCWSTR,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct FINDREPLACEW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub Flags: FINDREPLACE_FLAGS,
    pub lpstrFindWhat: windows_core::PWSTR,
    pub lpstrReplaceWith: windows_core::PWSTR,
    pub wFindWhatLen: u16,
    pub wReplaceWithLen: u16,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPFRHOOKPROC,
    pub lpTemplateName: windows_core::PCWSTR,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FINDREPLACE_FLAGS(pub u32);
impl FINDREPLACE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for FINDREPLACE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for FINDREPLACE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for FINDREPLACE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for FINDREPLACE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for FINDREPLACE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const FNERR_BUFFERTOOSMALL: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(12291u32);
pub const FNERR_FILENAMECODES: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(12288u32);
pub const FNERR_INVALIDFILENAME: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(12290u32);
pub const FNERR_SUBCLASSFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(12289u32);
pub const FRERR_BUFFERLENGTHZERO: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(16385u32);
pub const FRERR_FINDREPLACECODES: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(16384u32);
pub const FRM_FIRST: u32 = 1124u32;
pub const FRM_LAST: u32 = 1224u32;
pub const FRM_SETOPERATIONRESULT: u32 = 1124u32;
pub const FRM_SETOPERATIONRESULTTEXT: u32 = 1125u32;
pub const FR_DIALOGTERM: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(64u32);
pub const FR_DOWN: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(1u32);
pub const FR_ENABLEHOOK: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(256u32);
pub const FR_ENABLETEMPLATE: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(512u32);
pub const FR_ENABLETEMPLATEHANDLE: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(8192u32);
pub const FR_FINDNEXT: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(8u32);
pub const FR_HIDEMATCHCASE: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(32768u32);
pub const FR_HIDEUPDOWN: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(16384u32);
pub const FR_HIDEWHOLEWORD: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(65536u32);
pub const FR_MATCHALEFHAMZA: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(2147483648u32);
pub const FR_MATCHCASE: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(4u32);
pub const FR_MATCHDIAC: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(536870912u32);
pub const FR_MATCHKASHIDA: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(1073741824u32);
pub const FR_NOMATCHCASE: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(2048u32);
pub const FR_NOUPDOWN: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(1024u32);
pub const FR_NOWHOLEWORD: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(4096u32);
pub const FR_NOWRAPAROUND: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(524288u32);
pub const FR_RAW: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(131072u32);
pub const FR_REPLACE: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(16u32);
pub const FR_REPLACEALL: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(32u32);
pub const FR_SHOWHELP: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(128u32);
pub const FR_SHOWWRAPAROUND: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(262144u32);
pub const FR_WHOLEWORD: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(2u32);
pub const FR_WRAPAROUND: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(1048576u32);
pub const HELPMSGSTRING: windows_core::PCWSTR = windows_core::w!("commdlg_help");
pub const HELPMSGSTRINGA: windows_core::PCSTR = windows_core::s!("commdlg_help");
pub const HELPMSGSTRINGW: windows_core::PCWSTR = windows_core::w!("commdlg_help");
windows_core::imp::define_interface!(IPrintDialogCallback, IPrintDialogCallback_Vtbl, 0x5852a2c3_6530_11d1_b6a3_0000f8757bf9);
windows_core::imp::interface_hierarchy!(IPrintDialogCallback, windows_core::IUnknown);
impl IPrintDialogCallback {
    pub unsafe fn InitDone(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InitDone)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SelectionChange(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SelectionChange)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn HandleMessage(&self, hdlg: super::super::super::Foundation::HWND, umsg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, presult: *mut super::super::super::Foundation::LRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).HandleMessage)(windows_core::Interface::as_raw(self), hdlg, umsg, wparam, lparam, presult as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintDialogCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InitDone: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SelectionChange: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HandleMessage: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HWND, u32, super::super::super::Foundation::WPARAM, super::super::super::Foundation::LPARAM, *mut super::super::super::Foundation::LRESULT) -> windows_core::HRESULT,
}
pub trait IPrintDialogCallback_Impl: windows_core::IUnknownImpl {
    fn InitDone(&self) -> windows_core::Result<()>;
    fn SelectionChange(&self) -> windows_core::Result<()>;
    fn HandleMessage(&self, hdlg: super::super::super::Foundation::HWND, umsg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, presult: *mut super::super::super::Foundation::LRESULT) -> windows_core::Result<()>;
}
impl IPrintDialogCallback_Vtbl {
    pub const fn new<Identity: IPrintDialogCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InitDone<Identity: IPrintDialogCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintDialogCallback_Impl::InitDone(this).into()
            }
        }
        unsafe extern "system" fn SelectionChange<Identity: IPrintDialogCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintDialogCallback_Impl::SelectionChange(this).into()
            }
        }
        unsafe extern "system" fn HandleMessage<Identity: IPrintDialogCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdlg: super::super::super::Foundation::HWND, umsg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, presult: *mut super::super::super::Foundation::LRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintDialogCallback_Impl::HandleMessage(this, core::mem::transmute_copy(&hdlg), core::mem::transmute_copy(&umsg), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam), core::mem::transmute_copy(&presult)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InitDone: InitDone::<Identity, OFFSET>,
            SelectionChange: SelectionChange::<Identity, OFFSET>,
            HandleMessage: HandleMessage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintDialogCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPrintDialogCallback {}
windows_core::imp::define_interface!(IPrintDialogServices, IPrintDialogServices_Vtbl, 0x509aaeda_5639_11d1_b6a1_0000f8757bf9);
windows_core::imp::interface_hierarchy!(IPrintDialogServices, windows_core::IUnknown);
impl IPrintDialogServices {
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetCurrentDevMode(&self, pdevmode: *mut super::super::super::Graphics::Gdi::DEVMODEA, pcbsize: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCurrentDevMode)(windows_core::Interface::as_raw(self), pdevmode as _, pcbsize as _).ok() }
    }
    pub unsafe fn GetCurrentPrinterName(&self, pprintername: Option<windows_core::PWSTR>, pcchsize: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCurrentPrinterName)(windows_core::Interface::as_raw(self), pprintername.unwrap_or(core::mem::zeroed()) as _, pcchsize as _).ok() }
    }
    pub unsafe fn GetCurrentPortName(&self, pportname: Option<windows_core::PWSTR>, pcchsize: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCurrentPortName)(windows_core::Interface::as_raw(self), pportname.unwrap_or(core::mem::zeroed()) as _, pcchsize as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintDialogServices_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetCurrentDevMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Graphics::Gdi::DEVMODEA, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetCurrentDevMode: usize,
    pub GetCurrentPrinterName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub GetCurrentPortName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IPrintDialogServices_Impl: windows_core::IUnknownImpl {
    fn GetCurrentDevMode(&self, pdevmode: *mut super::super::super::Graphics::Gdi::DEVMODEA, pcbsize: *mut u32) -> windows_core::Result<()>;
    fn GetCurrentPrinterName(&self, pprintername: windows_core::PWSTR, pcchsize: *mut u32) -> windows_core::Result<()>;
    fn GetCurrentPortName(&self, pportname: windows_core::PWSTR, pcchsize: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IPrintDialogServices_Vtbl {
    pub const fn new<Identity: IPrintDialogServices_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCurrentDevMode<Identity: IPrintDialogServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevmode: *mut super::super::super::Graphics::Gdi::DEVMODEA, pcbsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintDialogServices_Impl::GetCurrentDevMode(this, core::mem::transmute_copy(&pdevmode), core::mem::transmute_copy(&pcbsize)).into()
            }
        }
        unsafe extern "system" fn GetCurrentPrinterName<Identity: IPrintDialogServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprintername: windows_core::PWSTR, pcchsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintDialogServices_Impl::GetCurrentPrinterName(this, core::mem::transmute_copy(&pprintername), core::mem::transmute_copy(&pcchsize)).into()
            }
        }
        unsafe extern "system" fn GetCurrentPortName<Identity: IPrintDialogServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pportname: windows_core::PWSTR, pcchsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintDialogServices_Impl::GetCurrentPortName(this, core::mem::transmute_copy(&pportname), core::mem::transmute_copy(&pcchsize)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrentDevMode: GetCurrentDevMode::<Identity, OFFSET>,
            GetCurrentPrinterName: GetCurrentPrinterName::<Identity, OFFSET>,
            GetCurrentPortName: GetCurrentPortName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintDialogServices as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IPrintDialogServices {}
pub const ITALIC_FONTTYPE: CHOOSEFONT_FONT_TYPE = CHOOSEFONT_FONT_TYPE(512u16);
pub const LBSELCHSTRING: windows_core::PCWSTR = windows_core::w!("commdlg_LBSelChangedNotify");
pub const LBSELCHSTRINGA: windows_core::PCSTR = windows_core::s!("commdlg_LBSelChangedNotify");
pub const LBSELCHSTRINGW: windows_core::PCWSTR = windows_core::w!("commdlg_LBSelChangedNotify");
pub type LPCCHOOKPROC = Option<unsafe extern "system" fn(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> usize>;
pub type LPCFHOOKPROC = Option<unsafe extern "system" fn(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> usize>;
pub type LPFRHOOKPROC = Option<unsafe extern "system" fn(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> usize>;
pub type LPOFNHOOKPROC = Option<unsafe extern "system" fn(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> usize>;
pub type LPPAGEPAINTHOOK = Option<unsafe extern "system" fn(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> usize>;
pub type LPPAGESETUPHOOK = Option<unsafe extern "system" fn(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> usize>;
pub type LPPRINTHOOKPROC = Option<unsafe extern "system" fn(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> usize>;
pub type LPSETUPHOOKPROC = Option<unsafe extern "system" fn(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> usize>;
pub const NUM_BASIC_COLORS: u32 = 48u32;
pub const NUM_CUSTOM_COLORS: u32 = 16u32;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct OFNOTIFYA {
    pub hdr: super::NMHDR,
    pub lpOFN: *mut OPENFILENAMEA,
    pub pszFile: windows_core::PSTR,
}
#[cfg(target_arch = "x86")]
impl Default for OFNOTIFYA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OFNOTIFYA {
    pub hdr: super::NMHDR,
    pub lpOFN: *mut OPENFILENAMEA,
    pub pszFile: windows_core::PSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for OFNOTIFYA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct OFNOTIFYEXA {
    pub hdr: super::NMHDR,
    pub lpOFN: *mut OPENFILENAMEA,
    pub psf: *mut core::ffi::c_void,
    pub pidl: *mut core::ffi::c_void,
}
#[cfg(target_arch = "x86")]
impl Default for OFNOTIFYEXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OFNOTIFYEXA {
    pub hdr: super::NMHDR,
    pub lpOFN: *mut OPENFILENAMEA,
    pub psf: *mut core::ffi::c_void,
    pub pidl: *mut core::ffi::c_void,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for OFNOTIFYEXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct OFNOTIFYEXW {
    pub hdr: super::NMHDR,
    pub lpOFN: *mut OPENFILENAMEW,
    pub psf: *mut core::ffi::c_void,
    pub pidl: *mut core::ffi::c_void,
}
#[cfg(target_arch = "x86")]
impl Default for OFNOTIFYEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OFNOTIFYEXW {
    pub hdr: super::NMHDR,
    pub lpOFN: *mut OPENFILENAMEW,
    pub psf: *mut core::ffi::c_void,
    pub pidl: *mut core::ffi::c_void,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for OFNOTIFYEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct OFNOTIFYW {
    pub hdr: super::NMHDR,
    pub lpOFN: *mut OPENFILENAMEW,
    pub pszFile: windows_core::PWSTR,
}
#[cfg(target_arch = "x86")]
impl Default for OFNOTIFYW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OFNOTIFYW {
    pub hdr: super::NMHDR,
    pub lpOFN: *mut OPENFILENAMEW,
    pub pszFile: windows_core::PWSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for OFNOTIFYW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const OFN_ALLOWMULTISELECT: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(512u32);
pub const OFN_CREATEPROMPT: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(8192u32);
pub const OFN_DONTADDTORECENT: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(33554432u32);
pub const OFN_ENABLEHOOK: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(32u32);
pub const OFN_ENABLEINCLUDENOTIFY: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(4194304u32);
pub const OFN_ENABLESIZING: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(8388608u32);
pub const OFN_ENABLETEMPLATE: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(64u32);
pub const OFN_ENABLETEMPLATEHANDLE: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(128u32);
pub const OFN_EXPLORER: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(524288u32);
pub const OFN_EXTENSIONDIFFERENT: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(1024u32);
pub const OFN_EX_NONE: OPEN_FILENAME_FLAGS_EX = OPEN_FILENAME_FLAGS_EX(0u32);
pub const OFN_EX_NOPLACESBAR: OPEN_FILENAME_FLAGS_EX = OPEN_FILENAME_FLAGS_EX(1u32);
pub const OFN_FILEMUSTEXIST: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(4096u32);
pub const OFN_FORCESHOWHIDDEN: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(268435456u32);
pub const OFN_HIDEREADONLY: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(4u32);
pub const OFN_LONGNAMES: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(2097152u32);
pub const OFN_NOCHANGEDIR: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(8u32);
pub const OFN_NODEREFERENCELINKS: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(1048576u32);
pub const OFN_NOLONGNAMES: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(262144u32);
pub const OFN_NONETWORKBUTTON: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(131072u32);
pub const OFN_NOREADONLYRETURN: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(32768u32);
pub const OFN_NOTESTFILECREATE: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(65536u32);
pub const OFN_NOVALIDATE: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(256u32);
pub const OFN_OVERWRITEPROMPT: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(2u32);
pub const OFN_PATHMUSTEXIST: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(2048u32);
pub const OFN_READONLY: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(1u32);
pub const OFN_SHAREAWARE: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(16384u32);
pub const OFN_SHAREFALLTHROUGH: u32 = 2u32;
pub const OFN_SHARENOWARN: u32 = 1u32;
pub const OFN_SHAREWARN: u32 = 0u32;
pub const OFN_SHOWHELP: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(16u32);
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct OPENFILENAMEA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpstrFilter: windows_core::PCSTR,
    pub lpstrCustomFilter: windows_core::PSTR,
    pub nMaxCustFilter: u32,
    pub nFilterIndex: u32,
    pub lpstrFile: windows_core::PSTR,
    pub nMaxFile: u32,
    pub lpstrFileTitle: windows_core::PSTR,
    pub nMaxFileTitle: u32,
    pub lpstrInitialDir: windows_core::PCSTR,
    pub lpstrTitle: windows_core::PCSTR,
    pub Flags: OPEN_FILENAME_FLAGS,
    pub nFileOffset: u16,
    pub nFileExtension: u16,
    pub lpstrDefExt: windows_core::PCSTR,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: windows_core::PCSTR,
    pub pvReserved: *mut core::ffi::c_void,
    pub dwReserved: u32,
    pub FlagsEx: OPEN_FILENAME_FLAGS_EX,
}
#[cfg(target_arch = "x86")]
impl Default for OPENFILENAMEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug)]
pub struct OPENFILENAMEA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpstrFilter: windows_core::PCSTR,
    pub lpstrCustomFilter: windows_core::PSTR,
    pub nMaxCustFilter: u32,
    pub nFilterIndex: u32,
    pub lpstrFile: windows_core::PSTR,
    pub nMaxFile: u32,
    pub lpstrFileTitle: windows_core::PSTR,
    pub nMaxFileTitle: u32,
    pub lpstrInitialDir: windows_core::PCSTR,
    pub lpstrTitle: windows_core::PCSTR,
    pub Flags: OPEN_FILENAME_FLAGS,
    pub nFileOffset: u16,
    pub nFileExtension: u16,
    pub lpstrDefExt: windows_core::PCSTR,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: windows_core::PCSTR,
    pub pvReserved: *mut core::ffi::c_void,
    pub dwReserved: u32,
    pub FlagsEx: OPEN_FILENAME_FLAGS_EX,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for OPENFILENAMEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct OPENFILENAMEW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpstrFilter: windows_core::PCWSTR,
    pub lpstrCustomFilter: windows_core::PWSTR,
    pub nMaxCustFilter: u32,
    pub nFilterIndex: u32,
    pub lpstrFile: windows_core::PWSTR,
    pub nMaxFile: u32,
    pub lpstrFileTitle: windows_core::PWSTR,
    pub nMaxFileTitle: u32,
    pub lpstrInitialDir: windows_core::PCWSTR,
    pub lpstrTitle: windows_core::PCWSTR,
    pub Flags: OPEN_FILENAME_FLAGS,
    pub nFileOffset: u16,
    pub nFileExtension: u16,
    pub lpstrDefExt: windows_core::PCWSTR,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: windows_core::PCWSTR,
    pub pvReserved: *mut core::ffi::c_void,
    pub dwReserved: u32,
    pub FlagsEx: OPEN_FILENAME_FLAGS_EX,
}
#[cfg(target_arch = "x86")]
impl Default for OPENFILENAMEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug)]
pub struct OPENFILENAMEW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpstrFilter: windows_core::PCWSTR,
    pub lpstrCustomFilter: windows_core::PWSTR,
    pub nMaxCustFilter: u32,
    pub nFilterIndex: u32,
    pub lpstrFile: windows_core::PWSTR,
    pub nMaxFile: u32,
    pub lpstrFileTitle: windows_core::PWSTR,
    pub nMaxFileTitle: u32,
    pub lpstrInitialDir: windows_core::PCWSTR,
    pub lpstrTitle: windows_core::PCWSTR,
    pub Flags: OPEN_FILENAME_FLAGS,
    pub nFileOffset: u16,
    pub nFileExtension: u16,
    pub lpstrDefExt: windows_core::PCWSTR,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: windows_core::PCWSTR,
    pub pvReserved: *mut core::ffi::c_void,
    pub dwReserved: u32,
    pub FlagsEx: OPEN_FILENAME_FLAGS_EX,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for OPENFILENAMEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct OPENFILENAME_NT4A {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpstrFilter: windows_core::PCSTR,
    pub lpstrCustomFilter: windows_core::PSTR,
    pub nMaxCustFilter: u32,
    pub nFilterIndex: u32,
    pub lpstrFile: windows_core::PSTR,
    pub nMaxFile: u32,
    pub lpstrFileTitle: windows_core::PSTR,
    pub nMaxFileTitle: u32,
    pub lpstrInitialDir: windows_core::PCSTR,
    pub lpstrTitle: windows_core::PCSTR,
    pub Flags: u32,
    pub nFileOffset: u16,
    pub nFileExtension: u16,
    pub lpstrDefExt: windows_core::PCSTR,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: windows_core::PCSTR,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct OPENFILENAME_NT4A {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpstrFilter: windows_core::PCSTR,
    pub lpstrCustomFilter: windows_core::PSTR,
    pub nMaxCustFilter: u32,
    pub nFilterIndex: u32,
    pub lpstrFile: windows_core::PSTR,
    pub nMaxFile: u32,
    pub lpstrFileTitle: windows_core::PSTR,
    pub nMaxFileTitle: u32,
    pub lpstrInitialDir: windows_core::PCSTR,
    pub lpstrTitle: windows_core::PCSTR,
    pub Flags: u32,
    pub nFileOffset: u16,
    pub nFileExtension: u16,
    pub lpstrDefExt: windows_core::PCSTR,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: windows_core::PCSTR,
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct OPENFILENAME_NT4W {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpstrFilter: windows_core::PCWSTR,
    pub lpstrCustomFilter: windows_core::PWSTR,
    pub nMaxCustFilter: u32,
    pub nFilterIndex: u32,
    pub lpstrFile: windows_core::PWSTR,
    pub nMaxFile: u32,
    pub lpstrFileTitle: windows_core::PWSTR,
    pub nMaxFileTitle: u32,
    pub lpstrInitialDir: windows_core::PCWSTR,
    pub lpstrTitle: windows_core::PCWSTR,
    pub Flags: u32,
    pub nFileOffset: u16,
    pub nFileExtension: u16,
    pub lpstrDefExt: windows_core::PCWSTR,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: windows_core::PCWSTR,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct OPENFILENAME_NT4W {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpstrFilter: windows_core::PCWSTR,
    pub lpstrCustomFilter: windows_core::PWSTR,
    pub nMaxCustFilter: u32,
    pub nFilterIndex: u32,
    pub lpstrFile: windows_core::PWSTR,
    pub nMaxFile: u32,
    pub lpstrFileTitle: windows_core::PWSTR,
    pub nMaxFileTitle: u32,
    pub lpstrInitialDir: windows_core::PCWSTR,
    pub lpstrTitle: windows_core::PCWSTR,
    pub Flags: u32,
    pub nFileOffset: u16,
    pub nFileExtension: u16,
    pub lpstrDefExt: windows_core::PCWSTR,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: windows_core::PCWSTR,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OPEN_FILENAME_FLAGS(pub u32);
impl OPEN_FILENAME_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for OPEN_FILENAME_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for OPEN_FILENAME_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for OPEN_FILENAME_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for OPEN_FILENAME_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for OPEN_FILENAME_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OPEN_FILENAME_FLAGS_EX(pub u32);
impl OPEN_FILENAME_FLAGS_EX {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for OPEN_FILENAME_FLAGS_EX {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for OPEN_FILENAME_FLAGS_EX {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for OPEN_FILENAME_FLAGS_EX {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for OPEN_FILENAME_FLAGS_EX {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for OPEN_FILENAME_FLAGS_EX {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct PAGESETUPDLGA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: super::super::super::Foundation::HGLOBAL,
    pub hDevNames: super::super::super::Foundation::HGLOBAL,
    pub Flags: PAGESETUPDLG_FLAGS,
    pub ptPaperSize: super::super::super::Foundation::POINT,
    pub rtMinMargin: super::super::super::Foundation::RECT,
    pub rtMargin: super::super::super::Foundation::RECT,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnPageSetupHook: LPPAGESETUPHOOK,
    pub lpfnPagePaintHook: LPPAGEPAINTHOOK,
    pub lpPageSetupTemplateName: windows_core::PCSTR,
    pub hPageSetupTemplate: super::super::super::Foundation::HGLOBAL,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct PAGESETUPDLGA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: super::super::super::Foundation::HGLOBAL,
    pub hDevNames: super::super::super::Foundation::HGLOBAL,
    pub Flags: PAGESETUPDLG_FLAGS,
    pub ptPaperSize: super::super::super::Foundation::POINT,
    pub rtMinMargin: super::super::super::Foundation::RECT,
    pub rtMargin: super::super::super::Foundation::RECT,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnPageSetupHook: LPPAGESETUPHOOK,
    pub lpfnPagePaintHook: LPPAGEPAINTHOOK,
    pub lpPageSetupTemplateName: windows_core::PCSTR,
    pub hPageSetupTemplate: super::super::super::Foundation::HGLOBAL,
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct PAGESETUPDLGW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: super::super::super::Foundation::HGLOBAL,
    pub hDevNames: super::super::super::Foundation::HGLOBAL,
    pub Flags: PAGESETUPDLG_FLAGS,
    pub ptPaperSize: super::super::super::Foundation::POINT,
    pub rtMinMargin: super::super::super::Foundation::RECT,
    pub rtMargin: super::super::super::Foundation::RECT,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnPageSetupHook: LPPAGESETUPHOOK,
    pub lpfnPagePaintHook: LPPAGEPAINTHOOK,
    pub lpPageSetupTemplateName: windows_core::PCWSTR,
    pub hPageSetupTemplate: super::super::super::Foundation::HGLOBAL,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct PAGESETUPDLGW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: super::super::super::Foundation::HGLOBAL,
    pub hDevNames: super::super::super::Foundation::HGLOBAL,
    pub Flags: PAGESETUPDLG_FLAGS,
    pub ptPaperSize: super::super::super::Foundation::POINT,
    pub rtMinMargin: super::super::super::Foundation::RECT,
    pub rtMargin: super::super::super::Foundation::RECT,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnPageSetupHook: LPPAGESETUPHOOK,
    pub lpfnPagePaintHook: LPPAGEPAINTHOOK,
    pub lpPageSetupTemplateName: windows_core::PCWSTR,
    pub hPageSetupTemplate: super::super::super::Foundation::HGLOBAL,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PAGESETUPDLG_FLAGS(pub u32);
impl PAGESETUPDLG_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PAGESETUPDLG_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PAGESETUPDLG_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PAGESETUPDLG_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PAGESETUPDLG_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PAGESETUPDLG_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const PDERR_CREATEICFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4106u32);
pub const PDERR_DEFAULTDIFFERENT: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4108u32);
pub const PDERR_DNDMMISMATCH: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4105u32);
pub const PDERR_GETDEVMODEFAIL: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4101u32);
pub const PDERR_INITFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4102u32);
pub const PDERR_LOADDRVFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4100u32);
pub const PDERR_NODEFAULTPRN: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4104u32);
pub const PDERR_NODEVICES: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4103u32);
pub const PDERR_PARSEFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4098u32);
pub const PDERR_PRINTERCODES: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4096u32);
pub const PDERR_PRINTERNOTFOUND: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4107u32);
pub const PDERR_RETDEFFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4099u32);
pub const PDERR_SETUPFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4097u32);
pub const PD_ALLPAGES: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(0u32);
pub const PD_COLLATE: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(16u32);
pub const PD_CURRENTPAGE: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(4194304u32);
pub const PD_DISABLEPRINTTOFILE: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(524288u32);
pub const PD_ENABLEPRINTHOOK: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(4096u32);
pub const PD_ENABLEPRINTTEMPLATE: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(16384u32);
pub const PD_ENABLEPRINTTEMPLATEHANDLE: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(65536u32);
pub const PD_ENABLESETUPHOOK: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(8192u32);
pub const PD_ENABLESETUPTEMPLATE: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(32768u32);
pub const PD_ENABLESETUPTEMPLATEHANDLE: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(131072u32);
pub const PD_EXCLUSIONFLAGS: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(16777216u32);
pub const PD_HIDEPRINTTOFILE: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(1048576u32);
pub const PD_NOCURRENTPAGE: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(8388608u32);
pub const PD_NONETWORKBUTTON: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(2097152u32);
pub const PD_NOPAGENUMS: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(8u32);
pub const PD_NOSELECTION: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(4u32);
pub const PD_NOWARNING: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(128u32);
pub const PD_PAGENUMS: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(2u32);
pub const PD_PRINTSETUP: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(64u32);
pub const PD_PRINTTOFILE: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(32u32);
pub const PD_RESULT_APPLY: u32 = 2u32;
pub const PD_RESULT_CANCEL: u32 = 0u32;
pub const PD_RESULT_PRINT: u32 = 1u32;
pub const PD_RETURNDC: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(256u32);
pub const PD_RETURNDEFAULT: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(1024u32);
pub const PD_RETURNIC: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(512u32);
pub const PD_SELECTION: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(1u32);
pub const PD_SHOWHELP: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(2048u32);
pub const PD_USEDEVMODECOPIES: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(262144u32);
pub const PD_USEDEVMODECOPIESANDCOLLATE: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(262144u32);
pub const PD_USELARGETEMPLATE: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(268435456u32);
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Default)]
pub struct PRINTDLGA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: super::super::super::Foundation::HGLOBAL,
    pub hDevNames: super::super::super::Foundation::HGLOBAL,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub Flags: PRINTDLGEX_FLAGS,
    pub nFromPage: u16,
    pub nToPage: u16,
    pub nMinPage: u16,
    pub nMaxPage: u16,
    pub nCopies: u16,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnPrintHook: LPPRINTHOOKPROC,
    pub lpfnSetupHook: LPSETUPHOOKPROC,
    pub lpPrintTemplateName: windows_core::PCSTR,
    pub lpSetupTemplateName: windows_core::PCSTR,
    pub hPrintTemplate: super::super::super::Foundation::HGLOBAL,
    pub hSetupTemplate: super::super::super::Foundation::HGLOBAL,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, Default)]
pub struct PRINTDLGA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: super::super::super::Foundation::HGLOBAL,
    pub hDevNames: super::super::super::Foundation::HGLOBAL,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub Flags: PRINTDLGEX_FLAGS,
    pub nFromPage: u16,
    pub nToPage: u16,
    pub nMinPage: u16,
    pub nMaxPage: u16,
    pub nCopies: u16,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnPrintHook: LPPRINTHOOKPROC,
    pub lpfnSetupHook: LPSETUPHOOKPROC,
    pub lpPrintTemplateName: windows_core::PCSTR,
    pub lpSetupTemplateName: windows_core::PCSTR,
    pub hPrintTemplate: super::super::super::Foundation::HGLOBAL,
    pub hSetupTemplate: super::super::super::Foundation::HGLOBAL,
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct PRINTDLGEXA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: super::super::super::Foundation::HGLOBAL,
    pub hDevNames: super::super::super::Foundation::HGLOBAL,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub Flags: PRINTDLGEX_FLAGS,
    pub Flags2: u32,
    pub ExclusionFlags: u32,
    pub nPageRanges: u32,
    pub nMaxPageRanges: u32,
    pub lpPageRanges: *mut PRINTPAGERANGE,
    pub nMinPage: u32,
    pub nMaxPage: u32,
    pub nCopies: u32,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpPrintTemplateName: windows_core::PCSTR,
    pub lpCallback: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub nPropertyPages: u32,
    pub lphPropertyPages: *mut super::HPROPSHEETPAGE,
    pub nStartPage: u32,
    pub dwResultAction: u32,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for PRINTDLGEXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Debug, PartialEq)]
pub struct PRINTDLGEXA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: super::super::super::Foundation::HGLOBAL,
    pub hDevNames: super::super::super::Foundation::HGLOBAL,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub Flags: PRINTDLGEX_FLAGS,
    pub Flags2: u32,
    pub ExclusionFlags: u32,
    pub nPageRanges: u32,
    pub nMaxPageRanges: u32,
    pub lpPageRanges: *mut PRINTPAGERANGE,
    pub nMinPage: u32,
    pub nMaxPage: u32,
    pub nCopies: u32,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpPrintTemplateName: windows_core::PCSTR,
    pub lpCallback: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub nPropertyPages: u32,
    pub lphPropertyPages: *mut super::HPROPSHEETPAGE,
    pub nStartPage: u32,
    pub dwResultAction: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for PRINTDLGEXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct PRINTDLGEXW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: super::super::super::Foundation::HGLOBAL,
    pub hDevNames: super::super::super::Foundation::HGLOBAL,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub Flags: PRINTDLGEX_FLAGS,
    pub Flags2: u32,
    pub ExclusionFlags: u32,
    pub nPageRanges: u32,
    pub nMaxPageRanges: u32,
    pub lpPageRanges: *mut PRINTPAGERANGE,
    pub nMinPage: u32,
    pub nMaxPage: u32,
    pub nCopies: u32,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpPrintTemplateName: windows_core::PCWSTR,
    pub lpCallback: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub nPropertyPages: u32,
    pub lphPropertyPages: *mut super::HPROPSHEETPAGE,
    pub nStartPage: u32,
    pub dwResultAction: u32,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for PRINTDLGEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Debug, PartialEq)]
pub struct PRINTDLGEXW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: super::super::super::Foundation::HGLOBAL,
    pub hDevNames: super::super::super::Foundation::HGLOBAL,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub Flags: PRINTDLGEX_FLAGS,
    pub Flags2: u32,
    pub ExclusionFlags: u32,
    pub nPageRanges: u32,
    pub nMaxPageRanges: u32,
    pub lpPageRanges: *mut PRINTPAGERANGE,
    pub nMinPage: u32,
    pub nMaxPage: u32,
    pub nCopies: u32,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpPrintTemplateName: windows_core::PCWSTR,
    pub lpCallback: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub nPropertyPages: u32,
    pub lphPropertyPages: *mut super::HPROPSHEETPAGE,
    pub nStartPage: u32,
    pub dwResultAction: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for PRINTDLGEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PRINTDLGEX_FLAGS(pub u32);
impl PRINTDLGEX_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PRINTDLGEX_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PRINTDLGEX_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PRINTDLGEX_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PRINTDLGEX_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PRINTDLGEX_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Default)]
pub struct PRINTDLGW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: super::super::super::Foundation::HGLOBAL,
    pub hDevNames: super::super::super::Foundation::HGLOBAL,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub Flags: PRINTDLGEX_FLAGS,
    pub nFromPage: u16,
    pub nToPage: u16,
    pub nMinPage: u16,
    pub nMaxPage: u16,
    pub nCopies: u16,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnPrintHook: LPPRINTHOOKPROC,
    pub lpfnSetupHook: LPSETUPHOOKPROC,
    pub lpPrintTemplateName: windows_core::PCWSTR,
    pub lpSetupTemplateName: windows_core::PCWSTR,
    pub hPrintTemplate: super::super::super::Foundation::HGLOBAL,
    pub hSetupTemplate: super::super::super::Foundation::HGLOBAL,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, Default)]
pub struct PRINTDLGW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: super::super::super::Foundation::HGLOBAL,
    pub hDevNames: super::super::super::Foundation::HGLOBAL,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub Flags: PRINTDLGEX_FLAGS,
    pub nFromPage: u16,
    pub nToPage: u16,
    pub nMinPage: u16,
    pub nMaxPage: u16,
    pub nCopies: u16,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnPrintHook: LPPRINTHOOKPROC,
    pub lpfnSetupHook: LPSETUPHOOKPROC,
    pub lpPrintTemplateName: windows_core::PCWSTR,
    pub lpSetupTemplateName: windows_core::PCWSTR,
    pub hPrintTemplate: super::super::super::Foundation::HGLOBAL,
    pub hSetupTemplate: super::super::super::Foundation::HGLOBAL,
}
pub const PRINTER_FONTTYPE: CHOOSEFONT_FONT_TYPE = CHOOSEFONT_FONT_TYPE(16384u16);
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct PRINTPAGERANGE {
    pub nFromPage: u32,
    pub nToPage: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PRINTPAGERANGE {
    pub nFromPage: u32,
    pub nToPage: u32,
}
pub const PSD_DEFAULTMINMARGINS: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(0u32);
pub const PSD_DISABLEMARGINS: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(16u32);
pub const PSD_DISABLEORIENTATION: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(256u32);
pub const PSD_DISABLEPAGEPAINTING: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(524288u32);
pub const PSD_DISABLEPAPER: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(512u32);
pub const PSD_DISABLEPRINTER: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(32u32);
pub const PSD_ENABLEPAGEPAINTHOOK: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(262144u32);
pub const PSD_ENABLEPAGESETUPHOOK: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(8192u32);
pub const PSD_ENABLEPAGESETUPTEMPLATE: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(32768u32);
pub const PSD_ENABLEPAGESETUPTEMPLATEHANDLE: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(131072u32);
pub const PSD_INHUNDREDTHSOFMILLIMETERS: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(8u32);
pub const PSD_INTHOUSANDTHSOFINCHES: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(4u32);
pub const PSD_INWININIINTLMEASURE: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(0u32);
pub const PSD_MARGINS: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(2u32);
pub const PSD_MINMARGINS: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(1u32);
pub const PSD_NONETWORKBUTTON: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(2097152u32);
pub const PSD_NOWARNING: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(128u32);
pub const PSD_RETURNDEFAULT: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(1024u32);
pub const PSD_SHOWHELP: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(2048u32);
pub const PS_OPENTYPE_FONTTYPE: u32 = 65536u32;
pub const REGULAR_FONTTYPE: CHOOSEFONT_FONT_TYPE = CHOOSEFONT_FONT_TYPE(1024u16);
pub const SCREEN_FONTTYPE: CHOOSEFONT_FONT_TYPE = CHOOSEFONT_FONT_TYPE(8192u16);
pub const SETRGBSTRING: windows_core::PCWSTR = windows_core::w!("commdlg_SetRGBColor");
pub const SETRGBSTRINGA: windows_core::PCSTR = windows_core::s!("commdlg_SetRGBColor");
pub const SETRGBSTRINGW: windows_core::PCWSTR = windows_core::w!("commdlg_SetRGBColor");
pub const SHAREVISTRING: windows_core::PCWSTR = windows_core::w!("commdlg_ShareViolation");
pub const SHAREVISTRINGA: windows_core::PCSTR = windows_core::s!("commdlg_ShareViolation");
pub const SHAREVISTRINGW: windows_core::PCWSTR = windows_core::w!("commdlg_ShareViolation");
pub const SIMULATED_FONTTYPE: CHOOSEFONT_FONT_TYPE = CHOOSEFONT_FONT_TYPE(32768u16);
pub const START_PAGE_GENERAL: u32 = 4294967295u32;
pub const SYMBOL_FONTTYPE: u32 = 524288u32;
pub const TT_OPENTYPE_FONTTYPE: u32 = 131072u32;
pub const TYPE1_FONTTYPE: u32 = 262144u32;
pub const WM_CHOOSEFONT_GETLOGFONT: u32 = 1025u32;
pub const WM_CHOOSEFONT_SETFLAGS: u32 = 1126u32;
pub const WM_CHOOSEFONT_SETLOGFONT: u32 = 1125u32;
pub const WM_PSD_ENVSTAMPRECT: u32 = 1029u32;
pub const WM_PSD_FULLPAGERECT: u32 = 1025u32;
pub const WM_PSD_GREEKTEXTRECT: u32 = 1028u32;
pub const WM_PSD_MARGINRECT: u32 = 1027u32;
pub const WM_PSD_MINMARGINRECT: u32 = 1026u32;
pub const WM_PSD_YAFULLPAGERECT: u32 = 1030u32;
