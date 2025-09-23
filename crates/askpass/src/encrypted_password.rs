//! This module provides [EncryptedPassword] for storage of passwords in memory.
//! On Windows that's implemented with CryptProtectMemory/CryptUnprotectMemory; on other platforms it just falls through
//! to string for now.
//!
//! The "safety" of this module lies in exploiting visibility rules of Rust:
//! 1. No outside module has access to the internal representation of [EncryptedPassword].
//! 2. [EncryptedPassword] cannot be converted into a [String] or any other plaintext representation.
//! All use cases that do need such functionality (of which we have two right now) are implemented within this module.
//!
//! Note that this is not bulletproof.
//! 1. [ProcessExt] is implemented for [smol::process::Command], which is a builder for smol processes.
//! Before the process itself is spawned the contents of [EncryptedPassword] are unencrypted in env var storage of said builder.
//! 2. We're also sending plaintext passwords over RPC with [proto::AskPassResponse]. Go figure how great that is.
//!
//! Still, the goal of this module is to not have passwords laying around nilly-willy in memory.
//! We do not claim that it is fool-proof.
use anyhow::Result;
use zeroize::Zeroize;

type LengthWithoutPadding = u32;
#[derive(Clone)]
pub struct EncryptedPassword(Vec<u8>, LengthWithoutPadding);

pub trait ProcessExt {
    fn encrypted_env(&mut self, name: &str, value: EncryptedPassword) -> &mut Self;
}

impl ProcessExt for smol::process::Command {
    fn encrypted_env(&mut self, name: &str, value: EncryptedPassword) -> &mut Self {
        if let Ok(password) = decrypt(value) {
            self.env(name, password);
        }
        self
    }
}

impl TryFrom<EncryptedPassword> for proto::AskPassResponse {
    type Error = anyhow::Error;
    fn try_from(pw: EncryptedPassword) -> Result<Self, Self::Error> {
        let pw = decrypt(pw)?;
        Ok(Self { response: pw })
    }
}

impl Drop for EncryptedPassword {
    fn drop(&mut self) {
        self.0.zeroize();
        self.1.zeroize();
    }
}

impl TryFrom<&str> for EncryptedPassword {
    type Error = anyhow::Error;
    fn try_from(password: &str) -> Result<EncryptedPassword> {
        let len: u32 = password.len().try_into()?;
        #[cfg(windows)]
        {
            use windows::Win32::Security::Cryptography::{
                CRYPTPROTECTMEMORY_BLOCK_SIZE, CRYPTPROTECTMEMORY_SAME_PROCESS, CryptProtectMemory,
            };
            let mut value = password.bytes().collect::<Vec<_>>();
            let padded_length = len.next_multiple_of(CRYPTPROTECTMEMORY_BLOCK_SIZE);
            if padded_length != len {
                value.resize(padded_length as usize, 0);
            }
            unsafe {
                CryptProtectMemory(
                    value.as_mut_ptr() as _,
                    len,
                    CRYPTPROTECTMEMORY_SAME_PROCESS,
                )?;
            }
            Ok(Self(value, len))
        }
        #[cfg(not(windows))]
        Ok(Self(String::from(password).into(), len))
    }
}

pub(crate) fn decrypt(mut password: EncryptedPassword) -> Result<String> {
    #[cfg(windows)]
    {
        use anyhow::Context;
        use windows::Win32::Security::Cryptography::{
            CRYPTPROTECTMEMORY_BLOCK_SIZE, CRYPTPROTECTMEMORY_SAME_PROCESS, CryptUnprotectMemory,
        };
        assert_eq!(
            password.0.len() % CRYPTPROTECTMEMORY_BLOCK_SIZE as usize,
            0,
            "Violated pre-condition (buffer size <{}> must be a multiple of CRYPTPROTECTMEMORY_BLOCK_SIZE <{}>) for CryptUnprotectMemory.",
            password.0.len(),
            CRYPTPROTECTMEMORY_BLOCK_SIZE
        );
        unsafe {
            CryptUnprotectMemory(
                password.0.as_mut_ptr() as _,
                password.1,
                CRYPTPROTECTMEMORY_SAME_PROCESS,
            )
            .context("while decrypting a SSH password")?
        };

        {
            // Remove padding
            _ = password.0.drain(password.1 as usize..);
        }
        Ok(String::from_utf8(std::mem::take(&mut password.0))?)
    }
    #[cfg(not(windows))]
    Ok(String::from_utf8(std::mem::take(&mut password.0))?)
}
