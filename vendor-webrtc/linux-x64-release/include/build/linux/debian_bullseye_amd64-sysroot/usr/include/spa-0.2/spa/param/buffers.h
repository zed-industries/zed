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

#ifndef SPA_PARAM_BUFFERS_H
#define SPA_PARAM_BUFFERS_H

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \addtogroup spa_param
 * \{
 */

#include <spa/param/param.h>

/** properties for SPA_TYPE_OBJECT_ParamBuffers */
enum spa_param_buffers {
	SPA_PARAM_BUFFERS_START,
	SPA_PARAM_BUFFERS_buffers,	/**< number of buffers (Int) */
	SPA_PARAM_BUFFERS_blocks,	/**< number of data blocks per buffer (Int) */
	SPA_PARAM_BUFFERS_size,		/**< size of a data block memory (Int)*/
	SPA_PARAM_BUFFERS_stride,	/**< stride of data block memory (Int) */
	SPA_PARAM_BUFFERS_align,	/**< alignment of data block memory (Int) */
	SPA_PARAM_BUFFERS_dataType,	/**< possible memory types (Int, mask of enum spa_data_type) */
};

/** properties for SPA_TYPE_OBJECT_ParamMeta */
enum spa_param_meta {
	SPA_PARAM_META_START,
	SPA_PARAM_META_type,	/**< the metadata, one of enum spa_meta_type (Id enum spa_meta_type) */
	SPA_PARAM_META_size,	/**< the expected maximum size the meta (Int) */
};

/** properties for SPA_TYPE_OBJECT_ParamIO */
enum spa_param_io {
	SPA_PARAM_IO_START,
	SPA_PARAM_IO_id,	/**< type ID, uniquely identifies the io area (Id enum spa_io_type) */
	SPA_PARAM_IO_size,	/**< size of the io area (Int) */
};

/**
 * \}
 */

#ifdef __cplusplus
}  /* extern "C" */
#endif

#endif /* SPA_PARAM_BUFFERS_H */
