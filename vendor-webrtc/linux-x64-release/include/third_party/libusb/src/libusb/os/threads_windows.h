/*
 * libusbx synchronization on Microsoft Windows
 *
 * Copyright © 2010 Michael Plante <michael.plante@gmail.com>
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

#ifndef LIBUSB_THREADS_WINDOWS_H
#define LIBUSB_THREADS_WINDOWS_H

#define usbi_mutex_static_t     volatile LONG
#define USBI_MUTEX_INITIALIZER  0

#define usbi_mutex_t            HANDLE

struct usbi_cond_perthread {
	struct list_head list;
	DWORD            tid;
	HANDLE           event;
};
struct usbi_cond_t_ {
	// Every time a thread touches the CV, it winds up in one of these lists.
	//   It stays there until the CV is destroyed, even if the thread
	//   terminates.
	struct list_head waiters;
	struct list_head not_waiting;
};
typedef struct usbi_cond_t_ usbi_cond_t;

// We *were* getting timespec from pthread.h:
#if (!defined(HAVE_STRUCT_TIMESPEC) && !defined(_TIMESPEC_DEFINED))
#define HAVE_STRUCT_TIMESPEC 1
#define _TIMESPEC_DEFINED 1
#if _MSC_VER < 1900
// VS 2015 and above include <time.h> in stdlib.h
struct timespec {
		long tv_sec;
		long tv_nsec;
};
#endif
#endif /* HAVE_STRUCT_TIMESPEC | _TIMESPEC_DEFINED */

// We *were* getting ETIMEDOUT from pthread.h:
#ifndef ETIMEDOUT
#  define ETIMEDOUT 10060     /* This is the value in winsock.h. */
#endif

#define usbi_mutexattr_t void
#define usbi_condattr_t  void

// all Windows mutexes are recursive
#define usbi_mutex_init_recursive(mutex, attr) usbi_mutex_init((mutex), (attr))

int usbi_mutex_static_lock(usbi_mutex_static_t *mutex);
int usbi_mutex_static_unlock(usbi_mutex_static_t *mutex);


int usbi_mutex_init(usbi_mutex_t *mutex,
					const usbi_mutexattr_t *attr);
int usbi_mutex_lock(usbi_mutex_t *mutex);
int usbi_mutex_unlock(usbi_mutex_t *mutex);
int usbi_mutex_trylock(usbi_mutex_t *mutex);
int usbi_mutex_destroy(usbi_mutex_t *mutex);

int usbi_cond_init(usbi_cond_t *cond,
				   const usbi_condattr_t *attr);
int usbi_cond_destroy(usbi_cond_t *cond);
int usbi_cond_wait(usbi_cond_t *cond, usbi_mutex_t *mutex);
int usbi_cond_timedwait(usbi_cond_t *cond,
						usbi_mutex_t *mutex,
						const struct timespec *abstime);
int usbi_cond_broadcast(usbi_cond_t *cond);
int usbi_cond_signal(usbi_cond_t *cond);

int usbi_get_tid(void);

#endif /* LIBUSB_THREADS_WINDOWS_H */
