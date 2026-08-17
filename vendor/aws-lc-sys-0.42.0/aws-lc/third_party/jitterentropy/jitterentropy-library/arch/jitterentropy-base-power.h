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
 * USE OF THIS SOFTWARE, EVEN IF NOT ADVISED OF THE POSSIBILITY OF SUCH
 * DAMAGE.
 */

#ifndef _JITTERENTROPY_BASE_POWER_H
#define _JITTERENTROPY_BASE_POWER_H

#include <sched.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

/* taken from http://www.ecrypt.eu.org/ebats/cpucycles.html */

static inline void jent_get_nstime(uint64_t *out)
{
	unsigned long high;
	unsigned long low;
	unsigned long newhigh;
	uint64_t result;
        __asm__ __volatile__(
		"Lcpucycles:mftbu %0;mftb %1;mftbu %2;cmpw %0,%2;bne Lcpucycles"
		: "=r" (high), "=r" (low), "=r" (newhigh)
		);
	result = high;
	result <<= 32;
	result |= low;
	*out = result;
}

static inline void *jent_zalloc(size_t len)
{
	void *tmp = NULL;
	/* we have no secure memory allocation! Hence
	 * we do not sed CRYPTO_CPU_JITTERENTROPY_SECURE_MEMORY */
	tmp = malloc(len);
	if(NULL != tmp)
		memset(tmp, 0, len);
	return tmp;
}

static inline void jent_zfree(void *ptr, unsigned int len)
{
	memset(ptr, 0, len);
	free(ptr);
}

static inline int jent_fips_enabled(void)
{
        return 0;
}

static inline long jent_ncpu(void)
{
	/*
	 * TODO: return number of available CPUs -
	 * this code disables timer thread as only one CPU is "detected".
	 */
	return 1;
}

static inline void jent_yield(void)
{
	sched_yield();
}

static inline uint32_t jent_cache_size_roundup(void)
{
#ifdef __linux__
	long l1 = sysconf(_SC_LEVEL1_DCACHE_SIZE);
	long l2 = sysconf(_SC_LEVEL2_CACHE_SIZE);
	long l3 = sysconf(_SC_LEVEL3_CACHE_SIZE);
	uint32_t cache_size = 0;

	/* Cache size reported by system */
	if (l1 > 0)
		cache_size += (uint32_t)l1;
	if (l2 > 0)
		cache_size += (uint32_t)l2;
	if (l3 > 0)
		cache_size += (uint32_t)l3;

	/* Force the output_size to be of the form (bounding_power_of_2 - 1). */
	cache_size |= (cache_size >> 1);
	cache_size |= (cache_size >> 2);
	cache_size |= (cache_size >> 4);
	cache_size |= (cache_size >> 8);
	cache_size |= (cache_size >> 16);

	if (cache_size == 0)
		return 0;

	/* Make the output_size the smallest power of 2 strictly greater than cache_size. */
	cache_size++;

	return cache_size;
#else
	return 0;
#endif
}

/* --- helpers needed in user space -- */

static inline uint64_t rol64(uint64_t x, int n)
{
	return ( (x << (n&(64-1))) | (x >> ((64-n)&(64-1))) );
}

#endif /* _JITTERENTROPY_BASE_POWER_H */

