/* PipeWire
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

#ifndef PIPEWIRE_WORK_QUEUE_H
#define PIPEWIRE_WORK_QUEUE_H

#ifdef __cplusplus
extern "C" {
#endif

/** \defgroup pw_work_queue Work Queue
 * Queued processing of work items.
 */

/**
 * \addtogroup pw_work_queue
 * \{
 */
struct pw_work_queue;

#include <pipewire/loop.h>

typedef void (*pw_work_func_t) (void *obj, void *data, int res, uint32_t id);

struct pw_work_queue *
pw_work_queue_new(struct pw_loop *loop);

void
pw_work_queue_destroy(struct pw_work_queue *queue);

uint32_t
pw_work_queue_add(struct pw_work_queue *queue,
		  void *obj, int res,
		  pw_work_func_t func, void *data);

int
pw_work_queue_cancel(struct pw_work_queue *queue, void *obj, uint32_t id);

int
pw_work_queue_complete(struct pw_work_queue *queue, void *obj, uint32_t seq, int res);

/**
 * \}
 */

#ifdef __cplusplus
}
#endif

#endif /* PIPEWIRE_WORK_QUEUE_H */
