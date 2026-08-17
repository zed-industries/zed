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

#ifndef SPA_BUFFER_TYPES_H
#define SPA_BUFFER_TYPES_H

/**
 * \addtogroup spa_buffer
 * \{
 */

#ifdef __cplusplus
extern "C" {
#endif

#include <spa/buffer/buffer.h>
#include <spa/buffer/meta.h>
#include <spa/utils/type.h>

#define SPA_TYPE_INFO_Buffer			SPA_TYPE_INFO_POINTER_BASE "Buffer"
#define SPA_TYPE_INFO_BUFFER_BASE		SPA_TYPE_INFO_Buffer ":"

/** Buffers contain data of a certain type */
#define SPA_TYPE_INFO_Data			SPA_TYPE_INFO_ENUM_BASE "Data"
#define SPA_TYPE_INFO_DATA_BASE			SPA_TYPE_INFO_Data ":"

/** base type for fd based memory */
#define SPA_TYPE_INFO_DATA_Fd			SPA_TYPE_INFO_DATA_BASE "Fd"
#define SPA_TYPE_INFO_DATA_FD_BASE		SPA_TYPE_INFO_DATA_Fd ":"

static const struct spa_type_info spa_type_data_type[] = {
	{ SPA_DATA_Invalid, SPA_TYPE_Int, SPA_TYPE_INFO_DATA_BASE "Invalid", NULL },
	{ SPA_DATA_MemPtr, SPA_TYPE_Int, SPA_TYPE_INFO_DATA_BASE "MemPtr", NULL },
	{ SPA_DATA_MemFd, SPA_TYPE_Int, SPA_TYPE_INFO_DATA_FD_BASE "MemFd", NULL },
	{ SPA_DATA_DmaBuf, SPA_TYPE_Int, SPA_TYPE_INFO_DATA_FD_BASE "DmaBuf", NULL },
	{ SPA_DATA_MemId, SPA_TYPE_Int, SPA_TYPE_INFO_DATA_BASE "MemId", NULL },
	{ 0, 0, NULL, NULL },
};

#define SPA_TYPE_INFO_Meta			SPA_TYPE_INFO_POINTER_BASE "Meta"
#define SPA_TYPE_INFO_META_BASE			SPA_TYPE_INFO_Meta ":"

#define SPA_TYPE_INFO_META_Array		SPA_TYPE_INFO_META_BASE "Array"
#define SPA_TYPE_INFO_META_ARRAY_BASE		SPA_TYPE_INFO_META_Array ":"

#define SPA_TYPE_INFO_META_Region		SPA_TYPE_INFO_META_BASE "Region"
#define SPA_TYPE_INFO_META_REGION_BASE		SPA_TYPE_INFO_META_Region ":"

#define SPA_TYPE_INFO_META_ARRAY_Region		SPA_TYPE_INFO_META_ARRAY_BASE "Region"
#define SPA_TYPE_INFO_META_ARRAY_REGION_BASE	SPA_TYPE_INFO_META_ARRAY_Region ":"

static const struct spa_type_info spa_type_meta_type[] = {
	{ SPA_META_Invalid, SPA_TYPE_Pointer, SPA_TYPE_INFO_META_BASE "Invalid", NULL },
	{ SPA_META_Header, SPA_TYPE_Pointer, SPA_TYPE_INFO_META_BASE "Header", NULL },
	{ SPA_META_VideoCrop, SPA_TYPE_Pointer, SPA_TYPE_INFO_META_REGION_BASE "VideoCrop", NULL },
	{ SPA_META_VideoDamage, SPA_TYPE_Pointer, SPA_TYPE_INFO_META_ARRAY_REGION_BASE "VideoDamage", NULL },
	{ SPA_META_Bitmap, SPA_TYPE_Pointer, SPA_TYPE_INFO_META_BASE "Bitmap", NULL },
	{ SPA_META_Cursor, SPA_TYPE_Pointer, SPA_TYPE_INFO_META_BASE "Cursor", NULL },
	{ SPA_META_Control, SPA_TYPE_Pointer, SPA_TYPE_INFO_META_BASE "Control", NULL },
	{ SPA_META_Busy, SPA_TYPE_Pointer, SPA_TYPE_INFO_META_BASE "Busy", NULL },
	{ 0, 0, NULL, NULL },
};

/**
 * \}
 */

#ifdef __cplusplus
}  /* extern "C" */
#endif

#endif /* SPA_BUFFER_TYPES_H */
