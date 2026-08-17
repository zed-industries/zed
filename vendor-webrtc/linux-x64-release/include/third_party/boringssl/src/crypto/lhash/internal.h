// Copyright 1995-2016 The OpenSSL Project Authors. All Rights Reserved.
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

#ifndef OPENSSL_HEADER_CRYPTO_LHASH_INTERNAL_H
#define OPENSSL_HEADER_CRYPTO_LHASH_INTERNAL_H

#include <openssl/base.h>

#if defined(__cplusplus)
extern "C" {
#endif


// lhash is a traditional, chaining hash table that automatically expands and
// contracts as needed. One should not use the lh_* functions directly, rather
// use the type-safe macro wrappers:
//
// A hash table of a specific type of object has type |LHASH_OF(type)|. This
// can be defined (once) with |DEFINE_LHASH_OF(type)| and declared where needed
// with |DECLARE_LHASH_OF(type)|. For example:
//
//   struct foo {
//     int bar;
//   };
//
//   DEFINE_LHASH_OF(struct foo)
//
// Although note that the hash table will contain /pointers/ to |foo|.
//
// A macro will be defined for each of the |OPENSSL_lh_*| functions below. For
// |LHASH_OF(foo)|, the macros would be |lh_foo_new|, |lh_foo_num_items| etc.
//
// TODO(davidben): Now that this type is completely internal, this can just be a
// C++ template without any macros.


#define LHASH_OF(type) struct lhash_st_##type
#define DECLARE_LHASH_OF(type) LHASH_OF(type);

// lhash_cmp_func is a comparison function that returns a value equal, or not
// equal, to zero depending on whether |*a| is equal, or not equal to |*b|,
// respectively. Note the difference between this and |stack_cmp_func| in that
// this takes pointers to the objects directly.
//
// This function's actual type signature is int (*)(const T*, const T*). The
// low-level |lh_*| functions will be passed a type-specific wrapper to call it
// correctly.
typedef int (*lhash_cmp_func)(const void *a, const void *b);
typedef int (*lhash_cmp_func_helper)(lhash_cmp_func func, const void *a,
                                     const void *b);

// lhash_hash_func is a function that maps an object to a uniformly distributed
// uint32_t.
//
// This function's actual type signature is uint32_t (*)(const T*). The
// low-level |lh_*| functions will be passed a type-specific wrapper to call it
// correctly.
typedef uint32_t (*lhash_hash_func)(const void *a);
typedef uint32_t (*lhash_hash_func_helper)(lhash_hash_func func, const void *a);

typedef struct lhash_st _LHASH;

// OPENSSL_lh_new returns a new, empty hash table or NULL on error.
OPENSSL_EXPORT _LHASH *OPENSSL_lh_new(lhash_hash_func hash,
                                      lhash_cmp_func comp);

// OPENSSL_lh_free frees the hash table itself but none of the elements. See
// |OPENSSL_lh_doall|.
OPENSSL_EXPORT void OPENSSL_lh_free(_LHASH *lh);

// OPENSSL_lh_num_items returns the number of items in |lh|.
OPENSSL_EXPORT size_t OPENSSL_lh_num_items(const _LHASH *lh);

// OPENSSL_lh_retrieve finds an element equal to |data| in the hash table and
// returns it. If no such element exists, it returns NULL.
OPENSSL_EXPORT void *OPENSSL_lh_retrieve(const _LHASH *lh, const void *data,
                                         lhash_hash_func_helper call_hash_func,
                                         lhash_cmp_func_helper call_cmp_func);

// OPENSSL_lh_retrieve_key finds an element matching |key|, given the specified
// hash and comparison function. This differs from |OPENSSL_lh_retrieve| in that
// the key may be a different type than the values stored in |lh|. |key_hash|
// and |cmp_key| must be compatible with the functions passed into
// |OPENSSL_lh_new|.
OPENSSL_EXPORT void *OPENSSL_lh_retrieve_key(const _LHASH *lh, const void *key,
                                             uint32_t key_hash,
                                             int (*cmp_key)(const void *key,
                                                            const void *value));

// OPENSSL_lh_insert inserts |data| into the hash table. If an existing element
// is equal to |data| (with respect to the comparison function) then |*old_data|
// will be set to that value and it will be replaced. Otherwise, or in the
// event of an error, |*old_data| will be set to NULL. It returns one on
// success or zero in the case of an allocation error.
OPENSSL_EXPORT int OPENSSL_lh_insert(_LHASH *lh, void **old_data, void *data,
                                     lhash_hash_func_helper call_hash_func,
                                     lhash_cmp_func_helper call_cmp_func);

// OPENSSL_lh_delete removes an element equal to |data| from the hash table and
// returns it. If no such element is found, it returns NULL.
OPENSSL_EXPORT void *OPENSSL_lh_delete(_LHASH *lh, const void *data,
                                       lhash_hash_func_helper call_hash_func,
                                       lhash_cmp_func_helper call_cmp_func);

// OPENSSL_lh_doall_arg calls |func| on each element of the hash table and also
// passes |arg| as the second argument.
// TODO(fork): rename this
OPENSSL_EXPORT void OPENSSL_lh_doall_arg(_LHASH *lh,
                                         void (*func)(void *, void *),
                                         void *arg);

#define DEFINE_LHASH_OF(type)                                                  \
  /* We disable MSVC C4191 in this macro, which warns when pointers are cast   \
   * to the wrong type. While the cast itself is valid, it is often a bug      \
   * because calling it through the cast is UB. However, we never actually     \
   * call functions as |lhash_cmp_func|. The type is just a type-erased        \
   * function pointer. (C does not guarantee function pointers fit in          \
   * |void*|, and GCC will warn on this.) Thus we just disable the false       \
   * positive warning. */                                                      \
  OPENSSL_MSVC_PRAGMA(warning(push))                                           \
  OPENSSL_MSVC_PRAGMA(warning(disable : 4191))                                 \
                                                                               \
  DECLARE_LHASH_OF(type)                                                       \
                                                                               \
  typedef int (*lhash_##type##_cmp_func)(const type *, const type *);          \
  typedef uint32_t (*lhash_##type##_hash_func)(const type *);                  \
                                                                               \
  inline int lh_##type##_call_cmp_func(lhash_cmp_func func, const void *a,     \
                                       const void *b) {                        \
    return ((lhash_##type##_cmp_func)func)((const type *)a, (const type *)b);  \
  }                                                                            \
                                                                               \
  inline uint32_t lh_##type##_call_hash_func(lhash_hash_func func,             \
                                             const void *a) {                  \
    return ((lhash_##type##_hash_func)func)((const type *)a);                  \
  }                                                                            \
                                                                               \
  inline LHASH_OF(type) *lh_##type##_new(lhash_##type##_hash_func hash,        \
                                         lhash_##type##_cmp_func comp) {       \
    return (LHASH_OF(type) *)OPENSSL_lh_new((lhash_hash_func)hash,             \
                                            (lhash_cmp_func)comp);             \
  }                                                                            \
                                                                               \
  inline void lh_##type##_free(LHASH_OF(type) *lh) {                           \
    OPENSSL_lh_free((_LHASH *)lh);                                             \
  }                                                                            \
                                                                               \
  inline size_t lh_##type##_num_items(const LHASH_OF(type) *lh) {              \
    return OPENSSL_lh_num_items((const _LHASH *)lh);                           \
  }                                                                            \
                                                                               \
  inline type *lh_##type##_retrieve(const LHASH_OF(type) *lh,                  \
                                    const type *data) {                        \
    return (type *)OPENSSL_lh_retrieve((const _LHASH *)lh, data,               \
                                       lh_##type##_call_hash_func,             \
                                       lh_##type##_call_cmp_func);             \
  }                                                                            \
                                                                               \
  typedef struct {                                                             \
    int (*cmp_key)(const void *key, const type *value);                        \
    const void *key;                                                           \
  } LHASH_CMP_KEY_##type;                                                      \
                                                                               \
  inline int lh_##type##_call_cmp_key(const void *key, const void *value) {    \
    const LHASH_CMP_KEY_##type *cb = (const LHASH_CMP_KEY_##type *)key;        \
    return cb->cmp_key(cb->key, (const type *)value);                          \
  }                                                                            \
                                                                               \
  inline type *lh_##type##_retrieve_key(                                       \
      const LHASH_OF(type) *lh, const void *key, uint32_t key_hash,            \
      int (*cmp_key)(const void *key, const type *value)) {                    \
    LHASH_CMP_KEY_##type cb = {cmp_key, key};                                  \
    return (type *)OPENSSL_lh_retrieve_key((const _LHASH *)lh, &cb, key_hash,  \
                                           lh_##type##_call_cmp_key);          \
  }                                                                            \
                                                                               \
  inline int lh_##type##_insert(LHASH_OF(type) *lh, type **old_data,           \
                                type *data) {                                  \
    void *old_data_void = NULL;                                                \
    int ret = OPENSSL_lh_insert((_LHASH *)lh, &old_data_void, data,            \
                                lh_##type##_call_hash_func,                    \
                                lh_##type##_call_cmp_func);                    \
    *old_data = (type *)old_data_void;                                         \
    return ret;                                                                \
  }                                                                            \
                                                                               \
  inline type *lh_##type##_delete(LHASH_OF(type) *lh, const type *data) {      \
    return (type *)OPENSSL_lh_delete((_LHASH *)lh, data,                       \
                                     lh_##type##_call_hash_func,               \
                                     lh_##type##_call_cmp_func);               \
  }                                                                            \
                                                                               \
  typedef struct {                                                             \
    void (*doall_arg)(type *, void *);                                         \
    void *arg;                                                                 \
  } LHASH_DOALL_##type;                                                        \
                                                                               \
  inline void lh_##type##_call_doall_arg(void *value, void *arg) {             \
    const LHASH_DOALL_##type *cb = (const LHASH_DOALL_##type *)arg;            \
    cb->doall_arg((type *)value, cb->arg);                                     \
  }                                                                            \
                                                                               \
  inline void lh_##type##_doall_arg(LHASH_OF(type) *lh,                        \
                                    void (*func)(type *, void *), void *arg) { \
    LHASH_DOALL_##type cb = {func, arg};                                       \
    OPENSSL_lh_doall_arg((_LHASH *)lh, lh_##type##_call_doall_arg, &cb);       \
  }                                                                            \
                                                                               \
  OPENSSL_MSVC_PRAGMA(warning(pop))


#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_CRYPTO_LHASH_INTERNAL_H
