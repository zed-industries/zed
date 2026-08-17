/* Simple Plugin API
 *
 * Copyright © 2020 Wim Taymans
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice (including the next
 * paragraph) shall be included in all copies or substantial portions of the
 * Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 * DEALINGS IN THE SOFTWARE.
 */

#ifndef SPA_PARAM_PROFILER_H
#define SPA_PARAM_PROFILER_H

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \addtogroup spa_param
 * \{
 */

#include <spa/param/param.h>

/** properties for SPA_TYPE_OBJECT_Profiler */
enum spa_profiler {
	SPA_PROFILER_START,

	SPA_PROFILER_START_Driver	= 0x10000,	/**< driver related profiler properties */
	SPA_PROFILER_info,				/**< Generic info, counter and CPU load,
							  * (Struct(
							  *      Long : counter,
							  *      Float : cpu_load fast,
							  *      Float : cpu_load medium,
							  *      Float : cpu_load slow),
							  *      Int : xrun-count))  */
	SPA_PROFILER_clock,				/**< clock information
							  *  (Struct(
							  *      Int : clock flags,
							  *      Int : clock id,
							  *      String: clock name,
							  *      Long : clock nsec,
							  *      Fraction : clock rate,
							  *      Long : clock position,
							  *      Long : clock duration,
							  *      Long : clock delay,
							  *      Double : clock rate_diff,
							  *      Long : clock next_nsec)) */
	SPA_PROFILER_driverBlock,			/**< generic driver info block
							  *  (Struct(
							  *      Int : driver_id,
							  *      String : name,
							  *      Long : driver prev_signal,
							  *      Long : driver signal,
							  *      Long : driver awake,
							  *      Long : driver finish,
							  *      Int : driver status),
							  *      Fraction : latency))  */

	SPA_PROFILER_START_Follower	= 0x20000,	/**< follower related profiler properties */
	SPA_PROFILER_followerBlock,			/**< generic follower info block
							  *  (Struct(
							  *      Int : id,
							  *      String : name,
							  *      Long : prev_signal,
							  *      Long : signal,
							  *      Long : awake,
							  *      Long : finish,
							  *      Int : status,
							  *      Fraction : latency))  */

	SPA_PROFILER_START_CUSTOM	= 0x1000000,
};

/**
 * \}
 */

#ifdef __cplusplus
}  /* extern "C" */
#endif

#endif /* SPA_PARAM_PROFILER_H */
