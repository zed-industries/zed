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

#ifndef JITTERENTROPY_HEALTH_H
#define JITTERENTROPY_HEALTH_H

#include "jitterentropy-internal.h"

#ifdef __cplusplus
extern "C"
{
#endif

void jent_health_cb_block_switch(void);
int jent_set_fips_failure_callback_internal(jent_fips_failure_cb cb);

static inline uint64_t jent_delta(uint64_t prev, uint64_t next)
{
	return (next - prev);
}

#ifdef JENT_HEALTH_LAG_PREDICTOR
void jent_lag_init(struct rand_data *ec, unsigned int osr);
#else /* JENT_HEALTH_LAG_PREDICTOR */
static inline void jent_lag_init(struct rand_data *ec, unsigned int osr)
{
	(void)ec;
	(void)osr;
}
#endif /* JENT_HEALTH_LAG_PREDICTOR */

/* RCT: Intermittent cutoff threshold for alpha = 2**-30 */
#define JENT_HEALTH_RCT_INTERMITTENT_CUTOFF(x) ((x) * 30)
/* RCT: permanent cutoff threshold for alpha = 2**-60 */
#define JENT_HEALTH_RCT_PERMANENT_CUTOFF(x) ((x) * 60)

void jent_apt_init(struct rand_data *ec, unsigned int osr);
void jent_apt_reinit(struct rand_data *ec,
		     uint64_t current_delta,
		     unsigned int apt_count,
		     unsigned int apt_observations);
unsigned int jent_stuck(struct rand_data *ec, uint64_t current_delta);
unsigned int jent_health_failure(struct rand_data *ec);

#ifdef __cplusplus
}
#endif

#endif /* JITTERENTROPY_HEALTH_H */
