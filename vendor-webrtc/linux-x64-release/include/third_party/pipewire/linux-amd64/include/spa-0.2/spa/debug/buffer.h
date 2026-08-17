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

#ifndef SPA_DEBUG_BUFFER_H
#define SPA_DEBUG_BUFFER_H

#ifdef __cplusplus
extern "C" {
#endif

/** \defgroup spa_debug Debug
 * Debugging utilities
 */

/**
 * \addtogroup spa_debug
 * \{
 */

#include <spa/debug/log.h>
#include <spa/debug/mem.h>
#include <spa/debug/types.h>
#include <spa/buffer/type-info.h>

static inline int spa_debug_buffer(int indent, const struct spa_buffer *buffer)
{
	uint32_t i;

	spa_debug("%*s" "struct spa_buffer %p:", indent, "", buffer);
	spa_debug("%*s" " n_metas: %u (at %p)", indent, "", buffer->n_metas, buffer->metas);
	for (i = 0; i < buffer->n_metas; i++) {
		struct spa_meta *m = &buffer->metas[i];
		const char *type_name;

		type_name = spa_debug_type_find_name(spa_type_meta_type, m->type);
		spa_debug("%*s" "  meta %d: type %d (%s), data %p, size %d:", indent, "", i, m->type,
			type_name, m->data, m->size);

		switch (m->type) {
		case SPA_META_Header:
		{
			struct spa_meta_header *h = (struct spa_meta_header*)m->data;
			spa_debug("%*s" "    struct spa_meta_header:", indent, "");
			spa_debug("%*s" "      flags:      %08x", indent, "", h->flags);
			spa_debug("%*s" "      offset:     %u", indent, "", h->offset);
			spa_debug("%*s" "      seq:        %" PRIu64, indent, "", h->seq);
			spa_debug("%*s" "      pts:        %" PRIi64, indent, "", h->pts);
			spa_debug("%*s" "      dts_offset: %" PRIi64, indent, "", h->dts_offset);
			break;
		}
		case SPA_META_VideoCrop:
		{
			struct spa_meta_region *h = (struct spa_meta_region*)m->data;
			spa_debug("%*s" "    struct spa_meta_region:", indent, "");
			spa_debug("%*s" "      x:      %d", indent, "", h->region.position.x);
			spa_debug("%*s" "      y:      %d", indent, "", h->region.position.y);
			spa_debug("%*s" "      width:  %d", indent, "", h->region.size.width);
			spa_debug("%*s" "      height: %d", indent, "", h->region.size.height);
			break;
		}
		case SPA_META_VideoDamage:
		{
			struct spa_meta_region *h;
			spa_meta_for_each(h, m) {
				spa_debug("%*s" "    struct spa_meta_region:", indent, "");
				spa_debug("%*s" "      x:      %d", indent, "", h->region.position.x);
				spa_debug("%*s" "      y:      %d", indent, "", h->region.position.y);
				spa_debug("%*s" "      width:  %d", indent, "", h->region.size.width);
				spa_debug("%*s" "      height: %d", indent, "", h->region.size.height);
			}
			break;
		}
		case SPA_META_Bitmap:
			break;
		case SPA_META_Cursor:
			break;
		default:
			spa_debug("%*s" "    Unknown:", indent, "");
			spa_debug_mem(5, m->data, m->size);
		}
	}
	spa_debug("%*s" " n_datas: \t%u (at %p)", indent, "", buffer->n_datas, buffer->datas);
	for (i = 0; i < buffer->n_datas; i++) {
		struct spa_data *d = &buffer->datas[i];
		spa_debug("%*s" "   type:    %d (%s)", indent, "", d->type,
			spa_debug_type_find_name(spa_type_data_type, d->type));
		spa_debug("%*s" "   flags:   %d", indent, "", d->flags);
		spa_debug("%*s" "   data:    %p", indent, "", d->data);
		spa_debug("%*s" "   fd:      %" PRIi64, indent, "", d->fd);
		spa_debug("%*s" "   offset:  %d", indent, "", d->mapoffset);
		spa_debug("%*s" "   maxsize: %u", indent, "", d->maxsize);
		spa_debug("%*s" "   chunk:   %p", indent, "", d->chunk);
		spa_debug("%*s" "    offset: %d", indent, "", d->chunk->offset);
		spa_debug("%*s" "    size:   %u", indent, "", d->chunk->size);
		spa_debug("%*s" "    stride: %d", indent, "", d->chunk->stride);
	}
	return 0;
}

/**
 * \}
 */

#ifdef __cplusplus
}  /* extern "C" */
#endif

#endif /* SPA_DEBUG_BUFFER_H */
