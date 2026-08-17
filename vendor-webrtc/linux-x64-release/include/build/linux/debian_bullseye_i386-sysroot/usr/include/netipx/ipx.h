/* Copyright (C) 1991-2020 Free Software Foundation, Inc.
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

#ifndef __NETIPX_IPX_H
#define __NETIPX_IPX_H 1

#include <sys/types.h>
#include <stdint.h>
#include <bits/sockaddr.h>

__BEGIN_DECLS

#define SOL_IPX    256          /* sockopt level */

#define IPX_TYPE        1
#define IPX_NODE_LEN	6
#define IPX_MTU		576

struct sockaddr_ipx
  {
    sa_family_t sipx_family;
    uint16_t sipx_port;
    uint32_t sipx_network;
    unsigned char sipx_node[IPX_NODE_LEN];
    uint8_t sipx_type;
    unsigned char sipx_zero;	/* 16 byte fill */
  };

/*
 *	So we can fit the extra info for SIOCSIFADDR into the address nicely
 */

#define sipx_special	sipx_port
#define sipx_action	sipx_zero
#define IPX_DLTITF	0
#define IPX_CRTITF	1

typedef struct ipx_route_definition
  {
    unsigned long ipx_network;
    unsigned long ipx_router_network;
    unsigned char ipx_router_node[IPX_NODE_LEN];
  }
ipx_route_definition;

typedef struct ipx_interface_definition
  {
    unsigned long ipx_network;
    unsigned char ipx_device[16];
    unsigned char ipx_dlink_type;
#define IPX_FRAME_NONE		0
#define IPX_FRAME_SNAP		1
#define IPX_FRAME_8022		2
#define IPX_FRAME_ETHERII	3
#define IPX_FRAME_8023		4
#define IPX_FRAME_TR_8022	5
    unsigned char ipx_special;
#define IPX_SPECIAL_NONE	0
#define IPX_PRIMARY		1
#define IPX_INTERNAL		2
    unsigned char ipx_node[IPX_NODE_LEN];
  }
ipx_interface_definition;

typedef struct ipx_config_data
  {
    unsigned char ipxcfg_auto_select_primary;
    unsigned char ipxcfg_auto_create_interfaces;
  }
ipx_config_data;

/*
 * OLD Route Definition for backward compatibility.
 */

struct ipx_route_def
  {
    unsigned long ipx_network;
    unsigned long ipx_router_network;
#define IPX_ROUTE_NO_ROUTER	0
    unsigned char ipx_router_node[IPX_NODE_LEN];
    unsigned char ipx_device[16];
    unsigned short ipx_flags;
#define IPX_RT_SNAP		8
#define IPX_RT_8022		4
#define IPX_RT_BLUEBOOK		2
#define IPX_RT_ROUTED		1
  };

#define SIOCAIPXITFCRT		(SIOCPROTOPRIVATE)
#define SIOCAIPXPRISLT		(SIOCPROTOPRIVATE + 1)
#define SIOCIPXCFGDATA		(SIOCPROTOPRIVATE + 2)
#define SIOCIPXNCPCONN		(SIOCPROTOPRIVATE + 3)

__END_DECLS

#endif /* netipx/ipx.h */
