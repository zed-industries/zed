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

#ifndef SPA_POD_DYNAMIC_H
#define SPA_POD_DYNAMIC_H

#ifdef __cplusplus
extern "C" {
#endif

#include <spa/pod/builder.h>

struct spa_pod_dynamic_builder {
	struct spa_pod_builder b;
	void *data;
	uint32_t extend;
	uint32_t _padding;
};

static int spa_pod_dynamic_builder_overflow(void *data, uint32_t size)
{
	struct spa_pod_dynamic_builder *d = (struct spa_pod_dynamic_builder*)data;
	int32_t old_size = d->b.size;
	int32_t new_size = SPA_ROUND_UP_N(size, d->extend);
	void *old_data = d->b.data;

	if (old_data == d->data)
		d->b.data = NULL;
	if ((d->b.data = realloc(d->b.data, new_size)) == NULL)
		return -errno;
	if (old_data == d->data && d->b.data != old_data && old_size > 0)
		memcpy(d->b.data, old_data, old_size);
	d->b.size = new_size;
        return 0;
}

static inline void spa_pod_dynamic_builder_init(struct spa_pod_dynamic_builder *builder,
		void *data, uint32_t size, uint32_t extend)
{
	static const struct spa_pod_builder_callbacks spa_pod_dynamic_builder_callbacks = {
		SPA_VERSION_POD_BUILDER_CALLBACKS,
		.overflow = spa_pod_dynamic_builder_overflow
	};
	builder->b = SPA_POD_BUILDER_INIT(data, size);
	spa_pod_builder_set_callbacks(&builder->b, &spa_pod_dynamic_builder_callbacks, builder);
	builder->extend = extend;
	builder->data = data;
}

static inline void spa_pod_dynamic_builder_clean(struct spa_pod_dynamic_builder *builder)
{
	if (builder->data != builder->b.data)
		free(builder->b.data);
}

#ifdef __cplusplus
}  /* extern "C" */
#endif

#endif /* SPA_POD_DYNAMIC_H */
