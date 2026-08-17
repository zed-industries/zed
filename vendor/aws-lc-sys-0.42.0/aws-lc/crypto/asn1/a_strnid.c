// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/asn1.h>

#include <assert.h>
#include <stdlib.h>
#include <string.h>

#include <openssl/err.h>
#include <openssl/mem.h>
#include <openssl/obj.h>

#include "../internal.h"
#include "../lhash/internal.h"
#include "internal.h"


DEFINE_LHASH_OF(ASN1_STRING_TABLE)

static LHASH_OF(ASN1_STRING_TABLE) *string_tables = NULL;
static struct CRYPTO_STATIC_MUTEX string_tables_lock = CRYPTO_STATIC_MUTEX_INIT;

void ASN1_STRING_set_default_mask(unsigned long mask) {}

unsigned long ASN1_STRING_get_default_mask(void) { return B_ASN1_UTF8STRING; }

int ASN1_STRING_set_default_mask_asc(const char *p) { return 1; }

static const ASN1_STRING_TABLE *asn1_string_table_get(int nid);

// The following function generates an ASN1_STRING based on limits in a
// table. Frequently the types and length of an ASN1_STRING are restricted by
// a corresponding OID. For example certificates and certificate requests.

ASN1_STRING *ASN1_STRING_set_by_NID(ASN1_STRING **out, const unsigned char *in,
                                    ossl_ssize_t len, int inform, int nid) {
  ASN1_STRING *str = NULL;
  int ret;
  if (!out) {
    out = &str;
  }
  const ASN1_STRING_TABLE *tbl = asn1_string_table_get(nid);
  if (tbl != NULL) {
    unsigned long mask = tbl->mask;
    if (!(tbl->flags & STABLE_NO_MASK)) {
      mask &= B_ASN1_UTF8STRING;
    }
    ret = ASN1_mbstring_ncopy(out, in, len, inform, mask, tbl->minsize,
                              tbl->maxsize);
  } else {
    ret = ASN1_mbstring_copy(out, in, len, inform, B_ASN1_UTF8STRING);
  }
  if (ret <= 0) {
    return NULL;
  }
  return *out;
}

// Now the tables and helper functions for the string table:

// See RFC 5280.
#define ub_name 32768
#define ub_common_name 64
#define ub_locality_name 128
#define ub_state_name 128
#define ub_organization_name 64
#define ub_organization_unit_name 64
#define ub_email_address 128
#define ub_serial_number 64

// This table must be kept in NID order

static const ASN1_STRING_TABLE tbl_standard[] = {
    {NID_commonName, 1, ub_common_name, DIRSTRING_TYPE, 0},
    {NID_countryName, 2, 2, B_ASN1_PRINTABLESTRING, STABLE_NO_MASK},
    {NID_localityName, 1, ub_locality_name, DIRSTRING_TYPE, 0},
    {NID_stateOrProvinceName, 1, ub_state_name, DIRSTRING_TYPE, 0},
    {NID_organizationName, 1, ub_organization_name, DIRSTRING_TYPE, 0},
    {NID_organizationalUnitName, 1, ub_organization_unit_name, DIRSTRING_TYPE,
     0},
    {NID_pkcs9_emailAddress, 1, ub_email_address, B_ASN1_IA5STRING,
     STABLE_NO_MASK},
    {NID_pkcs9_unstructuredName, 1, -1, PKCS9STRING_TYPE, 0},
    {NID_pkcs9_challengePassword, 1, -1, PKCS9STRING_TYPE, 0},
    {NID_pkcs9_unstructuredAddress, 1, -1, DIRSTRING_TYPE, 0},
    {NID_givenName, 1, ub_name, DIRSTRING_TYPE, 0},
    {NID_surname, 1, ub_name, DIRSTRING_TYPE, 0},
    {NID_initials, 1, ub_name, DIRSTRING_TYPE, 0},
    {NID_serialNumber, 1, ub_serial_number, B_ASN1_PRINTABLESTRING,
     STABLE_NO_MASK},
    {NID_friendlyName, -1, -1, B_ASN1_BMPSTRING, STABLE_NO_MASK},
    {NID_name, 1, ub_name, DIRSTRING_TYPE, 0},
    {NID_dnQualifier, -1, -1, B_ASN1_PRINTABLESTRING, STABLE_NO_MASK},
    {NID_domainComponent, 1, -1, B_ASN1_IA5STRING, STABLE_NO_MASK},
    {NID_ms_csp_name, -1, -1, B_ASN1_BMPSTRING, STABLE_NO_MASK}};

static int table_cmp(const ASN1_STRING_TABLE *a, const ASN1_STRING_TABLE *b) {
  if (a->nid < b->nid) {
    return -1;
  }
  if (a->nid > b->nid) {
    return 1;
  }
  return 0;
}

static int table_cmp_void(const void *a, const void *b) {
  return table_cmp(a, b);
}

static uint32_t table_hash(const ASN1_STRING_TABLE *tbl) {
  return OPENSSL_hash32(&tbl->nid, sizeof(tbl->nid));
}

static const ASN1_STRING_TABLE *asn1_string_table_get(int nid) {
  ASN1_STRING_TABLE key;
  key.nid = nid;
  const ASN1_STRING_TABLE *tbl =
      bsearch(&key, tbl_standard, OPENSSL_ARRAY_SIZE(tbl_standard),
              sizeof(ASN1_STRING_TABLE), table_cmp_void);
  if (tbl != NULL) {
    return tbl;
  }

  CRYPTO_STATIC_MUTEX_lock_read(&string_tables_lock);
  if (string_tables != NULL) {
    tbl = lh_ASN1_STRING_TABLE_retrieve(string_tables, &key);
  }
  CRYPTO_STATIC_MUTEX_unlock_read(&string_tables_lock);
  // Note returning |tbl| without the lock is only safe because
  // |ASN1_STRING_TABLE_add| cannot modify or delete existing entries. If we
  // wish to support that, this function must copy the result under a lock.
  return tbl;
}

int ASN1_STRING_TABLE_add(int nid, long minsize, long maxsize,
                          unsigned long mask, unsigned long flags) {
  // Existing entries cannot be overwritten.
  if (asn1_string_table_get(nid) != NULL) {
    OPENSSL_PUT_ERROR(ASN1, ERR_R_SHOULD_NOT_HAVE_BEEN_CALLED);
    return 0;
  }

  int ret = 0;
  CRYPTO_STATIC_MUTEX_lock_write(&string_tables_lock);

  if (string_tables == NULL) {
    string_tables = lh_ASN1_STRING_TABLE_new(table_hash, table_cmp);
    if (string_tables == NULL) {
      goto err;
    }
  } else {
    // Check again for an existing entry. One may have been added while
    // unlocked.
    ASN1_STRING_TABLE key;
    key.nid = nid;
    if (lh_ASN1_STRING_TABLE_retrieve(string_tables, &key) != NULL) {
      OPENSSL_PUT_ERROR(ASN1, ERR_R_SHOULD_NOT_HAVE_BEEN_CALLED);
      goto err;
    }
  }

  ASN1_STRING_TABLE *tbl = OPENSSL_malloc(sizeof(ASN1_STRING_TABLE));
  if (tbl == NULL) {
    goto err;
  }
  tbl->nid = nid;
  tbl->flags = flags;
  tbl->minsize = minsize;
  tbl->maxsize = maxsize;
  tbl->mask = mask;
  ASN1_STRING_TABLE *old_tbl;
  if (!lh_ASN1_STRING_TABLE_insert(string_tables, &old_tbl, tbl)) {
    OPENSSL_free(tbl);
    goto err;
  }
  assert(old_tbl == NULL);
  ret = 1;

err:
  CRYPTO_STATIC_MUTEX_unlock_write(&string_tables_lock);
  return ret;
}

void ASN1_STRING_TABLE_cleanup(void) {}

void asn1_get_string_table_for_testing(const ASN1_STRING_TABLE **out_ptr,
                                       size_t *out_len) {
  *out_ptr = tbl_standard;
  *out_len = OPENSSL_ARRAY_SIZE(tbl_standard);
}
