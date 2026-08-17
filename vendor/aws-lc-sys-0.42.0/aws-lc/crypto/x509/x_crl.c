// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/asn1.h>
#include <openssl/asn1t.h>
#include <openssl/digest.h>
#include <openssl/err.h>
#include <openssl/mem.h>
#include <openssl/obj.h>
#include <openssl/stack.h>
#include <openssl/thread.h>
#include <openssl/x509.h>
#include <openssl/x509v3.h>

#include <assert.h>

#include "../asn1/internal.h"
#include "../internal.h"
#include "internal.h"

static int X509_REVOKED_cmp(const X509_REVOKED *const *a,
                            const X509_REVOKED *const *b);
static int setup_idp(X509_CRL *crl, ISSUING_DIST_POINT *idp);

ASN1_SEQUENCE(X509_REVOKED) = {
    ASN1_SIMPLE(X509_REVOKED, serialNumber, ASN1_INTEGER),
    ASN1_SIMPLE(X509_REVOKED, revocationDate, ASN1_TIME),
    ASN1_SEQUENCE_OF_OPT(X509_REVOKED, extensions, X509_EXTENSION),
} ASN1_SEQUENCE_END(X509_REVOKED)

static int crl_lookup(X509_CRL *crl, X509_REVOKED **ret,
                      const ASN1_INTEGER *serial, X509_NAME *issuer);

// The X509_CRL_INFO structure needs a bit of customisation. Since we cache
// the original encoding the signature wont be affected by reordering of the
// revoked field.
static int crl_inf_cb(int operation, ASN1_VALUE **pval, const ASN1_ITEM *it,
                      void *exarg) {
  X509_CRL_INFO *a = (X509_CRL_INFO *)*pval;

  if (!a || !a->revoked) {
    return 1;
  }
  switch (operation) {
      // Just set cmp function here. We don't sort because that would
      // affect the output of X509_CRL_print().
    case ASN1_OP_D2I_POST:
      (void)sk_X509_REVOKED_set_cmp_func(a->revoked, X509_REVOKED_cmp);
      break;
  }
  return 1;
}


ASN1_SEQUENCE_enc(X509_CRL_INFO, enc, crl_inf_cb) = {
    ASN1_OPT(X509_CRL_INFO, version, ASN1_INTEGER),
    ASN1_SIMPLE(X509_CRL_INFO, sig_alg, X509_ALGOR),
    ASN1_SIMPLE(X509_CRL_INFO, issuer, X509_NAME),
    ASN1_SIMPLE(X509_CRL_INFO, lastUpdate, ASN1_TIME),
    ASN1_OPT(X509_CRL_INFO, nextUpdate, ASN1_TIME),
    ASN1_SEQUENCE_OF_OPT(X509_CRL_INFO, revoked, X509_REVOKED),
    ASN1_EXP_SEQUENCE_OF_OPT(X509_CRL_INFO, extensions, X509_EXTENSION, 0),
} ASN1_SEQUENCE_END_enc(X509_CRL_INFO, X509_CRL_INFO)

static int crl_parse_entry_extensions(X509_CRL *crl) {
  STACK_OF(X509_REVOKED) *revoked = X509_CRL_get_REVOKED(crl);
  for (size_t i = 0; i < sk_X509_REVOKED_num(revoked); i++) {
    X509_REVOKED *rev = sk_X509_REVOKED_value(revoked, i);

    int crit;
    ASN1_ENUMERATED *reason =
        X509_REVOKED_get_ext_d2i(rev, NID_crl_reason, &crit, NULL);
    if (!reason && crit != -1) {
      return 0;
    }

    if (reason) {
      rev->reason = ASN1_ENUMERATED_get(reason);
      ASN1_ENUMERATED_free(reason);
    } else {
      rev->reason = CRL_REASON_NONE;
    }

    // We do not support any critical CRL entry extensions.
    const STACK_OF(X509_EXTENSION) *exts = rev->extensions;
    for (size_t j = 0; j < sk_X509_EXTENSION_num(exts); j++) {
      const X509_EXTENSION *ext = sk_X509_EXTENSION_value(exts, j);
      if (X509_EXTENSION_get_critical(ext)) {
        crl->flags |= EXFLAG_CRITICAL;
        break;
      }
    }
  }

  return 1;
}

// The X509_CRL structure needs a bit of customisation. Cache some extensions
// and hash of the whole CRL.
static int crl_cb(int operation, ASN1_VALUE **pval, const ASN1_ITEM *it,
                  void *exarg) {
  X509_CRL *crl = (X509_CRL *)*pval;
  int i;

  switch (operation) {
    case ASN1_OP_NEW_POST:
      crl->idp = NULL;
      crl->akid = NULL;
      crl->flags = 0;
      crl->idp_flags = 0;
      break;

    case ASN1_OP_D2I_POST: {
      // The version must be one of v1(0) or v2(1).
      long version = X509_CRL_VERSION_1;
      if (crl->crl->version != NULL) {
        version = ASN1_INTEGER_get(crl->crl->version);
        // TODO(https://crbug.com/boringssl/364): |X509_CRL_VERSION_1|
        // should also be rejected. This means an explicitly-encoded X.509v1
        // version. v1 is DEFAULT, so DER requires it be omitted.
        if (version < X509_CRL_VERSION_1 || version > X509_CRL_VERSION_2) {
          OPENSSL_PUT_ERROR(X509, X509_R_INVALID_VERSION);
          return 0;
        }
      }

      // Per RFC 5280, section 5.1.2.1, extensions require v2.
      if (version != X509_CRL_VERSION_2 && crl->crl->extensions != NULL) {
        OPENSSL_PUT_ERROR(X509, X509_R_INVALID_FIELD_FOR_VERSION);
        return 0;
      }

      if (!X509_CRL_digest(crl, EVP_sha256(), crl->crl_hash, NULL)) {
        return 0;
      }

      crl->idp =
          X509_CRL_get_ext_d2i(crl, NID_issuing_distribution_point, &i, NULL);
      if (crl->idp != NULL) {
        if (!setup_idp(crl, crl->idp)) {
          ISSUING_DIST_POINT_free(crl->idp);
          crl->idp = NULL;
          return 0;
        }
      } else if (i != -1) {
        return 0;
      }

      crl->akid =
          X509_CRL_get_ext_d2i(crl, NID_authority_key_identifier, &i, NULL);
      if (crl->akid == NULL && i != -1) {
        ISSUING_DIST_POINT_free(crl->idp);
        crl->idp = NULL;
        return 0;
      }

      // See if we have any unhandled critical CRL extensions and indicate
      // this in a flag. We only currently handle IDP so anything else
      // critical sets the flag. This code accesses the X509_CRL structure
      // directly: applications shouldn't do this.
      const STACK_OF(X509_EXTENSION) *exts = crl->crl->extensions;
      for (size_t idx = 0; idx < sk_X509_EXTENSION_num(exts); idx++) {
        const X509_EXTENSION *ext = sk_X509_EXTENSION_value(exts, idx);
        int nid = OBJ_obj2nid(X509_EXTENSION_get_object(ext));
        if (X509_EXTENSION_get_critical(ext)) {
          if (nid == NID_issuing_distribution_point ||
              nid == NID_authority_key_identifier) {
            continue;
          }
          crl->flags |= EXFLAG_CRITICAL;
          break;
        }
      }

      if (!crl_parse_entry_extensions(crl)) {
        AUTHORITY_KEYID_free(crl->akid);
        crl->akid = NULL;
        ISSUING_DIST_POINT_free(crl->idp);
        crl->idp = NULL;
        return 0;
      }

      break;
    }

    case ASN1_OP_FREE_POST:
      AUTHORITY_KEYID_free(crl->akid);
      ISSUING_DIST_POINT_free(crl->idp);
      break;
  }
  return 1;
}

// Convert IDP into a more convenient form
//
// TODO(davidben): Each of these flags are already booleans, so this is not
// really more convenient. We can probably remove |idp_flags|.
static int setup_idp(X509_CRL *crl, ISSUING_DIST_POINT *idp) {
  int idp_only = 0;
  // Set various flags according to IDP
  crl->idp_flags |= IDP_PRESENT;
  if (idp->onlyuser > 0) {
    idp_only++;
    crl->idp_flags |= IDP_ONLYUSER;
  }
  if (idp->onlyCA > 0) {
    idp_only++;
    crl->idp_flags |= IDP_ONLYCA;
  }
  if (idp->onlyattr > 0) {
    idp_only++;
    crl->idp_flags |= IDP_ONLYATTR;
  }

  // Per RFC 5280, section 5.2.5, at most one of onlyContainsUserCerts,
  // onlyContainsCACerts, and onlyContainsAttributeCerts may be true.
  //
  // TODO(crbug.com/boringssl/443): Move this check to the |ISSUING_DIST_POINT|
  // parser.
  if (idp_only > 1) {
    crl->idp_flags |= IDP_INVALID;
  }

  if (idp->indirectCRL > 0) {
    crl->idp_flags |= IDP_INDIRECT;
  }

  if (idp->onlysomereasons) {
    crl->idp_flags |= IDP_REASONS;
  }

  // TODO(davidben): The new verifier does not support nameRelativeToCRLIssuer.
  // Remove this?
  return DIST_POINT_set_dpname(idp->distpoint, X509_CRL_get_issuer(crl));
}

ASN1_SEQUENCE_ref(X509_CRL, crl_cb) = {
    ASN1_SIMPLE(X509_CRL, crl, X509_CRL_INFO),
    ASN1_SIMPLE(X509_CRL, sig_alg, X509_ALGOR),
    ASN1_SIMPLE(X509_CRL, signature, ASN1_BIT_STRING),
} ASN1_SEQUENCE_END_ref(X509_CRL, X509_CRL)

// Although |X509_REVOKED| contains an |X509_NAME|, it can be const. It is not
// affected by https://crbug.com/boringssl/407 because the  |X509_NAME| does
// not participate in serialization.
IMPLEMENT_ASN1_FUNCTIONS_const(X509_REVOKED)
IMPLEMENT_ASN1_DUP_FUNCTION_const(X509_REVOKED)

IMPLEMENT_ASN1_FUNCTIONS(X509_CRL_INFO)
IMPLEMENT_ASN1_FUNCTIONS(X509_CRL)
IMPLEMENT_ASN1_DUP_FUNCTION(X509_CRL)

static int X509_REVOKED_cmp(const X509_REVOKED *const *a,
                            const X509_REVOKED *const *b) {
  return ASN1_STRING_cmp((*a)->serialNumber, (*b)->serialNumber);
}

int X509_CRL_add0_revoked(X509_CRL *crl, X509_REVOKED *rev) {
  X509_CRL_INFO *inf;
  inf = crl->crl;
  if (!inf->revoked) {
    inf->revoked = sk_X509_REVOKED_new(X509_REVOKED_cmp);
  }
  if (!inf->revoked || !sk_X509_REVOKED_push(inf->revoked, rev)) {
    return 0;
  }
  asn1_encoding_clear(&inf->enc);
  return 1;
}

int X509_CRL_verify(X509_CRL *crl, EVP_PKEY *pkey) {
  if (X509_ALGOR_cmp(crl->sig_alg, crl->crl->sig_alg) != 0) {
    OPENSSL_PUT_ERROR(X509, X509_R_SIGNATURE_ALGORITHM_MISMATCH);
    return 0;
  }

  return ASN1_item_verify(ASN1_ITEM_rptr(X509_CRL_INFO), crl->sig_alg,
                          crl->signature, crl->crl, pkey);
}

int X509_CRL_get0_by_serial(X509_CRL *crl, X509_REVOKED **ret,
                            const ASN1_INTEGER *serial) {
  return crl_lookup(crl, ret, serial, NULL);
}

int X509_CRL_get0_by_cert(X509_CRL *crl, X509_REVOKED **ret, X509 *x) {
  return crl_lookup(crl, ret, X509_get_serialNumber(x),
                    X509_get_issuer_name(x));
}

static int crl_revoked_issuer_match(X509_CRL *crl, X509_NAME *nm,
                                    X509_REVOKED *rev) {
  return nm == NULL || X509_NAME_cmp(nm, X509_CRL_get_issuer(crl)) == 0;
}

static struct CRYPTO_STATIC_MUTEX g_crl_sort_lock = CRYPTO_STATIC_MUTEX_INIT;

static int crl_lookup(X509_CRL *crl, X509_REVOKED **ret,
                      const ASN1_INTEGER *serial, X509_NAME *issuer) {
  // Use an assert, rather than a runtime error, because returning nothing for a
  // CRL is arguably failing open, rather than closed.
  assert(serial->type == V_ASN1_INTEGER || serial->type == V_ASN1_NEG_INTEGER);
  X509_REVOKED rtmp, *rev;
  size_t idx;
  rtmp.serialNumber = (ASN1_INTEGER *)serial;
  // Sort revoked into serial number order if not already sorted. Do this
  // under a lock to avoid race condition.

  CRYPTO_STATIC_MUTEX_lock_read(&g_crl_sort_lock);
  const int is_sorted = sk_X509_REVOKED_is_sorted(crl->crl->revoked);
  CRYPTO_STATIC_MUTEX_unlock_read(&g_crl_sort_lock);

  if (!is_sorted) {
    CRYPTO_STATIC_MUTEX_lock_write(&g_crl_sort_lock);
    if (!sk_X509_REVOKED_is_sorted(crl->crl->revoked)) {
      sk_X509_REVOKED_sort(crl->crl->revoked);
    }
    CRYPTO_STATIC_MUTEX_unlock_write(&g_crl_sort_lock);
  }

  if (!sk_X509_REVOKED_find_awslc(crl->crl->revoked, &idx, &rtmp)) {
    return 0;
  }
  // Need to look for matching name
  for (; idx < sk_X509_REVOKED_num(crl->crl->revoked); idx++) {
    rev = sk_X509_REVOKED_value(crl->crl->revoked, idx);
    if (ASN1_INTEGER_cmp(rev->serialNumber, serial)) {
      return 0;
    }
    if (crl_revoked_issuer_match(crl, issuer, rev)) {
      if (ret) {
        *ret = rev;
      }
      return 1;
    }
  }
  return 0;
}
