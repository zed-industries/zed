// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

use crate::digest::{digest, SHA256};
use crate::rsa::KeySize;
use crate::signature::{
    EcdsaKeyPair, Ed25519KeyPair, KeyPair, ParsedPublicKey, RsaKeyPair, UnparsedPublicKey,
    ECDSA_P256_SHA256_ASN1, ECDSA_P256_SHA256_ASN1_SIGNING, ECDSA_P256_SHA256_FIXED,
    ECDSA_P256_SHA256_FIXED_SIGNING, ECDSA_P384_SHA384_ASN1, ECDSA_P384_SHA384_ASN1_SIGNING,
    ED25519, RSA_PKCS1_2048_8192_SHA256, RSA_PSS_2048_8192_SHA256,
};
use crate::{rand, test};
use std::any::Any;

#[cfg(all(feature = "unstable", not(feature = "fips")))]
use crate::unstable::signature::{
    PqdsaKeyPair, ML_DSA_44, ML_DSA_44_SIGNING, ML_DSA_65, ML_DSA_65_SIGNING, ML_DSA_87,
    ML_DSA_87_SIGNING,
};

#[test]
fn test_parsed_public_key_ed25519() {
    let key_pair = Ed25519KeyPair::generate().unwrap();
    let public_key_bytes = key_pair.public_key().as_ref();

    let parsed = ParsedPublicKey::new(&ED25519, public_key_bytes).unwrap();

    assert_eq!(parsed.algorithm().type_id(), ED25519.type_id());

    let message = b"test message";
    let signature = key_pair.sign(message);

    assert!(parsed.verify_sig(message, signature.as_ref()).is_ok());
    assert!(parsed
        .verify_sig(b"wrong message", signature.as_ref())
        .is_err());
}

#[test]
fn test_parsed_public_key_ecdsa_p256() {
    let key_pair = EcdsaKeyPair::generate(&ECDSA_P256_SHA256_ASN1_SIGNING).unwrap();
    let public_key_bytes = key_pair.public_key().as_ref();

    let parsed = ParsedPublicKey::new(&ECDSA_P256_SHA256_ASN1, public_key_bytes).unwrap();

    assert_eq!(
        parsed.algorithm().type_id(),
        ECDSA_P256_SHA256_ASN1.type_id()
    );

    let rng = rand::SystemRandom::new();
    let message = b"test message";
    let signature = key_pair.sign(&rng, message).unwrap();

    assert!(parsed.verify_sig(message, signature.as_ref()).is_ok());
    assert!(parsed
        .verify_sig(b"wrong message", signature.as_ref())
        .is_err());
}

#[test]
fn test_parsed_public_key_ecdsa_p256_fixed() {
    let key_pair = EcdsaKeyPair::generate(&ECDSA_P256_SHA256_FIXED_SIGNING).unwrap();
    let public_key_bytes = key_pair.public_key().as_ref();

    let parsed = ParsedPublicKey::new(&ECDSA_P256_SHA256_FIXED, public_key_bytes).unwrap();

    let rng = rand::SystemRandom::new();
    let message = b"test message";
    let signature = key_pair.sign(&rng, message).unwrap();

    assert!(parsed.verify_sig(message, signature.as_ref()).is_ok());
}

#[test]
fn test_parsed_public_key_ecdsa_p384() {
    let key_pair = EcdsaKeyPair::generate(&ECDSA_P384_SHA384_ASN1_SIGNING).unwrap();
    let public_key_bytes = key_pair.public_key().as_ref();

    let parsed = ParsedPublicKey::new(&ECDSA_P384_SHA384_ASN1, public_key_bytes).unwrap();

    let rng = rand::SystemRandom::new();
    let message = b"test message";
    let signature = key_pair.sign(&rng, message).unwrap();

    assert!(parsed.verify_sig(message, signature.as_ref()).is_ok());
    assert!(parsed
        .verify_sig(b"wrong message", signature.as_ref())
        .is_err());
}

#[test]
fn test_parsed_public_key_rsa_pkcs1() {
    let key_pair = RsaKeyPair::generate(KeySize::Rsa2048).unwrap();
    let public_key_bytes = key_pair.public_key().as_ref();

    let parsed = ParsedPublicKey::new(&RSA_PKCS1_2048_8192_SHA256, public_key_bytes).unwrap();

    let rng = rand::SystemRandom::new();
    let message = b"test message";
    let mut signature = vec![0; key_pair.public_modulus_len()];
    key_pair
        .sign(
            &crate::signature::RSA_PKCS1_SHA256,
            &rng,
            message,
            &mut signature,
        )
        .unwrap();

    assert!(parsed.verify_sig(message, &signature).is_ok());
    assert!(parsed.verify_sig(b"wrong message", &signature).is_err());
}

#[test]
fn test_parsed_public_key_rsa_pss() {
    let key_pair = RsaKeyPair::generate(KeySize::Rsa2048).unwrap();
    let public_key_bytes = key_pair.public_key().as_ref();

    let parsed = ParsedPublicKey::new(&RSA_PSS_2048_8192_SHA256, public_key_bytes).unwrap();

    let rng = rand::SystemRandom::new();
    let message = b"test message";
    let mut signature = vec![0; key_pair.public_modulus_len()];
    key_pair
        .sign(
            &crate::signature::RSA_PSS_SHA256,
            &rng,
            message,
            &mut signature,
        )
        .unwrap();

    assert!(parsed.verify_sig(message, &signature).is_ok());
    assert!(parsed.verify_sig(b"wrong message", &signature).is_err());
}

#[test]
fn test_parsed_public_key_verify_digest() {
    let key_pair = Ed25519KeyPair::generate().unwrap();
    let public_key_bytes = key_pair.public_key().as_ref();

    let parsed = ParsedPublicKey::new(&ED25519, public_key_bytes).unwrap();

    let message = b"test message";
    let digest_value = digest(&SHA256, message);
    let signature = key_pair.sign(message);

    // Note: Ed25519 doesn't support digest verification in the same way as RSA/ECDSA
    // This test verifies the API exists and handles the case appropriately
    let result = parsed.verify_digest_sig(&digest_value, signature.as_ref());
    // Ed25519 should return an error for digest verification
    assert!(result.is_err());
}

#[test]
fn test_parsed_public_key_invalid_key() {
    // Test with clearly invalid Ed25519 key (wrong size)
    let invalid_key = [0u8; 31];
    assert!(ParsedPublicKey::new(&ED25519, invalid_key).is_err());

    // Test with invalid ECDSA key (wrong format)
    let invalid_key = [0u8; 65];
    assert!(ParsedPublicKey::new(&ECDSA_P256_SHA256_ASN1, invalid_key).is_err());
}

#[test]
fn test_unparsed_to_parsed_conversion() {
    let key_pair = Ed25519KeyPair::generate().unwrap();
    let public_key_bytes = key_pair.public_key().as_ref();

    let unparsed = UnparsedPublicKey::new(&ED25519, public_key_bytes);
    let parsed = unparsed.parse().unwrap();

    assert_eq!(parsed.algorithm().type_id(), ED25519.type_id());

    let message = b"test message";
    let signature = key_pair.sign(message);

    assert!(parsed.verify_sig(message, signature.as_ref()).is_ok());
    assert!(parsed
        .verify_sig(b"wrong message", signature.as_ref())
        .is_err());
}

#[test]
fn test_parsed_public_key_debug() {
    let key_pair = Ed25519KeyPair::generate().unwrap();
    let public_key_bytes = key_pair.public_key().as_ref();

    let parsed = ParsedPublicKey::new(&ED25519, public_key_bytes).unwrap();
    let debug_str = format!("{parsed:?}");

    assert!(debug_str.contains("ParsedPublicKey"));
    assert!(debug_str.contains("algorithm"));
}

#[test]
fn test_parsed_public_key_efficiency() {
    let key_pair = Ed25519KeyPair::generate().unwrap();
    let public_key_bytes = key_pair.public_key().as_ref();
    let message = b"test message";
    let signature = key_pair.sign(message);

    // Test that ParsedPublicKey can be reused efficiently
    let parsed = ParsedPublicKey::new(&ED25519, public_key_bytes).unwrap();

    // Multiple verifications with the same parsed key
    for _ in 0..10 {
        assert!(parsed.verify_sig(message, signature.as_ref()).is_ok());
    }
}

#[test]
fn test_parsed_vs_unparsed_equivalence() {
    let key_pair = Ed25519KeyPair::generate().unwrap();
    let public_key_bytes = key_pair.public_key().as_ref();
    let message = b"test message";
    let signature = key_pair.sign(message);

    let unparsed = UnparsedPublicKey::new(&ED25519, public_key_bytes);
    let parsed = ParsedPublicKey::new(&ED25519, public_key_bytes).unwrap();

    // Both should give the same verification result
    let unparsed_result = unparsed.verify(message, signature.as_ref());
    let parsed_result = parsed.verify_sig(message, signature.as_ref());

    assert_eq!(unparsed_result.is_ok(), parsed_result.is_ok());

    // Test with wrong message
    let unparsed_result = unparsed.verify(b"wrong", signature.as_ref());
    let parsed_result = parsed.verify_sig(b"wrong", signature.as_ref());

    assert!(unparsed_result.is_err()); // Not OK
    assert_eq!(unparsed_result.is_ok(), parsed_result.is_ok());
}

#[test]
fn test_parsed_public_key_with_test_data_files() {
    // Test with Ed25519 test data
    let ed25519_public_key = include_bytes!("../../tests/data/ed25519_test_public_key.bin");
    let parsed_ed25519 = ParsedPublicKey::new(&ED25519, ed25519_public_key).unwrap();

    // Test with ECDSA P256 test data
    let ecdsa_public_key = include_bytes!("../../tests/data/ecdsa_test_public_key_p256.der");
    let parsed_ecdsa = ParsedPublicKey::new(&ECDSA_P256_SHA256_ASN1, ecdsa_public_key).unwrap();

    // Test with RSA test data
    let rsa_public_key = include_bytes!("../../tests/data/rsa_test_public_key_2048.der");
    let parsed_rsa = ParsedPublicKey::new(&RSA_PKCS1_2048_8192_SHA256, rsa_public_key).unwrap();

    // Verify the parsed keys have the correct algorithms
    assert_eq!(parsed_ed25519.algorithm().type_id(), ED25519.type_id());
    assert_eq!(
        parsed_ecdsa.algorithm().type_id(),
        ECDSA_P256_SHA256_ASN1.type_id()
    );
    assert_eq!(
        parsed_rsa.algorithm().type_id(),
        RSA_PKCS1_2048_8192_SHA256.type_id()
    );
}

#[test]
fn test_parsed_public_key_with_known_vectors() {
    // Test with known Ed25519 test vectors
    let public_key =
        test::from_dirty_hex("d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a");
    let message = b"";
    let signature = test::from_dirty_hex(
            "e5564300c360ac729086e2cc806e828a84877f1eb8e5d974d873e065224901555fb8821590a33bacc61e39701cf9b46bd25bf5f0595bbe24655141438e7a100b"
        );

    let parsed = ParsedPublicKey::new(&ED25519, &public_key).unwrap();
    assert!(parsed.verify_sig(message, &signature).is_ok());

    // Test with wrong signature
    let mut wrong_signature = signature.clone();
    wrong_signature[0] ^= 1;
    assert!(parsed.verify_sig(message, &wrong_signature).is_err());
}

#[test]
fn test_parsed_public_key_algorithm_mismatch() {
    let ed25519_key_pair = Ed25519KeyPair::generate().unwrap();
    let ed25519_public_key = ed25519_key_pair.public_key().as_ref();

    // Try to parse Ed25519 key with ECDSA algorithm - should fail
    assert!(ParsedPublicKey::new(&ECDSA_P256_SHA256_ASN1, ed25519_public_key).is_err());
}

#[test]
fn test_parsed_public_key_empty_bytes() {
    let empty_bytes = [];
    assert!(ParsedPublicKey::new(&ED25519, empty_bytes).is_err());
    assert!(ParsedPublicKey::new(&ECDSA_P256_SHA256_ASN1, empty_bytes).is_err());
    assert!(ParsedPublicKey::new(&RSA_PKCS1_2048_8192_SHA256, empty_bytes).is_err());
}

#[test]
fn test_parsed_public_key_wrong_size() {
    // Ed25519 expects 32 bytes
    let wrong_size_ed25519 = [0u8; 31];
    assert!(ParsedPublicKey::new(&ED25519, wrong_size_ed25519).is_err());

    let wrong_size_ed25519 = [0u8; 33];
    assert!(ParsedPublicKey::new(&ED25519, wrong_size_ed25519).is_err());
}

#[test]
fn test_parsed_public_key_malformed_der() {
    // Test with malformed DER data
    let malformed_der = [0x30, 0x82, 0x01, 0x22]; // Incomplete DER structure
    assert!(ParsedPublicKey::new(&ECDSA_P256_SHA256_ASN1, malformed_der).is_err());
    assert!(ParsedPublicKey::new(&RSA_PKCS1_2048_8192_SHA256, malformed_der).is_err());
}

#[test]
fn test_parsed_public_key_cross_verification() {
    // Test Ed25519
    {
        let key_pair = Ed25519KeyPair::generate().unwrap();
        let public_key_bytes = key_pair.public_key().as_ref();
        let message = b"test message";
        let signature = key_pair.sign(message);

        let unparsed = UnparsedPublicKey::new(&ED25519, public_key_bytes);
        let parsed = ParsedPublicKey::new(&ED25519, public_key_bytes).unwrap();

        // Both should succeed with correct message
        assert_eq!(
            unparsed.verify(message, signature.as_ref()).is_ok(),
            parsed.verify_sig(message, signature.as_ref()).is_ok()
        );

        // Both should fail with wrong message
        let wrong_message = b"wrong message";
        assert_eq!(
            unparsed.verify(wrong_message, signature.as_ref()).is_ok(),
            parsed.verify_sig(wrong_message, signature.as_ref()).is_ok()
        );
    }

    // Test ECDSA P256
    {
        let key_pair = EcdsaKeyPair::generate(&ECDSA_P256_SHA256_ASN1_SIGNING).unwrap();
        let public_key_bytes = key_pair.public_key().as_ref();
        let rng = rand::SystemRandom::new();
        let message = b"test message";
        let signature = key_pair.sign(&rng, message).unwrap();

        let unparsed = UnparsedPublicKey::new(&ECDSA_P256_SHA256_ASN1, public_key_bytes);
        let parsed = ParsedPublicKey::new(&ECDSA_P256_SHA256_ASN1, public_key_bytes).unwrap();

        // Both should succeed with correct message
        assert_eq!(
            unparsed.verify(message, signature.as_ref()).is_ok(),
            parsed.verify_sig(message, signature.as_ref()).is_ok()
        );

        // Both should fail with wrong message
        let wrong_message = b"wrong message";
        assert_eq!(
            unparsed.verify(wrong_message, signature.as_ref()).is_ok(),
            parsed.verify_sig(wrong_message, signature.as_ref()).is_ok()
        );
    }
}

#[cfg(all(feature = "unstable", not(feature = "fips")))]
#[test]
fn test_parsed_public_key_ml_dsa_44() {
    let key_pair = PqdsaKeyPair::generate(&ML_DSA_44_SIGNING).unwrap();
    let public_key_bytes = key_pair.public_key().as_ref();

    let parsed = ParsedPublicKey::new(&ML_DSA_44, public_key_bytes).unwrap();

    assert_eq!(parsed.algorithm().type_id(), ML_DSA_44.type_id());

    let message = b"test message";
    let mut signature = vec![0; ML_DSA_44_SIGNING.signature_len()];
    let signature_len = key_pair.sign(message, &mut signature).unwrap();
    signature.truncate(signature_len);

    assert!(parsed.verify_sig(message, &signature).is_ok());
    assert!(parsed.verify_sig(b"wrong message", &signature).is_err());
}

#[cfg(all(feature = "unstable", not(feature = "fips")))]
#[test]
fn test_parsed_public_key_ml_dsa_65() {
    let key_pair = PqdsaKeyPair::generate(&ML_DSA_65_SIGNING).unwrap();
    let public_key_bytes = key_pair.public_key().as_ref();

    let parsed = ParsedPublicKey::new(&ML_DSA_65, public_key_bytes).unwrap();

    assert_eq!(parsed.algorithm().type_id(), ML_DSA_65.type_id());

    let message = b"test message";
    let mut signature = vec![0; ML_DSA_65_SIGNING.signature_len()];
    let signature_len = key_pair.sign(message, &mut signature).unwrap();
    signature.truncate(signature_len);

    assert!(parsed.verify_sig(message, &signature).is_ok());
    assert!(parsed.verify_sig(b"wrong message", &signature).is_err());
}

#[cfg(all(feature = "unstable", not(feature = "fips")))]
#[test]
fn test_parsed_public_key_ml_dsa_87() {
    let key_pair = PqdsaKeyPair::generate(&ML_DSA_87_SIGNING).unwrap();
    let public_key_bytes = key_pair.public_key().as_ref();

    let parsed = ParsedPublicKey::new(&ML_DSA_87, public_key_bytes).unwrap();

    assert_eq!(parsed.algorithm().type_id(), ML_DSA_87.type_id());

    let message = b"test message";
    let mut signature = vec![0; ML_DSA_87_SIGNING.signature_len()];
    let signature_len = key_pair.sign(message, &mut signature).unwrap();
    signature.truncate(signature_len);

    assert!(parsed.verify_sig(message, &signature).is_ok());
    assert!(parsed.verify_sig(b"wrong message", &signature).is_err());
}

#[cfg(all(feature = "unstable", not(feature = "fips")))]
#[test]
fn test_parsed_public_key_pqdsa_with_x509_der() {
    use crate::encoding::AsDer;

    let key_pair = PqdsaKeyPair::generate(&ML_DSA_44_SIGNING).unwrap();
    let x509_der = key_pair.public_key().as_der().unwrap();

    let parsed = ParsedPublicKey::new(&ML_DSA_44, x509_der.as_ref()).unwrap();

    let message = b"test message";
    let mut signature = vec![0; ML_DSA_44_SIGNING.signature_len()];
    let signature_len = key_pair.sign(message, &mut signature).unwrap();
    signature.truncate(signature_len);

    assert!(parsed.verify_sig(message, &signature).is_ok());
}

#[cfg(all(feature = "unstable", not(feature = "fips")))]
#[test]
fn test_parsed_public_key_pqdsa_invalid_key() {
    // Test with wrong size key for ML-DSA-44 (expects 1312 bytes)
    let invalid_key = [0u8; 1311];
    assert!(ParsedPublicKey::new(&ML_DSA_44, invalid_key).is_err());

    let invalid_key = [0u8; 1313];
    assert!(ParsedPublicKey::new(&ML_DSA_44, invalid_key).is_err());
}

#[cfg(all(feature = "unstable", not(feature = "fips")))]
#[test]
fn test_parsed_public_key_pqdsa_algorithm_mismatch() {
    let key_pair = PqdsaKeyPair::generate(&ML_DSA_44_SIGNING).unwrap();
    let public_key_bytes = key_pair.public_key().as_ref();

    // Try to parse ML-DSA-44 key with ML-DSA-65 algorithm - should fail
    assert!(ParsedPublicKey::new(&ML_DSA_65, public_key_bytes).is_err());

    // Try to parse ML-DSA-44 key with Ed25519 algorithm - should fail
    assert!(ParsedPublicKey::new(&ED25519, public_key_bytes).is_err());
}

#[cfg(all(feature = "unstable", not(feature = "fips")))]
#[test]
fn test_parsed_public_key_pqdsa_cross_verification() {
    let key_pair = PqdsaKeyPair::generate(&ML_DSA_44_SIGNING).unwrap();
    let public_key_bytes = key_pair.public_key().as_ref();
    let message = b"test message";
    let mut signature = vec![0; ML_DSA_44_SIGNING.signature_len()];
    let signature_len = key_pair.sign(message, &mut signature).unwrap();
    signature.truncate(signature_len);

    let unparsed = UnparsedPublicKey::new(&ML_DSA_44, public_key_bytes);
    let parsed = ParsedPublicKey::new(&ML_DSA_44, public_key_bytes).unwrap();

    // Both should succeed with correct message
    assert_eq!(
        unparsed.verify(message, &signature).is_ok(),
        parsed.verify_sig(message, &signature).is_ok()
    );

    // Both should fail with wrong message
    let wrong_message = b"wrong message";
    assert_eq!(
        unparsed.verify(wrong_message, &signature).is_ok(),
        parsed.verify_sig(wrong_message, &signature).is_ok()
    );
}

#[cfg(all(feature = "unstable", not(feature = "fips")))]
#[test]
fn test_parsed_public_key_pqdsa_verify_digest() {
    let key_pair = PqdsaKeyPair::generate(&ML_DSA_44_SIGNING).unwrap();
    let public_key_bytes = key_pair.public_key().as_ref();

    let parsed = ParsedPublicKey::new(&ML_DSA_44, public_key_bytes).unwrap();

    let message = b"test message";
    let digest_value = digest(&SHA256, message);
    let mut signature = vec![0; ML_DSA_44_SIGNING.signature_len()];
    let signature_len = key_pair.sign(message, &mut signature).unwrap();
    signature.truncate(signature_len);

    // PQDSA should return an error for digest verification
    let result = parsed.verify_digest_sig(&digest_value, &signature);
    assert!(result.is_err());
}
