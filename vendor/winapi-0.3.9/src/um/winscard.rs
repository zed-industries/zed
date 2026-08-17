// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Data Protection API Prototypes and Definitions
// This header file provides the definitions and symbols necessary for an
// Application or Smart Card Service Provider to access the Smartcard Subsystem.
use shared::basetsd::ULONG_PTR;
use shared::guiddef::{LPCGUID, LPGUID};
use shared::minwindef::{BOOL, BYTE, DWORD, LPBYTE, LPCVOID, LPDWORD, LPVOID, PBYTE};
use shared::rpcdce::UUID;
use shared::windef::{HICON, HWND};
use um::winnt::{CHAR, HANDLE, LONG, LPCSTR, LPCWSTR, LPSTR, LPWSTR, PVOID, WCHAR};
use um::winsmcrd::{LPCSCARD_IO_REQUEST, LPSCARD_IO_REQUEST};
pub type LPCBYTE = *const BYTE;
pub type SCARDCONTEXT = ULONG_PTR;
pub type PSCARDCONTEXT = *mut SCARDCONTEXT;
pub type LPSCARDCONTEXT = *mut SCARDCONTEXT;
pub type SCARDHANDLE = ULONG_PTR;
pub type PSCARDHANDLE = *mut SCARDHANDLE;
pub type LPSCARDHANDLE = *mut SCARDHANDLE;
pub const SCARD_AUTOALLOCATE: DWORD = -1i32 as u32;
pub const SCARD_SCOPE_USER: DWORD = 0;
pub const SCARD_SCOPE_TERMINAL: DWORD = 1;
pub const SCARD_SCOPE_SYSTEM: DWORD = 2;
extern "system" {
    pub fn SCardEstablishContext(
        dwScope: DWORD,
        pvReserved1: LPCVOID,
        pvReserved2: LPCVOID,
        phContext: LPSCARDCONTEXT,
    ) -> LONG;
    pub fn SCardReleaseContext(
        hContext: SCARDCONTEXT,
    ) -> LONG;
    pub fn SCardIsValidContext(
        hContext: SCARDCONTEXT,
    ) -> LONG;
}
pub const SCARD_PROVIDER_PRIMARY: DWORD = 1;
pub const SCARD_PROVIDER_CSP: DWORD = 2;
pub const SCARD_PROVIDER_KSP: DWORD = 3;
extern "system" {
    pub fn SCardListReaderGroupsA(
        hContext: SCARDCONTEXT,
        mszGroups: LPSTR,
        pcchGroups: LPDWORD,
    ) -> LONG;
    pub fn SCardListReaderGroupsW(
        hContext: SCARDCONTEXT,
        mszGroups: LPWSTR,
        pcchGroups: LPDWORD,
    ) -> LONG;
    pub fn SCardListReadersA(
        hContext: SCARDCONTEXT,
        mszGroups: LPCSTR,
        mszReaders: LPSTR,
        pcchReaders: LPDWORD,
    ) -> LONG;
    pub fn SCardListReadersW(
        hContext: SCARDCONTEXT,
        mszGroups: LPCWSTR,
        mszReaders: LPWSTR,
        pcchReaders: LPDWORD,
    ) -> LONG;
    pub fn SCardListCardsA(
        hContext: SCARDCONTEXT,
        pbAtr: LPCBYTE,
        rgquidInterfaces: LPCGUID,
        cguidInterfaceCount: DWORD,
        mszCards: *mut CHAR,
        pcchCards: LPDWORD,
    ) -> LONG;
    pub fn SCardListCardsW(
        hContext: SCARDCONTEXT,
        pbAtr: LPCBYTE,
        rgquidInterfaces: LPCGUID,
        cguidInterfaceCount: DWORD,
        mszCards: *mut WCHAR,
        pcchCards: LPDWORD,
    ) -> LONG;
    pub fn SCardListInterfacesA(
        hContext: SCARDCONTEXT,
        szCard: LPCSTR,
        pguidInterfaces: LPGUID,
        pcguidInterfaces: LPDWORD,
    ) -> LONG;
    pub fn SCardListInterfacesW(
        hContext: SCARDCONTEXT,
        szCard: LPCWSTR,
        pguidInterfaces: LPGUID,
        pcguidInterfaces: LPDWORD,
    ) -> LONG;
    pub fn SCardGetProviderIdA(
        hContext: SCARDCONTEXT,
        szCard: LPCSTR,
        pguidProviderId: LPGUID,
    ) -> LONG;
    pub fn SCardGetProviderIdW(
        hContext: SCARDCONTEXT,
        szCard: LPCWSTR,
        pguidProviderId: LPGUID,
    ) -> LONG;
    pub fn SCardGetCardTypeProviderNameA(
        hContext: SCARDCONTEXT,
        szCardName: LPCSTR,
        dwProviderId: DWORD,
        szProvider: *mut CHAR,
        pcchProvider: LPDWORD,
    ) -> LONG;
    pub fn SCardGetCardTypeProviderNameW(
        hContext: SCARDCONTEXT,
        szCardName: LPCWSTR,
        dwProviderId: DWORD,
        szProvider: *mut WCHAR,
        pcchProvider: LPDWORD,
    ) -> LONG;
    pub fn SCardIntroduceReaderGroupA(
        hContext: SCARDCONTEXT,
        szGroupName: LPCSTR,
    ) -> LONG;
    pub fn SCardIntroduceReaderGroupW(
        hContext: SCARDCONTEXT,
        szGroupName: LPCWSTR,
    ) -> LONG;
    pub fn SCardForgetReaderGroupA(
        hContext: SCARDCONTEXT,
        szGroupName: LPCSTR,
    ) -> LONG;
    pub fn SCardForgetReaderGroupW(
        hContext: SCARDCONTEXT,
        szGroupName: LPCWSTR,
    ) -> LONG;
    pub fn SCardIntroduceReaderA(
        hContext: SCARDCONTEXT,
        szReaderName: LPCSTR,
        szDeviceName: LPCSTR,
    ) -> LONG;
    pub fn SCardIntroduceReaderW(
        hContext: SCARDCONTEXT,
        szReaderName: LPCWSTR,
        szDeviceName: LPCWSTR,
    ) -> LONG;
    pub fn SCardForgetReaderA(
        hContext: SCARDCONTEXT,
        szReaderName: LPCSTR,
    ) -> LONG;
    pub fn SCardForgetReaderW(
        hContext: SCARDCONTEXT,
        szReaderName: LPCWSTR,
    ) -> LONG;
    pub fn SCardAddReaderToGroupA(
        hContext: SCARDCONTEXT,
        szReaderName: LPCSTR,
        szGroupName: LPCSTR,
    ) -> LONG;
    pub fn SCardAddReaderToGroupW(
        hContext: SCARDCONTEXT,
        szReaderName: LPCWSTR,
        szGroupName: LPCWSTR,
    ) -> LONG;
    pub fn SCardRemoveReaderFromGroupA(
        hContext: SCARDCONTEXT,
        szReaderName: LPCSTR,
        szGroupName: LPCSTR,
    ) -> LONG;
    pub fn SCardRemoveReaderFromGroupW(
        hContext: SCARDCONTEXT,
        szReaderName: LPCWSTR,
        szGroupName: LPCWSTR,
    ) -> LONG;
    pub fn SCardIntroduceCardTypeA(
        hContext: SCARDCONTEXT,
        szCardName: LPCSTR,
        pguidPrimaryProvider: LPCGUID,
        rgguidInterfaces: LPCGUID,
        dwInterfaceCount: DWORD,
        pbAtr: LPCBYTE,
        pbAtrMask: LPCBYTE,
        cbAtrLen: DWORD,
    ) -> LONG;
    pub fn SCardIntroduceCardTypeW(
        hContext: SCARDCONTEXT,
        szCardName: LPCWSTR,
        pguidPrimaryProvider: LPCGUID,
        rgguidInterfaces: LPCGUID,
        dwInterfaceCount: DWORD,
        pbAtr: LPCBYTE,
        pbAtrMask: LPCBYTE,
        cbAtrLen: DWORD,
    ) -> LONG;
    pub fn SCardSetCardTypeProviderNameA(
        hContext: SCARDCONTEXT,
        szCardName: LPCSTR,
        dwProviderId: DWORD,
        szProvider: LPCSTR,
    ) -> LONG;
    pub fn SCardSetCardTypeProviderNameW(
        hContext: SCARDCONTEXT,
        szCardName: LPCWSTR,
        dwProviderId: DWORD,
        szProvider: LPCWSTR,
    ) -> LONG;
    pub fn SCardForgetCardTypeA(
        hContext: SCARDCONTEXT,
        szCardName: LPCSTR,
    ) -> LONG;
    pub fn SCardForgetCardTypeW(
        hContext: SCARDCONTEXT,
        szCardName: LPCWSTR,
    ) -> LONG;
    pub fn SCardFreeMemory(
        hContext: SCARDCONTEXT,
        pvMem: LPCVOID,
    ) -> LONG;
    pub fn SCardAccessStartedEvent() -> HANDLE;
    pub fn SCardReleaseStartedEvent();
}
STRUCT!{struct SCARD_READERSTATEA {
    szReader: LPCSTR,
    pvUserData: LPVOID,
    dwCurrentState: DWORD,
    dwEventState: DWORD,
    cbAtr: DWORD,
    rgbAtr: [BYTE; 36],
}}
pub type PSCARD_READERSTATEA = *mut SCARD_READERSTATEA;
pub type LPSCARD_READERSTATEA = *mut SCARD_READERSTATEA;
STRUCT!{struct SCARD_READERSTATEW {
    szReader: LPCWSTR,
    pvUserData: LPVOID,
    dwCurrentState: DWORD,
    dwEventState: DWORD,
    cbAtr: DWORD,
    rgbAtr: [BYTE; 36],
}}
pub type PSCARD_READERSTATEW = *mut SCARD_READERSTATEW;
pub type LPSCARD_READERSTATEW = *mut SCARD_READERSTATEW;
pub type SCARD_READERSTATE_A = SCARD_READERSTATEA;
pub type SCARD_READERSTATE_W = SCARD_READERSTATEW;
pub type PSCARD_READERSTATE_A = PSCARD_READERSTATEA;
pub type PSCARD_READERSTATE_W = PSCARD_READERSTATEW;
pub type LPSCARD_READERSTATE_A = LPSCARD_READERSTATEA;
pub type LPSCARD_READERSTATE_W = LPSCARD_READERSTATEW;
pub const SCARD_STATE_UNAWARE: DWORD = 0x00000000;
pub const SCARD_STATE_IGNORE: DWORD = 0x00000001;
pub const SCARD_STATE_CHANGED: DWORD = 0x00000002;
pub const SCARD_STATE_UNKNOWN: DWORD = 0x00000004;
pub const SCARD_STATE_UNAVAILABLE: DWORD = 0x00000008;
pub const SCARD_STATE_EMPTY: DWORD = 0x00000010;
pub const SCARD_STATE_PRESENT: DWORD = 0x00000020;
pub const SCARD_STATE_ATRMATCH: DWORD = 0x00000040;
pub const SCARD_STATE_EXCLUSIVE: DWORD = 0x00000080;
pub const SCARD_STATE_INUSE: DWORD = 0x00000100;
pub const SCARD_STATE_MUTE: DWORD = 0x00000200;
pub const SCARD_STATE_UNPOWERED: DWORD = 0x00000400;
extern "system" {
    pub fn SCardLocateCardsA(
        hContext: SCARDCONTEXT,
        mszCards: LPCSTR,
        rgReaderStates: LPSCARD_READERSTATEA,
        cReaders: DWORD,
    ) -> LONG;
    pub fn SCardLocateCardsW(
        hContext: SCARDCONTEXT,
        mszCards: LPCWSTR,
        rgReaderStates: LPSCARD_READERSTATEW,
        cReaders: DWORD,
    ) -> LONG;
}
STRUCT!{struct SCARD_ATRMASK {
    cbAtr: DWORD,
    rgbAtr: [BYTE; 36],
    rgbMask: [BYTE; 36],
}}
pub type PSCARD_ATRMASK = *mut SCARD_ATRMASK;
pub type LPSCARD_ATRMASK = *mut SCARD_ATRMASK;
extern "system" {
    pub fn SCardLocateCardsByATRA(
        hContext: SCARDCONTEXT,
        rgAtrMasks: LPSCARD_ATRMASK,
        cAtrs: DWORD,
        rgReaderStates: LPSCARD_READERSTATEA,
        cReaders: DWORD,
    ) -> LONG;
    pub fn SCardLocateCardsByATRW(
        hContext: SCARDCONTEXT,
        rgAtrMasks: LPSCARD_ATRMASK,
        cAtrs: DWORD,
        rgReaderStates: LPSCARD_READERSTATEW,
        cReaders: DWORD,
    ) -> LONG;
    pub fn SCardGetStatusChangeA(
        hContext: SCARDCONTEXT,
        dwTimeout: DWORD,
        rgReaderStates: LPSCARD_READERSTATEA,
        cReaders: DWORD,
    ) -> LONG;
    pub fn SCardGetStatusChangeW(
        hContext: SCARDCONTEXT,
        dwTimeout: DWORD,
        rgReaderStates: LPSCARD_READERSTATEW,
        cReaders: DWORD,
    ) -> LONG;
    pub fn SCardCancel(
        hContext: SCARDCONTEXT,
    ) -> LONG;
}
pub const SCARD_SHARE_EXCLUSIVE: DWORD = 1;
pub const SCARD_SHARE_SHARED: DWORD = 2;
pub const SCARD_SHARE_DIRECT: DWORD = 3;
pub const SCARD_LEAVE_CARD: DWORD = 0;
pub const SCARD_RESET_CARD: DWORD = 1;
pub const SCARD_UNPOWER_CARD: DWORD = 2;
pub const SCARD_EJECT_CARD: DWORD = 3;
extern "system" {
    pub fn SCardConnectA(
        hContext: SCARDCONTEXT,
        szReader: LPCSTR,
        dwShareMode: DWORD,
        dwPreferredProtocols: DWORD,
        phCard: LPSCARDHANDLE,
        pdwActiveProtocol: LPDWORD,
    ) -> LONG;
    pub fn SCardConnectW(
        hContext: SCARDCONTEXT,
        szReader: LPCWSTR,
        dwShareMode: DWORD,
        dwPreferredProtocols: DWORD,
        phCard: LPSCARDHANDLE,
        pdwActiveProtocol: LPDWORD,
    ) -> LONG;
    pub fn SCardReconnect(
        hCard: SCARDHANDLE,
        dwShareMode: DWORD,
        dwPreferredProtocols: DWORD,
        dwInitialization: DWORD,
        pdwActiveProtocol: LPDWORD,
    ) -> LONG;
    pub fn SCardDisconnect(
        hCard: SCARDHANDLE,
        dwDisposition: DWORD,
    ) -> LONG;
    pub fn SCardBeginTransaction(
        hCard: SCARDHANDLE,
    ) -> LONG;
    pub fn SCardEndTransaction(
        hCard: SCARDHANDLE,
        dwDisposition: DWORD,
    ) -> LONG;
    pub fn SCardState(
        hCard: SCARDHANDLE,
        pdwState: LPDWORD,
        pdwProtocol: LPDWORD,
        pbAtr: LPBYTE,
        pcbAtrLen: LPDWORD,
    ) -> LONG;
    pub fn SCardStatusA(
        hCard: SCARDHANDLE,
        mszReaderNames: LPSTR,
        pcchReaderLen: LPDWORD,
        pdwState: LPDWORD,
        pdwProtocol: LPDWORD,
        pbAtr: LPBYTE,
        pcbAtrLen: LPDWORD,
    ) -> LONG;
    pub fn SCardStatusW(
        hCard: SCARDHANDLE,
        mszReaderNames: LPWSTR,
        pcchReaderLen: LPDWORD,
        pdwState: LPDWORD,
        pdwProtocol: LPDWORD,
        pbAtr: LPBYTE,
        pcbAtrLen: LPDWORD,
    ) -> LONG;
    pub fn SCardTransmit(
        hCard: SCARDHANDLE,
        pioSendPci: LPCSCARD_IO_REQUEST,
        pbSendBuffer: LPCBYTE,
        cbSendLength: DWORD,
        pioRecvPci: LPSCARD_IO_REQUEST,
        pbRecvBuffer: LPBYTE,
        pcbRecvLength: LPDWORD,
    ) -> LONG;
    pub fn SCardGetTransmitCount(
        hCard: SCARDHANDLE,
        pcTransmitCount: LPDWORD,
    ) -> LONG;
    pub fn SCardControl(
        hCard: SCARDHANDLE,
        dwControlCode: DWORD,
        lpInBuffer: LPCVOID,
        cbInBufferSize: DWORD,
        lpOutBuffer: LPVOID,
        cbOutBufferSize: DWORD,
        lpBytesReturned: LPDWORD,
    ) -> LONG;
    pub fn SCardGetAttrib(
        hCard: SCARDHANDLE,
        dwAttrId: DWORD,
        pbAttr: LPBYTE,
        pcbAttrLen: LPDWORD,
    ) -> LONG;
    pub fn SCardSetAttrib(
        hCard: SCARDHANDLE,
        dwAttrId: DWORD,
        pbAttr: LPCBYTE,
        cbAttrLen: DWORD,
    ) -> LONG;
}
pub const SC_DLG_MINIMAL_UI: DWORD = 0x01;
pub const SC_DLG_NO_UI: DWORD = 0x02;
pub const SC_DLG_FORCE_UI: DWORD = 0x04;
pub const SCERR_NOCARDNAME: DWORD = 0x4000;
pub const SCERR_NOGUIDS: DWORD = 0x8000;
FN!{stdcall LPOCNCONNPROCA(
    SCARDCONTEXT,
    LPSTR,
    LPSTR,
    PVOID,
) -> SCARDHANDLE}
FN!{stdcall LPOCNCONNPROCW(
    SCARDCONTEXT,
    LPWSTR,
    LPWSTR,
    PVOID,
) -> SCARDHANDLE}
FN!{stdcall LPOCNCHKPROC(
    SCARDCONTEXT,
    SCARDHANDLE,
    PVOID,
) -> BOOL}
FN!{stdcall LPOCNDSCPROC(
    SCARDCONTEXT,
    SCARDHANDLE,
    PVOID,
) -> ()}
STRUCT!{struct OPENCARD_SEARCH_CRITERIAA {
    dwStructSize: DWORD,
    lpstrGroupNames: LPSTR,
    nMaxGroupNames: DWORD,
    rgguidInterfaces: LPCGUID,
    cguidInterfaces: DWORD,
    lpstrCardNames: LPSTR,
    nMaxCardNames: DWORD,
    lpfnCheck: LPOCNCHKPROC,
    lpfnConnect: LPOCNCONNPROCA,
    lpfnDisconnect: LPOCNDSCPROC,
    pvUserData: LPVOID,
    dwShareMode: DWORD,
    dwPreferredProtocols: DWORD,
}}
pub type POPENCARD_SEARCH_CRITERIAA = *mut OPENCARD_SEARCH_CRITERIAA;
pub type LPOPENCARD_SEARCH_CRITERIAA = *mut OPENCARD_SEARCH_CRITERIAA;
STRUCT!{struct OPENCARD_SEARCH_CRITERIAW {
    dwStructSize: DWORD,
    lpstrGroupNames: LPWSTR,
    nMaxGroupNames: DWORD,
    rgguidInterfaces: LPCGUID,
    cguidInterfaces: DWORD,
    lpstrCardNames: LPWSTR,
    nMaxCardNames: DWORD,
    lpfnCheck: LPOCNCHKPROC,
    lpfnConnect: LPOCNCONNPROCW,
    lpfnDisconnect: LPOCNDSCPROC,
    pvUserData: LPVOID,
    dwShareMode: DWORD,
    dwPreferredProtocols: DWORD,
}}
pub type POPENCARD_SEARCH_CRITERIAW = *mut OPENCARD_SEARCH_CRITERIAW;
pub type LPOPENCARD_SEARCH_CRITERIAW = *mut OPENCARD_SEARCH_CRITERIAW;
STRUCT!{struct OPENCARDNAME_EXA {
    dwStructSize: DWORD,
    hSCardContext: SCARDCONTEXT,
    hwndOwner: HWND,
    dwFlags: DWORD,
    lpstrTitle: LPCSTR,
    lpstrSearchDesc: LPCSTR,
    hIcon: HICON,
    pOpenCardSearchCriteria: POPENCARD_SEARCH_CRITERIAA,
    lpfnConnect: LPOCNCONNPROCA,
    pvUserData: LPVOID,
    dwShareMode: DWORD,
    dwPreferredProtocols: DWORD,
    lpstrRdr: LPSTR,
    nMaxRdr: DWORD,
    lpstrCard: LPSTR,
    nMaxCard: DWORD,
    dwActiveProtocol: DWORD,
    hCardHandle: SCARDHANDLE,
}}
pub type POPENCARDNAME_EXA = *mut OPENCARDNAME_EXA;
pub type LPOPENCARDNAME_EXA = *mut OPENCARDNAME_EXA;
STRUCT!{struct OPENCARDNAME_EXW {
    dwStructSize: DWORD,
    hSCardContext: SCARDCONTEXT,
    hwndOwner: HWND,
    dwFlags: DWORD,
    lpstrTitle: LPCWSTR,
    lpstrSearchDesc: LPCWSTR,
    hIcon: HICON,
    pOpenCardSearchCriteria: POPENCARD_SEARCH_CRITERIAW,
    lpfnConnect: LPOCNCONNPROCW,
    pvUserData: LPVOID,
    dwShareMode: DWORD,
    dwPreferredProtocols: DWORD,
    lpstrRdr: LPWSTR,
    nMaxRdr: DWORD,
    lpstrCard: LPWSTR,
    nMaxCard: DWORD,
    dwActiveProtocol: DWORD,
    hCardHandle: SCARDHANDLE,
}}
pub type POPENCARDNAME_EXW = *mut OPENCARDNAME_EXW;
pub type LPOPENCARDNAME_EXW = *mut OPENCARDNAME_EXW;
pub type OPENCARDNAMEA_EX = OPENCARDNAME_EXA;
pub type OPENCARDNAMEW_EX = OPENCARDNAME_EXW;
pub type POPENCARDNAMEA_EX = POPENCARDNAME_EXA;
pub type POPENCARDNAMEW_EX = POPENCARDNAME_EXW;
pub type LPOPENCARDNAMEA_EX = LPOPENCARDNAME_EXA;
pub type LPOPENCARDNAMEW_EX = LPOPENCARDNAME_EXW;
pub const SCARD_READER_SEL_AUTH_PACKAGE: DWORD = -629i32 as u32;
ENUM!{enum READER_SEL_REQUEST_MATCH_TYPE {
    RSR_MATCH_TYPE_READER_AND_CONTAINER = 1,
    RSR_MATCH_TYPE_SERIAL_NUMBER,
    RSR_MATCH_TYPE_ALL_CARDS,
}}
STRUCT!{struct READER_SEL_REQUEST_ReaderAndContainerParameter {
    cbReaderNameOffset: DWORD,
    cchReaderNameLength: DWORD,
    cbContainerNameOffset: DWORD,
    cchContainerNameLength: DWORD,
    dwDesiredCardModuleVersion: DWORD,
    dwCspFlags: DWORD,
}}
STRUCT!{struct READER_SEL_REQUEST_SerialNumberParameter {
    cbSerialNumberOffset: DWORD,
    cbSerialNumberLength: DWORD,
    dwDesiredCardModuleVersion: DWORD,
}}
UNION!{union READER_SEL_REQUEST_u {
    [u32; 6],
    ReaderAndContainerParameter ReaderAndContainerParameter_mut:
        READER_SEL_REQUEST_ReaderAndContainerParameter,
    SerialNumberParameter SerialNumberParameter_mut: READER_SEL_REQUEST_SerialNumberParameter,
}}
STRUCT!{struct READER_SEL_REQUEST {
    dwShareMode: DWORD,
    dwPreferredProtocols: DWORD,
    MatchType: READER_SEL_REQUEST_MATCH_TYPE,
    u: READER_SEL_REQUEST_u,
}}
pub type PREADER_SEL_REQUEST = *mut READER_SEL_REQUEST;
STRUCT!{struct READER_SEL_RESPONSE {
    cbReaderNameOffset: DWORD,
    cchReaderNameLength: DWORD,
    cbCardNameOffset: DWORD,
    cchCardNameLength: DWORD,
}}
pub type PREADER_SEL_RESPONSE = *mut READER_SEL_RESPONSE;
STRUCT!{struct OPENCARDNAMEA {
    dwStructSize: DWORD,
    hwndOwner: HWND,
    hSCardContext: SCARDCONTEXT,
    lpstrGroupNames: LPSTR,
    nMaxGroupNames: DWORD,
    lpstrCardNames: LPSTR,
    nMaxCardNames: DWORD,
    rgguidInterfaces: LPCGUID,
    cguidInterfaces: DWORD,
    lpstrRdr: LPSTR,
    nMaxRdr: DWORD,
    lpstrCard: LPSTR,
    nMaxCard: DWORD,
    lpstrTitle: LPCSTR,
    dwFlags: DWORD,
    pvUserData: LPVOID,
    dwShareMode: DWORD,
    dwPreferredProtocols: DWORD,
    dwActiveProtocol: DWORD,
    lpfnConnect: LPOCNCONNPROCA,
    lpfnCheck: LPOCNCHKPROC,
    lpfnDisconnect: LPOCNDSCPROC,
    hCardHandle: SCARDHANDLE,
}}
pub type POPENCARDNAMEA = *mut OPENCARDNAMEA;
pub type LPOPENCARDNAMEA = *mut OPENCARDNAMEA;
STRUCT!{struct OPENCARDNAMEW {
    dwStructSize: DWORD,
    hwndOwner: HWND,
    hSCardContext: SCARDCONTEXT,
    lpstrGroupNames: LPWSTR,
    nMaxGroupNames: DWORD,
    lpstrCardNames: LPWSTR,
    nMaxCardNames: DWORD,
    rgguidInterfaces: LPCGUID,
    cguidInterfaces: DWORD,
    lpstrRdr: LPWSTR,
    nMaxRdr: DWORD,
    lpstrCard: LPWSTR,
    nMaxCard: DWORD,
    lpstrTitle: LPCWSTR,
    dwFlags: DWORD,
    pvUserData: LPVOID,
    dwShareMode: DWORD,
    dwPreferredProtocols: DWORD,
    dwActiveProtocol: DWORD,
    lpfnConnect: LPOCNCONNPROCW,
    lpfnCheck: LPOCNCHKPROC,
    lpfnDisconnect: LPOCNDSCPROC,
    hCardHandle: SCARDHANDLE,
}}
pub type POPENCARDNAMEW = *mut OPENCARDNAMEW;
pub type LPOPENCARDNAMEW = *mut OPENCARDNAMEW;
pub type OPENCARDNAME_A = OPENCARDNAMEA;
pub type OPENCARDNAME_W = OPENCARDNAMEW;
pub type POPENCARDNAME_A = POPENCARDNAMEA;
pub type POPENCARDNAME_W = POPENCARDNAMEW;
pub type LPOPENCARDNAME_A = LPOPENCARDNAMEA;
pub type LPOPENCARDNAME_W = LPOPENCARDNAMEW;
extern "system" {
    pub fn SCardReadCacheA(
        hContext: SCARDCONTEXT,
        CardIdentifier: *mut UUID,
        FreshnessCounter: DWORD,
        LookupName: LPSTR,
        Data: PBYTE,
        DataLen: *mut DWORD,
    ) -> LONG;
    pub fn SCardReadCacheW(
        hContext: SCARDCONTEXT,
        CardIdentifier: *mut UUID,
        FreshnessCounter: DWORD,
        LookupName: LPWSTR,
        Data: PBYTE,
        DataLen: *mut DWORD,
    ) -> LONG;
    pub fn SCardWriteCacheA(
        hContext: SCARDCONTEXT,
        CardIdentifier: *mut UUID,
        FreshnessCounter: DWORD,
        LookupName: LPSTR,
        Data: PBYTE,
        DataLen: DWORD,
    ) -> LONG;
    pub fn SCardWriteCacheW(
        hContext: SCARDCONTEXT,
        CardIdentifier: *mut UUID,
        FreshnessCounter: DWORD,
        LookupName: LPWSTR,
        Data: PBYTE,
        DataLen: DWORD,
    ) -> LONG;
    pub fn SCardGetReaderIconA(
        hContext: SCARDCONTEXT,
        szReaderName: LPCSTR,
        pbIcon: LPBYTE,
        pcbIcon: LPDWORD,
    ) -> LONG;
    pub fn SCardGetReaderIconW(
        hContext: SCARDCONTEXT,
        szReaderName: LPCWSTR,
        pbIcon: LPBYTE,
        pcbIcon: LPDWORD,
    ) -> LONG;
    pub fn SCardGetDeviceTypeIdA(
        hContext: SCARDCONTEXT,
        szReaderName: LPCSTR,
        pdwDeviceTypeId: LPDWORD,
    ) -> LONG;
    pub fn SCardGetDeviceTypeIdW(
        hContext: SCARDCONTEXT,
        szReaderName: LPCWSTR,
        pdwDeviceTypeId: LPDWORD,
    ) -> LONG;
    pub fn SCardGetReaderDeviceInstanceIdA(
        hContext: SCARDCONTEXT,
        szReaderName: LPCSTR,
        szDeviceInstanceId: LPSTR,
        pcchDeviceInstanceId: LPDWORD,
    ) -> LONG;
    pub fn SCardGetReaderDeviceInstanceIdW(
        hContext: SCARDCONTEXT,
        szReaderName: LPCWSTR,
        szDeviceInstanceId: LPWSTR,
        pcchDeviceInstanceId: LPDWORD,
    ) -> LONG;
    pub fn SCardListReadersWithDeviceInstanceIdA(
        hContext: SCARDCONTEXT,
        szDeviceInstanceId: LPCSTR,
        mszReaders: LPSTR,
        pcchReaders: LPDWORD,
    ) -> LONG;
    pub fn SCardListReadersWithDeviceInstanceIdW(
        hContext: SCARDCONTEXT,
        szDeviceInstanceId: LPCWSTR,
        mszReaders: LPWSTR,
        pcchReaders: LPDWORD,
    ) -> LONG;
}
pub const SCARD_AUDIT_CHV_FAILURE: DWORD = 0x0;
pub const SCARD_AUDIT_CHV_SUCCESS: DWORD = 0x1;
extern "system" {
    pub fn SCardAudit(
        hContext: SCARDCONTEXT,
        dwEvent: DWORD,
    ) -> LONG;
}
