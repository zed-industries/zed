/* Simple Plugin API
 *
 * Copyright © 2018 Wim Taymans
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

#ifndef SPA_PARAM_LATENCY_TYPES_H
#define SPA_PARAM_LATENCY_TYPES_H

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \addtogroup spa_param
 * \{
 */

#include <spa/utils/enum-types.h>
#include <spa/param/param-types.h>
#include <spa/param/latency.h>

#define SPA_TYPE_INFO_PARAM_Latency		SPA_TYPE_INFO_PARAM_BASE "Latency"
#define SPA_TYPE_INFO_PARAM_LATENCY_BASE	SPA_TYPE_INFO_PARAM_Latency ":"

static const struct spa_type_info spa_type_param_latency[] = {
	{ SPA_PARAM_LATENCY_START, SPA_TYPE_Id, SPA_TYPE_INFO_PARAM_LATENCY_BASE, spa_type_param, },
	{ SPA_PARAM_LATENCY_direction, SPA_TYPE_Id, SPA_TYPE_INFO_PARAM_LATENCY_BASE "direction", spa_type_direction, },
	{ SPA_PARAM_LATENCY_minQuantum, SPA_TYPE_Float, SPA_TYPE_INFO_PARAM_LATENCY_BASE "minQuantum", NULL, },
	{ SPA_PARAM_LATENCY_maxQuantum, SPA_TYPE_Float, SPA_TYPE_INFO_PARAM_LATENCY_BASE "maxQuantum", NULL, },
	{ SPA_PARAM_LATENCY_minRate, SPA_TYPE_Int, SPA_TYPE_INFO_PARAM_LATENCY_BASE "minRate", NULL, },
	{ SPA_PARAM_LATENCY_maxRate, SPA_TYPE_Int, SPA_TYPE_INFO_PARAM_LATENCY_BASE "maxRate", NULL, },
	{ SPA_PARAM_LATENCY_minNs, SPA_TYPE_Long, SPA_TYPE_INFO_PARAM_LATENCY_BASE "minNs", NULL, },
	{ SPA_PARAM_LATENCY_maxNs, SPA_TYPE_Long, SPA_TYPE_INFO_PARAM_LATENCY_BASE "maxNs", NULL, },
	{ 0, 0, NULL, NULL },
};

#define SPA_TYPE_INFO_PARAM_ProcessLatency		SPA_TYPE_INFO_PARAM_BASE "ProcessLatency"
#define SPA_TYPE_INFO_PARAM_PROCESS_LATENCY_BASE	SPA_TYPE_INFO_PARAM_ProcessLatency ":"

static const struct spa_type_info spa_type_param_process_latency[] = {
	{ SPA_PARAM_PROCESS_LATENCY_START, SPA_TYPE_Id, SPA_TYPE_INFO_PARAM_LATENCY_BASE, spa_type_param, },
	{ SPA_PARAM_PROCESS_LATENCY_quantum, SPA_TYPE_Float, SPA_TYPE_INFO_PARAM_PROCESS_LATENCY_BASE "quantum", NULL, },
	{ SPA_PARAM_PROCESS_LATENCY_rate, SPA_TYPE_Int, SPA_TYPE_INFO_PARAM_PROCESS_LATENCY_BASE "rate", NULL, },
	{ SPA_PARAM_PROCESS_LATENCY_ns, SPA_TYPE_Long, SPA_TYPE_INFO_PARAM_PROCESS_LATENCY_BASE "ns", NULL, },
	{ 0, 0, NULL, NULL },
};

/**
 * \}
 */

#ifdef __cplusplus
}  /* extern "C" */
#endif

#endif /* SPA_PARAM_LATENCY_TYPES_H */
