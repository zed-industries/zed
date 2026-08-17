#[inline]
pub unsafe fn BuildCommDCBA<P0>(lpdef: P0, lpdcb: *mut DCB) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn BuildCommDCBA(lpdef : windows_core::PCSTR, lpdcb : *mut DCB) -> windows_core::BOOL);
    unsafe { BuildCommDCBA(lpdef.param().abi(), lpdcb as _).ok() }
}
#[inline]
pub unsafe fn BuildCommDCBAndTimeoutsA<P0>(lpdef: P0, lpdcb: *mut DCB, lpcommtimeouts: *mut COMMTIMEOUTS) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn BuildCommDCBAndTimeoutsA(lpdef : windows_core::PCSTR, lpdcb : *mut DCB, lpcommtimeouts : *mut COMMTIMEOUTS) -> windows_core::BOOL);
    unsafe { BuildCommDCBAndTimeoutsA(lpdef.param().abi(), lpdcb as _, lpcommtimeouts as _).ok() }
}
#[inline]
pub unsafe fn BuildCommDCBAndTimeoutsW<P0>(lpdef: P0, lpdcb: *mut DCB, lpcommtimeouts: *mut COMMTIMEOUTS) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn BuildCommDCBAndTimeoutsW(lpdef : windows_core::PCWSTR, lpdcb : *mut DCB, lpcommtimeouts : *mut COMMTIMEOUTS) -> windows_core::BOOL);
    unsafe { BuildCommDCBAndTimeoutsW(lpdef.param().abi(), lpdcb as _, lpcommtimeouts as _).ok() }
}
#[inline]
pub unsafe fn BuildCommDCBW<P0>(lpdef: P0, lpdcb: *mut DCB) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn BuildCommDCBW(lpdef : windows_core::PCWSTR, lpdcb : *mut DCB) -> windows_core::BOOL);
    unsafe { BuildCommDCBW(lpdef.param().abi(), lpdcb as _).ok() }
}
#[inline]
pub unsafe fn ClearCommBreak(hfile: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn ClearCommBreak(hfile : super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { ClearCommBreak(hfile).ok() }
}
#[inline]
pub unsafe fn ClearCommError(hfile: super::super::Foundation::HANDLE, lperrors: Option<*mut CLEAR_COMM_ERROR_FLAGS>, lpstat: Option<*mut COMSTAT>) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn ClearCommError(hfile : super::super::Foundation:: HANDLE, lperrors : *mut CLEAR_COMM_ERROR_FLAGS, lpstat : *mut COMSTAT) -> windows_core::BOOL);
    unsafe { ClearCommError(hfile, lperrors.unwrap_or(core::mem::zeroed()) as _, lpstat.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CommConfigDialogA<P0>(lpszname: P0, hwnd: Option<super::super::Foundation::HWND>, lpcc: *mut COMMCONFIG) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CommConfigDialogA(lpszname : windows_core::PCSTR, hwnd : super::super::Foundation:: HWND, lpcc : *mut COMMCONFIG) -> windows_core::BOOL);
    unsafe { CommConfigDialogA(lpszname.param().abi(), hwnd.unwrap_or(core::mem::zeroed()) as _, lpcc as _).ok() }
}
#[inline]
pub unsafe fn CommConfigDialogW<P0>(lpszname: P0, hwnd: Option<super::super::Foundation::HWND>, lpcc: *mut COMMCONFIG) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CommConfigDialogW(lpszname : windows_core::PCWSTR, hwnd : super::super::Foundation:: HWND, lpcc : *mut COMMCONFIG) -> windows_core::BOOL);
    unsafe { CommConfigDialogW(lpszname.param().abi(), hwnd.unwrap_or(core::mem::zeroed()) as _, lpcc as _).ok() }
}
#[inline]
pub unsafe fn EscapeCommFunction(hfile: super::super::Foundation::HANDLE, dwfunc: ESCAPE_COMM_FUNCTION) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn EscapeCommFunction(hfile : super::super::Foundation:: HANDLE, dwfunc : ESCAPE_COMM_FUNCTION) -> windows_core::BOOL);
    unsafe { EscapeCommFunction(hfile, dwfunc).ok() }
}
#[inline]
pub unsafe fn GetCommConfig(hcommdev: super::super::Foundation::HANDLE, lpcc: Option<*mut COMMCONFIG>, lpdwsize: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn GetCommConfig(hcommdev : super::super::Foundation:: HANDLE, lpcc : *mut COMMCONFIG, lpdwsize : *mut u32) -> windows_core::BOOL);
    unsafe { GetCommConfig(hcommdev, lpcc.unwrap_or(core::mem::zeroed()) as _, lpdwsize as _).ok() }
}
#[inline]
pub unsafe fn GetCommMask(hfile: super::super::Foundation::HANDLE, lpevtmask: *mut COMM_EVENT_MASK) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn GetCommMask(hfile : super::super::Foundation:: HANDLE, lpevtmask : *mut COMM_EVENT_MASK) -> windows_core::BOOL);
    unsafe { GetCommMask(hfile, lpevtmask as _).ok() }
}
#[inline]
pub unsafe fn GetCommModemStatus(hfile: super::super::Foundation::HANDLE, lpmodemstat: *mut MODEM_STATUS_FLAGS) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn GetCommModemStatus(hfile : super::super::Foundation:: HANDLE, lpmodemstat : *mut MODEM_STATUS_FLAGS) -> windows_core::BOOL);
    unsafe { GetCommModemStatus(hfile, lpmodemstat as _).ok() }
}
#[inline]
pub unsafe fn GetCommPorts(lpportnumbers: &mut [u32], puportnumbersfound: *mut u32) -> u32 {
    windows_core::link!("api-ms-win-core-comm-l1-1-2.dll" "system" fn GetCommPorts(lpportnumbers : *mut u32, uportnumberscount : u32, puportnumbersfound : *mut u32) -> u32);
    unsafe { GetCommPorts(core::mem::transmute(lpportnumbers.as_ptr()), lpportnumbers.len().try_into().unwrap(), puportnumbersfound as _) }
}
#[inline]
pub unsafe fn GetCommProperties(hfile: super::super::Foundation::HANDLE, lpcommprop: *mut COMMPROP) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn GetCommProperties(hfile : super::super::Foundation:: HANDLE, lpcommprop : *mut COMMPROP) -> windows_core::BOOL);
    unsafe { GetCommProperties(hfile, lpcommprop as _).ok() }
}
#[inline]
pub unsafe fn GetCommState(hfile: super::super::Foundation::HANDLE, lpdcb: *mut DCB) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn GetCommState(hfile : super::super::Foundation:: HANDLE, lpdcb : *mut DCB) -> windows_core::BOOL);
    unsafe { GetCommState(hfile, lpdcb as _).ok() }
}
#[inline]
pub unsafe fn GetCommTimeouts(hfile: super::super::Foundation::HANDLE, lpcommtimeouts: *mut COMMTIMEOUTS) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn GetCommTimeouts(hfile : super::super::Foundation:: HANDLE, lpcommtimeouts : *mut COMMTIMEOUTS) -> windows_core::BOOL);
    unsafe { GetCommTimeouts(hfile, lpcommtimeouts as _).ok() }
}
#[inline]
pub unsafe fn GetDefaultCommConfigA<P0>(lpszname: P0, lpcc: *mut COMMCONFIG, lpdwsize: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetDefaultCommConfigA(lpszname : windows_core::PCSTR, lpcc : *mut COMMCONFIG, lpdwsize : *mut u32) -> windows_core::BOOL);
    unsafe { GetDefaultCommConfigA(lpszname.param().abi(), lpcc as _, lpdwsize as _).ok() }
}
#[inline]
pub unsafe fn GetDefaultCommConfigW<P0>(lpszname: P0, lpcc: *mut COMMCONFIG, lpdwsize: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetDefaultCommConfigW(lpszname : windows_core::PCWSTR, lpcc : *mut COMMCONFIG, lpdwsize : *mut u32) -> windows_core::BOOL);
    unsafe { GetDefaultCommConfigW(lpszname.param().abi(), lpcc as _, lpdwsize as _).ok() }
}
#[inline]
pub unsafe fn OpenCommPort(uportnumber: u32, dwdesiredaccess: u32, dwflagsandattributes: u32) -> super::super::Foundation::HANDLE {
    windows_core::link!("api-ms-win-core-comm-l1-1-1.dll" "system" fn OpenCommPort(uportnumber : u32, dwdesiredaccess : u32, dwflagsandattributes : u32) -> super::super::Foundation:: HANDLE);
    unsafe { OpenCommPort(uportnumber, dwdesiredaccess, dwflagsandattributes) }
}
#[inline]
pub unsafe fn PurgeComm(hfile: super::super::Foundation::HANDLE, dwflags: PURGE_COMM_FLAGS) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn PurgeComm(hfile : super::super::Foundation:: HANDLE, dwflags : PURGE_COMM_FLAGS) -> windows_core::BOOL);
    unsafe { PurgeComm(hfile, dwflags).ok() }
}
#[inline]
pub unsafe fn SetCommBreak(hfile: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn SetCommBreak(hfile : super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { SetCommBreak(hfile).ok() }
}
#[inline]
pub unsafe fn SetCommConfig(hcommdev: super::super::Foundation::HANDLE, lpcc: *const COMMCONFIG, dwsize: u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn SetCommConfig(hcommdev : super::super::Foundation:: HANDLE, lpcc : *const COMMCONFIG, dwsize : u32) -> windows_core::BOOL);
    unsafe { SetCommConfig(hcommdev, lpcc, dwsize).ok() }
}
#[inline]
pub unsafe fn SetCommMask(hfile: super::super::Foundation::HANDLE, dwevtmask: COMM_EVENT_MASK) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn SetCommMask(hfile : super::super::Foundation:: HANDLE, dwevtmask : COMM_EVENT_MASK) -> windows_core::BOOL);
    unsafe { SetCommMask(hfile, dwevtmask).ok() }
}
#[inline]
pub unsafe fn SetCommState(hfile: super::super::Foundation::HANDLE, lpdcb: *const DCB) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn SetCommState(hfile : super::super::Foundation:: HANDLE, lpdcb : *const DCB) -> windows_core::BOOL);
    unsafe { SetCommState(hfile, lpdcb).ok() }
}
#[inline]
pub unsafe fn SetCommTimeouts(hfile: super::super::Foundation::HANDLE, lpcommtimeouts: *const COMMTIMEOUTS) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn SetCommTimeouts(hfile : super::super::Foundation:: HANDLE, lpcommtimeouts : *const COMMTIMEOUTS) -> windows_core::BOOL);
    unsafe { SetCommTimeouts(hfile, lpcommtimeouts).ok() }
}
#[inline]
pub unsafe fn SetDefaultCommConfigA<P0>(lpszname: P0, lpcc: *const COMMCONFIG, dwsize: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetDefaultCommConfigA(lpszname : windows_core::PCSTR, lpcc : *const COMMCONFIG, dwsize : u32) -> windows_core::BOOL);
    unsafe { SetDefaultCommConfigA(lpszname.param().abi(), lpcc, dwsize).ok() }
}
#[inline]
pub unsafe fn SetDefaultCommConfigW<P0>(lpszname: P0, lpcc: *const COMMCONFIG, dwsize: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetDefaultCommConfigW(lpszname : windows_core::PCWSTR, lpcc : *const COMMCONFIG, dwsize : u32) -> windows_core::BOOL);
    unsafe { SetDefaultCommConfigW(lpszname.param().abi(), lpcc, dwsize).ok() }
}
#[inline]
pub unsafe fn SetupComm(hfile: super::super::Foundation::HANDLE, dwinqueue: u32, dwoutqueue: u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn SetupComm(hfile : super::super::Foundation:: HANDLE, dwinqueue : u32, dwoutqueue : u32) -> windows_core::BOOL);
    unsafe { SetupComm(hfile, dwinqueue, dwoutqueue).ok() }
}
#[inline]
pub unsafe fn TransmitCommChar(hfile: super::super::Foundation::HANDLE, cchar: i8) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn TransmitCommChar(hfile : super::super::Foundation:: HANDLE, cchar : i8) -> windows_core::BOOL);
    unsafe { TransmitCommChar(hfile, cchar).ok() }
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn WaitCommEvent(hfile: super::super::Foundation::HANDLE, lpevtmask: *mut COMM_EVENT_MASK, lpoverlapped: Option<*mut super::super::System::IO::OVERLAPPED>) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn WaitCommEvent(hfile : super::super::Foundation:: HANDLE, lpevtmask : *mut COMM_EVENT_MASK, lpoverlapped : *mut super::super::System::IO:: OVERLAPPED) -> windows_core::BOOL);
    unsafe { WaitCommEvent(hfile, lpevtmask as _, lpoverlapped.unwrap_or(core::mem::zeroed()) as _).ok() }
}
pub const CE_BREAK: CLEAR_COMM_ERROR_FLAGS = CLEAR_COMM_ERROR_FLAGS(16u32);
pub const CE_FRAME: CLEAR_COMM_ERROR_FLAGS = CLEAR_COMM_ERROR_FLAGS(8u32);
pub const CE_OVERRUN: CLEAR_COMM_ERROR_FLAGS = CLEAR_COMM_ERROR_FLAGS(2u32);
pub const CE_RXOVER: CLEAR_COMM_ERROR_FLAGS = CLEAR_COMM_ERROR_FLAGS(1u32);
pub const CE_RXPARITY: CLEAR_COMM_ERROR_FLAGS = CLEAR_COMM_ERROR_FLAGS(4u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLEAR_COMM_ERROR_FLAGS(pub u32);
impl CLEAR_COMM_ERROR_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CLEAR_COMM_ERROR_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CLEAR_COMM_ERROR_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CLEAR_COMM_ERROR_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CLEAR_COMM_ERROR_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CLEAR_COMM_ERROR_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CLRBREAK: ESCAPE_COMM_FUNCTION = ESCAPE_COMM_FUNCTION(9u32);
pub const CLRDTR: ESCAPE_COMM_FUNCTION = ESCAPE_COMM_FUNCTION(6u32);
pub const CLRRTS: ESCAPE_COMM_FUNCTION = ESCAPE_COMM_FUNCTION(4u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct COMMCONFIG {
    pub dwSize: u32,
    pub wVersion: u16,
    pub wReserved: u16,
    pub dcb: DCB,
    pub dwProviderSubType: u32,
    pub dwProviderOffset: u32,
    pub dwProviderSize: u32,
    pub wcProviderData: [u16; 1],
}
impl Default for COMMCONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct COMMPROP {
    pub wPacketLength: u16,
    pub wPacketVersion: u16,
    pub dwServiceMask: u32,
    pub dwReserved1: u32,
    pub dwMaxTxQueue: u32,
    pub dwMaxRxQueue: u32,
    pub dwMaxBaud: u32,
    pub dwProvSubType: u32,
    pub dwProvCapabilities: u32,
    pub dwSettableParams: u32,
    pub dwSettableBaud: u32,
    pub wSettableData: u16,
    pub wSettableStopParity: COMMPROP_STOP_PARITY,
    pub dwCurrentTxQueue: u32,
    pub dwCurrentRxQueue: u32,
    pub dwProvSpec1: u32,
    pub dwProvSpec2: u32,
    pub wcProvChar: [u16; 1],
}
impl Default for COMMPROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COMMPROP_STOP_PARITY(pub u16);
impl COMMPROP_STOP_PARITY {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for COMMPROP_STOP_PARITY {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for COMMPROP_STOP_PARITY {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for COMMPROP_STOP_PARITY {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for COMMPROP_STOP_PARITY {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for COMMPROP_STOP_PARITY {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct COMMTIMEOUTS {
    pub ReadIntervalTimeout: u32,
    pub ReadTotalTimeoutMultiplier: u32,
    pub ReadTotalTimeoutConstant: u32,
    pub WriteTotalTimeoutMultiplier: u32,
    pub WriteTotalTimeoutConstant: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COMM_EVENT_MASK(pub u32);
impl COMM_EVENT_MASK {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for COMM_EVENT_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for COMM_EVENT_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for COMM_EVENT_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for COMM_EVENT_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for COMM_EVENT_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct COMSTAT {
    pub _bitfield: u32,
    pub cbInQue: u32,
    pub cbOutQue: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DCB {
    pub DCBlength: u32,
    pub BaudRate: u32,
    pub _bitfield: u32,
    pub wReserved: u16,
    pub XonLim: u16,
    pub XoffLim: u16,
    pub ByteSize: u8,
    pub Parity: DCB_PARITY,
    pub StopBits: DCB_STOP_BITS,
    pub XonChar: i8,
    pub XoffChar: i8,
    pub ErrorChar: i8,
    pub EofChar: i8,
    pub EvtChar: i8,
    pub wReserved1: u16,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DCB_PARITY(pub u8);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DCB_STOP_BITS(pub u8);
pub const DIALOPTION_BILLING: MODEMDEVCAPS_DIAL_OPTIONS = MODEMDEVCAPS_DIAL_OPTIONS(64u32);
pub const DIALOPTION_DIALTONE: MODEMDEVCAPS_DIAL_OPTIONS = MODEMDEVCAPS_DIAL_OPTIONS(256u32);
pub const DIALOPTION_QUIET: MODEMDEVCAPS_DIAL_OPTIONS = MODEMDEVCAPS_DIAL_OPTIONS(128u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ESCAPE_COMM_FUNCTION(pub u32);
pub const EVENPARITY: DCB_PARITY = DCB_PARITY(2u8);
pub const EV_BREAK: COMM_EVENT_MASK = COMM_EVENT_MASK(64u32);
pub const EV_CTS: COMM_EVENT_MASK = COMM_EVENT_MASK(8u32);
pub const EV_DSR: COMM_EVENT_MASK = COMM_EVENT_MASK(16u32);
pub const EV_ERR: COMM_EVENT_MASK = COMM_EVENT_MASK(128u32);
pub const EV_EVENT1: COMM_EVENT_MASK = COMM_EVENT_MASK(2048u32);
pub const EV_EVENT2: COMM_EVENT_MASK = COMM_EVENT_MASK(4096u32);
pub const EV_PERR: COMM_EVENT_MASK = COMM_EVENT_MASK(512u32);
pub const EV_RING: COMM_EVENT_MASK = COMM_EVENT_MASK(256u32);
pub const EV_RLSD: COMM_EVENT_MASK = COMM_EVENT_MASK(32u32);
pub const EV_RX80FULL: COMM_EVENT_MASK = COMM_EVENT_MASK(1024u32);
pub const EV_RXCHAR: COMM_EVENT_MASK = COMM_EVENT_MASK(1u32);
pub const EV_RXFLAG: COMM_EVENT_MASK = COMM_EVENT_MASK(2u32);
pub const EV_TXEMPTY: COMM_EVENT_MASK = COMM_EVENT_MASK(4u32);
pub const MARKPARITY: DCB_PARITY = DCB_PARITY(3u8);
pub const MAXLENGTH_NAI: u32 = 72u32;
pub const MAXLENGTH_UICCDATASTORE: u32 = 10u32;
pub const MDMSPKRFLAG_CALLSETUP: MODEMDEVCAPS_SPEAKER_MODE = MODEMDEVCAPS_SPEAKER_MODE(8u32);
pub const MDMSPKRFLAG_DIAL: MODEMDEVCAPS_SPEAKER_MODE = MODEMDEVCAPS_SPEAKER_MODE(2u32);
pub const MDMSPKRFLAG_OFF: MODEMDEVCAPS_SPEAKER_MODE = MODEMDEVCAPS_SPEAKER_MODE(1u32);
pub const MDMSPKRFLAG_ON: MODEMDEVCAPS_SPEAKER_MODE = MODEMDEVCAPS_SPEAKER_MODE(4u32);
pub const MDMSPKR_CALLSETUP: MODEMSETTINGS_SPEAKER_MODE = MODEMSETTINGS_SPEAKER_MODE(8u32);
pub const MDMSPKR_DIAL: MODEMSETTINGS_SPEAKER_MODE = MODEMSETTINGS_SPEAKER_MODE(2u32);
pub const MDMSPKR_OFF: MODEMSETTINGS_SPEAKER_MODE = MODEMSETTINGS_SPEAKER_MODE(1u32);
pub const MDMSPKR_ON: MODEMSETTINGS_SPEAKER_MODE = MODEMSETTINGS_SPEAKER_MODE(4u32);
pub const MDMVOLFLAG_HIGH: MODEMDEVCAPS_SPEAKER_VOLUME = MODEMDEVCAPS_SPEAKER_VOLUME(4u32);
pub const MDMVOLFLAG_LOW: MODEMDEVCAPS_SPEAKER_VOLUME = MODEMDEVCAPS_SPEAKER_VOLUME(1u32);
pub const MDMVOLFLAG_MEDIUM: MODEMDEVCAPS_SPEAKER_VOLUME = MODEMDEVCAPS_SPEAKER_VOLUME(2u32);
pub const MDMVOL_HIGH: MODEM_SPEAKER_VOLUME = MODEM_SPEAKER_VOLUME(2u32);
pub const MDMVOL_LOW: MODEM_SPEAKER_VOLUME = MODEM_SPEAKER_VOLUME(0u32);
pub const MDMVOL_MEDIUM: MODEM_SPEAKER_VOLUME = MODEM_SPEAKER_VOLUME(1u32);
pub const MDM_ANALOG_RLP_OFF: u32 = 1u32;
pub const MDM_ANALOG_RLP_ON: u32 = 0u32;
pub const MDM_ANALOG_V34: u32 = 2u32;
pub const MDM_AUTO_ML_2: u32 = 2u32;
pub const MDM_AUTO_ML_DEFAULT: u32 = 0u32;
pub const MDM_AUTO_ML_NONE: u32 = 1u32;
pub const MDM_AUTO_SPEED_DEFAULT: u32 = 0u32;
pub const MDM_BEARERMODE_ANALOG: u32 = 0u32;
pub const MDM_BEARERMODE_GSM: u32 = 2u32;
pub const MDM_BEARERMODE_ISDN: u32 = 1u32;
pub const MDM_BLIND_DIAL: u32 = 512u32;
pub const MDM_CCITT_OVERRIDE: u32 = 64u32;
pub const MDM_CELLULAR: u32 = 8u32;
pub const MDM_COMPRESSION: u32 = 1u32;
pub const MDM_DIAGNOSTICS: u32 = 2048u32;
pub const MDM_ERROR_CONTROL: u32 = 2u32;
pub const MDM_FLOWCONTROL_HARD: u32 = 16u32;
pub const MDM_FLOWCONTROL_SOFT: u32 = 32u32;
pub const MDM_FORCED_EC: u32 = 4u32;
pub const MDM_HDLCPPP_AUTH_CHAP: u32 = 3u32;
pub const MDM_HDLCPPP_AUTH_DEFAULT: u32 = 0u32;
pub const MDM_HDLCPPP_AUTH_MSCHAP: u32 = 4u32;
pub const MDM_HDLCPPP_AUTH_NONE: u32 = 1u32;
pub const MDM_HDLCPPP_AUTH_PAP: u32 = 2u32;
pub const MDM_HDLCPPP_ML_2: u32 = 2u32;
pub const MDM_HDLCPPP_ML_DEFAULT: u32 = 0u32;
pub const MDM_HDLCPPP_ML_NONE: u32 = 1u32;
pub const MDM_HDLCPPP_SPEED_56K: u32 = 2u32;
pub const MDM_HDLCPPP_SPEED_64K: u32 = 1u32;
pub const MDM_HDLCPPP_SPEED_DEFAULT: u32 = 0u32;
pub const MDM_MASK_AUTO_SPEED: u32 = 7u32;
pub const MDM_MASK_BEARERMODE: u32 = 61440u32;
pub const MDM_MASK_HDLCPPP_SPEED: u32 = 7u32;
pub const MDM_MASK_PROTOCOLDATA: u32 = 267386880u32;
pub const MDM_MASK_PROTOCOLID: u32 = 983040u32;
pub const MDM_MASK_V110_SPEED: u32 = 15u32;
pub const MDM_MASK_V120_SPEED: u32 = 7u32;
pub const MDM_MASK_X75_DATA: u32 = 7u32;
pub const MDM_PIAFS_INCOMING: u32 = 0u32;
pub const MDM_PIAFS_OUTGOING: u32 = 1u32;
pub const MDM_PROTOCOLID_ANALOG: u32 = 7u32;
pub const MDM_PROTOCOLID_AUTO: u32 = 6u32;
pub const MDM_PROTOCOLID_DEFAULT: u32 = 0u32;
pub const MDM_PROTOCOLID_GPRS: u32 = 8u32;
pub const MDM_PROTOCOLID_HDLCPPP: u32 = 1u32;
pub const MDM_PROTOCOLID_PIAFS: u32 = 9u32;
pub const MDM_PROTOCOLID_V110: u32 = 4u32;
pub const MDM_PROTOCOLID_V120: u32 = 5u32;
pub const MDM_PROTOCOLID_V128: u32 = 2u32;
pub const MDM_PROTOCOLID_X75: u32 = 3u32;
pub const MDM_SHIFT_AUTO_ML: u32 = 6u32;
pub const MDM_SHIFT_AUTO_SPEED: u32 = 0u32;
pub const MDM_SHIFT_BEARERMODE: u32 = 12u32;
pub const MDM_SHIFT_EXTENDEDINFO: u32 = 12u32;
pub const MDM_SHIFT_HDLCPPP_AUTH: u32 = 3u32;
pub const MDM_SHIFT_HDLCPPP_ML: u32 = 6u32;
pub const MDM_SHIFT_HDLCPPP_SPEED: u32 = 0u32;
pub const MDM_SHIFT_PROTOCOLDATA: u32 = 20u32;
pub const MDM_SHIFT_PROTOCOLID: u32 = 16u32;
pub const MDM_SHIFT_PROTOCOLINFO: u32 = 16u32;
pub const MDM_SHIFT_V110_SPEED: u32 = 0u32;
pub const MDM_SHIFT_V120_ML: u32 = 6u32;
pub const MDM_SHIFT_V120_SPEED: u32 = 0u32;
pub const MDM_SHIFT_X75_DATA: u32 = 0u32;
pub const MDM_SPEED_ADJUST: u32 = 128u32;
pub const MDM_TONE_DIAL: u32 = 256u32;
pub const MDM_V110_SPEED_12DOT0K: u32 = 5u32;
pub const MDM_V110_SPEED_14DOT4K: u32 = 6u32;
pub const MDM_V110_SPEED_19DOT2K: u32 = 7u32;
pub const MDM_V110_SPEED_1DOT2K: u32 = 1u32;
pub const MDM_V110_SPEED_28DOT8K: u32 = 8u32;
pub const MDM_V110_SPEED_2DOT4K: u32 = 2u32;
pub const MDM_V110_SPEED_38DOT4K: u32 = 9u32;
pub const MDM_V110_SPEED_4DOT8K: u32 = 3u32;
pub const MDM_V110_SPEED_57DOT6K: u32 = 10u32;
pub const MDM_V110_SPEED_9DOT6K: u32 = 4u32;
pub const MDM_V110_SPEED_DEFAULT: u32 = 0u32;
pub const MDM_V120_ML_2: u32 = 2u32;
pub const MDM_V120_ML_DEFAULT: u32 = 0u32;
pub const MDM_V120_ML_NONE: u32 = 1u32;
pub const MDM_V120_SPEED_56K: u32 = 2u32;
pub const MDM_V120_SPEED_64K: u32 = 1u32;
pub const MDM_V120_SPEED_DEFAULT: u32 = 0u32;
pub const MDM_V23_OVERRIDE: u32 = 1024u32;
pub const MDM_X75_DATA_128K: u32 = 2u32;
pub const MDM_X75_DATA_64K: u32 = 1u32;
pub const MDM_X75_DATA_BTX: u32 = 4u32;
pub const MDM_X75_DATA_DEFAULT: u32 = 0u32;
pub const MDM_X75_DATA_T_70: u32 = 3u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MODEMDEVCAPS {
    pub dwActualSize: u32,
    pub dwRequiredSize: u32,
    pub dwDevSpecificOffset: u32,
    pub dwDevSpecificSize: u32,
    pub dwModemProviderVersion: u32,
    pub dwModemManufacturerOffset: u32,
    pub dwModemManufacturerSize: u32,
    pub dwModemModelOffset: u32,
    pub dwModemModelSize: u32,
    pub dwModemVersionOffset: u32,
    pub dwModemVersionSize: u32,
    pub dwDialOptions: MODEMDEVCAPS_DIAL_OPTIONS,
    pub dwCallSetupFailTimer: u32,
    pub dwInactivityTimeout: u32,
    pub dwSpeakerVolume: MODEMDEVCAPS_SPEAKER_VOLUME,
    pub dwSpeakerMode: MODEMDEVCAPS_SPEAKER_MODE,
    pub dwModemOptions: u32,
    pub dwMaxDTERate: u32,
    pub dwMaxDCERate: u32,
    pub abVariablePortion: [u8; 1],
}
impl Default for MODEMDEVCAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MODEMDEVCAPS_DIAL_OPTIONS(pub u32);
impl MODEMDEVCAPS_DIAL_OPTIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for MODEMDEVCAPS_DIAL_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for MODEMDEVCAPS_DIAL_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for MODEMDEVCAPS_DIAL_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for MODEMDEVCAPS_DIAL_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for MODEMDEVCAPS_DIAL_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MODEMDEVCAPS_SPEAKER_MODE(pub u32);
impl MODEMDEVCAPS_SPEAKER_MODE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for MODEMDEVCAPS_SPEAKER_MODE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for MODEMDEVCAPS_SPEAKER_MODE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for MODEMDEVCAPS_SPEAKER_MODE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for MODEMDEVCAPS_SPEAKER_MODE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for MODEMDEVCAPS_SPEAKER_MODE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MODEMDEVCAPS_SPEAKER_VOLUME(pub u32);
impl MODEMDEVCAPS_SPEAKER_VOLUME {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for MODEMDEVCAPS_SPEAKER_VOLUME {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for MODEMDEVCAPS_SPEAKER_VOLUME {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for MODEMDEVCAPS_SPEAKER_VOLUME {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for MODEMDEVCAPS_SPEAKER_VOLUME {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for MODEMDEVCAPS_SPEAKER_VOLUME {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MODEMSETTINGS {
    pub dwActualSize: u32,
    pub dwRequiredSize: u32,
    pub dwDevSpecificOffset: u32,
    pub dwDevSpecificSize: u32,
    pub dwCallSetupFailTimer: u32,
    pub dwInactivityTimeout: u32,
    pub dwSpeakerVolume: MODEM_SPEAKER_VOLUME,
    pub dwSpeakerMode: MODEMSETTINGS_SPEAKER_MODE,
    pub dwPreferredModemOptions: u32,
    pub dwNegotiatedModemOptions: u32,
    pub dwNegotiatedDCERate: u32,
    pub abVariablePortion: [u8; 1],
}
impl Default for MODEMSETTINGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MODEMSETTINGS_SPEAKER_MODE(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MODEM_SPEAKER_VOLUME(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MODEM_STATUS_FLAGS(pub u32);
impl MODEM_STATUS_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for MODEM_STATUS_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for MODEM_STATUS_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for MODEM_STATUS_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for MODEM_STATUS_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for MODEM_STATUS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const MS_CTS_ON: MODEM_STATUS_FLAGS = MODEM_STATUS_FLAGS(16u32);
pub const MS_DSR_ON: MODEM_STATUS_FLAGS = MODEM_STATUS_FLAGS(32u32);
pub const MS_RING_ON: MODEM_STATUS_FLAGS = MODEM_STATUS_FLAGS(64u32);
pub const MS_RLSD_ON: MODEM_STATUS_FLAGS = MODEM_STATUS_FLAGS(128u32);
pub const NOPARITY: DCB_PARITY = DCB_PARITY(0u8);
pub const ODDPARITY: DCB_PARITY = DCB_PARITY(1u8);
pub const ONE5STOPBITS: DCB_STOP_BITS = DCB_STOP_BITS(1u8);
pub const ONESTOPBIT: DCB_STOP_BITS = DCB_STOP_BITS(0u8);
pub const PARITY_EVEN: COMMPROP_STOP_PARITY = COMMPROP_STOP_PARITY(1024u16);
pub const PARITY_MARK: COMMPROP_STOP_PARITY = COMMPROP_STOP_PARITY(2048u16);
pub const PARITY_NONE: COMMPROP_STOP_PARITY = COMMPROP_STOP_PARITY(256u16);
pub const PARITY_ODD: COMMPROP_STOP_PARITY = COMMPROP_STOP_PARITY(512u16);
pub const PARITY_SPACE: COMMPROP_STOP_PARITY = COMMPROP_STOP_PARITY(4096u16);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PURGE_COMM_FLAGS(pub u32);
impl PURGE_COMM_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PURGE_COMM_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PURGE_COMM_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PURGE_COMM_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PURGE_COMM_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PURGE_COMM_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const PURGE_RXABORT: PURGE_COMM_FLAGS = PURGE_COMM_FLAGS(2u32);
pub const PURGE_RXCLEAR: PURGE_COMM_FLAGS = PURGE_COMM_FLAGS(8u32);
pub const PURGE_TXABORT: PURGE_COMM_FLAGS = PURGE_COMM_FLAGS(1u32);
pub const PURGE_TXCLEAR: PURGE_COMM_FLAGS = PURGE_COMM_FLAGS(4u32);
pub const SETBREAK: ESCAPE_COMM_FUNCTION = ESCAPE_COMM_FUNCTION(8u32);
pub const SETDTR: ESCAPE_COMM_FUNCTION = ESCAPE_COMM_FUNCTION(5u32);
pub const SETRTS: ESCAPE_COMM_FUNCTION = ESCAPE_COMM_FUNCTION(3u32);
pub const SETXOFF: ESCAPE_COMM_FUNCTION = ESCAPE_COMM_FUNCTION(1u32);
pub const SETXON: ESCAPE_COMM_FUNCTION = ESCAPE_COMM_FUNCTION(2u32);
pub const SID_3GPP_SUPSVCMODEL: windows_core::GUID = windows_core::GUID::from_u128(0xd7d08e07_d767_4478_b14a_eecc87ea12f7);
pub const SPACEPARITY: DCB_PARITY = DCB_PARITY(4u8);
pub const STOPBITS_10: COMMPROP_STOP_PARITY = COMMPROP_STOP_PARITY(1u16);
pub const STOPBITS_15: COMMPROP_STOP_PARITY = COMMPROP_STOP_PARITY(2u16);
pub const STOPBITS_20: COMMPROP_STOP_PARITY = COMMPROP_STOP_PARITY(4u16);
pub const TWOSTOPBITS: DCB_STOP_BITS = DCB_STOP_BITS(2u8);
