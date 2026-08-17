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

#ifndef SPA_EVENT_H
#define SPA_EVENT_H

#ifdef __cplusplus
extern "C" {
#endif

#include <spa/pod/pod.h>

/**
 * \addtogroup spa_pod
 * \{
 */

struct spa_event_body {
	struct spa_pod_object_body body;
};

struct spa_event {
	struct spa_pod pod;
	struct spa_event_body body;
};

#define SPA_EVENT_TYPE(ev)	((ev)->body.body.type)
#define SPA_EVENT_ID(ev,type)	(SPA_EVENT_TYPE(ev) == type ? \
					(ev)->body.body.id : SPA_ID_INVALID)

#define SPA_EVENT_INIT_FULL(t,size,type,id,...) (t)			\
	{ { size, SPA_TYPE_OBJECT },					\
	  { { type, id }, ##__VA_ARGS__ } }				\

#define SPA_EVENT_INIT(type,id)						\
	SPA_EVENT_INIT_FULL(struct spa_event,				\
			sizeof(struct spa_event_body), type, id)

/**
 * \}
 */

#ifdef __cplusplus
}  /* extern "C" */
#endif

#endif /* SPA_EVENT_H */
