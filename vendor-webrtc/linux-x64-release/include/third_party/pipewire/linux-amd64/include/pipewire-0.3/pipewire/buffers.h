/* PipeWire
 *
 * Copyright © 2019 Wim Taymans
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

#ifndef PIPEWIRE_BUFFERS_H
#define PIPEWIRE_BUFFERS_H

#include <spa/node/node.h>

#include <pipewire/context.h>
#include <pipewire/mem.h>

#ifdef __cplusplus
extern "C" {
#endif

/** \defgroup pw_buffers Buffers
 * Buffer handling
 */

/**
 * \addtogroup pw_buffers
 * \{
 */

#define PW_BUFFERS_FLAG_NONE		0
#define PW_BUFFERS_FLAG_NO_MEM		(1<<0)	/**< don't allocate buffer memory */
#define PW_BUFFERS_FLAG_SHARED		(1<<1)	/**< buffers can be shared */
#define PW_BUFFERS_FLAG_DYNAMIC		(1<<2)	/**< buffers have dynamic data */
#define PW_BUFFERS_FLAG_SHARED_MEM	(1<<3)	/**< buffers need shared memory */

struct pw_buffers {
	struct pw_memblock *mem;	/**< allocated buffer memory */
	struct spa_buffer **buffers;	/**< port buffers */
	uint32_t n_buffers;		/**< number of port buffers */
	uint32_t flags;			/**< flags */
};

int pw_buffers_negotiate(struct pw_context *context, uint32_t flags,
		struct spa_node *outnode, uint32_t out_port_id,
		struct spa_node *innode, uint32_t in_port_id,
		struct pw_buffers *result);

void pw_buffers_clear(struct pw_buffers *buffers);

/**
 * \}
 */

#ifdef __cplusplus
}
#endif

#endif /* PIPEWIRE_BUFFERS_H */
