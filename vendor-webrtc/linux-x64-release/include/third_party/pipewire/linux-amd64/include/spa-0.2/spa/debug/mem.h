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

#ifndef SPA_DEBUG_MEM_H
#define SPA_DEBUG_MEM_H

#ifdef __cplusplus
extern "C" {
#endif

#include <inttypes.h>

/**
 * \addtogroup spa_debug
 * \{
 */

#include <spa/debug/log.h>

static inline int spa_debug_mem(int indent, const void *data, size_t size)
{
	const uint8_t *t = (const uint8_t*)data;
	char buffer[512];
	size_t i;
	int pos = 0;

	for (i = 0; i < size; i++) {
		if (i % 16 == 0)
			pos = sprintf(buffer, "%p: ", &t[i]);
		pos += sprintf(buffer + pos, "%02x ", t[i]);
		if (i % 16 == 15 || i == size - 1) {
			spa_debug("%*s" "%s", indent, "", buffer);
		}
	}
	return 0;
}

/**
 * \}
 */

#ifdef __cplusplus
}  /* extern "C" */
#endif

#endif /* SPA_DEBUG_MEM_H */
