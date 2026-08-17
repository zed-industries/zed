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

#ifndef SPA_DEBUG_TYPES_H
#define SPA_DEBUG_TYPES_H

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \addtogroup spa_debug
 * \{
 */

#include <spa/utils/type-info.h>

#include <string.h>

static inline const struct spa_type_info *spa_debug_type_find(const struct spa_type_info *info, uint32_t type)
{
	const struct spa_type_info *res;

	if (info == NULL)
		info = SPA_TYPE_ROOT;

	while (info && info->name) {
		if (info->type == SPA_ID_INVALID) {
			if (info->values && (res = spa_debug_type_find(info->values, type)))
				return res;
		}
		else if (info->type == type)
			return info;
		info++;
	}
	return NULL;
}

static inline const char *spa_debug_type_short_name(const char *name)
{
	const char *h;
	if ((h = strrchr(name, ':')) != NULL)
		name = h + 1;
	return name;
}

static inline const char *spa_debug_type_find_name(const struct spa_type_info *info, uint32_t type)
{
	if ((info = spa_debug_type_find(info, type)) == NULL)
		return NULL;
	return info->name;
}

static inline const char *spa_debug_type_find_short_name(const struct spa_type_info *info, uint32_t type)
{
	const char *str;
	if ((str = spa_debug_type_find_name(info, type)) == NULL)
		return NULL;
	return spa_debug_type_short_name(str);
}

static inline uint32_t spa_debug_type_find_type(const struct spa_type_info *info, const char *name)
{
	if (info == NULL)
		info = SPA_TYPE_ROOT;

	while (info && info->name) {
		uint32_t res;
		if (strcmp(info->name, name) == 0)
			return info->type;
		if (info->values && (res = spa_debug_type_find_type(info->values, name)) != SPA_ID_INVALID)
			return res;
		info++;
	}
	return SPA_ID_INVALID;
}

static inline const struct spa_type_info *spa_debug_type_find_short(const struct spa_type_info *info, const char *name)
{
	while (info && info->name) {
		if (strcmp(spa_debug_type_short_name(info->name), name) == 0)
			return info;
		if (strcmp(info->name, name) == 0)
			return info;
		if (info->type != 0 && info->type == (uint32_t)atoi(name))
			return info;
		info++;
	}
	return NULL;
}

static inline uint32_t spa_debug_type_find_type_short(const struct spa_type_info *info, const char *name)
{
	if ((info = spa_debug_type_find_short(info, name)) == NULL)
		return SPA_ID_INVALID;
	return info->type;
}
/**
 * \}
 */

#ifdef __cplusplus
}  /* extern "C" */
#endif

#endif /* SPA_DEBUG_NODE_H */
