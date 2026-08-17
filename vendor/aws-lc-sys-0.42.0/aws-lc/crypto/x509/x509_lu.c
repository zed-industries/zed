// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <string.h>

#include <openssl/err.h>
#include <openssl/mem.h>
#include <openssl/thread.h>
#include <openssl/x509.h>

#include "../internal.h"
#include "internal.h"


static int X509_OBJECT_idx_by_subject(STACK_OF(X509_OBJECT) *h, int type,
                                      X509_NAME *name);
static X509_OBJECT *X509_OBJECT_retrieve_by_subject(
      STACK_OF(X509_OBJECT) *h, int type, X509_NAME *name);
static X509_OBJECT *X509_OBJECT_retrieve_match(STACK_OF(X509_OBJECT) *h,
                                               X509_OBJECT *x);
static int X509_OBJECT_up_ref_count(X509_OBJECT *a);

static X509_LOOKUP *X509_LOOKUP_new(const X509_LOOKUP_METHOD *method,
                                    X509_STORE *store);
static int X509_LOOKUP_by_subject(X509_LOOKUP *ctx, int type, X509_NAME *name,
                                  X509_OBJECT *ret);

static X509_LOOKUP *X509_LOOKUP_new(const X509_LOOKUP_METHOD *method,
                                    X509_STORE *store) {
  X509_LOOKUP *ret = OPENSSL_zalloc(sizeof(X509_LOOKUP));
  if (ret == NULL) {
    return NULL;
  }

  ret->method = method;
  ret->store_ctx = store;
  if ((method->new_item != NULL) && !method->new_item(ret)) {
    OPENSSL_free(ret);
    return NULL;
  }
  return ret;
}

void X509_LOOKUP_free(X509_LOOKUP *ctx) {
  if (ctx == NULL) {
    return;
  }
  if (ctx->method != NULL && ctx->method->free != NULL) {
    (*ctx->method->free)(ctx);
  }
  OPENSSL_free(ctx);
}

int X509_LOOKUP_ctrl(X509_LOOKUP *ctx, int cmd, const char *argc, long argl,
                     char **ret) {
  if (ctx->method == NULL) {
    return -1;
  }
  if (ctx->method->ctrl != NULL) {
    return ctx->method->ctrl(ctx, cmd, argc, argl, ret);
  } else {
    return 1;
  }
}

static int X509_LOOKUP_by_subject(X509_LOOKUP *ctx, int type, X509_NAME *name,
                                  X509_OBJECT *ret) {
  if (ctx->method == NULL || ctx->method->get_by_subject == NULL) {
    return 0;
  }
  // Note |get_by_subject| leaves |ret| in an inconsistent state. It has
  // pointers to an |X509| or |X509_CRL|, but has not bumped the refcount yet.
  // For now, the caller is expected to fix this, but ideally we'd fix the
  // |X509_LOOKUP| convention itself.
  return ctx->method->get_by_subject(ctx, type, name, ret) > 0;
}

static int x509_object_cmp(const X509_OBJECT *a, const X509_OBJECT *b) {
  int ret = a->type - b->type;
  if (ret) {
    return ret;
  }
  switch (a->type) {
    case X509_LU_X509:
      return X509_subject_name_cmp(a->data.x509, b->data.x509);
    case X509_LU_CRL:
      return X509_CRL_cmp(a->data.crl, b->data.crl);
    default:
      // abort();
      return 0;
  }
}

static int x509_object_cmp_sk(const X509_OBJECT *const *a,
                              const X509_OBJECT *const *b) {
  return x509_object_cmp(*a, *b);
}

static CRYPTO_EX_DATA_CLASS g_ex_data_class =
    CRYPTO_EX_DATA_CLASS_INIT_WITH_APP_DATA;

X509_STORE *X509_STORE_new(void) {
  X509_STORE *ret = OPENSSL_zalloc(sizeof(X509_STORE));
  if (ret == NULL) {
    return NULL;
  }

  ret->references = 1;
  CRYPTO_MUTEX_init(&ret->objs_lock);
  CRYPTO_new_ex_data(&ret->ex_data);
  ret->objs = sk_X509_OBJECT_new(x509_object_cmp_sk);
  ret->get_cert_methods = sk_X509_LOOKUP_new_null();
  ret->param = X509_VERIFY_PARAM_new();
  if (ret->objs == NULL ||
      ret->get_cert_methods == NULL ||
      ret->param == NULL) {
    X509_STORE_free(ret);
    return NULL;
  }

  return ret;
}

int X509_STORE_lock(X509_STORE *v) {
    if (v == NULL) {
      return 0;
    }
    CRYPTO_MUTEX_lock_write(&v->objs_lock);
    return 1;
}

int X509_STORE_unlock(X509_STORE *v) {
    if (v == NULL) {
      return 0;
    }
    CRYPTO_MUTEX_unlock_write(&v->objs_lock);
    return 1;
}

int X509_STORE_up_ref(X509_STORE *store) {
  CRYPTO_refcount_inc(&store->references);
  return 1;
}

void X509_STORE_free(X509_STORE *vfy) {
  if (vfy == NULL) {
    return;
  }

  if (!CRYPTO_refcount_dec_and_test_zero(&vfy->references)) {
    return;
  }

  CRYPTO_MUTEX_cleanup(&vfy->objs_lock);
  CRYPTO_free_ex_data(&g_ex_data_class, vfy, &vfy->ex_data);
  sk_X509_LOOKUP_pop_free(vfy->get_cert_methods, X509_LOOKUP_free);
  sk_X509_OBJECT_pop_free(vfy->objs, X509_OBJECT_free);
  X509_VERIFY_PARAM_free(vfy->param);
  OPENSSL_free(vfy);
}

X509_LOOKUP *X509_STORE_add_lookup(X509_STORE *v, const X509_LOOKUP_METHOD *m) {
  STACK_OF(X509_LOOKUP) *sk = v->get_cert_methods;
  for (size_t i = 0; i < sk_X509_LOOKUP_num(sk); i++) {
    X509_LOOKUP *lu = sk_X509_LOOKUP_value(sk, i);
    if (m == lu->method) {
      return lu;
    }
  }

  X509_LOOKUP *lu = X509_LOOKUP_new(m, v);
  if (lu == NULL || !sk_X509_LOOKUP_push(v->get_cert_methods, lu)) {
    X509_LOOKUP_free(lu);
    return NULL;
  }

  return lu;
}

int X509_STORE_CTX_get_by_subject(X509_STORE_CTX *vs, int type, X509_NAME *name,
                                  X509_OBJECT *ret) {
  X509_STORE *ctx = vs->ctx;
  X509_OBJECT stmp;
  CRYPTO_MUTEX_lock_write(&ctx->objs_lock);
  X509_OBJECT *tmp = X509_OBJECT_retrieve_by_subject(ctx->objs, type, name);
  CRYPTO_MUTEX_unlock_write(&ctx->objs_lock);

  if (tmp == NULL || type == X509_LU_CRL) {
    for (size_t i = 0; i < sk_X509_LOOKUP_num(ctx->get_cert_methods); i++) {
      X509_LOOKUP *lu = sk_X509_LOOKUP_value(ctx->get_cert_methods, i);
      if (X509_LOOKUP_by_subject(lu, type, name, &stmp)) {
        tmp = &stmp;
        break;
      }
    }
    if (tmp == NULL) {
      return 0;
    }
  }

  // if (ret->data.ptr != NULL) X509_OBJECT_free_contents(ret);

  ret->type = tmp->type;
  ret->data.ptr = tmp->data.ptr;

  X509_OBJECT_up_ref_count(ret);

  return 1;
}

static int x509_store_add(X509_STORE *ctx, void *x, int is_crl) {
  if (x == NULL) {
    return 0;
  }

  X509_OBJECT *const obj = X509_OBJECT_new();
  if (obj == NULL) {
    return 0;
  }

  if (is_crl) {
    obj->type = X509_LU_CRL;
    obj->data.crl = (X509_CRL *)x;
  } else {
    obj->type = X509_LU_X509;
    obj->data.x509 = (X509 *)x;
  }
  X509_OBJECT_up_ref_count(obj);

  CRYPTO_MUTEX_lock_write(&ctx->objs_lock);

  int ret = 1;
  int added = 0;
  // Duplicates are silently ignored
  if (!X509_OBJECT_retrieve_match(ctx->objs, obj)) {
    ret = added = (sk_X509_OBJECT_push(ctx->objs, obj) != 0);
  }

  CRYPTO_MUTEX_unlock_write(&ctx->objs_lock);

  if (!added) {
    X509_OBJECT_free(obj);
  }

  return ret;
}

int X509_STORE_add_cert(X509_STORE *ctx, X509 *x) {
  return x509_store_add(ctx, x, /*is_crl=*/0);
}

int X509_STORE_add_crl(X509_STORE *ctx, X509_CRL *x) {
  return x509_store_add(ctx, x, /*is_crl=*/1);
}

X509_OBJECT *X509_OBJECT_new(void) {
  return OPENSSL_zalloc(sizeof(X509_OBJECT));
}

void X509_OBJECT_free(X509_OBJECT *obj) {
  if (obj == NULL) {
    return;
  }
  X509_OBJECT_free_contents(obj);
  OPENSSL_free(obj);
}

static int X509_OBJECT_up_ref_count(X509_OBJECT *a) {
  switch (a->type) {
    case X509_LU_X509:
      X509_up_ref(a->data.x509);
      break;
    case X509_LU_CRL:
      X509_CRL_up_ref(a->data.crl);
      break;
  }
  return 1;
}

void X509_OBJECT_free_contents(X509_OBJECT *a) {
  switch (a->type) {
    case X509_LU_X509:
      X509_free(a->data.x509);
      break;
    case X509_LU_CRL:
      X509_CRL_free(a->data.crl);
      break;
  }

  OPENSSL_memset(a, 0, sizeof(X509_OBJECT));
}

int X509_OBJECT_get_type(const X509_OBJECT *a) { return a->type; }

X509 *X509_OBJECT_get0_X509(const X509_OBJECT *a) {
  if (a == NULL || a->type != X509_LU_X509) {
    return NULL;
  }
  return a->data.x509;
}

X509_CRL *X509_OBJECT_get0_X509_CRL(const X509_OBJECT *a) {
  if (a == NULL || a->type != X509_LU_CRL) {
    return NULL;
  }
  return a->data.crl;
}

int X509_OBJECT_set1_X509(X509_OBJECT *a, X509 *obj) {
    if (a == NULL || !X509_up_ref(obj)) {
      return 0;
    }

    X509_OBJECT_free_contents(a);
    a->type = X509_LU_X509;
    a->data.x509 = obj;
    return 1;
}

int X509_OBJECT_set1_X509_CRL(X509_OBJECT *a, X509_CRL *obj) {
    if (a == NULL || !X509_CRL_up_ref(obj)) {
      return 0;
    }

    X509_OBJECT_free_contents(a);
    a->type = X509_LU_CRL;
    a->data.crl = obj;
    return 1;
}

static int x509_object_idx_cnt(STACK_OF(X509_OBJECT) *h, int type,
                               X509_NAME *name, int *pnmatch) {
  X509_OBJECT stmp;
  X509 x509_s;
  X509_CINF cinf_s;
  X509_CRL crl_s;
  X509_CRL_INFO crl_info_s;

  stmp.type = type;
  switch (type) {
    case X509_LU_X509:
      stmp.data.x509 = &x509_s;
      x509_s.cert_info = &cinf_s;
      cinf_s.subject = name;
      break;
    case X509_LU_CRL:
      stmp.data.crl = &crl_s;
      crl_s.crl = &crl_info_s;
      crl_info_s.issuer = name;
      break;
    default:
      // abort();
      return -1;
  }

  size_t idx;
  sk_X509_OBJECT_sort(h);
  if (!sk_X509_OBJECT_find_awslc(h, &idx, &stmp)) {
    return -1;
  }

  if (pnmatch != NULL) {
    *pnmatch = 1;
    for (size_t tidx = idx + 1; tidx < sk_X509_OBJECT_num(h); tidx++) {
      const X509_OBJECT *tobj = sk_X509_OBJECT_value(h, tidx);
      if (x509_object_cmp(tobj, &stmp)) {
        break;
      }
      (*pnmatch)++;
    }
  }

  return (int)idx;
}

static int X509_OBJECT_idx_by_subject(STACK_OF(X509_OBJECT) *h, int type,
                                      X509_NAME *name) {
  return x509_object_idx_cnt(h, type, name, NULL);
}

X509_OBJECT *X509_OBJECT_retrieve_by_subject(STACK_OF(X509_OBJECT) *h,
                                                    int type, X509_NAME *name) {
  int idx;
  idx = X509_OBJECT_idx_by_subject(h, type, name);
  if (idx == -1) {
    return NULL;
  }
  return sk_X509_OBJECT_value(h, idx);
}

STACK_OF(X509_OBJECT) *X509_STORE_get0_objects(X509_STORE *st) {
  return st->objs;
}

STACK_OF(X509) *X509_STORE_CTX_get1_certs(X509_STORE_CTX *ctx, X509_NAME *nm) {
  int cnt;
  STACK_OF(X509) *sk = sk_X509_new_null();
  if (sk == NULL) {
    return NULL;
  }
  CRYPTO_MUTEX_lock_write(&ctx->ctx->objs_lock);
  int idx = x509_object_idx_cnt(ctx->ctx->objs, X509_LU_X509, nm, &cnt);
  if (idx < 0) {
    // Nothing found in cache: do lookup to possibly add new objects to
    // cache
    X509_OBJECT xobj;
    CRYPTO_MUTEX_unlock_write(&ctx->ctx->objs_lock);
    if (!X509_STORE_CTX_get_by_subject(ctx, X509_LU_X509, nm, &xobj)) {
      sk_X509_free(sk);
      return NULL;
    }
    X509_OBJECT_free_contents(&xobj);
    CRYPTO_MUTEX_lock_write(&ctx->ctx->objs_lock);
    idx = x509_object_idx_cnt(ctx->ctx->objs, X509_LU_X509, nm, &cnt);
    if (idx < 0) {
      CRYPTO_MUTEX_unlock_write(&ctx->ctx->objs_lock);
      sk_X509_free(sk);
      return NULL;
    }
  }
  for (int i = 0; i < cnt; i++, idx++) {
    X509_OBJECT *obj = sk_X509_OBJECT_value(ctx->ctx->objs, idx);
    X509 *x = obj->data.x509;
    if (!sk_X509_push(sk, x)) {
      CRYPTO_MUTEX_unlock_write(&ctx->ctx->objs_lock);
      sk_X509_pop_free(sk, X509_free);
      return NULL;
    }
    X509_up_ref(x);
  }
  CRYPTO_MUTEX_unlock_write(&ctx->ctx->objs_lock);
  return sk;
}

STACK_OF(X509_CRL) *X509_STORE_CTX_get1_crls(X509_STORE_CTX *ctx,
                                             X509_NAME *nm) {
  int cnt;
  X509_OBJECT xobj;
  STACK_OF(X509_CRL) *sk = sk_X509_CRL_new_null();
  if (sk == NULL) {
    return NULL;
  }

  // Always do lookup to possibly add new CRLs to cache.
  if (!X509_STORE_CTX_get_by_subject(ctx, X509_LU_CRL, nm, &xobj)) {
    sk_X509_CRL_free(sk);
    return NULL;
  }
  X509_OBJECT_free_contents(&xobj);
  CRYPTO_MUTEX_lock_write(&ctx->ctx->objs_lock);
  int idx = x509_object_idx_cnt(ctx->ctx->objs, X509_LU_CRL, nm, &cnt);
  if (idx < 0) {
    CRYPTO_MUTEX_unlock_write(&ctx->ctx->objs_lock);
    sk_X509_CRL_free(sk);
    return NULL;
  }

  for (int i = 0; i < cnt; i++, idx++) {
    X509_OBJECT *obj = sk_X509_OBJECT_value(ctx->ctx->objs, idx);
    X509_CRL *x = obj->data.crl;
    X509_CRL_up_ref(x);
    if (!sk_X509_CRL_push(sk, x)) {
      CRYPTO_MUTEX_unlock_write(&ctx->ctx->objs_lock);
      X509_CRL_free(x);
      sk_X509_CRL_pop_free(sk, X509_CRL_free);
      return NULL;
    }
  }
  CRYPTO_MUTEX_unlock_write(&ctx->ctx->objs_lock);
  return sk;
}

static X509_OBJECT *X509_OBJECT_retrieve_match(STACK_OF(X509_OBJECT) *h,
                                               X509_OBJECT *x) {
  sk_X509_OBJECT_sort(h);
  size_t idx;
  if (!sk_X509_OBJECT_find_awslc(h, &idx, x)) {
    return NULL;
  }
  if ((x->type != X509_LU_X509) && (x->type != X509_LU_CRL)) {
    return sk_X509_OBJECT_value(h, idx);
  }
  for (size_t i = idx; i < sk_X509_OBJECT_num(h); i++) {
    X509_OBJECT *obj = sk_X509_OBJECT_value(h, i);
    if (x509_object_cmp(obj, x)) {
      return NULL;
    }
    if (x->type == X509_LU_X509) {
      if (!X509_cmp(obj->data.x509, x->data.x509)) {
        return obj;
      }
    } else if (x->type == X509_LU_CRL) {
      if (!X509_CRL_match(obj->data.crl, x->data.crl)) {
        return obj;
      }
    } else {
      return obj;
    }
  }
  return NULL;
}

// Try to get issuer certificate from store. Due to limitations of the API
// this can only retrieve a single certificate matching a given subject name.
// However it will fill the cache with all matching certificates, so we can
// examine the cache for all matches. Return values are: 1 lookup
// successful.  0 certificate not found. -1 some other error.
int X509_STORE_CTX_get1_issuer(X509 **issuer, X509_STORE_CTX *ctx, X509 *x) {
  X509_NAME *xn;
  X509_OBJECT obj, *pobj;
  int idx, ret;
  size_t i;
  *issuer = NULL;
  xn = X509_get_issuer_name(x);
  if (!X509_STORE_CTX_get_by_subject(ctx, X509_LU_X509, xn, &obj)) {
    return 0;
  }
  // If certificate matches all OK
  if (x509_check_issued_with_callback(ctx, x, obj.data.x509)) {
    if (x509_check_cert_time(ctx, obj.data.x509, /*suppress_error*/1)) {
      *issuer = obj.data.x509;
      return 1;
    }
  }
  X509_OBJECT_free_contents(&obj);

  // Else find index of first cert accepted by
  // |x509_check_issued_with_callback|.
  ret = 0;
  CRYPTO_MUTEX_lock_write(&ctx->ctx->objs_lock);
  idx = X509_OBJECT_idx_by_subject(ctx->ctx->objs, X509_LU_X509, xn);
  if (idx != -1) {  // should be true as we've had at least one
                    // match
    // Look through all matching certs for suitable issuer
    for (i = idx; i < sk_X509_OBJECT_num(ctx->ctx->objs); i++) {
      pobj = sk_X509_OBJECT_value(ctx->ctx->objs, i);
      // See if we've run past the matches
      if (pobj->type != X509_LU_X509) {
        break;
      }
      if (X509_NAME_cmp(xn, X509_get_subject_name(pobj->data.x509))) {
        break;
      }
      if (x509_check_issued_with_callback(ctx, x, pobj->data.x509)) {
        *issuer = pobj->data.x509;
        ret = 1;
        // Break the loop with a match if the time check is valid, otherwise
        // we continue searching. We leave the last tested issuer certificate in
        // |issuer| on purpose. This returns the closest match if none of the
        // candidate issuer certificates' timestamps were valid.
        if (x509_check_cert_time(ctx, *issuer, /*suppress_error*/1)) {
          break;
        }
      }
    }
  }
  CRYPTO_MUTEX_unlock_write(&ctx->ctx->objs_lock);
  if(*issuer) {
    X509_up_ref(*issuer);
  }
  return ret;
}

int X509_STORE_set_flags(X509_STORE *ctx, unsigned long flags) {
  return X509_VERIFY_PARAM_set_flags(ctx->param, flags);
}

int X509_STORE_set_depth(X509_STORE *ctx, int depth) {
  X509_VERIFY_PARAM_set_depth(ctx->param, depth);
  return 1;
}

int X509_STORE_set_purpose(X509_STORE *ctx, int purpose) {
  return X509_VERIFY_PARAM_set_purpose(ctx->param, purpose);
}

int X509_STORE_set_trust(X509_STORE *ctx, int trust) {
  return X509_VERIFY_PARAM_set_trust(ctx->param, trust);
}

int X509_STORE_set1_param(X509_STORE *ctx, const X509_VERIFY_PARAM *param) {
  return X509_VERIFY_PARAM_set1(ctx->param, param);
}

X509_VERIFY_PARAM *X509_STORE_get0_param(X509_STORE *ctx) { return ctx->param; }

void X509_STORE_set_verify_cb(X509_STORE *ctx,
                              X509_STORE_CTX_verify_cb verify_cb) {
  ctx->verify_cb = verify_cb;
}

X509_STORE_CTX_verify_cb X509_STORE_get_verify_cb(X509_STORE *ctx) {
  return ctx->verify_cb;
}

X509_STORE_CTX_lookup_crls_fn X509_STORE_get_lookup_crls(X509_STORE *ctx) {
  return ctx->lookup_crls;
}

void X509_STORE_set_lookup_crls(X509_STORE *ctx,
                                X509_STORE_CTX_lookup_crls_fn lookup_crls) {
  ctx->lookup_crls = lookup_crls;
}

void X509_STORE_set_get_crl(X509_STORE *ctx,
                            X509_STORE_CTX_get_crl_fn get_crl) {
  ctx->get_crl = get_crl;
}

void X509_STORE_set_check_crl(X509_STORE *ctx,
                              X509_STORE_CTX_check_crl_fn check_crl) {
  ctx->check_crl = check_crl;
}

X509_STORE *X509_STORE_CTX_get0_store(X509_STORE_CTX *ctx) { return ctx->ctx; }

int X509_STORE_get_ex_new_index(long argl, void *argp, CRYPTO_EX_unused *unused,
                                CRYPTO_EX_dup *dup_unused,
                                CRYPTO_EX_free *free_func) {
  int index;
  if (!CRYPTO_get_ex_new_index(&g_ex_data_class, &index, argl, argp,
                               free_func)) {
    return -1;
  }
  return index;
}

int X509_STORE_set_ex_data(X509_STORE *ctx, int idx, void *data) {
  return CRYPTO_set_ex_data(&ctx->ex_data, idx, data);
}

void *X509_STORE_get_ex_data(X509_STORE *ctx, int idx) {
  return CRYPTO_get_ex_data(&ctx->ex_data, idx);
}
