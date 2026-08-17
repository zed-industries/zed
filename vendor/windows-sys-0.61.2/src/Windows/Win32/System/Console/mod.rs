windows_link::link!("kernel32.dll" "system" fn AddConsoleAliasA(source : windows_sys::core::PCSTR, target : windows_sys::core::PCSTR, exename : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn AddConsoleAliasW(source : windows_sys::core::PCWSTR, target : windows_sys::core::PCWSTR, exename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn AllocConsole() -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn AttachConsole(dwprocessid : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn ClosePseudoConsole(hpc : HPCON));
windows_link::link!("user32.dll" "system" fn ConsoleControl(command : CONSOLECONTROL, consoleinformation : *const core::ffi::c_void, consoleinformationlength : u32) -> super::super::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Security")]
windows_link::link!("kernel32.dll" "system" fn CreateConsoleScreenBuffer(dwdesiredaccess : u32, dwsharemode : u32, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, dwflags : u32, lpscreenbufferdata : *const core::ffi::c_void) -> super::super::Foundation:: HANDLE);
windows_link::link!("kernel32.dll" "system" fn CreatePseudoConsole(size : COORD, hinput : super::super::Foundation:: HANDLE, houtput : super::super::Foundation:: HANDLE, dwflags : u32, phpc : *mut HPCON) -> windows_sys::core::HRESULT);
windows_link::link!("kernel32.dll" "system" fn ExpungeConsoleCommandHistoryA(exename : windows_sys::core::PCSTR));
windows_link::link!("kernel32.dll" "system" fn ExpungeConsoleCommandHistoryW(exename : windows_sys::core::PCWSTR));
windows_link::link!("kernel32.dll" "system" fn FillConsoleOutputAttribute(hconsoleoutput : super::super::Foundation:: HANDLE, wattribute : u16, nlength : u32, dwwritecoord : COORD, lpnumberofattrswritten : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn FillConsoleOutputCharacterA(hconsoleoutput : super::super::Foundation:: HANDLE, ccharacter : i8, nlength : u32, dwwritecoord : COORD, lpnumberofcharswritten : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn FillConsoleOutputCharacterW(hconsoleoutput : super::super::Foundation:: HANDLE, ccharacter : u16, nlength : u32, dwwritecoord : COORD, lpnumberofcharswritten : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn FlushConsoleInputBuffer(hconsoleinput : super::super::Foundation:: HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn FreeConsole() -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GenerateConsoleCtrlEvent(dwctrlevent : u32, dwprocessgroupid : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetConsoleAliasA(source : windows_sys::core::PCSTR, targetbuffer : windows_sys::core::PSTR, targetbufferlength : u32, exename : windows_sys::core::PCSTR) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetConsoleAliasExesA(exenamebuffer : windows_sys::core::PSTR, exenamebufferlength : u32) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetConsoleAliasExesLengthA() -> u32);
windows_link::link!("kernel32.dll" "system" fn GetConsoleAliasExesLengthW() -> u32);
windows_link::link!("kernel32.dll" "system" fn GetConsoleAliasExesW(exenamebuffer : windows_sys::core::PWSTR, exenamebufferlength : u32) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetConsoleAliasW(source : windows_sys::core::PCWSTR, targetbuffer : windows_sys::core::PWSTR, targetbufferlength : u32, exename : windows_sys::core::PCWSTR) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetConsoleAliasesA(aliasbuffer : windows_sys::core::PSTR, aliasbufferlength : u32, exename : windows_sys::core::PCSTR) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetConsoleAliasesLengthA(exename : windows_sys::core::PCSTR) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetConsoleAliasesLengthW(exename : windows_sys::core::PCWSTR) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetConsoleAliasesW(aliasbuffer : windows_sys::core::PWSTR, aliasbufferlength : u32, exename : windows_sys::core::PCWSTR) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetConsoleCP() -> u32);
windows_link::link!("kernel32.dll" "system" fn GetConsoleCommandHistoryA(commands : windows_sys::core::PSTR, commandbufferlength : u32, exename : windows_sys::core::PCSTR) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetConsoleCommandHistoryLengthA(exename : windows_sys::core::PCSTR) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetConsoleCommandHistoryLengthW(exename : windows_sys::core::PCWSTR) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetConsoleCommandHistoryW(commands : windows_sys::core::PWSTR, commandbufferlength : u32, exename : windows_sys::core::PCWSTR) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetConsoleCursorInfo(hconsoleoutput : super::super::Foundation:: HANDLE, lpconsolecursorinfo : *mut CONSOLE_CURSOR_INFO) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetConsoleDisplayMode(lpmodeflags : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetConsoleFontSize(hconsoleoutput : super::super::Foundation:: HANDLE, nfont : u32) -> COORD);
windows_link::link!("kernel32.dll" "system" fn GetConsoleHistoryInfo(lpconsolehistoryinfo : *mut CONSOLE_HISTORY_INFO) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetConsoleMode(hconsolehandle : super::super::Foundation:: HANDLE, lpmode : *mut CONSOLE_MODE) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetConsoleOriginalTitleA(lpconsoletitle : windows_sys::core::PSTR, nsize : u32) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetConsoleOriginalTitleW(lpconsoletitle : windows_sys::core::PWSTR, nsize : u32) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetConsoleOutputCP() -> u32);
windows_link::link!("kernel32.dll" "system" fn GetConsoleProcessList(lpdwprocesslist : *mut u32, dwprocesscount : u32) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetConsoleScreenBufferInfo(hconsoleoutput : super::super::Foundation:: HANDLE, lpconsolescreenbufferinfo : *mut CONSOLE_SCREEN_BUFFER_INFO) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetConsoleScreenBufferInfoEx(hconsoleoutput : super::super::Foundation:: HANDLE, lpconsolescreenbufferinfoex : *mut CONSOLE_SCREEN_BUFFER_INFOEX) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetConsoleSelectionInfo(lpconsoleselectioninfo : *mut CONSOLE_SELECTION_INFO) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetConsoleTitleA(lpconsoletitle : windows_sys::core::PSTR, nsize : u32) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetConsoleTitleW(lpconsoletitle : windows_sys::core::PWSTR, nsize : u32) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetConsoleWindow() -> super::super::Foundation:: HWND);
windows_link::link!("kernel32.dll" "system" fn GetCurrentConsoleFont(hconsoleoutput : super::super::Foundation:: HANDLE, bmaximumwindow : windows_sys::core::BOOL, lpconsolecurrentfont : *mut CONSOLE_FONT_INFO) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetCurrentConsoleFontEx(hconsoleoutput : super::super::Foundation:: HANDLE, bmaximumwindow : windows_sys::core::BOOL, lpconsolecurrentfontex : *mut CONSOLE_FONT_INFOEX) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetLargestConsoleWindowSize(hconsoleoutput : super::super::Foundation:: HANDLE) -> COORD);
windows_link::link!("kernel32.dll" "system" fn GetNumberOfConsoleInputEvents(hconsoleinput : super::super::Foundation:: HANDLE, lpnumberofevents : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetNumberOfConsoleMouseButtons(lpnumberofmousebuttons : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetStdHandle(nstdhandle : STD_HANDLE) -> super::super::Foundation:: HANDLE);
windows_link::link!("kernel32.dll" "system" fn PeekConsoleInputA(hconsoleinput : super::super::Foundation:: HANDLE, lpbuffer : *mut INPUT_RECORD, nlength : u32, lpnumberofeventsread : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn PeekConsoleInputW(hconsoleinput : super::super::Foundation:: HANDLE, lpbuffer : *mut INPUT_RECORD, nlength : u32, lpnumberofeventsread : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn ReadConsoleA(hconsoleinput : super::super::Foundation:: HANDLE, lpbuffer : *mut core::ffi::c_void, nnumberofcharstoread : u32, lpnumberofcharsread : *mut u32, pinputcontrol : *const CONSOLE_READCONSOLE_CONTROL) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn ReadConsoleInputA(hconsoleinput : super::super::Foundation:: HANDLE, lpbuffer : *mut INPUT_RECORD, nlength : u32, lpnumberofeventsread : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn ReadConsoleInputW(hconsoleinput : super::super::Foundation:: HANDLE, lpbuffer : *mut INPUT_RECORD, nlength : u32, lpnumberofeventsread : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn ReadConsoleOutputA(hconsoleoutput : super::super::Foundation:: HANDLE, lpbuffer : *mut CHAR_INFO, dwbuffersize : COORD, dwbuffercoord : COORD, lpreadregion : *mut SMALL_RECT) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn ReadConsoleOutputAttribute(hconsoleoutput : super::super::Foundation:: HANDLE, lpattribute : *mut u16, nlength : u32, dwreadcoord : COORD, lpnumberofattrsread : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn ReadConsoleOutputCharacterA(hconsoleoutput : super::super::Foundation:: HANDLE, lpcharacter : windows_sys::core::PSTR, nlength : u32, dwreadcoord : COORD, lpnumberofcharsread : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn ReadConsoleOutputCharacterW(hconsoleoutput : super::super::Foundation:: HANDLE, lpcharacter : windows_sys::core::PWSTR, nlength : u32, dwreadcoord : COORD, lpnumberofcharsread : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn ReadConsoleOutputW(hconsoleoutput : super::super::Foundation:: HANDLE, lpbuffer : *mut CHAR_INFO, dwbuffersize : COORD, dwbuffercoord : COORD, lpreadregion : *mut SMALL_RECT) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn ReadConsoleW(hconsoleinput : super::super::Foundation:: HANDLE, lpbuffer : *mut core::ffi::c_void, nnumberofcharstoread : u32, lpnumberofcharsread : *mut u32, pinputcontrol : *const CONSOLE_READCONSOLE_CONTROL) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn ResizePseudoConsole(hpc : HPCON, size : COORD) -> windows_sys::core::HRESULT);
windows_link::link!("kernel32.dll" "system" fn ScrollConsoleScreenBufferA(hconsoleoutput : super::super::Foundation:: HANDLE, lpscrollrectangle : *const SMALL_RECT, lpcliprectangle : *const SMALL_RECT, dwdestinationorigin : COORD, lpfill : *const CHAR_INFO) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn ScrollConsoleScreenBufferW(hconsoleoutput : super::super::Foundation:: HANDLE, lpscrollrectangle : *const SMALL_RECT, lpcliprectangle : *const SMALL_RECT, dwdestinationorigin : COORD, lpfill : *const CHAR_INFO) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetConsoleActiveScreenBuffer(hconsoleoutput : super::super::Foundation:: HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetConsoleCP(wcodepageid : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetConsoleCtrlHandler(handlerroutine : PHANDLER_ROUTINE, add : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetConsoleCursorInfo(hconsoleoutput : super::super::Foundation:: HANDLE, lpconsolecursorinfo : *const CONSOLE_CURSOR_INFO) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetConsoleCursorPosition(hconsoleoutput : super::super::Foundation:: HANDLE, dwcursorposition : COORD) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetConsoleDisplayMode(hconsoleoutput : super::super::Foundation:: HANDLE, dwflags : u32, lpnewscreenbufferdimensions : *mut COORD) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetConsoleHistoryInfo(lpconsolehistoryinfo : *const CONSOLE_HISTORY_INFO) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetConsoleMode(hconsolehandle : super::super::Foundation:: HANDLE, dwmode : CONSOLE_MODE) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetConsoleNumberOfCommandsA(number : u32, exename : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetConsoleNumberOfCommandsW(number : u32, exename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetConsoleOutputCP(wcodepageid : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetConsoleScreenBufferInfoEx(hconsoleoutput : super::super::Foundation:: HANDLE, lpconsolescreenbufferinfoex : *const CONSOLE_SCREEN_BUFFER_INFOEX) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetConsoleScreenBufferSize(hconsoleoutput : super::super::Foundation:: HANDLE, dwsize : COORD) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetConsoleTextAttribute(hconsoleoutput : super::super::Foundation:: HANDLE, wattributes : CONSOLE_CHARACTER_ATTRIBUTES) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetConsoleTitleA(lpconsoletitle : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetConsoleTitleW(lpconsoletitle : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetConsoleWindowInfo(hconsoleoutput : super::super::Foundation:: HANDLE, babsolute : windows_sys::core::BOOL, lpconsolewindow : *const SMALL_RECT) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetCurrentConsoleFontEx(hconsoleoutput : super::super::Foundation:: HANDLE, bmaximumwindow : windows_sys::core::BOOL, lpconsolecurrentfontex : *const CONSOLE_FONT_INFOEX) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetStdHandle(nstdhandle : STD_HANDLE, hhandle : super::super::Foundation:: HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetStdHandleEx(nstdhandle : STD_HANDLE, hhandle : super::super::Foundation:: HANDLE, phprevvalue : *mut super::super::Foundation:: HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn WriteConsoleA(hconsoleoutput : super::super::Foundation:: HANDLE, lpbuffer : windows_sys::core::PCSTR, nnumberofcharstowrite : u32, lpnumberofcharswritten : *mut u32, lpreserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn WriteConsoleInputA(hconsoleinput : super::super::Foundation:: HANDLE, lpbuffer : *const INPUT_RECORD, nlength : u32, lpnumberofeventswritten : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn WriteConsoleInputW(hconsoleinput : super::super::Foundation:: HANDLE, lpbuffer : *const INPUT_RECORD, nlength : u32, lpnumberofeventswritten : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn WriteConsoleOutputA(hconsoleoutput : super::super::Foundation:: HANDLE, lpbuffer : *const CHAR_INFO, dwbuffersize : COORD, dwbuffercoord : COORD, lpwriteregion : *mut SMALL_RECT) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn WriteConsoleOutputAttribute(hconsoleoutput : super::super::Foundation:: HANDLE, lpattribute : *const u16, nlength : u32, dwwritecoord : COORD, lpnumberofattrswritten : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn WriteConsoleOutputCharacterA(hconsoleoutput : super::super::Foundation:: HANDLE, lpcharacter : windows_sys::core::PCSTR, nlength : u32, dwwritecoord : COORD, lpnumberofcharswritten : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn WriteConsoleOutputCharacterW(hconsoleoutput : super::super::Foundation:: HANDLE, lpcharacter : windows_sys::core::PCWSTR, nlength : u32, dwwritecoord : COORD, lpnumberofcharswritten : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn WriteConsoleOutputW(hconsoleoutput : super::super::Foundation:: HANDLE, lpbuffer : *const CHAR_INFO, dwbuffersize : COORD, dwbuffercoord : COORD, lpwriteregion : *mut SMALL_RECT) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn WriteConsoleW(hconsoleoutput : super::super::Foundation:: HANDLE, lpbuffer : windows_sys::core::PCWSTR, nnumberofcharstowrite : u32, lpnumberofcharswritten : *mut u32, lpreserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
pub const ALTNUMPAD_BIT: u32 = 67108864u32;
pub const ATTACH_PARENT_PROCESS: u32 = 4294967295u32;
pub const BACKGROUND_BLUE: CONSOLE_CHARACTER_ATTRIBUTES = 16u16;
pub const BACKGROUND_GREEN: CONSOLE_CHARACTER_ATTRIBUTES = 32u16;
pub const BACKGROUND_INTENSITY: CONSOLE_CHARACTER_ATTRIBUTES = 128u16;
pub const BACKGROUND_RED: CONSOLE_CHARACTER_ATTRIBUTES = 64u16;
pub const CAPSLOCK_ON: u32 = 128u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CHAR_INFO {
    pub Char: CHAR_INFO_0,
    pub Attributes: u16,
}
impl Default for CHAR_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CHAR_INFO_0 {
    pub UnicodeChar: u16,
    pub AsciiChar: i8,
}
impl Default for CHAR_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const COMMON_LVB_GRID_HORIZONTAL: CONSOLE_CHARACTER_ATTRIBUTES = 1024u16;
pub const COMMON_LVB_GRID_LVERTICAL: CONSOLE_CHARACTER_ATTRIBUTES = 2048u16;
pub const COMMON_LVB_GRID_RVERTICAL: CONSOLE_CHARACTER_ATTRIBUTES = 4096u16;
pub const COMMON_LVB_LEADING_BYTE: CONSOLE_CHARACTER_ATTRIBUTES = 256u16;
pub const COMMON_LVB_REVERSE_VIDEO: CONSOLE_CHARACTER_ATTRIBUTES = 16384u16;
pub const COMMON_LVB_SBCSDBCS: CONSOLE_CHARACTER_ATTRIBUTES = 768u16;
pub const COMMON_LVB_TRAILING_BYTE: CONSOLE_CHARACTER_ATTRIBUTES = 512u16;
pub const COMMON_LVB_UNDERSCORE: CONSOLE_CHARACTER_ATTRIBUTES = 32768u16;
pub type CONSOLECONTROL = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CONSOLEENDTASK {
    pub ProcessId: super::super::Foundation::HANDLE,
    pub hwnd: super::super::Foundation::HWND,
    pub ConsoleEventCode: u32,
    pub ConsoleFlags: u32,
}
impl Default for CONSOLEENDTASK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CONSOLESETFOREGROUND {
    pub hProcess: super::super::Foundation::HANDLE,
    pub bForeground: windows_sys::core::BOOL,
}
impl Default for CONSOLESETFOREGROUND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CONSOLEWINDOWOWNER {
    pub hwnd: super::super::Foundation::HWND,
    pub ProcessId: u32,
    pub ThreadId: u32,
}
impl Default for CONSOLEWINDOWOWNER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CONSOLE_CARET_INFO {
    pub hwnd: super::super::Foundation::HWND,
    pub rc: super::super::Foundation::RECT,
}
impl Default for CONSOLE_CARET_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CONSOLE_CHARACTER_ATTRIBUTES = u16;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CONSOLE_CURSOR_INFO {
    pub dwSize: u32,
    pub bVisible: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CONSOLE_FONT_INFO {
    pub nFont: u32,
    pub dwFontSize: COORD,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CONSOLE_FONT_INFOEX {
    pub cbSize: u32,
    pub nFont: u32,
    pub dwFontSize: COORD,
    pub FontFamily: u32,
    pub FontWeight: u32,
    pub FaceName: [u16; 32],
}
impl Default for CONSOLE_FONT_INFOEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CONSOLE_FULLSCREEN: u32 = 1u32;
pub const CONSOLE_FULLSCREEN_HARDWARE: u32 = 2u32;
pub const CONSOLE_FULLSCREEN_MODE: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CONSOLE_HISTORY_INFO {
    pub cbSize: u32,
    pub HistoryBufferSize: u32,
    pub NumberOfHistoryBuffers: u32,
    pub dwFlags: u32,
}
pub type CONSOLE_MODE = u32;
pub const CONSOLE_MOUSE_DOWN: u32 = 8u32;
pub const CONSOLE_MOUSE_SELECTION: u32 = 4u32;
pub const CONSOLE_NO_SELECTION: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CONSOLE_PROCESS_INFO {
    pub dwProcessID: u32,
    pub dwFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CONSOLE_READCONSOLE_CONTROL {
    pub nLength: u32,
    pub nInitialChars: u32,
    pub dwCtrlWakeupMask: u32,
    pub dwControlKeyState: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CONSOLE_SCREEN_BUFFER_INFO {
    pub dwSize: COORD,
    pub dwCursorPosition: COORD,
    pub wAttributes: CONSOLE_CHARACTER_ATTRIBUTES,
    pub srWindow: SMALL_RECT,
    pub dwMaximumWindowSize: COORD,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CONSOLE_SCREEN_BUFFER_INFOEX {
    pub cbSize: u32,
    pub dwSize: COORD,
    pub dwCursorPosition: COORD,
    pub wAttributes: CONSOLE_CHARACTER_ATTRIBUTES,
    pub srWindow: SMALL_RECT,
    pub dwMaximumWindowSize: COORD,
    pub wPopupAttributes: u16,
    pub bFullscreenSupported: windows_sys::core::BOOL,
    pub ColorTable: [super::super::Foundation::COLORREF; 16],
}
impl Default for CONSOLE_SCREEN_BUFFER_INFOEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CONSOLE_SELECTION_INFO {
    pub dwFlags: u32,
    pub dwSelectionAnchor: COORD,
    pub srSelection: SMALL_RECT,
}
pub const CONSOLE_SELECTION_IN_PROGRESS: u32 = 1u32;
pub const CONSOLE_SELECTION_NOT_EMPTY: u32 = 2u32;
pub const CONSOLE_TEXTMODE_BUFFER: u32 = 1u32;
pub const CONSOLE_WINDOWED_MODE: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct COORD {
    pub X: i16,
    pub Y: i16,
}
pub const CTRL_BREAK_EVENT: u32 = 1u32;
pub const CTRL_CLOSE_EVENT: u32 = 2u32;
pub const CTRL_C_EVENT: u32 = 0u32;
pub const CTRL_LOGOFF_EVENT: u32 = 5u32;
pub const CTRL_SHUTDOWN_EVENT: u32 = 6u32;
pub const ConsoleEndTask: CONSOLECONTROL = 7i32;
pub const ConsoleNotifyConsoleApplication: CONSOLECONTROL = 1i32;
pub const ConsoleSetCaretInfo: CONSOLECONTROL = 3i32;
pub const ConsoleSetForeground: CONSOLECONTROL = 5i32;
pub const ConsoleSetWindowOwner: CONSOLECONTROL = 6i32;
pub const DISABLE_NEWLINE_AUTO_RETURN: CONSOLE_MODE = 8u32;
pub const DOUBLE_CLICK: u32 = 2u32;
pub const ENABLE_AUTO_POSITION: CONSOLE_MODE = 256u32;
pub const ENABLE_ECHO_INPUT: CONSOLE_MODE = 4u32;
pub const ENABLE_EXTENDED_FLAGS: CONSOLE_MODE = 128u32;
pub const ENABLE_INSERT_MODE: CONSOLE_MODE = 32u32;
pub const ENABLE_LINE_INPUT: CONSOLE_MODE = 2u32;
pub const ENABLE_LVB_GRID_WORLDWIDE: CONSOLE_MODE = 16u32;
pub const ENABLE_MOUSE_INPUT: CONSOLE_MODE = 16u32;
pub const ENABLE_PROCESSED_INPUT: CONSOLE_MODE = 1u32;
pub const ENABLE_PROCESSED_OUTPUT: CONSOLE_MODE = 1u32;
pub const ENABLE_QUICK_EDIT_MODE: CONSOLE_MODE = 64u32;
pub const ENABLE_VIRTUAL_TERMINAL_INPUT: CONSOLE_MODE = 512u32;
pub const ENABLE_VIRTUAL_TERMINAL_PROCESSING: CONSOLE_MODE = 4u32;
pub const ENABLE_WINDOW_INPUT: CONSOLE_MODE = 8u32;
pub const ENABLE_WRAP_AT_EOL_OUTPUT: CONSOLE_MODE = 2u32;
pub const ENHANCED_KEY: u32 = 256u32;
pub const FOCUS_EVENT: u32 = 16u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FOCUS_EVENT_RECORD {
    pub bSetFocus: windows_sys::core::BOOL,
}
pub const FOREGROUND_BLUE: CONSOLE_CHARACTER_ATTRIBUTES = 1u16;
pub const FOREGROUND_GREEN: CONSOLE_CHARACTER_ATTRIBUTES = 2u16;
pub const FOREGROUND_INTENSITY: CONSOLE_CHARACTER_ATTRIBUTES = 8u16;
pub const FOREGROUND_RED: CONSOLE_CHARACTER_ATTRIBUTES = 4u16;
pub const FROM_LEFT_1ST_BUTTON_PRESSED: u32 = 1u32;
pub const FROM_LEFT_2ND_BUTTON_PRESSED: u32 = 4u32;
pub const FROM_LEFT_3RD_BUTTON_PRESSED: u32 = 8u32;
pub const FROM_LEFT_4TH_BUTTON_PRESSED: u32 = 16u32;
pub const HISTORY_NO_DUP_FLAG: u32 = 1u32;
pub type HPCON = isize;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct INPUT_RECORD {
    pub EventType: u16,
    pub Event: INPUT_RECORD_0,
}
impl Default for INPUT_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union INPUT_RECORD_0 {
    pub KeyEvent: KEY_EVENT_RECORD,
    pub MouseEvent: MOUSE_EVENT_RECORD,
    pub WindowBufferSizeEvent: WINDOW_BUFFER_SIZE_RECORD,
    pub MenuEvent: MENU_EVENT_RECORD,
    pub FocusEvent: FOCUS_EVENT_RECORD,
}
impl Default for INPUT_RECORD_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KEY_EVENT: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KEY_EVENT_RECORD {
    pub bKeyDown: windows_sys::core::BOOL,
    pub wRepeatCount: u16,
    pub wVirtualKeyCode: u16,
    pub wVirtualScanCode: u16,
    pub uChar: KEY_EVENT_RECORD_0,
    pub dwControlKeyState: u32,
}
impl Default for KEY_EVENT_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union KEY_EVENT_RECORD_0 {
    pub UnicodeChar: u16,
    pub AsciiChar: i8,
}
impl Default for KEY_EVENT_RECORD_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const LEFT_ALT_PRESSED: u32 = 2u32;
pub const LEFT_CTRL_PRESSED: u32 = 8u32;
pub const MENU_EVENT: u32 = 8u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MENU_EVENT_RECORD {
    pub dwCommandId: u32,
}
pub const MOUSE_EVENT: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MOUSE_EVENT_RECORD {
    pub dwMousePosition: COORD,
    pub dwButtonState: u32,
    pub dwControlKeyState: u32,
    pub dwEventFlags: u32,
}
pub const MOUSE_HWHEELED: u32 = 8u32;
pub const MOUSE_MOVED: u32 = 1u32;
pub const MOUSE_WHEELED: u32 = 4u32;
pub const NLS_ALPHANUMERIC: u32 = 0u32;
pub const NLS_DBCSCHAR: u32 = 65536u32;
pub const NLS_HIRAGANA: u32 = 262144u32;
pub const NLS_IME_CONVERSION: u32 = 8388608u32;
pub const NLS_IME_DISABLE: u32 = 536870912u32;
pub const NLS_KATAKANA: u32 = 131072u32;
pub const NLS_ROMAN: u32 = 4194304u32;
pub const NUMLOCK_ON: u32 = 32u32;
pub type PHANDLER_ROUTINE = Option<unsafe extern "system" fn(ctrltype: u32) -> windows_sys::core::BOOL>;
pub const PSEUDOCONSOLE_INHERIT_CURSOR: u32 = 1u32;
pub const RIGHTMOST_BUTTON_PRESSED: u32 = 2u32;
pub const RIGHT_ALT_PRESSED: u32 = 1u32;
pub const RIGHT_CTRL_PRESSED: u32 = 4u32;
pub const Reserved1: CONSOLECONTROL = 0i32;
pub const Reserved2: CONSOLECONTROL = 2i32;
pub const Reserved3: CONSOLECONTROL = 4i32;
pub const SCROLLLOCK_ON: u32 = 64u32;
pub const SHIFT_PRESSED: u32 = 16u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SMALL_RECT {
    pub Left: i16,
    pub Top: i16,
    pub Right: i16,
    pub Bottom: i16,
}
pub const STD_ERROR_HANDLE: STD_HANDLE = 4294967284u32;
pub type STD_HANDLE = u32;
pub const STD_INPUT_HANDLE: STD_HANDLE = 4294967286u32;
pub const STD_OUTPUT_HANDLE: STD_HANDLE = 4294967285u32;
pub const WINDOW_BUFFER_SIZE_EVENT: u32 = 4u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WINDOW_BUFFER_SIZE_RECORD {
    pub dwSize: COORD,
}
