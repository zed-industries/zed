// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

use crate::agreement::{agree, Algorithm, ParsedPublicKey, PrivateKey, PublicKey};
use crate::error::Unspecified;
use crate::rand::SecureRandom;
use core::fmt;
use core::fmt::{Debug, Formatter};

/// An ephemeral private key for use (only) with `agree_ephemeral`. The
/// signature of `agree_ephemeral` ensures that an `PrivateKey` can be
/// used for at most one key agreement.
#[allow(clippy::module_name_repetitions)]
pub struct EphemeralPrivateKey(PrivateKey);

impl Debug for EphemeralPrivateKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str(&format!(
            "EphemeralPrivateKey {{ algorithm: {:?} }}",
            self.0.inner_key.algorithm()
        ))
    }
}

impl EphemeralPrivateKey {
    #[inline]
    /// Generate a new ephemeral private key for the given algorithm.
    ///
    /// # *ring* Compatibility
    ///  Our implementation ignores the `SecureRandom` parameter.
    // # FIPS
    // Use this function with one of the following algorithms:
    // * `ECDH_P256`
    // * `ECDH_P384`
    // * `ECDH_P521`
    //
    /// # Errors
    /// `error::Unspecified` when operation fails due to internal error.
    pub fn generate(alg: &'static Algorithm, _rng: &dyn SecureRandom) -> Result<Self, Unspecified> {
        Ok(Self(PrivateKey::generate(alg)?))
    }

    #[cfg(any(test, dev_tests_only))]
    #[allow(missing_docs, clippy::missing_errors_doc)]
    pub fn generate_for_test(
        alg: &'static Algorithm,
        rng: &mut dyn SecureRandom,
    ) -> Result<Self, Unspecified> {
        Ok(Self(PrivateKey::generate_for_test(alg, rng)?))
    }

    /// Computes the public key from the private key.
    ///
    /// # Errors
    /// `error::Unspecified` when operation fails due to internal error.
    pub fn compute_public_key(&self) -> Result<PublicKey, Unspecified> {
        self.0.compute_public_key()
    }

    /// The algorithm for the private key.
    #[inline]
    #[must_use]
    pub fn algorithm(&self) -> &'static Algorithm {
        self.0.algorithm()
    }
}

/// Performs a key agreement with an ephemeral private key and the given public
/// key.
///
/// `my_private_key` is the ephemeral private key to use. Since it is moved, it
/// will not be usable after calling `agree_ephemeral`, thus guaranteeing that
/// the key is used for only one key agreement.
///
/// `peer_public_key` is the peer's public key. `agree_ephemeral` will return
/// `Err(error_value)` if it does not match `my_private_key's` algorithm/curve.
/// `agree_ephemeral` verifies that it is encoded in the standard form for the
/// algorithm and that the key is *valid*; see the algorithm's documentation for
/// details on how keys are to be encoded and what constitutes a valid key for
/// that algorithm.
///
/// `error_value` is the value to return if an error occurs before `kdf` is
/// called, e.g. when decoding of the peer's public key fails or when the public
/// key is otherwise invalid.
///
/// After the key agreement is done, `agree_ephemeral` calls `kdf` with the raw
/// key material from the key agreement operation and then returns what `kdf`
/// returns.
// # FIPS
// Use this function with one of the following key algorithms:
// * `ECDH_P256`
// * `ECDH_P384`
// * `ECDH_P521`
//
/// # Errors
/// `error_value` on internal failure.
#[inline]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::module_name_repetitions)]
pub fn agree_ephemeral<B: TryInto<ParsedPublicKey>, F, R, E>(
    my_private_key: EphemeralPrivateKey,
    peer_public_key: B,
    error_value: E,
    kdf: F,
) -> Result<R, E>
where
    F: FnOnce(&[u8]) -> Result<R, E>,
{
    agree(&my_private_key.0, peer_public_key, error_value, kdf)
}

#[cfg(test)]
mod tests {
    use crate::agreement::{AlgorithmID, PublicKey};
    use crate::encoding::{
        AsBigEndian, AsDer, EcPublicKeyCompressedBin, EcPublicKeyUncompressedBin, PublicKeyX509Der,
    };
    use crate::error::Unspecified;
    use crate::{agreement, rand, test, test_file};

    #[test]
    fn test_agreement_ecdh_x25519_rfc_iterated() {
        fn expect_iterated_x25519(
            expected_result: &str,
            range: core::ops::Range<usize>,
            k: &mut Vec<u8>,
            u: &mut Vec<u8>,
        ) {
            for _ in range {
                let new_k = x25519(k, u);
                u.clone_from(k);
                *k = new_k;
            }
            assert_eq!(&from_hex(expected_result), k);
        }

        let mut k = from_hex("0900000000000000000000000000000000000000000000000000000000000000");
        let mut u = k.clone();

        expect_iterated_x25519(
            "422c8e7a6227d7bca1350b3e2bb7279f7897b87bb6854b783c60e80311ae3079",
            0..1,
            &mut k,
            &mut u,
        );
        expect_iterated_x25519(
            "684cf59ba83309552800ef566f2f4d3c1c3887c49360e3875f2eb94d99532c51",
            1..1_000,
            &mut k,
            &mut u,
        );

        // The spec gives a test vector for 1,000,000 iterations but it takes
        // too long to do 1,000,000 iterations by default right now. This
        // 10,000 iteration vector is self-computed.
        #[cfg(not(disable_slow_tests))]
        expect_iterated_x25519(
            "2c125a20f639d504a7703d2e223c79a79de48c4ee8c23379aa19a62ecd211815",
            1_000..10_000,
            &mut k,
            &mut u,
        );
        /*
               expect_iterated_x25519(
                   "7c3911e0ab2586fd864497297e575e6f3bc601c0883c30df5f4dd2d24f665424",
                   10_000..1_000_000,
                   &mut k,
                   &mut u,
               );
        */
    }

    #[test]
    fn test_agreement_x25519() {
        let alg = &agreement::X25519;
        let peer_public = agreement::UnparsedPublicKey::new(
            alg,
            test::from_dirty_hex(
                "e6db6867583030db3594c1a424b15f7c726624ec26b3353b10a903a6d0ab1c4c",
            ),
        );

        let my_private = test::from_dirty_hex(
            "a546e36bf0527c9d3b16154b82465edd62144c0ac1fc5a18506a2244ba449ac4",
        );

        let my_private = {
            let mut rng = test::rand::FixedSliceRandom { bytes: &my_private };
            agreement::EphemeralPrivateKey::generate_for_test(alg, &mut rng).unwrap()
        };

        let my_public = test::from_dirty_hex(
            "1c9fd88f45606d932a80c71824ae151d15d73e77de38e8e000852e614fae7019",
        );
        let output = test::from_dirty_hex(
            "c3da55379de9c6908e94ea4df28d084f32eccf03491c71f754b4075577a28552",
        );

        assert_eq!(my_private.algorithm(), alg);

        let computed_public = my_private.compute_public_key().unwrap();
        assert_eq!(computed_public.as_ref(), &my_public[..]);

        assert_eq!(computed_public.algorithm(), alg);

        let result = agreement::agree_ephemeral(my_private, &peer_public, (), |key_material| {
            assert_eq!(key_material, &output[..]);
            Ok(())
        });
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_agreement_ecdh_p256() {
        let alg = &agreement::ECDH_P256;
        let peer_public = agreement::UnparsedPublicKey::new(
            alg,
            test::from_dirty_hex(
                "04D12DFB5289C8D4F81208B70270398C342296970A0BCCB74C736FC7554494BF6356FBF3CA366CC23E8157854C13C58D6AAC23F046ADA30F8353E74F33039872AB",
            ),
        );
        assert_eq!(peer_public.algorithm(), alg);
        assert_eq!(peer_public.bytes(), &peer_public.bytes);

        let my_private = test::from_dirty_hex(
            "C88F01F510D9AC3F70A292DAA2316DE544E9AAB8AFE84049C62A9C57862D1433",
        );

        let my_private = {
            let mut rng = test::rand::FixedSliceRandom { bytes: &my_private };
            agreement::EphemeralPrivateKey::generate_for_test(alg, &mut rng).unwrap()
        };

        let my_public = test::from_dirty_hex(
            "04DAD0B65394221CF9B051E1FECA5787D098DFE637FC90B9EF945D0C37725811805271A0461CDB8252D61F1C456FA3E59AB1F45B33ACCF5F58389E0577B8990BB3",
        );
        let output = test::from_dirty_hex(
            "D6840F6B42F6EDAFD13116E0E12565202FEF8E9ECE7DCE03812464D04B9442DE",
        );

        assert_eq!(my_private.algorithm(), alg);

        let computed_public = my_private.compute_public_key().unwrap();
        assert_eq!(computed_public.as_ref(), &my_public[..]);

        assert_eq!(computed_public.algorithm(), alg);

        let result = agreement::agree_ephemeral(my_private, &peer_public, (), |key_material| {
            assert_eq!(key_material, &output[..]);
            Ok(())
        });
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_agreement_ecdh_p384() {
        let alg = &agreement::ECDH_P384;
        let peer_public = agreement::UnparsedPublicKey::new(
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
            agreement::EphemeralPrivateKey::generate_for_test(alg, &mut rng).unwrap()
        };

        let my_public = test::from_dirty_hex(
            "04667842D7D180AC2CDE6F74F37551F55755C7645C20EF73E31634FE72B4C55EE6DE3AC808ACB4BDB4C88732AEE95F41AA9482ED1FC0EEB9CAFC4984625CCFC23F65032149E0E144ADA024181535A0F38EEB9FCFF3C2C947DAE69B4C634573A81C",
        );
        let output = test::from_dirty_hex(
            "11187331C279962D93D604243FD592CB9D0A926F422E47187521287E7156C5C4D603135569B9E9D09CF5D4A270F59746",
        );

        assert_eq!(my_private.algorithm(), alg);

        let computed_public = my_private.compute_public_key().unwrap();
        assert_eq!(computed_public.as_ref(), &my_public[..]);

        assert_eq!(computed_public.algorithm(), alg);

        let result = agreement::agree_ephemeral(my_private, &peer_public, (), |key_material| {
            assert_eq!(key_material, &output[..]);
            Ok(())
        });
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_agreement_ecdh_p521() {
        let alg = &agreement::ECDH_P521;
        let peer_public = agreement::UnparsedPublicKey::new(
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
            agreement::EphemeralPrivateKey::generate_for_test(alg, &mut rng).unwrap()
        };

        let my_public = test::from_dirty_hex(
            "04004e8583bbbb2ecd93f0714c332dff5ab3bc6396e62f3c560229664329baa5138c3bb1c36428abd4e23d17fcb7a2cfcc224b2e734c8941f6f121722d7b6b9415457601cf0874f204b0363f020864672fadbf87c8811eb147758b254b74b14fae742159f0f671a018212bbf25b8519e126d4cad778cfff50d288fd39ceb0cac635b175ec0",
        );
        let output = test::from_dirty_hex(
            "01aaf24e5d47e4080c18c55ea35581cd8da30f1a079565045d2008d51b12d0abb4411cda7a0785b15d149ed301a3697062f42da237aa7f07e0af3fd00eb1800d9c41",
        );

        assert_eq!(my_private.algorithm(), alg);

        let computed_public = my_private.compute_public_key().unwrap();
        assert_eq!(computed_public.as_ref(), &my_public[..]);

        assert_eq!(computed_public.algorithm(), alg);

        let result = agreement::agree_ephemeral(my_private, &peer_public, (), |key_material| {
            assert_eq!(key_material, &output[..]);
            Ok(())
        });
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn agreement_traits() {
        use crate::test;

        let mut rng = rand::SystemRandom::new();

        let ephemeral_private_key =
            agreement::EphemeralPrivateKey::generate_for_test(&agreement::ECDH_P256, &mut rng)
                .unwrap();

        test::compile_time_assert_send::<agreement::EphemeralPrivateKey>();
        test::compile_time_assert_sync::<agreement::EphemeralPrivateKey>();

        assert_eq!(
            format!("{ephemeral_private_key:?}"),
            "EphemeralPrivateKey { algorithm: Algorithm { curve: P256 } }"
        );
    }

    fn check_computed_public_key(
        algorithm: &AlgorithmID,
        expected_format: &str,
        expected_public_key_bytes: &[u8],
        computed_public: &PublicKey,
    ) {
        match (algorithm, expected_format) {
            (_, "X509") => {
                let der = AsDer::<PublicKeyX509Der>::as_der(computed_public)
                    .expect("serialize to uncompressed format");
                assert_eq!(
                    expected_public_key_bytes,
                    der.as_ref(),
                    "hex: {:x?}",
                    der.as_ref()
                );
            }
            (
                AlgorithmID::ECDH_P256 | AlgorithmID::ECDH_P384 | AlgorithmID::ECDH_P521,
                "COMPRESSED",
            ) => {
                let bin = AsBigEndian::<EcPublicKeyCompressedBin>::as_be_bytes(computed_public)
                    .expect("serialize to compressed format");
                assert_eq!(expected_public_key_bytes, bin.as_ref());
            }
            (
                AlgorithmID::ECDH_P256 | AlgorithmID::ECDH_P384 | AlgorithmID::ECDH_P521,
                "UNCOMPRESSED" | "",
            ) => {
                let bin = AsBigEndian::<EcPublicKeyUncompressedBin>::as_be_bytes(computed_public)
                    .expect("serialize to uncompressed format");
                assert_eq!(expected_public_key_bytes, bin.as_ref());
                assert_eq!(expected_public_key_bytes, computed_public.as_ref());
            }
            (AlgorithmID::X25519, "") => {
                assert_eq!(expected_public_key_bytes, computed_public.as_ref());
            }
            (ai, pf) => {
                panic!("Unexpected PeerFormat={pf:?} for {ai:?}")
            }
        }
    }

    #[test]
    fn agreement_agree_ephemeral() {
        let rng = rand::SystemRandom::new();

        test::run(
            test_file!("data/agreement_tests.txt"),
            |section, test_case| {
                assert_eq!(section, "");

                let curve_name = test_case.consume_string("Curve");
                let alg = alg_from_curve_name(&curve_name);
                let peer_public =
                    agreement::UnparsedPublicKey::new(alg, test_case.consume_bytes("PeerQ"));

                let myq_format = test_case
                    .consume_optional_string("MyQFormat")
                    .unwrap_or_default();

                if test_case.consume_optional_string("Error").is_none() {
                    let my_private_bytes = test_case.consume_bytes("D");
                    let my_private = {
                        let mut rng = test::rand::FixedSliceRandom {
                            bytes: &my_private_bytes,
                        };
                        agreement::EphemeralPrivateKey::generate_for_test(alg, &mut rng)?
                    };
                    let my_public = test_case.consume_bytes("MyQ");
                    let output = test_case.consume_bytes("Output");

                    assert_eq!(my_private.algorithm(), alg);

                    let computed_public = my_private.compute_public_key().unwrap();

                    check_computed_public_key(&alg.id, &myq_format, &my_public, &computed_public);

                    assert_eq!(my_private.algorithm(), alg);

                    let result =
                        agreement::agree_ephemeral(my_private, &peer_public, (), |key_material| {
                            assert_eq!(key_material, &output[..]);
                            Ok(())
                        });
                    assert_eq!(
                        result,
                        Ok(()),
                        "Failed on private key: {:?}",
                        test::to_hex(my_private_bytes)
                    );
                } else {
                    fn kdf_not_called(_: &[u8]) -> Result<(), ()> {
                        panic!(
                            "The KDF was called during ECDH when the peer's \
                         public key is invalid."
                        );
                    }
                    let dummy_private_key = agreement::EphemeralPrivateKey::generate(alg, &rng)?;
                    assert!(agreement::agree_ephemeral(
                        dummy_private_key,
                        &peer_public,
                        (),
                        kdf_not_called
                    )
                    .is_err());
                }

                Ok(())
            },
        );
    }

    fn from_hex(s: &str) -> Vec<u8> {
        match test::from_hex(s) {
            Ok(v) => v,
            Err(msg) => {
                panic!("{msg} in {s}");
            }
        }
    }

    fn alg_from_curve_name(curve_name: &str) -> &'static agreement::Algorithm {
        if curve_name == "P-256" {
            &agreement::ECDH_P256
        } else if curve_name == "P-384" {
            &agreement::ECDH_P384
        } else if curve_name == "P-521" {
            &agreement::ECDH_P521
        } else if curve_name == "X25519" {
            &agreement::X25519
        } else {
            panic!("Unsupported curve: {curve_name}");
        }
    }

    fn x25519(private_key: &[u8], public_key: &[u8]) -> Vec<u8> {
        try_x25519(private_key, public_key).unwrap()
    }

    fn try_x25519(private_key: &[u8], public_key: &[u8]) -> Result<Vec<u8>, Unspecified> {
        let mut rng = test::rand::FixedSliceRandom { bytes: private_key };
        let private_key =
            agreement::EphemeralPrivateKey::generate_for_test(&agreement::X25519, &mut rng)?;
        let public_key = agreement::UnparsedPublicKey::new(&agreement::X25519, public_key);
        agreement::agree_ephemeral(private_key, public_key, Unspecified, |agreed_value| {
            Ok(Vec::from(agreed_value))
        })
    }
}
