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

#ifndef PIPEWIRE_PERMISSION_H
#define PIPEWIRE_PERMISSION_H

#ifdef __cplusplus
extern "C" {
#endif

#include <spa/utils/defs.h>

/** \defgroup pw_permission Permission
 *
 * Permissions are kept for a client and describe what the client is
 * allowed to do with an object.
 *
 * See \ref page_core_api
 */

/**
 * \addtogroup pw_permission
 * \{
 */

#define PW_PERM_R	0400	/**< object can be seen and events can be received */
#define PW_PERM_W	0200	/**< methods can be called that modify the object */
#define PW_PERM_X	0100	/**< methods can be called on the object. The W flag must be
				  *  present in order to call methods that modify the object. */
#define PW_PERM_M	0010	/**< metadata can be set on object, Since 0.3.9 */

#define PW_PERM_RWX	(PW_PERM_R|PW_PERM_W|PW_PERM_X)
#define PW_PERM_RWXM	(PW_PERM_RWX|PW_PERM_M)

#define PW_PERM_IS_R(p) (((p)&PW_PERM_R) == PW_PERM_R)
#define PW_PERM_IS_W(p) (((p)&PW_PERM_W) == PW_PERM_W)
#define PW_PERM_IS_X(p) (((p)&PW_PERM_X) == PW_PERM_X)
#define PW_PERM_IS_M(p) (((p)&PW_PERM_M) == PW_PERM_M)

#define PW_PERM_ALL	PW_PERM_RWXM
#define PW_PERM_INVALID	(uint32_t)(0xffffffff)

struct pw_permission {
	uint32_t id;		/**< id of object, PW_ID_ANY for default permission */
	uint32_t permissions;	/**< bitmask of above permissions */
};

#define PW_PERMISSION_INIT(id,p) (struct pw_permission){ (id), (p) }

#define PW_PERMISSION_FORMAT "%c%c%c%c"
#define PW_PERMISSION_ARGS(permission)		\
	(permission) & PW_PERM_R ? 'r' : '-',	\
	(permission) & PW_PERM_W ? 'w' : '-',	\
	(permission) & PW_PERM_X ? 'x' : '-',	\
	(permission) & PW_PERM_M ? 'm' : '-'

/**
 * \}
 */

#ifdef __cplusplus
}
#endif

#endif /* PIPEWIRE_PERMISSION_H */
