use core_foundation_sys::array::CFArrayRef;
use core_foundation_sys::base::CFAllocatorRef;
#[cfg(target_os = "macos")]
use core_foundation_sys::base::CFTypeRef;
use core_foundation_sys::base::{Boolean, OSStatus};
use std::os::raw::{c_char, c_int, c_void};

use crate::cipher_suite::SSLCipherSuite;
use crate::trust::SecTrustRef;

pub enum SSLContext {}
pub type SSLContextRef = *mut SSLContext;

pub type SSLConnectionRef = *const c_void;

pub type SSLProtocol = c_int;
pub const kSSLProtocolUnknown: SSLProtocol = 0;
pub const kSSLProtocol3: SSLProtocol = 2;
pub const kTLSProtocol1: SSLProtocol = 4;
pub const kTLSProtocol11: SSLProtocol = 7;
pub const kTLSProtocol12: SSLProtocol = 8;
pub const kDTLSProtocol1: SSLProtocol = 9;
pub const kTLSProtocol13: SSLProtocol = 10;
pub const kSSLProtocol2: SSLProtocol = 1;
pub const kSSLProtocol3Only: SSLProtocol = 3;
pub const kTLSProtocol1Only: SSLProtocol = 5;
pub const kSSLProtocolAll: SSLProtocol = 6;

pub type SSLSessionOption = c_int;
#[deprecated(note = "deprecated by Apple")]
pub const kSSLSessionOptionBreakOnServerAuth: SSLSessionOption = 0;
#[deprecated(note = "deprecated by Apple")]
pub const kSSLSessionOptionBreakOnCertRequested: SSLSessionOption = 1;
#[deprecated(note = "deprecated by Apple")]
pub const kSSLSessionOptionBreakOnClientAuth: SSLSessionOption = 2;
#[deprecated(note = "deprecated by Apple")]
pub const kSSLSessionOptionFalseStart: SSLSessionOption = 3;
pub const kSSLSessionOptionSendOneByteRecord: SSLSessionOption = 4;
#[deprecated(note = "deprecated by Apple")]
pub const kSSLSessionOptionAllowServerIdentityChange: SSLSessionOption = 5;
#[cfg(target_os = "macos")]
#[deprecated(note = "deprecated by Apple")]
pub const kSSLSessionOptionFallback: SSLSessionOption = 6;
#[deprecated(note = "deprecated by Apple")]
pub const kSSLSessionOptionBreakOnClientHello: SSLSessionOption = 7;

pub type SSLSessionState = c_int;
pub const kSSLIdle: SSLSessionState = 0;
pub const kSSLHandshake: SSLSessionState = 1;
pub const kSSLConnected: SSLSessionState = 2;
pub const kSSLClosed: SSLSessionState = 3;
pub const kSSLAborted: SSLSessionState = 4;

pub type SSLReadFunc = unsafe extern "C" fn(
    connection: SSLConnectionRef,
    data: *mut c_void,
    dataLength: *mut usize,
) -> OSStatus;

pub type SSLWriteFunc = unsafe extern "C" fn(
    connection: SSLConnectionRef,
    data: *const c_void,
    dataLength: *mut usize,
) -> OSStatus;

pub type SSLProtocolSide = c_int;
pub const kSSLServerSide: SSLProtocolSide = 0;
pub const kSSLClientSide: SSLProtocolSide = 1;

pub type SSLConnectionType = c_int;
pub const kSSLStreamType: SSLConnectionType = 0;
pub const kSSLDatagramType: SSLConnectionType = 1;

pub const errSSLProtocol: OSStatus = -9800;
pub const errSSLNegotiation: OSStatus = -9801;
pub const errSSLFatalAlert: OSStatus = -9802;
pub const errSSLWouldBlock: OSStatus = -9803;
pub const errSSLSessionNotFound: OSStatus = -9804;
pub const errSSLClosedGraceful: OSStatus = -9805;
pub const errSSLClosedAbort: OSStatus = -9806;
pub const errSSLXCertChainInvalid: OSStatus = -9807;
pub const errSSLBadCert: OSStatus = -9808;
pub const errSSLCrypto: OSStatus = -9809;
pub const errSSLInternal: OSStatus = -9810;
pub const errSSLModuleAttach: OSStatus = -9811;
pub const errSSLUnknownRootCert: OSStatus = -9812;
pub const errSSLNoRootCert: OSStatus = -9813;
pub const errSSLCertExpired: OSStatus = -9814;
pub const errSSLCertNotYetValid: OSStatus = -9815;
pub const errSSLClosedNoNotify: OSStatus = -9816;
pub const errSSLBufferOverflow: OSStatus = -9817;
pub const errSSLBadCipherSuite: OSStatus = -9818;
pub const errSSLPeerUnexpectedMsg: OSStatus = -9819;
pub const errSSLPeerBadRecordMac: OSStatus = -9820;
pub const errSSLPeerDecryptionFail: OSStatus = -9821;
pub const errSSLPeerRecordOverflow: OSStatus = -9822;
pub const errSSLPeerDecompressFail: OSStatus = -9823;
pub const errSSLPeerHandshakeFail: OSStatus = -9824;
pub const errSSLPeerBadCert: OSStatus = -9825;
pub const errSSLPeerUnsupportedCert: OSStatus = -9826;
pub const errSSLPeerCertRevoked: OSStatus = -9827;
pub const errSSLPeerCertExpired: OSStatus = -9828;
pub const errSSLPeerCertUnknown: OSStatus = -9829;
pub const errSSLIllegalParam: OSStatus = -9830;
pub const errSSLPeerUnknownCA: OSStatus = -9831;
pub const errSSLPeerAccessDenied: OSStatus = -9832;
pub const errSSLPeerDecodeError: OSStatus = -9833;
pub const errSSLPeerDecryptError: OSStatus = -9834;
pub const errSSLPeerExportRestriction: OSStatus = -9835;
pub const errSSLPeerProtocolVersion: OSStatus = -9836;
pub const errSSLPeerInsufficientSecurity: OSStatus = -9837;
pub const errSSLPeerInternalError: OSStatus = -9838;
pub const errSSLPeerUserCancelled: OSStatus = -9839;
pub const errSSLPeerNoRenegotiation: OSStatus = -9840;
pub const errSSLPeerAuthCompleted: OSStatus = -9841;
pub const errSSLClientCertRequested: OSStatus = -9842;
pub const errSSLHostNameMismatch: OSStatus = -9843;
pub const errSSLConnectionRefused: OSStatus = -9844;
pub const errSSLDecryptionFail: OSStatus = -9845;
pub const errSSLBadRecordMac: OSStatus = -9846;
pub const errSSLRecordOverflow: OSStatus = -9847;
pub const errSSLBadConfiguration: OSStatus = -9848;
pub const errSSLClientHelloReceived: OSStatus = -9851;

pub type SSLAuthenticate = c_int;
pub const kNeverAuthenticate: SSLAuthenticate = 0;
pub const kAlwaysAuthenticate: SSLAuthenticate = 1;
pub const kTryAuthenticate: SSLAuthenticate = 2;

pub type SSLClientCertificateState = c_int;
pub const kSSLClientCertNone: SSLClientCertificateState = 0;
pub const kSSLClientCertRequested: SSLClientCertificateState = 1;
pub const kSSLClientCertSent: SSLClientCertificateState = 2;
pub const kSSLClientCertRejected: SSLClientCertificateState = 3;

extern "C" {
    pub fn SSLContextGetTypeID() -> ::core_foundation_sys::base::CFTypeID;
    pub fn SSLCreateContext(
        alloc: CFAllocatorRef,
        protocolSide: SSLProtocolSide,
        connectionType: SSLConnectionType,
    ) -> SSLContextRef;
    #[cfg(target_os = "macos")]
    pub fn SSLNewContext(isServer: Boolean, contextPtr: *mut SSLContextRef) -> OSStatus;
    #[cfg(target_os = "macos")]
    pub fn SSLDisposeContext(context: SSLContextRef) -> OSStatus;
    pub fn SSLSetConnection(context: SSLContextRef, connection: SSLConnectionRef) -> OSStatus;
    pub fn SSLGetConnection(context: SSLContextRef, connection: *mut SSLConnectionRef) -> OSStatus;
    pub fn SSLSetIOFuncs(
        context: SSLContextRef,
        read: SSLReadFunc,
        write: SSLWriteFunc,
    ) -> OSStatus;
    pub fn SSLHandshake(context: SSLContextRef) -> OSStatus;
    pub fn SSLClose(context: SSLContextRef) -> OSStatus;
    pub fn SSLRead(
        context: SSLContextRef,
        data: *mut c_void,
        dataLen: usize,
        processed: *mut usize,
    ) -> OSStatus;
    pub fn SSLWrite(
        context: SSLContextRef,
        data: *const c_void,
        dataLen: usize,
        processed: *mut usize,
    ) -> OSStatus;
    pub fn SSLSetPeerDomainName(
        context: SSLContextRef,
        peerName: *const c_char,
        peerNameLen: usize,
    ) -> OSStatus;
    pub fn SSLGetPeerDomainNameLength(context: SSLContextRef, peerNameLen: *mut usize) -> OSStatus;
    pub fn SSLGetPeerDomainName(
        context: SSLContextRef,
        peerName: *mut c_char,
        peerNameLen: *mut usize,
    ) -> OSStatus;
    pub fn SSLSetCertificate(context: SSLContextRef, certRefs: CFArrayRef) -> OSStatus;
    #[cfg(target_os = "macos")]
    pub fn SSLSetCertificateAuthorities(
        context: SSLContextRef,
        certificateOrArray: CFTypeRef,
        replaceExisting: Boolean,
    ) -> OSStatus;
    #[cfg(target_os = "macos")]
    pub fn SSLCopyCertificateAuthorities(
        context: SSLContextRef,
        certificates: *mut CFArrayRef,
    ) -> OSStatus;
    pub fn SSLSetSessionOption(
        context: SSLContextRef,
        option: SSLSessionOption,
        value: Boolean,
    ) -> OSStatus;
    pub fn SSLGetSessionOption(
        context: SSLContextRef,
        option: SSLSessionOption,
        value: *mut Boolean,
    ) -> OSStatus;
    pub fn SSLCopyPeerTrust(context: SSLContextRef, trust: *mut SecTrustRef) -> OSStatus;
    pub fn SSLGetSessionState(context: SSLContextRef, state: *mut SSLSessionState) -> OSStatus;
    pub fn SSLGetSupportedCiphers(
        context: SSLContextRef,
        ciphers: *mut SSLCipherSuite,
        numCiphers: *mut usize,
    ) -> OSStatus;
    pub fn SSLGetNumberSupportedCiphers(
        context: SSLContextRef,
        numCiphers: *mut usize,
    ) -> OSStatus;
    pub fn SSLGetEnabledCiphers(
        context: SSLContextRef,
        ciphers: *mut SSLCipherSuite,
        numCiphers: *mut usize,
    ) -> OSStatus;
    pub fn SSLGetNumberEnabledCiphers(context: SSLContextRef, numCiphers: *mut usize) -> OSStatus;
    pub fn SSLSetEnabledCiphers(
        context: SSLContextRef,
        ciphers: *const SSLCipherSuite,
        numCiphers: usize,
    ) -> OSStatus;
    pub fn SSLGetNegotiatedCipher(context: SSLContextRef, cipher: *mut SSLCipherSuite) -> OSStatus;
    pub fn SSLSetClientSideAuthenticate(context: SSLContextRef, auth: SSLAuthenticate) -> OSStatus;
    #[cfg(target_os = "macos")]
    pub fn SSLSetDiffieHellmanParams(
        context: SSLContextRef,
        dhParams: *const c_void,
        dhParamsLen: usize,
    ) -> OSStatus;
    #[cfg(target_os = "macos")]
    pub fn SSLGetDiffieHellmanParams(
        context: SSLContextRef,
        dhParams: *mut *const c_void,
        dhParamsLen: *mut usize,
    ) -> OSStatus;
    pub fn SSLSetPeerID(
        context: SSLContextRef,
        peerID: *const c_void,
        peerIDLen: usize,
    ) -> OSStatus;
    pub fn SSLGetPeerID(
        context: SSLContextRef,
        peerID: *mut *const c_void,
        peerIDLen: *mut usize,
    ) -> OSStatus;
    pub fn SSLGetBufferedReadSize(context: SSLContextRef, bufSize: *mut usize) -> OSStatus;
    pub fn SSLGetClientCertificateState(
        context: SSLContextRef,
        clientState: *mut SSLClientCertificateState,
    ) -> OSStatus;
    pub fn SSLGetNegotiatedProtocolVersion(
        context: SSLContextRef,
        protocol: *mut SSLProtocol,
    ) -> OSStatus;
    pub fn SSLGetProtocolVersionMax(
        context: SSLContextRef,
        maxVersion: *mut SSLProtocol,
    ) -> OSStatus;
    pub fn SSLGetProtocolVersionMin(
        context: SSLContextRef,
        minVersion: *mut SSLProtocol,
    ) -> OSStatus;
    pub fn SSLSetProtocolVersionMax(context: SSLContextRef, maxVersion: SSLProtocol) -> OSStatus;
    pub fn SSLSetProtocolVersionMin(context: SSLContextRef, minVersion: SSLProtocol) -> OSStatus;
    #[cfg(target_os = "macos")]
    pub fn SSLSetProtocolVersionEnabled(
        context: SSLContextRef,
        protocol: SSLProtocol,
        enable: Boolean,
    ) -> OSStatus;
    #[cfg(feature = "OSX_10_13")]
    pub fn SSLSetALPNProtocols(context: SSLContextRef, protocols: CFArrayRef) -> OSStatus;
    #[cfg(feature = "OSX_10_13")]
    pub fn SSLCopyALPNProtocols(context: SSLContextRef, protocols: *mut CFArrayRef) -> OSStatus;
    #[cfg(feature = "OSX_10_13")]
    pub fn SSLSetSessionTicketsEnabled(context: SSLContextRef, enabled: Boolean) -> OSStatus;
}
