use core::ptr::null_mut;
use crate::ntrtl::RTL_TIME_ZONE_INFORMATION;
use winapi::shared::basetsd::SIZE_T;
use winapi::shared::minwindef::{BYTE, DWORD, FILETIME};
use winapi::shared::ntdef::{
    BOOLEAN, CHAR, HANDLE, LARGE_INTEGER, LONG, PULONG, PVOID, PWSTR, UCHAR, ULONG, UNICODE_STRING,
    USHORT, WCHAR,
};
use winapi::shared::windef::HWND;
use winapi::um::winnt::{PSID, STANDARD_RIGHTS_REQUIRED};
pub const WINSTATION_QUERY: u32 = 0x00000001;
pub const WINSTATION_SET: u32 = 0x00000002;
pub const WINSTATION_RESET: u32 = 0x00000004;
pub const WINSTATION_VIRTUAL: u32 = 0x00000008;
pub const WINSTATION_SHADOW: u32 = 0x00000010;
pub const WINSTATION_LOGON: u32 = 0x00000020;
pub const WINSTATION_LOGOFF: u32 = 0x00000040;
pub const WINSTATION_MSG: u32 = 0x00000080;
pub const WINSTATION_CONNECT: u32 = 0x00000100;
pub const WINSTATION_DISCONNECT: u32 = 0x00000200;
pub const WINSTATION_GUEST_ACCESS: u32 = WINSTATION_LOGON;
pub const WINSTATION_CURRENT_GUEST_ACCESS: u32 = WINSTATION_VIRTUAL | WINSTATION_LOGOFF;
pub const WINSTATION_USER_ACCESS: u32 =
    WINSTATION_GUEST_ACCESS | WINSTATION_QUERY | WINSTATION_CONNECT;
pub const WINSTATION_CURRENT_USER_ACCESS: u32 = WINSTATION_SET | WINSTATION_RESET
    | WINSTATION_VIRTUAL | WINSTATION_LOGOFF | WINSTATION_DISCONNECT;
pub const WINSTATION_ALL_ACCESS: u32 = STANDARD_RIGHTS_REQUIRED | WINSTATION_QUERY | WINSTATION_SET
    | WINSTATION_RESET | WINSTATION_VIRTUAL | WINSTATION_SHADOW | WINSTATION_LOGON | WINSTATION_MSG
    | WINSTATION_CONNECT | WINSTATION_DISCONNECT;
pub const WDPREFIX_LENGTH: usize = 12;
pub const CALLBACK_LENGTH: usize = 50;
pub const DLLNAME_LENGTH: usize = 32;
pub const CDNAME_LENGTH: usize = 32;
pub const WDNAME_LENGTH: usize = 32;
pub const PDNAME_LENGTH: usize = 32;
pub const DEVICENAME_LENGTH: usize = 128;
pub const MODEMNAME_LENGTH: usize = DEVICENAME_LENGTH;
pub const STACK_ADDRESS_LENGTH: usize = 128;
pub const MAX_BR_NAME: usize = 65;
pub const DIRECTORY_LENGTH: usize = 256;
pub const INITIALPROGRAM_LENGTH: usize = 256;
pub const USERNAME_LENGTH: usize = 20;
pub const DOMAIN_LENGTH: usize = 17;
pub const PASSWORD_LENGTH: usize = 14;
pub const NASISPECIFICNAME_LENGTH: usize = 14;
pub const NASIUSERNAME_LENGTH: usize = 47;
pub const NASIPASSWORD_LENGTH: usize = 24;
pub const NASISESSIONNAME_LENGTH: usize = 16;
pub const NASIFILESERVER_LENGTH: usize = 47;
pub const CLIENTDATANAME_LENGTH: usize = 7;
pub const CLIENTNAME_LENGTH: usize = 20;
pub const CLIENTADDRESS_LENGTH: usize = 30;
pub const IMEFILENAME_LENGTH: usize = 32;
pub const CLIENTLICENSE_LENGTH: usize = 32;
pub const CLIENTMODEM_LENGTH: usize = 40;
pub const CLIENT_PRODUCT_ID_LENGTH: usize = 32;
pub const MAX_COUNTER_EXTENSIONS: u32 = 2;
pub const WINSTATIONNAME_LENGTH: usize = 32;
pub const TERMSRV_TOTAL_SESSIONS: u32 = 1;
pub const TERMSRV_DISC_SESSIONS: u32 = 2;
pub const TERMSRV_RECON_SESSIONS: u32 = 3;
pub const TERMSRV_CURRENT_ACTIVE_SESSIONS: u32 = 4;
pub const TERMSRV_CURRENT_DISC_SESSIONS: u32 = 5;
pub const TERMSRV_PENDING_SESSIONS: u32 = 6;
pub const TERMSRV_SUCC_TOTAL_LOGONS: u32 = 7;
pub const TERMSRV_SUCC_LOCAL_LOGONS: u32 = 8;
pub const TERMSRV_SUCC_REMOTE_LOGONS: u32 = 9;
pub const TERMSRV_SUCC_SESSION0_LOGONS: u32 = 10;
pub const TERMSRV_CURRENT_TERMINATING_SESSIONS: u32 = 11;
pub const TERMSRV_CURRENT_LOGGEDON_SESSIONS: u32 = 12;
pub type PTS_TIME_ZONE_INFORMATION = *mut RTL_TIME_ZONE_INFORMATION;
pub type TS_TIME_ZONE_INFORMATION = RTL_TIME_ZONE_INFORMATION;
pub type WINSTATIONNAME = [WCHAR; WINSTATIONNAME_LENGTH + 1];
STRUCT!{struct VARDATA_WIRE {
    Size: USHORT,
    Offset: USHORT,
}}
pub type PVARDATA_WIRE = *mut VARDATA_WIRE;
ENUM!{enum WINSTATIONSTATECLASS {
    State_Active = 0,
    State_Connected = 1,
    State_ConnectQuery = 2,
    State_Shadow = 3,
    State_Disconnected = 4,
    State_Idle = 5,
    State_Listen = 6,
    State_Reset = 7,
    State_Down = 8,
    State_Init = 9,
}}
UNION!{union SESSIONIDW_u {
    SessionId: ULONG,
    LogonId: ULONG,
}}
STRUCT!{struct SESSIONIDW {
    u: SESSIONIDW_u,
    WinStationName: WINSTATIONNAME,
    State: WINSTATIONSTATECLASS,
}}
pub type PSESSIONIDW = *mut SESSIONIDW;
ENUM!{enum WINSTATIONINFOCLASS {
    WinStationCreateData = 0,
    WinStationConfiguration = 1,
    WinStationPdParams = 2,
    WinStationWd = 3,
    WinStationPd = 4,
    WinStationPrinter = 5,
    WinStationClient = 6,
    WinStationModules = 7,
    WinStationInformation = 8,
    WinStationTrace = 9,
    WinStationBeep = 10,
    WinStationEncryptionOff = 11,
    WinStationEncryptionPerm = 12,
    WinStationNtSecurity = 13,
    WinStationUserToken = 14,
    WinStationUnused1 = 15,
    WinStationVideoData = 16,
    WinStationInitialProgram = 17,
    WinStationCd = 18,
    WinStationSystemTrace = 19,
    WinStationVirtualData = 20,
    WinStationClientData = 21,
    WinStationSecureDesktopEnter = 22,
    WinStationSecureDesktopExit = 23,
    WinStationLoadBalanceSessionTarget = 24,
    WinStationLoadIndicator = 25,
    WinStationShadowInfo = 26,
    WinStationDigProductId = 27,
    WinStationLockedState = 28,
    WinStationRemoteAddress = 29,
    WinStationIdleTime = 30,
    WinStationLastReconnectType = 31,
    WinStationDisallowAutoReconnect = 32,
    WinStationMprNotifyInfo = 33,
    WinStationExecSrvSystemPipe = 34,
    WinStationSmartCardAutoLogon = 35,
    WinStationIsAdminLoggedOn = 36,
    WinStationReconnectedFromId = 37,
    WinStationEffectsPolicy = 38,
    WinStationType = 39,
    WinStationInformationEx = 40,
    WinStationValidationInfo = 41,
}}
STRUCT!{struct WINSTATIONCREATE {
    Bitfields: ULONG,
    MaxInstanceCount: ULONG,
}}
BITFIELD!{WINSTATIONCREATE Bitfields: ULONG [
    fEnableWinStation set_fEnableWinStation[0..1],
]}
pub type PWINSTATIONCREATE = *mut WINSTATIONCREATE;
STRUCT!{struct WINSTACONFIGWIRE {
    Comment: [WCHAR; 61],
    OEMId: [CHAR; 4],
    UserConfig: VARDATA_WIRE,
    NewFields: VARDATA_WIRE,
}}
pub type PWINSTACONFIGWIRE = *mut WINSTACONFIGWIRE;
ENUM!{enum CALLBACKCLASS {
    Callback_Disable = 0,
    Callback_Roving = 1,
    Callback_Fixed = 2,
}}
ENUM!{enum SHADOWCLASS {
    Shadow_Disable = 0,
    Shadow_EnableInputNotify = 1,
    Shadow_EnableInputNoNotify = 2,
    Shadow_EnableNoInputNotify = 3,
    Shadow_EnableNoInputNoNotify = 4,
}}
STRUCT!{struct USERCONFIG {
    Bitfields: ULONG,
    Bitfields2: ULONG,
    UserName: [WCHAR; USERNAME_LENGTH + 1],
    Domain: [WCHAR; DOMAIN_LENGTH + 1],
    Password: [WCHAR; PASSWORD_LENGTH + 1],
    WorkDirectory: [WCHAR; DIRECTORY_LENGTH + 1],
    InitialProgram: [WCHAR; INITIALPROGRAM_LENGTH + 1],
    CallbackNumber: [WCHAR; CALLBACK_LENGTH + 1],
    Callback: CALLBACKCLASS,
    Shadow: SHADOWCLASS,
    MaxConnectionTime: ULONG,
    MaxDisconnectionTime: ULONG,
    MaxIdleTime: ULONG,
    KeyboardLayout: ULONG,
    MinEncryptionLevel: BYTE,
    NWLogonServer: [WCHAR; NASIFILESERVER_LENGTH + 1],
    PublishedName: [WCHAR; MAX_BR_NAME],
    WFProfilePath: [WCHAR; DIRECTORY_LENGTH + 1],
    WFHomeDir: [WCHAR; DIRECTORY_LENGTH + 1],
    WFHomeDirDrive: [WCHAR; 4],
}}
BITFIELD!{USERCONFIG Bitfields: ULONG [
    fInheritAutoLogon set_fInheritAutoLogon[0..1],
    fInheritResetBroken set_fInheritResetBroken[1..2],
    fInheritReconnectSame set_fInheritReconnectSame[2..3],
    fInheritInitialProgram set_fInheritInitialProgram[3..4],
    fInheritCallback set_fInheritCallback[4..5],
    fInheritCallbackNumber set_fInheritCallbackNumber[5..6],
    fInheritShadow set_fInheritShadow[6..7],
    fInheritMaxSessionTime set_fInheritMaxSessionTime[7..8],
    fInheritMaxDisconnectionTime set_fInheritMaxDisconnectionTime[8..9],
    fInheritMaxIdleTime set_fInheritMaxIdleTime[9..10],
    fInheritAutoClient set_fInheritAutoClient[10..11],
    fInheritSecurity set_fInheritSecurity[11..12],
    fPromptForPassword set_fPromptForPassword[12..13],
    fResetBroken set_fResetBroken[13..14],
    fReconnectSame set_fReconnectSame[14..15],
    fLogonDisabled set_fLogonDisabled[15..16],
    fWallPaperDisabled set_fWallPaperDisabled[16..17],
    fAutoClientDrives set_fAutoClientDrives[17..18],
    fAutoClientLpts set_fAutoClientLpts[18..19],
    fForceClientLptDef set_fForceClientLptDef[19..20],
    fRequireEncryption set_fRequireEncryption[20..21],
    fDisableEncryption set_fDisableEncryption[21..22],
    fUnused1 set_fUnused1[22..23],
    fHomeDirectoryMapRoot set_fHomeDirectoryMapRoot[23..24],
    fUseDefaultGina set_fUseDefaultGina[24..25],
    fCursorBlinkDisabled set_fCursorBlinkDisabled[25..26],
    fPublishedApp set_fPublishedApp[26..27],
    fHideTitleBar set_fHideTitleBar[27..28],
    fMaximize set_fMaximize[28..29],
    fDisableCpm set_fDisableCpm[29..30],
    fDisableCdm set_fDisableCdm[30..31],
    fDisableCcm set_fDisableCcm[31..32],
]}
BITFIELD!{USERCONFIG Bitfields2: ULONG [
    fDisableLPT set_fDisableLPT[0..1],
    fDisableClip set_fDisableClip[1..2],
    fDisableExe set_fDisableExe[2..3],
    fDisableCam set_fDisableCam[3..4],
    fDisableAutoReconnect set_fDisableAutoReconnect[4..5],
    ColorDepth set_ColorDepth[5..6],
    fInheritColorDepth set_fInheritColorDepth[6..7],
    fErrorInvalidProfile set_fErrorInvalidProfile[7..8],
    fPasswordIsScPin set_fPasswordIsScPin[8..9],
    fDisablePNPRedir set_fDisablePNPRedir[9..10],
]}
pub type PUSERCONFIG = *mut USERCONFIG;
ENUM!{enum SDCLASS {
    SdNone = 0,
    SdConsole = 1,
    SdNetwork = 2,
    SdAsync = 3,
    SdOemTransport = 4,
}}
pub type DEVICENAME = [WCHAR; DEVICENAME_LENGTH + 1];
pub type MODEMNAME = [WCHAR; MODEMNAME_LENGTH + 1];
pub type NASISPECIFICNAME = [WCHAR; NASISPECIFICNAME_LENGTH + 1];
pub type NASIUSERNAME = [WCHAR; NASIUSERNAME_LENGTH + 1];
pub type NASIPASSWORD = [WCHAR; NASIPASSWORD_LENGTH + 1];
pub type NASISESIONNAME = [WCHAR; NASISESSIONNAME_LENGTH + 1];
pub type NASIFILESERVER = [WCHAR; NASIFILESERVER_LENGTH + 1];
pub type WDNAME = [WCHAR; WDNAME_LENGTH + 1];
pub type WDPREFIX = [WCHAR; WDPREFIX_LENGTH + 1];
pub type CDNAME = [WCHAR; CDNAME_LENGTH + 1];
pub type DLLNAME = [WCHAR; DLLNAME_LENGTH + 1];
pub type PDNAME = [WCHAR; PDNAME_LENGTH + 1];
STRUCT!{struct NETWORKCONFIG {
    LanAdapter: LONG,
    NetworkName: DEVICENAME,
    Flags: ULONG,
}}
pub type PNETWORKCONFIG = *mut NETWORKCONFIG;
ENUM!{enum FLOWCONTROLCLASS {
    FlowControl_None = 0,
    FlowControl_Hardware = 1,
    FlowControl_Software = 2,
}}
ENUM!{enum RECEIVEFLOWCONTROLCLASS {
    ReceiveFlowControl_None = 0,
    ReceiveFlowControl_RTS = 1,
    ReceiveFlowControl_DTR = 2,
}}
ENUM!{enum TRANSMITFLOWCONTROLCLASS {
    TransmitFlowControl_None = 0,
    TransmitFlowControl_CTS = 1,
    TransmitFlowControl_DSR = 2,
}}
ENUM!{enum ASYNCCONNECTCLASS {
    Connect_CTS = 0,
    Connect_DSR = 1,
    Connect_RI = 2,
    Connect_DCD = 3,
    Connect_FirstChar = 4,
    Connect_Perm = 5,
}}
STRUCT!{struct FLOWCONTROLCONFIG {
    Bitfields: ULONG,
    XonChar: CHAR,
    XoffChar: CHAR,
    Type: FLOWCONTROLCLASS,
    HardwareReceive: RECEIVEFLOWCONTROLCLASS,
    HardwareTransmit: TRANSMITFLOWCONTROLCLASS,
}}
BITFIELD!{FLOWCONTROLCONFIG Bitfields: ULONG [
    fEnableSoftwareTx set_fEnableSoftwareTx[0..1],
    fEnableSoftwareRx set_fEnableSoftwareRx[1..2],
    fEnableDTR set_fEnableDTR[2..3],
    fEnableRTS set_fEnableRTS[3..4],
]}
pub type PFLOWCONTROLCONFIG = *mut FLOWCONTROLCONFIG;
STRUCT!{struct CONNECTCONFIG {
    Type: ASYNCCONNECTCLASS,
    Bitfields: ULONG,
}}
BITFIELD!{CONNECTCONFIG Bitfields: ULONG [
    fEnableBreakDisconnect set_fEnableBreakDisconnect[0..1],
]}
pub type PCONNECTCONFIG = *mut CONNECTCONFIG;
STRUCT!{struct ASYNCCONFIG {
    DeviceName: DEVICENAME,
    ModemName: MODEMNAME,
    BaudRate: ULONG,
    Parity: ULONG,
    StopBits: ULONG,
    ByteSize: ULONG,
    Bitfields: ULONG,
    FlowControl: FLOWCONTROLCONFIG,
    Connect: CONNECTCONFIG,
}}
BITFIELD!{ASYNCCONFIG Bitfields: ULONG [
    fEnableDsrSensitivity set_fEnableDsrSensitivity[0..1],
    fConnectionDriver set_fConnectionDriver[1..2],
]}
pub type PASYNCCONFIG = *mut ASYNCCONFIG;
STRUCT!{struct NASICONFIG {
    SpecificName: NASISPECIFICNAME,
    UserName: NASIUSERNAME,
    PassWord: NASIPASSWORD,
    SessionName: NASISESIONNAME,
    FileServer: NASIFILESERVER,
    GlobalSession: BOOLEAN,
}}
pub type PNASICONFIG = *mut NASICONFIG;
STRUCT!{struct OEMTDCONFIG {
    Adapter: LONG,
    DeviceName: DEVICENAME,
    Flags: ULONG,
}}
pub type POEMTDCONFIG = *mut OEMTDCONFIG;
UNION!{union PDPARAMS_u {
    Network: NETWORKCONFIG,
    Async: ASYNCCONFIG,
    Nasi: NASICONFIG,
    OemTd: OEMTDCONFIG,
}}
STRUCT!{struct PDPARAMS {
    SdClass: SDCLASS,
    u: PDPARAMS_u,
}}
pub type PPDPARAMS = *mut PDPARAMS;
STRUCT!{struct WDCONFIG {
    WdName: WDNAME,
    WdDLL: DLLNAME,
    WsxDLL: DLLNAME,
    WdFlag: ULONG,
    WdInputBufferLength: ULONG,
    CfgDLL: DLLNAME,
    WdPrefix: WDPREFIX,
}}
pub type PWDCONFIG = *mut WDCONFIG;
STRUCT!{struct PDCONFIG2 {
    PdName: PDNAME,
    SdClass: SDCLASS,
    PdDLL: DLLNAME,
    PdFlag: ULONG,
    OutBufLength: ULONG,
    OutBufCount: ULONG,
    OutBufDelay: ULONG,
    InteractiveDelay: ULONG,
    PortNumber: ULONG,
    KeepAliveTimeout: ULONG,
}}
pub type PPDCONFIG2 = *mut PDCONFIG2;
STRUCT!{struct WINSTATIONCLIENT {
    Bitfields: ULONG,
    ClientName: [WCHAR; CLIENTNAME_LENGTH + 1],
    Domain: [WCHAR; DOMAIN_LENGTH + 1],
    UserName: [WCHAR; USERNAME_LENGTH + 1],
    Password: [WCHAR; PASSWORD_LENGTH + 1],
    WorkDirectory: [WCHAR; DIRECTORY_LENGTH + 1],
    InitialProgram: [WCHAR; INITIALPROGRAM_LENGTH + 1],
    SerialNumber: ULONG,
    EncryptionLevel: BYTE,
    ClientAddressFamily: ULONG,
    ClientAddress: [WCHAR; CLIENTADDRESS_LENGTH + 1],
    HRes: USHORT,
    VRes: USHORT,
    ColorDepth: USHORT,
    ProtocolType: USHORT,
    KeyboardLayout: ULONG,
    KeyboardType: ULONG,
    KeyboardSubType: ULONG,
    KeyboardFunctionKey: ULONG,
    ImeFileName: [WCHAR; IMEFILENAME_LENGTH + 1],
    ClientDirectory: [WCHAR; DIRECTORY_LENGTH + 1],
    ClientLicense: [WCHAR; CLIENTLICENSE_LENGTH + 1],
    ClientModem: [WCHAR; CLIENTMODEM_LENGTH + 1],
    ClientBuildNumber: ULONG,
    ClientHardwareId: ULONG,
    ClientProductId: USHORT,
    OutBufCountHost: USHORT,
    OutBufCountClient: USHORT,
    OutBufLength: USHORT,
    AudioDriverName: [WCHAR; 9],
    ClientTimeZone: TS_TIME_ZONE_INFORMATION,
    ClientSessionId: ULONG,
    ClientDigProductId: [WCHAR; CLIENT_PRODUCT_ID_LENGTH],
    PerformanceFlags: ULONG,
    ActiveInputLocale: ULONG,
}}
BITFIELD!{WINSTATIONCLIENT Bitfields: ULONG [
    fTextOnly set_fTextOnly[0..1],
    fDisableCtrlAltDel set_fDisableCtrlAltDel[1..2],
    fMouse set_fMouse[2..3],
    fDoubleClickDetect set_fDoubleClickDetect[3..4],
    fINetClient set_fINetClient[4..5],
    fPromptForPassword set_fPromptForPassword[5..6],
    fMaximizeShell set_fMaximizeShell[6..7],
    fEnableWindowsKey set_fEnableWindowsKey[7..8],
    fRemoteConsoleAudio set_fRemoteConsoleAudio[8..9],
    fPasswordIsScPin set_fPasswordIsScPin[9..10],
    fNoAudioPlayback set_fNoAudioPlayback[10..11],
    fUsingSavedCreds set_fUsingSavedCreds[11..12],
]}
pub type PWINSTATIONCLIENT = *mut WINSTATIONCLIENT;
STRUCT!{struct TSHARE_COUNTERS {
    Reserved: ULONG,
}}
pub type PTSHARE_COUNTERS = *mut TSHARE_COUNTERS;
UNION!{union PROTOCOLCOUNTERS_Specific {
    TShareCounters: TSHARE_COUNTERS,
    Reserved: [ULONG; 100],
}}
STRUCT!{struct PROTOCOLCOUNTERS {
    WdBytes: ULONG,
    WdFrames: ULONG,
    WaitForOutBuf: ULONG,
    Frames: ULONG,
    Bytes: ULONG,
    CompressedBytes: ULONG,
    CompressFlushes: ULONG,
    Errors: ULONG,
    Timeouts: ULONG,
    AsyncFramingError: ULONG,
    AsyncOverrunError: ULONG,
    AsyncOverflowError: ULONG,
    AsyncParityError: ULONG,
    TdErrors: ULONG,
    ProtocolType: USHORT,
    Length: USHORT,
    Specific: PROTOCOLCOUNTERS_Specific,
}}
pub type PPROTOCOLCOUNTERS = *mut PROTOCOLCOUNTERS;
STRUCT!{struct THINWIRECACHE {
    CacheReads: ULONG,
    CacheHits: ULONG,
}}
pub type PTHINWIRECACHE = *mut THINWIRECACHE;
pub const MAX_THINWIRECACHE: usize = 4;
STRUCT!{struct RESERVED_CACHE {
    ThinWireCache: [THINWIRECACHE; MAX_THINWIRECACHE],
}}
pub type PRESERVED_CACHE = *mut RESERVED_CACHE;
STRUCT!{struct TSHARE_CACHE {
    Reserved: ULONG,
}}
pub type PTSHARE_CACHE = *mut TSHARE_CACHE;
UNION!{union CACHE_STATISTICS_Specific {
    ReservedCacheStats: RESERVED_CACHE,
    TShareCacheStats: TSHARE_CACHE,
    Reserved: [ULONG; 20],
}}
STRUCT!{struct CACHE_STATISTICS {
    ProtocolType: USHORT,
    Length: USHORT,
    Specific: CACHE_STATISTICS_Specific,
}}
pub type PCACHE_STATISTICS = *mut CACHE_STATISTICS;
STRUCT!{struct PROTOCOLSTATUS {
    Output: PROTOCOLCOUNTERS,
    Input: PROTOCOLCOUNTERS,
    Cache: CACHE_STATISTICS,
    AsyncSignal: ULONG,
    AsyncSignalMask: ULONG,
}}
pub type PPROTOCOLSTATUS = *mut PROTOCOLSTATUS;
STRUCT!{struct WINSTATIONINFORMATION {
    ConnectState: WINSTATIONSTATECLASS,
    WinStationName: WINSTATIONNAME,
    LogonId: ULONG,
    ConnectTime: LARGE_INTEGER,
    DisconnectTime: LARGE_INTEGER,
    LastInputTime: LARGE_INTEGER,
    LogonTime: LARGE_INTEGER,
    Status: PROTOCOLSTATUS,
    Domain: [WCHAR; DOMAIN_LENGTH + 1],
    UserName: [WCHAR; USERNAME_LENGTH + 1],
    CurrentTime: LARGE_INTEGER,
}}
pub type PWINSTATIONINFORMATION = *mut WINSTATIONINFORMATION;
STRUCT!{struct WINSTATIONUSERTOKEN {
    ProcessId: HANDLE,
    ThreadId: HANDLE,
    UserToken: HANDLE,
}}
pub type PWINSTATIONUSERTOKEN = *mut WINSTATIONUSERTOKEN;
STRUCT!{struct WINSTATIONVIDEODATA {
    HResolution: USHORT,
    VResolution: USHORT,
    fColorDepth: USHORT,
}}
pub type PWINSTATIONVIDEODATA = *mut WINSTATIONVIDEODATA;
ENUM!{enum CDCLASS {
    CdNone = 0,
    CdModem = 1,
    CdClass_Maximum = 2,
}}
STRUCT!{struct CDCONFIG {
    CdClass: CDCLASS,
    CdName: CDNAME,
    CdDLL: DLLNAME,
    CdFlag: ULONG,
}}
pub type PCDCONFIG = *mut CDCONFIG;
pub type CLIENTDATANAME = [CHAR; CLIENTDATANAME_LENGTH + 1];
pub type PCLIENTDATANAME = *mut CHAR;
STRUCT!{struct WINSTATIONCLIENTDATA {
    DataName: CLIENTDATANAME,
    fUnicodeData: BOOLEAN,
}}
pub type PWINSTATIONCLIENTDATA = *mut WINSTATIONCLIENTDATA;
ENUM!{enum LOADFACTORTYPE {
    ErrorConstraint = 0,
    PagedPoolConstraint = 1,
    NonPagedPoolConstraint = 2,
    AvailablePagesConstraint = 3,
    SystemPtesConstraint = 4,
    CPUConstraint = 5,
}}
STRUCT!{struct WINSTATIONLOADINDICATORDATA {
    RemainingSessionCapacity: ULONG,
    LoadFactor: LOADFACTORTYPE,
    TotalSessions: ULONG,
    DisconnectedSessions: ULONG,
    IdleCPU: LARGE_INTEGER,
    TotalCPU: LARGE_INTEGER,
    RawSessionCapacity: ULONG,
    reserved: [ULONG; 9],
}}
pub type PWINSTATIONLOADINDICATORDATA = *mut WINSTATIONLOADINDICATORDATA;
ENUM!{enum SHADOWSTATECLASS {
    State_NoShadow = 0,
    State_Shadowing = 1,
    State_Shadowed = 2,
}}
STRUCT!{struct WINSTATIONSHADOW {
    ShadowState: SHADOWSTATECLASS,
    ShadowClass: SHADOWCLASS,
    SessionId: ULONG,
    ProtocolType: ULONG,
}}
pub type PWINSTATIONSHADOW = *mut WINSTATIONSHADOW;
STRUCT!{struct WINSTATIONPRODID {
    DigProductId: [WCHAR; CLIENT_PRODUCT_ID_LENGTH],
    ClientDigProductId: [WCHAR; CLIENT_PRODUCT_ID_LENGTH],
    OuterMostDigProductId: [WCHAR; CLIENT_PRODUCT_ID_LENGTH],
    CurrentSessionId: ULONG,
    ClientSessionId: ULONG,
    OuterMostSessionId: ULONG,
}}
pub type PWINSTATIONPRODID = *mut WINSTATIONPRODID;
STRUCT!{struct WINSTATIONREMOTEADDRESS_u_ipv4 {
    sin_port: USHORT,
    sin_addr: ULONG,
    sin_zero: [UCHAR; 8],
}}
STRUCT!{struct WINSTATIONREMOTEADDRESS_u_ipv6 {
    sin6_port: USHORT,
    sin6_flowinfo: ULONG,
    sin6_addr: [USHORT; 8],
    sin6_scope_id: ULONG,
}}
UNION!{union WINSTATIONREMOTEADDRESS_u {
    ipv4: WINSTATIONREMOTEADDRESS_u_ipv4,
    ipv6: WINSTATIONREMOTEADDRESS_u_ipv6,
}}
STRUCT!{struct WINSTATIONREMOTEADDRESS {
    sin_family: USHORT,
    u: WINSTATIONREMOTEADDRESS_u,
}}
pub type PWINSTATIONREMOTEADDRESS = *mut WINSTATIONREMOTEADDRESS;
STRUCT!{struct WINSTATIONINFORMATIONEX_LEVEL1 {
    SessionId: ULONG,
    SessionState: WINSTATIONSTATECLASS,
    SessionFlags: LONG,
    WinStationName: WINSTATIONNAME,
    UserName: [WCHAR; USERNAME_LENGTH + 1],
    DomainName: [WCHAR; DOMAIN_LENGTH + 1],
    LogonTime: LARGE_INTEGER,
    ConnectTime: LARGE_INTEGER,
    DisconnectTime: LARGE_INTEGER,
    LastInputTime: LARGE_INTEGER,
    CurrentTime: LARGE_INTEGER,
    ProtocolStatus: PROTOCOLSTATUS,
}}
pub type PWINSTATIONINFORMATIONEX_LEVEL1 = *mut WINSTATIONINFORMATIONEX_LEVEL1;
STRUCT!{struct WINSTATIONINFORMATIONEX_LEVEL2 {
    SessionId: ULONG,
    SessionState: WINSTATIONSTATECLASS,
    SessionFlags: LONG,
    WinStationName: WINSTATIONNAME,
    SamCompatibleUserName: [WCHAR; USERNAME_LENGTH + 1],
    SamCompatibleDomainName: [WCHAR; DOMAIN_LENGTH + 1],
    LogonTime: LARGE_INTEGER,
    ConnectTime: LARGE_INTEGER,
    DisconnectTime: LARGE_INTEGER,
    LastInputTime: LARGE_INTEGER,
    CurrentTime: LARGE_INTEGER,
    ProtocolStatus: PROTOCOLSTATUS,
    UserName: [WCHAR; 257],
    DomainName: [WCHAR; 256],
}}
pub type PWINSTATIONINFORMATIONEX_LEVEL2 = *mut WINSTATIONINFORMATIONEX_LEVEL2;
UNION!{union WINSTATIONINFORMATIONEX_LEVEL {
    WinStationInfoExLevel1: WINSTATIONINFORMATIONEX_LEVEL1,
    WinStationInfoExLevel2: WINSTATIONINFORMATIONEX_LEVEL2,
}}
pub type PWINSTATIONINFORMATIONEX_LEVEL = *mut WINSTATIONINFORMATIONEX_LEVEL;
STRUCT!{struct WINSTATIONINFORMATIONEX {
    Level: ULONG,
    Data: WINSTATIONINFORMATIONEX_LEVEL,
}}
pub type PWINSTATIONINFORMATIONEX = *mut WINSTATIONINFORMATIONEX;
pub const TS_PROCESS_INFO_MAGIC_NT4: u32 = 0x23495452;
STRUCT!{struct TS_PROCESS_INFORMATION_NT4 {
    MagicNumber: ULONG,
    LogonId: ULONG,
    ProcessSid: PVOID,
    Pad: ULONG,
}}
pub type PTS_PROCESS_INFORMATION_NT4 = *mut TS_PROCESS_INFORMATION_NT4;
pub const SIZEOF_TS4_SYSTEM_THREAD_INFORMATION: u32 = 64;
pub const SIZEOF_TS4_SYSTEM_PROCESS_INFORMATION: u32 = 136;
STRUCT!{struct TS_SYS_PROCESS_INFORMATION {
    NextEntryOffset: ULONG,
    NumberOfThreads: ULONG,
    SpareLi1: LARGE_INTEGER,
    SpareLi2: LARGE_INTEGER,
    SpareLi3: LARGE_INTEGER,
    CreateTime: LARGE_INTEGER,
    UserTime: LARGE_INTEGER,
    KernelTime: LARGE_INTEGER,
    ImageName: UNICODE_STRING,
    BasePriority: LONG,
    UniqueProcessId: ULONG,
    InheritedFromUniqueProcessId: ULONG,
    HandleCount: ULONG,
    SessionId: ULONG,
    SpareUl3: ULONG,
    PeakVirtualSize: SIZE_T,
    VirtualSize: SIZE_T,
    PageFaultCount: ULONG,
    PeakWorkingSetSize: ULONG,
    WorkingSetSize: ULONG,
    QuotaPeakPagedPoolUsage: SIZE_T,
    QuotaPagedPoolUsage: SIZE_T,
    QuotaPeakNonPagedPoolUsage: SIZE_T,
    QuotaNonPagedPoolUsage: SIZE_T,
    PagefileUsage: SIZE_T,
    PeakPagefileUsage: SIZE_T,
    PrivatePageCount: SIZE_T,
}}
pub type PTS_SYS_PROCESS_INFORMATION = *mut TS_SYS_PROCESS_INFORMATION;
STRUCT!{struct TS_ALL_PROCESSES_INFO {
    pTsProcessInfo: PTS_SYS_PROCESS_INFORMATION,
    SizeOfSid: ULONG,
    pSid: PSID,
}}
pub type PTS_ALL_PROCESSES_INFO = *mut TS_ALL_PROCESSES_INFO;
STRUCT!{struct TS_COUNTER_HEADER {
    dwCounterID: DWORD,
    bResult: BOOLEAN,
}}
pub type PTS_COUNTER_HEADER = *mut TS_COUNTER_HEADER;
STRUCT!{struct TS_COUNTER {
    CounterHead: TS_COUNTER_HEADER,
    dwValue: DWORD,
    StartTime: LARGE_INTEGER,
}}
pub type PTS_COUNTER = *mut TS_COUNTER;
pub const WSD_LOGOFF: ULONG = 0x1;
pub const WSD_SHUTDOWN: ULONG = 0x2;
pub const WSD_REBOOT: ULONG = 0x4;
pub const WSD_POWEROFF: ULONG = 0x8;
pub const WEVENT_NONE: ULONG = 0x0;
pub const WEVENT_CREATE: ULONG = 0x1;
pub const WEVENT_DELETE: ULONG = 0x2;
pub const WEVENT_RENAME: ULONG = 0x4;
pub const WEVENT_CONNECT: ULONG = 0x8;
pub const WEVENT_DISCONNECT: ULONG = 0x10;
pub const WEVENT_LOGON: ULONG = 0x20;
pub const WEVENT_LOGOFF: ULONG = 0x40;
pub const WEVENT_STATECHANGE: ULONG = 0x80;
pub const WEVENT_LICENSE: ULONG = 0x100;
pub const WEVENT_ALL: ULONG = 0x7fffffff;
pub const WEVENT_FLUSH: ULONG = 0x80000000;
pub const KBDSHIFT: USHORT = 0x1;
pub const KBDCTRL: USHORT = 0x2;
pub const KBDALT: USHORT = 0x4;
pub const WNOTIFY_ALL_SESSIONS: ULONG = 0x1;
pub const LOGONID_CURRENT: i32 = -1;
pub const SERVERNAME_CURRENT: PWSTR = null_mut();
EXTERN!{extern "system" {
    fn WinStationFreeMemory(
        Buffer: PVOID,
    ) -> BOOLEAN;
    fn WinStationOpenServerW(
        ServerName: PWSTR,
    ) -> HANDLE;
    fn WinStationCloseServer(
        ServerHandle: HANDLE,
    ) -> BOOLEAN;
    fn WinStationServerPing(
        ServerHandle: HANDLE,
    ) -> BOOLEAN;
    fn WinStationGetTermSrvCountersValue(
        ServerHandle: HANDLE,
        Count: ULONG,
        Counters: PTS_COUNTER,
    ) -> BOOLEAN;
    fn WinStationShutdownSystem(
        ServerHandle: HANDLE,
        ShutdownFlags: ULONG,
    ) -> BOOLEAN;
    fn WinStationWaitSystemEvent(
        ServerHandle: HANDLE,
        EventMask: ULONG,
        EventFlags: PULONG,
    ) -> BOOLEAN;
    fn WinStationRegisterConsoleNotification(
        ServerHandle: HANDLE,
        WindowHandle: HWND,
        Flags: ULONG,
    ) -> BOOLEAN;
    fn WinStationUnRegisterConsoleNotification(
        ServerHandle: HANDLE,
        WindowHandle: HWND,
    ) -> BOOLEAN;
    fn WinStationEnumerateW(
        ServerHandle: HANDLE,
        SessionIds: *mut PSESSIONIDW,
        Count: PULONG,
    ) -> BOOLEAN;
    fn WinStationQueryInformationW(
        ServerHandle: HANDLE,
        SessionId: ULONG,
        WinStationInformationClass: WINSTATIONINFOCLASS,
        pWinStationInformation: PVOID,
        WinStationInformationLength: ULONG,
        pReturnLength: PULONG,
    ) -> BOOLEAN;
    fn WinStationSetInformationW(
        ServerHandle: HANDLE,
        SessionId: ULONG,
        WinStationInformationClass: WINSTATIONINFOCLASS,
        pWinStationInformation: PVOID,
        WinStationInformationLength: ULONG,
    ) -> BOOLEAN;
    fn WinStationNameFromLogonIdW(
        ServerHandle: HANDLE,
        SessionId: ULONG,
        pWinStationName: PWSTR,
    ) -> BOOLEAN;
    fn WinStationSendMessageW(
        ServerHandle: HANDLE,
        SessionId: ULONG,
        Title: PWSTR,
        TitleLength: ULONG,
        Message: PWSTR,
        MessageLength: ULONG,
        Style: ULONG,
        Timeout: ULONG,
        Response: PULONG,
        DoNotWait: BOOLEAN,
    ) -> BOOLEAN;
    fn WinStationConnectW(
        ServerHandle: HANDLE,
        SessionId: ULONG,
        TargetSessionId: ULONG,
        pPassword: PWSTR,
        bWait: BOOLEAN,
    ) -> BOOLEAN;
    fn WinStationDisconnect(
        ServerHandle: HANDLE,
        SessionId: ULONG,
        bWait: BOOLEAN,
    ) -> BOOLEAN;
    fn WinStationReset(
        ServerHandle: HANDLE,
        SessionId: ULONG,
        bWait: BOOLEAN,
    ) -> BOOLEAN;
    fn WinStationShadow(
        ServerHandle: HANDLE,
        TargetServerName: PWSTR,
        TargetSessionId: ULONG,
        HotKeyVk: UCHAR,
        HotkeyModifiers: USHORT,
    ) -> BOOLEAN;
    fn WinStationShadowStop(
        ServerHandle: HANDLE,
        SessionId: ULONG,
        bWait: BOOLEAN,
    ) -> BOOLEAN;
    fn WinStationEnumerateProcesses(
        ServerHandle: HANDLE,
        Processes: *mut PVOID,
    ) -> BOOLEAN;
    fn WinStationGetAllProcesses(
        ServerHandle: HANDLE,
        Level: ULONG,
        NumberOfProcesses: PULONG,
        Processes: *mut PTS_ALL_PROCESSES_INFO,
    ) -> BOOLEAN;
    fn WinStationFreeGAPMemory(
        Level: ULONG,
        Processes: PTS_ALL_PROCESSES_INFO,
        NumberOfProcesses: ULONG,
    ) -> BOOLEAN;
    fn WinStationTerminateProcess(
        ServerHandle: HANDLE,
        ProcessId: ULONG,
        ExitCode: ULONG,
    ) -> BOOLEAN;
    fn WinStationGetProcessSid(
        ServerHandle: HANDLE,
        ProcessId: ULONG,
        ProcessStartTime: FILETIME,
        pProcessUserSid: PVOID,
        dwSidSize: PULONG,
    ) -> BOOLEAN;
    fn WinStationSwitchToServicesSession() -> BOOLEAN;
    fn WinStationRevertFromServicesSession() -> BOOLEAN;
    fn _WinStationWaitForConnect() -> BOOLEAN;
}}
