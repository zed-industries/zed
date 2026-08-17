// Copyright 1995-2016 The OpenSSL Project Authors. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#include <openssl/bn.h>

#include <openssl/err.h>
#include <openssl/mem.h>

#include "../../internal.h"
#include "internal.h"


// kPrimes contains the first 1024 primes.
static const uint16_t kPrimes[] = {
    2,    3,    5,    7,    11,   13,   17,   19,   23,   29,   31,   37,
    41,   43,   47,   53,   59,   61,   67,   71,   73,   79,   83,   89,
    97,   101,  103,  107,  109,  113,  127,  131,  137,  139,  149,  151,
    157,  163,  167,  173,  179,  181,  191,  193,  197,  199,  211,  223,
    227,  229,  233,  239,  241,  251,  257,  263,  269,  271,  277,  281,
    283,  293,  307,  311,  313,  317,  331,  337,  347,  349,  353,  359,
    367,  373,  379,  383,  389,  397,  401,  409,  419,  421,  431,  433,
    439,  443,  449,  457,  461,  463,  467,  479,  487,  491,  499,  503,
    509,  521,  523,  541,  547,  557,  563,  569,  571,  577,  587,  593,
    599,  601,  607,  613,  617,  619,  631,  641,  643,  647,  653,  659,
    661,  673,  677,  683,  691,  701,  709,  719,  727,  733,  739,  743,
    751,  757,  761,  769,  773,  787,  797,  809,  811,  821,  823,  827,
    829,  839,  853,  857,  859,  863,  877,  881,  883,  887,  907,  911,
    919,  929,  937,  941,  947,  953,  967,  971,  977,  983,  991,  997,
    1009, 1013, 1019, 1021, 1031, 1033, 1039, 1049, 1051, 1061, 1063, 1069,
    1087, 1091, 1093, 1097, 1103, 1109, 1117, 1123, 1129, 1151, 1153, 1163,
    1171, 1181, 1187, 1193, 1201, 1213, 1217, 1223, 1229, 1231, 1237, 1249,
    1259, 1277, 1279, 1283, 1289, 1291, 1297, 1301, 1303, 1307, 1319, 1321,
    1327, 1361, 1367, 1373, 1381, 1399, 1409, 1423, 1427, 1429, 1433, 1439,
    1447, 1451, 1453, 1459, 1471, 1481, 1483, 1487, 1489, 1493, 1499, 1511,
    1523, 1531, 1543, 1549, 1553, 1559, 1567, 1571, 1579, 1583, 1597, 1601,
    1607, 1609, 1613, 1619, 1621, 1627, 1637, 1657, 1663, 1667, 1669, 1693,
    1697, 1699, 1709, 1721, 1723, 1733, 1741, 1747, 1753, 1759, 1777, 1783,
    1787, 1789, 1801, 1811, 1823, 1831, 1847, 1861, 1867, 1871, 1873, 1877,
    1879, 1889, 1901, 1907, 1913, 1931, 1933, 1949, 1951, 1973, 1979, 1987,
    1993, 1997, 1999, 2003, 2011, 2017, 2027, 2029, 2039, 2053, 2063, 2069,
    2081, 2083, 2087, 2089, 2099, 2111, 2113, 2129, 2131, 2137, 2141, 2143,
    2153, 2161, 2179, 2203, 2207, 2213, 2221, 2237, 2239, 2243, 2251, 2267,
    2269, 2273, 2281, 2287, 2293, 2297, 2309, 2311, 2333, 2339, 2341, 2347,
    2351, 2357, 2371, 2377, 2381, 2383, 2389, 2393, 2399, 2411, 2417, 2423,
    2437, 2441, 2447, 2459, 2467, 2473, 2477, 2503, 2521, 2531, 2539, 2543,
    2549, 2551, 2557, 2579, 2591, 2593, 2609, 2617, 2621, 2633, 2647, 2657,
    2659, 2663, 2671, 2677, 2683, 2687, 2689, 2693, 2699, 2707, 2711, 2713,
    2719, 2729, 2731, 2741, 2749, 2753, 2767, 2777, 2789, 2791, 2797, 2801,
    2803, 2819, 2833, 2837, 2843, 2851, 2857, 2861, 2879, 2887, 2897, 2903,
    2909, 2917, 2927, 2939, 2953, 2957, 2963, 2969, 2971, 2999, 3001, 3011,
    3019, 3023, 3037, 3041, 3049, 3061, 3067, 3079, 3083, 3089, 3109, 3119,
    3121, 3137, 3163, 3167, 3169, 3181, 3187, 3191, 3203, 3209, 3217, 3221,
    3229, 3251, 3253, 3257, 3259, 3271, 3299, 3301, 3307, 3313, 3319, 3323,
    3329, 3331, 3343, 3347, 3359, 3361, 3371, 3373, 3389, 3391, 3407, 3413,
    3433, 3449, 3457, 3461, 3463, 3467, 3469, 3491, 3499, 3511, 3517, 3527,
    3529, 3533, 3539, 3541, 3547, 3557, 3559, 3571, 3581, 3583, 3593, 3607,
    3613, 3617, 3623, 3631, 3637, 3643, 3659, 3671, 3673, 3677, 3691, 3697,
    3701, 3709, 3719, 3727, 3733, 3739, 3761, 3767, 3769, 3779, 3793, 3797,
    3803, 3821, 3823, 3833, 3847, 3851, 3853, 3863, 3877, 3881, 3889, 3907,
    3911, 3917, 3919, 3923, 3929, 3931, 3943, 3947, 3967, 3989, 4001, 4003,
    4007, 4013, 4019, 4021, 4027, 4049, 4051, 4057, 4073, 4079, 4091, 4093,
    4099, 4111, 4127, 4129, 4133, 4139, 4153, 4157, 4159, 4177, 4201, 4211,
    4217, 4219, 4229, 4231, 4241, 4243, 4253, 4259, 4261, 4271, 4273, 4283,
    4289, 4297, 4327, 4337, 4339, 4349, 4357, 4363, 4373, 4391, 4397, 4409,
    4421, 4423, 4441, 4447, 4451, 4457, 4463, 4481, 4483, 4493, 4507, 4513,
    4517, 4519, 4523, 4547, 4549, 4561, 4567, 4583, 4591, 4597, 4603, 4621,
    4637, 4639, 4643, 4649, 4651, 4657, 4663, 4673, 4679, 4691, 4703, 4721,
    4723, 4729, 4733, 4751, 4759, 4783, 4787, 4789, 4793, 4799, 4801, 4813,
    4817, 4831, 4861, 4871, 4877, 4889, 4903, 4909, 4919, 4931, 4933, 4937,
    4943, 4951, 4957, 4967, 4969, 4973, 4987, 4993, 4999, 5003, 5009, 5011,
    5021, 5023, 5039, 5051, 5059, 5077, 5081, 5087, 5099, 5101, 5107, 5113,
    5119, 5147, 5153, 5167, 5171, 5179, 5189, 5197, 5209, 5227, 5231, 5233,
    5237, 5261, 5273, 5279, 5281, 5297, 5303, 5309, 5323, 5333, 5347, 5351,
    5381, 5387, 5393, 5399, 5407, 5413, 5417, 5419, 5431, 5437, 5441, 5443,
    5449, 5471, 5477, 5479, 5483, 5501, 5503, 5507, 5519, 5521, 5527, 5531,
    5557, 5563, 5569, 5573, 5581, 5591, 5623, 5639, 5641, 5647, 5651, 5653,
    5657, 5659, 5669, 5683, 5689, 5693, 5701, 5711, 5717, 5737, 5741, 5743,
    5749, 5779, 5783, 5791, 5801, 5807, 5813, 5821, 5827, 5839, 5843, 5849,
    5851, 5857, 5861, 5867, 5869, 5879, 5881, 5897, 5903, 5923, 5927, 5939,
    5953, 5981, 5987, 6007, 6011, 6029, 6037, 6043, 6047, 6053, 6067, 6073,
    6079, 6089, 6091, 6101, 6113, 6121, 6131, 6133, 6143, 6151, 6163, 6173,
    6197, 6199, 6203, 6211, 6217, 6221, 6229, 6247, 6257, 6263, 6269, 6271,
    6277, 6287, 6299, 6301, 6311, 6317, 6323, 6329, 6337, 6343, 6353, 6359,
    6361, 6367, 6373, 6379, 6389, 6397, 6421, 6427, 6449, 6451, 6469, 6473,
    6481, 6491, 6521, 6529, 6547, 6551, 6553, 6563, 6569, 6571, 6577, 6581,
    6599, 6607, 6619, 6637, 6653, 6659, 6661, 6673, 6679, 6689, 6691, 6701,
    6703, 6709, 6719, 6733, 6737, 6761, 6763, 6779, 6781, 6791, 6793, 6803,
    6823, 6827, 6829, 6833, 6841, 6857, 6863, 6869, 6871, 6883, 6899, 6907,
    6911, 6917, 6947, 6949, 6959, 6961, 6967, 6971, 6977, 6983, 6991, 6997,
    7001, 7013, 7019, 7027, 7039, 7043, 7057, 7069, 7079, 7103, 7109, 7121,
    7127, 7129, 7151, 7159, 7177, 7187, 7193, 7207, 7211, 7213, 7219, 7229,
    7237, 7243, 7247, 7253, 7283, 7297, 7307, 7309, 7321, 7331, 7333, 7349,
    7351, 7369, 7393, 7411, 7417, 7433, 7451, 7457, 7459, 7477, 7481, 7487,
    7489, 7499, 7507, 7517, 7523, 7529, 7537, 7541, 7547, 7549, 7559, 7561,
    7573, 7577, 7583, 7589, 7591, 7603, 7607, 7621, 7639, 7643, 7649, 7669,
    7673, 7681, 7687, 7691, 7699, 7703, 7717, 7723, 7727, 7741, 7753, 7757,
    7759, 7789, 7793, 7817, 7823, 7829, 7841, 7853, 7867, 7873, 7877, 7879,
    7883, 7901, 7907, 7919, 7927, 7933, 7937, 7949, 7951, 7963, 7993, 8009,
    8011, 8017, 8039, 8053, 8059, 8069, 8081, 8087, 8089, 8093, 8101, 8111,
    8117, 8123, 8147, 8161,
};

// BN_prime_checks_for_size returns the number of Miller-Rabin iterations
// necessary for generating a 'bits'-bit candidate prime.
//
//
// This table is generated using the algorithm of FIPS PUB 186-4
// Digital Signature Standard (DSS), section F.1, page 117.
// (https://doi.org/10.6028/NIST.FIPS.186-4)
// The following magma script was used to generate the output:
// securitybits:=125;
// k:=1024;
// for t:=1 to 65 do
//   for M:=3 to Floor(2*Sqrt(k-1)-1) do
//     S:=0;
//     // Sum over m
//     for m:=3 to M do
//       s:=0;
//       // Sum over j
//       for j:=2 to m do
//         s+:=(RealField(32)!2)^-(j+(k-1)/j);
//       end for;
//       S+:=2^(m-(m-1)*t)*s;
//     end for;
//     A:=2^(k-2-M*t);
//     B:=8*(Pi(RealField(32))^2-6)/3*2^(k-2)*S;
//     pkt:=2.00743*Log(2)*k*2^-k*(A+B);
//     seclevel:=Floor(-Log(2,pkt));
//     if seclevel ge securitybits then
//       printf "k: %5o, security: %o bits  (t: %o, M: %o)\n",k,seclevel,t,M;
//       break;
//     end if;
//   end for;
//   if seclevel ge securitybits then break; end if;
// end for;
//
// It can be run online at: http://magma.maths.usyd.edu.au/calc
// And will output:
// k:  1024, security: 129 bits  (t: 6, M: 23)
// k is the number of bits of the prime, securitybits is the level we want to
// reach.
// prime length | RSA key size | # MR tests | security level
// -------------+--------------|------------+---------------
//  (b) >= 6394 |     >= 12788 |          3 |        256 bit
//  (b) >= 3747 |     >=  7494 |          3 |        192 bit
//  (b) >= 1345 |     >=  2690 |          4 |        128 bit
//  (b) >= 1080 |     >=  2160 |          5 |        128 bit
//  (b) >=  852 |     >=  1704 |          5 |        112 bit
//  (b) >=  476 |     >=   952 |          5 |         80 bit
//  (b) >=  400 |     >=   800 |          6 |         80 bit
//  (b) >=  347 |     >=   694 |          7 |         80 bit
//  (b) >=  308 |     >=   616 |          8 |         80 bit
//  (b) >=   55 |     >=   110 |         27 |         64 bit
//  (b) >=    6 |     >=    12 |         34 |         64 bit
static int BN_prime_checks_for_size(int bits) {
  if (bits >= 3747) {
    return 3;
  }
  if (bits >= 1345) {
    return 4;
  }
  if (bits >= 476) {
    return 5;
  }
  if (bits >= 400) {
    return 6;
  }
  if (bits >= 347) {
    return 7;
  }
  if (bits >= 308) {
    return 8;
  }
  if (bits >= 55) {
    return 27;
  }
  return 34;
}

// num_trial_division_primes returns the number of primes to try with trial
// division before using more expensive checks. For larger numbers, the value
// of excluding a candidate with trial division is larger.
static size_t num_trial_division_primes(const BIGNUM *n) {
  if (n->width * BN_BITS2 > 1024) {
    return OPENSSL_ARRAY_SIZE(kPrimes);
  }
  return OPENSSL_ARRAY_SIZE(kPrimes) / 2;
}

// BN_PRIME_CHECKS_BLINDED is the iteration count for blinding the constant-time
// primality test. See |BN_primality_test| for details. This number is selected
// so that, for a candidate N-bit RSA prime, picking |BN_PRIME_CHECKS_BLINDED|
// random N-bit numbers will have at least |BN_prime_checks_for_size(N)| values
// in range with high probability.
//
// The following Python script computes the blinding factor needed for the
// corresponding iteration count.
/*
import math

# We choose candidate RSA primes between sqrt(2)/2 * 2^N and 2^N and select
# witnesses by generating random N-bit numbers. Thus the probability of
# selecting one in range is at least sqrt(2)/2.
p = math.sqrt(2) / 2

# Target around 2^-8 probability of the blinding being insufficient given that
# key generation is a one-time, noisy operation.
epsilon = 2**-8

def choose(a, b):
  r = 1
  for i in xrange(b):
    r *= a - i
    r /= (i + 1)
  return r

def failure_rate(min_uniform, iterations):
  """ Returns the probability that, for |iterations| candidate witnesses, fewer
      than |min_uniform| of them will be uniform. """
  prob = 0.0
  for i in xrange(min_uniform):
    prob += (choose(iterations, i) *
             p**i * (1-p)**(iterations - i))
  return prob

for min_uniform in (3, 4, 5, 6, 8, 13, 19, 28):
  # Find the smallest number of iterations under the target failure rate.
  iterations = min_uniform
  while True:
    prob = failure_rate(min_uniform, iterations)
    if prob < epsilon:
      print min_uniform, iterations, prob
      break
    iterations += 1

Output:
  3 9 0.00368894873911
  4 11 0.00363319494662
  5 13 0.00336215573898
  6 15 0.00300145783158
  8 19 0.00225214119331
  13 27 0.00385610026955
  19 38 0.0021410539126
  28 52 0.00325405801769

16 iterations suffices for 400-bit primes and larger (6 uniform samples needed),
which is already well below the minimum acceptable key size for RSA.
*/
#define BN_PRIME_CHECKS_BLINDED 16

static int probable_prime(BIGNUM *rnd, int bits);
static int probable_prime_dh(BIGNUM *rnd, int bits, const BIGNUM *add,
                             const BIGNUM *rem, BN_CTX *ctx);
static int probable_prime_dh_safe(BIGNUM *rnd, int bits, const BIGNUM *add,
                                  const BIGNUM *rem, BN_CTX *ctx);

BN_GENCB *BN_GENCB_new(void) {
  return reinterpret_cast<BN_GENCB *>(OPENSSL_zalloc(sizeof(BN_GENCB)));
}

void BN_GENCB_free(BN_GENCB *callback) { OPENSSL_free(callback); }

void BN_GENCB_set(BN_GENCB *callback,
                  int (*f)(int event, int n, struct bn_gencb_st *), void *arg) {
  callback->callback = f;
  callback->arg = arg;
}

int BN_GENCB_call(BN_GENCB *callback, int event, int n) {
  if (!callback) {
    return 1;
  }

  return callback->callback(event, n, callback);
}

void *BN_GENCB_get_arg(const BN_GENCB *callback) { return callback->arg; }

int BN_generate_prime_ex(BIGNUM *ret, int bits, int safe, const BIGNUM *add,
                         const BIGNUM *rem, BN_GENCB *cb) {
  BIGNUM *t;
  int i, j, c1 = 0;
  int checks = BN_prime_checks_for_size(bits);

  if (bits < 2) {
    // There are no prime numbers this small.
    OPENSSL_PUT_ERROR(BN, BN_R_BITS_TOO_SMALL);
    return 0;
  } else if (bits == 2 && safe) {
    // The smallest safe prime (7) is three bits.
    OPENSSL_PUT_ERROR(BN, BN_R_BITS_TOO_SMALL);
    return 0;
  }

  bssl::UniquePtr<BN_CTX> ctx(BN_CTX_new());
  if (ctx == nullptr) {
    return 0;
  }
  bssl::BN_CTXScope scope(ctx.get());
  t = BN_CTX_get(ctx.get());
  if (!t) {
    return 0;
  }

loop:
  // make a random number and set the top and bottom bits
  if (add == NULL) {
    if (!probable_prime(ret, bits)) {
      return 0;
    }
  } else {
    if (safe) {
      if (!probable_prime_dh_safe(ret, bits, add, rem, ctx.get())) {
        return 0;
      }
    } else {
      if (!probable_prime_dh(ret, bits, add, rem, ctx.get())) {
        return 0;
      }
    }
  }

  if (!BN_GENCB_call(cb, BN_GENCB_GENERATED, c1++)) {
    // aborted
    return 0;
  }

  if (!safe) {
    i = BN_is_prime_fasttest_ex(ret, checks, ctx.get(), 0, cb);
    if (i == -1) {
      return 0;
    } else if (i == 0) {
      goto loop;
    }
  } else {
    // for "safe prime" generation, check that (p-1)/2 is prime. Since a prime
    // is odd, We just need to divide by 2
    if (!BN_rshift1(t, ret)) {
      return 0;
    }

    // Interleave |ret| and |t|'s primality tests to avoid paying the full
    // iteration count on |ret| only to quickly discover |t| is composite.
    //
    // TODO(davidben): This doesn't quite work because an iteration count of 1
    // still runs the blinding mechanism.
    for (i = 0; i < checks; i++) {
      j = BN_is_prime_fasttest_ex(ret, 1, ctx.get(), 0, NULL);
      if (j == -1) {
        return 0;
      } else if (j == 0) {
        goto loop;
      }

      j = BN_is_prime_fasttest_ex(t, 1, ctx.get(), 0, NULL);
      if (j == -1) {
        return 0;
      } else if (j == 0) {
        goto loop;
      }

      if (!BN_GENCB_call(cb, BN_GENCB_PRIME_TEST, i)) {
        return 0;
      }
      // We have a safe prime test pass
    }
  }

  // we have a prime :-)
  return 1;
}

static int bn_trial_division(uint16_t *out, const BIGNUM *bn) {
  const size_t num_primes = num_trial_division_primes(bn);
  for (size_t i = 1; i < num_primes; i++) {
    // During RSA key generation, |bn| may be secret, but only if |bn| was
    // prime, so it is safe to leak failed trial divisions.
    if (constant_time_declassify_int(bn_mod_u16_consttime(bn, kPrimes[i]) ==
                                     0)) {
      *out = kPrimes[i];
      return 1;
    }
  }
  return 0;
}

int bn_odd_number_is_obviously_composite(const BIGNUM *bn) {
  uint16_t prime;
  return bn_trial_division(&prime, bn) && !BN_is_word(bn, prime);
}

int bn_miller_rabin_init(BN_MILLER_RABIN *miller_rabin, const BN_MONT_CTX *mont,
                         BN_CTX *ctx) {
  // This function corresponds to steps 1 through 3 of FIPS 186-4, C.3.1.
  const BIGNUM *w = &mont->N;
  // Note we do not call |BN_CTX_start| in this function. We intentionally
  // allocate values in the containing scope so they outlive this function.
  miller_rabin->w1 = BN_CTX_get(ctx);
  miller_rabin->m = BN_CTX_get(ctx);
  miller_rabin->one_mont = BN_CTX_get(ctx);
  miller_rabin->w1_mont = BN_CTX_get(ctx);
  if (miller_rabin->w1 == NULL ||        //
      miller_rabin->m == NULL ||         //
      miller_rabin->one_mont == NULL ||  //
      miller_rabin->w1_mont == NULL) {
    return 0;
  }

  // See FIPS 186-4, C.3.1, steps 1 through 3.
  if (!bn_usub_consttime(miller_rabin->w1, w, BN_value_one())) {
    return 0;
  }
  miller_rabin->a = BN_count_low_zero_bits(miller_rabin->w1);
  if (!bn_rshift_secret_shift(miller_rabin->m, miller_rabin->w1,
                              miller_rabin->a, ctx)) {
    return 0;
  }
  miller_rabin->w_bits = BN_num_bits(w);

  // Precompute some values in Montgomery form.
  if (!bn_one_to_montgomery(miller_rabin->one_mont, mont, ctx) ||
      // w - 1 is -1 mod w, so we can compute it in the Montgomery domain, -R,
      // with a subtraction. (|one_mont| cannot be zero.)
      !bn_usub_consttime(miller_rabin->w1_mont, w, miller_rabin->one_mont)) {
    return 0;
  }

  return 1;
}

int bn_miller_rabin_iteration(const BN_MILLER_RABIN *miller_rabin,
                              int *out_is_possibly_prime, const BIGNUM *b,
                              const BN_MONT_CTX *mont, BN_CTX *ctx) {
  // This function corresponds to steps 4.3 through 4.5 of FIPS 186-4, C.3.1.
  bssl::BN_CTXScope scope(ctx);

  // Step 4.3. We use Montgomery-encoding for better performance and to avoid
  // timing leaks.
  const BIGNUM *w = &mont->N;
  BIGNUM *z = BN_CTX_get(ctx);
  crypto_word_t is_possibly_prime;
  if (z == NULL ||
      !BN_mod_exp_mont_consttime(z, b, miller_rabin->m, w, ctx, mont) ||
      !BN_to_montgomery(z, z, mont, ctx)) {
    return 0;
  }

  // is_possibly_prime is all ones if we have determined |b| is not a composite
  // witness for |w|. This is equivalent to going to step 4.7 in the original
  // algorithm. To avoid timing leaks, we run the algorithm to the end for prime
  // inputs.
  is_possibly_prime = 0;

  // Step 4.4. If z = 1 or z = w-1, b is not a composite witness and w is still
  // possibly prime.
  is_possibly_prime = BN_equal_consttime(z, miller_rabin->one_mont) |
                      BN_equal_consttime(z, miller_rabin->w1_mont);
  is_possibly_prime = 0 - is_possibly_prime;  // Make it all zeros or all ones.

  // Step 4.5.
  //
  // To avoid leaking |a|, we run the loop to |w_bits| and mask off all
  // iterations once |j| = |a|.
  for (int j = 1; j < miller_rabin->w_bits; j++) {
    if (constant_time_declassify_w(constant_time_eq_int(j, miller_rabin->a) &
                                   ~is_possibly_prime)) {
      // If the loop is done and we haven't seen z = 1 or z = w-1 yet, the
      // value is composite and we can break in variable time.
      break;
    }

    // Step 4.5.1.
    if (!BN_mod_mul_montgomery(z, z, z, mont, ctx)) {
      return 0;
    }

    // Step 4.5.2. If z = w-1 and the loop is not done, this is not a composite
    // witness.
    crypto_word_t z_is_w1_mont = BN_equal_consttime(z, miller_rabin->w1_mont);
    z_is_w1_mont = 0 - z_is_w1_mont;    // Make it all zeros or all ones.
    is_possibly_prime |= z_is_w1_mont;  // Go to step 4.7 if |z_is_w1_mont|.

    // Step 4.5.3. If z = 1 and the loop is not done, the previous value of z
    // was not -1. There are no non-trivial square roots of 1 modulo a prime, so
    // w is composite and we may exit in variable time.
    if (constant_time_declassify_w(
            BN_equal_consttime(z, miller_rabin->one_mont) &
            ~is_possibly_prime)) {
      break;
    }
  }

  *out_is_possibly_prime = constant_time_declassify_w(is_possibly_prime) & 1;
  return 1;
}

int BN_primality_test(int *out_is_probably_prime, const BIGNUM *w, int checks,
                      BN_CTX *ctx, int do_trial_division, BN_GENCB *cb) {
  // This function's secrecy and performance requirements come from RSA key
  // generation. We generate RSA keys by selecting two large, secret primes with
  // rejection sampling.
  //
  // We thus treat |w| as secret if turns out to be a large prime. However, if
  // |w| is composite, we treat this and |w| itself as public. (Conversely, if
  // |w| is prime, that it is prime is public. Only the value is secret.) This
  // is fine for RSA key generation, but note it is important that we use
  // rejection sampling, with each candidate prime chosen independently. This
  // would not work for, e.g., an algorithm which looked for primes in
  // consecutive integers. These assumptions allow us to discard composites
  // quickly. We additionally treat |w| as public when it is a small prime to
  // simplify trial decryption and some edge cases.
  //
  // One RSA key generation will call this function on exactly two primes and
  // many more composites. The overall cost is a combination of several factors:
  //
  // 1. Checking if |w| is divisible by a small prime is much faster than
  //    learning it is composite by Miller-Rabin (see below for details on that
  //    cost). Trial division by p saves 1/p of Miller-Rabin calls, so this is
  //    worthwhile until p exceeds the ratio of the two costs.
  //
  // 2. For a random (i.e. non-adversarial) candidate large prime and candidate
  //    witness, the probability of false witness is very low. (This is why FIPS
  //    186-4 only requires a few iterations.) Thus composites not discarded by
  //    trial decryption, in practice, cost one Miller-Rabin iteration. Only the
  //    two actual primes cost the full iteration count.
  //
  // 3. A Miller-Rabin iteration is a modular exponentiation plus |a| additional
  //    modular squares, where |a| is the number of factors of two in |w-1|. |a|
  //    is likely small (the distribution falls exponentially), but it is also
  //    potentially secret, so we loop up to its log(w) upper bound when |w| is
  //    prime. When |w| is composite, we break early, so only two calls pay this
  //    cost. (Note that all calls pay the modular exponentiation which is,
  //    itself, log(w) modular multiplications and squares.)
  //
  // 4. While there are only two prime calls, they multiplicatively pay the full
  //    costs of (2) and (3).
  //
  // 5. After the primes are chosen, RSA keys derive some values from the
  //    primes, but this cost is negligible in comparison.

  *out_is_probably_prime = 0;

  if (BN_cmp(w, BN_value_one()) <= 0) {
    return 1;
  }

  if (!BN_is_odd(w)) {
    // The only even prime is two.
    *out_is_probably_prime = BN_is_word(w, 2);
    return 1;
  }

  // Miller-Rabin does not work for three.
  if (BN_is_word(w, 3)) {
    *out_is_probably_prime = 1;
    return 1;
  }

  if (do_trial_division) {
    // Perform additional trial division checks to discard small primes.
    uint16_t prime;
    if (bn_trial_division(&prime, w)) {
      *out_is_probably_prime = BN_is_word(w, prime);
      return 1;
    }
    if (!BN_GENCB_call(cb, BN_GENCB_PRIME_TEST, -1)) {
      return 0;
    }
  }

  if (checks == BN_prime_checks_for_generation) {
    checks = BN_prime_checks_for_size(BN_num_bits(w));
  }

  bssl::UniquePtr<BN_CTX> new_ctx;
  if (ctx == nullptr) {
    new_ctx.reset(BN_CTX_new());
    if (new_ctx == nullptr) {
      return 0;
    }
    ctx = new_ctx.get();
  }

  // See C.3.1 from FIPS 186-4.
  bssl::BN_CTXScope scope(ctx);
  BIGNUM *b = BN_CTX_get(ctx);
  bssl::UniquePtr<BN_MONT_CTX> mont(BN_MONT_CTX_new_consttime(w, ctx));
  BN_MILLER_RABIN miller_rabin;
  crypto_word_t uniform_iterations = 0;
  if (b == nullptr || mont == nullptr ||
      // Steps 1-3.
      !bn_miller_rabin_init(&miller_rabin, mont.get(), ctx)) {
    return 0;
  }

  // The following loop performs in inner iteration of the Miller-Rabin
  // Primality test (Step 4).
  //
  // The algorithm as specified in FIPS 186-4 leaks information on |w|, the RSA
  // private key. Instead, we run through each iteration unconditionally,
  // performing modular multiplications, masking off any effects to behave
  // equivalently to the specified algorithm.
  //
  // We also blind the number of values of |b| we try. Steps 4.1–4.2 say to
  // discard out-of-range values. To avoid leaking information on |w|, we use
  // |bn_rand_secret_range| which, rather than discarding bad values, adjusts
  // them to be in range. Though not uniformly selected, these adjusted values
  // are still usable as Miller-Rabin checks.
  //
  // Miller-Rabin is already probabilistic, so we could reach the desired
  // confidence levels by just suitably increasing the iteration count. However,
  // to align with FIPS 186-4, we use a more pessimal analysis: we do not count
  // the non-uniform values towards the iteration count. As a result, this
  // function is more complex and has more timing risk than necessary.
  //
  // We count both total iterations and uniform ones and iterate until we've
  // reached at least |BN_PRIME_CHECKS_BLINDED| and |iterations|, respectively.
  // If the latter is large enough, it will be the limiting factor with high
  // probability and we won't leak information.
  //
  // Note this blinding does not impact most calls when picking primes because
  // composites are rejected early. Only the two secret primes see extra work.

  // Using |constant_time_lt_w| seems to prevent the compiler from optimizing
  // this into two jumps.
  for (int i = 1; constant_time_declassify_w(
           (i <= BN_PRIME_CHECKS_BLINDED) |
           constant_time_lt_w(uniform_iterations, checks));
       i++) {
    // Step 4.1-4.2
    int is_uniform;
    if (!bn_rand_secret_range(b, &is_uniform, 2, miller_rabin.w1)) {
      return 0;
    }
    uniform_iterations += is_uniform;

    // Steps 4.3-4.5
    int is_possibly_prime = 0;
    if (!bn_miller_rabin_iteration(&miller_rabin, &is_possibly_prime, b,
                                   mont.get(), ctx)) {
      return 0;
    }

    if (!is_possibly_prime) {
      // Step 4.6. We did not see z = w-1 before z = 1, so w must be composite.
      *out_is_probably_prime = 0;
      return 1;
    }

    // Step 4.7
    if (!BN_GENCB_call(cb, BN_GENCB_PRIME_TEST, i - 1)) {
      return 0;
    }
  }

  declassify_assert(uniform_iterations >= (crypto_word_t)checks);
  *out_is_probably_prime = 1;
  return 1;
}

int BN_is_prime_ex(const BIGNUM *candidate, int checks, BN_CTX *ctx,
                   BN_GENCB *cb) {
  return BN_is_prime_fasttest_ex(candidate, checks, ctx, 0, cb);
}

int BN_is_prime_fasttest_ex(const BIGNUM *a, int checks, BN_CTX *ctx,
                            int do_trial_division, BN_GENCB *cb) {
  int is_probably_prime;
  if (!BN_primality_test(&is_probably_prime, a, checks, ctx, do_trial_division,
                         cb)) {
    return -1;
  }
  return is_probably_prime;
}

int BN_enhanced_miller_rabin_primality_test(
    enum bn_primality_result_t *out_result, const BIGNUM *w, int checks,
    BN_CTX *ctx, BN_GENCB *cb) {
  // Enhanced Miller-Rabin is only valid on odd integers greater than 3.
  if (!BN_is_odd(w) || BN_cmp_word(w, 3) <= 0) {
    OPENSSL_PUT_ERROR(BN, BN_R_INVALID_INPUT);
    return 0;
  }

  if (checks == BN_prime_checks_for_generation) {
    checks = BN_prime_checks_for_size(BN_num_bits(w));
  }

  bssl::BN_CTXScope scope(ctx);
  BIGNUM *w1 = BN_CTX_get(ctx);
  if (w1 == nullptr || !BN_copy(w1, w) || !BN_sub_word(w1, 1)) {
    return 0;
  }

  // Write w1 as m*2^a (Steps 1 and 2).
  int a = 0;
  while (!BN_is_bit_set(w1, a)) {
    a++;
  }
  BIGNUM *m = BN_CTX_get(ctx);
  if (m == nullptr || !BN_rshift(m, w1, a)) {
    return 0;
  }

  BIGNUM *b = BN_CTX_get(ctx);
  BIGNUM *g = BN_CTX_get(ctx);
  BIGNUM *z = BN_CTX_get(ctx);
  BIGNUM *x = BN_CTX_get(ctx);
  BIGNUM *x1 = BN_CTX_get(ctx);
  if (b == nullptr || g == nullptr || z == nullptr || x == nullptr ||
      x1 == nullptr) {
    return 0;
  }

  // Montgomery setup for computations mod w
  bssl::UniquePtr<BN_MONT_CTX> mont(BN_MONT_CTX_new_for_modulus(w, ctx));
  if (mont == nullptr) {
    return 0;
  }

  // The following loop performs in inner iteration of the Enhanced Miller-Rabin
  // Primality test (Step 4).
  for (int i = 1; i <= checks; i++) {
    // Step 4.1-4.2
    if (!BN_rand_range_ex(b, 2, w1)) {
      return 0;
    }

    // Step 4.3-4.4
    if (!BN_gcd(g, b, w, ctx)) {
      return 0;
    }
    if (BN_cmp_word(g, 1) > 0) {
      *out_result = bn_composite;
      return 1;
    }

    // Step 4.5
    if (!BN_mod_exp_mont(z, b, m, w, ctx, mont.get())) {
      return 0;
    }

    // Step 4.6
    if (BN_is_one(z) || BN_cmp(z, w1) == 0) {
      goto loop;
    }

    // Step 4.7
    for (int j = 1; j < a; j++) {
      if (!BN_copy(x, z) || !BN_mod_mul(z, x, x, w, ctx)) {
        return 0;
      }
      if (BN_cmp(z, w1) == 0) {
        goto loop;
      }
      if (BN_is_one(z)) {
        goto composite;
      }
    }

    // Step 4.8-4.9
    if (!BN_copy(x, z) || !BN_mod_mul(z, x, x, w, ctx)) {
      return 0;
    }

    // Step 4.10-4.11
    if (!BN_is_one(z) && !BN_copy(x, z)) {
      return 0;
    }

  composite:
    // Step 4.12-4.14
    if (!BN_copy(x1, x) || !BN_sub_word(x1, 1) || !BN_gcd(g, x1, w, ctx)) {
      return 0;
    }
    if (BN_cmp_word(g, 1) > 0) {
      *out_result = bn_composite;
    } else {
      *out_result = bn_non_prime_power_composite;
    }

    return 1;

  loop:
    // Step 4.15
    if (!BN_GENCB_call(cb, BN_GENCB_PRIME_TEST, i - 1)) {
      return 0;
    }
  }

  *out_result = bn_probably_prime;
  return 1;
}

static int probable_prime(BIGNUM *rnd, int bits) {
  do {
    if (!BN_rand(rnd, bits, BN_RAND_TOP_TWO, BN_RAND_BOTTOM_ODD)) {
      return 0;
    }
  } while (bn_odd_number_is_obviously_composite(rnd));
  return 1;
}

static int probable_prime_dh(BIGNUM *rnd, int bits, const BIGNUM *add,
                             const BIGNUM *rem, BN_CTX *ctx) {
  bssl::BN_CTXScope scope(ctx);
  BIGNUM *t1;
  if ((t1 = BN_CTX_get(ctx)) == nullptr) {
    return 0;
  }

  if (!BN_rand(rnd, bits, BN_RAND_TOP_ONE, BN_RAND_BOTTOM_ODD)) {
    return 0;
  }

  // we need ((rnd-rem) % add) == 0
  if (!BN_mod(t1, rnd, add, ctx)) {
    return 0;
  }
  if (!BN_sub(rnd, rnd, t1)) {
    return 0;
  }
  if (rem == nullptr) {
    if (!BN_add_word(rnd, 1)) {
      return 0;
    }
  } else {
    if (!BN_add(rnd, rnd, rem)) {
      return 0;
    }
  }
  // we now have a random number 'rand' to test.

  size_t num_primes = num_trial_division_primes(rnd);
loop:
  for (size_t i = 1; i < num_primes; i++) {
    // check that rnd is a prime
    if (bn_mod_u16_consttime(rnd, kPrimes[i]) <= 1) {
      if (!BN_add(rnd, rnd, add)) {
        return 0;
      }
      goto loop;
    }
  }

  return 1;
}

static int probable_prime_dh_safe(BIGNUM *p, int bits, const BIGNUM *padd,
                                  const BIGNUM *rem, BN_CTX *ctx) {
  bits--;
  bssl::BN_CTXScope scope(ctx);
  BIGNUM *t1 = BN_CTX_get(ctx);
  BIGNUM *q = BN_CTX_get(ctx);
  BIGNUM *qadd = BN_CTX_get(ctx);
  if (qadd == NULL) {
    return 0;
  }

  if (!BN_rshift1(qadd, padd)) {
    return 0;
  }

  if (!BN_rand(q, bits, BN_RAND_TOP_ONE, BN_RAND_BOTTOM_ODD)) {
    return 0;
  }

  // we need ((rnd-rem) % add) == 0
  if (!BN_mod(t1, q, qadd, ctx)) {
    return 0;
  }

  if (!BN_sub(q, q, t1)) {
    return 0;
  }

  if (rem == NULL) {
    if (!BN_add_word(q, 1)) {
      return 0;
    }
  } else {
    if (!BN_rshift1(t1, rem)) {
      return 0;
    }
    if (!BN_add(q, q, t1)) {
      return 0;
    }
  }

  // we now have a random number 'rand' to test.
  if (!BN_lshift1(p, q)) {
    return 0;
  }
  if (!BN_add_word(p, 1)) {
    return 0;
  }

  size_t num_primes = num_trial_division_primes(p);
loop:
  for (size_t i = 1; i < num_primes; i++) {
    // check that p and q are prime
    // check that for p and q
    // gcd(p-1,primes) == 1 (except for 2)
    if (bn_mod_u16_consttime(p, kPrimes[i]) == 0 ||
        bn_mod_u16_consttime(q, kPrimes[i]) == 0) {
      if (!BN_add(p, p, padd)) {
        return 0;
      }
      if (!BN_add(q, q, qadd)) {
        return 0;
      }
      goto loop;
    }
  }

  return 1;
}
