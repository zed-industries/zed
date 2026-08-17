// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/obj.h>

#ifndef __STDC_FORMAT_MACROS
#define __STDC_FORMAT_MACROS
#endif
#include <inttypes.h>

#include <limits.h>
#include <string.h>

#include <openssl/asn1.h>
#include <openssl/bytestring.h>
#include <openssl/err.h>
#include <openssl/lhash.h>
#include <openssl/mem.h>
#include <openssl/thread.h>

#include "../asn1/internal.h"
#include "../internal.h"
#include "../lhash/internal.h"

// obj_data.h must be included after the definition of |ASN1_OBJECT|.
#include "obj_dat.h"


DEFINE_LHASH_OF(ASN1_OBJECT)

static struct CRYPTO_STATIC_MUTEX global_added_lock = CRYPTO_STATIC_MUTEX_INIT;
// These globals are protected by |global_added_lock|.
static LHASH_OF(ASN1_OBJECT) *global_added_by_data = NULL;
static LHASH_OF(ASN1_OBJECT) *global_added_by_nid = NULL;
static LHASH_OF(ASN1_OBJECT) *global_added_by_short_name = NULL;
static LHASH_OF(ASN1_OBJECT) *global_added_by_long_name = NULL;

static struct CRYPTO_STATIC_MUTEX global_next_nid_lock =
    CRYPTO_STATIC_MUTEX_INIT;
static unsigned global_next_nid = NUM_NID;

static int obj_next_nid(void) {
  int ret;

  CRYPTO_STATIC_MUTEX_lock_write(&global_next_nid_lock);
  ret = global_next_nid++;
  CRYPTO_STATIC_MUTEX_unlock_write(&global_next_nid_lock);

  return ret;
}

ASN1_OBJECT *OBJ_dup(const ASN1_OBJECT *o) {
  ASN1_OBJECT *r;
  unsigned char *data = NULL;
  char *sn = NULL, *ln = NULL;

  if (o == NULL) {
    return NULL;
  }

  if (!(o->flags & ASN1_OBJECT_FLAG_DYNAMIC)) {
    // TODO(fork): this is a little dangerous.
    return (ASN1_OBJECT *)o;
  }

  r = ASN1_OBJECT_new();
  if (r == NULL) {
    OPENSSL_PUT_ERROR(OBJ, ERR_R_ASN1_LIB);
    return NULL;
  }
  r->ln = r->sn = NULL;

  // once data is attached to an object, it remains const
  r->data = OPENSSL_memdup(o->data, o->length);
  if (o->length != 0 && r->data == NULL) {
    goto err;
  }

  r->length = o->length;
  r->nid = o->nid;

  if (o->ln != NULL) {
    ln = OPENSSL_strdup(o->ln);
    if (ln == NULL) {
      goto err;
    }
  }

  if (o->sn != NULL) {
    sn = OPENSSL_strdup(o->sn);
    if (sn == NULL) {
      goto err;
    }
  }

  r->sn = sn;
  r->ln = ln;

  r->flags =
      o->flags | (ASN1_OBJECT_FLAG_DYNAMIC | ASN1_OBJECT_FLAG_DYNAMIC_STRINGS |
                  ASN1_OBJECT_FLAG_DYNAMIC_DATA);
  return r;

err:
  OPENSSL_free(ln);
  OPENSSL_free(sn);
  OPENSSL_free(data);
  OPENSSL_free(r);
  return NULL;
}

int OBJ_cmp(const ASN1_OBJECT *a, const ASN1_OBJECT *b) {
  if (a->length < b->length) {
    return -1;
  } else if (a->length > b->length) {
    return 1;
  }
  return OPENSSL_memcmp(a->data, b->data, a->length);
}

const uint8_t *OBJ_get0_data(const ASN1_OBJECT *obj) {
  if (obj == NULL) {
    return NULL;
  }

  return obj->data;
}

size_t OBJ_length(const ASN1_OBJECT *obj) {
  if (obj == NULL || obj->length < 0) {
    return 0;
  }

  return (size_t)obj->length;
}

static const ASN1_OBJECT *get_builtin_object(int nid) {
  // |NID_undef| is stored separately, so all the indices are off by one. The
  // caller of this function must have a valid built-in, non-undef NID.
  BSSL_CHECK(nid > 0 && nid < NUM_NID);
  return &kObjects[nid - 1];
}

// obj_cmp is called to search the kNIDsInOIDOrder array. The |key| argument is
// an |ASN1_OBJECT|* that we're looking for and |element| is a pointer to an
// unsigned int in the array.
static int obj_cmp(const void *key, const void *element) {
  uint16_t nid = *((const uint16_t *)element);
  return OBJ_cmp(key, get_builtin_object(nid));
}

int OBJ_obj2nid(const ASN1_OBJECT *obj) {
  if (obj == NULL) {
    return NID_undef;
  }

  if (obj->nid != 0) {
    return obj->nid;
  }

  CRYPTO_STATIC_MUTEX_lock_read(&global_added_lock);
  if (global_added_by_data != NULL) {
    ASN1_OBJECT *match;

    match = lh_ASN1_OBJECT_retrieve(global_added_by_data, obj);
    if (match != NULL) {
      CRYPTO_STATIC_MUTEX_unlock_read(&global_added_lock);
      return match->nid;
    }
  }
  CRYPTO_STATIC_MUTEX_unlock_read(&global_added_lock);

  const uint16_t *nid_ptr =
      bsearch(obj, kNIDsInOIDOrder, OPENSSL_ARRAY_SIZE(kNIDsInOIDOrder),
              sizeof(kNIDsInOIDOrder[0]), obj_cmp);
  if (nid_ptr == NULL) {
    return NID_undef;
  }

  return get_builtin_object(*nid_ptr)->nid;
}

int OBJ_cbs2nid(const CBS *cbs) {
  if (CBS_len(cbs) > INT_MAX) {
    return NID_undef;
  }

  ASN1_OBJECT obj;
  OPENSSL_memset(&obj, 0, sizeof(obj));
  obj.data = CBS_data(cbs);
  obj.length = (int)CBS_len(cbs);

  return OBJ_obj2nid(&obj);
}

// short_name_cmp is called to search the kNIDsInShortNameOrder array. The
// |key| argument is name that we're looking for and |element| is a pointer to
// an unsigned int in the array.
static int short_name_cmp(const void *key, const void *element) {
  const char *name = (const char *)key;
  uint16_t nid = *((const uint16_t *)element);

  return strcmp(name, get_builtin_object(nid)->sn);
}

int OBJ_sn2nid(const char *short_name) {
  CRYPTO_STATIC_MUTEX_lock_read(&global_added_lock);
  if (global_added_by_short_name != NULL) {
    ASN1_OBJECT *match, template;

    template.sn = short_name;
    match = lh_ASN1_OBJECT_retrieve(global_added_by_short_name, &template);
    if (match != NULL) {
      CRYPTO_STATIC_MUTEX_unlock_read(&global_added_lock);
      return match->nid;
    }
  }
  CRYPTO_STATIC_MUTEX_unlock_read(&global_added_lock);

  const uint16_t *nid_ptr =
      bsearch(short_name, kNIDsInShortNameOrder,
              OPENSSL_ARRAY_SIZE(kNIDsInShortNameOrder),
              sizeof(kNIDsInShortNameOrder[0]), short_name_cmp);
  if (nid_ptr == NULL) {
    return NID_undef;
  }

  return get_builtin_object(*nid_ptr)->nid;
}

// long_name_cmp is called to search the kNIDsInLongNameOrder array. The
// |key| argument is name that we're looking for and |element| is a pointer to
// an unsigned int in the array.
static int long_name_cmp(const void *key, const void *element) {
  const char *name = (const char *)key;
  uint16_t nid = *((const uint16_t *)element);

  return strcmp(name, get_builtin_object(nid)->ln);
}

int OBJ_ln2nid(const char *long_name) {
  CRYPTO_STATIC_MUTEX_lock_read(&global_added_lock);
  if (global_added_by_long_name != NULL) {
    ASN1_OBJECT *match, template;

    template.ln = long_name;
    match = lh_ASN1_OBJECT_retrieve(global_added_by_long_name, &template);
    if (match != NULL) {
      CRYPTO_STATIC_MUTEX_unlock_read(&global_added_lock);
      return match->nid;
    }
  }
  CRYPTO_STATIC_MUTEX_unlock_read(&global_added_lock);

  const uint16_t *nid_ptr = bsearch(
      long_name, kNIDsInLongNameOrder, OPENSSL_ARRAY_SIZE(kNIDsInLongNameOrder),
      sizeof(kNIDsInLongNameOrder[0]), long_name_cmp);
  if (nid_ptr == NULL) {
    return NID_undef;
  }

  return get_builtin_object(*nid_ptr)->nid;
}

int OBJ_txt2nid(const char *s) {
  ASN1_OBJECT *obj;
  int nid;

  obj = OBJ_txt2obj(s, 0 /* search names */);
  nid = OBJ_obj2nid(obj);
  ASN1_OBJECT_free(obj);
  return nid;
}

OPENSSL_EXPORT int OBJ_nid2cbb(CBB *out, int nid) {
  const ASN1_OBJECT *obj = OBJ_nid2obj(nid);
  CBB oid;

  if (obj == NULL ||
      !CBB_add_asn1(out, &oid, CBS_ASN1_OBJECT) ||
      !CBB_add_bytes(&oid, obj->data, obj->length) ||
      !CBB_flush(out)) {
    return 0;
  }

  return 1;
}

const ASN1_OBJECT *OBJ_get_undef(void) {
  static const ASN1_OBJECT kUndef = {
      /*sn=*/SN_undef,
      /*ln=*/LN_undef,
      /*nid=*/NID_undef,
      /*length=*/0,
      /*data=*/NULL,
      /*flags=*/0,
  };
  return &kUndef;
}

ASN1_OBJECT *OBJ_nid2obj(int nid) {
  if (nid == NID_undef) {
    return (ASN1_OBJECT *)OBJ_get_undef();
  }

  if (nid > 0 && nid < NUM_NID) {
    const ASN1_OBJECT *obj = get_builtin_object(nid);
    if (nid != NID_undef && obj->nid == NID_undef) {
      goto err;
    }
    return (ASN1_OBJECT *)obj;
  }

  CRYPTO_STATIC_MUTEX_lock_read(&global_added_lock);
  if (global_added_by_nid != NULL) {
    ASN1_OBJECT *match, template;

    template.nid = nid;
    match = lh_ASN1_OBJECT_retrieve(global_added_by_nid, &template);
    if (match != NULL) {
      CRYPTO_STATIC_MUTEX_unlock_read(&global_added_lock);
      return match;
    }
  }
  CRYPTO_STATIC_MUTEX_unlock_read(&global_added_lock);

err:
  OPENSSL_PUT_ERROR(OBJ, OBJ_R_UNKNOWN_NID);
  return NULL;
}

const char *OBJ_nid2sn(int nid) {
  const ASN1_OBJECT *obj = OBJ_nid2obj(nid);
  if (obj == NULL) {
    return NULL;
  }

  return obj->sn;
}

const char *OBJ_nid2ln(int nid) {
  const ASN1_OBJECT *obj = OBJ_nid2obj(nid);
  if (obj == NULL) {
    return NULL;
  }

  return obj->ln;
}

static ASN1_OBJECT *create_object_with_text_oid(int (*get_nid)(void),
                                                const char *oid,
                                                const char *short_name,
                                                const char *long_name) {
  uint8_t *buf;
  size_t len;
  CBB cbb;
  if (!CBB_init(&cbb, 32) ||
      !CBB_add_asn1_oid_from_text(&cbb, oid, strlen(oid)) ||
      !CBB_finish(&cbb, &buf, &len)) {
    OPENSSL_PUT_ERROR(OBJ, OBJ_R_INVALID_OID_STRING);
    CBB_cleanup(&cbb);
    return NULL;
  }

  ASN1_OBJECT *ret = ASN1_OBJECT_create(get_nid ? get_nid() : NID_undef, buf,
                                        len, short_name, long_name);
  OPENSSL_free(buf);
  return ret;
}

ASN1_OBJECT *OBJ_txt2obj(const char *s, int dont_search_names) {
  if (!dont_search_names) {
    int nid = OBJ_sn2nid(s);
    if (nid == NID_undef) {
      nid = OBJ_ln2nid(s);
    }

    if (nid != NID_undef) {
      return OBJ_nid2obj(nid);
    }
  }

  return create_object_with_text_oid(NULL, s, NULL, NULL);
}

static int strlcpy_int(char *dst, const char *src, int dst_size) {
  size_t ret = OPENSSL_strlcpy(dst, src, dst_size < 0 ? 0 : (size_t)dst_size);
  if (ret > INT_MAX) {
    OPENSSL_PUT_ERROR(OBJ, ERR_R_OVERFLOW);
    return -1;
  }
  return (int)ret;
}

int OBJ_obj2txt(char *out, int out_len, const ASN1_OBJECT *obj,
                int always_return_oid) {
  // Python depends on the empty OID successfully encoding as the empty
  // string.
  if (obj == NULL || obj->length == 0) {
    return strlcpy_int(out, "", out_len);
  }

  if (!always_return_oid) {
    int nid = OBJ_obj2nid(obj);
    if (nid != NID_undef) {
      const char *name = OBJ_nid2ln(nid);
      if (name == NULL) {
        name = OBJ_nid2sn(nid);
      }
      if (name != NULL) {
        return strlcpy_int(out, name, out_len);
      }
    }
  }

  CBS cbs;
  CBS_init(&cbs, obj->data, obj->length);
  char *txt = CBS_asn1_oid_to_text(&cbs);
  if (txt == NULL) {
    if (out_len > 0) {
      out[0] = '\0';
    }
    return -1;
  }

  int ret = strlcpy_int(out, txt, out_len);
  OPENSSL_free(txt);
  return ret;
}

static uint32_t hash_nid(const ASN1_OBJECT *obj) {
  return obj->nid;
}

static int cmp_nid(const ASN1_OBJECT *a, const ASN1_OBJECT *b) {
  return a->nid - b->nid;
}

static uint32_t hash_data(const ASN1_OBJECT *obj) {
  return OPENSSL_hash32(obj->data, obj->length);
}

static uint32_t hash_short_name(const ASN1_OBJECT *obj) {
  return OPENSSL_strhash(obj->sn);
}

static int cmp_short_name(const ASN1_OBJECT *a, const ASN1_OBJECT *b) {
  return strcmp(a->sn, b->sn);
}

static uint32_t hash_long_name(const ASN1_OBJECT *obj) {
  return OPENSSL_strhash(obj->ln);
}

static int cmp_long_name(const ASN1_OBJECT *a, const ASN1_OBJECT *b) {
  return strcmp(a->ln, b->ln);
}

// obj_add_object inserts |obj| into the various global hashes for run-time
// added objects. It returns one on success or zero otherwise.
static int obj_add_object(ASN1_OBJECT *obj) {
  obj->flags &= ~(ASN1_OBJECT_FLAG_DYNAMIC | ASN1_OBJECT_FLAG_DYNAMIC_STRINGS |
                  ASN1_OBJECT_FLAG_DYNAMIC_DATA);

  CRYPTO_STATIC_MUTEX_lock_write(&global_added_lock);
  if (global_added_by_nid == NULL) {
    global_added_by_nid = lh_ASN1_OBJECT_new(hash_nid, cmp_nid);
  }
  if (global_added_by_data == NULL) {
    global_added_by_data = lh_ASN1_OBJECT_new(hash_data, OBJ_cmp);
  }
  if (global_added_by_short_name == NULL) {
    global_added_by_short_name =
        lh_ASN1_OBJECT_new(hash_short_name, cmp_short_name);
  }
  if (global_added_by_long_name == NULL) {
    global_added_by_long_name = lh_ASN1_OBJECT_new(hash_long_name, cmp_long_name);
  }

  int ok = 0;
  if (global_added_by_nid == NULL ||
      global_added_by_data == NULL ||
      global_added_by_short_name == NULL ||
      global_added_by_long_name == NULL) {
    goto err;
  }

  // We don't pay attention to |old_object| (which contains any previous object
  // that was evicted from the hashes) because we don't have a reference count
  // on ASN1_OBJECT values. Also, we should never have duplicates nids and so
  // should always have objects in |global_added_by_nid|.
  ASN1_OBJECT *old_object;
  ok = lh_ASN1_OBJECT_insert(global_added_by_nid, &old_object, obj);
  if (obj->length != 0 && obj->data != NULL) {
    ok &= lh_ASN1_OBJECT_insert(global_added_by_data, &old_object, obj);
  }
  if (obj->sn != NULL) {
    ok &= lh_ASN1_OBJECT_insert(global_added_by_short_name, &old_object, obj);
  }
  if (obj->ln != NULL) {
    ok &= lh_ASN1_OBJECT_insert(global_added_by_long_name, &old_object, obj);
  }

err:
  CRYPTO_STATIC_MUTEX_unlock_write(&global_added_lock);
  return ok;
}

int OBJ_create(const char *oid, const char *short_name, const char *long_name) {
  ASN1_OBJECT *op =
      create_object_with_text_oid(obj_next_nid, oid, short_name, long_name);
  if (op == NULL ||
      !obj_add_object(op)) {
    return NID_undef;
  }
  return op->nid;
}

void OBJ_cleanup(void) {}
