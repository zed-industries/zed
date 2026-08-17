/* Declarations for getopt (POSIX compatibility shim).
   Copyright (C) 1989-2020 Free Software Foundation, Inc.
   Unlike the bulk of the getopt implementation, this file is NOT part
   of gnulib.

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

#ifndef _GETOPT_POSIX_H
#define _GETOPT_POSIX_H 1

#if !defined _UNISTD_H && !defined _STDIO_H
#error "Never include getopt_posix.h directly; use unistd.h instead."
#endif

#include <bits/getopt_core.h>

__BEGIN_DECLS

#if defined __USE_POSIX2 && !defined __USE_POSIX_IMPLICITLY \
    && !defined __USE_GNU && !defined _GETOPT_H
/* GNU getopt has more functionality than POSIX getopt.  When we are
   explicitly conforming to POSIX and not GNU, and getopt.h (which is
   not part of POSIX) has not been included, the extra functionality
   is disabled.  */
# ifdef __REDIRECT
extern int __REDIRECT_NTH (getopt, (int ___argc, char *const *___argv,
				    const char *__shortopts),
			   __posix_getopt);
# else
extern int __posix_getopt (int ___argc, char *const *___argv,
			   const char *__shortopts)
  __THROW __nonnull ((2, 3));
#  define getopt __posix_getopt
# endif
#endif

__END_DECLS

#endif /* getopt_posix.h */
