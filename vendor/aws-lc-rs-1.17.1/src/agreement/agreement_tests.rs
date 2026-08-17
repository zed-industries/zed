// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

use crate::agreement::{
    agree, Algorithm, PrivateKey, PublicKey, UnparsedPublicKey, ECDH_P256, ECDH_P384, ECDH_P521,
    X25519,
};
use crate::encoding::{
    AsBigEndian, AsDer, Curve25519SeedBin, EcPrivateKeyBin, EcPrivateKeyRfc5915Der,
    EcPublicKeyCompressedBin, EcPublicKeyUncompressedBin, Pkcs8V1Der, PublicKeyX509Der,
};
use crate::{rand, test};

#[test]
fn test_agreement_x25519() {
    let alg = &X25519;
    let peer_public = UnparsedPublicKey::new(
        alg,
        test::from_dirty_hex("e6db6867583030db3594c1a424b15f7c726624ec26b3353b10a903a6d0ab1c4c"),
    );

    let my_private =
        test::from_dirty_hex("a546e36bf0527c9d3b16154b82465edd62144c0ac1fc5a18506a2244ba449ac4");

    let my_private = {
        let mut rng = test::rand::FixedSliceRandom { bytes: &my_private };
        PrivateKey::generate_for_test(alg, &mut rng).unwrap()
    };

    let my_public =
        test::from_dirty_hex("1c9fd88f45606d932a80c71824ae151d15d73e77de38e8e000852e614fae7019");
    let output =
        test::from_dirty_hex("c3da55379de9c6908e94ea4df28d084f32eccf03491c71f754b4075577a28552");

    assert_eq!(my_private.algorithm(), alg);

    let be_private_key_buffer: Curve25519SeedBin = my_private.as_be_bytes().unwrap();
    let be_private_key =
        PrivateKey::from_private_key(&X25519, be_private_key_buffer.as_ref()).unwrap();
    {
        let result = agree(&be_private_key, &peer_public, (), |key_material| {
            assert_eq!(key_material, &output[..]);
            Ok(())
        });
        assert_eq!(result, Ok(()));
    }

    let computed_public = my_private.compute_public_key().unwrap();
    assert_eq!(computed_public.as_ref(), &my_public[..]);

    assert_eq!(computed_public.algorithm(), alg);
    {
        let result = agree(&my_private, &peer_public, (), |key_material| {
            assert_eq!(key_material, &output[..]);
            Ok(())
        });
        assert_eq!(result, Ok(()));
    }
    {
        let result = agree(&my_private, &peer_public, (), |key_material| {
            assert_eq!(key_material, &output[..]);
            Ok(())
        });
        assert_eq!(result, Ok(()));
    }
}

#[test]
fn test_agreement_invalid_keys() {
    fn test_with_key(alg: &'static Algorithm, my_private_key: &PrivateKey, test_key: &[u8]) {
        assert!(PrivateKey::from_private_key(alg, test_key).is_err());
        assert!(PrivateKey::from_private_key_der(alg, test_key).is_err());
        assert!(agree(
            my_private_key,
            UnparsedPublicKey::new(alg, test_key),
            (),
            |_| Ok(())
        )
        .is_err());
    }

    let alg_variants: [&'static Algorithm; 4] = [&X25519, &ECDH_P256, &ECDH_P384, &ECDH_P521];

    for alg in alg_variants {
        let my_private_key = PrivateKey::generate(alg).unwrap();

        let empty_key = [];
        test_with_key(alg, &my_private_key, &empty_key);

        let wrong_size_key: [u8; 31] = [
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30,
        ];
        test_with_key(alg, &my_private_key, &wrong_size_key);
    }
}

#[test]
fn test_agreement_ecdh_p256() {
    let alg = &ECDH_P256;
    let peer_public = UnparsedPublicKey::new(
            alg,
            test::from_dirty_hex(
                "04D12DFB5289C8D4F81208B70270398C342296970A0BCCB74C736FC7554494BF6356FBF3CA366CC23E8157854C13C58D6AAC23F046ADA30F8353E74F33039872AB",
            ),
        );
    assert_eq!(peer_public.algorithm(), alg);
    assert_eq!(peer_public.bytes(), &peer_public.bytes);

    let my_private =
        test::from_dirty_hex("C88F01F510D9AC3F70A292DAA2316DE544E9AAB8AFE84049C62A9C57862D1433");

    let my_private = {
        let mut rng = test::rand::FixedSliceRandom { bytes: &my_private };
        PrivateKey::generate_for_test(alg, &mut rng).unwrap()
    };

    let my_public = test::from_dirty_hex(
            "04DAD0B65394221CF9B051E1FECA5787D098DFE637FC90B9EF945D0C37725811805271A0461CDB8252D61F1C456FA3E59AB1F45B33ACCF5F58389E0577B8990BB3",
        );
    let output =
        test::from_dirty_hex("D6840F6B42F6EDAFD13116E0E12565202FEF8E9ECE7DCE03812464D04B9442DE");

    assert_eq!(my_private.algorithm(), alg);

    let be_private_key_buffer: EcPrivateKeyBin = my_private.as_be_bytes().unwrap();
    let be_private_key =
        PrivateKey::from_private_key(&ECDH_P256, be_private_key_buffer.as_ref()).unwrap();
    {
        let result = agree(&be_private_key, &peer_public, (), |key_material| {
            assert_eq!(key_material, &output[..]);
            Ok(())
        });
        assert_eq!(result, Ok(()));
    }

    let der_private_key_buffer: EcPrivateKeyRfc5915Der = my_private.as_der().unwrap();
    let der_private_key =
        PrivateKey::from_private_key_der(&ECDH_P256, der_private_key_buffer.as_ref()).unwrap();
    {
        let result = agree(&der_private_key, &peer_public, (), |key_material| {
            assert_eq!(key_material, &output[..]);
            Ok(())
        });
        assert_eq!(result, Ok(()));
    }

    let pkcs8_private_key_buffer: Pkcs8V1Der = my_private.as_der().unwrap();
    let pkcs8_private_key =
        PrivateKey::from_private_key_der(&ECDH_P256, pkcs8_private_key_buffer.as_ref()).unwrap();
    {
        let result = agree(&pkcs8_private_key, &peer_public, (), |key_material| {
            assert_eq!(key_material, &output[..]);
            Ok(())
        });
        assert_eq!(result, Ok(()));
    }

    let computed_public = my_private.compute_public_key().unwrap();
    assert_eq!(computed_public.as_ref(), &my_public[..]);

    assert_eq!(computed_public.algorithm(), alg);

    {
        let result = agree(&my_private, &peer_public, (), |key_material| {
            assert_eq!(key_material, &output[..]);
            Ok(())
        });
        assert_eq!(result, Ok(()));
    }

    {
        let result = agree(&my_private, &peer_public, (), |key_material| {
            assert_eq!(key_material, &output[..]);
            Ok(())
        });
        assert_eq!(result, Ok(()));
    }
}

#[test]
fn test_agreement_ecdh_p384() {
    let alg = &ECDH_P384;
    let peer_public = UnparsedPublicKey::new(
            alg,
            test::from_dirty_hex(
                "04E558DBEF53EECDE3D3FCCFC1AEA08A89A987475D12FD950D83CFA41732BC509D0D1AC43A0336DEF96FDA41D0774A3571DCFBEC7AACF3196472169E838430367F66EEBE3C6E70C416DD5F0C68759DD1FFF83FA40142209DFF5EAAD96DB9E6386C",
            ),
        );

    let my_private = test::from_dirty_hex(
            "099F3C7034D4A2C699884D73A375A67F7624EF7C6B3C0F160647B67414DCE655E35B538041E649EE3FAEF896783AB194",
        );

    let my_private = {
        let mut rng = test::rand::FixedSliceRandom { bytes: &my_private };
        PrivateKey::generate_for_test(alg, &mut rng).unwrap()
    };

    let my_public = test::from_dirty_hex(
            "04667842D7D180AC2CDE6F74F37551F55755C7645C20EF73E31634FE72B4C55EE6DE3AC808ACB4BDB4C88732AEE95F41AA9482ED1FC0EEB9CAFC4984625CCFC23F65032149E0E144ADA024181535A0F38EEB9FCFF3C2C947DAE69B4C634573A81C",
        );
    let output = test::from_dirty_hex(
            "11187331C279962D93D604243FD592CB9D0A926F422E47187521287E7156C5C4D603135569B9E9D09CF5D4A270F59746",
        );

    assert_eq!(my_private.algorithm(), alg);

    let be_private_key_buffer: EcPrivateKeyBin = my_private.as_be_bytes().unwrap();
    let be_private_key =
        PrivateKey::from_private_key(&ECDH_P384, be_private_key_buffer.as_ref()).unwrap();
    {
        let result = agree(&be_private_key, &peer_public, (), |key_material| {
            assert_eq!(key_material, &output[..]);
            Ok(())
        });
        assert_eq!(result, Ok(()));
    }

    let der_private_key_buffer: EcPrivateKeyRfc5915Der = my_private.as_der().unwrap();
    let der_private_key =
        PrivateKey::from_private_key_der(&ECDH_P384, der_private_key_buffer.as_ref()).unwrap();
    {
        let result = agree(&der_private_key, &peer_public, (), |key_material| {
            assert_eq!(key_material, &output[..]);
            Ok(())
        });
        assert_eq!(result, Ok(()));
    }

    let computed_public = my_private.compute_public_key().unwrap();
    assert_eq!(computed_public.as_ref(), &my_public[..]);

    assert_eq!(computed_public.algorithm(), alg);

    {
        let result = agree(&my_private, &peer_public, (), |key_material| {
            assert_eq!(key_material, &output[..]);
            Ok(())
        });
        assert_eq!(result, Ok(()));
    }
}

#[test]
fn test_agreement_ecdh_p521() {
    let alg = &ECDH_P521;
    let peer_public = UnparsedPublicKey::new(
            alg,
            test::from_dirty_hex(
                "0401a32099b02c0bd85371f60b0dd20890e6c7af048c8179890fda308b359dbbc2b7a832bb8c6526c4af99a7ea3f0b3cb96ae1eb7684132795c478ad6f962e4a6f446d017627357b39e9d7632a1370b3e93c1afb5c851b910eb4ead0c9d387df67cde85003e0e427552f1cd09059aad0262e235cce5fba8cedc4fdc1463da76dcd4b6d1a46",
            ),
        );

    let my_private = test::from_dirty_hex(
            "00df14b1f1432a7b0fb053965fd8643afee26b2451ecb6a8a53a655d5fbe16e4c64ce8647225eb11e7fdcb23627471dffc5c2523bd2ae89957cba3a57a23933e5a78",
        );

    let my_private = {
        let mut rng = test::rand::FixedSliceRandom { bytes: &my_private };
        PrivateKey::generate_for_test(alg, &mut rng).unwrap()
    };

    let my_public = test::from_dirty_hex(
            "04004e8583bbbb2ecd93f0714c332dff5ab3bc6396e62f3c560229664329baa5138c3bb1c36428abd4e23d17fcb7a2cfcc224b2e734c8941f6f121722d7b6b9415457601cf0874f204b0363f020864672fadbf87c8811eb147758b254b74b14fae742159f0f671a018212bbf25b8519e126d4cad778cfff50d288fd39ceb0cac635b175ec0",
        );
    let output = test::from_dirty_hex(
            "01aaf24e5d47e4080c18c55ea35581cd8da30f1a079565045d2008d51b12d0abb4411cda7a0785b15d149ed301a3697062f42da237aa7f07e0af3fd00eb1800d9c41",
        );

    assert_eq!(my_private.algorithm(), alg);

    let be_private_key_buffer: EcPrivateKeyBin = my_private.as_be_bytes().unwrap();
    let be_private_key =
        PrivateKey::from_private_key(&ECDH_P521, be_private_key_buffer.as_ref()).unwrap();
    {
        let result = agree(&be_private_key, &peer_public, (), |key_material| {
            assert_eq!(key_material, &output[..]);
            Ok(())
        });
        assert_eq!(result, Ok(()));
    }

    let der_private_key_buffer: EcPrivateKeyRfc5915Der = my_private.as_der().unwrap();
    let der_private_key =
        PrivateKey::from_private_key_der(&ECDH_P521, der_private_key_buffer.as_ref()).unwrap();
    {
        let result = agree(&der_private_key, &peer_public, (), |key_material| {
            assert_eq!(key_material, &output[..]);
            Ok(())
        });
        assert_eq!(result, Ok(()));
    }

    let computed_public = my_private.compute_public_key().unwrap();
    assert_eq!(computed_public.as_ref(), &my_public[..]);

    assert_eq!(computed_public.algorithm(), alg);
    {
        let result = agree(&my_private, &peer_public, (), |key_material| {
            assert_eq!(key_material, &output[..]);
            Ok(())
        });
        assert_eq!(result, Ok(()));
    }
    {
        let result = agree(&my_private, &peer_public, (), |key_material| {
            assert_eq!(key_material, &output[..]);
            Ok(())
        });
        assert_eq!(result, Ok(()));
    }
}

#[test]
fn agreement_traits() {
    use crate::test;
    use regex::{self, Regex};

    let mut rng = rand::SystemRandom::new();
    let private_key = PrivateKey::generate_for_test(&ECDH_P256, &mut rng).unwrap();

    test::compile_time_assert_send::<PrivateKey>();
    test::compile_time_assert_sync::<PrivateKey>();

    assert_eq!(
        format!("{private_key:?}"),
        "PrivateKey { algorithm: Algorithm { curve: P256 } }"
    );

    let ephemeral_private_key = PrivateKey::generate_for_test(&ECDH_P256, &mut rng).unwrap();

    test::compile_time_assert_send::<PrivateKey>();
    test::compile_time_assert_sync::<PrivateKey>();

    assert_eq!(
        format!("{ephemeral_private_key:?}"),
        "PrivateKey { algorithm: Algorithm { curve: P256 } }"
    );

    let public_key = private_key.compute_public_key().unwrap();
    let pubkey_re = Regex::new(
        "PublicKey \\{ algorithm: Algorithm \\{ curve: P256 \\}, bytes: \"[0-9a-f]+\" \\}",
    )
    .unwrap();
    let pubkey_debug = format!("{public_key:?}");

    assert!(
        pubkey_re.is_match(&pubkey_debug),
        "pubkey_debug: {pubkey_debug}"
    );

    #[allow(clippy::redundant_clone)]
    let pubkey_clone = public_key.clone();
    assert_eq!(public_key.as_ref(), pubkey_clone.as_ref());
    assert_eq!(pubkey_debug, format!("{pubkey_clone:?}"));

    test::compile_time_assert_clone::<PublicKey>();
    test::compile_time_assert_send::<PublicKey>();
    test::compile_time_assert_sync::<PublicKey>();

    // Verify `PublicKey` implements `Debug`.
    //
    // TODO: Test the actual output.
    let _: &dyn core::fmt::Debug = &public_key;

    test::compile_time_assert_clone::<UnparsedPublicKey<&[u8]>>();
    test::compile_time_assert_copy::<UnparsedPublicKey<&[u8]>>();
    test::compile_time_assert_sync::<UnparsedPublicKey<&[u8]>>();

    test::compile_time_assert_clone::<UnparsedPublicKey<Vec<u8>>>();
    test::compile_time_assert_sync::<UnparsedPublicKey<Vec<u8>>>();

    let bytes = [0x01, 0x02, 0x03];

    let unparsed_public_key = UnparsedPublicKey::new(&X25519, &bytes);
    let unparsed_pubkey_clone = unparsed_public_key;
    assert_eq!(
        format!("{unparsed_public_key:?}"),
        r#"UnparsedPublicKey { algorithm: Algorithm { curve: Curve25519 }, bytes: "010203" }"#
    );
    assert_eq!(
        format!("{unparsed_pubkey_clone:?}"),
        r#"UnparsedPublicKey { algorithm: Algorithm { curve: Curve25519 }, bytes: "010203" }"#
    );

    let unparsed_public_key = UnparsedPublicKey::new(&X25519, Vec::from(bytes));
    #[allow(clippy::redundant_clone)]
    let unparsed_pubkey_clone = unparsed_public_key.clone();
    assert_eq!(
        format!("{unparsed_public_key:?}"),
        r#"UnparsedPublicKey { algorithm: Algorithm { curve: Curve25519 }, bytes: "010203" }"#
    );
    assert_eq!(
        format!("{unparsed_pubkey_clone:?}"),
        r#"UnparsedPublicKey { algorithm: Algorithm { curve: Curve25519 }, bytes: "010203" }"#
    );
}

#[test]
fn test_agreement_random() {
    let test_algorithms = [&ECDH_P256, &ECDH_P384, &ECDH_P521, &X25519];

    for alg in test_algorithms {
        test_agreement_random_helper(alg);
    }
}

fn test_agreement_random_helper(alg: &'static Algorithm) {
    let peer_private = PrivateKey::generate(alg).unwrap();
    let my_private = PrivateKey::generate(alg).unwrap();

    let peer_public_keys = public_key_formats_helper(&peer_private.compute_public_key().unwrap());

    let my_public_keys = public_key_formats_helper(&my_private.compute_public_key().unwrap());

    let mut results: Vec<Vec<u8>> = Vec::new();

    for peer_public in peer_public_keys {
        let peer_public = UnparsedPublicKey::new(alg, peer_public);
        let result = agree(&my_private, &peer_public, (), |key_material| {
            results.push(key_material.to_vec());
            Ok(())
        });
        assert_eq!(result, Ok(()));
    }

    for my_public in my_public_keys {
        let my_public = UnparsedPublicKey::new(alg, my_public);
        let result = agree(&peer_private, &my_public, (), |key_material| {
            results.push(key_material.to_vec());
            Ok(())
        });
        assert_eq!(result, Ok(()));
    }

    let key_types_tested = match alg.id {
        crate::agreement::AlgorithmID::ECDH_P256
        | crate::agreement::AlgorithmID::ECDH_P384
        | crate::agreement::AlgorithmID::ECDH_P521 => 4,
        crate::agreement::AlgorithmID::X25519 => 2,
    };

    assert_eq!(results.len(), key_types_tested * 2); // Multiplied by two because we tested the other direction

    assert_eq!(results[0..key_types_tested], results[key_types_tested..]);
}

fn public_key_formats_helper(public_key: &PublicKey) -> Vec<Vec<u8>> {
    let verify_ec_raw_traits = matches!(
        public_key.algorithm().id,
        crate::agreement::AlgorithmID::ECDH_P256
            | crate::agreement::AlgorithmID::ECDH_P384
            | crate::agreement::AlgorithmID::ECDH_P521
    );

    let mut public_keys = Vec::<Vec<u8>>::new();
    public_keys.push(public_key.as_ref().into());

    if verify_ec_raw_traits {
        let raw = AsBigEndian::<EcPublicKeyCompressedBin>::as_be_bytes(public_key).unwrap();
        public_keys.push(raw.as_ref().into());
        let raw = AsBigEndian::<EcPublicKeyUncompressedBin>::as_be_bytes(public_key).unwrap();
        public_keys.push(raw.as_ref().into());
    }

    let peer_x509 = AsDer::<PublicKeyX509Der>::as_der(public_key).unwrap();
    public_keys.push(peer_x509.as_ref().into());

    public_keys
}

#[test]
fn private_key_drop() {
    let private_key = PrivateKey::generate(&ECDH_P256).unwrap();
    let public_key = private_key.compute_public_key().unwrap();
    // PublicKey maintains a reference counted pointer to private keys EVP_PKEY so we test that with drop
    drop(private_key);
    let _ = AsBigEndian::<EcPublicKeyCompressedBin>::as_be_bytes(&public_key).unwrap();
    let _ = AsBigEndian::<EcPublicKeyUncompressedBin>::as_be_bytes(&public_key).unwrap();
    let _ = AsDer::<PublicKeyX509Der>::as_der(&public_key).unwrap();
}
