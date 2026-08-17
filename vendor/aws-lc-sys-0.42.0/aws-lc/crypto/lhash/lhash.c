// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/lhash.h>

#include <assert.h>
#include <limits.h>
#include <string.h>

#include <openssl/mem.h>

#include "internal.h"
#include "../internal.h"


// kMinNumBuckets is the minimum size of the buckets array in an |_LHASH|.
static const size_t kMinNumBuckets = 16;

// kMaxAverageChainLength contains the maximum, average chain length. When the
// average chain length exceeds this value, the hash table will be resized.
static const size_t kMaxAverageChainLength = 2;
static const size_t kMinAverageChainLength = 1;

// lhash_item_st is an element of a hash chain. It points to the opaque data
// for this element and to the next item in the chain. The linked-list is NULL
// terminated.
typedef struct lhash_item_st {
  void *data;
  struct lhash_item_st *next;
  // hash contains the cached, hash value of |data|.
  uint32_t hash;
} LHASH_ITEM;

struct lhash_st {
  // num_items contains the total number of items in the hash table.
  size_t num_items;
  // buckets is an array of |num_buckets| pointers. Each points to the head of
  // a chain of LHASH_ITEM objects that have the same hash value, mod
  // |num_buckets|.
  LHASH_ITEM **buckets;
  // num_buckets contains the length of |buckets|. This value is always >=
  // kMinNumBuckets.
  size_t num_buckets;
  // callback_depth contains the current depth of |lh_doall| or |lh_doall_arg|
  // calls. If non-zero then this suppresses resizing of the |buckets| array,
  // which would otherwise disrupt the iteration.
  unsigned callback_depth;

  lhash_cmp_func comp;
  lhash_hash_func hash;
};

_LHASH *OPENSSL_lh_new(lhash_hash_func hash, lhash_cmp_func comp) {
  _LHASH *ret = OPENSSL_zalloc(sizeof(_LHASH));
  if (ret == NULL) {
    return NULL;
  }

  ret->num_buckets = kMinNumBuckets;
  ret->buckets = OPENSSL_calloc(ret->num_buckets, sizeof(LHASH_ITEM *));
  if (ret->buckets == NULL) {
    OPENSSL_free(ret);
    return NULL;
  }

  ret->comp = comp;
  ret->hash = hash;
  return ret;
}

void OPENSSL_lh_free(_LHASH *lh) {
  if (lh == NULL) {
    return;
  }

  for (size_t i = 0; i < lh->num_buckets; i++) {
    LHASH_ITEM *next;
    for (LHASH_ITEM *n = lh->buckets[i]; n != NULL; n = next) {
      next = n->next;
      OPENSSL_free(n);
    }
  }

  OPENSSL_free(lh->buckets);
  OPENSSL_free(lh);
}

size_t OPENSSL_lh_num_items(const _LHASH *lh) { return lh->num_items; }

// get_next_ptr_and_hash returns a pointer to the pointer that points to the
// item equal to |data|. In other words, it searches for an item equal to |data|
// and, if it's at the start of a chain, then it returns a pointer to an
// element of |lh->buckets|, otherwise it returns a pointer to the |next|
// element of the previous item in the chain. If an element equal to |data| is
// not found, it returns a pointer that points to a NULL pointer. If |out_hash|
// is not NULL, then it also puts the hash value of |data| in |*out_hash|.
static LHASH_ITEM **get_next_ptr_and_hash(const _LHASH *lh, uint32_t *out_hash,
                                          const void *data,
                                          lhash_hash_func_helper call_hash_func,
                                          lhash_cmp_func_helper call_cmp_func) {
  const uint32_t hash = call_hash_func(lh->hash, data);
  if (out_hash != NULL) {
    *out_hash = hash;
  }

  LHASH_ITEM **ret = &lh->buckets[hash % lh->num_buckets];
  for (LHASH_ITEM *cur = *ret; cur != NULL; cur = *ret) {
    if (call_cmp_func(lh->comp, cur->data, data) == 0) {
      break;
    }
    ret = &cur->next;
  }

  return ret;
}

// get_next_ptr_by_key behaves like |get_next_ptr_and_hash| but takes a key
// which may be a different type from the values stored in |lh|.
static LHASH_ITEM **get_next_ptr_by_key(const _LHASH *lh, const void *key,
                                        uint32_t key_hash,
                                        int (*cmp_key)(const void *key,
                                                       const void *value)) {
  LHASH_ITEM **ret = &lh->buckets[key_hash % lh->num_buckets];
  for (LHASH_ITEM *cur = *ret; cur != NULL; cur = *ret) {
    if (cmp_key(key, cur->data) == 0) {
      break;
    }
    ret = &cur->next;
  }

  return ret;
}

void *OPENSSL_lh_retrieve(const _LHASH *lh, const void *data,
                          lhash_hash_func_helper call_hash_func,
                          lhash_cmp_func_helper call_cmp_func) {
  LHASH_ITEM **next_ptr =
      get_next_ptr_and_hash(lh, NULL, data, call_hash_func, call_cmp_func);
  return *next_ptr == NULL ? NULL : (*next_ptr)->data;
}

void *OPENSSL_lh_retrieve_key(const _LHASH *lh, const void *key,
                              uint32_t key_hash,
                              int (*cmp_key)(const void *key,
                                             const void *value)) {
  LHASH_ITEM **next_ptr = get_next_ptr_by_key(lh, key, key_hash, cmp_key);
  return *next_ptr == NULL ? NULL : (*next_ptr)->data;
}

// lh_rebucket allocates a new array of |new_num_buckets| pointers and
// redistributes the existing items into it before making it |lh->buckets| and
// freeing the old array.
static void lh_rebucket(_LHASH *lh, const size_t new_num_buckets) {
  LHASH_ITEM **new_buckets, *cur, *next;
  size_t i, alloc_size;

  alloc_size = sizeof(LHASH_ITEM *) * new_num_buckets;
  if (alloc_size / sizeof(LHASH_ITEM*) != new_num_buckets) {
    return;
  }

  new_buckets = OPENSSL_zalloc(alloc_size);
  if (new_buckets == NULL) {
    return;
  }

  for (i = 0; i < lh->num_buckets; i++) {
    for (cur = lh->buckets[i]; cur != NULL; cur = next) {
      const size_t new_bucket = cur->hash % new_num_buckets;
      next = cur->next;
      cur->next = new_buckets[new_bucket];
      new_buckets[new_bucket] = cur;
    }
  }

  OPENSSL_free(lh->buckets);

  lh->num_buckets = new_num_buckets;
  lh->buckets = new_buckets;
}

// lh_maybe_resize resizes the |buckets| array if needed.
static void lh_maybe_resize(_LHASH *lh) {
  size_t avg_chain_length;

  if (lh->callback_depth > 0) {
    // Don't resize the hash if we are currently iterating over it.
    return;
  }

  assert(lh->num_buckets >= kMinNumBuckets);
  avg_chain_length = lh->num_items / lh->num_buckets;

  if (avg_chain_length > kMaxAverageChainLength) {
    const size_t new_num_buckets = lh->num_buckets * 2;

    if (new_num_buckets > lh->num_buckets) {
      lh_rebucket(lh, new_num_buckets);
    }
  } else if (avg_chain_length < kMinAverageChainLength &&
             lh->num_buckets > kMinNumBuckets) {
    size_t new_num_buckets = lh->num_buckets / 2;

    if (new_num_buckets < kMinNumBuckets) {
      new_num_buckets = kMinNumBuckets;
    }

    lh_rebucket(lh, new_num_buckets);
  }
}

int OPENSSL_lh_insert(_LHASH *lh, void **old_data, void *data,
                      lhash_hash_func_helper call_hash_func,
                      lhash_cmp_func_helper call_cmp_func) {
  uint32_t hash;
  LHASH_ITEM **next_ptr, *item;

  *old_data = NULL;
  next_ptr =
      get_next_ptr_and_hash(lh, &hash, data, call_hash_func, call_cmp_func);


  if (*next_ptr != NULL) {
    // An element equal to |data| already exists in the hash table. It will be
    // replaced.
    *old_data = (*next_ptr)->data;
    (*next_ptr)->data = data;
    return 1;
  }

  // An element equal to |data| doesn't exist in the hash table yet.
  item = OPENSSL_zalloc(sizeof(LHASH_ITEM));
  if (item == NULL) {
    return 0;
  }

  item->data = data;
  item->hash = hash;
  *next_ptr = item;
  lh->num_items++;
  lh_maybe_resize(lh);

  return 1;
}

void *OPENSSL_lh_delete(_LHASH *lh, const void *data,
                        lhash_hash_func_helper call_hash_func,
                        lhash_cmp_func_helper call_cmp_func) {
  LHASH_ITEM **next_ptr, *item, *ret;

  next_ptr =
      get_next_ptr_and_hash(lh, NULL, data, call_hash_func, call_cmp_func);

  if (*next_ptr == NULL) {
    // No such element.
    return NULL;
  }

  item = *next_ptr;
  *next_ptr = item->next;
  ret = item->data;
  OPENSSL_free(item);

  lh->num_items--;
  lh_maybe_resize(lh);

  return ret;
}

void OPENSSL_lh_doall_arg(_LHASH *lh, void (*func)(void *, void *), void *arg) {
  if (lh == NULL) {
    return;
  }

  if (lh->callback_depth < UINT_MAX) {
    // |callback_depth| is a saturating counter.
    lh->callback_depth++;
  }

  for (size_t i = 0; i < lh->num_buckets; i++) {
    LHASH_ITEM *next;
    for (LHASH_ITEM *cur = lh->buckets[i]; cur != NULL; cur = next) {
      next = cur->next;
      func(cur->data, arg);
    }
  }

  if (lh->callback_depth < UINT_MAX) {
    lh->callback_depth--;
  }

  // The callback may have added or removed elements and the non-zero value of
  // |callback_depth| will have suppressed any resizing. Thus any needed
  // resizing is done here.
  lh_maybe_resize(lh);
}

void lh_doall_arg(_LHASH *lh, void (*func)(void *, void *), void *arg) {
  OPENSSL_lh_doall_arg(lh, func, arg);
}
