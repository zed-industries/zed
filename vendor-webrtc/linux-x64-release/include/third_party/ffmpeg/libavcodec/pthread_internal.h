/*
 * This file is part of FFmpeg.
 *
 * FFmpeg is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * FFmpeg is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with FFmpeg; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
 */

#ifndef AVCODEC_PTHREAD_INTERNAL_H
#define AVCODEC_PTHREAD_INTERNAL_H

#include "avcodec.h"

/* H.264 slice threading seems to be buggy with more than 16 threads,
 * limit the number of threads to 16 for automatic detection */
#define MAX_AUTO_THREADS 16

int ff_slice_thread_init(AVCodecContext *avctx);
void ff_slice_thread_free(AVCodecContext *avctx);

int ff_frame_thread_init(AVCodecContext *avctx);
void ff_frame_thread_free(AVCodecContext *avctx, int thread_count);

#define THREAD_SENTINEL 0 // This forbids putting a mutex/condition variable at the front.
/**
 * Initialize/destroy a list of mutexes/conditions contained in a structure.
 * The positions of these mutexes/conditions in the structure are given by
 * their offsets. Because it is undefined behaviour to destroy
 * an uninitialized mutex/condition, ff_pthread_init() stores the number
 * of successfully initialized mutexes and conditions in the object itself
 * and ff_pthread_free() uses this number to destroy exactly the mutexes and
 * condition variables that have been successfully initialized.
 *
 * @param     obj     The object containing the mutexes/conditions.
 * @param[in] offsets An array of offsets. Its first member gives the offset
 *                    of the variable that contains the count of successfully
 *                    initialized mutexes/condition variables; said variable
 *                    must be an unsigned int. Two arrays of offsets, each
 *                    delimited by a THREAD_SENTINEL follow. The first
 *                    contains the offsets of all the mutexes, the second
 *                    contains the offsets of all the condition variables.
 */
int  ff_pthread_init(void *obj, const unsigned offsets[]);
void ff_pthread_free(void *obj, const unsigned offsets[]);

/**
 * Macros to help creating the above lists. mutexes and conds need
 * to be parentheses-enclosed lists of offsets in the containing structure.
 */
#define OFFSET_ARRAY(...) __VA_ARGS__, THREAD_SENTINEL
#define DEFINE_OFFSET_ARRAY(type, name, cnt_variable, mutexes, conds)         \
static const unsigned name ## _offsets[] = { offsetof(type, cnt_variable),    \
                                             OFFSET_ARRAY mutexes,            \
                                             OFFSET_ARRAY conds }

#endif // AVCODEC_PTHREAD_INTERNAL_H
