use std::collections::BTreeMap;

use anyhow::{Context, Result};
use zeroize::Zeroize;

type LengthWithoutPadding = u32;
#[derive(Clone)]
pub struct EncryptedPassword(Vec<u8>, LengthWithoutPadding);

impl TryFrom<EncryptedPassword> for String {
    type Error = anyhow::Error;
    fn try_from(this: EncryptedPassword) -> Result<String> {
        decrypt_password(this)
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
        if cfg!(windows) {
            use windows::Win32::Security::Cryptography::{
                CRYPTPROTECTMEMORY_BLOCK_SIZE, CRYPTPROTECTMEMORY_SAME_PROCESS, CryptProtectMemory,
            };
            let mut value = password.bytes().collect::<Vec<_>>();
            let trailing_bytes = len % CRYPTPROTECTMEMORY_BLOCK_SIZE;
            if trailing_bytes != 0 {
                let required_padding = (len - trailing_bytes) as usize;
                value.resize(value.len() + required_padding, 0);
            }
            unsafe {
                CryptProtectMemory(
                    value.as_mut_ptr() as _,
                    len,
                    CRYPTPROTECTMEMORY_SAME_PROCESS,
                )?;
            }
            Ok(Self(value, len))
        } else {
            Ok(Self(String::from(password).into(), len))
        }
    }
}

pub(crate) fn decrypt_password(mut password: EncryptedPassword) -> Result<String> {
    if cfg!(windows) {
        use windows::Win32::Security::Cryptography::{
            CRYPTPROTECTMEMORY_BLOCK_SIZE, CRYPTPROTECTMEMORY_SAME_PROCESS, CryptUnprotectMemory,
        };
        assert_eq!(
            password.0.len() % CRYPTPROTECTMEMORY_BLOCK_SIZE as usize,
            0,
            "Violated pre-condition (buffer size being a multiple of CRYPTPROTECTMEMORY_BLOCK_SIZE) for CryptUnprotectMemory"
        );
        unsafe {
            CryptUnprotectMemory(
                password.0.as_mut_ptr() as _,
                password.1,
                CRYPTPROTECTMEMORY_SAME_PROCESS,
            )
            .context("while decrypting a SSH password")?
        };

        // Remove padding
        _ = password.0.drain(password.1 as usize..);

        Ok(String::from_utf8(std::mem::take(&mut password.0))?)
    } else {
        Ok(String::from_utf8(std::mem::take(&mut password.0))?)
    }
}

#[derive(Default)]
pub struct PassStore(BTreeMap<String, EncryptedPassword>);

impl PassStore {
    pub fn cached_password(&self, url: &str) -> Option<EncryptedPassword> {
        self.0.get(url).cloned()
    }

    pub fn cache_password(&mut self, url: String, pw: EncryptedPassword) -> Result<()> {
        // This function is currently a no-op on non Windows platforms, as we use a MasterPass on Linux and Mac with ssh.
        if cfg!(windows) {
            self.0.insert(url, pw);
        }
        Ok(())
    }

    pub fn prune_password(&mut self, url: &str) {
        self.0.remove(url);
    }
}
