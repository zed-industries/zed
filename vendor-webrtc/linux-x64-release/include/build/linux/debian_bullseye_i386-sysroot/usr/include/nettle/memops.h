/* memops.h

   Copyright (C) 2016 Niels Möller

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

#ifndef NETTLE_MEMOPS_H_INCLUDED
#define NETTLE_MEMOPS_H_INCLUDED

#include "memxor.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Name mangling */
#define cnd_memcpy nettle_cnd_memcpy
#define memeql_sec nettle_memeql_sec

int
memeql_sec (const void *a, const void *b, size_t n);

/* Side-channel silent conditional memcpy. cnd must be 0 (nop) or 1
   (copy). */
void
cnd_memcpy(int cnd, volatile void *dst, const volatile void *src, size_t n);

#ifdef __cplusplus
}
#endif

#endif /* NETTLE_MEMOPS_H_INCLUDED */
