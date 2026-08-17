/* Definitions for Rose packet radio address family.
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

/* What follows is copied from the 2.1.93 <linux/rose.h>.  */

#ifndef _NETROSE_ROSE_H
#define _NETROSE_ROSE_H 1

#include <sys/socket.h>
#include <netax25/ax25.h>

/* Socket level values.  */
#define SOL_ROSE        260


/* These are the public elements of the Linux kernel Rose
   implementation.  For kernel AX.25 see the file ax25.h. This file
   requires ax25.h for the definition of the ax25_address structure.  */
#define ROSE_MTU	251

#define ROSE_MAX_DIGIS	6

#define	ROSE_DEFER	1
#define	ROSE_T1		2
#define	ROSE_T2		3
#define	ROSE_T3		4
#define	ROSE_IDLE	5
#define	ROSE_QBITINCL	6
#define	ROSE_HOLDBACK	7

#define	SIOCRSGCAUSE		(SIOCPROTOPRIVATE + 0)
#define	SIOCRSSCAUSE		(SIOCPROTOPRIVATE + 1)
#define	SIOCRSL2CALL		(SIOCPROTOPRIVATE + 2)
#define	SIOCRSSL2CALL		(SIOCPROTOPRIVATE + 2)
#define	SIOCRSACCEPT		(SIOCPROTOPRIVATE + 3)
#define	SIOCRSCLRRT		(SIOCPROTOPRIVATE + 4)
#define	SIOCRSGL2CALL		(SIOCPROTOPRIVATE + 5)
#define	SIOCRSGFACILITIES	(SIOCPROTOPRIVATE + 6)

#define	ROSE_DTE_ORIGINATED	0x00
#define	ROSE_NUMBER_BUSY	0x01
#define	ROSE_INVALID_FACILITY	0x03
#define	ROSE_NETWORK_CONGESTION	0x05
#define	ROSE_OUT_OF_ORDER	0x09
#define	ROSE_ACCESS_BARRED	0x0B
#define	ROSE_NOT_OBTAINABLE	0x0D
#define	ROSE_REMOTE_PROCEDURE	0x11
#define	ROSE_LOCAL_PROCEDURE	0x13
#define	ROSE_SHIP_ABSENT	0x39


typedef struct
{
  char rose_addr[5];
} rose_address;

struct sockaddr_rose
{
  sa_family_t srose_family;
  rose_address srose_addr;
  ax25_address srose_call;
  int srose_ndigis;
  ax25_address	srose_digi;
};

struct full_sockaddr_rose
{
  sa_family_t srose_family;
  rose_address srose_addr;
  ax25_address srose_call;
  unsigned int srose_ndigis;
  ax25_address srose_digis[ROSE_MAX_DIGIS];
};

struct rose_route_struct
{
  rose_address address;
  unsigned short int mask;
  ax25_address	neighbour;
  char device[16];
  unsigned char	ndigis;
  ax25_address digipeaters[AX25_MAX_DIGIS];
};

struct rose_cause_struct
{
  unsigned char	cause;
  unsigned char	diagnostic;
};

struct rose_facilities_struct
{
  rose_address source_addr,   dest_addr;
  ax25_address source_call,   dest_call;
  unsigned char source_ndigis, dest_ndigis;
  ax25_address source_digis[ROSE_MAX_DIGIS];
  ax25_address dest_digis[ROSE_MAX_DIGIS];
  unsigned int rand;
  rose_address fail_addr;
  ax25_address fail_call;
};

#endif	/* netrose/rose.h */
