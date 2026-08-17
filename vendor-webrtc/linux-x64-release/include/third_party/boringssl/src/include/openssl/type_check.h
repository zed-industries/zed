// Copyright 1999-2016 The OpenSSL Project Authors. All Rights Reserved.
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

#ifndef OPENSSL_HEADER_TYPE_CHECK_H
#define OPENSSL_HEADER_TYPE_CHECK_H

#include <openssl/base.h>   // IWYU pragma: export

#if defined(__cplusplus)
extern "C" {
#endif


// CHECKED_CAST casts |p| from type |from| to type |to|.
//
// TODO(davidben): Although this macro is not public API and is unused in
// BoringSSL, wpa_supplicant uses it to define its own stacks. Remove this once
// wpa_supplicant has been fixed.
#define CHECKED_CAST(to, from, p) ((to) (1 ? (p) : (from)0))


#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_TYPE_CHECK_H
