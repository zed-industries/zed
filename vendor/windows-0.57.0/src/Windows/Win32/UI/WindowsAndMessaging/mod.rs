#[inline]
pub unsafe fn AdjustWindowRect<P0>(lprect: *mut super::super::Foundation::RECT, dwstyle: WINDOW_STYLE, bmenu: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("user32.dll" "system" fn AdjustWindowRect(lprect : *mut super::super::Foundation:: RECT, dwstyle : WINDOW_STYLE, bmenu : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    AdjustWindowRect(lprect, dwstyle, bmenu.param().abi()).ok()
}
#[inline]
pub unsafe fn AdjustWindowRectEx<P0>(lprect: *mut super::super::Foundation::RECT, dwstyle: WINDOW_STYLE, bmenu: P0, dwexstyle: WINDOW_EX_STYLE) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("user32.dll" "system" fn AdjustWindowRectEx(lprect : *mut super::super::Foundation:: RECT, dwstyle : WINDOW_STYLE, bmenu : super::super::Foundation:: BOOL, dwexstyle : WINDOW_EX_STYLE) -> super::super::Foundation:: BOOL);
    AdjustWindowRectEx(lprect, dwstyle, bmenu.param().abi(), dwexstyle).ok()
}
#[inline]
pub unsafe fn AllowSetForegroundWindow(dwprocessid: u32) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn AllowSetForegroundWindow(dwprocessid : u32) -> super::super::Foundation:: BOOL);
    AllowSetForegroundWindow(dwprocessid).ok()
}
#[inline]
pub unsafe fn AnimateWindow<P0>(hwnd: P0, dwtime: u32, dwflags: ANIMATE_WINDOW_FLAGS) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn AnimateWindow(hwnd : super::super::Foundation:: HWND, dwtime : u32, dwflags : ANIMATE_WINDOW_FLAGS) -> super::super::Foundation:: BOOL);
    AnimateWindow(hwnd.param().abi(), dwtime, dwflags).ok()
}
#[inline]
pub unsafe fn AnyPopup() -> super::super::Foundation::BOOL {
    windows_targets::link!("user32.dll" "system" fn AnyPopup() -> super::super::Foundation:: BOOL);
    AnyPopup()
}
#[inline]
pub unsafe fn AppendMenuA<P0, P1>(hmenu: P0, uflags: MENU_ITEM_FLAGS, uidnewitem: usize, lpnewitem: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<HMENU>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("user32.dll" "system" fn AppendMenuA(hmenu : HMENU, uflags : MENU_ITEM_FLAGS, uidnewitem : usize, lpnewitem : windows_core::PCSTR) -> super::super::Foundation:: BOOL);
    AppendMenuA(hmenu.param().abi(), uflags, uidnewitem, lpnewitem.param().abi()).ok()
}
#[inline]
pub unsafe fn AppendMenuW<P0, P1>(hmenu: P0, uflags: MENU_ITEM_FLAGS, uidnewitem: usize, lpnewitem: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<HMENU>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("user32.dll" "system" fn AppendMenuW(hmenu : HMENU, uflags : MENU_ITEM_FLAGS, uidnewitem : usize, lpnewitem : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    AppendMenuW(hmenu.param().abi(), uflags, uidnewitem, lpnewitem.param().abi()).ok()
}
#[inline]
pub unsafe fn ArrangeIconicWindows<P0>(hwnd: P0) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn ArrangeIconicWindows(hwnd : super::super::Foundation:: HWND) -> u32);
    ArrangeIconicWindows(hwnd.param().abi())
}
#[inline]
pub unsafe fn BeginDeferWindowPos(nnumwindows: i32) -> windows_core::Result<HDWP> {
    windows_targets::link!("user32.dll" "system" fn BeginDeferWindowPos(nnumwindows : i32) -> HDWP);
    let result__ = BeginDeferWindowPos(nnumwindows);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn BringWindowToTop<P0>(hwnd: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn BringWindowToTop(hwnd : super::super::Foundation:: HWND) -> super::super::Foundation:: BOOL);
    BringWindowToTop(hwnd.param().abi()).ok()
}
#[inline]
pub unsafe fn CalculatePopupWindowPosition(anchorpoint: *const super::super::Foundation::POINT, windowsize: *const super::super::Foundation::SIZE, flags: u32, excluderect: Option<*const super::super::Foundation::RECT>, popupwindowposition: *mut super::super::Foundation::RECT) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn CalculatePopupWindowPosition(anchorpoint : *const super::super::Foundation:: POINT, windowsize : *const super::super::Foundation:: SIZE, flags : u32, excluderect : *const super::super::Foundation:: RECT, popupwindowposition : *mut super::super::Foundation:: RECT) -> super::super::Foundation:: BOOL);
    CalculatePopupWindowPosition(anchorpoint, windowsize, flags, core::mem::transmute(excluderect.unwrap_or(std::ptr::null())), popupwindowposition).ok()
}
#[inline]
pub unsafe fn CallMsgFilterA(lpmsg: *const MSG, ncode: i32) -> super::super::Foundation::BOOL {
    windows_targets::link!("user32.dll" "system" fn CallMsgFilterA(lpmsg : *const MSG, ncode : i32) -> super::super::Foundation:: BOOL);
    CallMsgFilterA(lpmsg, ncode)
}
#[inline]
pub unsafe fn CallMsgFilterW(lpmsg: *const MSG, ncode: i32) -> super::super::Foundation::BOOL {
    windows_targets::link!("user32.dll" "system" fn CallMsgFilterW(lpmsg : *const MSG, ncode : i32) -> super::super::Foundation:: BOOL);
    CallMsgFilterW(lpmsg, ncode)
}
#[inline]
pub unsafe fn CallNextHookEx<P0, P1, P2>(hhk: P0, ncode: i32, wparam: P1, lparam: P2) -> super::super::Foundation::LRESULT
where
    P0: windows_core::Param<HHOOK>,
    P1: windows_core::Param<super::super::Foundation::WPARAM>,
    P2: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("user32.dll" "system" fn CallNextHookEx(hhk : HHOOK, ncode : i32, wparam : super::super::Foundation:: WPARAM, lparam : super::super::Foundation:: LPARAM) -> super::super::Foundation:: LRESULT);
    CallNextHookEx(hhk.param().abi(), ncode, wparam.param().abi(), lparam.param().abi())
}
#[inline]
pub unsafe fn CallWindowProcA<P0, P1, P2>(lpprevwndfunc: WNDPROC, hwnd: P0, msg: u32, wparam: P1, lparam: P2) -> super::super::Foundation::LRESULT
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::WPARAM>,
    P2: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("user32.dll" "system" fn CallWindowProcA(lpprevwndfunc : WNDPROC, hwnd : super::super::Foundation:: HWND, msg : u32, wparam : super::super::Foundation:: WPARAM, lparam : super::super::Foundation:: LPARAM) -> super::super::Foundation:: LRESULT);
    CallWindowProcA(lpprevwndfunc, hwnd.param().abi(), msg, wparam.param().abi(), lparam.param().abi())
}
#[inline]
pub unsafe fn CallWindowProcW<P0, P1, P2>(lpprevwndfunc: WNDPROC, hwnd: P0, msg: u32, wparam: P1, lparam: P2) -> super::super::Foundation::LRESULT
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::WPARAM>,
    P2: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("user32.dll" "system" fn CallWindowProcW(lpprevwndfunc : WNDPROC, hwnd : super::super::Foundation:: HWND, msg : u32, wparam : super::super::Foundation:: WPARAM, lparam : super::super::Foundation:: LPARAM) -> super::super::Foundation:: LRESULT);
    CallWindowProcW(lpprevwndfunc, hwnd.param().abi(), msg, wparam.param().abi(), lparam.param().abi())
}
#[inline]
pub unsafe fn CancelShutdown() -> super::super::Foundation::BOOL {
    windows_targets::link!("user32.dll" "system" fn CancelShutdown() -> super::super::Foundation:: BOOL);
    CancelShutdown()
}
#[inline]
pub unsafe fn CascadeWindows<P0>(hwndparent: P0, whow: CASCADE_WINDOWS_HOW, lprect: Option<*const super::super::Foundation::RECT>, lpkids: Option<&[super::super::Foundation::HWND]>) -> u16
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn CascadeWindows(hwndparent : super::super::Foundation:: HWND, whow : CASCADE_WINDOWS_HOW, lprect : *const super::super::Foundation:: RECT, ckids : u32, lpkids : *const super::super::Foundation:: HWND) -> u16);
    CascadeWindows(hwndparent.param().abi(), whow, core::mem::transmute(lprect.unwrap_or(std::ptr::null())), lpkids.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lpkids.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
}
#[inline]
pub unsafe fn ChangeMenuA<P0, P1>(hmenu: P0, cmd: u32, lpsznewitem: P1, cmdinsert: u32, flags: u32) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<HMENU>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("user32.dll" "system" fn ChangeMenuA(hmenu : HMENU, cmd : u32, lpsznewitem : windows_core::PCSTR, cmdinsert : u32, flags : u32) -> super::super::Foundation:: BOOL);
    ChangeMenuA(hmenu.param().abi(), cmd, lpsznewitem.param().abi(), cmdinsert, flags)
}
#[inline]
pub unsafe fn ChangeMenuW<P0, P1>(hmenu: P0, cmd: u32, lpsznewitem: P1, cmdinsert: u32, flags: u32) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<HMENU>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("user32.dll" "system" fn ChangeMenuW(hmenu : HMENU, cmd : u32, lpsznewitem : windows_core::PCWSTR, cmdinsert : u32, flags : u32) -> super::super::Foundation:: BOOL);
    ChangeMenuW(hmenu.param().abi(), cmd, lpsznewitem.param().abi(), cmdinsert, flags)
}
#[inline]
pub unsafe fn ChangeWindowMessageFilter(message: u32, dwflag: CHANGE_WINDOW_MESSAGE_FILTER_FLAGS) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn ChangeWindowMessageFilter(message : u32, dwflag : CHANGE_WINDOW_MESSAGE_FILTER_FLAGS) -> super::super::Foundation:: BOOL);
    ChangeWindowMessageFilter(message, dwflag).ok()
}
#[inline]
pub unsafe fn ChangeWindowMessageFilterEx<P0>(hwnd: P0, message: u32, action: WINDOW_MESSAGE_FILTER_ACTION, pchangefilterstruct: Option<*mut CHANGEFILTERSTRUCT>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn ChangeWindowMessageFilterEx(hwnd : super::super::Foundation:: HWND, message : u32, action : WINDOW_MESSAGE_FILTER_ACTION, pchangefilterstruct : *mut CHANGEFILTERSTRUCT) -> super::super::Foundation:: BOOL);
    ChangeWindowMessageFilterEx(hwnd.param().abi(), message, action, core::mem::transmute(pchangefilterstruct.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn CharLowerA(lpsz: windows_core::PSTR) -> windows_core::PSTR {
    windows_targets::link!("user32.dll" "system" fn CharLowerA(lpsz : windows_core::PSTR) -> windows_core::PSTR);
    CharLowerA(core::mem::transmute(lpsz))
}
#[inline]
pub unsafe fn CharLowerBuffA(lpsz: &mut [u8]) -> u32 {
    windows_targets::link!("user32.dll" "system" fn CharLowerBuffA(lpsz : windows_core::PSTR, cchlength : u32) -> u32);
    CharLowerBuffA(core::mem::transmute(lpsz.as_ptr()), lpsz.len().try_into().unwrap())
}
#[inline]
pub unsafe fn CharLowerBuffW(lpsz: &mut [u16]) -> u32 {
    windows_targets::link!("user32.dll" "system" fn CharLowerBuffW(lpsz : windows_core::PWSTR, cchlength : u32) -> u32);
    CharLowerBuffW(core::mem::transmute(lpsz.as_ptr()), lpsz.len().try_into().unwrap())
}
#[inline]
pub unsafe fn CharLowerW(lpsz: windows_core::PWSTR) -> windows_core::PWSTR {
    windows_targets::link!("user32.dll" "system" fn CharLowerW(lpsz : windows_core::PWSTR) -> windows_core::PWSTR);
    CharLowerW(core::mem::transmute(lpsz))
}
#[inline]
pub unsafe fn CharNextA<P0>(lpsz: P0) -> windows_core::PSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("user32.dll" "system" fn CharNextA(lpsz : windows_core::PCSTR) -> windows_core::PSTR);
    CharNextA(lpsz.param().abi())
}
#[inline]
pub unsafe fn CharNextExA<P0>(codepage: u16, lpcurrentchar: P0, dwflags: u32) -> windows_core::PSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("user32.dll" "system" fn CharNextExA(codepage : u16, lpcurrentchar : windows_core::PCSTR, dwflags : u32) -> windows_core::PSTR);
    CharNextExA(codepage, lpcurrentchar.param().abi(), dwflags)
}
#[inline]
pub unsafe fn CharNextW<P0>(lpsz: P0) -> windows_core::PWSTR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("user32.dll" "system" fn CharNextW(lpsz : windows_core::PCWSTR) -> windows_core::PWSTR);
    CharNextW(lpsz.param().abi())
}
#[inline]
pub unsafe fn CharPrevA<P0, P1>(lpszstart: P0, lpszcurrent: P1) -> windows_core::PSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("user32.dll" "system" fn CharPrevA(lpszstart : windows_core::PCSTR, lpszcurrent : windows_core::PCSTR) -> windows_core::PSTR);
    CharPrevA(lpszstart.param().abi(), lpszcurrent.param().abi())
}
#[inline]
pub unsafe fn CharPrevExA<P0, P1>(codepage: u16, lpstart: P0, lpcurrentchar: P1, dwflags: u32) -> windows_core::PSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("user32.dll" "system" fn CharPrevExA(codepage : u16, lpstart : windows_core::PCSTR, lpcurrentchar : windows_core::PCSTR, dwflags : u32) -> windows_core::PSTR);
    CharPrevExA(codepage, lpstart.param().abi(), lpcurrentchar.param().abi(), dwflags)
}
#[inline]
pub unsafe fn CharPrevW<P0, P1>(lpszstart: P0, lpszcurrent: P1) -> windows_core::PWSTR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("user32.dll" "system" fn CharPrevW(lpszstart : windows_core::PCWSTR, lpszcurrent : windows_core::PCWSTR) -> windows_core::PWSTR);
    CharPrevW(lpszstart.param().abi(), lpszcurrent.param().abi())
}
#[inline]
pub unsafe fn CharToOemA<P0>(psrc: P0, pdst: windows_core::PSTR) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("user32.dll" "system" fn CharToOemA(psrc : windows_core::PCSTR, pdst : windows_core::PSTR) -> super::super::Foundation:: BOOL);
    CharToOemA(psrc.param().abi(), core::mem::transmute(pdst)).ok()
}
#[inline]
pub unsafe fn CharToOemBuffA<P0>(lpszsrc: P0, lpszdst: &mut [u8]) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("user32.dll" "system" fn CharToOemBuffA(lpszsrc : windows_core::PCSTR, lpszdst : windows_core::PSTR, cchdstlength : u32) -> super::super::Foundation:: BOOL);
    CharToOemBuffA(lpszsrc.param().abi(), core::mem::transmute(lpszdst.as_ptr()), lpszdst.len().try_into().unwrap()).ok()
}
#[inline]
pub unsafe fn CharToOemBuffW<P0>(lpszsrc: P0, lpszdst: &mut [u8]) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("user32.dll" "system" fn CharToOemBuffW(lpszsrc : windows_core::PCWSTR, lpszdst : windows_core::PSTR, cchdstlength : u32) -> super::super::Foundation:: BOOL);
    CharToOemBuffW(lpszsrc.param().abi(), core::mem::transmute(lpszdst.as_ptr()), lpszdst.len().try_into().unwrap()).ok()
}
#[inline]
pub unsafe fn CharToOemW<P0>(psrc: P0, pdst: windows_core::PSTR) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("user32.dll" "system" fn CharToOemW(psrc : windows_core::PCWSTR, pdst : windows_core::PSTR) -> super::super::Foundation:: BOOL);
    CharToOemW(psrc.param().abi(), core::mem::transmute(pdst)).ok()
}
#[inline]
pub unsafe fn CharUpperA(lpsz: windows_core::PSTR) -> windows_core::PSTR {
    windows_targets::link!("user32.dll" "system" fn CharUpperA(lpsz : windows_core::PSTR) -> windows_core::PSTR);
    CharUpperA(core::mem::transmute(lpsz))
}
#[inline]
pub unsafe fn CharUpperBuffA(lpsz: &mut [u8]) -> u32 {
    windows_targets::link!("user32.dll" "system" fn CharUpperBuffA(lpsz : windows_core::PSTR, cchlength : u32) -> u32);
    CharUpperBuffA(core::mem::transmute(lpsz.as_ptr()), lpsz.len().try_into().unwrap())
}
#[inline]
pub unsafe fn CharUpperBuffW(lpsz: &mut [u16]) -> u32 {
    windows_targets::link!("user32.dll" "system" fn CharUpperBuffW(lpsz : windows_core::PWSTR, cchlength : u32) -> u32);
    CharUpperBuffW(core::mem::transmute(lpsz.as_ptr()), lpsz.len().try_into().unwrap())
}
#[inline]
pub unsafe fn CharUpperW(lpsz: windows_core::PWSTR) -> windows_core::PWSTR {
    windows_targets::link!("user32.dll" "system" fn CharUpperW(lpsz : windows_core::PWSTR) -> windows_core::PWSTR);
    CharUpperW(core::mem::transmute(lpsz))
}
#[inline]
pub unsafe fn CheckMenuItem<P0>(hmenu: P0, uidcheckitem: u32, ucheck: u32) -> u32
where
    P0: windows_core::Param<HMENU>,
{
    windows_targets::link!("user32.dll" "system" fn CheckMenuItem(hmenu : HMENU, uidcheckitem : u32, ucheck : u32) -> u32);
    CheckMenuItem(hmenu.param().abi(), uidcheckitem, ucheck)
}
#[inline]
pub unsafe fn CheckMenuRadioItem<P0>(hmenu: P0, first: u32, last: u32, check: u32, flags: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<HMENU>,
{
    windows_targets::link!("user32.dll" "system" fn CheckMenuRadioItem(hmenu : HMENU, first : u32, last : u32, check : u32, flags : u32) -> super::super::Foundation:: BOOL);
    CheckMenuRadioItem(hmenu.param().abi(), first, last, check, flags).ok()
}
#[inline]
pub unsafe fn ChildWindowFromPoint<P0>(hwndparent: P0, point: super::super::Foundation::POINT) -> super::super::Foundation::HWND
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn ChildWindowFromPoint(hwndparent : super::super::Foundation:: HWND, point : super::super::Foundation:: POINT) -> super::super::Foundation:: HWND);
    ChildWindowFromPoint(hwndparent.param().abi(), core::mem::transmute(point))
}
#[inline]
pub unsafe fn ChildWindowFromPointEx<P0>(hwnd: P0, pt: super::super::Foundation::POINT, flags: CWP_FLAGS) -> super::super::Foundation::HWND
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn ChildWindowFromPointEx(hwnd : super::super::Foundation:: HWND, pt : super::super::Foundation:: POINT, flags : CWP_FLAGS) -> super::super::Foundation:: HWND);
    ChildWindowFromPointEx(hwnd.param().abi(), core::mem::transmute(pt), flags)
}
#[inline]
pub unsafe fn ClipCursor(lprect: Option<*const super::super::Foundation::RECT>) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn ClipCursor(lprect : *const super::super::Foundation:: RECT) -> super::super::Foundation:: BOOL);
    ClipCursor(core::mem::transmute(lprect.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn CloseWindow<P0>(hwnd: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn CloseWindow(hwnd : super::super::Foundation:: HWND) -> super::super::Foundation:: BOOL);
    CloseWindow(hwnd.param().abi()).ok()
}
#[inline]
pub unsafe fn CopyAcceleratorTableA<P0>(haccelsrc: P0, lpacceldst: Option<&mut [ACCEL]>) -> i32
where
    P0: windows_core::Param<HACCEL>,
{
    windows_targets::link!("user32.dll" "system" fn CopyAcceleratorTableA(haccelsrc : HACCEL, lpacceldst : *mut ACCEL, caccelentries : i32) -> i32);
    CopyAcceleratorTableA(haccelsrc.param().abi(), core::mem::transmute(lpacceldst.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpacceldst.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn CopyAcceleratorTableW<P0>(haccelsrc: P0, lpacceldst: Option<&mut [ACCEL]>) -> i32
where
    P0: windows_core::Param<HACCEL>,
{
    windows_targets::link!("user32.dll" "system" fn CopyAcceleratorTableW(haccelsrc : HACCEL, lpacceldst : *mut ACCEL, caccelentries : i32) -> i32);
    CopyAcceleratorTableW(haccelsrc.param().abi(), core::mem::transmute(lpacceldst.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpacceldst.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn CopyIcon<P0>(hicon: P0) -> windows_core::Result<HICON>
where
    P0: windows_core::Param<HICON>,
{
    windows_targets::link!("user32.dll" "system" fn CopyIcon(hicon : HICON) -> HICON);
    let result__ = CopyIcon(hicon.param().abi());
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn CopyImage<P0>(h: P0, r#type: GDI_IMAGE_TYPE, cx: i32, cy: i32, flags: IMAGE_FLAGS) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("user32.dll" "system" fn CopyImage(h : super::super::Foundation:: HANDLE, r#type : GDI_IMAGE_TYPE, cx : i32, cy : i32, flags : IMAGE_FLAGS) -> super::super::Foundation:: HANDLE);
    let result__ = CopyImage(h.param().abi(), r#type, cx, cy, flags);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn CreateAcceleratorTableA(paccel: &[ACCEL]) -> windows_core::Result<HACCEL> {
    windows_targets::link!("user32.dll" "system" fn CreateAcceleratorTableA(paccel : *const ACCEL, caccel : i32) -> HACCEL);
    let result__ = CreateAcceleratorTableA(core::mem::transmute(paccel.as_ptr()), paccel.len().try_into().unwrap());
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn CreateAcceleratorTableW(paccel: &[ACCEL]) -> windows_core::Result<HACCEL> {
    windows_targets::link!("user32.dll" "system" fn CreateAcceleratorTableW(paccel : *const ACCEL, caccel : i32) -> HACCEL);
    let result__ = CreateAcceleratorTableW(core::mem::transmute(paccel.as_ptr()), paccel.len().try_into().unwrap());
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn CreateCaret<P0, P1>(hwnd: P0, hbitmap: P1, nwidth: i32, nheight: i32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Graphics::Gdi::HBITMAP>,
{
    windows_targets::link!("user32.dll" "system" fn CreateCaret(hwnd : super::super::Foundation:: HWND, hbitmap : super::super::Graphics::Gdi:: HBITMAP, nwidth : i32, nheight : i32) -> super::super::Foundation:: BOOL);
    CreateCaret(hwnd.param().abi(), hbitmap.param().abi(), nwidth, nheight).ok()
}
#[inline]
pub unsafe fn CreateCursor<P0>(hinst: P0, xhotspot: i32, yhotspot: i32, nwidth: i32, nheight: i32, pvandplane: *const core::ffi::c_void, pvxorplane: *const core::ffi::c_void) -> windows_core::Result<HCURSOR>
where
    P0: windows_core::Param<super::super::Foundation::HINSTANCE>,
{
    windows_targets::link!("user32.dll" "system" fn CreateCursor(hinst : super::super::Foundation:: HINSTANCE, xhotspot : i32, yhotspot : i32, nwidth : i32, nheight : i32, pvandplane : *const core::ffi::c_void, pvxorplane : *const core::ffi::c_void) -> HCURSOR);
    let result__ = CreateCursor(hinst.param().abi(), xhotspot, yhotspot, nwidth, nheight, pvandplane, pvxorplane);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn CreateDialogIndirectParamA<P0, P1, P2>(hinstance: P0, lptemplate: *const DLGTEMPLATE, hwndparent: P1, lpdialogfunc: DLGPROC, dwinitparam: P2) -> super::super::Foundation::HWND
where
    P0: windows_core::Param<super::super::Foundation::HINSTANCE>,
    P1: windows_core::Param<super::super::Foundation::HWND>,
    P2: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("user32.dll" "system" fn CreateDialogIndirectParamA(hinstance : super::super::Foundation:: HINSTANCE, lptemplate : *const DLGTEMPLATE, hwndparent : super::super::Foundation:: HWND, lpdialogfunc : DLGPROC, dwinitparam : super::super::Foundation:: LPARAM) -> super::super::Foundation:: HWND);
    CreateDialogIndirectParamA(hinstance.param().abi(), lptemplate, hwndparent.param().abi(), lpdialogfunc, dwinitparam.param().abi())
}
#[inline]
pub unsafe fn CreateDialogIndirectParamW<P0, P1, P2>(hinstance: P0, lptemplate: *const DLGTEMPLATE, hwndparent: P1, lpdialogfunc: DLGPROC, dwinitparam: P2) -> super::super::Foundation::HWND
where
    P0: windows_core::Param<super::super::Foundation::HINSTANCE>,
    P1: windows_core::Param<super::super::Foundation::HWND>,
    P2: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("user32.dll" "system" fn CreateDialogIndirectParamW(hinstance : super::super::Foundation:: HINSTANCE, lptemplate : *const DLGTEMPLATE, hwndparent : super::super::Foundation:: HWND, lpdialogfunc : DLGPROC, dwinitparam : super::super::Foundation:: LPARAM) -> super::super::Foundation:: HWND);
    CreateDialogIndirectParamW(hinstance.param().abi(), lptemplate, hwndparent.param().abi(), lpdialogfunc, dwinitparam.param().abi())
}
#[inline]
pub unsafe fn CreateDialogParamA<P0, P1, P2, P3>(hinstance: P0, lptemplatename: P1, hwndparent: P2, lpdialogfunc: DLGPROC, dwinitparam: P3) -> super::super::Foundation::HWND
where
    P0: windows_core::Param<super::super::Foundation::HINSTANCE>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<super::super::Foundation::HWND>,
    P3: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("user32.dll" "system" fn CreateDialogParamA(hinstance : super::super::Foundation:: HINSTANCE, lptemplatename : windows_core::PCSTR, hwndparent : super::super::Foundation:: HWND, lpdialogfunc : DLGPROC, dwinitparam : super::super::Foundation:: LPARAM) -> super::super::Foundation:: HWND);
    CreateDialogParamA(hinstance.param().abi(), lptemplatename.param().abi(), hwndparent.param().abi(), lpdialogfunc, dwinitparam.param().abi())
}
#[inline]
pub unsafe fn CreateDialogParamW<P0, P1, P2, P3>(hinstance: P0, lptemplatename: P1, hwndparent: P2, lpdialogfunc: DLGPROC, dwinitparam: P3) -> super::super::Foundation::HWND
where
    P0: windows_core::Param<super::super::Foundation::HINSTANCE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::super::Foundation::HWND>,
    P3: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("user32.dll" "system" fn CreateDialogParamW(hinstance : super::super::Foundation:: HINSTANCE, lptemplatename : windows_core::PCWSTR, hwndparent : super::super::Foundation:: HWND, lpdialogfunc : DLGPROC, dwinitparam : super::super::Foundation:: LPARAM) -> super::super::Foundation:: HWND);
    CreateDialogParamW(hinstance.param().abi(), lptemplatename.param().abi(), hwndparent.param().abi(), lpdialogfunc, dwinitparam.param().abi())
}
#[inline]
pub unsafe fn CreateIcon<P0>(hinstance: P0, nwidth: i32, nheight: i32, cplanes: u8, cbitspixel: u8, lpbandbits: *const u8, lpbxorbits: *const u8) -> windows_core::Result<HICON>
where
    P0: windows_core::Param<super::super::Foundation::HINSTANCE>,
{
    windows_targets::link!("user32.dll" "system" fn CreateIcon(hinstance : super::super::Foundation:: HINSTANCE, nwidth : i32, nheight : i32, cplanes : u8, cbitspixel : u8, lpbandbits : *const u8, lpbxorbits : *const u8) -> HICON);
    let result__ = CreateIcon(hinstance.param().abi(), nwidth, nheight, cplanes, cbitspixel, lpbandbits, lpbxorbits);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn CreateIconFromResource<P0>(presbits: &[u8], ficon: P0, dwver: u32) -> windows_core::Result<HICON>
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("user32.dll" "system" fn CreateIconFromResource(presbits : *const u8, dwressize : u32, ficon : super::super::Foundation:: BOOL, dwver : u32) -> HICON);
    let result__ = CreateIconFromResource(core::mem::transmute(presbits.as_ptr()), presbits.len().try_into().unwrap(), ficon.param().abi(), dwver);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn CreateIconFromResourceEx<P0>(presbits: &[u8], ficon: P0, dwver: u32, cxdesired: i32, cydesired: i32, flags: IMAGE_FLAGS) -> windows_core::Result<HICON>
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("user32.dll" "system" fn CreateIconFromResourceEx(presbits : *const u8, dwressize : u32, ficon : super::super::Foundation:: BOOL, dwver : u32, cxdesired : i32, cydesired : i32, flags : IMAGE_FLAGS) -> HICON);
    let result__ = CreateIconFromResourceEx(core::mem::transmute(presbits.as_ptr()), presbits.len().try_into().unwrap(), ficon.param().abi(), dwver, cxdesired, cydesired, flags);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn CreateIconIndirect(piconinfo: *const ICONINFO) -> windows_core::Result<HICON> {
    windows_targets::link!("user32.dll" "system" fn CreateIconIndirect(piconinfo : *const ICONINFO) -> HICON);
    let result__ = CreateIconIndirect(piconinfo);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn CreateMDIWindowA<P0, P1, P2, P3, P4>(lpclassname: P0, lpwindowname: P1, dwstyle: WINDOW_STYLE, x: i32, y: i32, nwidth: i32, nheight: i32, hwndparent: P2, hinstance: P3, lparam: P4) -> super::super::Foundation::HWND
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<super::super::Foundation::HWND>,
    P3: windows_core::Param<super::super::Foundation::HINSTANCE>,
    P4: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("user32.dll" "system" fn CreateMDIWindowA(lpclassname : windows_core::PCSTR, lpwindowname : windows_core::PCSTR, dwstyle : WINDOW_STYLE, x : i32, y : i32, nwidth : i32, nheight : i32, hwndparent : super::super::Foundation:: HWND, hinstance : super::super::Foundation:: HINSTANCE, lparam : super::super::Foundation:: LPARAM) -> super::super::Foundation:: HWND);
    CreateMDIWindowA(lpclassname.param().abi(), lpwindowname.param().abi(), dwstyle, x, y, nwidth, nheight, hwndparent.param().abi(), hinstance.param().abi(), lparam.param().abi())
}
#[inline]
pub unsafe fn CreateMDIWindowW<P0, P1, P2, P3, P4>(lpclassname: P0, lpwindowname: P1, dwstyle: WINDOW_STYLE, x: i32, y: i32, nwidth: i32, nheight: i32, hwndparent: P2, hinstance: P3, lparam: P4) -> super::super::Foundation::HWND
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::super::Foundation::HWND>,
    P3: windows_core::Param<super::super::Foundation::HINSTANCE>,
    P4: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("user32.dll" "system" fn CreateMDIWindowW(lpclassname : windows_core::PCWSTR, lpwindowname : windows_core::PCWSTR, dwstyle : WINDOW_STYLE, x : i32, y : i32, nwidth : i32, nheight : i32, hwndparent : super::super::Foundation:: HWND, hinstance : super::super::Foundation:: HINSTANCE, lparam : super::super::Foundation:: LPARAM) -> super::super::Foundation:: HWND);
    CreateMDIWindowW(lpclassname.param().abi(), lpwindowname.param().abi(), dwstyle, x, y, nwidth, nheight, hwndparent.param().abi(), hinstance.param().abi(), lparam.param().abi())
}
#[inline]
pub unsafe fn CreateMenu() -> windows_core::Result<HMENU> {
    windows_targets::link!("user32.dll" "system" fn CreateMenu() -> HMENU);
    let result__ = CreateMenu();
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn CreatePopupMenu() -> windows_core::Result<HMENU> {
    windows_targets::link!("user32.dll" "system" fn CreatePopupMenu() -> HMENU);
    let result__ = CreatePopupMenu();
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn CreateResourceIndexer<P0, P1>(projectroot: P0, extensiondllpath: P1, ppresourceindexer: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mrmsupport.dll" "system" fn CreateResourceIndexer(projectroot : windows_core::PCWSTR, extensiondllpath : windows_core::PCWSTR, ppresourceindexer : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    CreateResourceIndexer(projectroot.param().abi(), extensiondllpath.param().abi(), ppresourceindexer).ok()
}
#[inline]
pub unsafe fn CreateWindowExA<P0, P1, P2, P3, P4>(dwexstyle: WINDOW_EX_STYLE, lpclassname: P0, lpwindowname: P1, dwstyle: WINDOW_STYLE, x: i32, y: i32, nwidth: i32, nheight: i32, hwndparent: P2, hmenu: P3, hinstance: P4, lpparam: Option<*const core::ffi::c_void>) -> super::super::Foundation::HWND
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<super::super::Foundation::HWND>,
    P3: windows_core::Param<HMENU>,
    P4: windows_core::Param<super::super::Foundation::HINSTANCE>,
{
    windows_targets::link!("user32.dll" "system" fn CreateWindowExA(dwexstyle : WINDOW_EX_STYLE, lpclassname : windows_core::PCSTR, lpwindowname : windows_core::PCSTR, dwstyle : WINDOW_STYLE, x : i32, y : i32, nwidth : i32, nheight : i32, hwndparent : super::super::Foundation:: HWND, hmenu : HMENU, hinstance : super::super::Foundation:: HINSTANCE, lpparam : *const core::ffi::c_void) -> super::super::Foundation:: HWND);
    CreateWindowExA(dwexstyle, lpclassname.param().abi(), lpwindowname.param().abi(), dwstyle, x, y, nwidth, nheight, hwndparent.param().abi(), hmenu.param().abi(), hinstance.param().abi(), core::mem::transmute(lpparam.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn CreateWindowExW<P0, P1, P2, P3, P4>(dwexstyle: WINDOW_EX_STYLE, lpclassname: P0, lpwindowname: P1, dwstyle: WINDOW_STYLE, x: i32, y: i32, nwidth: i32, nheight: i32, hwndparent: P2, hmenu: P3, hinstance: P4, lpparam: Option<*const core::ffi::c_void>) -> super::super::Foundation::HWND
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::super::Foundation::HWND>,
    P3: windows_core::Param<HMENU>,
    P4: windows_core::Param<super::super::Foundation::HINSTANCE>,
{
    windows_targets::link!("user32.dll" "system" fn CreateWindowExW(dwexstyle : WINDOW_EX_STYLE, lpclassname : windows_core::PCWSTR, lpwindowname : windows_core::PCWSTR, dwstyle : WINDOW_STYLE, x : i32, y : i32, nwidth : i32, nheight : i32, hwndparent : super::super::Foundation:: HWND, hmenu : HMENU, hinstance : super::super::Foundation:: HINSTANCE, lpparam : *const core::ffi::c_void) -> super::super::Foundation:: HWND);
    CreateWindowExW(dwexstyle, lpclassname.param().abi(), lpwindowname.param().abi(), dwstyle, x, y, nwidth, nheight, hwndparent.param().abi(), hmenu.param().abi(), hinstance.param().abi(), core::mem::transmute(lpparam.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn DefDlgProcA<P0, P1, P2>(hdlg: P0, msg: u32, wparam: P1, lparam: P2) -> super::super::Foundation::LRESULT
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::WPARAM>,
    P2: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("user32.dll" "system" fn DefDlgProcA(hdlg : super::super::Foundation:: HWND, msg : u32, wparam : super::super::Foundation:: WPARAM, lparam : super::super::Foundation:: LPARAM) -> super::super::Foundation:: LRESULT);
    DefDlgProcA(hdlg.param().abi(), msg, wparam.param().abi(), lparam.param().abi())
}
#[inline]
pub unsafe fn DefDlgProcW<P0, P1, P2>(hdlg: P0, msg: u32, wparam: P1, lparam: P2) -> super::super::Foundation::LRESULT
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::WPARAM>,
    P2: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("user32.dll" "system" fn DefDlgProcW(hdlg : super::super::Foundation:: HWND, msg : u32, wparam : super::super::Foundation:: WPARAM, lparam : super::super::Foundation:: LPARAM) -> super::super::Foundation:: LRESULT);
    DefDlgProcW(hdlg.param().abi(), msg, wparam.param().abi(), lparam.param().abi())
}
#[inline]
pub unsafe fn DefFrameProcA<P0, P1, P2, P3>(hwnd: P0, hwndmdiclient: P1, umsg: u32, wparam: P2, lparam: P3) -> super::super::Foundation::LRESULT
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::HWND>,
    P2: windows_core::Param<super::super::Foundation::WPARAM>,
    P3: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("user32.dll" "system" fn DefFrameProcA(hwnd : super::super::Foundation:: HWND, hwndmdiclient : super::super::Foundation:: HWND, umsg : u32, wparam : super::super::Foundation:: WPARAM, lparam : super::super::Foundation:: LPARAM) -> super::super::Foundation:: LRESULT);
    DefFrameProcA(hwnd.param().abi(), hwndmdiclient.param().abi(), umsg, wparam.param().abi(), lparam.param().abi())
}
#[inline]
pub unsafe fn DefFrameProcW<P0, P1, P2, P3>(hwnd: P0, hwndmdiclient: P1, umsg: u32, wparam: P2, lparam: P3) -> super::super::Foundation::LRESULT
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::HWND>,
    P2: windows_core::Param<super::super::Foundation::WPARAM>,
    P3: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("user32.dll" "system" fn DefFrameProcW(hwnd : super::super::Foundation:: HWND, hwndmdiclient : super::super::Foundation:: HWND, umsg : u32, wparam : super::super::Foundation:: WPARAM, lparam : super::super::Foundation:: LPARAM) -> super::super::Foundation:: LRESULT);
    DefFrameProcW(hwnd.param().abi(), hwndmdiclient.param().abi(), umsg, wparam.param().abi(), lparam.param().abi())
}
#[inline]
pub unsafe fn DefMDIChildProcA<P0, P1, P2>(hwnd: P0, umsg: u32, wparam: P1, lparam: P2) -> super::super::Foundation::LRESULT
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::WPARAM>,
    P2: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("user32.dll" "system" fn DefMDIChildProcA(hwnd : super::super::Foundation:: HWND, umsg : u32, wparam : super::super::Foundation:: WPARAM, lparam : super::super::Foundation:: LPARAM) -> super::super::Foundation:: LRESULT);
    DefMDIChildProcA(hwnd.param().abi(), umsg, wparam.param().abi(), lparam.param().abi())
}
#[inline]
pub unsafe fn DefMDIChildProcW<P0, P1, P2>(hwnd: P0, umsg: u32, wparam: P1, lparam: P2) -> super::super::Foundation::LRESULT
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::WPARAM>,
    P2: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("user32.dll" "system" fn DefMDIChildProcW(hwnd : super::super::Foundation:: HWND, umsg : u32, wparam : super::super::Foundation:: WPARAM, lparam : super::super::Foundation:: LPARAM) -> super::super::Foundation:: LRESULT);
    DefMDIChildProcW(hwnd.param().abi(), umsg, wparam.param().abi(), lparam.param().abi())
}
#[inline]
pub unsafe fn DefWindowProcA<P0, P1, P2>(hwnd: P0, msg: u32, wparam: P1, lparam: P2) -> super::super::Foundation::LRESULT
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::WPARAM>,
    P2: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("user32.dll" "system" fn DefWindowProcA(hwnd : super::super::Foundation:: HWND, msg : u32, wparam : super::super::Foundation:: WPARAM, lparam : super::super::Foundation:: LPARAM) -> super::super::Foundation:: LRESULT);
    DefWindowProcA(hwnd.param().abi(), msg, wparam.param().abi(), lparam.param().abi())
}
#[inline]
pub unsafe fn DefWindowProcW<P0, P1, P2>(hwnd: P0, msg: u32, wparam: P1, lparam: P2) -> super::super::Foundation::LRESULT
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::WPARAM>,
    P2: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("user32.dll" "system" fn DefWindowProcW(hwnd : super::super::Foundation:: HWND, msg : u32, wparam : super::super::Foundation:: WPARAM, lparam : super::super::Foundation:: LPARAM) -> super::super::Foundation:: LRESULT);
    DefWindowProcW(hwnd.param().abi(), msg, wparam.param().abi(), lparam.param().abi())
}
#[inline]
pub unsafe fn DeferWindowPos<P0, P1, P2>(hwinposinfo: P0, hwnd: P1, hwndinsertafter: P2, x: i32, y: i32, cx: i32, cy: i32, uflags: SET_WINDOW_POS_FLAGS) -> windows_core::Result<HDWP>
where
    P0: windows_core::Param<HDWP>,
    P1: windows_core::Param<super::super::Foundation::HWND>,
    P2: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn DeferWindowPos(hwinposinfo : HDWP, hwnd : super::super::Foundation:: HWND, hwndinsertafter : super::super::Foundation:: HWND, x : i32, y : i32, cx : i32, cy : i32, uflags : SET_WINDOW_POS_FLAGS) -> HDWP);
    let result__ = DeferWindowPos(hwinposinfo.param().abi(), hwnd.param().abi(), hwndinsertafter.param().abi(), x, y, cx, cy, uflags);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn DeleteMenu<P0>(hmenu: P0, uposition: u32, uflags: MENU_ITEM_FLAGS) -> windows_core::Result<()>
where
    P0: windows_core::Param<HMENU>,
{
    windows_targets::link!("user32.dll" "system" fn DeleteMenu(hmenu : HMENU, uposition : u32, uflags : MENU_ITEM_FLAGS) -> super::super::Foundation:: BOOL);
    DeleteMenu(hmenu.param().abi(), uposition, uflags).ok()
}
#[inline]
pub unsafe fn DeregisterShellHookWindow<P0>(hwnd: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn DeregisterShellHookWindow(hwnd : super::super::Foundation:: HWND) -> super::super::Foundation:: BOOL);
    DeregisterShellHookWindow(hwnd.param().abi())
}
#[inline]
pub unsafe fn DestroyAcceleratorTable<P0>(haccel: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<HACCEL>,
{
    windows_targets::link!("user32.dll" "system" fn DestroyAcceleratorTable(haccel : HACCEL) -> super::super::Foundation:: BOOL);
    DestroyAcceleratorTable(haccel.param().abi())
}
#[inline]
pub unsafe fn DestroyCaret() -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn DestroyCaret() -> super::super::Foundation:: BOOL);
    DestroyCaret().ok()
}
#[inline]
pub unsafe fn DestroyCursor<P0>(hcursor: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<HCURSOR>,
{
    windows_targets::link!("user32.dll" "system" fn DestroyCursor(hcursor : HCURSOR) -> super::super::Foundation:: BOOL);
    DestroyCursor(hcursor.param().abi()).ok()
}
#[inline]
pub unsafe fn DestroyIcon<P0>(hicon: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<HICON>,
{
    windows_targets::link!("user32.dll" "system" fn DestroyIcon(hicon : HICON) -> super::super::Foundation:: BOOL);
    DestroyIcon(hicon.param().abi()).ok()
}
#[inline]
pub unsafe fn DestroyIndexedResults<P0>(resourceuri: P0, qualifiers: Option<&[IndexedResourceQualifier]>)
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mrmsupport.dll" "system" fn DestroyIndexedResults(resourceuri : windows_core::PCWSTR, qualifiercount : u32, qualifiers : *const IndexedResourceQualifier));
    DestroyIndexedResults(resourceuri.param().abi(), qualifiers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(qualifiers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
}
#[inline]
pub unsafe fn DestroyMenu<P0>(hmenu: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<HMENU>,
{
    windows_targets::link!("user32.dll" "system" fn DestroyMenu(hmenu : HMENU) -> super::super::Foundation:: BOOL);
    DestroyMenu(hmenu.param().abi()).ok()
}
#[inline]
pub unsafe fn DestroyResourceIndexer(resourceindexer: Option<*const core::ffi::c_void>) {
    windows_targets::link!("mrmsupport.dll" "system" fn DestroyResourceIndexer(resourceindexer : *const core::ffi::c_void));
    DestroyResourceIndexer(core::mem::transmute(resourceindexer.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn DestroyWindow<P0>(hwnd: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn DestroyWindow(hwnd : super::super::Foundation:: HWND) -> super::super::Foundation:: BOOL);
    DestroyWindow(hwnd.param().abi()).ok()
}
#[inline]
pub unsafe fn DialogBoxIndirectParamA<P0, P1, P2>(hinstance: P0, hdialogtemplate: *const DLGTEMPLATE, hwndparent: P1, lpdialogfunc: DLGPROC, dwinitparam: P2) -> isize
where
    P0: windows_core::Param<super::super::Foundation::HINSTANCE>,
    P1: windows_core::Param<super::super::Foundation::HWND>,
    P2: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("user32.dll" "system" fn DialogBoxIndirectParamA(hinstance : super::super::Foundation:: HINSTANCE, hdialogtemplate : *const DLGTEMPLATE, hwndparent : super::super::Foundation:: HWND, lpdialogfunc : DLGPROC, dwinitparam : super::super::Foundation:: LPARAM) -> isize);
    DialogBoxIndirectParamA(hinstance.param().abi(), hdialogtemplate, hwndparent.param().abi(), lpdialogfunc, dwinitparam.param().abi())
}
#[inline]
pub unsafe fn DialogBoxIndirectParamW<P0, P1, P2>(hinstance: P0, hdialogtemplate: *const DLGTEMPLATE, hwndparent: P1, lpdialogfunc: DLGPROC, dwinitparam: P2) -> isize
where
    P0: windows_core::Param<super::super::Foundation::HINSTANCE>,
    P1: windows_core::Param<super::super::Foundation::HWND>,
    P2: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("user32.dll" "system" fn DialogBoxIndirectParamW(hinstance : super::super::Foundation:: HINSTANCE, hdialogtemplate : *const DLGTEMPLATE, hwndparent : super::super::Foundation:: HWND, lpdialogfunc : DLGPROC, dwinitparam : super::super::Foundation:: LPARAM) -> isize);
    DialogBoxIndirectParamW(hinstance.param().abi(), hdialogtemplate, hwndparent.param().abi(), lpdialogfunc, dwinitparam.param().abi())
}
#[inline]
pub unsafe fn DialogBoxParamA<P0, P1, P2, P3>(hinstance: P0, lptemplatename: P1, hwndparent: P2, lpdialogfunc: DLGPROC, dwinitparam: P3) -> isize
where
    P0: windows_core::Param<super::super::Foundation::HINSTANCE>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<super::super::Foundation::HWND>,
    P3: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("user32.dll" "system" fn DialogBoxParamA(hinstance : super::super::Foundation:: HINSTANCE, lptemplatename : windows_core::PCSTR, hwndparent : super::super::Foundation:: HWND, lpdialogfunc : DLGPROC, dwinitparam : super::super::Foundation:: LPARAM) -> isize);
    DialogBoxParamA(hinstance.param().abi(), lptemplatename.param().abi(), hwndparent.param().abi(), lpdialogfunc, dwinitparam.param().abi())
}
#[inline]
pub unsafe fn DialogBoxParamW<P0, P1, P2, P3>(hinstance: P0, lptemplatename: P1, hwndparent: P2, lpdialogfunc: DLGPROC, dwinitparam: P3) -> isize
where
    P0: windows_core::Param<super::super::Foundation::HINSTANCE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::super::Foundation::HWND>,
    P3: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("user32.dll" "system" fn DialogBoxParamW(hinstance : super::super::Foundation:: HINSTANCE, lptemplatename : windows_core::PCWSTR, hwndparent : super::super::Foundation:: HWND, lpdialogfunc : DLGPROC, dwinitparam : super::super::Foundation:: LPARAM) -> isize);
    DialogBoxParamW(hinstance.param().abi(), lptemplatename.param().abi(), hwndparent.param().abi(), lpdialogfunc, dwinitparam.param().abi())
}
#[inline]
pub unsafe fn DisableProcessWindowsGhosting() {
    windows_targets::link!("user32.dll" "system" fn DisableProcessWindowsGhosting());
    DisableProcessWindowsGhosting()
}
#[inline]
pub unsafe fn DispatchMessageA(lpmsg: *const MSG) -> super::super::Foundation::LRESULT {
    windows_targets::link!("user32.dll" "system" fn DispatchMessageA(lpmsg : *const MSG) -> super::super::Foundation:: LRESULT);
    DispatchMessageA(lpmsg)
}
#[inline]
pub unsafe fn DispatchMessageW(lpmsg: *const MSG) -> super::super::Foundation::LRESULT {
    windows_targets::link!("user32.dll" "system" fn DispatchMessageW(lpmsg : *const MSG) -> super::super::Foundation:: LRESULT);
    DispatchMessageW(lpmsg)
}
#[inline]
pub unsafe fn DragObject<P0, P1, P2>(hwndparent: P0, hwndfrom: P1, fmt: u32, data: usize, hcur: P2) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::HWND>,
    P2: windows_core::Param<HCURSOR>,
{
    windows_targets::link!("user32.dll" "system" fn DragObject(hwndparent : super::super::Foundation:: HWND, hwndfrom : super::super::Foundation:: HWND, fmt : u32, data : usize, hcur : HCURSOR) -> u32);
    DragObject(hwndparent.param().abi(), hwndfrom.param().abi(), fmt, data, hcur.param().abi())
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DrawIcon<P0, P1>(hdc: P0, x: i32, y: i32, hicon: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Graphics::Gdi::HDC>,
    P1: windows_core::Param<HICON>,
{
    windows_targets::link!("user32.dll" "system" fn DrawIcon(hdc : super::super::Graphics::Gdi:: HDC, x : i32, y : i32, hicon : HICON) -> super::super::Foundation:: BOOL);
    DrawIcon(hdc.param().abi(), x, y, hicon.param().abi()).ok()
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DrawIconEx<P0, P1, P2>(hdc: P0, xleft: i32, ytop: i32, hicon: P1, cxwidth: i32, cywidth: i32, istepifanicur: u32, hbrflickerfreedraw: P2, diflags: DI_FLAGS) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Graphics::Gdi::HDC>,
    P1: windows_core::Param<HICON>,
    P2: windows_core::Param<super::super::Graphics::Gdi::HBRUSH>,
{
    windows_targets::link!("user32.dll" "system" fn DrawIconEx(hdc : super::super::Graphics::Gdi:: HDC, xleft : i32, ytop : i32, hicon : HICON, cxwidth : i32, cywidth : i32, istepifanicur : u32, hbrflickerfreedraw : super::super::Graphics::Gdi:: HBRUSH, diflags : DI_FLAGS) -> super::super::Foundation:: BOOL);
    DrawIconEx(hdc.param().abi(), xleft, ytop, hicon.param().abi(), cxwidth, cywidth, istepifanicur, hbrflickerfreedraw.param().abi(), diflags).ok()
}
#[inline]
pub unsafe fn DrawMenuBar<P0>(hwnd: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn DrawMenuBar(hwnd : super::super::Foundation:: HWND) -> super::super::Foundation:: BOOL);
    DrawMenuBar(hwnd.param().abi()).ok()
}
#[inline]
pub unsafe fn EnableMenuItem<P0>(hmenu: P0, uidenableitem: u32, uenable: MENU_ITEM_FLAGS) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<HMENU>,
{
    windows_targets::link!("user32.dll" "system" fn EnableMenuItem(hmenu : HMENU, uidenableitem : u32, uenable : MENU_ITEM_FLAGS) -> super::super::Foundation:: BOOL);
    EnableMenuItem(hmenu.param().abi(), uidenableitem, uenable)
}
#[inline]
pub unsafe fn EndDeferWindowPos<P0>(hwinposinfo: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<HDWP>,
{
    windows_targets::link!("user32.dll" "system" fn EndDeferWindowPos(hwinposinfo : HDWP) -> super::super::Foundation:: BOOL);
    EndDeferWindowPos(hwinposinfo.param().abi()).ok()
}
#[inline]
pub unsafe fn EndDialog<P0>(hdlg: P0, nresult: isize) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn EndDialog(hdlg : super::super::Foundation:: HWND, nresult : isize) -> super::super::Foundation:: BOOL);
    EndDialog(hdlg.param().abi(), nresult).ok()
}
#[inline]
pub unsafe fn EndMenu() -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn EndMenu() -> super::super::Foundation:: BOOL);
    EndMenu().ok()
}
#[inline]
pub unsafe fn EnumChildWindows<P0, P1>(hwndparent: P0, lpenumfunc: WNDENUMPROC, lparam: P1) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("user32.dll" "system" fn EnumChildWindows(hwndparent : super::super::Foundation:: HWND, lpenumfunc : WNDENUMPROC, lparam : super::super::Foundation:: LPARAM) -> super::super::Foundation:: BOOL);
    EnumChildWindows(hwndparent.param().abi(), lpenumfunc, lparam.param().abi())
}
#[inline]
pub unsafe fn EnumPropsA<P0>(hwnd: P0, lpenumfunc: PROPENUMPROCA) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn EnumPropsA(hwnd : super::super::Foundation:: HWND, lpenumfunc : PROPENUMPROCA) -> i32);
    EnumPropsA(hwnd.param().abi(), lpenumfunc)
}
#[inline]
pub unsafe fn EnumPropsExA<P0, P1>(hwnd: P0, lpenumfunc: PROPENUMPROCEXA, lparam: P1) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("user32.dll" "system" fn EnumPropsExA(hwnd : super::super::Foundation:: HWND, lpenumfunc : PROPENUMPROCEXA, lparam : super::super::Foundation:: LPARAM) -> i32);
    EnumPropsExA(hwnd.param().abi(), lpenumfunc, lparam.param().abi())
}
#[inline]
pub unsafe fn EnumPropsExW<P0, P1>(hwnd: P0, lpenumfunc: PROPENUMPROCEXW, lparam: P1) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("user32.dll" "system" fn EnumPropsExW(hwnd : super::super::Foundation:: HWND, lpenumfunc : PROPENUMPROCEXW, lparam : super::super::Foundation:: LPARAM) -> i32);
    EnumPropsExW(hwnd.param().abi(), lpenumfunc, lparam.param().abi())
}
#[inline]
pub unsafe fn EnumPropsW<P0>(hwnd: P0, lpenumfunc: PROPENUMPROCW) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn EnumPropsW(hwnd : super::super::Foundation:: HWND, lpenumfunc : PROPENUMPROCW) -> i32);
    EnumPropsW(hwnd.param().abi(), lpenumfunc)
}
#[inline]
pub unsafe fn EnumThreadWindows<P0>(dwthreadid: u32, lpfn: WNDENUMPROC, lparam: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("user32.dll" "system" fn EnumThreadWindows(dwthreadid : u32, lpfn : WNDENUMPROC, lparam : super::super::Foundation:: LPARAM) -> super::super::Foundation:: BOOL);
    EnumThreadWindows(dwthreadid, lpfn, lparam.param().abi())
}
#[inline]
pub unsafe fn EnumWindows<P0>(lpenumfunc: WNDENUMPROC, lparam: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("user32.dll" "system" fn EnumWindows(lpenumfunc : WNDENUMPROC, lparam : super::super::Foundation:: LPARAM) -> super::super::Foundation:: BOOL);
    EnumWindows(lpenumfunc, lparam.param().abi()).ok()
}
#[inline]
pub unsafe fn FindWindowA<P0, P1>(lpclassname: P0, lpwindowname: P1) -> super::super::Foundation::HWND
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("user32.dll" "system" fn FindWindowA(lpclassname : windows_core::PCSTR, lpwindowname : windows_core::PCSTR) -> super::super::Foundation:: HWND);
    FindWindowA(lpclassname.param().abi(), lpwindowname.param().abi())
}
#[inline]
pub unsafe fn FindWindowExA<P0, P1, P2, P3>(hwndparent: P0, hwndchildafter: P1, lpszclass: P2, lpszwindow: P3) -> super::super::Foundation::HWND
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::HWND>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("user32.dll" "system" fn FindWindowExA(hwndparent : super::super::Foundation:: HWND, hwndchildafter : super::super::Foundation:: HWND, lpszclass : windows_core::PCSTR, lpszwindow : windows_core::PCSTR) -> super::super::Foundation:: HWND);
    FindWindowExA(hwndparent.param().abi(), hwndchildafter.param().abi(), lpszclass.param().abi(), lpszwindow.param().abi())
}
#[inline]
pub unsafe fn FindWindowExW<P0, P1, P2, P3>(hwndparent: P0, hwndchildafter: P1, lpszclass: P2, lpszwindow: P3) -> super::super::Foundation::HWND
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::HWND>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("user32.dll" "system" fn FindWindowExW(hwndparent : super::super::Foundation:: HWND, hwndchildafter : super::super::Foundation:: HWND, lpszclass : windows_core::PCWSTR, lpszwindow : windows_core::PCWSTR) -> super::super::Foundation:: HWND);
    FindWindowExW(hwndparent.param().abi(), hwndchildafter.param().abi(), lpszclass.param().abi(), lpszwindow.param().abi())
}
#[inline]
pub unsafe fn FindWindowW<P0, P1>(lpclassname: P0, lpwindowname: P1) -> super::super::Foundation::HWND
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("user32.dll" "system" fn FindWindowW(lpclassname : windows_core::PCWSTR, lpwindowname : windows_core::PCWSTR) -> super::super::Foundation:: HWND);
    FindWindowW(lpclassname.param().abi(), lpwindowname.param().abi())
}
#[inline]
pub unsafe fn FlashWindow<P0, P1>(hwnd: P0, binvert: P1) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("user32.dll" "system" fn FlashWindow(hwnd : super::super::Foundation:: HWND, binvert : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    FlashWindow(hwnd.param().abi(), binvert.param().abi())
}
#[inline]
pub unsafe fn FlashWindowEx(pfwi: *const FLASHWINFO) -> super::super::Foundation::BOOL {
    windows_targets::link!("user32.dll" "system" fn FlashWindowEx(pfwi : *const FLASHWINFO) -> super::super::Foundation:: BOOL);
    FlashWindowEx(pfwi)
}
#[inline]
pub unsafe fn GetAltTabInfoA<P0>(hwnd: P0, iitem: i32, pati: *mut ALTTABINFO, pszitemtext: Option<&mut [u8]>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn GetAltTabInfoA(hwnd : super::super::Foundation:: HWND, iitem : i32, pati : *mut ALTTABINFO, pszitemtext : windows_core::PSTR, cchitemtext : u32) -> super::super::Foundation:: BOOL);
    GetAltTabInfoA(hwnd.param().abi(), iitem, pati, core::mem::transmute(pszitemtext.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pszitemtext.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())).ok()
}
#[inline]
pub unsafe fn GetAltTabInfoW<P0>(hwnd: P0, iitem: i32, pati: *mut ALTTABINFO, pszitemtext: Option<&mut [u16]>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn GetAltTabInfoW(hwnd : super::super::Foundation:: HWND, iitem : i32, pati : *mut ALTTABINFO, pszitemtext : windows_core::PWSTR, cchitemtext : u32) -> super::super::Foundation:: BOOL);
    GetAltTabInfoW(hwnd.param().abi(), iitem, pati, core::mem::transmute(pszitemtext.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pszitemtext.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())).ok()
}
#[inline]
pub unsafe fn GetAncestor<P0>(hwnd: P0, gaflags: GET_ANCESTOR_FLAGS) -> super::super::Foundation::HWND
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn GetAncestor(hwnd : super::super::Foundation:: HWND, gaflags : GET_ANCESTOR_FLAGS) -> super::super::Foundation:: HWND);
    GetAncestor(hwnd.param().abi(), gaflags)
}
#[inline]
pub unsafe fn GetCaretBlinkTime() -> u32 {
    windows_targets::link!("user32.dll" "system" fn GetCaretBlinkTime() -> u32);
    GetCaretBlinkTime()
}
#[inline]
pub unsafe fn GetCaretPos(lppoint: *mut super::super::Foundation::POINT) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn GetCaretPos(lppoint : *mut super::super::Foundation:: POINT) -> super::super::Foundation:: BOOL);
    GetCaretPos(lppoint).ok()
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetClassInfoA<P0, P1>(hinstance: P0, lpclassname: P1, lpwndclass: *mut WNDCLASSA) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HINSTANCE>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("user32.dll" "system" fn GetClassInfoA(hinstance : super::super::Foundation:: HINSTANCE, lpclassname : windows_core::PCSTR, lpwndclass : *mut WNDCLASSA) -> super::super::Foundation:: BOOL);
    GetClassInfoA(hinstance.param().abi(), lpclassname.param().abi(), lpwndclass).ok()
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetClassInfoExA<P0, P1>(hinstance: P0, lpszclass: P1, lpwcx: *mut WNDCLASSEXA) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HINSTANCE>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("user32.dll" "system" fn GetClassInfoExA(hinstance : super::super::Foundation:: HINSTANCE, lpszclass : windows_core::PCSTR, lpwcx : *mut WNDCLASSEXA) -> super::super::Foundation:: BOOL);
    GetClassInfoExA(hinstance.param().abi(), lpszclass.param().abi(), lpwcx).ok()
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetClassInfoExW<P0, P1>(hinstance: P0, lpszclass: P1, lpwcx: *mut WNDCLASSEXW) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HINSTANCE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("user32.dll" "system" fn GetClassInfoExW(hinstance : super::super::Foundation:: HINSTANCE, lpszclass : windows_core::PCWSTR, lpwcx : *mut WNDCLASSEXW) -> super::super::Foundation:: BOOL);
    GetClassInfoExW(hinstance.param().abi(), lpszclass.param().abi(), lpwcx).ok()
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetClassInfoW<P0, P1>(hinstance: P0, lpclassname: P1, lpwndclass: *mut WNDCLASSW) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HINSTANCE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("user32.dll" "system" fn GetClassInfoW(hinstance : super::super::Foundation:: HINSTANCE, lpclassname : windows_core::PCWSTR, lpwndclass : *mut WNDCLASSW) -> super::super::Foundation:: BOOL);
    GetClassInfoW(hinstance.param().abi(), lpclassname.param().abi(), lpwndclass).ok()
}
#[inline]
pub unsafe fn GetClassLongA<P0>(hwnd: P0, nindex: GET_CLASS_LONG_INDEX) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn GetClassLongA(hwnd : super::super::Foundation:: HWND, nindex : GET_CLASS_LONG_INDEX) -> u32);
    GetClassLongA(hwnd.param().abi(), nindex)
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn GetClassLongPtrA<P0>(hwnd: P0, nindex: GET_CLASS_LONG_INDEX) -> usize
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn GetClassLongPtrA(hwnd : super::super::Foundation:: HWND, nindex : GET_CLASS_LONG_INDEX) -> usize);
    GetClassLongPtrA(hwnd.param().abi(), nindex)
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn GetClassLongPtrW<P0>(hwnd: P0, nindex: GET_CLASS_LONG_INDEX) -> usize
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn GetClassLongPtrW(hwnd : super::super::Foundation:: HWND, nindex : GET_CLASS_LONG_INDEX) -> usize);
    GetClassLongPtrW(hwnd.param().abi(), nindex)
}
#[inline]
pub unsafe fn GetClassLongW<P0>(hwnd: P0, nindex: GET_CLASS_LONG_INDEX) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn GetClassLongW(hwnd : super::super::Foundation:: HWND, nindex : GET_CLASS_LONG_INDEX) -> u32);
    GetClassLongW(hwnd.param().abi(), nindex)
}
#[inline]
pub unsafe fn GetClassNameA<P0>(hwnd: P0, lpclassname: &mut [u8]) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn GetClassNameA(hwnd : super::super::Foundation:: HWND, lpclassname : windows_core::PSTR, nmaxcount : i32) -> i32);
    GetClassNameA(hwnd.param().abi(), core::mem::transmute(lpclassname.as_ptr()), lpclassname.len().try_into().unwrap())
}
#[inline]
pub unsafe fn GetClassNameW<P0>(hwnd: P0, lpclassname: &mut [u16]) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn GetClassNameW(hwnd : super::super::Foundation:: HWND, lpclassname : windows_core::PWSTR, nmaxcount : i32) -> i32);
    GetClassNameW(hwnd.param().abi(), core::mem::transmute(lpclassname.as_ptr()), lpclassname.len().try_into().unwrap())
}
#[inline]
pub unsafe fn GetClassWord<P0>(hwnd: P0, nindex: i32) -> u16
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn GetClassWord(hwnd : super::super::Foundation:: HWND, nindex : i32) -> u16);
    GetClassWord(hwnd.param().abi(), nindex)
}
#[inline]
pub unsafe fn GetClientRect<P0>(hwnd: P0, lprect: *mut super::super::Foundation::RECT) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn GetClientRect(hwnd : super::super::Foundation:: HWND, lprect : *mut super::super::Foundation:: RECT) -> super::super::Foundation:: BOOL);
    GetClientRect(hwnd.param().abi(), lprect).ok()
}
#[inline]
pub unsafe fn GetClipCursor(lprect: *mut super::super::Foundation::RECT) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn GetClipCursor(lprect : *mut super::super::Foundation:: RECT) -> super::super::Foundation:: BOOL);
    GetClipCursor(lprect).ok()
}
#[inline]
pub unsafe fn GetCursor() -> HCURSOR {
    windows_targets::link!("user32.dll" "system" fn GetCursor() -> HCURSOR);
    GetCursor()
}
#[inline]
pub unsafe fn GetCursorInfo(pci: *mut CURSORINFO) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn GetCursorInfo(pci : *mut CURSORINFO) -> super::super::Foundation:: BOOL);
    GetCursorInfo(pci).ok()
}
#[inline]
pub unsafe fn GetCursorPos(lppoint: *mut super::super::Foundation::POINT) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn GetCursorPos(lppoint : *mut super::super::Foundation:: POINT) -> super::super::Foundation:: BOOL);
    GetCursorPos(lppoint).ok()
}
#[inline]
pub unsafe fn GetDesktopWindow() -> super::super::Foundation::HWND {
    windows_targets::link!("user32.dll" "system" fn GetDesktopWindow() -> super::super::Foundation:: HWND);
    GetDesktopWindow()
}
#[inline]
pub unsafe fn GetDialogBaseUnits() -> i32 {
    windows_targets::link!("user32.dll" "system" fn GetDialogBaseUnits() -> i32);
    GetDialogBaseUnits()
}
#[inline]
pub unsafe fn GetDlgCtrlID<P0>(hwnd: P0) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn GetDlgCtrlID(hwnd : super::super::Foundation:: HWND) -> i32);
    GetDlgCtrlID(hwnd.param().abi())
}
#[inline]
pub unsafe fn GetDlgItem<P0>(hdlg: P0, niddlgitem: i32) -> super::super::Foundation::HWND
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn GetDlgItem(hdlg : super::super::Foundation:: HWND, niddlgitem : i32) -> super::super::Foundation:: HWND);
    GetDlgItem(hdlg.param().abi(), niddlgitem)
}
#[inline]
pub unsafe fn GetDlgItemInt<P0, P1>(hdlg: P0, niddlgitem: i32, lptranslated: Option<*mut super::super::Foundation::BOOL>, bsigned: P1) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("user32.dll" "system" fn GetDlgItemInt(hdlg : super::super::Foundation:: HWND, niddlgitem : i32, lptranslated : *mut super::super::Foundation:: BOOL, bsigned : super::super::Foundation:: BOOL) -> u32);
    GetDlgItemInt(hdlg.param().abi(), niddlgitem, core::mem::transmute(lptranslated.unwrap_or(std::ptr::null_mut())), bsigned.param().abi())
}
#[inline]
pub unsafe fn GetDlgItemTextA<P0>(hdlg: P0, niddlgitem: i32, lpstring: &mut [u8]) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn GetDlgItemTextA(hdlg : super::super::Foundation:: HWND, niddlgitem : i32, lpstring : windows_core::PSTR, cchmax : i32) -> u32);
    GetDlgItemTextA(hdlg.param().abi(), niddlgitem, core::mem::transmute(lpstring.as_ptr()), lpstring.len().try_into().unwrap())
}
#[inline]
pub unsafe fn GetDlgItemTextW<P0>(hdlg: P0, niddlgitem: i32, lpstring: &mut [u16]) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn GetDlgItemTextW(hdlg : super::super::Foundation:: HWND, niddlgitem : i32, lpstring : windows_core::PWSTR, cchmax : i32) -> u32);
    GetDlgItemTextW(hdlg.param().abi(), niddlgitem, core::mem::transmute(lpstring.as_ptr()), lpstring.len().try_into().unwrap())
}
#[inline]
pub unsafe fn GetForegroundWindow() -> super::super::Foundation::HWND {
    windows_targets::link!("user32.dll" "system" fn GetForegroundWindow() -> super::super::Foundation:: HWND);
    GetForegroundWindow()
}
#[inline]
pub unsafe fn GetGUIThreadInfo(idthread: u32, pgui: *mut GUITHREADINFO) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn GetGUIThreadInfo(idthread : u32, pgui : *mut GUITHREADINFO) -> super::super::Foundation:: BOOL);
    GetGUIThreadInfo(idthread, pgui).ok()
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetIconInfo<P0>(hicon: P0, piconinfo: *mut ICONINFO) -> windows_core::Result<()>
where
    P0: windows_core::Param<HICON>,
{
    windows_targets::link!("user32.dll" "system" fn GetIconInfo(hicon : HICON, piconinfo : *mut ICONINFO) -> super::super::Foundation:: BOOL);
    GetIconInfo(hicon.param().abi(), piconinfo).ok()
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetIconInfoExA<P0>(hicon: P0, piconinfo: *mut ICONINFOEXA) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<HICON>,
{
    windows_targets::link!("user32.dll" "system" fn GetIconInfoExA(hicon : HICON, piconinfo : *mut ICONINFOEXA) -> super::super::Foundation:: BOOL);
    GetIconInfoExA(hicon.param().abi(), piconinfo)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetIconInfoExW<P0>(hicon: P0, piconinfo: *mut ICONINFOEXW) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<HICON>,
{
    windows_targets::link!("user32.dll" "system" fn GetIconInfoExW(hicon : HICON, piconinfo : *mut ICONINFOEXW) -> super::super::Foundation:: BOOL);
    GetIconInfoExW(hicon.param().abi(), piconinfo)
}
#[inline]
pub unsafe fn GetInputState() -> super::super::Foundation::BOOL {
    windows_targets::link!("user32.dll" "system" fn GetInputState() -> super::super::Foundation:: BOOL);
    GetInputState()
}
#[inline]
pub unsafe fn GetLastActivePopup<P0>(hwnd: P0) -> super::super::Foundation::HWND
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn GetLastActivePopup(hwnd : super::super::Foundation:: HWND) -> super::super::Foundation:: HWND);
    GetLastActivePopup(hwnd.param().abi())
}
#[inline]
pub unsafe fn GetLayeredWindowAttributes<P0>(hwnd: P0, pcrkey: Option<*mut super::super::Foundation::COLORREF>, pbalpha: Option<*mut u8>, pdwflags: Option<*mut LAYERED_WINDOW_ATTRIBUTES_FLAGS>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn GetLayeredWindowAttributes(hwnd : super::super::Foundation:: HWND, pcrkey : *mut super::super::Foundation:: COLORREF, pbalpha : *mut u8, pdwflags : *mut LAYERED_WINDOW_ATTRIBUTES_FLAGS) -> super::super::Foundation:: BOOL);
    GetLayeredWindowAttributes(hwnd.param().abi(), core::mem::transmute(pcrkey.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pbalpha.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pdwflags.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn GetMenu<P0>(hwnd: P0) -> HMENU
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn GetMenu(hwnd : super::super::Foundation:: HWND) -> HMENU);
    GetMenu(hwnd.param().abi())
}
#[inline]
pub unsafe fn GetMenuBarInfo<P0>(hwnd: P0, idobject: OBJECT_IDENTIFIER, iditem: i32, pmbi: *mut MENUBARINFO) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn GetMenuBarInfo(hwnd : super::super::Foundation:: HWND, idobject : OBJECT_IDENTIFIER, iditem : i32, pmbi : *mut MENUBARINFO) -> super::super::Foundation:: BOOL);
    GetMenuBarInfo(hwnd.param().abi(), idobject, iditem, pmbi).ok()
}
#[inline]
pub unsafe fn GetMenuCheckMarkDimensions() -> i32 {
    windows_targets::link!("user32.dll" "system" fn GetMenuCheckMarkDimensions() -> i32);
    GetMenuCheckMarkDimensions()
}
#[inline]
pub unsafe fn GetMenuDefaultItem<P0>(hmenu: P0, fbypos: u32, gmdiflags: GET_MENU_DEFAULT_ITEM_FLAGS) -> u32
where
    P0: windows_core::Param<HMENU>,
{
    windows_targets::link!("user32.dll" "system" fn GetMenuDefaultItem(hmenu : HMENU, fbypos : u32, gmdiflags : GET_MENU_DEFAULT_ITEM_FLAGS) -> u32);
    GetMenuDefaultItem(hmenu.param().abi(), fbypos, gmdiflags)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetMenuInfo<P0>(param0: P0, param1: *mut MENUINFO) -> windows_core::Result<()>
where
    P0: windows_core::Param<HMENU>,
{
    windows_targets::link!("user32.dll" "system" fn GetMenuInfo(param0 : HMENU, param1 : *mut MENUINFO) -> super::super::Foundation:: BOOL);
    GetMenuInfo(param0.param().abi(), param1).ok()
}
#[inline]
pub unsafe fn GetMenuItemCount<P0>(hmenu: P0) -> i32
where
    P0: windows_core::Param<HMENU>,
{
    windows_targets::link!("user32.dll" "system" fn GetMenuItemCount(hmenu : HMENU) -> i32);
    GetMenuItemCount(hmenu.param().abi())
}
#[inline]
pub unsafe fn GetMenuItemID<P0>(hmenu: P0, npos: i32) -> u32
where
    P0: windows_core::Param<HMENU>,
{
    windows_targets::link!("user32.dll" "system" fn GetMenuItemID(hmenu : HMENU, npos : i32) -> u32);
    GetMenuItemID(hmenu.param().abi(), npos)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetMenuItemInfoA<P0, P1>(hmenu: P0, item: u32, fbyposition: P1, lpmii: *mut MENUITEMINFOA) -> windows_core::Result<()>
where
    P0: windows_core::Param<HMENU>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("user32.dll" "system" fn GetMenuItemInfoA(hmenu : HMENU, item : u32, fbyposition : super::super::Foundation:: BOOL, lpmii : *mut MENUITEMINFOA) -> super::super::Foundation:: BOOL);
    GetMenuItemInfoA(hmenu.param().abi(), item, fbyposition.param().abi(), lpmii).ok()
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetMenuItemInfoW<P0, P1>(hmenu: P0, item: u32, fbyposition: P1, lpmii: *mut MENUITEMINFOW) -> windows_core::Result<()>
where
    P0: windows_core::Param<HMENU>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("user32.dll" "system" fn GetMenuItemInfoW(hmenu : HMENU, item : u32, fbyposition : super::super::Foundation:: BOOL, lpmii : *mut MENUITEMINFOW) -> super::super::Foundation:: BOOL);
    GetMenuItemInfoW(hmenu.param().abi(), item, fbyposition.param().abi(), lpmii).ok()
}
#[inline]
pub unsafe fn GetMenuItemRect<P0, P1>(hwnd: P0, hmenu: P1, uitem: u32, lprcitem: *mut super::super::Foundation::RECT) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<HMENU>,
{
    windows_targets::link!("user32.dll" "system" fn GetMenuItemRect(hwnd : super::super::Foundation:: HWND, hmenu : HMENU, uitem : u32, lprcitem : *mut super::super::Foundation:: RECT) -> super::super::Foundation:: BOOL);
    GetMenuItemRect(hwnd.param().abi(), hmenu.param().abi(), uitem, lprcitem).ok()
}
#[inline]
pub unsafe fn GetMenuState<P0>(hmenu: P0, uid: u32, uflags: MENU_ITEM_FLAGS) -> u32
where
    P0: windows_core::Param<HMENU>,
{
    windows_targets::link!("user32.dll" "system" fn GetMenuState(hmenu : HMENU, uid : u32, uflags : MENU_ITEM_FLAGS) -> u32);
    GetMenuState(hmenu.param().abi(), uid, uflags)
}
#[inline]
pub unsafe fn GetMenuStringA<P0>(hmenu: P0, uiditem: u32, lpstring: Option<&mut [u8]>, flags: MENU_ITEM_FLAGS) -> i32
where
    P0: windows_core::Param<HMENU>,
{
    windows_targets::link!("user32.dll" "system" fn GetMenuStringA(hmenu : HMENU, uiditem : u32, lpstring : windows_core::PSTR, cchmax : i32, flags : MENU_ITEM_FLAGS) -> i32);
    GetMenuStringA(hmenu.param().abi(), uiditem, core::mem::transmute(lpstring.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpstring.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), flags)
}
#[inline]
pub unsafe fn GetMenuStringW<P0>(hmenu: P0, uiditem: u32, lpstring: Option<&mut [u16]>, flags: MENU_ITEM_FLAGS) -> i32
where
    P0: windows_core::Param<HMENU>,
{
    windows_targets::link!("user32.dll" "system" fn GetMenuStringW(hmenu : HMENU, uiditem : u32, lpstring : windows_core::PWSTR, cchmax : i32, flags : MENU_ITEM_FLAGS) -> i32);
    GetMenuStringW(hmenu.param().abi(), uiditem, core::mem::transmute(lpstring.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpstring.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), flags)
}
#[inline]
pub unsafe fn GetMessageA<P0>(lpmsg: *mut MSG, hwnd: P0, wmsgfiltermin: u32, wmsgfiltermax: u32) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn GetMessageA(lpmsg : *mut MSG, hwnd : super::super::Foundation:: HWND, wmsgfiltermin : u32, wmsgfiltermax : u32) -> super::super::Foundation:: BOOL);
    GetMessageA(lpmsg, hwnd.param().abi(), wmsgfiltermin, wmsgfiltermax)
}
#[inline]
pub unsafe fn GetMessageExtraInfo() -> super::super::Foundation::LPARAM {
    windows_targets::link!("user32.dll" "system" fn GetMessageExtraInfo() -> super::super::Foundation:: LPARAM);
    GetMessageExtraInfo()
}
#[inline]
pub unsafe fn GetMessagePos() -> u32 {
    windows_targets::link!("user32.dll" "system" fn GetMessagePos() -> u32);
    GetMessagePos()
}
#[inline]
pub unsafe fn GetMessageTime() -> i32 {
    windows_targets::link!("user32.dll" "system" fn GetMessageTime() -> i32);
    GetMessageTime()
}
#[inline]
pub unsafe fn GetMessageW<P0>(lpmsg: *mut MSG, hwnd: P0, wmsgfiltermin: u32, wmsgfiltermax: u32) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn GetMessageW(lpmsg : *mut MSG, hwnd : super::super::Foundation:: HWND, wmsgfiltermin : u32, wmsgfiltermax : u32) -> super::super::Foundation:: BOOL);
    GetMessageW(lpmsg, hwnd.param().abi(), wmsgfiltermin, wmsgfiltermax)
}
#[inline]
pub unsafe fn GetNextDlgGroupItem<P0, P1, P2>(hdlg: P0, hctl: P1, bprevious: P2) -> super::super::Foundation::HWND
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::HWND>,
    P2: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("user32.dll" "system" fn GetNextDlgGroupItem(hdlg : super::super::Foundation:: HWND, hctl : super::super::Foundation:: HWND, bprevious : super::super::Foundation:: BOOL) -> super::super::Foundation:: HWND);
    GetNextDlgGroupItem(hdlg.param().abi(), hctl.param().abi(), bprevious.param().abi())
}
#[inline]
pub unsafe fn GetNextDlgTabItem<P0, P1, P2>(hdlg: P0, hctl: P1, bprevious: P2) -> super::super::Foundation::HWND
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::HWND>,
    P2: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("user32.dll" "system" fn GetNextDlgTabItem(hdlg : super::super::Foundation:: HWND, hctl : super::super::Foundation:: HWND, bprevious : super::super::Foundation:: BOOL) -> super::super::Foundation:: HWND);
    GetNextDlgTabItem(hdlg.param().abi(), hctl.param().abi(), bprevious.param().abi())
}
#[inline]
pub unsafe fn GetParent<P0>(hwnd: P0) -> super::super::Foundation::HWND
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn GetParent(hwnd : super::super::Foundation:: HWND) -> super::super::Foundation:: HWND);
    GetParent(hwnd.param().abi())
}
#[inline]
pub unsafe fn GetPhysicalCursorPos(lppoint: *mut super::super::Foundation::POINT) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn GetPhysicalCursorPos(lppoint : *mut super::super::Foundation:: POINT) -> super::super::Foundation:: BOOL);
    GetPhysicalCursorPos(lppoint).ok()
}
#[inline]
pub unsafe fn GetProcessDefaultLayout(pdwdefaultlayout: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn GetProcessDefaultLayout(pdwdefaultlayout : *mut u32) -> super::super::Foundation:: BOOL);
    GetProcessDefaultLayout(pdwdefaultlayout).ok()
}
#[inline]
pub unsafe fn GetPropA<P0, P1>(hwnd: P0, lpstring: P1) -> super::super::Foundation::HANDLE
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("user32.dll" "system" fn GetPropA(hwnd : super::super::Foundation:: HWND, lpstring : windows_core::PCSTR) -> super::super::Foundation:: HANDLE);
    GetPropA(hwnd.param().abi(), lpstring.param().abi())
}
#[inline]
pub unsafe fn GetPropW<P0, P1>(hwnd: P0, lpstring: P1) -> super::super::Foundation::HANDLE
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("user32.dll" "system" fn GetPropW(hwnd : super::super::Foundation:: HWND, lpstring : windows_core::PCWSTR) -> super::super::Foundation:: HANDLE);
    GetPropW(hwnd.param().abi(), lpstring.param().abi())
}
#[inline]
pub unsafe fn GetQueueStatus(flags: QUEUE_STATUS_FLAGS) -> u32 {
    windows_targets::link!("user32.dll" "system" fn GetQueueStatus(flags : QUEUE_STATUS_FLAGS) -> u32);
    GetQueueStatus(flags)
}
#[inline]
pub unsafe fn GetScrollBarInfo<P0>(hwnd: P0, idobject: OBJECT_IDENTIFIER, psbi: *mut SCROLLBARINFO) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn GetScrollBarInfo(hwnd : super::super::Foundation:: HWND, idobject : OBJECT_IDENTIFIER, psbi : *mut SCROLLBARINFO) -> super::super::Foundation:: BOOL);
    GetScrollBarInfo(hwnd.param().abi(), idobject, psbi).ok()
}
#[inline]
pub unsafe fn GetScrollInfo<P0>(hwnd: P0, nbar: SCROLLBAR_CONSTANTS, lpsi: *mut SCROLLINFO) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn GetScrollInfo(hwnd : super::super::Foundation:: HWND, nbar : SCROLLBAR_CONSTANTS, lpsi : *mut SCROLLINFO) -> super::super::Foundation:: BOOL);
    GetScrollInfo(hwnd.param().abi(), nbar, lpsi).ok()
}
#[inline]
pub unsafe fn GetScrollPos<P0>(hwnd: P0, nbar: SCROLLBAR_CONSTANTS) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn GetScrollPos(hwnd : super::super::Foundation:: HWND, nbar : SCROLLBAR_CONSTANTS) -> i32);
    GetScrollPos(hwnd.param().abi(), nbar)
}
#[inline]
pub unsafe fn GetScrollRange<P0>(hwnd: P0, nbar: SCROLLBAR_CONSTANTS, lpminpos: *mut i32, lpmaxpos: *mut i32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn GetScrollRange(hwnd : super::super::Foundation:: HWND, nbar : SCROLLBAR_CONSTANTS, lpminpos : *mut i32, lpmaxpos : *mut i32) -> super::super::Foundation:: BOOL);
    GetScrollRange(hwnd.param().abi(), nbar, lpminpos, lpmaxpos).ok()
}
#[inline]
pub unsafe fn GetShellWindow() -> super::super::Foundation::HWND {
    windows_targets::link!("user32.dll" "system" fn GetShellWindow() -> super::super::Foundation:: HWND);
    GetShellWindow()
}
#[inline]
pub unsafe fn GetSubMenu<P0>(hmenu: P0, npos: i32) -> HMENU
where
    P0: windows_core::Param<HMENU>,
{
    windows_targets::link!("user32.dll" "system" fn GetSubMenu(hmenu : HMENU, npos : i32) -> HMENU);
    GetSubMenu(hmenu.param().abi(), npos)
}
#[inline]
pub unsafe fn GetSystemMenu<P0, P1>(hwnd: P0, brevert: P1) -> HMENU
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("user32.dll" "system" fn GetSystemMenu(hwnd : super::super::Foundation:: HWND, brevert : super::super::Foundation:: BOOL) -> HMENU);
    GetSystemMenu(hwnd.param().abi(), brevert.param().abi())
}
#[inline]
pub unsafe fn GetSystemMetrics(nindex: SYSTEM_METRICS_INDEX) -> i32 {
    windows_targets::link!("user32.dll" "system" fn GetSystemMetrics(nindex : SYSTEM_METRICS_INDEX) -> i32);
    GetSystemMetrics(nindex)
}
#[inline]
pub unsafe fn GetTitleBarInfo<P0>(hwnd: P0, pti: *mut TITLEBARINFO) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn GetTitleBarInfo(hwnd : super::super::Foundation:: HWND, pti : *mut TITLEBARINFO) -> super::super::Foundation:: BOOL);
    GetTitleBarInfo(hwnd.param().abi(), pti).ok()
}
#[inline]
pub unsafe fn GetTopWindow<P0>(hwnd: P0) -> super::super::Foundation::HWND
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn GetTopWindow(hwnd : super::super::Foundation:: HWND) -> super::super::Foundation:: HWND);
    GetTopWindow(hwnd.param().abi())
}
#[inline]
pub unsafe fn GetWindow<P0>(hwnd: P0, ucmd: GET_WINDOW_CMD) -> super::super::Foundation::HWND
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn GetWindow(hwnd : super::super::Foundation:: HWND, ucmd : GET_WINDOW_CMD) -> super::super::Foundation:: HWND);
    GetWindow(hwnd.param().abi(), ucmd)
}
#[inline]
pub unsafe fn GetWindowDisplayAffinity<P0>(hwnd: P0, pdwaffinity: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn GetWindowDisplayAffinity(hwnd : super::super::Foundation:: HWND, pdwaffinity : *mut u32) -> super::super::Foundation:: BOOL);
    GetWindowDisplayAffinity(hwnd.param().abi(), pdwaffinity).ok()
}
#[inline]
pub unsafe fn GetWindowInfo<P0>(hwnd: P0, pwi: *mut WINDOWINFO) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn GetWindowInfo(hwnd : super::super::Foundation:: HWND, pwi : *mut WINDOWINFO) -> super::super::Foundation:: BOOL);
    GetWindowInfo(hwnd.param().abi(), pwi).ok()
}
#[inline]
pub unsafe fn GetWindowLongA<P0>(hwnd: P0, nindex: WINDOW_LONG_PTR_INDEX) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn GetWindowLongA(hwnd : super::super::Foundation:: HWND, nindex : WINDOW_LONG_PTR_INDEX) -> i32);
    GetWindowLongA(hwnd.param().abi(), nindex)
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn GetWindowLongPtrA<P0>(hwnd: P0, nindex: WINDOW_LONG_PTR_INDEX) -> isize
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn GetWindowLongPtrA(hwnd : super::super::Foundation:: HWND, nindex : WINDOW_LONG_PTR_INDEX) -> isize);
    GetWindowLongPtrA(hwnd.param().abi(), nindex)
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn GetWindowLongPtrW<P0>(hwnd: P0, nindex: WINDOW_LONG_PTR_INDEX) -> isize
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn GetWindowLongPtrW(hwnd : super::super::Foundation:: HWND, nindex : WINDOW_LONG_PTR_INDEX) -> isize);
    GetWindowLongPtrW(hwnd.param().abi(), nindex)
}
#[inline]
pub unsafe fn GetWindowLongW<P0>(hwnd: P0, nindex: WINDOW_LONG_PTR_INDEX) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn GetWindowLongW(hwnd : super::super::Foundation:: HWND, nindex : WINDOW_LONG_PTR_INDEX) -> i32);
    GetWindowLongW(hwnd.param().abi(), nindex)
}
#[inline]
pub unsafe fn GetWindowModuleFileNameA<P0>(hwnd: P0, pszfilename: &mut [u8]) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn GetWindowModuleFileNameA(hwnd : super::super::Foundation:: HWND, pszfilename : windows_core::PSTR, cchfilenamemax : u32) -> u32);
    GetWindowModuleFileNameA(hwnd.param().abi(), core::mem::transmute(pszfilename.as_ptr()), pszfilename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn GetWindowModuleFileNameW<P0>(hwnd: P0, pszfilename: &mut [u16]) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn GetWindowModuleFileNameW(hwnd : super::super::Foundation:: HWND, pszfilename : windows_core::PWSTR, cchfilenamemax : u32) -> u32);
    GetWindowModuleFileNameW(hwnd.param().abi(), core::mem::transmute(pszfilename.as_ptr()), pszfilename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn GetWindowPlacement<P0>(hwnd: P0, lpwndpl: *mut WINDOWPLACEMENT) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn GetWindowPlacement(hwnd : super::super::Foundation:: HWND, lpwndpl : *mut WINDOWPLACEMENT) -> super::super::Foundation:: BOOL);
    GetWindowPlacement(hwnd.param().abi(), lpwndpl).ok()
}
#[inline]
pub unsafe fn GetWindowRect<P0>(hwnd: P0, lprect: *mut super::super::Foundation::RECT) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn GetWindowRect(hwnd : super::super::Foundation:: HWND, lprect : *mut super::super::Foundation:: RECT) -> super::super::Foundation:: BOOL);
    GetWindowRect(hwnd.param().abi(), lprect).ok()
}
#[inline]
pub unsafe fn GetWindowTextA<P0>(hwnd: P0, lpstring: &mut [u8]) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn GetWindowTextA(hwnd : super::super::Foundation:: HWND, lpstring : windows_core::PSTR, nmaxcount : i32) -> i32);
    GetWindowTextA(hwnd.param().abi(), core::mem::transmute(lpstring.as_ptr()), lpstring.len().try_into().unwrap())
}
#[inline]
pub unsafe fn GetWindowTextLengthA<P0>(hwnd: P0) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn GetWindowTextLengthA(hwnd : super::super::Foundation:: HWND) -> i32);
    GetWindowTextLengthA(hwnd.param().abi())
}
#[inline]
pub unsafe fn GetWindowTextLengthW<P0>(hwnd: P0) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn GetWindowTextLengthW(hwnd : super::super::Foundation:: HWND) -> i32);
    GetWindowTextLengthW(hwnd.param().abi())
}
#[inline]
pub unsafe fn GetWindowTextW<P0>(hwnd: P0, lpstring: &mut [u16]) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn GetWindowTextW(hwnd : super::super::Foundation:: HWND, lpstring : windows_core::PWSTR, nmaxcount : i32) -> i32);
    GetWindowTextW(hwnd.param().abi(), core::mem::transmute(lpstring.as_ptr()), lpstring.len().try_into().unwrap())
}
#[inline]
pub unsafe fn GetWindowThreadProcessId<P0>(hwnd: P0, lpdwprocessid: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn GetWindowThreadProcessId(hwnd : super::super::Foundation:: HWND, lpdwprocessid : *mut u32) -> u32);
    GetWindowThreadProcessId(hwnd.param().abi(), core::mem::transmute(lpdwprocessid.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn GetWindowWord<P0>(hwnd: P0, nindex: i32) -> u16
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn GetWindowWord(hwnd : super::super::Foundation:: HWND, nindex : i32) -> u16);
    GetWindowWord(hwnd.param().abi(), nindex)
}
#[inline]
pub unsafe fn HideCaret<P0>(hwnd: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn HideCaret(hwnd : super::super::Foundation:: HWND) -> super::super::Foundation:: BOOL);
    HideCaret(hwnd.param().abi()).ok()
}
#[inline]
pub unsafe fn HiliteMenuItem<P0, P1>(hwnd: P0, hmenu: P1, uidhiliteitem: u32, uhilite: u32) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<HMENU>,
{
    windows_targets::link!("user32.dll" "system" fn HiliteMenuItem(hwnd : super::super::Foundation:: HWND, hmenu : HMENU, uidhiliteitem : u32, uhilite : u32) -> super::super::Foundation:: BOOL);
    HiliteMenuItem(hwnd.param().abi(), hmenu.param().abi(), uidhiliteitem, uhilite)
}
#[inline]
pub unsafe fn InSendMessage() -> super::super::Foundation::BOOL {
    windows_targets::link!("user32.dll" "system" fn InSendMessage() -> super::super::Foundation:: BOOL);
    InSendMessage()
}
#[inline]
pub unsafe fn InSendMessageEx(lpreserved: Option<*const core::ffi::c_void>) -> u32 {
    windows_targets::link!("user32.dll" "system" fn InSendMessageEx(lpreserved : *const core::ffi::c_void) -> u32);
    InSendMessageEx(core::mem::transmute(lpreserved.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn IndexFilePath<P0>(resourceindexer: *const core::ffi::c_void, filepath: P0, ppresourceuri: *mut windows_core::PWSTR, pqualifiercount: *mut u32, ppqualifiers: *mut *mut IndexedResourceQualifier) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mrmsupport.dll" "system" fn IndexFilePath(resourceindexer : *const core::ffi::c_void, filepath : windows_core::PCWSTR, ppresourceuri : *mut windows_core::PWSTR, pqualifiercount : *mut u32, ppqualifiers : *mut *mut IndexedResourceQualifier) -> windows_core::HRESULT);
    IndexFilePath(resourceindexer, filepath.param().abi(), ppresourceuri, pqualifiercount, ppqualifiers).ok()
}
#[inline]
pub unsafe fn InheritWindowMonitor<P0, P1>(hwnd: P0, hwndinherit: P1) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn InheritWindowMonitor(hwnd : super::super::Foundation:: HWND, hwndinherit : super::super::Foundation:: HWND) -> super::super::Foundation:: BOOL);
    InheritWindowMonitor(hwnd.param().abi(), hwndinherit.param().abi())
}
#[inline]
pub unsafe fn InsertMenuA<P0, P1>(hmenu: P0, uposition: u32, uflags: MENU_ITEM_FLAGS, uidnewitem: usize, lpnewitem: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<HMENU>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("user32.dll" "system" fn InsertMenuA(hmenu : HMENU, uposition : u32, uflags : MENU_ITEM_FLAGS, uidnewitem : usize, lpnewitem : windows_core::PCSTR) -> super::super::Foundation:: BOOL);
    InsertMenuA(hmenu.param().abi(), uposition, uflags, uidnewitem, lpnewitem.param().abi()).ok()
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn InsertMenuItemA<P0, P1>(hmenu: P0, item: u32, fbyposition: P1, lpmi: *const MENUITEMINFOA) -> windows_core::Result<()>
where
    P0: windows_core::Param<HMENU>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("user32.dll" "system" fn InsertMenuItemA(hmenu : HMENU, item : u32, fbyposition : super::super::Foundation:: BOOL, lpmi : *const MENUITEMINFOA) -> super::super::Foundation:: BOOL);
    InsertMenuItemA(hmenu.param().abi(), item, fbyposition.param().abi(), lpmi).ok()
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn InsertMenuItemW<P0, P1>(hmenu: P0, item: u32, fbyposition: P1, lpmi: *const MENUITEMINFOW) -> windows_core::Result<()>
where
    P0: windows_core::Param<HMENU>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("user32.dll" "system" fn InsertMenuItemW(hmenu : HMENU, item : u32, fbyposition : super::super::Foundation:: BOOL, lpmi : *const MENUITEMINFOW) -> super::super::Foundation:: BOOL);
    InsertMenuItemW(hmenu.param().abi(), item, fbyposition.param().abi(), lpmi).ok()
}
#[inline]
pub unsafe fn InsertMenuW<P0, P1>(hmenu: P0, uposition: u32, uflags: MENU_ITEM_FLAGS, uidnewitem: usize, lpnewitem: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<HMENU>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("user32.dll" "system" fn InsertMenuW(hmenu : HMENU, uposition : u32, uflags : MENU_ITEM_FLAGS, uidnewitem : usize, lpnewitem : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    InsertMenuW(hmenu.param().abi(), uposition, uflags, uidnewitem, lpnewitem.param().abi()).ok()
}
#[inline]
pub unsafe fn InternalGetWindowText<P0>(hwnd: P0, pstring: &mut [u16]) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn InternalGetWindowText(hwnd : super::super::Foundation:: HWND, pstring : windows_core::PWSTR, cchmaxcount : i32) -> i32);
    InternalGetWindowText(hwnd.param().abi(), core::mem::transmute(pstring.as_ptr()), pstring.len().try_into().unwrap())
}
#[inline]
pub unsafe fn IsCharAlphaA(ch: i8) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn IsCharAlphaA(ch : i8) -> super::super::Foundation:: BOOL);
    IsCharAlphaA(ch).ok()
}
#[inline]
pub unsafe fn IsCharAlphaNumericA(ch: i8) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn IsCharAlphaNumericA(ch : i8) -> super::super::Foundation:: BOOL);
    IsCharAlphaNumericA(ch).ok()
}
#[inline]
pub unsafe fn IsCharAlphaNumericW(ch: u16) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn IsCharAlphaNumericW(ch : u16) -> super::super::Foundation:: BOOL);
    IsCharAlphaNumericW(ch).ok()
}
#[inline]
pub unsafe fn IsCharAlphaW(ch: u16) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn IsCharAlphaW(ch : u16) -> super::super::Foundation:: BOOL);
    IsCharAlphaW(ch).ok()
}
#[inline]
pub unsafe fn IsCharLowerA(ch: i8) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn IsCharLowerA(ch : i8) -> super::super::Foundation:: BOOL);
    IsCharLowerA(ch).ok()
}
#[inline]
pub unsafe fn IsCharUpperA(ch: i8) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn IsCharUpperA(ch : i8) -> super::super::Foundation:: BOOL);
    IsCharUpperA(ch).ok()
}
#[inline]
pub unsafe fn IsCharUpperW(ch: u16) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn IsCharUpperW(ch : u16) -> super::super::Foundation:: BOOL);
    IsCharUpperW(ch).ok()
}
#[inline]
pub unsafe fn IsChild<P0, P1>(hwndparent: P0, hwnd: P1) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn IsChild(hwndparent : super::super::Foundation:: HWND, hwnd : super::super::Foundation:: HWND) -> super::super::Foundation:: BOOL);
    IsChild(hwndparent.param().abi(), hwnd.param().abi())
}
#[inline]
pub unsafe fn IsDialogMessageA<P0>(hdlg: P0, lpmsg: *const MSG) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn IsDialogMessageA(hdlg : super::super::Foundation:: HWND, lpmsg : *const MSG) -> super::super::Foundation:: BOOL);
    IsDialogMessageA(hdlg.param().abi(), lpmsg)
}
#[inline]
pub unsafe fn IsDialogMessageW<P0>(hdlg: P0, lpmsg: *const MSG) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn IsDialogMessageW(hdlg : super::super::Foundation:: HWND, lpmsg : *const MSG) -> super::super::Foundation:: BOOL);
    IsDialogMessageW(hdlg.param().abi(), lpmsg)
}
#[inline]
pub unsafe fn IsGUIThread<P0>(bconvert: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("user32.dll" "system" fn IsGUIThread(bconvert : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    IsGUIThread(bconvert.param().abi())
}
#[inline]
pub unsafe fn IsHungAppWindow<P0>(hwnd: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn IsHungAppWindow(hwnd : super::super::Foundation:: HWND) -> super::super::Foundation:: BOOL);
    IsHungAppWindow(hwnd.param().abi())
}
#[inline]
pub unsafe fn IsIconic<P0>(hwnd: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn IsIconic(hwnd : super::super::Foundation:: HWND) -> super::super::Foundation:: BOOL);
    IsIconic(hwnd.param().abi())
}
#[inline]
pub unsafe fn IsMenu<P0>(hmenu: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<HMENU>,
{
    windows_targets::link!("user32.dll" "system" fn IsMenu(hmenu : HMENU) -> super::super::Foundation:: BOOL);
    IsMenu(hmenu.param().abi())
}
#[inline]
pub unsafe fn IsProcessDPIAware() -> super::super::Foundation::BOOL {
    windows_targets::link!("user32.dll" "system" fn IsProcessDPIAware() -> super::super::Foundation:: BOOL);
    IsProcessDPIAware()
}
#[inline]
pub unsafe fn IsWindow<P0>(hwnd: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn IsWindow(hwnd : super::super::Foundation:: HWND) -> super::super::Foundation:: BOOL);
    IsWindow(hwnd.param().abi())
}
#[inline]
pub unsafe fn IsWindowArranged<P0>(hwnd: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn IsWindowArranged(hwnd : super::super::Foundation:: HWND) -> super::super::Foundation:: BOOL);
    IsWindowArranged(hwnd.param().abi())
}
#[inline]
pub unsafe fn IsWindowUnicode<P0>(hwnd: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn IsWindowUnicode(hwnd : super::super::Foundation:: HWND) -> super::super::Foundation:: BOOL);
    IsWindowUnicode(hwnd.param().abi())
}
#[inline]
pub unsafe fn IsWindowVisible<P0>(hwnd: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn IsWindowVisible(hwnd : super::super::Foundation:: HWND) -> super::super::Foundation:: BOOL);
    IsWindowVisible(hwnd.param().abi())
}
#[inline]
pub unsafe fn IsWow64Message() -> super::super::Foundation::BOOL {
    windows_targets::link!("user32.dll" "system" fn IsWow64Message() -> super::super::Foundation:: BOOL);
    IsWow64Message()
}
#[inline]
pub unsafe fn IsZoomed<P0>(hwnd: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn IsZoomed(hwnd : super::super::Foundation:: HWND) -> super::super::Foundation:: BOOL);
    IsZoomed(hwnd.param().abi())
}
#[inline]
pub unsafe fn KillTimer<P0>(hwnd: P0, uidevent: usize) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn KillTimer(hwnd : super::super::Foundation:: HWND, uidevent : usize) -> super::super::Foundation:: BOOL);
    KillTimer(hwnd.param().abi(), uidevent).ok()
}
#[inline]
pub unsafe fn LoadAcceleratorsA<P0, P1>(hinstance: P0, lptablename: P1) -> windows_core::Result<HACCEL>
where
    P0: windows_core::Param<super::super::Foundation::HINSTANCE>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("user32.dll" "system" fn LoadAcceleratorsA(hinstance : super::super::Foundation:: HINSTANCE, lptablename : windows_core::PCSTR) -> HACCEL);
    let result__ = LoadAcceleratorsA(hinstance.param().abi(), lptablename.param().abi());
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn LoadAcceleratorsW<P0, P1>(hinstance: P0, lptablename: P1) -> windows_core::Result<HACCEL>
where
    P0: windows_core::Param<super::super::Foundation::HINSTANCE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("user32.dll" "system" fn LoadAcceleratorsW(hinstance : super::super::Foundation:: HINSTANCE, lptablename : windows_core::PCWSTR) -> HACCEL);
    let result__ = LoadAcceleratorsW(hinstance.param().abi(), lptablename.param().abi());
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn LoadCursorA<P0, P1>(hinstance: P0, lpcursorname: P1) -> windows_core::Result<HCURSOR>
where
    P0: windows_core::Param<super::super::Foundation::HINSTANCE>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("user32.dll" "system" fn LoadCursorA(hinstance : super::super::Foundation:: HINSTANCE, lpcursorname : windows_core::PCSTR) -> HCURSOR);
    let result__ = LoadCursorA(hinstance.param().abi(), lpcursorname.param().abi());
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn LoadCursorFromFileA<P0>(lpfilename: P0) -> windows_core::Result<HCURSOR>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("user32.dll" "system" fn LoadCursorFromFileA(lpfilename : windows_core::PCSTR) -> HCURSOR);
    let result__ = LoadCursorFromFileA(lpfilename.param().abi());
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn LoadCursorFromFileW<P0>(lpfilename: P0) -> windows_core::Result<HCURSOR>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("user32.dll" "system" fn LoadCursorFromFileW(lpfilename : windows_core::PCWSTR) -> HCURSOR);
    let result__ = LoadCursorFromFileW(lpfilename.param().abi());
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn LoadCursorW<P0, P1>(hinstance: P0, lpcursorname: P1) -> windows_core::Result<HCURSOR>
where
    P0: windows_core::Param<super::super::Foundation::HINSTANCE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("user32.dll" "system" fn LoadCursorW(hinstance : super::super::Foundation:: HINSTANCE, lpcursorname : windows_core::PCWSTR) -> HCURSOR);
    let result__ = LoadCursorW(hinstance.param().abi(), lpcursorname.param().abi());
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn LoadIconA<P0, P1>(hinstance: P0, lpiconname: P1) -> windows_core::Result<HICON>
where
    P0: windows_core::Param<super::super::Foundation::HINSTANCE>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("user32.dll" "system" fn LoadIconA(hinstance : super::super::Foundation:: HINSTANCE, lpiconname : windows_core::PCSTR) -> HICON);
    let result__ = LoadIconA(hinstance.param().abi(), lpiconname.param().abi());
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn LoadIconW<P0, P1>(hinstance: P0, lpiconname: P1) -> windows_core::Result<HICON>
where
    P0: windows_core::Param<super::super::Foundation::HINSTANCE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("user32.dll" "system" fn LoadIconW(hinstance : super::super::Foundation:: HINSTANCE, lpiconname : windows_core::PCWSTR) -> HICON);
    let result__ = LoadIconW(hinstance.param().abi(), lpiconname.param().abi());
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn LoadImageA<P0, P1>(hinst: P0, name: P1, r#type: GDI_IMAGE_TYPE, cx: i32, cy: i32, fuload: IMAGE_FLAGS) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<super::super::Foundation::HINSTANCE>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("user32.dll" "system" fn LoadImageA(hinst : super::super::Foundation:: HINSTANCE, name : windows_core::PCSTR, r#type : GDI_IMAGE_TYPE, cx : i32, cy : i32, fuload : IMAGE_FLAGS) -> super::super::Foundation:: HANDLE);
    let result__ = LoadImageA(hinst.param().abi(), name.param().abi(), r#type, cx, cy, fuload);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn LoadImageW<P0, P1>(hinst: P0, name: P1, r#type: GDI_IMAGE_TYPE, cx: i32, cy: i32, fuload: IMAGE_FLAGS) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<super::super::Foundation::HINSTANCE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("user32.dll" "system" fn LoadImageW(hinst : super::super::Foundation:: HINSTANCE, name : windows_core::PCWSTR, r#type : GDI_IMAGE_TYPE, cx : i32, cy : i32, fuload : IMAGE_FLAGS) -> super::super::Foundation:: HANDLE);
    let result__ = LoadImageW(hinst.param().abi(), name.param().abi(), r#type, cx, cy, fuload);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn LoadMenuA<P0, P1>(hinstance: P0, lpmenuname: P1) -> windows_core::Result<HMENU>
where
    P0: windows_core::Param<super::super::Foundation::HINSTANCE>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("user32.dll" "system" fn LoadMenuA(hinstance : super::super::Foundation:: HINSTANCE, lpmenuname : windows_core::PCSTR) -> HMENU);
    let result__ = LoadMenuA(hinstance.param().abi(), lpmenuname.param().abi());
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn LoadMenuIndirectA(lpmenutemplate: *const core::ffi::c_void) -> windows_core::Result<HMENU> {
    windows_targets::link!("user32.dll" "system" fn LoadMenuIndirectA(lpmenutemplate : *const core::ffi::c_void) -> HMENU);
    let result__ = LoadMenuIndirectA(lpmenutemplate);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn LoadMenuIndirectW(lpmenutemplate: *const core::ffi::c_void) -> windows_core::Result<HMENU> {
    windows_targets::link!("user32.dll" "system" fn LoadMenuIndirectW(lpmenutemplate : *const core::ffi::c_void) -> HMENU);
    let result__ = LoadMenuIndirectW(lpmenutemplate);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn LoadMenuW<P0, P1>(hinstance: P0, lpmenuname: P1) -> windows_core::Result<HMENU>
where
    P0: windows_core::Param<super::super::Foundation::HINSTANCE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("user32.dll" "system" fn LoadMenuW(hinstance : super::super::Foundation:: HINSTANCE, lpmenuname : windows_core::PCWSTR) -> HMENU);
    let result__ = LoadMenuW(hinstance.param().abi(), lpmenuname.param().abi());
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn LoadStringA<P0>(hinstance: P0, uid: u32, lpbuffer: windows_core::PSTR, cchbuffermax: i32) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HINSTANCE>,
{
    windows_targets::link!("user32.dll" "system" fn LoadStringA(hinstance : super::super::Foundation:: HINSTANCE, uid : u32, lpbuffer : windows_core::PSTR, cchbuffermax : i32) -> i32);
    LoadStringA(hinstance.param().abi(), uid, core::mem::transmute(lpbuffer), cchbuffermax)
}
#[inline]
pub unsafe fn LoadStringW<P0>(hinstance: P0, uid: u32, lpbuffer: windows_core::PWSTR, cchbuffermax: i32) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HINSTANCE>,
{
    windows_targets::link!("user32.dll" "system" fn LoadStringW(hinstance : super::super::Foundation:: HINSTANCE, uid : u32, lpbuffer : windows_core::PWSTR, cchbuffermax : i32) -> i32);
    LoadStringW(hinstance.param().abi(), uid, core::mem::transmute(lpbuffer), cchbuffermax)
}
#[inline]
pub unsafe fn LockSetForegroundWindow(ulockcode: FOREGROUND_WINDOW_LOCK_CODE) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn LockSetForegroundWindow(ulockcode : FOREGROUND_WINDOW_LOCK_CODE) -> super::super::Foundation:: BOOL);
    LockSetForegroundWindow(ulockcode).ok()
}
#[inline]
pub unsafe fn LogicalToPhysicalPoint<P0>(hwnd: P0, lppoint: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn LogicalToPhysicalPoint(hwnd : super::super::Foundation:: HWND, lppoint : *mut super::super::Foundation:: POINT) -> super::super::Foundation:: BOOL);
    LogicalToPhysicalPoint(hwnd.param().abi(), lppoint)
}
#[inline]
pub unsafe fn LookupIconIdFromDirectory<P0>(presbits: *const u8, ficon: P0) -> i32
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("user32.dll" "system" fn LookupIconIdFromDirectory(presbits : *const u8, ficon : super::super::Foundation:: BOOL) -> i32);
    LookupIconIdFromDirectory(presbits, ficon.param().abi())
}
#[inline]
pub unsafe fn LookupIconIdFromDirectoryEx<P0>(presbits: *const u8, ficon: P0, cxdesired: i32, cydesired: i32, flags: IMAGE_FLAGS) -> i32
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("user32.dll" "system" fn LookupIconIdFromDirectoryEx(presbits : *const u8, ficon : super::super::Foundation:: BOOL, cxdesired : i32, cydesired : i32, flags : IMAGE_FLAGS) -> i32);
    LookupIconIdFromDirectoryEx(presbits, ficon.param().abi(), cxdesired, cydesired, flags)
}
#[inline]
pub unsafe fn MapDialogRect<P0>(hdlg: P0, lprect: *mut super::super::Foundation::RECT) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn MapDialogRect(hdlg : super::super::Foundation:: HWND, lprect : *mut super::super::Foundation:: RECT) -> super::super::Foundation:: BOOL);
    MapDialogRect(hdlg.param().abi(), lprect).ok()
}
#[inline]
pub unsafe fn MenuItemFromPoint<P0, P1>(hwnd: P0, hmenu: P1, ptscreen: super::super::Foundation::POINT) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<HMENU>,
{
    windows_targets::link!("user32.dll" "system" fn MenuItemFromPoint(hwnd : super::super::Foundation:: HWND, hmenu : HMENU, ptscreen : super::super::Foundation:: POINT) -> i32);
    MenuItemFromPoint(hwnd.param().abi(), hmenu.param().abi(), core::mem::transmute(ptscreen))
}
#[inline]
pub unsafe fn MessageBoxA<P0, P1, P2>(hwnd: P0, lptext: P1, lpcaption: P2, utype: MESSAGEBOX_STYLE) -> MESSAGEBOX_RESULT
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("user32.dll" "system" fn MessageBoxA(hwnd : super::super::Foundation:: HWND, lptext : windows_core::PCSTR, lpcaption : windows_core::PCSTR, utype : MESSAGEBOX_STYLE) -> MESSAGEBOX_RESULT);
    MessageBoxA(hwnd.param().abi(), lptext.param().abi(), lpcaption.param().abi(), utype)
}
#[inline]
pub unsafe fn MessageBoxExA<P0, P1, P2>(hwnd: P0, lptext: P1, lpcaption: P2, utype: MESSAGEBOX_STYLE, wlanguageid: u16) -> MESSAGEBOX_RESULT
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("user32.dll" "system" fn MessageBoxExA(hwnd : super::super::Foundation:: HWND, lptext : windows_core::PCSTR, lpcaption : windows_core::PCSTR, utype : MESSAGEBOX_STYLE, wlanguageid : u16) -> MESSAGEBOX_RESULT);
    MessageBoxExA(hwnd.param().abi(), lptext.param().abi(), lpcaption.param().abi(), utype, wlanguageid)
}
#[inline]
pub unsafe fn MessageBoxExW<P0, P1, P2>(hwnd: P0, lptext: P1, lpcaption: P2, utype: MESSAGEBOX_STYLE, wlanguageid: u16) -> MESSAGEBOX_RESULT
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("user32.dll" "system" fn MessageBoxExW(hwnd : super::super::Foundation:: HWND, lptext : windows_core::PCWSTR, lpcaption : windows_core::PCWSTR, utype : MESSAGEBOX_STYLE, wlanguageid : u16) -> MESSAGEBOX_RESULT);
    MessageBoxExW(hwnd.param().abi(), lptext.param().abi(), lpcaption.param().abi(), utype, wlanguageid)
}
#[cfg(feature = "Win32_UI_Shell")]
#[inline]
pub unsafe fn MessageBoxIndirectA(lpmbp: *const MSGBOXPARAMSA) -> MESSAGEBOX_RESULT {
    windows_targets::link!("user32.dll" "system" fn MessageBoxIndirectA(lpmbp : *const MSGBOXPARAMSA) -> MESSAGEBOX_RESULT);
    MessageBoxIndirectA(lpmbp)
}
#[cfg(feature = "Win32_UI_Shell")]
#[inline]
pub unsafe fn MessageBoxIndirectW(lpmbp: *const MSGBOXPARAMSW) -> MESSAGEBOX_RESULT {
    windows_targets::link!("user32.dll" "system" fn MessageBoxIndirectW(lpmbp : *const MSGBOXPARAMSW) -> MESSAGEBOX_RESULT);
    MessageBoxIndirectW(lpmbp)
}
#[inline]
pub unsafe fn MessageBoxW<P0, P1, P2>(hwnd: P0, lptext: P1, lpcaption: P2, utype: MESSAGEBOX_STYLE) -> MESSAGEBOX_RESULT
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("user32.dll" "system" fn MessageBoxW(hwnd : super::super::Foundation:: HWND, lptext : windows_core::PCWSTR, lpcaption : windows_core::PCWSTR, utype : MESSAGEBOX_STYLE) -> MESSAGEBOX_RESULT);
    MessageBoxW(hwnd.param().abi(), lptext.param().abi(), lpcaption.param().abi(), utype)
}
#[inline]
pub unsafe fn ModifyMenuA<P0, P1>(hmnu: P0, uposition: u32, uflags: MENU_ITEM_FLAGS, uidnewitem: usize, lpnewitem: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<HMENU>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("user32.dll" "system" fn ModifyMenuA(hmnu : HMENU, uposition : u32, uflags : MENU_ITEM_FLAGS, uidnewitem : usize, lpnewitem : windows_core::PCSTR) -> super::super::Foundation:: BOOL);
    ModifyMenuA(hmnu.param().abi(), uposition, uflags, uidnewitem, lpnewitem.param().abi()).ok()
}
#[inline]
pub unsafe fn ModifyMenuW<P0, P1>(hmnu: P0, uposition: u32, uflags: MENU_ITEM_FLAGS, uidnewitem: usize, lpnewitem: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<HMENU>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("user32.dll" "system" fn ModifyMenuW(hmnu : HMENU, uposition : u32, uflags : MENU_ITEM_FLAGS, uidnewitem : usize, lpnewitem : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    ModifyMenuW(hmnu.param().abi(), uposition, uflags, uidnewitem, lpnewitem.param().abi()).ok()
}
#[inline]
pub unsafe fn MoveWindow<P0, P1>(hwnd: P0, x: i32, y: i32, nwidth: i32, nheight: i32, brepaint: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("user32.dll" "system" fn MoveWindow(hwnd : super::super::Foundation:: HWND, x : i32, y : i32, nwidth : i32, nheight : i32, brepaint : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    MoveWindow(hwnd.param().abi(), x, y, nwidth, nheight, brepaint.param().abi()).ok()
}
#[inline]
pub unsafe fn MrmCreateConfig<P0, P1>(platformversion: MrmPlatformVersion, defaultqualifiers: P0, outputxmlfile: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mrmsupport.dll" "system" fn MrmCreateConfig(platformversion : MrmPlatformVersion, defaultqualifiers : windows_core::PCWSTR, outputxmlfile : windows_core::PCWSTR) -> windows_core::HRESULT);
    MrmCreateConfig(platformversion, defaultqualifiers.param().abi(), outputxmlfile.param().abi()).ok()
}
#[inline]
pub unsafe fn MrmCreateConfigInMemory<P0>(platformversion: MrmPlatformVersion, defaultqualifiers: P0, outputxmldata: *mut *mut u8, outputxmlsize: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mrmsupport.dll" "system" fn MrmCreateConfigInMemory(platformversion : MrmPlatformVersion, defaultqualifiers : windows_core::PCWSTR, outputxmldata : *mut *mut u8, outputxmlsize : *mut u32) -> windows_core::HRESULT);
    MrmCreateConfigInMemory(platformversion, defaultqualifiers.param().abi(), outputxmldata, outputxmlsize).ok()
}
#[inline]
pub unsafe fn MrmCreateResourceFile<P0>(indexer: MrmResourceIndexerHandle, packagingmode: MrmPackagingMode, packagingoptions: MrmPackagingOptions, outputdirectory: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mrmsupport.dll" "system" fn MrmCreateResourceFile(indexer : MrmResourceIndexerHandle, packagingmode : MrmPackagingMode, packagingoptions : MrmPackagingOptions, outputdirectory : windows_core::PCWSTR) -> windows_core::HRESULT);
    MrmCreateResourceFile(core::mem::transmute(indexer), packagingmode, packagingoptions, outputdirectory.param().abi()).ok()
}
#[inline]
pub unsafe fn MrmCreateResourceFileInMemory(indexer: MrmResourceIndexerHandle, packagingmode: MrmPackagingMode, packagingoptions: MrmPackagingOptions, outputpridata: *mut *mut u8, outputprisize: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("mrmsupport.dll" "system" fn MrmCreateResourceFileInMemory(indexer : MrmResourceIndexerHandle, packagingmode : MrmPackagingMode, packagingoptions : MrmPackagingOptions, outputpridata : *mut *mut u8, outputprisize : *mut u32) -> windows_core::HRESULT);
    MrmCreateResourceFileInMemory(core::mem::transmute(indexer), packagingmode, packagingoptions, outputpridata, outputprisize).ok()
}
#[inline]
pub unsafe fn MrmCreateResourceFileWithChecksum<P0>(indexer: MrmResourceIndexerHandle, packagingmode: MrmPackagingMode, packagingoptions: MrmPackagingOptions, checksum: u32, outputdirectory: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mrmsupport.dll" "system" fn MrmCreateResourceFileWithChecksum(indexer : MrmResourceIndexerHandle, packagingmode : MrmPackagingMode, packagingoptions : MrmPackagingOptions, checksum : u32, outputdirectory : windows_core::PCWSTR) -> windows_core::HRESULT);
    MrmCreateResourceFileWithChecksum(core::mem::transmute(indexer), packagingmode, packagingoptions, checksum, outputdirectory.param().abi()).ok()
}
#[inline]
pub unsafe fn MrmCreateResourceIndexer<P0, P1, P2>(packagefamilyname: P0, projectroot: P1, platformversion: MrmPlatformVersion, defaultqualifiers: P2, indexer: *mut MrmResourceIndexerHandle) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mrmsupport.dll" "system" fn MrmCreateResourceIndexer(packagefamilyname : windows_core::PCWSTR, projectroot : windows_core::PCWSTR, platformversion : MrmPlatformVersion, defaultqualifiers : windows_core::PCWSTR, indexer : *mut MrmResourceIndexerHandle) -> windows_core::HRESULT);
    MrmCreateResourceIndexer(packagefamilyname.param().abi(), projectroot.param().abi(), platformversion, defaultqualifiers.param().abi(), indexer).ok()
}
#[inline]
pub unsafe fn MrmCreateResourceIndexerFromPreviousPriData<P0, P1>(projectroot: P0, platformversion: MrmPlatformVersion, defaultqualifiers: P1, pridata: &[u8], indexer: *mut MrmResourceIndexerHandle) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mrmsupport.dll" "system" fn MrmCreateResourceIndexerFromPreviousPriData(projectroot : windows_core::PCWSTR, platformversion : MrmPlatformVersion, defaultqualifiers : windows_core::PCWSTR, pridata : *const u8, prisize : u32, indexer : *mut MrmResourceIndexerHandle) -> windows_core::HRESULT);
    MrmCreateResourceIndexerFromPreviousPriData(projectroot.param().abi(), platformversion, defaultqualifiers.param().abi(), core::mem::transmute(pridata.as_ptr()), pridata.len().try_into().unwrap(), indexer).ok()
}
#[inline]
pub unsafe fn MrmCreateResourceIndexerFromPreviousPriFile<P0, P1, P2>(projectroot: P0, platformversion: MrmPlatformVersion, defaultqualifiers: P1, prifile: P2, indexer: *mut MrmResourceIndexerHandle) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mrmsupport.dll" "system" fn MrmCreateResourceIndexerFromPreviousPriFile(projectroot : windows_core::PCWSTR, platformversion : MrmPlatformVersion, defaultqualifiers : windows_core::PCWSTR, prifile : windows_core::PCWSTR, indexer : *mut MrmResourceIndexerHandle) -> windows_core::HRESULT);
    MrmCreateResourceIndexerFromPreviousPriFile(projectroot.param().abi(), platformversion, defaultqualifiers.param().abi(), prifile.param().abi(), indexer).ok()
}
#[inline]
pub unsafe fn MrmCreateResourceIndexerFromPreviousSchemaData<P0, P1>(projectroot: P0, platformversion: MrmPlatformVersion, defaultqualifiers: P1, schemaxmldata: &[u8], indexer: *mut MrmResourceIndexerHandle) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mrmsupport.dll" "system" fn MrmCreateResourceIndexerFromPreviousSchemaData(projectroot : windows_core::PCWSTR, platformversion : MrmPlatformVersion, defaultqualifiers : windows_core::PCWSTR, schemaxmldata : *const u8, schemaxmlsize : u32, indexer : *mut MrmResourceIndexerHandle) -> windows_core::HRESULT);
    MrmCreateResourceIndexerFromPreviousSchemaData(projectroot.param().abi(), platformversion, defaultqualifiers.param().abi(), core::mem::transmute(schemaxmldata.as_ptr()), schemaxmldata.len().try_into().unwrap(), indexer).ok()
}
#[inline]
pub unsafe fn MrmCreateResourceIndexerFromPreviousSchemaFile<P0, P1, P2>(projectroot: P0, platformversion: MrmPlatformVersion, defaultqualifiers: P1, schemafile: P2, indexer: *mut MrmResourceIndexerHandle) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mrmsupport.dll" "system" fn MrmCreateResourceIndexerFromPreviousSchemaFile(projectroot : windows_core::PCWSTR, platformversion : MrmPlatformVersion, defaultqualifiers : windows_core::PCWSTR, schemafile : windows_core::PCWSTR, indexer : *mut MrmResourceIndexerHandle) -> windows_core::HRESULT);
    MrmCreateResourceIndexerFromPreviousSchemaFile(projectroot.param().abi(), platformversion, defaultqualifiers.param().abi(), schemafile.param().abi(), indexer).ok()
}
#[inline]
pub unsafe fn MrmCreateResourceIndexerWithFlags<P0, P1, P2>(packagefamilyname: P0, projectroot: P1, platformversion: MrmPlatformVersion, defaultqualifiers: P2, flags: MrmIndexerFlags, indexer: *mut MrmResourceIndexerHandle) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mrmsupport.dll" "system" fn MrmCreateResourceIndexerWithFlags(packagefamilyname : windows_core::PCWSTR, projectroot : windows_core::PCWSTR, platformversion : MrmPlatformVersion, defaultqualifiers : windows_core::PCWSTR, flags : MrmIndexerFlags, indexer : *mut MrmResourceIndexerHandle) -> windows_core::HRESULT);
    MrmCreateResourceIndexerWithFlags(packagefamilyname.param().abi(), projectroot.param().abi(), platformversion, defaultqualifiers.param().abi(), flags, indexer).ok()
}
#[inline]
pub unsafe fn MrmDestroyIndexerAndMessages(indexer: MrmResourceIndexerHandle) -> windows_core::Result<()> {
    windows_targets::link!("mrmsupport.dll" "system" fn MrmDestroyIndexerAndMessages(indexer : MrmResourceIndexerHandle) -> windows_core::HRESULT);
    MrmDestroyIndexerAndMessages(core::mem::transmute(indexer)).ok()
}
#[inline]
pub unsafe fn MrmDumpPriDataInMemory(inputpridata: &[u8], schemapridata: Option<&[u8]>, dumptype: MrmDumpType, outputxmldata: *mut *mut u8, outputxmlsize: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("mrmsupport.dll" "system" fn MrmDumpPriDataInMemory(inputpridata : *const u8, inputprisize : u32, schemapridata : *const u8, schemaprisize : u32, dumptype : MrmDumpType, outputxmldata : *mut *mut u8, outputxmlsize : *mut u32) -> windows_core::HRESULT);
    MrmDumpPriDataInMemory(core::mem::transmute(inputpridata.as_ptr()), inputpridata.len().try_into().unwrap(), core::mem::transmute(schemapridata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), schemapridata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), dumptype, outputxmldata, outputxmlsize).ok()
}
#[inline]
pub unsafe fn MrmDumpPriFile<P0, P1, P2>(indexfilename: P0, schemaprifile: P1, dumptype: MrmDumpType, outputxmlfile: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mrmsupport.dll" "system" fn MrmDumpPriFile(indexfilename : windows_core::PCWSTR, schemaprifile : windows_core::PCWSTR, dumptype : MrmDumpType, outputxmlfile : windows_core::PCWSTR) -> windows_core::HRESULT);
    MrmDumpPriFile(indexfilename.param().abi(), schemaprifile.param().abi(), dumptype, outputxmlfile.param().abi()).ok()
}
#[inline]
pub unsafe fn MrmDumpPriFileInMemory<P0, P1>(indexfilename: P0, schemaprifile: P1, dumptype: MrmDumpType, outputxmldata: *mut *mut u8, outputxmlsize: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mrmsupport.dll" "system" fn MrmDumpPriFileInMemory(indexfilename : windows_core::PCWSTR, schemaprifile : windows_core::PCWSTR, dumptype : MrmDumpType, outputxmldata : *mut *mut u8, outputxmlsize : *mut u32) -> windows_core::HRESULT);
    MrmDumpPriFileInMemory(indexfilename.param().abi(), schemaprifile.param().abi(), dumptype, outputxmldata, outputxmlsize).ok()
}
#[inline]
pub unsafe fn MrmFreeMemory(data: *const u8) -> windows_core::Result<()> {
    windows_targets::link!("mrmsupport.dll" "system" fn MrmFreeMemory(data : *const u8) -> windows_core::HRESULT);
    MrmFreeMemory(data).ok()
}
#[inline]
pub unsafe fn MrmGetPriFileContentChecksum<P0>(prifile: P0) -> windows_core::Result<u32>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mrmsupport.dll" "system" fn MrmGetPriFileContentChecksum(prifile : windows_core::PCWSTR, checksum : *mut u32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    MrmGetPriFileContentChecksum(prifile.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn MrmIndexEmbeddedData<P0, P1>(indexer: MrmResourceIndexerHandle, resourceuri: P0, embeddeddata: &[u8], qualifiers: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mrmsupport.dll" "system" fn MrmIndexEmbeddedData(indexer : MrmResourceIndexerHandle, resourceuri : windows_core::PCWSTR, embeddeddata : *const u8, embeddeddatasize : u32, qualifiers : windows_core::PCWSTR) -> windows_core::HRESULT);
    MrmIndexEmbeddedData(core::mem::transmute(indexer), resourceuri.param().abi(), core::mem::transmute(embeddeddata.as_ptr()), embeddeddata.len().try_into().unwrap(), qualifiers.param().abi()).ok()
}
#[inline]
pub unsafe fn MrmIndexFile<P0, P1, P2>(indexer: MrmResourceIndexerHandle, resourceuri: P0, filepath: P1, qualifiers: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mrmsupport.dll" "system" fn MrmIndexFile(indexer : MrmResourceIndexerHandle, resourceuri : windows_core::PCWSTR, filepath : windows_core::PCWSTR, qualifiers : windows_core::PCWSTR) -> windows_core::HRESULT);
    MrmIndexFile(core::mem::transmute(indexer), resourceuri.param().abi(), filepath.param().abi(), qualifiers.param().abi()).ok()
}
#[inline]
pub unsafe fn MrmIndexFileAutoQualifiers<P0>(indexer: MrmResourceIndexerHandle, filepath: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mrmsupport.dll" "system" fn MrmIndexFileAutoQualifiers(indexer : MrmResourceIndexerHandle, filepath : windows_core::PCWSTR) -> windows_core::HRESULT);
    MrmIndexFileAutoQualifiers(core::mem::transmute(indexer), filepath.param().abi()).ok()
}
#[inline]
pub unsafe fn MrmIndexResourceContainerAutoQualifiers<P0>(indexer: MrmResourceIndexerHandle, containerpath: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mrmsupport.dll" "system" fn MrmIndexResourceContainerAutoQualifiers(indexer : MrmResourceIndexerHandle, containerpath : windows_core::PCWSTR) -> windows_core::HRESULT);
    MrmIndexResourceContainerAutoQualifiers(core::mem::transmute(indexer), containerpath.param().abi()).ok()
}
#[inline]
pub unsafe fn MrmIndexString<P0, P1, P2>(indexer: MrmResourceIndexerHandle, resourceuri: P0, resourcestring: P1, qualifiers: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mrmsupport.dll" "system" fn MrmIndexString(indexer : MrmResourceIndexerHandle, resourceuri : windows_core::PCWSTR, resourcestring : windows_core::PCWSTR, qualifiers : windows_core::PCWSTR) -> windows_core::HRESULT);
    MrmIndexString(core::mem::transmute(indexer), resourceuri.param().abi(), resourcestring.param().abi(), qualifiers.param().abi()).ok()
}
#[inline]
pub unsafe fn MrmPeekResourceIndexerMessages(handle: MrmResourceIndexerHandle, messages: *mut *mut MrmResourceIndexerMessage, nummsgs: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("mrmsupport.dll" "system" fn MrmPeekResourceIndexerMessages(handle : MrmResourceIndexerHandle, messages : *mut *mut MrmResourceIndexerMessage, nummsgs : *mut u32) -> windows_core::HRESULT);
    MrmPeekResourceIndexerMessages(core::mem::transmute(handle), messages, nummsgs).ok()
}
#[inline]
pub unsafe fn MsgWaitForMultipleObjects<P0>(phandles: Option<&[super::super::Foundation::HANDLE]>, fwaitall: P0, dwmilliseconds: u32, dwwakemask: QUEUE_STATUS_FLAGS) -> super::super::Foundation::WAIT_EVENT
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("user32.dll" "system" fn MsgWaitForMultipleObjects(ncount : u32, phandles : *const super::super::Foundation:: HANDLE, fwaitall : super::super::Foundation:: BOOL, dwmilliseconds : u32, dwwakemask : QUEUE_STATUS_FLAGS) -> super::super::Foundation:: WAIT_EVENT);
    MsgWaitForMultipleObjects(phandles.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(phandles.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), fwaitall.param().abi(), dwmilliseconds, dwwakemask)
}
#[inline]
pub unsafe fn MsgWaitForMultipleObjectsEx(phandles: Option<&[super::super::Foundation::HANDLE]>, dwmilliseconds: u32, dwwakemask: QUEUE_STATUS_FLAGS, dwflags: MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS) -> super::super::Foundation::WAIT_EVENT {
    windows_targets::link!("user32.dll" "system" fn MsgWaitForMultipleObjectsEx(ncount : u32, phandles : *const super::super::Foundation:: HANDLE, dwmilliseconds : u32, dwwakemask : QUEUE_STATUS_FLAGS, dwflags : MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS) -> super::super::Foundation:: WAIT_EVENT);
    MsgWaitForMultipleObjectsEx(phandles.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(phandles.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), dwmilliseconds, dwwakemask, dwflags)
}
#[inline]
pub unsafe fn OemToCharA<P0>(psrc: P0, pdst: windows_core::PSTR) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("user32.dll" "system" fn OemToCharA(psrc : windows_core::PCSTR, pdst : windows_core::PSTR) -> super::super::Foundation:: BOOL);
    OemToCharA(psrc.param().abi(), core::mem::transmute(pdst)).ok()
}
#[inline]
pub unsafe fn OemToCharBuffA<P0>(lpszsrc: P0, lpszdst: &mut [u8]) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("user32.dll" "system" fn OemToCharBuffA(lpszsrc : windows_core::PCSTR, lpszdst : windows_core::PSTR, cchdstlength : u32) -> super::super::Foundation:: BOOL);
    OemToCharBuffA(lpszsrc.param().abi(), core::mem::transmute(lpszdst.as_ptr()), lpszdst.len().try_into().unwrap()).ok()
}
#[inline]
pub unsafe fn OemToCharBuffW<P0>(lpszsrc: P0, lpszdst: &mut [u16]) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("user32.dll" "system" fn OemToCharBuffW(lpszsrc : windows_core::PCSTR, lpszdst : windows_core::PWSTR, cchdstlength : u32) -> super::super::Foundation:: BOOL);
    OemToCharBuffW(lpszsrc.param().abi(), core::mem::transmute(lpszdst.as_ptr()), lpszdst.len().try_into().unwrap()).ok()
}
#[inline]
pub unsafe fn OemToCharW<P0>(psrc: P0, pdst: windows_core::PWSTR) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("user32.dll" "system" fn OemToCharW(psrc : windows_core::PCSTR, pdst : windows_core::PWSTR) -> super::super::Foundation:: BOOL);
    OemToCharW(psrc.param().abi(), core::mem::transmute(pdst)).ok()
}
#[inline]
pub unsafe fn OpenIcon<P0>(hwnd: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn OpenIcon(hwnd : super::super::Foundation:: HWND) -> super::super::Foundation:: BOOL);
    OpenIcon(hwnd.param().abi()).ok()
}
#[inline]
pub unsafe fn PeekMessageA<P0>(lpmsg: *mut MSG, hwnd: P0, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: PEEK_MESSAGE_REMOVE_TYPE) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn PeekMessageA(lpmsg : *mut MSG, hwnd : super::super::Foundation:: HWND, wmsgfiltermin : u32, wmsgfiltermax : u32, wremovemsg : PEEK_MESSAGE_REMOVE_TYPE) -> super::super::Foundation:: BOOL);
    PeekMessageA(lpmsg, hwnd.param().abi(), wmsgfiltermin, wmsgfiltermax, wremovemsg)
}
#[inline]
pub unsafe fn PeekMessageW<P0>(lpmsg: *mut MSG, hwnd: P0, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: PEEK_MESSAGE_REMOVE_TYPE) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn PeekMessageW(lpmsg : *mut MSG, hwnd : super::super::Foundation:: HWND, wmsgfiltermin : u32, wmsgfiltermax : u32, wremovemsg : PEEK_MESSAGE_REMOVE_TYPE) -> super::super::Foundation:: BOOL);
    PeekMessageW(lpmsg, hwnd.param().abi(), wmsgfiltermin, wmsgfiltermax, wremovemsg)
}
#[inline]
pub unsafe fn PhysicalToLogicalPoint<P0>(hwnd: P0, lppoint: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn PhysicalToLogicalPoint(hwnd : super::super::Foundation:: HWND, lppoint : *mut super::super::Foundation:: POINT) -> super::super::Foundation:: BOOL);
    PhysicalToLogicalPoint(hwnd.param().abi(), lppoint)
}
#[inline]
pub unsafe fn PostMessageA<P0, P1, P2>(hwnd: P0, msg: u32, wparam: P1, lparam: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::WPARAM>,
    P2: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("user32.dll" "system" fn PostMessageA(hwnd : super::super::Foundation:: HWND, msg : u32, wparam : super::super::Foundation:: WPARAM, lparam : super::super::Foundation:: LPARAM) -> super::super::Foundation:: BOOL);
    PostMessageA(hwnd.param().abi(), msg, wparam.param().abi(), lparam.param().abi()).ok()
}
#[inline]
pub unsafe fn PostMessageW<P0, P1, P2>(hwnd: P0, msg: u32, wparam: P1, lparam: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::WPARAM>,
    P2: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("user32.dll" "system" fn PostMessageW(hwnd : super::super::Foundation:: HWND, msg : u32, wparam : super::super::Foundation:: WPARAM, lparam : super::super::Foundation:: LPARAM) -> super::super::Foundation:: BOOL);
    PostMessageW(hwnd.param().abi(), msg, wparam.param().abi(), lparam.param().abi()).ok()
}
#[inline]
pub unsafe fn PostQuitMessage(nexitcode: i32) {
    windows_targets::link!("user32.dll" "system" fn PostQuitMessage(nexitcode : i32));
    PostQuitMessage(nexitcode)
}
#[inline]
pub unsafe fn PostThreadMessageA<P0, P1>(idthread: u32, msg: u32, wparam: P0, lparam: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::WPARAM>,
    P1: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("user32.dll" "system" fn PostThreadMessageA(idthread : u32, msg : u32, wparam : super::super::Foundation:: WPARAM, lparam : super::super::Foundation:: LPARAM) -> super::super::Foundation:: BOOL);
    PostThreadMessageA(idthread, msg, wparam.param().abi(), lparam.param().abi()).ok()
}
#[inline]
pub unsafe fn PostThreadMessageW<P0, P1>(idthread: u32, msg: u32, wparam: P0, lparam: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::WPARAM>,
    P1: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("user32.dll" "system" fn PostThreadMessageW(idthread : u32, msg : u32, wparam : super::super::Foundation:: WPARAM, lparam : super::super::Foundation:: LPARAM) -> super::super::Foundation:: BOOL);
    PostThreadMessageW(idthread, msg, wparam.param().abi(), lparam.param().abi()).ok()
}
#[inline]
pub unsafe fn PrivateExtractIconsA(szfilename: &[u8; 260], niconindex: i32, cxicon: i32, cyicon: i32, phicon: Option<&mut [HICON]>, piconid: Option<*mut u32>, flags: u32) -> u32 {
    windows_targets::link!("user32.dll" "system" fn PrivateExtractIconsA(szfilename : windows_core::PCSTR, niconindex : i32, cxicon : i32, cyicon : i32, phicon : *mut HICON, piconid : *mut u32, nicons : u32, flags : u32) -> u32);
    PrivateExtractIconsA(core::mem::transmute(szfilename.as_ptr()), niconindex, cxicon, cyicon, core::mem::transmute(phicon.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), core::mem::transmute(piconid.unwrap_or(std::ptr::null_mut())), phicon.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), flags)
}
#[inline]
pub unsafe fn PrivateExtractIconsW(szfilename: &[u16; 260], niconindex: i32, cxicon: i32, cyicon: i32, phicon: Option<&mut [HICON]>, piconid: Option<*mut u32>, flags: u32) -> u32 {
    windows_targets::link!("user32.dll" "system" fn PrivateExtractIconsW(szfilename : windows_core::PCWSTR, niconindex : i32, cxicon : i32, cyicon : i32, phicon : *mut HICON, piconid : *mut u32, nicons : u32, flags : u32) -> u32);
    PrivateExtractIconsW(core::mem::transmute(szfilename.as_ptr()), niconindex, cxicon, cyicon, core::mem::transmute(phicon.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), core::mem::transmute(piconid.unwrap_or(std::ptr::null_mut())), phicon.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), flags)
}
#[inline]
pub unsafe fn RealChildWindowFromPoint<P0>(hwndparent: P0, ptparentclientcoords: super::super::Foundation::POINT) -> super::super::Foundation::HWND
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn RealChildWindowFromPoint(hwndparent : super::super::Foundation:: HWND, ptparentclientcoords : super::super::Foundation:: POINT) -> super::super::Foundation:: HWND);
    RealChildWindowFromPoint(hwndparent.param().abi(), core::mem::transmute(ptparentclientcoords))
}
#[inline]
pub unsafe fn RealGetWindowClassA<P0>(hwnd: P0, ptszclassname: &mut [u8]) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn RealGetWindowClassA(hwnd : super::super::Foundation:: HWND, ptszclassname : windows_core::PSTR, cchclassnamemax : u32) -> u32);
    RealGetWindowClassA(hwnd.param().abi(), core::mem::transmute(ptszclassname.as_ptr()), ptszclassname.len().try_into().unwrap())
}
#[inline]
pub unsafe fn RealGetWindowClassW<P0>(hwnd: P0, ptszclassname: &mut [u16]) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn RealGetWindowClassW(hwnd : super::super::Foundation:: HWND, ptszclassname : windows_core::PWSTR, cchclassnamemax : u32) -> u32);
    RealGetWindowClassW(hwnd.param().abi(), core::mem::transmute(ptszclassname.as_ptr()), ptszclassname.len().try_into().unwrap())
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn RegisterClassA(lpwndclass: *const WNDCLASSA) -> u16 {
    windows_targets::link!("user32.dll" "system" fn RegisterClassA(lpwndclass : *const WNDCLASSA) -> u16);
    RegisterClassA(lpwndclass)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn RegisterClassExA(param0: *const WNDCLASSEXA) -> u16 {
    windows_targets::link!("user32.dll" "system" fn RegisterClassExA(param0 : *const WNDCLASSEXA) -> u16);
    RegisterClassExA(param0)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn RegisterClassExW(param0: *const WNDCLASSEXW) -> u16 {
    windows_targets::link!("user32.dll" "system" fn RegisterClassExW(param0 : *const WNDCLASSEXW) -> u16);
    RegisterClassExW(param0)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn RegisterClassW(lpwndclass: *const WNDCLASSW) -> u16 {
    windows_targets::link!("user32.dll" "system" fn RegisterClassW(lpwndclass : *const WNDCLASSW) -> u16);
    RegisterClassW(lpwndclass)
}
#[inline]
pub unsafe fn RegisterDeviceNotificationA<P0>(hrecipient: P0, notificationfilter: *const core::ffi::c_void, flags: REGISTER_NOTIFICATION_FLAGS) -> windows_core::Result<HDEVNOTIFY>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("user32.dll" "system" fn RegisterDeviceNotificationA(hrecipient : super::super::Foundation:: HANDLE, notificationfilter : *const core::ffi::c_void, flags : REGISTER_NOTIFICATION_FLAGS) -> HDEVNOTIFY);
    let result__ = RegisterDeviceNotificationA(hrecipient.param().abi(), notificationfilter, flags);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn RegisterDeviceNotificationW<P0>(hrecipient: P0, notificationfilter: *const core::ffi::c_void, flags: REGISTER_NOTIFICATION_FLAGS) -> windows_core::Result<HDEVNOTIFY>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("user32.dll" "system" fn RegisterDeviceNotificationW(hrecipient : super::super::Foundation:: HANDLE, notificationfilter : *const core::ffi::c_void, flags : REGISTER_NOTIFICATION_FLAGS) -> HDEVNOTIFY);
    let result__ = RegisterDeviceNotificationW(hrecipient.param().abi(), notificationfilter, flags);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn RegisterForTooltipDismissNotification<P0>(hwnd: P0, tdflags: TOOLTIP_DISMISS_FLAGS) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn RegisterForTooltipDismissNotification(hwnd : super::super::Foundation:: HWND, tdflags : TOOLTIP_DISMISS_FLAGS) -> super::super::Foundation:: BOOL);
    RegisterForTooltipDismissNotification(hwnd.param().abi(), tdflags)
}
#[inline]
pub unsafe fn RegisterShellHookWindow<P0>(hwnd: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn RegisterShellHookWindow(hwnd : super::super::Foundation:: HWND) -> super::super::Foundation:: BOOL);
    RegisterShellHookWindow(hwnd.param().abi())
}
#[inline]
pub unsafe fn RegisterWindowMessageA<P0>(lpstring: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("user32.dll" "system" fn RegisterWindowMessageA(lpstring : windows_core::PCSTR) -> u32);
    RegisterWindowMessageA(lpstring.param().abi())
}
#[inline]
pub unsafe fn RegisterWindowMessageW<P0>(lpstring: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("user32.dll" "system" fn RegisterWindowMessageW(lpstring : windows_core::PCWSTR) -> u32);
    RegisterWindowMessageW(lpstring.param().abi())
}
#[inline]
pub unsafe fn RemoveMenu<P0>(hmenu: P0, uposition: u32, uflags: MENU_ITEM_FLAGS) -> windows_core::Result<()>
where
    P0: windows_core::Param<HMENU>,
{
    windows_targets::link!("user32.dll" "system" fn RemoveMenu(hmenu : HMENU, uposition : u32, uflags : MENU_ITEM_FLAGS) -> super::super::Foundation:: BOOL);
    RemoveMenu(hmenu.param().abi(), uposition, uflags).ok()
}
#[inline]
pub unsafe fn RemovePropA<P0, P1>(hwnd: P0, lpstring: P1) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("user32.dll" "system" fn RemovePropA(hwnd : super::super::Foundation:: HWND, lpstring : windows_core::PCSTR) -> super::super::Foundation:: HANDLE);
    let result__ = RemovePropA(hwnd.param().abi(), lpstring.param().abi());
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn RemovePropW<P0, P1>(hwnd: P0, lpstring: P1) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("user32.dll" "system" fn RemovePropW(hwnd : super::super::Foundation:: HWND, lpstring : windows_core::PCWSTR) -> super::super::Foundation:: HANDLE);
    let result__ = RemovePropW(hwnd.param().abi(), lpstring.param().abi());
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn ReplyMessage<P0>(lresult: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::LRESULT>,
{
    windows_targets::link!("user32.dll" "system" fn ReplyMessage(lresult : super::super::Foundation:: LRESULT) -> super::super::Foundation:: BOOL);
    ReplyMessage(lresult.param().abi())
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ScrollDC<P0, P1>(hdc: P0, dx: i32, dy: i32, lprcscroll: Option<*const super::super::Foundation::RECT>, lprcclip: Option<*const super::super::Foundation::RECT>, hrgnupdate: P1, lprcupdate: Option<*mut super::super::Foundation::RECT>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Graphics::Gdi::HDC>,
    P1: windows_core::Param<super::super::Graphics::Gdi::HRGN>,
{
    windows_targets::link!("user32.dll" "system" fn ScrollDC(hdc : super::super::Graphics::Gdi:: HDC, dx : i32, dy : i32, lprcscroll : *const super::super::Foundation:: RECT, lprcclip : *const super::super::Foundation:: RECT, hrgnupdate : super::super::Graphics::Gdi:: HRGN, lprcupdate : *mut super::super::Foundation:: RECT) -> super::super::Foundation:: BOOL);
    ScrollDC(hdc.param().abi(), dx, dy, core::mem::transmute(lprcscroll.unwrap_or(std::ptr::null())), core::mem::transmute(lprcclip.unwrap_or(std::ptr::null())), hrgnupdate.param().abi(), core::mem::transmute(lprcupdate.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn ScrollWindow<P0>(hwnd: P0, xamount: i32, yamount: i32, lprect: Option<*const super::super::Foundation::RECT>, lpcliprect: Option<*const super::super::Foundation::RECT>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn ScrollWindow(hwnd : super::super::Foundation:: HWND, xamount : i32, yamount : i32, lprect : *const super::super::Foundation:: RECT, lpcliprect : *const super::super::Foundation:: RECT) -> super::super::Foundation:: BOOL);
    ScrollWindow(hwnd.param().abi(), xamount, yamount, core::mem::transmute(lprect.unwrap_or(std::ptr::null())), core::mem::transmute(lpcliprect.unwrap_or(std::ptr::null()))).ok()
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ScrollWindowEx<P0, P1>(hwnd: P0, dx: i32, dy: i32, prcscroll: Option<*const super::super::Foundation::RECT>, prcclip: Option<*const super::super::Foundation::RECT>, hrgnupdate: P1, prcupdate: Option<*mut super::super::Foundation::RECT>, flags: SCROLL_WINDOW_FLAGS) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Graphics::Gdi::HRGN>,
{
    windows_targets::link!("user32.dll" "system" fn ScrollWindowEx(hwnd : super::super::Foundation:: HWND, dx : i32, dy : i32, prcscroll : *const super::super::Foundation:: RECT, prcclip : *const super::super::Foundation:: RECT, hrgnupdate : super::super::Graphics::Gdi:: HRGN, prcupdate : *mut super::super::Foundation:: RECT, flags : SCROLL_WINDOW_FLAGS) -> i32);
    ScrollWindowEx(hwnd.param().abi(), dx, dy, core::mem::transmute(prcscroll.unwrap_or(std::ptr::null())), core::mem::transmute(prcclip.unwrap_or(std::ptr::null())), hrgnupdate.param().abi(), core::mem::transmute(prcupdate.unwrap_or(std::ptr::null_mut())), flags)
}
#[inline]
pub unsafe fn SendDlgItemMessageA<P0, P1, P2>(hdlg: P0, niddlgitem: i32, msg: u32, wparam: P1, lparam: P2) -> super::super::Foundation::LRESULT
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::WPARAM>,
    P2: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("user32.dll" "system" fn SendDlgItemMessageA(hdlg : super::super::Foundation:: HWND, niddlgitem : i32, msg : u32, wparam : super::super::Foundation:: WPARAM, lparam : super::super::Foundation:: LPARAM) -> super::super::Foundation:: LRESULT);
    SendDlgItemMessageA(hdlg.param().abi(), niddlgitem, msg, wparam.param().abi(), lparam.param().abi())
}
#[inline]
pub unsafe fn SendDlgItemMessageW<P0, P1, P2>(hdlg: P0, niddlgitem: i32, msg: u32, wparam: P1, lparam: P2) -> super::super::Foundation::LRESULT
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::WPARAM>,
    P2: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("user32.dll" "system" fn SendDlgItemMessageW(hdlg : super::super::Foundation:: HWND, niddlgitem : i32, msg : u32, wparam : super::super::Foundation:: WPARAM, lparam : super::super::Foundation:: LPARAM) -> super::super::Foundation:: LRESULT);
    SendDlgItemMessageW(hdlg.param().abi(), niddlgitem, msg, wparam.param().abi(), lparam.param().abi())
}
#[inline]
pub unsafe fn SendMessageA<P0, P1, P2>(hwnd: P0, msg: u32, wparam: P1, lparam: P2) -> super::super::Foundation::LRESULT
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::WPARAM>,
    P2: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("user32.dll" "system" fn SendMessageA(hwnd : super::super::Foundation:: HWND, msg : u32, wparam : super::super::Foundation:: WPARAM, lparam : super::super::Foundation:: LPARAM) -> super::super::Foundation:: LRESULT);
    SendMessageA(hwnd.param().abi(), msg, wparam.param().abi(), lparam.param().abi())
}
#[inline]
pub unsafe fn SendMessageCallbackA<P0, P1, P2>(hwnd: P0, msg: u32, wparam: P1, lparam: P2, lpresultcallback: SENDASYNCPROC, dwdata: usize) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::WPARAM>,
    P2: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("user32.dll" "system" fn SendMessageCallbackA(hwnd : super::super::Foundation:: HWND, msg : u32, wparam : super::super::Foundation:: WPARAM, lparam : super::super::Foundation:: LPARAM, lpresultcallback : SENDASYNCPROC, dwdata : usize) -> super::super::Foundation:: BOOL);
    SendMessageCallbackA(hwnd.param().abi(), msg, wparam.param().abi(), lparam.param().abi(), lpresultcallback, dwdata).ok()
}
#[inline]
pub unsafe fn SendMessageCallbackW<P0, P1, P2>(hwnd: P0, msg: u32, wparam: P1, lparam: P2, lpresultcallback: SENDASYNCPROC, dwdata: usize) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::WPARAM>,
    P2: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("user32.dll" "system" fn SendMessageCallbackW(hwnd : super::super::Foundation:: HWND, msg : u32, wparam : super::super::Foundation:: WPARAM, lparam : super::super::Foundation:: LPARAM, lpresultcallback : SENDASYNCPROC, dwdata : usize) -> super::super::Foundation:: BOOL);
    SendMessageCallbackW(hwnd.param().abi(), msg, wparam.param().abi(), lparam.param().abi(), lpresultcallback, dwdata).ok()
}
#[inline]
pub unsafe fn SendMessageTimeoutA<P0, P1, P2>(hwnd: P0, msg: u32, wparam: P1, lparam: P2, fuflags: SEND_MESSAGE_TIMEOUT_FLAGS, utimeout: u32, lpdwresult: Option<*mut usize>) -> super::super::Foundation::LRESULT
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::WPARAM>,
    P2: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("user32.dll" "system" fn SendMessageTimeoutA(hwnd : super::super::Foundation:: HWND, msg : u32, wparam : super::super::Foundation:: WPARAM, lparam : super::super::Foundation:: LPARAM, fuflags : SEND_MESSAGE_TIMEOUT_FLAGS, utimeout : u32, lpdwresult : *mut usize) -> super::super::Foundation:: LRESULT);
    SendMessageTimeoutA(hwnd.param().abi(), msg, wparam.param().abi(), lparam.param().abi(), fuflags, utimeout, core::mem::transmute(lpdwresult.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn SendMessageTimeoutW<P0, P1, P2>(hwnd: P0, msg: u32, wparam: P1, lparam: P2, fuflags: SEND_MESSAGE_TIMEOUT_FLAGS, utimeout: u32, lpdwresult: Option<*mut usize>) -> super::super::Foundation::LRESULT
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::WPARAM>,
    P2: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("user32.dll" "system" fn SendMessageTimeoutW(hwnd : super::super::Foundation:: HWND, msg : u32, wparam : super::super::Foundation:: WPARAM, lparam : super::super::Foundation:: LPARAM, fuflags : SEND_MESSAGE_TIMEOUT_FLAGS, utimeout : u32, lpdwresult : *mut usize) -> super::super::Foundation:: LRESULT);
    SendMessageTimeoutW(hwnd.param().abi(), msg, wparam.param().abi(), lparam.param().abi(), fuflags, utimeout, core::mem::transmute(lpdwresult.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn SendMessageW<P0, P1, P2>(hwnd: P0, msg: u32, wparam: P1, lparam: P2) -> super::super::Foundation::LRESULT
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::WPARAM>,
    P2: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("user32.dll" "system" fn SendMessageW(hwnd : super::super::Foundation:: HWND, msg : u32, wparam : super::super::Foundation:: WPARAM, lparam : super::super::Foundation:: LPARAM) -> super::super::Foundation:: LRESULT);
    SendMessageW(hwnd.param().abi(), msg, wparam.param().abi(), lparam.param().abi())
}
#[inline]
pub unsafe fn SendNotifyMessageA<P0, P1, P2>(hwnd: P0, msg: u32, wparam: P1, lparam: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::WPARAM>,
    P2: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("user32.dll" "system" fn SendNotifyMessageA(hwnd : super::super::Foundation:: HWND, msg : u32, wparam : super::super::Foundation:: WPARAM, lparam : super::super::Foundation:: LPARAM) -> super::super::Foundation:: BOOL);
    SendNotifyMessageA(hwnd.param().abi(), msg, wparam.param().abi(), lparam.param().abi()).ok()
}
#[inline]
pub unsafe fn SendNotifyMessageW<P0, P1, P2>(hwnd: P0, msg: u32, wparam: P1, lparam: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::WPARAM>,
    P2: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("user32.dll" "system" fn SendNotifyMessageW(hwnd : super::super::Foundation:: HWND, msg : u32, wparam : super::super::Foundation:: WPARAM, lparam : super::super::Foundation:: LPARAM) -> super::super::Foundation:: BOOL);
    SendNotifyMessageW(hwnd.param().abi(), msg, wparam.param().abi(), lparam.param().abi()).ok()
}
#[inline]
pub unsafe fn SetAdditionalForegroundBoostProcesses<P0>(toplevelwindow: P0, processhandlearray: &[super::super::Foundation::HANDLE]) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn SetAdditionalForegroundBoostProcesses(toplevelwindow : super::super::Foundation:: HWND, processhandlecount : u32, processhandlearray : *const super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    SetAdditionalForegroundBoostProcesses(toplevelwindow.param().abi(), processhandlearray.len().try_into().unwrap(), core::mem::transmute(processhandlearray.as_ptr()))
}
#[inline]
pub unsafe fn SetCaretBlinkTime(umseconds: u32) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn SetCaretBlinkTime(umseconds : u32) -> super::super::Foundation:: BOOL);
    SetCaretBlinkTime(umseconds).ok()
}
#[inline]
pub unsafe fn SetCaretPos(x: i32, y: i32) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn SetCaretPos(x : i32, y : i32) -> super::super::Foundation:: BOOL);
    SetCaretPos(x, y).ok()
}
#[inline]
pub unsafe fn SetClassLongA<P0>(hwnd: P0, nindex: GET_CLASS_LONG_INDEX, dwnewlong: i32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn SetClassLongA(hwnd : super::super::Foundation:: HWND, nindex : GET_CLASS_LONG_INDEX, dwnewlong : i32) -> u32);
    SetClassLongA(hwnd.param().abi(), nindex, dwnewlong)
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn SetClassLongPtrA<P0>(hwnd: P0, nindex: GET_CLASS_LONG_INDEX, dwnewlong: isize) -> usize
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn SetClassLongPtrA(hwnd : super::super::Foundation:: HWND, nindex : GET_CLASS_LONG_INDEX, dwnewlong : isize) -> usize);
    SetClassLongPtrA(hwnd.param().abi(), nindex, dwnewlong)
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn SetClassLongPtrW<P0>(hwnd: P0, nindex: GET_CLASS_LONG_INDEX, dwnewlong: isize) -> usize
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn SetClassLongPtrW(hwnd : super::super::Foundation:: HWND, nindex : GET_CLASS_LONG_INDEX, dwnewlong : isize) -> usize);
    SetClassLongPtrW(hwnd.param().abi(), nindex, dwnewlong)
}
#[inline]
pub unsafe fn SetClassLongW<P0>(hwnd: P0, nindex: GET_CLASS_LONG_INDEX, dwnewlong: i32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn SetClassLongW(hwnd : super::super::Foundation:: HWND, nindex : GET_CLASS_LONG_INDEX, dwnewlong : i32) -> u32);
    SetClassLongW(hwnd.param().abi(), nindex, dwnewlong)
}
#[inline]
pub unsafe fn SetClassWord<P0>(hwnd: P0, nindex: i32, wnewword: u16) -> u16
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn SetClassWord(hwnd : super::super::Foundation:: HWND, nindex : i32, wnewword : u16) -> u16);
    SetClassWord(hwnd.param().abi(), nindex, wnewword)
}
#[inline]
pub unsafe fn SetCoalescableTimer<P0>(hwnd: P0, nidevent: usize, uelapse: u32, lptimerfunc: TIMERPROC, utolerancedelay: u32) -> usize
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn SetCoalescableTimer(hwnd : super::super::Foundation:: HWND, nidevent : usize, uelapse : u32, lptimerfunc : TIMERPROC, utolerancedelay : u32) -> usize);
    SetCoalescableTimer(hwnd.param().abi(), nidevent, uelapse, lptimerfunc, utolerancedelay)
}
#[inline]
pub unsafe fn SetCursor<P0>(hcursor: P0) -> HCURSOR
where
    P0: windows_core::Param<HCURSOR>,
{
    windows_targets::link!("user32.dll" "system" fn SetCursor(hcursor : HCURSOR) -> HCURSOR);
    SetCursor(hcursor.param().abi())
}
#[inline]
pub unsafe fn SetCursorPos(x: i32, y: i32) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn SetCursorPos(x : i32, y : i32) -> super::super::Foundation:: BOOL);
    SetCursorPos(x, y).ok()
}
#[inline]
pub unsafe fn SetDebugErrorLevel(dwlevel: u32) {
    windows_targets::link!("user32.dll" "system" fn SetDebugErrorLevel(dwlevel : u32));
    SetDebugErrorLevel(dwlevel)
}
#[inline]
pub unsafe fn SetDlgItemInt<P0, P1>(hdlg: P0, niddlgitem: i32, uvalue: u32, bsigned: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("user32.dll" "system" fn SetDlgItemInt(hdlg : super::super::Foundation:: HWND, niddlgitem : i32, uvalue : u32, bsigned : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    SetDlgItemInt(hdlg.param().abi(), niddlgitem, uvalue, bsigned.param().abi()).ok()
}
#[inline]
pub unsafe fn SetDlgItemTextA<P0, P1>(hdlg: P0, niddlgitem: i32, lpstring: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("user32.dll" "system" fn SetDlgItemTextA(hdlg : super::super::Foundation:: HWND, niddlgitem : i32, lpstring : windows_core::PCSTR) -> super::super::Foundation:: BOOL);
    SetDlgItemTextA(hdlg.param().abi(), niddlgitem, lpstring.param().abi()).ok()
}
#[inline]
pub unsafe fn SetDlgItemTextW<P0, P1>(hdlg: P0, niddlgitem: i32, lpstring: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("user32.dll" "system" fn SetDlgItemTextW(hdlg : super::super::Foundation:: HWND, niddlgitem : i32, lpstring : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    SetDlgItemTextW(hdlg.param().abi(), niddlgitem, lpstring.param().abi()).ok()
}
#[inline]
pub unsafe fn SetForegroundWindow<P0>(hwnd: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn SetForegroundWindow(hwnd : super::super::Foundation:: HWND) -> super::super::Foundation:: BOOL);
    SetForegroundWindow(hwnd.param().abi())
}
#[inline]
pub unsafe fn SetLayeredWindowAttributes<P0, P1>(hwnd: P0, crkey: P1, balpha: u8, dwflags: LAYERED_WINDOW_ATTRIBUTES_FLAGS) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::COLORREF>,
{
    windows_targets::link!("user32.dll" "system" fn SetLayeredWindowAttributes(hwnd : super::super::Foundation:: HWND, crkey : super::super::Foundation:: COLORREF, balpha : u8, dwflags : LAYERED_WINDOW_ATTRIBUTES_FLAGS) -> super::super::Foundation:: BOOL);
    SetLayeredWindowAttributes(hwnd.param().abi(), crkey.param().abi(), balpha, dwflags).ok()
}
#[inline]
pub unsafe fn SetMenu<P0, P1>(hwnd: P0, hmenu: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<HMENU>,
{
    windows_targets::link!("user32.dll" "system" fn SetMenu(hwnd : super::super::Foundation:: HWND, hmenu : HMENU) -> super::super::Foundation:: BOOL);
    SetMenu(hwnd.param().abi(), hmenu.param().abi()).ok()
}
#[inline]
pub unsafe fn SetMenuDefaultItem<P0>(hmenu: P0, uitem: u32, fbypos: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<HMENU>,
{
    windows_targets::link!("user32.dll" "system" fn SetMenuDefaultItem(hmenu : HMENU, uitem : u32, fbypos : u32) -> super::super::Foundation:: BOOL);
    SetMenuDefaultItem(hmenu.param().abi(), uitem, fbypos).ok()
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn SetMenuInfo<P0>(param0: P0, param1: *const MENUINFO) -> windows_core::Result<()>
where
    P0: windows_core::Param<HMENU>,
{
    windows_targets::link!("user32.dll" "system" fn SetMenuInfo(param0 : HMENU, param1 : *const MENUINFO) -> super::super::Foundation:: BOOL);
    SetMenuInfo(param0.param().abi(), param1).ok()
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn SetMenuItemBitmaps<P0, P1, P2>(hmenu: P0, uposition: u32, uflags: MENU_ITEM_FLAGS, hbitmapunchecked: P1, hbitmapchecked: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<HMENU>,
    P1: windows_core::Param<super::super::Graphics::Gdi::HBITMAP>,
    P2: windows_core::Param<super::super::Graphics::Gdi::HBITMAP>,
{
    windows_targets::link!("user32.dll" "system" fn SetMenuItemBitmaps(hmenu : HMENU, uposition : u32, uflags : MENU_ITEM_FLAGS, hbitmapunchecked : super::super::Graphics::Gdi:: HBITMAP, hbitmapchecked : super::super::Graphics::Gdi:: HBITMAP) -> super::super::Foundation:: BOOL);
    SetMenuItemBitmaps(hmenu.param().abi(), uposition, uflags, hbitmapunchecked.param().abi(), hbitmapchecked.param().abi()).ok()
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn SetMenuItemInfoA<P0, P1>(hmenu: P0, item: u32, fbypositon: P1, lpmii: *const MENUITEMINFOA) -> windows_core::Result<()>
where
    P0: windows_core::Param<HMENU>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("user32.dll" "system" fn SetMenuItemInfoA(hmenu : HMENU, item : u32, fbypositon : super::super::Foundation:: BOOL, lpmii : *const MENUITEMINFOA) -> super::super::Foundation:: BOOL);
    SetMenuItemInfoA(hmenu.param().abi(), item, fbypositon.param().abi(), lpmii).ok()
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn SetMenuItemInfoW<P0, P1>(hmenu: P0, item: u32, fbypositon: P1, lpmii: *const MENUITEMINFOW) -> windows_core::Result<()>
where
    P0: windows_core::Param<HMENU>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("user32.dll" "system" fn SetMenuItemInfoW(hmenu : HMENU, item : u32, fbypositon : super::super::Foundation:: BOOL, lpmii : *const MENUITEMINFOW) -> super::super::Foundation:: BOOL);
    SetMenuItemInfoW(hmenu.param().abi(), item, fbypositon.param().abi(), lpmii).ok()
}
#[inline]
pub unsafe fn SetMessageExtraInfo<P0>(lparam: P0) -> super::super::Foundation::LPARAM
where
    P0: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("user32.dll" "system" fn SetMessageExtraInfo(lparam : super::super::Foundation:: LPARAM) -> super::super::Foundation:: LPARAM);
    SetMessageExtraInfo(lparam.param().abi())
}
#[inline]
pub unsafe fn SetMessageQueue(cmessagesmax: i32) -> super::super::Foundation::BOOL {
    windows_targets::link!("user32.dll" "system" fn SetMessageQueue(cmessagesmax : i32) -> super::super::Foundation:: BOOL);
    SetMessageQueue(cmessagesmax)
}
#[inline]
pub unsafe fn SetParent<P0, P1>(hwndchild: P0, hwndnewparent: P1) -> super::super::Foundation::HWND
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn SetParent(hwndchild : super::super::Foundation:: HWND, hwndnewparent : super::super::Foundation:: HWND) -> super::super::Foundation:: HWND);
    SetParent(hwndchild.param().abi(), hwndnewparent.param().abi())
}
#[inline]
pub unsafe fn SetPhysicalCursorPos(x: i32, y: i32) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn SetPhysicalCursorPos(x : i32, y : i32) -> super::super::Foundation:: BOOL);
    SetPhysicalCursorPos(x, y).ok()
}
#[inline]
pub unsafe fn SetProcessDPIAware() -> super::super::Foundation::BOOL {
    windows_targets::link!("user32.dll" "system" fn SetProcessDPIAware() -> super::super::Foundation:: BOOL);
    SetProcessDPIAware()
}
#[inline]
pub unsafe fn SetProcessDefaultLayout(dwdefaultlayout: u32) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn SetProcessDefaultLayout(dwdefaultlayout : u32) -> super::super::Foundation:: BOOL);
    SetProcessDefaultLayout(dwdefaultlayout).ok()
}
#[inline]
pub unsafe fn SetPropA<P0, P1, P2>(hwnd: P0, lpstring: P1, hdata: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("user32.dll" "system" fn SetPropA(hwnd : super::super::Foundation:: HWND, lpstring : windows_core::PCSTR, hdata : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    SetPropA(hwnd.param().abi(), lpstring.param().abi(), hdata.param().abi()).ok()
}
#[inline]
pub unsafe fn SetPropW<P0, P1, P2>(hwnd: P0, lpstring: P1, hdata: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("user32.dll" "system" fn SetPropW(hwnd : super::super::Foundation:: HWND, lpstring : windows_core::PCWSTR, hdata : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    SetPropW(hwnd.param().abi(), lpstring.param().abi(), hdata.param().abi()).ok()
}
#[inline]
pub unsafe fn SetSystemCursor<P0>(hcur: P0, id: SYSTEM_CURSOR_ID) -> windows_core::Result<()>
where
    P0: windows_core::Param<HCURSOR>,
{
    windows_targets::link!("user32.dll" "system" fn SetSystemCursor(hcur : HCURSOR, id : SYSTEM_CURSOR_ID) -> super::super::Foundation:: BOOL);
    SetSystemCursor(hcur.param().abi(), id).ok()
}
#[inline]
pub unsafe fn SetTimer<P0>(hwnd: P0, nidevent: usize, uelapse: u32, lptimerfunc: TIMERPROC) -> usize
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn SetTimer(hwnd : super::super::Foundation:: HWND, nidevent : usize, uelapse : u32, lptimerfunc : TIMERPROC) -> usize);
    SetTimer(hwnd.param().abi(), nidevent, uelapse, lptimerfunc)
}
#[inline]
pub unsafe fn SetWindowDisplayAffinity<P0>(hwnd: P0, dwaffinity: WINDOW_DISPLAY_AFFINITY) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn SetWindowDisplayAffinity(hwnd : super::super::Foundation:: HWND, dwaffinity : WINDOW_DISPLAY_AFFINITY) -> super::super::Foundation:: BOOL);
    SetWindowDisplayAffinity(hwnd.param().abi(), dwaffinity).ok()
}
#[inline]
pub unsafe fn SetWindowLongA<P0>(hwnd: P0, nindex: WINDOW_LONG_PTR_INDEX, dwnewlong: i32) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn SetWindowLongA(hwnd : super::super::Foundation:: HWND, nindex : WINDOW_LONG_PTR_INDEX, dwnewlong : i32) -> i32);
    SetWindowLongA(hwnd.param().abi(), nindex, dwnewlong)
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn SetWindowLongPtrA<P0>(hwnd: P0, nindex: WINDOW_LONG_PTR_INDEX, dwnewlong: isize) -> isize
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn SetWindowLongPtrA(hwnd : super::super::Foundation:: HWND, nindex : WINDOW_LONG_PTR_INDEX, dwnewlong : isize) -> isize);
    SetWindowLongPtrA(hwnd.param().abi(), nindex, dwnewlong)
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn SetWindowLongPtrW<P0>(hwnd: P0, nindex: WINDOW_LONG_PTR_INDEX, dwnewlong: isize) -> isize
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn SetWindowLongPtrW(hwnd : super::super::Foundation:: HWND, nindex : WINDOW_LONG_PTR_INDEX, dwnewlong : isize) -> isize);
    SetWindowLongPtrW(hwnd.param().abi(), nindex, dwnewlong)
}
#[inline]
pub unsafe fn SetWindowLongW<P0>(hwnd: P0, nindex: WINDOW_LONG_PTR_INDEX, dwnewlong: i32) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn SetWindowLongW(hwnd : super::super::Foundation:: HWND, nindex : WINDOW_LONG_PTR_INDEX, dwnewlong : i32) -> i32);
    SetWindowLongW(hwnd.param().abi(), nindex, dwnewlong)
}
#[inline]
pub unsafe fn SetWindowPlacement<P0>(hwnd: P0, lpwndpl: *const WINDOWPLACEMENT) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn SetWindowPlacement(hwnd : super::super::Foundation:: HWND, lpwndpl : *const WINDOWPLACEMENT) -> super::super::Foundation:: BOOL);
    SetWindowPlacement(hwnd.param().abi(), lpwndpl).ok()
}
#[inline]
pub unsafe fn SetWindowPos<P0, P1>(hwnd: P0, hwndinsertafter: P1, x: i32, y: i32, cx: i32, cy: i32, uflags: SET_WINDOW_POS_FLAGS) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn SetWindowPos(hwnd : super::super::Foundation:: HWND, hwndinsertafter : super::super::Foundation:: HWND, x : i32, y : i32, cx : i32, cy : i32, uflags : SET_WINDOW_POS_FLAGS) -> super::super::Foundation:: BOOL);
    SetWindowPos(hwnd.param().abi(), hwndinsertafter.param().abi(), x, y, cx, cy, uflags).ok()
}
#[inline]
pub unsafe fn SetWindowTextA<P0, P1>(hwnd: P0, lpstring: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("user32.dll" "system" fn SetWindowTextA(hwnd : super::super::Foundation:: HWND, lpstring : windows_core::PCSTR) -> super::super::Foundation:: BOOL);
    SetWindowTextA(hwnd.param().abi(), lpstring.param().abi()).ok()
}
#[inline]
pub unsafe fn SetWindowTextW<P0, P1>(hwnd: P0, lpstring: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("user32.dll" "system" fn SetWindowTextW(hwnd : super::super::Foundation:: HWND, lpstring : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    SetWindowTextW(hwnd.param().abi(), lpstring.param().abi()).ok()
}
#[inline]
pub unsafe fn SetWindowWord<P0>(hwnd: P0, nindex: i32, wnewword: u16) -> u16
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn SetWindowWord(hwnd : super::super::Foundation:: HWND, nindex : i32, wnewword : u16) -> u16);
    SetWindowWord(hwnd.param().abi(), nindex, wnewword)
}
#[inline]
pub unsafe fn SetWindowsHookA(nfiltertype: i32, pfnfilterproc: HOOKPROC) -> HHOOK {
    windows_targets::link!("user32.dll" "system" fn SetWindowsHookA(nfiltertype : i32, pfnfilterproc : HOOKPROC) -> HHOOK);
    SetWindowsHookA(nfiltertype, pfnfilterproc)
}
#[inline]
pub unsafe fn SetWindowsHookExA<P0>(idhook: WINDOWS_HOOK_ID, lpfn: HOOKPROC, hmod: P0, dwthreadid: u32) -> windows_core::Result<HHOOK>
where
    P0: windows_core::Param<super::super::Foundation::HINSTANCE>,
{
    windows_targets::link!("user32.dll" "system" fn SetWindowsHookExA(idhook : WINDOWS_HOOK_ID, lpfn : HOOKPROC, hmod : super::super::Foundation:: HINSTANCE, dwthreadid : u32) -> HHOOK);
    let result__ = SetWindowsHookExA(idhook, lpfn, hmod.param().abi(), dwthreadid);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn SetWindowsHookExW<P0>(idhook: WINDOWS_HOOK_ID, lpfn: HOOKPROC, hmod: P0, dwthreadid: u32) -> windows_core::Result<HHOOK>
where
    P0: windows_core::Param<super::super::Foundation::HINSTANCE>,
{
    windows_targets::link!("user32.dll" "system" fn SetWindowsHookExW(idhook : WINDOWS_HOOK_ID, lpfn : HOOKPROC, hmod : super::super::Foundation:: HINSTANCE, dwthreadid : u32) -> HHOOK);
    let result__ = SetWindowsHookExW(idhook, lpfn, hmod.param().abi(), dwthreadid);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn SetWindowsHookW(nfiltertype: i32, pfnfilterproc: HOOKPROC) -> HHOOK {
    windows_targets::link!("user32.dll" "system" fn SetWindowsHookW(nfiltertype : i32, pfnfilterproc : HOOKPROC) -> HHOOK);
    SetWindowsHookW(nfiltertype, pfnfilterproc)
}
#[inline]
pub unsafe fn ShowCaret<P0>(hwnd: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn ShowCaret(hwnd : super::super::Foundation:: HWND) -> super::super::Foundation:: BOOL);
    ShowCaret(hwnd.param().abi()).ok()
}
#[inline]
pub unsafe fn ShowCursor<P0>(bshow: P0) -> i32
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("user32.dll" "system" fn ShowCursor(bshow : super::super::Foundation:: BOOL) -> i32);
    ShowCursor(bshow.param().abi())
}
#[inline]
pub unsafe fn ShowOwnedPopups<P0, P1>(hwnd: P0, fshow: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("user32.dll" "system" fn ShowOwnedPopups(hwnd : super::super::Foundation:: HWND, fshow : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    ShowOwnedPopups(hwnd.param().abi(), fshow.param().abi()).ok()
}
#[inline]
pub unsafe fn ShowWindow<P0>(hwnd: P0, ncmdshow: SHOW_WINDOW_CMD) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn ShowWindow(hwnd : super::super::Foundation:: HWND, ncmdshow : SHOW_WINDOW_CMD) -> super::super::Foundation:: BOOL);
    ShowWindow(hwnd.param().abi(), ncmdshow)
}
#[inline]
pub unsafe fn ShowWindowAsync<P0>(hwnd: P0, ncmdshow: SHOW_WINDOW_CMD) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn ShowWindowAsync(hwnd : super::super::Foundation:: HWND, ncmdshow : SHOW_WINDOW_CMD) -> super::super::Foundation:: BOOL);
    ShowWindowAsync(hwnd.param().abi(), ncmdshow)
}
#[inline]
pub unsafe fn SoundSentry() -> super::super::Foundation::BOOL {
    windows_targets::link!("user32.dll" "system" fn SoundSentry() -> super::super::Foundation:: BOOL);
    SoundSentry()
}
#[inline]
pub unsafe fn SwitchToThisWindow<P0, P1>(hwnd: P0, funknown: P1)
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("user32.dll" "system" fn SwitchToThisWindow(hwnd : super::super::Foundation:: HWND, funknown : super::super::Foundation:: BOOL));
    SwitchToThisWindow(hwnd.param().abi(), funknown.param().abi())
}
#[inline]
pub unsafe fn SystemParametersInfoA(uiaction: SYSTEM_PARAMETERS_INFO_ACTION, uiparam: u32, pvparam: Option<*mut core::ffi::c_void>, fwinini: SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn SystemParametersInfoA(uiaction : SYSTEM_PARAMETERS_INFO_ACTION, uiparam : u32, pvparam : *mut core::ffi::c_void, fwinini : SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS) -> super::super::Foundation:: BOOL);
    SystemParametersInfoA(uiaction, uiparam, core::mem::transmute(pvparam.unwrap_or(std::ptr::null_mut())), fwinini).ok()
}
#[inline]
pub unsafe fn SystemParametersInfoW(uiaction: SYSTEM_PARAMETERS_INFO_ACTION, uiparam: u32, pvparam: Option<*mut core::ffi::c_void>, fwinini: SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn SystemParametersInfoW(uiaction : SYSTEM_PARAMETERS_INFO_ACTION, uiparam : u32, pvparam : *mut core::ffi::c_void, fwinini : SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS) -> super::super::Foundation:: BOOL);
    SystemParametersInfoW(uiaction, uiparam, core::mem::transmute(pvparam.unwrap_or(std::ptr::null_mut())), fwinini).ok()
}
#[inline]
pub unsafe fn TileWindows<P0>(hwndparent: P0, whow: TILE_WINDOWS_HOW, lprect: Option<*const super::super::Foundation::RECT>, lpkids: Option<&[super::super::Foundation::HWND]>) -> u16
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn TileWindows(hwndparent : super::super::Foundation:: HWND, whow : TILE_WINDOWS_HOW, lprect : *const super::super::Foundation:: RECT, ckids : u32, lpkids : *const super::super::Foundation:: HWND) -> u16);
    TileWindows(hwndparent.param().abi(), whow, core::mem::transmute(lprect.unwrap_or(std::ptr::null())), lpkids.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lpkids.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
}
#[inline]
pub unsafe fn TrackPopupMenu<P0, P1>(hmenu: P0, uflags: TRACK_POPUP_MENU_FLAGS, x: i32, y: i32, nreserved: i32, hwnd: P1, prcrect: Option<*const super::super::Foundation::RECT>) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<HMENU>,
    P1: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn TrackPopupMenu(hmenu : HMENU, uflags : TRACK_POPUP_MENU_FLAGS, x : i32, y : i32, nreserved : i32, hwnd : super::super::Foundation:: HWND, prcrect : *const super::super::Foundation:: RECT) -> super::super::Foundation:: BOOL);
    TrackPopupMenu(hmenu.param().abi(), uflags, x, y, nreserved, hwnd.param().abi(), core::mem::transmute(prcrect.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn TrackPopupMenuEx<P0, P1>(hmenu: P0, uflags: u32, x: i32, y: i32, hwnd: P1, lptpm: Option<*const TPMPARAMS>) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<HMENU>,
    P1: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn TrackPopupMenuEx(hmenu : HMENU, uflags : u32, x : i32, y : i32, hwnd : super::super::Foundation:: HWND, lptpm : *const TPMPARAMS) -> super::super::Foundation:: BOOL);
    TrackPopupMenuEx(hmenu.param().abi(), uflags, x, y, hwnd.param().abi(), core::mem::transmute(lptpm.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn TranslateAcceleratorA<P0, P1>(hwnd: P0, hacctable: P1, lpmsg: *const MSG) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<HACCEL>,
{
    windows_targets::link!("user32.dll" "system" fn TranslateAcceleratorA(hwnd : super::super::Foundation:: HWND, hacctable : HACCEL, lpmsg : *const MSG) -> i32);
    TranslateAcceleratorA(hwnd.param().abi(), hacctable.param().abi(), lpmsg)
}
#[inline]
pub unsafe fn TranslateAcceleratorW<P0, P1>(hwnd: P0, hacctable: P1, lpmsg: *const MSG) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<HACCEL>,
{
    windows_targets::link!("user32.dll" "system" fn TranslateAcceleratorW(hwnd : super::super::Foundation:: HWND, hacctable : HACCEL, lpmsg : *const MSG) -> i32);
    TranslateAcceleratorW(hwnd.param().abi(), hacctable.param().abi(), lpmsg)
}
#[inline]
pub unsafe fn TranslateMDISysAccel<P0>(hwndclient: P0, lpmsg: *const MSG) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn TranslateMDISysAccel(hwndclient : super::super::Foundation:: HWND, lpmsg : *const MSG) -> super::super::Foundation:: BOOL);
    TranslateMDISysAccel(hwndclient.param().abi(), lpmsg)
}
#[inline]
pub unsafe fn TranslateMessage(lpmsg: *const MSG) -> super::super::Foundation::BOOL {
    windows_targets::link!("user32.dll" "system" fn TranslateMessage(lpmsg : *const MSG) -> super::super::Foundation:: BOOL);
    TranslateMessage(lpmsg)
}
#[inline]
pub unsafe fn UnhookWindowsHook(ncode: i32, pfnfilterproc: HOOKPROC) -> super::super::Foundation::BOOL {
    windows_targets::link!("user32.dll" "system" fn UnhookWindowsHook(ncode : i32, pfnfilterproc : HOOKPROC) -> super::super::Foundation:: BOOL);
    UnhookWindowsHook(ncode, pfnfilterproc)
}
#[inline]
pub unsafe fn UnhookWindowsHookEx<P0>(hhk: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<HHOOK>,
{
    windows_targets::link!("user32.dll" "system" fn UnhookWindowsHookEx(hhk : HHOOK) -> super::super::Foundation:: BOOL);
    UnhookWindowsHookEx(hhk.param().abi()).ok()
}
#[inline]
pub unsafe fn UnregisterClassA<P0, P1>(lpclassname: P0, hinstance: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<super::super::Foundation::HINSTANCE>,
{
    windows_targets::link!("user32.dll" "system" fn UnregisterClassA(lpclassname : windows_core::PCSTR, hinstance : super::super::Foundation:: HINSTANCE) -> super::super::Foundation:: BOOL);
    UnregisterClassA(lpclassname.param().abi(), hinstance.param().abi()).ok()
}
#[inline]
pub unsafe fn UnregisterClassW<P0, P1>(lpclassname: P0, hinstance: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::HINSTANCE>,
{
    windows_targets::link!("user32.dll" "system" fn UnregisterClassW(lpclassname : windows_core::PCWSTR, hinstance : super::super::Foundation:: HINSTANCE) -> super::super::Foundation:: BOOL);
    UnregisterClassW(lpclassname.param().abi(), hinstance.param().abi()).ok()
}
#[inline]
pub unsafe fn UnregisterDeviceNotification<P0>(handle: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<HDEVNOTIFY>,
{
    windows_targets::link!("user32.dll" "system" fn UnregisterDeviceNotification(handle : HDEVNOTIFY) -> super::super::Foundation:: BOOL);
    UnregisterDeviceNotification(handle.param().abi()).ok()
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn UpdateLayeredWindow<P0, P1, P2, P3>(hwnd: P0, hdcdst: P1, pptdst: Option<*const super::super::Foundation::POINT>, psize: Option<*const super::super::Foundation::SIZE>, hdcsrc: P2, pptsrc: Option<*const super::super::Foundation::POINT>, crkey: P3, pblend: Option<*const super::super::Graphics::Gdi::BLENDFUNCTION>, dwflags: UPDATE_LAYERED_WINDOW_FLAGS) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Graphics::Gdi::HDC>,
    P2: windows_core::Param<super::super::Graphics::Gdi::HDC>,
    P3: windows_core::Param<super::super::Foundation::COLORREF>,
{
    windows_targets::link!("user32.dll" "system" fn UpdateLayeredWindow(hwnd : super::super::Foundation:: HWND, hdcdst : super::super::Graphics::Gdi:: HDC, pptdst : *const super::super::Foundation:: POINT, psize : *const super::super::Foundation:: SIZE, hdcsrc : super::super::Graphics::Gdi:: HDC, pptsrc : *const super::super::Foundation:: POINT, crkey : super::super::Foundation:: COLORREF, pblend : *const super::super::Graphics::Gdi:: BLENDFUNCTION, dwflags : UPDATE_LAYERED_WINDOW_FLAGS) -> super::super::Foundation:: BOOL);
    UpdateLayeredWindow(hwnd.param().abi(), hdcdst.param().abi(), core::mem::transmute(pptdst.unwrap_or(std::ptr::null())), core::mem::transmute(psize.unwrap_or(std::ptr::null())), hdcsrc.param().abi(), core::mem::transmute(pptsrc.unwrap_or(std::ptr::null())), crkey.param().abi(), core::mem::transmute(pblend.unwrap_or(std::ptr::null())), dwflags).ok()
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn UpdateLayeredWindowIndirect<P0>(hwnd: P0, pulwinfo: *const UPDATELAYEREDWINDOWINFO) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn UpdateLayeredWindowIndirect(hwnd : super::super::Foundation:: HWND, pulwinfo : *const UPDATELAYEREDWINDOWINFO) -> super::super::Foundation:: BOOL);
    UpdateLayeredWindowIndirect(hwnd.param().abi(), pulwinfo)
}
#[inline]
pub unsafe fn WaitMessage() -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn WaitMessage() -> super::super::Foundation:: BOOL);
    WaitMessage().ok()
}
#[inline]
pub unsafe fn WindowFromPhysicalPoint(point: super::super::Foundation::POINT) -> super::super::Foundation::HWND {
    windows_targets::link!("user32.dll" "system" fn WindowFromPhysicalPoint(point : super::super::Foundation:: POINT) -> super::super::Foundation:: HWND);
    WindowFromPhysicalPoint(core::mem::transmute(point))
}
#[inline]
pub unsafe fn WindowFromPoint(point: super::super::Foundation::POINT) -> super::super::Foundation::HWND {
    windows_targets::link!("user32.dll" "system" fn WindowFromPoint(point : super::super::Foundation:: POINT) -> super::super::Foundation:: HWND);
    WindowFromPoint(core::mem::transmute(point))
}
#[inline]
pub unsafe fn wsprintfA<P0>(param0: windows_core::PSTR, param1: P0) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("user32.dll" "cdecl" fn wsprintfA(param0 : windows_core::PSTR, param1 : windows_core::PCSTR) -> i32);
    wsprintfA(core::mem::transmute(param0), param1.param().abi())
}
#[inline]
pub unsafe fn wsprintfW<P0>(param0: windows_core::PWSTR, param1: P0) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("user32.dll" "cdecl" fn wsprintfW(param0 : windows_core::PWSTR, param1 : windows_core::PCWSTR) -> i32);
    wsprintfW(core::mem::transmute(param0), param1.param().abi())
}
#[inline]
pub unsafe fn wvsprintfA<P0>(param0: windows_core::PSTR, param1: P0, arglist: *const i8) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("user32.dll" "system" fn wvsprintfA(param0 : windows_core::PSTR, param1 : windows_core::PCSTR, arglist : *const i8) -> i32);
    wvsprintfA(core::mem::transmute(param0), param1.param().abi(), arglist)
}
#[inline]
pub unsafe fn wvsprintfW<P0>(param0: windows_core::PWSTR, param1: P0, arglist: *const i8) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("user32.dll" "system" fn wvsprintfW(param0 : windows_core::PWSTR, param1 : windows_core::PCWSTR, arglist : *const i8) -> i32);
    wvsprintfW(core::mem::transmute(param0), param1.param().abi(), arglist)
}
pub const ARW_BOTTOMLEFT: MINIMIZEDMETRICS_ARRANGE = MINIMIZEDMETRICS_ARRANGE(0i32);
pub const ARW_BOTTOMRIGHT: MINIMIZEDMETRICS_ARRANGE = MINIMIZEDMETRICS_ARRANGE(1i32);
pub const ARW_DOWN: i32 = 4i32;
pub const ARW_HIDE: i32 = 8i32;
pub const ARW_LEFT: i32 = 0i32;
pub const ARW_RIGHT: i32 = 0i32;
pub const ARW_STARTMASK: i32 = 3i32;
pub const ARW_STARTRIGHT: i32 = 1i32;
pub const ARW_STARTTOP: i32 = 2i32;
pub const ARW_TOPLEFT: MINIMIZEDMETRICS_ARRANGE = MINIMIZEDMETRICS_ARRANGE(2i32);
pub const ARW_TOPRIGHT: MINIMIZEDMETRICS_ARRANGE = MINIMIZEDMETRICS_ARRANGE(3i32);
pub const ARW_UP: i32 = 4i32;
pub const ASFW_ANY: u32 = 4294967295u32;
pub const AW_ACTIVATE: ANIMATE_WINDOW_FLAGS = ANIMATE_WINDOW_FLAGS(131072u32);
pub const AW_BLEND: ANIMATE_WINDOW_FLAGS = ANIMATE_WINDOW_FLAGS(524288u32);
pub const AW_CENTER: ANIMATE_WINDOW_FLAGS = ANIMATE_WINDOW_FLAGS(16u32);
pub const AW_HIDE: ANIMATE_WINDOW_FLAGS = ANIMATE_WINDOW_FLAGS(65536u32);
pub const AW_HOR_NEGATIVE: ANIMATE_WINDOW_FLAGS = ANIMATE_WINDOW_FLAGS(2u32);
pub const AW_HOR_POSITIVE: ANIMATE_WINDOW_FLAGS = ANIMATE_WINDOW_FLAGS(1u32);
pub const AW_SLIDE: ANIMATE_WINDOW_FLAGS = ANIMATE_WINDOW_FLAGS(262144u32);
pub const AW_VER_NEGATIVE: ANIMATE_WINDOW_FLAGS = ANIMATE_WINDOW_FLAGS(8u32);
pub const AW_VER_POSITIVE: ANIMATE_WINDOW_FLAGS = ANIMATE_WINDOW_FLAGS(4u32);
pub const BM_CLICK: u32 = 245u32;
pub const BM_GETCHECK: u32 = 240u32;
pub const BM_GETIMAGE: u32 = 246u32;
pub const BM_GETSTATE: u32 = 242u32;
pub const BM_SETCHECK: u32 = 241u32;
pub const BM_SETDONTCLICK: u32 = 248u32;
pub const BM_SETIMAGE: u32 = 247u32;
pub const BM_SETSTATE: u32 = 243u32;
pub const BM_SETSTYLE: u32 = 244u32;
pub const BN_CLICKED: u32 = 0u32;
pub const BN_DBLCLK: u32 = 5u32;
pub const BN_DISABLE: u32 = 4u32;
pub const BN_DOUBLECLICKED: u32 = 5u32;
pub const BN_HILITE: u32 = 2u32;
pub const BN_KILLFOCUS: u32 = 7u32;
pub const BN_PAINT: u32 = 1u32;
pub const BN_PUSHED: u32 = 2u32;
pub const BN_SETFOCUS: u32 = 6u32;
pub const BN_UNHILITE: u32 = 3u32;
pub const BN_UNPUSHED: u32 = 3u32;
pub const BROADCAST_QUERY_DENY: u32 = 1112363332u32;
pub const BSF_MSGSRV32ISOK: u32 = 2147483648u32;
pub const BSF_MSGSRV32ISOK_BIT: u32 = 31u32;
pub const BSM_INSTALLABLEDRIVERS: u32 = 4u32;
pub const BSM_NETDRIVER: u32 = 2u32;
pub const BSM_VXDS: u32 = 1u32;
pub const BST_FOCUS: u32 = 8u32;
pub const BST_PUSHED: u32 = 4u32;
pub const BS_3STATE: i32 = 5i32;
pub const BS_AUTO3STATE: i32 = 6i32;
pub const BS_AUTOCHECKBOX: i32 = 3i32;
pub const BS_AUTORADIOBUTTON: i32 = 9i32;
pub const BS_BITMAP: i32 = 128i32;
pub const BS_BOTTOM: i32 = 2048i32;
pub const BS_CENTER: i32 = 768i32;
pub const BS_CHECKBOX: i32 = 2i32;
pub const BS_DEFPUSHBUTTON: i32 = 1i32;
pub const BS_FLAT: i32 = 32768i32;
pub const BS_GROUPBOX: i32 = 7i32;
pub const BS_ICON: i32 = 64i32;
pub const BS_LEFT: i32 = 256i32;
pub const BS_LEFTTEXT: i32 = 32i32;
pub const BS_MULTILINE: i32 = 8192i32;
pub const BS_NOTIFY: i32 = 16384i32;
pub const BS_OWNERDRAW: i32 = 11i32;
pub const BS_PUSHBOX: i32 = 10i32;
pub const BS_PUSHBUTTON: i32 = 0i32;
pub const BS_PUSHLIKE: i32 = 4096i32;
pub const BS_RADIOBUTTON: i32 = 4i32;
pub const BS_RIGHT: i32 = 512i32;
pub const BS_RIGHTBUTTON: i32 = 32i32;
pub const BS_TEXT: i32 = 0i32;
pub const BS_TOP: i32 = 1024i32;
pub const BS_TYPEMASK: i32 = 15i32;
pub const BS_USERBUTTON: i32 = 8i32;
pub const BS_VCENTER: i32 = 3072i32;
pub const CALERT_SYSTEM: u32 = 6u32;
pub const CBN_CLOSEUP: u32 = 8u32;
pub const CBN_DBLCLK: u32 = 2u32;
pub const CBN_DROPDOWN: u32 = 7u32;
pub const CBN_EDITCHANGE: u32 = 5u32;
pub const CBN_EDITUPDATE: u32 = 6u32;
pub const CBN_ERRSPACE: i32 = -1i32;
pub const CBN_KILLFOCUS: u32 = 4u32;
pub const CBN_SELCHANGE: u32 = 1u32;
pub const CBN_SELENDCANCEL: u32 = 10u32;
pub const CBN_SELENDOK: u32 = 9u32;
pub const CBN_SETFOCUS: u32 = 3u32;
pub const CBS_AUTOHSCROLL: i32 = 64i32;
pub const CBS_DISABLENOSCROLL: i32 = 2048i32;
pub const CBS_DROPDOWN: i32 = 2i32;
pub const CBS_DROPDOWNLIST: i32 = 3i32;
pub const CBS_HASSTRINGS: i32 = 512i32;
pub const CBS_LOWERCASE: i32 = 16384i32;
pub const CBS_NOINTEGRALHEIGHT: i32 = 1024i32;
pub const CBS_OEMCONVERT: i32 = 128i32;
pub const CBS_OWNERDRAWFIXED: i32 = 16i32;
pub const CBS_OWNERDRAWVARIABLE: i32 = 32i32;
pub const CBS_SIMPLE: i32 = 1i32;
pub const CBS_SORT: i32 = 256i32;
pub const CBS_UPPERCASE: i32 = 8192i32;
pub const CB_ADDSTRING: u32 = 323u32;
pub const CB_DELETESTRING: u32 = 324u32;
pub const CB_DIR: u32 = 325u32;
pub const CB_ERR: i32 = -1i32;
pub const CB_ERRSPACE: i32 = -2i32;
pub const CB_FINDSTRING: u32 = 332u32;
pub const CB_FINDSTRINGEXACT: u32 = 344u32;
pub const CB_GETCOMBOBOXINFO: u32 = 356u32;
pub const CB_GETCOUNT: u32 = 326u32;
pub const CB_GETCURSEL: u32 = 327u32;
pub const CB_GETDROPPEDCONTROLRECT: u32 = 338u32;
pub const CB_GETDROPPEDSTATE: u32 = 343u32;
pub const CB_GETDROPPEDWIDTH: u32 = 351u32;
pub const CB_GETEDITSEL: u32 = 320u32;
pub const CB_GETEXTENDEDUI: u32 = 342u32;
pub const CB_GETHORIZONTALEXTENT: u32 = 349u32;
pub const CB_GETITEMDATA: u32 = 336u32;
pub const CB_GETITEMHEIGHT: u32 = 340u32;
pub const CB_GETLBTEXT: u32 = 328u32;
pub const CB_GETLBTEXTLEN: u32 = 329u32;
pub const CB_GETLOCALE: u32 = 346u32;
pub const CB_GETTOPINDEX: u32 = 347u32;
pub const CB_INITSTORAGE: u32 = 353u32;
pub const CB_INSERTSTRING: u32 = 330u32;
pub const CB_LIMITTEXT: u32 = 321u32;
pub const CB_MSGMAX: u32 = 357u32;
pub const CB_MULTIPLEADDSTRING: u32 = 355u32;
pub const CB_OKAY: u32 = 0u32;
pub const CB_RESETCONTENT: u32 = 331u32;
pub const CB_SELECTSTRING: u32 = 333u32;
pub const CB_SETCURSEL: u32 = 334u32;
pub const CB_SETDROPPEDWIDTH: u32 = 352u32;
pub const CB_SETEDITSEL: u32 = 322u32;
pub const CB_SETEXTENDEDUI: u32 = 341u32;
pub const CB_SETHORIZONTALEXTENT: u32 = 350u32;
pub const CB_SETITEMDATA: u32 = 337u32;
pub const CB_SETITEMHEIGHT: u32 = 339u32;
pub const CB_SETLOCALE: u32 = 345u32;
pub const CB_SETTOPINDEX: u32 = 348u32;
pub const CB_SHOWDROPDOWN: u32 = 335u32;
pub const CCHILDREN_SCROLLBAR: u32 = 5u32;
pub const CCHILDREN_TITLEBAR: u32 = 5u32;
pub const CHILDID_SELF: u32 = 0u32;
pub const CONSOLE_APPLICATION_16BIT: u32 = 0u32;
pub const CONSOLE_CARET_SELECTION: u32 = 1u32;
pub const CONSOLE_CARET_VISIBLE: u32 = 2u32;
pub const CONTACTVISUALIZATION_OFF: u32 = 0u32;
pub const CONTACTVISUALIZATION_ON: u32 = 1u32;
pub const CONTACTVISUALIZATION_PRESENTATIONMODE: u32 = 2u32;
pub const CREATEPROCESS_MANIFEST_RESOURCE_ID: u32 = 1u32;
pub const CSOUND_SYSTEM: u32 = 16u32;
pub const CS_BYTEALIGNCLIENT: WNDCLASS_STYLES = WNDCLASS_STYLES(4096u32);
pub const CS_BYTEALIGNWINDOW: WNDCLASS_STYLES = WNDCLASS_STYLES(8192u32);
pub const CS_CLASSDC: WNDCLASS_STYLES = WNDCLASS_STYLES(64u32);
pub const CS_DBLCLKS: WNDCLASS_STYLES = WNDCLASS_STYLES(8u32);
pub const CS_DROPSHADOW: WNDCLASS_STYLES = WNDCLASS_STYLES(131072u32);
pub const CS_GLOBALCLASS: WNDCLASS_STYLES = WNDCLASS_STYLES(16384u32);
pub const CS_HREDRAW: WNDCLASS_STYLES = WNDCLASS_STYLES(2u32);
pub const CS_IME: WNDCLASS_STYLES = WNDCLASS_STYLES(65536u32);
pub const CS_NOCLOSE: WNDCLASS_STYLES = WNDCLASS_STYLES(512u32);
pub const CS_OWNDC: WNDCLASS_STYLES = WNDCLASS_STYLES(32u32);
pub const CS_PARENTDC: WNDCLASS_STYLES = WNDCLASS_STYLES(128u32);
pub const CS_SAVEBITS: WNDCLASS_STYLES = WNDCLASS_STYLES(2048u32);
pub const CS_VREDRAW: WNDCLASS_STYLES = WNDCLASS_STYLES(1u32);
pub const CTLCOLOR_BTN: u32 = 3u32;
pub const CTLCOLOR_DLG: u32 = 4u32;
pub const CTLCOLOR_EDIT: u32 = 1u32;
pub const CTLCOLOR_LISTBOX: u32 = 2u32;
pub const CTLCOLOR_MAX: u32 = 7u32;
pub const CTLCOLOR_MSGBOX: u32 = 0u32;
pub const CTLCOLOR_SCROLLBAR: u32 = 5u32;
pub const CTLCOLOR_STATIC: u32 = 6u32;
pub const CURSOR_CREATION_SCALING_DEFAULT: u32 = 2u32;
pub const CURSOR_CREATION_SCALING_NONE: u32 = 1u32;
pub const CURSOR_SHOWING: CURSORINFO_FLAGS = CURSORINFO_FLAGS(1u32);
pub const CURSOR_SUPPRESSED: CURSORINFO_FLAGS = CURSORINFO_FLAGS(2u32);
pub const CWF_CREATE_ONLY: u32 = 1u32;
pub const CWP_ALL: CWP_FLAGS = CWP_FLAGS(0u32);
pub const CWP_SKIPDISABLED: CWP_FLAGS = CWP_FLAGS(2u32);
pub const CWP_SKIPINVISIBLE: CWP_FLAGS = CWP_FLAGS(1u32);
pub const CWP_SKIPTRANSPARENT: CWP_FLAGS = CWP_FLAGS(4u32);
pub const CW_USEDEFAULT: i32 = -2147483648i32;
pub const DBTF_MEDIA: DEV_BROADCAST_VOLUME_FLAGS = DEV_BROADCAST_VOLUME_FLAGS(1u16);
pub const DBTF_NET: DEV_BROADCAST_VOLUME_FLAGS = DEV_BROADCAST_VOLUME_FLAGS(2u16);
pub const DBTF_RESOURCE: u32 = 1u32;
pub const DBTF_SLOWNET: u32 = 4u32;
pub const DBTF_XPORT: u32 = 2u32;
pub const DBT_APPYBEGIN: u32 = 0u32;
pub const DBT_APPYEND: u32 = 1u32;
pub const DBT_CONFIGCHANGECANCELED: u32 = 25u32;
pub const DBT_CONFIGCHANGED: u32 = 24u32;
pub const DBT_CONFIGMGAPI32: u32 = 34u32;
pub const DBT_CONFIGMGPRIVATE: u32 = 32767u32;
pub const DBT_CUSTOMEVENT: u32 = 32774u32;
pub const DBT_DEVICEARRIVAL: u32 = 32768u32;
pub const DBT_DEVICEQUERYREMOVE: u32 = 32769u32;
pub const DBT_DEVICEQUERYREMOVEFAILED: u32 = 32770u32;
pub const DBT_DEVICEREMOVECOMPLETE: u32 = 32772u32;
pub const DBT_DEVICEREMOVEPENDING: u32 = 32771u32;
pub const DBT_DEVICETYPESPECIFIC: u32 = 32773u32;
pub const DBT_DEVNODES_CHANGED: u32 = 7u32;
pub const DBT_DEVTYP_DEVICEINTERFACE: DEV_BROADCAST_HDR_DEVICE_TYPE = DEV_BROADCAST_HDR_DEVICE_TYPE(5u32);
pub const DBT_DEVTYP_DEVNODE: u32 = 1u32;
pub const DBT_DEVTYP_HANDLE: DEV_BROADCAST_HDR_DEVICE_TYPE = DEV_BROADCAST_HDR_DEVICE_TYPE(6u32);
pub const DBT_DEVTYP_NET: u32 = 4u32;
pub const DBT_DEVTYP_OEM: DEV_BROADCAST_HDR_DEVICE_TYPE = DEV_BROADCAST_HDR_DEVICE_TYPE(0u32);
pub const DBT_DEVTYP_PORT: DEV_BROADCAST_HDR_DEVICE_TYPE = DEV_BROADCAST_HDR_DEVICE_TYPE(3u32);
pub const DBT_DEVTYP_VOLUME: DEV_BROADCAST_HDR_DEVICE_TYPE = DEV_BROADCAST_HDR_DEVICE_TYPE(2u32);
pub const DBT_LOW_DISK_SPACE: u32 = 72u32;
pub const DBT_MONITORCHANGE: u32 = 27u32;
pub const DBT_NO_DISK_SPACE: u32 = 71u32;
pub const DBT_QUERYCHANGECONFIG: u32 = 23u32;
pub const DBT_SHELLLOGGEDON: u32 = 32u32;
pub const DBT_USERDEFINED: u32 = 65535u32;
pub const DBT_VOLLOCKLOCKFAILED: u32 = 32835u32;
pub const DBT_VOLLOCKLOCKRELEASED: u32 = 32837u32;
pub const DBT_VOLLOCKLOCKTAKEN: u32 = 32834u32;
pub const DBT_VOLLOCKQUERYLOCK: u32 = 32833u32;
pub const DBT_VOLLOCKQUERYUNLOCK: u32 = 32836u32;
pub const DBT_VOLLOCKUNLOCKFAILED: u32 = 32838u32;
pub const DBT_VPOWERDAPI: u32 = 33024u32;
pub const DBT_VXDINITCOMPLETE: u32 = 35u32;
pub const DCX_EXCLUDEUPDATE: i32 = 256i32;
pub const DC_HASDEFID: u32 = 21323u32;
pub const DEVICE_NOTIFY_ALL_INTERFACE_CLASSES: REGISTER_NOTIFICATION_FLAGS = REGISTER_NOTIFICATION_FLAGS(4u32);
pub const DEVICE_NOTIFY_CALLBACK: REGISTER_NOTIFICATION_FLAGS = REGISTER_NOTIFICATION_FLAGS(2u32);
pub const DEVICE_NOTIFY_SERVICE_HANDLE: REGISTER_NOTIFICATION_FLAGS = REGISTER_NOTIFICATION_FLAGS(1u32);
pub const DEVICE_NOTIFY_WINDOW_HANDLE: REGISTER_NOTIFICATION_FLAGS = REGISTER_NOTIFICATION_FLAGS(0u32);
pub const DIFFERENCE: u32 = 11u32;
pub const DI_COMPAT: DI_FLAGS = DI_FLAGS(4u32);
pub const DI_DEFAULTSIZE: DI_FLAGS = DI_FLAGS(8u32);
pub const DI_IMAGE: DI_FLAGS = DI_FLAGS(2u32);
pub const DI_MASK: DI_FLAGS = DI_FLAGS(1u32);
pub const DI_NOMIRROR: DI_FLAGS = DI_FLAGS(16u32);
pub const DI_NORMAL: DI_FLAGS = DI_FLAGS(3u32);
pub const DLGC_BUTTON: u32 = 8192u32;
pub const DLGC_DEFPUSHBUTTON: u32 = 16u32;
pub const DLGC_HASSETSEL: u32 = 8u32;
pub const DLGC_RADIOBUTTON: u32 = 64u32;
pub const DLGC_STATIC: u32 = 256u32;
pub const DLGC_UNDEFPUSHBUTTON: u32 = 32u32;
pub const DLGC_WANTALLKEYS: u32 = 4u32;
pub const DLGC_WANTARROWS: u32 = 1u32;
pub const DLGC_WANTCHARS: u32 = 128u32;
pub const DLGC_WANTMESSAGE: u32 = 4u32;
pub const DLGC_WANTTAB: u32 = 2u32;
pub const DLGWINDOWEXTRA: u32 = 30u32;
pub const DM_GETDEFID: u32 = 1024u32;
pub const DM_POINTERHITTEST: u32 = 592u32;
pub const DM_REPOSITION: u32 = 1026u32;
pub const DM_SETDEFID: u32 = 1025u32;
pub const DOF_DIRECTORY: u32 = 32771u32;
pub const DOF_DOCUMENT: u32 = 32770u32;
pub const DOF_EXECUTABLE: u32 = 32769u32;
pub const DOF_MULTIPLE: u32 = 32772u32;
pub const DOF_PROGMAN: u32 = 1u32;
pub const DOF_SHELLDATA: u32 = 2u32;
pub const DO_DROPFILE: i32 = 1162627398i32;
pub const DO_PRINTFILE: i32 = 1414419024i32;
pub const DS_3DLOOK: i32 = 4i32;
pub const DS_ABSALIGN: i32 = 1i32;
pub const DS_CENTER: i32 = 2048i32;
pub const DS_CENTERMOUSE: i32 = 4096i32;
pub const DS_CONTEXTHELP: i32 = 8192i32;
pub const DS_CONTROL: i32 = 1024i32;
pub const DS_FIXEDSYS: i32 = 8i32;
pub const DS_LOCALEDIT: i32 = 32i32;
pub const DS_MODALFRAME: i32 = 128i32;
pub const DS_NOFAILCREATE: i32 = 16i32;
pub const DS_NOIDLEMSG: i32 = 256i32;
pub const DS_SETFONT: i32 = 64i32;
pub const DS_SETFOREGROUND: i32 = 512i32;
pub const DS_SYSMODAL: i32 = 2i32;
pub const DS_USEPIXELS: i32 = 32768i32;
pub const DWLP_MSGRESULT: u32 = 0u32;
pub const DWL_DLGPROC: u32 = 4u32;
pub const DWL_MSGRESULT: u32 = 0u32;
pub const DWL_USER: u32 = 8u32;
pub const EC_LEFTMARGIN: u32 = 1u32;
pub const EC_RIGHTMARGIN: u32 = 2u32;
pub const EC_USEFONTINFO: u32 = 65535u32;
pub const EDD_GET_DEVICE_INTERFACE_NAME: u32 = 1u32;
pub const EDIT_CONTROL_FEATURE_ENTERPRISE_DATA_PROTECTION_PASTE_SUPPORT: EDIT_CONTROL_FEATURE = EDIT_CONTROL_FEATURE(0i32);
pub const EDIT_CONTROL_FEATURE_PASTE_NOTIFICATIONS: EDIT_CONTROL_FEATURE = EDIT_CONTROL_FEATURE(1i32);
pub const EIMES_CANCELCOMPSTRINFOCUS: u32 = 2u32;
pub const EIMES_COMPLETECOMPSTRKILLFOCUS: u32 = 4u32;
pub const EIMES_GETCOMPSTRATONCE: u32 = 1u32;
pub const EMSIS_COMPOSITIONSTRING: u32 = 1u32;
pub const ENDSESSION_CLOSEAPP: u32 = 1u32;
pub const ENDSESSION_CRITICAL: u32 = 1073741824u32;
pub const ENDSESSION_LOGOFF: u32 = 2147483648u32;
pub const EN_AFTER_PASTE: u32 = 2049u32;
pub const EN_ALIGN_LTR_EC: u32 = 1792u32;
pub const EN_ALIGN_RTL_EC: u32 = 1793u32;
pub const EN_BEFORE_PASTE: u32 = 2048u32;
pub const EN_CHANGE: u32 = 768u32;
pub const EN_ERRSPACE: u32 = 1280u32;
pub const EN_HSCROLL: u32 = 1537u32;
pub const EN_KILLFOCUS: u32 = 512u32;
pub const EN_MAXTEXT: u32 = 1281u32;
pub const EN_SETFOCUS: u32 = 256u32;
pub const EN_UPDATE: u32 = 1024u32;
pub const EN_VSCROLL: u32 = 1538u32;
pub const ES_AUTOHSCROLL: i32 = 128i32;
pub const ES_AUTOVSCROLL: i32 = 64i32;
pub const ES_CENTER: i32 = 1i32;
pub const ES_LEFT: i32 = 0i32;
pub const ES_LOWERCASE: i32 = 16i32;
pub const ES_MULTILINE: i32 = 4i32;
pub const ES_NOHIDESEL: i32 = 256i32;
pub const ES_NUMBER: i32 = 8192i32;
pub const ES_OEMCONVERT: i32 = 1024i32;
pub const ES_PASSWORD: i32 = 32i32;
pub const ES_READONLY: i32 = 2048i32;
pub const ES_RIGHT: i32 = 2i32;
pub const ES_UPPERCASE: i32 = 8i32;
pub const ES_WANTRETURN: i32 = 4096i32;
pub const EVENT_AIA_END: u32 = 45055u32;
pub const EVENT_AIA_START: u32 = 40960u32;
pub const EVENT_CONSOLE_CARET: u32 = 16385u32;
pub const EVENT_CONSOLE_END: u32 = 16639u32;
pub const EVENT_CONSOLE_END_APPLICATION: u32 = 16391u32;
pub const EVENT_CONSOLE_LAYOUT: u32 = 16389u32;
pub const EVENT_CONSOLE_START_APPLICATION: u32 = 16390u32;
pub const EVENT_CONSOLE_UPDATE_REGION: u32 = 16386u32;
pub const EVENT_CONSOLE_UPDATE_SCROLL: u32 = 16388u32;
pub const EVENT_CONSOLE_UPDATE_SIMPLE: u32 = 16387u32;
pub const EVENT_MAX: u32 = 2147483647u32;
pub const EVENT_MIN: u32 = 1u32;
pub const EVENT_OBJECT_ACCELERATORCHANGE: u32 = 32786u32;
pub const EVENT_OBJECT_CLOAKED: u32 = 32791u32;
pub const EVENT_OBJECT_CONTENTSCROLLED: u32 = 32789u32;
pub const EVENT_OBJECT_CREATE: u32 = 32768u32;
pub const EVENT_OBJECT_DEFACTIONCHANGE: u32 = 32785u32;
pub const EVENT_OBJECT_DESCRIPTIONCHANGE: u32 = 32781u32;
pub const EVENT_OBJECT_DESTROY: u32 = 32769u32;
pub const EVENT_OBJECT_DRAGCANCEL: u32 = 32802u32;
pub const EVENT_OBJECT_DRAGCOMPLETE: u32 = 32803u32;
pub const EVENT_OBJECT_DRAGDROPPED: u32 = 32806u32;
pub const EVENT_OBJECT_DRAGENTER: u32 = 32804u32;
pub const EVENT_OBJECT_DRAGLEAVE: u32 = 32805u32;
pub const EVENT_OBJECT_DRAGSTART: u32 = 32801u32;
pub const EVENT_OBJECT_END: u32 = 33023u32;
pub const EVENT_OBJECT_FOCUS: u32 = 32773u32;
pub const EVENT_OBJECT_HELPCHANGE: u32 = 32784u32;
pub const EVENT_OBJECT_HIDE: u32 = 32771u32;
pub const EVENT_OBJECT_HOSTEDOBJECTSINVALIDATED: u32 = 32800u32;
pub const EVENT_OBJECT_IME_CHANGE: u32 = 32809u32;
pub const EVENT_OBJECT_IME_HIDE: u32 = 32808u32;
pub const EVENT_OBJECT_IME_SHOW: u32 = 32807u32;
pub const EVENT_OBJECT_INVOKED: u32 = 32787u32;
pub const EVENT_OBJECT_LIVEREGIONCHANGED: u32 = 32793u32;
pub const EVENT_OBJECT_LOCATIONCHANGE: u32 = 32779u32;
pub const EVENT_OBJECT_NAMECHANGE: u32 = 32780u32;
pub const EVENT_OBJECT_PARENTCHANGE: u32 = 32783u32;
pub const EVENT_OBJECT_REORDER: u32 = 32772u32;
pub const EVENT_OBJECT_SELECTION: u32 = 32774u32;
pub const EVENT_OBJECT_SELECTIONADD: u32 = 32775u32;
pub const EVENT_OBJECT_SELECTIONREMOVE: u32 = 32776u32;
pub const EVENT_OBJECT_SELECTIONWITHIN: u32 = 32777u32;
pub const EVENT_OBJECT_SHOW: u32 = 32770u32;
pub const EVENT_OBJECT_STATECHANGE: u32 = 32778u32;
pub const EVENT_OBJECT_TEXTEDIT_CONVERSIONTARGETCHANGED: u32 = 32816u32;
pub const EVENT_OBJECT_TEXTSELECTIONCHANGED: u32 = 32788u32;
pub const EVENT_OBJECT_UNCLOAKED: u32 = 32792u32;
pub const EVENT_OBJECT_VALUECHANGE: u32 = 32782u32;
pub const EVENT_OEM_DEFINED_END: u32 = 511u32;
pub const EVENT_OEM_DEFINED_START: u32 = 257u32;
pub const EVENT_SYSTEM_ALERT: u32 = 2u32;
pub const EVENT_SYSTEM_ARRANGMENTPREVIEW: u32 = 32790u32;
pub const EVENT_SYSTEM_CAPTUREEND: u32 = 9u32;
pub const EVENT_SYSTEM_CAPTURESTART: u32 = 8u32;
pub const EVENT_SYSTEM_CONTEXTHELPEND: u32 = 13u32;
pub const EVENT_SYSTEM_CONTEXTHELPSTART: u32 = 12u32;
pub const EVENT_SYSTEM_DESKTOPSWITCH: u32 = 32u32;
pub const EVENT_SYSTEM_DIALOGEND: u32 = 17u32;
pub const EVENT_SYSTEM_DIALOGSTART: u32 = 16u32;
pub const EVENT_SYSTEM_DRAGDROPEND: u32 = 15u32;
pub const EVENT_SYSTEM_DRAGDROPSTART: u32 = 14u32;
pub const EVENT_SYSTEM_END: u32 = 255u32;
pub const EVENT_SYSTEM_FOREGROUND: u32 = 3u32;
pub const EVENT_SYSTEM_IME_KEY_NOTIFICATION: u32 = 41u32;
pub const EVENT_SYSTEM_MENUEND: u32 = 5u32;
pub const EVENT_SYSTEM_MENUPOPUPEND: u32 = 7u32;
pub const EVENT_SYSTEM_MENUPOPUPSTART: u32 = 6u32;
pub const EVENT_SYSTEM_MENUSTART: u32 = 4u32;
pub const EVENT_SYSTEM_MINIMIZEEND: u32 = 23u32;
pub const EVENT_SYSTEM_MINIMIZESTART: u32 = 22u32;
pub const EVENT_SYSTEM_MOVESIZEEND: u32 = 11u32;
pub const EVENT_SYSTEM_MOVESIZESTART: u32 = 10u32;
pub const EVENT_SYSTEM_SCROLLINGEND: u32 = 19u32;
pub const EVENT_SYSTEM_SCROLLINGSTART: u32 = 18u32;
pub const EVENT_SYSTEM_SOUND: u32 = 1u32;
pub const EVENT_SYSTEM_SWITCHEND: u32 = 21u32;
pub const EVENT_SYSTEM_SWITCHER_APPDROPPED: u32 = 38u32;
pub const EVENT_SYSTEM_SWITCHER_APPGRABBED: u32 = 36u32;
pub const EVENT_SYSTEM_SWITCHER_APPOVERTARGET: u32 = 37u32;
pub const EVENT_SYSTEM_SWITCHER_CANCELLED: u32 = 39u32;
pub const EVENT_SYSTEM_SWITCHSTART: u32 = 20u32;
pub const EVENT_UIA_EVENTID_END: u32 = 20223u32;
pub const EVENT_UIA_EVENTID_START: u32 = 19968u32;
pub const EVENT_UIA_PROPID_END: u32 = 30207u32;
pub const EVENT_UIA_PROPID_START: u32 = 29952u32;
pub const FALT: ACCEL_VIRT_FLAGS = ACCEL_VIRT_FLAGS(16u8);
pub const FAPPCOMMAND_KEY: u32 = 0u32;
pub const FAPPCOMMAND_MASK: u32 = 61440u32;
pub const FAPPCOMMAND_MOUSE: u32 = 32768u32;
pub const FAPPCOMMAND_OEM: u32 = 4096u32;
pub const FCONTROL: ACCEL_VIRT_FLAGS = ACCEL_VIRT_FLAGS(8u8);
pub const FE_FONTSMOOTHINGCLEARTYPE: u32 = 2u32;
pub const FE_FONTSMOOTHINGORIENTATIONBGR: u32 = 0u32;
pub const FE_FONTSMOOTHINGORIENTATIONRGB: u32 = 1u32;
pub const FE_FONTSMOOTHINGSTANDARD: u32 = 1u32;
pub const FKF_AVAILABLE: u32 = 2u32;
pub const FKF_CLICKON: u32 = 64u32;
pub const FKF_CONFIRMHOTKEY: u32 = 8u32;
pub const FKF_FILTERKEYSON: u32 = 1u32;
pub const FKF_HOTKEYACTIVE: u32 = 4u32;
pub const FKF_HOTKEYSOUND: u32 = 16u32;
pub const FKF_INDICATOR: u32 = 32u32;
pub const FLASHW_ALL: FLASHWINFO_FLAGS = FLASHWINFO_FLAGS(3u32);
pub const FLASHW_CAPTION: FLASHWINFO_FLAGS = FLASHWINFO_FLAGS(1u32);
pub const FLASHW_STOP: FLASHWINFO_FLAGS = FLASHWINFO_FLAGS(0u32);
pub const FLASHW_TIMER: FLASHWINFO_FLAGS = FLASHWINFO_FLAGS(4u32);
pub const FLASHW_TIMERNOFG: FLASHWINFO_FLAGS = FLASHWINFO_FLAGS(12u32);
pub const FLASHW_TRAY: FLASHWINFO_FLAGS = FLASHWINFO_FLAGS(2u32);
pub const FNOINVERT: ACCEL_VIRT_FLAGS = ACCEL_VIRT_FLAGS(2u8);
pub const FSHIFT: ACCEL_VIRT_FLAGS = ACCEL_VIRT_FLAGS(4u8);
pub const FVIRTKEY: ACCEL_VIRT_FLAGS = ACCEL_VIRT_FLAGS(1u8);
pub const GA_PARENT: GET_ANCESTOR_FLAGS = GET_ANCESTOR_FLAGS(1u32);
pub const GA_ROOT: GET_ANCESTOR_FLAGS = GET_ANCESTOR_FLAGS(2u32);
pub const GA_ROOTOWNER: GET_ANCESTOR_FLAGS = GET_ANCESTOR_FLAGS(3u32);
pub const GCF_INCLUDE_ANCESTORS: u32 = 1u32;
pub const GCLP_HBRBACKGROUND: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-10i32);
pub const GCLP_HCURSOR: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-12i32);
pub const GCLP_HICON: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-14i32);
pub const GCLP_HICONSM: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-34i32);
pub const GCLP_HMODULE: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-16i32);
pub const GCLP_MENUNAME: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-8i32);
pub const GCLP_WNDPROC: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-24i32);
pub const GCL_CBCLSEXTRA: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-20i32);
pub const GCL_CBWNDEXTRA: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-18i32);
pub const GCL_HBRBACKGROUND: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-10i32);
pub const GCL_HCURSOR: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-12i32);
pub const GCL_HICON: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-14i32);
pub const GCL_HICONSM: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-34i32);
pub const GCL_HMODULE: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-16i32);
pub const GCL_MENUNAME: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-8i32);
pub const GCL_STYLE: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-26i32);
pub const GCL_WNDPROC: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-24i32);
pub const GCW_ATOM: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-32i32);
pub const GESTURECONFIGMAXCOUNT: u32 = 256u32;
pub const GESTUREVISUALIZATION_DOUBLETAP: u32 = 2u32;
pub const GESTUREVISUALIZATION_OFF: u32 = 0u32;
pub const GESTUREVISUALIZATION_ON: u32 = 31u32;
pub const GESTUREVISUALIZATION_PRESSANDHOLD: u32 = 8u32;
pub const GESTUREVISUALIZATION_PRESSANDTAP: u32 = 4u32;
pub const GESTUREVISUALIZATION_RIGHTTAP: u32 = 16u32;
pub const GESTUREVISUALIZATION_TAP: u32 = 1u32;
pub const GF_BEGIN: u32 = 1u32;
pub const GF_END: u32 = 4u32;
pub const GF_INERTIA: u32 = 2u32;
pub const GIDC_ARRIVAL: u32 = 1u32;
pub const GIDC_REMOVAL: u32 = 2u32;
pub const GMDI_GOINTOPOPUPS: GET_MENU_DEFAULT_ITEM_FLAGS = GET_MENU_DEFAULT_ITEM_FLAGS(2u32);
pub const GMDI_USEDISABLED: GET_MENU_DEFAULT_ITEM_FLAGS = GET_MENU_DEFAULT_ITEM_FLAGS(1u32);
pub const GUID_DEVICE_EVENT_RBC: windows_core::GUID = windows_core::GUID::from_u128(0xd0744792_a98e_11d2_917a_00a0c9068ff3);
pub const GUID_IO_CDROM_EXCLUSIVE_LOCK: windows_core::GUID = windows_core::GUID::from_u128(0xbc56c139_7a10_47ee_a294_4c6a38f0149a);
pub const GUID_IO_CDROM_EXCLUSIVE_UNLOCK: windows_core::GUID = windows_core::GUID::from_u128(0xa3b6d27d_5e35_4885_81e5_ee18c00ed779);
pub const GUID_IO_DEVICE_BECOMING_READY: windows_core::GUID = windows_core::GUID::from_u128(0xd07433f0_a98e_11d2_917a_00a0c9068ff3);
pub const GUID_IO_DEVICE_EXTERNAL_REQUEST: windows_core::GUID = windows_core::GUID::from_u128(0xd07433d0_a98e_11d2_917a_00a0c9068ff3);
pub const GUID_IO_DISK_CLONE_ARRIVAL: windows_core::GUID = windows_core::GUID::from_u128(0x6a61885b_7c39_43dd_9b56_b8ac22a549aa);
pub const GUID_IO_DISK_HEALTH_NOTIFICATION: windows_core::GUID = windows_core::GUID::from_u128(0x0f1bd644_3916_49c5_b063_991940118fb2);
pub const GUID_IO_DISK_LAYOUT_CHANGE: windows_core::GUID = windows_core::GUID::from_u128(0x11dff54c_8469_41f9_b3de_ef836487c54a);
pub const GUID_IO_DRIVE_REQUIRES_CLEANING: windows_core::GUID = windows_core::GUID::from_u128(0x7207877c_90ed_44e5_a000_81428d4c79bb);
pub const GUID_IO_MEDIA_ARRIVAL: windows_core::GUID = windows_core::GUID::from_u128(0xd07433c0_a98e_11d2_917a_00a0c9068ff3);
pub const GUID_IO_MEDIA_EJECT_REQUEST: windows_core::GUID = windows_core::GUID::from_u128(0xd07433d1_a98e_11d2_917a_00a0c9068ff3);
pub const GUID_IO_MEDIA_REMOVAL: windows_core::GUID = windows_core::GUID::from_u128(0xd07433c1_a98e_11d2_917a_00a0c9068ff3);
pub const GUID_IO_TAPE_ERASE: windows_core::GUID = windows_core::GUID::from_u128(0x852d11eb_4bb8_4507_9d9b_417cc2b1b438);
pub const GUID_IO_VOLUME_BACKGROUND_FORMAT: windows_core::GUID = windows_core::GUID::from_u128(0xa2e5fc86_d5cd_4038_b2e3_4445065c2377);
pub const GUID_IO_VOLUME_CHANGE: windows_core::GUID = windows_core::GUID::from_u128(0x7373654a_812a_11d0_bec7_08002be2092f);
pub const GUID_IO_VOLUME_CHANGE_SIZE: windows_core::GUID = windows_core::GUID::from_u128(0x3a1625be_ad03_49f1_8ef8_6bbac182d1fd);
pub const GUID_IO_VOLUME_DEVICE_INTERFACE: windows_core::GUID = windows_core::GUID::from_u128(0x53f5630d_b6bf_11d0_94f2_00a0c91efb8b);
pub const GUID_IO_VOLUME_DISMOUNT: windows_core::GUID = windows_core::GUID::from_u128(0xd16a55e8_1059_11d2_8ffd_00a0c9a06d32);
pub const GUID_IO_VOLUME_DISMOUNT_FAILED: windows_core::GUID = windows_core::GUID::from_u128(0xe3c5b178_105d_11d2_8ffd_00a0c9a06d32);
pub const GUID_IO_VOLUME_FORCE_CLOSED: windows_core::GUID = windows_core::GUID::from_u128(0x411ad84f_433e_4dc2_a5ae_4a2d1a2de654);
pub const GUID_IO_VOLUME_FVE_STATUS_CHANGE: windows_core::GUID = windows_core::GUID::from_u128(0x062998b2_ee1f_4b6a_b857_e76cbbe9a6da);
pub const GUID_IO_VOLUME_INFO_MAKE_COMPAT: windows_core::GUID = windows_core::GUID::from_u128(0x3ab9a0d2_ef80_45cf_8cdc_cbe02a212906);
pub const GUID_IO_VOLUME_LOCK: windows_core::GUID = windows_core::GUID::from_u128(0x50708874_c9af_11d1_8fef_00a0c9a06d32);
pub const GUID_IO_VOLUME_LOCK_FAILED: windows_core::GUID = windows_core::GUID::from_u128(0xae2eed10_0ba8_11d2_8ffb_00a0c9a06d32);
pub const GUID_IO_VOLUME_MOUNT: windows_core::GUID = windows_core::GUID::from_u128(0xb5804878_1a96_11d2_8ffd_00a0c9a06d32);
pub const GUID_IO_VOLUME_NAME_CHANGE: windows_core::GUID = windows_core::GUID::from_u128(0x2de97f83_4c06_11d2_a532_00609713055a);
pub const GUID_IO_VOLUME_NEED_CHKDSK: windows_core::GUID = windows_core::GUID::from_u128(0x799a0960_0a0b_4e03_ad88_2fa7c6ce748a);
pub const GUID_IO_VOLUME_PHYSICAL_CONFIGURATION_CHANGE: windows_core::GUID = windows_core::GUID::from_u128(0x2de97f84_4c06_11d2_a532_00609713055a);
pub const GUID_IO_VOLUME_PREPARING_EJECT: windows_core::GUID = windows_core::GUID::from_u128(0xc79eb16e_0dac_4e7a_a86c_b25ceeaa88f6);
pub const GUID_IO_VOLUME_UNIQUE_ID_CHANGE: windows_core::GUID = windows_core::GUID::from_u128(0xaf39da42_6622_41f5_970b_139d092fa3d9);
pub const GUID_IO_VOLUME_UNLOCK: windows_core::GUID = windows_core::GUID::from_u128(0x9a8c3d68_d0cb_11d1_8fef_00a0c9a06d32);
pub const GUID_IO_VOLUME_WEARING_OUT: windows_core::GUID = windows_core::GUID::from_u128(0x873113ca_1486_4508_82ac_c3b2e5297aaa);
pub const GUID_IO_VOLUME_WORM_NEAR_FULL: windows_core::GUID = windows_core::GUID::from_u128(0xf3bfff82_f3de_48d2_af95_457f80b763f2);
pub const GUI_16BITTASK: u32 = 0u32;
pub const GUI_CARETBLINKING: GUITHREADINFO_FLAGS = GUITHREADINFO_FLAGS(1u32);
pub const GUI_INMENUMODE: GUITHREADINFO_FLAGS = GUITHREADINFO_FLAGS(4u32);
pub const GUI_INMOVESIZE: GUITHREADINFO_FLAGS = GUITHREADINFO_FLAGS(2u32);
pub const GUI_POPUPMENUMODE: GUITHREADINFO_FLAGS = GUITHREADINFO_FLAGS(16u32);
pub const GUI_SYSTEMMENUMODE: GUITHREADINFO_FLAGS = GUITHREADINFO_FLAGS(8u32);
pub const GWFS_INCLUDE_ANCESTORS: u32 = 1u32;
pub const GWLP_HINSTANCE: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-6i32);
pub const GWLP_HWNDPARENT: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-8i32);
pub const GWLP_ID: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-12i32);
pub const GWLP_USERDATA: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-21i32);
pub const GWLP_WNDPROC: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-4i32);
pub const GWL_EXSTYLE: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-20i32);
pub const GWL_HINSTANCE: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-6i32);
pub const GWL_HWNDPARENT: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-8i32);
pub const GWL_ID: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-12i32);
pub const GWL_STYLE: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-16i32);
pub const GWL_USERDATA: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-21i32);
pub const GWL_WNDPROC: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-4i32);
pub const GW_CHILD: GET_WINDOW_CMD = GET_WINDOW_CMD(5u32);
pub const GW_ENABLEDPOPUP: GET_WINDOW_CMD = GET_WINDOW_CMD(6u32);
pub const GW_HWNDFIRST: GET_WINDOW_CMD = GET_WINDOW_CMD(0u32);
pub const GW_HWNDLAST: GET_WINDOW_CMD = GET_WINDOW_CMD(1u32);
pub const GW_HWNDNEXT: GET_WINDOW_CMD = GET_WINDOW_CMD(2u32);
pub const GW_HWNDPREV: GET_WINDOW_CMD = GET_WINDOW_CMD(3u32);
pub const GW_MAX: u32 = 5u32;
pub const GW_OWNER: GET_WINDOW_CMD = GET_WINDOW_CMD(4u32);
pub const HANDEDNESS_LEFT: HANDEDNESS = HANDEDNESS(0i32);
pub const HANDEDNESS_RIGHT: HANDEDNESS = HANDEDNESS(1i32);
#[cfg(feature = "Win32_Graphics_Gdi")]
pub const HBMMENU_CALLBACK: super::super::Graphics::Gdi::HBITMAP = super::super::Graphics::Gdi::HBITMAP(-1i32 as _);
#[cfg(feature = "Win32_Graphics_Gdi")]
pub const HBMMENU_MBAR_CLOSE: super::super::Graphics::Gdi::HBITMAP = super::super::Graphics::Gdi::HBITMAP(5i32 as _);
#[cfg(feature = "Win32_Graphics_Gdi")]
pub const HBMMENU_MBAR_CLOSE_D: super::super::Graphics::Gdi::HBITMAP = super::super::Graphics::Gdi::HBITMAP(6i32 as _);
#[cfg(feature = "Win32_Graphics_Gdi")]
pub const HBMMENU_MBAR_MINIMIZE: super::super::Graphics::Gdi::HBITMAP = super::super::Graphics::Gdi::HBITMAP(3i32 as _);
#[cfg(feature = "Win32_Graphics_Gdi")]
pub const HBMMENU_MBAR_MINIMIZE_D: super::super::Graphics::Gdi::HBITMAP = super::super::Graphics::Gdi::HBITMAP(7i32 as _);
#[cfg(feature = "Win32_Graphics_Gdi")]
pub const HBMMENU_MBAR_RESTORE: super::super::Graphics::Gdi::HBITMAP = super::super::Graphics::Gdi::HBITMAP(2i32 as _);
#[cfg(feature = "Win32_Graphics_Gdi")]
pub const HBMMENU_POPUP_CLOSE: super::super::Graphics::Gdi::HBITMAP = super::super::Graphics::Gdi::HBITMAP(8i32 as _);
#[cfg(feature = "Win32_Graphics_Gdi")]
pub const HBMMENU_POPUP_MAXIMIZE: super::super::Graphics::Gdi::HBITMAP = super::super::Graphics::Gdi::HBITMAP(10i32 as _);
#[cfg(feature = "Win32_Graphics_Gdi")]
pub const HBMMENU_POPUP_MINIMIZE: super::super::Graphics::Gdi::HBITMAP = super::super::Graphics::Gdi::HBITMAP(11i32 as _);
#[cfg(feature = "Win32_Graphics_Gdi")]
pub const HBMMENU_POPUP_RESTORE: super::super::Graphics::Gdi::HBITMAP = super::super::Graphics::Gdi::HBITMAP(9i32 as _);
#[cfg(feature = "Win32_Graphics_Gdi")]
pub const HBMMENU_SYSTEM: super::super::Graphics::Gdi::HBITMAP = super::super::Graphics::Gdi::HBITMAP(1i32 as _);
pub const HCBT_ACTIVATE: u32 = 5u32;
pub const HCBT_CLICKSKIPPED: u32 = 6u32;
pub const HCBT_CREATEWND: u32 = 3u32;
pub const HCBT_DESTROYWND: u32 = 4u32;
pub const HCBT_KEYSKIPPED: u32 = 7u32;
pub const HCBT_MINMAX: u32 = 1u32;
pub const HCBT_MOVESIZE: u32 = 0u32;
pub const HCBT_QS: u32 = 2u32;
pub const HCBT_SETFOCUS: u32 = 9u32;
pub const HCBT_SYSCOMMAND: u32 = 8u32;
pub const HCF_DEFAULTDESKTOP: u32 = 512u32;
pub const HCF_LOGONDESKTOP: u32 = 256u32;
pub const HC_ACTION: u32 = 0u32;
pub const HC_GETNEXT: u32 = 1u32;
pub const HC_NOREM: u32 = 3u32;
pub const HC_NOREMOVE: u32 = 3u32;
pub const HC_SKIP: u32 = 2u32;
pub const HC_SYSMODALOFF: u32 = 5u32;
pub const HC_SYSMODALON: u32 = 4u32;
pub const HELP_COMMAND: i32 = 258i32;
pub const HELP_CONTENTS: i32 = 3i32;
pub const HELP_CONTEXT: i32 = 1i32;
pub const HELP_CONTEXTMENU: u32 = 10u32;
pub const HELP_CONTEXTPOPUP: i32 = 8i32;
pub const HELP_FINDER: u32 = 11u32;
pub const HELP_FORCEFILE: i32 = 9i32;
pub const HELP_HELPONHELP: i32 = 4i32;
pub const HELP_INDEX: i32 = 3i32;
pub const HELP_KEY: i32 = 257i32;
pub const HELP_MULTIKEY: i32 = 513i32;
pub const HELP_PARTIALKEY: i32 = 261i32;
pub const HELP_QUIT: i32 = 2i32;
pub const HELP_SETCONTENTS: i32 = 5i32;
pub const HELP_SETINDEX: i32 = 5i32;
pub const HELP_SETPOPUP_POS: u32 = 13u32;
pub const HELP_SETWINPOS: i32 = 515i32;
pub const HELP_TCARD: u32 = 32768u32;
pub const HELP_TCARD_DATA: u32 = 16u32;
pub const HELP_TCARD_OTHER_CALLER: u32 = 17u32;
pub const HELP_WM_HELP: u32 = 12u32;
pub const HIDE_WINDOW: u32 = 0u32;
pub const HKL_NEXT: u32 = 1u32;
pub const HKL_PREV: u32 = 0u32;
pub const HSHELL_ACCESSIBILITYSTATE: u32 = 11u32;
pub const HSHELL_ACTIVATESHELLWINDOW: u32 = 3u32;
pub const HSHELL_APPCOMMAND: u32 = 12u32;
pub const HSHELL_ENDTASK: u32 = 10u32;
pub const HSHELL_GETMINRECT: u32 = 5u32;
pub const HSHELL_HIGHBIT: u32 = 32768u32;
pub const HSHELL_LANGUAGE: u32 = 8u32;
pub const HSHELL_MONITORCHANGED: u32 = 16u32;
pub const HSHELL_REDRAW: u32 = 6u32;
pub const HSHELL_SYSMENU: u32 = 9u32;
pub const HSHELL_TASKMAN: u32 = 7u32;
pub const HSHELL_WINDOWACTIVATED: u32 = 4u32;
pub const HSHELL_WINDOWCREATED: u32 = 1u32;
pub const HSHELL_WINDOWDESTROYED: u32 = 2u32;
pub const HSHELL_WINDOWREPLACED: u32 = 13u32;
pub const HSHELL_WINDOWREPLACING: u32 = 14u32;
pub const HTBORDER: u32 = 18u32;
pub const HTBOTTOM: u32 = 15u32;
pub const HTBOTTOMLEFT: u32 = 16u32;
pub const HTBOTTOMRIGHT: u32 = 17u32;
pub const HTCAPTION: u32 = 2u32;
pub const HTCLIENT: u32 = 1u32;
pub const HTCLOSE: u32 = 20u32;
pub const HTERROR: i32 = -2i32;
pub const HTGROWBOX: u32 = 4u32;
pub const HTHELP: u32 = 21u32;
pub const HTHSCROLL: u32 = 6u32;
pub const HTLEFT: u32 = 10u32;
pub const HTMAXBUTTON: u32 = 9u32;
pub const HTMENU: u32 = 5u32;
pub const HTMINBUTTON: u32 = 8u32;
pub const HTNOWHERE: u32 = 0u32;
pub const HTOBJECT: u32 = 19u32;
pub const HTREDUCE: u32 = 8u32;
pub const HTRIGHT: u32 = 11u32;
pub const HTSIZE: u32 = 4u32;
pub const HTSIZEFIRST: u32 = 10u32;
pub const HTSIZELAST: u32 = 17u32;
pub const HTSYSMENU: u32 = 3u32;
pub const HTTOP: u32 = 12u32;
pub const HTTOPLEFT: u32 = 13u32;
pub const HTTOPRIGHT: u32 = 14u32;
pub const HTTRANSPARENT: i32 = -1i32;
pub const HTVSCROLL: u32 = 7u32;
pub const HTZOOM: u32 = 9u32;
pub const HWND_BOTTOM: super::super::Foundation::HWND = super::super::Foundation::HWND(1i32 as _);
pub const HWND_BROADCAST: super::super::Foundation::HWND = super::super::Foundation::HWND(65535i32 as _);
pub const HWND_DESKTOP: super::super::Foundation::HWND = super::super::Foundation::HWND(0i32 as _);
pub const HWND_MESSAGE: super::super::Foundation::HWND = super::super::Foundation::HWND(-3i32 as _);
pub const HWND_NOTOPMOST: super::super::Foundation::HWND = super::super::Foundation::HWND(-2i32 as _);
pub const HWND_TOP: super::super::Foundation::HWND = super::super::Foundation::HWND(0i32 as _);
pub const HWND_TOPMOST: super::super::Foundation::HWND = super::super::Foundation::HWND(-1i32 as _);
pub const ICON_BIG: u32 = 1u32;
pub const ICON_SMALL: u32 = 0u32;
pub const ICON_SMALL2: u32 = 2u32;
pub const IDABORT: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(3i32);
pub const IDANI_CAPTION: u32 = 3u32;
pub const IDANI_OPEN: u32 = 1u32;
pub const IDASYNC: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(32001i32);
pub const IDCANCEL: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(2i32);
pub const IDCLOSE: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(8i32);
pub const IDCONTINUE: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(11i32);
pub const IDC_APPSTARTING: windows_core::PCWSTR = windows_core::PCWSTR(32650u16 as _);
pub const IDC_ARROW: windows_core::PCWSTR = windows_core::PCWSTR(32512u16 as _);
pub const IDC_CROSS: windows_core::PCWSTR = windows_core::PCWSTR(32515u16 as _);
pub const IDC_HAND: windows_core::PCWSTR = windows_core::PCWSTR(32649u16 as _);
pub const IDC_HELP: windows_core::PCWSTR = windows_core::PCWSTR(32651u16 as _);
pub const IDC_IBEAM: windows_core::PCWSTR = windows_core::PCWSTR(32513u16 as _);
pub const IDC_ICON: windows_core::PCWSTR = windows_core::PCWSTR(32641u16 as _);
pub const IDC_NO: windows_core::PCWSTR = windows_core::PCWSTR(32648u16 as _);
pub const IDC_PERSON: windows_core::PCWSTR = windows_core::PCWSTR(32672u16 as _);
pub const IDC_PIN: windows_core::PCWSTR = windows_core::PCWSTR(32671u16 as _);
pub const IDC_SIZE: windows_core::PCWSTR = windows_core::PCWSTR(32640u16 as _);
pub const IDC_SIZEALL: windows_core::PCWSTR = windows_core::PCWSTR(32646u16 as _);
pub const IDC_SIZENESW: windows_core::PCWSTR = windows_core::PCWSTR(32643u16 as _);
pub const IDC_SIZENS: windows_core::PCWSTR = windows_core::PCWSTR(32645u16 as _);
pub const IDC_SIZENWSE: windows_core::PCWSTR = windows_core::PCWSTR(32642u16 as _);
pub const IDC_SIZEWE: windows_core::PCWSTR = windows_core::PCWSTR(32644u16 as _);
pub const IDC_UPARROW: windows_core::PCWSTR = windows_core::PCWSTR(32516u16 as _);
pub const IDC_WAIT: windows_core::PCWSTR = windows_core::PCWSTR(32514u16 as _);
pub const IDHELP: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(9i32);
pub const IDHOT_SNAPDESKTOP: i32 = -2i32;
pub const IDHOT_SNAPWINDOW: i32 = -1i32;
pub const IDH_CANCEL: u32 = 28444u32;
pub const IDH_GENERIC_HELP_BUTTON: u32 = 28442u32;
pub const IDH_HELP: u32 = 28445u32;
pub const IDH_MISSING_CONTEXT: u32 = 28441u32;
pub const IDH_NO_HELP: u32 = 28440u32;
pub const IDH_OK: u32 = 28443u32;
pub const IDIGNORE: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(5i32);
pub const IDI_APPLICATION: windows_core::PCWSTR = windows_core::PCWSTR(32512u32 as _);
pub const IDI_ASTERISK: windows_core::PCWSTR = windows_core::PCWSTR(32516u32 as _);
pub const IDI_ERROR: windows_core::PCWSTR = windows_core::PCWSTR(32513u32 as _);
pub const IDI_EXCLAMATION: windows_core::PCWSTR = windows_core::PCWSTR(32515u32 as _);
pub const IDI_HAND: windows_core::PCWSTR = windows_core::PCWSTR(32513u32 as _);
pub const IDI_INFORMATION: windows_core::PCWSTR = windows_core::PCWSTR(32516u32 as _);
pub const IDI_QUESTION: windows_core::PCWSTR = windows_core::PCWSTR(32514u32 as _);
pub const IDI_SHIELD: windows_core::PCWSTR = windows_core::PCWSTR(32518u32 as _);
pub const IDI_WARNING: windows_core::PCWSTR = windows_core::PCWSTR(32515u32 as _);
pub const IDI_WINLOGO: windows_core::PCWSTR = windows_core::PCWSTR(32517u32 as _);
pub const IDNO: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(7i32);
pub const IDOK: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(1i32);
pub const IDRETRY: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(4i32);
pub const IDTIMEOUT: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(32000i32);
pub const IDTRYAGAIN: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(10i32);
pub const IDYES: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(6i32);
pub const IMAGE_BITMAP: GDI_IMAGE_TYPE = GDI_IMAGE_TYPE(0u32);
pub const IMAGE_CURSOR: GDI_IMAGE_TYPE = GDI_IMAGE_TYPE(2u32);
pub const IMAGE_ENHMETAFILE: u32 = 3u32;
pub const IMAGE_ICON: GDI_IMAGE_TYPE = GDI_IMAGE_TYPE(1u32);
pub const INDEXID_CONTAINER: u32 = 0u32;
pub const INDEXID_OBJECT: u32 = 0u32;
pub const INPUTLANGCHANGE_BACKWARD: u32 = 4u32;
pub const INPUTLANGCHANGE_FORWARD: u32 = 2u32;
pub const INPUTLANGCHANGE_SYSCHARSET: u32 = 1u32;
pub const ISMEX_CALLBACK: u32 = 4u32;
pub const ISMEX_NOSEND: u32 = 0u32;
pub const ISMEX_NOTIFY: u32 = 2u32;
pub const ISMEX_REPLIED: u32 = 8u32;
pub const ISMEX_SEND: u32 = 1u32;
pub const ISOLATIONAWARE_MANIFEST_RESOURCE_ID: u32 = 2u32;
pub const ISOLATIONAWARE_NOSTATICIMPORT_MANIFEST_RESOURCE_ID: u32 = 3u32;
pub const ISOLATIONPOLICY_BROWSER_MANIFEST_RESOURCE_ID: u32 = 5u32;
pub const ISOLATIONPOLICY_MANIFEST_RESOURCE_ID: u32 = 4u32;
pub const KF_ALTDOWN: u32 = 8192u32;
pub const KF_DLGMODE: u32 = 2048u32;
pub const KF_EXTENDED: u32 = 256u32;
pub const KF_MENUMODE: u32 = 4096u32;
pub const KF_REPEAT: u32 = 16384u32;
pub const KF_UP: u32 = 32768u32;
pub const KL_NAMELENGTH: u32 = 9u32;
pub const LBN_DBLCLK: u32 = 2u32;
pub const LBN_ERRSPACE: i32 = -2i32;
pub const LBN_KILLFOCUS: u32 = 5u32;
pub const LBN_SELCANCEL: u32 = 3u32;
pub const LBN_SELCHANGE: u32 = 1u32;
pub const LBN_SETFOCUS: u32 = 4u32;
pub const LBS_COMBOBOX: i32 = 32768i32;
pub const LBS_DISABLENOSCROLL: i32 = 4096i32;
pub const LBS_EXTENDEDSEL: i32 = 2048i32;
pub const LBS_HASSTRINGS: i32 = 64i32;
pub const LBS_MULTICOLUMN: i32 = 512i32;
pub const LBS_MULTIPLESEL: i32 = 8i32;
pub const LBS_NODATA: i32 = 8192i32;
pub const LBS_NOINTEGRALHEIGHT: i32 = 256i32;
pub const LBS_NOREDRAW: i32 = 4i32;
pub const LBS_NOSEL: i32 = 16384i32;
pub const LBS_NOTIFY: i32 = 1i32;
pub const LBS_OWNERDRAWFIXED: i32 = 16i32;
pub const LBS_OWNERDRAWVARIABLE: i32 = 32i32;
pub const LBS_SORT: i32 = 2i32;
pub const LBS_STANDARD: i32 = 10485763i32;
pub const LBS_USETABSTOPS: i32 = 128i32;
pub const LBS_WANTKEYBOARDINPUT: i32 = 1024i32;
pub const LB_ADDFILE: u32 = 406u32;
pub const LB_ADDSTRING: u32 = 384u32;
pub const LB_CTLCODE: i32 = 0i32;
pub const LB_DELETESTRING: u32 = 386u32;
pub const LB_DIR: u32 = 397u32;
pub const LB_ERR: i32 = -1i32;
pub const LB_ERRSPACE: i32 = -2i32;
pub const LB_FINDSTRING: u32 = 399u32;
pub const LB_FINDSTRINGEXACT: u32 = 418u32;
pub const LB_GETANCHORINDEX: u32 = 413u32;
pub const LB_GETCARETINDEX: u32 = 415u32;
pub const LB_GETCOUNT: u32 = 395u32;
pub const LB_GETCURSEL: u32 = 392u32;
pub const LB_GETHORIZONTALEXTENT: u32 = 403u32;
pub const LB_GETITEMDATA: u32 = 409u32;
pub const LB_GETITEMHEIGHT: u32 = 417u32;
pub const LB_GETITEMRECT: u32 = 408u32;
pub const LB_GETLISTBOXINFO: u32 = 434u32;
pub const LB_GETLOCALE: u32 = 422u32;
pub const LB_GETSEL: u32 = 391u32;
pub const LB_GETSELCOUNT: u32 = 400u32;
pub const LB_GETSELITEMS: u32 = 401u32;
pub const LB_GETTEXT: u32 = 393u32;
pub const LB_GETTEXTLEN: u32 = 394u32;
pub const LB_GETTOPINDEX: u32 = 398u32;
pub const LB_INITSTORAGE: u32 = 424u32;
pub const LB_INSERTSTRING: u32 = 385u32;
pub const LB_ITEMFROMPOINT: u32 = 425u32;
pub const LB_MSGMAX: u32 = 435u32;
pub const LB_MULTIPLEADDSTRING: u32 = 433u32;
pub const LB_OKAY: u32 = 0u32;
pub const LB_RESETCONTENT: u32 = 388u32;
pub const LB_SELECTSTRING: u32 = 396u32;
pub const LB_SELITEMRANGE: u32 = 411u32;
pub const LB_SELITEMRANGEEX: u32 = 387u32;
pub const LB_SETANCHORINDEX: u32 = 412u32;
pub const LB_SETCARETINDEX: u32 = 414u32;
pub const LB_SETCOLUMNWIDTH: u32 = 405u32;
pub const LB_SETCOUNT: u32 = 423u32;
pub const LB_SETCURSEL: u32 = 390u32;
pub const LB_SETHORIZONTALEXTENT: u32 = 404u32;
pub const LB_SETITEMDATA: u32 = 410u32;
pub const LB_SETITEMHEIGHT: u32 = 416u32;
pub const LB_SETLOCALE: u32 = 421u32;
pub const LB_SETSEL: u32 = 389u32;
pub const LB_SETTABSTOPS: u32 = 402u32;
pub const LB_SETTOPINDEX: u32 = 407u32;
pub const LLKHF_ALTDOWN: KBDLLHOOKSTRUCT_FLAGS = KBDLLHOOKSTRUCT_FLAGS(32u32);
pub const LLKHF_EXTENDED: KBDLLHOOKSTRUCT_FLAGS = KBDLLHOOKSTRUCT_FLAGS(1u32);
pub const LLKHF_INJECTED: KBDLLHOOKSTRUCT_FLAGS = KBDLLHOOKSTRUCT_FLAGS(16u32);
pub const LLKHF_LOWER_IL_INJECTED: KBDLLHOOKSTRUCT_FLAGS = KBDLLHOOKSTRUCT_FLAGS(2u32);
pub const LLKHF_UP: KBDLLHOOKSTRUCT_FLAGS = KBDLLHOOKSTRUCT_FLAGS(128u32);
pub const LLMHF_INJECTED: u32 = 1u32;
pub const LLMHF_LOWER_IL_INJECTED: u32 = 2u32;
pub const LOCKF_LOGICAL_LOCK: u32 = 0u32;
pub const LOCKF_PHYSICAL_LOCK: u32 = 1u32;
pub const LOCKP_ALLOW_MEM_MAPPING: u32 = 0u32;
pub const LOCKP_ALLOW_WRITES: u32 = 1u32;
pub const LOCKP_FAIL_MEM_MAPPING: u32 = 2u32;
pub const LOCKP_FAIL_WRITES: u32 = 0u32;
pub const LOCKP_LOCK_FOR_FORMAT: u32 = 4u32;
pub const LOCKP_USER_MASK: u32 = 3u32;
pub const LR_COLOR: u32 = 2u32;
pub const LR_COPYDELETEORG: IMAGE_FLAGS = IMAGE_FLAGS(8u32);
pub const LR_COPYFROMRESOURCE: IMAGE_FLAGS = IMAGE_FLAGS(16384u32);
pub const LR_COPYRETURNORG: IMAGE_FLAGS = IMAGE_FLAGS(4u32);
pub const LR_CREATEDIBSECTION: IMAGE_FLAGS = IMAGE_FLAGS(8192u32);
pub const LR_DEFAULTCOLOR: IMAGE_FLAGS = IMAGE_FLAGS(0u32);
pub const LR_DEFAULTSIZE: IMAGE_FLAGS = IMAGE_FLAGS(64u32);
pub const LR_LOADFROMFILE: IMAGE_FLAGS = IMAGE_FLAGS(16u32);
pub const LR_LOADMAP3DCOLORS: IMAGE_FLAGS = IMAGE_FLAGS(4096u32);
pub const LR_LOADTRANSPARENT: IMAGE_FLAGS = IMAGE_FLAGS(32u32);
pub const LR_MONOCHROME: IMAGE_FLAGS = IMAGE_FLAGS(1u32);
pub const LR_SHARED: IMAGE_FLAGS = IMAGE_FLAGS(32768u32);
pub const LR_VGACOLOR: IMAGE_FLAGS = IMAGE_FLAGS(128u32);
pub const LSFW_LOCK: FOREGROUND_WINDOW_LOCK_CODE = FOREGROUND_WINDOW_LOCK_CODE(1u32);
pub const LSFW_UNLOCK: FOREGROUND_WINDOW_LOCK_CODE = FOREGROUND_WINDOW_LOCK_CODE(2u32);
pub const LWA_ALPHA: LAYERED_WINDOW_ATTRIBUTES_FLAGS = LAYERED_WINDOW_ATTRIBUTES_FLAGS(2u32);
pub const LWA_COLORKEY: LAYERED_WINDOW_ATTRIBUTES_FLAGS = LAYERED_WINDOW_ATTRIBUTES_FLAGS(1u32);
pub const MAXIMUM_RESERVED_MANIFEST_RESOURCE_ID: u32 = 16u32;
pub const MAX_LOGICALDPIOVERRIDE: u32 = 2u32;
pub const MAX_STR_BLOCKREASON: u32 = 256u32;
pub const MAX_TOUCH_COUNT: u32 = 256u32;
pub const MAX_TOUCH_PREDICTION_FILTER_TAPS: u32 = 3u32;
pub const MA_ACTIVATE: u32 = 1u32;
pub const MA_ACTIVATEANDEAT: u32 = 2u32;
pub const MA_NOACTIVATE: u32 = 3u32;
pub const MA_NOACTIVATEANDEAT: u32 = 4u32;
pub const MB_ABORTRETRYIGNORE: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(2u32);
pub const MB_APPLMODAL: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(0u32);
pub const MB_CANCELTRYCONTINUE: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(6u32);
pub const MB_DEFAULT_DESKTOP_ONLY: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(131072u32);
pub const MB_DEFBUTTON1: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(0u32);
pub const MB_DEFBUTTON2: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(256u32);
pub const MB_DEFBUTTON3: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(512u32);
pub const MB_DEFBUTTON4: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(768u32);
pub const MB_DEFMASK: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(3840u32);
pub const MB_HELP: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(16384u32);
pub const MB_ICONASTERISK: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(64u32);
pub const MB_ICONERROR: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(16u32);
pub const MB_ICONEXCLAMATION: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(48u32);
pub const MB_ICONHAND: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(16u32);
pub const MB_ICONINFORMATION: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(64u32);
pub const MB_ICONMASK: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(240u32);
pub const MB_ICONQUESTION: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(32u32);
pub const MB_ICONSTOP: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(16u32);
pub const MB_ICONWARNING: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(48u32);
pub const MB_MISCMASK: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(49152u32);
pub const MB_MODEMASK: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(12288u32);
pub const MB_NOFOCUS: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(32768u32);
pub const MB_OK: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(0u32);
pub const MB_OKCANCEL: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(1u32);
pub const MB_RETRYCANCEL: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(5u32);
pub const MB_RIGHT: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(524288u32);
pub const MB_RTLREADING: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(1048576u32);
pub const MB_SERVICE_NOTIFICATION: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(2097152u32);
pub const MB_SERVICE_NOTIFICATION_NT3X: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(262144u32);
pub const MB_SETFOREGROUND: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(65536u32);
pub const MB_SYSTEMMODAL: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(4096u32);
pub const MB_TASKMODAL: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(8192u32);
pub const MB_TOPMOST: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(262144u32);
pub const MB_TYPEMASK: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(15u32);
pub const MB_USERICON: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(128u32);
pub const MB_YESNO: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(4u32);
pub const MB_YESNOCANCEL: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(3u32);
pub const MDIS_ALLCHILDSTYLES: u32 = 1u32;
pub const MDITILE_HORIZONTAL: TILE_WINDOWS_HOW = TILE_WINDOWS_HOW(1u32);
pub const MDITILE_SKIPDISABLED: CASCADE_WINDOWS_HOW = CASCADE_WINDOWS_HOW(2u32);
pub const MDITILE_VERTICAL: TILE_WINDOWS_HOW = TILE_WINDOWS_HOW(0u32);
pub const MDITILE_ZORDER: CASCADE_WINDOWS_HOW = CASCADE_WINDOWS_HOW(4u32);
pub const METRICS_USEDEFAULT: i32 = -1i32;
pub const MFS_CHECKED: MENU_ITEM_STATE = MENU_ITEM_STATE(8u32);
pub const MFS_DEFAULT: MENU_ITEM_STATE = MENU_ITEM_STATE(4096u32);
pub const MFS_DISABLED: MENU_ITEM_STATE = MENU_ITEM_STATE(3u32);
pub const MFS_ENABLED: MENU_ITEM_STATE = MENU_ITEM_STATE(0u32);
pub const MFS_GRAYED: MENU_ITEM_STATE = MENU_ITEM_STATE(3u32);
pub const MFS_HILITE: MENU_ITEM_STATE = MENU_ITEM_STATE(128u32);
pub const MFS_UNCHECKED: MENU_ITEM_STATE = MENU_ITEM_STATE(0u32);
pub const MFS_UNHILITE: MENU_ITEM_STATE = MENU_ITEM_STATE(0u32);
pub const MFT_BITMAP: MENU_ITEM_TYPE = MENU_ITEM_TYPE(4u32);
pub const MFT_MENUBARBREAK: MENU_ITEM_TYPE = MENU_ITEM_TYPE(32u32);
pub const MFT_MENUBREAK: MENU_ITEM_TYPE = MENU_ITEM_TYPE(64u32);
pub const MFT_OWNERDRAW: MENU_ITEM_TYPE = MENU_ITEM_TYPE(256u32);
pub const MFT_RADIOCHECK: MENU_ITEM_TYPE = MENU_ITEM_TYPE(512u32);
pub const MFT_RIGHTJUSTIFY: MENU_ITEM_TYPE = MENU_ITEM_TYPE(16384u32);
pub const MFT_RIGHTORDER: MENU_ITEM_TYPE = MENU_ITEM_TYPE(8192u32);
pub const MFT_SEPARATOR: MENU_ITEM_TYPE = MENU_ITEM_TYPE(2048u32);
pub const MFT_STRING: MENU_ITEM_TYPE = MENU_ITEM_TYPE(0u32);
pub const MF_APPEND: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(256u32);
pub const MF_BITMAP: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(4u32);
pub const MF_BYCOMMAND: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(0u32);
pub const MF_BYPOSITION: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(1024u32);
pub const MF_CHANGE: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(128u32);
pub const MF_CHECKED: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(8u32);
pub const MF_DEFAULT: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(4096u32);
pub const MF_DELETE: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(512u32);
pub const MF_DISABLED: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(2u32);
pub const MF_ENABLED: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(0u32);
pub const MF_END: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(128u32);
pub const MF_GRAYED: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(1u32);
pub const MF_HELP: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(16384u32);
pub const MF_HILITE: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(128u32);
pub const MF_INSERT: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(0u32);
pub const MF_MENUBARBREAK: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(32u32);
pub const MF_MENUBREAK: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(64u32);
pub const MF_MOUSESELECT: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(32768u32);
pub const MF_OWNERDRAW: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(256u32);
pub const MF_POPUP: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(16u32);
pub const MF_REMOVE: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(4096u32);
pub const MF_RIGHTJUSTIFY: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(16384u32);
pub const MF_SEPARATOR: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(2048u32);
pub const MF_STRING: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(0u32);
pub const MF_SYSMENU: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(8192u32);
pub const MF_UNCHECKED: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(0u32);
pub const MF_UNHILITE: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(0u32);
pub const MF_USECHECKBITMAPS: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(512u32);
pub const MIIM_BITMAP: MENU_ITEM_MASK = MENU_ITEM_MASK(128u32);
pub const MIIM_CHECKMARKS: MENU_ITEM_MASK = MENU_ITEM_MASK(8u32);
pub const MIIM_DATA: MENU_ITEM_MASK = MENU_ITEM_MASK(32u32);
pub const MIIM_FTYPE: MENU_ITEM_MASK = MENU_ITEM_MASK(256u32);
pub const MIIM_ID: MENU_ITEM_MASK = MENU_ITEM_MASK(2u32);
pub const MIIM_STATE: MENU_ITEM_MASK = MENU_ITEM_MASK(1u32);
pub const MIIM_STRING: MENU_ITEM_MASK = MENU_ITEM_MASK(64u32);
pub const MIIM_SUBMENU: MENU_ITEM_MASK = MENU_ITEM_MASK(4u32);
pub const MIIM_TYPE: MENU_ITEM_MASK = MENU_ITEM_MASK(16u32);
pub const MIM_APPLYTOSUBMENUS: MENUINFO_MASK = MENUINFO_MASK(2147483648u32);
pub const MIM_BACKGROUND: MENUINFO_MASK = MENUINFO_MASK(2u32);
pub const MIM_HELPID: MENUINFO_MASK = MENUINFO_MASK(4u32);
pub const MIM_MAXHEIGHT: MENUINFO_MASK = MENUINFO_MASK(1u32);
pub const MIM_MENUDATA: MENUINFO_MASK = MENUINFO_MASK(8u32);
pub const MIM_STYLE: MENUINFO_MASK = MENUINFO_MASK(16u32);
pub const MINIMUM_RESERVED_MANIFEST_RESOURCE_ID: u32 = 1u32;
pub const MIN_LOGICALDPIOVERRIDE: i32 = -2i32;
pub const MKF_AVAILABLE: u32 = 2u32;
pub const MKF_CONFIRMHOTKEY: u32 = 8u32;
pub const MKF_HOTKEYACTIVE: u32 = 4u32;
pub const MKF_HOTKEYSOUND: u32 = 16u32;
pub const MKF_INDICATOR: u32 = 32u32;
pub const MKF_LEFTBUTTONDOWN: u32 = 16777216u32;
pub const MKF_LEFTBUTTONSEL: u32 = 268435456u32;
pub const MKF_MODIFIERS: u32 = 64u32;
pub const MKF_MOUSEKEYSON: u32 = 1u32;
pub const MKF_MOUSEMODE: u32 = 2147483648u32;
pub const MKF_REPLACENUMBERS: u32 = 128u32;
pub const MKF_RIGHTBUTTONDOWN: u32 = 33554432u32;
pub const MKF_RIGHTBUTTONSEL: u32 = 536870912u32;
pub const MNC_CLOSE: u32 = 1u32;
pub const MNC_EXECUTE: u32 = 2u32;
pub const MNC_IGNORE: u32 = 0u32;
pub const MNC_SELECT: u32 = 3u32;
pub const MND_CONTINUE: u32 = 0u32;
pub const MND_ENDMENU: u32 = 1u32;
pub const MNGOF_BOTTOMGAP: MENUGETOBJECTINFO_FLAGS = MENUGETOBJECTINFO_FLAGS(2u32);
pub const MNGOF_TOPGAP: MENUGETOBJECTINFO_FLAGS = MENUGETOBJECTINFO_FLAGS(1u32);
pub const MNGO_NOERROR: u32 = 1u32;
pub const MNGO_NOINTERFACE: u32 = 0u32;
pub const MNS_AUTODISMISS: MENUINFO_STYLE = MENUINFO_STYLE(268435456u32);
pub const MNS_CHECKORBMP: MENUINFO_STYLE = MENUINFO_STYLE(67108864u32);
pub const MNS_DRAGDROP: MENUINFO_STYLE = MENUINFO_STYLE(536870912u32);
pub const MNS_MODELESS: MENUINFO_STYLE = MENUINFO_STYLE(1073741824u32);
pub const MNS_NOCHECK: MENUINFO_STYLE = MENUINFO_STYLE(2147483648u32);
pub const MNS_NOTIFYBYPOS: MENUINFO_STYLE = MENUINFO_STYLE(134217728u32);
pub const MN_GETHMENU: u32 = 481u32;
pub const MONITORINFOF_PRIMARY: u32 = 1u32;
pub const MOUSEWHEEL_ROUTING_FOCUS: u32 = 0u32;
pub const MOUSEWHEEL_ROUTING_HYBRID: u32 = 1u32;
pub const MOUSEWHEEL_ROUTING_MOUSE_POS: u32 = 2u32;
pub const MSGFLTINFO_ALLOWED_HIGHER: MSGFLTINFO_STATUS = MSGFLTINFO_STATUS(3u32);
pub const MSGFLTINFO_ALREADYALLOWED_FORWND: MSGFLTINFO_STATUS = MSGFLTINFO_STATUS(1u32);
pub const MSGFLTINFO_ALREADYDISALLOWED_FORWND: MSGFLTINFO_STATUS = MSGFLTINFO_STATUS(2u32);
pub const MSGFLTINFO_NONE: MSGFLTINFO_STATUS = MSGFLTINFO_STATUS(0u32);
pub const MSGFLT_ADD: CHANGE_WINDOW_MESSAGE_FILTER_FLAGS = CHANGE_WINDOW_MESSAGE_FILTER_FLAGS(1u32);
pub const MSGFLT_ALLOW: WINDOW_MESSAGE_FILTER_ACTION = WINDOW_MESSAGE_FILTER_ACTION(1u32);
pub const MSGFLT_DISALLOW: WINDOW_MESSAGE_FILTER_ACTION = WINDOW_MESSAGE_FILTER_ACTION(2u32);
pub const MSGFLT_REMOVE: CHANGE_WINDOW_MESSAGE_FILTER_FLAGS = CHANGE_WINDOW_MESSAGE_FILTER_FLAGS(2u32);
pub const MSGFLT_RESET: WINDOW_MESSAGE_FILTER_ACTION = WINDOW_MESSAGE_FILTER_ACTION(0u32);
pub const MSGF_DIALOGBOX: u32 = 0u32;
pub const MSGF_MAX: u32 = 8u32;
pub const MSGF_MENU: u32 = 2u32;
pub const MSGF_MESSAGEBOX: u32 = 1u32;
pub const MSGF_NEXTWINDOW: u32 = 6u32;
pub const MSGF_SCROLLBAR: u32 = 5u32;
pub const MSGF_USER: u32 = 4096u32;
pub const MWMO_ALERTABLE: MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS = MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS(2u32);
pub const MWMO_INPUTAVAILABLE: MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS = MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS(4u32);
pub const MWMO_NONE: MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS = MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS(0u32);
pub const MWMO_WAITALL: MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS = MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS(1u32);
pub const MrmDumpType_Basic: MrmDumpType = MrmDumpType(0i32);
pub const MrmDumpType_Detailed: MrmDumpType = MrmDumpType(1i32);
pub const MrmDumpType_Schema: MrmDumpType = MrmDumpType(2i32);
pub const MrmIndexerFlagsAutoMerge: MrmIndexerFlags = MrmIndexerFlags(1i32);
pub const MrmIndexerFlagsCreateContentChecksum: MrmIndexerFlags = MrmIndexerFlags(2i32);
pub const MrmIndexerFlagsNone: MrmIndexerFlags = MrmIndexerFlags(0i32);
pub const MrmPackagingModeAutoSplit: MrmPackagingMode = MrmPackagingMode(1i32);
pub const MrmPackagingModeResourcePack: MrmPackagingMode = MrmPackagingMode(2i32);
pub const MrmPackagingModeStandaloneFile: MrmPackagingMode = MrmPackagingMode(0i32);
pub const MrmPackagingOptionsNone: MrmPackagingOptions = MrmPackagingOptions(0i32);
pub const MrmPackagingOptionsOmitSchemaFromResourcePacks: MrmPackagingOptions = MrmPackagingOptions(1i32);
pub const MrmPackagingOptionsSplitLanguageVariants: MrmPackagingOptions = MrmPackagingOptions(2i32);
pub const MrmPlatformVersion_Default: MrmPlatformVersion = MrmPlatformVersion(0i32);
pub const MrmPlatformVersion_Windows10_0_0_0: MrmPlatformVersion = MrmPlatformVersion(17432576i32);
pub const MrmPlatformVersion_Windows10_0_0_5: MrmPlatformVersion = MrmPlatformVersion(17432581i32);
pub const MrmResourceIndexerMessageSeverityError: MrmResourceIndexerMessageSeverity = MrmResourceIndexerMessageSeverity(3i32);
pub const MrmResourceIndexerMessageSeverityInfo: MrmResourceIndexerMessageSeverity = MrmResourceIndexerMessageSeverity(1i32);
pub const MrmResourceIndexerMessageSeverityVerbose: MrmResourceIndexerMessageSeverity = MrmResourceIndexerMessageSeverity(0i32);
pub const MrmResourceIndexerMessageSeverityWarning: MrmResourceIndexerMessageSeverity = MrmResourceIndexerMessageSeverity(2i32);
pub const NFR_ANSI: u32 = 1u32;
pub const NFR_UNICODE: u32 = 2u32;
pub const NF_QUERY: u32 = 3u32;
pub const NF_REQUERY: u32 = 4u32;
pub const NID_EXTERNAL_PEN: u32 = 8u32;
pub const NID_EXTERNAL_TOUCH: u32 = 2u32;
pub const NID_INTEGRATED_PEN: u32 = 4u32;
pub const NID_INTEGRATED_TOUCH: u32 = 1u32;
pub const NID_MULTI_INPUT: u32 = 64u32;
pub const NID_READY: u32 = 128u32;
pub const OBJID_ALERT: OBJECT_IDENTIFIER = OBJECT_IDENTIFIER(-10i32);
pub const OBJID_CARET: OBJECT_IDENTIFIER = OBJECT_IDENTIFIER(-8i32);
pub const OBJID_CLIENT: OBJECT_IDENTIFIER = OBJECT_IDENTIFIER(-4i32);
pub const OBJID_CURSOR: OBJECT_IDENTIFIER = OBJECT_IDENTIFIER(-9i32);
pub const OBJID_HSCROLL: OBJECT_IDENTIFIER = OBJECT_IDENTIFIER(-6i32);
pub const OBJID_MENU: OBJECT_IDENTIFIER = OBJECT_IDENTIFIER(-3i32);
pub const OBJID_NATIVEOM: OBJECT_IDENTIFIER = OBJECT_IDENTIFIER(-16i32);
pub const OBJID_QUERYCLASSNAMEIDX: OBJECT_IDENTIFIER = OBJECT_IDENTIFIER(-12i32);
pub const OBJID_SIZEGRIP: OBJECT_IDENTIFIER = OBJECT_IDENTIFIER(-7i32);
pub const OBJID_SOUND: OBJECT_IDENTIFIER = OBJECT_IDENTIFIER(-11i32);
pub const OBJID_SYSMENU: OBJECT_IDENTIFIER = OBJECT_IDENTIFIER(-1i32);
pub const OBJID_TITLEBAR: OBJECT_IDENTIFIER = OBJECT_IDENTIFIER(-2i32);
pub const OBJID_VSCROLL: OBJECT_IDENTIFIER = OBJECT_IDENTIFIER(-5i32);
pub const OBJID_WINDOW: OBJECT_IDENTIFIER = OBJECT_IDENTIFIER(0i32);
pub const OBM_BTNCORNERS: u32 = 32758u32;
pub const OBM_BTSIZE: u32 = 32761u32;
pub const OBM_CHECK: u32 = 32760u32;
pub const OBM_CHECKBOXES: u32 = 32759u32;
pub const OBM_CLOSE: u32 = 32754u32;
pub const OBM_COMBO: u32 = 32738u32;
pub const OBM_DNARROW: u32 = 32752u32;
pub const OBM_DNARROWD: u32 = 32742u32;
pub const OBM_DNARROWI: u32 = 32736u32;
pub const OBM_LFARROW: u32 = 32750u32;
pub const OBM_LFARROWD: u32 = 32740u32;
pub const OBM_LFARROWI: u32 = 32734u32;
pub const OBM_MNARROW: u32 = 32739u32;
pub const OBM_OLD_CLOSE: u32 = 32767u32;
pub const OBM_OLD_DNARROW: u32 = 32764u32;
pub const OBM_OLD_LFARROW: u32 = 32762u32;
pub const OBM_OLD_REDUCE: u32 = 32757u32;
pub const OBM_OLD_RESTORE: u32 = 32755u32;
pub const OBM_OLD_RGARROW: u32 = 32763u32;
pub const OBM_OLD_UPARROW: u32 = 32765u32;
pub const OBM_OLD_ZOOM: u32 = 32756u32;
pub const OBM_REDUCE: u32 = 32749u32;
pub const OBM_REDUCED: u32 = 32746u32;
pub const OBM_RESTORE: u32 = 32747u32;
pub const OBM_RESTORED: u32 = 32744u32;
pub const OBM_RGARROW: u32 = 32751u32;
pub const OBM_RGARROWD: u32 = 32741u32;
pub const OBM_RGARROWI: u32 = 32735u32;
pub const OBM_SIZE: u32 = 32766u32;
pub const OBM_UPARROW: u32 = 32753u32;
pub const OBM_UPARROWD: u32 = 32743u32;
pub const OBM_UPARROWI: u32 = 32737u32;
pub const OBM_ZOOM: u32 = 32748u32;
pub const OBM_ZOOMD: u32 = 32745u32;
pub const OCR_APPSTARTING: SYSTEM_CURSOR_ID = SYSTEM_CURSOR_ID(32650u32);
pub const OCR_CROSS: SYSTEM_CURSOR_ID = SYSTEM_CURSOR_ID(32515u32);
pub const OCR_HAND: SYSTEM_CURSOR_ID = SYSTEM_CURSOR_ID(32649u32);
pub const OCR_HELP: SYSTEM_CURSOR_ID = SYSTEM_CURSOR_ID(32651u32);
pub const OCR_IBEAM: SYSTEM_CURSOR_ID = SYSTEM_CURSOR_ID(32513u32);
pub const OCR_ICOCUR: u32 = 32647u32;
pub const OCR_ICON: u32 = 32641u32;
pub const OCR_NO: SYSTEM_CURSOR_ID = SYSTEM_CURSOR_ID(32648u32);
pub const OCR_NORMAL: SYSTEM_CURSOR_ID = SYSTEM_CURSOR_ID(32512u32);
pub const OCR_SIZE: u32 = 32640u32;
pub const OCR_SIZEALL: SYSTEM_CURSOR_ID = SYSTEM_CURSOR_ID(32646u32);
pub const OCR_SIZENESW: SYSTEM_CURSOR_ID = SYSTEM_CURSOR_ID(32643u32);
pub const OCR_SIZENS: SYSTEM_CURSOR_ID = SYSTEM_CURSOR_ID(32645u32);
pub const OCR_SIZENWSE: SYSTEM_CURSOR_ID = SYSTEM_CURSOR_ID(32642u32);
pub const OCR_SIZEWE: SYSTEM_CURSOR_ID = SYSTEM_CURSOR_ID(32644u32);
pub const OCR_UP: SYSTEM_CURSOR_ID = SYSTEM_CURSOR_ID(32516u32);
pub const OCR_WAIT: SYSTEM_CURSOR_ID = SYSTEM_CURSOR_ID(32514u32);
pub const OIC_BANG: u32 = 32515u32;
pub const OIC_ERROR: u32 = 32513u32;
pub const OIC_HAND: u32 = 32513u32;
pub const OIC_INFORMATION: u32 = 32516u32;
pub const OIC_NOTE: u32 = 32516u32;
pub const OIC_QUES: u32 = 32514u32;
pub const OIC_SAMPLE: u32 = 32512u32;
pub const OIC_SHIELD: u32 = 32518u32;
pub const OIC_WARNING: u32 = 32515u32;
pub const OIC_WINLOGO: u32 = 32517u32;
pub const ORD_LANGDRIVER: u32 = 1u32;
pub const PA_ACTIVATE: u32 = 1u32;
pub const PA_NOACTIVATE: u32 = 3u32;
pub const PBTF_APMRESUMEFROMFAILURE: u32 = 1u32;
pub const PBT_APMBATTERYLOW: u32 = 9u32;
pub const PBT_APMOEMEVENT: u32 = 11u32;
pub const PBT_APMPOWERSTATUSCHANGE: u32 = 10u32;
pub const PBT_APMQUERYSTANDBY: u32 = 1u32;
pub const PBT_APMQUERYSTANDBYFAILED: u32 = 3u32;
pub const PBT_APMQUERYSUSPEND: u32 = 0u32;
pub const PBT_APMQUERYSUSPENDFAILED: u32 = 2u32;
pub const PBT_APMRESUMEAUTOMATIC: u32 = 18u32;
pub const PBT_APMRESUMECRITICAL: u32 = 6u32;
pub const PBT_APMRESUMESTANDBY: u32 = 8u32;
pub const PBT_APMRESUMESUSPEND: u32 = 7u32;
pub const PBT_APMSTANDBY: u32 = 5u32;
pub const PBT_APMSUSPEND: u32 = 4u32;
pub const PBT_POWERSETTINGCHANGE: u32 = 32787u32;
pub const PDC_ARRIVAL: u32 = 1u32;
pub const PDC_MAPPING_CHANGE: u32 = 256u32;
pub const PDC_MODE_ASPECTRATIOPRESERVED: u32 = 2048u32;
pub const PDC_MODE_CENTERED: u32 = 128u32;
pub const PDC_MODE_DEFAULT: u32 = 64u32;
pub const PDC_ORIENTATION_0: u32 = 4u32;
pub const PDC_ORIENTATION_180: u32 = 16u32;
pub const PDC_ORIENTATION_270: u32 = 32u32;
pub const PDC_ORIENTATION_90: u32 = 8u32;
pub const PDC_ORIGIN: u32 = 1024u32;
pub const PDC_REMOVAL: u32 = 2u32;
pub const PDC_RESOLUTION: u32 = 512u32;
pub const PENARBITRATIONTYPE_FIS: u32 = 2u32;
pub const PENARBITRATIONTYPE_MAX: u32 = 4u32;
pub const PENARBITRATIONTYPE_NONE: u32 = 0u32;
pub const PENARBITRATIONTYPE_SPT: u32 = 3u32;
pub const PENARBITRATIONTYPE_WIN8: u32 = 1u32;
pub const PENVISUALIZATION_CURSOR: u32 = 32u32;
pub const PENVISUALIZATION_DOUBLETAP: u32 = 2u32;
pub const PENVISUALIZATION_OFF: u32 = 0u32;
pub const PENVISUALIZATION_ON: u32 = 35u32;
pub const PENVISUALIZATION_TAP: u32 = 1u32;
pub const PEN_FLAG_BARREL: u32 = 1u32;
pub const PEN_FLAG_ERASER: u32 = 4u32;
pub const PEN_FLAG_INVERTED: u32 = 2u32;
pub const PEN_FLAG_NONE: u32 = 0u32;
pub const PEN_MASK_NONE: u32 = 0u32;
pub const PEN_MASK_PRESSURE: u32 = 1u32;
pub const PEN_MASK_ROTATION: u32 = 2u32;
pub const PEN_MASK_TILT_X: u32 = 4u32;
pub const PEN_MASK_TILT_Y: u32 = 8u32;
pub const PMB_ACTIVE: u32 = 1u32;
pub const PM_NOREMOVE: PEEK_MESSAGE_REMOVE_TYPE = PEEK_MESSAGE_REMOVE_TYPE(0u32);
pub const PM_NOYIELD: PEEK_MESSAGE_REMOVE_TYPE = PEEK_MESSAGE_REMOVE_TYPE(2u32);
pub const PM_QS_INPUT: PEEK_MESSAGE_REMOVE_TYPE = PEEK_MESSAGE_REMOVE_TYPE(67567616u32);
pub const PM_QS_PAINT: PEEK_MESSAGE_REMOVE_TYPE = PEEK_MESSAGE_REMOVE_TYPE(2097152u32);
pub const PM_QS_POSTMESSAGE: PEEK_MESSAGE_REMOVE_TYPE = PEEK_MESSAGE_REMOVE_TYPE(9961472u32);
pub const PM_QS_SENDMESSAGE: PEEK_MESSAGE_REMOVE_TYPE = PEEK_MESSAGE_REMOVE_TYPE(4194304u32);
pub const PM_REMOVE: PEEK_MESSAGE_REMOVE_TYPE = PEEK_MESSAGE_REMOVE_TYPE(1u32);
pub const POINTER_DEVICE_PRODUCT_STRING_MAX: u32 = 520u32;
pub const POINTER_MESSAGE_FLAG_CANCELED: u32 = 32768u32;
pub const POINTER_MESSAGE_FLAG_CONFIDENCE: u32 = 16384u32;
pub const POINTER_MESSAGE_FLAG_FIFTHBUTTON: u32 = 256u32;
pub const POINTER_MESSAGE_FLAG_FIRSTBUTTON: u32 = 16u32;
pub const POINTER_MESSAGE_FLAG_FOURTHBUTTON: u32 = 128u32;
pub const POINTER_MESSAGE_FLAG_INCONTACT: u32 = 4u32;
pub const POINTER_MESSAGE_FLAG_INRANGE: u32 = 2u32;
pub const POINTER_MESSAGE_FLAG_NEW: u32 = 1u32;
pub const POINTER_MESSAGE_FLAG_PRIMARY: u32 = 8192u32;
pub const POINTER_MESSAGE_FLAG_SECONDBUTTON: u32 = 32u32;
pub const POINTER_MESSAGE_FLAG_THIRDBUTTON: u32 = 64u32;
pub const POINTER_MOD_CTRL: u32 = 8u32;
pub const POINTER_MOD_SHIFT: u32 = 4u32;
pub const PRF_CHECKVISIBLE: i32 = 1i32;
pub const PRF_CHILDREN: i32 = 16i32;
pub const PRF_CLIENT: i32 = 4i32;
pub const PRF_ERASEBKGND: i32 = 8i32;
pub const PRF_NONCLIENT: i32 = 2i32;
pub const PRF_OWNED: i32 = 32i32;
pub const PT_MOUSE: POINTER_INPUT_TYPE = POINTER_INPUT_TYPE(4i32);
pub const PT_PEN: POINTER_INPUT_TYPE = POINTER_INPUT_TYPE(3i32);
pub const PT_POINTER: POINTER_INPUT_TYPE = POINTER_INPUT_TYPE(1i32);
pub const PT_TOUCH: POINTER_INPUT_TYPE = POINTER_INPUT_TYPE(2i32);
pub const PT_TOUCHPAD: POINTER_INPUT_TYPE = POINTER_INPUT_TYPE(5i32);
pub const PWR_CRITICALRESUME: u32 = 3u32;
pub const PWR_FAIL: i32 = -1i32;
pub const PWR_OK: u32 = 1u32;
pub const PWR_SUSPENDREQUEST: u32 = 1u32;
pub const PWR_SUSPENDRESUME: u32 = 2u32;
pub const PW_RENDERFULLCONTENT: u32 = 2u32;
pub const QS_ALLEVENTS: QUEUE_STATUS_FLAGS = QUEUE_STATUS_FLAGS(1215u32);
pub const QS_ALLINPUT: QUEUE_STATUS_FLAGS = QUEUE_STATUS_FLAGS(1279u32);
pub const QS_ALLPOSTMESSAGE: QUEUE_STATUS_FLAGS = QUEUE_STATUS_FLAGS(256u32);
pub const QS_HOTKEY: QUEUE_STATUS_FLAGS = QUEUE_STATUS_FLAGS(128u32);
pub const QS_INPUT: QUEUE_STATUS_FLAGS = QUEUE_STATUS_FLAGS(1031u32);
pub const QS_KEY: QUEUE_STATUS_FLAGS = QUEUE_STATUS_FLAGS(1u32);
pub const QS_MOUSE: QUEUE_STATUS_FLAGS = QUEUE_STATUS_FLAGS(6u32);
pub const QS_MOUSEBUTTON: QUEUE_STATUS_FLAGS = QUEUE_STATUS_FLAGS(4u32);
pub const QS_MOUSEMOVE: QUEUE_STATUS_FLAGS = QUEUE_STATUS_FLAGS(2u32);
pub const QS_PAINT: QUEUE_STATUS_FLAGS = QUEUE_STATUS_FLAGS(32u32);
pub const QS_POINTER: u32 = 4096u32;
pub const QS_POSTMESSAGE: QUEUE_STATUS_FLAGS = QUEUE_STATUS_FLAGS(8u32);
pub const QS_RAWINPUT: QUEUE_STATUS_FLAGS = QUEUE_STATUS_FLAGS(1024u32);
pub const QS_SENDMESSAGE: QUEUE_STATUS_FLAGS = QUEUE_STATUS_FLAGS(64u32);
pub const QS_TIMER: QUEUE_STATUS_FLAGS = QUEUE_STATUS_FLAGS(16u32);
pub const QS_TOUCH: u32 = 2048u32;
pub const RES_CURSOR: u32 = 2u32;
pub const RES_ICON: u32 = 1u32;
pub const RIDEV_EXMODEMASK: u32 = 240u32;
pub const RIM_INPUT: u32 = 0u32;
pub const RIM_INPUTSINK: u32 = 1u32;
pub const RIM_TYPEMAX: u32 = 2u32;
pub const RI_KEY_BREAK: u32 = 1u32;
pub const RI_KEY_E0: u32 = 2u32;
pub const RI_KEY_E1: u32 = 4u32;
pub const RI_KEY_MAKE: u32 = 0u32;
pub const RI_KEY_TERMSRV_SET_LED: u32 = 8u32;
pub const RI_KEY_TERMSRV_SHADOW: u32 = 16u32;
pub const RI_MOUSE_BUTTON_1_DOWN: u32 = 1u32;
pub const RI_MOUSE_BUTTON_1_UP: u32 = 2u32;
pub const RI_MOUSE_BUTTON_2_DOWN: u32 = 4u32;
pub const RI_MOUSE_BUTTON_2_UP: u32 = 8u32;
pub const RI_MOUSE_BUTTON_3_DOWN: u32 = 16u32;
pub const RI_MOUSE_BUTTON_3_UP: u32 = 32u32;
pub const RI_MOUSE_BUTTON_4_DOWN: u32 = 64u32;
pub const RI_MOUSE_BUTTON_4_UP: u32 = 128u32;
pub const RI_MOUSE_BUTTON_5_DOWN: u32 = 256u32;
pub const RI_MOUSE_BUTTON_5_UP: u32 = 512u32;
pub const RI_MOUSE_HWHEEL: u32 = 2048u32;
pub const RI_MOUSE_LEFT_BUTTON_DOWN: u32 = 1u32;
pub const RI_MOUSE_LEFT_BUTTON_UP: u32 = 2u32;
pub const RI_MOUSE_MIDDLE_BUTTON_DOWN: u32 = 16u32;
pub const RI_MOUSE_MIDDLE_BUTTON_UP: u32 = 32u32;
pub const RI_MOUSE_RIGHT_BUTTON_DOWN: u32 = 4u32;
pub const RI_MOUSE_RIGHT_BUTTON_UP: u32 = 8u32;
pub const RI_MOUSE_WHEEL: u32 = 1024u32;
pub const RT_ACCELERATOR: windows_core::PCWSTR = windows_core::PCWSTR(9u16 as _);
pub const RT_ANICURSOR: windows_core::PCWSTR = windows_core::PCWSTR(21u16 as _);
pub const RT_ANIICON: windows_core::PCWSTR = windows_core::PCWSTR(22u16 as _);
pub const RT_BITMAP: windows_core::PCWSTR = windows_core::PCWSTR(2u16 as _);
pub const RT_CURSOR: windows_core::PCWSTR = windows_core::PCWSTR(1u16 as _);
pub const RT_DIALOG: windows_core::PCWSTR = windows_core::PCWSTR(5u16 as _);
pub const RT_DLGINCLUDE: windows_core::PCWSTR = windows_core::PCWSTR(17u16 as _);
pub const RT_FONT: windows_core::PCWSTR = windows_core::PCWSTR(8u16 as _);
pub const RT_FONTDIR: windows_core::PCWSTR = windows_core::PCWSTR(7u16 as _);
pub const RT_HTML: windows_core::PCWSTR = windows_core::PCWSTR(23u16 as _);
pub const RT_ICON: windows_core::PCWSTR = windows_core::PCWSTR(3u16 as _);
pub const RT_MANIFEST: u32 = 24u32;
pub const RT_MENU: windows_core::PCWSTR = windows_core::PCWSTR(4u16 as _);
pub const RT_MESSAGETABLE: windows_core::PCWSTR = windows_core::PCWSTR(11u16 as _);
pub const RT_PLUGPLAY: windows_core::PCWSTR = windows_core::PCWSTR(19u16 as _);
pub const RT_VERSION: windows_core::PCWSTR = windows_core::PCWSTR(16u16 as _);
pub const RT_VXD: windows_core::PCWSTR = windows_core::PCWSTR(20u16 as _);
pub const SBM_ENABLE_ARROWS: u32 = 228u32;
pub const SBM_GETPOS: u32 = 225u32;
pub const SBM_GETRANGE: u32 = 227u32;
pub const SBM_GETSCROLLBARINFO: u32 = 235u32;
pub const SBM_GETSCROLLINFO: u32 = 234u32;
pub const SBM_SETPOS: u32 = 224u32;
pub const SBM_SETRANGE: u32 = 226u32;
pub const SBM_SETRANGEREDRAW: u32 = 230u32;
pub const SBM_SETSCROLLINFO: u32 = 233u32;
pub const SBS_BOTTOMALIGN: i32 = 4i32;
pub const SBS_HORZ: i32 = 0i32;
pub const SBS_LEFTALIGN: i32 = 2i32;
pub const SBS_RIGHTALIGN: i32 = 4i32;
pub const SBS_SIZEBOX: i32 = 8i32;
pub const SBS_SIZEBOXBOTTOMRIGHTALIGN: i32 = 4i32;
pub const SBS_SIZEBOXTOPLEFTALIGN: i32 = 2i32;
pub const SBS_SIZEGRIP: i32 = 16i32;
pub const SBS_TOPALIGN: i32 = 2i32;
pub const SBS_VERT: i32 = 1i32;
pub const SB_BOTH: SCROLLBAR_CONSTANTS = SCROLLBAR_CONSTANTS(3i32);
pub const SB_BOTTOM: SCROLLBAR_COMMAND = SCROLLBAR_COMMAND(7i32);
pub const SB_CTL: SCROLLBAR_CONSTANTS = SCROLLBAR_CONSTANTS(2i32);
pub const SB_ENDSCROLL: SCROLLBAR_COMMAND = SCROLLBAR_COMMAND(8i32);
pub const SB_HORZ: SCROLLBAR_CONSTANTS = SCROLLBAR_CONSTANTS(0i32);
pub const SB_LEFT: SCROLLBAR_COMMAND = SCROLLBAR_COMMAND(6i32);
pub const SB_LINEDOWN: SCROLLBAR_COMMAND = SCROLLBAR_COMMAND(1i32);
pub const SB_LINELEFT: SCROLLBAR_COMMAND = SCROLLBAR_COMMAND(0i32);
pub const SB_LINERIGHT: SCROLLBAR_COMMAND = SCROLLBAR_COMMAND(1i32);
pub const SB_LINEUP: SCROLLBAR_COMMAND = SCROLLBAR_COMMAND(0i32);
pub const SB_PAGEDOWN: SCROLLBAR_COMMAND = SCROLLBAR_COMMAND(3i32);
pub const SB_PAGELEFT: SCROLLBAR_COMMAND = SCROLLBAR_COMMAND(2i32);
pub const SB_PAGERIGHT: SCROLLBAR_COMMAND = SCROLLBAR_COMMAND(3i32);
pub const SB_PAGEUP: SCROLLBAR_COMMAND = SCROLLBAR_COMMAND(2i32);
pub const SB_RIGHT: SCROLLBAR_COMMAND = SCROLLBAR_COMMAND(7i32);
pub const SB_THUMBPOSITION: SCROLLBAR_COMMAND = SCROLLBAR_COMMAND(4i32);
pub const SB_THUMBTRACK: SCROLLBAR_COMMAND = SCROLLBAR_COMMAND(5i32);
pub const SB_TOP: SCROLLBAR_COMMAND = SCROLLBAR_COMMAND(6i32);
pub const SB_VERT: SCROLLBAR_CONSTANTS = SCROLLBAR_CONSTANTS(1i32);
pub const SCF_ISSECURE: u32 = 1u32;
pub const SC_ARRANGE: u32 = 61712u32;
pub const SC_CLOSE: u32 = 61536u32;
pub const SC_CONTEXTHELP: u32 = 61824u32;
pub const SC_DEFAULT: u32 = 61792u32;
pub const SC_HOTKEY: u32 = 61776u32;
pub const SC_HSCROLL: u32 = 61568u32;
pub const SC_ICON: u32 = 61472u32;
pub const SC_KEYMENU: u32 = 61696u32;
pub const SC_MAXIMIZE: u32 = 61488u32;
pub const SC_MINIMIZE: u32 = 61472u32;
pub const SC_MONITORPOWER: u32 = 61808u32;
pub const SC_MOUSEMENU: u32 = 61584u32;
pub const SC_MOVE: u32 = 61456u32;
pub const SC_NEXTWINDOW: u32 = 61504u32;
pub const SC_PREVWINDOW: u32 = 61520u32;
pub const SC_RESTORE: u32 = 61728u32;
pub const SC_SEPARATOR: u32 = 61455u32;
pub const SC_SIZE: u32 = 61440u32;
pub const SC_TASKLIST: u32 = 61744u32;
pub const SC_VSCROLL: u32 = 61552u32;
pub const SC_ZOOM: u32 = 61488u32;
pub const SHOW_FULLSCREEN: u32 = 3u32;
pub const SHOW_ICONWINDOW: u32 = 2u32;
pub const SHOW_OPENNOACTIVATE: u32 = 4u32;
pub const SHOW_OPENWINDOW: u32 = 1u32;
pub const SIF_ALL: SCROLLINFO_MASK = SCROLLINFO_MASK(23u32);
pub const SIF_DISABLENOSCROLL: SCROLLINFO_MASK = SCROLLINFO_MASK(8u32);
pub const SIF_PAGE: SCROLLINFO_MASK = SCROLLINFO_MASK(2u32);
pub const SIF_POS: SCROLLINFO_MASK = SCROLLINFO_MASK(4u32);
pub const SIF_RANGE: SCROLLINFO_MASK = SCROLLINFO_MASK(1u32);
pub const SIF_TRACKPOS: SCROLLINFO_MASK = SCROLLINFO_MASK(16u32);
pub const SIZEFULLSCREEN: u32 = 2u32;
pub const SIZEICONIC: u32 = 1u32;
pub const SIZENORMAL: u32 = 0u32;
pub const SIZEZOOMHIDE: u32 = 4u32;
pub const SIZEZOOMSHOW: u32 = 3u32;
pub const SIZE_MAXHIDE: u32 = 4u32;
pub const SIZE_MAXIMIZED: u32 = 2u32;
pub const SIZE_MAXSHOW: u32 = 3u32;
pub const SIZE_MINIMIZED: u32 = 1u32;
pub const SIZE_RESTORED: u32 = 0u32;
pub const SMTO_ABORTIFHUNG: SEND_MESSAGE_TIMEOUT_FLAGS = SEND_MESSAGE_TIMEOUT_FLAGS(2u32);
pub const SMTO_BLOCK: SEND_MESSAGE_TIMEOUT_FLAGS = SEND_MESSAGE_TIMEOUT_FLAGS(1u32);
pub const SMTO_ERRORONEXIT: SEND_MESSAGE_TIMEOUT_FLAGS = SEND_MESSAGE_TIMEOUT_FLAGS(32u32);
pub const SMTO_NORMAL: SEND_MESSAGE_TIMEOUT_FLAGS = SEND_MESSAGE_TIMEOUT_FLAGS(0u32);
pub const SMTO_NOTIMEOUTIFNOTHUNG: SEND_MESSAGE_TIMEOUT_FLAGS = SEND_MESSAGE_TIMEOUT_FLAGS(8u32);
pub const SM_ARRANGE: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(56i32);
pub const SM_CARETBLINKINGENABLED: u32 = 8194u32;
pub const SM_CLEANBOOT: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(67i32);
pub const SM_CMETRICS: u32 = 76u32;
pub const SM_CMONITORS: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(80i32);
pub const SM_CMOUSEBUTTONS: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(43i32);
pub const SM_CONVERTIBLESLATEMODE: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(8195i32);
pub const SM_CXBORDER: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(5i32);
pub const SM_CXCURSOR: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(13i32);
pub const SM_CXDLGFRAME: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(7i32);
pub const SM_CXDOUBLECLK: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(36i32);
pub const SM_CXDRAG: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(68i32);
pub const SM_CXEDGE: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(45i32);
pub const SM_CXFIXEDFRAME: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(7i32);
pub const SM_CXFOCUSBORDER: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(83i32);
pub const SM_CXFRAME: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(32i32);
pub const SM_CXFULLSCREEN: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(16i32);
pub const SM_CXHSCROLL: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(21i32);
pub const SM_CXHTHUMB: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(10i32);
pub const SM_CXICON: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(11i32);
pub const SM_CXICONSPACING: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(38i32);
pub const SM_CXMAXIMIZED: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(61i32);
pub const SM_CXMAXTRACK: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(59i32);
pub const SM_CXMENUCHECK: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(71i32);
pub const SM_CXMENUSIZE: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(54i32);
pub const SM_CXMIN: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(28i32);
pub const SM_CXMINIMIZED: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(57i32);
pub const SM_CXMINSPACING: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(47i32);
pub const SM_CXMINTRACK: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(34i32);
pub const SM_CXPADDEDBORDER: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(92i32);
pub const SM_CXSCREEN: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(0i32);
pub const SM_CXSIZE: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(30i32);
pub const SM_CXSIZEFRAME: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(32i32);
pub const SM_CXSMICON: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(49i32);
pub const SM_CXSMSIZE: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(52i32);
pub const SM_CXVIRTUALSCREEN: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(78i32);
pub const SM_CXVSCROLL: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(2i32);
pub const SM_CYBORDER: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(6i32);
pub const SM_CYCAPTION: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(4i32);
pub const SM_CYCURSOR: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(14i32);
pub const SM_CYDLGFRAME: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(8i32);
pub const SM_CYDOUBLECLK: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(37i32);
pub const SM_CYDRAG: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(69i32);
pub const SM_CYEDGE: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(46i32);
pub const SM_CYFIXEDFRAME: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(8i32);
pub const SM_CYFOCUSBORDER: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(84i32);
pub const SM_CYFRAME: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(33i32);
pub const SM_CYFULLSCREEN: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(17i32);
pub const SM_CYHSCROLL: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(3i32);
pub const SM_CYICON: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(12i32);
pub const SM_CYICONSPACING: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(39i32);
pub const SM_CYKANJIWINDOW: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(18i32);
pub const SM_CYMAXIMIZED: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(62i32);
pub const SM_CYMAXTRACK: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(60i32);
pub const SM_CYMENU: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(15i32);
pub const SM_CYMENUCHECK: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(72i32);
pub const SM_CYMENUSIZE: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(55i32);
pub const SM_CYMIN: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(29i32);
pub const SM_CYMINIMIZED: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(58i32);
pub const SM_CYMINSPACING: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(48i32);
pub const SM_CYMINTRACK: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(35i32);
pub const SM_CYSCREEN: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(1i32);
pub const SM_CYSIZE: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(31i32);
pub const SM_CYSIZEFRAME: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(33i32);
pub const SM_CYSMCAPTION: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(51i32);
pub const SM_CYSMICON: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(50i32);
pub const SM_CYSMSIZE: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(53i32);
pub const SM_CYVIRTUALSCREEN: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(79i32);
pub const SM_CYVSCROLL: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(20i32);
pub const SM_CYVTHUMB: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(9i32);
pub const SM_DBCSENABLED: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(42i32);
pub const SM_DEBUG: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(22i32);
pub const SM_DIGITIZER: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(94i32);
pub const SM_IMMENABLED: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(82i32);
pub const SM_MAXIMUMTOUCHES: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(95i32);
pub const SM_MEDIACENTER: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(87i32);
pub const SM_MENUDROPALIGNMENT: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(40i32);
pub const SM_MIDEASTENABLED: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(74i32);
pub const SM_MOUSEHORIZONTALWHEELPRESENT: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(91i32);
pub const SM_MOUSEPRESENT: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(19i32);
pub const SM_MOUSEWHEELPRESENT: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(75i32);
pub const SM_NETWORK: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(63i32);
pub const SM_PENWINDOWS: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(41i32);
pub const SM_REMOTECONTROL: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(8193i32);
pub const SM_REMOTESESSION: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(4096i32);
pub const SM_RESERVED1: u32 = 24u32;
pub const SM_RESERVED2: u32 = 25u32;
pub const SM_RESERVED3: u32 = 26u32;
pub const SM_RESERVED4: u32 = 27u32;
pub const SM_SAMEDISPLAYFORMAT: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(81i32);
pub const SM_SECURE: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(44i32);
pub const SM_SERVERR2: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(89i32);
pub const SM_SHOWSOUNDS: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(70i32);
pub const SM_SHUTTINGDOWN: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(8192i32);
pub const SM_SLOWMACHINE: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(73i32);
pub const SM_STARTER: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(88i32);
pub const SM_SWAPBUTTON: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(23i32);
pub const SM_SYSTEMDOCKED: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(8196i32);
pub const SM_TABLETPC: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(86i32);
pub const SM_XVIRTUALSCREEN: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(76i32);
pub const SM_YVIRTUALSCREEN: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(77i32);
pub const SOUND_SYSTEM_APPEND: u32 = 14u32;
pub const SOUND_SYSTEM_APPSTART: u32 = 12u32;
pub const SOUND_SYSTEM_BEEP: u32 = 3u32;
pub const SOUND_SYSTEM_ERROR: u32 = 4u32;
pub const SOUND_SYSTEM_FAULT: u32 = 13u32;
pub const SOUND_SYSTEM_INFORMATION: u32 = 7u32;
pub const SOUND_SYSTEM_MAXIMIZE: u32 = 8u32;
pub const SOUND_SYSTEM_MENUCOMMAND: u32 = 15u32;
pub const SOUND_SYSTEM_MENUPOPUP: u32 = 16u32;
pub const SOUND_SYSTEM_MINIMIZE: u32 = 9u32;
pub const SOUND_SYSTEM_QUESTION: u32 = 5u32;
pub const SOUND_SYSTEM_RESTOREDOWN: u32 = 11u32;
pub const SOUND_SYSTEM_RESTOREUP: u32 = 10u32;
pub const SOUND_SYSTEM_SHUTDOWN: u32 = 2u32;
pub const SOUND_SYSTEM_STARTUP: u32 = 1u32;
pub const SOUND_SYSTEM_WARNING: u32 = 6u32;
pub const SPIF_SENDCHANGE: SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS = SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(2u32);
pub const SPIF_SENDWININICHANGE: SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS = SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(2u32);
pub const SPIF_UPDATEINIFILE: SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS = SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(1u32);
pub const SPI_GETACCESSTIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(60u32);
pub const SPI_GETACTIVEWINDOWTRACKING: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4096u32);
pub const SPI_GETACTIVEWNDTRKTIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8194u32);
pub const SPI_GETACTIVEWNDTRKZORDER: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4108u32);
pub const SPI_GETANIMATION: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(72u32);
pub const SPI_GETAUDIODESCRIPTION: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(116u32);
pub const SPI_GETBEEP: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(1u32);
pub const SPI_GETBLOCKSENDINPUTRESETS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4134u32);
pub const SPI_GETBORDER: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(5u32);
pub const SPI_GETCARETBROWSING: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4172u32);
pub const SPI_GETCARETTIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8226u32);
pub const SPI_GETCARETWIDTH: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8198u32);
pub const SPI_GETCLEARTYPE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4168u32);
pub const SPI_GETCLIENTAREAANIMATION: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4162u32);
pub const SPI_GETCOMBOBOXANIMATION: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4100u32);
pub const SPI_GETCONTACTVISUALIZATION: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8216u32);
pub const SPI_GETCURSORSHADOW: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4122u32);
pub const SPI_GETDEFAULTINPUTLANG: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(89u32);
pub const SPI_GETDESKWALLPAPER: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(115u32);
pub const SPI_GETDISABLEOVERLAPPEDCONTENT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4160u32);
pub const SPI_GETDOCKMOVING: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(144u32);
pub const SPI_GETDRAGFROMMAXIMIZE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(140u32);
pub const SPI_GETDRAGFULLWINDOWS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(38u32);
pub const SPI_GETDROPSHADOW: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4132u32);
pub const SPI_GETFASTTASKSWITCH: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(35u32);
pub const SPI_GETFILTERKEYS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(50u32);
pub const SPI_GETFLATMENU: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4130u32);
pub const SPI_GETFOCUSBORDERHEIGHT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8208u32);
pub const SPI_GETFOCUSBORDERWIDTH: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8206u32);
pub const SPI_GETFONTSMOOTHING: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(74u32);
pub const SPI_GETFONTSMOOTHINGCONTRAST: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8204u32);
pub const SPI_GETFONTSMOOTHINGORIENTATION: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8210u32);
pub const SPI_GETFONTSMOOTHINGTYPE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8202u32);
pub const SPI_GETFOREGROUNDFLASHCOUNT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8196u32);
pub const SPI_GETFOREGROUNDLOCKTIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8192u32);
pub const SPI_GETGESTUREVISUALIZATION: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8218u32);
pub const SPI_GETGRADIENTCAPTIONS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4104u32);
pub const SPI_GETGRIDGRANULARITY: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(18u32);
pub const SPI_GETHANDEDNESS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8228u32);
pub const SPI_GETHIGHCONTRAST: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(66u32);
pub const SPI_GETHOTTRACKING: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4110u32);
pub const SPI_GETHUNGAPPTIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(120u32);
pub const SPI_GETICONMETRICS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(45u32);
pub const SPI_GETICONTITLELOGFONT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(31u32);
pub const SPI_GETICONTITLEWRAP: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(25u32);
pub const SPI_GETKEYBOARDCUES: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4106u32);
pub const SPI_GETKEYBOARDDELAY: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(22u32);
pub const SPI_GETKEYBOARDPREF: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(68u32);
pub const SPI_GETKEYBOARDSPEED: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(10u32);
pub const SPI_GETLISTBOXSMOOTHSCROLLING: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4102u32);
pub const SPI_GETLOGICALDPIOVERRIDE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(158u32);
pub const SPI_GETLOWPOWERACTIVE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(83u32);
pub const SPI_GETLOWPOWERTIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(79u32);
pub const SPI_GETMENUANIMATION: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4098u32);
pub const SPI_GETMENUDROPALIGNMENT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(27u32);
pub const SPI_GETMENUFADE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4114u32);
pub const SPI_GETMENURECT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(162u32);
pub const SPI_GETMENUSHOWDELAY: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(106u32);
pub const SPI_GETMENUUNDERLINES: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4106u32);
pub const SPI_GETMESSAGEDURATION: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8214u32);
pub const SPI_GETMINIMIZEDMETRICS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(43u32);
pub const SPI_GETMINIMUMHITRADIUS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8212u32);
pub const SPI_GETMOUSE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(3u32);
pub const SPI_GETMOUSECLICKLOCK: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4126u32);
pub const SPI_GETMOUSECLICKLOCKTIME: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8200u32);
pub const SPI_GETMOUSEDOCKTHRESHOLD: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(126u32);
pub const SPI_GETMOUSEDRAGOUTTHRESHOLD: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(132u32);
pub const SPI_GETMOUSEHOVERHEIGHT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(100u32);
pub const SPI_GETMOUSEHOVERTIME: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(102u32);
pub const SPI_GETMOUSEHOVERWIDTH: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(98u32);
pub const SPI_GETMOUSEKEYS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(54u32);
pub const SPI_GETMOUSESIDEMOVETHRESHOLD: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(136u32);
pub const SPI_GETMOUSESONAR: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4124u32);
pub const SPI_GETMOUSESPEED: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(112u32);
pub const SPI_GETMOUSETRAILS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(94u32);
pub const SPI_GETMOUSEVANISH: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4128u32);
pub const SPI_GETMOUSEWHEELROUTING: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8220u32);
pub const SPI_GETNONCLIENTMETRICS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(41u32);
pub const SPI_GETPENARBITRATIONTYPE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8224u32);
pub const SPI_GETPENDOCKTHRESHOLD: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(128u32);
pub const SPI_GETPENDRAGOUTTHRESHOLD: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(134u32);
pub const SPI_GETPENSIDEMOVETHRESHOLD: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(138u32);
pub const SPI_GETPENVISUALIZATION: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8222u32);
pub const SPI_GETPOWEROFFACTIVE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(84u32);
pub const SPI_GETPOWEROFFTIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(80u32);
pub const SPI_GETSCREENREADER: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(70u32);
pub const SPI_GETSCREENSAVEACTIVE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(16u32);
pub const SPI_GETSCREENSAVERRUNNING: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(114u32);
pub const SPI_GETSCREENSAVESECURE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(118u32);
pub const SPI_GETSCREENSAVETIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(14u32);
pub const SPI_GETSELECTIONFADE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4116u32);
pub const SPI_GETSERIALKEYS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(62u32);
pub const SPI_GETSHOWIMEUI: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(110u32);
pub const SPI_GETSHOWSOUNDS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(56u32);
pub const SPI_GETSNAPSIZING: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(142u32);
pub const SPI_GETSNAPTODEFBUTTON: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(95u32);
pub const SPI_GETSOUNDSENTRY: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(64u32);
pub const SPI_GETSPEECHRECOGNITION: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4170u32);
pub const SPI_GETSTICKYKEYS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(58u32);
pub const SPI_GETSYSTEMLANGUAGEBAR: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4176u32);
pub const SPI_GETTHREADLOCALINPUTSETTINGS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4174u32);
pub const SPI_GETTOGGLEKEYS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(52u32);
pub const SPI_GETTOOLTIPANIMATION: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4118u32);
pub const SPI_GETTOOLTIPFADE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4120u32);
pub const SPI_GETTOUCHPREDICTIONPARAMETERS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(156u32);
pub const SPI_GETUIEFFECTS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4158u32);
pub const SPI_GETWAITTOKILLSERVICETIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(124u32);
pub const SPI_GETWAITTOKILLTIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(122u32);
pub const SPI_GETWHEELSCROLLCHARS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(108u32);
pub const SPI_GETWHEELSCROLLLINES: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(104u32);
pub const SPI_GETWINARRANGING: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(130u32);
pub const SPI_GETWINDOWSEXTENSION: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(92u32);
pub const SPI_GETWORKAREA: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(48u32);
pub const SPI_ICONHORIZONTALSPACING: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(13u32);
pub const SPI_ICONVERTICALSPACING: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(24u32);
pub const SPI_LANGDRIVER: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(12u32);
pub const SPI_SCREENSAVERRUNNING: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(97u32);
pub const SPI_SETACCESSTIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(61u32);
pub const SPI_SETACTIVEWINDOWTRACKING: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4097u32);
pub const SPI_SETACTIVEWNDTRKTIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8195u32);
pub const SPI_SETACTIVEWNDTRKZORDER: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4109u32);
pub const SPI_SETANIMATION: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(73u32);
pub const SPI_SETAUDIODESCRIPTION: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(117u32);
pub const SPI_SETBEEP: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(2u32);
pub const SPI_SETBLOCKSENDINPUTRESETS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4135u32);
pub const SPI_SETBORDER: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(6u32);
pub const SPI_SETCARETBROWSING: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4173u32);
pub const SPI_SETCARETTIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8227u32);
pub const SPI_SETCARETWIDTH: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8199u32);
pub const SPI_SETCLEARTYPE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4169u32);
pub const SPI_SETCLIENTAREAANIMATION: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4163u32);
pub const SPI_SETCOMBOBOXANIMATION: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4101u32);
pub const SPI_SETCONTACTVISUALIZATION: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8217u32);
pub const SPI_SETCURSORS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(87u32);
pub const SPI_SETCURSORSHADOW: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4123u32);
pub const SPI_SETDEFAULTINPUTLANG: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(90u32);
pub const SPI_SETDESKPATTERN: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(21u32);
pub const SPI_SETDESKWALLPAPER: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(20u32);
pub const SPI_SETDISABLEOVERLAPPEDCONTENT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4161u32);
pub const SPI_SETDOCKMOVING: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(145u32);
pub const SPI_SETDOUBLECLICKTIME: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(32u32);
pub const SPI_SETDOUBLECLKHEIGHT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(30u32);
pub const SPI_SETDOUBLECLKWIDTH: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(29u32);
pub const SPI_SETDRAGFROMMAXIMIZE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(141u32);
pub const SPI_SETDRAGFULLWINDOWS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(37u32);
pub const SPI_SETDRAGHEIGHT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(77u32);
pub const SPI_SETDRAGWIDTH: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(76u32);
pub const SPI_SETDROPSHADOW: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4133u32);
pub const SPI_SETFASTTASKSWITCH: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(36u32);
pub const SPI_SETFILTERKEYS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(51u32);
pub const SPI_SETFLATMENU: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4131u32);
pub const SPI_SETFOCUSBORDERHEIGHT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8209u32);
pub const SPI_SETFOCUSBORDERWIDTH: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8207u32);
pub const SPI_SETFONTSMOOTHING: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(75u32);
pub const SPI_SETFONTSMOOTHINGCONTRAST: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8205u32);
pub const SPI_SETFONTSMOOTHINGORIENTATION: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8211u32);
pub const SPI_SETFONTSMOOTHINGTYPE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8203u32);
pub const SPI_SETFOREGROUNDFLASHCOUNT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8197u32);
pub const SPI_SETFOREGROUNDLOCKTIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8193u32);
pub const SPI_SETGESTUREVISUALIZATION: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8219u32);
pub const SPI_SETGRADIENTCAPTIONS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4105u32);
pub const SPI_SETGRIDGRANULARITY: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(19u32);
pub const SPI_SETHANDEDNESS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8229u32);
pub const SPI_SETHANDHELD: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(78u32);
pub const SPI_SETHIGHCONTRAST: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(67u32);
pub const SPI_SETHOTTRACKING: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4111u32);
pub const SPI_SETHUNGAPPTIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(121u32);
pub const SPI_SETICONMETRICS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(46u32);
pub const SPI_SETICONS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(88u32);
pub const SPI_SETICONTITLELOGFONT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(34u32);
pub const SPI_SETICONTITLEWRAP: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(26u32);
pub const SPI_SETKEYBOARDCUES: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4107u32);
pub const SPI_SETKEYBOARDDELAY: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(23u32);
pub const SPI_SETKEYBOARDPREF: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(69u32);
pub const SPI_SETKEYBOARDSPEED: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(11u32);
pub const SPI_SETLANGTOGGLE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(91u32);
pub const SPI_SETLISTBOXSMOOTHSCROLLING: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4103u32);
pub const SPI_SETLOGICALDPIOVERRIDE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(159u32);
pub const SPI_SETLOWPOWERACTIVE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(85u32);
pub const SPI_SETLOWPOWERTIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(81u32);
pub const SPI_SETMENUANIMATION: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4099u32);
pub const SPI_SETMENUDROPALIGNMENT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(28u32);
pub const SPI_SETMENUFADE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4115u32);
pub const SPI_SETMENURECT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(163u32);
pub const SPI_SETMENUSHOWDELAY: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(107u32);
pub const SPI_SETMENUUNDERLINES: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4107u32);
pub const SPI_SETMESSAGEDURATION: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8215u32);
pub const SPI_SETMINIMIZEDMETRICS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(44u32);
pub const SPI_SETMINIMUMHITRADIUS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8213u32);
pub const SPI_SETMOUSE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4u32);
pub const SPI_SETMOUSEBUTTONSWAP: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(33u32);
pub const SPI_SETMOUSECLICKLOCK: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4127u32);
pub const SPI_SETMOUSECLICKLOCKTIME: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8201u32);
pub const SPI_SETMOUSEDOCKTHRESHOLD: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(127u32);
pub const SPI_SETMOUSEDRAGOUTTHRESHOLD: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(133u32);
pub const SPI_SETMOUSEHOVERHEIGHT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(101u32);
pub const SPI_SETMOUSEHOVERTIME: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(103u32);
pub const SPI_SETMOUSEHOVERWIDTH: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(99u32);
pub const SPI_SETMOUSEKEYS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(55u32);
pub const SPI_SETMOUSESIDEMOVETHRESHOLD: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(137u32);
pub const SPI_SETMOUSESONAR: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4125u32);
pub const SPI_SETMOUSESPEED: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(113u32);
pub const SPI_SETMOUSETRAILS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(93u32);
pub const SPI_SETMOUSEVANISH: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4129u32);
pub const SPI_SETMOUSEWHEELROUTING: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8221u32);
pub const SPI_SETNONCLIENTMETRICS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(42u32);
pub const SPI_SETPENARBITRATIONTYPE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8225u32);
pub const SPI_SETPENDOCKTHRESHOLD: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(129u32);
pub const SPI_SETPENDRAGOUTTHRESHOLD: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(135u32);
pub const SPI_SETPENSIDEMOVETHRESHOLD: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(139u32);
pub const SPI_SETPENVISUALIZATION: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8223u32);
pub const SPI_SETPENWINDOWS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(49u32);
pub const SPI_SETPOWEROFFACTIVE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(86u32);
pub const SPI_SETPOWEROFFTIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(82u32);
pub const SPI_SETSCREENREADER: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(71u32);
pub const SPI_SETSCREENSAVEACTIVE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(17u32);
pub const SPI_SETSCREENSAVERRUNNING: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(97u32);
pub const SPI_SETSCREENSAVESECURE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(119u32);
pub const SPI_SETSCREENSAVETIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(15u32);
pub const SPI_SETSELECTIONFADE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4117u32);
pub const SPI_SETSERIALKEYS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(63u32);
pub const SPI_SETSHOWIMEUI: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(111u32);
pub const SPI_SETSHOWSOUNDS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(57u32);
pub const SPI_SETSNAPSIZING: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(143u32);
pub const SPI_SETSNAPTODEFBUTTON: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(96u32);
pub const SPI_SETSOUNDSENTRY: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(65u32);
pub const SPI_SETSPEECHRECOGNITION: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4171u32);
pub const SPI_SETSTICKYKEYS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(59u32);
pub const SPI_SETSYSTEMLANGUAGEBAR: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4177u32);
pub const SPI_SETTHREADLOCALINPUTSETTINGS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4175u32);
pub const SPI_SETTOGGLEKEYS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(53u32);
pub const SPI_SETTOOLTIPANIMATION: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4119u32);
pub const SPI_SETTOOLTIPFADE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4121u32);
pub const SPI_SETTOUCHPREDICTIONPARAMETERS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(157u32);
pub const SPI_SETUIEFFECTS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4159u32);
pub const SPI_SETWAITTOKILLSERVICETIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(125u32);
pub const SPI_SETWAITTOKILLTIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(123u32);
pub const SPI_SETWHEELSCROLLCHARS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(109u32);
pub const SPI_SETWHEELSCROLLLINES: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(105u32);
pub const SPI_SETWINARRANGING: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(131u32);
pub const SPI_SETWORKAREA: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(47u32);
pub const STATE_SYSTEM_ALERT_HIGH: u32 = 268435456u32;
pub const STATE_SYSTEM_ALERT_LOW: u32 = 67108864u32;
pub const STATE_SYSTEM_ALERT_MEDIUM: u32 = 134217728u32;
pub const STATE_SYSTEM_ANIMATED: u32 = 16384u32;
pub const STATE_SYSTEM_BUSY: u32 = 2048u32;
pub const STATE_SYSTEM_CHECKED: u32 = 16u32;
pub const STATE_SYSTEM_COLLAPSED: u32 = 1024u32;
pub const STATE_SYSTEM_DEFAULT: u32 = 256u32;
pub const STATE_SYSTEM_EXPANDED: u32 = 512u32;
pub const STATE_SYSTEM_EXTSELECTABLE: u32 = 33554432u32;
pub const STATE_SYSTEM_FLOATING: u32 = 4096u32;
pub const STATE_SYSTEM_FOCUSED: u32 = 4u32;
pub const STATE_SYSTEM_HOTTRACKED: u32 = 128u32;
pub const STATE_SYSTEM_INDETERMINATE: u32 = 32u32;
pub const STATE_SYSTEM_LINKED: u32 = 4194304u32;
pub const STATE_SYSTEM_MARQUEED: u32 = 8192u32;
pub const STATE_SYSTEM_MIXED: u32 = 32u32;
pub const STATE_SYSTEM_MOVEABLE: u32 = 262144u32;
pub const STATE_SYSTEM_MULTISELECTABLE: u32 = 16777216u32;
pub const STATE_SYSTEM_PROTECTED: u32 = 536870912u32;
pub const STATE_SYSTEM_READONLY: u32 = 64u32;
pub const STATE_SYSTEM_SELECTABLE: u32 = 2097152u32;
pub const STATE_SYSTEM_SELECTED: u32 = 2u32;
pub const STATE_SYSTEM_SELFVOICING: u32 = 524288u32;
pub const STATE_SYSTEM_SIZEABLE: u32 = 131072u32;
pub const STATE_SYSTEM_TRAVERSED: u32 = 8388608u32;
pub const STATE_SYSTEM_VALID: u32 = 1073741823u32;
pub const STM_GETICON: u32 = 369u32;
pub const STM_GETIMAGE: u32 = 371u32;
pub const STM_MSGMAX: u32 = 372u32;
pub const STM_SETICON: u32 = 368u32;
pub const STM_SETIMAGE: u32 = 370u32;
pub const STN_CLICKED: u32 = 0u32;
pub const STN_DBLCLK: u32 = 1u32;
pub const STN_DISABLE: u32 = 3u32;
pub const STN_ENABLE: u32 = 2u32;
pub const STRSAFE_E_END_OF_FILE: windows_core::HRESULT = windows_core::HRESULT(0x80070026_u32 as _);
pub const STRSAFE_E_INSUFFICIENT_BUFFER: windows_core::HRESULT = windows_core::HRESULT(0x8007007A_u32 as _);
pub const STRSAFE_E_INVALID_PARAMETER: windows_core::HRESULT = windows_core::HRESULT(0x80070057_u32 as _);
pub const STRSAFE_FILL_BEHIND_NULL: u32 = 512u32;
pub const STRSAFE_FILL_ON_FAILURE: u32 = 1024u32;
pub const STRSAFE_IGNORE_NULLS: u32 = 256u32;
pub const STRSAFE_MAX_CCH: u32 = 2147483647u32;
pub const STRSAFE_MAX_LENGTH: u32 = 2147483646u32;
pub const STRSAFE_NO_TRUNCATION: u32 = 4096u32;
pub const STRSAFE_NULL_ON_FAILURE: u32 = 2048u32;
pub const STRSAFE_USE_SECURE_CRT: u32 = 0u32;
pub const SWP_ASYNCWINDOWPOS: SET_WINDOW_POS_FLAGS = SET_WINDOW_POS_FLAGS(16384u32);
pub const SWP_DEFERERASE: SET_WINDOW_POS_FLAGS = SET_WINDOW_POS_FLAGS(8192u32);
pub const SWP_DRAWFRAME: SET_WINDOW_POS_FLAGS = SET_WINDOW_POS_FLAGS(32u32);
pub const SWP_FRAMECHANGED: SET_WINDOW_POS_FLAGS = SET_WINDOW_POS_FLAGS(32u32);
pub const SWP_HIDEWINDOW: SET_WINDOW_POS_FLAGS = SET_WINDOW_POS_FLAGS(128u32);
pub const SWP_NOACTIVATE: SET_WINDOW_POS_FLAGS = SET_WINDOW_POS_FLAGS(16u32);
pub const SWP_NOCOPYBITS: SET_WINDOW_POS_FLAGS = SET_WINDOW_POS_FLAGS(256u32);
pub const SWP_NOMOVE: SET_WINDOW_POS_FLAGS = SET_WINDOW_POS_FLAGS(2u32);
pub const SWP_NOOWNERZORDER: SET_WINDOW_POS_FLAGS = SET_WINDOW_POS_FLAGS(512u32);
pub const SWP_NOREDRAW: SET_WINDOW_POS_FLAGS = SET_WINDOW_POS_FLAGS(8u32);
pub const SWP_NOREPOSITION: SET_WINDOW_POS_FLAGS = SET_WINDOW_POS_FLAGS(512u32);
pub const SWP_NOSENDCHANGING: SET_WINDOW_POS_FLAGS = SET_WINDOW_POS_FLAGS(1024u32);
pub const SWP_NOSIZE: SET_WINDOW_POS_FLAGS = SET_WINDOW_POS_FLAGS(1u32);
pub const SWP_NOZORDER: SET_WINDOW_POS_FLAGS = SET_WINDOW_POS_FLAGS(4u32);
pub const SWP_SHOWWINDOW: SET_WINDOW_POS_FLAGS = SET_WINDOW_POS_FLAGS(64u32);
pub const SW_ERASE: SCROLL_WINDOW_FLAGS = SCROLL_WINDOW_FLAGS(4u32);
pub const SW_FORCEMINIMIZE: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(11i32);
pub const SW_HIDE: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(0i32);
pub const SW_INVALIDATE: SCROLL_WINDOW_FLAGS = SCROLL_WINDOW_FLAGS(2u32);
pub const SW_MAX: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(11i32);
pub const SW_MAXIMIZE: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(3i32);
pub const SW_MINIMIZE: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(6i32);
pub const SW_NORMAL: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(1i32);
pub const SW_OTHERUNZOOM: SHOW_WINDOW_STATUS = SHOW_WINDOW_STATUS(4u32);
pub const SW_OTHERZOOM: SHOW_WINDOW_STATUS = SHOW_WINDOW_STATUS(2u32);
pub const SW_PARENTCLOSING: SHOW_WINDOW_STATUS = SHOW_WINDOW_STATUS(1u32);
pub const SW_PARENTOPENING: SHOW_WINDOW_STATUS = SHOW_WINDOW_STATUS(3u32);
pub const SW_RESTORE: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(9i32);
pub const SW_SCROLLCHILDREN: SCROLL_WINDOW_FLAGS = SCROLL_WINDOW_FLAGS(1u32);
pub const SW_SHOW: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(5i32);
pub const SW_SHOWDEFAULT: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(10i32);
pub const SW_SHOWMAXIMIZED: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(3i32);
pub const SW_SHOWMINIMIZED: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(2i32);
pub const SW_SHOWMINNOACTIVE: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(7i32);
pub const SW_SHOWNA: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(8i32);
pub const SW_SHOWNOACTIVATE: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(4i32);
pub const SW_SHOWNORMAL: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(1i32);
pub const SW_SMOOTHSCROLL: SCROLL_WINDOW_FLAGS = SCROLL_WINDOW_FLAGS(16u32);
pub const TDF_REGISTER: TOOLTIP_DISMISS_FLAGS = TOOLTIP_DISMISS_FLAGS(1i32);
pub const TDF_UNREGISTER: TOOLTIP_DISMISS_FLAGS = TOOLTIP_DISMISS_FLAGS(2i32);
pub const TIMERV_COALESCING_MAX: u32 = 2147483637u32;
pub const TIMERV_COALESCING_MIN: u32 = 1u32;
pub const TIMERV_DEFAULT_COALESCING: u32 = 0u32;
pub const TIMERV_NO_COALESCING: u32 = 4294967295u32;
pub const TKF_AVAILABLE: u32 = 2u32;
pub const TKF_CONFIRMHOTKEY: u32 = 8u32;
pub const TKF_HOTKEYACTIVE: u32 = 4u32;
pub const TKF_HOTKEYSOUND: u32 = 16u32;
pub const TKF_INDICATOR: u32 = 32u32;
pub const TKF_TOGGLEKEYSON: u32 = 1u32;
pub const TOUCHPREDICTIONPARAMETERS_DEFAULT_LATENCY: u32 = 8u32;
pub const TOUCHPREDICTIONPARAMETERS_DEFAULT_RLS_DELTA: f32 = 0.001f32;
pub const TOUCHPREDICTIONPARAMETERS_DEFAULT_RLS_EXPO_SMOOTH_ALPHA: f32 = 0.99f32;
pub const TOUCHPREDICTIONPARAMETERS_DEFAULT_RLS_LAMBDA_LEARNING_RATE: f32 = 0.001f32;
pub const TOUCHPREDICTIONPARAMETERS_DEFAULT_RLS_LAMBDA_MAX: f32 = 0.999f32;
pub const TOUCHPREDICTIONPARAMETERS_DEFAULT_RLS_LAMBDA_MIN: f32 = 0.9f32;
pub const TOUCHPREDICTIONPARAMETERS_DEFAULT_SAMPLETIME: u32 = 8u32;
pub const TOUCHPREDICTIONPARAMETERS_DEFAULT_USE_HW_TIMESTAMP: u32 = 1u32;
pub const TOUCH_FLAG_NONE: u32 = 0u32;
pub const TOUCH_HIT_TESTING_CLIENT: u32 = 1u32;
pub const TOUCH_HIT_TESTING_DEFAULT: u32 = 0u32;
pub const TOUCH_HIT_TESTING_NONE: u32 = 2u32;
pub const TOUCH_HIT_TESTING_PROXIMITY_CLOSEST: u32 = 0u32;
pub const TOUCH_HIT_TESTING_PROXIMITY_FARTHEST: u32 = 4095u32;
pub const TOUCH_MASK_CONTACTAREA: u32 = 1u32;
pub const TOUCH_MASK_NONE: u32 = 0u32;
pub const TOUCH_MASK_ORIENTATION: u32 = 2u32;
pub const TOUCH_MASK_PRESSURE: u32 = 4u32;
pub const TPM_BOTTOMALIGN: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(32u32);
pub const TPM_CENTERALIGN: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(4u32);
pub const TPM_HORIZONTAL: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(0u32);
pub const TPM_HORNEGANIMATION: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(2048u32);
pub const TPM_HORPOSANIMATION: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(1024u32);
pub const TPM_LAYOUTRTL: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(32768u32);
pub const TPM_LEFTALIGN: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(0u32);
pub const TPM_LEFTBUTTON: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(0u32);
pub const TPM_NOANIMATION: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(16384u32);
pub const TPM_NONOTIFY: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(128u32);
pub const TPM_RECURSE: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(1u32);
pub const TPM_RETURNCMD: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(256u32);
pub const TPM_RIGHTALIGN: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(8u32);
pub const TPM_RIGHTBUTTON: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(2u32);
pub const TPM_TOPALIGN: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(0u32);
pub const TPM_VCENTERALIGN: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(16u32);
pub const TPM_VERNEGANIMATION: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(8192u32);
pub const TPM_VERPOSANIMATION: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(4096u32);
pub const TPM_VERTICAL: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(64u32);
pub const TPM_WORKAREA: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(65536u32);
pub const UISF_ACTIVE: u32 = 4u32;
pub const UISF_HIDEACCEL: u32 = 2u32;
pub const UISF_HIDEFOCUS: u32 = 1u32;
pub const UIS_CLEAR: u32 = 2u32;
pub const UIS_INITIALIZE: u32 = 3u32;
pub const UIS_SET: u32 = 1u32;
pub const ULW_ALPHA: UPDATE_LAYERED_WINDOW_FLAGS = UPDATE_LAYERED_WINDOW_FLAGS(2u32);
pub const ULW_COLORKEY: UPDATE_LAYERED_WINDOW_FLAGS = UPDATE_LAYERED_WINDOW_FLAGS(1u32);
pub const ULW_EX_NORESIZE: UPDATE_LAYERED_WINDOW_FLAGS = UPDATE_LAYERED_WINDOW_FLAGS(8u32);
pub const ULW_OPAQUE: UPDATE_LAYERED_WINDOW_FLAGS = UPDATE_LAYERED_WINDOW_FLAGS(4u32);
pub const UNICODE_NOCHAR: u32 = 65535u32;
pub const UOI_TIMERPROC_EXCEPTION_SUPPRESSION: u32 = 7u32;
pub const USER_DEFAULT_SCREEN_DPI: u32 = 96u32;
pub const USER_TIMER_MAXIMUM: u32 = 2147483647u32;
pub const USER_TIMER_MINIMUM: u32 = 10u32;
pub const WA_ACTIVE: u32 = 1u32;
pub const WA_CLICKACTIVE: u32 = 2u32;
pub const WA_INACTIVE: u32 = 0u32;
pub const WDA_EXCLUDEFROMCAPTURE: WINDOW_DISPLAY_AFFINITY = WINDOW_DISPLAY_AFFINITY(17u32);
pub const WDA_MONITOR: WINDOW_DISPLAY_AFFINITY = WINDOW_DISPLAY_AFFINITY(1u32);
pub const WDA_NONE: WINDOW_DISPLAY_AFFINITY = WINDOW_DISPLAY_AFFINITY(0u32);
pub const WHEEL_DELTA: u32 = 120u32;
pub const WH_CALLWNDPROC: WINDOWS_HOOK_ID = WINDOWS_HOOK_ID(4i32);
pub const WH_CALLWNDPROCRET: WINDOWS_HOOK_ID = WINDOWS_HOOK_ID(12i32);
pub const WH_CBT: WINDOWS_HOOK_ID = WINDOWS_HOOK_ID(5i32);
pub const WH_DEBUG: WINDOWS_HOOK_ID = WINDOWS_HOOK_ID(9i32);
pub const WH_FOREGROUNDIDLE: WINDOWS_HOOK_ID = WINDOWS_HOOK_ID(11i32);
pub const WH_GETMESSAGE: WINDOWS_HOOK_ID = WINDOWS_HOOK_ID(3i32);
pub const WH_HARDWARE: u32 = 8u32;
pub const WH_JOURNALPLAYBACK: WINDOWS_HOOK_ID = WINDOWS_HOOK_ID(1i32);
pub const WH_JOURNALRECORD: WINDOWS_HOOK_ID = WINDOWS_HOOK_ID(0i32);
pub const WH_KEYBOARD: WINDOWS_HOOK_ID = WINDOWS_HOOK_ID(2i32);
pub const WH_KEYBOARD_LL: WINDOWS_HOOK_ID = WINDOWS_HOOK_ID(13i32);
pub const WH_MAX: u32 = 14u32;
pub const WH_MAXHOOK: u32 = 14u32;
pub const WH_MIN: i32 = -1i32;
pub const WH_MINHOOK: i32 = -1i32;
pub const WH_MOUSE: WINDOWS_HOOK_ID = WINDOWS_HOOK_ID(7i32);
pub const WH_MOUSE_LL: WINDOWS_HOOK_ID = WINDOWS_HOOK_ID(14i32);
pub const WH_MSGFILTER: WINDOWS_HOOK_ID = WINDOWS_HOOK_ID(-1i32);
pub const WH_SHELL: WINDOWS_HOOK_ID = WINDOWS_HOOK_ID(10i32);
pub const WH_SYSMSGFILTER: WINDOWS_HOOK_ID = WINDOWS_HOOK_ID(6i32);
pub const WINEVENT_INCONTEXT: u32 = 4u32;
pub const WINEVENT_OUTOFCONTEXT: u32 = 0u32;
pub const WINEVENT_SKIPOWNPROCESS: u32 = 2u32;
pub const WINEVENT_SKIPOWNTHREAD: u32 = 1u32;
pub const WINSTA_ACCESSCLIPBOARD: i32 = 4i32;
pub const WINSTA_ACCESSGLOBALATOMS: i32 = 32i32;
pub const WINSTA_CREATEDESKTOP: i32 = 8i32;
pub const WINSTA_ENUMDESKTOPS: i32 = 1i32;
pub const WINSTA_ENUMERATE: i32 = 256i32;
pub const WINSTA_EXITWINDOWS: i32 = 64i32;
pub const WINSTA_READATTRIBUTES: i32 = 2i32;
pub const WINSTA_READSCREEN: i32 = 512i32;
pub const WINSTA_WRITEATTRIBUTES: i32 = 16i32;
pub const WMSZ_BOTTOM: u32 = 6u32;
pub const WMSZ_BOTTOMLEFT: u32 = 7u32;
pub const WMSZ_BOTTOMRIGHT: u32 = 8u32;
pub const WMSZ_LEFT: u32 = 1u32;
pub const WMSZ_RIGHT: u32 = 2u32;
pub const WMSZ_TOP: u32 = 3u32;
pub const WMSZ_TOPLEFT: u32 = 4u32;
pub const WMSZ_TOPRIGHT: u32 = 5u32;
pub const WM_ACTIVATE: u32 = 6u32;
pub const WM_ACTIVATEAPP: u32 = 28u32;
pub const WM_AFXFIRST: u32 = 864u32;
pub const WM_AFXLAST: u32 = 895u32;
pub const WM_APP: u32 = 32768u32;
pub const WM_APPCOMMAND: u32 = 793u32;
pub const WM_ASKCBFORMATNAME: u32 = 780u32;
pub const WM_CANCELJOURNAL: u32 = 75u32;
pub const WM_CANCELMODE: u32 = 31u32;
pub const WM_CAPTURECHANGED: u32 = 533u32;
pub const WM_CHANGECBCHAIN: u32 = 781u32;
pub const WM_CHANGEUISTATE: u32 = 295u32;
pub const WM_CHAR: u32 = 258u32;
pub const WM_CHARTOITEM: u32 = 47u32;
pub const WM_CHILDACTIVATE: u32 = 34u32;
pub const WM_CLEAR: u32 = 771u32;
pub const WM_CLIPBOARDUPDATE: u32 = 797u32;
pub const WM_CLOSE: u32 = 16u32;
pub const WM_COMMAND: u32 = 273u32;
pub const WM_COMMNOTIFY: u32 = 68u32;
pub const WM_COMPACTING: u32 = 65u32;
pub const WM_COMPAREITEM: u32 = 57u32;
pub const WM_CONTEXTMENU: u32 = 123u32;
pub const WM_COPY: u32 = 769u32;
pub const WM_COPYDATA: u32 = 74u32;
pub const WM_CREATE: u32 = 1u32;
pub const WM_CTLCOLORBTN: u32 = 309u32;
pub const WM_CTLCOLORDLG: u32 = 310u32;
pub const WM_CTLCOLOREDIT: u32 = 307u32;
pub const WM_CTLCOLORLISTBOX: u32 = 308u32;
pub const WM_CTLCOLORMSGBOX: u32 = 306u32;
pub const WM_CTLCOLORSCROLLBAR: u32 = 311u32;
pub const WM_CTLCOLORSTATIC: u32 = 312u32;
pub const WM_CUT: u32 = 768u32;
pub const WM_DEADCHAR: u32 = 259u32;
pub const WM_DELETEITEM: u32 = 45u32;
pub const WM_DESTROY: u32 = 2u32;
pub const WM_DESTROYCLIPBOARD: u32 = 775u32;
pub const WM_DEVICECHANGE: u32 = 537u32;
pub const WM_DEVMODECHANGE: u32 = 27u32;
pub const WM_DISPLAYCHANGE: u32 = 126u32;
pub const WM_DPICHANGED: u32 = 736u32;
pub const WM_DPICHANGED_AFTERPARENT: u32 = 739u32;
pub const WM_DPICHANGED_BEFOREPARENT: u32 = 738u32;
pub const WM_DRAWCLIPBOARD: u32 = 776u32;
pub const WM_DRAWITEM: u32 = 43u32;
pub const WM_DROPFILES: u32 = 563u32;
pub const WM_DWMCOLORIZATIONCOLORCHANGED: u32 = 800u32;
pub const WM_DWMCOMPOSITIONCHANGED: u32 = 798u32;
pub const WM_DWMNCRENDERINGCHANGED: u32 = 799u32;
pub const WM_DWMSENDICONICLIVEPREVIEWBITMAP: u32 = 806u32;
pub const WM_DWMSENDICONICTHUMBNAIL: u32 = 803u32;
pub const WM_DWMWINDOWMAXIMIZEDCHANGE: u32 = 801u32;
pub const WM_ENABLE: u32 = 10u32;
pub const WM_ENDSESSION: u32 = 22u32;
pub const WM_ENTERIDLE: u32 = 289u32;
pub const WM_ENTERMENULOOP: u32 = 529u32;
pub const WM_ENTERSIZEMOVE: u32 = 561u32;
pub const WM_ERASEBKGND: u32 = 20u32;
pub const WM_EXITMENULOOP: u32 = 530u32;
pub const WM_EXITSIZEMOVE: u32 = 562u32;
pub const WM_FONTCHANGE: u32 = 29u32;
pub const WM_GESTURE: u32 = 281u32;
pub const WM_GESTURENOTIFY: u32 = 282u32;
pub const WM_GETDLGCODE: u32 = 135u32;
pub const WM_GETDPISCALEDSIZE: u32 = 740u32;
pub const WM_GETFONT: u32 = 49u32;
pub const WM_GETHOTKEY: u32 = 51u32;
pub const WM_GETICON: u32 = 127u32;
pub const WM_GETMINMAXINFO: u32 = 36u32;
pub const WM_GETOBJECT: u32 = 61u32;
pub const WM_GETTEXT: u32 = 13u32;
pub const WM_GETTEXTLENGTH: u32 = 14u32;
pub const WM_GETTITLEBARINFOEX: u32 = 831u32;
pub const WM_HANDHELDFIRST: u32 = 856u32;
pub const WM_HANDHELDLAST: u32 = 863u32;
pub const WM_HELP: u32 = 83u32;
pub const WM_HOTKEY: u32 = 786u32;
pub const WM_HSCROLL: u32 = 276u32;
pub const WM_HSCROLLCLIPBOARD: u32 = 782u32;
pub const WM_ICONERASEBKGND: u32 = 39u32;
pub const WM_IME_CHAR: u32 = 646u32;
pub const WM_IME_COMPOSITION: u32 = 271u32;
pub const WM_IME_COMPOSITIONFULL: u32 = 644u32;
pub const WM_IME_CONTROL: u32 = 643u32;
pub const WM_IME_ENDCOMPOSITION: u32 = 270u32;
pub const WM_IME_KEYDOWN: u32 = 656u32;
pub const WM_IME_KEYLAST: u32 = 271u32;
pub const WM_IME_KEYUP: u32 = 657u32;
pub const WM_IME_NOTIFY: u32 = 642u32;
pub const WM_IME_REQUEST: u32 = 648u32;
pub const WM_IME_SELECT: u32 = 645u32;
pub const WM_IME_SETCONTEXT: u32 = 641u32;
pub const WM_IME_STARTCOMPOSITION: u32 = 269u32;
pub const WM_INITDIALOG: u32 = 272u32;
pub const WM_INITMENU: u32 = 278u32;
pub const WM_INITMENUPOPUP: u32 = 279u32;
pub const WM_INPUT: u32 = 255u32;
pub const WM_INPUTLANGCHANGE: u32 = 81u32;
pub const WM_INPUTLANGCHANGEREQUEST: u32 = 80u32;
pub const WM_INPUT_DEVICE_CHANGE: u32 = 254u32;
pub const WM_KEYDOWN: u32 = 256u32;
pub const WM_KEYFIRST: u32 = 256u32;
pub const WM_KEYLAST: u32 = 265u32;
pub const WM_KEYUP: u32 = 257u32;
pub const WM_KILLFOCUS: u32 = 8u32;
pub const WM_LBUTTONDBLCLK: u32 = 515u32;
pub const WM_LBUTTONDOWN: u32 = 513u32;
pub const WM_LBUTTONUP: u32 = 514u32;
pub const WM_MBUTTONDBLCLK: u32 = 521u32;
pub const WM_MBUTTONDOWN: u32 = 519u32;
pub const WM_MBUTTONUP: u32 = 520u32;
pub const WM_MDIACTIVATE: u32 = 546u32;
pub const WM_MDICASCADE: u32 = 551u32;
pub const WM_MDICREATE: u32 = 544u32;
pub const WM_MDIDESTROY: u32 = 545u32;
pub const WM_MDIGETACTIVE: u32 = 553u32;
pub const WM_MDIICONARRANGE: u32 = 552u32;
pub const WM_MDIMAXIMIZE: u32 = 549u32;
pub const WM_MDINEXT: u32 = 548u32;
pub const WM_MDIREFRESHMENU: u32 = 564u32;
pub const WM_MDIRESTORE: u32 = 547u32;
pub const WM_MDISETMENU: u32 = 560u32;
pub const WM_MDITILE: u32 = 550u32;
pub const WM_MEASUREITEM: u32 = 44u32;
pub const WM_MENUCHAR: u32 = 288u32;
pub const WM_MENUCOMMAND: u32 = 294u32;
pub const WM_MENUDRAG: u32 = 291u32;
pub const WM_MENUGETOBJECT: u32 = 292u32;
pub const WM_MENURBUTTONUP: u32 = 290u32;
pub const WM_MENUSELECT: u32 = 287u32;
pub const WM_MOUSEACTIVATE: u32 = 33u32;
pub const WM_MOUSEFIRST: u32 = 512u32;
pub const WM_MOUSEHWHEEL: u32 = 526u32;
pub const WM_MOUSELAST: u32 = 526u32;
pub const WM_MOUSEMOVE: u32 = 512u32;
pub const WM_MOUSEWHEEL: u32 = 522u32;
pub const WM_MOVE: u32 = 3u32;
pub const WM_MOVING: u32 = 534u32;
pub const WM_NCACTIVATE: u32 = 134u32;
pub const WM_NCCALCSIZE: u32 = 131u32;
pub const WM_NCCREATE: u32 = 129u32;
pub const WM_NCDESTROY: u32 = 130u32;
pub const WM_NCHITTEST: u32 = 132u32;
pub const WM_NCLBUTTONDBLCLK: u32 = 163u32;
pub const WM_NCLBUTTONDOWN: u32 = 161u32;
pub const WM_NCLBUTTONUP: u32 = 162u32;
pub const WM_NCMBUTTONDBLCLK: u32 = 169u32;
pub const WM_NCMBUTTONDOWN: u32 = 167u32;
pub const WM_NCMBUTTONUP: u32 = 168u32;
pub const WM_NCMOUSEHOVER: u32 = 672u32;
pub const WM_NCMOUSELEAVE: u32 = 674u32;
pub const WM_NCMOUSEMOVE: u32 = 160u32;
pub const WM_NCPAINT: u32 = 133u32;
pub const WM_NCPOINTERDOWN: u32 = 578u32;
pub const WM_NCPOINTERUP: u32 = 579u32;
pub const WM_NCPOINTERUPDATE: u32 = 577u32;
pub const WM_NCRBUTTONDBLCLK: u32 = 166u32;
pub const WM_NCRBUTTONDOWN: u32 = 164u32;
pub const WM_NCRBUTTONUP: u32 = 165u32;
pub const WM_NCXBUTTONDBLCLK: u32 = 173u32;
pub const WM_NCXBUTTONDOWN: u32 = 171u32;
pub const WM_NCXBUTTONUP: u32 = 172u32;
pub const WM_NEXTDLGCTL: u32 = 40u32;
pub const WM_NEXTMENU: u32 = 531u32;
pub const WM_NOTIFY: u32 = 78u32;
pub const WM_NOTIFYFORMAT: u32 = 85u32;
pub const WM_NULL: u32 = 0u32;
pub const WM_PAINT: u32 = 15u32;
pub const WM_PAINTCLIPBOARD: u32 = 777u32;
pub const WM_PAINTICON: u32 = 38u32;
pub const WM_PALETTECHANGED: u32 = 785u32;
pub const WM_PALETTEISCHANGING: u32 = 784u32;
pub const WM_PARENTNOTIFY: u32 = 528u32;
pub const WM_PASTE: u32 = 770u32;
pub const WM_PENWINFIRST: u32 = 896u32;
pub const WM_PENWINLAST: u32 = 911u32;
pub const WM_POINTERACTIVATE: u32 = 587u32;
pub const WM_POINTERCAPTURECHANGED: u32 = 588u32;
pub const WM_POINTERDEVICECHANGE: u32 = 568u32;
pub const WM_POINTERDEVICEINRANGE: u32 = 569u32;
pub const WM_POINTERDEVICEOUTOFRANGE: u32 = 570u32;
pub const WM_POINTERDOWN: u32 = 582u32;
pub const WM_POINTERENTER: u32 = 585u32;
pub const WM_POINTERHWHEEL: u32 = 591u32;
pub const WM_POINTERLEAVE: u32 = 586u32;
pub const WM_POINTERROUTEDAWAY: u32 = 594u32;
pub const WM_POINTERROUTEDRELEASED: u32 = 595u32;
pub const WM_POINTERROUTEDTO: u32 = 593u32;
pub const WM_POINTERUP: u32 = 583u32;
pub const WM_POINTERUPDATE: u32 = 581u32;
pub const WM_POINTERWHEEL: u32 = 590u32;
pub const WM_POWER: u32 = 72u32;
pub const WM_POWERBROADCAST: u32 = 536u32;
pub const WM_PRINT: u32 = 791u32;
pub const WM_PRINTCLIENT: u32 = 792u32;
pub const WM_QUERYDRAGICON: u32 = 55u32;
pub const WM_QUERYENDSESSION: u32 = 17u32;
pub const WM_QUERYNEWPALETTE: u32 = 783u32;
pub const WM_QUERYOPEN: u32 = 19u32;
pub const WM_QUERYUISTATE: u32 = 297u32;
pub const WM_QUEUESYNC: u32 = 35u32;
pub const WM_QUIT: u32 = 18u32;
pub const WM_RBUTTONDBLCLK: u32 = 518u32;
pub const WM_RBUTTONDOWN: u32 = 516u32;
pub const WM_RBUTTONUP: u32 = 517u32;
pub const WM_RENDERALLFORMATS: u32 = 774u32;
pub const WM_RENDERFORMAT: u32 = 773u32;
pub const WM_SETCURSOR: u32 = 32u32;
pub const WM_SETFOCUS: u32 = 7u32;
pub const WM_SETFONT: u32 = 48u32;
pub const WM_SETHOTKEY: u32 = 50u32;
pub const WM_SETICON: u32 = 128u32;
pub const WM_SETREDRAW: u32 = 11u32;
pub const WM_SETTEXT: u32 = 12u32;
pub const WM_SETTINGCHANGE: u32 = 26u32;
pub const WM_SHOWWINDOW: u32 = 24u32;
pub const WM_SIZE: u32 = 5u32;
pub const WM_SIZECLIPBOARD: u32 = 779u32;
pub const WM_SIZING: u32 = 532u32;
pub const WM_SPOOLERSTATUS: u32 = 42u32;
pub const WM_STYLECHANGED: u32 = 125u32;
pub const WM_STYLECHANGING: u32 = 124u32;
pub const WM_SYNCPAINT: u32 = 136u32;
pub const WM_SYSCHAR: u32 = 262u32;
pub const WM_SYSCOLORCHANGE: u32 = 21u32;
pub const WM_SYSCOMMAND: u32 = 274u32;
pub const WM_SYSDEADCHAR: u32 = 263u32;
pub const WM_SYSKEYDOWN: u32 = 260u32;
pub const WM_SYSKEYUP: u32 = 261u32;
pub const WM_TABLET_FIRST: u32 = 704u32;
pub const WM_TABLET_LAST: u32 = 735u32;
pub const WM_TCARD: u32 = 82u32;
pub const WM_THEMECHANGED: u32 = 794u32;
pub const WM_TIMECHANGE: u32 = 30u32;
pub const WM_TIMER: u32 = 275u32;
pub const WM_TOOLTIPDISMISS: u32 = 837u32;
pub const WM_TOUCH: u32 = 576u32;
pub const WM_TOUCHHITTESTING: u32 = 589u32;
pub const WM_UNDO: u32 = 772u32;
pub const WM_UNICHAR: u32 = 265u32;
pub const WM_UNINITMENUPOPUP: u32 = 293u32;
pub const WM_UPDATEUISTATE: u32 = 296u32;
pub const WM_USER: u32 = 1024u32;
pub const WM_USERCHANGED: u32 = 84u32;
pub const WM_VKEYTOITEM: u32 = 46u32;
pub const WM_VSCROLL: u32 = 277u32;
pub const WM_VSCROLLCLIPBOARD: u32 = 778u32;
pub const WM_WINDOWPOSCHANGED: u32 = 71u32;
pub const WM_WINDOWPOSCHANGING: u32 = 70u32;
pub const WM_WININICHANGE: u32 = 26u32;
pub const WM_WTSSESSION_CHANGE: u32 = 689u32;
pub const WM_XBUTTONDBLCLK: u32 = 525u32;
pub const WM_XBUTTONDOWN: u32 = 523u32;
pub const WM_XBUTTONUP: u32 = 524u32;
pub const WPF_ASYNCWINDOWPLACEMENT: WINDOWPLACEMENT_FLAGS = WINDOWPLACEMENT_FLAGS(4u32);
pub const WPF_RESTORETOMAXIMIZED: WINDOWPLACEMENT_FLAGS = WINDOWPLACEMENT_FLAGS(2u32);
pub const WPF_SETMINPOSITION: WINDOWPLACEMENT_FLAGS = WINDOWPLACEMENT_FLAGS(1u32);
pub const WSF_VISIBLE: i32 = 1i32;
pub const WS_ACTIVECAPTION: WINDOW_STYLE = WINDOW_STYLE(1u32);
pub const WS_BORDER: WINDOW_STYLE = WINDOW_STYLE(8388608u32);
pub const WS_CAPTION: WINDOW_STYLE = WINDOW_STYLE(12582912u32);
pub const WS_CHILD: WINDOW_STYLE = WINDOW_STYLE(1073741824u32);
pub const WS_CHILDWINDOW: WINDOW_STYLE = WINDOW_STYLE(1073741824u32);
pub const WS_CLIPCHILDREN: WINDOW_STYLE = WINDOW_STYLE(33554432u32);
pub const WS_CLIPSIBLINGS: WINDOW_STYLE = WINDOW_STYLE(67108864u32);
pub const WS_DISABLED: WINDOW_STYLE = WINDOW_STYLE(134217728u32);
pub const WS_DLGFRAME: WINDOW_STYLE = WINDOW_STYLE(4194304u32);
pub const WS_EX_ACCEPTFILES: WINDOW_EX_STYLE = WINDOW_EX_STYLE(16u32);
pub const WS_EX_APPWINDOW: WINDOW_EX_STYLE = WINDOW_EX_STYLE(262144u32);
pub const WS_EX_CLIENTEDGE: WINDOW_EX_STYLE = WINDOW_EX_STYLE(512u32);
pub const WS_EX_COMPOSITED: WINDOW_EX_STYLE = WINDOW_EX_STYLE(33554432u32);
pub const WS_EX_CONTEXTHELP: WINDOW_EX_STYLE = WINDOW_EX_STYLE(1024u32);
pub const WS_EX_CONTROLPARENT: WINDOW_EX_STYLE = WINDOW_EX_STYLE(65536u32);
pub const WS_EX_DLGMODALFRAME: WINDOW_EX_STYLE = WINDOW_EX_STYLE(1u32);
pub const WS_EX_LAYERED: WINDOW_EX_STYLE = WINDOW_EX_STYLE(524288u32);
pub const WS_EX_LAYOUTRTL: WINDOW_EX_STYLE = WINDOW_EX_STYLE(4194304u32);
pub const WS_EX_LEFT: WINDOW_EX_STYLE = WINDOW_EX_STYLE(0u32);
pub const WS_EX_LEFTSCROLLBAR: WINDOW_EX_STYLE = WINDOW_EX_STYLE(16384u32);
pub const WS_EX_LTRREADING: WINDOW_EX_STYLE = WINDOW_EX_STYLE(0u32);
pub const WS_EX_MDICHILD: WINDOW_EX_STYLE = WINDOW_EX_STYLE(64u32);
pub const WS_EX_NOACTIVATE: WINDOW_EX_STYLE = WINDOW_EX_STYLE(134217728u32);
pub const WS_EX_NOINHERITLAYOUT: WINDOW_EX_STYLE = WINDOW_EX_STYLE(1048576u32);
pub const WS_EX_NOPARENTNOTIFY: WINDOW_EX_STYLE = WINDOW_EX_STYLE(4u32);
pub const WS_EX_NOREDIRECTIONBITMAP: WINDOW_EX_STYLE = WINDOW_EX_STYLE(2097152u32);
pub const WS_EX_OVERLAPPEDWINDOW: WINDOW_EX_STYLE = WINDOW_EX_STYLE(768u32);
pub const WS_EX_PALETTEWINDOW: WINDOW_EX_STYLE = WINDOW_EX_STYLE(392u32);
pub const WS_EX_RIGHT: WINDOW_EX_STYLE = WINDOW_EX_STYLE(4096u32);
pub const WS_EX_RIGHTSCROLLBAR: WINDOW_EX_STYLE = WINDOW_EX_STYLE(0u32);
pub const WS_EX_RTLREADING: WINDOW_EX_STYLE = WINDOW_EX_STYLE(8192u32);
pub const WS_EX_STATICEDGE: WINDOW_EX_STYLE = WINDOW_EX_STYLE(131072u32);
pub const WS_EX_TOOLWINDOW: WINDOW_EX_STYLE = WINDOW_EX_STYLE(128u32);
pub const WS_EX_TOPMOST: WINDOW_EX_STYLE = WINDOW_EX_STYLE(8u32);
pub const WS_EX_TRANSPARENT: WINDOW_EX_STYLE = WINDOW_EX_STYLE(32u32);
pub const WS_EX_WINDOWEDGE: WINDOW_EX_STYLE = WINDOW_EX_STYLE(256u32);
pub const WS_GROUP: WINDOW_STYLE = WINDOW_STYLE(131072u32);
pub const WS_HSCROLL: WINDOW_STYLE = WINDOW_STYLE(1048576u32);
pub const WS_ICONIC: WINDOW_STYLE = WINDOW_STYLE(536870912u32);
pub const WS_MAXIMIZE: WINDOW_STYLE = WINDOW_STYLE(16777216u32);
pub const WS_MAXIMIZEBOX: WINDOW_STYLE = WINDOW_STYLE(65536u32);
pub const WS_MINIMIZE: WINDOW_STYLE = WINDOW_STYLE(536870912u32);
pub const WS_MINIMIZEBOX: WINDOW_STYLE = WINDOW_STYLE(131072u32);
pub const WS_OVERLAPPED: WINDOW_STYLE = WINDOW_STYLE(0u32);
pub const WS_OVERLAPPEDWINDOW: WINDOW_STYLE = WINDOW_STYLE(13565952u32);
pub const WS_POPUP: WINDOW_STYLE = WINDOW_STYLE(2147483648u32);
pub const WS_POPUPWINDOW: WINDOW_STYLE = WINDOW_STYLE(2156396544u32);
pub const WS_SIZEBOX: WINDOW_STYLE = WINDOW_STYLE(262144u32);
pub const WS_SYSMENU: WINDOW_STYLE = WINDOW_STYLE(524288u32);
pub const WS_TABSTOP: WINDOW_STYLE = WINDOW_STYLE(65536u32);
pub const WS_THICKFRAME: WINDOW_STYLE = WINDOW_STYLE(262144u32);
pub const WS_TILED: WINDOW_STYLE = WINDOW_STYLE(0u32);
pub const WS_TILEDWINDOW: WINDOW_STYLE = WINDOW_STYLE(13565952u32);
pub const WS_VISIBLE: WINDOW_STYLE = WINDOW_STYLE(268435456u32);
pub const WS_VSCROLL: WINDOW_STYLE = WINDOW_STYLE(2097152u32);
pub const WTS_CONSOLE_CONNECT: u32 = 1u32;
pub const WTS_CONSOLE_DISCONNECT: u32 = 2u32;
pub const WTS_REMOTE_CONNECT: u32 = 3u32;
pub const WTS_REMOTE_DISCONNECT: u32 = 4u32;
pub const WTS_SESSION_CREATE: u32 = 10u32;
pub const WTS_SESSION_LOCK: u32 = 7u32;
pub const WTS_SESSION_LOGOFF: u32 = 6u32;
pub const WTS_SESSION_LOGON: u32 = 5u32;
pub const WTS_SESSION_REMOTE_CONTROL: u32 = 9u32;
pub const WTS_SESSION_TERMINATE: u32 = 11u32;
pub const WTS_SESSION_UNLOCK: u32 = 8u32;
pub const WVR_ALIGNBOTTOM: u32 = 64u32;
pub const WVR_ALIGNLEFT: u32 = 32u32;
pub const WVR_ALIGNRIGHT: u32 = 128u32;
pub const WVR_ALIGNTOP: u32 = 16u32;
pub const WVR_HREDRAW: u32 = 256u32;
pub const WVR_VALIDRECTS: u32 = 1024u32;
pub const WVR_VREDRAW: u32 = 512u32;
pub const XBUTTON1: u16 = 1u16;
pub const XBUTTON2: u16 = 2u16;
pub const __WARNING_BANNED_API_USAGE: u32 = 28719u32;
pub const __WARNING_CYCLOMATIC_COMPLEXITY: u32 = 28734u32;
pub const __WARNING_DEREF_NULL_PTR: u32 = 6011u32;
pub const __WARNING_HIGH_PRIORITY_OVERFLOW_POSTCONDITION: u32 = 26045u32;
pub const __WARNING_INCORRECT_ANNOTATION: u32 = 26007u32;
pub const __WARNING_INVALID_PARAM_VALUE_1: u32 = 6387u32;
pub const __WARNING_INVALID_PARAM_VALUE_3: u32 = 28183u32;
pub const __WARNING_MISSING_ZERO_TERMINATION2: u32 = 6054u32;
pub const __WARNING_POSTCONDITION_NULLTERMINATION_VIOLATION: u32 = 26036u32;
pub const __WARNING_POST_EXPECTED: u32 = 28210u32;
pub const __WARNING_POTENTIAL_BUFFER_OVERFLOW_HIGH_PRIORITY: u32 = 26015u32;
pub const __WARNING_POTENTIAL_RANGE_POSTCONDITION_VIOLATION: u32 = 26071u32;
pub const __WARNING_PRECONDITION_NULLTERMINATION_VIOLATION: u32 = 26035u32;
pub const __WARNING_RANGE_POSTCONDITION_VIOLATION: u32 = 26061u32;
pub const __WARNING_RETURNING_BAD_RESULT: u32 = 28196u32;
pub const __WARNING_RETURN_UNINIT_VAR: u32 = 6101u32;
pub const __WARNING_USING_UNINIT_VAR: u32 = 6001u32;
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ACCEL_VIRT_FLAGS(pub u8);
impl windows_core::TypeKind for ACCEL_VIRT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ACCEL_VIRT_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ACCEL_VIRT_FLAGS").field(&self.0).finish()
    }
}
impl ACCEL_VIRT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for ACCEL_VIRT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for ACCEL_VIRT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for ACCEL_VIRT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for ACCEL_VIRT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for ACCEL_VIRT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ANIMATE_WINDOW_FLAGS(pub u32);
impl windows_core::TypeKind for ANIMATE_WINDOW_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ANIMATE_WINDOW_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ANIMATE_WINDOW_FLAGS").field(&self.0).finish()
    }
}
impl ANIMATE_WINDOW_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for ANIMATE_WINDOW_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for ANIMATE_WINDOW_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for ANIMATE_WINDOW_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for ANIMATE_WINDOW_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for ANIMATE_WINDOW_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CASCADE_WINDOWS_HOW(pub u32);
impl windows_core::TypeKind for CASCADE_WINDOWS_HOW {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CASCADE_WINDOWS_HOW {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CASCADE_WINDOWS_HOW").field(&self.0).finish()
    }
}
impl CASCADE_WINDOWS_HOW {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CASCADE_WINDOWS_HOW {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CASCADE_WINDOWS_HOW {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CASCADE_WINDOWS_HOW {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CASCADE_WINDOWS_HOW {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CASCADE_WINDOWS_HOW {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CHANGE_WINDOW_MESSAGE_FILTER_FLAGS(pub u32);
impl windows_core::TypeKind for CHANGE_WINDOW_MESSAGE_FILTER_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CHANGE_WINDOW_MESSAGE_FILTER_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CHANGE_WINDOW_MESSAGE_FILTER_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CURSORINFO_FLAGS(pub u32);
impl windows_core::TypeKind for CURSORINFO_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CURSORINFO_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CURSORINFO_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CWP_FLAGS(pub u32);
impl windows_core::TypeKind for CWP_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CWP_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CWP_FLAGS").field(&self.0).finish()
    }
}
impl CWP_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CWP_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CWP_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CWP_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CWP_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CWP_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DEV_BROADCAST_HDR_DEVICE_TYPE(pub u32);
impl windows_core::TypeKind for DEV_BROADCAST_HDR_DEVICE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DEV_BROADCAST_HDR_DEVICE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DEV_BROADCAST_HDR_DEVICE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DEV_BROADCAST_VOLUME_FLAGS(pub u16);
impl windows_core::TypeKind for DEV_BROADCAST_VOLUME_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DEV_BROADCAST_VOLUME_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DEV_BROADCAST_VOLUME_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DI_FLAGS(pub u32);
impl windows_core::TypeKind for DI_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DI_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DI_FLAGS").field(&self.0).finish()
    }
}
impl DI_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DI_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DI_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DI_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DI_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DI_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct EDIT_CONTROL_FEATURE(pub i32);
impl windows_core::TypeKind for EDIT_CONTROL_FEATURE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for EDIT_CONTROL_FEATURE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EDIT_CONTROL_FEATURE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FLASHWINFO_FLAGS(pub u32);
impl windows_core::TypeKind for FLASHWINFO_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FLASHWINFO_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FLASHWINFO_FLAGS").field(&self.0).finish()
    }
}
impl FLASHWINFO_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for FLASHWINFO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for FLASHWINFO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for FLASHWINFO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for FLASHWINFO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for FLASHWINFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FOREGROUND_WINDOW_LOCK_CODE(pub u32);
impl windows_core::TypeKind for FOREGROUND_WINDOW_LOCK_CODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FOREGROUND_WINDOW_LOCK_CODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FOREGROUND_WINDOW_LOCK_CODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GDI_IMAGE_TYPE(pub u32);
impl windows_core::TypeKind for GDI_IMAGE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GDI_IMAGE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GDI_IMAGE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GET_ANCESTOR_FLAGS(pub u32);
impl windows_core::TypeKind for GET_ANCESTOR_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GET_ANCESTOR_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GET_ANCESTOR_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GET_CLASS_LONG_INDEX(pub i32);
impl windows_core::TypeKind for GET_CLASS_LONG_INDEX {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GET_CLASS_LONG_INDEX {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GET_CLASS_LONG_INDEX").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GET_MENU_DEFAULT_ITEM_FLAGS(pub u32);
impl windows_core::TypeKind for GET_MENU_DEFAULT_ITEM_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GET_MENU_DEFAULT_ITEM_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GET_MENU_DEFAULT_ITEM_FLAGS").field(&self.0).finish()
    }
}
impl GET_MENU_DEFAULT_ITEM_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for GET_MENU_DEFAULT_ITEM_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for GET_MENU_DEFAULT_ITEM_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for GET_MENU_DEFAULT_ITEM_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for GET_MENU_DEFAULT_ITEM_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for GET_MENU_DEFAULT_ITEM_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GET_WINDOW_CMD(pub u32);
impl windows_core::TypeKind for GET_WINDOW_CMD {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GET_WINDOW_CMD {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GET_WINDOW_CMD").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GUITHREADINFO_FLAGS(pub u32);
impl windows_core::TypeKind for GUITHREADINFO_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GUITHREADINFO_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GUITHREADINFO_FLAGS").field(&self.0).finish()
    }
}
impl GUITHREADINFO_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for GUITHREADINFO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for GUITHREADINFO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for GUITHREADINFO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for GUITHREADINFO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for GUITHREADINFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HANDEDNESS(pub i32);
impl windows_core::TypeKind for HANDEDNESS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HANDEDNESS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HANDEDNESS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IMAGE_FLAGS(pub u32);
impl windows_core::TypeKind for IMAGE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for IMAGE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IMAGE_FLAGS").field(&self.0).finish()
    }
}
impl IMAGE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for IMAGE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for IMAGE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for IMAGE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for IMAGE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for IMAGE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct KBDLLHOOKSTRUCT_FLAGS(pub u32);
impl windows_core::TypeKind for KBDLLHOOKSTRUCT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for KBDLLHOOKSTRUCT_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("KBDLLHOOKSTRUCT_FLAGS").field(&self.0).finish()
    }
}
impl KBDLLHOOKSTRUCT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for KBDLLHOOKSTRUCT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for KBDLLHOOKSTRUCT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for KBDLLHOOKSTRUCT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for KBDLLHOOKSTRUCT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for KBDLLHOOKSTRUCT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct LAYERED_WINDOW_ATTRIBUTES_FLAGS(pub u32);
impl windows_core::TypeKind for LAYERED_WINDOW_ATTRIBUTES_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for LAYERED_WINDOW_ATTRIBUTES_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("LAYERED_WINDOW_ATTRIBUTES_FLAGS").field(&self.0).finish()
    }
}
impl LAYERED_WINDOW_ATTRIBUTES_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for LAYERED_WINDOW_ATTRIBUTES_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for LAYERED_WINDOW_ATTRIBUTES_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for LAYERED_WINDOW_ATTRIBUTES_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for LAYERED_WINDOW_ATTRIBUTES_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for LAYERED_WINDOW_ATTRIBUTES_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MENUGETOBJECTINFO_FLAGS(pub u32);
impl windows_core::TypeKind for MENUGETOBJECTINFO_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MENUGETOBJECTINFO_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MENUGETOBJECTINFO_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MENUINFO_MASK(pub u32);
impl windows_core::TypeKind for MENUINFO_MASK {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MENUINFO_MASK {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MENUINFO_MASK").field(&self.0).finish()
    }
}
impl MENUINFO_MASK {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for MENUINFO_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for MENUINFO_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for MENUINFO_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for MENUINFO_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for MENUINFO_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MENUINFO_STYLE(pub u32);
impl windows_core::TypeKind for MENUINFO_STYLE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MENUINFO_STYLE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MENUINFO_STYLE").field(&self.0).finish()
    }
}
impl MENUINFO_STYLE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for MENUINFO_STYLE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for MENUINFO_STYLE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for MENUINFO_STYLE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for MENUINFO_STYLE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for MENUINFO_STYLE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MENU_ITEM_FLAGS(pub u32);
impl windows_core::TypeKind for MENU_ITEM_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MENU_ITEM_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MENU_ITEM_FLAGS").field(&self.0).finish()
    }
}
impl MENU_ITEM_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for MENU_ITEM_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for MENU_ITEM_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for MENU_ITEM_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for MENU_ITEM_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for MENU_ITEM_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MENU_ITEM_MASK(pub u32);
impl windows_core::TypeKind for MENU_ITEM_MASK {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MENU_ITEM_MASK {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MENU_ITEM_MASK").field(&self.0).finish()
    }
}
impl MENU_ITEM_MASK {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for MENU_ITEM_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for MENU_ITEM_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for MENU_ITEM_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for MENU_ITEM_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for MENU_ITEM_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MENU_ITEM_STATE(pub u32);
impl windows_core::TypeKind for MENU_ITEM_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MENU_ITEM_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MENU_ITEM_STATE").field(&self.0).finish()
    }
}
impl MENU_ITEM_STATE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for MENU_ITEM_STATE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for MENU_ITEM_STATE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for MENU_ITEM_STATE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for MENU_ITEM_STATE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for MENU_ITEM_STATE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MENU_ITEM_TYPE(pub u32);
impl windows_core::TypeKind for MENU_ITEM_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MENU_ITEM_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MENU_ITEM_TYPE").field(&self.0).finish()
    }
}
impl MENU_ITEM_TYPE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for MENU_ITEM_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for MENU_ITEM_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for MENU_ITEM_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for MENU_ITEM_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for MENU_ITEM_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MESSAGEBOX_RESULT(pub i32);
impl windows_core::TypeKind for MESSAGEBOX_RESULT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MESSAGEBOX_RESULT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MESSAGEBOX_RESULT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MESSAGEBOX_STYLE(pub u32);
impl windows_core::TypeKind for MESSAGEBOX_STYLE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MESSAGEBOX_STYLE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MESSAGEBOX_STYLE").field(&self.0).finish()
    }
}
impl MESSAGEBOX_STYLE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for MESSAGEBOX_STYLE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for MESSAGEBOX_STYLE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for MESSAGEBOX_STYLE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for MESSAGEBOX_STYLE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for MESSAGEBOX_STYLE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MINIMIZEDMETRICS_ARRANGE(pub i32);
impl windows_core::TypeKind for MINIMIZEDMETRICS_ARRANGE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MINIMIZEDMETRICS_ARRANGE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MINIMIZEDMETRICS_ARRANGE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MSGFLTINFO_STATUS(pub u32);
impl windows_core::TypeKind for MSGFLTINFO_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MSGFLTINFO_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MSGFLTINFO_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS(pub u32);
impl windows_core::TypeKind for MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS").field(&self.0).finish()
    }
}
impl MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MrmDumpType(pub i32);
impl windows_core::TypeKind for MrmDumpType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MrmDumpType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MrmDumpType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MrmIndexerFlags(pub i32);
impl windows_core::TypeKind for MrmIndexerFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MrmIndexerFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MrmIndexerFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MrmPackagingMode(pub i32);
impl windows_core::TypeKind for MrmPackagingMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MrmPackagingMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MrmPackagingMode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MrmPackagingOptions(pub i32);
impl windows_core::TypeKind for MrmPackagingOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MrmPackagingOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MrmPackagingOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MrmPlatformVersion(pub i32);
impl windows_core::TypeKind for MrmPlatformVersion {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MrmPlatformVersion {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MrmPlatformVersion").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MrmResourceIndexerMessageSeverity(pub i32);
impl windows_core::TypeKind for MrmResourceIndexerMessageSeverity {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MrmResourceIndexerMessageSeverity {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MrmResourceIndexerMessageSeverity").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OBJECT_IDENTIFIER(pub i32);
impl windows_core::TypeKind for OBJECT_IDENTIFIER {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OBJECT_IDENTIFIER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OBJECT_IDENTIFIER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PEEK_MESSAGE_REMOVE_TYPE(pub u32);
impl windows_core::TypeKind for PEEK_MESSAGE_REMOVE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PEEK_MESSAGE_REMOVE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PEEK_MESSAGE_REMOVE_TYPE").field(&self.0).finish()
    }
}
impl PEEK_MESSAGE_REMOVE_TYPE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PEEK_MESSAGE_REMOVE_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PEEK_MESSAGE_REMOVE_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PEEK_MESSAGE_REMOVE_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PEEK_MESSAGE_REMOVE_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PEEK_MESSAGE_REMOVE_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct POINTER_INPUT_TYPE(pub i32);
impl windows_core::TypeKind for POINTER_INPUT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for POINTER_INPUT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("POINTER_INPUT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct QUEUE_STATUS_FLAGS(pub u32);
impl windows_core::TypeKind for QUEUE_STATUS_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for QUEUE_STATUS_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("QUEUE_STATUS_FLAGS").field(&self.0).finish()
    }
}
impl QUEUE_STATUS_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for QUEUE_STATUS_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for QUEUE_STATUS_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for QUEUE_STATUS_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for QUEUE_STATUS_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for QUEUE_STATUS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct REGISTER_NOTIFICATION_FLAGS(pub u32);
impl windows_core::TypeKind for REGISTER_NOTIFICATION_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for REGISTER_NOTIFICATION_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("REGISTER_NOTIFICATION_FLAGS").field(&self.0).finish()
    }
}
impl REGISTER_NOTIFICATION_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for REGISTER_NOTIFICATION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for REGISTER_NOTIFICATION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for REGISTER_NOTIFICATION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for REGISTER_NOTIFICATION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for REGISTER_NOTIFICATION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SCROLLBAR_COMMAND(pub i32);
impl windows_core::TypeKind for SCROLLBAR_COMMAND {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SCROLLBAR_COMMAND {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SCROLLBAR_COMMAND").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SCROLLBAR_CONSTANTS(pub i32);
impl windows_core::TypeKind for SCROLLBAR_CONSTANTS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SCROLLBAR_CONSTANTS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SCROLLBAR_CONSTANTS").field(&self.0).finish()
    }
}
impl SCROLLBAR_CONSTANTS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for SCROLLBAR_CONSTANTS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for SCROLLBAR_CONSTANTS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for SCROLLBAR_CONSTANTS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for SCROLLBAR_CONSTANTS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for SCROLLBAR_CONSTANTS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SCROLLINFO_MASK(pub u32);
impl windows_core::TypeKind for SCROLLINFO_MASK {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SCROLLINFO_MASK {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SCROLLINFO_MASK").field(&self.0).finish()
    }
}
impl SCROLLINFO_MASK {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for SCROLLINFO_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for SCROLLINFO_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for SCROLLINFO_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for SCROLLINFO_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for SCROLLINFO_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SCROLL_WINDOW_FLAGS(pub u32);
impl windows_core::TypeKind for SCROLL_WINDOW_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SCROLL_WINDOW_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SCROLL_WINDOW_FLAGS").field(&self.0).finish()
    }
}
impl SCROLL_WINDOW_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for SCROLL_WINDOW_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for SCROLL_WINDOW_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for SCROLL_WINDOW_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for SCROLL_WINDOW_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for SCROLL_WINDOW_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SEND_MESSAGE_TIMEOUT_FLAGS(pub u32);
impl windows_core::TypeKind for SEND_MESSAGE_TIMEOUT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SEND_MESSAGE_TIMEOUT_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SEND_MESSAGE_TIMEOUT_FLAGS").field(&self.0).finish()
    }
}
impl SEND_MESSAGE_TIMEOUT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for SEND_MESSAGE_TIMEOUT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for SEND_MESSAGE_TIMEOUT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for SEND_MESSAGE_TIMEOUT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for SEND_MESSAGE_TIMEOUT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for SEND_MESSAGE_TIMEOUT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SET_WINDOW_POS_FLAGS(pub u32);
impl windows_core::TypeKind for SET_WINDOW_POS_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SET_WINDOW_POS_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SET_WINDOW_POS_FLAGS").field(&self.0).finish()
    }
}
impl SET_WINDOW_POS_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for SET_WINDOW_POS_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for SET_WINDOW_POS_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for SET_WINDOW_POS_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for SET_WINDOW_POS_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for SET_WINDOW_POS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SHOW_WINDOW_CMD(pub i32);
impl windows_core::TypeKind for SHOW_WINDOW_CMD {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SHOW_WINDOW_CMD {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SHOW_WINDOW_CMD").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SHOW_WINDOW_STATUS(pub u32);
impl windows_core::TypeKind for SHOW_WINDOW_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SHOW_WINDOW_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SHOW_WINDOW_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SYSTEM_CURSOR_ID(pub u32);
impl windows_core::TypeKind for SYSTEM_CURSOR_ID {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SYSTEM_CURSOR_ID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SYSTEM_CURSOR_ID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SYSTEM_METRICS_INDEX(pub i32);
impl windows_core::TypeKind for SYSTEM_METRICS_INDEX {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SYSTEM_METRICS_INDEX {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SYSTEM_METRICS_INDEX").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SYSTEM_PARAMETERS_INFO_ACTION(pub u32);
impl windows_core::TypeKind for SYSTEM_PARAMETERS_INFO_ACTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SYSTEM_PARAMETERS_INFO_ACTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SYSTEM_PARAMETERS_INFO_ACTION").field(&self.0).finish()
    }
}
impl SYSTEM_PARAMETERS_INFO_ACTION {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for SYSTEM_PARAMETERS_INFO_ACTION {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for SYSTEM_PARAMETERS_INFO_ACTION {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for SYSTEM_PARAMETERS_INFO_ACTION {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for SYSTEM_PARAMETERS_INFO_ACTION {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for SYSTEM_PARAMETERS_INFO_ACTION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(pub u32);
impl windows_core::TypeKind for SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS").field(&self.0).finish()
    }
}
impl SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TILE_WINDOWS_HOW(pub u32);
impl windows_core::TypeKind for TILE_WINDOWS_HOW {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TILE_WINDOWS_HOW {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TILE_WINDOWS_HOW").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TOOLTIP_DISMISS_FLAGS(pub i32);
impl windows_core::TypeKind for TOOLTIP_DISMISS_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TOOLTIP_DISMISS_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TOOLTIP_DISMISS_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TRACK_POPUP_MENU_FLAGS(pub u32);
impl windows_core::TypeKind for TRACK_POPUP_MENU_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TRACK_POPUP_MENU_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TRACK_POPUP_MENU_FLAGS").field(&self.0).finish()
    }
}
impl TRACK_POPUP_MENU_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for TRACK_POPUP_MENU_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for TRACK_POPUP_MENU_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for TRACK_POPUP_MENU_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for TRACK_POPUP_MENU_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for TRACK_POPUP_MENU_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UPDATE_LAYERED_WINDOW_FLAGS(pub u32);
impl windows_core::TypeKind for UPDATE_LAYERED_WINDOW_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UPDATE_LAYERED_WINDOW_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UPDATE_LAYERED_WINDOW_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WINDOWPLACEMENT_FLAGS(pub u32);
impl windows_core::TypeKind for WINDOWPLACEMENT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WINDOWPLACEMENT_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WINDOWPLACEMENT_FLAGS").field(&self.0).finish()
    }
}
impl WINDOWPLACEMENT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for WINDOWPLACEMENT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for WINDOWPLACEMENT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for WINDOWPLACEMENT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for WINDOWPLACEMENT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for WINDOWPLACEMENT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WINDOWS_HOOK_ID(pub i32);
impl windows_core::TypeKind for WINDOWS_HOOK_ID {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WINDOWS_HOOK_ID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WINDOWS_HOOK_ID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WINDOW_DISPLAY_AFFINITY(pub u32);
impl windows_core::TypeKind for WINDOW_DISPLAY_AFFINITY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WINDOW_DISPLAY_AFFINITY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WINDOW_DISPLAY_AFFINITY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WINDOW_EX_STYLE(pub u32);
impl windows_core::TypeKind for WINDOW_EX_STYLE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WINDOW_EX_STYLE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WINDOW_EX_STYLE").field(&self.0).finish()
    }
}
impl WINDOW_EX_STYLE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for WINDOW_EX_STYLE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for WINDOW_EX_STYLE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for WINDOW_EX_STYLE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for WINDOW_EX_STYLE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for WINDOW_EX_STYLE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WINDOW_LONG_PTR_INDEX(pub i32);
impl windows_core::TypeKind for WINDOW_LONG_PTR_INDEX {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WINDOW_LONG_PTR_INDEX {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WINDOW_LONG_PTR_INDEX").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WINDOW_MESSAGE_FILTER_ACTION(pub u32);
impl windows_core::TypeKind for WINDOW_MESSAGE_FILTER_ACTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WINDOW_MESSAGE_FILTER_ACTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WINDOW_MESSAGE_FILTER_ACTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WINDOW_STYLE(pub u32);
impl windows_core::TypeKind for WINDOW_STYLE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WINDOW_STYLE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WINDOW_STYLE").field(&self.0).finish()
    }
}
impl WINDOW_STYLE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for WINDOW_STYLE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for WINDOW_STYLE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for WINDOW_STYLE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for WINDOW_STYLE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for WINDOW_STYLE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WNDCLASS_STYLES(pub u32);
impl windows_core::TypeKind for WNDCLASS_STYLES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WNDCLASS_STYLES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WNDCLASS_STYLES").field(&self.0).finish()
    }
}
impl WNDCLASS_STYLES {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for WNDCLASS_STYLES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for WNDCLASS_STYLES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for WNDCLASS_STYLES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for WNDCLASS_STYLES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for WNDCLASS_STYLES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ACCEL {
    pub fVirt: ACCEL_VIRT_FLAGS,
    pub key: u16,
    pub cmd: u16,
}
impl windows_core::TypeKind for ACCEL {
    type TypeKind = windows_core::CopyType;
}
impl Default for ACCEL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ALTTABINFO {
    pub cbSize: u32,
    pub cItems: i32,
    pub cColumns: i32,
    pub cRows: i32,
    pub iColFocus: i32,
    pub iRowFocus: i32,
    pub cxItem: i32,
    pub cyItem: i32,
    pub ptStart: super::super::Foundation::POINT,
}
impl windows_core::TypeKind for ALTTABINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for ALTTABINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ANIMATIONINFO {
    pub cbSize: u32,
    pub iMinAnimate: i32,
}
impl windows_core::TypeKind for ANIMATIONINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for ANIMATIONINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AUDIODESCRIPTION {
    pub cbSize: u32,
    pub Enabled: super::super::Foundation::BOOL,
    pub Locale: u32,
}
impl windows_core::TypeKind for AUDIODESCRIPTION {
    type TypeKind = windows_core::CopyType;
}
impl Default for AUDIODESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CBTACTIVATESTRUCT {
    pub fMouse: super::super::Foundation::BOOL,
    pub hWndActive: super::super::Foundation::HWND,
}
impl windows_core::TypeKind for CBTACTIVATESTRUCT {
    type TypeKind = windows_core::CopyType;
}
impl Default for CBTACTIVATESTRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CBT_CREATEWNDA {
    pub lpcs: *mut CREATESTRUCTA,
    pub hwndInsertAfter: super::super::Foundation::HWND,
}
impl windows_core::TypeKind for CBT_CREATEWNDA {
    type TypeKind = windows_core::CopyType;
}
impl Default for CBT_CREATEWNDA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CBT_CREATEWNDW {
    pub lpcs: *mut CREATESTRUCTW,
    pub hwndInsertAfter: super::super::Foundation::HWND,
}
impl windows_core::TypeKind for CBT_CREATEWNDW {
    type TypeKind = windows_core::CopyType;
}
impl Default for CBT_CREATEWNDW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CHANGEFILTERSTRUCT {
    pub cbSize: u32,
    pub ExtStatus: MSGFLTINFO_STATUS,
}
impl windows_core::TypeKind for CHANGEFILTERSTRUCT {
    type TypeKind = windows_core::CopyType;
}
impl Default for CHANGEFILTERSTRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLIENTCREATESTRUCT {
    pub hWindowMenu: super::super::Foundation::HANDLE,
    pub idFirstChild: u32,
}
impl windows_core::TypeKind for CLIENTCREATESTRUCT {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLIENTCREATESTRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CREATESTRUCTA {
    pub lpCreateParams: *mut core::ffi::c_void,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub hMenu: HMENU,
    pub hwndParent: super::super::Foundation::HWND,
    pub cy: i32,
    pub cx: i32,
    pub y: i32,
    pub x: i32,
    pub style: i32,
    pub lpszName: windows_core::PCSTR,
    pub lpszClass: windows_core::PCSTR,
    pub dwExStyle: u32,
}
impl windows_core::TypeKind for CREATESTRUCTA {
    type TypeKind = windows_core::CopyType;
}
impl Default for CREATESTRUCTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CREATESTRUCTW {
    pub lpCreateParams: *mut core::ffi::c_void,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub hMenu: HMENU,
    pub hwndParent: super::super::Foundation::HWND,
    pub cy: i32,
    pub cx: i32,
    pub y: i32,
    pub x: i32,
    pub style: i32,
    pub lpszName: windows_core::PCWSTR,
    pub lpszClass: windows_core::PCWSTR,
    pub dwExStyle: u32,
}
impl windows_core::TypeKind for CREATESTRUCTW {
    type TypeKind = windows_core::CopyType;
}
impl Default for CREATESTRUCTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CURSORINFO {
    pub cbSize: u32,
    pub flags: CURSORINFO_FLAGS,
    pub hCursor: HCURSOR,
    pub ptScreenPos: super::super::Foundation::POINT,
}
impl windows_core::TypeKind for CURSORINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for CURSORINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CURSORSHAPE {
    pub xHotSpot: i32,
    pub yHotSpot: i32,
    pub cx: i32,
    pub cy: i32,
    pub cbWidth: i32,
    pub Planes: u8,
    pub BitsPixel: u8,
}
impl windows_core::TypeKind for CURSORSHAPE {
    type TypeKind = windows_core::CopyType;
}
impl Default for CURSORSHAPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CWPRETSTRUCT {
    pub lResult: super::super::Foundation::LRESULT,
    pub lParam: super::super::Foundation::LPARAM,
    pub wParam: super::super::Foundation::WPARAM,
    pub message: u32,
    pub hwnd: super::super::Foundation::HWND,
}
impl windows_core::TypeKind for CWPRETSTRUCT {
    type TypeKind = windows_core::CopyType;
}
impl Default for CWPRETSTRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CWPSTRUCT {
    pub lParam: super::super::Foundation::LPARAM,
    pub wParam: super::super::Foundation::WPARAM,
    pub message: u32,
    pub hwnd: super::super::Foundation::HWND,
}
impl windows_core::TypeKind for CWPSTRUCT {
    type TypeKind = windows_core::CopyType;
}
impl Default for CWPSTRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DEBUGHOOKINFO {
    pub idThread: u32,
    pub idThreadInstaller: u32,
    pub lParam: super::super::Foundation::LPARAM,
    pub wParam: super::super::Foundation::WPARAM,
    pub code: i32,
}
impl windows_core::TypeKind for DEBUGHOOKINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for DEBUGHOOKINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DEVICE_EVENT_BECOMING_READY {
    pub Version: u32,
    pub Reason: u32,
    pub Estimated100msToReady: u32,
}
impl windows_core::TypeKind for DEVICE_EVENT_BECOMING_READY {
    type TypeKind = windows_core::CopyType;
}
impl Default for DEVICE_EVENT_BECOMING_READY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DEVICE_EVENT_EXTERNAL_REQUEST {
    pub Version: u32,
    pub DeviceClass: u32,
    pub ButtonStatus: u16,
    pub Request: u16,
    pub SystemTime: i64,
}
impl windows_core::TypeKind for DEVICE_EVENT_EXTERNAL_REQUEST {
    type TypeKind = windows_core::CopyType;
}
impl Default for DEVICE_EVENT_EXTERNAL_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DEVICE_EVENT_GENERIC_DATA {
    pub EventNumber: u32,
}
impl windows_core::TypeKind for DEVICE_EVENT_GENERIC_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for DEVICE_EVENT_GENERIC_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DEVICE_EVENT_MOUNT {
    pub Version: u32,
    pub Flags: u32,
    pub FileSystemNameLength: u32,
    pub FileSystemNameOffset: u32,
}
impl windows_core::TypeKind for DEVICE_EVENT_MOUNT {
    type TypeKind = windows_core::CopyType;
}
impl Default for DEVICE_EVENT_MOUNT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DEVICE_EVENT_RBC_DATA {
    pub EventNumber: u32,
    pub SenseQualifier: u8,
    pub SenseCode: u8,
    pub SenseKey: u8,
    pub Reserved: u8,
    pub Information: u32,
}
impl windows_core::TypeKind for DEVICE_EVENT_RBC_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for DEVICE_EVENT_RBC_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DEV_BROADCAST_DEVICEINTERFACE_A {
    pub dbcc_size: u32,
    pub dbcc_devicetype: u32,
    pub dbcc_reserved: u32,
    pub dbcc_classguid: windows_core::GUID,
    pub dbcc_name: [i8; 1],
}
impl windows_core::TypeKind for DEV_BROADCAST_DEVICEINTERFACE_A {
    type TypeKind = windows_core::CopyType;
}
impl Default for DEV_BROADCAST_DEVICEINTERFACE_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DEV_BROADCAST_DEVICEINTERFACE_W {
    pub dbcc_size: u32,
    pub dbcc_devicetype: u32,
    pub dbcc_reserved: u32,
    pub dbcc_classguid: windows_core::GUID,
    pub dbcc_name: [u16; 1],
}
impl windows_core::TypeKind for DEV_BROADCAST_DEVICEINTERFACE_W {
    type TypeKind = windows_core::CopyType;
}
impl Default for DEV_BROADCAST_DEVICEINTERFACE_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DEV_BROADCAST_DEVNODE {
    pub dbcd_size: u32,
    pub dbcd_devicetype: u32,
    pub dbcd_reserved: u32,
    pub dbcd_devnode: u32,
}
impl windows_core::TypeKind for DEV_BROADCAST_DEVNODE {
    type TypeKind = windows_core::CopyType;
}
impl Default for DEV_BROADCAST_DEVNODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DEV_BROADCAST_HANDLE {
    pub dbch_size: u32,
    pub dbch_devicetype: u32,
    pub dbch_reserved: u32,
    pub dbch_handle: super::super::Foundation::HANDLE,
    pub dbch_hdevnotify: HDEVNOTIFY,
    pub dbch_eventguid: windows_core::GUID,
    pub dbch_nameoffset: i32,
    pub dbch_data: [u8; 1],
}
impl windows_core::TypeKind for DEV_BROADCAST_HANDLE {
    type TypeKind = windows_core::CopyType;
}
impl Default for DEV_BROADCAST_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DEV_BROADCAST_HANDLE32 {
    pub dbch_size: u32,
    pub dbch_devicetype: u32,
    pub dbch_reserved: u32,
    pub dbch_handle: u32,
    pub dbch_hdevnotify: u32,
    pub dbch_eventguid: windows_core::GUID,
    pub dbch_nameoffset: i32,
    pub dbch_data: [u8; 1],
}
impl windows_core::TypeKind for DEV_BROADCAST_HANDLE32 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DEV_BROADCAST_HANDLE32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DEV_BROADCAST_HANDLE64 {
    pub dbch_size: u32,
    pub dbch_devicetype: u32,
    pub dbch_reserved: u32,
    pub dbch_handle: u64,
    pub dbch_hdevnotify: u64,
    pub dbch_eventguid: windows_core::GUID,
    pub dbch_nameoffset: i32,
    pub dbch_data: [u8; 1],
}
impl windows_core::TypeKind for DEV_BROADCAST_HANDLE64 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DEV_BROADCAST_HANDLE64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DEV_BROADCAST_HDR {
    pub dbch_size: u32,
    pub dbch_devicetype: DEV_BROADCAST_HDR_DEVICE_TYPE,
    pub dbch_reserved: u32,
}
impl windows_core::TypeKind for DEV_BROADCAST_HDR {
    type TypeKind = windows_core::CopyType;
}
impl Default for DEV_BROADCAST_HDR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DEV_BROADCAST_NET {
    pub dbcn_size: u32,
    pub dbcn_devicetype: u32,
    pub dbcn_reserved: u32,
    pub dbcn_resource: u32,
    pub dbcn_flags: u32,
}
impl windows_core::TypeKind for DEV_BROADCAST_NET {
    type TypeKind = windows_core::CopyType;
}
impl Default for DEV_BROADCAST_NET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DEV_BROADCAST_OEM {
    pub dbco_size: u32,
    pub dbco_devicetype: u32,
    pub dbco_reserved: u32,
    pub dbco_identifier: u32,
    pub dbco_suppfunc: u32,
}
impl windows_core::TypeKind for DEV_BROADCAST_OEM {
    type TypeKind = windows_core::CopyType;
}
impl Default for DEV_BROADCAST_OEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DEV_BROADCAST_PORT_A {
    pub dbcp_size: u32,
    pub dbcp_devicetype: u32,
    pub dbcp_reserved: u32,
    pub dbcp_name: [i8; 1],
}
impl windows_core::TypeKind for DEV_BROADCAST_PORT_A {
    type TypeKind = windows_core::CopyType;
}
impl Default for DEV_BROADCAST_PORT_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DEV_BROADCAST_PORT_W {
    pub dbcp_size: u32,
    pub dbcp_devicetype: u32,
    pub dbcp_reserved: u32,
    pub dbcp_name: [u16; 1],
}
impl windows_core::TypeKind for DEV_BROADCAST_PORT_W {
    type TypeKind = windows_core::CopyType;
}
impl Default for DEV_BROADCAST_PORT_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DEV_BROADCAST_VOLUME {
    pub dbcv_size: u32,
    pub dbcv_devicetype: u32,
    pub dbcv_reserved: u32,
    pub dbcv_unitmask: u32,
    pub dbcv_flags: DEV_BROADCAST_VOLUME_FLAGS,
}
impl windows_core::TypeKind for DEV_BROADCAST_VOLUME {
    type TypeKind = windows_core::CopyType;
}
impl Default for DEV_BROADCAST_VOLUME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DISK_HEALTH_NOTIFICATION_DATA {
    pub DeviceGuid: windows_core::GUID,
}
impl windows_core::TypeKind for DISK_HEALTH_NOTIFICATION_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for DISK_HEALTH_NOTIFICATION_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct DLGITEMTEMPLATE {
    pub style: u32,
    pub dwExtendedStyle: u32,
    pub x: i16,
    pub y: i16,
    pub cx: i16,
    pub cy: i16,
    pub id: u16,
}
impl windows_core::TypeKind for DLGITEMTEMPLATE {
    type TypeKind = windows_core::CopyType;
}
impl Default for DLGITEMTEMPLATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct DLGTEMPLATE {
    pub style: u32,
    pub dwExtendedStyle: u32,
    pub cdit: u16,
    pub x: i16,
    pub y: i16,
    pub cx: i16,
    pub cy: i16,
}
impl windows_core::TypeKind for DLGTEMPLATE {
    type TypeKind = windows_core::CopyType;
}
impl Default for DLGTEMPLATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DROPSTRUCT {
    pub hwndSource: super::super::Foundation::HWND,
    pub hwndSink: super::super::Foundation::HWND,
    pub wFmt: u32,
    pub dwData: usize,
    pub ptDrop: super::super::Foundation::POINT,
    pub dwControlData: u32,
}
impl windows_core::TypeKind for DROPSTRUCT {
    type TypeKind = windows_core::CopyType;
}
impl Default for DROPSTRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EVENTMSG {
    pub message: u32,
    pub paramL: u32,
    pub paramH: u32,
    pub time: u32,
    pub hwnd: super::super::Foundation::HWND,
}
impl windows_core::TypeKind for EVENTMSG {
    type TypeKind = windows_core::CopyType;
}
impl Default for EVENTMSG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLASHWINFO {
    pub cbSize: u32,
    pub hwnd: super::super::Foundation::HWND,
    pub dwFlags: FLASHWINFO_FLAGS,
    pub uCount: u32,
    pub dwTimeout: u32,
}
impl windows_core::TypeKind for FLASHWINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for FLASHWINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GETCLIPBMETADATA {
    pub Version: u32,
    pub IsDelayRendered: super::super::Foundation::BOOL,
    pub IsSynthetic: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for GETCLIPBMETADATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for GETCLIPBMETADATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GUID_IO_DISK_CLONE_ARRIVAL_INFORMATION {
    pub DiskNumber: u32,
}
impl windows_core::TypeKind for GUID_IO_DISK_CLONE_ARRIVAL_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for GUID_IO_DISK_CLONE_ARRIVAL_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GUITHREADINFO {
    pub cbSize: u32,
    pub flags: GUITHREADINFO_FLAGS,
    pub hwndActive: super::super::Foundation::HWND,
    pub hwndFocus: super::super::Foundation::HWND,
    pub hwndCapture: super::super::Foundation::HWND,
    pub hwndMenuOwner: super::super::Foundation::HWND,
    pub hwndMoveSize: super::super::Foundation::HWND,
    pub hwndCaret: super::super::Foundation::HWND,
    pub rcCaret: super::super::Foundation::RECT,
}
impl windows_core::TypeKind for GUITHREADINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for GUITHREADINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HACCEL(pub isize);
impl HACCEL {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl windows_core::Free for HACCEL {
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            _ = DestroyAcceleratorTable(*self);
        }
    }
}
impl Default for HACCEL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HACCEL {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HARDWAREHOOKSTRUCT {
    pub hwnd: super::super::Foundation::HWND,
    pub message: u32,
    pub wParam: super::super::Foundation::WPARAM,
    pub lParam: super::super::Foundation::LPARAM,
}
impl windows_core::TypeKind for HARDWAREHOOKSTRUCT {
    type TypeKind = windows_core::CopyType;
}
impl Default for HARDWAREHOOKSTRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HCURSOR(pub isize);
impl HCURSOR {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl windows_core::Free for HCURSOR {
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            _ = DestroyCursor(*self);
        }
    }
}
impl Default for HCURSOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HCURSOR {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::imp::CanInto<HICON> for HCURSOR {}
impl From<HCURSOR> for HICON {
    fn from(value: HCURSOR) -> Self {
        Self(value.0)
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HDEVNOTIFY(pub *mut core::ffi::c_void);
impl HDEVNOTIFY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl windows_core::Free for HDEVNOTIFY {
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            _ = UnregisterDeviceNotification(*self);
        }
    }
}
impl Default for HDEVNOTIFY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HDEVNOTIFY {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HDWP(pub isize);
impl HDWP {
    pub fn is_invalid(&self) -> bool {
        self.0 == 0
    }
}
impl Default for HDWP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HDWP {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HHOOK(pub isize);
impl HHOOK {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl windows_core::Free for HHOOK {
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            _ = UnhookWindowsHookEx(*self);
        }
    }
}
impl Default for HHOOK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HHOOK {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HICON(pub isize);
impl HICON {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl windows_core::Free for HICON {
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            _ = DestroyIcon(*self);
        }
    }
}
impl Default for HICON {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HICON {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HMENU(pub isize);
impl HMENU {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl windows_core::Free for HMENU {
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            _ = DestroyMenu(*self);
        }
    }
}
impl Default for HMENU {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HMENU {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ICONINFO {
    pub fIcon: super::super::Foundation::BOOL,
    pub xHotspot: u32,
    pub yHotspot: u32,
    pub hbmMask: super::super::Graphics::Gdi::HBITMAP,
    pub hbmColor: super::super::Graphics::Gdi::HBITMAP,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::TypeKind for ICONINFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for ICONINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ICONINFOEXA {
    pub cbSize: u32,
    pub fIcon: super::super::Foundation::BOOL,
    pub xHotspot: u32,
    pub yHotspot: u32,
    pub hbmMask: super::super::Graphics::Gdi::HBITMAP,
    pub hbmColor: super::super::Graphics::Gdi::HBITMAP,
    pub wResID: u16,
    pub szModName: [i8; 260],
    pub szResName: [i8; 260],
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::TypeKind for ICONINFOEXA {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for ICONINFOEXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ICONINFOEXW {
    pub cbSize: u32,
    pub fIcon: super::super::Foundation::BOOL,
    pub xHotspot: u32,
    pub yHotspot: u32,
    pub hbmMask: super::super::Graphics::Gdi::HBITMAP,
    pub hbmColor: super::super::Graphics::Gdi::HBITMAP,
    pub wResID: u16,
    pub szModName: [u16; 260],
    pub szResName: [u16; 260],
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::TypeKind for ICONINFOEXW {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for ICONINFOEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ICONMETRICSA {
    pub cbSize: u32,
    pub iHorzSpacing: i32,
    pub iVertSpacing: i32,
    pub iTitleWrap: i32,
    pub lfFont: super::super::Graphics::Gdi::LOGFONTA,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::TypeKind for ICONMETRICSA {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for ICONMETRICSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ICONMETRICSW {
    pub cbSize: u32,
    pub iHorzSpacing: i32,
    pub iVertSpacing: i32,
    pub iTitleWrap: i32,
    pub lfFont: super::super::Graphics::Gdi::LOGFONTW,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::TypeKind for ICONMETRICSW {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for ICONMETRICSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IndexedResourceQualifier {
    pub name: windows_core::PWSTR,
    pub value: windows_core::PWSTR,
}
impl windows_core::TypeKind for IndexedResourceQualifier {
    type TypeKind = windows_core::CopyType;
}
impl Default for IndexedResourceQualifier {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KBDLLHOOKSTRUCT {
    pub vkCode: u32,
    pub scanCode: u32,
    pub flags: KBDLLHOOKSTRUCT_FLAGS,
    pub time: u32,
    pub dwExtraInfo: usize,
}
impl windows_core::TypeKind for KBDLLHOOKSTRUCT {
    type TypeKind = windows_core::CopyType;
}
impl Default for KBDLLHOOKSTRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MDICREATESTRUCTA {
    pub szClass: windows_core::PCSTR,
    pub szTitle: windows_core::PCSTR,
    pub hOwner: super::super::Foundation::HANDLE,
    pub x: i32,
    pub y: i32,
    pub cx: i32,
    pub cy: i32,
    pub style: WINDOW_STYLE,
    pub lParam: super::super::Foundation::LPARAM,
}
impl windows_core::TypeKind for MDICREATESTRUCTA {
    type TypeKind = windows_core::CopyType;
}
impl Default for MDICREATESTRUCTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MDICREATESTRUCTW {
    pub szClass: windows_core::PCWSTR,
    pub szTitle: windows_core::PCWSTR,
    pub hOwner: super::super::Foundation::HANDLE,
    pub x: i32,
    pub y: i32,
    pub cx: i32,
    pub cy: i32,
    pub style: WINDOW_STYLE,
    pub lParam: super::super::Foundation::LPARAM,
}
impl windows_core::TypeKind for MDICREATESTRUCTW {
    type TypeKind = windows_core::CopyType;
}
impl Default for MDICREATESTRUCTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MDINEXTMENU {
    pub hmenuIn: HMENU,
    pub hmenuNext: HMENU,
    pub hwndNext: super::super::Foundation::HWND,
}
impl windows_core::TypeKind for MDINEXTMENU {
    type TypeKind = windows_core::CopyType;
}
impl Default for MDINEXTMENU {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MENUBARINFO {
    pub cbSize: u32,
    pub rcBar: super::super::Foundation::RECT,
    pub hMenu: HMENU,
    pub hwndMenu: super::super::Foundation::HWND,
    pub _bitfield: i32,
}
impl windows_core::TypeKind for MENUBARINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for MENUBARINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MENUEX_TEMPLATE_HEADER {
    pub wVersion: u16,
    pub wOffset: u16,
    pub dwHelpId: u32,
}
impl windows_core::TypeKind for MENUEX_TEMPLATE_HEADER {
    type TypeKind = windows_core::CopyType;
}
impl Default for MENUEX_TEMPLATE_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MENUEX_TEMPLATE_ITEM {
    pub dwType: u32,
    pub dwState: u32,
    pub uId: u32,
    pub wFlags: u16,
    pub szText: [u16; 1],
}
impl windows_core::TypeKind for MENUEX_TEMPLATE_ITEM {
    type TypeKind = windows_core::CopyType;
}
impl Default for MENUEX_TEMPLATE_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MENUGETOBJECTINFO {
    pub dwFlags: MENUGETOBJECTINFO_FLAGS,
    pub uPos: u32,
    pub hmenu: HMENU,
    pub riid: *mut core::ffi::c_void,
    pub pvObj: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for MENUGETOBJECTINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for MENUGETOBJECTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MENUINFO {
    pub cbSize: u32,
    pub fMask: MENUINFO_MASK,
    pub dwStyle: MENUINFO_STYLE,
    pub cyMax: u32,
    pub hbrBack: super::super::Graphics::Gdi::HBRUSH,
    pub dwContextHelpID: u32,
    pub dwMenuData: usize,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::TypeKind for MENUINFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for MENUINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MENUITEMINFOA {
    pub cbSize: u32,
    pub fMask: MENU_ITEM_MASK,
    pub fType: MENU_ITEM_TYPE,
    pub fState: MENU_ITEM_STATE,
    pub wID: u32,
    pub hSubMenu: HMENU,
    pub hbmpChecked: super::super::Graphics::Gdi::HBITMAP,
    pub hbmpUnchecked: super::super::Graphics::Gdi::HBITMAP,
    pub dwItemData: usize,
    pub dwTypeData: windows_core::PSTR,
    pub cch: u32,
    pub hbmpItem: super::super::Graphics::Gdi::HBITMAP,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::TypeKind for MENUITEMINFOA {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for MENUITEMINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MENUITEMINFOW {
    pub cbSize: u32,
    pub fMask: MENU_ITEM_MASK,
    pub fType: MENU_ITEM_TYPE,
    pub fState: MENU_ITEM_STATE,
    pub wID: u32,
    pub hSubMenu: HMENU,
    pub hbmpChecked: super::super::Graphics::Gdi::HBITMAP,
    pub hbmpUnchecked: super::super::Graphics::Gdi::HBITMAP,
    pub dwItemData: usize,
    pub dwTypeData: windows_core::PWSTR,
    pub cch: u32,
    pub hbmpItem: super::super::Graphics::Gdi::HBITMAP,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::TypeKind for MENUITEMINFOW {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for MENUITEMINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MENUITEMTEMPLATE {
    pub mtOption: u16,
    pub mtID: u16,
    pub mtString: [u16; 1],
}
impl windows_core::TypeKind for MENUITEMTEMPLATE {
    type TypeKind = windows_core::CopyType;
}
impl Default for MENUITEMTEMPLATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MENUITEMTEMPLATEHEADER {
    pub versionNumber: u16,
    pub offset: u16,
}
impl windows_core::TypeKind for MENUITEMTEMPLATEHEADER {
    type TypeKind = windows_core::CopyType;
}
impl Default for MENUITEMTEMPLATEHEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MENUTEMPLATEEX {
    pub Anonymous: MENUTEMPLATEEX_0,
}
impl windows_core::TypeKind for MENUTEMPLATEEX {
    type TypeKind = windows_core::CopyType;
}
impl Default for MENUTEMPLATEEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union MENUTEMPLATEEX_0 {
    pub Menu: MENUTEMPLATEEX_0_1,
    pub MenuEx: MENUTEMPLATEEX_0_0,
}
impl windows_core::TypeKind for MENUTEMPLATEEX_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for MENUTEMPLATEEX_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MENUTEMPLATEEX_0_0 {
    pub mexHeader: MENUEX_TEMPLATE_HEADER,
    pub mexItem: [MENUEX_TEMPLATE_ITEM; 1],
}
impl windows_core::TypeKind for MENUTEMPLATEEX_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for MENUTEMPLATEEX_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MENUTEMPLATEEX_0_1 {
    pub mitHeader: MENUITEMTEMPLATEHEADER,
    pub miTemplate: [MENUITEMTEMPLATE; 1],
}
impl windows_core::TypeKind for MENUTEMPLATEEX_0_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for MENUTEMPLATEEX_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MESSAGE_RESOURCE_BLOCK {
    pub LowId: u32,
    pub HighId: u32,
    pub OffsetToEntries: u32,
}
impl windows_core::TypeKind for MESSAGE_RESOURCE_BLOCK {
    type TypeKind = windows_core::CopyType;
}
impl Default for MESSAGE_RESOURCE_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MESSAGE_RESOURCE_DATA {
    pub NumberOfBlocks: u32,
    pub Blocks: [MESSAGE_RESOURCE_BLOCK; 1],
}
impl windows_core::TypeKind for MESSAGE_RESOURCE_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for MESSAGE_RESOURCE_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MESSAGE_RESOURCE_ENTRY {
    pub Length: u16,
    pub Flags: u16,
    pub Text: [u8; 1],
}
impl windows_core::TypeKind for MESSAGE_RESOURCE_ENTRY {
    type TypeKind = windows_core::CopyType;
}
impl Default for MESSAGE_RESOURCE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MINIMIZEDMETRICS {
    pub cbSize: u32,
    pub iWidth: i32,
    pub iHorzGap: i32,
    pub iVertGap: i32,
    pub iArrange: MINIMIZEDMETRICS_ARRANGE,
}
impl windows_core::TypeKind for MINIMIZEDMETRICS {
    type TypeKind = windows_core::CopyType;
}
impl Default for MINIMIZEDMETRICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MINMAXINFO {
    pub ptReserved: super::super::Foundation::POINT,
    pub ptMaxSize: super::super::Foundation::POINT,
    pub ptMaxPosition: super::super::Foundation::POINT,
    pub ptMinTrackSize: super::super::Foundation::POINT,
    pub ptMaxTrackSize: super::super::Foundation::POINT,
}
impl windows_core::TypeKind for MINMAXINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for MINMAXINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MOUSEHOOKSTRUCT {
    pub pt: super::super::Foundation::POINT,
    pub hwnd: super::super::Foundation::HWND,
    pub wHitTestCode: u32,
    pub dwExtraInfo: usize,
}
impl windows_core::TypeKind for MOUSEHOOKSTRUCT {
    type TypeKind = windows_core::CopyType;
}
impl Default for MOUSEHOOKSTRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MOUSEHOOKSTRUCTEX {
    pub Base: MOUSEHOOKSTRUCT,
    pub mouseData: u32,
}
impl windows_core::TypeKind for MOUSEHOOKSTRUCTEX {
    type TypeKind = windows_core::CopyType;
}
impl Default for MOUSEHOOKSTRUCTEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MSG {
    pub hwnd: super::super::Foundation::HWND,
    pub message: u32,
    pub wParam: super::super::Foundation::WPARAM,
    pub lParam: super::super::Foundation::LPARAM,
    pub time: u32,
    pub pt: super::super::Foundation::POINT,
}
impl windows_core::TypeKind for MSG {
    type TypeKind = windows_core::CopyType;
}
impl Default for MSG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_Shell")]
#[derive(Clone, Copy, Debug)]
pub struct MSGBOXPARAMSA {
    pub cbSize: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszText: windows_core::PCSTR,
    pub lpszCaption: windows_core::PCSTR,
    pub dwStyle: MESSAGEBOX_STYLE,
    pub lpszIcon: windows_core::PCSTR,
    pub dwContextHelpId: usize,
    pub lpfnMsgBoxCallback: MSGBOXCALLBACK,
    pub dwLanguageId: u32,
}
#[cfg(feature = "Win32_UI_Shell")]
impl windows_core::TypeKind for MSGBOXPARAMSA {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_UI_Shell")]
impl Default for MSGBOXPARAMSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_Shell")]
#[derive(Clone, Copy, Debug)]
pub struct MSGBOXPARAMSW {
    pub cbSize: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszText: windows_core::PCWSTR,
    pub lpszCaption: windows_core::PCWSTR,
    pub dwStyle: MESSAGEBOX_STYLE,
    pub lpszIcon: windows_core::PCWSTR,
    pub dwContextHelpId: usize,
    pub lpfnMsgBoxCallback: MSGBOXCALLBACK,
    pub dwLanguageId: u32,
}
#[cfg(feature = "Win32_UI_Shell")]
impl windows_core::TypeKind for MSGBOXPARAMSW {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_UI_Shell")]
impl Default for MSGBOXPARAMSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MSLLHOOKSTRUCT {
    pub pt: super::super::Foundation::POINT,
    pub mouseData: u32,
    pub flags: u32,
    pub time: u32,
    pub dwExtraInfo: usize,
}
impl windows_core::TypeKind for MSLLHOOKSTRUCT {
    type TypeKind = windows_core::CopyType;
}
impl Default for MSLLHOOKSTRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MrmResourceIndexerHandle {
    pub handle: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for MrmResourceIndexerHandle {
    type TypeKind = windows_core::CopyType;
}
impl Default for MrmResourceIndexerHandle {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MrmResourceIndexerMessage {
    pub severity: MrmResourceIndexerMessageSeverity,
    pub id: u32,
    pub text: windows_core::PCWSTR,
}
impl windows_core::TypeKind for MrmResourceIndexerMessage {
    type TypeKind = windows_core::CopyType;
}
impl Default for MrmResourceIndexerMessage {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NCCALCSIZE_PARAMS {
    pub rgrc: [super::super::Foundation::RECT; 3],
    pub lppos: *mut WINDOWPOS,
}
impl windows_core::TypeKind for NCCALCSIZE_PARAMS {
    type TypeKind = windows_core::CopyType;
}
impl Default for NCCALCSIZE_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NONCLIENTMETRICSA {
    pub cbSize: u32,
    pub iBorderWidth: i32,
    pub iScrollWidth: i32,
    pub iScrollHeight: i32,
    pub iCaptionWidth: i32,
    pub iCaptionHeight: i32,
    pub lfCaptionFont: super::super::Graphics::Gdi::LOGFONTA,
    pub iSmCaptionWidth: i32,
    pub iSmCaptionHeight: i32,
    pub lfSmCaptionFont: super::super::Graphics::Gdi::LOGFONTA,
    pub iMenuWidth: i32,
    pub iMenuHeight: i32,
    pub lfMenuFont: super::super::Graphics::Gdi::LOGFONTA,
    pub lfStatusFont: super::super::Graphics::Gdi::LOGFONTA,
    pub lfMessageFont: super::super::Graphics::Gdi::LOGFONTA,
    pub iPaddedBorderWidth: i32,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::TypeKind for NONCLIENTMETRICSA {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for NONCLIENTMETRICSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NONCLIENTMETRICSW {
    pub cbSize: u32,
    pub iBorderWidth: i32,
    pub iScrollWidth: i32,
    pub iScrollHeight: i32,
    pub iCaptionWidth: i32,
    pub iCaptionHeight: i32,
    pub lfCaptionFont: super::super::Graphics::Gdi::LOGFONTW,
    pub iSmCaptionWidth: i32,
    pub iSmCaptionHeight: i32,
    pub lfSmCaptionFont: super::super::Graphics::Gdi::LOGFONTW,
    pub iMenuWidth: i32,
    pub iMenuHeight: i32,
    pub lfMenuFont: super::super::Graphics::Gdi::LOGFONTW,
    pub lfStatusFont: super::super::Graphics::Gdi::LOGFONTW,
    pub lfMessageFont: super::super::Graphics::Gdi::LOGFONTW,
    pub iPaddedBorderWidth: i32,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::TypeKind for NONCLIENTMETRICSW {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for NONCLIENTMETRICSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCROLLBARINFO {
    pub cbSize: u32,
    pub rcScrollBar: super::super::Foundation::RECT,
    pub dxyLineButton: i32,
    pub xyThumbTop: i32,
    pub xyThumbBottom: i32,
    pub reserved: i32,
    pub rgstate: [u32; 6],
}
impl windows_core::TypeKind for SCROLLBARINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for SCROLLBARINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCROLLINFO {
    pub cbSize: u32,
    pub fMask: SCROLLINFO_MASK,
    pub nMin: i32,
    pub nMax: i32,
    pub nPage: u32,
    pub nPos: i32,
    pub nTrackPos: i32,
}
impl windows_core::TypeKind for SCROLLINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for SCROLLINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SHELLHOOKINFO {
    pub hwnd: super::super::Foundation::HWND,
    pub rc: super::super::Foundation::RECT,
}
impl windows_core::TypeKind for SHELLHOOKINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for SHELLHOOKINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STYLESTRUCT {
    pub styleOld: u32,
    pub styleNew: u32,
}
impl windows_core::TypeKind for STYLESTRUCT {
    type TypeKind = windows_core::CopyType;
}
impl Default for STYLESTRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TITLEBARINFO {
    pub cbSize: u32,
    pub rcTitleBar: super::super::Foundation::RECT,
    pub rgstate: [u32; 6],
}
impl windows_core::TypeKind for TITLEBARINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for TITLEBARINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TITLEBARINFOEX {
    pub cbSize: u32,
    pub rcTitleBar: super::super::Foundation::RECT,
    pub rgstate: [u32; 6],
    pub rgrect: [super::super::Foundation::RECT; 6],
}
impl windows_core::TypeKind for TITLEBARINFOEX {
    type TypeKind = windows_core::CopyType;
}
impl Default for TITLEBARINFOEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TOUCHPREDICTIONPARAMETERS {
    pub cbSize: u32,
    pub dwLatency: u32,
    pub dwSampleTime: u32,
    pub bUseHWTimeStamp: u32,
}
impl windows_core::TypeKind for TOUCHPREDICTIONPARAMETERS {
    type TypeKind = windows_core::CopyType;
}
impl Default for TOUCHPREDICTIONPARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TPMPARAMS {
    pub cbSize: u32,
    pub rcExclude: super::super::Foundation::RECT,
}
impl windows_core::TypeKind for TPMPARAMS {
    type TypeKind = windows_core::CopyType;
}
impl Default for TPMPARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UPDATELAYEREDWINDOWINFO {
    pub cbSize: u32,
    pub hdcDst: super::super::Graphics::Gdi::HDC,
    pub pptDst: *const super::super::Foundation::POINT,
    pub psize: *const super::super::Foundation::SIZE,
    pub hdcSrc: super::super::Graphics::Gdi::HDC,
    pub pptSrc: *const super::super::Foundation::POINT,
    pub crKey: super::super::Foundation::COLORREF,
    pub pblend: *const super::super::Graphics::Gdi::BLENDFUNCTION,
    pub dwFlags: UPDATE_LAYERED_WINDOW_FLAGS,
    pub prcDirty: *const super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::TypeKind for UPDATELAYEREDWINDOWINFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for UPDATELAYEREDWINDOWINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VolLockBroadcast {
    pub vlb_dbh: DEV_BROADCAST_HDR,
    pub vlb_owner: u32,
    pub vlb_perms: u8,
    pub vlb_lockType: u8,
    pub vlb_drive: u8,
    pub vlb_flags: u8,
}
impl windows_core::TypeKind for VolLockBroadcast {
    type TypeKind = windows_core::CopyType;
}
impl Default for VolLockBroadcast {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WINDOWINFO {
    pub cbSize: u32,
    pub rcWindow: super::super::Foundation::RECT,
    pub rcClient: super::super::Foundation::RECT,
    pub dwStyle: WINDOW_STYLE,
    pub dwExStyle: WINDOW_EX_STYLE,
    pub dwWindowStatus: u32,
    pub cxWindowBorders: u32,
    pub cyWindowBorders: u32,
    pub atomWindowType: u16,
    pub wCreatorVersion: u16,
}
impl windows_core::TypeKind for WINDOWINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for WINDOWINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WINDOWPLACEMENT {
    pub length: u32,
    pub flags: WINDOWPLACEMENT_FLAGS,
    pub showCmd: u32,
    pub ptMinPosition: super::super::Foundation::POINT,
    pub ptMaxPosition: super::super::Foundation::POINT,
    pub rcNormalPosition: super::super::Foundation::RECT,
}
impl windows_core::TypeKind for WINDOWPLACEMENT {
    type TypeKind = windows_core::CopyType;
}
impl Default for WINDOWPLACEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WINDOWPOS {
    pub hwnd: super::super::Foundation::HWND,
    pub hwndInsertAfter: super::super::Foundation::HWND,
    pub x: i32,
    pub y: i32,
    pub cx: i32,
    pub cy: i32,
    pub flags: SET_WINDOW_POS_FLAGS,
}
impl windows_core::TypeKind for WINDOWPOS {
    type TypeKind = windows_core::CopyType;
}
impl Default for WINDOWPOS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug)]
pub struct WNDCLASSA {
    pub style: WNDCLASS_STYLES,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: i32,
    pub cbWndExtra: i32,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: super::super::Graphics::Gdi::HBRUSH,
    pub lpszMenuName: windows_core::PCSTR,
    pub lpszClassName: windows_core::PCSTR,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::TypeKind for WNDCLASSA {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for WNDCLASSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug)]
pub struct WNDCLASSEXA {
    pub cbSize: u32,
    pub style: WNDCLASS_STYLES,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: i32,
    pub cbWndExtra: i32,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: super::super::Graphics::Gdi::HBRUSH,
    pub lpszMenuName: windows_core::PCSTR,
    pub lpszClassName: windows_core::PCSTR,
    pub hIconSm: HICON,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::TypeKind for WNDCLASSEXA {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for WNDCLASSEXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug)]
pub struct WNDCLASSEXW {
    pub cbSize: u32,
    pub style: WNDCLASS_STYLES,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: i32,
    pub cbWndExtra: i32,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: super::super::Graphics::Gdi::HBRUSH,
    pub lpszMenuName: windows_core::PCWSTR,
    pub lpszClassName: windows_core::PCWSTR,
    pub hIconSm: HICON,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::TypeKind for WNDCLASSEXW {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for WNDCLASSEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug)]
pub struct WNDCLASSW {
    pub style: WNDCLASS_STYLES,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: i32,
    pub cbWndExtra: i32,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: super::super::Graphics::Gdi::HBRUSH,
    pub lpszMenuName: windows_core::PCWSTR,
    pub lpszClassName: windows_core::PCWSTR,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::TypeKind for WNDCLASSW {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for WNDCLASSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct _DEV_BROADCAST_HEADER {
    pub dbcd_size: u32,
    pub dbcd_devicetype: u32,
    pub dbcd_reserved: u32,
}
impl windows_core::TypeKind for _DEV_BROADCAST_HEADER {
    type TypeKind = windows_core::CopyType;
}
impl Default for _DEV_BROADCAST_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct _DEV_BROADCAST_USERDEFINED {
    pub dbud_dbh: DEV_BROADCAST_HDR,
    pub dbud_szName: [i8; 1],
}
impl windows_core::TypeKind for _DEV_BROADCAST_USERDEFINED {
    type TypeKind = windows_core::CopyType;
}
impl Default for _DEV_BROADCAST_USERDEFINED {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DLGPROC = Option<unsafe extern "system" fn(param0: super::super::Foundation::HWND, param1: u32, param2: super::super::Foundation::WPARAM, param3: super::super::Foundation::LPARAM) -> isize>;
pub type HOOKPROC = Option<unsafe extern "system" fn(code: i32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT>;
#[cfg(feature = "Win32_UI_Shell")]
pub type MSGBOXCALLBACK = Option<unsafe extern "system" fn(lphelpinfo: *mut super::Shell::HELPINFO)>;
pub type NAMEENUMPROCA = Option<unsafe extern "system" fn(param0: windows_core::PCSTR, param1: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL>;
pub type NAMEENUMPROCW = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL>;
pub type PREGISTERCLASSNAMEW = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR) -> super::super::Foundation::BOOLEAN>;
pub type PROPENUMPROCA = Option<unsafe extern "system" fn(param0: super::super::Foundation::HWND, param1: windows_core::PCSTR, param2: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL>;
pub type PROPENUMPROCEXA = Option<unsafe extern "system" fn(param0: super::super::Foundation::HWND, param1: windows_core::PCSTR, param2: super::super::Foundation::HANDLE, param3: usize) -> super::super::Foundation::BOOL>;
pub type PROPENUMPROCEXW = Option<unsafe extern "system" fn(param0: super::super::Foundation::HWND, param1: windows_core::PCWSTR, param2: super::super::Foundation::HANDLE, param3: usize) -> super::super::Foundation::BOOL>;
pub type PROPENUMPROCW = Option<unsafe extern "system" fn(param0: super::super::Foundation::HWND, param1: windows_core::PCWSTR, param2: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL>;
pub type SENDASYNCPROC = Option<unsafe extern "system" fn(param0: super::super::Foundation::HWND, param1: u32, param2: usize, param3: super::super::Foundation::LRESULT)>;
pub type TIMERPROC = Option<unsafe extern "system" fn(param0: super::super::Foundation::HWND, param1: u32, param2: usize, param3: u32)>;
pub type WNDENUMPROC = Option<unsafe extern "system" fn(param0: super::super::Foundation::HWND, param1: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL>;
pub type WNDPROC = Option<unsafe extern "system" fn(param0: super::super::Foundation::HWND, param1: u32, param2: super::super::Foundation::WPARAM, param3: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT>;
#[cfg(target_pointer_width = "32")]
pub use GetWindowLongA as GetWindowLongPtrA;
#[cfg(target_pointer_width = "32")]
pub use GetWindowLongW as GetWindowLongPtrW;
#[cfg(target_pointer_width = "32")]
pub use SetWindowLongA as SetWindowLongPtrA;
#[cfg(target_pointer_width = "32")]
pub use SetWindowLongW as SetWindowLongPtrW;
