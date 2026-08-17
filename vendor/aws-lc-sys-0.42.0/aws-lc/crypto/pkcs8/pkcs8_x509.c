// Written by Dr Stephen N Henson (steve@openssl.org) for the OpenSSL project 1999.
// Copyright (c) 1999 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/pkcs8.h>

#include <limits.h>

#include <openssl/asn1t.h>
#include <openssl/asn1.h>
#include <openssl/bio.h>
#include <openssl/buf.h>
#include <openssl/bytestring.h>
#include <openssl/err.h>
#include <openssl/evp.h>
#include <openssl/digest.h>
#include <openssl/hmac.h>
#include <openssl/mem.h>
#include <openssl/rand.h>
#include <openssl/x509.h>

#include "internal.h"
#include "../bytestring/internal.h"
#include "../internal.h"


int pkcs12_iterations_acceptable(uint64_t iterations) {
#if defined(BORINGSSL_UNSAFE_FUZZER_MODE)
  static const uint64_t kIterationsLimit = 2048;
#else
  // Windows imposes a limit of 600K. Mozilla say: “so them increasing
  // maximum to something like 100M or 1G (to have few decades of breathing
  // room) would be very welcome”[1]. So here we set the limit to 100M.
  //
  // [1] https://bugzilla.mozilla.org/show_bug.cgi?id=1436873#c14
  static const uint64_t kIterationsLimit = 100 * 1000000;
#endif

  assert(kIterationsLimit <= UINT32_MAX);
  return 0 < iterations && iterations <= kIterationsLimit;
}

ASN1_SEQUENCE(PKCS8_PRIV_KEY_INFO) = {
    ASN1_SIMPLE(PKCS8_PRIV_KEY_INFO, version, ASN1_INTEGER),
    ASN1_SIMPLE(PKCS8_PRIV_KEY_INFO, pkeyalg, X509_ALGOR),
    ASN1_SIMPLE(PKCS8_PRIV_KEY_INFO, pkey, ASN1_OCTET_STRING),
    ASN1_IMP_SET_OF_OPT(PKCS8_PRIV_KEY_INFO, attributes, X509_ATTRIBUTE, 0),
} ASN1_SEQUENCE_END(PKCS8_PRIV_KEY_INFO)

IMPLEMENT_ASN1_FUNCTIONS_const(PKCS8_PRIV_KEY_INFO)

EVP_PKEY *EVP_PKCS82PKEY(const PKCS8_PRIV_KEY_INFO *p8) {
  uint8_t *der = NULL;
  int der_len = i2d_PKCS8_PRIV_KEY_INFO(p8, &der);
  if (der_len < 0) {
    return NULL;
  }

  CBS cbs;
  CBS_init(&cbs, der, (size_t)der_len);
  EVP_PKEY *ret = EVP_parse_private_key(&cbs);
  if (ret == NULL || CBS_len(&cbs) != 0) {
    OPENSSL_PUT_ERROR(PKCS8, PKCS8_R_DECODE_ERROR);
    EVP_PKEY_free(ret);
    OPENSSL_free(der);
    return NULL;
  }

  OPENSSL_free(der);
  return ret;
}

PKCS8_PRIV_KEY_INFO *EVP_PKEY2PKCS8(const EVP_PKEY *pkey) {
  CBB cbb;
  uint8_t *der = NULL;
  size_t der_len;
  if (!CBB_init(&cbb, 0) ||
      !EVP_marshal_private_key(&cbb, pkey) ||
      !CBB_finish(&cbb, &der, &der_len) ||
      der_len > LONG_MAX) {
    CBB_cleanup(&cbb);
    OPENSSL_PUT_ERROR(PKCS8, PKCS8_R_ENCODE_ERROR);
    goto err;
  }

  const uint8_t *p = der;
  PKCS8_PRIV_KEY_INFO *p8 = d2i_PKCS8_PRIV_KEY_INFO(NULL, &p, (long)der_len);
  if (p8 == NULL || p != der + der_len) {
    PKCS8_PRIV_KEY_INFO_free(p8);
    OPENSSL_PUT_ERROR(PKCS8, PKCS8_R_DECODE_ERROR);
    goto err;
  }

  OPENSSL_free(der);
  return p8;

err:
  OPENSSL_free(der);
  return NULL;
}

PKCS8_PRIV_KEY_INFO *PKCS8_decrypt(X509_SIG *pkcs8, const char *pass,
                                   int pass_len_in) {
  size_t pass_len;
  if (pass_len_in < 0 && pass != NULL) {
    pass_len = strlen(pass);
  } else {
    pass_len = (size_t)pass_len_in;
  }

  PKCS8_PRIV_KEY_INFO *ret = NULL;
  EVP_PKEY *pkey = NULL;
  uint8_t *in = NULL;

  // Convert the legacy ASN.1 object to a byte string.
  int in_len = i2d_X509_SIG(pkcs8, &in);
  if (in_len < 0) {
    goto err;
  }

  CBS cbs;
  CBS_init(&cbs, in, in_len);
  pkey = PKCS8_parse_encrypted_private_key(&cbs, pass, pass_len);
  if (pkey == NULL || CBS_len(&cbs) != 0) {
    goto err;
  }

  ret = EVP_PKEY2PKCS8(pkey);

err:
  OPENSSL_free(in);
  EVP_PKEY_free(pkey);
  return ret;
}

X509_SIG *PKCS8_encrypt(int pbe_nid, const EVP_CIPHER *cipher, const char *pass,
                        int pass_len_in, const uint8_t *salt, size_t salt_len,
                        int iterations, PKCS8_PRIV_KEY_INFO *p8inf) {
  size_t pass_len;
  if (pass == NULL) {
    pass_len = 0;
  } else if (pass_len_in < 0) {
    pass_len = strlen(pass);
  } else {
    pass_len = (size_t)pass_len_in;
  }

  // Parse out the private key.
  EVP_PKEY *pkey = EVP_PKCS82PKEY(p8inf);
  if (pkey == NULL) {
    return NULL;
  }

  X509_SIG *ret = NULL;
  uint8_t *der = NULL;
  size_t der_len;
  CBB cbb;
  if (!CBB_init(&cbb, 128) ||
      !PKCS8_marshal_encrypted_private_key(&cbb, pbe_nid, cipher, pass,
                                           pass_len, salt, salt_len, iterations,
                                           pkey) ||
      !CBB_finish(&cbb, &der, &der_len)) {
    CBB_cleanup(&cbb);
    goto err;
  }

  // Convert back to legacy ASN.1 objects.
  const uint8_t *ptr = der;
  ret = d2i_X509_SIG(NULL, &ptr, der_len);
  if (ret == NULL || ptr != der + der_len) {
    OPENSSL_PUT_ERROR(PKCS8, ERR_R_INTERNAL_ERROR);
    X509_SIG_free(ret);
    ret = NULL;
  }

err:
  OPENSSL_free(der);
  EVP_PKEY_free(pkey);
  return ret;
}

struct pkcs12_context {
  EVP_PKEY **out_key;
  STACK_OF(X509) *out_certs;
  const char *password;
  size_t password_len;
};

// PKCS12_handle_sequence parses a BER-encoded SEQUENCE of elements in a PKCS#12
// structure.
static int PKCS12_handle_sequence(
    CBS *sequence, struct pkcs12_context *ctx,
    int (*handle_element)(CBS *cbs, struct pkcs12_context *ctx)) {
  uint8_t *storage = NULL;
  CBS in;
  int ret = 0;

  // Although a BER->DER conversion is done at the beginning of |PKCS12_parse|,
  // the ASN.1 data gets wrapped in OCTETSTRINGs and/or encrypted and the
  // conversion cannot see through those wrappings. So each time we step
  // through one we need to convert to DER again.
  if (!CBS_asn1_ber_to_der(sequence, &in, &storage)) {
    OPENSSL_PUT_ERROR(PKCS8, PKCS8_R_BAD_PKCS12_DATA);
    return 0;
  }

  CBS child;
  if (!CBS_get_asn1(&in, &child, CBS_ASN1_SEQUENCE) ||
      CBS_len(&in) != 0) {
    OPENSSL_PUT_ERROR(PKCS8, PKCS8_R_BAD_PKCS12_DATA);
    goto err;
  }

  while (CBS_len(&child) > 0) {
    CBS element;
    if (!CBS_get_asn1(&child, &element, CBS_ASN1_SEQUENCE)) {
      OPENSSL_PUT_ERROR(PKCS8, PKCS8_R_BAD_PKCS12_DATA);
      goto err;
    }

    if (!handle_element(&element, ctx)) {
      goto err;
    }
  }

  ret = 1;

err:
  OPENSSL_free(storage);
  return ret;
}

// 1.2.840.113549.1.12.10.1.1
static const uint8_t kKeyBag[] = {0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d,
                                  0x01, 0x0c, 0x0a, 0x01, 0x01};

// 1.2.840.113549.1.12.10.1.2
static const uint8_t kPKCS8ShroudedKeyBag[] = {
    0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x0c, 0x0a, 0x01, 0x02};

// 1.2.840.113549.1.12.10.1.3
static const uint8_t kCertBag[] = {0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d,
                                   0x01, 0x0c, 0x0a, 0x01, 0x03};

// 1.2.840.113549.1.9.20
static const uint8_t kFriendlyName[] = {0x2a, 0x86, 0x48, 0x86, 0xf7,
                                        0x0d, 0x01, 0x09, 0x14};

// 1.2.840.113549.1.9.21
static const uint8_t kLocalKeyID[] = {0x2a, 0x86, 0x48, 0x86, 0xf7,
                                      0x0d, 0x01, 0x09, 0x15};

// 1.2.840.113549.1.9.22.1
static const uint8_t kX509Certificate[] = {0x2a, 0x86, 0x48, 0x86, 0xf7,
                                           0x0d, 0x01, 0x09, 0x16, 0x01};

// parse_bag_attributes parses the bagAttributes field of a SafeBag structure.
// It sets |*out_friendly_name| to a newly-allocated copy of the friendly name,
// encoded as a UTF-8 string, or NULL if there is none. It returns one on
// success and zero on error.
static int parse_bag_attributes(CBS *attrs, uint8_t **out_friendly_name,
                                size_t *out_friendly_name_len) {
  *out_friendly_name = NULL;
  *out_friendly_name_len = 0;

  // See https://tools.ietf.org/html/rfc7292#section-4.2.
  while (CBS_len(attrs) != 0) {
    CBS attr, oid, values;
    if (!CBS_get_asn1(attrs, &attr, CBS_ASN1_SEQUENCE) ||
        !CBS_get_asn1(&attr, &oid, CBS_ASN1_OBJECT) ||
        !CBS_get_asn1(&attr, &values, CBS_ASN1_SET) ||
        CBS_len(&attr) != 0) {
      OPENSSL_PUT_ERROR(PKCS8, PKCS8_R_BAD_PKCS12_DATA);
      goto err;
    }
    if (CBS_mem_equal(&oid, kFriendlyName, sizeof(kFriendlyName))) {
      // See https://tools.ietf.org/html/rfc2985, section 5.5.1.
      CBS value;
      if (*out_friendly_name != NULL ||
          !CBS_get_asn1(&values, &value, CBS_ASN1_BMPSTRING) ||
          CBS_len(&values) != 0 ||
          CBS_len(&value) == 0) {
        OPENSSL_PUT_ERROR(PKCS8, PKCS8_R_BAD_PKCS12_DATA);
        goto err;
      }
      // Convert the friendly name to UTF-8.
      CBB cbb;
      if (!CBB_init(&cbb, CBS_len(&value))) {
        goto err;
      }
      while (CBS_len(&value) != 0) {
        uint32_t c;
        if (!cbs_get_ucs2_be(&value, &c) ||
            !cbb_add_utf8(&cbb, c)) {
          OPENSSL_PUT_ERROR(PKCS8, PKCS8_R_INVALID_CHARACTERS);
          CBB_cleanup(&cbb);
          goto err;
        }
      }
      if (!CBB_finish(&cbb, out_friendly_name, out_friendly_name_len)) {
        CBB_cleanup(&cbb);
        goto err;
      }
    }
  }

  return 1;

err:
  OPENSSL_free(*out_friendly_name);
  *out_friendly_name = NULL;
  *out_friendly_name_len = 0;
  return 0;
}

// PKCS12_handle_safe_bag parses a single SafeBag element in a PKCS#12
// structure.
static int PKCS12_handle_safe_bag(CBS *safe_bag, struct pkcs12_context *ctx) {
  CBS bag_id, wrapped_value, bag_attrs;
  if (!CBS_get_asn1(safe_bag, &bag_id, CBS_ASN1_OBJECT) ||
      !CBS_get_asn1(safe_bag, &wrapped_value,
                    CBS_ASN1_CONTEXT_SPECIFIC | CBS_ASN1_CONSTRUCTED | 0)) {
    OPENSSL_PUT_ERROR(PKCS8, PKCS8_R_BAD_PKCS12_DATA);
    return 0;
  }
  if (CBS_len(safe_bag) == 0) {
    CBS_init(&bag_attrs, NULL, 0);
  } else if (!CBS_get_asn1(safe_bag, &bag_attrs, CBS_ASN1_SET) ||
             CBS_len(safe_bag) != 0) {
    OPENSSL_PUT_ERROR(PKCS8, PKCS8_R_BAD_PKCS12_DATA);
    return 0;
  }

  const int is_key_bag = CBS_mem_equal(&bag_id, kKeyBag, sizeof(kKeyBag));
  const int is_shrouded_key_bag = CBS_mem_equal(&bag_id, kPKCS8ShroudedKeyBag,
                                                sizeof(kPKCS8ShroudedKeyBag));
  if (is_key_bag || is_shrouded_key_bag) {
    // See RFC 7292, section 4.2.1 and 4.2.2.
    if (*ctx->out_key) {
      OPENSSL_PUT_ERROR(PKCS8, PKCS8_R_MULTIPLE_PRIVATE_KEYS_IN_PKCS12);
      return 0;
    }

    EVP_PKEY *pkey =
        is_key_bag ? EVP_parse_private_key(&wrapped_value)
                   : PKCS8_parse_encrypted_private_key(
                         &wrapped_value, ctx->password, ctx->password_len);
    if (pkey == NULL) {
      return 0;
    }

    if (CBS_len(&wrapped_value) != 0) {
      OPENSSL_PUT_ERROR(PKCS8, PKCS8_R_BAD_PKCS12_DATA);
      EVP_PKEY_free(pkey);
      return 0;
    }

    *ctx->out_key = pkey;
    return 1;
  }

  if (CBS_mem_equal(&bag_id, kCertBag, sizeof(kCertBag))) {
    // See RFC 7292, section 4.2.3.
    CBS cert_bag, cert_type, wrapped_cert, cert;
    if (!CBS_get_asn1(&wrapped_value, &cert_bag, CBS_ASN1_SEQUENCE) ||
        !CBS_get_asn1(&cert_bag, &cert_type, CBS_ASN1_OBJECT) ||
        !CBS_get_asn1(&cert_bag, &wrapped_cert,
                      CBS_ASN1_CONTEXT_SPECIFIC | CBS_ASN1_CONSTRUCTED | 0) ||
        !CBS_get_asn1(&wrapped_cert, &cert, CBS_ASN1_OCTETSTRING)) {
      OPENSSL_PUT_ERROR(PKCS8, PKCS8_R_BAD_PKCS12_DATA);
      return 0;
    }

    // Skip unknown certificate types.
    if (!CBS_mem_equal(&cert_type, kX509Certificate,
                       sizeof(kX509Certificate))) {
      return 1;
    }

    if (CBS_len(&cert) > LONG_MAX) {
      OPENSSL_PUT_ERROR(PKCS8, PKCS8_R_BAD_PKCS12_DATA);
      return 0;
    }

    const uint8_t *inp = CBS_data(&cert);
    X509 *x509 = d2i_X509(NULL, &inp, (long)CBS_len(&cert));
    if (!x509) {
      OPENSSL_PUT_ERROR(PKCS8, PKCS8_R_BAD_PKCS12_DATA);
      return 0;
    }

    if (inp != CBS_data(&cert) + CBS_len(&cert)) {
      OPENSSL_PUT_ERROR(PKCS8, PKCS8_R_BAD_PKCS12_DATA);
      X509_free(x509);
      return 0;
    }

    uint8_t *friendly_name;
    size_t friendly_name_len;
    if (!parse_bag_attributes(&bag_attrs, &friendly_name, &friendly_name_len)) {
      X509_free(x509);
      return 0;
    }
    int ok = friendly_name_len == 0 ||
             X509_alias_set1(x509, friendly_name, friendly_name_len);
    OPENSSL_free(friendly_name);
    if (!ok ||
        0 == sk_X509_push(ctx->out_certs, x509)) {
      X509_free(x509);
      return 0;
    }

    return 1;
  }

  // Unknown element type - ignore it.
  return 1;
}

// 1.2.840.113549.1.7.1
static const uint8_t kPKCS7Data[] = {0x2a, 0x86, 0x48, 0x86, 0xf7,
                                     0x0d, 0x01, 0x07, 0x01};

// 1.2.840.113549.1.7.6
static const uint8_t kPKCS7EncryptedData[] = {0x2a, 0x86, 0x48, 0x86, 0xf7,
                                              0x0d, 0x01, 0x07, 0x06};

// PKCS12_handle_content_info parses a single PKCS#7 ContentInfo element in a
// PKCS#12 structure.
static int PKCS12_handle_content_info(CBS *content_info,
                                      struct pkcs12_context *ctx) {
  CBS content_type, wrapped_contents, contents;
  int ret = 0;
  uint8_t *storage = NULL;

  if (!CBS_get_asn1(content_info, &content_type, CBS_ASN1_OBJECT) ||
      !CBS_get_asn1(content_info, &wrapped_contents,
                        CBS_ASN1_CONTEXT_SPECIFIC | CBS_ASN1_CONSTRUCTED | 0) ||
      CBS_len(content_info) != 0) {
    OPENSSL_PUT_ERROR(PKCS8, PKCS8_R_BAD_PKCS12_DATA);
    goto err;
  }

  if (CBS_mem_equal(&content_type, kPKCS7EncryptedData,
                    sizeof(kPKCS7EncryptedData))) {
    // See https://tools.ietf.org/html/rfc2315#section-13.
    //
    // PKCS#7 encrypted data inside a PKCS#12 structure is generally an
    // encrypted certificate bag and it's generally encrypted with 40-bit
    // RC2-CBC.
    CBS version_bytes, eci, contents_type, ai, encrypted_contents;
    uint8_t *out;
    size_t out_len;

    if (!CBS_get_asn1(&wrapped_contents, &contents, CBS_ASN1_SEQUENCE) ||
        !CBS_get_asn1(&contents, &version_bytes, CBS_ASN1_INTEGER) ||
        // EncryptedContentInfo, see
        // https://tools.ietf.org/html/rfc2315#section-10.1
        !CBS_get_asn1(&contents, &eci, CBS_ASN1_SEQUENCE) ||
        !CBS_get_asn1(&eci, &contents_type, CBS_ASN1_OBJECT) ||
        // AlgorithmIdentifier, see
        // https://tools.ietf.org/html/rfc5280#section-4.1.1.2
        !CBS_get_asn1(&eci, &ai, CBS_ASN1_SEQUENCE) ||
        !CBS_get_asn1_implicit_string(
            &eci, &encrypted_contents, &storage,
            CBS_ASN1_CONTEXT_SPECIFIC | 0, CBS_ASN1_OCTETSTRING)) {
      OPENSSL_PUT_ERROR(PKCS8, PKCS8_R_BAD_PKCS12_DATA);
      goto err;
    }

    if (!CBS_mem_equal(&contents_type, kPKCS7Data, sizeof(kPKCS7Data))) {
      OPENSSL_PUT_ERROR(PKCS8, PKCS8_R_BAD_PKCS12_DATA);
      goto err;
    }

    if (!pkcs8_pbe_decrypt(&out, &out_len, &ai, ctx->password,
                           ctx->password_len, CBS_data(&encrypted_contents),
                           CBS_len(&encrypted_contents))) {
      goto err;
    }

    CBS safe_contents;
    CBS_init(&safe_contents, out, out_len);
    ret = PKCS12_handle_sequence(&safe_contents, ctx, PKCS12_handle_safe_bag);
    OPENSSL_free(out);
  } else if (CBS_mem_equal(&content_type, kPKCS7Data, sizeof(kPKCS7Data))) {
    CBS octet_string_contents;

    if (!CBS_get_asn1(&wrapped_contents, &octet_string_contents,
                      CBS_ASN1_OCTETSTRING)) {
      OPENSSL_PUT_ERROR(PKCS8, PKCS8_R_BAD_PKCS12_DATA);
      goto err;
    }

    ret = PKCS12_handle_sequence(&octet_string_contents, ctx,
                                 PKCS12_handle_safe_bag);
  } else {
    // Unknown element type - ignore it.
    ret = 1;
  }

err:
  OPENSSL_free(storage);
  return ret;
}

static int pkcs12_check_mac(int *out_mac_ok, const char *password,
                            size_t password_len, const CBS *salt,
                            uint32_t iterations, const EVP_MD *md,
                            const CBS *authsafes, const CBS *expected_mac) {
  int ret = 0;
  uint8_t hmac_key[EVP_MAX_MD_SIZE];
  if (!pkcs12_key_gen(password, password_len, CBS_data(salt), CBS_len(salt),
                      PKCS12_MAC_ID, iterations, EVP_MD_size(md), hmac_key,
                      md)) {
    goto err;
  }

  uint8_t hmac[EVP_MAX_MD_SIZE];
  unsigned hmac_len;
  if (NULL == HMAC(md, hmac_key, EVP_MD_size(md), CBS_data(authsafes),
                   CBS_len(authsafes), hmac, &hmac_len)) {
    goto err;
  }

  *out_mac_ok = CBS_mem_equal(expected_mac, hmac, hmac_len);
#if defined(BORINGSSL_UNSAFE_FUZZER_MODE)
  *out_mac_ok = 1;
#endif
  ret = 1;

err:
  OPENSSL_cleanse(hmac_key, sizeof(hmac_key));
  return ret;
}


int PKCS12_get_key_and_certs(EVP_PKEY **out_key, STACK_OF(X509) *out_certs,
                             CBS *ber_in, const char *password) {
  uint8_t *storage = NULL;
  CBS in, pfx, mac_data, authsafe, content_type, wrapped_authsafes, authsafes;
  uint64_t version;
  int ret = 0;
  struct pkcs12_context ctx;
  const size_t original_out_certs_len = sk_X509_num(out_certs);

  // The input may be in BER format.
  if (!CBS_asn1_ber_to_der(ber_in, &in, &storage)) {
    OPENSSL_PUT_ERROR(PKCS8, PKCS8_R_BAD_PKCS12_DATA);
    return 0;
  }

  *out_key = NULL;
  OPENSSL_memset(&ctx, 0, sizeof(ctx));

  // See ftp://ftp.rsasecurity.com/pub/pkcs/pkcs-12/pkcs-12v1.pdf, section
  // four.
  if (!CBS_get_asn1(&in, &pfx, CBS_ASN1_SEQUENCE) ||
      CBS_len(&in) != 0 ||
      !CBS_get_asn1_uint64(&pfx, &version)) {
    OPENSSL_PUT_ERROR(PKCS8, PKCS8_R_BAD_PKCS12_DATA);
    goto err;
  }

  if (version < 3) {
    OPENSSL_PUT_ERROR(PKCS8, PKCS8_R_BAD_PKCS12_VERSION);
    goto err;
  }

  if (!CBS_get_asn1(&pfx, &authsafe, CBS_ASN1_SEQUENCE)) {
    OPENSSL_PUT_ERROR(PKCS8, PKCS8_R_BAD_PKCS12_DATA);
    goto err;
  }

  if (CBS_len(&pfx) == 0) {
    OPENSSL_PUT_ERROR(PKCS8, PKCS8_R_MISSING_MAC);
    goto err;
  }

  if (!CBS_get_asn1(&pfx, &mac_data, CBS_ASN1_SEQUENCE)) {
    OPENSSL_PUT_ERROR(PKCS8, PKCS8_R_BAD_PKCS12_DATA);
    goto err;
  }

  // authsafe is a PKCS#7 ContentInfo. See
  // https://tools.ietf.org/html/rfc2315#section-7.
  if (!CBS_get_asn1(&authsafe, &content_type, CBS_ASN1_OBJECT) ||
      !CBS_get_asn1(&authsafe, &wrapped_authsafes,
                        CBS_ASN1_CONTEXT_SPECIFIC | CBS_ASN1_CONSTRUCTED | 0)) {
    OPENSSL_PUT_ERROR(PKCS8, PKCS8_R_BAD_PKCS12_DATA);
    goto err;
  }

  // The content type can either be data or signedData. The latter indicates
  // that it's signed by a public key, which isn't supported.
  if (!CBS_mem_equal(&content_type, kPKCS7Data, sizeof(kPKCS7Data))) {
    OPENSSL_PUT_ERROR(PKCS8, PKCS8_R_PKCS12_PUBLIC_KEY_INTEGRITY_NOT_SUPPORTED);
    goto err;
  }

  if (!CBS_get_asn1(&wrapped_authsafes, &authsafes, CBS_ASN1_OCTETSTRING)) {
    OPENSSL_PUT_ERROR(PKCS8, PKCS8_R_BAD_PKCS12_DATA);
    goto err;
  }

  ctx.out_key = out_key;
  ctx.out_certs = out_certs;
  ctx.password = password;
  ctx.password_len = password != NULL ? strlen(password) : 0;

  // Verify the MAC.
  {
    CBS mac, salt, expected_mac;
    if (!CBS_get_asn1(&mac_data, &mac, CBS_ASN1_SEQUENCE)) {
      OPENSSL_PUT_ERROR(PKCS8, PKCS8_R_BAD_PKCS12_DATA);
      goto err;
    }

    const EVP_MD *md = EVP_parse_digest_algorithm(&mac);
    if (md == NULL) {
      goto err;
    }

    if (!CBS_get_asn1(&mac, &expected_mac, CBS_ASN1_OCTETSTRING) ||
        !CBS_get_asn1(&mac_data, &salt, CBS_ASN1_OCTETSTRING)) {
      OPENSSL_PUT_ERROR(PKCS8, PKCS8_R_BAD_PKCS12_DATA);
      goto err;
    }

    // The iteration count is optional and the default is one.
    uint32_t iterations = 1;
    if (CBS_len(&mac_data) > 0) {
      uint64_t iterations_u64;
      if (!CBS_get_asn1_uint64(&mac_data, &iterations_u64) ||
          !pkcs12_iterations_acceptable(iterations_u64)) {
        OPENSSL_PUT_ERROR(PKCS8, PKCS8_R_BAD_PKCS12_DATA);
        goto err;
      }
      iterations = (uint32_t)iterations_u64;
    }

    int mac_ok;
    if (!pkcs12_check_mac(&mac_ok, ctx.password, ctx.password_len, &salt,
                          iterations, md, &authsafes, &expected_mac)) {
      goto err;
    }
    if (!mac_ok && ctx.password_len == 0) {
      // PKCS#12 encodes passwords as NUL-terminated UCS-2, so the empty
      // password is encoded as {0, 0}. Some implementations use the empty byte
      // array for "no password". OpenSSL considers a non-NULL password as {0,
      // 0} and a NULL password as {}. It then, in high-level PKCS#12 parsing
      // code, tries both options. We match this behavior.
      ctx.password = ctx.password != NULL ? NULL : "";
      if (!pkcs12_check_mac(&mac_ok, ctx.password, ctx.password_len, &salt,
                            iterations, md, &authsafes, &expected_mac)) {
        goto err;
      }
    }
    if (!mac_ok) {
      OPENSSL_PUT_ERROR(PKCS12, PKCS12_R_MAC_VERIFY_FAILURE);
      goto err;
    }
  }

  // authsafes contains a series of PKCS#7 ContentInfos.
  if (!PKCS12_handle_sequence(&authsafes, &ctx, PKCS12_handle_content_info)) {
    goto err;
  }

  ret = 1;

err:
  OPENSSL_free(storage);
  if (!ret) {
    EVP_PKEY_free(*out_key);
    *out_key = NULL;
    while (sk_X509_num(out_certs) > original_out_certs_len) {
      X509 *x509 = sk_X509_pop(out_certs);
      X509_free(x509);
    }
  }

  return ret;
}

void PKCS12_PBE_add(void) {}

struct pkcs12_st {
  uint8_t *ber_bytes;
  size_t ber_len;
};

PKCS12 *d2i_PKCS12(PKCS12 **out_p12, const uint8_t **ber_bytes,
                   size_t ber_len) {
  PKCS12 *p12 = PKCS12_new();
  if (!p12) {
    return NULL;
  }

  p12->ber_bytes = OPENSSL_memdup(*ber_bytes, ber_len);
  if (!p12->ber_bytes) {
    OPENSSL_free(p12);
    return NULL;
  }

  p12->ber_len = ber_len;
  *ber_bytes += ber_len;

  if (out_p12) {
    PKCS12_free(*out_p12);
    *out_p12 = p12;
  }

  return p12;
}

PKCS12* d2i_PKCS12_bio(BIO *bio, PKCS12 **out_p12) {
  size_t used = 0;
  BUF_MEM *buf;
  const uint8_t *dummy;
  static const size_t kMaxSize = 256 * 1024;
  PKCS12 *ret = NULL;

  buf = BUF_MEM_new();
  if (buf == NULL) {
    return NULL;
  }
  if (BUF_MEM_grow(buf, 8192) == 0) {
    goto out;
  }

  for (;;) {
    size_t max_read = buf->length - used;
    int n = BIO_read(bio, &buf->data[used],
                     max_read > INT_MAX ? INT_MAX : (int)max_read);
    if (n < 0) {
      if (used == 0) {
        goto out;
      }
      // Workaround a bug in node.js. It uses a memory BIO for this in the wrong
      // mode.
      n = 0;
    }

    if (n == 0) {
      break;
    }
    used += n;

    if (used < buf->length) {
      continue;
    }

    if (buf->length > kMaxSize ||
        BUF_MEM_grow(buf, buf->length * 2) == 0) {
      goto out;
    }
  }

  dummy = (uint8_t*) buf->data;
  ret = d2i_PKCS12(out_p12, &dummy, used);

out:
  BUF_MEM_free(buf);
  return ret;
}

PKCS12* d2i_PKCS12_fp(FILE *fp, PKCS12 **out_p12) {
  BIO *bio;
  PKCS12 *ret;

  bio = BIO_new_fp(fp, 0 /* don't take ownership */);
  if (!bio) {
    return NULL;
  }

  ret = d2i_PKCS12_bio(bio, out_p12);
  BIO_free(bio);
  return ret;
}

int i2d_PKCS12(const PKCS12 *p12, uint8_t **out) {
  if (p12->ber_len > INT_MAX) {
    OPENSSL_PUT_ERROR(PKCS8, ERR_R_OVERFLOW);
    return -1;
  }

  if (out == NULL) {
    return (int)p12->ber_len;
  }

  if (*out == NULL) {
    *out = OPENSSL_memdup(p12->ber_bytes, p12->ber_len);
    if (*out == NULL) {
      return -1;
    }
  } else {
    OPENSSL_memcpy(*out, p12->ber_bytes, p12->ber_len);
    *out += p12->ber_len;
  }
  return (int)p12->ber_len;
}

int i2d_PKCS12_bio(BIO *bio, const PKCS12 *p12) {
  return BIO_write_all(bio, p12->ber_bytes, p12->ber_len);
}

int i2d_PKCS12_fp(FILE *fp, const PKCS12 *p12) {
  BIO *bio = BIO_new_fp(fp, 0 /* don't take ownership */);
  if (bio == NULL) {
    return 0;
  }

  int ret = i2d_PKCS12_bio(bio, p12);
  BIO_free(bio);
  return ret;
}

int PKCS12_parse(const PKCS12 *p12, const char *password, EVP_PKEY **out_pkey,
                 X509 **out_cert, STACK_OF(X509) **out_ca_certs) {
  CBS ber_bytes;
  STACK_OF(X509) *ca_certs = NULL;
  char ca_certs_alloced = 0;

  if (out_ca_certs != NULL && *out_ca_certs != NULL) {
    ca_certs = *out_ca_certs;
  }

  if (!ca_certs) {
    ca_certs = sk_X509_new_null();
    if (ca_certs == NULL) {
      return 0;
    }
    ca_certs_alloced = 1;
  }

  CBS_init(&ber_bytes, p12->ber_bytes, p12->ber_len);
  if (!PKCS12_get_key_and_certs(out_pkey, ca_certs, &ber_bytes, password)) {
    if (ca_certs_alloced) {
      sk_X509_free(ca_certs);
    }
    return 0;
  }

  // OpenSSL selects the last certificate which matches the private key as
  // |out_cert|.
  *out_cert = NULL;
  size_t num_certs = sk_X509_num(ca_certs);
  if (*out_pkey != NULL && num_certs > 0) {
    for (size_t i = num_certs - 1; i < num_certs; i--) {
      X509 *cert = sk_X509_value(ca_certs, i);
      if (X509_check_private_key(cert, *out_pkey)) {
        *out_cert = cert;
        sk_X509_delete(ca_certs, i);
        break;
      }
      ERR_clear_error();
    }
  }

  if (out_ca_certs) {
    *out_ca_certs = ca_certs;
  } else {
    sk_X509_pop_free(ca_certs, X509_free);
  }

  return 1;
}

int PKCS12_verify_mac(const PKCS12 *p12, const char *password,
                      int password_len) {
  if (password == NULL) {
    if (password_len != 0) {
      return 0;
    }
  } else if (password_len < -1) {
    return 0;
  } else if (password_len != -1 &&
             (password[password_len] != 0 ||
              OPENSSL_memchr(password, 0, password_len) != NULL)) {
    return 0;
  }

  EVP_PKEY *pkey = NULL;
  X509 *cert = NULL;
  if (!PKCS12_parse(p12, password, &pkey, &cert, NULL)) {
    ERR_clear_error();
    return 0;
  }

  EVP_PKEY_free(pkey);
  X509_free(cert);

  return 1;
}

// add_bag_attributes adds the bagAttributes field of a SafeBag structure,
// containing the specified friendlyName and localKeyId attributes.
static int add_bag_attributes(CBB *bag, const char *name, size_t name_len,
                              const uint8_t *key_id, size_t key_id_len) {
  if (name == NULL && key_id_len == 0) {
    return 1;  // Omit the OPTIONAL SET.
  }
  // See https://tools.ietf.org/html/rfc7292#section-4.2.
  CBB attrs, attr, oid, values, value;
  if (!CBB_add_asn1(bag, &attrs, CBS_ASN1_SET)) {
    return 0;
  }
  if (name_len != 0) {
    // See https://tools.ietf.org/html/rfc2985, section 5.5.1.
    if (!CBB_add_asn1(&attrs, &attr, CBS_ASN1_SEQUENCE) ||
        !CBB_add_asn1(&attr, &oid, CBS_ASN1_OBJECT) ||
        !CBB_add_bytes(&oid, kFriendlyName, sizeof(kFriendlyName)) ||
        !CBB_add_asn1(&attr, &values, CBS_ASN1_SET) ||
        !CBB_add_asn1(&values, &value, CBS_ASN1_BMPSTRING)) {
      return 0;
    }
    // Convert the friendly name to a BMPString.
    CBS name_cbs;
    CBS_init(&name_cbs, (const uint8_t *)name, name_len);
    while (CBS_len(&name_cbs) != 0) {
      uint32_t c;
      if (!cbs_get_utf8(&name_cbs, &c) ||
          !cbb_add_ucs2_be(&value, c)) {
        OPENSSL_PUT_ERROR(PKCS8, PKCS8_R_INVALID_CHARACTERS);
        return 0;
      }
    }
  }
  if (key_id_len != 0) {
    // See https://tools.ietf.org/html/rfc2985, section 5.5.2.
    if (!CBB_add_asn1(&attrs, &attr, CBS_ASN1_SEQUENCE) ||
        !CBB_add_asn1(&attr, &oid, CBS_ASN1_OBJECT) ||
        !CBB_add_bytes(&oid, kLocalKeyID, sizeof(kLocalKeyID)) ||
        !CBB_add_asn1(&attr, &values, CBS_ASN1_SET) ||
        !CBB_add_asn1(&values, &value, CBS_ASN1_OCTETSTRING) ||
        !CBB_add_bytes(&value, key_id, key_id_len)) {
      return 0;
    }
  }
  return CBB_flush_asn1_set_of(&attrs) &&
         CBB_flush(bag);
}

static int add_cert_bag(CBB *cbb, X509 *cert, const char *name,
                        const uint8_t *key_id, size_t key_id_len) {
  CBB bag, bag_oid, bag_contents, cert_bag, cert_type, wrapped_cert, cert_value;
  if (// See https://tools.ietf.org/html/rfc7292#section-4.2.
      !CBB_add_asn1(cbb, &bag, CBS_ASN1_SEQUENCE) ||
      !CBB_add_asn1(&bag, &bag_oid, CBS_ASN1_OBJECT) ||
      !CBB_add_bytes(&bag_oid, kCertBag, sizeof(kCertBag)) ||
      !CBB_add_asn1(&bag, &bag_contents,
                    CBS_ASN1_CONSTRUCTED | CBS_ASN1_CONTEXT_SPECIFIC | 0) ||
      // See https://tools.ietf.org/html/rfc7292#section-4.2.3.
      !CBB_add_asn1(&bag_contents, &cert_bag, CBS_ASN1_SEQUENCE) ||
      !CBB_add_asn1(&cert_bag, &cert_type, CBS_ASN1_OBJECT) ||
      !CBB_add_bytes(&cert_type, kX509Certificate, sizeof(kX509Certificate)) ||
      !CBB_add_asn1(&cert_bag, &wrapped_cert,
                    CBS_ASN1_CONSTRUCTED | CBS_ASN1_CONTEXT_SPECIFIC | 0) ||
      !CBB_add_asn1(&wrapped_cert, &cert_value, CBS_ASN1_OCTETSTRING)) {
    return 0;
  }
  uint8_t *buf;
  int len = i2d_X509(cert, NULL);

  int int_name_len = 0;
  const char *cert_name = (const char *)X509_alias_get0(cert, &int_name_len);
  size_t name_len = int_name_len;
  if (name) {
    if (name_len != 0) {
      OPENSSL_PUT_ERROR(PKCS8, PKCS8_R_AMBIGUOUS_FRIENDLY_NAME);
      return 0;
    }
    name_len = strlen(name);
  } else {
    name = cert_name;
  }

  if (len < 0 ||
      !CBB_add_space(&cert_value, &buf, (size_t)len) ||
      i2d_X509(cert, &buf) < 0 ||
      !add_bag_attributes(&bag, name, name_len, key_id, key_id_len) ||
      !CBB_flush(cbb)) {
    return 0;
  }
  return 1;
}

static int add_cert_safe_contents(CBB *cbb, X509 *cert,
                                  const STACK_OF(X509) *chain, const char *name,
                                  const uint8_t *key_id, size_t key_id_len) {
  CBB safe_contents;
  if (!CBB_add_asn1(cbb, &safe_contents, CBS_ASN1_SEQUENCE) ||
      (cert != NULL &&
       !add_cert_bag(&safe_contents, cert, name, key_id, key_id_len))) {
    return 0;
  }

  for (size_t i = 0; i < sk_X509_num(chain); i++) {
    // Only the leaf certificate gets attributes.
    if (!add_cert_bag(&safe_contents, sk_X509_value(chain, i), NULL, NULL, 0)) {
      return 0;
    }
  }

  return CBB_flush(cbb);
}

static int add_encrypted_data(CBB *out, int pbe_nid, const char *password,
                              size_t password_len, uint32_t iterations,
                              const uint8_t *in, size_t in_len) {
  uint8_t salt[PKCS12_SALT_LEN];
  AWSLC_ABORT_IF_NOT_ONE(RAND_bytes(salt, sizeof(salt)));

  int ret = 0;
  EVP_CIPHER_CTX ctx;
  EVP_CIPHER_CTX_init(&ctx);
  CBB content_info, type, wrapper, encrypted_data, encrypted_content_info,
      inner_type, encrypted_content;
  if (// Add the ContentInfo wrapping.
      !CBB_add_asn1(out, &content_info, CBS_ASN1_SEQUENCE) ||
      !CBB_add_asn1(&content_info, &type, CBS_ASN1_OBJECT) ||
      !CBB_add_bytes(&type, kPKCS7EncryptedData, sizeof(kPKCS7EncryptedData)) ||
      !CBB_add_asn1(&content_info, &wrapper,
                    CBS_ASN1_CONSTRUCTED | CBS_ASN1_CONTEXT_SPECIFIC | 0) ||
      // See https://tools.ietf.org/html/rfc2315#section-13.
      !CBB_add_asn1(&wrapper, &encrypted_data, CBS_ASN1_SEQUENCE) ||
      !CBB_add_asn1_uint64(&encrypted_data, 0 /* version */) ||
      // See https://tools.ietf.org/html/rfc2315#section-10.1.
      !CBB_add_asn1(&encrypted_data, &encrypted_content_info,
                    CBS_ASN1_SEQUENCE) ||
      !CBB_add_asn1(&encrypted_content_info, &inner_type, CBS_ASN1_OBJECT) ||
      !CBB_add_bytes(&inner_type, kPKCS7Data, sizeof(kPKCS7Data)) ||
      // Set up encryption and fill in contentEncryptionAlgorithm.
      !pkcs12_pbe_encrypt_init(&encrypted_content_info, &ctx, pbe_nid,
                               iterations, password, password_len, salt,
                               sizeof(salt)) ||
      // Note this tag is primitive. It is an implicitly-tagged OCTET_STRING, so
      // it inherits the inner tag's constructed bit.
      !CBB_add_asn1(&encrypted_content_info, &encrypted_content,
                    CBS_ASN1_CONTEXT_SPECIFIC | 0)) {
    goto err;
  }

  // |EVP_CipherUpdate| takes |int| for the input length and may output up to
  // |in_len + block_size| bytes. Ensure the total fits in |int| to avoid
  // truncation and cannot overflow.
  size_t block_size = EVP_CIPHER_CTX_block_size(&ctx);
  if (in_len > INT_MAX - block_size) {
    OPENSSL_PUT_ERROR(PKCS8, ERR_R_OVERFLOW);
    goto err;
  }
  size_t max_out = in_len + block_size;

  uint8_t *ptr;
  int n1, n2;
  if (!CBB_reserve(&encrypted_content, &ptr, max_out) ||
      !EVP_CipherUpdate(&ctx, ptr, &n1, in, (int)in_len) ||
      !EVP_CipherFinal_ex(&ctx, ptr + n1, &n2) ||
      !CBB_did_write(&encrypted_content, n1 + n2) ||
      !CBB_flush(out)) {
    goto err;
  }

  ret = 1;

err:
  EVP_CIPHER_CTX_cleanup(&ctx);
  return ret;
}

static int pkcs12_gen_and_write_mac(CBB *out_pfx, const uint8_t *auth_safe_data,
                                    size_t auth_safe_data_len,
                                    const char *password, size_t password_len,
                                    uint8_t *mac_salt, size_t salt_len,
                                    int mac_iterations, const EVP_MD *md) {
  int ret = 0;
  uint8_t mac_key[EVP_MAX_MD_SIZE];
  uint8_t mac[EVP_MAX_MD_SIZE];
  unsigned mac_len;
  if (!pkcs12_key_gen(password, password_len, mac_salt, salt_len, PKCS12_MAC_ID,
                      mac_iterations, EVP_MD_size(md), mac_key, md) ||
      !HMAC(md, mac_key, EVP_MD_size(md), auth_safe_data, auth_safe_data_len,
            mac, &mac_len)) {
    goto out;
  }

  CBB mac_data, digest_info, mac_cbb, mac_salt_cbb;
  if (!CBB_add_asn1(out_pfx, &mac_data, CBS_ASN1_SEQUENCE) ||
      !CBB_add_asn1(&mac_data, &digest_info, CBS_ASN1_SEQUENCE) ||
      !EVP_marshal_digest_algorithm(&digest_info, md) ||
      !CBB_add_asn1(&digest_info, &mac_cbb, CBS_ASN1_OCTETSTRING) ||
      !CBB_add_bytes(&mac_cbb, mac, mac_len) ||
      !CBB_add_asn1(&mac_data, &mac_salt_cbb, CBS_ASN1_OCTETSTRING) ||
      !CBB_add_bytes(&mac_salt_cbb, mac_salt, salt_len) ||
      // The iteration count has a DEFAULT of 1, but RFC 7292 says "The default
      // is for historical reasons and its use is deprecated." Thus we
      // explicitly encode the iteration count, though it is not valid DER.
      !CBB_add_asn1_uint64(&mac_data, mac_iterations) ||
      !CBB_flush(out_pfx)) {
    goto out;
  }
  ret = 1;

out:
  OPENSSL_cleanse(mac_key, sizeof(mac_key));
  return ret;
}

PKCS12 *PKCS12_create(const char *password, const char *name,
                      const EVP_PKEY *pkey, X509 *cert,
                      const STACK_OF(X509)* chain, int key_nid, int cert_nid,
                      int iterations, int mac_iterations, int key_type) {
  if (key_nid == 0) {
    key_nid = NID_pbe_WithSHA1And3_Key_TripleDES_CBC;
  }
  if (cert_nid == 0) {
    cert_nid = NID_pbe_WithSHA1And40BitRC2_CBC;
  }
  if (iterations == 0) {
    iterations = PKCS12_DEFAULT_ITER;
  }
  if (mac_iterations == 0) {
    mac_iterations = 1;
  }
  if (// In OpenSSL, this specifies a non-standard Microsoft key usage extension
      // which we do not currently support.
      key_type != 0 ||
      // In OpenSSL, -1 here means to omit the MAC, which we do not
      // currently support. Omitting it is also invalid for a password-based
      // PKCS#12 file.
      mac_iterations < 0 ||
      // Don't encode empty objects.
      (pkey == NULL && cert == NULL && sk_X509_num(chain) == 0)) {
    OPENSSL_PUT_ERROR(PKCS8, PKCS8_R_UNSUPPORTED_OPTIONS);
    return 0;
  }

  // PKCS#12 is a very confusing recursive data format, built out of another
  // recursive data format. Section 5.1 of RFC 7292 describes the encoding
  // algorithm, but there is no clear overview. A quick summary:
  //
  // PKCS#7 defines a ContentInfo structure, which is a overgeneralized typed
  // combinator structure for applying cryptography. We care about two types. A
  // data ContentInfo contains an OCTET STRING and is a leaf node of the
  // combinator tree. An encrypted-data ContentInfo contains encryption
  // parameters (key derivation and encryption) and wraps another ContentInfo,
  // usually data.
  //
  // A PKCS#12 file is a PFX structure (section 4), which contains a single data
  // ContentInfo and a MAC over it. This root ContentInfo is the
  // AuthenticatedSafe and its payload is a SEQUENCE of other ContentInfos, so
  // that different parts of the PKCS#12 file can by differently protected.
  //
  // Each ContentInfo in the AuthenticatedSafe, after undoing all the PKCS#7
  // combinators, has SafeContents payload. A SafeContents is a SEQUENCE of
  // SafeBag. SafeBag is PKCS#12's typed structure, with subtypes such as KeyBag
  // and CertBag. Confusingly, there is a SafeContents bag type which itself
  // recursively contains more SafeBags, but we do not implement this. Bags also
  // can have attributes.
  //
  // The grouping of SafeBags into intermediate ContentInfos does not appear to
  // be significant, except that all SafeBags sharing a ContentInfo have the
  // same level of protection. Additionally, while keys may be encrypted by
  // placing a KeyBag in an encrypted-data ContentInfo, PKCS#12 also defines a
  // key-specific encryption container, PKCS8ShroudedKeyBag, which is used
  // instead.

  // Note that |password| may be NULL to specify no password, rather than the
  // empty string. They are encoded differently in PKCS#12. (One is the empty
  // byte array and the other is NUL-terminated UCS-2.)
  size_t password_len = password != NULL ? strlen(password) : 0;

  uint8_t key_id[EVP_MAX_MD_SIZE];
  unsigned key_id_len = 0;
  if (cert != NULL && pkey != NULL) {
    if (!X509_check_private_key(cert, pkey) ||
        // Matching OpenSSL, use the SHA-1 hash of the certificate as the local
        // key ID. Some PKCS#12 consumers require one to connect the private key
        // and certificate.
        !X509_digest(cert, EVP_sha1(), key_id, &key_id_len)) {
      return 0;
    }
  }

  // See https://tools.ietf.org/html/rfc7292#section-4.
  PKCS12 *ret = NULL;
  CBB cbb, pfx, auth_safe, auth_safe_oid, auth_safe_wrapper, auth_safe_data,
      content_infos;
  if (!CBB_init(&cbb, 0) || !CBB_add_asn1(&cbb, &pfx, CBS_ASN1_SEQUENCE) ||
      !CBB_add_asn1_uint64(&pfx, 3) ||
      // auth_safe is a data ContentInfo.
      !CBB_add_asn1(&pfx, &auth_safe, CBS_ASN1_SEQUENCE) ||
      !CBB_add_asn1(&auth_safe, &auth_safe_oid, CBS_ASN1_OBJECT) ||
      !CBB_add_bytes(&auth_safe_oid, kPKCS7Data, sizeof(kPKCS7Data)) ||
      !CBB_add_asn1(&auth_safe, &auth_safe_wrapper,
                    CBS_ASN1_CONSTRUCTED | CBS_ASN1_CONTEXT_SPECIFIC | 0) ||
      !CBB_add_asn1(&auth_safe_wrapper, &auth_safe_data,
                    CBS_ASN1_OCTETSTRING) ||
      // See https://tools.ietf.org/html/rfc7292#section-4.1. |auth_safe|'s
      // contains a SEQUENCE of ContentInfos.
      !CBB_add_asn1(&auth_safe_data, &content_infos, CBS_ASN1_SEQUENCE)) {
    goto err;
  }

  // If there are any certificates, place them in CertBags wrapped in a single
  // encrypted ContentInfo.
  if (cert != NULL || sk_X509_num(chain) > 0) {
    if (cert_nid < 0) {
      // Place the certificates in an unencrypted ContentInfo. This could be
      // more compactly-encoded by reusing the same ContentInfo as the key, but
      // OpenSSL does not do this. We keep them separate for consistency. (Keys,
      // even when encrypted, are always placed in unencrypted ContentInfos.
      // PKCS#12 defines bag-level encryption for keys.)
      CBB content_info, oid, wrapper, data;
      if (!CBB_add_asn1(&content_infos, &content_info, CBS_ASN1_SEQUENCE) ||
          !CBB_add_asn1(&content_info, &oid, CBS_ASN1_OBJECT) ||
          !CBB_add_bytes(&oid, kPKCS7Data, sizeof(kPKCS7Data)) ||
          !CBB_add_asn1(&content_info, &wrapper,
                        CBS_ASN1_CONSTRUCTED | CBS_ASN1_CONTEXT_SPECIFIC | 0) ||
          !CBB_add_asn1(&wrapper, &data, CBS_ASN1_OCTETSTRING) ||
          !add_cert_safe_contents(&data, cert, chain, name, key_id,
                                  key_id_len) ||
          !CBB_flush(&content_infos)) {
        goto err;
      }
    } else {
      CBB plaintext_cbb;
      int ok = CBB_init(&plaintext_cbb, 0) &&
               add_cert_safe_contents(&plaintext_cbb, cert, chain, name, key_id,
                                      key_id_len) &&
               add_encrypted_data(
                   &content_infos, cert_nid, password, password_len, iterations,
                   CBB_data(&plaintext_cbb), CBB_len(&plaintext_cbb));
      CBB_cleanup(&plaintext_cbb);
      if (!ok) {
        goto err;
      }
    }
  }

  // If there is a key, place it in a single KeyBag or PKCS8ShroudedKeyBag
  // wrapped in an unencrypted ContentInfo. (One could also place it in a KeyBag
  // inside an encrypted ContentInfo, but OpenSSL does not do this and some
  // PKCS#12 consumers do not support KeyBags.)
  if (pkey != NULL) {
    CBB content_info, oid, wrapper, data, safe_contents, bag, bag_oid,
        bag_contents;
    if (// Add another data ContentInfo.
        !CBB_add_asn1(&content_infos, &content_info, CBS_ASN1_SEQUENCE) ||
        !CBB_add_asn1(&content_info, &oid, CBS_ASN1_OBJECT) ||
        !CBB_add_bytes(&oid, kPKCS7Data, sizeof(kPKCS7Data)) ||
        !CBB_add_asn1(&content_info, &wrapper,
                      CBS_ASN1_CONSTRUCTED | CBS_ASN1_CONTEXT_SPECIFIC | 0) ||
        !CBB_add_asn1(&wrapper, &data, CBS_ASN1_OCTETSTRING) ||
        !CBB_add_asn1(&data, &safe_contents, CBS_ASN1_SEQUENCE) ||
        // Add a SafeBag containing a PKCS8ShroudedKeyBag.
        !CBB_add_asn1(&safe_contents, &bag, CBS_ASN1_SEQUENCE) ||
        !CBB_add_asn1(&bag, &bag_oid, CBS_ASN1_OBJECT)) {
      goto err;
    }
    if (key_nid < 0) {
      if (!CBB_add_bytes(&bag_oid, kKeyBag, sizeof(kKeyBag)) ||
          !CBB_add_asn1(&bag, &bag_contents,
                        CBS_ASN1_CONSTRUCTED | CBS_ASN1_CONTEXT_SPECIFIC | 0) ||
          !EVP_marshal_private_key(&bag_contents, pkey)) {
        goto err;
      }
    } else {
      if (!CBB_add_bytes(&bag_oid, kPKCS8ShroudedKeyBag,
                         sizeof(kPKCS8ShroudedKeyBag)) ||
          !CBB_add_asn1(&bag, &bag_contents,
                        CBS_ASN1_CONSTRUCTED | CBS_ASN1_CONTEXT_SPECIFIC | 0) ||
          !PKCS8_marshal_encrypted_private_key(
              &bag_contents, key_nid, NULL, password, password_len,
              NULL /* generate a random salt */,
              0 /* use default salt length */, iterations, pkey)) {
        goto err;
      }
    }
    size_t name_len = 0;
    if (name) {
      name_len = strlen(name);
    }
    if (!add_bag_attributes(&bag, name, name_len, key_id, key_id_len) ||
        !CBB_flush(&content_infos)) {
      goto err;
    }
  }

  // Compute the MAC. Match OpenSSL in using SHA-1 as the hash function. The MAC
  // covers |auth_safe_data|.
  // TODO (CryptoAlg-2897): Update the default |md| to SHA-256 to align with
  //                        OpenSSL 3.x.
  const EVP_MD *mac_md = EVP_sha1();
  uint8_t mac_salt[PKCS12_SALT_LEN];
  AWSLC_ABORT_IF_NOT_ONE(RAND_bytes(mac_salt, sizeof(mac_salt)));
  if (!CBB_flush(&auth_safe_data) ||
      !pkcs12_gen_and_write_mac(
          &pfx, CBB_data(&auth_safe_data), CBB_len(&auth_safe_data), password,
          password_len, mac_salt, sizeof(mac_salt), mac_iterations, mac_md)) {
    goto err;
  }

  ret = PKCS12_new();
  if (ret == NULL ||
      !CBB_finish(&cbb, &ret->ber_bytes, &ret->ber_len)) {
    OPENSSL_free(ret);
    ret = NULL;
    goto err;
  }

err:
  CBB_cleanup(&cbb);
  return ret;
}

PKCS12 *PKCS12_new(void) {
  return OPENSSL_zalloc(sizeof(PKCS12));
}

void PKCS12_free(PKCS12 *p12) {
  if (p12 == NULL) {
    return;
  }
  OPENSSL_free(p12->ber_bytes);
  OPENSSL_free(p12);
}

int PKCS12_set_mac(PKCS12 *p12, const char *password, int password_len,
                   unsigned char *salt, int salt_len, int mac_iterations,
                   const EVP_MD *md) {
  GUARD_PTR(p12);
  int ret = 0;
  uint8_t *storage = NULL;

  // Validate and normalize |password_len|. A value of -1 means the password is
  // NUL-terminated and we should use strlen, matching |PKCS12_verify_mac|.
  if (password == NULL) {
    password_len = 0;
  } else if (password_len == -1) {
    password_len = (int)strlen(password);
  } else if (password_len < 0) {
    return 0;
  }

  if (mac_iterations == 0) {
    mac_iterations = 1;
  }
  if (salt_len < 0) {
    return 0;
  }
  if (salt_len == 0) {
    salt_len = PKCS12_SALT_LEN;
  }
  // Generate |mac_salt| if |salt| is NULL and copy if NULL.
  uint8_t *mac_salt = OPENSSL_malloc(salt_len);
  if (mac_salt == NULL) {
    goto out;
  }
  if (salt == NULL) {
    AWSLC_ABORT_IF_NOT_ONE(RAND_bytes(mac_salt, salt_len));
  } else {
    OPENSSL_memcpy(mac_salt, salt, salt_len);
  }
  // TODO (CryptoAlg-2897): Update the default |md| to SHA-256 to align with
  //                        OpenSSL 3.x.
  if (md == NULL) {
    md = EVP_sha1();
  }

  CBS ber_bytes, in, pfx, authsafe, content_type, wrapped_authsafes, authsafes;
  uint64_t version;
  // The input may be in BER format.
  CBS_init(&ber_bytes, p12->ber_bytes, p12->ber_len);
  if (!CBS_asn1_ber_to_der(&ber_bytes, &in, &storage)) {
    OPENSSL_PUT_ERROR(PKCS8, PKCS8_R_BAD_PKCS12_DATA);
    goto out;
  }
  if (!CBS_get_asn1(&in, &pfx, CBS_ASN1_SEQUENCE) || CBS_len(&in) != 0 ||
      !CBS_get_asn1_uint64(&pfx, &version)) {
    OPENSSL_PUT_ERROR(PKCS8, PKCS8_R_BAD_PKCS12_DATA);
    goto out;
  }
  if (version < 3) {
    OPENSSL_PUT_ERROR(PKCS8, PKCS8_R_BAD_PKCS12_VERSION);
    goto out;
  }

  if (!CBS_get_asn1(&pfx, &authsafe, CBS_ASN1_SEQUENCE)) {
    OPENSSL_PUT_ERROR(PKCS8, PKCS8_R_BAD_PKCS12_DATA);
    goto out;
  }
  // Save contents of |authsafe| to write back before the CBS is advanced.
  const uint8_t *orig_authsafe = CBS_data(&authsafe);
  size_t orig_authsafe_len = CBS_len(&authsafe);

  // Parse for |authsafes| which is the data that we should be running HMAC on.
  if (!CBS_get_asn1(&authsafe, &content_type, CBS_ASN1_OBJECT) ||
      !CBS_get_asn1(&authsafe, &wrapped_authsafes,
                    CBS_ASN1_CONTEXT_SPECIFIC | CBS_ASN1_CONSTRUCTED | 0) ||
      !CBS_get_asn1(&wrapped_authsafes, &authsafes, CBS_ASN1_OCTETSTRING)) {
    OPENSSL_PUT_ERROR(PKCS8, PKCS8_R_BAD_PKCS12_DATA);
    goto out;
  }

  // Rewrite contents of |p12| with the original contents and updated MAC.
  CBB cbb, out_pfx, out_auth_safe;
  if (!CBB_init(&cbb, 0) || !CBB_add_asn1(&cbb, &out_pfx, CBS_ASN1_SEQUENCE) ||
      !CBB_add_asn1_uint64(&out_pfx, version) ||
      !CBB_add_asn1(&out_pfx, &out_auth_safe, CBS_ASN1_SEQUENCE) ||
      !CBB_add_bytes(&out_auth_safe, orig_authsafe, orig_authsafe_len) ||
      !pkcs12_gen_and_write_mac(&out_pfx, CBS_data(&authsafes),
                                CBS_len(&authsafes), password, password_len,
                                mac_salt, salt_len, mac_iterations, md)) {
    CBB_cleanup(&cbb);
    goto out;
  }

  // Free the old buffer and null it out before |CBB_finish| so that |p12|
  // remains in a safe (empty) state rather than holding a dangling pointer
  // if |CBB_finish| fails.
  OPENSSL_free(p12->ber_bytes);
  p12->ber_bytes = NULL;
  p12->ber_len = 0;

  // Verify that the new password is consistent with the original. This is
  // behavior specific to AWS-LC.
  if (!CBB_finish(&cbb, &p12->ber_bytes, &p12->ber_len) ||
      !PKCS12_verify_mac(p12, password, password_len)) {
    CBB_cleanup(&cbb);
    goto out;
  }

  ret = 1;

out:
  OPENSSL_free(storage);
  OPENSSL_free(mac_salt);
  return ret;
}
