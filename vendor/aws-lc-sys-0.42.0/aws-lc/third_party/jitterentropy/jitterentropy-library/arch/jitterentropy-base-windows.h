/*
 * Non-physical true random number generator based on timing jitter.
 *
 * Copyright Stephan Mueller <smueller@chronox.de>, 2013 - 2025
 *
 * License
 * =======
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, and the entire permission notice in its entirety,
 *    including the disclaimer of warranties.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. The name of the author may not be used to endorse or promote
 *    products derived from this software without specific prior
 *    written permission.
 *
 * ALTERNATIVELY, this product may be distributed under the terms of
 * the GNU General Public License, in which case the provisions of the GPL are
 * required INSTEAD OF the above restrictions.  (This clause is
 * necessary due to a potential bad interaction between the GPL and
 * the restrictions contained in a BSD-style copyright.)
 *
 * THIS SOFTWARE IS PROVIDED ``AS IS'' AND ANY EXPRESS OR IMPLIED
 * WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES
 * OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE, ALL OF
 * WHICH ARE HEREBY DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT
 * OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR
 * BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
 * LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
 e USE OF THIS SOFTWARE, EVEN IF NOT ADVISED OF THE POSSIBILITY OF SUCH
 * DAMAGE.
 */

#ifndef _JITTERENTROPY_BASE_X86_H
#define _JITTERENTROPY_BASE_X86_H

#include <stdint.h>

#if defined(_MSC_VER)
typedef int64_t ssize_t;
#include <windows.h>
#endif

#include <stdlib.h>
#include <string.h>
#include <intrin.h>

#if defined(AWSLC)
#include <openssl/crypto.h>
#endif

#if defined(_M_ARM) || defined(_M_ARM64)
#include <profileapi.h>
#include <windows.h>
#endif

static inline void jent_get_nstime(uint64_t *out)
{
#if defined(_M_ARM) || defined(_M_ARM64)

	/* Generic code. */
	LARGE_INTEGER ticks;
	QueryPerformanceCounter(&ticks);
	*out = ticks.QuadPart;

#else

       /* x86, x86_64 intrinsic */
	*out = __rdtsc();

#endif
}

static inline void *jent_zalloc(size_t len)
{
	void *tmp = NULL;
	/* we have no secure memory allocation! Hence
	 * we do not sed CRYPTO_CPU_JITTERENTROPY_SECURE_MEMORY */
#if defined(AWSLC)
	tmp = OPENSSL_malloc(len);
#else
	tmp = malloc(len);
#endif
	if(NULL != tmp)
		memset(tmp, 0, len);
	return tmp;
}

static inline void jent_zfree(void *ptr, unsigned int len)
{
#if defined(AWSLC)
	(void) len;
	OPENSSL_free(ptr);
#else
	memset(ptr, 0, len);
	free(ptr);
#endif
}

static inline int jent_fips_enabled(void)
{
#if defined(AWSLC)
	return FIPS_mode();
#else
	return 0;
#endif
}

static inline void jent_memset_secure(void *s, size_t n)
{
#if defined(AWSLC)
	OPENSSL_cleanse(s, n);
#else
	SecureZeroMemory(s, n);
#endif
}

static inline long jent_ncpu(void)
{
	/*
	 * TODO: return number of available CPUs -
	 * this code disables timer thread as only one CPU is "detected".
	 */
	return 1;
}

static inline void jent_yield(void) { }

static inline uint32_t jent_cache_size_roundup(void)
{
	return 0;
}

/* --- helpers needed in user space -- */

/* note: these helper functions are shamelessly stolen from the kernel :-) */

static inline uint64_t rol64(uint64_t word, unsigned int shift)
{
	return (word << shift) | (word >> (64 - shift));
}

#endif /* _JITTERENTROPY_BASE_X86_H */

