// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef OPENSSL_HEADER_TYPE_CHECK_H
#define OPENSSL_HEADER_TYPE_CHECK_H

#include <openssl/base.h>

#if defined(__cplusplus)
extern "C" {
#endif

// CHECKED_CAST casts |p| from type |from| to type |to|.
//
// TODO(davidben): Although this macro is not public API and is unused in
// BoringSSL, wpa_supplicant uses it to define its own stacks. Remove this once
// wpa_supplicant has been fixed.
#define CHECKED_CAST(to, from, p) ((to) (1 ? (p) : (from)0))

// CHECKED_PTR_OF casts a given pointer to void* and statically checks that it
// was a pointer to |type|.
#define CHECKED_PTR_OF(type, p) CHECKED_CAST(void*, type*, (p))

#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_TYPE_CHECK_H
