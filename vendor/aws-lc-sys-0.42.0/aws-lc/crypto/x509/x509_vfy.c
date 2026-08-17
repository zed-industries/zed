// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <ctype.h>
#include <limits.h>
#include <string.h>
#include <time.h>

#include <openssl/asn1.h>
#include <openssl/err.h>
#include <openssl/evp.h>
#include <openssl/mem.h>
#include <openssl/obj.h>
#include <openssl/thread.h>
#include <openssl/x509.h>

#include "../internal.h"
#include "internal.h"

static CRYPTO_EX_DATA_CLASS g_ex_data_class =
    CRYPTO_EX_DATA_CLASS_INIT_WITH_APP_DATA;

// CRL score values
//
// The bit layout below is carefully chosen so that a numerical comparison
// against |CRL_SCORE_VALID| is equivalent to "NOCRITICAL, SCOPE, and TIME
// are all set." For that to hold, the sum of every bit strictly below TIME
// (ISSUER_NAME | ISSUER_CERT | SAME_PATH | AKID) must be less than TIME,
// TIME must be less than SCOPE, and SCOPE less than NOCRITICAL. Preserve
// this invariant when adjusting these values.

// No unhandled critical extensions
#define CRL_SCORE_NOCRITICAL 0x200

// certificate is within CRL scope
#define CRL_SCORE_SCOPE 0x100

// CRL times valid
#define CRL_SCORE_TIME 0x080

// CRL's IDP specifically matches the certificate's CRLDP. Always set
// alongside CRL_SCORE_SCOPE; refines a broad in-scope match into a
// specific one so a specific-IDP CRL outranks a broad no-IDP CRL of
// equal freshness. 
//
// This bit is placed strictly below CRL_SCORE_TIME 
// so that an invalid specific-IDP CRL cannot win over a valid broad CRL. 
// A specific-IDP CRL must still be fresh (TIME) and free of unhandled critical 
// extensions (NOCRITICAL) to qualify as valid.
#define CRL_SCORE_IDP_MATCH 0x040

// Issuer name matches certificate
#define CRL_SCORE_ISSUER_NAME 0x020

// If this score or above CRL is probably valid.
//
// CRL_SCORE_IDP_MATCH is intentionally excluded: a specific-IDP CRL that
// is missing NOCRITICAL or TIME must not be considered valid on the basis
// of its scope match alone.
#define CRL_SCORE_VALID \
  (CRL_SCORE_NOCRITICAL | CRL_SCORE_TIME | CRL_SCORE_SCOPE)

// CRL issuer is certificate issuer
#define CRL_SCORE_ISSUER_CERT 0x018

// CRL issuer is on certificate path
#define CRL_SCORE_SAME_PATH 0x008

// CRL issuer matches CRL AKID
#define CRL_SCORE_AKID 0x004

static int null_callback(int ok, X509_STORE_CTX *e);
static X509 *find_issuer(X509_STORE_CTX *ctx, STACK_OF(X509) *sk, X509 *x);
static int check_chain_extensions(X509_STORE_CTX *ctx);
static int check_name_constraints(X509_STORE_CTX *ctx);
static int check_id(X509_STORE_CTX *ctx);
static int check_trust(X509_STORE_CTX *ctx);
static int check_revocation(X509_STORE_CTX *ctx);
static int check_cert(X509_STORE_CTX *ctx);
static int check_policy(X509_STORE_CTX *ctx);

static int get_issuer(X509 **issuer, X509_STORE_CTX *ctx, X509 *x);
static int get_crl_score(X509_STORE_CTX *ctx, X509 **pissuer, X509_CRL *crl,
                         X509 *x);
static int get_crl(X509_STORE_CTX *ctx, X509_CRL **pcrl, X509 *x);
static int crl_akid_check(X509_STORE_CTX *ctx, X509_CRL *crl, X509 **pissuer,
                          int *pcrl_score);
static int crl_crldp_check(X509 *x, X509_CRL *crl, int crl_score, int *idp_match);
static int cert_crl(X509_STORE_CTX *ctx, X509_CRL *crl, X509 *x);

static int internal_verify(X509_STORE_CTX *ctx);

static int null_callback(int ok, X509_STORE_CTX *e) { return ok; }
static int null_verify_custom_crit_oids_callback(X509_STORE_CTX *ctx,
                                                 X509 *x509,
                                                 STACK_OF(ASN1_OBJECT) *oids) {
  // This returns 0 by default, so that the callback must be configured by the
  // user when enabling the custom critical extensions feature.
  return 0;
}

// cert_self_signed checks if |x| is self-signed. If |x| is valid, it returns
// one and sets |*out_is_self_signed| to the result. If |x| is invalid, it
// returns zero.
static int cert_self_signed(X509 *x, int *out_is_self_signed) {
  if (!x509v3_cache_extensions(x)) {
    return 0;
  }
  *out_is_self_signed = (x->ex_flags & EXFLAG_SS) != 0;
  return 1;
}

static int call_verify_cb(int ok, X509_STORE_CTX *ctx) {
  ok = ctx->verify_cb(ok, ctx);
  // Historically, callbacks returning values like -1 would be treated as a mix
  // of success or failure. Insert that callers check correctly.
  //
  // TODO(davidben): Also use this wrapper to constrain which errors may be
  // suppressed, and ensure all |verify_cb| calls remember to fill in an error.
  BSSL_CHECK(ok == 0 || ok == 1);
  return ok;
}

// Given a certificate try and find an exact match in the store
static X509 *lookup_cert_match(X509_STORE_CTX *ctx, X509 *x) {
  STACK_OF(X509) *certs;
  X509 *xtmp = NULL;
  size_t i;
  // Lookup all certs with matching subject name
  certs = X509_STORE_CTX_get1_certs(ctx, X509_get_subject_name(x));
  if (certs == NULL) {
    return NULL;
  }
  // Look for exact match
  for (i = 0; i < sk_X509_num(certs); i++) {
    xtmp = sk_X509_value(certs, i);
    if (!X509_cmp(xtmp, x)) {
      break;
    }
  }
  if (i < sk_X509_num(certs)) {
    X509_up_ref(xtmp);
  } else {
    xtmp = NULL;
  }
  sk_X509_pop_free(certs, X509_free);
  return xtmp;
}

int X509_verify_cert(X509_STORE_CTX *ctx) {
  X509 *xtmp, *xtmp2, *chain_ss = NULL;
  int bad_chain = 0;
  X509_VERIFY_PARAM *param = ctx->param;
  int i, ok = 0;
  int j, retry, trust;
  STACK_OF(X509) *sktmp = NULL;

  if (ctx->cert == NULL) {
    OPENSSL_PUT_ERROR(X509, X509_R_NO_CERT_SET_FOR_US_TO_VERIFY);
    ctx->error = X509_V_ERR_INVALID_CALL;
    return -1;
  }

  if (ctx->chain != NULL) {
    // This X509_STORE_CTX has already been used to verify a cert. We
    // cannot do another one.
    OPENSSL_PUT_ERROR(X509, ERR_R_SHOULD_NOT_HAVE_BEEN_CALLED);
    ctx->error = X509_V_ERR_INVALID_CALL;
    return -1;
  }

  if (ctx->param->flags &
      (X509_V_FLAG_EXTENDED_CRL_SUPPORT | X509_V_FLAG_USE_DELTAS)) {
    // We do not support indirect or delta CRLs. The flags still exist for
    // compatibility with bindings libraries, but to ensure we do not
    // inadvertently skip a CRL check that the caller expects, fail closed.
    OPENSSL_PUT_ERROR(X509, ERR_R_SHOULD_NOT_HAVE_BEEN_CALLED);
    ctx->error = X509_V_ERR_INVALID_CALL;
    return -1;
  }

  // first we make sure the chain we are going to build is present and that
  // the first entry is in place
  ctx->chain = sk_X509_new_null();
  if (ctx->chain == NULL || !sk_X509_push(ctx->chain, ctx->cert)) {
    ctx->error = X509_V_ERR_OUT_OF_MEM;
    goto end;
  }
  X509_up_ref(ctx->cert);
  ctx->last_untrusted = 1;

  // We use a temporary STACK so we can chop and hack at it.
  if (ctx->untrusted != NULL && (sktmp = sk_X509_dup(ctx->untrusted)) == NULL) {
    ctx->error = X509_V_ERR_OUT_OF_MEM;
    goto end;
  }

  int num = (int)sk_X509_num(ctx->chain);
  X509 *x = sk_X509_value(ctx->chain, num - 1);
  // |param->depth| does not include the leaf certificate or the trust anchor,
  // so the maximum size is 2 more.
  int max_chain = param->depth >= INT_MAX - 2 ? INT_MAX : param->depth + 2;

  for (;;) {
    if (num >= max_chain) {
      // FIXME: If this happens, we should take note of it and, if appropriate,
      // use the X509_V_ERR_CERT_CHAIN_TOO_LONG error code later.
      break;
    }

    int is_self_signed;
    if (!cert_self_signed(x, &is_self_signed)) {
      ctx->error = X509_V_ERR_INVALID_EXTENSION;
      goto end;
    }

    // If we are self signed, we break
    if (is_self_signed) {
      break;
    }
    // If asked see if we can find issuer in trusted store first
    if (ctx->param->flags & X509_V_FLAG_TRUSTED_FIRST) {
      ok = get_issuer(&xtmp, ctx, x);
      if (ok < 0) {
        ctx->error = X509_V_ERR_STORE_LOOKUP;
        goto end;
      }
      // If successful for now free up cert so it will be picked up
      // again later.
      if (ok > 0) {
        X509_free(xtmp);
        break;
      }
    }

    // If we were passed a cert chain, use it first
    if (sktmp != NULL) {
      xtmp = find_issuer(ctx, sktmp, x);
      if (xtmp != NULL) {
        if (!sk_X509_push(ctx->chain, xtmp)) {
          ctx->error = X509_V_ERR_OUT_OF_MEM;
          ok = 0;
          goto end;
        }
        X509_up_ref(xtmp);
        (void)sk_X509_delete_ptr(sktmp, xtmp);
        ctx->last_untrusted++;
        x = xtmp;
        num++;
        // reparse the full chain for the next one
        continue;
      }
    }
    break;
  }

  // Remember how many untrusted certs we have
  j = num;
  // at this point, chain should contain a list of untrusted certificates.
  // We now need to add at least one trusted one, if possible, otherwise we
  // complain.

  do {
    // Examine last certificate in chain and see if it is self signed.
    i = (int)sk_X509_num(ctx->chain);
    x = sk_X509_value(ctx->chain, i - 1);

    int is_self_signed;
    if (!cert_self_signed(x, &is_self_signed)) {
      ctx->error = X509_V_ERR_INVALID_EXTENSION;
      goto end;
    }

    if (is_self_signed) {
      // we have a self signed certificate
      if (sk_X509_num(ctx->chain) == 1) {
        // We have a single self signed certificate: see if we can
        // find it in the store. We must have an exact match to avoid
        // possible impersonation.
        ok = get_issuer(&xtmp, ctx, x);
        if ((ok <= 0) || X509_cmp(x, xtmp)) {
          ctx->error = X509_V_ERR_DEPTH_ZERO_SELF_SIGNED_CERT;
          ctx->current_cert = x;
          ctx->error_depth = i - 1;
          if (ok == 1) {
            X509_free(xtmp);
          }
          bad_chain = 1;
          ok = call_verify_cb(0, ctx);
          if (!ok) {
            goto end;
          }
        } else {
          // We have a match: replace certificate with store
          // version so we get any trust settings.
          X509_free(x);
          x = xtmp;
          (void)sk_X509_set(ctx->chain, i - 1, x);
          ctx->last_untrusted = 0;
        }
      } else {
        // extract and save self signed certificate for later use
        chain_ss = sk_X509_pop(ctx->chain);
        ctx->last_untrusted--;
        num--;
        j--;
        x = sk_X509_value(ctx->chain, num - 1);
      }
    }
    // We now lookup certs from the certificate store
    for (;;) {
      if (num >= max_chain) {
        // FIXME: If this happens, we should take note of it and, if
        // appropriate, use the X509_V_ERR_CERT_CHAIN_TOO_LONG error code later.
        break;
      }
      if (!cert_self_signed(x, &is_self_signed)) {
        ctx->error = X509_V_ERR_INVALID_EXTENSION;
        goto end;
      }
      // If we are self signed, we break
      if (is_self_signed) {
        break;
      }
      ok = get_issuer(&xtmp, ctx, x);

      if (ok < 0) {
        ctx->error = X509_V_ERR_STORE_LOOKUP;
        goto end;
      }
      if (ok == 0) {
        break;
      }
      x = xtmp;
      if (!sk_X509_push(ctx->chain, x)) {
        X509_free(xtmp);
        ctx->error = X509_V_ERR_OUT_OF_MEM;
        ok = 0;
        goto end;
      }

      // OpenSSL 1.1.1 continuously re-checks for trust and breaks the loop
      // as soon as trust has been established. 1.0.2 builds the chain with all
      // possible certs first and only checks for trust if the final cert in
      // the chain is self-signed.
      // This caused additional unanticipated certs to be in the established
      // certificate chain, particularly when |X509_V_FLAG_PARTIAL_CHAIN| was
      // set. We try checking continuously for trust here for better 1.1.1
      // compatibility.
      trust = check_trust(ctx);
      if (trust == X509_TRUST_TRUSTED || trust == X509_TRUST_REJECTED) {
        break;
      }

      num++;
    }

    // we now have our chain, lets check it...
    trust = check_trust(ctx);

    // If explicitly rejected error
    if (trust == X509_TRUST_REJECTED) {
      ok = 0;
      goto end;
    }
    // If it's not explicitly trusted then check if there is an alternative
    // chain that could be used. We only do this if we haven't already
    // checked via TRUSTED_FIRST and the user hasn't switched off alternate
    // chain checking
    retry = 0;
    if (trust != X509_TRUST_TRUSTED &&
        !(ctx->param->flags & X509_V_FLAG_TRUSTED_FIRST) &&
        !(ctx->param->flags & X509_V_FLAG_NO_ALT_CHAINS)) {
      while (j-- > 1) {
        xtmp2 = sk_X509_value(ctx->chain, j - 1);
        ok = get_issuer(&xtmp, ctx, xtmp2);
        if (ok < 0) {
          goto end;
        }
        // Check if we found an alternate chain
        if (ok > 0) {
          // Free up the found cert we'll add it again later
          X509_free(xtmp);

          // Dump all the certs above this point - we've found an
          // alternate chain
          while (num > j) {
            xtmp = sk_X509_pop(ctx->chain);
            X509_free(xtmp);
            num--;
          }
          ctx->last_untrusted = (int)sk_X509_num(ctx->chain);
          retry = 1;
          break;
        }
      }
    }
  } while (retry);

  // If not explicitly trusted then indicate error unless it's a single
  // self signed certificate in which case we've indicated an error already
  // and set bad_chain == 1
  if (trust != X509_TRUST_TRUSTED && !bad_chain) {
    if (chain_ss == NULL ||
        !x509_check_issued_with_callback(ctx, x, chain_ss)) {
      if (ctx->last_untrusted >= num) {
        ctx->error = X509_V_ERR_UNABLE_TO_GET_ISSUER_CERT_LOCALLY;
      } else {
        ctx->error = X509_V_ERR_UNABLE_TO_GET_ISSUER_CERT;
      }
      ctx->current_cert = x;
    } else {
      if (!sk_X509_push(ctx->chain, chain_ss)) {
        ctx->error = X509_V_ERR_OUT_OF_MEM;
        ok = 0;
        goto end;
      }
      num++;
      ctx->last_untrusted = num;
      ctx->current_cert = chain_ss;
      ctx->error = X509_V_ERR_SELF_SIGNED_CERT_IN_CHAIN;
      chain_ss = NULL;
    }

    ctx->error_depth = num - 1;
    bad_chain = 1;
    ok = call_verify_cb(0, ctx);
    if (!ok) {
      goto end;
    }
  }

  // We have the chain complete: now we need to check its purpose
  ok = check_chain_extensions(ctx);

  if (!ok) {
    goto end;
  }

  ok = check_id(ctx);

  if (!ok) {
    goto end;
  }

  // Check revocation status: we do this after copying parameters because
  // they may be needed for CRL signature verification.
  ok = check_revocation(ctx);
  if (!ok) {
    goto end;
  }

  // At this point, we have a chain and need to verify it
  ok = internal_verify(ctx);
  if (!ok) {
    goto end;
  }

  // Check name constraints

  ok = check_name_constraints(ctx);
  if (!ok) {
    goto end;
  }

  // If we get this far evaluate policies
  if (!bad_chain && (ctx->param->flags & X509_V_FLAG_POLICY_CHECK)) {
    ok = check_policy(ctx);
  }

end:
  if (sktmp != NULL) {
    sk_X509_free(sktmp);
  }
  if (chain_ss != NULL) {
    X509_free(chain_ss);
  }

  // Safety net, error returns must set ctx->error
  if (ok <= 0 && ctx->error == X509_V_OK) {
    ctx->error = X509_V_ERR_UNSPECIFIED;
  }
  return ok;
}

// Given a STACK_OF(X509) find the issuer of cert (if any)
//
// |find_issuer| will directly return the pointer of the corresponding index
// within |sk|. Callers of |find_issuer| should remember to bump the reference
// count of the returned |X509| if the call is successful.
static X509 *find_issuer(X509_STORE_CTX *ctx, STACK_OF(X509) *sk, X509 *x) {
  size_t i;
  X509 *issuer, *candidate = NULL;
  for (i = 0; i < sk_X509_num(sk); i++) {
    issuer = sk_X509_value(sk, i);
    if (x509_check_issued_with_callback(ctx, x, issuer)) {
      candidate = issuer;
      if (x509_check_cert_time(ctx, candidate, /*suppress_error*/ 1)) {
        break;
      }
    }
  }
  return candidate;
}

// Given a possible certificate and issuer check them

int x509_check_issued_with_callback(X509_STORE_CTX *ctx, X509 *x,
                                    X509 *issuer) {
  int ret;
  ret = X509_check_issued(issuer, x);
  if (ret == X509_V_OK) {
    return 1;
  }
  // If we haven't asked for issuer errors don't set ctx
  if (!(ctx->param->flags & X509_V_FLAG_CB_ISSUER_CHECK)) {
    return 0;
  }

  ctx->error = ret;
  ctx->current_cert = x;
  ctx->current_issuer = issuer;
  return call_verify_cb(0, ctx);
}

static int get_issuer(X509 **issuer, X509_STORE_CTX *ctx, X509 *x) {
  if (ctx->trusted_stack != NULL) {
    // Ignore the store and use the configured stack instead.
    *issuer = find_issuer(ctx, ctx->trusted_stack, x);
    if (*issuer) {
      X509_up_ref(*issuer);
      return 1;
    }
    return 0;
  }

  return X509_STORE_CTX_get1_issuer(issuer, ctx, x);
}

static int check_custom_critical_extensions(X509_STORE_CTX *ctx, X509 *x) {
  if (ctx->custom_crit_oids == NULL) {
    // Fail if custom critical extensions are enabled, but none were set.
    return 0;
  }
  size_t known_oid_count = sk_ASN1_OBJECT_num(ctx->custom_crit_oids);
  if (known_oid_count == 0) {
    return 0;
  }

  // Allocate |found_exts| to pass to the callback.
  STACK_OF(ASN1_OBJECT) *found_exts = sk_ASN1_OBJECT_new_null();
  if (found_exts == NULL) {
    return 0;
  }

  // Iterate through all critical extensions of |x| and validate against the
  // ones that aren't recognized by |X509_supported_extension|.
  int last_pos = X509_get_ext_by_critical(x, 1, -1);
  while (last_pos >= 0) {
    const X509_EXTENSION *ext = X509_get_ext(x, last_pos);
    if (!X509_supported_extension(ext)) {
      int found = 0;

      // Iterate through all set |custom_crit_oids|.
      for (size_t i = 0; i < known_oid_count; i++) {
        const ASN1_OBJECT *known_ext =
            sk_ASN1_OBJECT_value(ctx->custom_crit_oids, i);
        if (OBJ_cmp(ext->object, known_ext) == 0) {
          // |sk_ASN1_OBJECT_value| returns a direct pointer.
          ASN1_OBJECT *dup_obj = OBJ_dup(known_ext);
          if (dup_obj == NULL || !sk_ASN1_OBJECT_push(found_exts, dup_obj)) {
            ASN1_OBJECT_free(dup_obj);
            sk_ASN1_OBJECT_pop_free(found_exts, ASN1_OBJECT_free);
            return 0;
          }
          found = 1;
          break;
        }
      }

      if (!found) {
        // If any critical extension isn't in our known list, return early.
        sk_ASN1_OBJECT_pop_free(found_exts, ASN1_OBJECT_free);
        return 0;
      }
    }
    last_pos = X509_get_ext_by_critical(x, 1, last_pos);
  }

  // If we get here, all unknown critical extensions in |x| were
  // properly handled and we pass the ones that were found to the caller.
  if (!ctx->verify_custom_crit_oids(ctx, x, found_exts)) {
    sk_ASN1_OBJECT_pop_free(found_exts, ASN1_OBJECT_free);
    return 0;
  }

  sk_ASN1_OBJECT_pop_free(found_exts, ASN1_OBJECT_free);
  return 1;
}

static int check_curve_decoded_by_name(X509 *x) {
  EVP_PKEY *pkey = X509_get0_pubkey(x);
  if (!pkey) {
    return -1;
  }

  if (EVP_PKEY_id(pkey) != EVP_PKEY_EC) {
    return 1;
  }

  EC_KEY *ec_key = EVP_PKEY_get0_EC_KEY(pkey);
  if (ec_key == NULL) {
    return -1;
  }

  OPENSSL_BEGIN_ALLOW_DEPRECATED
  int is_explicit = EC_KEY_decoded_from_explicit_params(ec_key);
  OPENSSL_END_ALLOW_DEPRECATED

  return !is_explicit;
}

// Check a certificate chains extensions for consistency with the supplied
// purpose

static int check_chain_extensions(X509_STORE_CTX *ctx) {
  int ok = 0, plen = 0;
  int purpose = ctx->param->purpose;

  int num = (int)sk_X509_num(ctx->chain);

  for (int i = 0; i < num; i++) {
    X509 *x = sk_X509_value(ctx->chain, i);
    if (  // OpenSSL's historic check for unknown critical extensions.
          // |EXFLAG_CRITICAL| indicates an unsupported critical extension was
          // found in |x| during the initial parsing of the certificate.
        (!(ctx->param->flags & X509_V_FLAG_IGNORE_CRITICAL) &&
         (x->ex_flags & EXFLAG_CRITICAL)) &&
        // AWS-LC specific logic for enabling custom unknown critical
        // extensions.
        !check_custom_critical_extensions(ctx, x)) {
      ctx->error = X509_V_ERR_UNHANDLED_CRITICAL_EXTENSION;
      ctx->error_depth = i;
      ctx->current_cert = x;
      ok = call_verify_cb(0, ctx);
      if (!ok) {
        goto end;
      }
    }

    // The |num > 1| condition intentionally skips this check for single-certificate chains
    // (e.g., a self-signed leaf or a partial chain with only the target certificate). This
    // maintains consistency with OpenSSL 1.1.1 behavior.
    if (num > 1 && (ctx->param->awslc_flags&AWSLC_V_ENABLE_EC_KEY_EXPLICIT_PARAMS) == 0) {
      // In OpenSSL 1.1.1 this check was done if |X509_V_FLAG_X509_STRICT| was enabled.
      // We did not perform this check explicitly before, but the behavior was baked into
      // SPKI with explicit parameters not being parsed for elliptic curve key types.
      //
      // A conforming public CA should not issue certificate with explicitly encoded parameters for
      // EC keys, but for privately operated CA's this could be doing so still. To be backwards
      // compatible with OpenSSL 1.1.1 we have added support for parsing SPKI with explicit parameters,
      // so we have added a new option for enabling this during chain validation.
      int ret = check_curve_decoded_by_name(x);
      if (ret < 0) {
        ctx->error = X509_V_ERR_UNSPECIFIED;
        ctx->error_depth = i;
        ctx->current_cert = x;
        ok = 0;
        goto end;
      } else if (ret == 0) {
        ctx->error = X509_V_ERR_EC_KEY_EXPLICIT_PARAMS;
        ctx->error_depth = i;
        ctx->current_cert = x;
        ok = 0;
        goto end;
      }
    }

    int must_be_ca = i > 0;
    if (must_be_ca && !X509_check_ca(x)) {
      ctx->error = X509_V_ERR_INVALID_CA;
      ctx->error_depth = i;
      ctx->current_cert = x;
      ok = call_verify_cb(0, ctx);
      if (!ok) {
        goto end;
      }
    }
    if (ctx->param->purpose > 0 &&
        X509_check_purpose(x, purpose, must_be_ca) != 1) {
      ctx->error = X509_V_ERR_INVALID_PURPOSE;
      ctx->error_depth = i;
      ctx->current_cert = x;
      ok = call_verify_cb(0, ctx);
      if (!ok) {
        goto end;
      }
    }
    // Check pathlen if not self issued
    if (i > 1 && !(x->ex_flags & EXFLAG_SI) && x->ex_pathlen != -1 &&
        plen - 1 > x->ex_pathlen) {
      ctx->error = X509_V_ERR_PATH_LENGTH_EXCEEDED;
      ctx->error_depth = i;
      ctx->current_cert = x;
      ok = call_verify_cb(0, ctx);
      if (!ok) {
        goto end;
      }
    }
    // Increment path length if not self issued
    if (!(x->ex_flags & EXFLAG_SI)) {
      plen++;
    }
  }
  ok = 1;
end:
  return ok;
}

static int has_san_id(X509 *x, int type_id) {
  int ret = 0;

  GENERAL_NAMES *gs = X509_get_ext_d2i(x, NID_subject_alt_name, NULL, NULL);
  if (gs == NULL) {
    return 0;
  }

  for (size_t i = 0; i < sk_GENERAL_NAME_num(gs); i++) {
    GENERAL_NAME *g = sk_GENERAL_NAME_value(gs, i);
    if (g->type == type_id) {
      ret = 1;
      break;
    }
  }
  GENERAL_NAMES_free(gs);
  return ret;
}

static int check_name_constraints(X509_STORE_CTX *ctx) {
  int i, j, rv;
  // Check name constraints for all certificates
  for (i = (int)sk_X509_num(ctx->chain) - 1; i >= 0; i--) {
    X509 *x = sk_X509_value(ctx->chain, i);
    // Ignore self issued certs unless last in chain
    if (i && (x->ex_flags & EXFLAG_SI)) {
      continue;
    }
    // Check against constraints for all certificates higher in chain
    // including trust anchor. Trust anchor not strictly speaking needed
    // but if it includes constraints it is to be assumed it expects them
    // to be obeyed.
    for (j = (int)sk_X509_num(ctx->chain) - 1; j > i; j--) {
      NAME_CONSTRAINTS *nc = sk_X509_value(ctx->chain, j)->nc;
      if (nc) {
        rv = NAME_CONSTRAINTS_check(x, nc);
        if (rv == X509_V_OK &&
            (ctx->param->hostflags & X509_CHECK_FLAG_NEVER_CHECK_SUBJECT) ==
                0 &&
            ((ctx->param->hostflags & X509_CHECK_FLAG_ALWAYS_CHECK_SUBJECT) !=
                0 ||!has_san_id(x, GEN_DNS) )) {
          rv = NAME_CONSTRAINTS_check_CN(x, nc);
        }
        switch (rv) {
          case X509_V_OK:
            continue;
          case X509_V_ERR_OUT_OF_MEM:
            ctx->error = rv;
            return 0;
          default:
            ctx->error = rv;
            ctx->error_depth = i;
            ctx->current_cert = x;
            if (!call_verify_cb(0, ctx)) {
              return 0;
            }
            break;
        }
      }
    }
  }

  return 1;
}

static int check_id_error(X509_STORE_CTX *ctx, int errcode) {
  ctx->error = errcode;
  ctx->current_cert = ctx->cert;
  ctx->error_depth = 0;
  return call_verify_cb(0, ctx);
}

static int check_hosts(X509 *x, X509_VERIFY_PARAM *param) {
  size_t i;
  size_t n = sk_OPENSSL_STRING_num(param->hosts);
  char *name;

  for (i = 0; i < n; ++i) {
    name = sk_OPENSSL_STRING_value(param->hosts, i);
    if (X509_check_host(x, name, strlen(name), param->hostflags, NULL) > 0) {
      return 1;
    }
  }
  return n == 0;
}

static int check_id(X509_STORE_CTX *ctx) {
  X509_VERIFY_PARAM *vpm = ctx->param;
  X509 *x = ctx->cert;
  if (vpm->poison) {
    if (!check_id_error(ctx, X509_V_ERR_INVALID_CALL)) {
      return 0;
    }
  }
  if (vpm->hosts && check_hosts(x, vpm) <= 0) {
    if (!check_id_error(ctx, X509_V_ERR_HOSTNAME_MISMATCH)) {
      return 0;
    }
  }
  if (vpm->email && X509_check_email(x, vpm->email, vpm->emaillen, 0) <= 0) {
    if (!check_id_error(ctx, X509_V_ERR_EMAIL_MISMATCH)) {
      return 0;
    }
  }
  if (vpm->ip && X509_check_ip(x, vpm->ip, vpm->iplen, 0) <= 0) {
    if (!check_id_error(ctx, X509_V_ERR_IP_ADDRESS_MISMATCH)) {
      return 0;
    }
  }
  return 1;
}

static int check_trust(X509_STORE_CTX *ctx) {
  int ok;
  X509 *x = NULL;
  // Check all trusted certificates in chain
  for (size_t i = ctx->last_untrusted; i < sk_X509_num(ctx->chain); i++) {
    x = sk_X509_value(ctx->chain, i);
    ok = X509_check_trust(x, ctx->param->trust, 0);
    // If explicitly trusted return trusted
    if (ok == X509_TRUST_TRUSTED) {
      return X509_TRUST_TRUSTED;
    }
    // If explicitly rejected notify callback and reject if not
    // overridden.
    if (ok == X509_TRUST_REJECTED) {
      ctx->error_depth = (int)i;
      ctx->current_cert = x;
      ctx->error = X509_V_ERR_CERT_REJECTED;
      ok = call_verify_cb(0, ctx);
      if (!ok) {
        return X509_TRUST_REJECTED;
      }
    }
  }
  // If we accept partial chains and have at least one trusted certificate
  // return success.
  if (ctx->param->flags & X509_V_FLAG_PARTIAL_CHAIN) {
    X509 *mx;
    if (ctx->last_untrusted < (int)sk_X509_num(ctx->chain)) {
      return X509_TRUST_TRUSTED;
    }
    x = sk_X509_value(ctx->chain, 0);
    mx = lookup_cert_match(ctx, x);
    if (mx) {
      (void)sk_X509_set(ctx->chain, 0, mx);
      X509_free(x);
      ctx->last_untrusted = 0;
      return X509_TRUST_TRUSTED;
    }
  }

  // If no trusted certs in chain at all return untrusted and allow
  // standard (no issuer cert) etc errors to be indicated.
  return X509_TRUST_UNTRUSTED;
}

static int check_revocation(X509_STORE_CTX *ctx) {
  if (!(ctx->param->flags & X509_V_FLAG_CRL_CHECK)) {
    return 1;
  }
  int last;
  if (ctx->param->flags & X509_V_FLAG_CRL_CHECK_ALL) {
    last = (int)sk_X509_num(ctx->chain) - 1;
  } else {
    last = 0;
  }
  for (int i = 0; i <= last; i++) {
    ctx->error_depth = i;
    int ok = check_cert(ctx);
    if (!ok) {
      return ok;
    }
  }
  return 1;
}

static int check_cert(X509_STORE_CTX *ctx) {
  X509_CRL *crl = NULL;
  int ok = 0, cnum = ctx->error_depth;
  X509 *x = sk_X509_value(ctx->chain, cnum);
  ctx->current_cert = x;
  ctx->current_issuer = NULL;
  ctx->current_crl_score = 0;

  // Try to retrieve relevant CRL
  ok = ctx->get_crl(ctx, &crl, x);
  // If error looking up CRL, nothing we can do except notify callback
  if (!ok) {
    ctx->error = X509_V_ERR_UNABLE_TO_GET_CRL;
    ok = call_verify_cb(0, ctx);
    goto err;
  }
  ctx->current_crl = crl;
  ok = ctx->check_crl(ctx, crl);
  if (!ok) {
    goto err;
  }

  ok = cert_crl(ctx, crl, x);
  if (!ok) {
    goto err;
  }

err:
  X509_CRL_free(crl);
  ctx->current_crl = NULL;
  return ok;
}

// Check CRL times against values in X509_STORE_CTX
static int check_crl_time(X509_STORE_CTX *ctx, X509_CRL *crl, int notify) {
  if (ctx->param->flags & X509_V_FLAG_NO_CHECK_TIME) {
    return 1;
  }

  if (notify) {
    ctx->current_crl = crl;
  }
  int64_t ptime;
  if (ctx->param->flags & X509_V_FLAG_USE_CHECK_TIME) {
    ptime = ctx->param->check_time;
  } else {
    ptime = time(NULL);
  }

  int i = X509_cmp_time_posix(X509_CRL_get0_lastUpdate(crl), ptime);
  if (i == 0) {
    if (!notify) {
      return 0;
    }
    ctx->error = X509_V_ERR_ERROR_IN_CRL_LAST_UPDATE_FIELD;
    if (!call_verify_cb(0, ctx)) {
      return 0;
    }
  }

  if (i > 0) {
    if (!notify) {
      return 0;
    }
    ctx->error = X509_V_ERR_CRL_NOT_YET_VALID;
    if (!call_verify_cb(0, ctx)) {
      return 0;
    }
  }

  if (X509_CRL_get0_nextUpdate(crl)) {
    i = X509_cmp_time_posix(X509_CRL_get0_nextUpdate(crl), ptime);

    if (i == 0) {
      if (!notify) {
        return 0;
      }
      ctx->error = X509_V_ERR_ERROR_IN_CRL_NEXT_UPDATE_FIELD;
      if (!call_verify_cb(0, ctx)) {
        return 0;
      }
    }
    if (i < 0) {
      if (!notify) {
        return 0;
      }
      ctx->error = X509_V_ERR_CRL_HAS_EXPIRED;
      if (!call_verify_cb(0, ctx)) {
        return 0;
      }
    }
  }

  if (notify) {
    ctx->current_crl = NULL;
  }

  return 1;
}

static int get_crl_sk(X509_STORE_CTX *ctx, X509_CRL **pcrl, X509 **pissuer,
                      int *pscore, STACK_OF(X509_CRL) *crls) {
  int crl_score, best_score = *pscore;
  X509 *x = ctx->current_cert;
  X509_CRL *best_crl = NULL;
  X509 *crl_issuer = NULL, *best_crl_issuer = NULL;

  for (size_t i = 0; i < sk_X509_CRL_num(crls); i++) {
    X509_CRL *crl = sk_X509_CRL_value(crls, i);
    crl_score = get_crl_score(ctx, &crl_issuer, crl, x);
    if (crl_score < best_score || crl_score == 0) {
      continue;
    }
    // If current CRL is equivalent use it if it is newer
    if (crl_score == best_score && best_crl != NULL) {
      int day, sec;
      if (ASN1_TIME_diff(&day, &sec, X509_CRL_get0_lastUpdate(best_crl),
                         X509_CRL_get0_lastUpdate(crl)) == 0) {
        continue;
      }
      // ASN1_TIME_diff never returns inconsistent signs for |day|
      // and |sec|.
      if (day <= 0 && sec <= 0) {
        continue;
      }
    }
    best_crl = crl;
    best_crl_issuer = crl_issuer;
    best_score = crl_score;
  }

  if (best_crl) {
    if (*pcrl) {
      X509_CRL_free(*pcrl);
    }
    *pcrl = best_crl;
    *pissuer = best_crl_issuer;
    *pscore = best_score;
    X509_CRL_up_ref(best_crl);
  }

  if (best_score >= CRL_SCORE_VALID) {
    return 1;
  }

  return 0;
}

// For a given CRL return how suitable it is for the supplied certificate
// 'x'. The return value is a mask of several criteria. If the issuer is not
// the certificate issuer this is returned in *pissuer.
static int get_crl_score(X509_STORE_CTX *ctx, X509 **pissuer, X509_CRL *crl,
                         X509 *x) {
  int crl_score = 0;

  // First see if we can reject CRL straight away

  // Invalid IDP cannot be processed
  if (crl->idp_flags & IDP_INVALID) {
    return 0;
  }
  // Reason codes and indirect CRLs are not supported.
  if (crl->idp_flags & (IDP_INDIRECT | IDP_REASONS)) {
    return 0;
  }
  // We do not support indirect CRLs, so the issuer names must match.
  if (X509_NAME_cmp(X509_get_issuer_name(x), X509_CRL_get_issuer(crl))) {
    return 0;
  }
  crl_score |= CRL_SCORE_ISSUER_NAME;

  if (!(crl->flags & EXFLAG_CRITICAL)) {
    crl_score |= CRL_SCORE_NOCRITICAL;
  }

  // Check expiry
  if (check_crl_time(ctx, crl, 0)) {
    crl_score |= CRL_SCORE_TIME;
  }

  // Check authority key ID and locate certificate issuer
  if (!crl_akid_check(ctx, crl, pissuer, &crl_score)) {
    // If we can't locate certificate issuer at this point forget it
    return 0;
  }

  int idp_match = 0;
  if (crl_crldp_check(x, crl, crl_score, &idp_match)) {
    crl_score |= CRL_SCORE_SCOPE;
    if (idp_match) {
      crl_score |= CRL_SCORE_IDP_MATCH;
    }
  }
  return crl_score;
}

static int crl_akid_check(X509_STORE_CTX *ctx, X509_CRL *crl, X509 **pissuer,
                          int *pcrl_score) {
  X509 *crl_issuer = NULL;
  X509_NAME *cnm = X509_CRL_get_issuer(crl);
  int cidx = ctx->error_depth;

  if ((size_t)cidx != sk_X509_num(ctx->chain) - 1) {
    cidx++;
  }

  crl_issuer = sk_X509_value(ctx->chain, cidx);

  if (X509_check_akid(crl_issuer, crl->akid) == X509_V_OK) {
    *pcrl_score |= CRL_SCORE_AKID | CRL_SCORE_ISSUER_CERT;
    *pissuer = crl_issuer;
    return 1;
  }

  for (cidx++; cidx < (int)sk_X509_num(ctx->chain); cidx++) {
    crl_issuer = sk_X509_value(ctx->chain, cidx);
    if (X509_NAME_cmp(X509_get_subject_name(crl_issuer), cnm)) {
      continue;
    }
    if (X509_check_akid(crl_issuer, crl->akid) == X509_V_OK) {
      *pcrl_score |= CRL_SCORE_AKID | CRL_SCORE_SAME_PATH;
      *pissuer = crl_issuer;
      return 1;
    }
  }

  return 0;
}

// Check for match between two dist point names: three separate cases. 1.
// Both are relative names and compare X509_NAME types. 2. One full, one
// relative. Compare X509_NAME to GENERAL_NAMES. 3. Both are full names and
// compare two GENERAL_NAMES. 4. One is NULL: automatic match.
static int idp_check_dp(DIST_POINT_NAME *a, DIST_POINT_NAME *b) {
  X509_NAME *nm = NULL;
  GENERAL_NAMES *gens = NULL;
  GENERAL_NAME *gena, *genb;
  size_t i, j;
  if (!a || !b) {
    return 1;
  }
  if (a->type == 1) {
    if (!a->dpname) {
      return 0;
    }
    // Case 1: two X509_NAME
    if (b->type == 1) {
      if (!b->dpname) {
        return 0;
      }
      if (!X509_NAME_cmp(a->dpname, b->dpname)) {
        return 1;
      } else {
        return 0;
      }
    }
    // Case 2: set name and GENERAL_NAMES appropriately
    nm = a->dpname;
    gens = b->name.fullname;
  } else if (b->type == 1) {
    if (!b->dpname) {
      return 0;
    }
    // Case 2: set name and GENERAL_NAMES appropriately
    gens = a->name.fullname;
    nm = b->dpname;
  }

  // Handle case 2 with one GENERAL_NAMES and one X509_NAME
  if (nm) {
    for (i = 0; i < sk_GENERAL_NAME_num(gens); i++) {
      gena = sk_GENERAL_NAME_value(gens, i);
      if (gena->type != GEN_DIRNAME) {
        continue;
      }
      if (!X509_NAME_cmp(nm, gena->d.directoryName)) {
        return 1;
      }
    }
    return 0;
  }

  // Else case 3: two GENERAL_NAMES

  for (i = 0; i < sk_GENERAL_NAME_num(a->name.fullname); i++) {
    gena = sk_GENERAL_NAME_value(a->name.fullname, i);
    for (j = 0; j < sk_GENERAL_NAME_num(b->name.fullname); j++) {
      genb = sk_GENERAL_NAME_value(b->name.fullname, j);
      if (!GENERAL_NAME_cmp(gena, genb)) {
        return 1;
      }
    }
  }

  return 0;
}

// Check CRLDP and IDP. Return true when the CRL is a good
// candidate CRL from which to check revocation of the certificate.
static int crl_crldp_check(X509 *x, X509_CRL *crl, int crl_score, int *idp_match) {
  if (crl->idp_flags & IDP_ONLYATTR) {
    return 0;
  }
  if (x->ex_flags & EXFLAG_CA) {
    if (crl->idp_flags & IDP_ONLYUSER) {
      return 0;
    }
  } else {
    if (crl->idp_flags & IDP_ONLYCA) {
      return 0;
    }
  }
  for (size_t i = 0; i < sk_DIST_POINT_num(x->crldp); i++) {
    DIST_POINT *dp = sk_DIST_POINT_value(x->crldp, i);
    // Skip distribution points with a reasons field or a CRL issuer:
    //
    // We do not support CRLs partitioned by reason code. RFC 5280 requires CAs
    // include at least one DistributionPoint that covers all reasons.
    //
    // We also do not support indirect CRLs, and a CRL issuer can only match
    // indirect CRLs (RFC 5280, section 6.3.3, step b.1).
    if (dp->reasons != NULL || dp->CRLissuer != NULL) {
      continue;
    }
    // At this point we have already checked that the CRL issuer matches
    // the certificate issuer (and set CRL_SCORE_ISSUER_NAME);

    // RFC 5280 Section 6.3.3 step b.2

    // A CRL with a specific IDP match will be preferred
    // over a broad no-IDP/empty-IDP match.
    if (crl->idp && crl->idp->distpoint &&
        idp_check_dp(dp->distpoint, crl->idp->distpoint)) {
      *idp_match = 1;
      return 1;
    }
  }

  // If the CRL does not specify an issuing distribution point, allow it to
  // match anything.
  // RFC5280 section 6.3.3 check (b).(2) does not prescribe what to do if the
  // CRL does not include an IDP. This fallback returns CRL_SCORE_SCOPE (broad
  // match) if the CRL did not include an IDP or an IDP without a DP. Such a
  // CRL could still be a good candidate CRL to check against although we
  // cannot check if it matches the DP in the certificate. A CRL with a
  // specific IDP match receives (CRL_SCORE_SCOPE | CRL_SCORE_IDP_MATCH), so it will be preferred
  // over a broad match. Among CRLs with the same scope class, get_crl_sk()
  // will pick the freshest one.
  return !crl->idp || !crl->idp->distpoint;
}

// Retrieve CRL corresponding to current certificate.
static int get_crl(X509_STORE_CTX *ctx, X509_CRL **pcrl, X509 *x) {
  int ok;
  X509 *issuer = NULL;
  int crl_score = 0;
  X509_CRL *crl = NULL;
  STACK_OF(X509_CRL) *skcrl;
  X509_NAME *nm = X509_get_issuer_name(x);
  ok = get_crl_sk(ctx, &crl, &issuer, &crl_score, ctx->crls);
  if (ok) {
    goto done;
  }

  // Lookup CRLs from store
  skcrl = ctx->lookup_crls(ctx, nm);

  // If no CRLs found and a near match from get_crl_sk use that
  if (!skcrl && crl) {
    goto done;
  }

  get_crl_sk(ctx, &crl, &issuer, &crl_score, skcrl);

  sk_X509_CRL_pop_free(skcrl, X509_CRL_free);

done:

  // If we got any kind of CRL use it and return success
  if (crl) {
    ctx->current_issuer = issuer;
    ctx->current_crl_score = crl_score;
    *pcrl = crl;
    return 1;
  }

  return 0;
}

// Check CRL validity
static int check_crl(X509_STORE_CTX *ctx, X509_CRL *crl) {
  X509 *issuer = NULL;
  int cnum = ctx->error_depth;
  int chnum = (int)sk_X509_num(ctx->chain) - 1;
  // if we have an alternative CRL issuer cert use that
  if (ctx->current_issuer) {
    issuer = ctx->current_issuer;
  }

  // Else find CRL issuer: if not last certificate then issuer is next
  // certificate in chain.
  else if (cnum < chnum) {
    issuer = sk_X509_value(ctx->chain, cnum + 1);
  } else {
    issuer = sk_X509_value(ctx->chain, chnum);
    // If not self signed, can't check signature
    if (!x509_check_issued_with_callback(ctx, issuer, issuer)) {
      ctx->error = X509_V_ERR_UNABLE_TO_GET_CRL_ISSUER;
      if (!call_verify_cb(0, ctx)) {
        return 0;
      }
    }
  }

  if (issuer) {
    // Check for cRLSign bit if keyUsage present
    if ((issuer->ex_flags & EXFLAG_KUSAGE) &&
        !(issuer->ex_kusage & X509v3_KU_CRL_SIGN)) {
      ctx->error = X509_V_ERR_KEYUSAGE_NO_CRL_SIGN;
      if (!call_verify_cb(0, ctx)) {
        return 0;
      }
    }

    if (!(ctx->current_crl_score & CRL_SCORE_SCOPE)) {
      ctx->error = X509_V_ERR_DIFFERENT_CRL_SCOPE;
      if (!call_verify_cb(0, ctx)) {
        return 0;
      }
    }

    if (crl->idp_flags & IDP_INVALID) {
      ctx->error = X509_V_ERR_INVALID_EXTENSION;
      if (!call_verify_cb(0, ctx)) {
        return 0;
      }
    }

    if (!(ctx->current_crl_score & CRL_SCORE_TIME)) {
      if (!check_crl_time(ctx, crl, 1)) {
        return 0;
      }
    }

    // Attempt to get issuer certificate public key
    EVP_PKEY *ikey = X509_get0_pubkey(issuer);
    if (!ikey) {
      ctx->error = X509_V_ERR_UNABLE_TO_DECODE_ISSUER_PUBLIC_KEY;
      if (!call_verify_cb(0, ctx)) {
        return 0;
      }
    } else {
      // Verify CRL signature
      if (X509_CRL_verify(crl, ikey) <= 0) {
        ctx->error = X509_V_ERR_CRL_SIGNATURE_FAILURE;
        if (!call_verify_cb(0, ctx)) {
          return 0;
        }
      }
    }
  }

  return 1;
}

// Check certificate against CRL
static int cert_crl(X509_STORE_CTX *ctx, X509_CRL *crl, X509 *x) {
  int ok;
  X509_REVOKED *rev;
  // The rules changed for this... previously if a CRL contained unhandled
  // critical extensions it could still be used to indicate a certificate
  // was revoked. This has since been changed since critical extension can
  // change the meaning of CRL entries.
  if (!(ctx->param->flags & X509_V_FLAG_IGNORE_CRITICAL) &&
      (crl->flags & EXFLAG_CRITICAL)) {
    ctx->error = X509_V_ERR_UNHANDLED_CRITICAL_CRL_EXTENSION;
    ok = call_verify_cb(0, ctx);
    if (!ok) {
      return 0;
    }
  }
  // Look for serial number of certificate in CRL.
  if (X509_CRL_get0_by_cert(crl, &rev, x)) {
    ctx->error = X509_V_ERR_CERT_REVOKED;
    ok = call_verify_cb(0, ctx);
    if (!ok) {
      return 0;
    }
  }

  return 1;
}

static int check_policy(X509_STORE_CTX *ctx) {
  X509 *current_cert = NULL;
  int ret = X509_policy_check(ctx->chain, ctx->param->policies,
                              ctx->param->flags, &current_cert);
  if (ret != X509_V_OK) {
    ctx->current_cert = current_cert;
    ctx->error = ret;
    if (ret == X509_V_ERR_OUT_OF_MEM) {
      return 0;
    }
    return call_verify_cb(0, ctx);
  }

  return 1;
}

int x509_check_cert_time(X509_STORE_CTX *ctx, X509 *x509, int suppress_error) {
  if (ctx->param->flags & X509_V_FLAG_NO_CHECK_TIME) {
    return 1;
  }

  int64_t ptime;
  if (ctx->param->flags & X509_V_FLAG_USE_CHECK_TIME) {
    ptime = ctx->param->check_time;
  } else {
    ptime = time(NULL);
  }

  int i = X509_cmp_time_posix(X509_get_notBefore(x509), ptime);
  if (i == 0) {
    if (suppress_error != 0) {
      return 0;
    }
    ctx->error = X509_V_ERR_ERROR_IN_CERT_NOT_BEFORE_FIELD;
    ctx->current_cert = x509;
    if (!call_verify_cb(0, ctx)) {
      return 0;
    }
  }

  if (i > 0) {
    if (suppress_error != 0) {
      return 0;
    }
    ctx->error = X509_V_ERR_CERT_NOT_YET_VALID;
    ctx->current_cert = x509;
    if (!call_verify_cb(0, ctx)) {
      return 0;
    }
  }

  i = X509_cmp_time_posix(X509_get_notAfter(x509), ptime);
  if (i == 0) {
    if (suppress_error != 0) {
      return 0;
    }
    ctx->error = X509_V_ERR_ERROR_IN_CERT_NOT_AFTER_FIELD;
    ctx->current_cert = x509;
    if (!call_verify_cb(0, ctx)) {
      return 0;
    }
  }

  if (i < 0) {
    if (suppress_error != 0) {
      return 0;
    }
    ctx->error = X509_V_ERR_CERT_HAS_EXPIRED;
    ctx->current_cert = x509;
    if (!call_verify_cb(0, ctx)) {
      return 0;
    }
  }

  return 1;
}

static int internal_verify(X509_STORE_CTX *ctx) {
  int ok = 0;
  X509 *xs, *xi;

  int n = (int)sk_X509_num(ctx->chain);
  ctx->error_depth = n - 1;
  n--;
  xi = sk_X509_value(ctx->chain, n);

  if (x509_check_issued_with_callback(ctx, xi, xi)) {
    xs = xi;
  } else {
    if (ctx->param->flags & X509_V_FLAG_PARTIAL_CHAIN) {
      xs = xi;
      goto check_cert;
    }
    if (n <= 0) {
      ctx->error = X509_V_ERR_UNABLE_TO_VERIFY_LEAF_SIGNATURE;
      ctx->current_cert = xi;
      ok = call_verify_cb(0, ctx);
      goto end;
    } else {
      n--;
      ctx->error_depth = n;
      xs = sk_X509_value(ctx->chain, n);
    }
  }

  //      ctx->error=0;  not needed
  while (n >= 0) {
    ctx->error_depth = n;

    // Skip signature check for self signed certificates unless
    // explicitly asked for. It doesn't add any security and just wastes
    // time.
    if (xs != xi || (ctx->param->flags & X509_V_FLAG_CHECK_SS_SIGNATURE)) {
      EVP_PKEY *pkey = X509_get0_pubkey(xi);
      if (pkey == NULL) {
        ctx->error = X509_V_ERR_UNABLE_TO_DECODE_ISSUER_PUBLIC_KEY;
        ctx->current_cert = xi;
        ok = call_verify_cb(0, ctx);
        if (!ok) {
          goto end;
        }
      } else if (X509_verify(xs, pkey) <= 0) {
        ctx->error = X509_V_ERR_CERT_SIGNATURE_FAILURE;
        ctx->current_cert = xs;
        ok = call_verify_cb(0, ctx);
        if (!ok) {
          goto end;
        }
      }
    }

  check_cert:
    ok = x509_check_cert_time(ctx, xs, /*suppress_error*/ 0);
    if (!ok) {
      goto end;
    }

    // The last error (if any) is still in the error value
    ctx->current_issuer = xi;
    ctx->current_cert = xs;
    ok = call_verify_cb(1, ctx);
    if (!ok) {
      goto end;
    }

    n--;
    if (n >= 0) {
      xi = xs;
      xs = sk_X509_value(ctx->chain, n);
    }
  }

  // Check that the final certificate subject (leaf)
  // has a public key we actually support. Otherwise
  // we shouldn't consider the certificate okay. It is possible
  // for it to be reject sooner during |check_chain_extensions| as well
  // but disabling certain checks can result in an unuseable leaf certificate
  // making it to this point.
  //
  // This covers situations like invalid curves or points.
  EVP_PKEY *pkey = X509_get0_pubkey(xs);
  if(pkey == NULL) {
    ctx->error = X509_V_UNABLE_TO_GET_CERTS_PUBLIC_KEY;
    ctx->current_cert = xs;
    ok = 0;
    goto end;
  }

  ok = 1;
end:
  return ok;
}

int X509_cmp_current_time(const ASN1_TIME *ctm) {
  return X509_cmp_time_posix(ctm, time(NULL));
}

int X509_cmp_time(const ASN1_TIME *ctm, const time_t *cmp_time) {
  int64_t compare_time = (cmp_time == NULL) ? time(NULL) : *cmp_time;
  return X509_cmp_time_posix(ctm, compare_time);
}

int X509_cmp_time_posix(const ASN1_TIME *ctm, int64_t cmp_time) {
  int64_t ctm_time;
  if (!ASN1_TIME_to_posix(ctm, &ctm_time)) {
    return 0;
  }
  // The return value 0 is reserved for errors.
  return (ctm_time - cmp_time <= 0) ? -1 : 1;
}

ASN1_TIME *X509_gmtime_adj(ASN1_TIME *s, long offset_sec) {
  return X509_time_adj(s, offset_sec, NULL);
}

ASN1_TIME *X509_time_adj(ASN1_TIME *s, long offset_sec, const time_t *in_tm) {
  return X509_time_adj_ex(s, 0, offset_sec, in_tm);
}

ASN1_TIME *X509_time_adj_ex(ASN1_TIME *s, int offset_day, long offset_sec,
                            const time_t *in_tm) {
  int64_t t = 0;

  if (in_tm) {
    t = *in_tm;
  } else {
    t = time(NULL);
  }

  return ASN1_TIME_adj(s, t, offset_day, offset_sec);
}

int X509_STORE_CTX_get_ex_new_index(long argl, void *argp,
                                    CRYPTO_EX_unused *unused,
                                    CRYPTO_EX_dup *dup_unused,
                                    CRYPTO_EX_free *free_func) {
  // This function is (usually) called only once, by
  // SSL_get_ex_data_X509_STORE_CTX_idx (ssl/ssl_cert.c).
  int index;
  if (!CRYPTO_get_ex_new_index(&g_ex_data_class, &index, argl, argp,
                               free_func)) {
    return -1;
  }
  return index;
}

int X509_STORE_CTX_set_ex_data(X509_STORE_CTX *ctx, int idx, void *data) {
  return CRYPTO_set_ex_data(&ctx->ex_data, idx, data);
}

void *X509_STORE_CTX_get_ex_data(X509_STORE_CTX *ctx, int idx) {
  return CRYPTO_get_ex_data(&ctx->ex_data, idx);
}

int X509_STORE_CTX_get_error(X509_STORE_CTX *ctx) { return ctx->error; }

void X509_STORE_CTX_set_error(X509_STORE_CTX *ctx, int err) {
  ctx->error = err;
}

int X509_STORE_CTX_get_error_depth(X509_STORE_CTX *ctx) {
  return ctx->error_depth;
}

X509 *X509_STORE_CTX_get_current_cert(X509_STORE_CTX *ctx) {
  return ctx->current_cert;
}

STACK_OF(X509) *X509_STORE_CTX_get_chain(X509_STORE_CTX *ctx) {
  return ctx->chain;
}

STACK_OF(X509) *X509_STORE_CTX_get0_chain(X509_STORE_CTX *ctx) {
  return ctx->chain;
}

STACK_OF(X509) *X509_STORE_CTX_get1_chain(X509_STORE_CTX *ctx) {
  if (!ctx->chain) {
    return NULL;
  }
  return X509_chain_up_ref(ctx->chain);
}

X509 *X509_STORE_CTX_get0_current_issuer(X509_STORE_CTX *ctx) {
  return ctx->current_issuer;
}

X509_CRL *X509_STORE_CTX_get0_current_crl(X509_STORE_CTX *ctx) {
  return ctx->current_crl;
}

X509_STORE_CTX *X509_STORE_CTX_get0_parent_ctx(X509_STORE_CTX *ctx) {
  // In OpenSSL, an |X509_STORE_CTX| sometimes has a parent context during CRL
  // path validation for indirect CRLs. We require the CRL to be issued
  // somewhere along the certificate path, so this is always NULL.
  return NULL;
}

void X509_STORE_CTX_set_cert(X509_STORE_CTX *ctx, X509 *x) { ctx->cert = x; }

void X509_STORE_CTX_set_chain(X509_STORE_CTX *ctx, STACK_OF(X509) *sk) {
  ctx->untrusted = sk;
}

void X509_STORE_CTX_set0_untrusted(X509_STORE_CTX *ctx, STACK_OF(X509) *sk) {
  X509_STORE_CTX_set_chain(ctx, sk);
}

STACK_OF(X509) *X509_STORE_CTX_get0_untrusted(X509_STORE_CTX *ctx) {
  return ctx->untrusted;
}

void X509_STORE_CTX_set0_crls(X509_STORE_CTX *ctx, STACK_OF(X509_CRL) *sk) {
  ctx->crls = sk;
}

int X509_STORE_CTX_set_purpose(X509_STORE_CTX *ctx, int purpose) {
  // If |purpose| is zero, this function historically silently did nothing.
  if (purpose == 0) {
    return 1;
  }

  int idx = X509_PURPOSE_get_by_id(purpose);
  if (idx == -1) {
    OPENSSL_PUT_ERROR(X509, X509_R_UNKNOWN_PURPOSE_ID);
    return 0;
  }

  int trust = X509_PURPOSE_get_trust(X509_PURPOSE_get0(idx));
  if (!X509_STORE_CTX_set_trust(ctx, trust)) {
    return 0;
  }

  if (ctx->param->purpose == 0) {
    ctx->param->purpose = purpose;
  }
  return 1;
}

int X509_STORE_CTX_set_trust(X509_STORE_CTX *ctx, int trust) {
  // If |trust| is zero, this function historically silently did nothing.
  if (trust == 0) {
    return 1;
  }

  if (X509_TRUST_get_by_id(trust) == -1) {
    OPENSSL_PUT_ERROR(X509, X509_R_UNKNOWN_TRUST_ID);
    return 0;
  }

  if (ctx->param->trust == 0) {
    ctx->param->trust = trust;
  }
  return 1;
}

X509_STORE_CTX *X509_STORE_CTX_new(void) {
  return OPENSSL_zalloc(sizeof(X509_STORE_CTX));
}

void X509_STORE_CTX_free(X509_STORE_CTX *ctx) {
  if (ctx == NULL) {
    return;
  }
  X509_STORE_CTX_cleanup(ctx);
  OPENSSL_free(ctx);
}

int X509_STORE_CTX_init(X509_STORE_CTX *ctx, X509_STORE *store, X509 *x509,
                        STACK_OF(X509) *chain) {
  X509_STORE_CTX_cleanup(ctx);

  ctx->ctx = store;
  ctx->cert = x509;
  ctx->untrusted = chain;

  CRYPTO_new_ex_data(&ctx->ex_data);

  if (store == NULL) {
    OPENSSL_PUT_ERROR(X509, ERR_R_PASSED_NULL_PARAMETER);
    goto err;
  }

  ctx->param = X509_VERIFY_PARAM_new();
  if (!ctx->param) {
    goto err;
  }

  // Inherit callbacks and flags from X509_STORE.

  ctx->verify_cb = store->verify_cb;

  if (!X509_VERIFY_PARAM_inherit(ctx->param, store->param) ||
      !X509_VERIFY_PARAM_inherit(ctx->param,
                                 X509_VERIFY_PARAM_lookup("default"))) {
    goto err;
  }

  if (store->verify_cb) {
    ctx->verify_cb = store->verify_cb;
  } else {
    ctx->verify_cb = null_callback;
  }

  if (store->get_crl) {
    ctx->get_crl = store->get_crl;
  } else {
    ctx->get_crl = get_crl;
  }

  if (store->check_crl) {
    ctx->check_crl = store->check_crl;
  } else {
    ctx->check_crl = check_crl;
  }

  if (store->lookup_crls) {
    ctx->lookup_crls = store->lookup_crls;
  } else {
    ctx->lookup_crls = X509_STORE_get1_crls;
  }

  ctx->verify_custom_crit_oids = null_verify_custom_crit_oids_callback;

  return 1;

err:
  CRYPTO_free_ex_data(&g_ex_data_class, ctx, &ctx->ex_data);
  if (ctx->param != NULL) {
    X509_VERIFY_PARAM_free(ctx->param);
  }

  OPENSSL_memset(ctx, 0, sizeof(X509_STORE_CTX));
  return 0;
}

// Set alternative lookup method: just a STACK of trusted certificates. This
// avoids X509_STORE nastiness where it isn't needed.

void X509_STORE_CTX_set0_trusted_stack(X509_STORE_CTX *ctx,
                                       STACK_OF(X509) *sk) {
  ctx->trusted_stack = sk;
}

void X509_STORE_CTX_trusted_stack(X509_STORE_CTX *ctx, STACK_OF(X509) *sk) {
  X509_STORE_CTX_set0_trusted_stack(ctx, sk);
}

void X509_STORE_CTX_cleanup(X509_STORE_CTX *ctx) {
  CRYPTO_free_ex_data(&g_ex_data_class, ctx, &(ctx->ex_data));
  X509_VERIFY_PARAM_free(ctx->param);
  sk_X509_pop_free(ctx->chain, X509_free);
  sk_ASN1_OBJECT_pop_free(ctx->custom_crit_oids, ASN1_OBJECT_free);
  OPENSSL_memset(ctx, 0, sizeof(X509_STORE_CTX));
}

void X509_STORE_CTX_set_depth(X509_STORE_CTX *ctx, int depth) {
  X509_VERIFY_PARAM_set_depth(ctx->param, depth);
}

void X509_STORE_CTX_set_flags(X509_STORE_CTX *ctx, unsigned long flags) {
  X509_VERIFY_PARAM_set_flags(ctx->param, flags);
}

void X509_STORE_CTX_set_time_posix(X509_STORE_CTX *ctx, unsigned long flags,
                                   int64_t t) {
  X509_VERIFY_PARAM_set_time_posix(ctx->param, t);
}

void X509_STORE_CTX_set_time(X509_STORE_CTX *ctx, unsigned long flags,
                             time_t t) {
  X509_STORE_CTX_set_time_posix(ctx, flags, t);
}

X509 *X509_STORE_CTX_get0_cert(X509_STORE_CTX *ctx) {
  return ctx->cert;
}

void X509_STORE_CTX_set_verify_cb(X509_STORE_CTX *ctx,
                                  int (*verify_cb)(int, X509_STORE_CTX *)) {
  ctx->verify_cb = verify_cb;
}

int X509_STORE_CTX_set_default(X509_STORE_CTX *ctx, const char *name) {
  const X509_VERIFY_PARAM *param = X509_VERIFY_PARAM_lookup(name);
  if (!param) {
    return 0;
  }
  return X509_VERIFY_PARAM_inherit(ctx->param, param);
}

X509_VERIFY_PARAM *X509_STORE_CTX_get0_param(X509_STORE_CTX *ctx) {
  return ctx->param;
}

void X509_STORE_CTX_set0_param(X509_STORE_CTX *ctx, X509_VERIFY_PARAM *param) {
  if (ctx->param) {
    X509_VERIFY_PARAM_free(ctx->param);
  }
  ctx->param = param;
}

int X509_STORE_CTX_add_custom_crit_oid(X509_STORE_CTX *ctx, ASN1_OBJECT *oid) {
  GUARD_PTR(ctx);
  GUARD_PTR(oid);

  ASN1_OBJECT *oid_dup = OBJ_dup(oid);
  if (oid_dup == NULL) {
    return 0;
  }
  if (ctx->custom_crit_oids == NULL) {
    ctx->custom_crit_oids = sk_ASN1_OBJECT_new_null();
    if (ctx->custom_crit_oids == NULL) {
      ASN1_OBJECT_free(oid_dup);
      return 0;
    }
  }

  if (!sk_ASN1_OBJECT_push(ctx->custom_crit_oids, oid_dup)) {
    ASN1_OBJECT_free(oid_dup);
    return 0;
  }
  return 1;
}

void X509_STORE_CTX_set_verify_crit_oids(
    X509_STORE_CTX *ctx,
    X509_STORE_CTX_verify_crit_oids_cb verify_custom_crit_oids) {
  ctx->verify_custom_crit_oids = verify_custom_crit_oids;
}
