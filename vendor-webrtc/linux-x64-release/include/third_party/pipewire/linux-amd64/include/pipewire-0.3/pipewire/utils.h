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

#ifndef PIPEWIRE_UTILS_H
#define PIPEWIRE_UTILS_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdlib.h>
#include <string.h>
#include <sys/un.h>
#ifndef _POSIX_C_SOURCE
# include <sys/mount.h>
#endif

#include <spa/utils/defs.h>
#include <spa/pod/pod.h>

/** \defgroup pw_utils Utilities
 *
 * Various utility functions
 */

/**
 * \addtogroup pw_utils
 * \{
 */

/** a function to destroy an item */
typedef void (*pw_destroy_t) (void *object);

const char *
pw_split_walk(const char *str, const char *delimiter, size_t *len, const char **state);

char **
pw_split_strv(const char *str, const char *delimiter, int max_tokens, int *n_tokens);

void
pw_free_strv(char **str);

char *
pw_strip(char *str, const char *whitespace);

#if !defined(strndupa)
# define strndupa(s, n)								      \
	({									      \
		const char *__old = (s);					      \
		size_t __len = strnlen(__old, (n));				      \
		char *__new = (char *) __builtin_alloca(__len + 1);		      \
		memcpy(__new, __old, __len);					      \
		__new[__len] = '\0';						      \
		__new;								      \
	})
#endif

#if !defined(strdupa)
# define strdupa(s)								      \
	({									      \
		const char *__old = (s);					      \
		size_t __len = strlen(__old) + 1;				      \
		char *__new = (char *) alloca(__len);				      \
		(char *) memcpy(__new, __old, __len);				      \
	})
#endif

ssize_t pw_getrandom(void *buf, size_t buflen, unsigned int flags);

void* pw_reallocarray(void *ptr, size_t nmemb, size_t size);

/**
 * \}
 */

#ifdef __cplusplus
} /* extern "C" */
#endif

#endif /* PIPEWIRE_UTILS_H */
