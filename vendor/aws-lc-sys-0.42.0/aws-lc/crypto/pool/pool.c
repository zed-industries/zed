// Copyright (c) 2016, Google Inc.
// SPDX-License-Identifier: ISC

#include <openssl/pool.h>

#include <assert.h>
#include <string.h>

#include <openssl/bytestring.h>
#include <openssl/mem.h>
#include <openssl/rand.h>
#include <openssl/siphash.h>
#include <openssl/thread.h>

#include "../internal.h"
#include "internal.h"


static uint32_t CRYPTO_BUFFER_hash(const CRYPTO_BUFFER *buf) {
  return (uint32_t)SIPHASH_24(buf->pool->hash_key, buf->data, buf->len);
}

static int CRYPTO_BUFFER_cmp(const CRYPTO_BUFFER *a, const CRYPTO_BUFFER *b) {
  // Only |CRYPTO_BUFFER|s from the same pool have compatible hashes.
  assert(a->pool != NULL);
  assert(a->pool == b->pool);
  if (a->len != b->len) {
    return 1;
  }
  return OPENSSL_memcmp(a->data, b->data, a->len);
}

CRYPTO_BUFFER_POOL* CRYPTO_BUFFER_POOL_new(void) {
  CRYPTO_BUFFER_POOL *pool = OPENSSL_zalloc(sizeof(CRYPTO_BUFFER_POOL));
  if (pool == NULL) {
    return NULL;
  }

  pool->bufs = lh_CRYPTO_BUFFER_new(CRYPTO_BUFFER_hash, CRYPTO_BUFFER_cmp);
  if (pool->bufs == NULL) {
    OPENSSL_free(pool);
    return NULL;
  }

  CRYPTO_MUTEX_init(&pool->lock);
  AWSLC_ABORT_IF_NOT_ONE(RAND_bytes((uint8_t *)&pool->hash_key, sizeof(pool->hash_key)));

  return pool;
}

void CRYPTO_BUFFER_POOL_free(CRYPTO_BUFFER_POOL *pool) {
  if (pool == NULL) {
    return;
  }

#if !defined(NDEBUG)
  CRYPTO_MUTEX_lock_write(&pool->lock);
  assert(lh_CRYPTO_BUFFER_num_items(pool->bufs) == 0);
  CRYPTO_MUTEX_unlock_write(&pool->lock);
#endif

  lh_CRYPTO_BUFFER_free(pool->bufs);
  CRYPTO_MUTEX_cleanup(&pool->lock);
  OPENSSL_free(pool);
}

static void crypto_buffer_free_object(CRYPTO_BUFFER *buf) {
  if (!buf->data_is_static) {
    OPENSSL_free(buf->data);
  }
  OPENSSL_free(buf);
}

static CRYPTO_BUFFER *crypto_buffer_new(const uint8_t *data, size_t len,
                                        int data_is_static,
                                        CRYPTO_BUFFER_POOL *pool) {
  if (pool != NULL) {
    CRYPTO_BUFFER tmp;
    tmp.data = (uint8_t *) data;
    tmp.len = len;
    tmp.pool = pool;

    CRYPTO_MUTEX_lock_read(&pool->lock);
    CRYPTO_BUFFER *duplicate = lh_CRYPTO_BUFFER_retrieve(pool->bufs, &tmp);
    if (data_is_static && duplicate != NULL && !duplicate->data_is_static) {
      // If the new |CRYPTO_BUFFER| would have static data, but the duplicate
      // does not, we replace the old one with the new static version.
      duplicate = NULL;
    }
    if (duplicate != NULL) {
      CRYPTO_refcount_inc(&duplicate->references);
    }
    CRYPTO_MUTEX_unlock_read(&pool->lock);

    if (duplicate != NULL) {
      return duplicate;
    }
  }

  CRYPTO_BUFFER *const buf = OPENSSL_zalloc(sizeof(CRYPTO_BUFFER));
  if (buf == NULL) {
    return NULL;
  }

  if (data_is_static) {
    buf->data = (uint8_t *)data;
    buf->data_is_static = 1;
  } else {
    buf->data = OPENSSL_memdup(data, len);
    if (len != 0 && buf->data == NULL) {
      OPENSSL_free(buf);
      return NULL;
    }
  }

  buf->len = len;
  buf->references = 1;

  if (pool == NULL) {
    return buf;
  }

  buf->pool = pool;

  CRYPTO_MUTEX_lock_write(&pool->lock);
  CRYPTO_BUFFER *duplicate = lh_CRYPTO_BUFFER_retrieve(pool->bufs, buf);
  if (data_is_static && duplicate != NULL && !duplicate->data_is_static) {
    // If the new |CRYPTO_BUFFER| would have static data, but the duplicate does
    // not, we replace the old one with the new static version.
    duplicate = NULL;
  }
  int inserted = 0;
  if (duplicate == NULL) {
    CRYPTO_BUFFER *old = NULL;
    inserted = lh_CRYPTO_BUFFER_insert(pool->bufs, &old, buf);
    // |old| may be non-NULL if a match was found but ignored. |pool->bufs| does
    // not increment refcounts, so there is no need to clean up after the
    // replacement.
  } else {
    CRYPTO_refcount_inc(&duplicate->references);
  }
  CRYPTO_MUTEX_unlock_write(&pool->lock);

  if (!inserted) {
    // We raced to insert |buf| into the pool and lost, or else there was an
    // error inserting.
    crypto_buffer_free_object(buf);
    return duplicate;
  }

  return buf;
}

CRYPTO_BUFFER *CRYPTO_BUFFER_new(const uint8_t *data, size_t len,
                                 CRYPTO_BUFFER_POOL *pool) {
  return crypto_buffer_new(data, len, /*data_is_static=*/0, pool);
}

CRYPTO_BUFFER *CRYPTO_BUFFER_alloc(uint8_t **out_data, size_t len) {
  CRYPTO_BUFFER *const buf = OPENSSL_zalloc(sizeof(CRYPTO_BUFFER));
  if (buf == NULL) {
    return NULL;
  }

  buf->data = OPENSSL_malloc(len);
  if (len != 0 && buf->data == NULL) {
    OPENSSL_free(buf);
    return NULL;
  }
  buf->len = len;
  buf->references = 1;

  *out_data = buf->data;
  return buf;
}

CRYPTO_BUFFER *CRYPTO_BUFFER_new_from_CBS(const CBS *cbs,
                                          CRYPTO_BUFFER_POOL *pool) {
  return CRYPTO_BUFFER_new(CBS_data(cbs), CBS_len(cbs), pool);
}

CRYPTO_BUFFER *CRYPTO_BUFFER_new_from_static_data_unsafe(
    const uint8_t *data, size_t len, CRYPTO_BUFFER_POOL *pool) {
  return crypto_buffer_new(data, len, /*data_is_static=*/1, pool);
}

void CRYPTO_BUFFER_free(CRYPTO_BUFFER *buf) {
  if (buf == NULL) {
    return;
  }

  CRYPTO_BUFFER_POOL *const pool = buf->pool;
  if (pool == NULL) {
    if (CRYPTO_refcount_dec_and_test_zero(&buf->references)) {
      // If a reference count of zero is observed, there cannot be a reference
      // from any pool to this buffer and thus we are able to free this
      // buffer.
      crypto_buffer_free_object(buf);
    }

    return;
  }

  CRYPTO_MUTEX_lock_write(&pool->lock);
  if (!CRYPTO_refcount_dec_and_test_zero(&buf->references)) {
    CRYPTO_MUTEX_unlock_write(&buf->pool->lock);
    return;
  }

  // We have an exclusive lock on the pool, therefore no concurrent lookups can
  // find this buffer and increment the reference count. Thus, if the count is
  // zero there are and can never be any more references and thus we can free
  // this buffer.
  //
  // Note it is possible |buf| is no longer in the pool, if it was replaced by a
  // static version. If that static version was since removed, it is even
  // possible for |found| to be NULL.
  CRYPTO_BUFFER *found = lh_CRYPTO_BUFFER_retrieve(pool->bufs, buf);
  if (found == buf) {
    found = lh_CRYPTO_BUFFER_delete(pool->bufs, buf);
    assert(found == buf);
    (void)found;
  }

  CRYPTO_MUTEX_unlock_write(&buf->pool->lock);
  crypto_buffer_free_object(buf);
}

int CRYPTO_BUFFER_up_ref(CRYPTO_BUFFER *buf) {
  // This is safe in the case that |buf->pool| is NULL because it's just
  // standard reference counting in that case.
  //
  // This is also safe if |buf->pool| is non-NULL because, if it were racing
  // with |CRYPTO_BUFFER_free| then the two callers must have independent
  // references already and so the reference count will never hit zero.
  CRYPTO_refcount_inc(&buf->references);
  return 1;
}

const uint8_t *CRYPTO_BUFFER_data(const CRYPTO_BUFFER *buf) {
  return buf->data;
}

size_t CRYPTO_BUFFER_len(const CRYPTO_BUFFER *buf) {
  return buf->len;
}

void CRYPTO_BUFFER_init_CBS(const CRYPTO_BUFFER *buf, CBS *out) {
  CBS_init(out, buf->data, buf->len);
}
