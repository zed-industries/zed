/*****************************************************************************
 *
 * mtdev - Multitouch Protocol Translation Library (MIT license)
 *
 * Copyright (C) 2010 Henrik Rydberg <rydberg@euromail.se>
 * Copyright (C) 2010 Canonical Ltd.
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
 *
 ****************************************************************************/

#ifndef _MTDEV_PLUMBING_H
#define _MTDEV_PLUMBING_H

#ifdef __cplusplus
extern "C" {
#endif

#include <mtdev.h>

/**
 * mtdev_new - allocate a new mtdev
 *
 * Allocate a new mtdev.
 *
 * Returns zero in case of memory allocation failure.
 */
struct mtdev *mtdev_new(void);

/**
 * mtdev_init - initialize mtdev converter
 * @dev: the mtdev to initialize
 *
 * Sets up the internal data structures.
 *
 * Returns zero on success, negative error number otherwise.
 */
int mtdev_init(struct mtdev *dev);

/**
 * mtdev_set_mt_event - set event type
 * @dev: the mtdev in use
 * @code: the ABS_MT code to set
 * @value: boolean value
 *
 * Returns true if the given event code is present.
 */
void mtdev_set_mt_event(struct mtdev *dev, int code, int value);

/**
 * mtdev_set_abs_<property> - set abs event property
 * @dev: the mtdev in use
 * @code: the ABS_MT code to set
 */
void mtdev_set_abs_minimum(struct mtdev *dev, int code, int value);
void mtdev_set_abs_maximum(struct mtdev *dev, int code, int value);
void mtdev_set_abs_fuzz(struct mtdev *dev, int code, int value);
void mtdev_set_abs_resolution(struct mtdev *dev, int code, int value);

/**
 * mtdev_configure - configure the mtdev converter
 * @dev: the mtdev to configure
 * @fd: file descriptor of the kernel device
 *
 * Reads the device properties to set up the protocol capabilities.
 * If preferred, this can be done by hand, omitting this call.
 *
 * Returns zero on success, negative error number otherwise.
 */
int mtdev_configure(struct mtdev *dev, int fd);

/**
 * mtdev_fetch_event - fetch an event from the kernel device
 * @dev: the mtdev in use
 * @fd: file descriptor of the kernel device
 * @ev: the kernel input event to fill
 *
 * Fetch a kernel event from the kernel device. The read operation
 * behaves as dictated by the file descriptor; if O_NONBLOCK is not
 * set, the read will block until an event is available.
 *
 * On success, returns the number of events read (0 or 1). Otherwise,
 * a standard negative error number is returned.
 */
int mtdev_fetch_event(struct mtdev *dev, int fd, struct input_event *ev);

/**
 * mtdev_put_event - put an event into the converter
 * @dev: the mtdev in use
 * @ev: the kernel input event to put
 *
 * Put a kernel event into the mtdev converter. The event should
 * come straight from the device.
 *
 * This call does not block; if the buffer becomes full, older events
 * are dropped. The buffer is guaranteed to handle several complete MT
 * packets.
 */
void mtdev_put_event(struct mtdev *dev, const struct input_event *ev);

/**
 * mtdev_empty - check if there are events to get
 * @dev: the mtdev in use
 *
 * Returns true if the processed event queue is empty, false otherwise.
 */
int mtdev_empty(struct mtdev *dev);

/**
 * mtdev_get_event - get processed events from mtdev
 * @dev: the mtdev in use
 * @ev: the input event to fill
 *
 * Get a processed event from mtdev. The events appear as if they came
 * from a type B device emitting MT slot events.
 *
 * The queue must be non-empty before calling this function.
 */
void mtdev_get_event(struct mtdev *dev, struct input_event* ev);

/**
 * mtdev_delete - free a previously allocated mtdev
 *
 * Frees the memory associated with the mtdev and invalidates the
 * mtdev pointer.
 */
void mtdev_delete(struct mtdev *dev);

#ifdef __cplusplus
}
#endif

#endif
