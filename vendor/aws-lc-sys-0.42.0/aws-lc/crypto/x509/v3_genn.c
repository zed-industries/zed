// Written by Dr Stephen N Henson (steve@openssl.org) for the OpenSSL project 1999.
// Copyright (c) 1999-2008 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <stdio.h>

#include <openssl/asn1t.h>
#include <openssl/conf.h>
#include <openssl/obj.h>
#include <openssl/x509.h>

#include "internal.h"


ASN1_SEQUENCE(OTHERNAME) = {
    ASN1_SIMPLE(OTHERNAME, type_id, ASN1_OBJECT),
    // Maybe have a true ANY DEFINED BY later
    ASN1_EXP(OTHERNAME, value, ASN1_ANY, 0),
} ASN1_SEQUENCE_END(OTHERNAME)

IMPLEMENT_ASN1_ALLOC_FUNCTIONS(OTHERNAME)

ASN1_SEQUENCE(EDIPARTYNAME) = {
    // DirectoryString is a CHOICE type, so use explicit tagging.
    ASN1_EXP_OPT(EDIPARTYNAME, nameAssigner, DIRECTORYSTRING, 0),
    ASN1_EXP(EDIPARTYNAME, partyName, DIRECTORYSTRING, 1),
} ASN1_SEQUENCE_END(EDIPARTYNAME)

IMPLEMENT_ASN1_ALLOC_FUNCTIONS(EDIPARTYNAME)

ASN1_CHOICE(GENERAL_NAME) = {
    ASN1_IMP(GENERAL_NAME, d.otherName, OTHERNAME, GEN_OTHERNAME),
    ASN1_IMP(GENERAL_NAME, d.rfc822Name, ASN1_IA5STRING, GEN_EMAIL),
    ASN1_IMP(GENERAL_NAME, d.dNSName, ASN1_IA5STRING, GEN_DNS),
    // Don't decode this
    ASN1_IMP(GENERAL_NAME, d.x400Address, ASN1_SEQUENCE, GEN_X400),
    // X509_NAME is a CHOICE type so use EXPLICIT
    ASN1_EXP(GENERAL_NAME, d.directoryName, X509_NAME, GEN_DIRNAME),
    ASN1_IMP(GENERAL_NAME, d.ediPartyName, EDIPARTYNAME, GEN_EDIPARTY),
    ASN1_IMP(GENERAL_NAME, d.uniformResourceIdentifier, ASN1_IA5STRING,
             GEN_URI),
    ASN1_IMP(GENERAL_NAME, d.iPAddress, ASN1_OCTET_STRING, GEN_IPADD),
    ASN1_IMP(GENERAL_NAME, d.registeredID, ASN1_OBJECT, GEN_RID),
} ASN1_CHOICE_END(GENERAL_NAME)

IMPLEMENT_ASN1_FUNCTIONS(GENERAL_NAME)

ASN1_ITEM_TEMPLATE(GENERAL_NAMES) = ASN1_EX_TEMPLATE_TYPE(ASN1_TFLG_SEQUENCE_OF,
                                                          0, GeneralNames,
                                                          GENERAL_NAME)
ASN1_ITEM_TEMPLATE_END(GENERAL_NAMES)

IMPLEMENT_ASN1_FUNCTIONS(GENERAL_NAMES)

IMPLEMENT_ASN1_DUP_FUNCTION(GENERAL_NAME)

static int edipartyname_cmp(const EDIPARTYNAME *a, const EDIPARTYNAME *b) {
  // nameAssigner is optional and may be NULL.
  if (a->nameAssigner == NULL) {
    if (b->nameAssigner != NULL) {
      return -1;
    }
  } else {
    if (b->nameAssigner == NULL ||
        ASN1_STRING_cmp(a->nameAssigner, b->nameAssigner) != 0) {
      return -1;
    }
  }

  // partyName may not be NULL.
  return ASN1_STRING_cmp(a->partyName, b->partyName);
}

// Returns 0 if they are equal, != 0 otherwise.
static int othername_cmp(const OTHERNAME *a, const OTHERNAME *b) {
  int result = -1;

  if (!a || !b) {
    return -1;
  }
  // Check their type first.
  if ((result = OBJ_cmp(a->type_id, b->type_id)) != 0) {
    return result;
  }
  // Check the value.
  result = ASN1_TYPE_cmp(a->value, b->value);
  return result;
}

// Returns 0 if they are equal, != 0 otherwise.
int GENERAL_NAME_cmp(const GENERAL_NAME *a, const GENERAL_NAME *b) {
  if (!a || !b || a->type != b->type) {
    return -1;
  }

  switch (a->type) {
    case GEN_X400:
      return ASN1_STRING_cmp(a->d.x400Address, b->d.x400Address);

    case GEN_EDIPARTY:
      return edipartyname_cmp(a->d.ediPartyName, b->d.ediPartyName);

    case GEN_OTHERNAME:
      return othername_cmp(a->d.otherName, b->d.otherName);

    case GEN_EMAIL:
    case GEN_DNS:
    case GEN_URI:
      return ASN1_STRING_cmp(a->d.ia5, b->d.ia5);

    case GEN_DIRNAME:
      return X509_NAME_cmp(a->d.dirn, b->d.dirn);

    case GEN_IPADD:
      return ASN1_OCTET_STRING_cmp(a->d.ip, b->d.ip);

    case GEN_RID:
      return OBJ_cmp(a->d.rid, b->d.rid);
  }

  return -1;
}

void GENERAL_NAME_set0_value(GENERAL_NAME *a, int type, void *value) {
  switch (type) {
    case GEN_X400:
      a->d.x400Address = value;
      break;

    case GEN_EDIPARTY:
      a->d.ediPartyName = value;
      break;

    case GEN_OTHERNAME:
      a->d.otherName = value;
      break;

    case GEN_EMAIL:
    case GEN_DNS:
    case GEN_URI:
      a->d.ia5 = value;
      break;

    case GEN_DIRNAME:
      a->d.dirn = value;
      break;

    case GEN_IPADD:
      a->d.ip = value;
      break;

    case GEN_RID:
      a->d.rid = value;
      break;
  }
  a->type = type;
}

void *GENERAL_NAME_get0_value(const GENERAL_NAME *a, int *out_type) {
  if (out_type) {
    *out_type = a->type;
  }
  switch (a->type) {
    case GEN_X400:
      return a->d.x400Address;

    case GEN_EDIPARTY:
      return a->d.ediPartyName;

    case GEN_OTHERNAME:
      return a->d.otherName;

    case GEN_EMAIL:
    case GEN_DNS:
    case GEN_URI:
      return a->d.ia5;

    case GEN_DIRNAME:
      return a->d.dirn;

    case GEN_IPADD:
      return a->d.ip;

    case GEN_RID:
      return a->d.rid;

    default:
      return NULL;
  }
}

int GENERAL_NAME_set0_othername(GENERAL_NAME *gen, ASN1_OBJECT *oid,
                                ASN1_TYPE *value) {
  OTHERNAME *oth;
  oth = OTHERNAME_new();
  if (!oth) {
    return 0;
  }
  ASN1_TYPE_free(oth->value);
  oth->type_id = oid;
  oth->value = value;
  GENERAL_NAME_set0_value(gen, GEN_OTHERNAME, oth);
  return 1;
}

int GENERAL_NAME_get0_otherName(const GENERAL_NAME *gen, ASN1_OBJECT **out_oid,
                                ASN1_TYPE **out_value) {
  if (gen->type != GEN_OTHERNAME) {
    return 0;
  }
  if (out_oid != NULL) {
    *out_oid = gen->d.otherName->type_id;
  }
  if (out_value != NULL) {
    *out_value = gen->d.otherName->value;
  }
  return 1;
}
