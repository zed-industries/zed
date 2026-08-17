/* Copyright (C) 1997-2020 Free Software Foundation, Inc.
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

#ifndef _NETINET_IF_FDDI_H
#define	_NETINET_IF_FDDI_H 1

#include <sys/types.h>
#include <stdint.h>
#include <linux/if_fddi.h>

#ifdef __USE_MISC

struct fddi_header {
  uint8_t fddi_fc;                    /* Frame Control (FC) value */
  uint8_t fddi_dhost[FDDI_K_ALEN];    /* Destination host */
  uint8_t fddi_shost[FDDI_K_ALEN];    /* Source host */
};
#endif

#endif	/* netinet/if_fddi.h */
