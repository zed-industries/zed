// Copyright (c) 2018, Google Inc.
// SPDX-License-Identifier: ISC

#ifndef OPENSSL_HEADER_SHA_INTERNAL_H
#define OPENSSL_HEADER_SHA_INTERNAL_H

#include <openssl/base.h>

#include <openssl/hmac.h>

#include "../../internal.h"
#include "../cpucap/internal.h"

#if defined(__cplusplus)
extern "C" {
#endif

// Internal SHA2 constants

// SHA*_CHAINING_LENGTH is the chaining length in bytes of SHA-*
// It corresponds to the length in bytes of the h part of the state
#define SHA1_CHAINING_LENGTH 20
#define SHA224_CHAINING_LENGTH 32
#define SHA256_CHAINING_LENGTH 32
#define SHA384_CHAINING_LENGTH 64
#define SHA512_CHAINING_LENGTH 64
#define SHA512_224_CHAINING_LENGTH 64
#define SHA512_256_CHAINING_LENGTH 64


// SHA3 constants, from NIST FIPS202.
// https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.202.pdf
#define KECCAK1600_ROWS 5
#define KECCAK1600_WIDTH 1600

#define SHA3_224_CAPACITY_BYTES 56
#define SHA3_224_CBLOCK SHA3_BLOCKSIZE(SHA3_224_DIGEST_BITLENGTH)
#define SHA3_224_DIGEST_BITLENGTH 224
#define SHA3_224_DIGEST_LENGTH 28

#define SHA3_256_CAPACITY_BYTES 64
#define SHA3_256_CBLOCK SHA3_BLOCKSIZE(SHA3_256_DIGEST_BITLENGTH)
#define SHA3_256_DIGEST_BITLENGTH 256
#define SHA3_256_DIGEST_LENGTH 32

#define SHA3_384_CAPACITY_BYTES 96
#define SHA3_384_CBLOCK SHA3_BLOCKSIZE(SHA3_384_DIGEST_BITLENGTH)
#define SHA3_384_DIGEST_BITLENGTH 384
#define SHA3_384_DIGEST_LENGTH 48

#define SHA3_512_CAPACITY_BYTES 128
#define SHA3_512_CBLOCK SHA3_BLOCKSIZE(SHA3_512_DIGEST_BITLENGTH)
#define SHA3_512_DIGEST_BITLENGTH 512
#define SHA3_512_DIGEST_LENGTH 64

#define SHA3_BLOCKSIZE(bitlen) (KECCAK1600_WIDTH - bitlen * 2) / 8
#define SHA3_PAD_CHAR 0x06

// SHAKE constants, from NIST FIPS202.
// https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.202.pdf
#define SHAKE_PAD_CHAR 0x1F
#define SHAKE128_BLOCKSIZE ((KECCAK1600_WIDTH - 128 * 2) / 8)
#define SHAKE256_BLOCKSIZE ((KECCAK1600_WIDTH - 256 * 2) / 8)
#define XOF_BLOCKBYTES SHAKE128_BLOCKSIZE

// SHAKE128 has the maximum block size among the SHA3/SHAKE algorithms.
#define SHA3_MAX_BLOCKSIZE SHAKE128_BLOCKSIZE

// Define state flag values for Keccak-based functions
#define KECCAK1600_STATE_ABSORB     0 
// KECCAK1600_STATE_SQUEEZE is set when |SHAKE_Squeeze| is called.
// It remains set while |SHAKE_Squeeze| is called repeatedly to output 
// chunks of the XOF output.
#define KECCAK1600_STATE_SQUEEZE    1  
// KECCAK1600_STATE_FINAL is set once |SHAKE_Final| is called 
// so that |SHAKE_Squeeze| cannot be called anymore.
#define KECCAK1600_STATE_FINAL      2 

typedef struct keccak_ctx_st KECCAK1600_CTX;

// The data buffer should have at least the maximum number of
// block size bytes to fit any SHA3/SHAKE block length.
struct keccak_ctx_st {
  uint64_t A[KECCAK1600_ROWS][KECCAK1600_ROWS];
  size_t block_size;                               // cached ctx->digest->block_size
  size_t md_size;                                  // output length, variable in XOF (SHAKE)
  size_t buf_load;                                 // used bytes in below buffer
  uint8_t buf[SHA3_MAX_BLOCKSIZE];                 // should have at least the max data block size bytes
  uint8_t pad;                                     // padding character
  uint8_t state;                                   // denotes the keccak phase (absorb, squeeze, final)
};

// To avoid externalizing KECCAK1600_CTX, we hard-code the context size in
// hmac.h's |md_ctx_union| and use a compile time check here to make sure
// |KECCAK1600_CTX|'s size never exceeds that of |md_ctx_union|. This means
// that whenever a new field is added to |keccak_ctx_st| we must also update
// the hard-coded size of |sha3| in hmac.h's |md_ctx_union| with the new
// value given by |sizeof(keccaak_ctx_st)|.
OPENSSL_STATIC_ASSERT(sizeof(KECCAK1600_CTX) <= sizeof(union md_ctx_union),
                      hmac_md_ctx_union_sha3_size_needs_update)

// KECCAK1600 x4 batched context structure
typedef struct keccak_ctx_st_x4 KECCAK1600_CTX_x4;

struct keccak_ctx_st_x4 {
  uint64_t A[4][KECCAK1600_ROWS][KECCAK1600_ROWS];
};

// Define SHA{n}[_{variant}]_ASM if sha{n}_block_data_order[_{variant}] is
// defined in assembly.

#if defined(OPENSSL_PPC64LE)
#define SHA1_ALTIVEC

void sha1_block_data_order(uint32_t *state, const uint8_t *data,
                             size_t num_blocks);

#elif !defined(OPENSSL_NO_ASM) && defined(OPENSSL_ARM)

#define SHA1_ASM_NOHW
#define SHA256_ASM_NOHW
#define SHA512_ASM_NOHW

#define SHA1_ASM_HW
OPENSSL_INLINE int sha1_hw_capable(void) {
  return CRYPTO_is_ARMv8_SHA1_capable();
}

#define SHA1_ASM_NEON
void sha1_block_data_order_neon(uint32_t state[5], const uint8_t *data,
                                size_t num);

#define SHA256_ASM_HW
OPENSSL_INLINE int sha256_hw_capable(void) {
  return CRYPTO_is_ARMv8_SHA256_capable();
}

#define SHA256_ASM_NEON
void sha256_block_data_order_neon(uint32_t state[8], const uint8_t *data,
                                  size_t num);

// Armv8.2 SHA-512 instructions are not available in 32-bit.
#define SHA512_ASM_NEON
void sha512_block_data_order_neon(uint64_t state[8], const uint8_t *data,
                                  size_t num);

#elif !defined(OPENSSL_NO_ASM) && defined(OPENSSL_AARCH64)

#define SHA1_ASM_NOHW
#define SHA256_ASM_NOHW
#define SHA512_ASM_NOHW

#define SHA1_ASM_HW
OPENSSL_INLINE int sha1_hw_capable(void) {
  return CRYPTO_is_ARMv8_SHA1_capable();
}

#define SHA256_ASM_HW
OPENSSL_INLINE int sha256_hw_capable(void) {
  return CRYPTO_is_ARMv8_SHA256_capable();
}

#define SHA512_ASM_HW
OPENSSL_INLINE int sha512_hw_capable(void) {
  return CRYPTO_is_ARMv8_SHA512_capable();
}

#elif !defined(OPENSSL_NO_ASM) && defined(OPENSSL_X86)

#define SHA1_ASM_NOHW
#define SHA256_ASM_NOHW

#define SHA1_ASM_SSSE3
OPENSSL_INLINE int sha1_ssse3_capable(void) {
  // TODO(davidben): Do we need to check the FXSR bit? The Intel manual does not
  // say to.
  return CRYPTO_is_SSSE3_capable() && CRYPTO_is_FXSR_capable();
}
void sha1_block_data_order_ssse3(uint32_t state[5], const uint8_t *data,
                                 size_t num);

#define SHA1_ASM_AVX
OPENSSL_INLINE int sha1_avx_capable(void) {
  // Pre-Zen AMD CPUs had slow SHLD/SHRD; Zen added the SHA extension; see the
  // discussion in sha1-586.pl.
  //
  // TODO(davidben): Should we enable SHAEXT on 32-bit x86?
  // TODO(davidben): Do we need to check the FXSR bit? The Intel manual does not
  // say to.
  return CRYPTO_is_AVX_capable() && CRYPTO_is_intel_cpu() &&
         CRYPTO_is_FXSR_capable();
}
void sha1_block_data_order_avx(uint32_t state[5], const uint8_t *data,
                               size_t num);

#define SHA256_ASM_SSSE3
OPENSSL_INLINE int sha256_ssse3_capable(void) {
  // TODO(davidben): Do we need to check the FXSR bit? The Intel manual does not
  // say to.
  return CRYPTO_is_SSSE3_capable() && CRYPTO_is_FXSR_capable();
}
void sha256_block_data_order_ssse3(uint32_t state[8], const uint8_t *data,
                                   size_t num);

#define SHA256_ASM_AVX
OPENSSL_INLINE int sha256_avx_capable(void) {
  // Pre-Zen AMD CPUs had slow SHLD/SHRD; Zen added the SHA extension; see the
  // discussion in sha1-586.pl.
  //
  // TODO(davidben): Should we enable SHAEXT on 32-bit x86?
  // TODO(davidben): Do we need to check the FXSR bit? The Intel manual does not
  // say to.
  return CRYPTO_is_AVX_capable() && CRYPTO_is_intel_cpu() &&
         CRYPTO_is_FXSR_capable();
}
void sha256_block_data_order_avx(uint32_t state[8], const uint8_t *data,
                                 size_t num);

// TODO(crbug.com/boringssl/673): Move the remaining CPU dispatch to C.
#define SHA512_ASM
void sha512_block_data_order(uint64_t state[8], const uint8_t *data,
                             size_t num_blocks);

#elif !defined(OPENSSL_NO_ASM) && defined(OPENSSL_X86_64)

#define SHA1_ASM_NOHW
#define SHA256_ASM_NOHW
#define SHA512_ASM_NOHW

#define SHA1_ASM_HW
OPENSSL_INLINE int sha1_hw_capable(void) {
  return CRYPTO_is_SHAEXT_capable() && CRYPTO_is_SSSE3_capable();
}

#define SHA1_ASM_AVX2
OPENSSL_INLINE int sha1_avx2_capable(void) {
  // TODO: Simplify this logic, which was extracted from the assembly:
  //  * Does AVX2 imply SSSE3?
  //  * sha1_block_data_order_avx2 does not seem to use SSSE3 instructions.
  return CRYPTO_is_AVX2_capable() && CRYPTO_is_BMI2_capable() &&
         CRYPTO_is_BMI1_capable() && CRYPTO_is_SSSE3_capable();
}
void sha1_block_data_order_avx2(uint32_t state[5], const uint8_t *data,
                                size_t num);

#define SHA1_ASM_AVX
OPENSSL_INLINE int sha1_avx_capable(void) {
  // TODO: Simplify this logic, which was extracted from the assembly:
  //  * Does AVX imply SSSE3?
  //  * sha1_block_data_order_avx does not seem to use SSSE3 instructions.
  // Pre-Zen AMD CPUs had slow SHLD/SHRD; Zen added the SHA extension; see the
  // discussion in sha1-586.pl.
  return CRYPTO_is_AVX_capable() && CRYPTO_is_SSSE3_capable() &&
         CRYPTO_is_intel_cpu();
}
void sha1_block_data_order_avx(uint32_t state[5], const uint8_t *data,
                               size_t num);

#define SHA1_ASM_SSSE3
OPENSSL_INLINE int sha1_ssse3_capable(void) {
  return CRYPTO_is_SSSE3_capable();
}
void sha1_block_data_order_ssse3(uint32_t state[5], const uint8_t *data,
                                 size_t num);

#define SHA256_ASM_HW
OPENSSL_INLINE int sha256_hw_capable(void) {
  return CRYPTO_is_SHAEXT_capable();
}

#define SHA256_ASM_AVX
OPENSSL_INLINE int sha256_avx_capable(void) {
  // TODO: Simplify this logic, which was extracted from the assembly:
  //  * Does AVX imply SSSE3?
  //  * sha256_block_data_order_avx does not seem to use SSSE3 instructions.
  // Pre-Zen AMD CPUs had slow SHLD/SHRD; Zen added the SHA extension; see the
  // discussion in sha1-586.pl.
  return CRYPTO_is_AVX_capable() && CRYPTO_is_SSSE3_capable() &&
         CRYPTO_is_intel_cpu();
}
void sha256_block_data_order_avx(uint32_t state[8], const uint8_t *data,
                                 size_t num);

#define SHA256_ASM_SSSE3
OPENSSL_INLINE int sha256_ssse3_capable(void) {
  return CRYPTO_is_SSSE3_capable();
}
void sha256_block_data_order_ssse3(uint32_t state[8], const uint8_t *data,
                                   size_t num);

#define SHA512_ASM_AVX
OPENSSL_INLINE int sha512_avx_capable(void) {
  // TODO: Simplify this logic, which was extracted from the assembly:
  //  * Does AVX imply SSSE3?
  //  * sha512_block_data_order_avx does not seem to use SSSE3 instructions.
  // Pre-Zen AMD CPUs had slow SHLD/SHRD; Zen added the SHA extension; see the
  // discussion in sha1-586.pl.
  return CRYPTO_is_AVX_capable() && CRYPTO_is_SSSE3_capable() &&
         CRYPTO_is_intel_cpu();
}
void sha512_block_data_order_avx(uint64_t state[8], const uint8_t *data,
                                 size_t num);

#endif

#if defined(SHA1_ASM_HW)
void sha1_block_data_order_hw(uint32_t state[5], const uint8_t *data,
                              size_t num);
#endif
#if defined(SHA1_ASM_NOHW)
void sha1_block_data_order_nohw(uint32_t state[5], const uint8_t *data,
                                size_t num);
#endif

#if defined(SHA256_ASM_HW)
void sha256_block_data_order_hw(uint32_t state[8], const uint8_t *data,
                                size_t num);
#endif
#if defined(SHA256_ASM_NOHW)
void sha256_block_data_order_nohw(uint32_t state[8], const uint8_t *data,
                                  size_t num);
#endif

#if defined(SHA512_ASM_HW)
void sha512_block_data_order_hw(uint64_t state[8], const uint8_t *data,
                                size_t num);
#endif

#if defined(SHA512_ASM_NOHW)
void sha512_block_data_order_nohw(uint64_t state[8], const uint8_t *data,
                                  size_t num);
#endif

#if !defined(OPENSSL_NO_ASM)
#if defined(OPENSSL_AARCH64)
#define KECCAK1600_ASM
#if defined(OPENSSL_LINUX) || defined(OPENSSL_APPLE)
#define KECCAK1600_S2N_BIGNUM_ASM
#include "../../../third_party/s2n-bignum/s2n-bignum_aws-lc.h"
#endif
#endif
#if defined(OPENSSL_X86_64) && !defined(MY_ASSEMBLER_IS_TOO_OLD_FOR_512AVX)
#if defined(OPENSSL_LINUX) || defined(OPENSSL_APPLE)
#define KECCAK1600_ASM
#define KECCAK1600_S2N_BIGNUM_ASM
#include "../../../third_party/s2n-bignum/s2n-bignum_aws-lc.h"
#endif
#endif
#endif

// SHAx_Init_from_state is a low-level function that initializes |sha| with a
// custom state. |h| is the hash state in big endian. |n| is the number of bits
// processed at this point. It must be a multiple of |SHAy_CBLOCK*8|,
// where SHAy=SHA1 if SHAx=SHA1, SHAy=SHA256 if SHAx=SHA224 or SHA256, and
// SHAy=SHA512 otherwise.
// This function returns one on success and zero on error.
// This function is for internal use only and should never be directly called.
OPENSSL_EXPORT int SHA1_Init_from_state(
    SHA_CTX *sha, const uint8_t h[SHA1_CHAINING_LENGTH], uint64_t n);
OPENSSL_EXPORT int SHA224_Init_from_state(
    SHA256_CTX *sha, const uint8_t h[SHA224_CHAINING_LENGTH], uint64_t n);
OPENSSL_EXPORT int SHA256_Init_from_state(
    SHA256_CTX *sha, const uint8_t h[SHA256_CHAINING_LENGTH], uint64_t n);
OPENSSL_EXPORT int SHA384_Init_from_state(
    SHA512_CTX *sha, const uint8_t h[SHA384_CHAINING_LENGTH], uint64_t n);
OPENSSL_EXPORT int SHA512_Init_from_state(
    SHA512_CTX *sha, const uint8_t h[SHA512_CHAINING_LENGTH], uint64_t n);
OPENSSL_EXPORT int SHA512_224_Init_from_state(
    SHA512_CTX *sha, const uint8_t h[SHA512_224_CHAINING_LENGTH], uint64_t n);
OPENSSL_EXPORT int SHA512_256_Init_from_state(
    SHA512_CTX *sha, const uint8_t h[SHA512_256_CHAINING_LENGTH], uint64_t n);

// SHAx_get_state is a low-level function that exports the hash state in big
// endian into |out_h| and the number of bits processed at this point in
// |out_n|. |SHAx_Final| must not have been called before (otherwise results
// are not guaranteed). Furthermore, the number of bytes processed by
// |SHAx_Update| must be a multiple of the block length |SHAy_CBLOCK| and
// must be less than 2^61 (otherwise it fails). See comment above about
// |SHAx_Init_from_state| for the definition of SHAy.
// This function returns one on success and zero on error.
// This function is for internal use only and should never be directly called.
OPENSSL_EXPORT int SHA1_get_state(
    SHA_CTX *ctx, uint8_t out_h[SHA1_CHAINING_LENGTH], uint64_t *out_n);
OPENSSL_EXPORT int SHA224_get_state(
    SHA256_CTX *ctx, uint8_t out_h[SHA224_CHAINING_LENGTH], uint64_t *out_n);
OPENSSL_EXPORT int SHA256_get_state(
    SHA256_CTX *ctx, uint8_t out_h[SHA256_CHAINING_LENGTH], uint64_t *out_n);
OPENSSL_EXPORT int SHA384_get_state(
    SHA512_CTX *ctx, uint8_t out_h[SHA384_CHAINING_LENGTH], uint64_t *out_n);
OPENSSL_EXPORT int SHA512_get_state(
    SHA512_CTX *ctx, uint8_t out_h[SHA512_CHAINING_LENGTH], uint64_t *out_n);
OPENSSL_EXPORT int SHA512_224_get_state(
    SHA512_CTX *ctx, uint8_t out_h[SHA512_224_CHAINING_LENGTH],
    uint64_t *out_n);
OPENSSL_EXPORT int SHA512_256_get_state(
    SHA512_CTX *ctx, uint8_t out_h[SHA512_256_CHAINING_LENGTH],
    uint64_t *out_n);

/*
 * SHA3/SHAKE single-shot APIs implement SHA3 functionalities on top
 * of SHA3/SHAKE API layer
 *
 * SHA3/SHAKE single-shot functions never fail when the later call-discipline is
 * adhered to: (a) the pointers passed to the functions are valid.
 */

// SHA3_224 writes the digest of |len| bytes from |data| to |out| and returns |out|.
// There must be at least |SHA3_224_DIGEST_LENGTH| bytes of space in |out|.
// On failure |SHA3_224| returns NULL.
OPENSSL_EXPORT uint8_t *SHA3_224(const uint8_t *data, size_t len,
                                 uint8_t out[SHA3_224_DIGEST_LENGTH]);

// SHA3_256 writes the digest of |len| bytes from |data| to |out| and returns |out|.
// There must be at least |SHA3_256_DIGEST_LENGTH| bytes of space in |out|.
// On failure |SHA3_256| returns NULL.
OPENSSL_EXPORT uint8_t *SHA3_256(const uint8_t *data, size_t len,
                                 uint8_t out[SHA3_256_DIGEST_LENGTH]);

// SHA3_384 writes the digest of |len| bytes from |data| to |out| and returns |out|.
// There must be at least |SHA3_384_DIGEST_LENGTH| bytes of space in |out|.
// On failure |SHA3_384| returns NULL.
OPENSSL_EXPORT uint8_t *SHA3_384(const uint8_t *data, size_t len,
                                 uint8_t out[SHA3_384_DIGEST_LENGTH]);

// SHA3_512 writes the digest of |len| bytes from |data| to |out| and returns |out|.
// There must be at least |SHA3_512_DIGEST_LENGTH| bytes of space in |out|.
// On failure |SHA3_512| returns NULL.
OPENSSL_EXPORT uint8_t *SHA3_512(const uint8_t *data, size_t len,
                  uint8_t out[SHA3_512_DIGEST_LENGTH]);

// SHAKE128 writes the |out_len| bytes output from |in_len| bytes |data|
// to |out| and returns |out| on success and NULL on failure.
OPENSSL_EXPORT uint8_t *SHAKE128(const uint8_t *data, const size_t in_len,
                                 uint8_t *out, size_t out_len);

// SHAKE256 writes |out_len| bytes output from |in_len| bytes |data|
// to |out| and returns |out| on success and NULL on failure.
OPENSSL_EXPORT uint8_t *SHAKE256(const uint8_t *data, const size_t in_len,
                                 uint8_t *out, size_t out_len);
/*
 * SHA3 APIs implement SHA3 functionalities on top of FIPS202 API layer
 *
 * SHA3 context must go through the flow: (a) Init, (b) Update [multiple times],
 * (c) Final [one time].
 *
 * SHA3 functions never fail when the later call-discipline is adhered to:
 * (a) the context execution flow is followed (b) the pointers passed to the
 * functions are valid (c) any additional per-function parameter value conditions,
 * detailed above each SHA3_ function signature, is satisfied.
 */

// SHA3_Init initialises |ctx| field through |FIPS202_Init| and
// returns 1 on success and 0 on failure. When call-discipline is
// maintained and |bitlen| value corresponds to a SHA3 digest length
// in bits, this function never fails.
OPENSSL_EXPORT int SHA3_Init(KECCAK1600_CTX *ctx, size_t bitlen);

// SHA3_Update checks |ctx| pointer and |len| value, calls |FIPS202_Update|
// and returns 1 on success and 0 on failure. When call-discipline is
// maintained and |len| value corresponds to the input message length
// (including zero), this function never fails.
int SHA3_Update(KECCAK1600_CTX *ctx, const void *data, size_t len);

// SHA3_Final pads the last data block and absorbs it through |FIPS202_Finalize|.
// It then calls |Keccak1600_Squeeze| and returns 1 on success and 0 on failure.
// When call-discipline is maintained, this function never fails.
int SHA3_Final(uint8_t *md, KECCAK1600_CTX *ctx);

// SHA3_224_Init initialises |sha| and returns 1.
int SHA3_224_Init(KECCAK1600_CTX *sha);

// SHA3_224_Update adds |len| bytes from |data| to |sha| and returns 1.
int SHA3_224_Update(KECCAK1600_CTX *sha, const void *data, size_t len);

// SHA3_224_Final adds the final padding to |sha| and writes the resulting
// digest to |out|. It returns one on success and zero on programmer error.
int SHA3_224_Final(uint8_t out[SHA3_224_DIGEST_LENGTH], KECCAK1600_CTX *sha);

// SHA3_256_Init initialises |sha| and returns 1.
int SHA3_256_Init(KECCAK1600_CTX *sha);

// SHA3_256_Update adds |len| bytes from |data| to |sha| and returns 1.
int SHA3_256_Update(KECCAK1600_CTX *sha, const void *data, size_t len);

// SHA3_256_Final adds the final padding to |sha| and writes the resulting
// digest to |out|. It returns one on success and zero on programmer error.
int SHA3_256_Final(uint8_t out[SHA3_256_DIGEST_LENGTH], KECCAK1600_CTX *sha);

// SHA3_384_Init initialises |sha| and returns 1.
int SHA3_384_Init(KECCAK1600_CTX *sha);

// SHA3_384_Update adds |len| bytes from |data| to |sha| and returns 1.
int SHA3_384_Update(KECCAK1600_CTX *sha, const void *data, size_t len);

// SHA3_384_Final adds the final padding to |sha| and writes the resulting
// digest to |out|. It returns one on success and zero on programmer error.
int SHA3_384_Final(uint8_t out[SHA3_384_DIGEST_LENGTH], KECCAK1600_CTX *sha);

// SHA3_512_Init initialises |sha| and returns 1.
int SHA3_512_Init(KECCAK1600_CTX *sha);

// SHA3_512_Update adds |len| bytes from |data| to |sha| and returns 1.
int SHA3_512_Update(KECCAK1600_CTX *sha, const void *data, size_t len);

// SHA3_512_Final adds the final padding to |sha| and writes the resulting
// digest to |out|. It returns one on success and zero on programmer error.
int SHA3_512_Final(uint8_t out[SHA3_512_DIGEST_LENGTH], KECCAK1600_CTX *sha);

/*
 * SHAKE APIs implement SHAKE functionalities on top of FIPS202 API layer
 *
 * SHAKE context must go through the flow: (a) Init, (b) Absorb [multiple times],
 * (c) Final [one time] or Squeeze [multiple times]
 *
 * SHAKE functions never fail when the later call-discipline is adhered to:
 * (a) the context execution flow is followed (b) the pointers passed to the
 * functions are valid (c) any additional per-function parameter value conditions,
 * detailed above each SHAKE_ function signature, is satisfied.
 */

// SHAKE_Init initialises |ctx| fields through |FIPS202_Init| and
// returns 1 on success and 0 on failure. When call-discipline is
// maintained and |block_size| value corresponds to a SHAKE block size length
// in bytes, this function never fails.
int SHAKE_Init(KECCAK1600_CTX *ctx, size_t block_size);

// SHAKE_Absorb checks |ctx| pointer and |len| values. It updates and absorbs
// input blocks via |FIPS202_Update|. When call-discipline is
// maintained and |len| value corresponds to the input message length
// (including zero), this function never fails.
int SHAKE_Absorb(KECCAK1600_CTX *ctx, const void *data,
                               size_t len);

// SHAKE_Squeeze pads the last data block and absorbs it through
// |FIPS202_Finalize| on first call. It writes |len| bytes of incremental
// XOF output to |md| and returns 1 on success and 0 on failure. It can be
// called multiple times. When call-discipline is maintained, this function
// never fails.
int SHAKE_Squeeze(uint8_t *md, KECCAK1600_CTX *ctx, size_t len);

// SHAKE_Final writes |len| bytes of finalized extendible output to |md|, returns 1 on
// success and 0 on failure. It should be called once to finalize absorb and
// squeeze phases. Incremental XOF output should be generated via |SHAKE_Squeeze|.
// When call-discipline is maintained, this function never fails.
int SHAKE_Final(uint8_t *md, KECCAK1600_CTX *ctx, size_t len);

/*
 * SHAKE128_x4_ batched APIs implement x4 SHAKE functionalities on top of FIPS202 API layer
 *
 * SHAKE128_x4_ context must go through the flow: (a) Init_x4, (b) Absorb_once_x4 [one time;
 * maximum input length of |SHAKE128_BLOCKSIZE - 1|] (c) Squeezeblocks_x4 [multiple times]
 *
 * SHAKE128_x4_ functions never fail when the later call-discipline is adhered to:
 * (a) the context execution flow is followed (b) the pointers passed to the
 * functions are valid (c) any additional per-function parameter value conditions,
 * detailed above each SHAKE128_x4_ function signature, is satisfied.
 */

// SHAKE128_Init_x4 is a batched API that operates on four independent
// Keccak bitstates. It initialises all four |ctx| fields and returns
// 1 on success and 0 on failure. When call-discipline is maintained,
// this function never fails.
OPENSSL_EXPORT int SHAKE128_Init_x4(KECCAK1600_CTX_x4 *ctx);

// SHAKE128_Absorb_once_x4 is a batched API that operates on four independent
// Keccak bitstates. It absorbs all four inputs |data0|, |data1|, |data2|, |data3|
// of equal length of |len| bytes returns 1 on success and 0 on failure. When
// is maintained and |len| value corresponds to the input messages length
// call-discipline (including zero), this function never fails.
OPENSSL_EXPORT int SHAKE128_Absorb_once_x4(KECCAK1600_CTX_x4 *ctx, const void *data0, const void *data1,
                                  const void *data2, const void *data3, size_t len);

// SHAKE128_Squeezeblocks_x4 is a batched API that operates on four independent Keccak
// bitstates. It squeezes |blks| number of blocks for all four XOF digests and returns
// 1 on success and 0 on failure. When call-discipline is maintained, this function
// never fails.
OPENSSL_EXPORT int SHAKE128_Squeezeblocks_x4(uint8_t *md0, uint8_t *md1, uint8_t *md2, uint8_t *md3,
                                  KECCAK1600_CTX_x4 *ctx, size_t blks);
/*
 * SHAKE256_x4_ signle-shot batched API implements x4 SHAKE256 functionalities on top
 * of FIPS202 API layer
 *
 * SHAKE256_x4_ function never fails when the later call-discipline is adhered to:
 * (a) the pointers passed to the functions are valid.
 */

// SHAKE256_x4 is a batched API that operates on four independent
// Keccak bitstates. It writes all four |out_len|-byte outputs from
// |in_len|-byte inputs to |out0|, |out1|, |out2|, |out3| and returns
// 1 on success and 0 on failure.
// When call-discipline is maintained, this function never fails.
OPENSSL_EXPORT int SHAKE256_x4(const uint8_t *data0, const uint8_t *data1,
                                  const uint8_t *data2, const uint8_t *data3,
                                  const size_t in_len, uint8_t *out0, uint8_t *out1,
                                  uint8_t *out2, uint8_t *out3, size_t out_len);

/*
 * Keccak1600_ APIs implement Keccak absorb and squeeze phases
 */

// Keccak1600_Absorb processes the largest multiple of |r| (block size) out of
// |len| bytes and returns the remaining number of bytes.
size_t Keccak1600_Absorb(uint64_t A[KECCAK1600_ROWS][KECCAK1600_ROWS],
                                  const uint8_t *data, size_t len, size_t r);

// Keccak1600_Absorb_once_x4 absorbs exactly |len| bytes from four inputs into four
// Keccak states, applying padding character |p|. Unlike Keccak1600_Absorb, this
// processes a single block and takes the padding character as an additional argument.
void Keccak1600_Absorb_once_x4(uint64_t A[4][KECCAK1600_ROWS][KECCAK1600_ROWS],
                               const uint8_t *inp0, const uint8_t *inp1,
                               const uint8_t *inp2, const uint8_t *inp3,
                               size_t len, size_t r, uint8_t p);

// Keccak1600_Squeezeblocks_x4 squeezes |num_blocks| blocks from four Keccak states
// into four output buffers, with each block being |r| bytes.
void Keccak1600_Squeezeblocks_x4(uint64_t A[4][KECCAK1600_ROWS][KECCAK1600_ROWS],
                                 uint8_t *out0, uint8_t *out1, uint8_t *out2, uint8_t *out3,
                                 size_t num_blocks, size_t r);

// Keccak1600_Squeeze generates |out| value of |len| bytes (per call). It can be called
// multiple times when used as eXtendable Output Function. |padded| indicates
// whether it is the first call to Keccak1600_Squeeze; i.e., if the current block has
// been already processed and padded right after the last call to Keccak1600_Absorb.
// Squeezes full blocks of |r| bytes each. When performing multiple squeezes, any
// left over bytes from previous squeezes are not consumed, and |len| must be a
// multiple of the block size (except on the final squeeze).
OPENSSL_EXPORT void Keccak1600_Squeeze(uint64_t A[KECCAK1600_ROWS][KECCAK1600_ROWS],
                                 uint8_t *out, size_t len, size_t r, int padded);

#if defined(__cplusplus)
}  // extern "C"
#endif

#endif  // OPENSSL_HEADER_SHA_INTERNAL_H
