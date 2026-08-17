// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#include "internal.h"
#include "../cpucap/internal.h"

#if defined(CURVE25519_S2N_BIGNUM_CAPABLE)
#include "../../../third_party/s2n-bignum/s2n-bignum_aws-lc.h"

void x25519_scalar_mult_generic_s2n_bignum(
  uint8_t out_shared_key[X25519_SHARED_KEY_LEN],
  const uint8_t private_key[X25519_PRIVATE_KEY_LEN],
  const uint8_t peer_public_value[X25519_PUBLIC_VALUE_LEN]) {

  uint8_t private_key_internal_demask[X25519_PRIVATE_KEY_LEN];
  OPENSSL_memcpy(private_key_internal_demask, private_key, X25519_PRIVATE_KEY_LEN);
  private_key_internal_demask[0] &= 248;
  private_key_internal_demask[31] &= 127;
  private_key_internal_demask[31] |= 64;

  curve25519_x25519_byte_selector(out_shared_key,
                                  private_key_internal_demask,
                                  peer_public_value);

  OPENSSL_cleanse(private_key_internal_demask, sizeof(private_key_internal_demask));
}

void x25519_public_from_private_s2n_bignum(
  uint8_t out_public_value[X25519_PUBLIC_VALUE_LEN],
	const uint8_t private_key[X25519_PRIVATE_KEY_LEN]) {

  uint8_t private_key_internal_demask[X25519_PRIVATE_KEY_LEN];
  OPENSSL_memcpy(private_key_internal_demask, private_key, X25519_PRIVATE_KEY_LEN);
  private_key_internal_demask[0] &= 248;
  private_key_internal_demask[31] &= 127;
  private_key_internal_demask[31] |= 64;

  curve25519_x25519base_byte_selector(out_public_value, private_key_internal_demask);

  OPENSSL_cleanse(private_key_internal_demask, sizeof(private_key_internal_demask));
}

void ed25519_public_key_from_hashed_seed_s2n_bignum(
  uint8_t out_public_key[ED25519_PUBLIC_KEY_LEN],
  uint8_t az[SHA512_DIGEST_LENGTH]) {

  uint64_t uint64_point[8] = {0};
  uint64_t uint64_hashed_seed[4] = {0};
  OPENSSL_memcpy(uint64_hashed_seed, az, 32);

  edwards25519_scalarmulbase_selector(uint64_point, uint64_hashed_seed);

  edwards25519_encode(out_public_key, uint64_point);

  OPENSSL_cleanse(uint64_hashed_seed, sizeof(uint64_hashed_seed));
}

void ed25519_sign_s2n_bignum(uint8_t out_sig[ED25519_SIGNATURE_LEN],
  uint8_t r[SHA512_DIGEST_LENGTH], const uint8_t *s, const uint8_t *A,
  const void *message, size_t message_len, const uint8_t *dom2, size_t dom2_len) {

  uint8_t k[SHA512_DIGEST_LENGTH] = {0};
  uint64_t R[8] = {0};
  uint64_t S[4] = {0};
  uint64_t uint64_r[8] = {0};
  uint64_t uint64_k[8] = {0};
  uint64_t uint64_s[4] = {0};
  OPENSSL_memcpy(uint64_r, r, 64);
  OPENSSL_memcpy(uint64_s, s, 32);

  // Reduce r modulo the order of the base-point B.
  bignum_mod_n25519(uint64_r, 8, uint64_r);

  // Compute [r]B.
  edwards25519_scalarmulbase_selector(R, uint64_r);
  edwards25519_encode(out_sig, R);

  // R is of length 32 octets
  if (dom2_len > 0) {
    // Compute k = SHA512(dom2(phflag, context) || R || A || message)
    ed25519_sha512(k, dom2, dom2_len, out_sig, 32, A, ED25519_PUBLIC_KEY_LEN, message,
                   message_len);
  } else {
    // Compute k = SHA512(R || A || message)
    ed25519_sha512(k, out_sig, 32, A, ED25519_PUBLIC_KEY_LEN, message,
                   message_len, NULL, 0);
  }
  OPENSSL_memcpy(uint64_k, k, SHA512_DIGEST_LENGTH);
  bignum_mod_n25519(uint64_k, 8, uint64_k);

  // Compute S = r + k * s modulo the order of the base-point B.
  // out_sig = R || S
  bignum_madd_n25519_selector(S, uint64_k, uint64_s, uint64_r);
  OPENSSL_memcpy(out_sig + 32, S, 32);

  OPENSSL_cleanse(k, sizeof(k));
  OPENSSL_cleanse(uint64_r, sizeof(uint64_r));
  OPENSSL_cleanse(uint64_k, sizeof(uint64_k));
  OPENSSL_cleanse(uint64_s, sizeof(uint64_s));
}

int ed25519_verify_s2n_bignum(uint8_t R_computed_encoded[32],
  const uint8_t public_key[ED25519_PUBLIC_KEY_LEN], uint8_t R_expected[32],
  uint8_t S[32], const uint8_t *message, size_t message_len, const uint8_t *dom2, size_t dom2_len) {

  uint8_t k[SHA512_DIGEST_LENGTH] = {0};
  uint64_t uint64_k[8] = {0};
  uint64_t uint64_R[8] = {0};
  uint64_t uint64_S[4] = {0};
  uint64_t A[8] = {0};

  // Decode public key as A'.
  if (edwards25519_decode_selector(A, public_key) != 0) {
    return 0;
  }

  // Step: rfc8032 5.1.7.2
  if(dom2_len > 0) {
    // Compute k = SHA512(dom2(phflag, context) || R_expected || public_key || message).
    ed25519_sha512(k, dom2, dom2_len, R_expected, 32, public_key,
                   ED25519_PUBLIC_KEY_LEN, message, message_len);
  } else {
    // Compute k = SHA512(R_expected || public_key || message).
    ed25519_sha512(k, R_expected, 32, public_key, ED25519_PUBLIC_KEY_LEN,
                   message, message_len, NULL, 0);
  }
  OPENSSL_memcpy(uint64_k, k, SHA512_DIGEST_LENGTH);
  bignum_mod_n25519(uint64_k, 8, uint64_k);

  // Step: rfc8032 5.1.7.3
  // Recall, we must compute [S]B - [k]A'.
  // First negate A'. Point negation for the twisted edwards curve when points
  // are represented in the extended coordinate system is simply:
  //   -(X,Y,Z,T) = (-X,Y,Z,-T).
  // See "Twisted Edwards curves revisited" https://ia.cr/2008/522.
  bignum_neg_p25519(A, A);

  // Compute R_have <- [S]B - [k]A'.
  OPENSSL_memcpy(uint64_S, S, 32);
  edwards25519_scalarmuldouble_selector(uint64_R, uint64_k, A, uint64_S);
  edwards25519_encode(R_computed_encoded, uint64_R);

  return 1;
}

int ed25519_check_public_key_s2n_bignum(const uint8_t public_key[ED25519_PUBLIC_KEY_LEN]) {
  uint64_t A[8] = {0};
  if (edwards25519_decode_selector(A, public_key) != 0) {
    return 0;
  }
  return 1;
}

#endif
