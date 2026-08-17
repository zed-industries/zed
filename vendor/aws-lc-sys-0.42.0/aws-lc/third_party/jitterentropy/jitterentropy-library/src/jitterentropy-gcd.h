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

#ifndef JITTERENTROPY_GCD_H
#define JITTERENTROPY_GCD_H

#include "jitterentropy-internal.h"

#ifdef __cplusplus
extern "C"
{
#endif

int jent_gcd_analyze(uint64_t *delta_history, size_t nelem, size_t osr);
uint64_t *jent_gcd_init(size_t nelem);
void jent_gcd_fini(uint64_t *delta_history, size_t nelem);
int jent_gcd_get(uint64_t *value);
int jent_gcd_selftest(void);

/* Watch for common adjacent GCD values */
#define jent_gcd_add_value(delta_history, delta, idx)			\
	delta_history[idx] = delta;

#ifdef __cplusplus
}
#endif

#endif /* JITTERENTROPY_GCD_H */
