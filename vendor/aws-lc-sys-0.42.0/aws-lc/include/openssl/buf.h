// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef OPENSSL_HEADER_BUFFER_H
#define OPENSSL_HEADER_BUFFER_H

#include <openssl/base.h>

#if defined(__cplusplus)
extern "C" {
#endif


// Memory and string functions, see also mem.h.


// buf_mem_st (aka |BUF_MEM|) is a generic buffer object used by OpenSSL.
struct buf_mem_st {
  size_t length;  // current number of bytes
  char *data;
  size_t max;  // size of buffer
};

// BUF_MEM_new creates a new BUF_MEM which has no allocated data buffer.
OPENSSL_EXPORT BUF_MEM *BUF_MEM_new(void);

// BUF_MEM_free frees |buf->data| if needed and then frees |buf| itself.
OPENSSL_EXPORT void BUF_MEM_free(BUF_MEM *buf);

// BUF_MEM_reserve ensures |buf| has capacity |cap| and allocates memory if
// needed. It returns one on success and zero on error.
OPENSSL_EXPORT int BUF_MEM_reserve(BUF_MEM *buf, size_t cap);

// BUF_MEM_grow ensures that |buf| has length |len| and allocates memory if
// needed. If the length of |buf| increased, the new bytes are filled with
// zeros. It returns the length of |buf|, or zero if there's an error.
OPENSSL_EXPORT size_t BUF_MEM_grow(BUF_MEM *buf, size_t len);

// BUF_MEM_grow_clean calls |BUF_MEM_grow|. BoringSSL always zeros memory
// allocated memory on free.
OPENSSL_EXPORT size_t BUF_MEM_grow_clean(BUF_MEM *buf, size_t len);

// BUF_MEM_append appends |in| to |buf|. It returns one on success and zero on
// error.
OPENSSL_EXPORT int BUF_MEM_append(BUF_MEM *buf, const void *in, size_t len);


// Deprecated functions.

// BUF_strdup calls |OPENSSL_strdup|.
OPENSSL_EXPORT char *BUF_strdup(const char *str);

// BUF_strnlen calls |OPENSSL_strnlen|.
OPENSSL_EXPORT size_t BUF_strnlen(const char *str, size_t max_len);

// BUF_strndup calls |OPENSSL_strndup|.
OPENSSL_EXPORT char *BUF_strndup(const char *str, size_t size);

// BUF_memdup calls |OPENSSL_memdup|.
OPENSSL_EXPORT void *BUF_memdup(const void *data, size_t size);

// BUF_strlcpy calls |OPENSSL_strlcpy|.
OPENSSL_EXPORT size_t BUF_strlcpy(char *dst, const char *src, size_t dst_size);

// BUF_strlcat calls |OPENSSL_strlcat|.
OPENSSL_EXPORT size_t BUF_strlcat(char *dst, const char *src, size_t dst_size);


#if defined(__cplusplus)
}  // extern C

extern "C++" {

BSSL_NAMESPACE_BEGIN

BORINGSSL_MAKE_DELETER(BUF_MEM, BUF_MEM_free)

BSSL_NAMESPACE_END

}  // extern C++

#endif

#endif  // OPENSSL_HEADER_BUFFER_H
