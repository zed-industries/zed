// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com)
// Copyright (c) 1998-2001 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/ex_data.h>

#include <assert.h>
#include <limits.h>
#include <stdlib.h>
#include <string.h>

#include <openssl/crypto.h>
#include <openssl/err.h>
#include <openssl/mem.h>
#include <openssl/stack.h>
#include <openssl/thread.h>

#include "internal.h"


DEFINE_STACK_OF(CRYPTO_EX_DATA_FUNCS)

struct crypto_ex_data_func_st {
  long argl;   // Arbitary long
  void *argp;  // Arbitary void pointer
  CRYPTO_EX_free *free_func;
};

int CRYPTO_get_ex_new_index(CRYPTO_EX_DATA_CLASS *ex_data_class, int *out_index,
                            long argl, void *argp, CRYPTO_EX_free *free_func) {
  CRYPTO_EX_DATA_FUNCS *funcs;
  int ret = 0;

  funcs = OPENSSL_malloc(sizeof(CRYPTO_EX_DATA_FUNCS));
  if (funcs == NULL) {
    return 0;
  }

  funcs->argl = argl;
  funcs->argp = argp;
  funcs->free_func = free_func;

  CRYPTO_STATIC_MUTEX_lock_write(&ex_data_class->lock);

  if (ex_data_class->meth == NULL) {
    ex_data_class->meth = sk_CRYPTO_EX_DATA_FUNCS_new_null();
  }

  if (ex_data_class->meth == NULL) {
    goto err;
  }

  // The index must fit in |int|.
  if (sk_CRYPTO_EX_DATA_FUNCS_num(ex_data_class->meth) >
          (size_t)(INT_MAX - ex_data_class->num_reserved)) {
    OPENSSL_PUT_ERROR(CRYPTO, ERR_R_OVERFLOW);
    goto err;
  }

  if (!sk_CRYPTO_EX_DATA_FUNCS_push(ex_data_class->meth, funcs)) {
    goto err;
  }
  funcs = NULL;  // |sk_CRYPTO_EX_DATA_FUNCS_push| takes ownership.

  *out_index = (int)sk_CRYPTO_EX_DATA_FUNCS_num(ex_data_class->meth) - 1 +
               ex_data_class->num_reserved;
  ret = 1;

err:
  CRYPTO_STATIC_MUTEX_unlock_write(&ex_data_class->lock);
  OPENSSL_free(funcs);
  return ret;
}

int CRYPTO_set_ex_data(CRYPTO_EX_DATA *ad, int index, void *val) {
  if (index < 0) {
    // A caller that can accidentally pass in an invalid index into this
    // function will hit an memory error if |index| happened to be valid, and
    // expected |val| to be of a different type.
    abort();
  }

  if (ad->sk == NULL) {
    ad->sk = sk_void_new_null();
    if (ad->sk == NULL) {
      return 0;
    }
  }

  // Add NULL values until the stack is long enough.
  for (size_t i = sk_void_num(ad->sk); i <= (size_t)index; i++) {
    if (!sk_void_push(ad->sk, NULL)) {
      return 0;
    }
  }

  sk_void_set(ad->sk, (size_t)index, val);
  return 1;
}

void *CRYPTO_get_ex_data(const CRYPTO_EX_DATA *ad, int idx) {
  if (ad->sk == NULL || idx < 0 || (size_t)idx >= sk_void_num(ad->sk)) {
    return NULL;
  }
  return sk_void_value(ad->sk, idx);
}

// get_func_pointers takes a copy of the CRYPTO_EX_DATA_FUNCS pointers, if any,
// for the given class. If there are some pointers, it sets |*out| to point to
// a fresh stack of them. Otherwise it sets |*out| to NULL. It returns one on
// success or zero on error.
static int get_func_pointers(STACK_OF(CRYPTO_EX_DATA_FUNCS) **out,
                             CRYPTO_EX_DATA_CLASS *ex_data_class) {
  size_t n;

  *out = NULL;

  // CRYPTO_EX_DATA_FUNCS structures are static once set, so we can take a
  // shallow copy of the list under lock and then use the structures without
  // the lock held.
  CRYPTO_STATIC_MUTEX_lock_read(&ex_data_class->lock);
  n = sk_CRYPTO_EX_DATA_FUNCS_num(ex_data_class->meth);
  if (n > 0) {
    *out = sk_CRYPTO_EX_DATA_FUNCS_dup(ex_data_class->meth);
  }
  CRYPTO_STATIC_MUTEX_unlock_read(&ex_data_class->lock);

  if (n > 0 && *out == NULL) {
    return 0;
  }

  return 1;
}

void CRYPTO_new_ex_data(CRYPTO_EX_DATA *ad) {
  ad->sk = NULL;
}

void CRYPTO_free_ex_data(CRYPTO_EX_DATA_CLASS *ex_data_class, void *obj,
                         CRYPTO_EX_DATA *ad) {
  if (ad->sk == NULL) {
    // Nothing to do.
    return;
  }

  STACK_OF(CRYPTO_EX_DATA_FUNCS) *func_pointers;
  if (!get_func_pointers(&func_pointers, ex_data_class)) {
    // TODO(davidben): This leaks memory on malloc error.
    return;
  }

  // |CRYPTO_get_ex_new_index| will not allocate indices beyond |INT_MAX|.
  assert(sk_CRYPTO_EX_DATA_FUNCS_num(func_pointers) <=
         (size_t)(INT_MAX - ex_data_class->num_reserved));
  for (int i = 0; i < (int)sk_CRYPTO_EX_DATA_FUNCS_num(func_pointers); i++) {
    CRYPTO_EX_DATA_FUNCS *func_pointer =
        sk_CRYPTO_EX_DATA_FUNCS_value(func_pointers, i);
    if (func_pointer->free_func) {
      void *ptr = CRYPTO_get_ex_data(ad, i + ex_data_class->num_reserved);
      func_pointer->free_func(obj, ptr, ad, i + ex_data_class->num_reserved,
                              func_pointer->argl, func_pointer->argp);
    }
  }

  sk_CRYPTO_EX_DATA_FUNCS_free(func_pointers);

  sk_void_free(ad->sk);
  ad->sk = NULL;
}

void CRYPTO_cleanup_all_ex_data(void) {}
