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

#ifndef COMPAT_DISPATCH_SEMAPHORE_SEMAPHORE_H
#define COMPAT_DISPATCH_SEMAPHORE_SEMAPHORE_H

#include <dispatch/dispatch.h>
#include <errno.h>

#define sem_t dispatch_semaphore_t
#define sem_post(psem)              dispatch_semaphore_signal(*psem)
#define sem_wait(psem)              dispatch_semaphore_wait(*psem, DISPATCH_TIME_FOREVER)
#define sem_timedwait(psem, val)    dispatch_semaphore_wait(*psem, dispatch_walltime(val, 0))
#define sem_destroy(psem)           dispatch_release(*psem)

static inline int compat_sem_init(dispatch_semaphore_t *psem,
                                  int unused, int val)
{
    int ret = !!(*psem = dispatch_semaphore_create(val)) - 1;
    if (ret < 0)
        errno = ENOMEM;
    return ret;
}

#define sem_init compat_sem_init

#endif /* COMPAT_DISPATCH_SEMAPHORE_SEMAPHORE_H */
