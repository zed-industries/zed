// Copyright 2016 The OpenSSL Project Authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#include <assert.h>
#include "internal.h"
#include "../../internal.h"
#include "../cpucap/internal.h"

static const uint64_t iotas[] = {
    0x0000000000000001ULL,
    0x0000000000008082ULL,
    0x800000000000808aULL,
    0x8000000080008000ULL,
    0x000000000000808bULL,
    0x0000000080000001ULL,
    0x8000000080008081ULL,
    0x8000000000008009ULL,
    0x000000000000008aULL,
    0x0000000000000088ULL,
    0x0000000080008009ULL,
    0x000000008000000aULL,
    0x000000008000808bULL,
    0x800000000000008bULL,
    0x8000000000008089ULL,
    0x8000000000008003ULL,
    0x8000000000008002ULL,
    0x8000000000000080ULL,
    0x000000000000800aULL,
    0x800000008000000aULL,
    0x8000000080008081ULL,
    0x8000000000008080ULL,
    0x0000000080000001ULL,
    0x8000000080008008ULL
};

#if defined(OPENSSL_X86_64)
static const uint64_t keccak_rho8[4] = {
    0x0605040302010007ULL, 0x0E0D0C0B0A09080FULL,
    0x0605040302010007ULL, 0x0E0D0C0B0A09080FULL
};

static const uint64_t keccak_rho56[4] = {
    0x0007060504030201ULL, 0x080F0E0D0C0B0A09ULL,
    0x0007060504030201ULL, 0x080F0E0D0C0B0A09ULL
};
#endif

#if !defined(KECCAK1600_ASM)

static const uint8_t rhotates[KECCAK1600_ROWS][KECCAK1600_ROWS] = {
    {  0,  1, 62, 28, 27 },
    { 36, 44,  6, 55, 20 },
    {  3, 10, 43, 25, 39 },
    { 41, 45, 15, 21,  8 },
    { 18,  2, 61, 56, 14 }
};

#if defined(__i386) || defined(__i386__) || defined(_M_IX86) || \
    (defined(__x86_64) && !defined(__BMI__)) || defined(_M_X64) || \
    defined(__mips) || defined(__riscv) || defined(__s390__) || defined(__loongarch__) || \
    defined(__EMSCRIPTEN__)

 // These platforms don't support "logical and with complement" instruction.
# define KECCAK_COMPLEMENTING_TRANSFORM
#endif

static uint64_t ROL64(uint64_t val, int offset) {
    if (offset == 0) {
        return val;
    } else {
        return (val << offset) | (val >> (64-offset));
    }
}

 // KECCAK_2X:
 // This is the default implementation used in OpenSSL and the most efficient;
 // the other implementations were removed from this file.
 // This implementation is a variant of KECCAK_1X (see OpenSSL)
 // This implementation allows to take temporary storage
 // out of round procedure and simplify references to it by alternating
 // it with actual data (see round loop below).
 // It ensures best compiler interpretation to assembly and provides best
 // instruction per processed byte ratio at minimal round unroll factor.
static void Round(uint64_t R[KECCAK1600_ROWS][KECCAK1600_ROWS], uint64_t A[KECCAK1600_ROWS][KECCAK1600_ROWS], size_t i) {
    uint64_t C[KECCAK1600_ROWS], D[KECCAK1600_ROWS];

    assert(i < (sizeof(iotas) / sizeof(iotas[0])));

    C[0] = A[0][0] ^ A[1][0] ^ A[2][0] ^ A[3][0] ^ A[4][0];
    C[1] = A[0][1] ^ A[1][1] ^ A[2][1] ^ A[3][1] ^ A[4][1];
    C[2] = A[0][2] ^ A[1][2] ^ A[2][2] ^ A[3][2] ^ A[4][2];
    C[3] = A[0][3] ^ A[1][3] ^ A[2][3] ^ A[3][3] ^ A[4][3];
    C[4] = A[0][4] ^ A[1][4] ^ A[2][4] ^ A[3][4] ^ A[4][4];

    D[0] = ROL64(C[1], 1) ^ C[4];
    D[1] = ROL64(C[2], 1) ^ C[0];
    D[2] = ROL64(C[3], 1) ^ C[1];
    D[3] = ROL64(C[4], 1) ^ C[2];
    D[4] = ROL64(C[0], 1) ^ C[3];

    C[0] =       A[0][0] ^ D[0];
    C[1] = ROL64(A[1][1] ^ D[1], rhotates[1][1]);
    C[2] = ROL64(A[2][2] ^ D[2], rhotates[2][2]);
    C[3] = ROL64(A[3][3] ^ D[3], rhotates[3][3]);
    C[4] = ROL64(A[4][4] ^ D[4], rhotates[4][4]);

#ifdef KECCAK_COMPLEMENTING_TRANSFORM
    R[0][0] = C[0] ^ ( C[1] | C[2]) ^ iotas[i];
    R[0][1] = C[1] ^ (~C[2] | C[3]);
    R[0][2] = C[2] ^ ( C[3] & C[4]);
    R[0][3] = C[3] ^ ( C[4] | C[0]);
    R[0][4] = C[4] ^ ( C[0] & C[1]);
#else
    R[0][0] = C[0] ^ (~C[1] & C[2]) ^ iotas[i];
    R[0][1] = C[1] ^ (~C[2] & C[3]);
    R[0][2] = C[2] ^ (~C[3] & C[4]);
    R[0][3] = C[3] ^ (~C[4] & C[0]);
    R[0][4] = C[4] ^ (~C[0] & C[1]);
#endif

    C[0] = ROL64(A[0][3] ^ D[3], rhotates[0][3]);
    C[1] = ROL64(A[1][4] ^ D[4], rhotates[1][4]);
    C[2] = ROL64(A[2][0] ^ D[0], rhotates[2][0]);
    C[3] = ROL64(A[3][1] ^ D[1], rhotates[3][1]);
    C[4] = ROL64(A[4][2] ^ D[2], rhotates[4][2]);

#ifdef KECCAK_COMPLEMENTING_TRANSFORM
    R[1][0] = C[0] ^ (C[1] |  C[2]);
    R[1][1] = C[1] ^ (C[2] &  C[3]);
    R[1][2] = C[2] ^ (C[3] | ~C[4]);
    R[1][3] = C[3] ^ (C[4] |  C[0]);
    R[1][4] = C[4] ^ (C[0] &  C[1]);
#else
    R[1][0] = C[0] ^ (~C[1] & C[2]);
    R[1][1] = C[1] ^ (~C[2] & C[3]);
    R[1][2] = C[2] ^ (~C[3] & C[4]);
    R[1][3] = C[3] ^ (~C[4] & C[0]);
    R[1][4] = C[4] ^ (~C[0] & C[1]);
#endif

    C[0] = ROL64(A[0][1] ^ D[1], rhotates[0][1]);
    C[1] = ROL64(A[1][2] ^ D[2], rhotates[1][2]);
    C[2] = ROL64(A[2][3] ^ D[3], rhotates[2][3]);
    C[3] = ROL64(A[3][4] ^ D[4], rhotates[3][4]);
    C[4] = ROL64(A[4][0] ^ D[0], rhotates[4][0]);

#ifdef KECCAK_COMPLEMENTING_TRANSFORM
    R[2][0] =  C[0] ^ ( C[1] | C[2]);
    R[2][1] =  C[1] ^ ( C[2] & C[3]);
    R[2][2] =  C[2] ^ (~C[3] & C[4]);
    R[2][3] = ~C[3] ^ ( C[4] | C[0]);
    R[2][4] =  C[4] ^ ( C[0] & C[1]);
#else
    R[2][0] = C[0] ^ (~C[1] & C[2]);
    R[2][1] = C[1] ^ (~C[2] & C[3]);
    R[2][2] = C[2] ^ (~C[3] & C[4]);
    R[2][3] = C[3] ^ (~C[4] & C[0]);
    R[2][4] = C[4] ^ (~C[0] & C[1]);
#endif

    C[0] = ROL64(A[0][4] ^ D[4], rhotates[0][4]);
    C[1] = ROL64(A[1][0] ^ D[0], rhotates[1][0]);
    C[2] = ROL64(A[2][1] ^ D[1], rhotates[2][1]);
    C[3] = ROL64(A[3][2] ^ D[2], rhotates[3][2]);
    C[4] = ROL64(A[4][3] ^ D[3], rhotates[4][3]);

#ifdef KECCAK_COMPLEMENTING_TRANSFORM
    R[3][0] =  C[0] ^ ( C[1] & C[2]);
    R[3][1] =  C[1] ^ ( C[2] | C[3]);
    R[3][2] =  C[2] ^ (~C[3] | C[4]);
    R[3][3] = ~C[3] ^ ( C[4] & C[0]);
    R[3][4] =  C[4] ^ ( C[0] | C[1]);
#else
    R[3][0] = C[0] ^ (~C[1] & C[2]);
    R[3][1] = C[1] ^ (~C[2] & C[3]);
    R[3][2] = C[2] ^ (~C[3] & C[4]);
    R[3][3] = C[3] ^ (~C[4] & C[0]);
    R[3][4] = C[4] ^ (~C[0] & C[1]);
#endif

    C[0] = ROL64(A[0][2] ^ D[2], rhotates[0][2]);
    C[1] = ROL64(A[1][3] ^ D[3], rhotates[1][3]);
    C[2] = ROL64(A[2][4] ^ D[4], rhotates[2][4]);
    C[3] = ROL64(A[3][0] ^ D[0], rhotates[3][0]);
    C[4] = ROL64(A[4][1] ^ D[1], rhotates[4][1]);

#ifdef KECCAK_COMPLEMENTING_TRANSFORM
    R[4][0] =  C[0] ^ (~C[1] & C[2]);
    R[4][1] = ~C[1] ^ ( C[2] | C[3]);
    R[4][2] =  C[2] ^ ( C[3] & C[4]);
    R[4][3] =  C[3] ^ ( C[4] | C[0]);
    R[4][4] =  C[4] ^ ( C[0] & C[1]);
#else
    R[4][0] = C[0] ^ (~C[1] & C[2]);
    R[4][1] = C[1] ^ (~C[2] & C[3]);
    R[4][2] = C[2] ^ (~C[3] & C[4]);
    R[4][3] = C[3] ^ (~C[4] & C[0]);
    R[4][4] = C[4] ^ (~C[0] & C[1]);
#endif
}

static void KeccakF1600_c(uint64_t A[KECCAK1600_ROWS][KECCAK1600_ROWS]) {
    uint64_t T[KECCAK1600_ROWS][KECCAK1600_ROWS];
    size_t i;

#ifdef KECCAK_COMPLEMENTING_TRANSFORM
    A[0][1] = ~A[0][1];
    A[0][2] = ~A[0][2];
    A[1][3] = ~A[1][3];
    A[2][2] = ~A[2][2];
    A[3][2] = ~A[3][2];
    A[4][0] = ~A[4][0];
#endif

    for (i = 0; i < 24; i += 2) {
        Round(T, A, i);
        Round(A, T, i + 1);
    }

#ifdef KECCAK_COMPLEMENTING_TRANSFORM
    A[0][1] = ~A[0][1];
    A[0][2] = ~A[0][2];
    A[1][3] = ~A[1][3];
    A[2][2] = ~A[2][2];
    A[3][2] = ~A[3][2];
    A[4][0] = ~A[4][0];
#endif
}
#endif // !KECCAK1600_ASM

// Forward declaration for KeccakF1600 function
void KeccakF1600(uint64_t A[KECCAK1600_ROWS][KECCAK1600_ROWS]);

 // Keccak1600_Absorb can be called multiple times; at each invocation the
 // largest multiple of |r| out of |len| bytes are processed. The
 // remaining amount of bytes is returned. This is done to spare caller
 // trouble of calculating the largest multiple of |r|. |r| can be viewed
 // as blocksize. It is commonly (1600 - 256*n)/8, e.g. 168, 136, 104,
 // 72, but can also be (1600 - 448)/8 = 144. All this means that message
 // padding and intermediate sub-block buffering, byte- or bitwise, is
 // caller's responsibility.

// KeccakF1600_XORBytes XORs |len| bytes from |inp| into the Keccak state |A|.
// |len| must be a multiple of 8.
static void KeccakF1600_XORBytes(uint64_t A[KECCAK1600_ROWS][KECCAK1600_ROWS], const uint8_t *inp, size_t len) {
    assert(len <= SHA3_MAX_BLOCKSIZE);
    assert((len % 8) == 0);

    uint64_t *A_flat = (uint64_t *)A;
    size_t w = len / 8;

    for (size_t i = 0; i < w; i++) {
        uint64_t Ai = (uint64_t)inp[0]       | (uint64_t)inp[1] << 8  |
                      (uint64_t)inp[2] << 16 | (uint64_t)inp[3] << 24 |
                      (uint64_t)inp[4] << 32 | (uint64_t)inp[5] << 40 |
                      (uint64_t)inp[6] << 48 | (uint64_t)inp[7] << 56;
        inp += 8;
        A_flat[i] ^= Ai;
    }
}

size_t Keccak1600_Absorb(uint64_t A[KECCAK1600_ROWS][KECCAK1600_ROWS], const uint8_t *inp, size_t len,
                         size_t r) {
    assert(r < (25 * sizeof(A[0][0])) && (r % 8) == 0);

    while (len >= r) {
        KeccakF1600_XORBytes(A, inp, r);
        KeccakF1600(A);
        inp += r;
        len -= r;
    }

    return len;
}

// KeccakF1600_ExtractBytes extracts |len| bytes from the Keccak state |A| into |out|.
// This function operates on up to block_size bytes (a single block). For extracting
// more data, the state must be processed again through KeccakF1600 (see Keccak1600_Squeeze).
static void KeccakF1600_ExtractBytes(uint64_t A[KECCAK1600_ROWS][KECCAK1600_ROWS], uint8_t *out, size_t len) {
    uint64_t *A_flat = (uint64_t *)A;
    assert(len <= SHA3_MAX_BLOCKSIZE);
    size_t i = 0;

    while (len != 0) {
        uint64_t Ai = A_flat[i];

        if (len < 8) {
            for (size_t j = 0; j < len; j++) {
                *out++ = (uint8_t)Ai;
                Ai >>= 8;
            }
            return;
        }

        out[0] = (uint8_t)(Ai);
        out[1] = (uint8_t)(Ai >> 8);
        out[2] = (uint8_t)(Ai >> 16);
        out[3] = (uint8_t)(Ai >> 24);
        out[4] = (uint8_t)(Ai >> 32);
        out[5] = (uint8_t)(Ai >> 40);
        out[6] = (uint8_t)(Ai >> 48);
        out[7] = (uint8_t)(Ai >> 56);
        out += 8;
        len -= 8;
        i++;
    }
}

void Keccak1600_Squeeze(uint64_t A[KECCAK1600_ROWS][KECCAK1600_ROWS], uint8_t *out, size_t len, size_t r, int padded) {
    assert(r < (25 * sizeof(A[0][0])) && (r % 8) == 0);

    while (len != 0) {
        if (padded) {
            KeccakF1600(A);
        }
        padded = 1;

        size_t extract_len = len < r ? len : r;
        KeccakF1600_ExtractBytes(A, out, extract_len);
        out += extract_len;
        len -= extract_len;
    }
}

#if defined(KECCAK1600_ASM)

// Scalar implementation from OpenSSL provided by keccak1600-armv8.pl
extern void KeccakF1600_hw(uint64_t state[25]);

#if defined(OPENSSL_AARCH64) || defined(OPENSSL_X86_64)
static void keccak_log_dispatch(size_t id) {
#if BORINGSSL_DISPATCH_TEST
    BORINGSSL_function_hit[id] = 1;
#endif
}
#endif

void KeccakF1600(uint64_t A[KECCAK1600_ROWS][KECCAK1600_ROWS]) {
    // Dispatch logic for Keccak-x1 on AArch64:
    //
    // 1. If ASM is disabled, we use the C implementation.
    // 2. If ASM is enabled:
    //   - For Neoverse N1, V1, V2, we use scalar Keccak assembly from s2n-bignum
    //     (`sha3_keccak_f1600()`)
    //     leveraging lazy rotations from https://eprint.iacr.org/2022/1243.
    //   - Otherwise, if the Neon SHA3 extension is supported, we use the Neon
    //     Keccak assembly from s2n-bignum (`sha3_keccak_f1600_alt()`),
    //     leveraging that extension.
    //   - Otherwise, fall back to scalar Keccak implementation from OpenSSL,
    //     (`Keccak1600_hw()`), not using lazy rotations.
    //
    // Lazy rotations improve performance by up to 10% on CPUs with free
    // Barrel shifting, which includes Neoverse N1, V1, and V2. Not all
    // CPUs have free Barrel shifting (e.g. Apple M1 or Cortex-A72), so we
    // don't use it by default.
    //
    // Neoverse V1 and V2 do support SHA3 instructions, but they are only
    // implemented on 1/4 of Neon units, and are thus slower than a scalar
    // implementation.
#if defined(OPENSSL_AARCH64)
#if defined(KECCAK1600_S2N_BIGNUM_ASM)
    if (CRYPTO_is_Neoverse_N1() || CRYPTO_is_Neoverse_V1() || CRYPTO_is_Neoverse_V2()) {
        keccak_log_dispatch(10); // kFlag_sha3_keccak_f1600
        sha3_keccak_f1600((uint64_t *)A, iotas);
        return;
    }

#if defined(MY_ASSEMBLER_SUPPORTS_NEON_SHA3_EXTENSION)
    if (CRYPTO_is_ARMv8_SHA3_capable()) {
        keccak_log_dispatch(11); // kFlag_sha3_keccak_f1600_alt
        sha3_keccak_f1600_alt((uint64_t *)A, iotas);
        return;
    }
#endif
#endif

    keccak_log_dispatch(9); // kFlag_KeccakF1600_hw
    KeccakF1600_hw((uint64_t *) A);

#elif defined(OPENSSL_X86_64)
    keccak_log_dispatch(9); // kFlag_sha3_keccak_f1600
    sha3_keccak_f1600((uint64_t *)A, iotas);
#endif
}

#else // KECCAK1600_ASM

void KeccakF1600(uint64_t A[KECCAK1600_ROWS][KECCAK1600_ROWS])
{
    KeccakF1600_c(A);
}

#endif // !KECCAK1600_ASM

// KeccakF1600_XORBytes_x4 XORs |len| bytes from |inp0|, |inp1|, |inp2|, |inp3|
// into the four Keccak states in |A|. |len| must be a multiple of 8.
static void KeccakF1600_XORBytes_x4(uint64_t A[4][KECCAK1600_ROWS][KECCAK1600_ROWS],
                                    const uint8_t *inp0, const uint8_t *inp1,
                                    const uint8_t *inp2, const uint8_t *inp3,
                                    size_t len) {
    KeccakF1600_XORBytes(A[0], inp0, len);
    KeccakF1600_XORBytes(A[1], inp1, len);
    KeccakF1600_XORBytes(A[2], inp2, len);
    KeccakF1600_XORBytes(A[3], inp3, len);
}

// KeccakF1600_ExtractBytes_x4 extracts |len| bytes from the four Keccak states in |A|
// into |out0|, |out1|, |out2|, |out3|.
static void KeccakF1600_ExtractBytes_x4(uint64_t A[4][KECCAK1600_ROWS][KECCAK1600_ROWS],
                                        uint8_t *out0, uint8_t *out1,
                                        uint8_t *out2, uint8_t *out3,
                                        size_t len) {
    KeccakF1600_ExtractBytes(A[0], out0, len);
    KeccakF1600_ExtractBytes(A[1], out1, len);
    KeccakF1600_ExtractBytes(A[2], out2, len);
    KeccakF1600_ExtractBytes(A[3], out3, len);
}

static void Keccak1600_x4(uint64_t A[4][KECCAK1600_ROWS][KECCAK1600_ROWS]) {
    // Dispatch logic for Keccak-x4 on AArch64:
    //
    // 1. If ASM is disabled, we use 4x the C implementation.
    // 2. If ASM is enabled:
    // - For Neoverse N1, we use scalar batched hybrid Keccak assembly from s2n-bignum
    //   (`sha3_keccak4_f1600_alt()`) leveraging Neon and scalar assembly with
    //   lazy rotations.
    // - For Neoverse V1, V2, we use SIMD batched hybrid Keccak assembly from s2n-bignum
    //   (`sha3_keccak4_f1600_alt2()`) leveraging Neon, Neon SHA3 extension,
    //   and scalar assembly with lazy rotations.
    // - Otherwise, if the Neon SHA3 extension is supported, we use the 2-fold
    //   Keccak assembly from s2n-bignum (`sha3_keccak2_f1600()`) twice,
    //   which is a straightforward implementation using the SHA3 extension.
    // - Otherwise, fall back to four times the 1-fold Keccak implementation
    //   (which has its own dispatch logic).
#if defined(KECCAK1600_S2N_BIGNUM_ASM) && defined(OPENSSL_AARCH64)
    if (CRYPTO_is_Neoverse_N1()) {
        keccak_log_dispatch(13); // kFlag_sha3_keccak4_f1600_alt
        sha3_keccak4_f1600_alt((uint64_t *)A, iotas);
        return;
    }

#if defined(MY_ASSEMBLER_SUPPORTS_NEON_SHA3_EXTENSION)
    if (CRYPTO_is_Neoverse_V1() || CRYPTO_is_Neoverse_V2()) {
        keccak_log_dispatch(14); // kFlag_sha3_keccak4_f1600_alt2
        sha3_keccak4_f1600_alt2((uint64_t *)A, iotas);
        return;
    }

    if (CRYPTO_is_ARMv8_SHA3_capable()) {
        keccak_log_dispatch(12); // kFlag_sha3_keccak2_f1600
        // Use 2-fold function twice: A[0:1] and A[2:3]
        sha3_keccak2_f1600((uint64_t *)&A[0], iotas);
        sha3_keccak2_f1600((uint64_t *)&A[2], iotas);
        return;
    }
#endif
#endif

#if defined(KECCAK1600_S2N_BIGNUM_ASM) && defined(OPENSSL_X86_64)
    if (CRYPTO_is_AVX2_capable()) {
        keccak_log_dispatch(10); // kFlag_sha3_keccak4_f1600_alt
        sha3_keccak4_f1600_alt((uint64_t *)A, iotas, keccak_rho8, keccak_rho56);
        return;
    }
#endif

    // Fallback: 4x individual KeccakF1600 calls (each with their own dispatch)
    KeccakF1600(A[0]);
    KeccakF1600(A[1]);
    KeccakF1600(A[2]);
    KeccakF1600(A[3]);
}

// One-shot absorb + finalize. Note that in contract to non-batched Keccak,
// this does _not_ run a Keccak permutation at the end, allowing for a uniform
// implementation of Keccak1600_Squeezeblocks_x4() without `padded` parameter
// as in the non-batched implementation.
void Keccak1600_Absorb_once_x4(uint64_t A[4][KECCAK1600_ROWS][KECCAK1600_ROWS],
                               const uint8_t *inp0, const uint8_t *inp1,
                               const uint8_t *inp2, const uint8_t *inp3,
                               size_t len, size_t r, uint8_t p) {
    assert(r <= SHA3_MAX_BLOCKSIZE);

    while (len >= r) {
        KeccakF1600_XORBytes_x4(A, inp0, inp1, inp2, inp3, r);
        Keccak1600_x4(A);
        inp0 += r;
        inp1 += r;
        inp2 += r;
        inp3 += r;
        len -= r;
    }

    // Build 16-byte aligned final blocks for each input
    alignas(16) uint8_t final[4][SHA3_MAX_BLOCKSIZE] = {{0}};

    // Copy the remainder bytes to final blocks
    OPENSSL_memcpy(final[0], inp0, len);
    OPENSSL_memcpy(final[1], inp1, len);
    OPENSSL_memcpy(final[2], inp2, len);
    OPENSSL_memcpy(final[3], inp3, len);

    if (len == r - 1) {
        p |= 128;
    } else {
        final[0][r - 1] |= 128;
        final[1][r - 1] |= 128;
        final[2][r - 1] |= 128;
        final[3][r - 1] |= 128;
    }

    final[0][len] |= p;
    final[1][len] |= p;
    final[2][len] |= p;
    final[3][len] |= p;

    KeccakF1600_XORBytes_x4(A, final[0], final[1], final[2], final[3], r);

    // Clean up final blocks to avoid stack leakage
    OPENSSL_cleanse(final, sizeof(final));
}

void Keccak1600_Squeezeblocks_x4(uint64_t A[4][KECCAK1600_ROWS][KECCAK1600_ROWS], uint8_t *out0, uint8_t *out1,
                                 uint8_t *out2, uint8_t *out3,
                                 size_t num_blocks, size_t r) {
    while (num_blocks != 0) {
        Keccak1600_x4(A);
        KeccakF1600_ExtractBytes_x4(A, out0, out1, out2, out3, r);

        out0 += r;
        out1 += r;
        out2 += r;
        out3 += r;
        num_blocks--;
    }
}
