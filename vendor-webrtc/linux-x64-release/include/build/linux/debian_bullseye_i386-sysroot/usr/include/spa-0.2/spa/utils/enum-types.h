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

#ifndef SPA_ENUM_TYPES_H
#define SPA_ENUM_TYPES_H

#ifdef __cplusplus
extern "C" {
#endif

#include <spa/utils/type.h>

#define SPA_TYPE_INFO_Direction			SPA_TYPE_INFO_ENUM_BASE "Direction"
#define SPA_TYPE_INFO_DIRECTION_BASE		SPA_TYPE_INFO_Direction ":"

static const struct spa_type_info spa_type_direction[] = {
	{ SPA_DIRECTION_INPUT, SPA_TYPE_Int, SPA_TYPE_INFO_DIRECTION_BASE "Input", NULL  },
	{ SPA_DIRECTION_OUTPUT, SPA_TYPE_Int, SPA_TYPE_INFO_DIRECTION_BASE "Output", NULL  },
	{ 0, 0, NULL, NULL }
};

#include <spa/pod/pod.h>

#define SPA_TYPE_INFO_Choice			SPA_TYPE_INFO_ENUM_BASE "Choice"
#define SPA_TYPE_INFO_CHOICE_BASE		SPA_TYPE_INFO_Choice ":"

static const struct spa_type_info spa_type_choice[] = {
	{ SPA_CHOICE_None, SPA_TYPE_Int, SPA_TYPE_INFO_CHOICE_BASE "None", NULL  },
	{ SPA_CHOICE_Range, SPA_TYPE_Int, SPA_TYPE_INFO_CHOICE_BASE "Range", NULL  },
	{ SPA_CHOICE_Step, SPA_TYPE_Int, SPA_TYPE_INFO_CHOICE_BASE "Step", NULL  },
	{ SPA_CHOICE_Enum, SPA_TYPE_Int, SPA_TYPE_INFO_CHOICE_BASE "Enum", NULL  },
	{ SPA_CHOICE_Flags, SPA_TYPE_Int, SPA_TYPE_INFO_CHOICE_BASE "Flags", NULL  },
	{ 0, 0, NULL, NULL }
};

/**
 * \}
 */

#ifdef __cplusplus
}  /* extern "C" */
#endif

#endif /* SPA_TYPE_INFO_H */
