/* Copyright (C) 2004-2020 Free Software Foundation, Inc.
   This file is part of the GNU C Library.

   The GNU C Library is free software; you can redistribute it and/or
   modify it under the terms of the GNU Lesser General Public
   License as published by the Free Software Foundation; either
   version 2.1 of the License, or (at your option) any later version.

   The GNU C Library is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public
   License along with the GNU C Library; if not, see
   <https://www.gnu.org/licenses/>.  */

#ifndef _MQUEUE_H
#define _MQUEUE_H	1

#include <features.h>
#include <sys/types.h>
#include <fcntl.h>
#include <bits/types/sigevent_t.h>
#include <bits/types/struct_timespec.h>
/* Get the definition of mqd_t and struct mq_attr.  */
#include <bits/mqueue.h>

__BEGIN_DECLS

/* Establish connection between a process and a message queue NAME and
   return message queue descriptor or (mqd_t) -1 on error.  OFLAG determines
   the type of access used.  If O_CREAT is on OFLAG, the third argument is
   taken as a `mode_t', the mode of the created message queue, and the fourth
   argument is taken as `struct mq_attr *', pointer to message queue
   attributes.  If the fourth argument is NULL, default attributes are
   used.  */
extern mqd_t mq_open (const char *__name, int __oflag, ...)
  __THROW __nonnull ((1));

/* Removes the association between message queue descriptor MQDES and its
   message queue.  */
extern int mq_close (mqd_t __mqdes) __THROW;

/* Query status and attributes of message queue MQDES.  */
extern int mq_getattr (mqd_t __mqdes, struct mq_attr *__mqstat)
  __THROW __nonnull ((2));

/* Set attributes associated with message queue MQDES and if OMQSTAT is
   not NULL also query its old attributes.  */
extern int mq_setattr (mqd_t __mqdes,
		       const struct mq_attr *__restrict __mqstat,
		       struct mq_attr *__restrict __omqstat)
  __THROW __nonnull ((2));

/* Remove message queue named NAME.  */
extern int mq_unlink (const char *__name) __THROW __nonnull ((1));

/* Register notification issued upon message arrival to an empty
   message queue MQDES.  */
extern int mq_notify (mqd_t __mqdes, const struct sigevent *__notification)
     __THROW;

/* Receive the oldest from highest priority messages in message queue
   MQDES.  */
extern ssize_t mq_receive (mqd_t __mqdes, char *__msg_ptr, size_t __msg_len,
			   unsigned int *__msg_prio) __nonnull ((2));

/* Add message pointed by MSG_PTR to message queue MQDES.  */
extern int mq_send (mqd_t __mqdes, const char *__msg_ptr, size_t __msg_len,
		    unsigned int __msg_prio) __nonnull ((2));

#ifdef __USE_XOPEN2K
/* Receive the oldest from highest priority messages in message queue
   MQDES, stop waiting if ABS_TIMEOUT expires.  */
extern ssize_t mq_timedreceive (mqd_t __mqdes, char *__restrict __msg_ptr,
				size_t __msg_len,
				unsigned int *__restrict __msg_prio,
				const struct timespec *__restrict __abs_timeout)
  __nonnull ((2, 5));

/* Add message pointed by MSG_PTR to message queue MQDES, stop blocking
   on full message queue if ABS_TIMEOUT expires.  */
extern int mq_timedsend (mqd_t __mqdes, const char *__msg_ptr,
			 size_t __msg_len, unsigned int __msg_prio,
			 const struct timespec *__abs_timeout)
  __nonnull ((2, 5));
#endif

/* Define some inlines helping to catch common problems.  */
#if __USE_FORTIFY_LEVEL > 0 && defined __fortify_function \
    && defined __va_arg_pack_len
# include <bits/mqueue2.h>
#endif

__END_DECLS

#endif /* mqueue.h */
