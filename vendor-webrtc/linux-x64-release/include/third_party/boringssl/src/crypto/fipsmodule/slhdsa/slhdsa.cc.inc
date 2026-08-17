// Copyright 2014 The BoringSSL Authors
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

#include <openssl/base.h>

#include <string.h>

#include <openssl/bytestring.h>
#include <openssl/obj.h>
#include <openssl/rand.h>

#include "../../internal.h"
#include "../bcm_interface.h"
#include "address.h"
#include "fors.h"
#include "merkle.h"
#include "params.h"
#include "thash.h"


namespace {

namespace fips {
void ensure_keygen_self_test();
void ensure_sign_self_test();
void ensure_verify_self_test();
}  // namespace fips

// The OBJECT IDENTIFIER header is also included in these values, per the spec.
const uint8_t kSHA256OID[] = {0x06, 0x09, 0x60, 0x86, 0x48, 0x01,
                              0x65, 0x03, 0x04, 0x02, 0x01};
const uint8_t kSHA384OID[] = {0x06, 0x09, 0x60, 0x86, 0x48, 0x01,
                              0x65, 0x03, 0x04, 0x02, 0x02};
#define MAX_OID_LENGTH 11
#define MAX_CONTEXT_LENGTH 255

bcm_infallible generate_key_from_seed_no_self_test(
    uint8_t out_public_key[BCM_SLHDSA_SHA2_128S_PUBLIC_KEY_BYTES],
    uint8_t out_secret_key[BCM_SLHDSA_SHA2_128S_PRIVATE_KEY_BYTES],
    const uint8_t seed[3 * BCM_SLHDSA_SHA2_128S_N]) {
  // Initialize SK.seed || SK.prf || PK.seed from seed.
  OPENSSL_memcpy(out_secret_key, seed, 3 * BCM_SLHDSA_SHA2_128S_N);

  // Initialize PK.seed from seed.
  OPENSSL_memcpy(out_public_key, seed + 2 * BCM_SLHDSA_SHA2_128S_N,
                 BCM_SLHDSA_SHA2_128S_N);

  uint8_t addr[32] = {0};
  slhdsa_set_layer_addr(addr, SLHDSA_SHA2_128S_D - 1);

  // Set PK.root
  slhdsa_treehash(out_public_key + BCM_SLHDSA_SHA2_128S_N, out_secret_key, 0,
                  SLHDSA_SHA2_128S_TREE_HEIGHT, out_public_key, addr);
  OPENSSL_memcpy(out_secret_key + 3 * BCM_SLHDSA_SHA2_128S_N,
                 out_public_key + BCM_SLHDSA_SHA2_128S_N,
                 BCM_SLHDSA_SHA2_128S_N);

  // FIPS 140-3 IG 10.3.A comment 1 says of the pair-wise consistency test for
  // SLH-DSA:
  //
  // "For key pairs generated for use with approved algorithms in SP 800-208 and
  // FIPS 205, the PCT (described by the tester in TE10.35.02) may be limited to
  // confirming the same key identifier (I in the case of LMS, SEED in the case
  // of XMSS and PK.SEED for SLH-DSA) is shared by the resulting public and
  // private key following generation."
  //
  // Since this is cheap, we always do this.

  if (boringssl_fips_break_test("SLHDSA_PWCT")) {
    out_public_key[0] ^= 1;
  }
  if (OPENSSL_memcmp(out_public_key,
                     out_secret_key + 2 * BCM_SLHDSA_SHA2_128S_N,
                     BCM_SLHDSA_SHA2_128S_N) != 0) {
    abort();
  }

  return bcm_infallible::not_approved;
}

// Note that this overreads by a byte. This is fine in the context that it's
// used.
uint64_t load_tree_index(const uint8_t in[8]) {
  static_assert(SLHDSA_SHA2_128S_TREE_BYTES == 7,
                "This code needs to be updated");
  uint64_t index = CRYPTO_load_u64_be(in);
  index >>= 8;
  index &= (~(uint64_t)0) >> (64 - SLHDSA_SHA2_128S_TREE_BITS);
  return index;
}

// Implements Algorithm 22: slh_sign function (Section 10.2.1, page 39)
bcm_infallible sign_internal_no_self_test(
    uint8_t out_signature[BCM_SLHDSA_SHA2_128S_SIGNATURE_BYTES],
    const uint8_t secret_key[BCM_SLHDSA_SHA2_128S_PRIVATE_KEY_BYTES],
    const uint8_t header[BCM_SLHDSA_M_PRIME_HEADER_LEN], const uint8_t *context,
    size_t context_len, const uint8_t *msg, size_t msg_len,
    const uint8_t entropy[BCM_SLHDSA_SHA2_128S_N]) {
  const uint8_t *sk_seed = secret_key;
  const uint8_t *sk_prf = secret_key + BCM_SLHDSA_SHA2_128S_N;
  const uint8_t *pk_seed = secret_key + 2 * BCM_SLHDSA_SHA2_128S_N;
  const uint8_t *pk_root = secret_key + 3 * BCM_SLHDSA_SHA2_128S_N;

  // Derive randomizer R and copy it to signature
  uint8_t R[BCM_SLHDSA_SHA2_128S_N];
  slhdsa_thash_prfmsg(R, sk_prf, entropy, header, context, context_len, msg,
                      msg_len);
  OPENSSL_memcpy(out_signature, R, BCM_SLHDSA_SHA2_128S_N);

  // Compute message digest
  uint8_t digest[SLHDSA_SHA2_128S_DIGEST_SIZE];
  slhdsa_thash_hmsg(digest, R, pk_seed, pk_root, header, context, context_len,
                    msg, msg_len);

  uint8_t fors_digest[SLHDSA_SHA2_128S_FORS_MSG_BYTES];
  OPENSSL_memcpy(fors_digest, digest, SLHDSA_SHA2_128S_FORS_MSG_BYTES);

  const uint64_t idx_tree =
      load_tree_index(digest + SLHDSA_SHA2_128S_FORS_MSG_BYTES);
  uint32_t idx_leaf = CRYPTO_load_u16_be(
      digest + SLHDSA_SHA2_128S_FORS_MSG_BYTES + SLHDSA_SHA2_128S_TREE_BYTES);
  idx_leaf &= (~(uint32_t)0) >> (32 - SLHDSA_SHA2_128S_LEAF_BITS);

  uint8_t addr[32] = {0};
  slhdsa_set_tree_addr(addr, idx_tree);
  slhdsa_set_type(addr, SLHDSA_SHA2_128S_ADDR_TYPE_FORSTREE);
  slhdsa_set_keypair_addr(addr, idx_leaf);

  slhdsa_fors_sign(out_signature + BCM_SLHDSA_SHA2_128S_N, fors_digest, sk_seed,
                   pk_seed, addr);

  uint8_t pk_fors[BCM_SLHDSA_SHA2_128S_N];
  slhdsa_fors_pk_from_sig(pk_fors, out_signature + BCM_SLHDSA_SHA2_128S_N,
                          fors_digest, pk_seed, addr);

  slhdsa_ht_sign(
      out_signature + BCM_SLHDSA_SHA2_128S_N + SLHDSA_SHA2_128S_FORS_BYTES,
      pk_fors, idx_tree, idx_leaf, sk_seed, pk_seed);
  return bcm_infallible::approved;
}

bcm_status verify_internal(
    const uint8_t *signature, size_t signature_len,
    const uint8_t public_key[BCM_SLHDSA_SHA2_128S_PUBLIC_KEY_BYTES],
    const uint8_t header[BCM_SLHDSA_M_PRIME_HEADER_LEN], const uint8_t *context,
    size_t context_len, const uint8_t *msg, size_t msg_len) {
  if (signature_len != BCM_SLHDSA_SHA2_128S_SIGNATURE_BYTES) {
    return bcm_status::failure;
  }
  const uint8_t *pk_seed = public_key;
  const uint8_t *pk_root = public_key + BCM_SLHDSA_SHA2_128S_N;

  const uint8_t *r = signature;
  const uint8_t *sig_fors = signature + BCM_SLHDSA_SHA2_128S_N;
  const uint8_t *sig_ht = sig_fors + SLHDSA_SHA2_128S_FORS_BYTES;

  uint8_t digest[SLHDSA_SHA2_128S_DIGEST_SIZE];
  slhdsa_thash_hmsg(digest, r, pk_seed, pk_root, header, context, context_len,
                    msg, msg_len);

  uint8_t fors_digest[SLHDSA_SHA2_128S_FORS_MSG_BYTES];
  OPENSSL_memcpy(fors_digest, digest, SLHDSA_SHA2_128S_FORS_MSG_BYTES);

  const uint64_t idx_tree =
      load_tree_index(digest + SLHDSA_SHA2_128S_FORS_MSG_BYTES);
  uint32_t idx_leaf = CRYPTO_load_u16_be(
      digest + SLHDSA_SHA2_128S_FORS_MSG_BYTES + SLHDSA_SHA2_128S_TREE_BYTES);
  idx_leaf &= (~(uint32_t)0) >> (32 - SLHDSA_SHA2_128S_LEAF_BITS);

  uint8_t addr[32] = {0};
  slhdsa_set_tree_addr(addr, idx_tree);
  slhdsa_set_type(addr, SLHDSA_SHA2_128S_ADDR_TYPE_FORSTREE);
  slhdsa_set_keypair_addr(addr, idx_leaf);

  uint8_t pk_fors[BCM_SLHDSA_SHA2_128S_N];
  slhdsa_fors_pk_from_sig(pk_fors, sig_fors, fors_digest, pk_seed, addr);

  if (!slhdsa_ht_verify(sig_ht, pk_fors, idx_tree, idx_leaf, pk_root,
                        pk_seed)) {
    return bcm_status::failure;
  }

  return bcm_status::approved;
}

namespace fips {

#include "fips_known_values.inc"

static int keygen_self_test() {
  uint8_t seed[3 * BCM_SLHDSA_SHA2_128S_N] = {0};
  uint8_t pub[BCM_SLHDSA_SHA2_128S_PUBLIC_KEY_BYTES];
  uint8_t priv[BCM_SLHDSA_SHA2_128S_PRIVATE_KEY_BYTES];
  generate_key_from_seed_no_self_test(pub, priv, seed);

  static_assert(sizeof(kExpectedPublicKey) == sizeof(pub));
  static_assert(sizeof(kExpectedPrivateKey) == sizeof(priv));
  if (!BORINGSSL_check_test(kExpectedPublicKey, pub, sizeof(pub),
                            "SLH-DSA public key") ||
      !BORINGSSL_check_test(kExpectedPrivateKey, priv, sizeof(priv),
                            "SLH-DSA private key")) {
    return 0;
  }

  return 1;
}

static int sign_self_test() {
  uint8_t header[BCM_SLHDSA_M_PRIME_HEADER_LEN] = {0};
  uint8_t entropy[BCM_SLHDSA_SHA2_128S_N] = {0};
  uint8_t sig[BCM_SLHDSA_SHA2_128S_SIGNATURE_BYTES];
  sign_internal_no_self_test(sig, kExpectedPrivateKey, header, nullptr, 0,
                             nullptr, 0, entropy);
  uint8_t digest[32];
  SHA256(sig, sizeof(sig), digest);

  static_assert(sizeof(kExpectedSignatureSHA256) == sizeof(digest));
  if (!BORINGSSL_check_test(kExpectedSignatureSHA256, digest, sizeof(digest),
                            "SLH-DSA signature")) {
    return 0;
  }

  return 1;
}

static int verify_self_test() {
  uint8_t header[BCM_SLHDSA_M_PRIME_HEADER_LEN] = {0};
  return verify_internal(kExpectedSignature, sizeof(kExpectedSignature),
                         kExpectedPublicKey, header, nullptr, 0, nullptr,
                         0) == bcm_status::approved;
}

#if defined(BORINGSSL_FIPS)

DEFINE_STATIC_ONCE(g_slhdsa_keygen_self_test_once)

void ensure_keygen_self_test(void) {
  CRYPTO_once(g_slhdsa_keygen_self_test_once_bss_get(), []() {
    if (!keygen_self_test()) {
      BORINGSSL_FIPS_abort();
    }
  });
}

DEFINE_STATIC_ONCE(g_slhdsa_sign_self_test_once)

void ensure_sign_self_test(void) {
  CRYPTO_once(g_slhdsa_sign_self_test_once_bss_get(), []() {
    if (!sign_self_test()) {
      BORINGSSL_FIPS_abort();
    }
  });
}

DEFINE_STATIC_ONCE(g_slhdsa_verify_self_test_once)

void ensure_verify_self_test(void) {
  CRYPTO_once(g_slhdsa_verify_self_test_once_bss_get(), []() {
    if (!verify_self_test()) {
      BORINGSSL_FIPS_abort();
    }
  });
}

#else

void ensure_keygen_self_test(void) {}
void ensure_sign_self_test(void) {}
void ensure_verify_self_test(void) {}

#endif

}  // namespace fips

}  // namespace

bcm_infallible BCM_slhdsa_sha2_128s_generate_key_from_seed(
    uint8_t out_public_key[BCM_SLHDSA_SHA2_128S_PUBLIC_KEY_BYTES],
    uint8_t out_secret_key[BCM_SLHDSA_SHA2_128S_PRIVATE_KEY_BYTES],
    const uint8_t seed[3 * BCM_SLHDSA_SHA2_128S_N]) {
  fips::ensure_keygen_self_test();
  return generate_key_from_seed_no_self_test(out_public_key, out_secret_key,
                                             seed);
}

bcm_status BCM_slhdsa_sha2_128s_generate_key_from_seed_fips(
    uint8_t out_public_key[BCM_SLHDSA_SHA2_128S_PUBLIC_KEY_BYTES],
    uint8_t out_secret_key[BCM_SLHDSA_SHA2_128S_PRIVATE_KEY_BYTES],
    const uint8_t seed[3 * BCM_SLHDSA_SHA2_128S_N]) {
  if (out_public_key == nullptr || out_secret_key == nullptr) {
    return bcm_status::failure;
  }
  BCM_slhdsa_sha2_128s_generate_key_from_seed(out_public_key, out_secret_key,
                                              seed);
  return bcm_status::approved;
}

bcm_infallible BCM_slhdsa_sha2_128s_generate_key(
    uint8_t out_public_key[BCM_SLHDSA_SHA2_128S_PUBLIC_KEY_BYTES],
    uint8_t out_private_key[BCM_SLHDSA_SHA2_128S_PRIVATE_KEY_BYTES]) {
  uint8_t seed[3 * BCM_SLHDSA_SHA2_128S_N];
  RAND_bytes(seed, 3 * BCM_SLHDSA_SHA2_128S_N);
  return BCM_slhdsa_sha2_128s_generate_key_from_seed(out_public_key,
                                                     out_private_key, seed);
}

bcm_status BCM_slhdsa_sha2_128s_generate_key_fips(
    uint8_t out_public_key[BCM_SLHDSA_SHA2_128S_PUBLIC_KEY_BYTES],
    uint8_t out_private_key[BCM_SLHDSA_SHA2_128S_PRIVATE_KEY_BYTES]) {
  if (out_public_key == nullptr || out_private_key == nullptr) {
    return bcm_status::failure;
  }
  BCM_slhdsa_sha2_128s_generate_key(out_public_key, out_private_key);
  return bcm_status::approved;
}

bcm_infallible BCM_slhdsa_sha2_128s_public_from_private(
    uint8_t out_public_key[BCM_SLHDSA_SHA2_128S_PUBLIC_KEY_BYTES],
    const uint8_t private_key[BCM_SLHDSA_SHA2_128S_PRIVATE_KEY_BYTES]) {
  OPENSSL_memcpy(out_public_key, private_key + 2 * BCM_SLHDSA_SHA2_128S_N,
                 BCM_SLHDSA_SHA2_128S_N * 2);
  return bcm_infallible::approved;
}

bcm_status BCM_slhdsa_sha2_128s_sign(
    uint8_t out_signature[BCM_SLHDSA_SHA2_128S_SIGNATURE_BYTES],
    const uint8_t private_key[BCM_SLHDSA_SHA2_128S_PRIVATE_KEY_BYTES],
    const uint8_t *msg, size_t msg_len, const uint8_t *context,
    size_t context_len) {
  if (context_len > MAX_CONTEXT_LENGTH) {
    return bcm_status::failure;
  }

  // Construct header for M' as specified in Algorithm 22
  uint8_t M_prime_header[2];
  M_prime_header[0] = 0;  // domain separator for pure signing
  M_prime_header[1] = (uint8_t)context_len;

  uint8_t entropy[BCM_SLHDSA_SHA2_128S_N];
  RAND_bytes(entropy, sizeof(entropy));
  BCM_slhdsa_sha2_128s_sign_internal(out_signature, private_key, M_prime_header,
                                     context, context_len, msg, msg_len,
                                     entropy);
  return bcm_status::approved;
}

static int slhdsa_get_context_and_oid(uint8_t *out_context_and_oid,
                                      size_t *out_context_and_oid_len,
                                      size_t max_out_context_and_oid,
                                      const uint8_t *context,
                                      size_t context_len, int hash_nid,
                                      size_t hashed_msg_len) {
  const uint8_t *oid;
  size_t oid_len;
  size_t expected_hash_len;
  switch (hash_nid) {
    case NID_sha256:
      oid = kSHA256OID;
      oid_len = sizeof(kSHA256OID);
      static_assert(sizeof(kSHA256OID) <= MAX_OID_LENGTH, "");
      expected_hash_len = 32;
      break;

    // The SLH-DSA spec only lists SHA-256 and SHA-512. This function also
    // supports SHA-384, which is non-standard.
    case NID_sha384:
      oid = kSHA384OID;
      oid_len = sizeof(kSHA384OID);
      static_assert(sizeof(kSHA384OID) <= MAX_OID_LENGTH, "");
      expected_hash_len = 48;
      break;

    // If adding a hash function with a larger `oid_len`, update the size of
    // `context_and_oid` in the callers.
    default:
      return 0;
  }

  if (hashed_msg_len != expected_hash_len) {
    return 0;
  }

  *out_context_and_oid_len = context_len + oid_len;
  if (*out_context_and_oid_len > max_out_context_and_oid) {
    return 0;
  }

  OPENSSL_memcpy(out_context_and_oid, context, context_len);
  OPENSSL_memcpy(out_context_and_oid + context_len, oid, oid_len);

  return 1;
}

bcm_infallible BCM_slhdsa_sha2_128s_sign_internal(
    uint8_t out_signature[BCM_SLHDSA_SHA2_128S_SIGNATURE_BYTES],
    const uint8_t secret_key[BCM_SLHDSA_SHA2_128S_PRIVATE_KEY_BYTES],
    const uint8_t header[BCM_SLHDSA_M_PRIME_HEADER_LEN], const uint8_t *context,
    size_t context_len, const uint8_t *msg, size_t msg_len,
    const uint8_t entropy[BCM_SLHDSA_SHA2_128S_N]) {
  fips::ensure_sign_self_test();
  return sign_internal_no_self_test(out_signature, secret_key, header, context,
                                    context_len, msg, msg_len, entropy);
}

bcm_status BCM_slhdsa_sha2_128s_prehash_sign(
    uint8_t out_signature[BCM_SLHDSA_SHA2_128S_SIGNATURE_BYTES],
    const uint8_t private_key[BCM_SLHDSA_SHA2_128S_PRIVATE_KEY_BYTES],
    const uint8_t *hashed_msg, size_t hashed_msg_len, int hash_nid,
    const uint8_t *context, size_t context_len) {
  if (context_len > MAX_CONTEXT_LENGTH) {
    return bcm_status::failure;
  }

  uint8_t M_prime_header[2];
  M_prime_header[0] = 1;  // domain separator for prehashed signing
  M_prime_header[1] = (uint8_t)context_len;

  uint8_t context_and_oid[MAX_CONTEXT_LENGTH + MAX_OID_LENGTH];
  size_t context_and_oid_len;
  if (!slhdsa_get_context_and_oid(context_and_oid, &context_and_oid_len,
                                  sizeof(context_and_oid), context, context_len,
                                  hash_nid, hashed_msg_len)) {
    return bcm_status::failure;
  }

  uint8_t entropy[BCM_SLHDSA_SHA2_128S_N];
  RAND_bytes(entropy, sizeof(entropy));
  BCM_slhdsa_sha2_128s_sign_internal(out_signature, private_key, M_prime_header,
                                     context_and_oid, context_and_oid_len,
                                     hashed_msg, hashed_msg_len, entropy);
  return bcm_status::approved;
}

// Implements Algorithm 24: slh_verify function (Section 10.3, page 41)
bcm_status BCM_slhdsa_sha2_128s_verify(
    const uint8_t *signature, size_t signature_len,
    const uint8_t public_key[BCM_SLHDSA_SHA2_128S_PUBLIC_KEY_BYTES],
    const uint8_t *msg, size_t msg_len, const uint8_t *context,
    size_t context_len) {
  if (context_len > MAX_CONTEXT_LENGTH) {
    return bcm_status::failure;
  }

  // Construct header for M' as specified in Algorithm 24
  uint8_t M_prime_header[2];
  M_prime_header[0] = 0;  // domain separator for pure verification
  M_prime_header[1] = (uint8_t)context_len;

  return BCM_slhdsa_sha2_128s_verify_internal(
      signature, signature_len, public_key, M_prime_header, context,
      context_len, msg, msg_len);
}

bcm_status BCM_slhdsa_sha2_128s_prehash_verify(
    const uint8_t *signature, size_t signature_len,
    const uint8_t public_key[BCM_SLHDSA_SHA2_128S_PUBLIC_KEY_BYTES],
    const uint8_t *hashed_msg, size_t hashed_msg_len, int hash_nid,
    const uint8_t *context, size_t context_len) {
  if (context_len > MAX_CONTEXT_LENGTH) {
    return bcm_status::failure;
  }

  uint8_t M_prime_header[2];
  M_prime_header[0] = 1;  // domain separator for prehashed verification
  M_prime_header[1] = (uint8_t)context_len;

  uint8_t context_and_oid[MAX_CONTEXT_LENGTH + MAX_OID_LENGTH];
  size_t context_and_oid_len;
  if (!slhdsa_get_context_and_oid(context_and_oid, &context_and_oid_len,
                                  sizeof(context_and_oid), context, context_len,
                                  hash_nid, hashed_msg_len)) {
    return bcm_status::failure;
  }

  return BCM_slhdsa_sha2_128s_verify_internal(
      signature, signature_len, public_key, M_prime_header, context_and_oid,
      context_and_oid_len, hashed_msg, hashed_msg_len);
}

bcm_status BCM_slhdsa_sha2_128s_verify_internal(
    const uint8_t *signature, size_t signature_len,
    const uint8_t public_key[BCM_SLHDSA_SHA2_128S_PUBLIC_KEY_BYTES],
    const uint8_t header[BCM_SLHDSA_M_PRIME_HEADER_LEN], const uint8_t *context,
    size_t context_len, const uint8_t *msg, size_t msg_len) {
  fips::ensure_verify_self_test();
  return verify_internal(signature, signature_len, public_key, header, context,
                         context_len, msg, msg_len);
}

int boringssl_self_test_slhdsa() {
  return fips::keygen_self_test() && fips::sign_self_test() &&
         fips::verify_self_test();
}
