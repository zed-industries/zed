#[cfg(feature = "Win32_UI_Input_Ime")]
pub mod Ime;
#[cfg(feature = "Win32_UI_Input_Ink")]
pub mod Ink;
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
pub mod KeyboardAndMouse;
#[cfg(feature = "Win32_UI_Input_Pointer")]
pub mod Pointer;
#[cfg(feature = "Win32_UI_Input_Radial")]
pub mod Radial;
#[cfg(feature = "Win32_UI_Input_Touch")]
pub mod Touch;
#[cfg(feature = "Win32_UI_Input_XboxController")]
pub mod XboxController;
#[inline]
pub unsafe fn DefRawInputProc(parawinput: &[*const RAWINPUT], cbsizeheader: u32) -> super::super::Foundation::LRESULT {
    windows_targets::link!("user32.dll" "system" fn DefRawInputProc(parawinput : *const *const RAWINPUT, ninput : i32, cbsizeheader : u32) -> super::super::Foundation:: LRESULT);
    DefRawInputProc(core::mem::transmute(parawinput.as_ptr()), parawinput.len().try_into().unwrap(), cbsizeheader)
}
#[inline]
pub unsafe fn GetCIMSSM(inputmessagesource: *mut INPUT_MESSAGE_SOURCE) -> super::super::Foundation::BOOL {
    windows_targets::link!("user32.dll" "system" fn GetCIMSSM(inputmessagesource : *mut INPUT_MESSAGE_SOURCE) -> super::super::Foundation:: BOOL);
    GetCIMSSM(inputmessagesource)
}
#[inline]
pub unsafe fn GetCurrentInputMessageSource(inputmessagesource: *mut INPUT_MESSAGE_SOURCE) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn GetCurrentInputMessageSource(inputmessagesource : *mut INPUT_MESSAGE_SOURCE) -> super::super::Foundation:: BOOL);
    GetCurrentInputMessageSource(inputmessagesource).ok()
}
#[inline]
pub unsafe fn GetRawInputBuffer(pdata: Option<*mut RAWINPUT>, pcbsize: *mut u32, cbsizeheader: u32) -> u32 {
    windows_targets::link!("user32.dll" "system" fn GetRawInputBuffer(pdata : *mut RAWINPUT, pcbsize : *mut u32, cbsizeheader : u32) -> u32);
    GetRawInputBuffer(core::mem::transmute(pdata.unwrap_or(std::ptr::null_mut())), pcbsize, cbsizeheader)
}
#[inline]
pub unsafe fn GetRawInputData<P0>(hrawinput: P0, uicommand: RAW_INPUT_DATA_COMMAND_FLAGS, pdata: Option<*mut core::ffi::c_void>, pcbsize: *mut u32, cbsizeheader: u32) -> u32
where
    P0: windows_core::Param<HRAWINPUT>,
{
    windows_targets::link!("user32.dll" "system" fn GetRawInputData(hrawinput : HRAWINPUT, uicommand : RAW_INPUT_DATA_COMMAND_FLAGS, pdata : *mut core::ffi::c_void, pcbsize : *mut u32, cbsizeheader : u32) -> u32);
    GetRawInputData(hrawinput.param().abi(), uicommand, core::mem::transmute(pdata.unwrap_or(std::ptr::null_mut())), pcbsize, cbsizeheader)
}
#[inline]
pub unsafe fn GetRawInputDeviceInfoA<P0>(hdevice: P0, uicommand: RAW_INPUT_DEVICE_INFO_COMMAND, pdata: Option<*mut core::ffi::c_void>, pcbsize: *mut u32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("user32.dll" "system" fn GetRawInputDeviceInfoA(hdevice : super::super::Foundation:: HANDLE, uicommand : RAW_INPUT_DEVICE_INFO_COMMAND, pdata : *mut core::ffi::c_void, pcbsize : *mut u32) -> u32);
    GetRawInputDeviceInfoA(hdevice.param().abi(), uicommand, core::mem::transmute(pdata.unwrap_or(std::ptr::null_mut())), pcbsize)
}
#[inline]
pub unsafe fn GetRawInputDeviceInfoW<P0>(hdevice: P0, uicommand: RAW_INPUT_DEVICE_INFO_COMMAND, pdata: Option<*mut core::ffi::c_void>, pcbsize: *mut u32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("user32.dll" "system" fn GetRawInputDeviceInfoW(hdevice : super::super::Foundation:: HANDLE, uicommand : RAW_INPUT_DEVICE_INFO_COMMAND, pdata : *mut core::ffi::c_void, pcbsize : *mut u32) -> u32);
    GetRawInputDeviceInfoW(hdevice.param().abi(), uicommand, core::mem::transmute(pdata.unwrap_or(std::ptr::null_mut())), pcbsize)
}
#[inline]
pub unsafe fn GetRawInputDeviceList(prawinputdevicelist: Option<*mut RAWINPUTDEVICELIST>, puinumdevices: *mut u32, cbsize: u32) -> u32 {
    windows_targets::link!("user32.dll" "system" fn GetRawInputDeviceList(prawinputdevicelist : *mut RAWINPUTDEVICELIST, puinumdevices : *mut u32, cbsize : u32) -> u32);
    GetRawInputDeviceList(core::mem::transmute(prawinputdevicelist.unwrap_or(std::ptr::null_mut())), puinumdevices, cbsize)
}
#[inline]
pub unsafe fn GetRegisteredRawInputDevices(prawinputdevices: Option<*mut RAWINPUTDEVICE>, puinumdevices: *mut u32, cbsize: u32) -> u32 {
    windows_targets::link!("user32.dll" "system" fn GetRegisteredRawInputDevices(prawinputdevices : *mut RAWINPUTDEVICE, puinumdevices : *mut u32, cbsize : u32) -> u32);
    GetRegisteredRawInputDevices(core::mem::transmute(prawinputdevices.unwrap_or(std::ptr::null_mut())), puinumdevices, cbsize)
}
#[inline]
pub unsafe fn RegisterRawInputDevices(prawinputdevices: &[RAWINPUTDEVICE], cbsize: u32) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn RegisterRawInputDevices(prawinputdevices : *const RAWINPUTDEVICE, uinumdevices : u32, cbsize : u32) -> super::super::Foundation:: BOOL);
    RegisterRawInputDevices(core::mem::transmute(prawinputdevices.as_ptr()), prawinputdevices.len().try_into().unwrap(), cbsize).ok()
}
pub const IMDT_KEYBOARD: INPUT_MESSAGE_DEVICE_TYPE = INPUT_MESSAGE_DEVICE_TYPE(1i32);
pub const IMDT_MOUSE: INPUT_MESSAGE_DEVICE_TYPE = INPUT_MESSAGE_DEVICE_TYPE(2i32);
pub const IMDT_PEN: INPUT_MESSAGE_DEVICE_TYPE = INPUT_MESSAGE_DEVICE_TYPE(8i32);
pub const IMDT_TOUCH: INPUT_MESSAGE_DEVICE_TYPE = INPUT_MESSAGE_DEVICE_TYPE(4i32);
pub const IMDT_TOUCHPAD: INPUT_MESSAGE_DEVICE_TYPE = INPUT_MESSAGE_DEVICE_TYPE(16i32);
pub const IMDT_UNAVAILABLE: INPUT_MESSAGE_DEVICE_TYPE = INPUT_MESSAGE_DEVICE_TYPE(0i32);
pub const IMO_HARDWARE: INPUT_MESSAGE_ORIGIN_ID = INPUT_MESSAGE_ORIGIN_ID(1i32);
pub const IMO_INJECTED: INPUT_MESSAGE_ORIGIN_ID = INPUT_MESSAGE_ORIGIN_ID(2i32);
pub const IMO_SYSTEM: INPUT_MESSAGE_ORIGIN_ID = INPUT_MESSAGE_ORIGIN_ID(4i32);
pub const IMO_UNAVAILABLE: INPUT_MESSAGE_ORIGIN_ID = INPUT_MESSAGE_ORIGIN_ID(0i32);
pub const MOUSE_ATTRIBUTES_CHANGED: MOUSE_STATE = MOUSE_STATE(4u16);
pub const MOUSE_MOVE_ABSOLUTE: MOUSE_STATE = MOUSE_STATE(1u16);
pub const MOUSE_MOVE_NOCOALESCE: MOUSE_STATE = MOUSE_STATE(8u16);
pub const MOUSE_MOVE_RELATIVE: MOUSE_STATE = MOUSE_STATE(0u16);
pub const MOUSE_VIRTUAL_DESKTOP: MOUSE_STATE = MOUSE_STATE(2u16);
pub const RIDEV_APPKEYS: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(1024u32);
pub const RIDEV_CAPTUREMOUSE: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(512u32);
pub const RIDEV_DEVNOTIFY: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(8192u32);
pub const RIDEV_EXCLUDE: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(16u32);
pub const RIDEV_EXINPUTSINK: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(4096u32);
pub const RIDEV_INPUTSINK: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(256u32);
pub const RIDEV_NOHOTKEYS: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(512u32);
pub const RIDEV_NOLEGACY: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(48u32);
pub const RIDEV_PAGEONLY: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(32u32);
pub const RIDEV_REMOVE: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(1u32);
pub const RIDI_DEVICEINFO: RAW_INPUT_DEVICE_INFO_COMMAND = RAW_INPUT_DEVICE_INFO_COMMAND(536870923u32);
pub const RIDI_DEVICENAME: RAW_INPUT_DEVICE_INFO_COMMAND = RAW_INPUT_DEVICE_INFO_COMMAND(536870919u32);
pub const RIDI_PREPARSEDDATA: RAW_INPUT_DEVICE_INFO_COMMAND = RAW_INPUT_DEVICE_INFO_COMMAND(536870917u32);
pub const RID_HEADER: RAW_INPUT_DATA_COMMAND_FLAGS = RAW_INPUT_DATA_COMMAND_FLAGS(268435461u32);
pub const RID_INPUT: RAW_INPUT_DATA_COMMAND_FLAGS = RAW_INPUT_DATA_COMMAND_FLAGS(268435459u32);
pub const RIM_TYPEHID: RID_DEVICE_INFO_TYPE = RID_DEVICE_INFO_TYPE(2u32);
pub const RIM_TYPEKEYBOARD: RID_DEVICE_INFO_TYPE = RID_DEVICE_INFO_TYPE(1u32);
pub const RIM_TYPEMOUSE: RID_DEVICE_INFO_TYPE = RID_DEVICE_INFO_TYPE(0u32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct INPUT_MESSAGE_DEVICE_TYPE(pub i32);
impl windows_core::TypeKind for INPUT_MESSAGE_DEVICE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for INPUT_MESSAGE_DEVICE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("INPUT_MESSAGE_DEVICE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct INPUT_MESSAGE_ORIGIN_ID(pub i32);
impl windows_core::TypeKind for INPUT_MESSAGE_ORIGIN_ID {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for INPUT_MESSAGE_ORIGIN_ID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("INPUT_MESSAGE_ORIGIN_ID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MOUSE_STATE(pub u16);
impl windows_core::TypeKind for MOUSE_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MOUSE_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MOUSE_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RAWINPUTDEVICE_FLAGS(pub u32);
impl windows_core::TypeKind for RAWINPUTDEVICE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RAWINPUTDEVICE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RAWINPUTDEVICE_FLAGS").field(&self.0).finish()
    }
}
impl RAWINPUTDEVICE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for RAWINPUTDEVICE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for RAWINPUTDEVICE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for RAWINPUTDEVICE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for RAWINPUTDEVICE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for RAWINPUTDEVICE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RAW_INPUT_DATA_COMMAND_FLAGS(pub u32);
impl windows_core::TypeKind for RAW_INPUT_DATA_COMMAND_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RAW_INPUT_DATA_COMMAND_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RAW_INPUT_DATA_COMMAND_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RAW_INPUT_DEVICE_INFO_COMMAND(pub u32);
impl windows_core::TypeKind for RAW_INPUT_DEVICE_INFO_COMMAND {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RAW_INPUT_DEVICE_INFO_COMMAND {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RAW_INPUT_DEVICE_INFO_COMMAND").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RID_DEVICE_INFO_TYPE(pub u32);
impl windows_core::TypeKind for RID_DEVICE_INFO_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RID_DEVICE_INFO_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RID_DEVICE_INFO_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HRAWINPUT(pub *mut core::ffi::c_void);
impl HRAWINPUT {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl Default for HRAWINPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HRAWINPUT {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct INPUT_MESSAGE_SOURCE {
    pub deviceType: INPUT_MESSAGE_DEVICE_TYPE,
    pub originId: INPUT_MESSAGE_ORIGIN_ID,
}
impl windows_core::TypeKind for INPUT_MESSAGE_SOURCE {
    type TypeKind = windows_core::CopyType;
}
impl Default for INPUT_MESSAGE_SOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RAWHID {
    pub dwSizeHid: u32,
    pub dwCount: u32,
    pub bRawData: [u8; 1],
}
impl windows_core::TypeKind for RAWHID {
    type TypeKind = windows_core::CopyType;
}
impl Default for RAWHID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RAWINPUT {
    pub header: RAWINPUTHEADER,
    pub data: RAWINPUT_0,
}
impl windows_core::TypeKind for RAWINPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for RAWINPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union RAWINPUT_0 {
    pub mouse: RAWMOUSE,
    pub keyboard: RAWKEYBOARD,
    pub hid: RAWHID,
}
impl windows_core::TypeKind for RAWINPUT_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for RAWINPUT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RAWINPUTDEVICE {
    pub usUsagePage: u16,
    pub usUsage: u16,
    pub dwFlags: RAWINPUTDEVICE_FLAGS,
    pub hwndTarget: super::super::Foundation::HWND,
}
impl windows_core::TypeKind for RAWINPUTDEVICE {
    type TypeKind = windows_core::CopyType;
}
impl Default for RAWINPUTDEVICE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RAWINPUTDEVICELIST {
    pub hDevice: super::super::Foundation::HANDLE,
    pub dwType: RID_DEVICE_INFO_TYPE,
}
impl windows_core::TypeKind for RAWINPUTDEVICELIST {
    type TypeKind = windows_core::CopyType;
}
impl Default for RAWINPUTDEVICELIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RAWINPUTHEADER {
    pub dwType: u32,
    pub dwSize: u32,
    pub hDevice: super::super::Foundation::HANDLE,
    pub wParam: super::super::Foundation::WPARAM,
}
impl windows_core::TypeKind for RAWINPUTHEADER {
    type TypeKind = windows_core::CopyType;
}
impl Default for RAWINPUTHEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RAWKEYBOARD {
    pub MakeCode: u16,
    pub Flags: u16,
    pub Reserved: u16,
    pub VKey: u16,
    pub Message: u32,
    pub ExtraInformation: u32,
}
impl windows_core::TypeKind for RAWKEYBOARD {
    type TypeKind = windows_core::CopyType;
}
impl Default for RAWKEYBOARD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RAWMOUSE {
    pub usFlags: MOUSE_STATE,
    pub Anonymous: RAWMOUSE_0,
    pub ulRawButtons: u32,
    pub lLastX: i32,
    pub lLastY: i32,
    pub ulExtraInformation: u32,
}
impl windows_core::TypeKind for RAWMOUSE {
    type TypeKind = windows_core::CopyType;
}
impl Default for RAWMOUSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union RAWMOUSE_0 {
    pub ulButtons: u32,
    pub Anonymous: RAWMOUSE_0_0,
}
impl windows_core::TypeKind for RAWMOUSE_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for RAWMOUSE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RAWMOUSE_0_0 {
    pub usButtonFlags: u16,
    pub usButtonData: u16,
}
impl windows_core::TypeKind for RAWMOUSE_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for RAWMOUSE_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RID_DEVICE_INFO {
    pub cbSize: u32,
    pub dwType: RID_DEVICE_INFO_TYPE,
    pub Anonymous: RID_DEVICE_INFO_0,
}
impl windows_core::TypeKind for RID_DEVICE_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for RID_DEVICE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union RID_DEVICE_INFO_0 {
    pub mouse: RID_DEVICE_INFO_MOUSE,
    pub keyboard: RID_DEVICE_INFO_KEYBOARD,
    pub hid: RID_DEVICE_INFO_HID,
}
impl windows_core::TypeKind for RID_DEVICE_INFO_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for RID_DEVICE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RID_DEVICE_INFO_HID {
    pub dwVendorId: u32,
    pub dwProductId: u32,
    pub dwVersionNumber: u32,
    pub usUsagePage: u16,
    pub usUsage: u16,
}
impl windows_core::TypeKind for RID_DEVICE_INFO_HID {
    type TypeKind = windows_core::CopyType;
}
impl Default for RID_DEVICE_INFO_HID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RID_DEVICE_INFO_KEYBOARD {
    pub dwType: u32,
    pub dwSubType: u32,
    pub dwKeyboardMode: u32,
    pub dwNumberOfFunctionKeys: u32,
    pub dwNumberOfIndicators: u32,
    pub dwNumberOfKeysTotal: u32,
}
impl windows_core::TypeKind for RID_DEVICE_INFO_KEYBOARD {
    type TypeKind = windows_core::CopyType;
}
impl Default for RID_DEVICE_INFO_KEYBOARD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RID_DEVICE_INFO_MOUSE {
    pub dwId: u32,
    pub dwNumberOfButtons: u32,
    pub dwSampleRate: u32,
    pub fHasHorizontalWheel: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for RID_DEVICE_INFO_MOUSE {
    type TypeKind = windows_core::CopyType;
}
impl Default for RID_DEVICE_INFO_MOUSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
