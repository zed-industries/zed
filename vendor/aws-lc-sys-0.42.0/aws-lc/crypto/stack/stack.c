// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/stack.h>

#include <assert.h>
#include <limits.h>

#include <openssl/err.h>
#include <openssl/mem.h>

#include "../internal.h"


struct stack_st {
  // num contains the number of valid pointers in |data|.
  size_t num;
  void **data;
  // sorted is non-zero if the values pointed to by |data| are in ascending
  // order, based on |comp|.
  int sorted;
  // num_alloc contains the number of pointers allocated in the buffer pointed
  // to by |data|, which may be larger than |num|.
  size_t num_alloc;
  // comp is an optional comparison function.
  OPENSSL_sk_cmp_func comp;
};

// kMinSize is the number of pointers that will be initially allocated in a new
// stack.
static const size_t kMinSize = 4;

OPENSSL_STACK *OPENSSL_sk_new(OPENSSL_sk_cmp_func comp) {
  OPENSSL_STACK *ret = OPENSSL_zalloc(sizeof(OPENSSL_STACK));
  if (ret == NULL) {
    return NULL;
  }

  ret->data = OPENSSL_calloc(kMinSize, sizeof(void *));
  if (ret->data == NULL) {
    goto err;
  }

  ret->comp = comp;
  ret->num_alloc = kMinSize;

  return ret;

err:
  OPENSSL_free(ret);
  return NULL;
}

OPENSSL_STACK *OPENSSL_sk_new_null(void) { return OPENSSL_sk_new(NULL); }

size_t OPENSSL_sk_num(const OPENSSL_STACK *sk) {
  if (sk == NULL) {
    return 0;
  }
  return sk->num;
}

void OPENSSL_sk_zero(OPENSSL_STACK *sk) {
  if (sk == NULL || sk->num == 0) {
    return;
  }
  OPENSSL_memset(sk->data, 0, sizeof(void*) * sk->num);
  sk->num = 0;
  sk->sorted = 0;
}

void *OPENSSL_sk_value(const OPENSSL_STACK *sk, size_t i) {
  if (!sk || i >= sk->num) {
    return NULL;
  }
  return sk->data[i];
}

void *OPENSSL_sk_set(OPENSSL_STACK *sk, size_t i, void *value) {
  if (!sk || i >= sk->num) {
    return NULL;
  }
  return sk->data[i] = value;
}

void OPENSSL_sk_free(OPENSSL_STACK *sk) {
  if (sk == NULL) {
    return;
  }
  OPENSSL_free(sk->data);
  OPENSSL_free(sk);
}

void OPENSSL_sk_pop_free_ex(OPENSSL_STACK *sk,
                            OPENSSL_sk_call_free_func call_free_func,
                            OPENSSL_sk_free_func free_func) {
  if (sk == NULL) {
    return;
  }

  for (size_t i = 0; i < sk->num; i++) {
    if (sk->data[i] != NULL) {
      call_free_func(free_func, sk->data[i]);
    }
  }
  OPENSSL_sk_free(sk);
}

// Historically, |sk_pop_free| called the function as |OPENSSL_sk_free_func|
// directly. This is undefined in C. Some callers called |sk_pop_free| directly,
// so we must maintain a compatibility version for now.
static void call_free_func_legacy(OPENSSL_sk_free_func func, void *ptr) {
  func(ptr);
}

void sk_pop_free(OPENSSL_STACK *sk, OPENSSL_sk_free_func free_func) {
  OPENSSL_sk_pop_free_ex(sk, call_free_func_legacy, free_func);
}

size_t OPENSSL_sk_insert(OPENSSL_STACK *sk, void *p, size_t where) {
  if (sk == NULL) {
    return 0;
  }

  if (sk->num >= INT_MAX) {
    OPENSSL_PUT_ERROR(CRYPTO, ERR_R_OVERFLOW);
    return 0;
  }

  if (sk->num_alloc <= sk->num + 1) {
    // Attempt to double the size of the array.
    size_t new_alloc = sk->num_alloc << 1;
    size_t alloc_size = new_alloc * sizeof(void *);
    void **data;

    // If the doubling overflowed, try to increment.
    if (new_alloc < sk->num_alloc || alloc_size / sizeof(void *) != new_alloc) {
      new_alloc = sk->num_alloc + 1;
      alloc_size = new_alloc * sizeof(void *);
    }

    // If the increment also overflowed, fail.
    if (new_alloc < sk->num_alloc || alloc_size / sizeof(void *) != new_alloc) {
      return 0;
    }

    data = OPENSSL_realloc(sk->data, alloc_size);
    if (data == NULL) {
      return 0;
    }

    sk->data = data;
    sk->num_alloc = new_alloc;
  }

  if (where >= sk->num) {
    sk->data[sk->num] = p;
  } else {
    OPENSSL_memmove(&sk->data[where + 1], &sk->data[where],
                    sizeof(void *) * (sk->num - where));
    sk->data[where] = p;
  }

  sk->num++;
  sk->sorted = 0;

  return sk->num;
}

void *OPENSSL_sk_delete(OPENSSL_STACK *sk, size_t where) {
  void *ret;

  if (!sk || where >= sk->num) {
    return NULL;
  }

  ret = sk->data[where];

  if (where != sk->num - 1) {
    OPENSSL_memmove(&sk->data[where], &sk->data[where + 1],
                    sizeof(void *) * (sk->num - where - 1));
  }

  sk->num--;
  return ret;
}

void *OPENSSL_sk_delete_ptr(OPENSSL_STACK *sk, const void *p) {
  if (sk == NULL) {
    return NULL;
  }

  for (size_t i = 0; i < sk->num; i++) {
    if (sk->data[i] == p) {
      return OPENSSL_sk_delete(sk, i);
    }
  }

  return NULL;
}

void OPENSSL_sk_delete_if(OPENSSL_STACK *sk,
                          OPENSSL_sk_call_delete_if_func call_func,
                          OPENSSL_sk_delete_if_func func, void *data) {
  if (sk == NULL) {
    return;
  }

  size_t new_num = 0;
  for (size_t i = 0; i < sk->num; i++) {
    if (!call_func(func, sk->data[i], data)) {
      sk->data[new_num] = sk->data[i];
      new_num++;
    }
  }
  sk->num = new_num;
}

int OPENSSL_sk_find(const OPENSSL_STACK *sk, size_t *out_index, const void *p,
                    OPENSSL_sk_call_cmp_func call_cmp_func) {
  if (sk == NULL) {
    return 0;
  }

  if (sk->comp == NULL) {
    // Use pointer equality when no comparison function has been set.
    for (size_t i = 0; i < sk->num; i++) {
      if (sk->data[i] == p) {
        if (out_index) {
          *out_index = i;
        }
        return 1;
      }
    }
    return 0;
  }

  if (p == NULL) {
    return 0;
  }

  if (!OPENSSL_sk_is_sorted(sk)) {
    for (size_t i = 0; i < sk->num; i++) {
      if (call_cmp_func(sk->comp, p, sk->data[i]) == 0) {
        if (out_index) {
          *out_index = i;
        }
        return 1;
      }
    }
    return 0;
  }

  // The stack is sorted, so binary search to find the element.
  //
  // |lo| and |hi| maintain a half-open interval of where the answer may be. All
  // indices such that |lo <= idx < hi| are candidates.
  size_t lo = 0, hi = sk->num;
  while (lo < hi) {
    // Bias |mid| towards |lo|. See the |r == 0| case below.
    size_t mid = lo + (hi - lo - 1) / 2;
    assert(lo <= mid && mid < hi);
    int r = call_cmp_func(sk->comp, p, sk->data[mid]);
    if (r > 0) {
      lo = mid + 1;  // |mid| is too low.
    } else if (r < 0) {
      hi = mid;  // |mid| is too high.
    } else {
      // |mid| matches. However, this function returns the earliest match, so we
      // can only return if the range has size one.
      if (hi - lo == 1) {
        if (out_index != NULL) {
          *out_index = mid;
        }
        return 1;
      }
      // The sample is biased towards |lo|. |mid| can only be |hi - 1| if
      // |hi - lo| was one, so this makes forward progress.
      assert(mid + 1 < hi);
      hi = mid + 1;
    }
  }

  assert(lo == hi);
  return 0;  // Not found.
}

int OPENSSL_sk_unshift(OPENSSL_STACK *sk, void *data) {
    return (int)OPENSSL_sk_insert(sk, data, 0);
}

void *OPENSSL_sk_shift(OPENSSL_STACK *sk) {
  if (sk == NULL) {
    return NULL;
  }
  if (sk->num == 0) {
    return NULL;
  }
  return OPENSSL_sk_delete(sk, 0);
}

size_t OPENSSL_sk_push(OPENSSL_STACK *sk, void *p) {
  return OPENSSL_sk_insert(sk, p, sk->num);
}

void *OPENSSL_sk_pop(OPENSSL_STACK *sk) {
  if (sk == NULL) {
    return NULL;
  }
  if (sk->num == 0) {
    return NULL;
  }
  return OPENSSL_sk_delete(sk, sk->num - 1);
}

OPENSSL_STACK *OPENSSL_sk_dup(const OPENSSL_STACK *sk) {
  if (sk == NULL) {
    return NULL;
  }

  OPENSSL_STACK *ret = OPENSSL_zalloc(sizeof(OPENSSL_STACK));
  if (ret == NULL) {
    return NULL;
  }

  ret->data = OPENSSL_memdup(sk->data, sizeof(void *) * sk->num_alloc);
  if (ret->data == NULL) {
    goto err;
  }

  ret->num = sk->num;
  ret->sorted = sk->sorted;
  ret->num_alloc = sk->num_alloc;
  ret->comp = sk->comp;
  return ret;

err:
  OPENSSL_sk_free(ret);
  return NULL;
}

#if defined(_MSC_VER)
struct sort_compare_ctx {
  OPENSSL_sk_call_cmp_func call_cmp_func;
  OPENSSL_sk_cmp_func cmp_func;
};

static int sort_compare(void *ctx_v, const void *a, const void *b) {
  struct sort_compare_ctx *ctx = ctx_v;
  // |a| and |b| point to |void*| pointers which contain the actual values.
  const void *const *a_ptr = a;
  const void *const *b_ptr = b;
  return ctx->call_cmp_func(ctx->cmp_func, *a_ptr, *b_ptr);
}
#endif

void OPENSSL_sk_sort(OPENSSL_STACK *sk,
                     OPENSSL_sk_call_cmp_func call_cmp_func) {
  if (sk == NULL || sk->comp == NULL || sk->sorted) {
    return;
  }

  if (sk->num >= 2) {
#if defined(_MSC_VER)
    // MSVC's |qsort_s| is different from the C11 one.
    // https://docs.microsoft.com/en-us/cpp/c-runtime-library/reference/qsort-s?view=msvc-170
    struct sort_compare_ctx ctx = {call_cmp_func, sk->comp};
    qsort_s(sk->data, sk->num, sizeof(void *), sort_compare, &ctx);
#else
    // sk->comp is a function that takes pointers to pointers to elements, but
    // qsort take a comparison function that just takes pointers to elements.
    // However, since we're passing an array of pointers to qsort, we can just
    // cast the comparison function and everything works.
    //
    // TODO(davidben): This is undefined behavior, but the call is in libc so,
    // e.g., CFI does not notice. |qsort| is missing a void* parameter in its
    // callback, while no one defines |qsort_r| or |qsort_s| consistently. See
    // https://stackoverflow.com/a/39561369
    int (*comp_func)(const void *, const void *) =
        (int (*)(const void *, const void *))(sk->comp);
    qsort(sk->data, sk->num, sizeof(void *), comp_func);
#endif
  }
  sk->sorted = 1;
}

int OPENSSL_sk_is_sorted(const OPENSSL_STACK *sk) {
  if (!sk) {
    return 1;
  }
  // Zero- and one-element lists are always sorted.
  return sk->sorted || (sk->comp != NULL && sk->num < 2);
}

OPENSSL_sk_cmp_func OPENSSL_sk_set_cmp_func(OPENSSL_STACK *sk,
                                            OPENSSL_sk_cmp_func comp) {
  OPENSSL_sk_cmp_func old = sk->comp;

  if (sk->comp != comp) {
    sk->sorted = 0;
  }
  sk->comp = comp;

  return old;
}

OPENSSL_STACK *OPENSSL_sk_deep_copy(const OPENSSL_STACK *sk,
                                    OPENSSL_sk_call_copy_func call_copy_func,
                                    OPENSSL_sk_copy_func copy_func,
                                    OPENSSL_sk_call_free_func call_free_func,
                                    OPENSSL_sk_free_func free_func) {
  OPENSSL_STACK *ret = OPENSSL_sk_dup(sk);
  if (ret == NULL) {
    return NULL;
  }

  for (size_t i = 0; i < ret->num; i++) {
    if (ret->data[i] == NULL) {
      continue;
    }
    ret->data[i] = call_copy_func(copy_func, ret->data[i]);
    if (ret->data[i] == NULL) {
      for (size_t j = 0; j < i; j++) {
        if (ret->data[j] != NULL) {
          call_free_func(free_func, ret->data[j]);
        }
      }
      OPENSSL_sk_free(ret);
      return NULL;
    }
  }

  return ret;
}
