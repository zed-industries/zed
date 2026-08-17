/*
 * Non-physical true random number generator based on timing jitter.
 *
 * Copyright Stephan Mueller <smueller@chronox.de>, 2014 - 2025
 *
 * Design
 * ======
 *
 * See documentation in doc/ folder.
 *
 * Interface
 * =========
 *
 * See documentation in jitterentropy(3) man page.
 *
 * License: see LICENSE file in root directory
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

#include "jitterentropy-base.h"
#include "jitterentropy-gcd.h"
#include "jitterentropy-health.h"
#include "jitterentropy-internal.h"
#include "jitterentropy-noise.h"
#include "jitterentropy-timer.h"
#include "jitterentropy-sha3.h"

/***************************************************************************
 * Jitter RNG Static Definitions
 *
 * None of the following should be altered
 ***************************************************************************/

#ifdef __OPTIMIZE__
 #error "The CPU Jitter random number generator must not be compiled with optimizations. See documentation. Use the compiler switch -O0 for compiling jitterentropy.c."
#endif

/*
 * JENT_POWERUP_TESTLOOPCOUNT needs some loops to identify edge
 * systems. 100 is definitely too little.
 *
 * SP800-90B requires at least 1024 initial test cycles.
 */
#define JENT_POWERUP_TESTLOOPCOUNT 1024

/*
 * ensure_osr_is_at_least_minimal ensures that the over sampling rate is, at
 * minimum, JENT_MIN_OSR.
 *
 * @return returns the argument current_osr if equal to or larger than
 * JENT_MIN_OSR otherwise, returns JENT_MIN_OSR.
 */
static unsigned int ensure_osr_is_at_least_minimal(unsigned int current_osr)
{
	if (current_osr < JENT_MIN_OSR)
		return (unsigned int) JENT_MIN_OSR;
	else
		return current_osr;
}

/**
 * jent_version() - Return machine-usable version number of jent library
 *
 * The function returns a version number that is monotonic increasing
 * for newer versions. The version numbers are multiples of 100. For example,
 * version 1.2.3 is converted to 1020300 -- the last two digits are reserved
 * for future use.
 *
 * The result of this function can be used in comparing the version number
 * in a calling program if version-specific calls need to be make.
 *
 * @return Version number of jitterentropy library
 */
JENT_PRIVATE_STATIC
unsigned int jent_version(void)
{
	return JENT_VERSION;
}

/***************************************************************************
 * Helper
 ***************************************************************************/

/* Calculate log2 of given value assuming that the value is a power of 2 */
static inline unsigned int jent_log2_simple(unsigned int val)
{
	unsigned int idx = 0;

	while (val >>= 1)
		idx++;
	return idx;
}

/* Increase the memory size by one step */
static inline unsigned int jent_update_memsize(unsigned int flags)
{
	unsigned int global_max = JENT_FLAGS_TO_MAX_MEMSIZE(
							JENT_MAX_MEMSIZE_MAX);
	unsigned int max;

	max = JENT_FLAGS_TO_MAX_MEMSIZE(flags);

	if (!max) {
		/*
		 * The safe starting value is the amount of memory we allocated
		 * last round.
		 */
		max = jent_log2_simple(JENT_MEMORY_SIZE);
		/* Adjust offset */
		max = (max > JENT_MAX_MEMSIZE_OFFSET) ?
			max - JENT_MAX_MEMSIZE_OFFSET :	0;
	} else {
		max++;
	}

	max = (max > global_max) ? global_max : max;

	/* Clear out the max size */
	flags &= ~JENT_MAX_MEMSIZE_MASK;
	/* Set the freshly calculated max size */
	flags |= JENT_MAX_MEMSIZE_TO_FLAGS(max);

	return flags;
}

/***************************************************************************
 * Random Number Generation
 ***************************************************************************/

/**
 * Entry function: Obtain entropy for the caller.
 *
 * This function invokes the entropy gathering logic as often to generate
 * as many bytes as requested by the caller. The entropy gathering logic
 * creates 64 bit per invocation.
 *
 * This function truncates the last 64 bit entropy value output to the exact
 * size specified by the caller.
 *
 * @ec [in] Reference to entropy collector
 * @data [out] pointer to buffer for storing random data -- buffer must
 *	       already exist
 * @len [in] size of the buffer, specifying also the requested number of random
 *	     in bytes
 *
 * @return number of bytes returned when request is fulfilled or an error
 *
 * The following error codes can occur:
 *	-1	entropy_collector is NULL
 *	-2	RCT failed
 *	-3	APT failed
 *	-4	The timer cannot be initialized
 *	-5	LAG failure
 *	-6	RCT permanent failure
 *	-7	APT permanent failure
 *	-8	LAG permanent failure
 */
JENT_PRIVATE_STATIC
ssize_t jent_read_entropy(struct rand_data *ec, char *data, size_t len)
{
	char *p = data;
	size_t orig_len = len;
	int ret = 0;

	if (NULL == ec)
		return -1;

	if (jent_notime_settick(ec))
		return -4;

	while (len > 0) {
		size_t tocopy;
		unsigned int health_test_result;

		jent_random_data(ec);

		if ((health_test_result = jent_health_failure(ec))) {
			if (health_test_result & JENT_RCT_FAILURE_PERMANENT)
				ret = -6;
			else if (health_test_result &
				 JENT_APT_FAILURE_PERMANENT)
				ret = -7;
			else if (health_test_result &
				 JENT_LAG_FAILURE_PERMANENT)
				ret = -8;
			else if (health_test_result & JENT_RCT_FAILURE)
				ret = -2;
			else if (health_test_result & JENT_APT_FAILURE)
				ret = -3;
			else
				ret = -5;

			goto err;
		}

		if ((DATA_SIZE_BITS / 8) < len)
			tocopy = (DATA_SIZE_BITS / 8);
		else
			tocopy = len;

		jent_read_random_block(ec, p, tocopy);

		len -= tocopy;
		p += tocopy;
	}

	/*
	 * Enhanced backtracking support: At this point, the hash state
	 * contains the digest of the previous Jitter RNG collection round
	 * which is inserted there by jent_read_random_block with the SHA
	 * update operation. At the current code location we completed
	 * one request for a caller and we do not know how long it will
	 * take until a new request is sent to us. To guarantee enhanced
	 * backtracking resistance at this point (i.e. ensure that an attacker
	 * cannot obtain information about prior random numbers we generated),
	 * but still stirring the hash state with old data the Jitter RNG
	 * obtains a new message digest from its state and re-inserts it.
	 * After this operation, the Jitter RNG state is still stirred with
	 * the old data, but an attacker who gets access to the memory after
	 * this point cannot deduce the random numbers produced by the
	 * Jitter RNG prior to this point.
	 */
	/*
	 * If we use secured memory, where backtracking support may not be
	 * needed because the state is protected in a different method,
	 * it is permissible to drop this support. But strongly weigh the
	 * pros and cons considering that the SHA3 operation is not that
	 * expensive.
	 */
#ifndef CONFIG_CRYPTO_CPU_JITTERENTROPY_SECURE_MEMORY
	jent_read_random_block(ec, NULL, 0);
#endif

err:
	jent_notime_unsettick(ec);
	return ret ? ret : (ssize_t)orig_len;
}

static struct rand_data *_jent_entropy_collector_alloc(unsigned int osr,
						       unsigned int flags);

/**
 * Entry function: Obtain entropy for the caller.
 *
 * This is a service function to jent_read_entropy() with the difference
 * that it automatically re-allocates the entropy collector if a health
 * test failure is observed. Before reallocation, a new power-on health test
 * is performed. The allocation of the new entropy collector automatically
 * increases the OSR by one. This is done based on the idea that a health
 * test failure indicates that the assumed entropy rate is too high.
 *
 * Note the function returns with an health test error if the OSR is
 * getting too large. If an error is returned by this function, the Jitter RNG
 * is not safe to be used on the current system.
 *
 * @ec [in] Reference to entropy collector - this is a double pointer as
 *	    The entropy collector may be freed and reallocated.
 * @data [out] pointer to buffer for storing random data -- buffer must
 *	       already exist
 * @len [in] size of the buffer, specifying also the requested number of random
 *	     in bytes
 *
 * @return see jent_read_entropy()
 */
JENT_PRIVATE_STATIC
ssize_t jent_read_entropy_safe(struct rand_data **ec, char *data, size_t len)
{
	char *p = data;
	size_t orig_len = len;
	ssize_t ret = 0;

	if (!ec)
		return -1;

	while (len > 0) {
		unsigned int osr, flags, max_mem_set, apt_observations = 0,
			     lag_prediction_success_run,
			     lag_prediction_success_count;
		uint64_t current_delta;

		ret = jent_read_entropy(*ec, p, len);

		switch (ret) {
		case -1:
		case -4:

			/* Permanent errors are returned immediately */
		case -6:
		case -7:
		case -8:
			return ret;

		case -2:
		case -3:
		case -5:
			apt_observations = (*ec)->apt_observations;
			current_delta = (*ec)->apt_base;
			lag_prediction_success_run =
				(*ec)->lag_prediction_success_run;
			lag_prediction_success_count =
				(*ec)->lag_prediction_success_count;

			osr = (*ec)->osr + 1;
			flags = (*ec)->flags;
			max_mem_set = (*ec)->max_mem_set;

			/* generic arbitrary cutoff */
			if (osr > 20)
				return ret;

			/*
			 * If the caller did not set any specific maximum value
			 * let the Jitter RNG increase the maximum memory by
			 * one step.
			 */
			if (!max_mem_set)
				flags = jent_update_memsize(flags);

			/*
			 * re-allocate entropy collector with higher OSR and
			 * memory size
			 */
			jent_entropy_collector_free(*ec);
			*ec = NULL;

			/* Perform new health test with updated OSR */
			while (jent_entropy_init_ex(osr, flags)) {
				osr++;
				if (osr > 20)
					return -1;
			}

			*ec = _jent_entropy_collector_alloc(osr, flags);
			if (!*ec)
				return -1;

			/* Remember whether caller configured memory size */
			(*ec)->max_mem_set = !!max_mem_set;

			/*
			 * Set the health test state in case of intermittent
			 * failures.
			 */
			if (apt_observations) {
				/*
				 * APT re-initialization to intermittent error
				 */
				jent_apt_reinit(*ec, current_delta, 0,
						apt_observations);

				/*
				 * RCT re-initialization to intermittent error
				 */
				(*ec)->rct_count =
					(int)(JENT_HEALTH_RCT_INTERMITTENT_CUTOFF(osr));

				/* LAG re-initialization */
				(*ec)->lag_prediction_success_run =
					lag_prediction_success_run;
				(*ec)->lag_prediction_success_count =
					lag_prediction_success_count;
			}

			/*
			 * We are not returning the intermittent or permanent
			 * errors here. If a caller wants them, he should
			 * register a callback with
			 * jent_set_fips_failure_callback.
			 */

			break;

		default:
			len -= (size_t)ret;
			p += (size_t)ret;
		}
	}

	return (ssize_t)orig_len;
}

/***************************************************************************
 * Initialization logic
 ***************************************************************************/

/*
 * Obtain memory size to allocate for memory access variations.
 *
 * The maximum variations we can get from the memory access is when we allocate
 * a bit more memory than we have as data cache. But allocating as much
 * memory as we have as data cache might strain the resources on the system
 * more than necessary.
 *
 * On a lot of systems it is not necessary to need so much memory as the
 * variations coming from the general Jitter RNG execution commonly provide
 * large amount of variations.
 *
 * Thus, the default is:
 *
 * min(JENT_MEMORY_SIZE, data cache size)
 *
 * In case the data cache size cannot be obtained, use JENT_MEMORY_SIZE.
 *
 * If the caller provides a maximum memory size, use
 * min(provided max memory, data cache size).
 */
static inline uint32_t jent_memsize(unsigned int flags)
{
	uint32_t cache_memsize=0, max_memsize=0, memsize=0;

	max_memsize = JENT_FLAGS_TO_MAX_MEMSIZE(flags);

	if (max_memsize == 0) {
		max_memsize = JENT_MEMORY_SIZE;
	} else {
		max_memsize = UINT32_C(1) << (max_memsize +
					      JENT_MAX_MEMSIZE_OFFSET);
	}

	/* Allocate memory for adding variations based on memory access */
	cache_memsize = jent_cache_size_roundup();
	memsize = cache_memsize << JENT_CACHE_SHIFT_BITS;
	/* If this value is left-shifted too much, it may be cleared. */
	/* If so, set the maximum possible power of two. */
	if (cache_memsize > memsize) memsize = 0x80000000;

	/* Limit the memory as defined by caller */
	memsize = (memsize > max_memsize) ? max_memsize : memsize;

	/* Set a value if none was found */
	if (!memsize)
		memsize = JENT_MEMORY_SIZE;

	return memsize;
}

static int jent_selftest_run = 0;

static struct rand_data
*jent_entropy_collector_alloc_internal(unsigned int osr, unsigned int flags)
{
	struct rand_data *entropy_collector;
	uint32_t memsize = 0;

	/*
	 * Requesting disabling and forcing of internal timer
	 * makes no sense.
	 */
	if ((flags & JENT_DISABLE_INTERNAL_TIMER) &&
	    (flags & JENT_FORCE_INTERNAL_TIMER))
		return NULL;

	/*
	 * Ensure over sampling rate is not too low.
	 */
	osr = ensure_osr_is_at_least_minimal(osr);

	/* Force the self test to be run */
	if (!jent_selftest_run && jent_entropy_init_ex(osr, flags))
		return NULL;

	/*
	 * If the initial test code concludes to force the internal timer
	 * and the user requests it not to be used, do not allocate
	 * the Jitter RNG instance.
	 */
	if (jent_notime_forced() && (flags & JENT_DISABLE_INTERNAL_TIMER))
		return NULL;

	entropy_collector = jent_zalloc(sizeof(struct rand_data));
	if (NULL == entropy_collector)
		return NULL;

	if (!(flags & JENT_DISABLE_MEMORY_ACCESS)) {
		memsize = jent_memsize(flags);
		entropy_collector->mem = (unsigned char *)jent_zalloc(memsize);

#ifdef JENT_RANDOM_MEMACCESS
		/*
		 * Transform the size into a mask - it is assumed that size is
		 * a power of 2.
		 */
		entropy_collector->memmask = memsize - 1;
#else /* JENT_RANDOM_MEMACCESS */
		entropy_collector->memblocksize = memsize / JENT_MEMORY_BLOCKS;
		entropy_collector->memblocks = JENT_MEMORY_BLOCKS;

		/* sanity check */
		if (entropy_collector->memblocksize *
		    entropy_collector->memblocks != memsize)
			goto err;

#endif /* JENT_RANDOM_MEMACCESS */

		if (entropy_collector->mem == NULL)
			goto err;
		entropy_collector->memaccessloops = JENT_MEMORY_ACCESSLOOPS;
	}

	if (jent_sha3_alloc(&entropy_collector->hash_state))
		goto err;

	/* Initialize the hash state */
	jent_sha3_256_init(entropy_collector->hash_state);

	/* Set the oversampling rate */
	entropy_collector->osr = osr;
	entropy_collector->flags = flags;

	if ((flags & JENT_FORCE_FIPS) || jent_fips_enabled())
		entropy_collector->fips_enabled = 1;

	/* Initialize the APT */
	jent_apt_init(entropy_collector, osr);

	/* Initialize the Lag Predictor Test */
	jent_lag_init(entropy_collector, osr);

	/* Was jent_entropy_init run (establishing the common GCD)? */
	if (jent_gcd_get(&entropy_collector->jent_common_timer_gcd)) {
		/*
		 * It was not. This should probably be an error, but this
		 * behavior breaks the test code. Set the gcd to a value that
		 * won't hurt anything.
		 */
		entropy_collector->jent_common_timer_gcd = 1;
	}

	/*
	 * Use timer-less noise source - note, OSR must be set in
	 * entropy_collector!
	 */
	if (!(flags & JENT_DISABLE_INTERNAL_TIMER)) {
		if (jent_notime_enable(entropy_collector, flags))
			goto err;
	}

	return entropy_collector;

err:
	if (entropy_collector->mem != NULL)
		jent_zfree(entropy_collector->mem, memsize);
	if (entropy_collector->hash_state != NULL)
		jent_sha3_dealloc(entropy_collector->hash_state);
	jent_zfree(entropy_collector, sizeof(struct rand_data));
	return NULL;
}

static struct rand_data *_jent_entropy_collector_alloc(unsigned int osr,
						       unsigned int flags)
{
	struct rand_data *ec = jent_entropy_collector_alloc_internal(osr,
								     flags);

	if (!ec)
		return ec;

	/* fill the data pad with non-zero values */
	if (jent_notime_settick(ec)) {
		jent_entropy_collector_free(ec);
		return NULL;
	}
	jent_random_data(ec);
	jent_notime_unsettick(ec);

	return ec;
}

JENT_PRIVATE_STATIC
struct rand_data *jent_entropy_collector_alloc(unsigned int osr,
					       unsigned int flags)
{
	struct rand_data *ec = _jent_entropy_collector_alloc(osr, flags);

	/* Remember that the caller provided a maximum size flag */
	if (ec)
		ec->max_mem_set = !!JENT_FLAGS_TO_MAX_MEMSIZE(flags);

	return ec;
}

JENT_PRIVATE_STATIC
void jent_entropy_collector_free(struct rand_data *entropy_collector)
{
	if (entropy_collector != NULL) {
		jent_sha3_dealloc(entropy_collector->hash_state);
		jent_notime_disable(entropy_collector);
		if (entropy_collector->mem != NULL) {
			jent_zfree(entropy_collector->mem,
				   jent_memsize(entropy_collector->flags));
			entropy_collector->mem = NULL;
		}
		jent_zfree(entropy_collector, sizeof(struct rand_data));
	}
}

int jent_time_entropy_init(unsigned int osr, unsigned int flags)
{
	struct rand_data *ec;
	uint64_t *delta_history;
	int i, time_backwards = 0, count_stuck = 0, ret = 0;
	unsigned int health_test_result;

	/*
	 * Ensure over sampling rate is not too low.
	 */
	osr = ensure_osr_is_at_least_minimal(osr);

	delta_history = jent_gcd_init(JENT_POWERUP_TESTLOOPCOUNT);
	if (!delta_history)
		return EMEM;

	if (flags & JENT_FORCE_INTERNAL_TIMER)
		jent_notime_force();
	else
		flags |= JENT_DISABLE_INTERNAL_TIMER;

	/*
	 * If the start-up health tests (including the APT and RCT) are not
	 * run, then the entropy source is not 90B compliant. We could test if
	 * fips_enabled should be set using the jent_fips_enabled() function,
	 * but this can be overridden using the JENT_FORCE_FIPS flag, which
	 * isn't passed in yet. It is better to run the tests on the small
	 * amount of data that we have, which should not fail unless things
	 * are really bad.
	 */
	flags |= JENT_FORCE_FIPS;
	ec = jent_entropy_collector_alloc_internal(osr, flags);
	if (!ec) {
		ret = EMEM;
		goto out;
	}

	if (jent_notime_settick(ec)) {
		ret = EMEM;
		goto out;
	}

	/* To initialize the prior time. */
	jent_measure_jitter(ec, 0, NULL);

	/* We could perform statistical tests here, but the problem is
	 * that we only have a few loop counts to do testing. These
	 * loop counts may show some slight skew leading to false positives.
	 */

	/*
	 * We could add a check for system capabilities such as clock_getres or
	 * check for CONFIG_X86_TSC, but it does not make much sense as the
	 * following sanity checks verify that we have a high-resolution
	 * timer.
	 */
#define CLEARCACHE 100
	for (i = -CLEARCACHE; i < JENT_POWERUP_TESTLOOPCOUNT; i++) {
		uint64_t start_time = 0, end_time = 0, delta = 0;
		unsigned int stuck;

		/* Invoke core entropy collection logic */
		stuck = jent_measure_jitter(ec, 0, &delta);
		end_time = ec->prev_time;
		start_time = ec->prev_time - delta;

		/* test whether timer works */
		if (!start_time || !end_time) {
			ret = ENOTIME;
			goto out;
		}

		/*
		 * test whether timer is fine grained enough to provide
		 * delta even when called shortly after each other -- this
		 * implies that we also have a high resolution timer
		 */
		if (!delta || (end_time == start_time)) {
			ret = ECOARSETIME;
			goto out;
		}

		/*
		 * up to here we did not modify any variable that will be
		 * evaluated later, but we already performed some work. Thus we
		 * already have had an impact on the caches, branch prediction,
		 * etc. with the goal to clear it to get the worst case
		 * measurements.
		 */
		if (i < 0)
			continue;

		if (stuck)
			count_stuck++;

		/* test whether we have an increasing timer */
		if (!(end_time > start_time))
			time_backwards++;

		/* Watch for common adjacent GCD values */
		jent_gcd_add_value(delta_history, delta, i);
	}

	/*
	 * we allow up to three times the time running backwards.
	 * CLOCK_REALTIME is affected by adjtime and NTP operations. Thus,
	 * if such an operation just happens to interfere with our test, it
	 * should not fail. The value of 3 should cover the NTP case being
	 * performed during our test run.
	 */
	if (time_backwards > 3) {
		ret = ENOMONOTONIC;
		goto out;
	}

	/* First, did we encounter a health test failure? */
	if ((health_test_result = jent_health_failure(ec))) {
		ret = (health_test_result & JENT_RCT_FAILURE) ? ERCT : EHEALTH;
		goto out;
	}

	ret = jent_gcd_analyze(delta_history, JENT_POWERUP_TESTLOOPCOUNT, osr);
	if (ret)
		goto out;

	/*
	 * If we have more than 90% stuck results, then this Jitter RNG is
	 * likely to not work well.
	 */
	if (JENT_STUCK_INIT_THRES(JENT_POWERUP_TESTLOOPCOUNT) < count_stuck)
		ret = ESTUCK;

out:
	jent_gcd_fini(delta_history, JENT_POWERUP_TESTLOOPCOUNT);

	if ((flags & JENT_FORCE_INTERNAL_TIMER) && ec)
		jent_notime_unsettick(ec);

	jent_entropy_collector_free(ec);

	return ret;
}

static inline int jent_entropy_init_common_pre(void)
{
	int ret;

	jent_notime_block_switch();
	jent_health_cb_block_switch();

	if (jent_sha3_tester())
		return EHASH;

	ret = jent_gcd_selftest();

	jent_selftest_run = 1;

	return ret;
}

static inline int jent_entropy_init_common_post(int ret)
{
	/* Unmark the execution of the self tests if they failed. */
	if (ret)
		jent_selftest_run = 0;

	return ret;
}

JENT_PRIVATE_STATIC
int jent_entropy_init(void)
{
	int ret = jent_entropy_init_common_pre();

	if (ret)
		return ret;

	ret = jent_time_entropy_init(0, JENT_DISABLE_INTERNAL_TIMER);

#ifdef JENT_CONF_ENABLE_INTERNAL_TIMER
	if (ret)
		ret = jent_time_entropy_init(0, JENT_FORCE_INTERNAL_TIMER);
#endif /* JENT_CONF_ENABLE_INTERNAL_TIMER */

	return jent_entropy_init_common_post(ret);
}

JENT_PRIVATE_STATIC
int jent_entropy_init_ex(unsigned int osr, unsigned int flags)
{
	int ret = jent_entropy_init_common_pre();

	if (ret)
		return ret;

	ret = ENOTIME;

	/* Test without internal timer unless caller does not want it */
	if (!(flags & JENT_FORCE_INTERNAL_TIMER))
		ret = jent_time_entropy_init(osr,
					flags | JENT_DISABLE_INTERNAL_TIMER);

#ifdef JENT_CONF_ENABLE_INTERNAL_TIMER
	/* Test with internal timer unless caller does not want it */
	if (ret && !(flags & JENT_DISABLE_INTERNAL_TIMER))
		ret = jent_time_entropy_init(osr,
					     flags | JENT_FORCE_INTERNAL_TIMER);
#endif /* JENT_CONF_ENABLE_INTERNAL_TIMER */

	return jent_entropy_init_common_post(ret);
}

JENT_PRIVATE_STATIC
int jent_entropy_switch_notime_impl(struct jent_notime_thread *new_thread)
{
	return jent_notime_switch(new_thread);
}

JENT_PRIVATE_STATIC
int jent_set_fips_failure_callback(jent_fips_failure_cb cb)
{
	return jent_set_fips_failure_callback_internal(cb);
}
