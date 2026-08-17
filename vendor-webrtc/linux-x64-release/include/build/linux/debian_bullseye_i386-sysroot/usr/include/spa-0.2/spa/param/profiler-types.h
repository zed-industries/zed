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

#ifndef SPA_PARAM_PROFILER_TYPES_H
#define SPA_PARAM_PROFILER_TYPES_H

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \addtogroup spa_param
 * \{
 */

#include <spa/param/param-types.h>
#include <spa/param/profiler.h>

#define SPA_TYPE_INFO_Profiler		SPA_TYPE_INFO_OBJECT_BASE "Profiler"
#define SPA_TYPE_INFO_PROFILER_BASE	SPA_TYPE_INFO_Profiler ":"

static const struct spa_type_info spa_type_profiler[] = {
	{ SPA_PROFILER_START, SPA_TYPE_Id, SPA_TYPE_INFO_PROFILER_BASE, spa_type_param, },
	{ SPA_PROFILER_info, SPA_TYPE_Struct, SPA_TYPE_INFO_PROFILER_BASE "info", NULL, },
	{ SPA_PROFILER_clock, SPA_TYPE_Struct, SPA_TYPE_INFO_PROFILER_BASE "clock", NULL, },
	{ SPA_PROFILER_driverBlock, SPA_TYPE_Struct, SPA_TYPE_INFO_PROFILER_BASE "driverBlock", NULL, },
	{ SPA_PROFILER_followerBlock, SPA_TYPE_Struct, SPA_TYPE_INFO_PROFILER_BASE "followerBlock", NULL, },
	{ 0, 0, NULL, NULL },
};

/**
 * \}
 */

#ifdef __cplusplus
}  /* extern "C" */
#endif

#endif /* SPA_PARAM_PROFILER_TYPES_H */
