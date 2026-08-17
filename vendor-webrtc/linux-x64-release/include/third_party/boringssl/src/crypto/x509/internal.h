// Copyright 2013-2016 The OpenSSL Project Authors. All Rights Reserved.
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

#ifndef OPENSSL_HEADER_CRYPTO_X509_INTERNAL_H
#define OPENSSL_HEADER_CRYPTO_X509_INTERNAL_H

#include <openssl/base.h>
#include <openssl/evp.h>
#include <openssl/x509.h>

#include "../asn1/internal.h"
#include "../internal.h"

#if defined(__cplusplus)
extern "C" {
#endif


// Internal structures.

typedef struct X509_val_st {
  ASN1_TIME *notBefore;
  ASN1_TIME *notAfter;
} X509_VAL;

DECLARE_ASN1_FUNCTIONS_const(X509_VAL)

struct X509_pubkey_st {
  X509_ALGOR *algor;
  ASN1_BIT_STRING *public_key;
  EVP_PKEY *pkey;
} /* X509_PUBKEY */;

// X509_PUBKEY is an |ASN1_ITEM| whose ASN.1 type is SubjectPublicKeyInfo and C
// type is |X509_PUBKEY*|.
DECLARE_ASN1_ITEM(X509_PUBKEY)

struct X509_name_entry_st {
  ASN1_OBJECT *object;
  ASN1_STRING *value;
  int set;
} /* X509_NAME_ENTRY */;

// X509_NAME_ENTRY is an |ASN1_ITEM| whose ASN.1 type is AttributeTypeAndValue
// (RFC 5280) and C type is |X509_NAME_ENTRY*|.
DECLARE_ASN1_ITEM(X509_NAME_ENTRY)

// we always keep X509_NAMEs in 2 forms.
struct X509_name_st {
  STACK_OF(X509_NAME_ENTRY) *entries;
  int modified;  // true if 'bytes' needs to be built
  BUF_MEM *bytes;
  unsigned char *canon_enc;
  int canon_enclen;
} /* X509_NAME */;

struct x509_attributes_st {
  ASN1_OBJECT *object;
  STACK_OF(ASN1_TYPE) *set;
} /* X509_ATTRIBUTE */;

// X509_ATTRIBUTE is an |ASN1_ITEM| whose ASN.1 type is Attribute (RFC 2986) and
// C type is |X509_ATTRIBUTE*|.
DECLARE_ASN1_ITEM(X509_ATTRIBUTE)

typedef struct x509_cert_aux_st {
  STACK_OF(ASN1_OBJECT) *trust;   // trusted uses
  STACK_OF(ASN1_OBJECT) *reject;  // rejected uses
  ASN1_UTF8STRING *alias;         // "friendly name"
  ASN1_OCTET_STRING *keyid;       // key id of private key
} X509_CERT_AUX;

DECLARE_ASN1_FUNCTIONS_const(X509_CERT_AUX)

struct X509_extension_st {
  ASN1_OBJECT *object;
  ASN1_BOOLEAN critical;
  ASN1_OCTET_STRING *value;
} /* X509_EXTENSION */;

// X509_EXTENSION is an |ASN1_ITEM| whose ASN.1 type is X.509 Extension (RFC
// 5280) and C type is |X509_EXTENSION*|.
DECLARE_ASN1_ITEM(X509_EXTENSION)

// X509_EXTENSIONS is an |ASN1_ITEM| whose ASN.1 type is SEQUENCE of Extension
// (RFC 5280) and C type is |STACK_OF(X509_EXTENSION)*|.
DECLARE_ASN1_ITEM(X509_EXTENSIONS)

typedef struct {
  ASN1_INTEGER *version;  // [ 0 ] default of v1
  ASN1_INTEGER *serialNumber;
  X509_ALGOR *signature;
  X509_NAME *issuer;
  X509_VAL *validity;
  X509_NAME *subject;
  X509_PUBKEY *key;
  ASN1_BIT_STRING *issuerUID;            // [ 1 ] optional in v2
  ASN1_BIT_STRING *subjectUID;           // [ 2 ] optional in v2
  STACK_OF(X509_EXTENSION) *extensions;  // [ 3 ] optional in v3
  ASN1_ENCODING enc;
} X509_CINF;

// TODO(https://crbug.com/boringssl/407): This is not const because it contains
// an |X509_NAME|.
DECLARE_ASN1_FUNCTIONS(X509_CINF)

struct x509_st {
  X509_CINF *cert_info;
  X509_ALGOR *sig_alg;
  ASN1_BIT_STRING *signature;
  CRYPTO_refcount_t references;
  CRYPTO_EX_DATA ex_data;
  // These contain copies of various extension values
  long ex_pathlen;
  uint32_t ex_flags;
  uint32_t ex_kusage;
  uint32_t ex_xkusage;
  ASN1_OCTET_STRING *skid;
  AUTHORITY_KEYID *akid;
  STACK_OF(DIST_POINT) *crldp;
  STACK_OF(GENERAL_NAME) *altname;
  NAME_CONSTRAINTS *nc;
  unsigned char cert_hash[SHA256_DIGEST_LENGTH];
  X509_CERT_AUX *aux;
  CRYPTO_MUTEX lock;
} /* X509 */;

// X509 is an |ASN1_ITEM| whose ASN.1 type is X.509 Certificate (RFC 5280) and C
// type is |X509*|.
DECLARE_ASN1_ITEM(X509)

typedef struct {
  ASN1_ENCODING enc;
  ASN1_INTEGER *version;
  X509_NAME *subject;
  X509_PUBKEY *pubkey;
  //  d=2 hl=2 l=  0 cons: cont: 00
  STACK_OF(X509_ATTRIBUTE) *attributes;  // [ 0 ]
} X509_REQ_INFO;

// TODO(https://crbug.com/boringssl/407): This is not const because it contains
// an |X509_NAME|.
DECLARE_ASN1_FUNCTIONS(X509_REQ_INFO)

struct X509_req_st {
  X509_REQ_INFO *req_info;
  X509_ALGOR *sig_alg;
  ASN1_BIT_STRING *signature;
} /* X509_REQ */;

// X509_REQ is an |ASN1_ITEM| whose ASN.1 type is CertificateRequest (RFC 2986)
// and C type is |X509_REQ*|.
DECLARE_ASN1_ITEM(X509_REQ)

struct x509_revoked_st {
  ASN1_INTEGER *serialNumber;
  ASN1_TIME *revocationDate;
  STACK_OF(X509_EXTENSION) /* optional */ *extensions;
  // Revocation reason
  int reason;
} /* X509_REVOKED */;

// X509_REVOKED is an |ASN1_ITEM| whose ASN.1 type is an element of the
// revokedCertificates field of TBSCertList (RFC 5280) and C type is
// |X509_REVOKED*|.
DECLARE_ASN1_ITEM(X509_REVOKED)

typedef struct {
  ASN1_INTEGER *version;
  X509_ALGOR *sig_alg;
  X509_NAME *issuer;
  ASN1_TIME *lastUpdate;
  ASN1_TIME *nextUpdate;
  STACK_OF(X509_REVOKED) *revoked;
  STACK_OF(X509_EXTENSION) /* [0] */ *extensions;
  ASN1_ENCODING enc;
} X509_CRL_INFO;

// TODO(https://crbug.com/boringssl/407): This is not const because it contains
// an |X509_NAME|.
DECLARE_ASN1_FUNCTIONS(X509_CRL_INFO)

// Values in idp_flags field
// IDP present
#define IDP_PRESENT 0x1
// IDP values inconsistent
#define IDP_INVALID 0x2
// onlyuser true
#define IDP_ONLYUSER 0x4
// onlyCA true
#define IDP_ONLYCA 0x8
// onlyattr true
#define IDP_ONLYATTR 0x10
// indirectCRL true
#define IDP_INDIRECT 0x20
// onlysomereasons present
#define IDP_REASONS 0x40

struct X509_crl_st {
  // actual signature
  X509_CRL_INFO *crl;
  X509_ALGOR *sig_alg;
  ASN1_BIT_STRING *signature;
  CRYPTO_refcount_t references;
  int flags;
  // Copies of various extensions
  AUTHORITY_KEYID *akid;
  ISSUING_DIST_POINT *idp;
  // Convenient breakdown of IDP
  int idp_flags;
  unsigned char crl_hash[SHA256_DIGEST_LENGTH];
} /* X509_CRL */;

// X509_CRL is an |ASN1_ITEM| whose ASN.1 type is X.509 CertificateList (RFC
// 5280) and C type is |X509_CRL*|.
DECLARE_ASN1_ITEM(X509_CRL)

// GENERAL_NAME is an |ASN1_ITEM| whose ASN.1 type is GeneralName and C type is
// |GENERAL_NAME*|.
DECLARE_ASN1_ITEM(GENERAL_NAME)

// GENERAL_NAMES is an |ASN1_ITEM| whose ASN.1 type is SEQUENCE OF GeneralName
// and C type is |GENERAL_NAMES*|, aka |STACK_OF(GENERAL_NAME)*|.
DECLARE_ASN1_ITEM(GENERAL_NAMES)

struct X509_VERIFY_PARAM_st {
  int64_t check_time;               // POSIX time to use
  unsigned long flags;              // Various verify flags
  int purpose;                      // purpose to check untrusted certificates
  int trust;                        // trust setting to check
  int depth;                        // Verify depth
  STACK_OF(ASN1_OBJECT) *policies;  // Permissible policies
  // The following fields specify acceptable peer identities.
  STACK_OF(OPENSSL_STRING) *hosts;  // Set of acceptable names
  unsigned int hostflags;           // Flags to control matching features
  char *email;                      // If not NULL email address to match
  size_t emaillen;
  unsigned char *ip;     // If not NULL IP address to match
  size_t iplen;          // Length of IP address
  unsigned char poison;  // Fail all verifications at name checking
} /* X509_VERIFY_PARAM */;

struct x509_object_st {
  // one of the above types
  int type;
  union {
    char *ptr;
    X509 *x509;
    X509_CRL *crl;
    EVP_PKEY *pkey;
  } data;
} /* X509_OBJECT */;

// NETSCAPE_SPKI is an |ASN1_ITEM| whose ASN.1 type is
// SignedPublicKeyAndChallenge and C type is |NETSCAPE_SPKI*|.
DECLARE_ASN1_ITEM(NETSCAPE_SPKI)

// NETSCAPE_SPKAC is an |ASN1_ITEM| whose ASN.1 type is PublicKeyAndChallenge
// and C type is |NETSCAPE_SPKAC*|.
DECLARE_ASN1_ITEM(NETSCAPE_SPKAC)

// This is a static that defines the function interface
struct x509_lookup_method_st {
  int (*new_item)(X509_LOOKUP *ctx);
  void (*free)(X509_LOOKUP *ctx);
  int (*ctrl)(X509_LOOKUP *ctx, int cmd, const char *argc, long argl,
              char **ret);
  int (*get_by_subject)(X509_LOOKUP *ctx, int type, X509_NAME *name,
                        X509_OBJECT *ret);
} /* X509_LOOKUP_METHOD */;

DEFINE_STACK_OF(X509_LOOKUP)

// This is used to hold everything.  It is used for all certificate
// validation.  Once we have a certificate chain, the 'verify'
// function is then called to actually check the cert chain.
struct x509_store_st {
  // The following is a cache of trusted certs
  STACK_OF(X509_OBJECT) *objs;  // Cache of all objects
  CRYPTO_MUTEX objs_lock;

  // These are external lookup methods
  STACK_OF(X509_LOOKUP) *get_cert_methods;

  X509_VERIFY_PARAM *param;

  // Callbacks for various operations
  X509_STORE_CTX_verify_cb verify_cb;       // error callback

  CRYPTO_refcount_t references;
} /* X509_STORE */;

// This is the functions plus an instance of the local variables.
struct x509_lookup_st {
  const X509_LOOKUP_METHOD *method;  // the functions
  void *method_data;           // method data

  X509_STORE *store_ctx;  // who owns us
} /* X509_LOOKUP */;

// This is a used when verifying cert chains.  Since the
// gathering of the cert chain can take some time (and have to be
// 'retried', this needs to be kept and passed around.
struct x509_store_ctx_st {
  X509_STORE *ctx;

  // The following are set by the caller
  X509 *cert;                 // The cert to check
  STACK_OF(X509) *untrusted;  // chain of X509s - untrusted - passed in
  STACK_OF(X509_CRL) *crls;   // set of CRLs passed in

  X509_VERIFY_PARAM *param;

  // trusted_stack, if non-NULL, is a set of trusted certificates to consider
  // instead of those from |X509_STORE|.
  STACK_OF(X509) *trusted_stack;

  // Callbacks for various operations
  X509_STORE_CTX_verify_cb verify_cb;       // error callback

  // The following is built up
  int last_untrusted;     // index of last untrusted cert
  STACK_OF(X509) *chain;  // chain of X509s - built up and trusted

  // When something goes wrong, this is why
  int error_depth;
  int error;
  X509 *current_cert;
  X509_CRL *current_crl;  // current CRL

  X509 *current_crl_issuer;  // issuer of current CRL
  int current_crl_score;     // score of current CRL

  CRYPTO_EX_DATA ex_data;
} /* X509_STORE_CTX */;

ASN1_TYPE *ASN1_generate_v3(const char *str, const X509V3_CTX *cnf);

int X509_CERT_AUX_print(BIO *bp, X509_CERT_AUX *x, int indent);


// RSA-PSS functions.

// x509_rsa_pss_to_ctx configures |ctx| for an RSA-PSS operation based on
// signature algorithm parameters in |sigalg| (which must have type
// |NID_rsassaPss|) and key |pkey|. It returns one on success and zero on
// error.
int x509_rsa_pss_to_ctx(EVP_MD_CTX *ctx, const X509_ALGOR *sigalg,
                        EVP_PKEY *pkey);

// x509_rsa_pss_to_ctx sets |algor| to the signature algorithm parameters for
// |ctx|, which must have been configured for an RSA-PSS signing operation. It
// returns one on success and zero on error.
int x509_rsa_ctx_to_pss(EVP_MD_CTX *ctx, X509_ALGOR *algor);

// x509_print_rsa_pss_params prints a human-readable representation of RSA-PSS
// parameters in |sigalg| to |bp|. It returns one on success and zero on
// error.
int x509_print_rsa_pss_params(BIO *bp, const X509_ALGOR *sigalg, int indent,
                              ASN1_PCTX *pctx);


// Signature algorithm functions.

// x509_digest_sign_algorithm encodes the signing parameters of |ctx| as an
// AlgorithmIdentifier and saves the result in |algor|. It returns one on
// success, or zero on error.
int x509_digest_sign_algorithm(EVP_MD_CTX *ctx, X509_ALGOR *algor);

// x509_digest_verify_init sets up |ctx| for a signature verification operation
// with public key |pkey| and parameters from |algor|. The |ctx| argument must
// have been initialised with |EVP_MD_CTX_init|. It returns one on success, or
// zero on error.
int x509_digest_verify_init(EVP_MD_CTX *ctx, const X509_ALGOR *sigalg,
                            EVP_PKEY *pkey);


// Path-building functions.

// X509_policy_check checks certificate policies in |certs|. |user_policies| is
// the user-initial-policy-set. If |user_policies| is NULL or empty, it is
// interpreted as anyPolicy. |flags| is a set of |X509_V_FLAG_*| values to
// apply. It returns |X509_V_OK| on success and |X509_V_ERR_*| on error. It
// additionally sets |*out_current_cert| to the certificate where the error
// occurred. If the function succeeded, or the error applies to the entire
// chain, it sets |*out_current_cert| to NULL.
int X509_policy_check(const STACK_OF(X509) *certs,
                      const STACK_OF(ASN1_OBJECT) *user_policies,
                      unsigned long flags, X509 **out_current_cert);

// x509_check_issued_with_callback calls |X509_check_issued|, but allows the
// verify callback to override the result. It returns one on success and zero on
// error.
//
// TODO(davidben): Reduce the scope of the verify callback and remove this. The
// callback only runs with |X509_V_FLAG_CB_ISSUER_CHECK|, which is only used by
// one internal project and rust-openssl, who use it by mistake.
int x509_check_issued_with_callback(X509_STORE_CTX *ctx, X509 *x, X509 *issuer);

// x509v3_bytes_to_hex encodes |len| bytes from |in| to hex and returns a
// newly-allocated NUL-terminated string containing the result, or NULL on
// allocation error.
//
// This function was historically named |hex_to_string| in OpenSSL. Despite the
// name, |hex_to_string| converted to hex.
OPENSSL_EXPORT char *x509v3_bytes_to_hex(const uint8_t *in, size_t len);

// x509v3_hex_string_to_bytes decodes |str| in hex and returns a newly-allocated
// array containing the result, or NULL on error. On success, it sets |*len| to
// the length of the result. Colon separators between bytes in the input are
// allowed and ignored.
//
// This function was historically named |string_to_hex| in OpenSSL. Despite the
// name, |string_to_hex| converted from hex.
unsigned char *x509v3_hex_to_bytes(const char *str, size_t *len);

// x509v3_conf_name_matches returns one if |name| is equal to |cmp| or begins
// with |cmp| followed by '.', and zero otherwise.
int x509v3_conf_name_matches(const char *name, const char *cmp);

// x509v3_looks_like_dns_name returns one if |in| looks like a DNS name and zero
// otherwise.
OPENSSL_EXPORT int x509v3_looks_like_dns_name(const unsigned char *in,
                                              size_t len);

// x509v3_cache_extensions fills in a number of fields relating to X.509
// extensions in |x|. It returns one on success and zero if some extensions were
// invalid.
OPENSSL_EXPORT int x509v3_cache_extensions(X509 *x);

// x509v3_a2i_ipadd decodes |ipasc| as an IPv4 or IPv6 address. IPv6 addresses
// use colon-separated syntax while IPv4 addresses use dotted decimal syntax. If
// it decodes an IPv4 address, it writes the result to the first four bytes of
// |ipout| and returns four. If it decodes an IPv6 address, it writes the result
// to all 16 bytes of |ipout| and returns 16. Otherwise, it returns zero.
int x509v3_a2i_ipadd(unsigned char ipout[16], const char *ipasc);

// A |BIT_STRING_BITNAME| is used to contain a list of bit names.
typedef struct {
  int bitnum;
  const char *lname;
  const char *sname;
} BIT_STRING_BITNAME;

// x509V3_add_value_asn1_string appends a |CONF_VALUE| with the specified name
// and value to |*extlist|. if |*extlist| is NULL, it sets |*extlist| to a
// newly-allocated |STACK_OF(CONF_VALUE)| first. It returns one on success and
// zero on error.
int x509V3_add_value_asn1_string(const char *name, const ASN1_STRING *value,
                                 STACK_OF(CONF_VALUE) **extlist);

// X509V3_NAME_from_section adds attributes to |nm| by interpreting the
// key/value pairs in |dn_sk|. It returns one on success and zero on error.
// |chtype|, which should be one of |MBSTRING_*| constants, determines the
// character encoding used to interpret values.
int X509V3_NAME_from_section(X509_NAME *nm, const STACK_OF(CONF_VALUE) *dn_sk,
                             int chtype);

// X509V3_bool_from_string decodes |str| as a boolean. On success, it returns
// one and sets |*out_bool| to resulting value. Otherwise, it returns zero.
int X509V3_bool_from_string(const char *str, ASN1_BOOLEAN *out_bool);

// X509V3_get_value_bool decodes |value| as a boolean. On success, it returns
// one and sets |*out_bool| to the resulting value. Otherwise, it returns zero.
int X509V3_get_value_bool(const CONF_VALUE *value, ASN1_BOOLEAN *out_bool);

// X509V3_get_value_int decodes |value| as an integer. On success, it returns
// one and sets |*aint| to the resulting value. Otherwise, it returns zero. If
// |*aint| was non-NULL at the start of the function, it frees the previous
// value before writing a new one.
int X509V3_get_value_int(const CONF_VALUE *value, ASN1_INTEGER **aint);

// X509V3_get_section behaves like |NCONF_get_section| but queries |ctx|'s
// config database.
const STACK_OF(CONF_VALUE) *X509V3_get_section(const X509V3_CTX *ctx,
                                               const char *section);

// X509V3_add_value appends a |CONF_VALUE| containing |name| and |value| to
// |*extlist|. It returns one on success and zero on error. If |*extlist| is
// NULL, it sets |*extlist| to a newly-allocated |STACK_OF(CONF_VALUE)|
// containing the result. Either |name| or |value| may be NULL to omit the
// field.
//
// On failure, if |*extlist| was NULL, |*extlist| will remain NULL when the
// function returns.
int X509V3_add_value(const char *name, const char *value,
                     STACK_OF(CONF_VALUE) **extlist);

// X509V3_add_value_bool behaves like |X509V3_add_value| but stores the value
// "TRUE" if |asn1_bool| is non-zero and "FALSE" otherwise.
int X509V3_add_value_bool(const char *name, int asn1_bool,
                          STACK_OF(CONF_VALUE) **extlist);

// X509V3_add_value_bool behaves like |X509V3_add_value| but stores a string
// representation of |aint|. Note this string representation may be decimal or
// hexadecimal, depending on the size of |aint|.
int X509V3_add_value_int(const char *name, const ASN1_INTEGER *aint,
                         STACK_OF(CONF_VALUE) **extlist);

STACK_OF(CONF_VALUE) *X509V3_parse_list(const char *line);

#define X509V3_conf_err(val)                                               \
  ERR_add_error_data(6, "section:", (val)->section, ",name:", (val)->name, \
                     ",value:", (val)->value);

// GENERAL_NAME_cmp returns zero if |a| and |b| are equal and a non-zero
// value otherwise. Note this function does not provide a comparison suitable
// for sorting.
//
// This function is exported for testing.
OPENSSL_EXPORT int GENERAL_NAME_cmp(const GENERAL_NAME *a,
                                    const GENERAL_NAME *b);

// X509_VERIFY_PARAM_lookup returns a pre-defined |X509_VERIFY_PARAM| named by
// |name|, or NULL if no such name is defined.
const X509_VERIFY_PARAM *X509_VERIFY_PARAM_lookup(const char *name);

GENERAL_NAME *v2i_GENERAL_NAME(const X509V3_EXT_METHOD *method,
                               const X509V3_CTX *ctx, const CONF_VALUE *cnf);
GENERAL_NAME *v2i_GENERAL_NAME_ex(GENERAL_NAME *out,
                                  const X509V3_EXT_METHOD *method,
                                  const X509V3_CTX *ctx, const CONF_VALUE *cnf,
                                  int is_nc);
GENERAL_NAMES *v2i_GENERAL_NAMES(const X509V3_EXT_METHOD *method,
                                 const X509V3_CTX *ctx,
                                 const STACK_OF(CONF_VALUE) *nval);

// TODO(https://crbug.com/boringssl/407): Make |issuer| const once the
// |X509_NAME| issue is resolved.
int X509_check_akid(X509 *issuer, const AUTHORITY_KEYID *akid);

int X509_is_valid_trust_id(int trust);

int X509_PURPOSE_get_trust(const X509_PURPOSE *xp);

// TODO(https://crbug.com/boringssl/695): Remove this.
int DIST_POINT_set_dpname(DIST_POINT_NAME *dpn, X509_NAME *iname);

// x509_marshal_name marshals |in| as a DER-encoded, X.509 Name and writes the
// result to |out|. It returns one on success and zero on error.
//
// TODO(https://crbug.com/boringssl/407): This function should be const and
// thread-safe but is currently neither in some cases, notably if |in| was
// mutated.
int x509_marshal_name(CBB *out, X509_NAME *in);

// x509_marshal_algorithm marshals |in| as a DER-encoded, AlgorithmIdentifier
// and writes the result to |out|. It returns one on success and zero on error.
int x509_marshal_algorithm(CBB *out, const X509_ALGOR *in);


#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_CRYPTO_X509_INTERNAL_H
