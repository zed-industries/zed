use rand_core::{CryptoRng, RngCore};

/// This is a dummy RNG for cases when we need a concrete RNG type
/// which does not get used.
#[derive(Copy, Clone)]
pub(crate) struct DummyRng;

impl RngCore for DummyRng {
    fn next_u32(&mut self) -> u32 {
        unimplemented!();
    }

    fn next_u64(&mut self) -> u64 {
        unimplemented!();
    }

    fn fill_bytes(&mut self, _: &mut [u8]) {
        unimplemented!();
    }

    fn try_fill_bytes(&mut self, _: &mut [u8]) -> core::result::Result<(), rand_core::Error> {
        unimplemented!();
    }
}

impl CryptoRng for DummyRng {}
