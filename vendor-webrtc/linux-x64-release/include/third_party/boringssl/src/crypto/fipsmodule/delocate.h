// Copyright 2017 The BoringSSL Authors
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

#ifndef OPENSSL_HEADER_CRYPTO_FIPSMODULE_DELOCATE_H
#define OPENSSL_HEADER_CRYPTO_FIPSMODULE_DELOCATE_H

#include <openssl/base.h>

#include "../internal.h"


#if !defined(BORINGSSL_SHARED_LIBRARY) && defined(BORINGSSL_FIPS) && \
    !defined(OPENSSL_ASAN) && !defined(OPENSSL_MSAN)
#define DEFINE_BSS_GET(type, name, init_value)                                 \
  /* delocate needs C linkage and for |name| to be unique across BCM. */       \
  extern "C" {                                                                 \
  extern type bcm_##name;                                                      \
  type bcm_##name = init_value;                                                \
  type *bcm_##name##_bss_get(void) __attribute__((const));                     \
  } /* extern "C" */                                                           \
                                                                               \
  /* The getter functions are exported, but static variables are usually named \
   * with short names. Define a static wrapper function so the caller can use  \
   * a short name, while the symbol itself is prefixed. */                     \
  static type *name##_bss_get(void) { return bcm_##name##_bss_get(); }
// For FIPS builds we require that CRYPTO_ONCE_INIT be zero.
#define DEFINE_STATIC_ONCE(name) \
  DEFINE_BSS_GET(CRYPTO_once_t, name, CRYPTO_ONCE_INIT)
// For FIPS builds we require that CRYPTO_MUTEX_INIT be zero.
#define DEFINE_STATIC_MUTEX(name) \
  DEFINE_BSS_GET(CRYPTO_MUTEX, name, CRYPTO_MUTEX_INIT)
// For FIPS builds we require that CRYPTO_EX_DATA_CLASS_INIT be zero.
#define DEFINE_STATIC_EX_DATA_CLASS(name) \
  DEFINE_BSS_GET(CRYPTO_EX_DATA_CLASS, name, CRYPTO_EX_DATA_CLASS_INIT)
#else
#define DEFINE_BSS_GET(type, name, init_value) \
  static type name = init_value;               \
  static type *name##_bss_get(void) { return &name; }
#define DEFINE_STATIC_ONCE(name)                \
  static CRYPTO_once_t name = CRYPTO_ONCE_INIT; \
  static CRYPTO_once_t *name##_bss_get(void) { return &name; }
#define DEFINE_STATIC_MUTEX(name)               \
  static CRYPTO_MUTEX name = CRYPTO_MUTEX_INIT; \
  static CRYPTO_MUTEX *name##_bss_get(void) { return &name; }
#define DEFINE_STATIC_EX_DATA_CLASS(name)                       \
  static CRYPTO_EX_DATA_CLASS name = CRYPTO_EX_DATA_CLASS_INIT; \
  static CRYPTO_EX_DATA_CLASS *name##_bss_get(void) { return &name; }
#endif

#define DEFINE_DATA(type, name, accessor_decorations)                         \
  DEFINE_BSS_GET(type, name##_storage, {})                                    \
  DEFINE_STATIC_ONCE(name##_once)                                             \
  static void name##_do_init(type *out);                                      \
  static void name##_init(void) { name##_do_init(name##_storage_bss_get()); } \
  accessor_decorations type *name(void) {                                     \
    CRYPTO_once(name##_once_bss_get(), name##_init);                          \
    /* See http://c-faq.com/ansi/constmismatch.html for why the following     \
     * cast is needed. */                                                     \
    return (const type *)name##_storage_bss_get();                            \
  }                                                                           \
  static void name##_do_init(type *out)

// DEFINE_METHOD_FUNCTION defines a function named |name| which returns a
// method table of type const |type|*. In FIPS mode, to avoid rel.ro data, it
// is split into a CRYPTO_once_t-guarded initializer in the module and
// unhashed, non-module accessor functions to space reserved in the BSS. The
// method table is initialized by a caller-supplied function which takes a
// parameter named |out| of type |type|*. The caller should follow the macro
// invocation with the body of this function:
//
//     DEFINE_METHOD_FUNCTION(EVP_MD, EVP_md4) {
//       out->type = NID_md4;
//       out->md_size = MD4_DIGEST_LENGTH;
//       out->flags = 0;
//       out->init = md4_init;
//       out->update = md4_update;
//       out->final = md4_final;
//       out->block_size = 64;
//       out->ctx_size = sizeof(MD4_CTX);
//     }
//
// This mechanism does not use a static initializer because their execution
// order is undefined. See FIPS.md for more details.
#define DEFINE_METHOD_FUNCTION(type, name) DEFINE_DATA(type, name, const)

#define DEFINE_LOCAL_DATA(type, name) DEFINE_DATA(type, name, static const)

#endif  // OPENSSL_HEADER_CRYPTO_FIPSMODULE_DELOCATE_H
