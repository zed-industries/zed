// Copyright 2000-2016 The OpenSSL Project Authors. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#include <openssl/bn.h>

#include <assert.h>
#include <string.h>

#include <openssl/err.h>
#include <openssl/mem.h>

#include "../../internal.h"


// The stack frame info is resizing, set a first-time expansion size;
#define BN_CTX_START_FRAMES 32


// BN_STACK

// A |BN_STACK| is a stack of |size_t| values.
typedef struct {
  // Array of indexes into |ctx->bignums|.
  size_t *indexes;
  // Number of stack frames, and the size of the allocated array
  size_t depth, size;
} BN_STACK;

static void BN_STACK_init(BN_STACK *);
static void BN_STACK_cleanup(BN_STACK *);
static int BN_STACK_push(BN_STACK *, size_t idx);
static size_t BN_STACK_pop(BN_STACK *);


// BN_CTX

DEFINE_STACK_OF(BIGNUM)

// The opaque BN_CTX type
struct bignum_ctx {
  // bignums is the stack of |BIGNUM|s managed by this |BN_CTX|.
  STACK_OF(BIGNUM) *bignums;
  // stack is the stack of |BN_CTX_start| frames. It is the value of |used| at
  // the time |BN_CTX_start| was called.
  BN_STACK stack;
  // used is the number of |BIGNUM|s from |bignums| that have been used.
  size_t used;
  // error is one if any operation on this |BN_CTX| failed. All subsequent
  // operations will fail.
  char error;
  // defer_error is one if an operation on this |BN_CTX| has failed, but no
  // error has been pushed to the queue yet. This is used to defer errors from
  // |BN_CTX_start| to |BN_CTX_get|.
  char defer_error;
};

BN_CTX *BN_CTX_new(void) {
  BN_CTX *ret = reinterpret_cast<BN_CTX *>(OPENSSL_malloc(sizeof(BN_CTX)));
  if (!ret) {
    return NULL;
  }

  // Initialise the structure
  ret->bignums = NULL;
  BN_STACK_init(&ret->stack);
  ret->used = 0;
  ret->error = 0;
  ret->defer_error = 0;
  return ret;
}

void BN_CTX_free(BN_CTX *ctx) {
  // All |BN_CTX_start| calls must be matched with |BN_CTX_end|, otherwise the
  // function may use more memory than expected, potentially without bound if
  // done in a loop. Assert that all |BIGNUM|s have been released.
  if (ctx == nullptr) {
    return;
  }
  assert(ctx->used == 0 || ctx->error);
  sk_BIGNUM_pop_free(ctx->bignums, BN_free);
  BN_STACK_cleanup(&ctx->stack);
  OPENSSL_free(ctx);
}

void BN_CTX_start(BN_CTX *ctx) {
  if (ctx->error) {
    // Once an operation has failed, |ctx->stack| no longer matches the number
    // of |BN_CTX_end| calls to come. Do nothing.
    return;
  }

  if (!BN_STACK_push(&ctx->stack, ctx->used)) {
    ctx->error = 1;
    // |BN_CTX_start| cannot fail, so defer the error to |BN_CTX_get|.
    ctx->defer_error = 1;
  }
}

BIGNUM *BN_CTX_get(BN_CTX *ctx) {
  // Once any operation has failed, they all do.
  if (ctx->error) {
    if (ctx->defer_error) {
      OPENSSL_PUT_ERROR(BN, BN_R_TOO_MANY_TEMPORARY_VARIABLES);
      ctx->defer_error = 0;
    }
    return NULL;
  }

  if (ctx->bignums == NULL) {
    ctx->bignums = sk_BIGNUM_new_null();
    if (ctx->bignums == NULL) {
      ctx->error = 1;
      return NULL;
    }
  }

  if (ctx->used == sk_BIGNUM_num(ctx->bignums)) {
    BIGNUM *bn = BN_new();
    if (bn == NULL || !sk_BIGNUM_push(ctx->bignums, bn)) {
      OPENSSL_PUT_ERROR(BN, BN_R_TOO_MANY_TEMPORARY_VARIABLES);
      BN_free(bn);
      ctx->error = 1;
      return NULL;
    }
  }

  BIGNUM *ret = sk_BIGNUM_value(ctx->bignums, ctx->used);
  BN_zero(ret);
  // This is bounded by |sk_BIGNUM_num|, so it cannot overflow.
  ctx->used++;
  return ret;
}

void BN_CTX_end(BN_CTX *ctx) {
  if (ctx->error) {
    // Once an operation has failed, |ctx->stack| no longer matches the number
    // of |BN_CTX_end| calls to come. Do nothing.
    return;
  }

  ctx->used = BN_STACK_pop(&ctx->stack);
}


// BN_STACK

static void BN_STACK_init(BN_STACK *st) {
  st->indexes = NULL;
  st->depth = st->size = 0;
}

static void BN_STACK_cleanup(BN_STACK *st) { OPENSSL_free(st->indexes); }

static int BN_STACK_push(BN_STACK *st, size_t idx) {
  if (st->depth == st->size) {
    // This function intentionally does not push to the error queue on error.
    // Error-reporting is deferred to |BN_CTX_get|.
    size_t new_size = st->size != 0 ? st->size * 3 / 2 : BN_CTX_START_FRAMES;
    if (new_size <= st->size || new_size > SIZE_MAX / sizeof(size_t)) {
      return 0;
    }
    size_t *new_indexes = reinterpret_cast<size_t *>(
        OPENSSL_realloc(st->indexes, new_size * sizeof(size_t)));
    if (new_indexes == NULL) {
      return 0;
    }
    st->indexes = new_indexes;
    st->size = new_size;
  }

  st->indexes[st->depth] = idx;
  st->depth++;
  return 1;
}

static size_t BN_STACK_pop(BN_STACK *st) {
  assert(st->depth > 0);
  st->depth--;
  return st->indexes[st->depth];
}
