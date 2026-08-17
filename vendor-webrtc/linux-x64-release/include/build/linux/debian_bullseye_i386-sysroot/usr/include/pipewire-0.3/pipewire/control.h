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

#ifndef PIPEWIRE_CONTROL_H
#define PIPEWIRE_CONTROL_H

#ifdef __cplusplus
extern "C" {
#endif

#include <spa/utils/hook.h>

/** \defgroup pw_control Control
 *
 * \brief A control can be used to control a port property.
 */

/**
 * \addtogroup pw_control
 * \{
 */
struct pw_control;

#include <pipewire/impl.h>

/** Port events, use \ref pw_control_add_listener */
struct pw_control_events {
#define PW_VERSION_CONTROL_EVENTS 0
	uint32_t version;

	/** The control is destroyed */
	void (*destroy) (void *data);

	/** The control is freed */
	void (*free) (void *data);

	/** control is linked to another control */
	void (*linked) (void *data, struct pw_control *other);
	/** control is unlinked from another control */
	void (*unlinked) (void *data, struct pw_control *other);

};

/** Get the control parent port or NULL when not set */
struct pw_impl_port *pw_control_get_port(struct pw_control *control);

/** Add an event listener on the control */
void pw_control_add_listener(struct pw_control *control,
			     struct spa_hook *listener,
			     const struct pw_control_events *events,
			     void *data);

/**
 * \}
 */

#ifdef __cplusplus
}
#endif

#endif /* PIPEWIRE_CONTROL_H */
