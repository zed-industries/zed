#[inline]
pub unsafe fn AddConsoleAliasA<P0, P1, P2>(source: P0, target: P1, exename: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn AddConsoleAliasA(source : windows_core::PCSTR, target : windows_core::PCSTR, exename : windows_core::PCSTR) -> super::super::Foundation:: BOOL);
    AddConsoleAliasA(source.param().abi(), target.param().abi(), exename.param().abi()).ok()
}
#[inline]
pub unsafe fn AddConsoleAliasW<P0, P1, P2>(source: P0, target: P1, exename: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn AddConsoleAliasW(source : windows_core::PCWSTR, target : windows_core::PCWSTR, exename : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    AddConsoleAliasW(source.param().abi(), target.param().abi(), exename.param().abi()).ok()
}
#[inline]
pub unsafe fn AllocConsole() -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn AllocConsole() -> super::super::Foundation:: BOOL);
    AllocConsole().ok()
}
#[inline]
pub unsafe fn AttachConsole(dwprocessid: u32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn AttachConsole(dwprocessid : u32) -> super::super::Foundation:: BOOL);
    AttachConsole(dwprocessid).ok()
}
#[inline]
pub unsafe fn ClosePseudoConsole<P0>(hpc: P0)
where
    P0: windows_core::Param<HPCON>,
{
    windows_targets::link!("kernel32.dll" "system" fn ClosePseudoConsole(hpc : HPCON));
    ClosePseudoConsole(hpc.param().abi())
}
#[inline]
pub unsafe fn ConsoleControl(command: CONSOLECONTROL, consoleinformation: *const core::ffi::c_void, consoleinformationlength: u32) -> super::super::Foundation::NTSTATUS {
    windows_targets::link!("user32.dll" "system" fn ConsoleControl(command : CONSOLECONTROL, consoleinformation : *const core::ffi::c_void, consoleinformationlength : u32) -> super::super::Foundation:: NTSTATUS);
    ConsoleControl(command, consoleinformation, consoleinformationlength)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateConsoleScreenBuffer(dwdesiredaccess: u32, dwsharemode: u32, lpsecurityattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, dwflags: u32, lpscreenbufferdata: Option<*const core::ffi::c_void>) -> windows_core::Result<super::super::Foundation::HANDLE> {
    windows_targets::link!("kernel32.dll" "system" fn CreateConsoleScreenBuffer(dwdesiredaccess : u32, dwsharemode : u32, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, dwflags : u32, lpscreenbufferdata : *const core::ffi::c_void) -> super::super::Foundation:: HANDLE);
    let result__ = CreateConsoleScreenBuffer(dwdesiredaccess, dwsharemode, core::mem::transmute(lpsecurityattributes.unwrap_or(std::ptr::null())), dwflags, core::mem::transmute(lpscreenbufferdata.unwrap_or(std::ptr::null())));
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn CreatePseudoConsole<P0, P1>(size: COORD, hinput: P0, houtput: P1, dwflags: u32) -> windows_core::Result<HPCON>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreatePseudoConsole(size : COORD, hinput : super::super::Foundation:: HANDLE, houtput : super::super::Foundation:: HANDLE, dwflags : u32, phpc : *mut HPCON) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CreatePseudoConsole(core::mem::transmute(size), hinput.param().abi(), houtput.param().abi(), dwflags, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn ExpungeConsoleCommandHistoryA<P0>(exename: P0)
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn ExpungeConsoleCommandHistoryA(exename : windows_core::PCSTR));
    ExpungeConsoleCommandHistoryA(exename.param().abi())
}
#[inline]
pub unsafe fn ExpungeConsoleCommandHistoryW<P0>(exename: P0)
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn ExpungeConsoleCommandHistoryW(exename : windows_core::PCWSTR));
    ExpungeConsoleCommandHistoryW(exename.param().abi())
}
#[inline]
pub unsafe fn FillConsoleOutputAttribute<P0>(hconsoleoutput: P0, wattribute: u16, nlength: u32, dwwritecoord: COORD, lpnumberofattrswritten: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn FillConsoleOutputAttribute(hconsoleoutput : super::super::Foundation:: HANDLE, wattribute : u16, nlength : u32, dwwritecoord : COORD, lpnumberofattrswritten : *mut u32) -> super::super::Foundation:: BOOL);
    FillConsoleOutputAttribute(hconsoleoutput.param().abi(), wattribute, nlength, core::mem::transmute(dwwritecoord), lpnumberofattrswritten).ok()
}
#[inline]
pub unsafe fn FillConsoleOutputCharacterA<P0>(hconsoleoutput: P0, ccharacter: i8, nlength: u32, dwwritecoord: COORD, lpnumberofcharswritten: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn FillConsoleOutputCharacterA(hconsoleoutput : super::super::Foundation:: HANDLE, ccharacter : i8, nlength : u32, dwwritecoord : COORD, lpnumberofcharswritten : *mut u32) -> super::super::Foundation:: BOOL);
    FillConsoleOutputCharacterA(hconsoleoutput.param().abi(), ccharacter, nlength, core::mem::transmute(dwwritecoord), lpnumberofcharswritten).ok()
}
#[inline]
pub unsafe fn FillConsoleOutputCharacterW<P0>(hconsoleoutput: P0, ccharacter: u16, nlength: u32, dwwritecoord: COORD, lpnumberofcharswritten: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn FillConsoleOutputCharacterW(hconsoleoutput : super::super::Foundation:: HANDLE, ccharacter : u16, nlength : u32, dwwritecoord : COORD, lpnumberofcharswritten : *mut u32) -> super::super::Foundation:: BOOL);
    FillConsoleOutputCharacterW(hconsoleoutput.param().abi(), ccharacter, nlength, core::mem::transmute(dwwritecoord), lpnumberofcharswritten).ok()
}
#[inline]
pub unsafe fn FlushConsoleInputBuffer<P0>(hconsoleinput: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn FlushConsoleInputBuffer(hconsoleinput : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    FlushConsoleInputBuffer(hconsoleinput.param().abi()).ok()
}
#[inline]
pub unsafe fn FreeConsole() -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn FreeConsole() -> super::super::Foundation:: BOOL);
    FreeConsole().ok()
}
#[inline]
pub unsafe fn GenerateConsoleCtrlEvent(dwctrlevent: u32, dwprocessgroupid: u32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn GenerateConsoleCtrlEvent(dwctrlevent : u32, dwprocessgroupid : u32) -> super::super::Foundation:: BOOL);
    GenerateConsoleCtrlEvent(dwctrlevent, dwprocessgroupid).ok()
}
#[inline]
pub unsafe fn GetConsoleAliasA<P0, P1>(source: P0, targetbuffer: &mut [u8], exename: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetConsoleAliasA(source : windows_core::PCSTR, targetbuffer : windows_core::PSTR, targetbufferlength : u32, exename : windows_core::PCSTR) -> u32);
    GetConsoleAliasA(source.param().abi(), core::mem::transmute(targetbuffer.as_ptr()), targetbuffer.len().try_into().unwrap(), exename.param().abi())
}
#[inline]
pub unsafe fn GetConsoleAliasExesA(exenamebuffer: &mut [u8]) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn GetConsoleAliasExesA(exenamebuffer : windows_core::PSTR, exenamebufferlength : u32) -> u32);
    GetConsoleAliasExesA(core::mem::transmute(exenamebuffer.as_ptr()), exenamebuffer.len().try_into().unwrap())
}
#[inline]
pub unsafe fn GetConsoleAliasExesLengthA() -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn GetConsoleAliasExesLengthA() -> u32);
    GetConsoleAliasExesLengthA()
}
#[inline]
pub unsafe fn GetConsoleAliasExesLengthW() -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn GetConsoleAliasExesLengthW() -> u32);
    GetConsoleAliasExesLengthW()
}
#[inline]
pub unsafe fn GetConsoleAliasExesW(exenamebuffer: &mut [u16]) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn GetConsoleAliasExesW(exenamebuffer : windows_core::PWSTR, exenamebufferlength : u32) -> u32);
    GetConsoleAliasExesW(core::mem::transmute(exenamebuffer.as_ptr()), exenamebuffer.len().try_into().unwrap())
}
#[inline]
pub unsafe fn GetConsoleAliasW<P0, P1>(source: P0, targetbuffer: &mut [u16], exename: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetConsoleAliasW(source : windows_core::PCWSTR, targetbuffer : windows_core::PWSTR, targetbufferlength : u32, exename : windows_core::PCWSTR) -> u32);
    GetConsoleAliasW(source.param().abi(), core::mem::transmute(targetbuffer.as_ptr()), targetbuffer.len().try_into().unwrap(), exename.param().abi())
}
#[inline]
pub unsafe fn GetConsoleAliasesA<P0>(aliasbuffer: &mut [u8], exename: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetConsoleAliasesA(aliasbuffer : windows_core::PSTR, aliasbufferlength : u32, exename : windows_core::PCSTR) -> u32);
    GetConsoleAliasesA(core::mem::transmute(aliasbuffer.as_ptr()), aliasbuffer.len().try_into().unwrap(), exename.param().abi())
}
#[inline]
pub unsafe fn GetConsoleAliasesLengthA<P0>(exename: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetConsoleAliasesLengthA(exename : windows_core::PCSTR) -> u32);
    GetConsoleAliasesLengthA(exename.param().abi())
}
#[inline]
pub unsafe fn GetConsoleAliasesLengthW<P0>(exename: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetConsoleAliasesLengthW(exename : windows_core::PCWSTR) -> u32);
    GetConsoleAliasesLengthW(exename.param().abi())
}
#[inline]
pub unsafe fn GetConsoleAliasesW<P0>(aliasbuffer: &mut [u16], exename: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetConsoleAliasesW(aliasbuffer : windows_core::PWSTR, aliasbufferlength : u32, exename : windows_core::PCWSTR) -> u32);
    GetConsoleAliasesW(core::mem::transmute(aliasbuffer.as_ptr()), aliasbuffer.len().try_into().unwrap(), exename.param().abi())
}
#[inline]
pub unsafe fn GetConsoleCP() -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn GetConsoleCP() -> u32);
    GetConsoleCP()
}
#[inline]
pub unsafe fn GetConsoleCommandHistoryA<P0>(commands: &mut [u8], exename: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetConsoleCommandHistoryA(commands : windows_core::PSTR, commandbufferlength : u32, exename : windows_core::PCSTR) -> u32);
    GetConsoleCommandHistoryA(core::mem::transmute(commands.as_ptr()), commands.len().try_into().unwrap(), exename.param().abi())
}
#[inline]
pub unsafe fn GetConsoleCommandHistoryLengthA<P0>(exename: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetConsoleCommandHistoryLengthA(exename : windows_core::PCSTR) -> u32);
    GetConsoleCommandHistoryLengthA(exename.param().abi())
}
#[inline]
pub unsafe fn GetConsoleCommandHistoryLengthW<P0>(exename: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetConsoleCommandHistoryLengthW(exename : windows_core::PCWSTR) -> u32);
    GetConsoleCommandHistoryLengthW(exename.param().abi())
}
#[inline]
pub unsafe fn GetConsoleCommandHistoryW<P0>(commands: windows_core::PWSTR, commandbufferlength: u32, exename: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetConsoleCommandHistoryW(commands : windows_core::PWSTR, commandbufferlength : u32, exename : windows_core::PCWSTR) -> u32);
    GetConsoleCommandHistoryW(core::mem::transmute(commands), commandbufferlength, exename.param().abi())
}
#[inline]
pub unsafe fn GetConsoleCursorInfo<P0>(hconsoleoutput: P0, lpconsolecursorinfo: *mut CONSOLE_CURSOR_INFO) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetConsoleCursorInfo(hconsoleoutput : super::super::Foundation:: HANDLE, lpconsolecursorinfo : *mut CONSOLE_CURSOR_INFO) -> super::super::Foundation:: BOOL);
    GetConsoleCursorInfo(hconsoleoutput.param().abi(), lpconsolecursorinfo).ok()
}
#[inline]
pub unsafe fn GetConsoleDisplayMode(lpmodeflags: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn GetConsoleDisplayMode(lpmodeflags : *mut u32) -> super::super::Foundation:: BOOL);
    GetConsoleDisplayMode(lpmodeflags).ok()
}
#[inline]
pub unsafe fn GetConsoleFontSize<P0>(hconsoleoutput: P0, nfont: u32) -> COORD
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetConsoleFontSize(hconsoleoutput : super::super::Foundation:: HANDLE, nfont : u32) -> COORD);
    GetConsoleFontSize(hconsoleoutput.param().abi(), nfont)
}
#[inline]
pub unsafe fn GetConsoleHistoryInfo(lpconsolehistoryinfo: *mut CONSOLE_HISTORY_INFO) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn GetConsoleHistoryInfo(lpconsolehistoryinfo : *mut CONSOLE_HISTORY_INFO) -> super::super::Foundation:: BOOL);
    GetConsoleHistoryInfo(lpconsolehistoryinfo).ok()
}
#[inline]
pub unsafe fn GetConsoleMode<P0>(hconsolehandle: P0, lpmode: *mut CONSOLE_MODE) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetConsoleMode(hconsolehandle : super::super::Foundation:: HANDLE, lpmode : *mut CONSOLE_MODE) -> super::super::Foundation:: BOOL);
    GetConsoleMode(hconsolehandle.param().abi(), lpmode).ok()
}
#[inline]
pub unsafe fn GetConsoleOriginalTitleA(lpconsoletitle: &mut [u8]) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn GetConsoleOriginalTitleA(lpconsoletitle : windows_core::PSTR, nsize : u32) -> u32);
    GetConsoleOriginalTitleA(core::mem::transmute(lpconsoletitle.as_ptr()), lpconsoletitle.len().try_into().unwrap())
}
#[inline]
pub unsafe fn GetConsoleOriginalTitleW(lpconsoletitle: &mut [u16]) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn GetConsoleOriginalTitleW(lpconsoletitle : windows_core::PWSTR, nsize : u32) -> u32);
    GetConsoleOriginalTitleW(core::mem::transmute(lpconsoletitle.as_ptr()), lpconsoletitle.len().try_into().unwrap())
}
#[inline]
pub unsafe fn GetConsoleOutputCP() -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn GetConsoleOutputCP() -> u32);
    GetConsoleOutputCP()
}
#[inline]
pub unsafe fn GetConsoleProcessList(lpdwprocesslist: &mut [u32]) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn GetConsoleProcessList(lpdwprocesslist : *mut u32, dwprocesscount : u32) -> u32);
    GetConsoleProcessList(core::mem::transmute(lpdwprocesslist.as_ptr()), lpdwprocesslist.len().try_into().unwrap())
}
#[inline]
pub unsafe fn GetConsoleScreenBufferInfo<P0>(hconsoleoutput: P0, lpconsolescreenbufferinfo: *mut CONSOLE_SCREEN_BUFFER_INFO) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetConsoleScreenBufferInfo(hconsoleoutput : super::super::Foundation:: HANDLE, lpconsolescreenbufferinfo : *mut CONSOLE_SCREEN_BUFFER_INFO) -> super::super::Foundation:: BOOL);
    GetConsoleScreenBufferInfo(hconsoleoutput.param().abi(), lpconsolescreenbufferinfo).ok()
}
#[inline]
pub unsafe fn GetConsoleScreenBufferInfoEx<P0>(hconsoleoutput: P0, lpconsolescreenbufferinfoex: *mut CONSOLE_SCREEN_BUFFER_INFOEX) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetConsoleScreenBufferInfoEx(hconsoleoutput : super::super::Foundation:: HANDLE, lpconsolescreenbufferinfoex : *mut CONSOLE_SCREEN_BUFFER_INFOEX) -> super::super::Foundation:: BOOL);
    GetConsoleScreenBufferInfoEx(hconsoleoutput.param().abi(), lpconsolescreenbufferinfoex).ok()
}
#[inline]
pub unsafe fn GetConsoleSelectionInfo(lpconsoleselectioninfo: *mut CONSOLE_SELECTION_INFO) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn GetConsoleSelectionInfo(lpconsoleselectioninfo : *mut CONSOLE_SELECTION_INFO) -> super::super::Foundation:: BOOL);
    GetConsoleSelectionInfo(lpconsoleselectioninfo).ok()
}
#[inline]
pub unsafe fn GetConsoleTitleA(lpconsoletitle: &mut [u8]) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn GetConsoleTitleA(lpconsoletitle : windows_core::PSTR, nsize : u32) -> u32);
    GetConsoleTitleA(core::mem::transmute(lpconsoletitle.as_ptr()), lpconsoletitle.len().try_into().unwrap())
}
#[inline]
pub unsafe fn GetConsoleTitleW(lpconsoletitle: &mut [u16]) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn GetConsoleTitleW(lpconsoletitle : windows_core::PWSTR, nsize : u32) -> u32);
    GetConsoleTitleW(core::mem::transmute(lpconsoletitle.as_ptr()), lpconsoletitle.len().try_into().unwrap())
}
#[inline]
pub unsafe fn GetConsoleWindow() -> super::super::Foundation::HWND {
    windows_targets::link!("kernel32.dll" "system" fn GetConsoleWindow() -> super::super::Foundation:: HWND);
    GetConsoleWindow()
}
#[inline]
pub unsafe fn GetCurrentConsoleFont<P0, P1>(hconsoleoutput: P0, bmaximumwindow: P1, lpconsolecurrentfont: *mut CONSOLE_FONT_INFO) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetCurrentConsoleFont(hconsoleoutput : super::super::Foundation:: HANDLE, bmaximumwindow : super::super::Foundation:: BOOL, lpconsolecurrentfont : *mut CONSOLE_FONT_INFO) -> super::super::Foundation:: BOOL);
    GetCurrentConsoleFont(hconsoleoutput.param().abi(), bmaximumwindow.param().abi(), lpconsolecurrentfont).ok()
}
#[inline]
pub unsafe fn GetCurrentConsoleFontEx<P0, P1>(hconsoleoutput: P0, bmaximumwindow: P1, lpconsolecurrentfontex: *mut CONSOLE_FONT_INFOEX) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetCurrentConsoleFontEx(hconsoleoutput : super::super::Foundation:: HANDLE, bmaximumwindow : super::super::Foundation:: BOOL, lpconsolecurrentfontex : *mut CONSOLE_FONT_INFOEX) -> super::super::Foundation:: BOOL);
    GetCurrentConsoleFontEx(hconsoleoutput.param().abi(), bmaximumwindow.param().abi(), lpconsolecurrentfontex).ok()
}
#[inline]
pub unsafe fn GetLargestConsoleWindowSize<P0>(hconsoleoutput: P0) -> COORD
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetLargestConsoleWindowSize(hconsoleoutput : super::super::Foundation:: HANDLE) -> COORD);
    GetLargestConsoleWindowSize(hconsoleoutput.param().abi())
}
#[inline]
pub unsafe fn GetNumberOfConsoleInputEvents<P0>(hconsoleinput: P0, lpnumberofevents: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetNumberOfConsoleInputEvents(hconsoleinput : super::super::Foundation:: HANDLE, lpnumberofevents : *mut u32) -> super::super::Foundation:: BOOL);
    GetNumberOfConsoleInputEvents(hconsoleinput.param().abi(), lpnumberofevents).ok()
}
#[inline]
pub unsafe fn GetNumberOfConsoleMouseButtons(lpnumberofmousebuttons: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn GetNumberOfConsoleMouseButtons(lpnumberofmousebuttons : *mut u32) -> super::super::Foundation:: BOOL);
    GetNumberOfConsoleMouseButtons(lpnumberofmousebuttons).ok()
}
#[inline]
pub unsafe fn GetStdHandle(nstdhandle: STD_HANDLE) -> windows_core::Result<super::super::Foundation::HANDLE> {
    windows_targets::link!("kernel32.dll" "system" fn GetStdHandle(nstdhandle : STD_HANDLE) -> super::super::Foundation:: HANDLE);
    let result__ = GetStdHandle(nstdhandle);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn PeekConsoleInputA<P0>(hconsoleinput: P0, lpbuffer: &mut [INPUT_RECORD], lpnumberofeventsread: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn PeekConsoleInputA(hconsoleinput : super::super::Foundation:: HANDLE, lpbuffer : *mut INPUT_RECORD, nlength : u32, lpnumberofeventsread : *mut u32) -> super::super::Foundation:: BOOL);
    PeekConsoleInputA(hconsoleinput.param().abi(), core::mem::transmute(lpbuffer.as_ptr()), lpbuffer.len().try_into().unwrap(), lpnumberofeventsread).ok()
}
#[inline]
pub unsafe fn PeekConsoleInputW<P0>(hconsoleinput: P0, lpbuffer: &mut [INPUT_RECORD], lpnumberofeventsread: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn PeekConsoleInputW(hconsoleinput : super::super::Foundation:: HANDLE, lpbuffer : *mut INPUT_RECORD, nlength : u32, lpnumberofeventsread : *mut u32) -> super::super::Foundation:: BOOL);
    PeekConsoleInputW(hconsoleinput.param().abi(), core::mem::transmute(lpbuffer.as_ptr()), lpbuffer.len().try_into().unwrap(), lpnumberofeventsread).ok()
}
#[inline]
pub unsafe fn ReadConsoleA<P0>(hconsoleinput: P0, lpbuffer: *mut core::ffi::c_void, nnumberofcharstoread: u32, lpnumberofcharsread: *mut u32, pinputcontrol: Option<*const CONSOLE_READCONSOLE_CONTROL>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn ReadConsoleA(hconsoleinput : super::super::Foundation:: HANDLE, lpbuffer : *mut core::ffi::c_void, nnumberofcharstoread : u32, lpnumberofcharsread : *mut u32, pinputcontrol : *const CONSOLE_READCONSOLE_CONTROL) -> super::super::Foundation:: BOOL);
    ReadConsoleA(hconsoleinput.param().abi(), lpbuffer, nnumberofcharstoread, lpnumberofcharsread, core::mem::transmute(pinputcontrol.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn ReadConsoleInputA<P0>(hconsoleinput: P0, lpbuffer: &mut [INPUT_RECORD], lpnumberofeventsread: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn ReadConsoleInputA(hconsoleinput : super::super::Foundation:: HANDLE, lpbuffer : *mut INPUT_RECORD, nlength : u32, lpnumberofeventsread : *mut u32) -> super::super::Foundation:: BOOL);
    ReadConsoleInputA(hconsoleinput.param().abi(), core::mem::transmute(lpbuffer.as_ptr()), lpbuffer.len().try_into().unwrap(), lpnumberofeventsread).ok()
}
#[inline]
pub unsafe fn ReadConsoleInputW<P0>(hconsoleinput: P0, lpbuffer: &mut [INPUT_RECORD], lpnumberofeventsread: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn ReadConsoleInputW(hconsoleinput : super::super::Foundation:: HANDLE, lpbuffer : *mut INPUT_RECORD, nlength : u32, lpnumberofeventsread : *mut u32) -> super::super::Foundation:: BOOL);
    ReadConsoleInputW(hconsoleinput.param().abi(), core::mem::transmute(lpbuffer.as_ptr()), lpbuffer.len().try_into().unwrap(), lpnumberofeventsread).ok()
}
#[inline]
pub unsafe fn ReadConsoleOutputA<P0>(hconsoleoutput: P0, lpbuffer: *mut CHAR_INFO, dwbuffersize: COORD, dwbuffercoord: COORD, lpreadregion: *mut SMALL_RECT) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn ReadConsoleOutputA(hconsoleoutput : super::super::Foundation:: HANDLE, lpbuffer : *mut CHAR_INFO, dwbuffersize : COORD, dwbuffercoord : COORD, lpreadregion : *mut SMALL_RECT) -> super::super::Foundation:: BOOL);
    ReadConsoleOutputA(hconsoleoutput.param().abi(), lpbuffer, core::mem::transmute(dwbuffersize), core::mem::transmute(dwbuffercoord), lpreadregion).ok()
}
#[inline]
pub unsafe fn ReadConsoleOutputAttribute<P0>(hconsoleoutput: P0, lpattribute: &mut [u16], dwreadcoord: COORD, lpnumberofattrsread: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn ReadConsoleOutputAttribute(hconsoleoutput : super::super::Foundation:: HANDLE, lpattribute : *mut u16, nlength : u32, dwreadcoord : COORD, lpnumberofattrsread : *mut u32) -> super::super::Foundation:: BOOL);
    ReadConsoleOutputAttribute(hconsoleoutput.param().abi(), core::mem::transmute(lpattribute.as_ptr()), lpattribute.len().try_into().unwrap(), core::mem::transmute(dwreadcoord), lpnumberofattrsread).ok()
}
#[inline]
pub unsafe fn ReadConsoleOutputCharacterA<P0>(hconsoleoutput: P0, lpcharacter: &mut [u8], dwreadcoord: COORD, lpnumberofcharsread: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn ReadConsoleOutputCharacterA(hconsoleoutput : super::super::Foundation:: HANDLE, lpcharacter : windows_core::PSTR, nlength : u32, dwreadcoord : COORD, lpnumberofcharsread : *mut u32) -> super::super::Foundation:: BOOL);
    ReadConsoleOutputCharacterA(hconsoleoutput.param().abi(), core::mem::transmute(lpcharacter.as_ptr()), lpcharacter.len().try_into().unwrap(), core::mem::transmute(dwreadcoord), lpnumberofcharsread).ok()
}
#[inline]
pub unsafe fn ReadConsoleOutputCharacterW<P0>(hconsoleoutput: P0, lpcharacter: &mut [u16], dwreadcoord: COORD, lpnumberofcharsread: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn ReadConsoleOutputCharacterW(hconsoleoutput : super::super::Foundation:: HANDLE, lpcharacter : windows_core::PWSTR, nlength : u32, dwreadcoord : COORD, lpnumberofcharsread : *mut u32) -> super::super::Foundation:: BOOL);
    ReadConsoleOutputCharacterW(hconsoleoutput.param().abi(), core::mem::transmute(lpcharacter.as_ptr()), lpcharacter.len().try_into().unwrap(), core::mem::transmute(dwreadcoord), lpnumberofcharsread).ok()
}
#[inline]
pub unsafe fn ReadConsoleOutputW<P0>(hconsoleoutput: P0, lpbuffer: *mut CHAR_INFO, dwbuffersize: COORD, dwbuffercoord: COORD, lpreadregion: *mut SMALL_RECT) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn ReadConsoleOutputW(hconsoleoutput : super::super::Foundation:: HANDLE, lpbuffer : *mut CHAR_INFO, dwbuffersize : COORD, dwbuffercoord : COORD, lpreadregion : *mut SMALL_RECT) -> super::super::Foundation:: BOOL);
    ReadConsoleOutputW(hconsoleoutput.param().abi(), lpbuffer, core::mem::transmute(dwbuffersize), core::mem::transmute(dwbuffercoord), lpreadregion).ok()
}
#[inline]
pub unsafe fn ReadConsoleW<P0>(hconsoleinput: P0, lpbuffer: *mut core::ffi::c_void, nnumberofcharstoread: u32, lpnumberofcharsread: *mut u32, pinputcontrol: Option<*const CONSOLE_READCONSOLE_CONTROL>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn ReadConsoleW(hconsoleinput : super::super::Foundation:: HANDLE, lpbuffer : *mut core::ffi::c_void, nnumberofcharstoread : u32, lpnumberofcharsread : *mut u32, pinputcontrol : *const CONSOLE_READCONSOLE_CONTROL) -> super::super::Foundation:: BOOL);
    ReadConsoleW(hconsoleinput.param().abi(), lpbuffer, nnumberofcharstoread, lpnumberofcharsread, core::mem::transmute(pinputcontrol.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn ResizePseudoConsole<P0>(hpc: P0, size: COORD) -> windows_core::Result<()>
where
    P0: windows_core::Param<HPCON>,
{
    windows_targets::link!("kernel32.dll" "system" fn ResizePseudoConsole(hpc : HPCON, size : COORD) -> windows_core::HRESULT);
    ResizePseudoConsole(hpc.param().abi(), core::mem::transmute(size)).ok()
}
#[inline]
pub unsafe fn ScrollConsoleScreenBufferA<P0>(hconsoleoutput: P0, lpscrollrectangle: *const SMALL_RECT, lpcliprectangle: Option<*const SMALL_RECT>, dwdestinationorigin: COORD, lpfill: *const CHAR_INFO) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn ScrollConsoleScreenBufferA(hconsoleoutput : super::super::Foundation:: HANDLE, lpscrollrectangle : *const SMALL_RECT, lpcliprectangle : *const SMALL_RECT, dwdestinationorigin : COORD, lpfill : *const CHAR_INFO) -> super::super::Foundation:: BOOL);
    ScrollConsoleScreenBufferA(hconsoleoutput.param().abi(), lpscrollrectangle, core::mem::transmute(lpcliprectangle.unwrap_or(std::ptr::null())), core::mem::transmute(dwdestinationorigin), lpfill).ok()
}
#[inline]
pub unsafe fn ScrollConsoleScreenBufferW<P0>(hconsoleoutput: P0, lpscrollrectangle: *const SMALL_RECT, lpcliprectangle: Option<*const SMALL_RECT>, dwdestinationorigin: COORD, lpfill: *const CHAR_INFO) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn ScrollConsoleScreenBufferW(hconsoleoutput : super::super::Foundation:: HANDLE, lpscrollrectangle : *const SMALL_RECT, lpcliprectangle : *const SMALL_RECT, dwdestinationorigin : COORD, lpfill : *const CHAR_INFO) -> super::super::Foundation:: BOOL);
    ScrollConsoleScreenBufferW(hconsoleoutput.param().abi(), lpscrollrectangle, core::mem::transmute(lpcliprectangle.unwrap_or(std::ptr::null())), core::mem::transmute(dwdestinationorigin), lpfill).ok()
}
#[inline]
pub unsafe fn SetConsoleActiveScreenBuffer<P0>(hconsoleoutput: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetConsoleActiveScreenBuffer(hconsoleoutput : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    SetConsoleActiveScreenBuffer(hconsoleoutput.param().abi()).ok()
}
#[inline]
pub unsafe fn SetConsoleCP(wcodepageid: u32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn SetConsoleCP(wcodepageid : u32) -> super::super::Foundation:: BOOL);
    SetConsoleCP(wcodepageid).ok()
}
#[inline]
pub unsafe fn SetConsoleCtrlHandler<P0>(handlerroutine: PHANDLER_ROUTINE, add: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetConsoleCtrlHandler(handlerroutine : PHANDLER_ROUTINE, add : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    SetConsoleCtrlHandler(handlerroutine, add.param().abi()).ok()
}
#[inline]
pub unsafe fn SetConsoleCursorInfo<P0>(hconsoleoutput: P0, lpconsolecursorinfo: *const CONSOLE_CURSOR_INFO) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetConsoleCursorInfo(hconsoleoutput : super::super::Foundation:: HANDLE, lpconsolecursorinfo : *const CONSOLE_CURSOR_INFO) -> super::super::Foundation:: BOOL);
    SetConsoleCursorInfo(hconsoleoutput.param().abi(), lpconsolecursorinfo).ok()
}
#[inline]
pub unsafe fn SetConsoleCursorPosition<P0>(hconsoleoutput: P0, dwcursorposition: COORD) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetConsoleCursorPosition(hconsoleoutput : super::super::Foundation:: HANDLE, dwcursorposition : COORD) -> super::super::Foundation:: BOOL);
    SetConsoleCursorPosition(hconsoleoutput.param().abi(), core::mem::transmute(dwcursorposition)).ok()
}
#[inline]
pub unsafe fn SetConsoleDisplayMode<P0>(hconsoleoutput: P0, dwflags: u32, lpnewscreenbufferdimensions: Option<*mut COORD>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetConsoleDisplayMode(hconsoleoutput : super::super::Foundation:: HANDLE, dwflags : u32, lpnewscreenbufferdimensions : *mut COORD) -> super::super::Foundation:: BOOL);
    SetConsoleDisplayMode(hconsoleoutput.param().abi(), dwflags, core::mem::transmute(lpnewscreenbufferdimensions.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn SetConsoleHistoryInfo(lpconsolehistoryinfo: *const CONSOLE_HISTORY_INFO) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn SetConsoleHistoryInfo(lpconsolehistoryinfo : *const CONSOLE_HISTORY_INFO) -> super::super::Foundation:: BOOL);
    SetConsoleHistoryInfo(lpconsolehistoryinfo).ok()
}
#[inline]
pub unsafe fn SetConsoleMode<P0>(hconsolehandle: P0, dwmode: CONSOLE_MODE) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetConsoleMode(hconsolehandle : super::super::Foundation:: HANDLE, dwmode : CONSOLE_MODE) -> super::super::Foundation:: BOOL);
    SetConsoleMode(hconsolehandle.param().abi(), dwmode).ok()
}
#[inline]
pub unsafe fn SetConsoleNumberOfCommandsA<P0>(number: u32, exename: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetConsoleNumberOfCommandsA(number : u32, exename : windows_core::PCSTR) -> super::super::Foundation:: BOOL);
    SetConsoleNumberOfCommandsA(number, exename.param().abi())
}
#[inline]
pub unsafe fn SetConsoleNumberOfCommandsW<P0>(number: u32, exename: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetConsoleNumberOfCommandsW(number : u32, exename : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    SetConsoleNumberOfCommandsW(number, exename.param().abi())
}
#[inline]
pub unsafe fn SetConsoleOutputCP(wcodepageid: u32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn SetConsoleOutputCP(wcodepageid : u32) -> super::super::Foundation:: BOOL);
    SetConsoleOutputCP(wcodepageid).ok()
}
#[inline]
pub unsafe fn SetConsoleScreenBufferInfoEx<P0>(hconsoleoutput: P0, lpconsolescreenbufferinfoex: *const CONSOLE_SCREEN_BUFFER_INFOEX) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetConsoleScreenBufferInfoEx(hconsoleoutput : super::super::Foundation:: HANDLE, lpconsolescreenbufferinfoex : *const CONSOLE_SCREEN_BUFFER_INFOEX) -> super::super::Foundation:: BOOL);
    SetConsoleScreenBufferInfoEx(hconsoleoutput.param().abi(), lpconsolescreenbufferinfoex).ok()
}
#[inline]
pub unsafe fn SetConsoleScreenBufferSize<P0>(hconsoleoutput: P0, dwsize: COORD) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetConsoleScreenBufferSize(hconsoleoutput : super::super::Foundation:: HANDLE, dwsize : COORD) -> super::super::Foundation:: BOOL);
    SetConsoleScreenBufferSize(hconsoleoutput.param().abi(), core::mem::transmute(dwsize)).ok()
}
#[inline]
pub unsafe fn SetConsoleTextAttribute<P0>(hconsoleoutput: P0, wattributes: CONSOLE_CHARACTER_ATTRIBUTES) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetConsoleTextAttribute(hconsoleoutput : super::super::Foundation:: HANDLE, wattributes : CONSOLE_CHARACTER_ATTRIBUTES) -> super::super::Foundation:: BOOL);
    SetConsoleTextAttribute(hconsoleoutput.param().abi(), wattributes).ok()
}
#[inline]
pub unsafe fn SetConsoleTitleA<P0>(lpconsoletitle: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetConsoleTitleA(lpconsoletitle : windows_core::PCSTR) -> super::super::Foundation:: BOOL);
    SetConsoleTitleA(lpconsoletitle.param().abi()).ok()
}
#[inline]
pub unsafe fn SetConsoleTitleW<P0>(lpconsoletitle: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetConsoleTitleW(lpconsoletitle : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    SetConsoleTitleW(lpconsoletitle.param().abi()).ok()
}
#[inline]
pub unsafe fn SetConsoleWindowInfo<P0, P1>(hconsoleoutput: P0, babsolute: P1, lpconsolewindow: *const SMALL_RECT) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetConsoleWindowInfo(hconsoleoutput : super::super::Foundation:: HANDLE, babsolute : super::super::Foundation:: BOOL, lpconsolewindow : *const SMALL_RECT) -> super::super::Foundation:: BOOL);
    SetConsoleWindowInfo(hconsoleoutput.param().abi(), babsolute.param().abi(), lpconsolewindow).ok()
}
#[inline]
pub unsafe fn SetCurrentConsoleFontEx<P0, P1>(hconsoleoutput: P0, bmaximumwindow: P1, lpconsolecurrentfontex: *const CONSOLE_FONT_INFOEX) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetCurrentConsoleFontEx(hconsoleoutput : super::super::Foundation:: HANDLE, bmaximumwindow : super::super::Foundation:: BOOL, lpconsolecurrentfontex : *const CONSOLE_FONT_INFOEX) -> super::super::Foundation:: BOOL);
    SetCurrentConsoleFontEx(hconsoleoutput.param().abi(), bmaximumwindow.param().abi(), lpconsolecurrentfontex).ok()
}
#[inline]
pub unsafe fn SetStdHandle<P0>(nstdhandle: STD_HANDLE, hhandle: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetStdHandle(nstdhandle : STD_HANDLE, hhandle : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    SetStdHandle(nstdhandle, hhandle.param().abi()).ok()
}
#[inline]
pub unsafe fn SetStdHandleEx<P0>(nstdhandle: STD_HANDLE, hhandle: P0, phprevvalue: Option<*mut super::super::Foundation::HANDLE>) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetStdHandleEx(nstdhandle : STD_HANDLE, hhandle : super::super::Foundation:: HANDLE, phprevvalue : *mut super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    SetStdHandleEx(nstdhandle, hhandle.param().abi(), core::mem::transmute(phprevvalue.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn WriteConsoleA<P0>(hconsoleoutput: P0, lpbuffer: &[u8], lpnumberofcharswritten: Option<*mut u32>, lpreserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn WriteConsoleA(hconsoleoutput : super::super::Foundation:: HANDLE, lpbuffer : *const core::ffi::c_void, nnumberofcharstowrite : u32, lpnumberofcharswritten : *mut u32, lpreserved : *const core::ffi::c_void) -> super::super::Foundation:: BOOL);
    WriteConsoleA(hconsoleoutput.param().abi(), core::mem::transmute(lpbuffer.as_ptr()), lpbuffer.len().try_into().unwrap(), core::mem::transmute(lpnumberofcharswritten.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpreserved.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn WriteConsoleInputA<P0>(hconsoleinput: P0, lpbuffer: &[INPUT_RECORD], lpnumberofeventswritten: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn WriteConsoleInputA(hconsoleinput : super::super::Foundation:: HANDLE, lpbuffer : *const INPUT_RECORD, nlength : u32, lpnumberofeventswritten : *mut u32) -> super::super::Foundation:: BOOL);
    WriteConsoleInputA(hconsoleinput.param().abi(), core::mem::transmute(lpbuffer.as_ptr()), lpbuffer.len().try_into().unwrap(), lpnumberofeventswritten).ok()
}
#[inline]
pub unsafe fn WriteConsoleInputW<P0>(hconsoleinput: P0, lpbuffer: &[INPUT_RECORD], lpnumberofeventswritten: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn WriteConsoleInputW(hconsoleinput : super::super::Foundation:: HANDLE, lpbuffer : *const INPUT_RECORD, nlength : u32, lpnumberofeventswritten : *mut u32) -> super::super::Foundation:: BOOL);
    WriteConsoleInputW(hconsoleinput.param().abi(), core::mem::transmute(lpbuffer.as_ptr()), lpbuffer.len().try_into().unwrap(), lpnumberofeventswritten).ok()
}
#[inline]
pub unsafe fn WriteConsoleOutputA<P0>(hconsoleoutput: P0, lpbuffer: *const CHAR_INFO, dwbuffersize: COORD, dwbuffercoord: COORD, lpwriteregion: *mut SMALL_RECT) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn WriteConsoleOutputA(hconsoleoutput : super::super::Foundation:: HANDLE, lpbuffer : *const CHAR_INFO, dwbuffersize : COORD, dwbuffercoord : COORD, lpwriteregion : *mut SMALL_RECT) -> super::super::Foundation:: BOOL);
    WriteConsoleOutputA(hconsoleoutput.param().abi(), lpbuffer, core::mem::transmute(dwbuffersize), core::mem::transmute(dwbuffercoord), lpwriteregion).ok()
}
#[inline]
pub unsafe fn WriteConsoleOutputAttribute<P0>(hconsoleoutput: P0, lpattribute: &[u16], dwwritecoord: COORD, lpnumberofattrswritten: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn WriteConsoleOutputAttribute(hconsoleoutput : super::super::Foundation:: HANDLE, lpattribute : *const u16, nlength : u32, dwwritecoord : COORD, lpnumberofattrswritten : *mut u32) -> super::super::Foundation:: BOOL);
    WriteConsoleOutputAttribute(hconsoleoutput.param().abi(), core::mem::transmute(lpattribute.as_ptr()), lpattribute.len().try_into().unwrap(), core::mem::transmute(dwwritecoord), lpnumberofattrswritten).ok()
}
#[inline]
pub unsafe fn WriteConsoleOutputCharacterA<P0>(hconsoleoutput: P0, lpcharacter: &[u8], dwwritecoord: COORD, lpnumberofcharswritten: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn WriteConsoleOutputCharacterA(hconsoleoutput : super::super::Foundation:: HANDLE, lpcharacter : windows_core::PCSTR, nlength : u32, dwwritecoord : COORD, lpnumberofcharswritten : *mut u32) -> super::super::Foundation:: BOOL);
    WriteConsoleOutputCharacterA(hconsoleoutput.param().abi(), core::mem::transmute(lpcharacter.as_ptr()), lpcharacter.len().try_into().unwrap(), core::mem::transmute(dwwritecoord), lpnumberofcharswritten).ok()
}
#[inline]
pub unsafe fn WriteConsoleOutputCharacterW<P0>(hconsoleoutput: P0, lpcharacter: &[u16], dwwritecoord: COORD, lpnumberofcharswritten: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn WriteConsoleOutputCharacterW(hconsoleoutput : super::super::Foundation:: HANDLE, lpcharacter : windows_core::PCWSTR, nlength : u32, dwwritecoord : COORD, lpnumberofcharswritten : *mut u32) -> super::super::Foundation:: BOOL);
    WriteConsoleOutputCharacterW(hconsoleoutput.param().abi(), core::mem::transmute(lpcharacter.as_ptr()), lpcharacter.len().try_into().unwrap(), core::mem::transmute(dwwritecoord), lpnumberofcharswritten).ok()
}
#[inline]
pub unsafe fn WriteConsoleOutputW<P0>(hconsoleoutput: P0, lpbuffer: *const CHAR_INFO, dwbuffersize: COORD, dwbuffercoord: COORD, lpwriteregion: *mut SMALL_RECT) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn WriteConsoleOutputW(hconsoleoutput : super::super::Foundation:: HANDLE, lpbuffer : *const CHAR_INFO, dwbuffersize : COORD, dwbuffercoord : COORD, lpwriteregion : *mut SMALL_RECT) -> super::super::Foundation:: BOOL);
    WriteConsoleOutputW(hconsoleoutput.param().abi(), lpbuffer, core::mem::transmute(dwbuffersize), core::mem::transmute(dwbuffercoord), lpwriteregion).ok()
}
#[inline]
pub unsafe fn WriteConsoleW<P0>(hconsoleoutput: P0, lpbuffer: &[u8], lpnumberofcharswritten: Option<*mut u32>, lpreserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn WriteConsoleW(hconsoleoutput : super::super::Foundation:: HANDLE, lpbuffer : *const core::ffi::c_void, nnumberofcharstowrite : u32, lpnumberofcharswritten : *mut u32, lpreserved : *const core::ffi::c_void) -> super::super::Foundation:: BOOL);
    WriteConsoleW(hconsoleoutput.param().abi(), core::mem::transmute(lpbuffer.as_ptr()), lpbuffer.len().try_into().unwrap(), core::mem::transmute(lpnumberofcharswritten.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpreserved.unwrap_or(std::ptr::null()))).ok()
}
pub const ALTNUMPAD_BIT: u32 = 67108864u32;
pub const ATTACH_PARENT_PROCESS: u32 = 4294967295u32;
pub const BACKGROUND_BLUE: CONSOLE_CHARACTER_ATTRIBUTES = CONSOLE_CHARACTER_ATTRIBUTES(16u16);
pub const BACKGROUND_GREEN: CONSOLE_CHARACTER_ATTRIBUTES = CONSOLE_CHARACTER_ATTRIBUTES(32u16);
pub const BACKGROUND_INTENSITY: CONSOLE_CHARACTER_ATTRIBUTES = CONSOLE_CHARACTER_ATTRIBUTES(128u16);
pub const BACKGROUND_RED: CONSOLE_CHARACTER_ATTRIBUTES = CONSOLE_CHARACTER_ATTRIBUTES(64u16);
pub const CAPSLOCK_ON: u32 = 128u32;
pub const COMMON_LVB_GRID_HORIZONTAL: CONSOLE_CHARACTER_ATTRIBUTES = CONSOLE_CHARACTER_ATTRIBUTES(1024u16);
pub const COMMON_LVB_GRID_LVERTICAL: CONSOLE_CHARACTER_ATTRIBUTES = CONSOLE_CHARACTER_ATTRIBUTES(2048u16);
pub const COMMON_LVB_GRID_RVERTICAL: CONSOLE_CHARACTER_ATTRIBUTES = CONSOLE_CHARACTER_ATTRIBUTES(4096u16);
pub const COMMON_LVB_LEADING_BYTE: CONSOLE_CHARACTER_ATTRIBUTES = CONSOLE_CHARACTER_ATTRIBUTES(256u16);
pub const COMMON_LVB_REVERSE_VIDEO: CONSOLE_CHARACTER_ATTRIBUTES = CONSOLE_CHARACTER_ATTRIBUTES(16384u16);
pub const COMMON_LVB_SBCSDBCS: CONSOLE_CHARACTER_ATTRIBUTES = CONSOLE_CHARACTER_ATTRIBUTES(768u16);
pub const COMMON_LVB_TRAILING_BYTE: CONSOLE_CHARACTER_ATTRIBUTES = CONSOLE_CHARACTER_ATTRIBUTES(512u16);
pub const COMMON_LVB_UNDERSCORE: CONSOLE_CHARACTER_ATTRIBUTES = CONSOLE_CHARACTER_ATTRIBUTES(32768u16);
pub const CONSOLE_FULLSCREEN: u32 = 1u32;
pub const CONSOLE_FULLSCREEN_HARDWARE: u32 = 2u32;
pub const CONSOLE_FULLSCREEN_MODE: u32 = 1u32;
pub const CONSOLE_MOUSE_DOWN: u32 = 8u32;
pub const CONSOLE_MOUSE_SELECTION: u32 = 4u32;
pub const CONSOLE_NO_SELECTION: u32 = 0u32;
pub const CONSOLE_SELECTION_IN_PROGRESS: u32 = 1u32;
pub const CONSOLE_SELECTION_NOT_EMPTY: u32 = 2u32;
pub const CONSOLE_TEXTMODE_BUFFER: u32 = 1u32;
pub const CONSOLE_WINDOWED_MODE: u32 = 2u32;
pub const CTRL_BREAK_EVENT: u32 = 1u32;
pub const CTRL_CLOSE_EVENT: u32 = 2u32;
pub const CTRL_C_EVENT: u32 = 0u32;
pub const CTRL_LOGOFF_EVENT: u32 = 5u32;
pub const CTRL_SHUTDOWN_EVENT: u32 = 6u32;
pub const ConsoleEndTask: CONSOLECONTROL = CONSOLECONTROL(7i32);
pub const ConsoleNotifyConsoleApplication: CONSOLECONTROL = CONSOLECONTROL(1i32);
pub const ConsoleSetCaretInfo: CONSOLECONTROL = CONSOLECONTROL(3i32);
pub const ConsoleSetForeground: CONSOLECONTROL = CONSOLECONTROL(5i32);
pub const ConsoleSetWindowOwner: CONSOLECONTROL = CONSOLECONTROL(6i32);
pub const DISABLE_NEWLINE_AUTO_RETURN: CONSOLE_MODE = CONSOLE_MODE(8u32);
pub const DOUBLE_CLICK: u32 = 2u32;
pub const ENABLE_AUTO_POSITION: CONSOLE_MODE = CONSOLE_MODE(256u32);
pub const ENABLE_ECHO_INPUT: CONSOLE_MODE = CONSOLE_MODE(4u32);
pub const ENABLE_EXTENDED_FLAGS: CONSOLE_MODE = CONSOLE_MODE(128u32);
pub const ENABLE_INSERT_MODE: CONSOLE_MODE = CONSOLE_MODE(32u32);
pub const ENABLE_LINE_INPUT: CONSOLE_MODE = CONSOLE_MODE(2u32);
pub const ENABLE_LVB_GRID_WORLDWIDE: CONSOLE_MODE = CONSOLE_MODE(16u32);
pub const ENABLE_MOUSE_INPUT: CONSOLE_MODE = CONSOLE_MODE(16u32);
pub const ENABLE_PROCESSED_INPUT: CONSOLE_MODE = CONSOLE_MODE(1u32);
pub const ENABLE_PROCESSED_OUTPUT: CONSOLE_MODE = CONSOLE_MODE(1u32);
pub const ENABLE_QUICK_EDIT_MODE: CONSOLE_MODE = CONSOLE_MODE(64u32);
pub const ENABLE_VIRTUAL_TERMINAL_INPUT: CONSOLE_MODE = CONSOLE_MODE(512u32);
pub const ENABLE_VIRTUAL_TERMINAL_PROCESSING: CONSOLE_MODE = CONSOLE_MODE(4u32);
pub const ENABLE_WINDOW_INPUT: CONSOLE_MODE = CONSOLE_MODE(8u32);
pub const ENABLE_WRAP_AT_EOL_OUTPUT: CONSOLE_MODE = CONSOLE_MODE(2u32);
pub const ENHANCED_KEY: u32 = 256u32;
pub const FOCUS_EVENT: u32 = 16u32;
pub const FOREGROUND_BLUE: CONSOLE_CHARACTER_ATTRIBUTES = CONSOLE_CHARACTER_ATTRIBUTES(1u16);
pub const FOREGROUND_GREEN: CONSOLE_CHARACTER_ATTRIBUTES = CONSOLE_CHARACTER_ATTRIBUTES(2u16);
pub const FOREGROUND_INTENSITY: CONSOLE_CHARACTER_ATTRIBUTES = CONSOLE_CHARACTER_ATTRIBUTES(8u16);
pub const FOREGROUND_RED: CONSOLE_CHARACTER_ATTRIBUTES = CONSOLE_CHARACTER_ATTRIBUTES(4u16);
pub const FROM_LEFT_1ST_BUTTON_PRESSED: u32 = 1u32;
pub const FROM_LEFT_2ND_BUTTON_PRESSED: u32 = 4u32;
pub const FROM_LEFT_3RD_BUTTON_PRESSED: u32 = 8u32;
pub const FROM_LEFT_4TH_BUTTON_PRESSED: u32 = 16u32;
pub const HISTORY_NO_DUP_FLAG: u32 = 1u32;
pub const KEY_EVENT: u32 = 1u32;
pub const LEFT_ALT_PRESSED: u32 = 2u32;
pub const LEFT_CTRL_PRESSED: u32 = 8u32;
pub const MENU_EVENT: u32 = 8u32;
pub const MOUSE_EVENT: u32 = 2u32;
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
pub const PSEUDOCONSOLE_INHERIT_CURSOR: u32 = 1u32;
pub const RIGHTMOST_BUTTON_PRESSED: u32 = 2u32;
pub const RIGHT_ALT_PRESSED: u32 = 1u32;
pub const RIGHT_CTRL_PRESSED: u32 = 4u32;
pub const Reserved1: CONSOLECONTROL = CONSOLECONTROL(0i32);
pub const Reserved2: CONSOLECONTROL = CONSOLECONTROL(2i32);
pub const Reserved3: CONSOLECONTROL = CONSOLECONTROL(4i32);
pub const SCROLLLOCK_ON: u32 = 64u32;
pub const SHIFT_PRESSED: u32 = 16u32;
pub const STD_ERROR_HANDLE: STD_HANDLE = STD_HANDLE(4294967284u32);
pub const STD_INPUT_HANDLE: STD_HANDLE = STD_HANDLE(4294967286u32);
pub const STD_OUTPUT_HANDLE: STD_HANDLE = STD_HANDLE(4294967285u32);
pub const WINDOW_BUFFER_SIZE_EVENT: u32 = 4u32;
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CONSOLECONTROL(pub i32);
impl windows_core::TypeKind for CONSOLECONTROL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CONSOLECONTROL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CONSOLECONTROL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CONSOLE_CHARACTER_ATTRIBUTES(pub u16);
impl windows_core::TypeKind for CONSOLE_CHARACTER_ATTRIBUTES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CONSOLE_CHARACTER_ATTRIBUTES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CONSOLE_CHARACTER_ATTRIBUTES").field(&self.0).finish()
    }
}
impl CONSOLE_CHARACTER_ATTRIBUTES {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CONSOLE_CHARACTER_ATTRIBUTES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CONSOLE_CHARACTER_ATTRIBUTES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CONSOLE_CHARACTER_ATTRIBUTES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CONSOLE_CHARACTER_ATTRIBUTES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CONSOLE_CHARACTER_ATTRIBUTES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CONSOLE_MODE(pub u32);
impl windows_core::TypeKind for CONSOLE_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CONSOLE_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CONSOLE_MODE").field(&self.0).finish()
    }
}
impl CONSOLE_MODE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CONSOLE_MODE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CONSOLE_MODE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CONSOLE_MODE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CONSOLE_MODE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CONSOLE_MODE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct STD_HANDLE(pub u32);
impl windows_core::TypeKind for STD_HANDLE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for STD_HANDLE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("STD_HANDLE").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CHAR_INFO {
    pub Char: CHAR_INFO_0,
    pub Attributes: u16,
}
impl windows_core::TypeKind for CHAR_INFO {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for CHAR_INFO_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for CHAR_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CONSOLEENDTASK {
    pub ProcessId: super::super::Foundation::HANDLE,
    pub hwnd: super::super::Foundation::HWND,
    pub ConsoleEventCode: u32,
    pub ConsoleFlags: u32,
}
impl windows_core::TypeKind for CONSOLEENDTASK {
    type TypeKind = windows_core::CopyType;
}
impl Default for CONSOLEENDTASK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CONSOLESETFOREGROUND {
    pub hProcess: super::super::Foundation::HANDLE,
    pub bForeground: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for CONSOLESETFOREGROUND {
    type TypeKind = windows_core::CopyType;
}
impl Default for CONSOLESETFOREGROUND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CONSOLEWINDOWOWNER {
    pub hwnd: super::super::Foundation::HWND,
    pub ProcessId: u32,
    pub ThreadId: u32,
}
impl windows_core::TypeKind for CONSOLEWINDOWOWNER {
    type TypeKind = windows_core::CopyType;
}
impl Default for CONSOLEWINDOWOWNER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CONSOLE_CARET_INFO {
    pub hwnd: super::super::Foundation::HWND,
    pub rc: super::super::Foundation::RECT,
}
impl windows_core::TypeKind for CONSOLE_CARET_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for CONSOLE_CARET_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CONSOLE_CURSOR_INFO {
    pub dwSize: u32,
    pub bVisible: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for CONSOLE_CURSOR_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for CONSOLE_CURSOR_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CONSOLE_FONT_INFO {
    pub nFont: u32,
    pub dwFontSize: COORD,
}
impl windows_core::TypeKind for CONSOLE_FONT_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for CONSOLE_FONT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CONSOLE_FONT_INFOEX {
    pub cbSize: u32,
    pub nFont: u32,
    pub dwFontSize: COORD,
    pub FontFamily: u32,
    pub FontWeight: u32,
    pub FaceName: [u16; 32],
}
impl windows_core::TypeKind for CONSOLE_FONT_INFOEX {
    type TypeKind = windows_core::CopyType;
}
impl Default for CONSOLE_FONT_INFOEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CONSOLE_HISTORY_INFO {
    pub cbSize: u32,
    pub HistoryBufferSize: u32,
    pub NumberOfHistoryBuffers: u32,
    pub dwFlags: u32,
}
impl windows_core::TypeKind for CONSOLE_HISTORY_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for CONSOLE_HISTORY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CONSOLE_PROCESS_INFO {
    pub dwProcessID: u32,
    pub dwFlags: u32,
}
impl windows_core::TypeKind for CONSOLE_PROCESS_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for CONSOLE_PROCESS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CONSOLE_READCONSOLE_CONTROL {
    pub nLength: u32,
    pub nInitialChars: u32,
    pub dwCtrlWakeupMask: u32,
    pub dwControlKeyState: u32,
}
impl windows_core::TypeKind for CONSOLE_READCONSOLE_CONTROL {
    type TypeKind = windows_core::CopyType;
}
impl Default for CONSOLE_READCONSOLE_CONTROL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CONSOLE_SCREEN_BUFFER_INFO {
    pub dwSize: COORD,
    pub dwCursorPosition: COORD,
    pub wAttributes: CONSOLE_CHARACTER_ATTRIBUTES,
    pub srWindow: SMALL_RECT,
    pub dwMaximumWindowSize: COORD,
}
impl windows_core::TypeKind for CONSOLE_SCREEN_BUFFER_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for CONSOLE_SCREEN_BUFFER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CONSOLE_SCREEN_BUFFER_INFOEX {
    pub cbSize: u32,
    pub dwSize: COORD,
    pub dwCursorPosition: COORD,
    pub wAttributes: CONSOLE_CHARACTER_ATTRIBUTES,
    pub srWindow: SMALL_RECT,
    pub dwMaximumWindowSize: COORD,
    pub wPopupAttributes: u16,
    pub bFullscreenSupported: super::super::Foundation::BOOL,
    pub ColorTable: [super::super::Foundation::COLORREF; 16],
}
impl windows_core::TypeKind for CONSOLE_SCREEN_BUFFER_INFOEX {
    type TypeKind = windows_core::CopyType;
}
impl Default for CONSOLE_SCREEN_BUFFER_INFOEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CONSOLE_SELECTION_INFO {
    pub dwFlags: u32,
    pub dwSelectionAnchor: COORD,
    pub srSelection: SMALL_RECT,
}
impl windows_core::TypeKind for CONSOLE_SELECTION_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for CONSOLE_SELECTION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct COORD {
    pub X: i16,
    pub Y: i16,
}
impl windows_core::TypeKind for COORD {
    type TypeKind = windows_core::CopyType;
}
impl Default for COORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FOCUS_EVENT_RECORD {
    pub bSetFocus: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for FOCUS_EVENT_RECORD {
    type TypeKind = windows_core::CopyType;
}
impl Default for FOCUS_EVENT_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HPCON(pub isize);
impl HPCON {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl windows_core::Free for HPCON {
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            ClosePseudoConsole(*self);
        }
    }
}
impl Default for HPCON {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HPCON {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct INPUT_RECORD {
    pub EventType: u16,
    pub Event: INPUT_RECORD_0,
}
impl windows_core::TypeKind for INPUT_RECORD {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for INPUT_RECORD_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for INPUT_RECORD_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KEY_EVENT_RECORD {
    pub bKeyDown: super::super::Foundation::BOOL,
    pub wRepeatCount: u16,
    pub wVirtualKeyCode: u16,
    pub wVirtualScanCode: u16,
    pub uChar: KEY_EVENT_RECORD_0,
    pub dwControlKeyState: u32,
}
impl windows_core::TypeKind for KEY_EVENT_RECORD {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for KEY_EVENT_RECORD_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for KEY_EVENT_RECORD_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MENU_EVENT_RECORD {
    pub dwCommandId: u32,
}
impl windows_core::TypeKind for MENU_EVENT_RECORD {
    type TypeKind = windows_core::CopyType;
}
impl Default for MENU_EVENT_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MOUSE_EVENT_RECORD {
    pub dwMousePosition: COORD,
    pub dwButtonState: u32,
    pub dwControlKeyState: u32,
    pub dwEventFlags: u32,
}
impl windows_core::TypeKind for MOUSE_EVENT_RECORD {
    type TypeKind = windows_core::CopyType;
}
impl Default for MOUSE_EVENT_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SMALL_RECT {
    pub Left: i16,
    pub Top: i16,
    pub Right: i16,
    pub Bottom: i16,
}
impl windows_core::TypeKind for SMALL_RECT {
    type TypeKind = windows_core::CopyType;
}
impl Default for SMALL_RECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WINDOW_BUFFER_SIZE_RECORD {
    pub dwSize: COORD,
}
impl windows_core::TypeKind for WINDOW_BUFFER_SIZE_RECORD {
    type TypeKind = windows_core::CopyType;
}
impl Default for WINDOW_BUFFER_SIZE_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PHANDLER_ROUTINE = Option<unsafe extern "system" fn(ctrltype: u32) -> super::super::Foundation::BOOL>;
