// Copyright (c) 2020, Google Inc.
// SPDX-License-Identifier: ISC

#ifndef OPENSSL_HEADER_CURVE25519_INTERNAL_H
#define OPENSSL_HEADER_CURVE25519_INTERNAL_H

#if defined(__cplusplus)
extern "C" {
#endif

#include <openssl/base.h>
#include <openssl/curve25519.h>

#include "../../internal.h"

typedef enum {
  ED25519_ALG,
  ED25519CTX_ALG,
  ED25519PH_ALG,
} ed25519_algorithm_t;

#define DOM2_PREFIX_SIZE 32
#define DOM2_F_SIZE 1
#define DOM2_C_SIZE 1
#define DOM2_F_OFFSET DOM2_PREFIX_SIZE
#define DOM2_C_OFFSET (DOM2_F_OFFSET + DOM2_F_SIZE)
#define DOM2_CONTEXT_OFFSET (DOM2_C_OFFSET + DOM2_C_SIZE)
#define MAX_DOM2_CONTEXT_SIZE 255
#define MAX_DOM2_SIZE \
  (DOM2_PREFIX_SIZE + DOM2_F_SIZE + DOM2_C_SIZE + MAX_DOM2_CONTEXT_SIZE)

int ED25519_keypair_internal(
    uint8_t out_public_key[ED25519_PUBLIC_KEY_LEN],
    uint8_t out_private_key[ED25519_PRIVATE_KEY_LEN]);

int ed25519_sign_internal(
    ed25519_algorithm_t alg,
    uint8_t out_sig[ED25519_SIGNATURE_LEN],
    const uint8_t *message, size_t message_len,
    const uint8_t private_key[ED25519_PRIVATE_KEY_LEN],
    const uint8_t *ctx, size_t ctx_len);

int ed25519_verify_internal(
    ed25519_algorithm_t alg,
    const uint8_t *message, size_t message_len,
    const uint8_t signature[ED25519_SIGNATURE_LEN],
    const uint8_t public_key[ED25519_PUBLIC_KEY_LEN],
    const uint8_t *ctx, size_t ctx_len);

int ED25519_sign_no_self_test(
    uint8_t out_sig[ED25519_SIGNATURE_LEN],
    const uint8_t *message, size_t message_len,
    const uint8_t private_key[ED25519_PRIVATE_KEY_LEN]);

int ED25519_verify_no_self_test(
    const uint8_t *message, size_t message_len,
    const uint8_t signature[ED25519_SIGNATURE_LEN],
    const uint8_t public_key[ED25519_PUBLIC_KEY_LEN]);

int ED25519ctx_sign_no_self_test(
    uint8_t out_sig[ED25519_SIGNATURE_LEN],
    const uint8_t *message, size_t message_len,
    const uint8_t private_key[ED25519_PRIVATE_KEY_LEN],
    const uint8_t *context, size_t context_len);

int ED25519ctx_verify_no_self_test(
    const uint8_t *message, size_t message_len,
    const uint8_t signature[ED25519_SIGNATURE_LEN],
    const uint8_t public_key[ED25519_PUBLIC_KEY_LEN],
    const uint8_t *context, size_t context_len);

int ED25519ph_sign_no_self_test(
    uint8_t out_sig[ED25519_SIGNATURE_LEN],
    const uint8_t *message, size_t message_len,
    const uint8_t private_key[ED25519_PRIVATE_KEY_LEN],
    const uint8_t *context, size_t context_len);

int ED25519ph_sign_digest_no_self_test(
    uint8_t out_sig[ED25519_SIGNATURE_LEN],
    const uint8_t digest[SHA512_DIGEST_LENGTH],
    const uint8_t private_key[ED25519_PRIVATE_KEY_LEN],
    const uint8_t *context, size_t context_len);

int ED25519ph_verify_no_self_test(
    const uint8_t *message, size_t message_len,
    const uint8_t signature[ED25519_SIGNATURE_LEN],
    const uint8_t public_key[ED25519_PUBLIC_KEY_LEN],
    const uint8_t *context, size_t context_len);

int ED25519ph_verify_digest_no_self_test(
    const uint8_t digest[SHA512_DIGEST_LENGTH],
    const uint8_t signature[ED25519_SIGNATURE_LEN],
    const uint8_t public_key[ED25519_PUBLIC_KEY_LEN],
    const uint8_t *context, size_t context_len);

// If (1) x86_64 or aarch64, (2) linux or apple, and (3) OPENSSL_NO_ASM is not
// set, s2n-bignum path is capable.
#if ((defined(OPENSSL_X86_64) && !defined(MY_ASSEMBLER_IS_TOO_OLD_FOR_512AVX)) || \
     defined(OPENSSL_AARCH64)) &&                                              \
    (defined(OPENSSL_LINUX) || defined(OPENSSL_APPLE) ||                       \
     defined(OPENSSL_OPENBSD) || defined(OPENSSL_FREEBSD) || \
     defined(OPENSSL_NETBSD)) &&                  \
    !defined(OPENSSL_NO_ASM)
#define CURVE25519_S2N_BIGNUM_CAPABLE
#endif

#if defined(BORINGSSL_HAS_UINT128)
#define BORINGSSL_CURVE25519_64BIT
#endif

#if defined(BORINGSSL_CURVE25519_64BIT)
// fe means field element. Here the field is \Z/(2^255-19). An element t,
// entries t[0]...t[4], represents the integer t[0]+2^51 t[1]+2^102 t[2]+2^153
// t[3]+2^204 t[4].
// fe limbs are bounded by 1.125*2^51.
// Multiplication and carrying produce fe from fe_loose.
typedef struct fe { uint64_t v[5]; } fe;

// fe_loose limbs are bounded by 3.375*2^51.
// Addition and subtraction produce fe_loose from (fe, fe).
typedef struct fe_loose { uint64_t v[5]; } fe_loose;
#else
// fe means field element. Here the field is \Z/(2^255-19). An element t,
// entries t[0]...t[9], represents the integer t[0]+2^26 t[1]+2^51 t[2]+2^77
// t[3]+2^102 t[4]+...+2^230 t[9].
// fe limbs are bounded by 1.125*2^26,1.125*2^25,1.125*2^26,1.125*2^25,etc.
// Multiplication and carrying produce fe from fe_loose.
typedef struct fe { uint32_t v[10]; } fe;

// fe_loose limbs are bounded by 3.375*2^26,3.375*2^25,3.375*2^26,3.375*2^25,etc.
// Addition and subtraction produce fe_loose from (fe, fe).
typedef struct fe_loose { uint32_t v[10]; } fe_loose;
#endif

// ge means group element.
//
// Here the group is the set of pairs (x,y) of field elements (see fe.h)
// satisfying -x^2 + y^2 = 1 + d x^2y^2
// where d = -121665/121666.
//
// Representations:
//   ge_p2 (projective): (X:Y:Z) satisfying x=X/Z, y=Y/Z
//   ge_p3 (extended): (X:Y:Z:T) satisfying x=X/Z, y=Y/Z, XY=ZT
//   ge_p1p1 (completed): ((X:Z),(Y:T)) satisfying x=X/Z, y=Y/T
//   ge_precomp (Duif): (y+x,y-x,2dxy)

typedef struct {
  fe X;
  fe Y;
  fe Z;
} ge_p2;

typedef struct {
  fe X;
  fe Y;
  fe Z;
  fe T;
} ge_p3;

typedef struct {
  fe_loose X;
  fe_loose Y;
  fe_loose Z;
  fe_loose T;
} ge_p1p1;

typedef struct {
  fe_loose yplusx;
  fe_loose yminusx;
  fe_loose xy2d;
} ge_precomp;

typedef struct {
  fe_loose YplusX;
  fe_loose YminusX;
  fe_loose Z;
  fe_loose T2d;
} ge_cached;

void x25519_ge_tobytes(uint8_t s[32], const ge_p2 *h);
int x25519_ge_frombytes_vartime(ge_p3 *h, const uint8_t s[32]);
void x25519_ge_p3_to_cached(ge_cached *r, const ge_p3 *p);
void x25519_ge_p1p1_to_p2(ge_p2 *r, const ge_p1p1 *p);
void x25519_ge_p1p1_to_p3(ge_p3 *r, const ge_p1p1 *p);
void x25519_ge_add(ge_p1p1 *r, const ge_p3 *p, const ge_cached *q);
void x25519_ge_sub(ge_p1p1 *r, const ge_p3 *p, const ge_cached *q);
void x25519_ge_scalarmult_small_precomp(
    ge_p3 *h, const uint8_t a[32], const uint8_t precomp_table[15 * 2 * 32]);
void x25519_ge_scalarmult_base(ge_p3 *h, const uint8_t a[32]);
void x25519_ge_scalarmult(ge_p2 *r, const uint8_t *scalar, const ge_p3 *A);
void x25519_sc_reduce(uint8_t s[64]);

// x25519_scalar_mult_generic_[s2n_bignum,nohw] computes the x25519 function
// from rfc7748 6.1 using the peer coordinate (either K_A or K_B) encoded in
// |peer_public_value| and the scalar is |private_key|. The resulting shared key
// is returned in |out_shared_key|.
void x25519_scalar_mult_generic_s2n_bignum(
  uint8_t out_shared_key[X25519_SHARED_KEY_LEN],
  const uint8_t private_key[X25519_PRIVATE_KEY_LEN],
  const uint8_t peer_public_value[X25519_PUBLIC_VALUE_LEN]);
void x25519_scalar_mult_generic_nohw(
  uint8_t out_shared_key[X25519_SHARED_KEY_LEN],
  const uint8_t private_key[X25519_PRIVATE_KEY_LEN],
  const uint8_t peer_public_value[X25519_PUBLIC_VALUE_LEN]);

// x25519_public_from_private_[s2n_bignum,nohw] computes the x25519 function
// from rfc7748 6.1 using the base-coordinate 9 and scalar |private_key|. The
// resulting (encoded) public key coordinate (either K_A or K_B) is returned in
// |out_public_value|.
void x25519_public_from_private_s2n_bignum(
  uint8_t out_public_value[X25519_PUBLIC_VALUE_LEN],
  const uint8_t private_key[X25519_PRIVATE_KEY_LEN]);
void x25519_public_from_private_nohw(
  uint8_t out_public_value[X25519_PUBLIC_VALUE_LEN],
  const uint8_t private_key[X25519_PRIVATE_KEY_LEN]);

// ed25519_public_key_from_hashed_seed_[s2n_bignum,nohw] handles steps
// rfc8032 5.1.5.[3,4]. Computes [az]B and encodes the public key to a 32-byte
// octet string returning it in |out_public_key|.
void ed25519_public_key_from_hashed_seed_s2n_bignum(
  uint8_t out_public_key[ED25519_PUBLIC_KEY_LEN],
  uint8_t az[SHA512_DIGEST_LENGTH]);
void ed25519_public_key_from_hashed_seed_nohw(
  uint8_t out_public_key[ED25519_PUBLIC_KEY_LEN],
  uint8_t az[SHA512_DIGEST_LENGTH]);

// ed25519_sign_[s2n_bignum,nohw] handles steps rfc8032 5.1.6.[3,5,6,7].
// Computes the signature S = r + k * s modulo the order of the base-point B.
// Returns R || S in |out_sig|. |s| must have length
// |ED25519_PRIVATE_KEY_SEED_LEN| and |A| must have length
// |ED25519_PUBLIC_KEY_LEN|.
void ed25519_sign_s2n_bignum(uint8_t out_sig[ED25519_SIGNATURE_LEN],
                             uint8_t r[SHA512_DIGEST_LENGTH], const uint8_t *s,
                             const uint8_t *A,
                             const void *message, size_t message_len,
                             const uint8_t *dom2, size_t dom2_len);
void ed25519_sign_nohw(uint8_t out_sig[ED25519_SIGNATURE_LEN],
                       uint8_t r[SHA512_DIGEST_LENGTH], const uint8_t *s,
                       const uint8_t *A,
                       const void *message, size_t message_len,
                       const uint8_t *dom2, size_t dom2_len);

// ed25519_verify_[s2n_bignum,nohw] handles steps rfc8032 5.1.7.[1,2,3].
// Computes [S]B - [k]A' and returns the result in |R_computed_encoded|. Returns
// 1 on success and 0 otherwise. The failure case occurs if decoding of the
// public key |public_key| fails.
int ed25519_verify_s2n_bignum(uint8_t R_computed_encoded[32],
                              const uint8_t public_key[ED25519_PUBLIC_KEY_LEN],
                              uint8_t R_expected[32], uint8_t S[32],
                              const uint8_t *message, size_t message_len,
                              const uint8_t *dom2, size_t dom2_len);
int ed25519_verify_nohw(uint8_t R_computed_encoded[32],
                        const uint8_t public_key[ED25519_PUBLIC_KEY_LEN],
                        uint8_t R_expected[32], uint8_t S[32],
                        const uint8_t *message, size_t message_len,
                        const uint8_t *dom2, size_t dom2_len);

// Computes the SHA512 function of up to four input pairs: (|input1|, |len1|),
// (|input2|, |len2|), (|input3|, |len3|), (|input4|, |len4|). Specifically, the
// hash is computed over the concatenation: |input1| || |input2| || |input3| ||
// |input4|. The final two pairs may have |len3| == 0 or |len4| == 0, meaning
// those input values will be ignored. The result is written to |out|.
void ed25519_sha512(uint8_t out[SHA512_DIGEST_LENGTH],
  const void *input1, size_t len1, const void *input2, size_t len2,
  const void *input3, size_t len3, const void *input4, size_t len4);


int ed25519_check_public_key_s2n_bignum(const uint8_t public_key[ED25519_PUBLIC_KEY_LEN]);
int ed25519_check_public_key_nohw(const uint8_t public_key[ED25519_PUBLIC_KEY_LEN]);
OPENSSL_EXPORT int ED25519_check_public_key(const uint8_t public_key[ED25519_PUBLIC_KEY_LEN]);

#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_CURVE25519_INTERNAL_H
