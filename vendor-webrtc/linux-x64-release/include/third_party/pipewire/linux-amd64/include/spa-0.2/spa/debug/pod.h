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

#ifndef SPA_DEBUG_POD_H
#define SPA_DEBUG_POD_H

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \addtogroup spa_debug
 * \{
 */

#include <spa/debug/log.h>
#include <spa/debug/mem.h>
#include <spa/debug/types.h>
#include <spa/pod/pod.h>
#include <spa/pod/iter.h>

static inline int
spa_debug_pod_value(int indent, const struct spa_type_info *info,
		uint32_t type, void *body, uint32_t size)
{
	switch (type) {
	case SPA_TYPE_Bool:
		spa_debug("%*s" "Bool %s", indent, "", (*(int32_t *) body) ? "true" : "false");
		break;
	case SPA_TYPE_Id:
		spa_debug("%*s" "Id %-8d (%s)", indent, "", *(int32_t *) body,
		       spa_debug_type_find_name(info, *(int32_t *) body));
		break;
	case SPA_TYPE_Int:
		spa_debug("%*s" "Int %d", indent, "", *(int32_t *) body);
		break;
	case SPA_TYPE_Long:
		spa_debug("%*s" "Long %" PRIi64 "", indent, "", *(int64_t *) body);
		break;
	case SPA_TYPE_Float:
		spa_debug("%*s" "Float %f", indent, "", *(float *) body);
		break;
	case SPA_TYPE_Double:
		spa_debug("%*s" "Double %f", indent, "", *(double *) body);
		break;
	case SPA_TYPE_String:
		spa_debug("%*s" "String \"%s\"", indent, "", (char *) body);
		break;
	case SPA_TYPE_Fd:
		spa_debug("%*s" "Fd %d", indent, "", *(int *) body);
		break;
	case SPA_TYPE_Pointer:
	{
		struct spa_pod_pointer_body *b = (struct spa_pod_pointer_body *)body;
		spa_debug("%*s" "Pointer %s %p", indent, "",
		       spa_debug_type_find_name(SPA_TYPE_ROOT, b->type), b->value);
		break;
	}
	case SPA_TYPE_Rectangle:
	{
		struct spa_rectangle *r = (struct spa_rectangle *)body;
		spa_debug("%*s" "Rectangle %dx%d", indent, "", r->width, r->height);
		break;
	}
	case SPA_TYPE_Fraction:
	{
		struct spa_fraction *f = (struct spa_fraction *)body;
		spa_debug("%*s" "Fraction %d/%d", indent, "", f->num, f->denom);
		break;
	}
	case SPA_TYPE_Bitmap:
		spa_debug("%*s" "Bitmap", indent, "");
		break;
	case SPA_TYPE_Array:
	{
		struct spa_pod_array_body *b = (struct spa_pod_array_body *)body;
		void *p;
		const struct spa_type_info *ti = spa_debug_type_find(SPA_TYPE_ROOT, b->child.type);

		spa_debug("%*s" "Array: child.size %d, child.type %s", indent, "",
		       b->child.size, ti ? ti->name : "unknown");

		info = info && info->values ? info->values : info;
		SPA_POD_ARRAY_BODY_FOREACH(b, size, p)
			spa_debug_pod_value(indent + 2, info, b->child.type, p, b->child.size);
		break;
	}
	case SPA_TYPE_Choice:
	{
		struct spa_pod_choice_body *b = (struct spa_pod_choice_body *)body;
		void *p;
		const struct spa_type_info *ti = spa_debug_type_find(spa_type_choice, b->type);

		spa_debug("%*s" "Choice: type %s, flags %08x %d %d", indent, "",
		       ti ? ti->name : "unknown", b->flags, size, b->child.size);

		SPA_POD_CHOICE_BODY_FOREACH(b, size, p)
			spa_debug_pod_value(indent + 2, info, b->child.type, p, b->child.size);
		break;
	}
	case SPA_TYPE_Struct:
	{
		struct spa_pod *b = (struct spa_pod *)body, *p;
		spa_debug("%*s" "Struct: size %d", indent, "", size);
		SPA_POD_FOREACH(b, size, p)
			spa_debug_pod_value(indent + 2, info, p->type, SPA_POD_BODY(p), p->size);
		break;
	}
	case SPA_TYPE_Object:
	{
		struct spa_pod_object_body *b = (struct spa_pod_object_body *)body;
		struct spa_pod_prop *p;
		const struct spa_type_info *ti, *ii;

		ti = spa_debug_type_find(info, b->type);
		ii = ti ? spa_debug_type_find(ti->values, 0) : NULL;
		ii = ii ? spa_debug_type_find(ii->values, b->id) : NULL;

		spa_debug("%*s" "Object: size %d, type %s (%d), id %s (%d)", indent, "", size,
		       ti ? ti->name : "unknown", b->type, ii ? ii->name : "unknown", b->id);

		info = ti ? ti->values : info;

		SPA_POD_OBJECT_BODY_FOREACH(b, size, p) {
			ii = spa_debug_type_find(info, p->key);

			spa_debug("%*s" "Prop: key %s (%d), flags %08x", indent+2, "",
					ii ? ii->name : "unknown", p->key, p->flags);

			spa_debug_pod_value(indent + 4, ii ? ii->values : NULL,
					p->value.type,
					SPA_POD_CONTENTS(struct spa_pod_prop, p),
					p->value.size);
		}
		break;
	}
	case SPA_TYPE_Sequence:
	{
		struct spa_pod_sequence_body *b = (struct spa_pod_sequence_body *)body;
		const struct spa_type_info *ti, *ii;
		struct spa_pod_control *c;

		ti = spa_debug_type_find(info, b->unit);

		spa_debug("%*s" "Sequence: size %d, unit %s", indent, "", size,
		       ti ? ti->name : "unknown");

		SPA_POD_SEQUENCE_BODY_FOREACH(b, size, c) {
			ii = spa_debug_type_find(spa_type_control, c->type);

			spa_debug("%*s" "Control: offset %d, type %s", indent+2, "",
					c->offset, ii ? ii->name : "unknown");

			spa_debug_pod_value(indent + 4, ii ? ii->values : NULL,
					c->value.type,
					SPA_POD_CONTENTS(struct spa_pod_control, c),
					c->value.size);
		}
		break;
	}
	case SPA_TYPE_Bytes:
		spa_debug("%*s" "Bytes", indent, "");
		spa_debug_mem(indent + 2, body, size);
		break;
	case SPA_TYPE_None:
		spa_debug("%*s" "None", indent, "");
		spa_debug_mem(indent + 2, body, size);
		break;
	default:
		spa_debug("%*s" "unhandled POD type %d", indent, "", type);
		break;
	}
	return 0;
}

static inline int spa_debug_pod(int indent,
		const struct spa_type_info *info, const struct spa_pod *pod)
{
	return spa_debug_pod_value(indent, info ? info : SPA_TYPE_ROOT,
			SPA_POD_TYPE(pod),
			SPA_POD_BODY(pod),
			SPA_POD_BODY_SIZE(pod));
}

/**
 * \}
 */


#ifdef __cplusplus
}  /* extern "C" */
#endif

#endif /* SPA_DEBUG_POD_H */
