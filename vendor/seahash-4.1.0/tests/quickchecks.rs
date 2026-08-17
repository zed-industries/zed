extern crate seahash;

#[macro_use]
extern crate quickcheck;
use quickcheck::TestResult;

use seahash::hash;
use seahash::reference::hash as reference;
use seahash::SeaHasher;
use std::hash::Hasher;
use std::num::{NonZeroU8, NonZeroUsize};

quickcheck! {
    #[cfg_attr(miri, ignore)] // very slow to run on miri
    fn chunked_matches_buffered(xs: Vec<u8>, chunk_size: NonZeroUsize, times: NonZeroU8, additional: u8) -> TestResult {
        let target_size = xs.len() * times.get() as usize + additional as usize;
        if xs.is_empty() || target_size > 10_000_000 {
            TestResult::discard()
        } else {
            let xs = xs.into_iter()
                .cycle()
                // the vecs produced by quickcheck are perhaps a bit small by default.
                // additional should add some noise to avoid only getting nice even lengths.
                .take(target_size)
                .collect::<Vec<_>>();

            // write all at once
            let mut h0 = SeaHasher::default();
            h0.write(&xs);
            let h0 = h0.finish();

            // write in chunks
            let mut h1 = SeaHasher::default();
            for chunk in xs.chunks(chunk_size.get()) {
                h1.write(chunk);
            }
            let h1 = h1.finish();

            // compare all, including to buffered and reference
            let outcome = h0 == h1
                && h0 == hash(&xs)
                && h0 == reference(&xs);

            TestResult::from_bool(outcome)
        }
    }
}
