//! Property-based tests.

use proptest::prelude::*;
use rand_chacha::ChaCha8Rng;
use rand_core::SeedableRng;
use rsa::{
    pkcs1v15,
    signature::{Keypair, SignatureEncoding, Signer, Verifier},
    RsaPrivateKey,
};
use sha2::Sha256;

prop_compose! {
    // WARNING: do *NOT* copy and paste this code. It's insecure and optimized for test speed.
    fn private_key()(seed in any::<[u8; 32]>()) -> RsaPrivateKey {
        let mut rng = ChaCha8Rng::from_seed(seed);
        RsaPrivateKey::new(&mut rng, 512).unwrap()
    }
}

proptest! {
    #[test]
    fn pkcs1v15_sign_roundtrip(private_key in private_key(), msg in any::<Vec<u8>>()) {
        let signing_key = pkcs1v15::SigningKey::<Sha256>::new(private_key);
        let signature_bytes = signing_key.sign(&msg).to_bytes();

        let verifying_key = signing_key.verifying_key();
        let signature = pkcs1v15::Signature::try_from(&*signature_bytes).unwrap();
        prop_assert!(verifying_key.verify(&msg, &signature).is_ok());
    }

    // TODO(tarcieri): debug why these are failing
    // #[test]
    // fn pss_sign_roundtrip(private_key in private_key(), msg in any::<Vec<u8>>()) {
    //     let signing_key = pss::SigningKey::<Sha256>::new(private_key);
    //     let signature_bytes = signing_key.sign(&msg).to_bytes();
    //
    //     let verifying_key = signing_key.verifying_key();
    //     let signature = pss::Signature::try_from(&*signature_bytes).unwrap();
    //     prop_assert!(verifying_key.verify(&msg, &signature).is_ok());
    // }
}
