use cap_rand::Rng;
use cap_rand::distributions::Standard;

use crate::p3::bindings::random::{insecure, insecure_seed, random};
use crate::random::WasiRandomCtx;

impl random::Host for WasiRandomCtx {
    fn get_random_bytes(&mut self, len: u64) -> wasmtime::Result<Vec<u8>> {
        Ok((&mut self.random)
            .sample_iter(Standard)
            .take(len as usize)
            .collect())
    }

    fn get_random_u64(&mut self) -> wasmtime::Result<u64> {
        Ok(self.random.sample(Standard))
    }
}

impl insecure::Host for WasiRandomCtx {
    fn get_insecure_random_bytes(&mut self, len: u64) -> wasmtime::Result<Vec<u8>> {
        Ok((&mut self.insecure_random)
            .sample_iter(Standard)
            .take(len as usize)
            .collect())
    }

    fn get_insecure_random_u64(&mut self) -> wasmtime::Result<u64> {
        Ok(self.insecure_random.sample(Standard))
    }
}

impl insecure_seed::Host for WasiRandomCtx {
    fn insecure_seed(&mut self) -> wasmtime::Result<(u64, u64)> {
        let seed: u128 = self.insecure_random_seed;
        Ok((seed as u64, (seed >> 64) as u64))
    }
}
