//! Generate prime components for the RSA Private Key

use alloc::vec::Vec;
use num_bigint::{BigUint, RandPrime};
#[allow(unused_imports)]
use num_traits::Float;
use num_traits::Zero;
use rand_core::CryptoRngCore;

use crate::{
    algorithms::rsa::{compute_modulus, compute_private_exponent_euler_totient},
    errors::{Error, Result},
};

pub struct RsaPrivateKeyComponents {
    pub n: BigUint,
    pub e: BigUint,
    pub d: BigUint,
    pub primes: Vec<BigUint>,
}

/// Generates a multi-prime RSA keypair of the given bit size, public exponent,
/// and the given random source, as suggested in [1]. Although the public
/// keys are compatible (actually, indistinguishable) from the 2-prime case,
/// the private keys are not. Thus it may not be possible to export multi-prime
/// private keys in certain formats or to subsequently import them into other
/// code.
///
/// Table 1 in [2] suggests maximum numbers of primes for a given size.
///
/// [1]: https://patents.google.com/patent/US4405829A/en
/// [2]: http://www.cacr.math.uwaterloo.ca/techreports/2006/cacr2006-16.pdf
pub(crate) fn generate_multi_prime_key_with_exp<R: CryptoRngCore + ?Sized>(
    rng: &mut R,
    nprimes: usize,
    bit_size: usize,
    exp: &BigUint,
) -> Result<RsaPrivateKeyComponents> {
    if nprimes < 2 {
        return Err(Error::NprimesTooSmall);
    }

    if bit_size < 64 {
        let prime_limit = (1u64 << (bit_size / nprimes) as u64) as f64;

        // pi aproximates the number of primes less than prime_limit
        let mut pi = prime_limit / (prime_limit.ln() - 1f64);
        // Generated primes start with 0b11, so we can only use a quarter of them.
        pi /= 4f64;
        // Use a factor of two to ensure that key generation terminates in a
        // reasonable amount of time.
        pi /= 2f64;

        if pi < nprimes as f64 {
            return Err(Error::TooFewPrimes);
        }
    }

    let mut primes = vec![BigUint::zero(); nprimes];
    let n_final: BigUint;
    let d_final: BigUint;

    'next: loop {
        let mut todo = bit_size;
        // `gen_prime` should set the top two bits in each prime.
        // Thus each prime has the form
        //   p_i = 2^bitlen(p_i) × 0.11... (in base 2).
        // And the product is:
        //   P = 2^todo × α
        // where α is the product of nprimes numbers of the form 0.11...
        //
        // If α < 1/2 (which can happen for nprimes > 2), we need to
        // shift todo to compensate for lost bits: the mean value of 0.11...
        // is 7/8, so todo + shift - nprimes * log2(7/8) ~= bits - 1/2
        // will give good results.
        if nprimes >= 7 {
            todo += (nprimes - 2) / 5;
        }

        for (i, prime) in primes.iter_mut().enumerate() {
            *prime = rng.gen_prime(todo / (nprimes - i));
            todo -= prime.bits();
        }

        // Makes sure that primes is pairwise unequal.
        for (i, prime1) in primes.iter().enumerate() {
            for prime2 in primes.iter().take(i) {
                if prime1 == prime2 {
                    continue 'next;
                }
            }
        }

        let n = compute_modulus(&primes);

        if n.bits() != bit_size {
            // This should never happen for nprimes == 2 because
            // gen_prime should set the top two bits in each prime.
            // For nprimes > 2 we hope it does not happen often.
            continue 'next;
        }

        if let Ok(d) = compute_private_exponent_euler_totient(&primes, exp) {
            n_final = n;
            d_final = d;
            break;
        }
    }

    Ok(RsaPrivateKeyComponents {
        n: n_final,
        e: exp.clone(),
        d: d_final,
        primes,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigUint;
    use num_traits::FromPrimitive;
    use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};

    const EXP: u64 = 65537;

    #[test]
    fn test_impossible_keys() {
        let mut rng = ChaCha8Rng::from_seed([42; 32]);
        let exp = BigUint::from_u64(EXP).expect("invalid static exponent");
        for i in 0..32 {
            let _ = generate_multi_prime_key_with_exp(&mut rng, 2, i, &exp);
            let _ = generate_multi_prime_key_with_exp(&mut rng, 3, i, &exp);
            let _ = generate_multi_prime_key_with_exp(&mut rng, 4, i, &exp);
            let _ = generate_multi_prime_key_with_exp(&mut rng, 5, i, &exp);
        }
    }

    macro_rules! key_generation {
        ($name:ident, $multi:expr, $size:expr) => {
            #[test]
            fn $name() {
                let mut rng = ChaCha8Rng::from_seed([42; 32]);
                let exp = BigUint::from_u64(EXP).expect("invalid static exponent");

                for _ in 0..10 {
                    let components =
                        generate_multi_prime_key_with_exp(&mut rng, $multi, $size, &exp).unwrap();
                    assert_eq!(components.n.bits(), $size);
                    assert_eq!(components.primes.len(), $multi);
                }
            }
        };
    }

    key_generation!(key_generation_128, 2, 128);
    key_generation!(key_generation_1024, 2, 1024);

    key_generation!(key_generation_multi_3_256, 3, 256);

    key_generation!(key_generation_multi_4_64, 4, 64);

    key_generation!(key_generation_multi_5_64, 5, 64);
    key_generation!(key_generation_multi_8_576, 8, 576);
    key_generation!(key_generation_multi_16_1024, 16, 1024);
}
