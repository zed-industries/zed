//  Written by Dr Stephen N Henson (steve@openssl.org) for the OpenSSL project 1999.
// Copyright (c) 1999-2004 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <stdio.h>
#include <string.h>

#include <openssl/asn1.h>
#include <openssl/asn1t.h>
#include <openssl/conf.h>
#include <openssl/err.h>
#include <openssl/mem.h>
#include <openssl/obj.h>
#include <openssl/stack.h>
#include <openssl/x509.h>

#include "internal.h"

// Certificate policies extension support: this one is a bit complex...

static int i2r_certpol(const X509V3_EXT_METHOD *method, void *ext, BIO *out,
                       int indent);
static void *r2i_certpol(const X509V3_EXT_METHOD *method, const X509V3_CTX *ctx,
                         const char *value);
static void print_qualifiers(BIO *out, const STACK_OF(POLICYQUALINFO) *quals,
                             int indent);
static void print_notice(BIO *out, const USERNOTICE *notice, int indent);
static POLICYINFO *policy_section(const X509V3_CTX *ctx,
                                  const STACK_OF(CONF_VALUE) *polstrs,
                                  int ia5org);
static POLICYQUALINFO *notice_section(const X509V3_CTX *ctx,
                                      const STACK_OF(CONF_VALUE) *unot,
                                      int ia5org);
static int nref_nos(STACK_OF(ASN1_INTEGER) *nnums,
                    const STACK_OF(CONF_VALUE) *nos);

const X509V3_EXT_METHOD v3_cpols = {
    NID_certificate_policies,
    0,
    ASN1_ITEM_ref(CERTIFICATEPOLICIES),
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    i2r_certpol,
    r2i_certpol,
    NULL,
};

DECLARE_ASN1_ITEM(POLICYINFO)
DECLARE_ASN1_ITEM(POLICYQUALINFO)
DECLARE_ASN1_ITEM(USERNOTICE)
DECLARE_ASN1_ITEM(NOTICEREF)

ASN1_ITEM_TEMPLATE(CERTIFICATEPOLICIES) = ASN1_EX_TEMPLATE_TYPE(
    ASN1_TFLG_SEQUENCE_OF, 0, CERTIFICATEPOLICIES, POLICYINFO)
ASN1_ITEM_TEMPLATE_END(CERTIFICATEPOLICIES)

IMPLEMENT_ASN1_FUNCTIONS_const(CERTIFICATEPOLICIES)

ASN1_SEQUENCE(POLICYINFO) = {
    ASN1_SIMPLE(POLICYINFO, policyid, ASN1_OBJECT),
    ASN1_SEQUENCE_OF_OPT(POLICYINFO, qualifiers, POLICYQUALINFO),
} ASN1_SEQUENCE_END(POLICYINFO)

IMPLEMENT_ASN1_ALLOC_FUNCTIONS(POLICYINFO)

ASN1_ADB_TEMPLATE(policydefault) = ASN1_SIMPLE(POLICYQUALINFO, d.other,
                                               ASN1_ANY);

ASN1_ADB(POLICYQUALINFO) = {
    ADB_ENTRY(NID_id_qt_cps,
              ASN1_SIMPLE(POLICYQUALINFO, d.cpsuri, ASN1_IA5STRING)),
    ADB_ENTRY(NID_id_qt_unotice,
              ASN1_SIMPLE(POLICYQUALINFO, d.usernotice, USERNOTICE)),
} ASN1_ADB_END(POLICYQUALINFO, 0, pqualid, 0, &policydefault_tt, NULL);

ASN1_SEQUENCE(POLICYQUALINFO) = {
    ASN1_SIMPLE(POLICYQUALINFO, pqualid, ASN1_OBJECT),
    ASN1_ADB_OBJECT(POLICYQUALINFO),
} ASN1_SEQUENCE_END(POLICYQUALINFO)

IMPLEMENT_ASN1_ALLOC_FUNCTIONS(POLICYQUALINFO)

ASN1_SEQUENCE(USERNOTICE) = {
    ASN1_OPT(USERNOTICE, noticeref, NOTICEREF),
    ASN1_OPT(USERNOTICE, exptext, DISPLAYTEXT),
} ASN1_SEQUENCE_END(USERNOTICE)

IMPLEMENT_ASN1_ALLOC_FUNCTIONS(USERNOTICE)

ASN1_SEQUENCE(NOTICEREF) = {
    ASN1_SIMPLE(NOTICEREF, organization, DISPLAYTEXT),
    ASN1_SEQUENCE_OF(NOTICEREF, noticenos, ASN1_INTEGER),
} ASN1_SEQUENCE_END(NOTICEREF)

IMPLEMENT_ASN1_ALLOC_FUNCTIONS(NOTICEREF)

static void *r2i_certpol(const X509V3_EXT_METHOD *method, const X509V3_CTX *ctx,
                         const char *value) {
  STACK_OF(POLICYINFO) *pols = sk_POLICYINFO_new_null();
  if (pols == NULL) {
    return NULL;
  }
  STACK_OF(CONF_VALUE) *vals = X509V3_parse_list(value);
  if (vals == NULL) {
    OPENSSL_PUT_ERROR(X509V3, ERR_R_X509V3_LIB);
    goto err;
  }
  int ia5org = 0;
  for (size_t i = 0; i < sk_CONF_VALUE_num(vals); i++) {
    const CONF_VALUE *cnf = sk_CONF_VALUE_value(vals, i);
    if (cnf->value || !cnf->name) {
      OPENSSL_PUT_ERROR(X509V3, X509V3_R_INVALID_POLICY_IDENTIFIER);
      X509V3_conf_err(cnf);
      goto err;
    }
    POLICYINFO *pol;
    const char *pstr = cnf->name;
    if (!strcmp(pstr, "ia5org")) {
      ia5org = 1;
      continue;
    } else if (*pstr == '@') {
      const STACK_OF(CONF_VALUE) *polsect = X509V3_get_section(ctx, pstr + 1);
      if (!polsect) {
        OPENSSL_PUT_ERROR(X509V3, X509V3_R_INVALID_SECTION);

        X509V3_conf_err(cnf);
        goto err;
      }
      pol = policy_section(ctx, polsect, ia5org);
      if (!pol) {
        goto err;
      }
    } else {
      ASN1_OBJECT *pobj = OBJ_txt2obj(cnf->name, 0);
      if (pobj == NULL) {
        OPENSSL_PUT_ERROR(X509V3, X509V3_R_INVALID_OBJECT_IDENTIFIER);
        X509V3_conf_err(cnf);
        goto err;
      }
      pol = POLICYINFO_new();
      if (pol == NULL) {
        ASN1_OBJECT_free(pobj);
        goto err;
      }
      pol->policyid = pobj;
    }
    if (!sk_POLICYINFO_push(pols, pol)) {
      POLICYINFO_free(pol);
      goto err;
    }
  }
  sk_CONF_VALUE_pop_free(vals, X509V3_conf_free);
  return pols;
err:
  sk_CONF_VALUE_pop_free(vals, X509V3_conf_free);
  sk_POLICYINFO_pop_free(pols, POLICYINFO_free);
  return NULL;
}

static POLICYINFO *policy_section(const X509V3_CTX *ctx,
                                  const STACK_OF(CONF_VALUE) *polstrs,
                                  int ia5org) {
  POLICYINFO *pol;
  POLICYQUALINFO *qual;
  if (!(pol = POLICYINFO_new())) {
    goto err;
  }
  for (size_t i = 0; i < sk_CONF_VALUE_num(polstrs); i++) {
    const CONF_VALUE *cnf = sk_CONF_VALUE_value(polstrs, i);
    if (!strcmp(cnf->name, "policyIdentifier")) {
      ASN1_OBJECT *pobj;
      if (!(pobj = OBJ_txt2obj(cnf->value, 0))) {
        OPENSSL_PUT_ERROR(X509V3, X509V3_R_INVALID_OBJECT_IDENTIFIER);
        X509V3_conf_err(cnf);
        goto err;
      }
      pol->policyid = pobj;

    } else if (x509v3_conf_name_matches(cnf->name, "CPS")) {
      if (!pol->qualifiers) {
        pol->qualifiers = sk_POLICYQUALINFO_new_null();
      }
      if (!(qual = POLICYQUALINFO_new())) {
        goto err;
      }
      if (!sk_POLICYQUALINFO_push(pol->qualifiers, qual)) {
        goto err;
      }
      qual->pqualid = OBJ_nid2obj(NID_id_qt_cps);
      if (qual->pqualid == NULL) {
        OPENSSL_PUT_ERROR(X509V3, ERR_R_INTERNAL_ERROR);
        goto err;
      }
      qual->d.cpsuri = ASN1_IA5STRING_new();
      if (qual->d.cpsuri == NULL) {
        goto err;
      }
      if (!ASN1_STRING_set(qual->d.cpsuri, cnf->value, strlen(cnf->value))) {
        goto err;
      }
    } else if (x509v3_conf_name_matches(cnf->name, "userNotice")) {
      if (*cnf->value != '@') {
        OPENSSL_PUT_ERROR(X509V3, X509V3_R_EXPECTED_A_SECTION_NAME);
        X509V3_conf_err(cnf);
        goto err;
      }
      const STACK_OF(CONF_VALUE) *unot =
          X509V3_get_section(ctx, cnf->value + 1);
      if (!unot) {
        OPENSSL_PUT_ERROR(X509V3, X509V3_R_INVALID_SECTION);
        X509V3_conf_err(cnf);
        goto err;
      }
      qual = notice_section(ctx, unot, ia5org);
      if (!qual) {
        goto err;
      }
      if (!pol->qualifiers) {
        pol->qualifiers = sk_POLICYQUALINFO_new_null();
      }
      if (!sk_POLICYQUALINFO_push(pol->qualifiers, qual)) {
        goto err;
      }
    } else {
      OPENSSL_PUT_ERROR(X509V3, X509V3_R_INVALID_OPTION);

      X509V3_conf_err(cnf);
      goto err;
    }
  }
  if (!pol->policyid) {
    OPENSSL_PUT_ERROR(X509V3, X509V3_R_NO_POLICY_IDENTIFIER);
    goto err;
  }

  return pol;

err:
  POLICYINFO_free(pol);
  return NULL;
}

static POLICYQUALINFO *notice_section(const X509V3_CTX *ctx,
                                      const STACK_OF(CONF_VALUE) *unot,
                                      int ia5org) {
  USERNOTICE *notice;
  POLICYQUALINFO *qual;
  if (!(qual = POLICYQUALINFO_new())) {
    goto err;
  }
  qual->pqualid = OBJ_nid2obj(NID_id_qt_unotice);
  if (qual->pqualid == NULL) {
    OPENSSL_PUT_ERROR(X509V3, ERR_R_INTERNAL_ERROR);
    goto err;
  }
  if (!(notice = USERNOTICE_new())) {
    goto err;
  }
  qual->d.usernotice = notice;
  for (size_t i = 0; i < sk_CONF_VALUE_num(unot); i++) {
    const CONF_VALUE *cnf = sk_CONF_VALUE_value(unot, i);
    if (!strcmp(cnf->name, "explicitText")) {
      notice->exptext = ASN1_VISIBLESTRING_new();
      if (notice->exptext == NULL) {
        goto err;
      }
      if (!ASN1_STRING_set(notice->exptext, cnf->value, strlen(cnf->value))) {
        goto err;
      }
    } else if (!strcmp(cnf->name, "organization")) {
      NOTICEREF *nref;
      if (!notice->noticeref) {
        if (!(nref = NOTICEREF_new())) {
          goto err;
        }
        notice->noticeref = nref;
      } else {
        nref = notice->noticeref;
      }
      if (ia5org) {
        nref->organization->type = V_ASN1_IA5STRING;
      } else {
        nref->organization->type = V_ASN1_VISIBLESTRING;
      }
      if (!ASN1_STRING_set(nref->organization, cnf->value,
                           strlen(cnf->value))) {
        goto err;
      }
    } else if (!strcmp(cnf->name, "noticeNumbers")) {
      NOTICEREF *nref;
      STACK_OF(CONF_VALUE) *nos;
      if (!notice->noticeref) {
        if (!(nref = NOTICEREF_new())) {
          goto err;
        }
        notice->noticeref = nref;
      } else {
        nref = notice->noticeref;
      }
      nos = X509V3_parse_list(cnf->value);
      if (!nos || !sk_CONF_VALUE_num(nos)) {
        OPENSSL_PUT_ERROR(X509V3, X509V3_R_INVALID_NUMBERS);
        X509V3_conf_err(cnf);
        goto err;
      }
      int ret = nref_nos(nref->noticenos, nos);
      sk_CONF_VALUE_pop_free(nos, X509V3_conf_free);
      if (!ret) {
        goto err;
      }
    } else {
      OPENSSL_PUT_ERROR(X509V3, X509V3_R_INVALID_OPTION);
      X509V3_conf_err(cnf);
      goto err;
    }
  }

  if (notice->noticeref &&
      (!notice->noticeref->noticenos || !notice->noticeref->organization)) {
    OPENSSL_PUT_ERROR(X509V3, X509V3_R_NEED_ORGANIZATION_AND_NUMBERS);
    goto err;
  }

  return qual;

err:
  POLICYQUALINFO_free(qual);
  return NULL;
}

static int nref_nos(STACK_OF(ASN1_INTEGER) *nnums,
                    const STACK_OF(CONF_VALUE) *nos) {
  for (size_t i = 0; i < sk_CONF_VALUE_num(nos); i++) {
    const CONF_VALUE *cnf = sk_CONF_VALUE_value(nos, i);
    ASN1_INTEGER *aint = s2i_ASN1_INTEGER(NULL, cnf->name);
    if (aint == NULL) {
      OPENSSL_PUT_ERROR(X509V3, X509V3_R_INVALID_NUMBER);
      return 0;
    }
    if (!sk_ASN1_INTEGER_push(nnums, aint)) {
      ASN1_INTEGER_free(aint);
      return 0;
    }
  }
  return 1;
}

static int i2r_certpol(const X509V3_EXT_METHOD *method, void *ext, BIO *out,
                       int indent) {
  const STACK_OF(POLICYINFO) *pol = ext;
  // First print out the policy OIDs
  for (size_t i = 0; i < sk_POLICYINFO_num(pol); i++) {
    const POLICYINFO *pinfo = sk_POLICYINFO_value(pol, i);
    BIO_printf(out, "%*sPolicy: ", indent, "");
    i2a_ASN1_OBJECT(out, pinfo->policyid);
    BIO_puts(out, "\n");
    if (pinfo->qualifiers) {
      print_qualifiers(out, pinfo->qualifiers, indent + 2);
    }
  }
  return 1;
}

static void print_qualifiers(BIO *out, const STACK_OF(POLICYQUALINFO) *quals,
                             int indent) {
  for (size_t i = 0; i < sk_POLICYQUALINFO_num(quals); i++) {
    const POLICYQUALINFO *qualinfo = sk_POLICYQUALINFO_value(quals, i);
    switch (OBJ_obj2nid(qualinfo->pqualid)) {
      case NID_id_qt_cps:
        BIO_printf(out, "%*sCPS: %.*s\n", indent, "",
                   qualinfo->d.cpsuri->length, qualinfo->d.cpsuri->data);
        break;

      case NID_id_qt_unotice:
        BIO_printf(out, "%*sUser Notice:\n", indent, "");
        print_notice(out, qualinfo->d.usernotice, indent + 2);
        break;

      default:
        BIO_printf(out, "%*sUnknown Qualifier: ", indent + 2, "");

        i2a_ASN1_OBJECT(out, qualinfo->pqualid);
        BIO_puts(out, "\n");
        break;
    }
  }
}

static void print_notice(BIO *out, const USERNOTICE *notice, int indent) {
  if (notice->noticeref) {
    NOTICEREF *ref;
    ref = notice->noticeref;
    BIO_printf(out, "%*sOrganization: %.*s\n", indent, "",
               ref->organization->length, ref->organization->data);
    BIO_printf(out, "%*sNumber%s: ", indent, "",
               sk_ASN1_INTEGER_num(ref->noticenos) > 1 ? "s" : "");
    for (size_t i = 0; i < sk_ASN1_INTEGER_num(ref->noticenos); i++) {
      ASN1_INTEGER *num;
      char *tmp;
      num = sk_ASN1_INTEGER_value(ref->noticenos, i);
      if (i) {
        BIO_puts(out, ", ");
      }
      if (num == NULL) {
        BIO_puts(out, "(null)");
      } else {
        tmp = i2s_ASN1_INTEGER(NULL, num);
        if (tmp == NULL) {
          return;
        }
        BIO_puts(out, tmp);
        OPENSSL_free(tmp);
      }
    }
    BIO_puts(out, "\n");
  }
  if (notice->exptext) {
    BIO_printf(out, "%*sExplicit Text: %.*s\n", indent, "",
               notice->exptext->length, notice->exptext->data);
  }
}
