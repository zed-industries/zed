// Copyright 2014 The BoringSSL Authors
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

#ifndef OPENSSL_HEADER_ENGINE_H
#define OPENSSL_HEADER_ENGINE_H

#include <openssl/base.h>   // IWYU pragma: export

#if defined(__cplusplus)
extern "C" {
#endif


// Engines are collections of methods. Methods are tables of function pointers,
// defined for certain algorithms, that allow operations on those algorithms to
// be overridden via a callback. This can be used, for example, to implement an
// RSA* that forwards operations to a hardware module.
//
// Methods are reference counted but |ENGINE|s are not. When creating a method,
// you should zero the whole structure and fill in the function pointers that
// you wish before setting it on an |ENGINE|. Any functions pointers that
// are NULL indicate that the default behaviour should be used.


// Allocation and destruction.

// ENGINE_new returns an empty ENGINE that uses the default method for all
// algorithms.
OPENSSL_EXPORT ENGINE *ENGINE_new(void);

// ENGINE_free decrements the reference counts for all methods linked from
// |engine| and frees |engine| itself. It returns one.
OPENSSL_EXPORT int ENGINE_free(ENGINE *engine);


// Method accessors.
//
// Method accessors take a method pointer and the size of the structure. The
// size allows for ABI compatibility in the case that the method structure is
// extended with extra elements at the end. Methods are always copied by the
// set functions.
//
// Set functions return one on success and zero on allocation failure.

OPENSSL_EXPORT int ENGINE_set_RSA_method(ENGINE *engine,
                                         const RSA_METHOD *method,
                                         size_t method_size);
OPENSSL_EXPORT RSA_METHOD *ENGINE_get_RSA_method(const ENGINE *engine);

OPENSSL_EXPORT int ENGINE_set_ECDSA_method(ENGINE *engine,
                                           const ECDSA_METHOD *method,
                                           size_t method_size);
OPENSSL_EXPORT ECDSA_METHOD *ENGINE_get_ECDSA_method(const ENGINE *engine);


// Generic method functions.
//
// These functions take a void* type but actually operate on all method
// structures.

// METHOD_ref increments the reference count of |method|. This is a no-op for
// now because all methods are currently static.
void METHOD_ref(void *method);

// METHOD_unref decrements the reference count of |method| and frees it if the
// reference count drops to zero. This is a no-op for now because all methods
// are currently static.
void METHOD_unref(void *method);


// Private functions.

// openssl_method_common_st contains the common part of all method structures.
// This must be the first member of all method structures.
struct openssl_method_common_st {
  int references;  // dummy – not used.
  char is_static;
};


#if defined(__cplusplus)
}  // extern C

extern "C++" {

BSSL_NAMESPACE_BEGIN

BORINGSSL_MAKE_DELETER(ENGINE, ENGINE_free)

BSSL_NAMESPACE_END

}  // extern C++

#endif

#define ENGINE_R_OPERATION_NOT_SUPPORTED 100

#endif  // OPENSSL_HEADER_ENGINE_H
