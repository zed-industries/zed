// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

//! cipher - Perform symmetric cipher encryption/decryption on utf8 plaintext.
//!
//! *cipher* is an example program demonstrating the `aws_lc_rs::cipher` API for *aws-lc-rs*.
//! It demonstrates CTR & CBC mode encryption using AES 128 or 256 bit keys.
//!
//! The program can be run from the command line using cargo:
//! ```sh
//! $ cargo run --example cipher -- encrypt --mode ctr  "Hello World"
//! key: b331133eb742497c67ced9520c9a7de3
//! iv: 4e967c7b799e0670431888e2e959e154
//! ciphertext: 88bcbd8d1656d60de739c5
//!
//! $ cargo run --example cipher -- decrypt --mode ctr --key b331133eb742497c67ced9520c9a7de3 --iv 4e967c7b799e0670431888e2e959e154 88bcbd8d1656d60de739c5
//! Hello World
//!
//! $ cargo run --example cipher -- encrypt --mode cbc "Hello World"
//! key: 6489d8ce0c4facf18b872705a05d5ee4
//! iv: 5cd56fb752830ec2459889226c5431bd
//! ciphertext: 6311c14e8104730be124ce1e57e51fe3
//!
//! $ cargo run --example cipher -- decrypt --mode cbc --key 6489d8ce0c4facf18b872705a05d5ee4 --iv 5cd56fb752830ec2459889226c5431bd 6311c14e8104730be124ce1e57e51fe3
//! Hello World
//! ```
use aws_lc_rs::cipher::{
    DecryptingKey, DecryptionContext, EncryptingKey, EncryptionContext, PaddedBlockDecryptingKey,
    PaddedBlockEncryptingKey, UnboundCipherKey, AES_128, AES_128_KEY_LEN, AES_192, AES_192_KEY_LEN,
    AES_256, AES_256_KEY_LEN, AES_CBC_IV_LEN, AES_CTR_IV_LEN,
};
use aws_lc_rs::iv::FixedLength;
use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(author, version, name = "cipher")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(ValueEnum, Clone, Copy)]
enum Mode {
    Ctr,
    Cbc,
}

#[derive(Subcommand)]
enum Commands {
    Encrypt {
        #[arg(short, long, help = "Initalization Vector (IV) in hex")]
        iv: Option<String>,

        #[arg(
            short,
            long,
            help = "AES 128 or 256 bit key in hex, if not provided defaults to 128"
        )]
        key: Option<String>,

        #[arg(short, long, value_enum, help = "AES cipher mode")]
        mode: Mode,

        plaintext: String,
    },
    Decrypt {
        #[arg(short, long, help = "Initalization Vector (IV) in hex")]
        iv: String,

        #[arg(short, long, help = "AES 128 or 256 bit key in hex")]
        key: String,

        #[arg(short, long, value_enum, help = "AES cipher mode")]
        mode: Mode,

        ciphertext: String,
    },
}

fn main() -> Result<(), &'static str> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encrypt {
            iv,
            key,
            mode,
            plaintext,
        } => {
            if matches!(mode, Mode::Ctr) {
                aes_ctr_encrypt(key, iv, plaintext)
            } else {
                aes_cbc_encrypt(key, iv, plaintext)
            }
        }
        Commands::Decrypt {
            iv,
            key,
            mode,
            ciphertext,
        } => {
            if matches!(mode, Mode::Ctr) {
                aes_ctr_decrypt(key, iv, ciphertext)
            } else {
                aes_cbc_decrypt(key, iv, ciphertext)
            }
        }
    }?;

    Ok(())
}

fn construct_key_bytes(key: Option<String>) -> Result<Vec<u8>, &'static str> {
    if let Some(key) = key {
        match hex::decode(key) {
            Ok(v) => Ok(v),
            Err(..) => Err("invalid key"),
        }
    } else {
        let mut v = vec![0u8; AES_128_KEY_LEN];
        aws_lc_rs::rand::fill(v.as_mut_slice()).map_err(|_| "failed to generate key")?;
        Ok(v)
    }
}

fn aes_ctr_encrypt(
    key: Option<String>,
    iv: Option<String>,
    plaintext: String,
) -> Result<(), &'static str> {
    let key_bytes = construct_key_bytes(key)?;
    let hex_key = hex::encode(key_bytes.as_slice());
    let key = new_unbound_key(key_bytes.as_slice())?;

    let key = EncryptingKey::ctr(key).map_err(|_| "failed to initalized aes encryption")?;

    let mut ciphertext = Vec::from(plaintext);

    let context = match iv {
        Some(iv) => {
            let context = {
                let v = hex::decode(iv).map_err(|_| "invalid iv")?;
                let v: FixedLength<AES_CTR_IV_LEN> =
                    v.as_slice().try_into().map_err(|_| "invalid iv")?;
                EncryptionContext::Iv128(v)
            };
            key.less_safe_encrypt(ciphertext.as_mut(), context)
        }
        None => key.encrypt(ciphertext.as_mut()),
    }
    .map_err(|_| "failed to encrypt plaintext")?;

    let iv: &[u8] = (&context)
        .try_into()
        .map_err(|_| "unexpected encryption context")?;

    let ciphertext = hex::encode(ciphertext.as_slice());

    println!("key: {hex_key}");
    println!("iv: {}", hex::encode(iv));
    println!("ciphertext: {ciphertext}");

    Ok(())
}

fn aes_ctr_decrypt(key: String, iv: String, ciphertext: String) -> Result<(), &'static str> {
    let key_bytes = construct_key_bytes(Some(key))?;
    let key = new_unbound_key(key_bytes.as_slice())?;
    let iv = {
        let v = hex::decode(iv).map_err(|_| "invalid iv")?;
        let v: FixedLength<AES_CTR_IV_LEN> = v.as_slice().try_into().map_err(|_| "invalid iv")?;
        v
    };

    let key = DecryptingKey::ctr(key).map_err(|_| "failed to initalized aes decryption")?;

    let mut ciphertext =
        hex::decode(ciphertext).map_err(|_| "ciphertext is not valid hex encoding")?;

    let plaintext = key
        .decrypt(ciphertext.as_mut(), DecryptionContext::Iv128(iv))
        .map_err(|_| "failed to decrypt ciphertext")?;

    let plaintext =
        String::from_utf8(plaintext.into()).map_err(|_| "decrypted text was not a utf8 string")?;

    println!("{plaintext}");

    Ok(())
}

fn aes_cbc_encrypt(
    key: Option<String>,
    iv: Option<String>,
    plaintext: String,
) -> Result<(), &'static str> {
    let key_bytes = construct_key_bytes(key)?;
    let hex_key = hex::encode(key_bytes.as_slice());
    let key = new_unbound_key(key_bytes.as_slice())?;

    let key = PaddedBlockEncryptingKey::cbc_pkcs7(key)
        .map_err(|_| "failed to initalized aes encryption")?;

    let mut ciphertext = Vec::from(plaintext);

    let context = match iv {
        Some(iv) => {
            let context = {
                let v = hex::decode(iv).map_err(|_| "invalid iv")?;
                let v: FixedLength<AES_CBC_IV_LEN> =
                    v.as_slice().try_into().map_err(|_| "invalid iv")?;
                EncryptionContext::Iv128(v)
            };
            key.less_safe_encrypt(&mut ciphertext, context)
        }
        None => key.encrypt(&mut ciphertext),
    }
    .map_err(|_| "failed to initalized aes encryption")?;

    let iv: &[u8] = (&context)
        .try_into()
        .map_err(|_| "unexpected encryption context")?;

    let ciphertext = hex::encode(ciphertext.as_slice());

    println!("key: {hex_key}");
    println!("iv: {}", hex::encode(iv));
    println!("ciphertext: {ciphertext}");

    Ok(())
}

fn aes_cbc_decrypt(key: String, iv: String, ciphertext: String) -> Result<(), &'static str> {
    let key_bytes = construct_key_bytes(Some(key))?;
    let key = new_unbound_key(key_bytes.as_slice())?;
    let iv = {
        let v = hex::decode(iv).map_err(|_| "invalid iv")?;
        let v: FixedLength<AES_CBC_IV_LEN> = v.as_slice().try_into().map_err(|_| "invalid iv")?;
        v
    };

    let key = PaddedBlockDecryptingKey::cbc_pkcs7(key)
        .map_err(|_| "failed to initalized aes decryption")?;

    let mut ciphertext =
        hex::decode(ciphertext).map_err(|_| "ciphertext is not valid hex encoding")?;

    let plaintext = key
        .decrypt(ciphertext.as_mut(), DecryptionContext::Iv128(iv))
        .map_err(|_| "failed to decrypt ciphertext")?;

    let plaintext =
        String::from_utf8(plaintext.into()).map_err(|_| "decrypted text was not a utf8 string")?;

    println!("{plaintext}");

    Ok(())
}

fn new_unbound_key(key: &[u8]) -> Result<UnboundCipherKey, &'static str> {
    let alg = match key.len() {
        AES_128_KEY_LEN => &AES_128,
        AES_192_KEY_LEN => &AES_192,
        AES_256_KEY_LEN => &AES_256,
        _ => {
            return Err("invalid aes key length");
        }
    };

    UnboundCipherKey::new(alg, key).map_err(|_| "failed to construct aes key")
}
