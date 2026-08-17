/* PipeWire
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

#ifndef PIPEWIRE_TYPE_H
#define PIPEWIRE_TYPE_H

#ifdef __cplusplus
extern "C" {
#endif

#include <spa/utils/type.h>

/** \defgroup pw_type Type info
 * Type information
 */

/**
 * \addtogroup pw_type
 * \{
 */

enum {
	PW_TYPE_FIRST = SPA_TYPE_VENDOR_PipeWire,
};

#define PW_TYPE_INFO_BASE		"PipeWire:"

#define PW_TYPE_INFO_Object		PW_TYPE_INFO_BASE "Object"
#define PW_TYPE_INFO_OBJECT_BASE	PW_TYPE_INFO_Object ":"

#define PW_TYPE_INFO_Interface		PW_TYPE_INFO_BASE "Interface"
#define PW_TYPE_INFO_INTERFACE_BASE	PW_TYPE_INFO_Interface ":"

const struct spa_type_info * pw_type_info(void);

/**
 * \}
 */

#ifdef __cplusplus
}
#endif

#endif /* PIPEWIRE_TYPE_H */
