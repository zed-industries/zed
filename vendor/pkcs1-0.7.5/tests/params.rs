//! PKCS#1 algorithm params tests

use const_oid::db;
use der::{
    asn1::{AnyRef, ObjectIdentifier, OctetStringRef},
    oid::AssociatedOid,
    Encode,
};
use hex_literal::hex;
use pkcs1::{RsaOaepParams, RsaPssParams, TrailerField};

/// Default PSS parameters using all default values (SHA1, MGF1)
const RSA_PSS_PARAMETERS_DEFAULTS: &[u8] = &hex!("3000");
/// Example PSS parameters using SHA256 instead of SHA1
const RSA_PSS_PARAMETERS_SHA2_256: &[u8] = &hex!("3034a00f300d06096086480165030402010500a11c301a06092a864886f70d010108300d06096086480165030402010500a203020120");

/// Default OAEP parameters using all default values (SHA1, MGF1, Empty)
const RSA_OAEP_PARAMETERS_DEFAULTS: &[u8] = &hex!("3000");
/// Example OAEP parameters using SHA256 instead of SHA1
const RSA_OAEP_PARAMETERS_SHA2_256: &[u8] = &hex!("302fa00f300d06096086480165030402010500a11c301a06092a864886f70d010108300d06096086480165030402010500");

struct Sha1Mock {}
impl AssociatedOid for Sha1Mock {
    const OID: ObjectIdentifier = ObjectIdentifier::new_unwrap("1.3.14.3.2.26");
}

struct Sha256Mock {}
impl AssociatedOid for Sha256Mock {
    const OID: ObjectIdentifier = ObjectIdentifier::new_unwrap("2.16.840.1.101.3.4.2.1");
}

#[test]
fn decode_pss_param() {
    let param = RsaPssParams::try_from(RSA_PSS_PARAMETERS_SHA2_256).unwrap();

    assert!(param
        .hash
        .assert_algorithm_oid(db::rfc5912::ID_SHA_256)
        .is_ok());
    assert_eq!(param.hash.parameters, Some(AnyRef::NULL));
    assert!(param
        .mask_gen
        .assert_algorithm_oid(db::rfc5912::ID_MGF_1)
        .is_ok());
    assert!(param
        .mask_gen
        .parameters
        .unwrap()
        .assert_algorithm_oid(db::rfc5912::ID_SHA_256)
        .is_ok());
    assert_eq!(param.salt_len, 32);
    assert_eq!(param.trailer_field, TrailerField::BC);
}

#[test]
fn encode_pss_param() {
    let mut buf = [0_u8; 256];
    let param = RsaPssParams::try_from(RSA_PSS_PARAMETERS_SHA2_256).unwrap();
    assert_eq!(
        param.encode_to_slice(&mut buf).unwrap(),
        RSA_PSS_PARAMETERS_SHA2_256
    );
}

#[test]
fn decode_pss_param_default() {
    let param = RsaPssParams::try_from(RSA_PSS_PARAMETERS_DEFAULTS).unwrap();

    assert!(param
        .hash
        .assert_algorithm_oid(db::rfc5912::ID_SHA_1)
        .is_ok());
    assert_eq!(param.hash.parameters, Some(AnyRef::NULL));
    assert!(param
        .mask_gen
        .assert_algorithm_oid(db::rfc5912::ID_MGF_1)
        .is_ok());
    assert!(param
        .mask_gen
        .parameters
        .unwrap()
        .assert_algorithm_oid(db::rfc5912::ID_SHA_1)
        .is_ok());
    assert_eq!(
        param.mask_gen.parameters.unwrap().parameters,
        Some(AnyRef::NULL)
    );
    assert_eq!(param.salt_len, 20);
    assert_eq!(param.trailer_field, TrailerField::BC);
    assert_eq!(param, Default::default())
}

#[test]
fn encode_pss_param_default() {
    let mut buf = [0_u8; 256];
    assert_eq!(
        RsaPssParams::default().encode_to_slice(&mut buf).unwrap(),
        RSA_PSS_PARAMETERS_DEFAULTS
    );
}

#[test]
fn new_pss_param() {
    let mut buf = [0_u8; 256];

    let param = RsaPssParams::new::<Sha1Mock>(20);
    assert_eq!(
        param.encode_to_slice(&mut buf).unwrap(),
        RSA_PSS_PARAMETERS_DEFAULTS
    );

    let param = RsaPssParams::new::<Sha256Mock>(32);
    assert_eq!(
        param.encode_to_slice(&mut buf).unwrap(),
        RSA_PSS_PARAMETERS_SHA2_256
    );
}

#[test]
fn decode_oaep_param() {
    let param = RsaOaepParams::try_from(RSA_OAEP_PARAMETERS_SHA2_256).unwrap();

    assert!(param
        .hash
        .assert_algorithm_oid(db::rfc5912::ID_SHA_256)
        .is_ok());
    assert_eq!(param.hash.parameters, Some(AnyRef::NULL));
    assert!(param
        .mask_gen
        .assert_algorithm_oid(db::rfc5912::ID_MGF_1)
        .is_ok());
    assert!(param
        .mask_gen
        .parameters
        .unwrap()
        .assert_algorithm_oid(db::rfc5912::ID_SHA_256)
        .is_ok());
    assert!(param
        .p_source
        .assert_algorithm_oid(db::rfc5912::ID_P_SPECIFIED)
        .is_ok());
    assert!(param
        .p_source
        .parameters_any()
        .unwrap()
        .decode_as::<OctetStringRef<'_>>()
        .unwrap()
        .is_empty(),);
}

#[test]
fn encode_oaep_param() {
    let mut buf = [0_u8; 256];
    let param = RsaOaepParams::try_from(RSA_OAEP_PARAMETERS_SHA2_256).unwrap();
    assert_eq!(
        param.encode_to_slice(&mut buf).unwrap(),
        RSA_OAEP_PARAMETERS_SHA2_256
    );
}

#[test]
fn decode_oaep_param_default() {
    let param = RsaOaepParams::try_from(RSA_OAEP_PARAMETERS_DEFAULTS).unwrap();

    assert!(param
        .hash
        .assert_algorithm_oid(db::rfc5912::ID_SHA_1)
        .is_ok());
    assert_eq!(param.hash.parameters, Some(AnyRef::NULL));
    assert!(param
        .mask_gen
        .assert_algorithm_oid(db::rfc5912::ID_MGF_1)
        .is_ok());
    assert!(param
        .mask_gen
        .parameters
        .unwrap()
        .assert_algorithm_oid(db::rfc5912::ID_SHA_1)
        .is_ok());
    assert_eq!(
        param.mask_gen.parameters.unwrap().parameters,
        Some(AnyRef::NULL)
    );
    assert!(param
        .p_source
        .assert_algorithm_oid(db::rfc5912::ID_P_SPECIFIED)
        .is_ok());
    assert!(param
        .p_source
        .parameters_any()
        .unwrap()
        .decode_as::<OctetStringRef<'_>>()
        .unwrap()
        .is_empty(),);
    assert_eq!(param, Default::default())
}

#[test]
fn encode_oaep_param_default() {
    let mut buf = [0_u8; 256];
    assert_eq!(
        RsaOaepParams::default().encode_to_slice(&mut buf).unwrap(),
        RSA_OAEP_PARAMETERS_DEFAULTS
    );
}

#[test]
fn new_oaep_param() {
    let mut buf = [0_u8; 256];

    let param = RsaOaepParams::new::<Sha1Mock>();
    assert_eq!(
        param.encode_to_slice(&mut buf).unwrap(),
        RSA_OAEP_PARAMETERS_DEFAULTS
    );

    let param = RsaOaepParams::new::<Sha256Mock>();
    println!("{:02x?}", param.encode_to_slice(&mut buf).unwrap());
    assert_eq!(
        param.encode_to_slice(&mut buf).unwrap(),
        RSA_OAEP_PARAMETERS_SHA2_256
    );
}
