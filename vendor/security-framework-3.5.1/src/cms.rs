//! Cryptographic Message Syntax support

use std::{fmt, ptr};

use core_foundation::array::CFArray;
use core_foundation::base::TCFType;
use core_foundation::data::CFData;
use core_foundation::string::CFString;
use core_foundation_sys::array::CFArrayRef;
use core_foundation_sys::base::{OSStatus, TCFTypeRef};
use core_foundation_sys::data::CFDataRef;
use core_foundation_sys::date::CFAbsoluteTime;
use core_foundation_sys::string::CFStringRef;
use security_framework_sys::cms::*;
use security_framework_sys::trust::SecTrustRef;

use crate::base::Result;
use crate::certificate::SecCertificate;
use crate::cvt;
use crate::policy::SecPolicy;
use crate::trust::SecTrust;

pub use decoder::CMSDecoder;

pub use encoder::cms_encode_content;
pub use encoder::CMSEncoder;
pub use encoder::SignedAttributes;
pub use encoder::CMS_DIGEST_ALGORITHM_SHA1;
pub use encoder::CMS_DIGEST_ALGORITHM_SHA256;

mod encoder {
    use super::*;
    use crate::identity::SecIdentity;
    use core_foundation::{declare_TCFType, impl_TCFType};

    /// SHA1 digest algorithm
    pub const CMS_DIGEST_ALGORITHM_SHA1: &str = "sha1";
    /// SHA256 digest algorithm
    pub const CMS_DIGEST_ALGORITHM_SHA256: &str = "sha256";

    bitflags::bitflags! {
        /// Optional attributes you can add to a signed message
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct SignedAttributes: CMSSignedAttributes {
            /// Identify signature, encryption, and digest algorithms supported by the encoder
            const SMIME_CAPABILITIES = kCMSAttrSmimeCapabilities;
            /// Indicate that the signing certificate included with the message is the preferred one for S/MIME encryption
            const SMIME_ENCRYPTION_KEY_PREFS = kCMSAttrSmimeEncryptionKeyPrefs;
            /// Indicate that the signing certificate included with the message is the preferred one for S/MIME encryption,
            /// but using an attribute object identifier (OID) preferred by Microsoft
            const SMIME_MS_ENCRYPTION_KEY_PREFS = kCMSAttrSmimeMSEncryptionKeyPrefs;
            /// Include the signing time
            const SIGNING_TIME =  kCMSAttrSigningTime;
            /// Include Apple codesigning hash agility
            const APPLE_CODESIGNING_HASH_AGILITY = kCMSAttrAppleCodesigningHashAgility;
            /// Include Apple codesigning hash agility, version 2
            const APPLE_CODESIGNING_HASH_AGILITY_V2 = kCMSAttrAppleCodesigningHashAgilityV2;
            /// Include the expiration time
            const APPLE_EXPIRATION_TIME = kCMSAttrAppleExpirationTime;
        }
    }

    declare_TCFType! {
        /// A type representing CMS encoder
        CMSEncoder, CMSEncoderRef
    }
    impl_TCFType!(CMSEncoder, CMSEncoderRef, CMSEncoderGetTypeID);

    unsafe impl Sync for CMSEncoder {}
    unsafe impl Send for CMSEncoder {}

    impl fmt::Debug for CMSEncoder {
        #[cold]
        fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt.debug_struct("CMSEncoder").finish()
        }
    }

    impl CMSEncoder {
        /// Create a new instance of `CMSEncoder`
        pub fn create() -> Result<Self> {
            let mut inner: CMSEncoderRef = ptr::null_mut();
            cvt(unsafe { CMSEncoderCreate(&mut inner) })?;
            Ok(Self(inner))
        }

        /// Sets the digest algorithm to use for the signer.
        /// Can be one of the predefined constants:
        ///
        /// * `CMS_DIGEST_ALGORITHM_SHA1`
        /// * `CMS_DIGEST_ALGORITHM_SHA256`
        pub fn set_signer_algorithm(&self, digest_algorithm: &str) -> Result<()> {
            let alg = CFString::new(digest_algorithm);

            cvt(unsafe { CMSEncoderSetSignerAlgorithm(self.0, alg.as_concrete_TypeRef()) })?;
            Ok(())
        }

        /// Specify signers of the CMS message; implies that the message will be signed
        pub fn add_signers(&self, signers: &[SecIdentity]) -> Result<()> {
            let signers = CFArray::from_CFTypes(signers);
            cvt(unsafe {
                CMSEncoderAddSigners(
                    self.0,
                    if signers.is_empty() { ptr::null() } else { signers.as_CFTypeRef() })
            })?;
            Ok(())
        }

        /// Obtains the array of signers specified with the `add_signers` function
        pub fn get_signers(&self) -> Result<Vec<SecIdentity>> {
            let mut out: CFArrayRef = ptr::null_mut();
            cvt(unsafe { CMSEncoderCopySigners(self.0, &mut out) })?;

            if out.is_null() {
                Ok(Vec::new())
            } else {
                let array = unsafe { CFArray::<SecIdentity>::wrap_under_create_rule(out) };
                Ok(array.into_iter().map(|c| c.clone()).collect())
            }
        }

        /// Specifies a message is to be encrypted and specifies the recipients of the message
        pub fn add_recipients(&self, recipients: &[SecCertificate]) -> Result<()> {
            let recipients = CFArray::from_CFTypes(recipients);
            cvt(unsafe {
                CMSEncoderAddRecipients(
                    self.0,
                    if recipients.is_empty() { ptr::null() } else { recipients.as_CFTypeRef() },
                )
            })?;
            Ok(())
        }

        /// Obtains the array of recipients specified with the `add_recipients` function
        pub fn get_recipients(&self) -> Result<Vec<SecCertificate>> {
            let mut out: CFArrayRef = ptr::null_mut();
            cvt(unsafe { CMSEncoderCopyRecipients(self.0, &mut out) })?;

            if out.is_null() {
                Ok(Vec::new())
            } else {
                let array = unsafe { CFArray::<SecCertificate>::wrap_under_create_rule(out) };
                Ok(array.into_iter().map(|c| c.clone()).collect())
            }
        }

        /// Specifies whether the signed data is to be separate from the message
        pub fn set_has_detached_content(&self, has_detached_content: bool) -> Result<()> {
            cvt(unsafe { CMSEncoderSetHasDetachedContent(self.0, has_detached_content.into()) })?;
            Ok(())
        }

        /// Indicates whether the message is to have detached content
        pub fn get_has_detached_content(&self) -> Result<bool> {
            let mut has_detached_content = 0;
            cvt(unsafe { CMSEncoderGetHasDetachedContent(self.0, &mut has_detached_content) })?;
            Ok(has_detached_content != 0)
        }

        /// Specifies an object identifier for the encapsulated data of a signed message
        pub fn set_encapsulated_content_type_oid(&self, oid: &str) -> Result<()> {
            let oid = CFString::new(oid);
            cvt(unsafe { CMSEncoderSetEncapsulatedContentTypeOID(self.0, oid.as_CFTypeRef()) })?;
            Ok(())
        }

        /// Obtains the object identifier for the encapsulated data of a signed message
        pub fn get_encapsulated_content_type(&self) -> Result<Vec<u8>> {
            let mut out: CFDataRef = ptr::null_mut();
            cvt(unsafe { CMSEncoderCopyEncapsulatedContentType(self.0, &mut out) })?;
            Ok(unsafe { CFData::wrap_under_create_rule(out).to_vec() })
        }

        /// Adds certificates to a message
        pub fn add_supporting_certs(&self, certs: &[SecCertificate]) -> Result<()> {
            let certs = CFArray::from_CFTypes(certs);
            cvt(unsafe {
                CMSEncoderAddSupportingCerts(
                    self.0,
                    if !certs.is_empty() { certs.as_CFTypeRef() } else { ptr::null() })
            })?;
            Ok(())
        }

        /// Obtains the certificates added to a message with `add_supporting_certs`
        pub fn get_supporting_certs(&self) -> Result<Vec<SecCertificate>> {
            let mut out: CFArrayRef = ptr::null_mut();
            cvt(unsafe { CMSEncoderCopySupportingCerts(self.0, &mut out) })?;

            if out.is_null() {
                Ok(Vec::new())
            } else {
                let array = unsafe { CFArray::<SecCertificate>::wrap_under_create_rule(out) };
                Ok(array.into_iter().map(|c| c.clone()).collect())
            }
        }

        /// Specifies attributes for a signed message
        pub fn add_signed_attributes(&self, signed_attributes: SignedAttributes) -> Result<()> {
            cvt(unsafe { CMSEncoderAddSignedAttributes(self.0, signed_attributes.bits()) })?;
            Ok(())
        }

        /// Specifies which certificates to include in a signed CMS message
        pub fn set_certificate_chain_mode(&self, certificate_chain_mode: CMSCertificateChainMode) -> Result<()> {
            cvt(unsafe { CMSEncoderSetCertificateChainMode(self.0, certificate_chain_mode) })?;
            Ok(())
        }

        /// Obtains a constant that indicates which certificates are to be included in a signed CMS message
        pub fn get_certificate_chain_mode(&self) -> Result<CMSCertificateChainMode> {
            let mut out = CMSCertificateChainMode::kCMSCertificateNone;
            cvt(unsafe { CMSEncoderGetCertificateChainMode(self.0, &mut out) })?;
            Ok(out)
        }

        /// Feeds content bytes into the encoder
        pub fn update_content(&self, content: &[u8]) -> Result<()> {
            cvt(unsafe { CMSEncoderUpdateContent(self.0, content.as_ptr().cast(), content.len()) })?;
            Ok(())
        }

        /// Finishes encoding the message and obtains the encoded result
        pub fn get_encoded_content(&self) -> Result<Vec<u8>> {
            let mut out: CFDataRef = ptr::null_mut();
            cvt(unsafe { CMSEncoderCopyEncodedContent(self.0, &mut out) })?;
            Ok(unsafe { CFData::wrap_under_create_rule(out).to_vec() })
        }

        /// Returns the timestamp of a signer of a CMS message, if present
        pub fn get_signer_timestamp(&self, signer_index: usize) -> Result<CFAbsoluteTime> {
            let mut out = CFAbsoluteTime::default();
            cvt(unsafe { CMSEncoderCopySignerTimestamp(self.0, signer_index, &mut out) })?;
            Ok(out)
        }

        /// Returns the timestamp of a signer of a CMS message using a particular policy, if present
        pub fn get_signer_timestamp_with_policy(
            &self,
            timestamp_policy: Option<CFStringRef>,
            signer_index: usize,
        ) -> Result<CFAbsoluteTime> {
            let mut out = CFAbsoluteTime::default();
            cvt(unsafe {
                CMSEncoderCopySignerTimestampWithPolicy(
                    self.0,
                    timestamp_policy.map(|p| p.as_void_ptr()).unwrap_or(ptr::null()),
                    signer_index,
                    &mut out,
                )
            })?;

            Ok(out)
        }
    }

    /// Encodes a message and obtains the result in one high-level function call
    pub fn cms_encode_content(
        signers: &[SecIdentity],
        recipients: &[SecCertificate],
        content_type_oid: Option<&str>,
        detached_content: bool,
        signed_attributes: SignedAttributes,
        content: &[u8],
    ) -> Result<Vec<u8>> {
        let mut out: CFDataRef = ptr::null_mut();
        let signers = CFArray::from_CFTypes(signers);
        let recipients = CFArray::from_CFTypes(recipients);
        let content_type_oid = content_type_oid.map(CFString::new);

        cvt(unsafe {
            CMSEncodeContent(
                if signers.is_empty() { ptr::null() } else { signers.as_CFTypeRef() },
                if recipients.is_empty() { ptr::null() } else { recipients.as_CFTypeRef() },
                content_type_oid.as_ref().map(|oid| oid.as_CFTypeRef()).unwrap_or(ptr::null()),
                detached_content.into(),
                signed_attributes.bits(),
                content.as_ptr().cast(),
                content.len(),
                &mut out,
            )
        })?;

        Ok(unsafe { CFData::wrap_under_create_rule(out).to_vec() })
    }
}

mod decoder {
    use super::*;
    use core_foundation::{declare_TCFType, impl_TCFType};

    /// Holds a result of the `CMSDecoder::get_signer_status` function
    pub struct SignerStatus {
        /// Signature status
        pub signer_status: CMSSignerStatus,
        /// Trust instance that was used to verify the signer’s certificate
        pub sec_trust: SecTrust,
        /// Result of the certificate verification
        pub cert_verify_result: Result<()>,
    }

    declare_TCFType! {
        /// A type representing CMS Decoder
        CMSDecoder, CMSDecoderRef
    }
    impl_TCFType!(CMSDecoder, CMSDecoderRef, CMSDecoderGetTypeID);

    unsafe impl Sync for CMSDecoder {}
    unsafe impl Send for CMSDecoder {}

    impl fmt::Debug for CMSDecoder {
        #[cold]
        fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt.debug_struct("CMSDecoder").finish()
        }
    }

    impl CMSDecoder {
        /// Create a new instance of `CMSDecoder`
        pub fn create() -> Result<Self> {
            let mut inner: CMSDecoderRef = ptr::null_mut();
            cvt(unsafe { CMSDecoderCreate(&mut inner) })?;
            Ok(Self(inner))
        }

        /// Feeds raw bytes of the message to be decoded into the decoder
        pub fn update_message(&self, message: &[u8]) -> Result<()> {
            cvt(unsafe { CMSDecoderUpdateMessage(self.0, message.as_ptr().cast(), message.len()) })?;
            Ok(())
        }

        /// Indicates that there is no more data to decode
        pub fn finalize_message(&self) -> Result<()> {
            cvt(unsafe { CMSDecoderFinalizeMessage(self.0) })?;
            Ok(())
        }

        /// Specifies the message’s detached content, if any
        pub fn set_detached_content(&self, detached_content: &[u8]) -> Result<()> {
            let data = CFData::from_buffer(detached_content);
            cvt(unsafe { CMSDecoderSetDetachedContent(self.0, data.as_concrete_TypeRef()) })?;
            Ok(())
        }

        /// Obtains the detached content specified with the `set_detached_content` function
        pub fn get_detached_content(&self) -> Result<Vec<u8>> {
            unsafe {
                let mut out: CFDataRef = ptr::null_mut();
                cvt(CMSDecoderCopyDetachedContent(self.0, &mut out))?;
                if out.is_null() {
                    Ok(Vec::new())
                } else {
                    Ok(CFData::wrap_under_create_rule(out).to_vec())
                }
            }
        }

        /// Obtains the number of signers of a message
        pub fn get_num_signers(&self) -> Result<usize> {
            let mut out = 0;
            cvt(unsafe { CMSDecoderGetNumSigners(self.0, &mut out) })?;
            Ok(out)
        }

        /// Obtains the status of a CMS message’s signature
        pub fn get_signer_status(
            &self,
            signer_index: usize,
            policies: &[SecPolicy],
        ) -> Result<SignerStatus> {
            let policies = CFArray::from_CFTypes(policies);

            let mut signer_status = CMSSignerStatus::kCMSSignerUnsigned;
            let mut sec_trust: SecTrustRef = ptr::null_mut();
            let mut verify_result = OSStatus::default();

            cvt(unsafe {
                CMSDecoderCopySignerStatus(
                    self.0,
                    signer_index,
                    if policies.is_empty() { ptr::null() } else { policies.as_CFTypeRef() },
                    true.into(),
                    &mut signer_status,
                    &mut sec_trust,
                    &mut verify_result,
                )
            })?;

            Ok(SignerStatus {
                signer_status,
                sec_trust: unsafe { SecTrust::wrap_under_create_rule(sec_trust) },
                cert_verify_result: cvt(verify_result),
            })
        }

        /// Obtains the email address of the specified signer of a CMS message
        pub fn get_signer_email_address(&self, signer_index: usize) -> Result<String> {
            let mut out: CFStringRef = ptr::null_mut();
            cvt(unsafe { CMSDecoderCopySignerEmailAddress(self.0, signer_index, &mut out) })?;
            Ok(unsafe { CFString::wrap_under_create_rule(out).to_string() })
        }

        /// Determines whether a CMS message was encrypted
        pub fn is_content_encrypted(&self) -> Result<bool> {
            let mut out = 0;
            cvt(unsafe { CMSDecoderIsContentEncrypted(self.0, &mut out) })?;
            Ok(out != 0)
        }

        /// Obtains the object identifier for the encapsulated data of a signed message
        pub fn get_encapsulated_content_type(&self) -> Result<Vec<u8>> {
            let mut out: CFDataRef = ptr::null_mut();
            if out.is_null() {
                Ok(Vec::new())
            } else {
                cvt(unsafe { CMSDecoderCopyEncapsulatedContentType(self.0, &mut out) })?;
                Ok(unsafe { CFData::wrap_under_create_rule(out).to_vec() })
            }
        }

        /// Obtains an array of all of the certificates in a message
        pub fn get_all_certs(&self) -> Result<Vec<SecCertificate>> {
            let mut out: CFArrayRef = ptr::null_mut();
            cvt(unsafe { CMSDecoderCopyAllCerts(self.0, &mut out) })?;

            if out.is_null() {
                Ok(Vec::new())
            } else {
                let array = unsafe { CFArray::<SecCertificate>::wrap_under_create_rule(out) };
                Ok(array.into_iter().map(|c| c.clone()).collect())
            }
        }

        /// Obtains the message content, if any
        pub fn get_content(&self) -> Result<Vec<u8>> {
            let mut out: CFDataRef = ptr::null_mut();

            cvt(unsafe { CMSDecoderCopyContent(self.0, &mut out) })?;

            if out.is_null() {
                Ok(Vec::new())
            } else {
                Ok(unsafe { CFData::wrap_under_create_rule(out).to_vec() })
            }
        }

        /// Obtains the signing time of a CMS message, if present
        pub fn get_signer_signing_time(&self, signer_index: usize) -> Result<CFAbsoluteTime> {
            let mut out = CFAbsoluteTime::default();
            cvt(unsafe { CMSDecoderCopySignerSigningTime(self.0, signer_index, &mut out) })?;
            Ok(out)
        }

        /// Returns the timestamp of a signer of a CMS message, if present
        pub fn get_signer_timestamp(&self, signer_index: usize) -> Result<CFAbsoluteTime> {
            let mut out = CFAbsoluteTime::default();
            cvt(unsafe { CMSDecoderCopySignerTimestamp(self.0, signer_index, &mut out) })?;
            Ok(out)
        }

        /// Returns the timestamp of a signer of a CMS message using a given policy, if present
        pub fn get_signer_timestamp_with_policy(
            &self,
            timestamp_policy: Option<CFStringRef>,
            signer_index: usize,
        ) -> Result<CFAbsoluteTime> {
            let mut out = CFAbsoluteTime::default();
            cvt(unsafe {
                CMSDecoderCopySignerTimestampWithPolicy(
                    self.0,
                    timestamp_policy.map(|p| p.as_void_ptr()).unwrap_or(ptr::null()),
                    signer_index,
                    &mut out,
                )
            })?;

            Ok(out)
        }

        /// Returns an array containing the certificates from a timestamp response
        pub fn get_signer_timestamp_certificates(&self, signer_index: usize) -> Result<Vec<SecCertificate>> {
            let mut out: CFArrayRef = ptr::null_mut();
            cvt(unsafe { CMSDecoderCopySignerTimestampCertificates(self.0, signer_index, &mut out) })?;

            if out.is_null() {
                Ok(Vec::new())
            } else {
                let array = unsafe { CFArray::<SecCertificate>::wrap_under_create_rule(out) };
                Ok(array.into_iter().map(|c| c.clone()).collect())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cms::{cms_encode_content, CMSDecoder, SignedAttributes};
    use crate::import_export::{ImportedIdentity, Pkcs12ImportOptions};
    use crate::policy::SecPolicy;
    use security_framework_sys::cms::CMSSignerStatus;
    use std::sync::{Mutex, MutexGuard};

    const KEYSTORE: &[u8] = include_bytes!("../test/cms/keystore.p12");
    const ENCRYPTED_CMS: &[u8] = include_bytes!("../test/cms/encrypted.p7m");
    const SIGNED_ENCRYPTED_CMS: &[u8] = include_bytes!("../test/cms/signed-encrypted.p7m");

    static SHARED_KEYCHAIN: Mutex<()> = Mutex::new(());

    fn import_keystore() -> (MutexGuard<'static, ()>, Vec<ImportedIdentity>) {
        let lock = SHARED_KEYCHAIN.lock().unwrap();
        let mut import_opts = Pkcs12ImportOptions::new();
        let id = import_opts.passphrase("cms").import(KEYSTORE).expect("import keystore.p12");
        (lock, id)
    }

    #[test]
    fn test_decode_encrypted() {
        let _lock = import_keystore();

        let decoder = CMSDecoder::create().expect("create");
        decoder.update_message(ENCRYPTED_CMS).expect("update");
        decoder.finalize_message().expect("finalize");

        assert!(decoder.is_content_encrypted().unwrap());
        assert_eq!(decoder.get_content().unwrap(), b"encrypted message\n");
        assert_eq!(decoder.get_all_certs().unwrap().len(), 0);
        assert_eq!(decoder.get_num_signers().unwrap(), 0);
    }

    #[test]
    fn test_decode_signed_and_encrypted() {
        let _lock = import_keystore();

        let decoder = CMSDecoder::create().unwrap();
        decoder.update_message(SIGNED_ENCRYPTED_CMS).unwrap();
        decoder.finalize_message().unwrap();

        assert!(decoder.is_content_encrypted().unwrap());

        let signed_content = decoder.get_content().unwrap();

        let decoder2 = CMSDecoder::create().unwrap();
        decoder2.update_message(&signed_content).unwrap();
        decoder2.finalize_message().unwrap();
        assert_eq!(decoder2.get_content().unwrap(), b"encrypted message\n");
        assert_eq!(decoder2.get_num_signers().unwrap(), 1);

        let policies = vec![SecPolicy::create_x509()];
        let status = decoder2.get_signer_status(0, &policies).unwrap();
        assert!(status.cert_verify_result.is_err());
        assert_eq!(status.signer_status, CMSSignerStatus::kCMSSignerInvalidCert);
    }

    #[test]
    fn test_encode_encrypted() {
        let (_lock, identities) = import_keystore();

        let chain = identities
            .iter().find_map(|id| id.cert_chain.as_ref())
            .unwrap();

        let message = cms_encode_content(
            &[],
            &chain[0..1],
            None,
            false,
            SignedAttributes::empty(),
            b"encrypted message\n",
        ).unwrap();

        let decoder = CMSDecoder::create().unwrap();
        decoder.update_message(&message).unwrap();
        decoder.finalize_message().unwrap();
        assert_eq!(decoder.get_content().unwrap(), b"encrypted message\n");
    }

    #[test]
    fn test_encode_signed_encrypted() {
        let (_lock, identities) = import_keystore();

        let chain = identities
            .iter().find_map(|id| id.cert_chain.as_ref())
            .unwrap();

        let identity = identities
            .iter().find_map(|id| id.identity.as_ref())
            .unwrap();

        let message = cms_encode_content(
            std::slice::from_ref(identity),
            &chain[0..1],
            None,
            false,
            SignedAttributes::empty(),
            b"encrypted message\n",
        ).unwrap();

        let decoder = CMSDecoder::create().unwrap();
        decoder.update_message(&message).unwrap();
        decoder.finalize_message().unwrap();
        assert_eq!(decoder.get_content().unwrap(), b"encrypted message\n");
        assert_eq!(decoder.get_num_signers().unwrap(), 1);
    }

    #[test]
    fn test_encode_with_cms_encoder() {
        let (_lock, identities) = import_keystore();

        let chain = identities
            .iter().find_map(|id| id.cert_chain.as_ref())
            .unwrap();

        let identity = identities
            .iter().find_map(|id| id.identity.as_ref())
            .unwrap();

        let message = cms_encode_content(
            std::slice::from_ref(identity),
            &chain[0..1],
            None,
            false,
            SignedAttributes::empty(),
            b"encrypted message\n",
        ).unwrap();

        let decoder = CMSDecoder::create().unwrap();
        decoder.update_message(&message).unwrap();
        decoder.finalize_message().unwrap();
        assert_eq!(decoder.get_content().unwrap(), b"encrypted message\n");
        assert_eq!(decoder.get_num_signers().unwrap(), 1);
    }
}
