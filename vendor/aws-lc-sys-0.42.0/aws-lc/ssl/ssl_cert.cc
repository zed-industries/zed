// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com)
// Copyright (c) 1998-2007 The OpenSSL Project.  All rights reserved.
// Copyright 2002 Sun Microsystems, Inc. ALL RIGHTS RESERVED.
//
// ECC cipher suite support in OpenSSL originally developed by
// SUN MICROSYSTEMS, INC., and contributed to the OpenSSL project.
//
// SPDX-License-Identifier: Apache-2.0

#include <openssl/ssl.h>

#include <assert.h>
#include <limits.h>
#include <string.h>

#include <utility>

#include <openssl/bn.h>
#include <openssl/bytestring.h>
#include <openssl/ec_key.h>
#include <openssl/err.h>
#include <openssl/mem.h>
#include <openssl/sha.h>
#include <openssl/x509.h>

#include "../crypto/internal.h"
#include "internal.h"


BSSL_NAMESPACE_BEGIN

CERT::CERT(const SSL_X509_METHOD *x509_method_arg)
    : x509_method(x509_method_arg) {
  this->cert_private_key_idx = SSL_PKEY_RSA;
  if (!this->cert_private_keys.Init(SSL_PKEY_SIZE)) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_MALLOC_FAILURE);
    return;
  }
}

CERT::~CERT() { x509_method->cert_free(this); }

bool CERT::SetKeyMethod(const SSL_PRIVATE_KEY_METHOD *method, int slot_idx) {
  key_method = method;
  if (slot_idx >= 0 &&
      slot_idx < static_cast<int>(cert_private_keys.size())) {
    cert_private_keys[slot_idx].privatekey.reset();
    return true;
  }
  // If |method| is non-NULL and |slot_idx| is out of range, the mutual
  // exclusivity invariant with per-slot |privatekey| cannot be enforced.
  return method == nullptr;
}

bool CERT::SetSlotPrivateKey(int slot_idx, EVP_PKEY *pkey) {
  if (slot_idx < 0 ||
      slot_idx >= static_cast<int>(cert_private_keys.size())) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
    return false;
  }
  cert_private_keys[slot_idx].privatekey = UpRef(pkey);
  key_method = nullptr;
  return true;
}

CRYPTO_BUFFER *buffer_up_ref(const CRYPTO_BUFFER *buffer) {
  CRYPTO_BUFFER_up_ref(const_cast<CRYPTO_BUFFER *>(buffer));
  return const_cast<CRYPTO_BUFFER *>(buffer);
}

UniquePtr<CERT> ssl_cert_dup(CERT *cert) {
  if (cert == nullptr) {
    return nullptr;
  }

  UniquePtr<CERT> ret = MakeUnique<CERT>(cert->x509_method);
  if (!ret) {
    return nullptr;
  }

  ret->cert_private_key_idx = cert->cert_private_key_idx;
  if (!ssl_cert_check_cert_private_keys_usage(cert) ||
      !ssl_cert_check_cert_private_keys_usage(ret.get())) {
    return nullptr;
  }
  for (int i = 0; i < SSL_PKEY_SIZE; i++) {
    CERT_PKEY &cert_pkey = cert->cert_private_keys[i];
    CERT_PKEY &ret_pkey = ret->cert_private_keys[i];

    if (cert_pkey.chain) {
      ret_pkey.chain.reset(sk_CRYPTO_BUFFER_deep_copy(
          cert_pkey.chain.get(), buffer_up_ref, CRYPTO_BUFFER_free));
      if (!ret_pkey.chain) {
        return nullptr;
      }
    }

    if (cert_pkey.x509_leaf != nullptr) {
      X509_up_ref(cert_pkey.x509_leaf);
      ret_pkey.x509_leaf = cert_pkey.x509_leaf;
    }

    ret_pkey.privatekey = UpRef(cert_pkey.privatekey);
  }

  ret->key_method = cert->key_method;

  if (!ret->sigalgs.CopyFrom(cert->sigalgs)) {
    return nullptr;
  }

  ret->cert_cb = cert->cert_cb;
  ret->cert_cb_arg = cert->cert_cb_arg;

  ret->x509_method->cert_dup(ret.get(), cert);

  ret->signed_cert_timestamp_list = UpRef(cert->signed_cert_timestamp_list);
  ret->ocsp_response = UpRef(cert->ocsp_response);

  ret->sid_ctx_length = cert->sid_ctx_length;
  OPENSSL_memcpy(ret->sid_ctx, cert->sid_ctx, sizeof(ret->sid_ctx));

  if (cert->dc) {
    ret->dc = cert->dc->Dup();
    if (!ret->dc) {
      return nullptr;
    }
  }

  ret->dc_privatekey = UpRef(cert->dc_privatekey);
  ret->dc_key_method = cert->dc_key_method;

  return ret;
}

static void ssl_cert_set_cert_cb(CERT *cert, int (*cb)(SSL *ssl, void *arg),
                                 void *arg) {
  cert->cert_cb = cb;
  cert->cert_cb_arg = arg;
}

enum leaf_cert_and_privkey_result_t {
  leaf_cert_and_privkey_error,
  leaf_cert_and_privkey_ok,
  leaf_cert_and_privkey_mismatch,
};

// do_leaf_cert_and_privkey_checks does the necessary checks against |pubkey|
// and |privkey|. It's  expected that the |pubkey| has been parsed from
// |cert_cbs|.
static enum leaf_cert_and_privkey_result_t do_leaf_cert_and_privkey_checks(
    const CBS *cert_cbs, EVP_PKEY *pubkey, EVP_PKEY *privkey) {
  if (!ssl_is_key_type_supported(EVP_PKEY_id(pubkey))) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_UNKNOWN_CERTIFICATE_TYPE);
    return leaf_cert_and_privkey_error;
  }

  // An ECC certificate may be usable for ECDH or ECDSA. We only support ECDSA
  // certificates, so sanity-check the key usage extension.
  if (EVP_PKEY_id(pubkey) == EVP_PKEY_EC &&
      !ssl_cert_check_key_usage(cert_cbs, key_usage_digital_signature)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_UNKNOWN_CERTIFICATE_TYPE);
    return leaf_cert_and_privkey_error;
  }

  if (privkey != nullptr &&
      // Sanity-check that the private key and the certificate match.
      !ssl_compare_public_and_private_key(pubkey, privkey)) {
    ERR_clear_error();
    return leaf_cert_and_privkey_mismatch;
  }

  return leaf_cert_and_privkey_ok;
}

// check_leaf_cert_and_privkey checks whether the certificate in |leaf_buffer|
// and the private key in |privkey| are suitable and coherent. It returns
// |leaf_cert_and_privkey_error| and pushes to the error queue if a problem is
// found. If the certificate and private key are valid, but incoherent, it
// returns |leaf_cert_and_privkey_mismatch|. Otherwise it returns
// |leaf_cert_and_privkey_ok|.
static enum leaf_cert_and_privkey_result_t check_leaf_cert_and_privkey(
    CRYPTO_BUFFER *leaf_buffer, EVP_PKEY *privkey, int *out_pubslot_idx) {
  CBS cert_cbs;
  CRYPTO_BUFFER_init_CBS(leaf_buffer, &cert_cbs);
  UniquePtr<EVP_PKEY> pubkey = ssl_cert_parse_pubkey(&cert_cbs);
  if (pubkey == nullptr) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_DECODE_ERROR);
    return leaf_cert_and_privkey_error;
  }
  enum leaf_cert_and_privkey_result_t result = do_leaf_cert_and_privkey_checks(
      &cert_cbs, pubkey.get(), privkey);
  if(out_pubslot_idx != nullptr) {
    if(result == leaf_cert_and_privkey_ok) {
      *out_pubslot_idx = ssl_get_certificate_slot_index(pubkey.get());
    } else {
      *out_pubslot_idx = -1;
    }
  }
  return result;
}

static int cert_set_chain_and_key(
    CERT *cert, UniquePtr<STACK_OF(CRYPTO_BUFFER)> &certs, size_t num_certs,
    EVP_PKEY *privkey, const SSL_PRIVATE_KEY_METHOD *privkey_method) {
  if (num_certs == 0 || (privkey == NULL && privkey_method == NULL)) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }

  if (privkey != NULL && privkey_method != NULL) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_CANNOT_HAVE_BOTH_PRIVKEY_AND_METHOD);
    return 0;
  }

  CRYPTO_BUFFER *leaf_buf = sk_CRYPTO_BUFFER_value(certs.get(), 0);
  if (leaf_buf == nullptr) {
    return 0;
  }

  int slot_idx = -1;

  switch (check_leaf_cert_and_privkey(leaf_buf, privkey, &slot_idx)) {
    case leaf_cert_and_privkey_error:
      return 0;
    case leaf_cert_and_privkey_mismatch:
      OPENSSL_PUT_ERROR(SSL, SSL_R_CERTIFICATE_AND_PRIVATE_KEY_MISMATCH);
      return 0;
    case leaf_cert_and_privkey_ok:
      break;
  }
  assert(slot_idx >= 0);

  if (!ssl_cert_check_cert_private_keys_usage(cert)) {
    return 0;
  }

  // Certificate slot validity already checked and set by
  // |check_leaf_cert_and_privkey|.
  CERT_PKEY *cert_pkey = &cert->cert_private_keys[slot_idx];

  if(privkey != nullptr) {
    if (!cert->SetSlotPrivateKey(slot_idx, privkey)) {
      return 0;
    }
  } else {
    if (!cert->SetKeyMethod(privkey_method, slot_idx)) {
      return 0;
    }
  }

  if (cert_pkey->chain) {
    cert_pkey->chain.reset();
  }
  cert_pkey->chain = std::move(certs);

  // Invalidate the parsed X509 leaf and chain caches since the backing
  // CRYPTO_BUFFER chain was just replaced. Dispatch through |x509_method| to
  // respect the abstraction boundary and to correctly handle the noop case
  // (e.g. TLS_with_buffers_method) where these are no-ops.
  cert->x509_method->cert_flush_cached_chain(cert);
  cert->x509_method->cert_flush_leaf(cert);

  cert->cert_private_key_idx = slot_idx;

  return 1;
}

bool ssl_set_cert(CERT *cert, UniquePtr<CRYPTO_BUFFER> buffer) {
  if (!ssl_cert_check_cert_private_keys_usage(cert)) {
    return false;
  }

  // Parse the right certificate slot index from |buffer|.
  CBS cert_cbs;
  CRYPTO_BUFFER_init_CBS(buffer.get(), &cert_cbs);
  UniquePtr<EVP_PKEY> pubkey = ssl_cert_parse_pubkey(&cert_cbs);
  if (!pubkey) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_DECODE_ERROR);
    return false;
  }
  int slot_index = ssl_get_certificate_slot_index(pubkey.get());
  if (slot_index < 0) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_UNKNOWN_CERTIFICATE_TYPE);
    return false;
  }

  CERT_PKEY &cert_pkey = cert->cert_private_keys[slot_index];
  switch (do_leaf_cert_and_privkey_checks(&cert_cbs, pubkey.get(),
                                          cert_pkey.privatekey.get())) {
    case leaf_cert_and_privkey_error:
      return false;
    case leaf_cert_and_privkey_mismatch:
      // don't fail for a cert/key mismatch, just free current private key
      // (when switching to a different cert & key, first this function should
      // be used, then |ssl_set_pkey|.
      cert_pkey.privatekey.reset();
      break;
    case leaf_cert_and_privkey_ok:
      break;
  }

  if (cert_pkey.chain != nullptr) {
    CRYPTO_BUFFER_free(sk_CRYPTO_BUFFER_value(cert_pkey.chain.get(), 0));
    sk_CRYPTO_BUFFER_set(cert_pkey.chain.get(), 0, buffer.release());

    // The backing CRYPTO_BUFFER leaf was just replaced; invalidate the cached
    // X509 leaf to prevent stale data from being returned to callers.
    cert->x509_method->cert_flush_leaf(cert);

    // Update certificate slot index if all checks have passed.
    cert->cert_private_key_idx = slot_index;
    return true;
  }

  cert_pkey.chain.reset(sk_CRYPTO_BUFFER_new_null());
  if (cert_pkey.chain == nullptr) {
    return false;
  }

  if (!PushToStack(cert_pkey.chain.get(), std::move(buffer))) {
    cert_pkey.chain.reset();
    return false;
  }

  // Update certificate slot index if all checks have passed.
  cert->cert_private_key_idx = slot_index;
  return true;
}

bool ssl_has_certificate(const SSL_HANDSHAKE *hs) {
  if (!ssl_cert_check_cert_private_keys_usage(hs->config->cert.get())) {
    return false;
  }

  CERT_PKEY &cert_pkey =
      hs->config->cert
          ->cert_private_keys[hs->config->cert->cert_private_key_idx];
  return cert_pkey.chain != nullptr &&
         sk_CRYPTO_BUFFER_value(cert_pkey.chain.get(), 0) != nullptr &&
         ssl_has_private_key(hs);
}

bool ssl_parse_cert_chain(uint8_t *out_alert,
                          UniquePtr<STACK_OF(CRYPTO_BUFFER)> *out_chain,
                          UniquePtr<EVP_PKEY> *out_pubkey,
                          uint8_t *out_leaf_sha256, CBS *cbs,
                          CRYPTO_BUFFER_POOL *pool) {
  out_chain->reset();
  out_pubkey->reset();

  CBS certificate_list;
  if (!CBS_get_u24_length_prefixed(cbs, &certificate_list)) {
    *out_alert = SSL_AD_DECODE_ERROR;
    OPENSSL_PUT_ERROR(SSL, SSL_R_DECODE_ERROR);
    return false;
  }

  if (CBS_len(&certificate_list) == 0) {
    return true;
  }

  UniquePtr<STACK_OF(CRYPTO_BUFFER)> chain(sk_CRYPTO_BUFFER_new_null());
  if (!chain) {
    *out_alert = SSL_AD_INTERNAL_ERROR;
    return false;
  }

  UniquePtr<EVP_PKEY> pubkey;
  while (CBS_len(&certificate_list) > 0) {
    CBS certificate;
    if (!CBS_get_u24_length_prefixed(&certificate_list, &certificate) ||
        CBS_len(&certificate) == 0) {
      *out_alert = SSL_AD_DECODE_ERROR;
      OPENSSL_PUT_ERROR(SSL, SSL_R_CERT_LENGTH_MISMATCH);
      return false;
    }

    if (sk_CRYPTO_BUFFER_num(chain.get()) == 0) {
      pubkey = ssl_cert_parse_pubkey(&certificate);
      if (!pubkey) {
        *out_alert = SSL_AD_DECODE_ERROR;
        return false;
      }

      // Retain the hash of the leaf certificate if requested.
      if (out_leaf_sha256 != NULL) {
        SHA256(CBS_data(&certificate), CBS_len(&certificate), out_leaf_sha256);
      }
    }

    UniquePtr<CRYPTO_BUFFER> buf(
        CRYPTO_BUFFER_new_from_CBS(&certificate, pool));
    if (!buf || !PushToStack(chain.get(), std::move(buf))) {
      *out_alert = SSL_AD_INTERNAL_ERROR;
      return false;
    }
  }

  *out_chain = std::move(chain);
  *out_pubkey = std::move(pubkey);
  return true;
}

bool ssl_add_cert_chain(SSL_HANDSHAKE *hs, CBB *cbb) {
  if (!ssl_has_certificate(hs)) {
    return CBB_add_u24(cbb, 0);
  }

  CBB certs;
  if (!CBB_add_u24_length_prefixed(cbb, &certs)) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
    return false;
  }

  // |cert_private_keys| already checked above in |ssl_has_certificate|.
  int idx = hs->config->cert->cert_private_key_idx;
  STACK_OF(CRYPTO_BUFFER) *chain =
      hs->config->cert->cert_private_keys[idx].chain.get();
  for (size_t i = 0; i < sk_CRYPTO_BUFFER_num(chain); i++) {
    CRYPTO_BUFFER *buffer = sk_CRYPTO_BUFFER_value(chain, i);
    CBB child;
    if (!CBB_add_u24_length_prefixed(&certs, &child) ||
        !CBB_add_bytes(&child, CRYPTO_BUFFER_data(buffer),
                       CRYPTO_BUFFER_len(buffer)) ||
        !CBB_flush(&certs)) {
      OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
      return false;
    }
  }

  return CBB_flush(cbb);
}

// ssl_cert_skip_to_spki parses a DER-encoded, X.509 certificate from |in| and
// positions |*out_tbs_cert| to cover the TBSCertificate, starting at the
// subjectPublicKeyInfo.
static bool ssl_cert_skip_to_spki(const CBS *in, CBS *out_tbs_cert) {
  /* From RFC 5280, section 4.1
   *    Certificate  ::=  SEQUENCE  {
   *      tbsCertificate       TBSCertificate,
   *      signatureAlgorithm   AlgorithmIdentifier,
   *      signatureValue       BIT STRING  }

   * TBSCertificate  ::=  SEQUENCE  {
   *      version         [0]  EXPLICIT Version DEFAULT v1,
   *      serialNumber         CertificateSerialNumber,
   *      signature            AlgorithmIdentifier,
   *      issuer               Name,
   *      validity             Validity,
   *      subject              Name,
   *      subjectPublicKeyInfo SubjectPublicKeyInfo,
   *      ... } */
  CBS buf = *in;

  CBS toplevel;
  if (!CBS_get_asn1(&buf, &toplevel, CBS_ASN1_SEQUENCE) || CBS_len(&buf) != 0 ||
      !CBS_get_asn1(&toplevel, out_tbs_cert, CBS_ASN1_SEQUENCE) ||
      // version
      !CBS_get_optional_asn1(
          out_tbs_cert, NULL, NULL,
          CBS_ASN1_CONSTRUCTED | CBS_ASN1_CONTEXT_SPECIFIC | 0) ||
      // serialNumber
      !CBS_get_asn1(out_tbs_cert, NULL, CBS_ASN1_INTEGER) ||
      // signature algorithm
      !CBS_get_asn1(out_tbs_cert, NULL, CBS_ASN1_SEQUENCE) ||
      // issuer
      !CBS_get_asn1(out_tbs_cert, NULL, CBS_ASN1_SEQUENCE) ||
      // validity
      !CBS_get_asn1(out_tbs_cert, NULL, CBS_ASN1_SEQUENCE) ||
      // subject
      !CBS_get_asn1(out_tbs_cert, NULL, CBS_ASN1_SEQUENCE)) {
    return false;
  }

  return true;
}

UniquePtr<EVP_PKEY> ssl_cert_parse_pubkey(const CBS *in) {
  CBS buf = *in, tbs_cert;
  if (!ssl_cert_skip_to_spki(&buf, &tbs_cert)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_CANNOT_PARSE_LEAF_CERT);
    return nullptr;
  }

  return UniquePtr<EVP_PKEY>(EVP_parse_public_key(&tbs_cert));
}

bool ssl_compare_public_and_private_key(const EVP_PKEY *pubkey,
                                        const EVP_PKEY *privkey) {
  if (EVP_PKEY_is_opaque(privkey)) {
    // We cannot check an opaque private key and have to trust that it
    // matches.
    return true;
  }

  switch (EVP_PKEY_cmp(pubkey, privkey)) {
    case 1:
      return true;
    case 0:
      OPENSSL_PUT_ERROR(X509, X509_R_KEY_VALUES_MISMATCH);
      return false;
    case -1:
      OPENSSL_PUT_ERROR(X509, X509_R_KEY_TYPE_MISMATCH);
      return false;
    case -2:
      OPENSSL_PUT_ERROR(X509, X509_R_UNKNOWN_KEY_TYPE);
      return false;
  }

  assert(0);
  return false;
}

bool ssl_cert_check_private_key(const CERT *cert, const EVP_PKEY *privkey) {
  if (privkey == nullptr) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_NO_PRIVATE_KEY_ASSIGNED);
    return false;
  }

  // |cert_private_keys| already checked before usages of
  // |ssl_cert_check_private_key|.
  STACK_OF(CRYPTO_BUFFER) *chain =
      cert->cert_private_keys[cert->cert_private_key_idx].chain.get();

  if (chain == nullptr || sk_CRYPTO_BUFFER_value(chain, 0) == nullptr) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_NO_CERTIFICATE_ASSIGNED);
    return false;
  }

  CBS cert_cbs;
  CRYPTO_BUFFER_init_CBS(sk_CRYPTO_BUFFER_value(chain, 0), &cert_cbs);
  UniquePtr<EVP_PKEY> pubkey = ssl_cert_parse_pubkey(&cert_cbs);
  if (!pubkey) {
    OPENSSL_PUT_ERROR(X509, X509_R_UNKNOWN_KEY_TYPE);
    return false;
  }

  return ssl_compare_public_and_private_key(pubkey.get(), privkey);
}

bool ssl_cert_check_key_usage(const CBS *in, enum ssl_key_usage_t bit) {
  CBS buf = *in;

  CBS tbs_cert, outer_extensions;
  int has_extensions;
  if (!ssl_cert_skip_to_spki(&buf, &tbs_cert) ||
      // subjectPublicKeyInfo
      !CBS_get_asn1(&tbs_cert, NULL, CBS_ASN1_SEQUENCE) ||
      // issuerUniqueID
      !CBS_get_optional_asn1(&tbs_cert, NULL, NULL,
                             CBS_ASN1_CONTEXT_SPECIFIC | 1) ||
      // subjectUniqueID
      !CBS_get_optional_asn1(&tbs_cert, NULL, NULL,
                             CBS_ASN1_CONTEXT_SPECIFIC | 2) ||
      !CBS_get_optional_asn1(
          &tbs_cert, &outer_extensions, &has_extensions,
          CBS_ASN1_CONSTRUCTED | CBS_ASN1_CONTEXT_SPECIFIC | 3)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_CANNOT_PARSE_LEAF_CERT);
    return false;
  }

  if (!has_extensions) {
    return true;
  }

  CBS extensions;
  if (!CBS_get_asn1(&outer_extensions, &extensions, CBS_ASN1_SEQUENCE)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_CANNOT_PARSE_LEAF_CERT);
    return false;
  }

  while (CBS_len(&extensions) > 0) {
    CBS extension, oid, contents;
    if (!CBS_get_asn1(&extensions, &extension, CBS_ASN1_SEQUENCE) ||
        !CBS_get_asn1(&extension, &oid, CBS_ASN1_OBJECT) ||
        (CBS_peek_asn1_tag(&extension, CBS_ASN1_BOOLEAN) &&
         !CBS_get_asn1(&extension, NULL, CBS_ASN1_BOOLEAN)) ||
        !CBS_get_asn1(&extension, &contents, CBS_ASN1_OCTETSTRING) ||
        CBS_len(&extension) != 0) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_CANNOT_PARSE_LEAF_CERT);
      return false;
    }

    static const uint8_t kKeyUsageOID[3] = {0x55, 0x1d, 0x0f};
    if (CBS_len(&oid) != sizeof(kKeyUsageOID) ||
        OPENSSL_memcmp(CBS_data(&oid), kKeyUsageOID, sizeof(kKeyUsageOID)) !=
            0) {
      continue;
    }

    CBS bit_string;
    if (!CBS_get_asn1(&contents, &bit_string, CBS_ASN1_BITSTRING) ||
        CBS_len(&contents) != 0) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_CANNOT_PARSE_LEAF_CERT);
      return false;
    }

    // This is the KeyUsage extension. See
    // https://tools.ietf.org/html/rfc5280#section-4.2.1.3
    if (!CBS_is_valid_asn1_bitstring(&bit_string)) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_CANNOT_PARSE_LEAF_CERT);
      return false;
    }

    if (!CBS_asn1_bitstring_has_bit(&bit_string, bit)) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_KEY_USAGE_BIT_INCORRECT);
      return false;
    }

    return true;
  }

  // No KeyUsage extension found.
  return true;
}

UniquePtr<STACK_OF(CRYPTO_BUFFER)> ssl_parse_client_CA_list(SSL *ssl,
                                                            uint8_t *out_alert,
                                                            CBS *cbs) {
  CRYPTO_BUFFER_POOL *const pool = ssl->ctx->pool;

  UniquePtr<STACK_OF(CRYPTO_BUFFER)> ret(sk_CRYPTO_BUFFER_new_null());
  if (!ret) {
    *out_alert = SSL_AD_INTERNAL_ERROR;
    return nullptr;
  }

  CBS child;
  if (!CBS_get_u16_length_prefixed(cbs, &child)) {
    *out_alert = SSL_AD_DECODE_ERROR;
    OPENSSL_PUT_ERROR(SSL, SSL_R_LENGTH_MISMATCH);
    return nullptr;
  }

  while (CBS_len(&child) > 0) {
    CBS distinguished_name;
    if (!CBS_get_u16_length_prefixed(&child, &distinguished_name)) {
      *out_alert = SSL_AD_DECODE_ERROR;
      OPENSSL_PUT_ERROR(SSL, SSL_R_CA_DN_TOO_LONG);
      return nullptr;
    }

    UniquePtr<CRYPTO_BUFFER> buffer(
        CRYPTO_BUFFER_new_from_CBS(&distinguished_name, pool));
    if (!buffer || !PushToStack(ret.get(), std::move(buffer))) {
      *out_alert = SSL_AD_INTERNAL_ERROR;
      return nullptr;
    }
  }

  if (!ssl->ctx->x509_method->check_client_CA_list(ret.get())) {
    *out_alert = SSL_AD_DECODE_ERROR;
    OPENSSL_PUT_ERROR(SSL, SSL_R_DECODE_ERROR);
    return nullptr;
  }

  return ret;
}

bool ssl_has_client_CAs(const SSL_CONFIG *cfg) {
  const STACK_OF(CRYPTO_BUFFER) *names = cfg->client_CA.get();
  if (names == nullptr) {
    names = cfg->ssl->ctx->client_CA.get();
  }
  if (names == nullptr) {
    return false;
  }
  return sk_CRYPTO_BUFFER_num(names) > 0;
}

bool ssl_add_client_CA_list(SSL_HANDSHAKE *hs, CBB *cbb) {
  CBB child, name_cbb;
  if (!CBB_add_u16_length_prefixed(cbb, &child)) {
    return false;
  }

  const STACK_OF(CRYPTO_BUFFER) *names = hs->config->client_CA.get();
  if (names == NULL) {
    names = hs->ssl->ctx->client_CA.get();
  }
  if (names == NULL) {
    return CBB_flush(cbb);
  }

  for (const CRYPTO_BUFFER *name : names) {
    if (!CBB_add_u16_length_prefixed(&child, &name_cbb) ||
        !CBB_add_bytes(&name_cbb, CRYPTO_BUFFER_data(name),
                       CRYPTO_BUFFER_len(name))) {
      return false;
    }
  }

  return CBB_flush(cbb);
}

bool ssl_check_leaf_certificate(SSL_HANDSHAKE *hs, EVP_PKEY *pkey,
                                const CRYPTO_BUFFER *leaf) {
  assert(ssl_protocol_version(hs->ssl) < TLS1_3_VERSION);

  // Check the certificate's type matches the cipher.
  if (!(hs->new_cipher->algorithm_auth & ssl_cipher_auth_mask_for_key(pkey))) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_WRONG_CERTIFICATE_TYPE);
    return false;
  }

  if (EVP_PKEY_id(pkey) == EVP_PKEY_EC) {
    // Check the key's group and point format are acceptable.
    EC_KEY *ec_key = EVP_PKEY_get0_EC_KEY(pkey);
    uint16_t group_id;
    if (!ssl_nid_to_group_id(
            &group_id, EC_GROUP_get_curve_name(EC_KEY_get0_group(ec_key))) ||
        !tls1_check_group_id(hs, group_id) ||
        EC_KEY_get_conv_form(ec_key) != POINT_CONVERSION_UNCOMPRESSED) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_BAD_ECC_CERT);
      return false;
    }
  }

  return true;
}

bool ssl_on_certificate_selected(SSL_HANDSHAKE *hs) {
  SSL *const ssl = hs->ssl;
  if (!ssl_has_certificate(hs)) {
    // Nothing to do.
    return true;
  }

  if (!ssl->ctx->x509_method->ssl_auto_chain_if_needed(hs) ||
      !ssl_handshake_load_local_pubkey(hs)) {
    return false;
  }

  // Sanity check that cached certificate public key type matches the chosen
  // certificate slot index type.
  assert(ssl_signing_with_dc(hs) ||
         (ssl_get_certificate_slot_index(hs->local_pubkey.get()) ==
          hs->config->cert->cert_private_key_idx));

  return true;
}

bool ssl_handshake_load_local_pubkey(SSL_HANDSHAKE *hs) {
  if (!ssl_cert_check_cert_private_keys_usage(hs->config->cert.get())) {
    return false;
  }

  STACK_OF(CRYPTO_BUFFER) *chain =
      hs->config->cert
          ->cert_private_keys[hs->config->cert->cert_private_key_idx]
          .chain.get();

  if (ssl_signing_with_dc(hs)) {
    hs->local_pubkey = UpRef(hs->config->cert->dc->pkey);
  } else {
    hs->local_pubkey = ssl_cert_parse_leaf_pubkey(chain);
  }
  return hs->local_pubkey != nullptr;
}


// Delegated credentials.

DC::DC() = default;
DC::~DC() = default;

UniquePtr<DC> DC::Dup() {
  bssl::UniquePtr<DC> ret = MakeUnique<DC>();
  if (!ret) {
    return nullptr;
  }

  ret->raw = UpRef(raw);
  ret->dc_cert_verify_algorithm = dc_cert_verify_algorithm;
  ret->pkey = UpRef(pkey);
  return ret;
}

// static
UniquePtr<DC> DC::Parse(CRYPTO_BUFFER *in, uint8_t *out_alert) {
  UniquePtr<DC> dc = MakeUnique<DC>();
  if (!dc) {
    *out_alert = SSL_AD_INTERNAL_ERROR;
    return nullptr;
  }

  dc->raw = UpRef(in);

  CBS pubkey, deleg, sig;
  uint32_t valid_time;
  uint16_t algorithm;
  CRYPTO_BUFFER_init_CBS(dc->raw.get(), &deleg);
  if (!CBS_get_u32(&deleg, &valid_time) ||
      !CBS_get_u16(&deleg, &dc->dc_cert_verify_algorithm) ||
      !CBS_get_u24_length_prefixed(&deleg, &pubkey) ||
      !CBS_get_u16(&deleg, &algorithm) ||
      !CBS_get_u16_length_prefixed(&deleg, &sig) || CBS_len(&deleg) != 0) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_DECODE_ERROR);
    *out_alert = SSL_AD_DECODE_ERROR;
    return nullptr;
  }

  dc->pkey.reset(EVP_parse_public_key(&pubkey));
  if (dc->pkey == nullptr) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_DECODE_ERROR);
    *out_alert = SSL_AD_DECODE_ERROR;
    return nullptr;
  }

  // RFC 9345 forbids algorithms that use the rsaEncryption OID. As the
  // RSASSA-PSS OID is unusably complicated, this effectively means we will not
  // support RSA delegated credentials.
  if (SSL_get_signature_algorithm_key_type(dc->dc_cert_verify_algorithm) ==
      EVP_PKEY_RSA) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_INVALID_SIGNATURE_ALGORITHM);
    *out_alert = SSL_AD_ILLEGAL_PARAMETER;
    return nullptr;
  }

  return dc;
}

// ssl_can_serve_dc returns true if the host has configured a DC that it can
// serve in the handshake. Specifically, it checks that a DC has been
// configured and that the DC signature algorithm is supported by the peer.
static bool ssl_can_serve_dc(const SSL_HANDSHAKE *hs) {
  // Check that a DC has been configured.
  const CERT *cert = hs->config->cert.get();
  if (cert->dc == nullptr || cert->dc->raw == nullptr ||
      (cert->dc_privatekey == nullptr && cert->dc_key_method == nullptr)) {
    return false;
  }

  // Check that 1.3 or higher has been negotiated.
  const DC *dc = cert->dc.get();
  assert(hs->ssl->s3->have_version);
  if (ssl_protocol_version(hs->ssl) < TLS1_3_VERSION) {
    return false;
  }

  // Check that the DC signature algorithm is supported by the peer.
  Span<const uint16_t> peer_sigalgs = hs->peer_delegated_credential_sigalgs;
  for (uint16_t peer_sigalg : peer_sigalgs) {
    if (dc->dc_cert_verify_algorithm == peer_sigalg) {
      return true;
    }
  }
  return false;
}

bool ssl_signing_with_dc(const SSL_HANDSHAKE *hs) {
  // We only support delegated credentials as a server.
  return hs->ssl->server && ssl_can_serve_dc(hs);
}

static int cert_set_dc(CERT *cert, CRYPTO_BUFFER *const raw, EVP_PKEY *privkey,
                       const SSL_PRIVATE_KEY_METHOD *key_method) {
  if (privkey == nullptr && key_method == nullptr) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }

  if (privkey != nullptr && key_method != nullptr) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_CANNOT_HAVE_BOTH_PRIVKEY_AND_METHOD);
    return 0;
  }

  uint8_t alert;
  UniquePtr<DC> dc = DC::Parse(raw, &alert);
  if (dc == nullptr) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_INVALID_DELEGATED_CREDENTIAL);
    return 0;
  }

  if (privkey) {
    // Check that the public and private keys match.
    if (!ssl_compare_public_and_private_key(dc->pkey.get(), privkey)) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_CERTIFICATE_AND_PRIVATE_KEY_MISMATCH);
      return 0;
    }
  }

  cert->dc = std::move(dc);
  cert->dc_privatekey = UpRef(privkey);
  cert->dc_key_method = key_method;

  return 1;
}

bool ssl_cert_check_cert_private_keys_usage(const CERT *cert) {
  if (cert == nullptr || cert->cert_private_keys.size() != SSL_PKEY_SIZE ||
      cert->cert_private_key_idx < 0 ||
      cert->cert_private_key_idx >= SSL_PKEY_SIZE) {
    return false;
  }
  return true;
}

BSSL_NAMESPACE_END

using namespace bssl;

static int cert_array_to_stack(CRYPTO_BUFFER *const *from,
                              UniquePtr<STACK_OF(CRYPTO_BUFFER)> *to,
                              size_t num_certs) {
  for (size_t i = 0; i < num_certs; i++) {
    if (!PushToStack(to->get(), UpRef(from[i]))) {
      return 0;
    }
  }
  return 1;
}

int SSL_set_chain_and_key(SSL *ssl, CRYPTO_BUFFER *const *certs,
                          size_t num_certs, EVP_PKEY *privkey,
                          const SSL_PRIVATE_KEY_METHOD *privkey_method) {
  if (!ssl->config) {
    return 0;
  }
  UniquePtr<STACK_OF(CRYPTO_BUFFER)> crypto_certs(sk_CRYPTO_BUFFER_new_null());
  if (!cert_array_to_stack(certs, &crypto_certs, num_certs)) {
    return 0;
  }
  return cert_set_chain_and_key(ssl->config->cert.get(), crypto_certs, num_certs,
                                privkey, privkey_method);
}

int SSL_CTX_set_chain_and_key(SSL_CTX *ctx, CRYPTO_BUFFER *const *certs,
                              size_t num_certs, EVP_PKEY *privkey,
                              const SSL_PRIVATE_KEY_METHOD *privkey_method) {
  UniquePtr<STACK_OF(CRYPTO_BUFFER)> crypto_certs(sk_CRYPTO_BUFFER_new_null());
  if (!cert_array_to_stack(certs, &crypto_certs, num_certs)) {
    return 0;
  }
  return cert_set_chain_and_key(ctx->cert.get(), crypto_certs, num_certs, privkey,
                                privkey_method);
}

static int cert_use_cert_and_key(CERT *cert, X509 *x509, EVP_PKEY *privatekey,
                                 STACK_OF(X509) *chain, int override) {
  if (privatekey == nullptr || x509 == nullptr) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }

  // Check override value
  int idx = ssl_get_certificate_slot_index(privatekey);
  if (idx < 0) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_UNKNOWN_CERTIFICATE_TYPE);
    return 0;
  }
  CERT_PKEY *cert_pkey = &cert->cert_private_keys[idx];
  if (!override && (cert_pkey->privatekey != NULL ||
      cert_pkey->x509_leaf != NULL ||
      cert_pkey->chain != NULL)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_NOT_REPLACING_CERTIFICATE);
    return 0;
  }

  UniquePtr<STACK_OF(CRYPTO_BUFFER)> leaf_and_chain(sk_CRYPTO_BUFFER_new_null());

  // Convert leaf certificate |x509| to CRYPTO_BUFFER
  uint8_t *buf = nullptr;
  int cert_len = i2d_X509(x509, &buf);
  if (cert_len <= 0) {
    OPENSSL_free(buf);
    return 0;
  }

  // Add leaf cert first to the chain
  UniquePtr<CRYPTO_BUFFER> leaf_buf(CRYPTO_BUFFER_new(buf, cert_len, nullptr));
  OPENSSL_free(buf);
  if (!leaf_buf ||
      !PushToStack(leaf_and_chain.get(), std::move(leaf_buf))) {
    return 0;
  }

  // Convert other chain certificates to CRYPTO_BUFFER objects
  if (chain != nullptr) {
    for (size_t i = 0; i < sk_X509_num(chain); i++) {
      buf = nullptr;
      cert_len = i2d_X509(sk_X509_value(chain, i), &buf);
      if (cert_len <= 0) {
        OPENSSL_free(buf);
        return 0;
      }

      UniquePtr<CRYPTO_BUFFER> chain_buf(CRYPTO_BUFFER_new(buf, cert_len, nullptr));
      OPENSSL_free(buf);
      if (!chain_buf ||
          !PushToStack(leaf_and_chain.get(), std::move(chain_buf))) {
        return 0;
      }
    }
  }

  // Set the chain and key
  if (!cert_set_chain_and_key(cert, leaf_and_chain,
                              sk_CRYPTO_BUFFER_num(leaf_and_chain.get()),
                              privatekey, nullptr)) {
    return 0;
  }

  // Update the leaf certificate
  if (cert_pkey->x509_leaf) {
    X509_free(cert_pkey->x509_leaf);
  }
  X509_up_ref(x509);
  cert_pkey->x509_leaf = x509;
  return 1;
}

int SSL_CTX_use_cert_and_key(SSL_CTX *ctx, X509 *x509, EVP_PKEY *privatekey,
                             STACK_OF(X509) *chain, int override) {
  return cert_use_cert_and_key(ctx->cert.get(), x509, privatekey, chain,
                               override);
}

int SSL_use_cert_and_key(SSL *ssl, X509 *x509, EVP_PKEY *privatekey,
                         STACK_OF(X509) *chain, int override) {
  if (!ssl->config) {
    return 0;
  }
  return cert_use_cert_and_key(ssl->config->cert.get(), x509, privatekey,
                               chain, override);
}

void SSL_certs_clear(SSL *ssl) {
  if (!ssl->config) {
    return;
  }

  CERT *cert = ssl->config->cert.get();
  cert->x509_method->cert_clear(cert);

  cert->cert_private_key_idx = -1;
  for (auto &cert_private_key : cert->cert_private_keys) {
    cert_private_key.chain.reset();
    cert_private_key.privatekey.reset();
  }
  cert->key_method = nullptr;

  cert->dc.reset();
  cert->dc_privatekey.reset();
  cert->dc_key_method = nullptr;
}

const STACK_OF(CRYPTO_BUFFER) *SSL_CTX_get0_chain(const SSL_CTX *ctx) {
  if (!ssl_cert_check_cert_private_keys_usage(ctx->cert.get())) {
    return nullptr;
  }
  return ctx->cert->cert_private_keys[ctx->cert->cert_private_key_idx]
      .chain.get();
}

int SSL_CTX_use_certificate_ASN1(SSL_CTX *ctx, size_t der_len,
                                 const uint8_t *der) {
  UniquePtr<CRYPTO_BUFFER> buffer(CRYPTO_BUFFER_new(der, der_len, NULL));
  if (!buffer) {
    return 0;
  }

  return ssl_set_cert(ctx->cert.get(), std::move(buffer));
}

int SSL_use_certificate_ASN1(SSL *ssl, const uint8_t *der, size_t der_len) {
  UniquePtr<CRYPTO_BUFFER> buffer(CRYPTO_BUFFER_new(der, der_len, NULL));
  if (!buffer || !ssl->config) {
    return 0;
  }

  return ssl_set_cert(ssl->config->cert.get(), std::move(buffer));
}

void SSL_CTX_set_cert_cb(SSL_CTX *ctx, int (*cb)(SSL *ssl, void *arg),
                         void *arg) {
  ssl_cert_set_cert_cb(ctx->cert.get(), cb, arg);
}

void SSL_set_cert_cb(SSL *ssl, int (*cb)(SSL *ssl, void *arg), void *arg) {
  if (!ssl->config) {
    return;
  }
  ssl_cert_set_cert_cb(ssl->config->cert.get(), cb, arg);
}

const STACK_OF(CRYPTO_BUFFER) *SSL_get0_peer_certificates(const SSL *ssl) {
  SSL_SESSION *session = SSL_get_session(ssl);
  if (session == NULL) {
    return NULL;
  }

  return session->certs.get();
}

const STACK_OF(CRYPTO_BUFFER) *SSL_get0_server_requested_CAs(const SSL *ssl) {
  if (ssl->s3->hs != NULL) {
    return ssl->s3->hs->ca_names.get();
  }
  return NULL;
}

static int set_signed_cert_timestamp_list(CERT *cert, const uint8_t *list,
                                          size_t list_len) {
  CBS sct_list;
  CBS_init(&sct_list, list, list_len);
  if (!ssl_is_sct_list_valid(&sct_list)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_INVALID_SCT_LIST);
    return 0;
  }

  cert->signed_cert_timestamp_list.reset(
      CRYPTO_BUFFER_new(CBS_data(&sct_list), CBS_len(&sct_list), nullptr));
  return cert->signed_cert_timestamp_list != nullptr;
}

int SSL_CTX_set_signed_cert_timestamp_list(SSL_CTX *ctx, const uint8_t *list,
                                           size_t list_len) {
  return set_signed_cert_timestamp_list(ctx->cert.get(), list, list_len);
}

int SSL_set_signed_cert_timestamp_list(SSL *ssl, const uint8_t *list,
                                       size_t list_len) {
  if (!ssl->config) {
    return 0;
  }
  return set_signed_cert_timestamp_list(ssl->config->cert.get(), list,
                                        list_len);
}

int SSL_CTX_set_ocsp_response(SSL_CTX *ctx, const uint8_t *response,
                              size_t response_len) {
  ctx->cert->ocsp_response.reset(
      CRYPTO_BUFFER_new(response, response_len, nullptr));
  return ctx->cert->ocsp_response != nullptr;
}

int SSL_set_ocsp_response(SSL *ssl, const uint8_t *response,
                          size_t response_len) {
  if (!ssl->config) {
    return 0;
  }
  ssl->config->cert->ocsp_response.reset(
      CRYPTO_BUFFER_new(response, response_len, nullptr));
  return ssl->config->cert->ocsp_response != nullptr;
}

void SSL_CTX_set0_client_CAs(SSL_CTX *ctx, STACK_OF(CRYPTO_BUFFER) *name_list) {
  ctx->x509_method->ssl_ctx_flush_cached_client_CA(ctx);
  ctx->client_CA.reset(name_list);
}

void SSL_set0_client_CAs(SSL *ssl, STACK_OF(CRYPTO_BUFFER) *name_list) {
  if (!ssl->config) {
    return;
  }
  ssl->ctx->x509_method->ssl_flush_cached_client_CA(ssl->config.get());
  ssl->config->client_CA.reset(name_list);
}

int SSL_set1_delegated_credential(SSL *ssl, CRYPTO_BUFFER *dc, EVP_PKEY *pkey,
                                  const SSL_PRIVATE_KEY_METHOD *key_method) {
  if (!ssl->config) {
    return 0;
  }

  return cert_set_dc(ssl->config->cert.get(), dc, pkey, key_method);
}

int SSL_delegated_credential_used(const SSL *ssl) {
  return ssl->s3->delegated_credential_used;
}

int SSL_CTX_get_security_level(const SSL_CTX *ctx) { return 0; }

void SSL_CTX_set_security_level(const SSL_CTX *ctx, int level) {}

int SSL_get_security_level(const SSL *ssl) { return 0; }

void SSL_set_security_level(const SSL *ssl, int level) {}

void SSL_CTX_set_security_callback(SSL_CTX *ctx, SSL_security_callback cb) {
  if (ctx == nullptr) {
    return;
  }
  ctx->security_callback = cb;
}

SSL_security_callback SSL_CTX_get_security_callback(const SSL_CTX *ctx) {
  if (ctx == nullptr) {
    return nullptr;
  }
  return ctx->security_callback;
}

void SSL_CTX_set0_security_ex_data(SSL_CTX *ctx, void *ex) {
  if (ctx == nullptr) {
    return;
  }
  ctx->security_callback_ex_data = ex;
}

void *SSL_CTX_get0_security_ex_data(const SSL_CTX *ctx) {
  if (ctx == nullptr) {
    return nullptr;
  }
  return ctx->security_callback_ex_data;
}

void SSL_set_security_callback(SSL *ssl, SSL_security_callback cb) {
  if (ssl == nullptr) {
    return;
  }
  ssl->security_callback = cb;
}

SSL_security_callback SSL_get_security_callback(const SSL *ssl) {
  if (ssl == nullptr) {
    return nullptr;
  }
  return ssl->security_callback;
}

void SSL_set0_security_ex_data(SSL *ssl, void *ex) {
  if (ssl == nullptr) {
    return;
  }
  ssl->security_callback_ex_data = ex;
}

void *SSL_get0_security_ex_data(const SSL *ssl) {
  if (ssl == nullptr) {
    return nullptr;
  }
  return ssl->security_callback_ex_data;
}
