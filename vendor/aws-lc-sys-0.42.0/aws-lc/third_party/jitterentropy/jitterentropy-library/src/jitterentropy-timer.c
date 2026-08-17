/* Jitter RNG: Internal timer implementation
 *
 * Copyright (C) 2021 - 2025, Stephan Mueller <smueller@chronox.de>
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
#include "jitterentropy-timer.h"

#ifdef JENT_CONF_ENABLE_INTERNAL_TIMER
/***************************************************************************
 * Thread handler
 ***************************************************************************/

#ifdef USE_OLDER_GLIBC
__asm__(".symver pthread_join, pthread_join@GLIBC_2.2.5");
__asm__(".symver pthread_create, pthread_create@GLIBC_2.2.5");
#endif

JENT_PRIVATE_STATIC
int jent_notime_init(void **ctx)
{
	struct jent_notime_ctx *thread_ctx;
	long ncpu = jent_ncpu();

	if (ncpu < 0)
		return (int)ncpu;

	/* We need at least two CPUs to enable the timer thread */
	if (ncpu < 2)
		return -EOPNOTSUPP;

	thread_ctx = calloc(1, sizeof(struct jent_notime_ctx));
	if (!thread_ctx)
		return -errno;

	*ctx = thread_ctx;

	return 0;
}

JENT_PRIVATE_STATIC
void jent_notime_fini(void *ctx)
{
	struct jent_notime_ctx *thread_ctx = (struct jent_notime_ctx *)ctx;

	if (thread_ctx)
		free(thread_ctx);
}

static int jent_notime_start(void *ctx,
			     void *(*start_routine) (void *), void *arg)
{
	struct jent_notime_ctx *thread_ctx = (struct jent_notime_ctx *)ctx;
	int ret;

	if (!thread_ctx)
		return -EINVAL;

	ret = -pthread_attr_init(&thread_ctx->notime_pthread_attr);
	if (ret)
		return ret;

	return -pthread_create(&thread_ctx->notime_thread_id,
			       &thread_ctx->notime_pthread_attr,
			       start_routine, arg);
}

static void jent_notime_stop(void *ctx)
{
	struct jent_notime_ctx *thread_ctx = (struct jent_notime_ctx *)ctx;

	pthread_join(thread_ctx->notime_thread_id, NULL);
	pthread_attr_destroy(&thread_ctx->notime_pthread_attr);
}

static struct jent_notime_thread jent_notime_thread_builtin = {
	.jent_notime_init  = jent_notime_init,
	.jent_notime_fini  = jent_notime_fini,
	.jent_notime_start = jent_notime_start,
	.jent_notime_stop  = jent_notime_stop
};

/***************************************************************************
 * Timer-less timer replacement
 *
 * If there is no high-resolution hardware timer available, we create one
 * ourselves. This logic is only used when the initialization identifies
 * that no suitable time source is available.
 ***************************************************************************/

static int jent_force_internal_timer = 0;
static int jent_notime_switch_blocked = 0;

void jent_notime_block_switch(void)
{
	jent_notime_switch_blocked = 1;
}

static struct jent_notime_thread *notime_thread = &jent_notime_thread_builtin;

/**
 * Timer-replacement loop
 *
 * @brief The measurement loop triggers the read of the value from the
 * counter function. It conceptually acts as the low resolution
 * samples timer from a ring oscillator.
 */
static void *jent_notime_sample_timer(void *arg)
{
	struct rand_data *ec = (struct rand_data *)arg;

	ec->notime_timer = 0;

	while (1) {
		if (ec->notime_interrupt)
			return NULL;

		ec->notime_timer++;
	}

	return NULL;
}

/*
 * Enable the clock: spawn a new thread that holds a counter.
 *
 * Note, although creating a thread is expensive, we do that every time a
 * caller wants entropy from us and terminate the thread afterwards. This
 * is to ensure an attacker cannot easily identify the ticking thread.
 */
int jent_notime_settick(struct rand_data *ec)
{
	if (!ec->enable_notime || !notime_thread)
		return 0;

	ec->notime_interrupt = 0;
	ec->notime_prev_timer = 0;
	ec->notime_timer = 0;

	return notime_thread->jent_notime_start(ec->notime_thread_ctx,
					       jent_notime_sample_timer, ec);
}

void jent_notime_unsettick(struct rand_data *ec)
{
	if (!ec->enable_notime || !notime_thread)
		return;

	ec->notime_interrupt = 1;
	notime_thread->jent_notime_stop(ec->notime_thread_ctx);
}

void jent_get_nstime_internal(struct rand_data *ec, uint64_t *out)
{
	if (ec->enable_notime) {
		/*
		 * Allow the counting thread to be initialized and guarantee
		 * that it ticked since last time we looked.
		 *
		 * Note, we do not use an atomic operation here for reading
		 * jent_notime_timer since if this integer is garbled, it even
		 * adds to entropy. But on most architectures, read/write
		 * of an uint64_t should be atomic anyway.
		 */
		while (ec->notime_timer == ec->notime_prev_timer)
			jent_yield();

		ec->notime_prev_timer = ec->notime_timer;
		*out = ec->notime_prev_timer;
	} else {
		jent_get_nstime(out);
	}
}

static inline int jent_notime_enable_thread(struct rand_data *ec)
{
	if (notime_thread)
		return notime_thread->jent_notime_init(&ec->notime_thread_ctx);
	return 0;
}

void jent_notime_disable(struct rand_data *ec)
{
	if (notime_thread)
		notime_thread->jent_notime_fini(ec->notime_thread_ctx);
}

int jent_notime_enable(struct rand_data *ec, unsigned int flags)
{
	/* Use internal timer */
	if (jent_force_internal_timer || (flags & JENT_FORCE_INTERNAL_TIMER)) {
		/* Self test not run yet */
		if (!jent_force_internal_timer &&
		    jent_time_entropy_init(ec->osr,
					   flags | JENT_FORCE_INTERNAL_TIMER))
			return EHEALTH;

		ec->enable_notime = 1;
		return jent_notime_enable_thread(ec);
	}

	return 0;
}

int jent_notime_switch(struct jent_notime_thread *new_thread)
{
	if (jent_notime_switch_blocked)
		return -EAGAIN;
	notime_thread = new_thread;
	return 0;
}

void jent_notime_force(void)
{
	jent_force_internal_timer = 1;
}

int jent_notime_forced(void)
{
	return jent_force_internal_timer;
}

#endif /* JENT_CONF_ENABLE_INTERNAL_TIMER */
