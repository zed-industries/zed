// Copyright 2020-2021 The OpenSSL Project Authors. All Rights Reserved.
// Copyright (c) 2020-2021, Intel Corporation. All Rights Reserved.
//
// Originally written by Sergey Kirillov and Andrey Matyukov. Special
// thanks to Ilya Albrekht for his valuable hints.
//
// Intel Corporation
//
// SPDX-License-Identifier: Apache-2.0

#ifdef RSAZ_512_ENABLED

#include <openssl/crypto.h>
#include <assert.h>
#include "../../internal.h"
#include "rsaz_exp.h"

//  Internal radix
# define DIGIT_SIZE (52)
// 52-bit mask
# define DIGIT_MASK ((uint64_t)0xFFFFFFFFFFFFF)

# define BITS2WORD8_SIZE(x)  (((x) + 7) >> 3)
# define BITS2WORD64_SIZE(x) (((x) + 63) >> 6)

// Number of registers required to hold |digits_num| amount of qword
// digits
# define NUMBER_OF_REGISTERS(digits_num, register_size)            \
    (((digits_num) * 64 + (register_size) - 1) / (register_size))

OPENSSL_INLINE uint64_t get_digit(const uint8_t *in, int in_len);
OPENSSL_INLINE void put_digit(uint8_t *out, int out_len, uint64_t digit);
static void to_words52(uint64_t *out, int out_len, const uint64_t *in,
                       int in_bitsize);
static void from_words52(uint64_t *bn_out, int out_bitsize, const uint64_t *in);
OPENSSL_INLINE void set_bit(uint64_t *a, int idx);

// Number of |digit_size|-bit digits in |bitsize|-bit value
OPENSSL_INLINE int number_of_digits(int bitsize, int digit_size)
{
    return (bitsize + digit_size - 1) / digit_size;
}


// Dual {1024,1536,2048}-bit w-ary modular exponentiation using prime moduli of
// the same bit size using Almost Montgomery Multiplication, optimized with
// AVX512_IFMA256 ISA.
//
// The parameter w (window size) = 5.
//
//  [out] res      - result of modular exponentiation: 2x{20,30,40} qword
//                   values in 2^52 radix.
//  [in]  base     - base (2x{20,30,40} qword values in 2^52 radix)
//  [in]  exp      - array of 2 pointers to {16,24,32} qword values in 2^64 radix.
//                   Exponent is not converted to redundant representation.
//  [in]  m        - moduli (2x{20,30,40} qword values in 2^52 radix)
//  [in]  rr       - Montgomery parameter for 2 moduli:
//                     RR(1024) = 2^2080 mod m.
//                     RR(1536) = 2^3120 mod m.
//                     RR(2048) = 2^4160 mod m.
//                   (2x{20,30,40} qword values in 2^52 radix)
//  [in]  k0       - Montgomery parameter for 2 moduli: k0 = -1/m mod 2^64
//
// \return (void).
static int rsaz_mod_exp_x2_ifma256(uint64_t *res, const uint64_t *base,
                                   const uint64_t *exp[2], const uint64_t *m,
                                   const uint64_t *rr, const uint64_t k0[2],
                                   int modlen);

// NB: This function does not do any checks on its arguments, its
// caller `BN_mod_exp_mont_consttime_x2`, checks args. It should be
// the function used directly.
int RSAZ_mod_exp_avx512_x2(uint64_t *res1,
                           const uint64_t *base1,
                           const uint64_t *exp1,
                           const uint64_t *m1,
                           const uint64_t *rr1,
                           uint64_t k0_1,
                           uint64_t *res2,
                           const uint64_t *base2,
                           const uint64_t *exp2,
                           const uint64_t *m2,
                           const uint64_t *rr2,
                           uint64_t k0_2,
                           int modlen)
{
#ifdef BORINGSSL_DISPATCH_TEST
    BORINGSSL_function_hit[8] = 1;
#endif
    typedef void (*AMM)(uint64_t *res, const uint64_t *a,
                        const uint64_t *b, const uint64_t *m, uint64_t k0);
    int ret = 0;

    // Number of word-size (uint64_t) digits to store values in
    // redundant representation.
    int red_digits = number_of_digits(modlen + 2, DIGIT_SIZE);

    // n = modlen, d = DIGIT_SIZE, s = d * ceil((n+2)/d) > n
    // k = 4 * (s - n) = bitlen_diff
    //
    // Given the Montgomery domain conversion value RR = R^2 mod m[i]
    // = 2^2n mod m[i] and that for the larger representation in s
    // bits, RR' = R'^2 mod m[i] = 2^2s mod m[i], bitlen_diff is
    // needed to convert from RR to RR' as explained below in its
    // calculation.
    int bitlen_diff = 4 * (DIGIT_SIZE * red_digits - modlen);

    // Number of YMM registers required to store a value
    int num_ymm_regs = NUMBER_OF_REGISTERS(red_digits, 256);
    // Capacity of the register set (in qwords = 64-bits) to store a
    // value
    int regs_capacity = num_ymm_regs * 4;

    // The following 7 values are in redundant representation and are
    // to be stored contiguously in storage_aligned as needed by the
    // function rsaz_mod_exp_x2_ifma256.
    uint64_t *base1_red, *m1_red, *rr1_red;
    uint64_t *base2_red, *m2_red, *rr2_red;
    uint64_t *coeff_red;

    uint64_t *storage = NULL;
    uint64_t *storage_aligned = NULL;
    int storage_len_bytes = 7 * regs_capacity * sizeof(uint64_t)
      + 64; // alignment

    const uint64_t *exp[2] = {0};
    uint64_t k0[2] = {0};
    // AMM = Almost Montgomery Multiplication
    AMM amm = NULL;

    switch (modlen) {
    case 1024:
        amm = rsaz_amm52x20_x1_ifma256;
        break;
    case 1536:
        amm = rsaz_amm52x30_x1_ifma256;
        break;
    case 2048:
        amm = rsaz_amm52x40_x1_ifma256;
        break;
    default:
        goto err;
    }

    storage = (uint64_t *)OPENSSL_malloc(storage_len_bytes);
    if (storage == NULL)
        goto err;
    storage_aligned = (uint64_t *)align_pointer(storage, 64);

    // Memory layout for red(undant) representations
    base1_red = storage_aligned;
    base2_red = storage_aligned + 1 * regs_capacity;
    m1_red    = storage_aligned + 2 * regs_capacity;
    m2_red    = storage_aligned + 3 * regs_capacity;
    rr1_red   = storage_aligned + 4 * regs_capacity;
    rr2_red   = storage_aligned + 5 * regs_capacity;
    coeff_red = storage_aligned + 6 * regs_capacity;

    // Convert base_i, m_i, rr_i, from regular to 52-bit radix
    to_words52(base1_red, regs_capacity, base1, modlen);
    to_words52(base2_red, regs_capacity, base2, modlen);
    to_words52(m1_red,    regs_capacity, m1,    modlen);
    to_words52(m2_red,    regs_capacity, m2,    modlen);
    to_words52(rr1_red,   regs_capacity, rr1,   modlen);
    to_words52(rr2_red,   regs_capacity, rr2,   modlen);

    // Based on the definition of n and s above, we have
    //   R  = 2^n mod m; RR  = R^2 mod m
    //   R' = 2^s mod m; RR' = R'^2 mod m
    // To obtain R'^2 from R^2:
    //   - Let t = AMM(RR, RR) = R^4 / R' mod m                 -- (1)
    //   - Note that R'4 = R^4 * 2^{4*(s-n)} mod m
    //   - Let k = 4 * (s - n)
    //   - We have AMM(t, 2^k) = R^4 * 2^{4*(s-n)} / R'^2 mod m -- (2)
    //                         = R'^4 / R'^2 mod m
    //                         = R'^2 mod m
    // For example, for n = 1024, s = 1040, k = 64,
    //                 RR = 2^2048 mod m, RR' = 2^2080 mod m
    
    OPENSSL_memset(coeff_red, 0, red_digits * sizeof(uint64_t));
    // coeff_red = 2^k = 1 << bitlen_diff taking into account the
    // redundant representation in digits of DIGIT_SIZE bits
    set_bit(coeff_red, 64 * (int)(bitlen_diff / DIGIT_SIZE) + bitlen_diff % DIGIT_SIZE);

    amm(rr1_red, rr1_red, rr1_red, m1_red, k0_1); // (1) for m1
    amm(rr1_red, rr1_red, coeff_red, m1_red, k0_1); // (2) for m1

    amm(rr2_red, rr2_red, rr2_red, m2_red, k0_2); // (1) for m2
    amm(rr2_red, rr2_red, coeff_red, m2_red, k0_2); // (2) for m2

    exp[0] = exp1;
    exp[1] = exp2;

    k0[0] = k0_1;
    k0[1] = k0_2;

    // Compute res|i| = base|i| ^ exp|i| mod m|i| in parallel in
    // their contiguous form.
    ret = rsaz_mod_exp_x2_ifma256(rr1_red, base1_red, exp, m1_red, rr1_red,
                                  k0, modlen);
    if (!ret)
        goto err;

    // Convert rr_i back to regular radix
    from_words52(res1, modlen, rr1_red);
    from_words52(res2, modlen, rr2_red);

    // bn_reduce_once_in_place expects number of uint64_t, not bit
    // size
    modlen /= sizeof(uint64_t) * 8;

    bn_reduce_once_in_place(res1, 0, m1, storage, modlen);
    bn_reduce_once_in_place(res2, 0, m2, storage, modlen);

err:
    if (storage != NULL) {
        OPENSSL_cleanse(storage, storage_len_bytes);
        OPENSSL_free(storage);
    }
    return ret;
}

int rsaz_mod_exp_x2_ifma256(uint64_t *out,
                            const uint64_t *base,
                            const uint64_t *exp[2],
                            const uint64_t *m,
                            const uint64_t *rr,
                            const uint64_t k0[2],
                            int modlen)
{

    typedef void (*DAMM)(uint64_t *res, const uint64_t *a,
                         const uint64_t *b, const uint64_t *m,
                         const uint64_t k0[2]);
    typedef void (*DEXTRACT)(uint64_t *res, const uint64_t *red_table,
                             int red_table_idx, int tbl_idx);

    int ret = 0;
    int idx;

    // Exponent window size
    int exp_win_size = 5;
    int two_to_exp_win_size = 1U << exp_win_size;
    int exp_win_mask = two_to_exp_win_size - 1;

    // Number of digits (64-bit words) in redundant representation to
    // handle modulus bits
    int red_digits = 0;
    // Number of digits (64-bit words) to store the two exponents,
    // found in `exp`.
    int exp_digits = 0;

    uint64_t *storage = NULL;
    uint64_t *storage_aligned = NULL;
    int storage_len_bytes = 0;

    // Red(undant) result Y and multiplier X
    uint64_t *red_Y = NULL;     // [2][red_digits]
    uint64_t *red_X = NULL;     // [2][red_digits]
    /* Pre-computed table of base powers */
    uint64_t *red_table = NULL; // [two_to_exp_win_size][2][red_digits]
    // Expanded exponent
    uint64_t *expz = NULL;      // [2][exp_digits + 1]

    // Dual AMM
    DAMM damm = NULL;
    // Extractor from red_table
    DEXTRACT extract = NULL;

// Squaring is done using multiplication now. That can be a subject of
// optimization in future.
# define DAMS(r,a,m,k0) damm((r),(a),(a),(m),(k0))

    switch (modlen) {
    case 1024:
        red_digits = 20;
        exp_digits = 16;
        damm = rsaz_amm52x20_x2_ifma256;
        extract = extract_multiplier_2x20_win5;
        break;
    case 1536:
        // Extended with 2 digits padding to avoid mask ops in high YMM register 
        red_digits = 30 + 2;
        exp_digits = 24;
        damm = rsaz_amm52x30_x2_ifma256;
        extract = extract_multiplier_2x30_win5;
        break;
    case 2048:
        red_digits = 40;
        exp_digits = 32;
        damm = rsaz_amm52x40_x2_ifma256;
        extract = extract_multiplier_2x40_win5;
        break;
    default:
        goto err;
    }

    // allocate space for 2x num digits, aligned because the data in
    // the vectors need to be 64-bit aligned.
    storage_len_bytes = (2 * red_digits                       // red_Y
                       + 2 * red_digits                       // red_X
                       + 2 * red_digits * two_to_exp_win_size // red_table
                       + 2 * (exp_digits + 1))                // expz
                       * sizeof(uint64_t)
                       + 64;                                  // alignment

    storage = (uint64_t *)OPENSSL_malloc(storage_len_bytes);
    if (storage == NULL)
        goto err;
    OPENSSL_cleanse(storage, storage_len_bytes);
    storage_aligned = (uint64_t *)align_pointer(storage, 64);

    red_Y     = storage_aligned;
    red_X     = red_Y + 2 * red_digits;
    red_table = red_X + 2 * red_digits;
    expz      = red_table + 2 * red_digits * two_to_exp_win_size;

    // Compute table of powers base^i mod m,
    // i = 0, ..., (2^EXP_WIN_SIZE) - 1
    // using the dual multiplication. Each table entry contains
    // base1^i mod m1, then base2^i mod m2.

    red_X[0 * red_digits] = 1;
    red_X[1 * red_digits] = 1;
    damm(&red_table[0 * 2 * red_digits], (const uint64_t*)red_X, rr, m, k0);
    damm(&red_table[1 * 2 * red_digits], base,  rr, m, k0);

    for (idx = 1; idx < (int)(two_to_exp_win_size / 2); idx++) {
        DAMS(&red_table[(2 * idx + 0) * 2 * red_digits],
             &red_table[(1 * idx)     * 2 * red_digits], m, k0);
        damm(&red_table[(2 * idx + 1) * 2 * red_digits],
             &red_table[(2 * idx)     * 2 * red_digits],
             &red_table[1 * 2 * red_digits], m, k0);
    }

    // Copy and expand exponents
    memcpy(&expz[0 * (exp_digits + 1)], exp[0], exp_digits * sizeof(uint64_t));
    expz[1 * (exp_digits + 1) - 1] = 0;
    memcpy(&expz[1 * (exp_digits + 1)], exp[1], exp_digits * sizeof(uint64_t));
    expz[2 * (exp_digits + 1) - 1] = 0;


    // Exponentiation
    //
    // This is Algorithm 3 in iacr 2011-239 which is cited below as
    // well.
    //
    // Rather than compute base^{exp} in one shot, the powers of
    // base^i for i = [0..2^{exp_win_size}) are precomputed and stored
    // in `red_table`. Each window of the exponent is then used as an
    // index to look up the power in the table, and then that result
    // goes through a "series of squaring", which repositions it with
    // respect to where it appears in the complete exponent. That
    // result is then multiplied by the previous result.
    //
    // The `extract` routine does the lookup, `DAMS` wraps the `damm`
    // routine to set up squaring, while `damm` is the AMM
    // routine. That is what you find happening in each iteration of
    // this loop—the stepping through the exponent one
    // `win_exp_size`-bit window at a time.
    {
        const int rem = modlen % exp_win_size;
        const uint64_t table_idx_mask = exp_win_mask;

        int exp_bit_no = modlen - rem;
        int exp_chunk_no = exp_bit_no / 64;
        int exp_chunk_shift = exp_bit_no % 64;

        uint64_t red_table_idx_1, red_table_idx_2;

	// `rem` is { 1024, 1536, 2048 } % 5 which is { 4, 1, 3 }
        // respectively.
        //
        // If this assertion ever fails then we should set this easy
        // fix exp_bit_no = modlen - exp_win_size
        assert(rem == 4 || rem == 1 || rem == 3);


        // Find the location of the 5-bit window in the exponent which
        // is stored in 64-bit digits. Left pad it with 0s to form a
        // 64-bit digit to become an index in the precomputed table.
        // The window location in the exponent is identified by its
        // least significant bit `exp_bit_no`.

#define EXP_CHUNK(i) (exp_chunk_no) + ((i) * (exp_digits + 1))
#define EXP_CHUNK1(i) (exp_chunk_no) + 1 + ((i) * (exp_digits + 1))

        // Process 1-st exp window - just init result
	red_table_idx_1 = expz[EXP_CHUNK(0)];
        red_table_idx_2 = expz[EXP_CHUNK(1)];

	// The function operates with fixed moduli sizes divisible by
	// 64, thus table index here is always in supported range [0,
	// EXP_WIN_SIZE).
        red_table_idx_1 >>= exp_chunk_shift;
        red_table_idx_2 >>= exp_chunk_shift;

        extract(&red_Y[0 * red_digits], (const uint64_t*)red_table,
		(int)red_table_idx_1, (int)red_table_idx_2);

        // Process other exp windows
        for (exp_bit_no -= exp_win_size; exp_bit_no >= 0; exp_bit_no -= exp_win_size) {
            // Extract pre-computed multiplier from the table
            {
                uint64_t T;

                exp_chunk_no = exp_bit_no / 64;
                exp_chunk_shift = exp_bit_no % 64;
                {
		  red_table_idx_1 = expz[EXP_CHUNK(0)];
                    T = expz[EXP_CHUNK1(0)];

                    red_table_idx_1 >>= exp_chunk_shift;
		    // Get additional bits from then next quadword
		    // when 64-bit boundaries are crossed.
                    if (exp_chunk_shift > 64 - exp_win_size) {
                        T <<= (64 - exp_chunk_shift);
                        red_table_idx_1 ^= T;
                    }
                    red_table_idx_1 &= table_idx_mask;
                }
                {
		  red_table_idx_2 = expz[EXP_CHUNK(1)];
                    T = expz[EXP_CHUNK1(1)];

                    red_table_idx_2 >>= exp_chunk_shift;
		    // Get additional bits from then next quadword
		    // when 64-bit boundaries are crossed.
                    if (exp_chunk_shift > 64 - exp_win_size) {
                        T <<= (64 - exp_chunk_shift);
                        red_table_idx_2 ^= T;
                    }
                    red_table_idx_2 &= table_idx_mask;
                }

                extract(&red_X[0 * red_digits], (const uint64_t*)red_table,
			(int)red_table_idx_1, (int)red_table_idx_2);
            }

            // The number of squarings is equal to the window size.
            DAMS((uint64_t*)red_Y, (const uint64_t*)red_Y, m, k0);
            DAMS((uint64_t*)red_Y, (const uint64_t*)red_Y, m, k0);
            DAMS((uint64_t*)red_Y, (const uint64_t*)red_Y, m, k0);
            DAMS((uint64_t*)red_Y, (const uint64_t*)red_Y, m, k0);
            DAMS((uint64_t*)red_Y, (const uint64_t*)red_Y, m, k0);

            damm((uint64_t*)red_Y, (const uint64_t*)red_Y, (const uint64_t*)red_X, m, k0);
        }
    }

    // NB: After the last AMM of exponentiation in Montgomery domain, the result
    // may be (modlen + 1), but the conversion out of Montgomery domain
    // performs an AMM(x,1) which guarantees that the final result is less than
    // |m|, so no conditional subtraction is needed here. See [1] for details.
    //
    // [1] Gueron, S. Efficient software implementations of modular exponentiation.
    //     DOI: 10.1007/s13389-012-0031-5

    // Convert exponentiation result out of Montgomery form but still
    // in the redundant DIGIT_SIZE-bit representation.
    memset(red_X, 0, 2 * red_digits * sizeof(uint64_t));
    red_X[0 * red_digits] = 1;
    red_X[1 * red_digits] = 1;
    damm(out, (const uint64_t*)red_Y, (const uint64_t*)red_X, m, k0);

    ret = 1;

err:
    if (storage != NULL) {
        // Clear whole storage
        OPENSSL_cleanse(storage, storage_len_bytes);
        OPENSSL_free(storage);
    }

#undef DAMS
    return ret;
}

// Compute the digit represented by the bytes given in |in|.
OPENSSL_INLINE uint64_t get_digit(const uint8_t *in, int in_len)
{
    uint64_t digit = 0;

    assert(in != NULL);
    assert(in_len <= 8);

    for (; in_len > 0; in_len--) {
        digit <<= 8;
        digit += (uint64_t)(in[in_len - 1]);
    }
    return digit;
}

// Convert array of words in regular (base=2^64) representation to
// array of words in redundant (base=2^52) one. This is because the
// multiply/add instruction uses 52-bit representations to leave room
// for carries.
static void to_words52(uint64_t *out, int out_len,
                       const uint64_t *in, int in_bitsize)
{
    uint8_t *in_str = NULL;

    assert(out != NULL);
    assert(in != NULL);
    // Check destination buffer capacity
    assert(out_len >= number_of_digits(in_bitsize, DIGIT_SIZE));

    in_str = (uint8_t *)in;

    for (; in_bitsize >= (2 * DIGIT_SIZE); in_bitsize -= (2 * DIGIT_SIZE), out += 2) {
        uint64_t digit;

        memcpy(&digit, in_str, sizeof(digit));
        out[0] = digit & DIGIT_MASK;
        in_str += 6;
        memcpy(&digit, in_str, sizeof(digit));
        out[1] = (digit >> 4) & DIGIT_MASK;
        in_str += 7;
        out_len -= 2;
    }

    if (in_bitsize > DIGIT_SIZE) {
        uint64_t digit = get_digit(in_str, 7);

        out[0] = digit & DIGIT_MASK;
        in_str += 6;
        in_bitsize -= DIGIT_SIZE;
        digit = get_digit(in_str, BITS2WORD8_SIZE(in_bitsize));
        out[1] = digit >> 4;
        out += 2;
        out_len -= 2;
    } else if (in_bitsize > 0) {
        out[0] = get_digit(in_str, BITS2WORD8_SIZE(in_bitsize));
        out++;
        out_len--;
    }

    while (out_len > 0) {
        *out = 0;
        out_len--;
        out++;
    }
}

// Convert a 64-bit unsigned integer into a byte array, |out|, which
// is in little-endian order.
OPENSSL_INLINE void put_digit(uint8_t *out, int out_len, uint64_t digit)
{
    assert(out != NULL);
    assert(out_len <= 8);

    for (; out_len > 0; out_len--) {
        *out++ = (uint8_t)(digit & 0xFF);
        digit >>= 8;
    }
}

// Convert array of words in redundant (base=2^52) representation to
// array of words in regular (base=2^64) one. This is because the
// multiply/add instruction uses 52-bit representations to leave room
// for carries.
static void from_words52(uint64_t *out, int out_bitsize, const uint64_t *in)
{
    int i;
    int out_len = BITS2WORD64_SIZE(out_bitsize);

    assert(out != NULL);
    assert(in != NULL);

    for (i = 0; i < out_len; i++)
        out[i] = 0;

    {
        uint8_t *out_str = (uint8_t *)out;

        for (; out_bitsize >= (2 * DIGIT_SIZE);
               out_bitsize -= (2 * DIGIT_SIZE), in += 2) {
            uint64_t digit;

            digit = in[0];
            memcpy(out_str, &digit, sizeof(digit));
            out_str += 6;
            digit = digit >> 48 | in[1] << 4;
            memcpy(out_str, &digit, sizeof(digit));
            out_str += 7;
        }

        if (out_bitsize > DIGIT_SIZE) {
            put_digit(out_str, 7, in[0]);
            out_str += 6;
            out_bitsize -= DIGIT_SIZE;
            put_digit(out_str, BITS2WORD8_SIZE(out_bitsize),
                        (in[1] << 4 | in[0] >> 48));
        } else if (out_bitsize) {
            put_digit(out_str, BITS2WORD8_SIZE(out_bitsize), in[0]);
        }
    }
}

// Set bit at index |idx| in the words array |a|. It does not do any
// boundaries checks, make sure the index is valid before calling the
// function.
OPENSSL_INLINE void set_bit(uint64_t *a, int idx)
{
    assert(a != NULL);

    {
        int i, j;

        i = idx / BN_BITS2;
        j = idx % BN_BITS2;
        a[i] |= (((uint64_t)1) << j);
    }
}

#endif
