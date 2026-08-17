use super::*;

pub enum EVP_PKEY {}

cfg_if! {
    if #[cfg(ossl300)] {
        pub enum EVP_SIGNATURE {}
    }
}
