/* Copyright (C) 2010-2020 Free Software Foundation, Inc.
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

#ifndef	_SYS_FANOTIFY_H
#define	_SYS_FANOTIFY_H	1

#include <stdint.h>
#include <linux/fanotify.h>


__BEGIN_DECLS

/* Create and initialize fanotify group.  */
extern int fanotify_init (unsigned int __flags, unsigned int __event_f_flags)
  __THROW;

/* Add, remove, or modify an fanotify mark on a filesystem object.  */
extern int fanotify_mark (int __fanotify_fd, unsigned int __flags,
			  uint64_t __mask, int __dfd, const char *__pathname)
     __THROW;

__END_DECLS

#endif /* sys/fanotify.h */
