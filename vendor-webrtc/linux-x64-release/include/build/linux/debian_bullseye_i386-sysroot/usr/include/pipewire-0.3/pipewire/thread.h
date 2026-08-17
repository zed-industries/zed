/* PipeWire
 *
 * Copyright © 2021 Wim Taymans
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

#ifndef PIPEWIRE_THREAD_H
#define PIPEWIRE_THREAD_H

#ifdef __cplusplus
extern "C" {
#endif

#include <string.h>
#include <errno.h>

#include <spa/support/thread.h>

/** \defgroup pw_thread Thread
 *
 * \brief functions to manipulate threads
 */

/**
 * \addtogroup pw_thread
 * \{
 */

SPA_DEPRECATED
void pw_thread_utils_set(struct spa_thread_utils *impl);
struct spa_thread_utils *pw_thread_utils_get(void);

#define pw_thread_utils_create(...)		spa_thread_utils_create(pw_thread_utils_get(), ##__VA_ARGS__)
#define pw_thread_utils_join(...)		spa_thread_utils_join(pw_thread_utils_get(), ##__VA_ARGS__)
#define pw_thread_utils_get_rt_range(...)	spa_thread_utils_get_rt_range(pw_thread_utils_get(), ##__VA_ARGS__)
#define pw_thread_utils_acquire_rt(...)		spa_thread_utils_acquire_rt(pw_thread_utils_get(), ##__VA_ARGS__)
#define pw_thread_utils_drop_rt(...)		spa_thread_utils_drop_rt(pw_thread_utils_get(), ##__VA_ARGS__)

/**
 * \}
 */

#ifdef __cplusplus
}  /* extern "C" */
#endif

#endif /* PIPEWIRE_THREAD_H */
