/* System specific type definitions for networking code.
   Copyright (C) 1997-2020 Free Software Foundation, Inc.
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

#ifndef _NETINET_IN_SYSTM_H
#define _NETINET_IN_SYSTM_H 1

#include <sys/types.h>
#include <stdint.h>

__BEGIN_DECLS

/*
 * Network order versions of various data types. Unfortunately, BSD
 * assumes specific sizes for shorts (16 bit) and longs (32 bit) which
 * don't hold in general. As a consequence, the network order versions
 * may not reflect the actual size of the native data types.
 */

typedef uint16_t n_short;      /* short as received from the net */
typedef uint32_t n_long;       /* long as received from the net  */
typedef uint32_t n_time;       /* ms since 00:00 GMT, byte rev   */

__END_DECLS

#endif /* netinet/in_systm.h */
