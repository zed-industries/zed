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

#ifndef _JITTERENTROPY_BASE_USER_H
#define _JITTERENTROPY_BASE_USER_H

/*
 * Set the following defines as needed for your environment
 * Compilation for AWS-LC     #define AWSLC
 * Compilation for libgcrypt  #define LIBGCRYPT
 * Compilation for OpenSSL    #define OPENSSL
 */

#include <sys/types.h>
#include <sys/stat.h>

#include <limits.h>
#include <time.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <fcntl.h>
#include <unistd.h>
#include <errno.h>
#include <sched.h>

/* Timer-less entropy source */
#ifdef JENT_CONF_ENABLE_INTERNAL_TIMER
#include <pthread.h>
#endif /* JENT_CONF_ENABLE_INTERNAL_TIMER */

#ifdef LIBGCRYPT
#include <config.h>
#include "g10lib.h"
#endif

#ifdef OPENSSL
#include <openssl/crypto.h>
#ifdef OPENSSL_FIPS
#include <openssl/fips.h>
#endif
#endif

#if defined(AWSLC)
#include <openssl/crypto.h>
#endif

#ifdef __MACH__
#include <assert.h>
#include <CoreServices/CoreServices.h>
#include <mach/mach.h>
#include <mach/mach_time.h>
#include <unistd.h>
#endif

#if (__x86_64__) || (__i386__)
/* Support rdtsc read on 64-bit and 32-bit x86 architectures */

#ifdef __x86_64__
/* specify 64 bit type since long is 32 bits in LLP64 x86_64 systems */
# define DECLARE_ARGS(val, low, high)    uint64_t low, high
# define EAX_EDX_VAL(val, low, high)     ((low) | (high) << 32)
# define EAX_EDX_RET(val, low, high)     "=a" (low), "=d" (high)
#elif __i386__
# define DECLARE_ARGS(val, low, high)    unsigned long val
# define EAX_EDX_VAL(val, low, high)     val
# define EAX_EDX_RET(val, low, high)     "=A" (val)
#endif

static inline void jent_get_nstime(uint64_t *out)
{
	DECLARE_ARGS(val, low, high);
#ifdef __sun__
	__asm("rdtsc" : EAX_EDX_RET(val, low, high));
#else
	__asm__ __volatile__("rdtsc" : EAX_EDX_RET(val, low, high));
#endif
	*out = EAX_EDX_VAL(val, low, high);
}

#elif defined(__aarch64__)

#ifndef AARCH64_NSTIME_REGISTER
#define AARCH64_NSTIME_REGISTER "cntvct_el0"
#endif

static inline void jent_get_nstime(uint64_t *out)
{
        uint64_t ctr_val;
#if !defined(__MACH__)
        /*
         * Use the system counter for aarch64 (64 bit ARM)...
         */
        __asm__ __volatile__("mrs %0, " AARCH64_NSTIME_REGISTER : "=r" (ctr_val));
#else
        /*
         * Except on modern Apple platforms. Especially on M1 generation Arm64
         * CPUs, the system counter is too coarse. Instead, use
         * clock_gettime_nsec_np(CLOCK_UPTIME_RAW), that is equivalent to
         * march_absolute_time(), but scaled to nanoseconds. See e.g.
         * https://www.manpagez.com/man/3/clock_gettime_nsec_np/.
         */
        ctr_val = clock_gettime_nsec_np(CLOCK_UPTIME_RAW);
#endif
        *out = ctr_val;
}

#elif defined(__s390x__)

static inline void jent_get_nstime(uint64_t *out)
{
	/*
	 * This is MVS+STCK code! Enable it with -S in the compiler.
	 *
	 * uint64_t clk;
	 * __asm__ __volatile__("stck %0" : "=m" (clk) : : "cc");
	 * *out = (uint64_t)(clk);
	 */

	/*
	 * This is GCC+STCKE code. STCKE command and data format:
	 * z/Architecture - Principles of Operation
	 * http://publibz.boulder.ibm.com/epubs/pdf/dz9zr007.pdf
	 *
	 * The current value of bits 0-103 of the TOD clock is stored in bytes
	 * 1-13 of the sixteen-byte output:
	 *
	 * bits 0-7: zeros (reserved for future extention)
	 * bits 8-111: TOD Clock value
	 * bits 112-127: Programmable Field
	 *
	 * Output bit 59 (TOD-Clock bit 51) effectively increments every
	 * microsecond. Bits 60 to 111 of STCKE output are fractions of
	 * a miscrosecond: bit 59 is 1.0us, bit 60 is .5us, bit 61 is .25us,
	 * bit 62 is .125us, bit 63 is 62.5ns, etc.
	 *
	 * Some of these bits can be implemented, some not. 64 bits of
	 * the TOD clock are implemented usually nowadays, these are
	 * bits 8-71 of the output.
	 *
	 * The stepping value of TOD-clock bit position 63, if implemented,
	 * is 2^-12 microseconds, or approximately 244 picoseconds. This value
	 * is called a clock unit.
	 */

	uint8_t clk[16];

	__asm__ __volatile__("stcke %0" : "=Q" (clk) : : "cc");

	/* s390x is big-endian, so just perfom a byte-by-byte copy */
	*out = *(uint64_t *)(clk + 1);
}

#elif defined(__powerpc)
/*
 * Uncomment this for newer PPC CPUs
 * Newer PPC CPUs do not support mftbu/mftb
 * these instructions were obsoleted and replaced by
 * mfspr.  special processor registers 268 and 269 are the
 * ones we want.
 */
 /* #define POWER_PC_USE_NEW_INSTRUCTIONS */

/* taken from http://www.ecrypt.eu.org/ebats/cpucycles.html */

static inline void jent_get_nstime(uint64_t *out)
{
	unsigned long high;
	unsigned long low;
	unsigned long newhigh;
	uint64_t result;
#ifdef POWER_PC_USE_NEW_INSTRUCTIONS /* Newer PPC CPUs do not support mftbu/mftb */
    __asm__ __volatile__(
        "Lcpucycles:mfspr %0, 269;mfspr %1, 268;mfspr %2, 269;cmpw %0,%2;bne Lcpucycles"
		: "=r" (high), "=r" (low), "=r" (newhigh)
		);
#else
    __asm__ __volatile__(
		"Lcpucycles:mftbu %0;mftb %1;mftbu %2;cmpw %0,%2;bne Lcpucycles"
		: "=r" (high), "=r" (low), "=r" (newhigh)
		);
#endif
	result = high;
	result <<= 32;
	result |= low;
	*out = result;
}

#else /* (__x86_64__) || (__i386__) || (__aarch64__) || (__s390x__) || (__powerpc) */

static inline void jent_get_nstime(uint64_t *out)
{
	/* OSX does not have clock_gettime -- taken from
	 * http://developer.apple.com/library/mac/qa/qa1398/_index.html */
# ifdef __MACH__
	*out = mach_absolute_time();
# elif _AIX
	/* clock_gettime() on AIX returns a timer value that increments in
	 * steps of 1000
	 */
	uint64_t tmp = 0;
	timebasestruct_t aixtime;
	tmp = aixtime.tb_high * 1000000000UL;
	tmp += aixtime.tb_low;
	*out = tmp;
# else /* __MACH__ */
	/* we could use CLOCK_MONOTONIC(_RAW), but with CLOCK_REALTIME
	 * we get some nice extra entropy once in a while from the NTP actions
	 * that we want to use as well... though, we do not rely on that
	 * extra little entropy */
	uint64_t tmp = 0;
	struct timespec time;
	if (clock_gettime(CLOCK_REALTIME, &time) == 0)
	{
		tmp = ((uint64_t)time.tv_sec & 0xFFFFFFFF) * 1000000000UL;
		tmp = tmp + (uint64_t)time.tv_nsec;
	}
	*out = tmp;
# endif /* __MACH__ */
}

#endif /* (__x86_64__) || (__i386__) || (__aarch64__) */

static inline void *jent_zalloc(size_t len)
{
	void *tmp = NULL;
#ifdef LIBGCRYPT
	/* When using the libgcrypt secure memory mechanism, all precautions
	 * are taken to protect our state. If the user disables secmem during
	 * runtime, it is his decision and we thus try not to overrule his
	 * decision for less memory protection. */
#define CONFIG_CRYPTO_CPU_JITTERENTROPY_SECURE_MEMORY
	tmp = gcry_xmalloc_secure(len);
#elif defined(OPENSSL) || defined(AWSLC)
	/* Does this allocation implies secure memory use? */
	tmp = OPENSSL_malloc(len);
#else
	/* we have no secure memory allocation! Hence
	 * we do not set CONFIG_CRYPTO_CPU_JITTERENTROPY_SECURE_MEMORY */
	tmp = malloc(len);
#endif /* LIBGCRYPT */
	if(NULL != tmp)
		memset(tmp, 0, len);
	return tmp;
}

static inline void jent_zfree(void *ptr, unsigned int len)
{
#ifdef LIBGCRYPT
	memset(ptr, 0, len);
	gcry_free(ptr);
#elif defined(AWSLC)
    /* AWS-LC stores the length of allocated memory internally and automatically wipes it in OPENSSL_free */
	(void) len;
	OPENSSL_free(ptr);
#elif defined(OPENSSL)
	OPENSSL_cleanse(ptr, len);
	OPENSSL_free(ptr);
#else
	memset(ptr, 0, len);
	free(ptr);
#endif /* LIBGCRYPT */
}

static inline int jent_fips_enabled(void)
{
#ifdef LIBGCRYPT
	return fips_mode();
#elif defined(AWSLC)
	return FIPS_mode();
#elif defined(OPENSSL)
#ifdef OPENSSL_FIPS
	return 1;
#else
	return 0;
#endif
#else
#define FIPS_MODE_SWITCH_FILE "/proc/sys/crypto/fips_enabled"
	char buf[2] = "0";
	int fd = 0;

	if ((fd = open(FIPS_MODE_SWITCH_FILE, O_RDONLY)) >= 0) {
		while (read(fd, buf, sizeof(buf)) < 0 && errno == EINTR);
		close(fd);
	}
	if (buf[0] == '1')
		return 1;
	else
		return 0;
#endif
}

static inline void jent_memset_secure(void *s, size_t n)
{
#if defined(AWSLC)
	OPENSSL_cleanse(s, n);
#else
	memset(s, 0, n);
	__asm__ __volatile__("" : : "r" (s) : "memory");
#endif
}

static inline long jent_ncpu(void)
{
#if defined(_POSIX_SOURCE) || defined(__APPLE__)
	long ncpu = sysconf(_SC_NPROCESSORS_ONLN);

	if (ncpu == -1)
		return -errno;

	if (ncpu == 0)
		return -EFAULT;

	return ncpu;
#else
	return 1;
#endif
}

#ifdef __linux__

# if defined(_SC_LEVEL1_DCACHE_SIZE) &&					\
     defined(_SC_LEVEL2_CACHE_SIZE) &&					\
     defined(_SC_LEVEL3_CACHE_SIZE)

static inline void jent_get_cachesize(long *l1, long *l2, long *l3)
{
	*l1 = sysconf(_SC_LEVEL1_DCACHE_SIZE);
	*l2 = sysconf(_SC_LEVEL2_CACHE_SIZE);
	*l3 = sysconf(_SC_LEVEL3_CACHE_SIZE);
}

# else

static inline void jent_get_cachesize(long *l1, long *l2, long *l3)
{
#define JENT_SYSFS_CACHE_DIR "/sys/devices/system/cpu/cpu0/cache"
	long val;
	unsigned int i;
	char buf[10], file[50];
	int fd = 0;

	/* Iterate over all caches */
	for (i = 0; i < 4; i++) {
		unsigned int shift = 0;
		char *ext;

		/*
		 * Check the cache type - we are only interested in Unified
		 * and Data caches.
		 */
		memset(buf, 0, sizeof(buf));
		snprintf(file, sizeof(file), "%s/index%u/type",
			 JENT_SYSFS_CACHE_DIR, i);
		fd = open(file, O_RDONLY);
		if (fd < 0)
			continue;
		while (read(fd, buf, sizeof(buf)) < 0 && errno == EINTR);
		close(fd);
		buf[sizeof(buf) - 1] = '\0';

		if (strncmp(buf, "Data", 4) && strncmp(buf, "Unified", 7))
			continue;

		/* Get size of cache */
		memset(buf, 0, sizeof(buf));
		snprintf(file, sizeof(file), "%s/index%u/size",
			 JENT_SYSFS_CACHE_DIR, i);

		fd = open(file, O_RDONLY);
		if (fd < 0)
			continue;
		while (read(fd, buf, sizeof(buf)) < 0 && errno == EINTR);
		close(fd);
		buf[sizeof(buf) - 1] = '\0';

		ext = strstr(buf, "K");
		if (ext) {
			shift = 10;
			*ext = '\0';
		} else {
			ext = strstr(buf, "M");
			if (ext) {
				shift = 20;
				*ext = '\0';
			}
		}

		val = strtol(buf, NULL, 10);
		if (val == LONG_MAX)
			continue;
		val <<= shift;

		if (!*l1)
			*l1 = val;
		else if (!*l2)
			*l2 = val;
		else {
			*l3 = val;
			break;
		}
	}
#undef JENT_SYSFS_CACHE_DIR
}

# endif

static inline uint32_t jent_cache_size_roundup(void)
{
	static int checked = 0;
	static uint32_t cache_size = 0;

	if (!checked) {
		long l1 = 0, l2 = 0, l3 = 0;

		jent_get_cachesize(&l1, &l2, &l3);
		checked = 1;

		/* Cache size reported by system */
		if (l1 > 0)
			cache_size += (uint32_t)l1;
		if (l2 > 0)
			cache_size += (uint32_t)l2;
		if (l3 > 0)
			cache_size += (uint32_t)l3;

		/*
		 * Force the output_size to be of the form
		 * (bounding_power_of_2 - 1).
		 */
		cache_size |= (cache_size >> 1);
		cache_size |= (cache_size >> 2);
		cache_size |= (cache_size >> 4);
		cache_size |= (cache_size >> 8);
		cache_size |= (cache_size >> 16);

		if (cache_size == 0)
			return 0;

		/*
		 * Make the output_size the smallest power of 2 strictly
		 * greater than cache_size.
		 */
		cache_size++;
	}

	return cache_size;
}

#else /* __linux__ */

static inline uint32_t jent_cache_size_roundup(void)
{
	return 0;
}

#endif /* __linux__ */

static inline void jent_yield(void)
{
	sched_yield();
}

/* --- helpers needed in user space -- */

static inline uint64_t rol64(uint64_t x, int n)
{
	return ( (x << (n&(64-1))) | (x >> ((64-n)&(64-1))) );
}

#endif /* _JITTERENTROPY_BASE_USER_H */
