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

#ifndef SPA_DEBUG_CONTEXT_H
#define SPA_DEBUG_CONTEXT_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdio.h>
#include <stdarg.h>

#include <spa/utils/defs.h>
/**
 * \addtogroup spa_debug
 * \{
 */

#ifndef spa_debugn
#define spa_debugn(_fmt,...)	printf((_fmt), ## __VA_ARGS__)
#endif
#ifndef spa_debug
#define spa_debug(_fmt,...)	spa_debugn(_fmt"\n", ## __VA_ARGS__)
#endif

struct spa_debug_context {
	void (*log) (struct spa_debug_context *ctx, const char *fmt, ...) SPA_PRINTF_FUNC(2, 3);
};

#define spa_debugc(_c,_fmt,...)	(_c)?((_c)->log((_c),_fmt, ## __VA_ARGS__)):(void)spa_debug(_fmt, ## __VA_ARGS__)

/**
 * \}
 */

#ifdef __cplusplus
}  /* extern "C" */
#endif

#endif /* SPA_DEBUG_CONTEXT_H */
