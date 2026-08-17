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

#ifndef SPA_PARAM_PROFILE_TYPES_H
#define SPA_PARAM_PROFILE_TYPES_H

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \addtogroup spa_param
 * \{
 */

#include <spa/param/param-types.h>

#include <spa/param/profile.h>

#define SPA_TYPE_INFO_PARAM_Profile		SPA_TYPE_INFO_PARAM_BASE "Profile"
#define SPA_TYPE_INFO_PARAM_PROFILE_BASE	SPA_TYPE_INFO_PARAM_Profile ":"

static const struct spa_type_info spa_type_param_profile[] = {
	{ SPA_PARAM_PROFILE_START, SPA_TYPE_Id, SPA_TYPE_INFO_PARAM_PROFILE_BASE, spa_type_param, },
	{ SPA_PARAM_PROFILE_index, SPA_TYPE_Int, SPA_TYPE_INFO_PARAM_PROFILE_BASE "index", NULL },
	{ SPA_PARAM_PROFILE_name, SPA_TYPE_String, SPA_TYPE_INFO_PARAM_PROFILE_BASE "name", NULL },
	{ SPA_PARAM_PROFILE_description, SPA_TYPE_String, SPA_TYPE_INFO_PARAM_PROFILE_BASE "description", NULL },
	{ SPA_PARAM_PROFILE_priority, SPA_TYPE_Int, SPA_TYPE_INFO_PARAM_PROFILE_BASE "priority", NULL },
	{ SPA_PARAM_PROFILE_available, SPA_TYPE_Id, SPA_TYPE_INFO_PARAM_PROFILE_BASE "available", spa_type_param_availability, },
	{ SPA_PARAM_PROFILE_info, SPA_TYPE_Struct, SPA_TYPE_INFO_PARAM_PROFILE_BASE "info", NULL, },
	{ SPA_PARAM_PROFILE_classes, SPA_TYPE_Struct, SPA_TYPE_INFO_PARAM_PROFILE_BASE "classes", NULL, },
	{ SPA_PARAM_PROFILE_save, SPA_TYPE_Bool, SPA_TYPE_INFO_PARAM_PROFILE_BASE "save", NULL, },
	{ 0, 0, NULL, NULL },
};

/**
 * \}
 */

#ifdef __cplusplus
}  /* extern "C" */
#endif

#endif /* SPA_PARAM_PROFILE_TYPES_H */
