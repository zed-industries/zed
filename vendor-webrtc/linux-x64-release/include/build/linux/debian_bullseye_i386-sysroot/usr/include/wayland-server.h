/*
 * Copyright © 2008 Kristian Høgsberg
 *
 * Permission is hereby granted, free of charge, to any person obtaining
 * a copy of this software and associated documentation files (the
 * "Software"), to deal in the Software without restriction, including
 * without limitation the rights to use, copy, modify, merge, publish,
 * distribute, sublicense, and/or sell copies of the Software, and to
 * permit persons to whom the Software is furnished to do so, subject to
 * the following conditions:
 *
 * The above copyright notice and this permission notice (including the
 * next paragraph) shall be included in all copies or substantial
 * portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT.  IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
 * BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
 * ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

/** \file
 *
 *  \brief Include the server API, deprecations and protocol C API.
 *
 *  \warning Use of this header file is discouraged. Prefer including
 *  wayland-server-core.h instead, which does not include the
 *  server protocol header and as such only defines the library
 *  API, excluding the deprecated API below.
 */

#ifndef WAYLAND_SERVER_H
#define WAYLAND_SERVER_H

#include <stdint.h>
#include "wayland-server-core.h"

#ifdef  __cplusplus
extern "C" {
#endif

/*
 * The user can set this macro to hide the wl_object, wl_resource and wl_buffer
 * objects alongside the associated API.
 *
 * The structs were meant to be opaque, although we missed that in the early days.
 *
 * NOTE: the list of structs, functions, etc in this section MUST NEVER GROW.
 * Otherwise we will break forward compatibility and applications that used to
 * build fine will no longer be able to do so.
 */
#ifndef WL_HIDE_DEPRECATED

struct wl_object {
	const struct wl_interface *interface;
	const void *implementation;
	uint32_t id;
};

struct wl_resource {
	struct wl_object object;
	wl_resource_destroy_func_t destroy;
	struct wl_list link;
	struct wl_signal destroy_signal;
	struct wl_client *client;
	void *data;
};

uint32_t
wl_client_add_resource(struct wl_client *client,
		       struct wl_resource *resource) WL_DEPRECATED;

struct wl_resource *
wl_client_add_object(struct wl_client *client,
		     const struct wl_interface *interface,
		     const void *implementation,
		     uint32_t id, void *data) WL_DEPRECATED;

struct wl_resource *
wl_client_new_object(struct wl_client *client,
		     const struct wl_interface *interface,
		     const void *implementation, void *data) WL_DEPRECATED;

struct wl_global *
wl_display_add_global(struct wl_display *display,
		      const struct wl_interface *interface,
		      void *data,
		      wl_global_bind_func_t bind) WL_DEPRECATED;

void
wl_display_remove_global(struct wl_display *display,
			 struct wl_global *global) WL_DEPRECATED;

#endif

#ifdef  __cplusplus
}
#endif

#include "wayland-server-protocol.h"

#endif
