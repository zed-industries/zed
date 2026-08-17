/*
 * libusbx synchronization using POSIX Threads
 *
 * Copyright © 2010 Peter Stuge <peter@stuge.se>
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
 */

#ifndef LIBUSB_THREADS_POSIX_H
#define LIBUSB_THREADS_POSIX_H

#include <pthread.h>

#define usbi_mutex_static_t		pthread_mutex_t
#define USBI_MUTEX_INITIALIZER		PTHREAD_MUTEX_INITIALIZER
#define usbi_mutex_static_lock		pthread_mutex_lock
#define usbi_mutex_static_unlock	pthread_mutex_unlock

#define usbi_mutex_t			pthread_mutex_t
#define usbi_mutex_init			pthread_mutex_init
#define usbi_mutex_lock			pthread_mutex_lock
#define usbi_mutex_unlock		pthread_mutex_unlock
#define usbi_mutex_trylock		pthread_mutex_trylock
#define usbi_mutex_destroy		pthread_mutex_destroy

#define usbi_cond_t			pthread_cond_t
#define usbi_cond_init			pthread_cond_init
#define usbi_cond_wait			pthread_cond_wait
#define usbi_cond_timedwait		pthread_cond_timedwait
#define usbi_cond_broadcast		pthread_cond_broadcast
#define usbi_cond_destroy		pthread_cond_destroy
#define usbi_cond_signal		pthread_cond_signal

extern int usbi_mutex_init_recursive(pthread_mutex_t *mutex, pthread_mutexattr_t *attr);

int usbi_get_tid(void);

#endif /* LIBUSB_THREADS_POSIX_H */
