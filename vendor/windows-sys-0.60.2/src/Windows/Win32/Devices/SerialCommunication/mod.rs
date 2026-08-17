windows_targets::link!("msports.dll" "system" fn ComDBClaimNextFreePort(hcomdb : HCOMDB, comnumber : *mut u32) -> i32);
windows_targets::link!("msports.dll" "system" fn ComDBClaimPort(hcomdb : HCOMDB, comnumber : u32, forceclaim : windows_sys::core::BOOL, forced : *mut windows_sys::core::BOOL) -> i32);
windows_targets::link!("msports.dll" "system" fn ComDBClose(hcomdb : HCOMDB) -> i32);
windows_targets::link!("msports.dll" "system" fn ComDBGetCurrentPortUsage(hcomdb : HCOMDB, buffer : *mut u8, buffersize : u32, reporttype : u32, maxportsreported : *mut u32) -> i32);
windows_targets::link!("msports.dll" "system" fn ComDBOpen(phcomdb : *mut HCOMDB) -> i32);
windows_targets::link!("msports.dll" "system" fn ComDBReleasePort(hcomdb : HCOMDB, comnumber : u32) -> i32);
windows_targets::link!("msports.dll" "system" fn ComDBResizeDatabase(hcomdb : HCOMDB, newsize : u32) -> i32);
pub const CDB_REPORT_BITS: u32 = 0u32;
pub const CDB_REPORT_BYTES: u32 = 1u32;
pub const COMDB_MAX_PORTS_ARBITRATED: u32 = 4096u32;
pub const COMDB_MIN_PORTS_ARBITRATED: u32 = 256u32;
pub const DEVPKEY_DeviceInterface_Serial_PortName: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_sys::core::GUID::from_u128(0x4c6bf15c_4c03_4aac_91f5_64c0f852bcf4), pid: 4 };
pub const DEVPKEY_DeviceInterface_Serial_UsbProductId: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_sys::core::GUID::from_u128(0x4c6bf15c_4c03_4aac_91f5_64c0f852bcf4), pid: 3 };
pub const DEVPKEY_DeviceInterface_Serial_UsbVendorId: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_sys::core::GUID::from_u128(0x4c6bf15c_4c03_4aac_91f5_64c0f852bcf4), pid: 2 };
pub const EVEN_PARITY: u32 = 2u32;
pub type HCOMDB = *mut core::ffi::c_void;
pub const IOCTL_INTERNAL_SERENUM_REMOVE_SELF: u32 = 3604999u32;
pub const IOCTL_SERIAL_APPLY_DEFAULT_CONFIGURATION: u32 = 1769632u32;
pub const IOCTL_SERIAL_CLEAR_STATS: u32 = 1769616u32;
pub const IOCTL_SERIAL_CLR_DTR: u32 = 1769512u32;
pub const IOCTL_SERIAL_CLR_RTS: u32 = 1769524u32;
pub const IOCTL_SERIAL_CONFIG_SIZE: u32 = 1769600u32;
pub const IOCTL_SERIAL_GET_BAUD_RATE: u32 = 1769552u32;
pub const IOCTL_SERIAL_GET_CHARS: u32 = 1769560u32;
pub const IOCTL_SERIAL_GET_COMMCONFIG: u32 = 1769604u32;
pub const IOCTL_SERIAL_GET_COMMSTATUS: u32 = 1769580u32;
pub const IOCTL_SERIAL_GET_DTRRTS: u32 = 1769592u32;
pub const IOCTL_SERIAL_GET_HANDFLOW: u32 = 1769568u32;
pub const IOCTL_SERIAL_GET_LINE_CONTROL: u32 = 1769556u32;
pub const IOCTL_SERIAL_GET_MODEMSTATUS: u32 = 1769576u32;
pub const IOCTL_SERIAL_GET_MODEM_CONTROL: u32 = 1769620u32;
pub const IOCTL_SERIAL_GET_PROPERTIES: u32 = 1769588u32;
pub const IOCTL_SERIAL_GET_STATS: u32 = 1769612u32;
pub const IOCTL_SERIAL_GET_TIMEOUTS: u32 = 1769504u32;
pub const IOCTL_SERIAL_GET_WAIT_MASK: u32 = 1769536u32;
pub const IOCTL_SERIAL_IMMEDIATE_CHAR: u32 = 1769496u32;
pub const IOCTL_SERIAL_INTERNAL_BASIC_SETTINGS: u32 = 1769484u32;
pub const IOCTL_SERIAL_INTERNAL_CANCEL_WAIT_WAKE: u32 = 1769480u32;
pub const IOCTL_SERIAL_INTERNAL_DO_WAIT_WAKE: u32 = 1769476u32;
pub const IOCTL_SERIAL_INTERNAL_RESTORE_SETTINGS: u32 = 1769488u32;
pub const IOCTL_SERIAL_PURGE: u32 = 1769548u32;
pub const IOCTL_SERIAL_RESET_DEVICE: u32 = 1769516u32;
pub const IOCTL_SERIAL_SET_BAUD_RATE: u32 = 1769476u32;
pub const IOCTL_SERIAL_SET_BREAK_OFF: u32 = 1769492u32;
pub const IOCTL_SERIAL_SET_BREAK_ON: u32 = 1769488u32;
pub const IOCTL_SERIAL_SET_CHARS: u32 = 1769564u32;
pub const IOCTL_SERIAL_SET_COMMCONFIG: u32 = 1769608u32;
pub const IOCTL_SERIAL_SET_DTR: u32 = 1769508u32;
pub const IOCTL_SERIAL_SET_FIFO_CONTROL: u32 = 1769628u32;
pub const IOCTL_SERIAL_SET_HANDFLOW: u32 = 1769572u32;
pub const IOCTL_SERIAL_SET_INTERVAL_TIMER_RESOLUTION: u32 = 1769636u32;
pub const IOCTL_SERIAL_SET_LINE_CONTROL: u32 = 1769484u32;
pub const IOCTL_SERIAL_SET_MODEM_CONTROL: u32 = 1769624u32;
pub const IOCTL_SERIAL_SET_QUEUE_SIZE: u32 = 1769480u32;
pub const IOCTL_SERIAL_SET_RTS: u32 = 1769520u32;
pub const IOCTL_SERIAL_SET_TIMEOUTS: u32 = 1769500u32;
pub const IOCTL_SERIAL_SET_WAIT_MASK: u32 = 1769540u32;
pub const IOCTL_SERIAL_SET_XOFF: u32 = 1769528u32;
pub const IOCTL_SERIAL_SET_XON: u32 = 1769532u32;
pub const IOCTL_SERIAL_WAIT_ON_MASK: u32 = 1769544u32;
pub const IOCTL_SERIAL_XOFF_COUNTER: u32 = 1769584u32;
pub const MARK_PARITY: u32 = 3u32;
pub const NO_PARITY: u32 = 0u32;
pub const ODD_PARITY: u32 = 1u32;
pub type PSERENUM_READPORT = Option<unsafe extern "system" fn(serportaddress: *const core::ffi::c_void) -> u8>;
pub type PSERENUM_WRITEPORT = Option<unsafe extern "system" fn(serportaddress: *const core::ffi::c_void, value: u8)>;
pub type SERENUM_PORTION = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SERENUM_PORT_DESC {
    pub Size: u32,
    pub PortHandle: *mut core::ffi::c_void,
    pub PortAddress: i64,
    pub Reserved: [u16; 1],
}
impl Default for SERENUM_PORT_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SERENUM_PORT_PARAMETERS {
    pub Size: u32,
    pub ReadAccessor: PSERENUM_READPORT,
    pub WriteAccessor: PSERENUM_WRITEPORT,
    pub SerPortAddress: *mut core::ffi::c_void,
    pub HardwareHandle: *mut core::ffi::c_void,
    pub Portion: SERENUM_PORTION,
    pub NumberAxis: u16,
    pub Reserved: [u16; 3],
}
impl Default for SERENUM_PORT_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SERIALCONFIG {
    pub Size: u32,
    pub Version: u16,
    pub SubType: u32,
    pub ProvOffset: u32,
    pub ProviderSize: u32,
    pub ProviderData: [u16; 1],
}
impl Default for SERIALCONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SERIALPERF_STATS {
    pub ReceivedCount: u32,
    pub TransmittedCount: u32,
    pub FrameErrorCount: u32,
    pub SerialOverrunErrorCount: u32,
    pub BufferOverrunErrorCount: u32,
    pub ParityErrorCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SERIAL_BASIC_SETTINGS {
    pub Timeouts: SERIAL_TIMEOUTS,
    pub HandFlow: SERIAL_HANDFLOW,
    pub RxFifo: u32,
    pub TxFifo: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SERIAL_BAUD_RATE {
    pub BaudRate: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SERIAL_CHARS {
    pub EofChar: u8,
    pub ErrorChar: u8,
    pub BreakChar: u8,
    pub EventChar: u8,
    pub XonChar: u8,
    pub XoffChar: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SERIAL_COMMPROP {
    pub PacketLength: u16,
    pub PacketVersion: u16,
    pub ServiceMask: u32,
    pub Reserved1: u32,
    pub MaxTxQueue: u32,
    pub MaxRxQueue: u32,
    pub MaxBaud: u32,
    pub ProvSubType: u32,
    pub ProvCapabilities: u32,
    pub SettableParams: u32,
    pub SettableBaud: u32,
    pub SettableData: u16,
    pub SettableStopParity: u16,
    pub CurrentTxQueue: u32,
    pub CurrentRxQueue: u32,
    pub ProvSpec1: u32,
    pub ProvSpec2: u32,
    pub ProvChar: [u16; 1],
}
impl Default for SERIAL_COMMPROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SERIAL_EV_BREAK: u32 = 64u32;
pub const SERIAL_EV_CTS: u32 = 8u32;
pub const SERIAL_EV_DSR: u32 = 16u32;
pub const SERIAL_EV_ERR: u32 = 128u32;
pub const SERIAL_EV_EVENT1: u32 = 2048u32;
pub const SERIAL_EV_EVENT2: u32 = 4096u32;
pub const SERIAL_EV_PERR: u32 = 512u32;
pub const SERIAL_EV_RING: u32 = 256u32;
pub const SERIAL_EV_RLSD: u32 = 32u32;
pub const SERIAL_EV_RX80FULL: u32 = 1024u32;
pub const SERIAL_EV_RXCHAR: u32 = 1u32;
pub const SERIAL_EV_RXFLAG: u32 = 2u32;
pub const SERIAL_EV_TXEMPTY: u32 = 4u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SERIAL_HANDFLOW {
    pub ControlHandShake: u32,
    pub FlowReplace: u32,
    pub XonLimit: i32,
    pub XoffLimit: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SERIAL_LINE_CONTROL {
    pub StopBits: u8,
    pub Parity: u8,
    pub WordLength: u8,
}
pub const SERIAL_LSRMST_ESCAPE: u16 = 0u16;
pub const SERIAL_LSRMST_LSR_DATA: u16 = 1u16;
pub const SERIAL_LSRMST_LSR_NODATA: u16 = 2u16;
pub const SERIAL_LSRMST_MST: u16 = 3u16;
pub const SERIAL_PURGE_RXABORT: u32 = 2u32;
pub const SERIAL_PURGE_RXCLEAR: u32 = 8u32;
pub const SERIAL_PURGE_TXABORT: u32 = 1u32;
pub const SERIAL_PURGE_TXCLEAR: u32 = 4u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SERIAL_QUEUE_SIZE {
    pub InSize: u32,
    pub OutSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SERIAL_STATUS {
    pub Errors: u32,
    pub HoldReasons: u32,
    pub AmountInInQueue: u32,
    pub AmountInOutQueue: u32,
    pub EofReceived: bool,
    pub WaitForImmediate: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SERIAL_TIMEOUTS {
    pub ReadIntervalTimeout: u32,
    pub ReadTotalTimeoutMultiplier: u32,
    pub ReadTotalTimeoutConstant: u32,
    pub WriteTotalTimeoutMultiplier: u32,
    pub WriteTotalTimeoutConstant: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SERIAL_XOFF_COUNTER {
    pub Timeout: u32,
    pub Counter: i32,
    pub XoffChar: u8,
}
pub const SPACE_PARITY: u32 = 4u32;
pub const STOP_BITS_1_5: u32 = 1u32;
pub const STOP_BITS_2: u32 = 2u32;
pub const STOP_BIT_1: u32 = 0u32;
pub const SerenumFirstHalf: SERENUM_PORTION = 0i32;
pub const SerenumSecondHalf: SERENUM_PORTION = 1i32;
pub const SerenumWhole: SERENUM_PORTION = 2i32;
