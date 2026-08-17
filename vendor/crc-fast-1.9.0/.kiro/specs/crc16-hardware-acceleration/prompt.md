I would like to add hardware accelerated CRC-16 calculations to this Rust library which already has working hardware accelerated CRC-32 and CRC-64 calculations, as well as a working software fallback.

There are reference implementations written in MASM in the `/reference` folder which should be useful when comparing the current algorithm implementation in Rust.

I have also stubbed out known good values in `/crc16` already for both forward and reverse CRC-16 variants, which match the values in `/reference/crc16f` (forward) and `/reference/crc16r` (reverse) exactly. Those values are missing the wide 256-byte folding keys, though, which we must generate, since the reference doesn't include them.

Step one is definitely to update `generate.rs` to properly generate all the CRC-16 folding keys, including the missing 256-byte keys for use with AVX512. The CRC-32 and CRC-64 logic for key generating is working, so we should update it to also generate CRC-16 keys, and test against the known good values in `/reference` and `/crc16`.

Once we're sure key generation is correct, including test coverage, we can begin working on the algorithm changes to support CRC-16 calculation as well. By comparing our existing, working Rust implementations of CRC-32 and CRC-16, plus the working MASM examples for CRC-16, CRC-32, and CRC-64 in the `/reference` folder, we should be able to update our Rust algorithm to also compute CRC-16.

It will be important to pay special attention to not just the algorithm, but any other constants that might differ from CRC-16, CRC-32, and CRC-64, such as the shuffling constants. 

We can use the industry standard "123456789" input string to test CRC-16, using the known good check values already established in `/crc16`, as the basis for our tests to ensure we've correctly calculated the CRC-16 values. Do not test any other input strings until the industry standard is to be correctly working.

It's very important that we do not break any of the existing functionality around CRC-32 and CRC-64 calculation while adding CRC-16 support, and that we keep the API totally backwards compatible.

It's also important that, while we focus on the two reference CRC-16 variants, we are able to support all CRC-16 variants, including ones using custom polynomials, just like we do for CRC-32 and CRC-64.