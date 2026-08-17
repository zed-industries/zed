// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef OPENSSL_HEADER_LHASH_H
#define OPENSSL_HEADER_LHASH_H

#include <openssl/base.h>

#if defined(__cplusplus)
extern "C" {
#endif


// Hash table implementation for internal use.


typedef struct lhash_st _LHASH;

// lhash is an internal library and not exported for use outside BoringSSL. This
// header is provided for compatibility with code that expects OpenSSL.


// These two macros are exported for compatibility with existing callers of
// |X509V3_EXT_conf_nid|. Do not use these symbols outside BoringSSL.
#define LHASH_OF(type) struct lhash_st_##type
#define DECLARE_LHASH_OF(type) LHASH_OF(type);

OPENSSL_EXPORT void lh_doall_arg(_LHASH *lh, void (*func)(void *, void *),
                                 void *arg);

// These two macros are the bare minimum of |LHASH| macros downstream consumers
// use.
#define IMPLEMENT_LHASH_DOALL_ARG_FN(name, o_type, a_type) \
	void name##_LHASH_DOALL_ARG(void *arg1, void *arg2) { \
		o_type *a = arg1; \
		a_type *b = arg2; \
		name##_doall_arg(a, b); }
#define LHASH_DOALL_ARG_FN(name) name##_LHASH_DOALL_ARG


#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_LHASH_H
