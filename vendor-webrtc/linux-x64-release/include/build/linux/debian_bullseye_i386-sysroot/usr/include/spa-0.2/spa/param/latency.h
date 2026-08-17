/* Simple Plugin API
 *
 * Copyright © 2023 Wim Taymans
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

#ifndef SPA_PARAM_LATENY_H
#define SPA_PARAM_LATENY_H

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \addtogroup spa_param
 * \{
 */

#include <spa/param/param.h>

/** properties for SPA_TYPE_OBJECT_ParamLatency */
enum spa_param_latency {
	SPA_PARAM_LATENCY_START,
	SPA_PARAM_LATENCY_direction,		/**< direction, input/output (Id enum spa_direction) */
	SPA_PARAM_LATENCY_minQuantum,		/**< min latency relative to quantum (Float) */
	SPA_PARAM_LATENCY_maxQuantum,		/**< max latency relative to quantum (Float) */
	SPA_PARAM_LATENCY_minRate,		/**< min latency (Int) relative to rate */
	SPA_PARAM_LATENCY_maxRate,		/**< max latency (Int) relative to rate */
	SPA_PARAM_LATENCY_minNs,		/**< min latency (Long) in nanoseconds */
	SPA_PARAM_LATENCY_maxNs,		/**< max latency (Long) in nanoseconds */
};

/** helper structure for managing latency objects */
struct spa_latency_info {
	enum spa_direction direction;
	float min_quantum;
	float max_quantum;
	uint32_t min_rate;
	uint32_t max_rate;
	uint64_t min_ns;
	uint64_t max_ns;
};

#define SPA_LATENCY_INFO(dir,...) ((struct spa_latency_info) { .direction = (dir), ## __VA_ARGS__ })

/** properties for SPA_TYPE_OBJECT_ParamProcessLatency */
enum spa_param_process_latency {
	SPA_PARAM_PROCESS_LATENCY_START,
	SPA_PARAM_PROCESS_LATENCY_quantum,	/**< latency relative to quantum (Float) */
	SPA_PARAM_PROCESS_LATENCY_rate,		/**< latency (Int) relative to rate */
	SPA_PARAM_PROCESS_LATENCY_ns,		/**< latency (Long) in nanoseconds */
};

/** Helper structure for managing process latency objects */
struct spa_process_latency_info {
	float quantum;
	uint32_t rate;
	uint64_t ns;
};

#define SPA_PROCESS_LATENCY_INFO_INIT(...)	((struct spa_process_latency_info) { __VA_ARGS__ })

/**
 * \}
 */

#ifdef __cplusplus
}  /* extern "C" */
#endif

#endif /* SPA_PARAM_LATENY_H */
