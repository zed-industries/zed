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

#ifndef PIPEWIRE_EXT_METADATA_H
#define PIPEWIRE_EXT_METADATA_H

#ifdef __cplusplus
extern "C" {
#endif

#include <spa/utils/defs.h>

/** \defgroup pw_metadata Metadata
 * Metadata interface
 */

/**
 * \addtogroup pw_metadata
 * \{
 */
#define PW_TYPE_INTERFACE_Metadata		PW_TYPE_INFO_INTERFACE_BASE "Metadata"

#define PW_VERSION_METADATA			3
struct pw_metadata;

#define PW_EXTENSION_MODULE_METADATA		PIPEWIRE_MODULE_PREFIX "module-metadata"

#define PW_METADATA_EVENT_PROPERTY		0
#define PW_METADATA_EVENT_NUM			1

/** \ref pw_metadata events */
struct pw_metadata_events {
#define PW_VERSION_METADATA_EVENTS		0
	uint32_t version;

	int (*property) (void *data,
			uint32_t subject,
			const char *key,
			const char *type,
			const char *value);
};

#define PW_METADATA_METHOD_ADD_LISTENER		0
#define PW_METADATA_METHOD_SET_PROPERTY		1
#define PW_METADATA_METHOD_CLEAR		2
#define PW_METADATA_METHOD_NUM			3

/** \ref pw_metadata methods */
struct pw_metadata_methods {
#define PW_VERSION_METADATA_METHODS		0
	uint32_t version;

	int (*add_listener) (void *object,
			struct spa_hook *listener,
			const struct pw_metadata_events *events,
			void *data);

	int (*set_property) (void *object,
			uint32_t subject,
			const char *key,
			const char *type,
			const char *value);

	int (*clear) (void *object);
};


#define pw_metadata_method(o,method,version,...)			\
({									\
	int _res = -ENOTSUP;						\
	spa_interface_call_res((struct spa_interface*)o,		\
			struct pw_metadata_methods, _res,		\
			method, version, ##__VA_ARGS__);		\
	_res;								\
})

#define pw_metadata_add_listener(c,...)		pw_metadata_method(c,add_listener,0,__VA_ARGS__)
#define pw_metadata_set_property(c,...)		pw_metadata_method(c,set_property,0,__VA_ARGS__)
#define pw_metadata_clear(c)			pw_metadata_method(c,clear,0)

#define PW_KEY_METADATA_NAME		"metadata.name"

/**
 * \}
 */

#ifdef __cplusplus
}  /* extern "C" */
#endif

#endif /* PIPEWIRE_EXT_METADATA_H */
