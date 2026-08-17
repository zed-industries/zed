// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/mem.h>

#include <assert.h>
#include <errno.h>
#include <limits.h>
#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>

#include <openssl/err.h>

#if defined(OPENSSL_WINDOWS)
OPENSSL_MSVC_PRAGMA(warning(push, 3))
#include <windows.h>
OPENSSL_MSVC_PRAGMA(warning(pop))
#endif

#include "internal.h"


#define OPENSSL_MALLOC_PREFIX 8
OPENSSL_STATIC_ASSERT(OPENSSL_MALLOC_PREFIX >= sizeof(size_t),
                      size_t_too_large)

#if defined(OPENSSL_ASAN)
void __asan_poison_memory_region(const volatile void *addr, size_t size);
void __asan_unpoison_memory_region(const volatile void *addr, size_t size);
#else
static void __asan_poison_memory_region(const void *addr, size_t size) {}
static void __asan_unpoison_memory_region(const void *addr, size_t size) {}
#endif

#define AWSLC_FILE ""
#define AWSLC_LINE 0

// sdallocx is a sized |free| function. By passing the size (which we happen to
// always know in BoringSSL), the malloc implementation can save work. We cannot
// depend on |sdallocx| being available, however, so it's a weak symbol.
//
// This will always be safe, but will only be overridden if the malloc
// implementation is statically linked with BoringSSL. So, if |sdallocx| is
// provided in, say, libc.so, we still won't use it because that's dynamically
// linked. This isn't an ideal result, but its helps in some cases.
WEAK_SYMBOL_FUNC(void, sdallocx, (void *ptr, size_t size, int flags))

// The following four functions can be defined to override default heap
// allocation and freeing. If defined, it is the responsibility of
// |OPENSSL_memory_free| to zero out the memory before returning it to the
// system. |OPENSSL_memory_free| will not be passed NULL pointers.
//
// WARNING: These functions are called on every allocation and free in
// BoringSSL across the entire process. They may be called by any code in the
// process which calls BoringSSL, including in process initializers and thread
// destructors. When called, BoringSSL may hold pthreads locks. Any other code
// in the process which, directly or indirectly, calls BoringSSL may be on the
// call stack and may itself be using arbitrary synchronization primitives.
//
// As a result, these functions may not have the usual programming environment
// available to most C or C++ code. In particular, they may not call into
// BoringSSL, or any library which depends on BoringSSL. Any synchronization
// primitives used must tolerate every other synchronization primitive linked
// into the process, including pthreads locks. Failing to meet these constraints
// may result in deadlocks, crashes, or memory corruption.
WEAK_SYMBOL_FUNC(void *, OPENSSL_memory_alloc, (size_t size))
WEAK_SYMBOL_FUNC(void, OPENSSL_memory_free, (void *ptr))
WEAK_SYMBOL_FUNC(size_t, OPENSSL_memory_get_size, (void *ptr))
WEAK_SYMBOL_FUNC(void *, OPENSSL_memory_realloc, (void *ptr, size_t size))

// Control function for crypto memory debugging Always returns 0 in AWS-LC
// |mode| Unused parameter AWS-LC doesn't support |CRYPTO_MDEBUG|
// Always returns 0
int CRYPTO_mem_ctrl(int mode) {
  #if defined (OPENSSL_NO_CRYPTO_MDEBUG)
    (void)mode;  // Silence unused parameter warning
    return 0;
  #else
    #error "Must compile with OPENSSL_NO_CRYPTO_MDEBUG defined."
  #endif
}

// Below can be customized by |CRYPTO_set_mem_functions| only once.
static void *(*malloc_impl)(size_t, const char *, int) = NULL;
static void *(*realloc_impl)(void *, size_t, const char *, int) = NULL;
static void (*free_impl)(void *, const char *, int) = NULL;

int CRYPTO_set_mem_functions(
  void *(*m)(size_t, const char *, int),
  void *(*r)(void *, size_t, const char *, int),
  void (*f)(void *, const char *, int)) {
  if (m == NULL || r == NULL || f == NULL) {
    return 0;
  }
  // |malloc_impl|, |realloc_impl| and |free_impl| can be set only once.
  if (malloc_impl != NULL || realloc_impl != NULL || free_impl != NULL) {
    return 0;
  }
  if (OPENSSL_memory_alloc != NULL ||
      OPENSSL_memory_free != NULL ||
      OPENSSL_memory_get_size != NULL ||
      OPENSSL_memory_realloc != NULL) {
    // |OPENSSL_malloc/free/realloc| are customized by overriding the symbols.
    OPENSSL_PUT_ERROR(CRYPTO, ERR_R_SHOULD_NOT_HAVE_BEEN_CALLED);
    return 0;
  }
  malloc_impl = m;
  realloc_impl = r;
  free_impl = f;
  return 1;
}

void *OPENSSL_malloc(size_t size) {
  if (malloc_impl != NULL) {
    assert(OPENSSL_memory_alloc == NULL);
    assert(OPENSSL_memory_realloc == NULL);
    assert(OPENSSL_memory_free == NULL);
    assert(OPENSSL_memory_get_size == NULL);
    assert(realloc_impl != NULL);
    assert(free_impl != NULL);
    return malloc_impl(size, AWSLC_FILE, AWSLC_LINE);
  }
  if (OPENSSL_memory_alloc != NULL) {
    assert(OPENSSL_memory_free != NULL);
    assert(OPENSSL_memory_get_size != NULL);
    void *ptr = OPENSSL_memory_alloc(size);
    if (ptr == NULL && size != 0) {
      goto err;
    }
    return ptr;
  }

  if (size + OPENSSL_MALLOC_PREFIX < size) {
    goto err;
  }

  void *ptr = malloc(size + OPENSSL_MALLOC_PREFIX);
  if (ptr == NULL) {
    goto err;
  }

  *(size_t *)ptr = size;

  __asan_poison_memory_region(ptr, OPENSSL_MALLOC_PREFIX);
  return ((uint8_t *)ptr) + OPENSSL_MALLOC_PREFIX;

 err:
  // This only works because ERR does not call OPENSSL_malloc.
  OPENSSL_PUT_ERROR(CRYPTO, ERR_R_MALLOC_FAILURE);
  return NULL;
}

void *OPENSSL_zalloc(size_t size) {
  void *ret = OPENSSL_malloc(size);
  if (ret != NULL) {
    OPENSSL_memset(ret, 0, size);
  }
  return ret;
}

void *OPENSSL_calloc(size_t num, size_t size) {
  if (size != 0 && num > SIZE_MAX / size) {
    OPENSSL_PUT_ERROR(CRYPTO, ERR_R_OVERFLOW);
    return NULL;
  }

  return OPENSSL_zalloc(num * size);
}

void OPENSSL_free(void *orig_ptr) {
  if (orig_ptr == NULL) {
    return;
  }
  if (free_impl != NULL) {
    assert(OPENSSL_memory_alloc == NULL);
    assert(OPENSSL_memory_realloc == NULL);
    assert(OPENSSL_memory_free == NULL);
    assert(OPENSSL_memory_get_size == NULL);
    assert(malloc_impl != NULL);
    assert(realloc_impl != NULL);
    free_impl(orig_ptr, AWSLC_FILE, AWSLC_LINE);
    return;
  }

  if (OPENSSL_memory_free != NULL) {
    OPENSSL_memory_free(orig_ptr);
    return;
  }

  void *ptr = ((uint8_t *)orig_ptr) - OPENSSL_MALLOC_PREFIX;
  __asan_unpoison_memory_region(ptr, OPENSSL_MALLOC_PREFIX);

  size_t size = *(size_t *)ptr;
  OPENSSL_cleanse(ptr, size + OPENSSL_MALLOC_PREFIX);

// ASan knows to intercept malloc and free, but not sdallocx.
#if defined(OPENSSL_ASAN)
  (void)sdallocx;
  free(ptr);
  (void) sdallocx;
#else
  if (sdallocx) {
    sdallocx(ptr, size + OPENSSL_MALLOC_PREFIX, 0 /* flags */);
  } else {
    free(ptr);
  }
#endif
}

void *OPENSSL_realloc(void *orig_ptr, size_t new_size) {
  if (orig_ptr == NULL) {
    return OPENSSL_malloc(new_size);
  }
  if (realloc_impl != NULL) {
    assert(OPENSSL_memory_alloc == NULL);
    assert(OPENSSL_memory_realloc == NULL);
    assert(OPENSSL_memory_free == NULL);
    assert(OPENSSL_memory_get_size == NULL);
    assert(malloc_impl != NULL);
    assert(free_impl != NULL);
    return realloc_impl(orig_ptr, new_size, AWSLC_FILE, AWSLC_LINE);
  }
  if (OPENSSL_memory_realloc != NULL) {
    assert(OPENSSL_memory_alloc != NULL);
    assert(OPENSSL_memory_free != NULL);
    assert(OPENSSL_memory_get_size != NULL);
    return OPENSSL_memory_realloc(orig_ptr, new_size);
  }
  size_t old_size;
  if (OPENSSL_memory_get_size != NULL) {
    old_size = OPENSSL_memory_get_size(orig_ptr);
  } else {
    void *ptr = ((uint8_t *)orig_ptr) - OPENSSL_MALLOC_PREFIX;
    __asan_unpoison_memory_region(ptr, OPENSSL_MALLOC_PREFIX);
    old_size = *(size_t *)ptr;
    __asan_poison_memory_region(ptr, OPENSSL_MALLOC_PREFIX);
  }

  void *ret = OPENSSL_malloc(new_size);
  if (ret == NULL) {
    return NULL;
  }

  size_t to_copy = new_size;
  if (old_size < to_copy) {
    to_copy = old_size;
  }

  memcpy(ret, orig_ptr, to_copy);
  OPENSSL_free(orig_ptr);

  return ret;
}

void OPENSSL_cleanse(void *ptr, size_t len) {

  if (ptr == NULL || len == 0) {
    return;
  }

#if defined(OPENSSL_WINDOWS)
  SecureZeroMemory(ptr, len);
#else
  OPENSSL_memset(ptr, 0, len);

#if !defined(OPENSSL_NO_ASM)
  /* As best as we can tell, this is sufficient to break any optimisations that
     might try to eliminate "superfluous" memsets. If there's an easy way to
     detect memset_s, it would be better to use that. */
  __asm__ __volatile__("" : : "r"(ptr) : "memory");
#endif
#endif  // !OPENSSL_NO_ASM
}

void OPENSSL_clear_free(void *ptr, size_t unused) { OPENSSL_free(ptr); }

int CRYPTO_secure_malloc_init(size_t size, size_t min_size) { return 0; }

int CRYPTO_secure_malloc_initialized(void) { return 0; }

size_t CRYPTO_secure_used(void) { return 0; }

void *OPENSSL_secure_malloc(size_t size) { return OPENSSL_malloc(size); }

void *OPENSSL_secure_zalloc(size_t size) { return OPENSSL_zalloc(size); }

void OPENSSL_secure_clear_free(void *ptr, size_t len) {
  OPENSSL_clear_free(ptr, len);
}

int CRYPTO_memcmp(const void *in_a, const void *in_b, size_t len) {
  const uint8_t *a = in_a;
  const uint8_t *b = in_b;
  uint8_t x = 0;

  for (size_t i = 0; i < len; i++) {
    x |= a[i] ^ b[i];
  }

  return x;
}

uint32_t OPENSSL_hash32(const void *ptr, size_t len) {
  // These are the FNV-1a parameters for 32 bits.
  static const uint32_t kPrime = 16777619u;
  static const uint32_t kOffsetBasis = 2166136261u;

  const uint8_t *in = ptr;
  uint32_t h = kOffsetBasis;

  for (size_t i = 0; i < len; i++) {
    h ^= in[i];
    h *= kPrime;
  }

  return h;
}

uint32_t OPENSSL_strhash(const char *s) { return OPENSSL_hash32(s, strlen(s)); }

size_t OPENSSL_strnlen(const char *s, size_t len) {
  for (size_t i = 0; i < len; i++) {
    if (s[i] == 0) {
      return i;
    }
  }

  return len;
}

char *OPENSSL_strdup(const char *s) {
  if (s == NULL) {
    return NULL;
  }
  // Copy the NUL terminator.
  return OPENSSL_memdup(s, strlen(s) + 1);
}

int OPENSSL_isalpha(int c) {
  return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z');
}

int OPENSSL_isdigit(int c) { return c >= '0' && c <= '9'; }

int OPENSSL_isxdigit(int c) {
  return OPENSSL_isdigit(c) || (c >= 'a' && c <= 'f') || (c >= 'A' && c <= 'F');
}

int OPENSSL_fromxdigit(uint8_t *out, int c) {
  if (OPENSSL_isdigit(c)) {
    *out = c - '0';
    return 1;
  }
  if ('a' <= c && c <= 'f') {
    *out = c - 'a' + 10;
    return 1;
  }
  if ('A' <= c && c <= 'F') {
    *out = c - 'A' + 10;
    return 1;
  }
  return 0;
}

uint8_t *OPENSSL_hexstr2buf(const char *str, size_t *len) {
  if (str == NULL || len == NULL) {
    return NULL;
  }

  const size_t slen = OPENSSL_strnlen(str, INT16_MAX);
  if (slen % 2 != 0) {
    return NULL;
  }

  const size_t buflen = slen / 2;
  uint8_t *buf = OPENSSL_zalloc(buflen);
  if (buf == NULL) {
    return NULL;
  }

  for (size_t i = 0; i < buflen; i++) {
    uint8_t hi, lo;
    if (!OPENSSL_fromxdigit(&hi, str[2 * i]) ||
        !OPENSSL_fromxdigit(&lo, str[2 * i + 1])) {
      OPENSSL_free(buf);
      return NULL;
    }
    buf[i] = (hi << 4) | lo;
  }

  *len = buflen;
  return buf;
}

int OPENSSL_isalnum(int c) { return OPENSSL_isalpha(c) || OPENSSL_isdigit(c); }

int OPENSSL_tolower(int c) {
  if (c >= 'A' && c <= 'Z') {
    return c + ('a' - 'A');
  }
  return c;
}

int OPENSSL_isspace(int c) {
  return c == '\t' || c == '\n' || c == '\v' || c == '\f' || c == '\r' ||
         c == ' ';
}

int OPENSSL_strcasecmp(const char *a, const char *b) {
  for (size_t i = 0;; i++) {
    const int aa = OPENSSL_tolower(a[i]);
    const int bb = OPENSSL_tolower(b[i]);

    if (aa < bb) {
      return -1;
    } else if (aa > bb) {
      return 1;
    } else if (aa == 0) {
      return 0;
    }
  }
}

int OPENSSL_strncasecmp(const char *a, const char *b, size_t n) {
  for (size_t i = 0; i < n; i++) {
    const int aa = OPENSSL_tolower(a[i]);
    const int bb = OPENSSL_tolower(b[i]);

    if (aa < bb) {
      return -1;
    } else if (aa > bb) {
      return 1;
    } else if (aa == 0) {
      return 0;
    }
  }

  return 0;
}

int BIO_snprintf(char *buf, size_t n, const char *format, ...) {
  va_list args;
  va_start(args, format);
  int ret = BIO_vsnprintf(buf, n, format, args);
  va_end(args);
  return ret;
}

int BIO_vsnprintf(char *buf, size_t n, const char *format, va_list args) {
  return vsnprintf(buf, n, format, args);
}

int OPENSSL_vasprintf_internal(char **str, const char *format, va_list args,
                               int system_malloc) {
  void *(*allocate)(size_t) = system_malloc ? malloc : OPENSSL_malloc;
  void (*deallocate)(void *) = system_malloc ? free : OPENSSL_free;
  void *(*reallocate)(void *, size_t) =
      system_malloc ? realloc : OPENSSL_realloc;
  char *candidate = NULL;
  size_t candidate_len = 64;  // TODO(bbe) what's the best initial size?

  if ((candidate = allocate(candidate_len)) == NULL) {
    goto err;
  }
  va_list args_copy;
  va_copy(args_copy, args);
  int ret = vsnprintf(candidate, candidate_len, format, args_copy);
  va_end(args_copy);
  if (ret < 0) {
    goto err;
  }
  if ((size_t)ret >= candidate_len) {
    // Too big to fit in allocation.
    char *tmp;

    candidate_len = (size_t)ret + 1;
    if ((tmp = reallocate(candidate, candidate_len)) == NULL) {
      goto err;
    }
    candidate = tmp;
    ret = vsnprintf(candidate, candidate_len, format, args);
  }
  // At this point this should not happen unless vsnprintf is insane.
  if (ret < 0 || (size_t)ret >= candidate_len) {
    goto err;
  }
  *str = candidate;
  return ret;

 err:
  deallocate(candidate);
  *str = NULL;
  errno = ENOMEM;
  return -1;
}

int OPENSSL_vasprintf(char **str, const char *format, va_list args) {
  return OPENSSL_vasprintf_internal(str, format, args, /*system_malloc=*/0);
}

int OPENSSL_asprintf(char **str, const char *format, ...) {
  va_list args;
  va_start(args, format);
  int ret = OPENSSL_vasprintf(str, format, args);
  va_end(args);
  return ret;
}

char *OPENSSL_strndup(const char *str, size_t size) {
  size = OPENSSL_strnlen(str, size);

  size_t alloc_size = size + 1;
  if (alloc_size < size) {
    // overflow
    OPENSSL_PUT_ERROR(CRYPTO, ERR_R_MALLOC_FAILURE);
    return NULL;
  }
  char *ret = OPENSSL_malloc(alloc_size);
  if (ret == NULL) {
    return NULL;
  }

  OPENSSL_memcpy(ret, str, size);
  ret[size] = '\0';
  return ret;
}

size_t OPENSSL_strlcpy(char *dst, const char *src, size_t dst_size) {
  size_t l = 0;

  for (; dst_size > 1 && *src; dst_size--) {
    *dst++ = *src++;
    l++;
  }

  if (dst_size) {
    *dst = 0;
  }

  return l + strlen(src);
}

size_t OPENSSL_strlcat(char *dst, const char *src, size_t dst_size) {
  size_t l = 0;
  for (; dst_size > 0 && *dst; dst_size--, dst++) {
    l++;
  }
  return l + OPENSSL_strlcpy(dst, src, dst_size);
}

void *OPENSSL_memdup(const void *data, size_t size) {
  if (size == 0) {
    return NULL;
  }

  void *ret = OPENSSL_malloc(size);
  if (ret == NULL) {
    return NULL;
  }

  OPENSSL_memcpy(ret, data, size);
  return ret;
}

void *CRYPTO_malloc(size_t size, const char *file, int line) {
  return OPENSSL_malloc(size);
}

void *CRYPTO_realloc(void *ptr, size_t new_size, const char *file, int line) {
  return OPENSSL_realloc(ptr, new_size);
}

void CRYPTO_free(void *ptr, const char *file, int line) { OPENSSL_free(ptr); }
