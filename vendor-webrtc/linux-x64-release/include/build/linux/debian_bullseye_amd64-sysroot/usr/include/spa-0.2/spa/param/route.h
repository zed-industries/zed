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

#ifndef SPA_PARAM_ROUTE_H
#define SPA_PARAM_ROUTE_H

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \addtogroup spa_param
 * \{
 */

#include <spa/param/param.h>

/** properties for SPA_TYPE_OBJECT_ParamRoute */
enum spa_param_route {
	SPA_PARAM_ROUTE_START,
	SPA_PARAM_ROUTE_index,			/**< index of the routing destination (Int) */
	SPA_PARAM_ROUTE_direction,		/**< direction, input/output (Id enum spa_direction) */
	SPA_PARAM_ROUTE_device,			/**< device id (Int) */
	SPA_PARAM_ROUTE_name,			/**< name of the routing destination (String) */
	SPA_PARAM_ROUTE_description,		/**< description of the destination (String) */
	SPA_PARAM_ROUTE_priority,		/**< priority of the destination (Int) */
	SPA_PARAM_ROUTE_available,		/**< availability of the destination
						  *  (Id enum spa_param_availability) */
	SPA_PARAM_ROUTE_info,			/**< info (Struct(
						  *		  Int : n_items,
						  *		  (String : key,
						  *		   String : value)*)) */
	SPA_PARAM_ROUTE_profiles,		/**< associated profile indexes (Array of Int) */
	SPA_PARAM_ROUTE_props,			/**< properties SPA_TYPE_OBJECT_Props */
	SPA_PARAM_ROUTE_devices,		/**< associated device indexes (Array of Int) */
	SPA_PARAM_ROUTE_profile,		/**< profile id (Int) */
	SPA_PARAM_ROUTE_save,			/**< If route should be saved (Bool) */
};

/**
 * \}
 */

#ifdef __cplusplus
}  /* extern "C" */
#endif

#endif /* SPA_PARAM_ROUTE_H */
