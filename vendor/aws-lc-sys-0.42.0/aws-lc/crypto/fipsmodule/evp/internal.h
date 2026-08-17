// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef OPENSSL_HEADER_EVP_INTERNAL_H
#define OPENSSL_HEADER_EVP_INTERNAL_H

#include <openssl/base.h>

#include <openssl/rsa.h>
#include <openssl/hmac.h>
#include <openssl/evp.h>

#if defined(__cplusplus)
extern "C" {
#endif

// EVP_MD_CTX_FLAG_KEEP_PKEY_CTX ensures |md_ctx->pctx| is not freed up in
// |EVP_MD_CTX_cleanup|. Only intended for internal use when |*pctx| was set
// externally with |EVP_MD_CTX_set_pkey_ctx|.
#define EVP_MD_CTX_FLAG_KEEP_PKEY_CTX 0x0400

// EVP_MD_CTX_HMAC causes the |EVP_MD|'s |init| function not to
// be called, the |update| member not to be copied from the |EVP_MD| in
// |EVP_DigestInit_ex| and for |md_data| not to be initialised.
// This is an implementation detail of |EVP_PKEY_HMAC|.
#define EVP_MD_CTX_HMAC 0x0800

typedef struct evp_pkey_method_st EVP_PKEY_METHOD;

struct evp_pkey_asn1_method_st {
  int pkey_id;
  uint8_t oid[11];
  uint8_t oid_len;

  const char *pem_str;
  const char *info;

  // pub_decode decodes |params| and |key| as a SubjectPublicKeyInfo
  // and writes the result into |out|. It returns one on success and zero on
  // error. |params| is the AlgorithmIdentifier after the OBJECT IDENTIFIER
  // type field, and |key| is the contents of the subjectPublicKey with the
  // leading padding byte checked and removed. Although X.509 uses BIT STRINGs
  // to represent SubjectPublicKeyInfo, every key type defined encodes the key
  // as a byte string with the same conversion to BIT STRING.
  int (*pub_decode)(EVP_PKEY *out, CBS *oid, CBS *params, CBS *key);

  // pub_encode encodes |key| as a SubjectPublicKeyInfo and appends the result
  // to |out|. It returns one on success and zero on error.
  int (*pub_encode)(CBB *out, const EVP_PKEY *key);

  int (*pub_cmp)(const EVP_PKEY *a, const EVP_PKEY *b);

  // priv_decode decodes |params| and |key| as a PrivateKeyInfo and writes the
  // result into |out|. It returns one on success and zero on error. |params| is
  // the AlgorithmIdentifier after the OBJECT IDENTIFIER type field, and |key|
  // is the contents of the OCTET STRING privateKey field.
  int (*priv_decode)(EVP_PKEY *out, CBS *oid, CBS *params, CBS *key, CBS *pubkey);

  // priv_encode encodes |key| as a PrivateKeyInfo and appends the result to
  // |out|. It returns one on success and zero on error.
  int (*priv_encode)(CBB *out, const EVP_PKEY *key);

  // priv_encode_v2 encodes |key| as a OneAsymmetricKey (RFC 5958) and appends
  // the result to |out|. It returns one on success and zero on error.
  int (*priv_encode_v2)(CBB *out, const EVP_PKEY *key);

  int (*set_priv_raw)(EVP_PKEY *pkey, const uint8_t *privkey, size_t privkey_len, const uint8_t *pubkey, size_t pubkey_len);
  int (*set_pub_raw)(EVP_PKEY *pkey, const uint8_t *in, size_t len);
  int (*get_priv_raw)(const EVP_PKEY *pkey, uint8_t *out, size_t *out_len);
  int (*get_pub_raw)(const EVP_PKEY *pkey, uint8_t *out, size_t *out_len);
  int (*get_priv_seed)(const EVP_PKEY *pkey, uint8_t *out, size_t *out_len);

  // pkey_opaque returns 1 if the |pk| is opaque. Opaque keys are backed by
  // custom implementations which do not expose key material and parameters.
  int (*pkey_opaque)(const EVP_PKEY *pk);

  int (*pkey_size)(const EVP_PKEY *pk);
  int (*pkey_bits)(const EVP_PKEY *pk);

  int (*param_missing)(const EVP_PKEY *pk);
  int (*param_copy)(EVP_PKEY *to, const EVP_PKEY *from);
  int (*param_cmp)(const EVP_PKEY *a, const EVP_PKEY *b);

  void (*pkey_free)(EVP_PKEY *pkey);
}; // EVP_PKEY_ASN1_METHOD

struct evp_pkey_st {
  CRYPTO_refcount_t references;

  // type contains one of the EVP_PKEY_* values or NID_undef and determines
  // which element (if any) of the |pkey| union is valid.
  int type;

  union {
    void *ptr;
    RSA *rsa;
    DSA *dsa;
    DH *dh;
    EC_KEY *ec;
    KEM_KEY *kem_key;
    PQDSA_KEY *pqdsa_key;
  } pkey;

  // ameth contains a pointer to a method table that contains many ASN.1
  // methods for the key type.
  const EVP_PKEY_ASN1_METHOD *ameth;
} /* EVP_PKEY */;

#define EVP_PKEY_OP_UNDEFINED 0
#define EVP_PKEY_OP_KEYGEN (1 << 2)
#define EVP_PKEY_OP_SIGN (1 << 3)
#define EVP_PKEY_OP_VERIFY (1 << 4)
#define EVP_PKEY_OP_VERIFYRECOVER (1 << 5)
#define EVP_PKEY_OP_ENCRYPT (1 << 6)
#define EVP_PKEY_OP_DECRYPT (1 << 7)
#define EVP_PKEY_OP_DERIVE (1 << 8)
#define EVP_PKEY_OP_PARAMGEN (1 << 9)

#define EVP_PKEY_OP_TYPE_SIG \
  (EVP_PKEY_OP_SIGN | EVP_PKEY_OP_VERIFY | EVP_PKEY_OP_VERIFYRECOVER)

#define EVP_PKEY_OP_TYPE_CRYPT (EVP_PKEY_OP_ENCRYPT | EVP_PKEY_OP_DECRYPT)

#define EVP_PKEY_OP_TYPE_NOGEN \
  (EVP_PKEY_OP_SIG | EVP_PKEY_OP_CRYPT | EVP_PKEY_OP_DERIVE)

#define EVP_PKEY_OP_TYPE_GEN (EVP_PKEY_OP_KEYGEN | EVP_PKEY_OP_PARAMGEN)

// EVP_PKEY_CTX_ctrl performs |cmd| on |ctx|. The |keytype| and |optype|
// arguments can be -1 to specify that any type and operation are acceptable,
// otherwise |keytype| must match the type of |ctx| and the bits of |optype|
// must intersect the operation flags set on |ctx|.
//
// The |p1| and |p2| arguments depend on the value of |cmd|.
//
// It returns one on success and zero on error.
OPENSSL_EXPORT int EVP_PKEY_CTX_ctrl(EVP_PKEY_CTX *ctx, int keytype, int optype,
                                     int cmd, int p1, void *p2);

// EVP_PKEY_CTX_md sets the message digest type for a specific operation.
// This function is deprecated and should not be used in new code.
//
// |ctx| is the context to operate on.
// |optype| is the operation type (e.g., EVP_PKEY_OP_TYPE_SIG, EVP_PKEY_OP_KEYGEN).
// |cmd| is the specific command (e.g., EVP_PKEY_CTRL_MD).
// |md| is the name of the message digest algorithm to use.
//
// It returns 1 for success and 0 or a negative value for failure.
OPENSSL_EXPORT int EVP_PKEY_CTX_md(EVP_PKEY_CTX *ctx, int optype, int cmd, const char *md);

// EVP_RSA_PKEY_CTX_ctrl is a wrapper of |EVP_PKEY_CTX_ctrl|.
// Before calling |EVP_PKEY_CTX_ctrl|, a check is added to make sure
// the |ctx->pmeth->pkey_id| is either |EVP_PKEY_RSA| or |EVP_PKEY_RSA_PSS|.
int EVP_RSA_PKEY_CTX_ctrl(EVP_PKEY_CTX *ctx, int optype, int cmd, int p1, void *p2);

#define EVP_PKEY_CTRL_MD 1
#define EVP_PKEY_CTRL_GET_MD 2
#define EVP_PKEY_CTRL_SIGNING_CONTEXT 3
#define EVP_PKEY_CTRL_GET_SIGNING_CONTEXT 4

// EVP_PKEY_CTRL_PEER_KEY is called with different values of |p1|:
//   0: Is called from |EVP_PKEY_derive_set_peer| and |p2| contains a peer key.
//      If the return value is <= 0, the key is rejected.
//   1: Is called at the end of |EVP_PKEY_derive_set_peer| and |p2| contains a
//      peer key. If the return value is <= 0, the key is rejected.
//   2: Is called with |p2| == NULL to test whether the peer's key was used.
//      (EC)DH always return one in this case.
//   3: Is called with |p2| == NULL to set whether the peer's key was used.
//      (EC)DH always return one in this case. This was only used for GOST.
#define EVP_PKEY_CTRL_PEER_KEY 3

// EVP_PKEY_ALG_CTRL is the base value from which key-type specific ctrl
// commands are numbered.
#define EVP_PKEY_ALG_CTRL 0x1000

#define EVP_PKEY_CTRL_RSA_PADDING (EVP_PKEY_ALG_CTRL + 1)
#define EVP_PKEY_CTRL_GET_RSA_PADDING (EVP_PKEY_ALG_CTRL + 2)
#define EVP_PKEY_CTRL_RSA_PSS_SALTLEN (EVP_PKEY_ALG_CTRL + 3)
#define EVP_PKEY_CTRL_GET_RSA_PSS_SALTLEN (EVP_PKEY_ALG_CTRL + 4)
#define EVP_PKEY_CTRL_RSA_KEYGEN_BITS (EVP_PKEY_ALG_CTRL + 5)
#define EVP_PKEY_CTRL_RSA_KEYGEN_PUBEXP (EVP_PKEY_ALG_CTRL + 6)
#define EVP_PKEY_CTRL_RSA_OAEP_MD (EVP_PKEY_ALG_CTRL + 7)
#define EVP_PKEY_CTRL_GET_RSA_OAEP_MD (EVP_PKEY_ALG_CTRL + 8)
#define EVP_PKEY_CTRL_RSA_MGF1_MD (EVP_PKEY_ALG_CTRL + 9)
#define EVP_PKEY_CTRL_GET_RSA_MGF1_MD (EVP_PKEY_ALG_CTRL + 10)
#define EVP_PKEY_CTRL_RSA_OAEP_LABEL (EVP_PKEY_ALG_CTRL + 11)
#define EVP_PKEY_CTRL_GET_RSA_OAEP_LABEL (EVP_PKEY_ALG_CTRL + 12)
#define EVP_PKEY_CTRL_EC_PARAMGEN_CURVE_NID (EVP_PKEY_ALG_CTRL + 13)
#define EVP_PKEY_CTRL_HKDF_MODE (EVP_PKEY_ALG_CTRL + 14)
#define EVP_PKEY_CTRL_HKDF_MD (EVP_PKEY_ALG_CTRL + 15)
#define EVP_PKEY_CTRL_HKDF_KEY (EVP_PKEY_ALG_CTRL + 16)
#define EVP_PKEY_CTRL_HKDF_SALT (EVP_PKEY_ALG_CTRL + 17)
#define EVP_PKEY_CTRL_HKDF_INFO (EVP_PKEY_ALG_CTRL + 18)
#define EVP_PKEY_CTRL_DH_PAD (EVP_PKEY_ALG_CTRL + 19)
#define EVP_PKEY_CTRL_DH_PARAMGEN_PRIME_LEN (EVP_PKEY_ALG_CTRL + 20)
#define EVP_PKEY_CTRL_DH_PARAMGEN_GENERATOR (EVP_PKEY_ALG_CTRL + 21)
#define EVP_PKEY_CTRL_SET_MAC_KEY (EVP_PKEY_ALG_CTRL + 22)
#define EVP_PKEY_CTRL_DSA_PARAMGEN_BITS (EVP_PKEY_ALG_CTRL + 23)
#define EVP_PKEY_CTRL_DSA_PARAMGEN_Q_BITS (EVP_PKEY_ALG_CTRL + 24)
#define EVP_PKEY_CTRL_DSA_PARAMGEN_MD (EVP_PKEY_ALG_CTRL + 25)

// EVP_PKEY_CTX_KEYGEN_INFO_COUNT is the maximum array length for
// |EVP_PKEY_CTX->keygen_info|. The array length corresponds to the number of
// arguments |BN_GENCB|'s callback function handles.
//
// |ctx->keygen_info| map to the following values in |BN_GENCB|:
//     1. |ctx->keygen_info[0]| -> |event|
//     2. |ctx->keygen_info[1]| -> |n|
#define EVP_PKEY_CTX_KEYGEN_INFO_COUNT 2

struct evp_pkey_ctx_st {
  // Method associated with this operation
  const EVP_PKEY_METHOD *pmeth;
  // Engine that implements this method or NULL if builtin
  ENGINE *engine;
  // Key: may be NULL
  EVP_PKEY *pkey;
  // Peer key for key agreement, may be NULL
  EVP_PKEY *peerkey;
  // operation contains one of the |EVP_PKEY_OP_*| values.
  int operation;
  // Algorithm specific data
  void *data;
  // Application specific data used by the callback.
  void *app_data;
  // Callback and specific keygen data that is mapped to |BN_GENCB| for relevant
  // implementations. This is only used for DSA, DH, and RSA in OpenSSL. AWS-LC
  // only supports RSA as of now.
  // See |EVP_PKEY_CTX_get_keygen_info| for more details.
  EVP_PKEY_gen_cb *pkey_gencb;
  int keygen_info[EVP_PKEY_CTX_KEYGEN_INFO_COUNT];
}; // EVP_PKEY_CTX

struct evp_pkey_method_st {
  int pkey_id;

  int (*init)(EVP_PKEY_CTX *ctx);
  int (*copy)(EVP_PKEY_CTX *dst, EVP_PKEY_CTX *src);
  void (*cleanup)(EVP_PKEY_CTX *ctx);

  int (*keygen)(EVP_PKEY_CTX *ctx, EVP_PKEY *pkey);

  int (*sign_init)(EVP_PKEY_CTX *ctx);
  int (*sign)(EVP_PKEY_CTX *ctx, uint8_t *sig, size_t *siglen,
              const uint8_t *tbs, size_t tbslen);

  int (*sign_message)(EVP_PKEY_CTX *ctx, uint8_t *sig, size_t *siglen,
                      const uint8_t *tbs, size_t tbslen);
  int (*verify_init)(EVP_PKEY_CTX *ctx);
  int (*verify)(EVP_PKEY_CTX *ctx, const uint8_t *sig, size_t siglen,
                const uint8_t *tbs, size_t tbslen);

  int (*verify_message)(EVP_PKEY_CTX *ctx, const uint8_t *sig, size_t siglen,
                        const uint8_t *tbs, size_t tbslen);

  int (*verify_recover)(EVP_PKEY_CTX *ctx, uint8_t *out, size_t *out_len,
                        const uint8_t *sig, size_t sig_len);

  int (*encrypt)(EVP_PKEY_CTX *ctx, uint8_t *out, size_t *outlen,
                 const uint8_t *in, size_t inlen);

  int (*decrypt)(EVP_PKEY_CTX *ctx, uint8_t *out, size_t *outlen,
                 const uint8_t *in, size_t inlen);

  int (*derive)(EVP_PKEY_CTX *ctx, uint8_t *key, size_t *keylen);

  int (*paramgen)(EVP_PKEY_CTX *ctx, EVP_PKEY *pkey);

  int (*ctrl)(EVP_PKEY_CTX *ctx, int type, int p1, void *p2);

  int (*ctrl_str) (EVP_PKEY_CTX *ctx, const char *type, const char *value);

  // Encapsulate, encapsulate_deterministic, keygen_deterministic, and
  // decapsulate are operations defined for a Key Encapsulation Mechanism (KEM).
  int (*keygen_deterministic)(EVP_PKEY_CTX *ctx,
                              EVP_PKEY *pkey,
                              const uint8_t *seed,
                              size_t *seed_len);

  int (*encapsulate_deterministic)(EVP_PKEY_CTX *ctx,
                                   uint8_t *ciphertext,
                                   size_t *ciphertext_len,
                                   uint8_t *shared_secret,
                                   size_t *shared_secret_len,
                                   const uint8_t *seed,
                                   size_t *seed_len);

  int (*encapsulate)(EVP_PKEY_CTX *ctx,
                     uint8_t *ciphertext, size_t *ciphertext_len,
                     uint8_t *shared_secret, size_t *shared_secret_len);

  int (*decapsulate)(EVP_PKEY_CTX *ctx,
                     uint8_t *shared_secret, size_t *shared_secret_len,
                     const uint8_t *ciphertext, size_t ciphertext_len);
}; // EVP_PKEY_METHOD

// used_for_hmac indicates if |ctx| is used specifically for the |EVP_PKEY_HMAC|
// operation.
int used_for_hmac(EVP_MD_CTX *ctx);

typedef struct {
  uint8_t *key;
  size_t key_len;
} HMAC_KEY;

typedef struct {
  const EVP_MD *md; // MD for HMAC use.
  HMAC_CTX ctx;
  HMAC_KEY ktmp;
} HMAC_PKEY_CTX;

// HMAC_KEY_set copies provided key into hmac_key. It frees any existing key
// on hmac_key. It returns 1 on success, and 0 otherwise.
int HMAC_KEY_set(HMAC_KEY* hmac_key, const uint8_t* key, const size_t key_len);
// HMAC_KEY_copy allocates and a new |HMAC_KEY| with identical contents (internal use).
int HMAC_KEY_copy(HMAC_KEY* dest, HMAC_KEY* src);
// HMAC_KEY_new allocates and zeroizes a |HMAC_KEY| for internal use.
HMAC_KEY *HMAC_KEY_new(void);

typedef struct {
  // key is the concatenation of the private seed and public key. It is stored
  // as a single 64-bit array to allow passing to |ED25519_sign|. If
  // |has_private| is false, the first 32 bytes are uninitialized and the public
  // key is in the last 32 bytes.
  uint8_t key[64];
  char has_private;
} ED25519_KEY;

// evp_pkey_set_cb_translate translates |ctx|'s |pkey_gencb| and sets it as the
// callback function for |cb|.
void evp_pkey_set_cb_translate(BN_GENCB *cb, EVP_PKEY_CTX *ctx);

#define ED25519_PUBLIC_KEY_OFFSET 32
#define FIPS_EVP_PKEY_METHODS 9
#define NON_FIPS_EVP_PKEY_METHODS 3
#define ASN1_EVP_PKEY_METHODS 11

struct fips_evp_pkey_methods {
  const EVP_PKEY_METHOD * methods[FIPS_EVP_PKEY_METHODS];
};

const EVP_PKEY_METHOD *EVP_PKEY_rsa_pkey_meth(void);
const EVP_PKEY_METHOD *EVP_PKEY_rsa_pss_pkey_meth(void);
const EVP_PKEY_METHOD *EVP_PKEY_ec_pkey_meth(void);
const EVP_PKEY_METHOD *EVP_PKEY_hkdf_pkey_meth(void);
const EVP_PKEY_METHOD *EVP_PKEY_hmac_pkey_meth(void);
const EVP_PKEY_METHOD *EVP_PKEY_ed25519_pkey_meth(void);
const EVP_PKEY_METHOD *EVP_PKEY_kem_pkey_meth(void);
const EVP_PKEY_METHOD *EVP_PKEY_pqdsa_pkey_meth(void);
const EVP_PKEY_METHOD *EVP_PKEY_ed25519ph_pkey_meth(void);

struct evp_pkey_ctx_signature_context_params_st {
  const uint8_t *context;
  size_t context_len;
}; // EVP_PKEY_CTX_SIGNATURE_CONTEXT_PARAMS

#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_EVP_INTERNAL_H
