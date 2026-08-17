// Written by Dr Stephen N Henson (steve@openssl.org) for the OpenSSL project 2001.
// Copyright (c) 1999-2004 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <assert.h>
#include <limits.h>
#include <string.h>

#include <openssl/digest.h>
#include <openssl/err.h>
#include <openssl/mem.h>
#include <openssl/obj.h>
#include <openssl/thread.h>
#include <openssl/x509.h>

#include "../internal.h"
#include "internal.h"

#define V1_ROOT (EXFLAG_V1 | EXFLAG_SS)
#define ku_reject(x, usage) \
  (((x)->ex_flags & EXFLAG_KUSAGE) && !((x)->ex_kusage & (usage)))
#define xku_reject(x, usage) \
  (((x)->ex_flags & EXFLAG_XKUSAGE) && !((x)->ex_xkusage & (usage)))
#define ns_reject(x, usage) \
  (((x)->ex_flags & EXFLAG_NSCERT) && !((x)->ex_nscert & (usage)))

static int check_purpose_ssl_client(const X509_PURPOSE *xp, const X509 *x,
                                    int ca);
static int check_purpose_ssl_server(const X509_PURPOSE *xp, const X509 *x,
                                    int ca);
static int check_purpose_ns_ssl_server(const X509_PURPOSE *xp, const X509 *x,
                                       int ca);
static int purpose_smime(const X509 *x, int ca);
static int check_purpose_smime_sign(const X509_PURPOSE *xp, const X509 *x,
                                    int ca);
static int check_purpose_smime_encrypt(const X509_PURPOSE *xp, const X509 *x,
                                       int ca);
static int check_purpose_crl_sign(const X509_PURPOSE *xp, const X509 *x,
                                  int ca);
static int check_purpose_timestamp_sign(const X509_PURPOSE *xp, const X509 *x,
                                        int ca);
static int no_check(const X509_PURPOSE *xp, const X509 *x, int ca);
static int ocsp_helper(const X509_PURPOSE *xp, const X509 *x, int ca);

// X509_TRUST_NONE is not a valid |X509_TRUST_*| constant. It is used by
// |X509_PURPOSE_ANY| to indicate that it has no corresponding trust type and
// cannot be used with |X509_STORE_CTX_set_purpose|.
#define X509_TRUST_NONE (-1)

static const X509_PURPOSE xstandard[] = {
    {X509_PURPOSE_SSL_CLIENT, X509_TRUST_SSL_CLIENT, 0,
     check_purpose_ssl_client, (char *)"SSL client", (char *)"sslclient", NULL},
    {X509_PURPOSE_SSL_SERVER, X509_TRUST_SSL_SERVER, 0,
     check_purpose_ssl_server, (char *)"SSL server", (char *)"sslserver", NULL},
    {X509_PURPOSE_NS_SSL_SERVER, X509_TRUST_SSL_SERVER, 0,
     check_purpose_ns_ssl_server, (char *)"Netscape SSL server",
     (char *)"nssslserver", NULL},
    {X509_PURPOSE_SMIME_SIGN, X509_TRUST_EMAIL, 0, check_purpose_smime_sign,
     (char *)"S/MIME signing", (char *)"smimesign", NULL},
    {X509_PURPOSE_SMIME_ENCRYPT, X509_TRUST_EMAIL, 0,
     check_purpose_smime_encrypt, (char *)"S/MIME encryption",
     (char *)"smimeencrypt", NULL},
    {X509_PURPOSE_CRL_SIGN, X509_TRUST_COMPAT, 0, check_purpose_crl_sign,
     (char *)"CRL signing", (char *)"crlsign", NULL},
    {X509_PURPOSE_ANY, X509_TRUST_NONE, 0, no_check, (char *)"Any Purpose",
     (char *)"any", NULL},
    {X509_PURPOSE_OCSP_HELPER, X509_TRUST_COMPAT, 0, ocsp_helper,
     (char *)"OCSP helper", (char *)"ocsphelper", NULL},
    {X509_PURPOSE_TIMESTAMP_SIGN, X509_TRUST_TSA, 0,
     check_purpose_timestamp_sign, (char *)"Time Stamp signing",
     (char *)"timestampsign", NULL},
};

int X509_check_purpose(X509 *x, int id, int ca) {
  // This differs from OpenSSL, which uses -1 to indicate a fatal error and 0 to
  // indicate an invalid certificate. BoringSSL uses 0 for both.
  if (!x509v3_cache_extensions(x)) {
    return 0;
  }

  if (id == -1) {
    return 1;
  }
  int idx = X509_PURPOSE_get_by_id(id);
  if (idx == -1) {
    return 0;
  }
  const X509_PURPOSE *pt = X509_PURPOSE_get0(idx);
  return pt->check_purpose(pt, x, ca);
}

int X509_PURPOSE_set(int *p, int purpose) {
  if (X509_PURPOSE_get_by_id(purpose) == -1) {
    OPENSSL_PUT_ERROR(X509V3, X509V3_R_INVALID_PURPOSE);
    return 0;
  }
  *p = purpose;
  return 1;
}

int X509_PURPOSE_get_count(void) { return OPENSSL_ARRAY_SIZE(xstandard); }

const X509_PURPOSE *X509_PURPOSE_get0(int idx) {
  if (idx < 0 || (size_t)idx >= OPENSSL_ARRAY_SIZE(xstandard)) {
    return NULL;
  }
  return xstandard + idx;
}

int X509_PURPOSE_get_by_sname(const char *sname) {
  const X509_PURPOSE *xptmp;
  for (int i = 0; i < X509_PURPOSE_get_count(); i++) {
    xptmp = X509_PURPOSE_get0(i);
    if (!strcmp(xptmp->sname, sname)) {
      return i;
    }
  }
  return -1;
}

int X509_PURPOSE_get_by_id(int purpose) {
  for (size_t i = 0; i <OPENSSL_ARRAY_SIZE(xstandard); i++) {
    if (xstandard[i].purpose == purpose) {
      OPENSSL_STATIC_ASSERT(OPENSSL_ARRAY_SIZE(xstandard) <= INT_MAX,
                    indices_must_fit_in_int);
      return (int)i;
    }
  }
  return -1;
}

int X509_PURPOSE_get_id(const X509_PURPOSE *xp) { return xp->purpose; }

char *X509_PURPOSE_get0_name(const X509_PURPOSE *xp) { return xp->name; }

char *X509_PURPOSE_get0_sname(const X509_PURPOSE *xp) { return xp->sname; }

int X509_PURPOSE_get_trust(const X509_PURPOSE *xp) { return xp->trust; }

int X509_supported_extension(const X509_EXTENSION *ex) {
  int nid = OBJ_obj2nid(X509_EXTENSION_get_object(ex));
  return nid == NID_netscape_cert_type || 
         nid == NID_key_usage ||
         nid == NID_subject_alt_name || 
         nid == NID_basic_constraints ||
         nid == NID_certificate_policies ||
         nid == NID_crl_distribution_points || 
         nid == NID_ext_key_usage ||
         nid == NID_policy_constraints || 
         nid == NID_name_constraints ||
         nid == NID_policy_mappings || 
         nid == NID_inhibit_any_policy;
}

static int setup_dp(X509 *x, DIST_POINT *dp) {
  if (!dp->distpoint || (dp->distpoint->type != 1)) {
    return 1;
  }
  X509_NAME *iname = NULL;
  for (size_t i = 0; i < sk_GENERAL_NAME_num(dp->CRLissuer); i++) {
    GENERAL_NAME *gen = sk_GENERAL_NAME_value(dp->CRLissuer, i);
    if (gen->type == GEN_DIRNAME) {
      iname = gen->d.directoryName;
      break;
    }
  }
  if (!iname) {
    iname = X509_get_issuer_name(x);
  }

  return DIST_POINT_set_dpname(dp->distpoint, iname);
}

static int setup_crldp(X509 *x) {
  int j;
  x->crldp = X509_get_ext_d2i(x, NID_crl_distribution_points, &j, NULL);
  if (x->crldp == NULL && j != -1) {
    return 0;
  }
  for (size_t i = 0; i < sk_DIST_POINT_num(x->crldp); i++) {
    if (!setup_dp(x, sk_DIST_POINT_value(x->crldp, i))) {
      return 0;
    }
  }
  return 1;
}

int x509v3_cache_extensions(X509 *x) {
  BASIC_CONSTRAINTS *bs;
  ASN1_BIT_STRING *usage;
  ASN1_BIT_STRING *ns;
  EXTENDED_KEY_USAGE *extusage;
  size_t i;
  int j;

  CRYPTO_MUTEX_lock_read(&x->lock);
  const int is_set = x->ex_flags & EXFLAG_SET;
  CRYPTO_MUTEX_unlock_read(&x->lock);

  if (is_set) {
    return (x->ex_flags & EXFLAG_INVALID) == 0;
  }

  CRYPTO_MUTEX_lock_write(&x->lock);
  if (x->ex_flags & EXFLAG_SET) {
    CRYPTO_MUTEX_unlock_write(&x->lock);
    return (x->ex_flags & EXFLAG_INVALID) == 0;
  }

  if (!X509_digest(x, EVP_sha256(), x->cert_hash, NULL)) {
    x->ex_flags |= EXFLAG_INVALID;
  }
  // V1 should mean no extensions ...
  if (X509_get_version(x) == X509_VERSION_1) {
    x->ex_flags |= EXFLAG_V1;
  }
  // Handle basic constraints
  if ((bs = X509_get_ext_d2i(x, NID_basic_constraints, &j, NULL))) {
    if (bs->ca) {
      x->ex_flags |= EXFLAG_CA;
    }
    if (bs->pathlen) {
      if ((bs->pathlen->type == V_ASN1_NEG_INTEGER) || !bs->ca) {
        x->ex_flags |= EXFLAG_INVALID;
        x->ex_pathlen = 0;
      } else {
        // TODO(davidben): |ASN1_INTEGER_get| returns -1 on overflow,
        // which currently acts as if the constraint isn't present. This
        // works (an overflowing path length constraint may as well be
        // infinity), but Chromium's verifier simply treats values above
        // 255 as an error.
        x->ex_pathlen = ASN1_INTEGER_get(bs->pathlen);
      }
    } else {
      x->ex_pathlen = -1;
    }
    BASIC_CONSTRAINTS_free(bs);
    x->ex_flags |= EXFLAG_BCONS;
  } else if (j != -1) {
    x->ex_flags |= EXFLAG_INVALID;
  }
  // Handle key usage
  if ((usage = X509_get_ext_d2i(x, NID_key_usage, &j, NULL))) {
    if (usage->length > 0) {
      x->ex_kusage = usage->data[0];
      if (usage->length > 1) {
        x->ex_kusage |= usage->data[1] << 8;
      }
    } else {
      x->ex_kusage = 0;
    }
    x->ex_flags |= EXFLAG_KUSAGE;
    ASN1_BIT_STRING_free(usage);
  } else if (j != -1) {
    x->ex_flags |= EXFLAG_INVALID;
  }
  x->ex_xkusage = 0;
  if ((extusage = X509_get_ext_d2i(x, NID_ext_key_usage, &j, NULL))) {
    x->ex_flags |= EXFLAG_XKUSAGE;
    for (i = 0; i < sk_ASN1_OBJECT_num(extusage); i++) {
      switch (OBJ_obj2nid(sk_ASN1_OBJECT_value(extusage, i))) {
        case NID_server_auth:
          x->ex_xkusage |= XKU_SSL_SERVER;
          break;

        case NID_client_auth:
          x->ex_xkusage |= XKU_SSL_CLIENT;
          break;

        case NID_email_protect:
          x->ex_xkusage |= XKU_SMIME;
          break;

        case NID_code_sign:
          x->ex_xkusage |= XKU_CODE_SIGN;
          break;

        case NID_ms_sgc:
        case NID_ns_sgc:
          x->ex_xkusage |= XKU_SGC;
          break;

        case NID_OCSP_sign:
          x->ex_xkusage |= XKU_OCSP_SIGN;
          break;

        case NID_time_stamp:
          x->ex_xkusage |= XKU_TIMESTAMP;
          break;

        case NID_dvcs:
          x->ex_xkusage |= XKU_DVCS;
          break;

        case NID_anyExtendedKeyUsage:
          x->ex_xkusage |= XKU_ANYEKU;
          break;
      }
    }
    sk_ASN1_OBJECT_pop_free(extusage, ASN1_OBJECT_free);
  } else if (j != -1) {
    x->ex_flags |= EXFLAG_INVALID;
  }

  if ((ns = X509_get_ext_d2i(x, NID_netscape_cert_type, &j, NULL))) {
    if (ns->length > 0) {
      x->ex_nscert = ns->data[0];
    } else {
      x->ex_nscert = 0;
    }
    x->ex_flags |= EXFLAG_NSCERT;
    ASN1_BIT_STRING_free(ns);
  } else if (j != -1) {
    x->ex_flags |= EXFLAG_INVALID;
  }
  x->skid = X509_get_ext_d2i(x, NID_subject_key_identifier, &j, NULL);
  if (x->skid == NULL && j != -1) {
    x->ex_flags |= EXFLAG_INVALID;
  }
  x->akid = X509_get_ext_d2i(x, NID_authority_key_identifier, &j, NULL);
  if (x->akid == NULL && j != -1) {
    x->ex_flags |= EXFLAG_INVALID;
  }
  // Does subject name match issuer ?
  if (!X509_NAME_cmp(X509_get_subject_name(x), X509_get_issuer_name(x))) {
    x->ex_flags |= EXFLAG_SI;
    // If SKID matches AKID also indicate self signed
    if (X509_check_akid(x, x->akid) == X509_V_OK &&
        !ku_reject(x, X509v3_KU_KEY_CERT_SIGN)) {
      x->ex_flags |= EXFLAG_SS;
    }
  }
  x->altname = X509_get_ext_d2i(x, NID_subject_alt_name, &j, NULL);
  if (x->altname == NULL && j != -1) {
    x->ex_flags |= EXFLAG_INVALID;
  }
  x->nc = X509_get_ext_d2i(x, NID_name_constraints, &j, NULL);
  if (x->nc == NULL && j != -1) {
    x->ex_flags |= EXFLAG_INVALID;
  }
  if (!setup_crldp(x)) {
    x->ex_flags |= EXFLAG_INVALID;
  }

  for (j = 0; j < X509_get_ext_count(x); j++) {
    const X509_EXTENSION *ex = X509_get_ext(x, j);
    if (!X509_EXTENSION_get_critical(ex)) {
      continue;
    }
    if (!X509_supported_extension(ex)) {
      x->ex_flags |= EXFLAG_CRITICAL;
      break;
    }
  }

  // Set x->sig_info. Errors here are ignored so that we emit similar errors
  // to OpenSSL, instead of failing early.
  (void)x509_init_signature_info(x);

  x->ex_flags |= EXFLAG_SET;

  CRYPTO_MUTEX_unlock_write(&x->lock);
  return (x->ex_flags & EXFLAG_INVALID) == 0;
}

// check_ca returns one if |x| should be considered a CA certificate and zero
// otherwise.
static int check_ca(const X509 *x) {
  // keyUsage if present should allow cert signing
  if (ku_reject(x, X509v3_KU_KEY_CERT_SIGN)) {
    return 0;
  }
  // Version 1 certificates are considered CAs and don't have extensions.
  if ((x->ex_flags & V1_ROOT) == V1_ROOT) {
    return 1;
  }
  // Otherwise, it's only a CA if basicConstraints says so.
  return ((x->ex_flags & EXFLAG_BCONS) && (x->ex_flags & EXFLAG_CA));
}

int X509_check_ca(X509 *x) {
  if (!x509v3_cache_extensions(x)) {
    return 0;
  }
  return check_ca(x);
}

static int check_purpose_ssl_client(const X509_PURPOSE *xp, const X509 *x,
                                    int ca) {
  if (xku_reject(x, XKU_SSL_CLIENT)) {
    return 0;
  }
  if (ca) {
    // TODO(davidben): Move the various |check_ca| calls out of the
    // |check_purpose| callbacks. Those checks are purpose-independent. They are
    // also redundant when called from |X509_verify_cert|, though
    // not when |X509_check_purpose| is called directly.
    return check_ca(x);
  }
  // We need to do digital signatures or key agreement
  if (ku_reject(x, X509v3_KU_DIGITAL_SIGNATURE | X509v3_KU_KEY_AGREEMENT)) {
    return 0;
  }
  // nsCertType if present should allow SSL client use
  if (ns_reject(x, NS_SSL_CLIENT)) {
    return 0;
  }
  return 1;
}

// Key usage needed for TLS/SSL server: digital signature, encipherment or
// key agreement. The ssl code can check this more thoroughly for individual
// key types.
#define X509v3_KU_TLS                                         \
  (X509v3_KU_DIGITAL_SIGNATURE | X509v3_KU_KEY_ENCIPHERMENT | \
   X509v3_KU_KEY_AGREEMENT)

static int check_purpose_ssl_server(const X509_PURPOSE *xp, const X509 *x,
                                    int ca) {
  if (xku_reject(x, XKU_SSL_SERVER)) {
    return 0;
  }
  if (ca) {
    return check_ca(x);
  }

  if (ns_reject(x, NS_SSL_SERVER)) {
    return 0;
  }
  if (ku_reject(x, X509v3_KU_TLS)) {
    return 0;
  }

  return 1;
}

static int check_purpose_ns_ssl_server(const X509_PURPOSE *xp, const X509 *x,
                                       int ca) {
  int ret = check_purpose_ssl_server(xp, x, ca);
  if (!ret || ca) {
    return ret;
  }
  // We need to encipher or Netscape complains
  if (ku_reject(x, X509v3_KU_KEY_ENCIPHERMENT)) {
    return 0;
  }
  return ret;
}

// purpose_smime returns one if |x| is a valid S/MIME leaf (|ca| is zero) or CA
// (|ca| is one) certificate, and zero otherwise.
static int purpose_smime(const X509 *x, int ca) {
  if (xku_reject(x, XKU_SMIME)) {
    return 0;
  }
  if (ca) {
    // check nsCertType if present
    if ((x->ex_flags & EXFLAG_NSCERT) && (x->ex_nscert & NS_SMIME_CA) == 0) {
      return 0;
    }

    return check_ca(x);
  }
  if (x->ex_flags & EXFLAG_NSCERT) {
    return (x->ex_nscert & NS_SMIME) == NS_SMIME;
  }
  return 1;
}

static int check_purpose_smime_sign(const X509_PURPOSE *xp, const X509 *x,
                                    int ca) {
  int ret = purpose_smime(x, ca);
  if (!ret || ca) {
    return ret;
  }
  if (ku_reject(x, X509v3_KU_DIGITAL_SIGNATURE | X509v3_KU_NON_REPUDIATION)) {
    return 0;
  }
  return ret;
}

static int check_purpose_smime_encrypt(const X509_PURPOSE *xp, const X509 *x,
                                       int ca) {
  int ret = purpose_smime(x, ca);
  if (!ret || ca) {
    return ret;
  }
  if (ku_reject(x, X509v3_KU_KEY_ENCIPHERMENT)) {
    return 0;
  }
  return ret;
}

static int check_purpose_crl_sign(const X509_PURPOSE *xp, const X509 *x,
                                  int ca) {
  if (ca) {
    return check_ca(x);
  }
  if (ku_reject(x, X509v3_KU_CRL_SIGN)) {
    return 0;
  }
  return 1;
}

// OCSP helper: this is *not* a full OCSP check. It just checks that each CA
// is valid. Additional checks must be made on the chain.

static int ocsp_helper(const X509_PURPOSE *xp, const X509 *x, int ca) {
  if (ca) {
    return check_ca(x);
  }
  // leaf certificate is checked in OCSP_verify()
  return 1;
}

static int check_purpose_timestamp_sign(const X509_PURPOSE *xp, const X509 *x,
                                        int ca) {
  // If ca is true we must return if this is a valid CA certificate.
  if (ca) {
    return check_ca(x);
  }

  // Check the optional key usage field:
  // if Key Usage is present, it must be one of digitalSignature
  // and/or nonRepudiation (other values are not consistent and shall
  // be rejected).
  if ((x->ex_flags & EXFLAG_KUSAGE) &&
      ((x->ex_kusage &
        ~(X509v3_KU_NON_REPUDIATION | X509v3_KU_DIGITAL_SIGNATURE)) ||
       !(x->ex_kusage &
         (X509v3_KU_NON_REPUDIATION | X509v3_KU_DIGITAL_SIGNATURE)))) {
    return 0;
  }

  // Only time stamp key usage is permitted and it's required.
  if (!(x->ex_flags & EXFLAG_XKUSAGE) || x->ex_xkusage != XKU_TIMESTAMP) {
    return 0;
  }

  // Extended Key Usage MUST be critical
  int i_ext = X509_get_ext_by_NID(x, NID_ext_key_usage, -1);
  if (i_ext >= 0) {
    const X509_EXTENSION *ext = X509_get_ext(x, i_ext);
    if (!X509_EXTENSION_get_critical(ext)) {
      return 0;
    }
  }

  return 1;
}

static int no_check(const X509_PURPOSE *xp, const X509 *x, int ca) { return 1; }

int X509_check_issued(X509 *issuer, X509 *subject) {
  if (X509_NAME_cmp(X509_get_subject_name(issuer),
                    X509_get_issuer_name(subject))) {
    return X509_V_ERR_SUBJECT_ISSUER_MISMATCH;
  }
  if (!x509v3_cache_extensions(issuer) || !x509v3_cache_extensions(subject)) {
    return X509_V_ERR_UNSPECIFIED;
  }

  if (subject->akid) {
    int ret = X509_check_akid(issuer, subject->akid);
    if (ret != X509_V_OK) {
      return ret;
    }
  }

  if (ku_reject(issuer, X509v3_KU_KEY_CERT_SIGN)) {
    return X509_V_ERR_KEYUSAGE_NO_CERTSIGN;
  }
  return X509_V_OK;
}

int X509_check_akid(X509 *issuer, const AUTHORITY_KEYID *akid) {
  if (!akid) {
    return X509_V_OK;
  }

  // Check key ids (if present)
  if (akid->keyid && issuer->skid &&
      ASN1_OCTET_STRING_cmp(akid->keyid, issuer->skid)) {
    return X509_V_ERR_AKID_SKID_MISMATCH;
  }
  // Check serial number
  if (akid->serial &&
      ASN1_INTEGER_cmp(X509_get_serialNumber(issuer), akid->serial)) {
    return X509_V_ERR_AKID_ISSUER_SERIAL_MISMATCH;
  }
  // Check issuer name
  if (akid->issuer) {
    // Ugh, for some peculiar reason AKID includes SEQUENCE OF
    // GeneralName. So look for a DirName. There may be more than one but
    // we only take any notice of the first.
    GENERAL_NAMES *gens;
    GENERAL_NAME *gen;
    X509_NAME *nm = NULL;
    size_t i;
    gens = akid->issuer;
    for (i = 0; i < sk_GENERAL_NAME_num(gens); i++) {
      gen = sk_GENERAL_NAME_value(gens, i);
      if (gen->type == GEN_DIRNAME) {
        nm = gen->d.dirn;
        break;
      }
    }
    if (nm && X509_NAME_cmp(nm, X509_get_issuer_name(issuer))) {
      return X509_V_ERR_AKID_ISSUER_SERIAL_MISMATCH;
    }
  }
  return X509_V_OK;
}

uint32_t X509_get_extension_flags(X509 *x) {
  // Ignore the return value. On failure, |x->ex_flags| will include
  // |EXFLAG_INVALID|.
  x509v3_cache_extensions(x);
  return x->ex_flags;
}

uint32_t X509_get_key_usage(X509 *x) {
  if (!x509v3_cache_extensions(x)) {
    return 0;
  }
  if (x->ex_flags & EXFLAG_KUSAGE) {
    return x->ex_kusage;
  }
  // If there is no extension, key usage is unconstrained, so set all bits to
  // one. Note that, although we use |UINT32_MAX|, |ex_kusage| only contains the
  // first 16 bits when the extension is present.
  return UINT32_MAX;
}

uint32_t X509_get_extended_key_usage(X509 *x) {
  if (!x509v3_cache_extensions(x)) {
    return 0;
  }
  if (x->ex_flags & EXFLAG_XKUSAGE) {
    return x->ex_xkusage;
  }
  // If there is no extension, extended key usage is unconstrained, so set all
  // bits to one.
  return UINT32_MAX;
}

const ASN1_OCTET_STRING *X509_get0_subject_key_id(X509 *x509) {
  if (!x509v3_cache_extensions(x509)) {
    return NULL;
  }
  return x509->skid;
}

const ASN1_OCTET_STRING *X509_get0_authority_key_id(X509 *x509) {
  if (!x509v3_cache_extensions(x509)) {
    return NULL;
  }
  return x509->akid != NULL ? x509->akid->keyid : NULL;
}

const GENERAL_NAMES *X509_get0_authority_issuer(X509 *x509) {
  if (!x509v3_cache_extensions(x509)) {
    return NULL;
  }
  return x509->akid != NULL ? x509->akid->issuer : NULL;
}

const ASN1_INTEGER *X509_get0_authority_serial(X509 *x509) {
  if (!x509v3_cache_extensions(x509)) {
    return NULL;
  }
  return x509->akid != NULL ? x509->akid->serial : NULL;
}

long X509_get_pathlen(X509 *x509) {
  if (!x509v3_cache_extensions(x509) || (x509->ex_flags & EXFLAG_BCONS) == 0) {
    return -1;
  }
  return x509->ex_pathlen;
}
