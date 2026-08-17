// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

use crate::agreement::{
    agree, ParsedPublicKey, ParsedPublicKeyFormat, PrivateKey, UnparsedPublicKey, ECDH_P256,
    ECDH_P384, ECDH_P521, X25519,
};
use crate::encoding::{AsBigEndian, AsDer, EcPublicKeyCompressedBin, PublicKeyX509Der};
use crate::test;

#[test]
fn test_types() {
    test::compile_time_assert_send::<UnparsedPublicKey<&[u8]>>();
    test::compile_time_assert_sync::<UnparsedPublicKey<&[u8]>>();
    test::compile_time_assert_send::<UnparsedPublicKey<Vec<u8>>>();
    test::compile_time_assert_sync::<UnparsedPublicKey<Vec<u8>>>();
    test::compile_time_assert_clone::<UnparsedPublicKey<&[u8]>>();
    test::compile_time_assert_clone::<UnparsedPublicKey<Vec<u8>>>();
    test::compile_time_assert_send::<ParsedPublicKey>();
    test::compile_time_assert_sync::<ParsedPublicKey>();
    test::compile_time_assert_clone::<ParsedPublicKey>();
}

#[test]
fn test_parsed_public_key_x25519_raw() {
    let raw_key =
        test::from_dirty_hex("e6db6867583030db3594c1a424b15f7c726624ec26b3353b10a903a6d0ab1c4c");
    let parsed = ParsedPublicKey::new(&raw_key, X25519.id.nid()).unwrap();
    assert_eq!(&raw_key, parsed.as_ref());

    assert_eq!(parsed.format(), ParsedPublicKeyFormat::Raw);
    assert_eq!(parsed.alg(), &X25519);
}

#[test]
fn test_parsed_public_key_x25519_x509() {
    let private_key = PrivateKey::generate(&X25519).unwrap();
    let public_key = private_key.compute_public_key().unwrap();
    let x509_der: PublicKeyX509Der = public_key.as_der().unwrap();

    let parsed = ParsedPublicKey::new(x509_der.as_ref(), X25519.id.nid()).unwrap();

    assert_eq!(parsed.format(), ParsedPublicKeyFormat::X509);
    assert_eq!(parsed.alg(), &X25519);
}

#[test]
fn test_parsed_public_key_p256_uncompressed() {
    let uncompressed_key = test::from_dirty_hex(
            "04D12DFB5289C8D4F81208B70270398C342296970A0BCCB74C736FC7554494BF6356FBF3CA366CC23E8157854C13C58D6AAC23F046ADA30F8353E74F33039872AB",
        );
    let parsed = ParsedPublicKey::new(&uncompressed_key, ECDH_P256.id.nid()).unwrap();

    assert_eq!(parsed.format(), ParsedPublicKeyFormat::Uncompressed);
    assert_eq!(parsed.alg(), &ECDH_P256);
}

#[test]
fn test_parsed_public_key_p256_compressed() {
    let private_key = PrivateKey::generate(&ECDH_P256).unwrap();
    let public_key = private_key.compute_public_key().unwrap();
    let compressed: EcPublicKeyCompressedBin = public_key.as_be_bytes().unwrap();

    let parsed = ParsedPublicKey::new(compressed.as_ref(), ECDH_P256.id.nid()).unwrap();

    assert_eq!(parsed.format(), ParsedPublicKeyFormat::Compressed);
    assert_eq!(parsed.alg(), &ECDH_P256);
}

#[test]
fn test_parsed_public_key_p256_x509() {
    let private_key = PrivateKey::generate(&ECDH_P256).unwrap();
    let public_key = private_key.compute_public_key().unwrap();
    let x509_der: PublicKeyX509Der = public_key.as_der().unwrap();

    let parsed = ParsedPublicKey::new(x509_der.as_ref(), ECDH_P256.id.nid()).unwrap();

    assert_eq!(parsed.format(), ParsedPublicKeyFormat::X509);
    assert_eq!(parsed.alg(), &ECDH_P256);
}

#[test]
fn test_parsed_public_key_p384() {
    let private_key = PrivateKey::generate(&ECDH_P384).unwrap();
    let public_key = private_key.compute_public_key().unwrap();
    let x509_der: PublicKeyX509Der = public_key.as_der().unwrap();

    let parsed = ParsedPublicKey::new(x509_der.as_ref(), ECDH_P384.id.nid()).unwrap();

    assert_eq!(parsed.format(), ParsedPublicKeyFormat::X509);
    assert_eq!(parsed.alg(), &ECDH_P384);
}

#[test]
fn test_parsed_public_key_p521() {
    let private_key = PrivateKey::generate(&ECDH_P521).unwrap();
    let public_key = private_key.compute_public_key().unwrap();
    let x509_der: PublicKeyX509Der = public_key.as_der().unwrap();

    let parsed = ParsedPublicKey::new(x509_der.as_ref(), ECDH_P521.id.nid()).unwrap();

    assert_eq!(parsed.format(), ParsedPublicKeyFormat::X509);
    assert_eq!(parsed.alg(), &ECDH_P521);
}

#[test]
fn test_parsed_public_key_invalid_empty() {
    let empty_key = [];
    assert!(ParsedPublicKey::new(empty_key, X25519.id.nid()).is_err());
    assert!(ParsedPublicKey::new(empty_key, ECDH_P256.id.nid()).is_err());
}

#[test]
fn test_parsed_public_key_invalid_nid() {
    let raw_key =
        test::from_dirty_hex("e6db6867583030db3594c1a424b15f7c726624ec26b3353b10a903a6d0ab1c4c");
    assert!(ParsedPublicKey::new(&raw_key, 999).is_err());
}

#[test]
fn test_unparsed_to_parsed_conversion() {
    let raw_key =
        test::from_dirty_hex("e6db6867583030db3594c1a424b15f7c726624ec26b3353b10a903a6d0ab1c4c");
    let unparsed = UnparsedPublicKey::new(&X25519, raw_key);

    let parsed: ParsedPublicKey = (&unparsed).try_into().unwrap();
    assert_eq!(parsed.format(), ParsedPublicKeyFormat::Raw);
    assert_eq!(parsed.alg(), &X25519);

    let parsed: ParsedPublicKey = unparsed.try_into().unwrap();
    assert_eq!(parsed.format(), ParsedPublicKeyFormat::Raw);
    assert_eq!(parsed.alg(), &X25519);
}

#[test]
fn test_agree_with_parsed_public_key() {
    let my_private = PrivateKey::generate(&X25519).unwrap();
    let peer_private = PrivateKey::generate(&X25519).unwrap();
    let peer_public = peer_private.compute_public_key().unwrap();

    let parsed = ParsedPublicKey::new(peer_public.as_ref(), X25519.id.nid()).unwrap();

    let result = agree(&my_private, parsed, (), |_key_material| Ok(()));
    assert!(result.is_ok());
}

#[test]
fn test_agree_with_parsed_public_key_algorithm_mismatch() {
    let my_private = PrivateKey::generate(&ECDH_P256).unwrap();
    let peer_private = PrivateKey::generate(&X25519).unwrap();
    let peer_public = peer_private.compute_public_key().unwrap();

    let parsed = ParsedPublicKey::new(peer_public.as_ref(), X25519.id.nid()).unwrap();

    let result = agree(&my_private, parsed, "error", |_key_material| Ok(()));
    assert_eq!(result, Err("error"));
}

#[test]
fn test_parsed_public_key_debug() {
    let raw_key =
        test::from_dirty_hex("e6db6867583030db3594c1a424b15f7c726624ec26b3353b10a903a6d0ab1c4c");
    let parsed = ParsedPublicKey::new(&raw_key, X25519.id.nid()).unwrap();

    let debug_str = format!("{parsed:?}");
    assert!(debug_str.contains("ParsedPublicKey"));
}

#[test]
fn test_parsed_public_key_format_debug() {
    assert_eq!(format!("{:?}", ParsedPublicKeyFormat::Raw), "Raw");
    assert_eq!(format!("{:?}", ParsedPublicKeyFormat::X509), "X509");
    assert_eq!(
        format!("{:?}", ParsedPublicKeyFormat::Compressed),
        "Compressed"
    );
    assert_eq!(
        format!("{:?}", ParsedPublicKeyFormat::Uncompressed),
        "Uncompressed"
    );
}

#[test]
fn test_parsed_public_key_send_sync() {
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    assert_send::<ParsedPublicKey>();
    assert_sync::<ParsedPublicKey>();
}
