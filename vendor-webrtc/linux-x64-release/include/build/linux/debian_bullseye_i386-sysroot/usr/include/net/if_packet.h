/* Definitions for use with Linux SOCK_PACKET sockets.
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

#ifndef __IF_PACKET_H
#define __IF_PACKET_H

#include <features.h>
#include <bits/sockaddr.h>

/* This is the SOCK_PACKET address structure as used in Linux 2.0.
   From Linux 2.1 the AF_PACKET interface is preferred and you should
   consider using it in place of this one.  */

struct sockaddr_pkt
  {
    __SOCKADDR_COMMON (spkt_);
    unsigned char spkt_device[14];
    unsigned short spkt_protocol;
  };

#endif
