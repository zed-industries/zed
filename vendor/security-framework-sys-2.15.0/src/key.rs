use core_foundation_sys::base::CFTypeID;
use core_foundation_sys::data::CFDataRef;
use core_foundation_sys::dictionary::CFDictionaryRef;
use core_foundation_sys::error::CFErrorRef;
#[cfg(any(feature = "OSX_10_12", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
use core_foundation_sys::string::CFStringRef;

use crate::base::SecKeyRef;

#[cfg(any(feature = "OSX_10_12", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
pub type SecKeyAlgorithm = CFStringRef;

#[cfg(any(feature = "OSX_10_12", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
pub type SecKeyOperationType = u32;
#[cfg(any(feature = "OSX_10_12", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
pub const kSecKeyOperationTypeSign: SecKeyOperationType = 0;
#[cfg(any(feature = "OSX_10_12", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
pub const kSecKeyOperationTypeVerify: SecKeyOperationType = 1;
#[cfg(any(feature = "OSX_10_12", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
pub const kSecKeyOperationTypeEncrypt: SecKeyOperationType = 2;
#[cfg(any(feature = "OSX_10_12", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
pub const kSecKeyOperationTypeDecrypt: SecKeyOperationType = 3;
#[cfg(any(feature = "OSX_10_12", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
pub const kSecKeyOperationTypeKeyExchange: SecKeyOperationType = 4;

extern "C" {
    pub fn SecKeyGetTypeID() -> CFTypeID;

    #[cfg(any(feature = "OSX_10_12", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
    pub fn SecKeyCreateRandomKey(parameters: CFDictionaryRef, error: *mut CFErrorRef) -> SecKeyRef;

    #[cfg(any(feature = "OSX_10_13", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
    pub fn SecKeyCreateWithData(
        keyData: CFDataRef,
        attributes: CFDictionaryRef,
        error: *mut CFErrorRef,
    ) -> SecKeyRef;

    #[cfg(target_os = "macos")]
    pub fn SecKeyCreateFromData(
        parameters: CFDictionaryRef,
        keyData: CFDataRef,
        error: *mut CFErrorRef,
    ) -> SecKeyRef;

    #[cfg(any(feature = "OSX_10_12", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
    pub fn SecKeyCopyExternalRepresentation(key: SecKeyRef, error: *mut CFErrorRef) -> CFDataRef;
    #[cfg(any(feature = "OSX_10_12", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
    pub fn SecKeyCopyAttributes(key: SecKeyRef) -> CFDictionaryRef;
    #[cfg(any(feature = "OSX_10_12", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
    pub fn SecKeyCopyPublicKey(key: SecKeyRef) -> SecKeyRef;

    #[cfg(any(feature = "OSX_10_12", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
    pub fn SecKeyCreateSignature(
        key: SecKeyRef,
        algorithm: SecKeyAlgorithm,
        dataToSign: CFDataRef,
        error: *mut CFErrorRef,
    ) -> CFDataRef;

    #[cfg(any(feature = "OSX_10_12", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
    pub fn SecKeyVerifySignature(
        key: SecKeyRef,
        algorithm: SecKeyAlgorithm,
        signedData: CFDataRef,
        signature: CFDataRef,
        error: *mut CFErrorRef,
    ) -> core_foundation_sys::base::Boolean;

    #[cfg(any(feature = "OSX_10_12", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
    pub fn SecKeyCreateEncryptedData(
        key: SecKeyRef,
        algorithm: SecKeyAlgorithm,
        plaintext: CFDataRef,
        error: *mut CFErrorRef,
    ) -> CFDataRef;

    #[cfg(any(feature = "OSX_10_12", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
    pub fn SecKeyCreateDecryptedData(
        key: SecKeyRef,
        algorithm: SecKeyAlgorithm,
        ciphertext: CFDataRef,
        error: *mut CFErrorRef,
    ) -> CFDataRef;

    #[cfg(any(feature = "OSX_10_12", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
    pub fn SecKeyIsAlgorithmSupported(
        key: SecKeyRef,
        operation: SecKeyOperationType,
        algorithm: SecKeyAlgorithm,
    ) -> core_foundation_sys::base::Boolean;

    #[cfg(any(feature = "OSX_10_12", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
    pub fn SecKeyCopyKeyExchangeResult(
        privateKey: SecKeyRef,
        algorithm: SecKeyAlgorithm,
        publicKey: SecKeyRef,
        parameters: CFDictionaryRef,
        error: *mut CFErrorRef,
    ) -> CFDataRef;
}

#[cfg(any(feature = "OSX_10_12", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
macro_rules! names {
    ($( $(# $meta:literal )* $i:ident => $x:ident),*) => {
        extern "C" {
            $($(#[cfg(feature = $meta)])* pub static $x: SecKeyAlgorithm;)*
        }

        #[non_exhaustive]
        #[derive(Copy, Clone)]
        pub enum Algorithm {
            $( $(#[cfg(feature = $meta)])* $i, )*
        }

        impl From<Algorithm> for SecKeyAlgorithm {
            fn from(m: Algorithm) -> Self {
                unsafe { match m {
                    $( $(#[cfg(feature = $meta)])* Algorithm::$i => $x, )*
                } }
            }
        }
    }
}

#[cfg(any(feature = "OSX_10_12", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
names! {
    ECIESEncryptionStandardX963SHA1AESGCM => kSecKeyAlgorithmECIESEncryptionStandardX963SHA1AESGCM,
    ECIESEncryptionStandardX963SHA224AESGCM => kSecKeyAlgorithmECIESEncryptionStandardX963SHA224AESGCM,
    ECIESEncryptionStandardX963SHA256AESGCM => kSecKeyAlgorithmECIESEncryptionStandardX963SHA256AESGCM,
    ECIESEncryptionStandardX963SHA384AESGCM => kSecKeyAlgorithmECIESEncryptionStandardX963SHA384AESGCM,
    ECIESEncryptionStandardX963SHA512AESGCM => kSecKeyAlgorithmECIESEncryptionStandardX963SHA512AESGCM,

    ECIESEncryptionStandardVariableIVX963SHA224AESGCM => kSecKeyAlgorithmECIESEncryptionStandardVariableIVX963SHA224AESGCM,
    ECIESEncryptionStandardVariableIVX963SHA256AESGCM => kSecKeyAlgorithmECIESEncryptionStandardVariableIVX963SHA256AESGCM,
    ECIESEncryptionStandardVariableIVX963SHA384AESGCM => kSecKeyAlgorithmECIESEncryptionStandardVariableIVX963SHA384AESGCM,
    ECIESEncryptionStandardVariableIVX963SHA512AESGCM => kSecKeyAlgorithmECIESEncryptionStandardVariableIVX963SHA512AESGCM,

    ECIESEncryptionCofactorVariableIVX963SHA224AESGCM => kSecKeyAlgorithmECIESEncryptionCofactorVariableIVX963SHA224AESGCM,
    ECIESEncryptionCofactorVariableIVX963SHA256AESGCM => kSecKeyAlgorithmECIESEncryptionCofactorVariableIVX963SHA256AESGCM,
    ECIESEncryptionCofactorVariableIVX963SHA384AESGCM => kSecKeyAlgorithmECIESEncryptionCofactorVariableIVX963SHA384AESGCM,
    ECIESEncryptionCofactorVariableIVX963SHA512AESGCM => kSecKeyAlgorithmECIESEncryptionCofactorVariableIVX963SHA512AESGCM,

    #"OSX_10_13" ECIESEncryptionCofactorX963SHA1AESGCM => kSecKeyAlgorithmECIESEncryptionCofactorX963SHA1AESGCM,
    #"OSX_10_13" ECIESEncryptionCofactorX963SHA224AESGCM => kSecKeyAlgorithmECIESEncryptionCofactorX963SHA224AESGCM,
    #"OSX_10_13" ECIESEncryptionCofactorX963SHA256AESGCM => kSecKeyAlgorithmECIESEncryptionCofactorX963SHA256AESGCM,
    #"OSX_10_13" ECIESEncryptionCofactorX963SHA384AESGCM => kSecKeyAlgorithmECIESEncryptionCofactorX963SHA384AESGCM,
    #"OSX_10_13" ECIESEncryptionCofactorX963SHA512AESGCM => kSecKeyAlgorithmECIESEncryptionCofactorX963SHA512AESGCM,

    ECDSASignatureRFC4754 => kSecKeyAlgorithmECDSASignatureRFC4754,

    ECDSASignatureDigestX962 => kSecKeyAlgorithmECDSASignatureDigestX962,
    ECDSASignatureDigestX962SHA1 => kSecKeyAlgorithmECDSASignatureDigestX962SHA1,
    ECDSASignatureDigestX962SHA224 => kSecKeyAlgorithmECDSASignatureDigestX962SHA224,
    ECDSASignatureDigestX962SHA256 => kSecKeyAlgorithmECDSASignatureDigestX962SHA256,
    ECDSASignatureDigestX962SHA384 => kSecKeyAlgorithmECDSASignatureDigestX962SHA384,
    ECDSASignatureDigestX962SHA512 => kSecKeyAlgorithmECDSASignatureDigestX962SHA512,

    ECDSASignatureMessageX962SHA1 => kSecKeyAlgorithmECDSASignatureMessageX962SHA1,
    ECDSASignatureMessageX962SHA224 => kSecKeyAlgorithmECDSASignatureMessageX962SHA224,
    ECDSASignatureMessageX962SHA256 => kSecKeyAlgorithmECDSASignatureMessageX962SHA256,
    ECDSASignatureMessageX962SHA384 => kSecKeyAlgorithmECDSASignatureMessageX962SHA384,
    ECDSASignatureMessageX962SHA512 => kSecKeyAlgorithmECDSASignatureMessageX962SHA512,

    ECDHKeyExchangeCofactor => kSecKeyAlgorithmECDHKeyExchangeCofactor,
    ECDHKeyExchangeStandard => kSecKeyAlgorithmECDHKeyExchangeStandard,
    ECDHKeyExchangeCofactorX963SHA1 => kSecKeyAlgorithmECDHKeyExchangeCofactorX963SHA1,
    ECDHKeyExchangeStandardX963SHA1 => kSecKeyAlgorithmECDHKeyExchangeStandardX963SHA1,
    ECDHKeyExchangeCofactorX963SHA224 => kSecKeyAlgorithmECDHKeyExchangeCofactorX963SHA224,
    ECDHKeyExchangeCofactorX963SHA256 => kSecKeyAlgorithmECDHKeyExchangeCofactorX963SHA256,
    ECDHKeyExchangeCofactorX963SHA384 => kSecKeyAlgorithmECDHKeyExchangeCofactorX963SHA384,
    ECDHKeyExchangeCofactorX963SHA512 => kSecKeyAlgorithmECDHKeyExchangeCofactorX963SHA512,
    ECDHKeyExchangeStandardX963SHA224 => kSecKeyAlgorithmECDHKeyExchangeStandardX963SHA224,
    ECDHKeyExchangeStandardX963SHA256 => kSecKeyAlgorithmECDHKeyExchangeStandardX963SHA256,
    ECDHKeyExchangeStandardX963SHA384 => kSecKeyAlgorithmECDHKeyExchangeStandardX963SHA384,
    ECDHKeyExchangeStandardX963SHA512 => kSecKeyAlgorithmECDHKeyExchangeStandardX963SHA512,

    RSAEncryptionRaw => kSecKeyAlgorithmRSAEncryptionRaw,
    RSAEncryptionPKCS1 => kSecKeyAlgorithmRSAEncryptionPKCS1,

    RSAEncryptionOAEPSHA1 => kSecKeyAlgorithmRSAEncryptionOAEPSHA1,
    RSAEncryptionOAEPSHA224 => kSecKeyAlgorithmRSAEncryptionOAEPSHA224,
    RSAEncryptionOAEPSHA256 => kSecKeyAlgorithmRSAEncryptionOAEPSHA256,
    RSAEncryptionOAEPSHA384 => kSecKeyAlgorithmRSAEncryptionOAEPSHA384,
    RSAEncryptionOAEPSHA512 => kSecKeyAlgorithmRSAEncryptionOAEPSHA512,

    RSAEncryptionOAEPSHA1AESGCM => kSecKeyAlgorithmRSAEncryptionOAEPSHA1AESGCM,
    RSAEncryptionOAEPSHA224AESGCM => kSecKeyAlgorithmRSAEncryptionOAEPSHA224AESGCM,
    RSAEncryptionOAEPSHA256AESGCM => kSecKeyAlgorithmRSAEncryptionOAEPSHA256AESGCM,
    RSAEncryptionOAEPSHA384AESGCM => kSecKeyAlgorithmRSAEncryptionOAEPSHA384AESGCM,
    RSAEncryptionOAEPSHA512AESGCM => kSecKeyAlgorithmRSAEncryptionOAEPSHA512AESGCM,

    RSASignatureRaw => kSecKeyAlgorithmRSASignatureRaw,

    RSASignatureDigestPKCS1v15Raw => kSecKeyAlgorithmRSASignatureDigestPKCS1v15Raw,
    RSASignatureDigestPKCS1v15SHA1 => kSecKeyAlgorithmRSASignatureDigestPKCS1v15SHA1,
    RSASignatureDigestPKCS1v15SHA224 => kSecKeyAlgorithmRSASignatureDigestPKCS1v15SHA224,
    RSASignatureDigestPKCS1v15SHA256 => kSecKeyAlgorithmRSASignatureDigestPKCS1v15SHA256,
    RSASignatureDigestPKCS1v15SHA384 => kSecKeyAlgorithmRSASignatureDigestPKCS1v15SHA384,
    RSASignatureDigestPKCS1v15SHA512 => kSecKeyAlgorithmRSASignatureDigestPKCS1v15SHA512,

    RSASignatureMessagePKCS1v15SHA1 => kSecKeyAlgorithmRSASignatureMessagePKCS1v15SHA1,
    RSASignatureMessagePKCS1v15SHA224 => kSecKeyAlgorithmRSASignatureMessagePKCS1v15SHA224,
    RSASignatureMessagePKCS1v15SHA256 => kSecKeyAlgorithmRSASignatureMessagePKCS1v15SHA256,
    RSASignatureMessagePKCS1v15SHA384 => kSecKeyAlgorithmRSASignatureMessagePKCS1v15SHA384,
    RSASignatureMessagePKCS1v15SHA512 => kSecKeyAlgorithmRSASignatureMessagePKCS1v15SHA512,

    RSASignatureDigestPSSSHA1 => kSecKeyAlgorithmRSASignatureDigestPSSSHA1,
    RSASignatureDigestPSSSHA224 => kSecKeyAlgorithmRSASignatureDigestPSSSHA224,
    RSASignatureDigestPSSSHA256 => kSecKeyAlgorithmRSASignatureDigestPSSSHA256,
    RSASignatureDigestPSSSHA384 => kSecKeyAlgorithmRSASignatureDigestPSSSHA384,
    RSASignatureDigestPSSSHA512 => kSecKeyAlgorithmRSASignatureDigestPSSSHA512,

    RSASignatureMessagePSSSHA1 => kSecKeyAlgorithmRSASignatureMessagePSSSHA1,
    RSASignatureMessagePSSSHA224 => kSecKeyAlgorithmRSASignatureMessagePSSSHA224,
    RSASignatureMessagePSSSHA256 => kSecKeyAlgorithmRSASignatureMessagePSSSHA256,
    RSASignatureMessagePSSSHA384 => kSecKeyAlgorithmRSASignatureMessagePSSSHA384,
    RSASignatureMessagePSSSHA512 => kSecKeyAlgorithmRSASignatureMessagePSSSHA512
}
