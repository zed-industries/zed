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

#ifndef OPENSSL_HEADER_MEM_H
#define OPENSSL_HEADER_MEM_H

#include <openssl/base.h>   // IWYU pragma: export

#include <stdlib.h>
#include <stdarg.h>

#if defined(__cplusplus)
extern "C" {
#endif


// Memory and string functions, see also buf.h.
//
// BoringSSL has its own set of allocation functions, which keep track of
// allocation lengths and zero them out before freeing. All memory returned by
// BoringSSL API calls must therefore generally be freed using |OPENSSL_free|
// unless stated otherwise.


#ifndef _BORINGSSL_PROHIBIT_OPENSSL_MALLOC
// OPENSSL_malloc is similar to a regular |malloc|, but allocates additional
// private data. The resulting pointer must be freed with |OPENSSL_free|. In
// the case of a malloc failure, prior to returning NULL |OPENSSL_malloc| will
// push |ERR_R_MALLOC_FAILURE| onto the openssl error stack.
OPENSSL_EXPORT void *OPENSSL_malloc(size_t size);

// OPENSSL_zalloc behaves like |OPENSSL_malloc| except it also initializes the
// resulting memory to zero.
OPENSSL_EXPORT void *OPENSSL_zalloc(size_t size);

// OPENSSL_calloc is similar to a regular |calloc|, but allocates data with
// |OPENSSL_malloc|. On overflow, it will push |ERR_R_OVERFLOW| onto the error
// queue.
OPENSSL_EXPORT void *OPENSSL_calloc(size_t num, size_t size);

// OPENSSL_realloc returns a pointer to a buffer of |new_size| bytes that
// contains the contents of |ptr|. Unlike |realloc|, a new buffer is always
// allocated and the data at |ptr| is always wiped and freed. Memory is
// allocated with |OPENSSL_malloc| and must be freed with |OPENSSL_free|.
OPENSSL_EXPORT void *OPENSSL_realloc(void *ptr, size_t new_size);
#endif // !_BORINGSSL_PROHIBIT_OPENSSL_MALLOC

// OPENSSL_free does nothing if |ptr| is NULL. Otherwise it zeros out the
// memory allocated at |ptr| and frees it along with the private data.
// It must only be used on on |ptr| values obtained from |OPENSSL_malloc|
OPENSSL_EXPORT void OPENSSL_free(void *ptr);

// OPENSSL_cleanse zeros out |len| bytes of memory at |ptr|. This is similar to
// |memset_s| from C11.
OPENSSL_EXPORT void OPENSSL_cleanse(void *ptr, size_t len);

// CRYPTO_memcmp returns zero iff the |len| bytes at |a| and |b| are equal. It
// takes an amount of time dependent on |len|, but independent of the contents
// of |a| and |b|. Unlike memcmp, it cannot be used to put elements into a
// defined order as the return value when a != b is undefined, other than to be
// non-zero.
OPENSSL_EXPORT int CRYPTO_memcmp(const void *a, const void *b, size_t len);

// OPENSSL_hash32 implements the 32 bit, FNV-1a hash.
OPENSSL_EXPORT uint32_t OPENSSL_hash32(const void *ptr, size_t len);

// OPENSSL_strhash calls |OPENSSL_hash32| on the NUL-terminated string |s|.
OPENSSL_EXPORT uint32_t OPENSSL_strhash(const char *s);

// OPENSSL_strdup has the same behaviour as strdup(3).
OPENSSL_EXPORT char *OPENSSL_strdup(const char *s);

// OPENSSL_strnlen has the same behaviour as strnlen(3).
OPENSSL_EXPORT size_t OPENSSL_strnlen(const char *s, size_t len);

// OPENSSL_isalpha is a locale-independent, ASCII-only version of isalpha(3), It
// only recognizes 'a' through 'z' and 'A' through 'Z' as alphabetic.
OPENSSL_EXPORT int OPENSSL_isalpha(int c);

// OPENSSL_isdigit is a locale-independent, ASCII-only version of isdigit(3), It
// only recognizes '0' through '9' as digits.
OPENSSL_EXPORT int OPENSSL_isdigit(int c);

// OPENSSL_isxdigit is a locale-independent, ASCII-only version of isxdigit(3),
// It only recognizes '0' through '9', 'a' through 'f', and 'A through 'F' as
// digits.
OPENSSL_EXPORT int OPENSSL_isxdigit(int c);

// OPENSSL_fromxdigit returns one if |c| is a hexadecimal digit as recognized
// by OPENSSL_isxdigit, and sets |out| to the corresponding value. Otherwise
// zero is returned.
OPENSSL_EXPORT int OPENSSL_fromxdigit(uint8_t *out, int c);

// OPENSSL_isalnum is a locale-independent, ASCII-only version of isalnum(3), It
// only recognizes what |OPENSSL_isalpha| and |OPENSSL_isdigit| recognize.
OPENSSL_EXPORT int OPENSSL_isalnum(int c);

// OPENSSL_tolower is a locale-independent, ASCII-only version of tolower(3). It
// only lowercases ASCII values. Other values are returned as-is.
OPENSSL_EXPORT int OPENSSL_tolower(int c);

// OPENSSL_isspace is a locale-independent, ASCII-only version of isspace(3). It
// only recognizes '\t', '\n', '\v', '\f', '\r', and ' '.
OPENSSL_EXPORT int OPENSSL_isspace(int c);

// OPENSSL_strcasecmp is a locale-independent, ASCII-only version of
// strcasecmp(3).
OPENSSL_EXPORT int OPENSSL_strcasecmp(const char *a, const char *b);

// OPENSSL_strncasecmp is a locale-independent, ASCII-only version of
// strncasecmp(3).
OPENSSL_EXPORT int OPENSSL_strncasecmp(const char *a, const char *b, size_t n);

// DECIMAL_SIZE returns an upper bound for the length of the decimal
// representation of the given type.
#define DECIMAL_SIZE(type)	((sizeof(type)*8+2)/3+1)

// BIO_snprintf has the same behavior as snprintf(3).
OPENSSL_EXPORT int BIO_snprintf(char *buf, size_t n, const char *format, ...)
    OPENSSL_PRINTF_FORMAT_FUNC(3, 4);

// BIO_vsnprintf has the same behavior as vsnprintf(3).
OPENSSL_EXPORT int BIO_vsnprintf(char *buf, size_t n, const char *format,
                                 va_list args) OPENSSL_PRINTF_FORMAT_FUNC(3, 0);

// OPENSSL_vasprintf has the same behavior as vasprintf(3), except that
// memory allocated in a returned string must be freed with |OPENSSL_free|.
OPENSSL_EXPORT int OPENSSL_vasprintf(char **str, const char *format,
                                     va_list args)
    OPENSSL_PRINTF_FORMAT_FUNC(2, 0);

// OPENSSL_asprintf has the same behavior as asprintf(3), except that
// memory allocated in a returned string must be freed with |OPENSSL_free|.
OPENSSL_EXPORT int OPENSSL_asprintf(char **str, const char *format, ...)
    OPENSSL_PRINTF_FORMAT_FUNC(2, 3);

// OPENSSL_strndup returns an allocated, duplicate of |str|, which is, at most,
// |size| bytes. The result is always NUL terminated. The memory allocated
// must be freed with |OPENSSL_free|.
OPENSSL_EXPORT char *OPENSSL_strndup(const char *str, size_t size);

// OPENSSL_memdup returns an allocated, duplicate of |size| bytes from |data| or
// NULL on allocation failure. The memory allocated must be freed with
// |OPENSSL_free|.
OPENSSL_EXPORT void *OPENSSL_memdup(const void *data, size_t size);

// OPENSSL_strlcpy acts like strlcpy(3).
OPENSSL_EXPORT size_t OPENSSL_strlcpy(char *dst, const char *src,
                                      size_t dst_size);

// OPENSSL_strlcat acts like strlcat(3).
OPENSSL_EXPORT size_t OPENSSL_strlcat(char *dst, const char *src,
                                      size_t dst_size);


// Deprecated functions.

// CRYPTO_malloc calls |OPENSSL_malloc|. |file| and |line| are ignored.
OPENSSL_EXPORT void *CRYPTO_malloc(size_t size, const char *file, int line);

// CRYPTO_realloc calls |OPENSSL_realloc|. |file| and |line| are ignored.
OPENSSL_EXPORT void *CRYPTO_realloc(void *ptr, size_t new_size,
                                    const char *file, int line);

// CRYPTO_free calls |OPENSSL_free|. |file| and |line| are ignored.
OPENSSL_EXPORT void CRYPTO_free(void *ptr, const char *file, int line);

// OPENSSL_clear_free calls |OPENSSL_free|. BoringSSL automatically clears all
// allocations on free, but we define |OPENSSL_clear_free| for compatibility.
OPENSSL_EXPORT void OPENSSL_clear_free(void *ptr, size_t len);

// CRYPTO_secure_malloc_init returns zero.
OPENSSL_EXPORT int CRYPTO_secure_malloc_init(size_t size, size_t min_size);

// CRYPTO_secure_malloc_initialized returns zero.
OPENSSL_EXPORT int CRYPTO_secure_malloc_initialized(void);

// CRYPTO_secure_used returns zero.
OPENSSL_EXPORT size_t CRYPTO_secure_used(void);

// OPENSSL_secure_malloc calls |OPENSSL_malloc|.
OPENSSL_EXPORT void *OPENSSL_secure_malloc(size_t size);

// OPENSSL_secure_clear_free calls |OPENSSL_clear_free|.
OPENSSL_EXPORT void OPENSSL_secure_clear_free(void *ptr, size_t len);


#if defined(__cplusplus)
}  // extern C

extern "C++" {

BSSL_NAMESPACE_BEGIN

BORINGSSL_MAKE_DELETER(char, OPENSSL_free)
BORINGSSL_MAKE_DELETER(uint8_t, OPENSSL_free)

BSSL_NAMESPACE_END

}  // extern C++

#endif

#endif  // OPENSSL_HEADER_MEM_H
