// Copyright (c) 2017, Google Inc.
// SPDX-License-Identifier: ISC

/* test_fips exercises various cryptographic primitives for demonstration
 * purposes in the validation process only. */

#include <stdio.h>

#include <openssl/aead.h>
#include <openssl/aes.h>
#include <openssl/bn.h>
#include <openssl/crypto.h>
#include <openssl/ctrdrbg.h>
#include <openssl/curve25519.h>
#include <openssl/dh.h>
#include <openssl/ec_key.h>
#include <openssl/ecdsa.h>
#include <openssl/evp.h>
#include <openssl/hmac.h>
#include <openssl/kdf.h>
#include <openssl/nid.h>
#include <openssl/rsa.h>
#include <openssl/sha.h>

#include "../../crypto/fipsmodule/evp/internal.h"
#include "../../crypto/fipsmodule/kem/internal.h"
#include "../../crypto/fipsmodule/pqdsa/internal.h"
#include "../../crypto/fipsmodule/rand/internal.h"
#include "../../crypto/internal.h"


static void hexdump(const void *a, size_t len) {
  const unsigned char *in = (const unsigned char *)a;
  for (size_t i = 0; i < len; i++) {
    printf("%02x", in[i]);
  }

  printf("\n");
}

int main(int argc, char **argv) {
  CRYPTO_library_init();

  static const uint8_t kAESKey[16] = {'B', 'o', 'r', 'i', 'n', 'g', 'C', 'r',
                                      'y', 'p', 't', 'o', ' ', 'K', 'e', 'y'};

  static const uint8_t kPlaintext[64] = {
      'B', 'o', 'r', 'i', 'n', 'g', 'C', 'r', 'y', 'p', 't', 'o', 'M',
      'o', 'd', 'u', 'l', 'e', ' ', 'F', 'I', 'P', 'S', ' ', 'K', 'A',
      'T', ' ', 'E', 'n', 'c', 'r', 'y', 'p', 't', 'i', 'o', 'n', ' ',
      'a', 'n', 'd', ' ', 'D', 'e', 'c', 'r', 'y', 'p', 't', 'i', 'o',
      'n', ' ', 'P', 'l', 'a', 'i', 'n', 't', 'e', 'x', 't', '!'};

  static const uint8_t kPlaintextSHA256[32] = {
      0x37, 0xbd, 0x70, 0x53, 0x72, 0xfc, 0xd4, 0x03, 0x79, 0x70, 0xfb,
      0x06, 0x95, 0xb1, 0x2a, 0x82, 0x48, 0xe1, 0x3e, 0xf2, 0x33, 0xfb,
      0xef, 0x29, 0x81, 0x22, 0x45, 0x40, 0x43, 0x70, 0xce, 0x0f};
  const uint8_t kDRBGEntropy[48] = {
      'D', 'B', 'R', 'G', ' ', 'I', 'n', 'i', 't', 'i', 'a', 'l',
      ' ', 'E', 'n', 't', 'r', 'o', 'p', 'y', ' ', ' ', ' ', ' ',
      ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
      ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '};

  const uint8_t kDRBGPersonalization[18] = {'B', 'C', 'M', 'P', 'e', 'r',
                                            's', 'o', 'n', 'a', 'l', 'i',
                                            'z', 'a', 't', 'i', 'o', 'n'};

  const uint8_t kDRBGAD[16] = {'B', 'C', 'M', ' ', 'D', 'R', 'B', 'G',
                               ' ', 'A', 'D', ' ', ' ', ' ', ' ', ' '};

  const uint8_t kDRBGEntropy2[48] = {
      'D', 'B', 'R', 'G', ' ', 'R', 'e', 's', 'e', 'e', 'd', ' ',
      'E', 'n', 't', 'r', 'o', 'p', 'y', ' ', ' ', ' ', ' ', ' ',
      ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
      ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '};

  AES_KEY aes_key;
  uint8_t aes_iv[16];
  uint8_t output[256];

  /* AES-CBC Encryption */
  OPENSSL_memset(aes_iv, 0, sizeof(aes_iv));
  if (AES_set_encrypt_key(kAESKey, 8 * sizeof(kAESKey), &aes_key) != 0) {
    printf("AES_set_encrypt_key failed\n");
    goto err;
  }

  printf("About to AES-CBC encrypt ");
  hexdump(kPlaintext, sizeof(kPlaintext));
  AES_cbc_encrypt(kPlaintext, output, sizeof(kPlaintext), &aes_key, aes_iv,
                  AES_ENCRYPT);
  printf("  got ");
  hexdump(output, sizeof(kPlaintext));

  /* AES-CBC Decryption */
  OPENSSL_memset(aes_iv, 0, sizeof(aes_iv));
  if (AES_set_decrypt_key(kAESKey, 8 * sizeof(kAESKey), &aes_key) != 0) {
    printf("AES decrypt failed\n");
    goto err;
  }
  printf("About to AES-CBC decrypt ");
  hexdump(output, sizeof(kPlaintext));
  AES_cbc_encrypt(output, output, sizeof(kPlaintext), &aes_key, aes_iv,
                  AES_DECRYPT);
  printf("  got ");
  hexdump(output, sizeof(kPlaintext));

  OPENSSL_cleanse(&aes_key, sizeof(aes_key));

  /* AES-GCM */
  size_t out_len;
  uint8_t nonce[EVP_AEAD_MAX_NONCE_LENGTH];
  OPENSSL_memset(nonce, 0, sizeof(nonce));
  EVP_AEAD_CTX aead_ctx;
  if (!EVP_AEAD_CTX_init(&aead_ctx, EVP_aead_aes_128_gcm(), kAESKey,
                         sizeof(kAESKey), 0, NULL)) {
    printf("EVP_AEAD_CTX_init failed\n");
    goto err;
  }

  /* AES-GCM Encryption */
  OPENSSL_memset(aes_iv, 0, sizeof(aes_iv));
  if (AES_set_encrypt_key(kAESKey, 8 * sizeof(kAESKey), &aes_key) != 0) {
    printf("AES_set_encrypt_key failed\n");
    goto err;
  }
  printf("About to AES-GCM seal ");
  hexdump(output, sizeof(kPlaintext));
  if (!EVP_AEAD_CTX_seal(&aead_ctx, output, &out_len, sizeof(output), nonce,
                         EVP_AEAD_nonce_length(EVP_aead_aes_128_gcm()),
                         kPlaintext, sizeof(kPlaintext), NULL, 0)) {
    printf("AES-GCM encrypt failed\n");
    goto err;
  }
  printf("  got ");
  hexdump(output, out_len);

  /* AES-GCM Decryption */
  OPENSSL_memset(aes_iv, 0, sizeof(aes_iv));
  printf("About to AES-GCM open ");
  hexdump(output, out_len);
  if (!EVP_AEAD_CTX_open(&aead_ctx, output, &out_len, sizeof(output), nonce,
                         EVP_AEAD_nonce_length(EVP_aead_aes_128_gcm()),
                         output, out_len, NULL, 0)) {
    printf("AES-GCM decrypt failed\n");
    goto err;
  }
  printf("  got ");
  hexdump(output, out_len);

  EVP_AEAD_CTX_zero(&aead_ctx);
  EVP_AEAD_CTX_cleanup(&aead_ctx);

  /* AES-CCM */
  OPENSSL_memset(nonce, 0, sizeof(nonce));
  if (!EVP_AEAD_CTX_init(&aead_ctx, EVP_aead_aes_128_ccm_bluetooth(), kAESKey, sizeof(kAESKey), 0, NULL)) {
    fprintf(stderr, "EVP_AED_CTX_init for AES-128-CCM failed.\n");
    goto err;
  }

  /* AES-CCM Encryption */
  OPENSSL_memset(aes_iv, 0, sizeof(aes_iv));
  printf("About to AES-CCM seal ");
  hexdump(kPlaintext, sizeof(kPlaintext));
  if (!EVP_AEAD_CTX_seal(&aead_ctx, output, &out_len, sizeof(output), nonce,
                        EVP_AEAD_nonce_length(EVP_aead_aes_128_ccm_bluetooth()),
                        kPlaintext, sizeof(kPlaintext), NULL, 0)) {
    fprintf(stderr, "EVP_AEAD_CTX_seal for AES-128-CCM failed.\n");
    goto err;
  }
  printf("  got ");
  hexdump(output, out_len);

  /* AES-CCM Decryption */
  OPENSSL_memset(aes_iv, 0, sizeof(aes_iv));
  if (AES_set_decrypt_key(kAESKey, 8 * sizeof(kAESKey), &aes_key) != 0) {
    printf("AES decrypt failed\n");
    goto err;
  }
  printf("About to AES-CCM open ");
  hexdump(output, out_len);
  if (!EVP_AEAD_CTX_open(&aead_ctx, output, &out_len, sizeof(output), nonce,
                        EVP_AEAD_nonce_length(EVP_aead_aes_128_ccm_bluetooth()),
                        output, out_len, NULL, 0)) {
    fprintf(stderr, "EVP_AEAD_CTX_open for AES-128-CCM failed.\n");
    goto err;
  }
  printf("  got ");
  hexdump(output, out_len);
  
  OPENSSL_cleanse(&aes_key, sizeof(aes_key));
  EVP_AEAD_CTX_zero(&aead_ctx);

  /* AES-ECB */
  /* AES-ECB Encryption */
  OPENSSL_memset(aes_iv, 0, sizeof(aes_iv));
  if (AES_set_encrypt_key(kAESKey, 8 * sizeof(kAESKey), &aes_key) != 0) {
    fprintf(stderr, "AES_set_encrypt_key failed.\n");
    goto err;
  }
  printf("About to AES-ECB encrypt ");
  hexdump(kPlaintext, sizeof(kPlaintext));
  for (size_t j = 0; j < sizeof(kPlaintext) / 16; j++) {
    AES_ecb_encrypt(&kPlaintext[j * 16], &output[j * 16], &aes_key, AES_ENCRYPT);
  }
  printf("  got ");
  hexdump(output, sizeof(kPlaintext));

  /* AES-ECB Decryption */
  OPENSSL_memset(aes_iv, 0, sizeof(aes_iv));
  if (AES_set_decrypt_key(kAESKey, 8 * sizeof(kAESKey), &aes_key) != 0) {
    printf("AES decrypt failed\n");
    goto err;
  }
  printf("About to AES-ECB decrypt ");
  hexdump(output, sizeof(kPlaintext));
  for (size_t j = 0; j < sizeof(kPlaintext) / 16; j++) {
    AES_ecb_encrypt(&output[j * 16], &output[j * 16], &aes_key, AES_DECRYPT);
  }
  printf("  got ");
  hexdump(output, sizeof(kPlaintext));

  OPENSSL_cleanse(&aes_key, sizeof(aes_key));

  unsigned int num = 0;
  uint8_t ecount_buf[128];

  /* AES-CTR */
  /* AES-CTR Encryption */
  OPENSSL_memset(aes_iv, 0, sizeof(aes_iv));
  if (AES_set_encrypt_key(kAESKey, 8 * sizeof(kAESKey), &aes_key) != 0) {
    fprintf(stderr, "AES_set_encrypt_key failed.\n");
    goto err;
  }
  printf("About to AES-CTR Encrypt ");
  hexdump(kPlaintext, sizeof(kPlaintext));
  AES_ctr128_encrypt(kPlaintext, output, sizeof(kPlaintext), &aes_key, aes_iv, ecount_buf, &num);
  printf("  got ");
  hexdump(output, sizeof(kPlaintext));


  /* AES-CTR Decryption */
  OPENSSL_memset(aes_iv, 0, sizeof(aes_iv));
  printf("About to AES-CTR Decrypt ");
  hexdump(output, sizeof(kPlaintext));
  AES_ctr128_encrypt(output, output, sizeof(kPlaintext), &aes_key, aes_iv, ecount_buf, &num);
  printf("  got ");
  hexdump(output, sizeof(kPlaintext));

  OPENSSL_cleanse(&aes_key, sizeof(aes_key));

  /* AES-KW Wrap */
  if (AES_set_encrypt_key(kAESKey, 8 * sizeof(kAESKey), &aes_key) != 0) {
    fprintf(stderr, "AES_set_encrypt_key failed.\n");
    goto err;
  }
  printf("About to AES-KW Wrap ");
  hexdump(kPlaintext, sizeof(kPlaintext));
  out_len = AES_wrap_key(&aes_key, NULL, output, kPlaintext, sizeof(kPlaintext));
  printf("  got ");
  hexdump(output, out_len);

  /* AES-KW Unwrap */
  if (AES_set_decrypt_key(kAESKey, 8 * sizeof(kAESKey), &aes_key) != 0) {
    fprintf(stderr, "AES decrypt failed.\n");
    goto err;
  }
  printf("About to AES-KW Unwrap ");
  hexdump(output, out_len);
  out_len = AES_unwrap_key(&aes_key, NULL, output, output, out_len);
  printf("  got ");
  hexdump(output, out_len);

  OPENSSL_cleanse(&aes_key, sizeof(aes_key));

  /* SHA-1 */
  printf("About to SHA-1 hash ");
  hexdump(kPlaintext, sizeof(kPlaintext));
  SHA1(kPlaintext, sizeof(kPlaintext), output);
  printf("  got ");
  hexdump(output, SHA_DIGEST_LENGTH);

  /* SHA-256 */
  printf("About to SHA-256 hash ");
  hexdump(kPlaintext, sizeof(kPlaintext));
  SHA256(kPlaintext, sizeof(kPlaintext), output);
  printf("  got ");
  hexdump(output, SHA256_DIGEST_LENGTH);

  /* SHA-512 */
  printf("About to SHA-512 hash ");
  hexdump(kPlaintext, sizeof(kPlaintext));
  SHA512(kPlaintext, sizeof(kPlaintext), output);
  printf("  got ");
  hexdump(output, SHA512_DIGEST_LENGTH);

  RSA *rsa_key = RSA_new();
  printf("About to generate RSA key\n");
  if (!RSA_generate_key_fips(rsa_key, 2048, NULL)) {
    printf("RSA_generate_key_fips failed\n");
    goto err;
  }

  /* RSA Sign */
  unsigned sig_len;
  printf("About to RSA sign ");
  hexdump(kPlaintextSHA256, sizeof(kPlaintextSHA256));
  if (!RSA_sign(NID_sha256, kPlaintextSHA256, sizeof(kPlaintextSHA256), output,
                &sig_len, rsa_key)) {
    printf("RSA Sign failed\n");
    goto err;
  }
  printf("  got ");
  hexdump(output, sig_len);

  /* RSA Verify */
  printf("About to RSA verify ");
  hexdump(output, sig_len);
  if (!RSA_verify(NID_sha256, kPlaintextSHA256, sizeof(kPlaintextSHA256),
                  output, sig_len, rsa_key)) {
    printf("RSA Verify failed.\n");
    goto err;
  }

  RSA_free(rsa_key);

  EC_KEY *ec_key = EC_KEY_new_by_curve_name(NID_X9_62_prime256v1);
  if (ec_key == NULL) {
    printf("invalid ECDSA key\n");
    goto err;
  }

  printf("About to generate P-256 key\n");
  if (!EC_KEY_generate_key_fips(ec_key)) {
    printf("EC_KEY_generate_key_fips failed\n");
    goto err;
  }

  /* Primitive Z Computation */
  const EC_GROUP *const ec_group = EC_KEY_get0_group(ec_key);
  EC_POINT *z_point = EC_POINT_new(ec_group);
  uint8_t z_result[65];
  printf("About to compute key-agreement Z with P-256:\n");
  if (!EC_POINT_mul(ec_group, z_point, NULL, EC_KEY_get0_public_key(ec_key),
                    EC_KEY_get0_private_key(ec_key), NULL) ||
      EC_POINT_point2oct(ec_group, z_point, POINT_CONVERSION_UNCOMPRESSED,
                         z_result, sizeof(z_result),
                         NULL) != sizeof(z_result)) {
    fprintf(stderr, "EC_POINT_mul failed.\n");
    goto err;
  }
  EC_POINT_free(z_point);

  printf("  got ");
  hexdump(z_result, sizeof(z_result));

  /* ECDSA Sign/Verify PWCT */
  printf("About to ECDSA sign ");
  hexdump(kPlaintextSHA256, sizeof(kPlaintextSHA256));
  ECDSA_SIG *sig =
      ECDSA_do_sign(kPlaintextSHA256, sizeof(kPlaintextSHA256), ec_key);
  if (sig == NULL ||
      !ECDSA_do_verify(kPlaintextSHA256, sizeof(kPlaintextSHA256), sig,
                       ec_key)) {
    printf("ECDSA Sign/Verify PWCT failed.\n");
    goto err;
  }

  ECDSA_SIG_free(sig);
  EC_KEY_free(ec_key);

  /* Ed25519 */
  printf("About to Ed25519 sign ");
  hexdump(kPlaintextSHA256, sizeof(kPlaintextSHA256));
  uint8_t ed_public_key[ED25519_PUBLIC_KEY_LEN];
  uint8_t ed_private_key[ED25519_PRIVATE_KEY_LEN];
  ED25519_keypair(ed_public_key, ed_private_key);
  uint8_t ed_signature[ED25519_SIGNATURE_LEN];
  if (!ED25519_sign(ed_signature,kPlaintextSHA256, sizeof(kPlaintextSHA256), ed_private_key) ||
    !ED25519_verify(kPlaintextSHA256, sizeof(kPlaintextSHA256), ed_signature, ed_public_key)) {
    printf("ED25519 Sign/Verify PWCT failed.\n");
    goto err;
  }
  printf("got signature ");
  hexdump(ed_signature, sizeof(ed_signature));

  /* Ed25519ph */
  printf("About to Ed25519ph sign ");
  hexdump(kPlaintextSHA256, sizeof(kPlaintextSHA256));
  uint8_t ed25519_ph_context[32] = {
    0xfe, 0x52, 0xbb, 0xd2, 0x45, 0x54, 0x46, 0xad, 0xa5, 0x24, 0x6b, 0x5a,
    0xf3, 0xba, 0x82, 0x93, 0x9c, 0xed, 0xa6, 0xa1, 0x8f, 0x59, 0xd3, 0x37,
    0x48, 0xde, 0x40, 0x7a, 0xfe, 0x31, 0x48, 0xd1
  };
  if (!ED25519ph_sign(ed_signature, kPlaintextSHA256, sizeof(kPlaintextSHA256), ed_private_key, ed25519_ph_context, sizeof(ed25519_ph_context)) ||
    !ED25519ph_verify(kPlaintextSHA256, sizeof(kPlaintextSHA256), ed_signature, ed_public_key, ed25519_ph_context, sizeof(ed25519_ph_context))) {
    printf("ED25519ph Sign/Verify PWCT failed.\n");
    goto err;
  }
  printf("got signature ");
  hexdump(ed_signature, sizeof(ed_signature));

  /* ML-KEM */
  printf("About to Generate ML-KEM key\n");
  EVP_PKEY *kem_raw = NULL;
  EVP_PKEY_CTX *kem_ctx = EVP_PKEY_CTX_new_id(EVP_PKEY_KEM, NULL);
  if (kem_ctx == NULL || !EVP_PKEY_CTX_kem_set_params(kem_ctx, NID_MLKEM512) ||
    !EVP_PKEY_keygen_init(kem_ctx) ||
    !EVP_PKEY_keygen(kem_ctx, &kem_raw)) {
    printf("ML-KEM keygen failed.\n");
    goto err;
  }
  printf("Generated public key: ");
  hexdump(kem_raw->pkey.kem_key->public_key, kem_raw->pkey.kem_key->kem->public_key_len);
  EVP_PKEY_free(kem_raw);
  EVP_PKEY_CTX_free(kem_ctx);

  /* ML-DSA */
  printf("About to Generate ML-DSA key\n");
  EVP_PKEY *dsa_raw = NULL;
  EVP_PKEY_CTX *dsa_ctx = EVP_PKEY_CTX_new_id(EVP_PKEY_PQDSA, NULL);
  if (dsa_ctx == NULL || !EVP_PKEY_CTX_pqdsa_set_params(dsa_ctx, NID_MLDSA44) ||
    !EVP_PKEY_keygen_init(dsa_ctx) ||
    !EVP_PKEY_keygen(dsa_ctx, &dsa_raw)) {
    printf("ML-DSA keygen failed.\n");
    goto err;
    }
  printf("Generated public key: ");
  hexdump(dsa_raw->pkey.pqdsa_key->public_key, dsa_raw->pkey.pqdsa_key->pqdsa->public_key_len);
  EVP_PKEY_free(dsa_raw);
  EVP_PKEY_CTX_free(dsa_ctx);

  /* DBRG */
  CTR_DRBG_STATE drbg;
  printf("About to seed CTR-DRBG with ");
  hexdump(kDRBGEntropy, sizeof(kDRBGEntropy));
  if (!CTR_DRBG_init(&drbg, kDRBGEntropy, kDRBGPersonalization,
                     sizeof(kDRBGPersonalization)) ||
      !CTR_DRBG_generate(&drbg, output, sizeof(output), kDRBGAD,
                         sizeof(kDRBGAD)) ||
      !CTR_DRBG_reseed(&drbg, kDRBGEntropy2, kDRBGAD, sizeof(kDRBGAD)) ||
      !CTR_DRBG_generate(&drbg, output, sizeof(output), kDRBGAD,
                         sizeof(kDRBGAD))) {
    printf("DRBG failed\n");
    goto err;
  }
  printf("  generated ");
  hexdump(output, sizeof(output));
  CTR_DRBG_clear(&drbg);

  /* TLS KDF */
  printf("About to run TLS KDF\n");
  uint8_t tls_output[32];
  if (!CRYPTO_tls1_prf(EVP_sha256(), tls_output, sizeof(tls_output), kAESKey,
                       sizeof(kAESKey), "foo", 3, kPlaintextSHA256,
                       sizeof(kPlaintextSHA256), kPlaintextSHA256,
                       sizeof(kPlaintextSHA256))) {
    fprintf(stderr, "TLS KDF failed.\n");
    goto err;
  }
  printf("  got ");
  hexdump(tls_output, sizeof(tls_output));

  /* TLS 1.3 KDF (HKDF-Expand-Label) */
  printf("About to run TLS 1.3 KDF\n");
  uint8_t tls13_output[32];
  static const uint8_t kTLS13Label[] = "c e traffic";
  if (!CRYPTO_tls13_hkdf_expand_label(
          tls13_output, sizeof(tls13_output), EVP_sha256(), kAESKey,
          sizeof(kAESKey), kTLS13Label, sizeof(kTLS13Label) - 1,
          kPlaintextSHA256, sizeof(kPlaintextSHA256))) {
    fprintf(stderr, "TLS 1.3 KDF failed.\n");
    goto err;
  }
  printf("  got ");
  hexdump(tls13_output, sizeof(tls13_output));

  /* FFDH */
  printf("About to compute FFDH key-agreement:\n");
  DH *dh = DH_get_rfc7919_2048();
  uint8_t dh_result[2048/8];
  if (!dh ||
      !DH_generate_key(dh) ||
      sizeof(dh_result) != DH_size(dh) ||
      DH_compute_key_padded(dh_result, DH_get0_pub_key(dh), dh) !=
          sizeof(dh_result)) {
    fprintf(stderr, "FFDH failed.\n");
    goto err;
  }
  DH_free(dh);

  printf("  got ");
  hexdump(dh_result, sizeof(dh_result));

  printf("PASS\n");
  return 0;

err:
  printf("FAIL\n");
  fflush(stdout);
  abort();
}
