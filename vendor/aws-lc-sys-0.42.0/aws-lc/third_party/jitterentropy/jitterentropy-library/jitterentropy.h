/*
 * Non-physical true random number generator based on timing jitter.
 *
 * Copyright Stephan Mueller <smueller@chronox.de>, 2014 - 2025
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

#ifndef _JITTERENTROPY_H
#define _JITTERENTROPY_H

#ifdef __cplusplus
extern "C" {
#endif

/*
 * API / ABI incompatible changes, functional changes that require consumer to
 * be updated (as long as this number is zero, the API is not considered stable
 * and can change without a bump of the major version).
 */
#define JENT_MAJVERSION 3

/*
 * API compatible, ABI may change, functional enhancements only, consumer can be
 * left unchanged if enhancements are not considered.
 */
#define JENT_MINVERSION 6

/*
 * API / ABI compatible, no functional changes, no enhancements, bug fixes only.
 * Also, the entropy collection is not changed in any way that would necessitate
 * a re-assessment.
 */
#define JENT_PATCHLEVEL 3

#define JENT_VERSION (JENT_MAJVERSION * 1000000 + \
		      JENT_MINVERSION * 10000 + \
		      JENT_PATCHLEVEL * 100)

/***************************************************************************
 * Jitter RNG Configuration Section
 *
 * You may alter the following options
 ***************************************************************************/

/*
 * Enable timer-less timer support with JENT_CONF_ENABLE_INTERNAL_TIMER
 *
 * In case the hardware is identified to not provide a high-resolution time
 * stamp, this option enables a built-in high-resolution time stamp mechanism.
 *
 * The timer-less noise source is based on threads. This noise source requires
 * the linking with the POSIX threads library. I.e. the executing environment
 * must offer POSIX threads. If this option is disabled, no linking
 * with the POSIX threads library is needed.
 */

/*
 * Disable the loop shuffle operation
 *
 * The shuffle operation enlarges the timing of the conditioning function
 * by a variable length defined by the LSB of a time stamp. Some mathematicians
 * are concerned that this pseudo-random selection of the loop iteration count
 * may create some form of dependency between the different loop counts
 * and the associated time duration of the conditioning function. It
 * also complicates entropy assessment because it effectively combines a bunch
 * of shifted/scaled copies the same distribution and masks failures from the
 * health testing.
 *
 * By enabling this flag, the loop shuffle operation is disabled and
 * the entropy collection operates in a way that honor the concerns.
 *
 * By enabling this flag, the time of collecting entropy may be enlarged.
 */
#define JENT_CONF_DISABLE_LOOP_SHUFFLE

/*
 * Shall the LAG predictor health test be enabled?
 */
#define JENT_HEALTH_LAG_PREDICTOR

/*
 * Shall the jent_memaccess use a (statistically) random selection for the
 * memory to update?
 */
#define JENT_RANDOM_MEMACCESS

/*
 * Mask specifying the number of bits of the raw entropy data of the time delta
 * value used for the APT.
 *
 * This value implies that for the APT, only the bits specified by
 * JENT_APT_MASK are taken. This was suggested in a draft IG D.K resolution 22
 * provided by NIST, but further analysis
 * (https://www.untruth.org/~josh/sp80090b/CMUF%20EWG%20Draft%20IG%20D.K%20Comments%20D10.pdf)
 * suggests that this truncation / translation generally results in a health
 * test with both a higher false positive rate (because multiple raw symbols
 * map to the same symbol within the health test) and a lower statistical power
 * when the APT cutoff is selected based on the apparent truncated entropy
 * (i.e., truncation generally makes the test worse). NIST has since withdrawn
 * this draft and stated that they will not propose truncation prior to
 * health testing.
 * Because the general tendency of such truncation to make the health test
 * worse the default value is set such that no data is masked out and this
 * should only be changed if a hardware-specific analysis suggests that some
 * other mask setting is beneficial.
 * The mask is applied to a time stamp where the GCD is already divided out, and thus no
 * "non-moving" low-order bits are present.
 */
#define JENT_APT_MASK		(UINT64_C(0xffffffffffffffff))

/* This parameter establishes the multiplicative factor that the desired
 * memory region size should be larger than the observed cache size; the
 * multiplicative factor is 2^JENT_CACHE_SHIFT_BITS.
 * Set this to 0 if the desired memory region should be at least as large as
 * the cache. If one wants most of the memory updates to result in a memory
 * update, then this value should be at least 1.
 * If the memory updates should dominantly result in a memory update, then
 * the value should be set to at least 3.
 * The actual size of the memory region is never larger than requested by
 * the passed in JENT_MAX_MEMSIZE_* flag (if provided) or JENT_MEMORY_SIZE
 * (if no JENT_MAX_MEMSIZE_* flag is provided).
 */
#ifndef JENT_CACHE_SHIFT_BITS
#define JENT_CACHE_SHIFT_BITS 0
#endif

/***************************************************************************
 * Jitter RNG State Definition Section
 ***************************************************************************/

#if defined(_MSC_VER)
#include "arch/jitterentropy-base-windows.h"
#else
#include "jitterentropy-base-user.h"
#endif

#define JENT_SHA3_256_SIZE_DIGEST_BITS	256
#define JENT_SHA3_256_SIZE_DIGEST	(JENT_SHA3_256_SIZE_DIGEST_BITS >> 3)

/*
 * The output 256 bits can receive more than 256 bits of min entropy,
 * of course, but the 256-bit output of SHA3-256(M) can only asymptotically
 * approach 256 bits of min entropy, not attain that bound. Random maps will
 * tend to have output collisions, which reduces the creditable output entropy
 * (that is what SP 800-90B Section 3.1.5.1.2 attempts to bound).
 *
 * The value "64" is justified in Appendix A.4 of the current 90C draft,
 * and aligns with NIST's in "epsilon" definition in this document, which is
 * that a string can be considered "full entropy" if you can bound the min
 * entropy in each bit of output to at least 1-epsilon, where epsilon is
 * required to be <= 2^(-32).
 */
#define ENTROPY_SAFETY_FACTOR		64

/**
 * Function pointer data structure to register an external thread handler
 * used for the timer-less mode of the Jitter RNG.
 *
 * The external caller provides these function pointers to handle the
 * management of the timer thread that is spawned by the Jitter RNG.
 *
 * @var jent_notime_init This function is intended to initialize the threading
 *	support. All data that is required by the threading code must be
 *	held in the data structure @param ctx. The Jitter RNG maintains the
 *	data structure and uses it for every invocation of the following calls.
 *
 * @var jent_notime_fini This function shall terminate the threading support.
 *	The function must dispose of all memory and resources used for the
 *	threading operation. It must also dispose of the @param ctx memory.
 *
 * @var jent_notime_start This function is called when the Jitter RNG wants
 *	to start a thread. Besides providing a pointer to the @param ctx
 *	allocated during initialization time, the Jitter RNG provides a
 *	pointer to the function the thread shall execute and the argument
 *	the function shall be invoked with. These two parameters have the
 *	same purpose as the trailing two parameters of pthread_create(3).
 *
 * @var jent_notime_stop This function is invoked by the Jitter RNG when the
 *	thread should be stopped. Note, the Jitter RNG intends to start/stop
 *	the thread frequently.
 *
 * An example implementation is found in the Jitter RNG itself with its
 * default thread handler of jent_notime_thread_builtin.
 *
 * If the caller wants to register its own thread handler, it must be done
 * with the API call jent_entropy_switch_notime_impl as the first
 * call to interact with the Jitter RNG, even before jent_entropy_init.
 * After jent_entropy_init is called, changing of the threading implementation
 * is not allowed.
 */
struct jent_notime_thread {
	int (*jent_notime_init)(void **ctx);
	void (*jent_notime_fini)(void *ctx);
	int (*jent_notime_start)(void *ctx,
				 void *(*start_routine) (void *), void *arg);
	void (*jent_notime_stop)(void *ctx);
};

/* The entropy pool */
struct rand_data
{
	/* all data values that are vital to maintain the security
	 * of the RNG are marked as SENSITIVE. A user must not
	 * access that information while the RNG executes its loops to
	 * calculate the next random value. */
	void *hash_state;		/* SENSITIVE hash state entropy pool */
	uint64_t prev_time;		/* SENSITIVE Previous time stamp */
#define DATA_SIZE_BITS (JENT_SHA3_256_SIZE_DIGEST_BITS)

#ifndef JENT_HEALTH_LAG_PREDICTOR
	uint64_t last_delta;		/* SENSITIVE stuck test */
	uint64_t last_delta2;		/* SENSITIVE stuck test */
#endif /* JENT_HEALTH_LAG_PREDICTOR */

	unsigned int flags;		/* Flags used to initialize */
	unsigned int osr;		/* Oversampling rate */

#ifdef JENT_RANDOM_MEMACCESS
  /* The step size should be larger than the cacheline size. */
# ifndef JENT_MEMORY_BITS
#  define JENT_MEMORY_BITS 17
# endif
# ifndef JENT_MEMORY_SIZE
#  define JENT_MEMORY_SIZE (UINT32_C(1)<<JENT_MEMORY_BITS)
# endif
#else /* JENT_RANDOM_MEMACCESS */
# ifndef JENT_MEMORY_BLOCKS
#  define JENT_MEMORY_BLOCKS 512
# endif
# ifndef JENT_MEMORY_BLOCKSIZE
#  define JENT_MEMORY_BLOCKSIZE 128
# endif
# ifndef JENT_MEMORY_SIZE
#  define JENT_MEMORY_SIZE (JENT_MEMORY_BLOCKS*JENT_MEMORY_BLOCKSIZE)
# endif
#endif /* JENT_RANDOM_MEMACCESS */

#define JENT_MEMORY_ACCESSLOOPS 128
	unsigned char *mem;		/* Memory access location with size of
					 * JENT_MEMORY_SIZE or memsize */
#ifdef JENT_RANDOM_MEMACCESS
	uint32_t memmask;		/* Memory mask (size of memory - 1) */
#else
	unsigned int memlocation; 	/* Pointer to byte in *mem */
	unsigned int memblocks;		/* Number of memory blocks in *mem */
	unsigned int memblocksize; 	/* Size of one memory block in bytes */
#endif
	unsigned int memaccessloops;	/* Number of memory accesses per random
					 * bit generation */

	/* Repetition Count Test */
	int rct_count;			/* Number of stuck values */

	/* Adaptive Proportion Test for a significance level of 2^-30 */
	unsigned int apt_cutoff;	/* Intermittent health test failure */
	unsigned int apt_cutoff_permanent; /* Permanent health test failure */
#define JENT_APT_WINDOW_SIZE	512	/* Data window size */
	unsigned int apt_observations;	/* Number of collected observations in
					 * current window. */
	unsigned int apt_count;		/* The number of times the reference
					 * symbol been encountered in the
					 * window. */
	uint64_t apt_base;		/* APT base reference */
	unsigned int health_failure;	/* Permanent health failure */

	unsigned int apt_base_set:1;	/* APT base reference set? */
	unsigned int fips_enabled:1;
	unsigned int enable_notime:1;	/* Use internal high-res timer */
	unsigned int max_mem_set:1;	/* Maximum memory configured by user */

#ifdef JENT_CONF_ENABLE_INTERNAL_TIMER
	volatile uint8_t notime_interrupt;	/* indicator to interrupt ctr */
	volatile uint64_t notime_timer;		/* high-res timer mock-up */
	uint64_t notime_prev_timer;		/* previous timer value */
	void *notime_thread_ctx;		/* register thread data */
#endif /* JENT_CONF_ENABLE_INTERNAL_TIMER */

	uint64_t jent_common_timer_gcd;	/* Common divisor for all time deltas */

#ifdef JENT_HEALTH_LAG_PREDICTOR
	/* Lag predictor test to look for re-occurring patterns. */

	/* The lag global cutoff selected based on the selection of osr. */
	unsigned int lag_global_cutoff;

	/* The lag local cutoff selected based on the selection of osr. */
	unsigned int lag_local_cutoff;

	/*
	 * The number of times the lag predictor was correct. Compared to the
	 * global cutoff.
	 */
	unsigned int lag_prediction_success_count;

	/*
	 * The size of the current run of successes. Compared to the local
	 * cutoff.
	 */
	unsigned int lag_prediction_success_run;

	/*
	 * The total number of collected observations since the health test was
	 * last reset.
	 */
	unsigned int lag_best_predictor;

	/*
	 * The total number of collected observations since the health test was
	 * last reset.
	 */
	unsigned int lag_observations;

	/*
	 * This is the size of the window used by the predictor. The predictor
	 * is reset between windows.
	 */
#define JENT_LAG_WINDOW_SIZE (1U<<17)

	/*
	 * The amount of history to base predictions on. This must be a power
	 * of 2. Must be 4 or greater.
	 */
#define JENT_LAG_HISTORY_SIZE 8
#define JENT_LAG_MASK (JENT_LAG_HISTORY_SIZE - 1)

	/* The delta history for the lag predictor. */
	uint64_t lag_delta_history[JENT_LAG_HISTORY_SIZE];

	/* The scoreboard that tracks how successful each predictor lag is. */
	unsigned int lag_scoreboard[JENT_LAG_HISTORY_SIZE];
#endif /* JENT_HEALTH_LAG_PREDICTOR */
};

/* Flags that can be used to initialize the RNG */
#define JENT_DISABLE_STIR (1<<0) 	/* UNUSED */
#define JENT_DISABLE_UNBIAS (1<<1) 	/* UNUSED */
#define JENT_DISABLE_MEMORY_ACCESS (1<<2) /* Disable memory access for more
					     entropy, saves MEMORY_SIZE RAM for
					     entropy collector */
#define JENT_FORCE_INTERNAL_TIMER (1<<3)  /* Force the use of the internal
					     timer */
#define JENT_DISABLE_INTERNAL_TIMER (1<<4)  /* Disable the potential use of
					       the internal timer. */
#define JENT_FORCE_FIPS (1<<5)		  /* Force FIPS compliant mode
					     including full SP800-90B
					     compliance. */

/* Flags field limiting the amount of memory to be used for memory access */
#define JENT_FLAGS_TO_MEMSIZE_SHIFT	28
#define JENT_FLAGS_TO_MAX_MEMSIZE(val)	(val >> JENT_FLAGS_TO_MEMSIZE_SHIFT)
#define JENT_MAX_MEMSIZE_TO_FLAGS(val)	(val << JENT_FLAGS_TO_MEMSIZE_SHIFT)
#define JENT_MAX_MEMSIZE_32kB		JENT_MAX_MEMSIZE_TO_FLAGS(UINT32_C( 1))
#define JENT_MAX_MEMSIZE_64kB		JENT_MAX_MEMSIZE_TO_FLAGS(UINT32_C( 2))
#define JENT_MAX_MEMSIZE_128kB		JENT_MAX_MEMSIZE_TO_FLAGS(UINT32_C( 3))
#define JENT_MAX_MEMSIZE_256kB		JENT_MAX_MEMSIZE_TO_FLAGS(UINT32_C( 4))
#define JENT_MAX_MEMSIZE_512kB		JENT_MAX_MEMSIZE_TO_FLAGS(UINT32_C( 5))
#define JENT_MAX_MEMSIZE_1MB		JENT_MAX_MEMSIZE_TO_FLAGS(UINT32_C( 6))
#define JENT_MAX_MEMSIZE_2MB		JENT_MAX_MEMSIZE_TO_FLAGS(UINT32_C( 7))
#define JENT_MAX_MEMSIZE_4MB		JENT_MAX_MEMSIZE_TO_FLAGS(UINT32_C( 8))
#define JENT_MAX_MEMSIZE_8MB		JENT_MAX_MEMSIZE_TO_FLAGS(UINT32_C( 9))
#define JENT_MAX_MEMSIZE_16MB		JENT_MAX_MEMSIZE_TO_FLAGS(UINT32_C(10))
#define JENT_MAX_MEMSIZE_32MB		JENT_MAX_MEMSIZE_TO_FLAGS(UINT32_C(11))
#define JENT_MAX_MEMSIZE_64MB		JENT_MAX_MEMSIZE_TO_FLAGS(UINT32_C(12))
#define JENT_MAX_MEMSIZE_128MB		JENT_MAX_MEMSIZE_TO_FLAGS(UINT32_C(13))
#define JENT_MAX_MEMSIZE_256MB		JENT_MAX_MEMSIZE_TO_FLAGS(UINT32_C(14))
#define JENT_MAX_MEMSIZE_512MB		JENT_MAX_MEMSIZE_TO_FLAGS(UINT32_C(15))
#define JENT_MAX_MEMSIZE_MAX		JENT_MAX_MEMSIZE_512MB
#define JENT_MAX_MEMSIZE_MASK		JENT_MAX_MEMSIZE_MAX
/* We start at 32kB -> offset is log2(32768) */
#define JENT_MAX_MEMSIZE_OFFSET		14

#ifdef JENT_CONF_DISABLE_LOOP_SHUFFLE
# define JENT_MIN_OSR	3
#else
# define JENT_MIN_OSR	1
#endif

#ifdef JENT_PRIVATE_COMPILE
# define JENT_PRIVATE_STATIC static
#else /* JENT_PRIVATE_COMPILE */
#if defined(_MSC_VER)
#define JENT_PRIVATE_STATIC __declspec(dllexport)
#else
#define JENT_PRIVATE_STATIC __attribute__((visibility("default")))
#endif
#endif

/* Number of low bits of the time value that we want to consider */
/* get raw entropy */
JENT_PRIVATE_STATIC
ssize_t jent_read_entropy(struct rand_data *ec, char *data, size_t len);
JENT_PRIVATE_STATIC
ssize_t jent_read_entropy_safe(struct rand_data **ec, char *data, size_t len);
/* initialize an instance of the entropy collector */
JENT_PRIVATE_STATIC
struct rand_data *jent_entropy_collector_alloc(unsigned int osr,
	       				       unsigned int flags);
/* clearing of entropy collector */
JENT_PRIVATE_STATIC
void jent_entropy_collector_free(struct rand_data *entropy_collector);

/* initialization of entropy collector */
JENT_PRIVATE_STATIC
int jent_entropy_init(void);
JENT_PRIVATE_STATIC
int jent_entropy_init_ex(unsigned int osr, unsigned int flags);

/*
 * Set a callback to run on health failure in FIPS mode.
 * This function will take an action determined by the caller.
 */
typedef void (*jent_fips_failure_cb)(struct rand_data *ec,
				     unsigned int health_failure);
JENT_PRIVATE_STATIC
int jent_set_fips_failure_callback(jent_fips_failure_cb cb);

/* return version number of core library */
JENT_PRIVATE_STATIC
unsigned int jent_version(void);

/* Set a different thread handling logic for the notimer support */
JENT_PRIVATE_STATIC
int jent_entropy_switch_notime_impl(struct jent_notime_thread *new_thread);

/* -- END of Main interface functions -- */

/* -- BEGIN timer-less threading support functions to prevent code dupes -- */

#ifdef JENT_CONF_ENABLE_INTERNAL_TIMER

struct jent_notime_ctx {
	pthread_attr_t notime_pthread_attr;	/* pthreads library */
	pthread_t notime_thread_id;		/* pthreads thread ID */
};

JENT_PRIVATE_STATIC
int jent_notime_init(void **ctx);

JENT_PRIVATE_STATIC
void jent_notime_fini(void *ctx);

#else

static inline int jent_notime_init(void **ctx) { (void)ctx; return 0; }
static inline void jent_notime_fini(void *ctx) { (void)ctx; }

#endif /* JENT_CONF_ENABLE_INTERNAL_TIMER */

/* -- END timer-less threading support functions to prevent code dupes -- */

/* -- BEGIN error codes for init function -- */
#define ENOTIME  	1 /* Timer service not available */
#define ECOARSETIME	2 /* Timer too coarse for RNG */
#define ENOMONOTONIC	3 /* Timer is not monotonic increasing */
#define EMINVARIATION	4 /* Timer variations too small for RNG */
#define EVARVAR		5 /* Timer does not produce variations of variations
			     (2nd derivation of time is zero) */
#define EMINVARVAR	6 /* Timer variations of variations is too small */
#define EPROGERR	7 /* Programming error */
#define ESTUCK		8 /* Too many stuck results during init. */
#define EHEALTH		9 /* Health test failed during initialization */
#define ERCT		10 /* RCT failed during initialization */
#define EHASH		11 /* Hash self test failed */
#define EMEM		12 /* Can't allocate memory for initialization */
#define EGCD		13 /* GCD self-test failed */
/* -- END error codes for init function -- */

/* -- BEGIN error masks for health tests -- */
#define JENT_RCT_FAILURE	1 /* Failure in RCT health test. */
#define JENT_APT_FAILURE	2 /* Failure in APT health test. */
#define JENT_LAG_FAILURE	4 /* Failure in Lag predictor health test. */
#define JENT_PERMANENT_FAILURE_SHIFT	16
#define JENT_PERMANENT_FAILURE(x)	(x << JENT_PERMANENT_FAILURE_SHIFT)
#define JENT_RCT_FAILURE_PERMANENT	JENT_PERMANENT_FAILURE(JENT_RCT_FAILURE)
#define JENT_APT_FAILURE_PERMANENT	JENT_PERMANENT_FAILURE(JENT_APT_FAILURE)
#define JENT_LAG_FAILURE_PERMANENT	JENT_PERMANENT_FAILURE(JENT_LAG_FAILURE)
/* -- END error masks for health tests -- */

/* -- BEGIN statistical test functions only complied with CONFIG_CRYPTO_CPU_JITTERENTROPY_STAT -- */

#ifdef CONFIG_CRYPTO_CPU_JITTERENTROPY_STAT
JENT_PRIVATE_STATIC
uint64_t jent_lfsr_var_stat(struct rand_data *ec, unsigned int min);
#endif /* CONFIG_CRYPTO_CPU_JITTERENTROPY_STAT */

/* -- END of statistical test function -- */

#ifdef __cplusplus
}
#endif

#endif /* _JITTERENTROPY_H */
