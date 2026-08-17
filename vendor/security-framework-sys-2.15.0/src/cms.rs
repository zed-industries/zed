//! Cryptographic Message Syntax support

use std::os::raw::c_void;

use core_foundation_sys::array::CFArrayRef;
use core_foundation_sys::base::{Boolean, CFTypeID, CFTypeRef, OSStatus};
use core_foundation_sys::data::CFDataRef;
use core_foundation_sys::date::CFAbsoluteTime;
use core_foundation_sys::string::CFStringRef;

use crate::base::SecCertificateRef;
use crate::trust::SecTrustRef;

pub enum OpaqueCMSEncoderRef {}
pub type CMSEncoderRef = *mut OpaqueCMSEncoderRef;

pub enum OpaqueCMSDecoderRef {}
pub type CMSDecoderRef = *mut OpaqueCMSEncoderRef;

#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum CMSSignerStatus {
    kCMSSignerUnsigned = 0,
    kCMSSignerValid = 1,
    kCMSSignerNeedsDetachedContent = 2,
    kCMSSignerInvalidSignature = 3,
    kCMSSignerInvalidCert = 4,
    kCMSSignerInvalidIndex = 5,
}

pub type CMSSignedAttributes = u32;
pub const kCMSAttrNone: CMSSignedAttributes = 0x0000;
pub const kCMSAttrSmimeCapabilities: CMSSignedAttributes = 0x0001;
pub const kCMSAttrSmimeEncryptionKeyPrefs: CMSSignedAttributes = 0x0002;
pub const kCMSAttrSmimeMSEncryptionKeyPrefs: CMSSignedAttributes = 0x0004;
pub const kCMSAttrSigningTime: CMSSignedAttributes = 0x0008;
pub const kCMSAttrAppleCodesigningHashAgility: CMSSignedAttributes = 0x0010;
pub const kCMSAttrAppleCodesigningHashAgilityV2: CMSSignedAttributes = 0x0020;
pub const kCMSAttrAppleExpirationTime: CMSSignedAttributes = 0x0040;

#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum CMSCertificateChainMode {
    kCMSCertificateNone = 0,
    kCMSCertificateSignerOnly = 1,
    kCMSCertificateChain = 2,
    kCMSCertificateChainWithRoot = 3,
    kCMSCertificateChainWithRootOrFail = 4,
}

extern "C" {

    // CMS decoder

    pub fn CMSDecoderGetTypeID() -> CFTypeID;

    pub fn CMSDecoderCreate(output: *mut CMSDecoderRef) -> OSStatus;

    pub fn CMSDecoderUpdateMessage(
        decoder: CMSDecoderRef,
        msg_bytes: *const c_void,
        msg_bytes_len: usize,
    ) -> OSStatus;

    pub fn CMSDecoderFinalizeMessage(decoder: CMSDecoderRef) -> OSStatus;

    pub fn CMSDecoderSetDetachedContent(
        decoder: CMSDecoderRef,
        detached_content: CFDataRef,
    ) -> OSStatus;

    pub fn CMSDecoderCopyDetachedContent(
        decoder: CMSDecoderRef,
        detached_content_out: *mut CFDataRef,
    ) -> OSStatus;

    pub fn CMSDecoderGetNumSigners(
        decoder: CMSDecoderRef,
        num_signers_out: *mut usize,
    ) -> OSStatus;

    pub fn CMSDecoderCopySignerStatus(
        decoder: CMSDecoderRef,
        signer_index: usize,
        policy_or_array: CFTypeRef,
        evaluate_sec_trust: Boolean,
        signer_status_out: *mut CMSSignerStatus,
        sec_trust_out: *mut SecTrustRef,
        cert_verify_result_code_out: *mut OSStatus,
    ) -> OSStatus;

    pub fn CMSDecoderCopySignerEmailAddress(
        decoder: CMSDecoderRef,
        signer_index: usize,
        signer_email_address_out: *mut CFStringRef,
    ) -> OSStatus;

    pub fn CMSDecoderCopySignerCert(
        decoder: CMSDecoderRef,
        signer_index: usize,
        signer_cert_out: *mut SecCertificateRef,
    ) -> OSStatus;

    pub fn CMSDecoderIsContentEncrypted(
        decoder: CMSDecoderRef,
        is_encrypted_out: *mut Boolean,
    ) -> OSStatus;

    pub fn CMSDecoderCopyEncapsulatedContentType(
        decoder: CMSDecoderRef,
        content_type_out: *mut CFDataRef,
    ) -> OSStatus;

    pub fn CMSDecoderCopyAllCerts(decoder: CMSDecoderRef, certs_out: *mut CFArrayRef) -> OSStatus;

    pub fn CMSDecoderCopyContent(decoder: CMSDecoderRef, content_out: *mut CFDataRef) -> OSStatus;

    pub fn CMSDecoderCopySignerSigningTime(
        decoder: CMSDecoderRef,
        signer_index: usize,
        sign_time_out: *mut CFAbsoluteTime,
    ) -> OSStatus;

    pub fn CMSDecoderCopySignerTimestamp(
        decoder: CMSDecoderRef,
        signer_index: usize,
        timestamp: *mut CFAbsoluteTime,
    ) -> OSStatus;

    pub fn CMSDecoderCopySignerTimestampWithPolicy(
        decoder: CMSDecoderRef,
        timestamp_policy: CFTypeRef,
        signer_index: usize,
        timestamp: *mut CFAbsoluteTime,
    ) -> OSStatus;

    pub fn CMSDecoderCopySignerTimestampCertificates(
        decoder: CMSDecoderRef,
        signer_index: usize,
        certificate_refs: *mut CFArrayRef,
    ) -> OSStatus;

    // CMS encoder

    pub static kCMSEncoderDigestAlgorithmSHA1: CFStringRef;
    pub static kCMSEncoderDigestAlgorithmSHA256: CFStringRef;

    pub fn CMSEncoderGetTypeID() -> CFTypeID;

    pub fn CMSEncoderCreate(encoder_out: *mut CMSEncoderRef) -> OSStatus;

    pub fn CMSEncoderSetSignerAlgorithm(
        encoder: CMSEncoderRef,
        digest_alogrithm: CFStringRef,
    ) -> OSStatus;

    pub fn CMSEncoderAddSigners(encoder: CMSEncoderRef, signer_or_array: CFTypeRef) -> OSStatus;

    pub fn CMSEncoderCopySigners(encoder: CMSEncoderRef, signers_out: *mut CFArrayRef) -> OSStatus;

    pub fn CMSEncoderAddRecipients(
        encoder: CMSEncoderRef,
        recipient_or_array: CFTypeRef,
    ) -> OSStatus;

    pub fn CMSEncoderCopyRecipients(
        encoder: CMSEncoderRef,
        recipients_out: *mut CFArrayRef,
    ) -> OSStatus;

    pub fn CMSEncoderSetHasDetachedContent(
        encoder: CMSEncoderRef,
        detached_content: Boolean,
    ) -> OSStatus;

    pub fn CMSEncoderGetHasDetachedContent(
        encoder: CMSEncoderRef,
        detached_content_out: *mut Boolean,
    ) -> OSStatus;

    pub fn CMSEncoderSetEncapsulatedContentTypeOID(
        encoder: CMSEncoderRef,
        content_type_oid: CFTypeRef,
    ) -> OSStatus;

    pub fn CMSEncoderCopyEncapsulatedContentType(
        encoder: CMSEncoderRef,
        content_type_out: *mut CFDataRef,
    ) -> OSStatus;

    pub fn CMSEncoderAddSupportingCerts(
        encoder: CMSEncoderRef,
        cert_or_array: CFTypeRef,
    ) -> OSStatus;

    pub fn CMSEncoderCopySupportingCerts(
        encoder: CMSEncoderRef,
        certs_out: *mut CFArrayRef,
    ) -> OSStatus;

    pub fn CMSEncoderAddSignedAttributes(
        encoder: CMSEncoderRef,
        signed_attributes: CMSSignedAttributes,
    ) -> OSStatus;

    pub fn CMSEncoderSetCertificateChainMode(
        encoder: CMSEncoderRef,
        chain_mode: CMSCertificateChainMode,
    ) -> OSStatus;

    pub fn CMSEncoderGetCertificateChainMode(
        encoder: CMSEncoderRef,
        chain_mode_out: *mut CMSCertificateChainMode,
    ) -> OSStatus;

    pub fn CMSEncoderUpdateContent(
        encoder: CMSEncoderRef,
        content: *const c_void,
        content_len: usize,
    ) -> OSStatus;

    pub fn CMSEncoderCopyEncodedContent(
        encoder: CMSEncoderRef,
        encoded_content_out: *mut CFDataRef,
    ) -> OSStatus;

    pub fn CMSEncodeContent(
        signers: CFTypeRef,
        recipients: CFTypeRef,
        content_type_oid: CFTypeRef,
        detached_content: Boolean,
        signed_attributes: CMSSignedAttributes,
        content: *const c_void,
        content_len: usize,
        encoded_content_out: *mut CFDataRef,
    ) -> OSStatus;

    pub fn CMSEncoderCopySignerTimestamp(
        encoder: CMSEncoderRef,
        signer_index: usize,
        timestamp: *mut CFAbsoluteTime,
    ) -> OSStatus;

    pub fn CMSEncoderCopySignerTimestampWithPolicy(
        encoder: CMSEncoderRef,
        timestamp_policy: CFTypeRef,
        signer_index: usize,
        timestamp: *mut CFAbsoluteTime,
    ) -> OSStatus;
}
