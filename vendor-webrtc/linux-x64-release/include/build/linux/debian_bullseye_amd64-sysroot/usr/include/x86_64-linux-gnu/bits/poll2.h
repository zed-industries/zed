/* Checking macros for poll functions.
   Copyright (C) 2012-2020 Free Software Foundation, Inc.
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

#ifndef _SYS_POLL_H
# error "Never include <bits/poll2.h> directly; use <sys/poll.h> instead."
#endif


__BEGIN_DECLS

extern int __REDIRECT (__poll_alias, (struct pollfd *__fds, nfds_t __nfds,
				      int __timeout), poll);
extern int __poll_chk (struct pollfd *__fds, nfds_t __nfds, int __timeout,
		       __SIZE_TYPE__ __fdslen);
extern int __REDIRECT (__poll_chk_warn, (struct pollfd *__fds, nfds_t __nfds,
					 int __timeout, __SIZE_TYPE__ __fdslen),
		       __poll_chk)
  __warnattr ("poll called with fds buffer too small file nfds entries");

__fortify_function int
poll (struct pollfd *__fds, nfds_t __nfds, int __timeout)
{
  if (__bos (__fds) != (__SIZE_TYPE__) -1)
    {
      if (! __builtin_constant_p (__nfds))
	return __poll_chk (__fds, __nfds, __timeout, __bos (__fds));
      else if (__bos (__fds) / sizeof (*__fds) < __nfds)
	return __poll_chk_warn (__fds, __nfds, __timeout, __bos (__fds));
    }

  return __poll_alias (__fds, __nfds, __timeout);
}


#ifdef __USE_GNU
extern int __REDIRECT (__ppoll_alias, (struct pollfd *__fds, nfds_t __nfds,
				       const struct timespec *__timeout,
				       const __sigset_t *__ss), ppoll);
extern int __ppoll_chk (struct pollfd *__fds, nfds_t __nfds,
			const struct timespec *__timeout,
			const __sigset_t *__ss, __SIZE_TYPE__ __fdslen);
extern int __REDIRECT (__ppoll_chk_warn, (struct pollfd *__fds, nfds_t __nfds,
					  const struct timespec *__timeout,
					  const __sigset_t *__ss,
					  __SIZE_TYPE__ __fdslen),
		       __ppoll_chk)
  __warnattr ("ppoll called with fds buffer too small file nfds entries");

__fortify_function int
ppoll (struct pollfd *__fds, nfds_t __nfds, const struct timespec *__timeout,
       const __sigset_t *__ss)
{
  if (__bos (__fds) != (__SIZE_TYPE__) -1)
    {
      if (! __builtin_constant_p (__nfds))
	return __ppoll_chk (__fds, __nfds, __timeout, __ss, __bos (__fds));
      else if (__bos (__fds) / sizeof (*__fds) < __nfds)
	return __ppoll_chk_warn (__fds, __nfds, __timeout, __ss,
				 __bos (__fds));
    }

  return __ppoll_alias (__fds, __nfds, __timeout, __ss);
}
#endif

__END_DECLS
