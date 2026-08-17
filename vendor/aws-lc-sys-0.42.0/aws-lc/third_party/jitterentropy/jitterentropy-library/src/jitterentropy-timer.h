/*
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

#ifndef JITTERENTROPY_TIMER_H
#define JITTERENTROPY_TIMER_H

#include "jitterentropy-internal.h"

#ifdef __cplusplus
extern "C"
{
#endif

#ifdef JENT_CONF_ENABLE_INTERNAL_TIMER

void jent_notime_block_switch(void);
int jent_notime_settick(struct rand_data *ec);
void jent_notime_unsettick(struct rand_data *ec);
void jent_get_nstime_internal(struct rand_data *ec, uint64_t *out);
int jent_notime_enable(struct rand_data *ec, unsigned int flags);
void jent_notime_disable(struct rand_data *ec);
int jent_notime_switch(struct jent_notime_thread *new_thread);
void jent_notime_force(void);
int jent_notime_forced(void);

#else /* JENT_CONF_ENABLE_INTERNAL_TIMER */

static inline void jent_notime_block_switch(void) { }

static inline int jent_notime_settick(struct rand_data *ec)
{
	(void)ec;
	return 0;
}

static inline void jent_notime_unsettick(struct rand_data *ec) { (void)ec; }

static inline void jent_get_nstime_internal(struct rand_data *ec, uint64_t *out)
{
	(void)ec;
	jent_get_nstime(out);
}

static inline int jent_notime_enable(struct rand_data *ec, unsigned int flags)
{
	(void)ec;

	/* If we force the timer-less noise source, we return an error */
	if (flags & JENT_FORCE_INTERNAL_TIMER)
		return EHEALTH;

	return 0;
}

static inline void jent_notime_disable(struct rand_data *ec)
{
	(void)ec;
}

static inline int jent_notime_switch(struct jent_notime_thread *new_thread)
{
	(void)new_thread;
	return -EOPNOTSUPP;
}

static inline void jent_notime_force(void) { }

static inline int jent_notime_forced(void) { return 0; }

#endif /* JENT_CONF_ENABLE_INTERNAL_TIMER */

#ifdef __cplusplus
}
#endif

#endif /* JITTERENTROPY-TIMER_H */
