// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/evp.h>

#include <assert.h>
#include <string.h>

#include <openssl/curve25519.h>
#include <openssl/dsa.h>
#include <openssl/ec.h>
#include <openssl/err.h>
#include <openssl/mem.h>
#include <openssl/nid.h>
#include <openssl/rsa.h>
#include <openssl/thread.h>

#include "../../evp_extra/internal.h"
#include "../../pem/internal.h"
#include "../../console/internal.h"
#include "../../internal.h"
#include "../kem/internal.h"
#include "../pqdsa/internal.h"
#include "internal.h"


// Node depends on |EVP_R_NOT_XOF_OR_INVALID_LENGTH|.
//
// TODO(davidben): Fix Node to not touch the error queue itself and remove this.
OPENSSL_DECLARE_ERROR_REASON(EVP, NOT_XOF_OR_INVALID_LENGTH)

// The HPKE module uses the EVP error namespace, but it lives in another
// directory.
OPENSSL_DECLARE_ERROR_REASON(EVP, EMPTY_PSK)

EVP_PKEY *EVP_PKEY_new(void) {
  EVP_PKEY *ret;

  ret = OPENSSL_zalloc(sizeof(EVP_PKEY));
  if (ret == NULL) {
    return NULL;
  }

  ret->type = EVP_PKEY_NONE;
  ret->references = 1;

  return ret;
}

static void free_it(EVP_PKEY *pkey) {
  if (pkey->ameth && pkey->ameth->pkey_free) {
    pkey->ameth->pkey_free(pkey);
  }
  pkey->pkey.ptr = NULL;
  pkey->type = EVP_PKEY_NONE;
  pkey->ameth = NULL;
}

void EVP_PKEY_free(EVP_PKEY *pkey) {
  SET_DIT_AUTO_RESET;
  if (pkey == NULL) {
    return;
  }

  if (!CRYPTO_refcount_dec_and_test_zero(&pkey->references)) {
    return;
  }

  free_it(pkey);
  OPENSSL_free(pkey);
}

int EVP_PKEY_up_ref(EVP_PKEY *pkey) {
  SET_DIT_AUTO_RESET;
  CRYPTO_refcount_inc(&pkey->references);
  return 1;
}

int EVP_PKEY_is_opaque(const EVP_PKEY *pkey) {
  SET_DIT_AUTO_RESET;
  if (pkey->ameth && pkey->ameth->pkey_opaque) {
    return pkey->ameth->pkey_opaque(pkey);
  }
  return 0;
}

int EVP_PKEY_cmp(const EVP_PKEY *a, const EVP_PKEY *b) {
  SET_DIT_AUTO_RESET;
  if (a->type != b->type) {
    return -1;
  }

  if (a->ameth) {
    int ret;
    // Compare parameters if the algorithm has them
    if (a->ameth->param_cmp) {
      ret = a->ameth->param_cmp(a, b);
      if (ret <= 0) {
        return ret;
      }
    }

    if (a->ameth->pub_cmp) {
      return a->ameth->pub_cmp(a, b);
    }
  }

  return -2;
}

char *EVP_get_pw_prompt(void) {
    return (char*)"Enter PEM passphrase:";
}

int EVP_read_pw_string(char *buf, int length, const char *prompt, int verify) {
  return EVP_read_pw_string_min(buf, 0, length, prompt, verify);
}

int EVP_read_pw_string_min(char *buf, int min_length, int length,
                           const char *prompt, int verify) {
  int ret = -1;
  char verify_buf[1024];

  if (!buf || min_length < 0 || min_length >= length) {
    return -1;
  }

  if (prompt == NULL) {
    prompt = EVP_get_pw_prompt();
  }

  // Proactively zeroize |buf| and verify_buf
  OPENSSL_cleanse(buf, length);
  OPENSSL_cleanse(verify_buf, sizeof(verify_buf));

  // acquire write lock
  openssl_console_acquire_mutex();

  if (!openssl_console_open()) {
    goto err;
  }

  // Write initial password prompt
  if (!openssl_console_write(prompt)) {
    goto err;
  }

  // Read password with echo disabled. Returns 0 on success.
  // While |buf| and |length| are user-provided and can be arbitrarily large,
  // passwords exceeding 1024 characters will be rejected with AWS-LC. OpenSSL
  // handles this by silently truncating |length| before reading the password.
  ret = openssl_console_read(buf, min_length, length, 0);
  if (ret != 0) {
    OPENSSL_cleanse(buf, length);
    OPENSSL_PUT_ERROR(PEM, PEM_R_PROBLEMS_GETTING_PASSWORD);
    goto err;
  }

  if (verify) {
    openssl_console_write("Verifying - ");
    openssl_console_write(prompt);

    ret = openssl_console_read(verify_buf, min_length, sizeof(verify_buf), 0);

    if (ret == 0) {
      if (strncmp(buf, verify_buf, length) != 0) {
        openssl_console_write("Verify failure\n");
        ret = -1;
      }
    } else {
      OPENSSL_PUT_ERROR(PEM, PEM_R_PROBLEMS_GETTING_PASSWORD);
      goto err;
    }
  }

  openssl_console_close();

err:
  openssl_console_release_mutex();
  OPENSSL_cleanse(verify_buf, sizeof(verify_buf));
  return ret;
}

int EVP_PKEY_copy_parameters(EVP_PKEY *to, const EVP_PKEY *from) {
  SET_DIT_AUTO_RESET;
  if (to->type == EVP_PKEY_NONE) {
    // TODO(crbug.com/42290409): This shouldn't leave |to| in a half-empty state
    // on error. The complexity here largely comes from parameterless DSA keys,
    // which we no longer support, so this function can probably be trimmed
    // down.
    evp_pkey_set0(to, from->ameth, NULL);
  } else if (to->type != from->type) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_DIFFERENT_KEY_TYPES);
    return 0;
  }

  if (EVP_PKEY_missing_parameters(from)) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_MISSING_PARAMETERS);
    return 0;
  }

  // Once set, parameters may not change.
  if (!EVP_PKEY_missing_parameters(to)) {
    if (EVP_PKEY_cmp_parameters(to, from) == 1) {
      return 1;
    }
    OPENSSL_PUT_ERROR(EVP, EVP_R_DIFFERENT_PARAMETERS);
    return 0;
  }

  if (from->ameth && from->ameth->param_copy) {
    return from->ameth->param_copy(to, from);
  }

  // TODO(https://crbug.com/boringssl/536): If the algorithm takes no
  // parameters, copying them should vacuously succeed.
  return 0;
}

int EVP_PKEY_missing_parameters(const EVP_PKEY *pkey) {
  SET_DIT_AUTO_RESET;
  if (pkey->ameth && pkey->ameth->param_missing) {
    return pkey->ameth->param_missing(pkey);
  }
  return 0;
}

int EVP_PKEY_size(const EVP_PKEY *pkey) {
  SET_DIT_AUTO_RESET;
  if (pkey && pkey->ameth && pkey->ameth->pkey_size) {
    return pkey->ameth->pkey_size(pkey);
  }
  return 0;
}

int EVP_PKEY_bits(const EVP_PKEY *pkey) {
  SET_DIT_AUTO_RESET;
  if (pkey && pkey->ameth && pkey->ameth->pkey_bits) {
    return pkey->ameth->pkey_bits(pkey);
  }
  return 0;
}

int EVP_PKEY_id(const EVP_PKEY *pkey) {
  SET_DIT_AUTO_RESET;
  return pkey->type;
}

int EVP_PKEY_pqdsa_get_type(const EVP_PKEY *pkey) {
  SET_DIT_AUTO_RESET;
  if (pkey->type != EVP_PKEY_PQDSA) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_EXPECTING_A_PQDSA_KEY);
    return 0;
  }
  if (!pkey->pkey.pqdsa_key || !pkey->pkey.pqdsa_key->pqdsa) {
    return 0;
  }
  return pkey->pkey.pqdsa_key->pqdsa->nid;
}

int EVP_PKEY_kem_get_type(const EVP_PKEY *pkey) {
  SET_DIT_AUTO_RESET;
  if (pkey->type != EVP_PKEY_KEM) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_EXPECTING_A_KEM_KEY);
    return 0;
  }
  if (!pkey->pkey.kem_key || !pkey->pkey.kem_key->kem) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_NO_PARAMETERS_SET);
    return 0;
  }
  return pkey->pkey.kem_key->kem->nid;
}

int EVP_MD_get_pkey_type(const EVP_MD *md) {
  if (md) {
    int sig_nid = 0;
    if (OBJ_find_sigid_by_algs(&sig_nid, md->type, NID_rsaEncryption)) {
      return sig_nid;
    }
  }
  return 0;
}

int EVP_MD_pkey_type(const EVP_MD *md){
  return EVP_MD_get_pkey_type(md);
}

const char *EVP_MD_get0_name(const EVP_MD *md) {
  if (md != NULL) {
    return OBJ_nid2sn(EVP_MD_nid(md));
  }
  return NULL;
}

const char *EVP_MD_name(const EVP_MD *md) {
  return EVP_MD_get0_name(md);
}

// evp_pkey_asn1_find returns the ASN.1 method table for the given |nid|, which
// should be one of the |EVP_PKEY_*| values. It returns NULL if |nid| is
// unknown.
static const EVP_PKEY_ASN1_METHOD *evp_pkey_asn1_find(int nid) {

  const EVP_PKEY_ASN1_METHOD *const *methods = AWSLC_non_fips_pkey_evp_asn1_methods();
  for (size_t i = 0; i < ASN1_EVP_PKEY_METHODS; i++) {
    if (methods[i]->pkey_id == nid) {
      return methods[i];
    }
  }

  return NULL;
}

void evp_pkey_set0(EVP_PKEY *pkey, const EVP_PKEY_ASN1_METHOD *method,
                   void *pkey_data) {
  free_it(pkey);
  pkey->ameth = method;
  pkey->type = method ? method->pkey_id : EVP_PKEY_NONE;
  pkey->pkey.ptr = pkey_data;
}

static int pkey_set_type(EVP_PKEY *pkey, int type, const char *str, int len) {
  if (pkey && pkey->pkey.ptr) {
    // This isn't strictly necessary, but historically |EVP_PKEY_set_type| would
    // clear |pkey| even if |evp_pkey_asn1_find| failed, so we preserve that
    // behavior.
    evp_pkey_set0(pkey, NULL, NULL);
  }

  const EVP_PKEY_ASN1_METHOD *ameth = NULL;

  if (str != NULL) {
    ameth = EVP_PKEY_asn1_find_str(NULL, str, len);
  } else {
    ameth = evp_pkey_asn1_find(type);
  }

  if (ameth == NULL) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_UNSUPPORTED_ALGORITHM);
    ERR_add_error_dataf("algorithm %d", type);
    return 0;
  }

  if (pkey) {
    evp_pkey_set0(pkey, ameth, NULL);
  }

  return 1;
}

int EVP_PKEY_type(int nid) {
  // In OpenSSL, this was used to map between type aliases. BoringSSL supports
  // no type aliases, so this function is just the identity.
  return nid;
}

EVP_PKEY *EVP_PKEY_new_mac_key(int type, ENGINE *engine, const uint8_t *mac_key,
                               size_t mac_key_len) {
  SET_DIT_AUTO_RESET;
  // Only |EVP_PKEY_HMAC| is supported as of now.
  if (type != EVP_PKEY_HMAC) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATION_NOT_SUPPORTED_FOR_THIS_KEYTYPE);
    return NULL;
  }
  // NULL |mac_key| will result in a complete zero-key being used, but in that
  // case, the length must be zero.
  if (mac_key == NULL && mac_key_len > 0) {
    return NULL;
  }

  EVP_PKEY *ret = EVP_PKEY_new();
  if (ret == NULL) {
    OPENSSL_PUT_ERROR(EVP, ERR_LIB_EVP);
    return NULL;
  }

  HMAC_KEY *key = HMAC_KEY_new();
  if(key == NULL) {
    goto err;
  }
  key->key = OPENSSL_memdup(mac_key, mac_key_len);
  if (key->key == NULL && mac_key_len > 0) {
    OPENSSL_free(key);
    goto err;
  }
  key->key_len = mac_key_len;

  if(!EVP_PKEY_assign(ret, EVP_PKEY_HMAC, key)) {
    OPENSSL_free(key);
    goto err;
  }
  return ret;

err:
  OPENSSL_PUT_ERROR(EVP, ERR_LIB_EVP);
  EVP_PKEY_free(ret);
  return NULL;
}

int EVP_PKEY_set1_RSA(EVP_PKEY *pkey, RSA *key) {
  SET_DIT_AUTO_RESET;
  if (EVP_PKEY_assign_RSA(pkey, key)) {
    RSA_up_ref(key);
    return 1;
  }
  return 0;
}

int EVP_PKEY_assign_RSA(EVP_PKEY *pkey, RSA *key) {
  SET_DIT_AUTO_RESET;
  if (key == NULL) {
    return 0;
  }
  const EVP_PKEY_ASN1_METHOD *meth = evp_pkey_asn1_find(EVP_PKEY_RSA);
  assert(meth != NULL);
  evp_pkey_set0(pkey, meth, key);
  return 1;
}

RSA *EVP_PKEY_get0_RSA(const EVP_PKEY *pkey) {
  SET_DIT_AUTO_RESET;
  if (pkey->type != EVP_PKEY_RSA && pkey->type != EVP_PKEY_RSA_PSS) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_EXPECTING_AN_RSA_KEY);
    return NULL;
  }
  return pkey->pkey.rsa;
}

RSA *EVP_PKEY_get1_RSA(const EVP_PKEY *pkey) {
  SET_DIT_AUTO_RESET;
  RSA *rsa = EVP_PKEY_get0_RSA(pkey);
  if (rsa != NULL) {
    RSA_up_ref(rsa);
  }
  return rsa;
}

int EVP_PKEY_set1_DSA(EVP_PKEY *pkey, DSA *key) {
  SET_DIT_AUTO_RESET;
  if (EVP_PKEY_assign_DSA(pkey, key)) {
    DSA_up_ref(key);
    return 1;
  }
  return 0;
}

int EVP_PKEY_assign_DSA(EVP_PKEY *pkey, DSA *key) {
  SET_DIT_AUTO_RESET;
  if (key == NULL) {
    return 0;
  }
  const EVP_PKEY_ASN1_METHOD *meth = evp_pkey_asn1_find(EVP_PKEY_DSA);
  assert(meth != NULL);
  evp_pkey_set0(pkey, meth, key);
  return 1;
}

DSA *EVP_PKEY_get0_DSA(const EVP_PKEY *pkey) {
  SET_DIT_AUTO_RESET;
  if (pkey->type != EVP_PKEY_DSA) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_EXPECTING_A_DSA_KEY);
    return NULL;
  }
  return pkey->pkey.dsa;
}

DSA *EVP_PKEY_get1_DSA(const EVP_PKEY *pkey) {
  SET_DIT_AUTO_RESET;
  DSA *dsa = EVP_PKEY_get0_DSA(pkey);
  if (dsa != NULL) {
    DSA_up_ref(dsa);
  }
  return dsa;
}

int EVP_PKEY_set1_EC_KEY(EVP_PKEY *pkey, EC_KEY *key) {
  SET_DIT_AUTO_RESET;
  if (EVP_PKEY_assign_EC_KEY(pkey, key)) {
    EC_KEY_up_ref(key);
    return 1;
  }
  return 0;
}

int EVP_PKEY_assign_EC_KEY(EVP_PKEY *pkey, EC_KEY *key) {
  SET_DIT_AUTO_RESET;
  if (key == NULL) {
    return 0;
  }
  const EVP_PKEY_ASN1_METHOD *meth = evp_pkey_asn1_find(EVP_PKEY_EC);
  assert(meth != NULL);
  evp_pkey_set0(pkey, meth, key);
  return 1;
}

EC_KEY *EVP_PKEY_get0_EC_KEY(const EVP_PKEY *pkey) {
  SET_DIT_AUTO_RESET;
  if (pkey->type != EVP_PKEY_EC) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_EXPECTING_A_EC_KEY_KEY);
    return NULL;
  }
  return pkey->pkey.ec;
}

EC_KEY *EVP_PKEY_get1_EC_KEY(const EVP_PKEY *pkey) {
  SET_DIT_AUTO_RESET;
  EC_KEY *ec_key = EVP_PKEY_get0_EC_KEY(pkey);
  if (ec_key != NULL) {
    EC_KEY_up_ref(ec_key);
  }
  return ec_key;
}

int EVP_PKEY_assign(EVP_PKEY *pkey, int type, void *key) {
  // This function can only be used to assign RSA, DSA, EC, and DH keys. Other
  // key types have internal representations which are not exposed through the
  // public API.
  SET_DIT_AUTO_RESET;
  switch (type) {
    case EVP_PKEY_RSA:
      return EVP_PKEY_assign_RSA(pkey, key);
    case EVP_PKEY_DSA:
      return EVP_PKEY_assign_DSA(pkey, key);
    case EVP_PKEY_EC:
      return EVP_PKEY_assign_EC_KEY(pkey, key);
    case EVP_PKEY_DH:
      return EVP_PKEY_assign_DH(pkey, key);
    default:
      if (!EVP_PKEY_set_type(pkey, type)) {
        return 0;
      }
      pkey->pkey.ptr = key;
      return key != NULL;
  }
}

int EVP_PKEY_set_type(EVP_PKEY *pkey, int type) {
  SET_DIT_AUTO_RESET;
  return pkey_set_type(pkey, type, NULL, -1);
}

int EVP_PKEY_set_type_str(EVP_PKEY *pkey, const char *str, int len) {
  SET_DIT_AUTO_RESET;
  return pkey_set_type(pkey, EVP_PKEY_NONE, str, len);
}

EVP_PKEY *EVP_PKEY_new_raw_private_key(int type, ENGINE *unused,
                                       const uint8_t *in, size_t len) {
  SET_DIT_AUTO_RESET;
  // To avoid pulling in all key types, look for specifically the key types that
  // support |set_priv_raw|.
  const EVP_PKEY_ASN1_METHOD *method;
  switch (type) {
    case EVP_PKEY_X25519:
      method = &x25519_asn1_meth;
      break;
    case EVP_PKEY_ED25519:
      method = &ed25519_asn1_meth;
      break;
    case EVP_PKEY_ED25519PH:
      method = &ed25519ph_asn1_meth;
      break;
    case EVP_PKEY_HMAC:
      method = &hmac_asn1_meth;
      break;
    default:
      OPENSSL_PUT_ERROR(EVP, EVP_R_UNSUPPORTED_ALGORITHM);
      return 0;
  }

  EVP_PKEY *ret = EVP_PKEY_new();
  if (ret == NULL) {
    goto err;
  }
  evp_pkey_set0(ret, method, NULL);

  if (!method->set_priv_raw(ret, in, len, NULL, 0)) {
    goto err;
  }

  return ret;

err:
  EVP_PKEY_free(ret);
  return NULL;
}

EVP_PKEY *EVP_PKEY_new_raw_public_key(int type, ENGINE *unused,
                                      const uint8_t *in, size_t len) {
  // To avoid pulling in all key types, look for specifically the key types that
  // support |set_pub_raw|.
  const EVP_PKEY_ASN1_METHOD *method;
  switch (type) {
    case EVP_PKEY_X25519:
      method = &x25519_asn1_meth;
      break;
    case EVP_PKEY_ED25519:
      method = &ed25519_asn1_meth;
      break;
    case EVP_PKEY_ED25519PH:
      method = &ed25519ph_asn1_meth;
      break;
    default:
      OPENSSL_PUT_ERROR(EVP, EVP_R_UNSUPPORTED_ALGORITHM);
      return 0;
  }

  EVP_PKEY *ret = EVP_PKEY_new();
  if (ret == NULL) {
    goto err;
  }
  evp_pkey_set0(ret, method, NULL);

  if (!method->set_pub_raw(ret, in, len)) {
    goto err;
  }

  return ret;

err:
  EVP_PKEY_free(ret);
  return NULL;
}

int EVP_PKEY_get_raw_private_key(const EVP_PKEY *pkey, uint8_t *out,
                                 size_t *out_len) {
  SET_DIT_AUTO_RESET;
  if (pkey == NULL ||
      pkey->ameth == NULL ||
      pkey->ameth->get_priv_raw == NULL) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATION_NOT_SUPPORTED_FOR_THIS_KEYTYPE);
    return 0;
  }

  return pkey->ameth->get_priv_raw(pkey, out, out_len);
}

int EVP_PKEY_get_raw_public_key(const EVP_PKEY *pkey, uint8_t *out,
                                size_t *out_len) {
  SET_DIT_AUTO_RESET;
  if (pkey == NULL ||
      pkey->ameth == NULL ||
      pkey->ameth->get_pub_raw == NULL) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATION_NOT_SUPPORTED_FOR_THIS_KEYTYPE);
    return 0;
  }

  return pkey->ameth->get_pub_raw(pkey, out, out_len);
}

int EVP_PKEY_cmp_parameters(const EVP_PKEY *a, const EVP_PKEY *b) {
  SET_DIT_AUTO_RESET;
  if (a->type != b->type) {
    return -1;
  }
  if (a->ameth && a->ameth->param_cmp) {
    return a->ameth->param_cmp(a, b);
  }
  // TODO(https://crbug.com/boringssl/536): If the algorithm doesn't use
  // parameters, they should compare as vacuously equal.
  return -2;
}

int EVP_PKEY_CTX_set_signature_md(EVP_PKEY_CTX *ctx, const EVP_MD *md) {
  SET_DIT_AUTO_RESET;
  return EVP_PKEY_CTX_ctrl(ctx, -1, EVP_PKEY_OP_TYPE_SIG, EVP_PKEY_CTRL_MD, 0,
                           (void *)md);
}

int EVP_PKEY_CTX_get_signature_md(EVP_PKEY_CTX *ctx, const EVP_MD **out_md) {
  SET_DIT_AUTO_RESET;
  return EVP_PKEY_CTX_ctrl(ctx, -1, EVP_PKEY_OP_TYPE_SIG, EVP_PKEY_CTRL_GET_MD,
                           0, (void *)out_md);
}

int EVP_PKEY_CTX_set1_signature_context_string(EVP_PKEY_CTX *ctx,
                                       const uint8_t *context,
                                       size_t context_len) {
  EVP_PKEY_CTX_SIGNATURE_CONTEXT_PARAMS params = {context, context_len};
  return EVP_PKEY_CTX_ctrl(ctx, -1, EVP_PKEY_OP_TYPE_SIG,
                           EVP_PKEY_CTRL_SIGNING_CONTEXT, 0, &params);
}

int EVP_PKEY_CTX_set_signature_context(EVP_PKEY_CTX *ctx,
                                       const uint8_t *context,
                                       size_t context_len) {
  return EVP_PKEY_CTX_set1_signature_context_string(ctx, context, context_len);
}

int EVP_PKEY_CTX_get0_signature_context(EVP_PKEY_CTX *ctx,
                                        const uint8_t **context,
                                        size_t *context_len) {
  GUARD_PTR(context);
  GUARD_PTR(context_len);
  EVP_PKEY_CTX_SIGNATURE_CONTEXT_PARAMS params = {NULL, 0};
  if (!EVP_PKEY_CTX_ctrl(ctx, -1, EVP_PKEY_OP_TYPE_SIG,
                         EVP_PKEY_CTRL_GET_SIGNING_CONTEXT, 0, &params)) {
    return 0;
  }
  *context = params.context;
  *context_len = params.context_len;
  return 1;
}


void *EVP_PKEY_get0(const EVP_PKEY *pkey) {
  SET_DIT_AUTO_RESET;
  GUARD_PTR(pkey);
  switch (pkey->type) {
    case EVP_PKEY_RSA:
    case EVP_PKEY_RSA_PSS:
    case EVP_PKEY_DSA:
    case EVP_PKEY_EC:
    case EVP_PKEY_DH:
      return pkey->pkey.ptr;
    default:
      return NULL;
  }
}

void OpenSSL_add_all_algorithms(void) {}

void OPENSSL_add_all_algorithms_conf(void) {}

void OpenSSL_add_all_ciphers(void) {}

void OpenSSL_add_all_digests(void) {}

void EVP_cleanup(void) {}

int EVP_PKEY_base_id(const EVP_PKEY *pkey) {
  SET_DIT_AUTO_RESET;
  // OpenSSL has two notions of key type because it supports multiple OIDs for
  // the same algorithm: NID_rsa vs NID_rsaEncryption and five distinct spelling
  // of DSA. We do not support these, so the base ID is simply the ID.
  return EVP_PKEY_id(pkey);
}

static int evp_pkey_tls_encodedpoint_ec_curve_supported(const EC_KEY *ec_key) {

  int ret = 0;
  int curve_nid = 0;
  const EC_GROUP *ec_key_group = NULL;

  if (NULL == ec_key) {
    OPENSSL_PUT_ERROR(EVP, ERR_R_PASSED_NULL_PARAMETER);
    goto err;
  }

  ec_key_group = EC_KEY_get0_group(ec_key);
  if (NULL == ec_key_group) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_MISSING_PARAMETERS);
    goto err;
  }

  curve_nid = EC_GROUP_get_curve_name(ec_key_group);
  if ((NID_secp224r1 != curve_nid) &&
      (NID_X9_62_prime256v1 != curve_nid) &&
      (NID_secp384r1 != curve_nid) &&
      (NID_secp521r1 != curve_nid)) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_UNSUPPORTED_PUBLIC_KEY_TYPE);
    goto err;
  }

  ret = 1;

err:
  return ret;
}

static int evp_pkey_set1_tls_encodedpoint_ec_key(EVP_PKEY *pkey,
                                                  const uint8_t *in,
                                                  size_t len) {
  int ret = 0;
  EC_KEY *ec_key = NULL;
  const EC_GROUP *ec_key_group = NULL;
  EC_POINT *ec_point = NULL;

  if ((NULL == pkey) || (NULL == in)) {
    OPENSSL_PUT_ERROR(EVP, ERR_R_PASSED_NULL_PARAMETER);
    goto err;
  }

  if (1 > len) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_INVALID_PARAMETERS);
    goto err;
  }

  if (EVP_PKEY_EC != pkey->type) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_UNSUPPORTED_PUBLIC_KEY_TYPE);
    goto err;
  }

  // This function is TLS-specific. Only support TLS EC point representation,
  // which must be uncompressed
  // (https://tools.ietf.org/html/rfc8422#section-5.4.1)
  // TLS wire-encoding format for supported NIST curves are:
  // compression || x-coordinate || y-coordinate
  // where:
  // compression = 0x04 if uncompressed
  // compression = 0x02/0x03 if compressed (depending on y-coordinate parity)
  if (POINT_CONVERSION_UNCOMPRESSED != in[0]) {
    OPENSSL_PUT_ERROR(EVP, ERR_R_EVP_LIB);
    goto err;
  }

  ec_key = EVP_PKEY_get0_EC_KEY(pkey);
  if (NULL == ec_key) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_NO_KEY_SET);
    goto err;
  }

  if (0 == evp_pkey_tls_encodedpoint_ec_curve_supported(ec_key)) {
    OPENSSL_PUT_ERROR(EVP, ERR_R_EVP_LIB);
    goto err;
  }

  ec_key_group = EC_KEY_get0_group(ec_key);
  if (NULL == ec_key_group) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_MISSING_PARAMETERS);
    goto err;
  }

  ec_point = EC_POINT_new(ec_key_group);
  if (NULL == ec_point) {
    OPENSSL_PUT_ERROR(EVP, ERR_R_EVP_LIB);
    goto err;
  }

  if (0 == EC_POINT_oct2point(ec_key_group, ec_point, in, len, NULL)) {
    OPENSSL_PUT_ERROR(EVP, ERR_R_EVP_LIB);
    goto err;
  }

  if (0 == EC_KEY_set_public_key(ec_key, (const EC_POINT *) ec_point)) {
    OPENSSL_PUT_ERROR(EVP, ERR_R_EVP_LIB);
    goto err;
  }

  ret = 1;

err:
  EC_POINT_free(ec_point);
  return ret;
}

static int evp_pkey_set1_tls_encodedpoint_x25519(EVP_PKEY *pkey,
                                                    const uint8_t *in,
                                                    size_t len) {
  int ret = 0;

  if ((NULL == pkey) || (NULL == in)) {
    OPENSSL_PUT_ERROR(EVP, ERR_R_PASSED_NULL_PARAMETER);
    goto err;
  }

  if (EVP_PKEY_X25519 != pkey->type) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_UNSUPPORTED_PUBLIC_KEY_TYPE);
    goto err;
  }

  if (1 > len) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_INVALID_PARAMETERS);
    goto err;
  }

  if ((NULL == pkey->ameth) || (NULL == pkey->ameth->set_pub_raw)) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATION_NOT_SUPPORTED_FOR_THIS_KEYTYPE);
    goto err;
  }

  if (0 == pkey->ameth->set_pub_raw(pkey, in, len)) {
    OPENSSL_PUT_ERROR(EVP, ERR_R_EVP_LIB);
    goto err;
  }

  ret = 1;

err:
  return ret;
}

int EVP_PKEY_set1_tls_encodedpoint(EVP_PKEY *pkey, const uint8_t *in,
                                    size_t len) {
  SET_DIT_AUTO_RESET;
  if (NULL == pkey) {
    OPENSSL_PUT_ERROR(EVP, ERR_R_PASSED_NULL_PARAMETER);
    goto err;
  }

  switch (pkey->type) {
    case EVP_PKEY_X25519:
      return evp_pkey_set1_tls_encodedpoint_x25519(pkey, in, len);
    case EVP_PKEY_EC:
      return evp_pkey_set1_tls_encodedpoint_ec_key(pkey, in, len);
    default:
      OPENSSL_PUT_ERROR(EVP, EVP_R_UNSUPPORTED_PUBLIC_KEY_TYPE);
      goto err;
  }

err:
  return 0;
}

static size_t evp_pkey_get1_tls_encodedpoint_ec_key(const EVP_PKEY *pkey,
                                                      uint8_t **out_ptr) {

  size_t ret = 0;
  const EC_KEY *ec_key = NULL;

  if ((NULL == pkey) || (NULL == out_ptr)) {
    OPENSSL_PUT_ERROR(EVP, ERR_R_PASSED_NULL_PARAMETER);
    goto err;
  }

  if (EVP_PKEY_EC != pkey->type) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_UNSUPPORTED_PUBLIC_KEY_TYPE);
    goto err;
  }

  ec_key = EVP_PKEY_get0_EC_KEY(pkey);
  if (NULL == ec_key) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_NO_KEY_SET);
    goto err;
  }

  if (0 == evp_pkey_tls_encodedpoint_ec_curve_supported(ec_key)) {
    OPENSSL_PUT_ERROR(EVP, ERR_R_EVP_LIB);
    goto err;
  }

  // This function is TLS-specific. Only support TLS EC point representation,
  // which must be uncompressed
  // (https://tools.ietf.org/html/rfc8422#section-5.4.1)
  if (POINT_CONVERSION_UNCOMPRESSED != EC_KEY_get_conv_form(ec_key)) {
    OPENSSL_PUT_ERROR(EVP, ERR_R_EVP_LIB);
    goto err;
  }

  // Returns the length of |*out_ptr|
  ret = EC_KEY_key2buf(ec_key, POINT_CONVERSION_UNCOMPRESSED, out_ptr, NULL);
  if (0 == ret) {
    OPENSSL_PUT_ERROR(EVP, ERR_R_EVP_LIB);
    goto err;
  }

err:
  return ret;
}

static size_t evp_pkey_get1_tls_encodedpoint_x25519(const EVP_PKEY *pkey,
                                                      uint8_t **out_ptr) {

  size_t ret = 0;
  size_t out_len = 0;

  if ((NULL == pkey) || (NULL == out_ptr)) {
    OPENSSL_PUT_ERROR(EVP, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }

  if (EVP_PKEY_X25519 != pkey->type) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_UNSUPPORTED_PUBLIC_KEY_TYPE);
    return 0;
  }

  if ((NULL == pkey->ameth) || (NULL == pkey->ameth->get_pub_raw)) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATION_NOT_SUPPORTED_FOR_THIS_KEYTYPE);
    return 0;
  }

  out_len = X25519_SHARED_KEY_LEN;
  *out_ptr = OPENSSL_malloc(X25519_SHARED_KEY_LEN);
  if (NULL == *out_ptr) {
    return 0;
  }

  if (0 == pkey->ameth->get_pub_raw(pkey, *out_ptr, &out_len)) {
    OPENSSL_PUT_ERROR(EVP, ERR_R_EVP_LIB);
    goto err;
  }

  if (X25519_SHARED_KEY_LEN != out_len) {
    OPENSSL_PUT_ERROR(EVP, ERR_R_EVP_LIB);
    goto err;
  }

  ret = X25519_SHARED_KEY_LEN;

err:
  if (0 == ret) {
    OPENSSL_free(*out_ptr);
    *out_ptr = NULL;
  }
  return ret;
}

size_t EVP_PKEY_get1_tls_encodedpoint(const EVP_PKEY *pkey, uint8_t **out_ptr) {
  SET_DIT_AUTO_RESET;
  if (NULL == pkey) {
    OPENSSL_PUT_ERROR(EVP, ERR_R_PASSED_NULL_PARAMETER);
    goto err;
  }

  switch (pkey->type) {
    case EVP_PKEY_X25519:
      return evp_pkey_get1_tls_encodedpoint_x25519(pkey, out_ptr);
    case EVP_PKEY_EC:
      return evp_pkey_get1_tls_encodedpoint_ec_key(pkey, out_ptr);
    default:
      OPENSSL_PUT_ERROR(EVP, EVP_R_UNSUPPORTED_PUBLIC_KEY_TYPE);
      goto err;
    }

err:
  return 0;
}
