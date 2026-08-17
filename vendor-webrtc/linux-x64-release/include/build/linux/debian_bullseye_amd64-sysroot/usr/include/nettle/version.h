/* version.h

   Information about library version.

   Copyright (C) 2015 Red Hat, Inc.
   Copyright (C) 2015 Niels Möller

   This file is part of GNU Nettle.

   GNU Nettle is free software: you can redistribute it and/or
   modify it under the terms of either:

     * the GNU Lesser General Public License as published by the Free
       Software Foundation; either version 3 of the License, or (at your
       option) any later version.

   or

     * the GNU General Public License as published by the Free
       Software Foundation; either version 2 of the License, or (at your
       option) any later version.

   or both in parallel, as here.

   GNU Nettle is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   General Public License for more details.

   You should have received copies of the GNU General Public License and
   the GNU Lesser General Public License along with this program.  If
   not, see http://www.gnu.org/licenses/.
*/

#ifndef NETTLE_VERSION_H_INCLUDED
#define NETTLE_VERSION_H_INCLUDED

#ifdef __cplusplus
extern "C" {
#endif

/* Individual version numbers in decimal */
#define NETTLE_VERSION_MAJOR 3
#define NETTLE_VERSION_MINOR 7

#define NETTLE_USE_MINI_GMP 0

/* We need a preprocessor constant for GMP_NUMB_BITS, simply using
   sizeof(mp_limb_t) * CHAR_BIT is not good enough. */
#if NETTLE_USE_MINI_GMP
# define GMP_NUMB_BITS n/a
#endif

int
nettle_version_major (void);

int
nettle_version_minor (void);

#ifdef __cplusplus
}
#endif

#endif /* NETTLE_VERSION_H_INCLUDED */
