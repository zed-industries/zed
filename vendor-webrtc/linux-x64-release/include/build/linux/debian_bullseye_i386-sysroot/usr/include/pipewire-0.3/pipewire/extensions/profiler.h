/* PipeWire
 *
 * Copyright © 2020 Wim Taymans
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

#ifndef PIPEWIRE_EXT_PROFILER_H
#define PIPEWIRE_EXT_PROFILER_H

#ifdef __cplusplus
extern "C" {
#endif

#include <spa/utils/defs.h>

/** \defgroup pw_profiler Profiler
 * Profiler interface
 */

/**
 * \addtogroup pw_profiler
 * \{
 */
#define PW_TYPE_INTERFACE_Profiler		PW_TYPE_INFO_INTERFACE_BASE "Profiler"

#define PW_VERSION_PROFILER			3
struct pw_profiler;

#define PW_EXTENSION_MODULE_PROFILER		PIPEWIRE_MODULE_PREFIX "module-profiler"

#define PW_PROFILER_EVENT_PROFILE		0
#define PW_PROFILER_EVENT_NUM			1

/** \ref pw_profiler events */
struct pw_profiler_events {
#define PW_VERSION_PROFILER_EVENTS		0
	uint32_t version;

	void (*profile) (void *data, const struct spa_pod *pod);
};

#define PW_PROFILER_METHOD_ADD_LISTENER		0
#define PW_PROFILER_METHOD_NUM			1

/** \ref pw_profiler methods */
struct pw_profiler_methods {
#define PW_VERSION_PROFILER_METHODS		0
	uint32_t version;

	int (*add_listener) (void *object,
			struct spa_hook *listener,
			const struct pw_profiler_events *events,
			void *data);
};

#define pw_profiler_method(o,method,version,...)			\
({									\
	int _res = -ENOTSUP;						\
	spa_interface_call_res((struct spa_interface*)o,		\
			struct pw_profiler_methods, _res,		\
			method, version, ##__VA_ARGS__);		\
	_res;								\
})

#define pw_profiler_add_listener(c,...)		pw_profiler_method(c,add_listener,0,__VA_ARGS__)

#define PW_KEY_PROFILER_NAME		"profiler.name"

/**
 * \}
 */

#ifdef __cplusplus
}  /* extern "C" */
#endif

#endif /* PIPEWIRE_EXT_PROFILER_H */
