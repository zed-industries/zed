// Copyright 2015-2016 Brian Smith.
// SPDX-License-Identifier: ISC
// Modifications copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

//! RSA Signature and Encryption Support.
//!
//! # OAEP Encryption / Decryption
//!
//! ```rust
//! # use std::error::Error;
//! # fn main() -> Result<(), Box<dyn Error>> {
//! use aws_lc_rs::{
//!     encoding::{AsDer, Pkcs8V1Der, PublicKeyX509Der},
//!     rsa::{KeySize, OAEP_SHA256_MGF1SHA256, OaepPublicEncryptingKey, OaepPrivateDecryptingKey, PublicEncryptingKey, PrivateDecryptingKey}
//! };
//!
//! // Generate a RSA 2048-bit key.
//! let private_key = PrivateDecryptingKey::generate(KeySize::Rsa2048)?;
//!
//! // Serialize the RSA private key to DER encoded PKCS#8 format for later usage.
//! let private_key_der = AsDer::<Pkcs8V1Der>::as_der(&private_key)?;
//! let private_key_der_bytes = private_key_der.as_ref();
//!
//! // Load a RSA private key from DER encoded PKCS#8 document.
//! let private_key = PrivateDecryptingKey::from_pkcs8(private_key_der_bytes)?;
//!
//! // Retrieve the RSA public key
//! let public_key = private_key.public_key();
//!
//! // Serialize the RSA public key to DER encoded X.509 SubjectPublicKeyInfo for later usage.
//! let public_key_der = AsDer::<PublicKeyX509Der>::as_der(&public_key)?;
//! let public_key_der_bytes = public_key_der.as_ref();
//!
//! // Load a RSA public key from DER encoded X.509 SubjectPublicKeyInfo.
//! let public_key = PublicEncryptingKey::from_der(public_key_der_bytes)?;
//!
//! // Construct a RSA-OAEP public encrypting key
//! let public_key = OaepPublicEncryptingKey::new(public_key)?;
//!
//! // The maximum size plaintext can be determined by calling `OaepPublicEncryptingKey::max_plaintext_size`
//! let message = b"hello world";
//! let mut ciphertext = vec![0u8; public_key.ciphertext_size()]; // Output will be the size of the RSA key length in bytes rounded up.
//!
//! // Encrypt a message with the public key without the optional label provided.
//! let ciphertext = public_key.encrypt(&OAEP_SHA256_MGF1SHA256, message, &mut ciphertext, None)?;
//!
//! assert_ne!(message, ciphertext);
//!
//! // Construct a RSA-OAEP private decrypting key
//! let private_key = OaepPrivateDecryptingKey::new(private_key)?;
//!
//! // Decrypt a message with the private key.
//! let mut plaintext = vec![0u8; private_key.min_output_size()];
//! let plaintext = private_key.decrypt(&OAEP_SHA256_MGF1SHA256, ciphertext, &mut plaintext, None)?;
//!
//! assert_eq!(message, plaintext);
//!
//! # Ok(())
//! # }
//! ```

// *R* and *r* in Montgomery math refer to different things, so we always use
// `R` to refer to *R* to avoid confusion, even when that's against the normal
// naming conventions. Also the standard camelCase names are used for `KeyPair`
// components.

mod encoding;
mod encryption;
pub(crate) mod key;
pub(crate) mod signature;

pub use self::encryption::oaep::{
    OaepAlgorithm, OaepPrivateDecryptingKey, OaepPublicEncryptingKey, OAEP_SHA1_MGF1SHA1,
    OAEP_SHA256_MGF1SHA256, OAEP_SHA384_MGF1SHA384, OAEP_SHA512_MGF1SHA512,
};
pub use self::encryption::pkcs1::{Pkcs1PrivateDecryptingKey, Pkcs1PublicEncryptingKey};
pub use self::encryption::{EncryptionAlgorithmId, PrivateDecryptingKey, PublicEncryptingKey};
pub use self::key::{KeyPair, KeySize, PublicKey, PublicKeyComponents};
#[allow(clippy::module_name_repetitions)]
pub use self::signature::RsaParameters;

pub(crate) use self::signature::RsaVerificationAlgorithmId;

#[cfg(test)]
mod tests {
    #[cfg(feature = "fips")]
    mod fips;

    #[cfg(feature = "ring-io")]
    #[test]
    fn test_rsa() {
        use crate::signature::KeyPair;
        use crate::test::from_dirty_hex;
        let rsa_pkcs8_input: Vec<u8> = from_dirty_hex(
            r"308204bd020100300d06092a864886f70d0101010500048204a7308204a30201000282010100b9d7a
        f84fa4184a5f22037ec8aff2db5f78bd8c21e714e579ae57c6398c4950f3a694b17bfccf488766159aec5bb7c2c4
        3d59c798cbd45a09c9c86933f126879ee7eadcd404f61ecfc425197cab03946ba381a49ef3b4d0f60b17f8a747cd
        e56a834a7f6008f35ffb2f60a54ceda1974ff2a9963aba7f80d4e2916a93d8c74bb1ba5f3b189a4e8f0377bd3e94
        b5cc3f9c53cb8c8c7c0af394818755e968b7a76d9cada8da7af5fbe25da2a09737d5e4e4d7092aa16a0718d7322c
        e8aca767015128d6d35775ea9cb8bb1ac6512e1b787d34015221be780a37b1d69bc3708bfd8832591be6095a768f
        0fd3b3457927e6ae3641d55799a29a0a269cb4a693bc14b0203010001028201001c5fb7e69fa6dd2fd0f5e653f12
        ce0b7c5a1ce6864e97bc2985dad4e2f86e4133d21d25b3fe774f658cca83aace9e11d8905d62c20b6cd28a680a77
        357cfe1afac201f3d1532898afb40cce0560bedd2c49fc833bd98da3d1cd03cded0c637d4173e62de865b572d410
        f9ba83324cd7a3573359428232f1628f6d104e9e6c5f380898b5570201cf11eb5f7e0c4933139c7e7fba67582287
        ffb81b84fa81e9a2d9739815a25790c06ead7abcf286bd43c6e3d009d01f15fca3d720bbea48b0c8ccf8764f3c82
        2e61159d8efcbff38c794f8afe040b45df14c976a91b1b6d886a55b8e68969bcb30c7197920d97d7721d78d954d8
        9ffecbcc93c6ee82a86fe754102818100eba1cbe453f5cb2fb7eabc12d697267d25785a8f7b43cc2cb14555d3618
        c63929b19839dcd4212397ecda8ad872f97ede6ac95ebda7322bbc9409bac2b24ae56ad62202800c670365ae2867
        1195fe934978a5987bee2fcea06561b782630b066b0a35c3f559a281f0f729fc282ef8ebdbb065d60000223da6ed
        b732fa32d82bb02818100c9e81e353315fd88eff53763ed7b3859f419a0a158f5155851ce0fe6e43188e44fb43dd
        25bcdb7f3839fe84a5db88c6525e5bcbae513bae5ff54398106bd8ae4d241c082f8a64a9089531f7b57b09af5204
        2efa097140702dda55a2141c174dd7a324761267728a6cc4ce386c034393d855ebe985c4e5f2aec2bd3f2e2123ab
        1028180566889dd9c50798771397a68aa1ad9b970e136cc811676ac3901c51c741c48737dbf187de8c47eec68acc
        05b8a4490c164230c0366a36c2c52fc075a56a3e7eecf3c39b091c0336c2b5e00913f0de5f62c5046ceb9d88188c
        c740d34bd44839bd4d0c346527cea93a15596727d139e53c35eed25043bc4ac18950f237c02777b0281800f9dd98
        049e44088efee6a8b5b19f5c0d765880c12c25a154bb6817a5d5a0b798544aea76f9c58c707fe3d4c4b3573fe7ad
        0eb291580d22ae9f5ccc0d311a40590d1af1f3236427c2d72f57367d3ec185b9771cb5d041a8ab93409e59a9d68f
        99c72f91c658a3fe5aed59f9f938c368530a4a45f4a7c7155f3906c4354030ef102818100c89e0ba805c970abd84
        a70770d8fc57bfaa34748a58b77fcddaf0ca285db91953ef5728c1be7470da5540df6af56bb04c0f5ec500f83b08
        057664cb1551e1e29c58d8b1e9d70e23ed57fdf9936c591a83c1dc954f6654d4a245b6d8676d045c2089ffce537d
        234fc88e98d92afa92926c75b286e8fee70e273d762bbe63cd63b",
        );

        let key = super::KeyPair::from_pkcs8(&rsa_pkcs8_input).unwrap();
        let pk = key.public_key();
        let modulus_bytes = pk.modulus().big_endian_without_leading_zero();
        assert_eq!(&rsa_pkcs8_input[38..294], modulus_bytes);
    }

    #[test]
    fn test_debug() {
        use crate::signature;
        assert_eq!(
            "{ RSA_PSS_SHA512 }",
            format!("{:?}", signature::RSA_PSS_SHA512)
        );

        assert_eq!(
            "{ RSA_PSS_2048_8192_SHA256 }",
            format!("{:?}", signature::RSA_PSS_2048_8192_SHA256)
        );
    }
}
