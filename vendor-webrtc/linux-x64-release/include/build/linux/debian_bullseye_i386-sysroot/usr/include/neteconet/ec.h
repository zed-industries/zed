/* Definitions for use with Linux AF_ECONET sockets.
   Copyright (C) 1998-2020 Free Software Foundation, Inc.
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

#ifndef _NETECONET_EC_H
#define _NETECONET_EC_H	1

#include <features.h>
#include <bits/sockaddr.h>

struct ec_addr
  {
    unsigned char station;		/* Station number.  */
    unsigned char net;			/* Network number.  */
  };

struct sockaddr_ec
  {
    __SOCKADDR_COMMON (sec_);
    unsigned char port;			/* Port number.  */
    unsigned char cb;			/* Control/flag byte.  */
    unsigned char type;			/* Type of message.  */
    struct ec_addr addr;
    unsigned long cookie;
  };

#define ECTYPE_PACKET_RECEIVED		0	/* Packet received */
#define ECTYPE_TRANSMIT_STATUS		0x10	/* Transmit completed */

#define ECTYPE_TRANSMIT_OK		1
#define ECTYPE_TRANSMIT_NOT_LISTENING	2
#define ECTYPE_TRANSMIT_NET_ERROR	3
#define ECTYPE_TRANSMIT_NO_CLOCK	4
#define ECTYPE_TRANSMIT_LINE_JAMMED	5
#define ECTYPE_TRANSMIT_NOT_PRESENT	6

#endif
