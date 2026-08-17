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

#ifndef _NETAX25_AX25_H
#define _NETAX25_AX25_H	1

#include <features.h>
#include <bits/sockaddr.h>

/* Setsockoptions(2) level.  Thanks to BSD these must match IPPROTO_xxx.  */
#define SOL_AX25	257

/* AX.25 flags: */
#define AX25_WINDOW	1
#define AX25_T1		2
#define AX25_T2		5
#define AX25_T3		4
#define AX25_N2		3
#define AX25_BACKOFF	6
#define AX25_EXTSEQ	7
#define AX25_PIDINCL	8
#define AX25_IDLE	9
#define	AX25_PACLEN	10
#define AX25_IPMAXQUEUE 11
#define AX25_IAMDIGI	12
#define AX25_KILL	99

/* AX.25 socket ioctls: */
#define SIOCAX25GETUID		(SIOCPROTOPRIVATE)
#define SIOCAX25ADDUID		(SIOCPROTOPRIVATE+1)
#define SIOCAX25DELUID		(SIOCPROTOPRIVATE+2)
#define SIOCAX25NOUID		(SIOCPROTOPRIVATE+3)
#define SIOCAX25BPQADDR		(SIOCPROTOPRIVATE+4)
#define SIOCAX25GETPARMS	(SIOCPROTOPRIVATE+5)
#define SIOCAX25SETPARMS	(SIOCPROTOPRIVATE+6)
#define SIOCAX25OPTRT		(SIOCPROTOPRIVATE+7)
#define SIOCAX25CTLCON		(SIOCPROTOPRIVATE+8)
#define SIOCAX25GETINFO		(SIOCPROTOPRIVATE+9)
#define SIOCAX25ADDFWD		(SIOCPROTOPRIVATE+10)
#define SIOCAX25DELFWD		(SIOCPROTOPRIVATE+11)

/* unknown: */
#define AX25_NOUID_DEFAULT	0
#define AX25_NOUID_BLOCK	1
#define AX25_SET_RT_IPMODE	2

/* Digipeating flags: */
#define AX25_DIGI_INBAND	0x01	/* Allow digipeating within port */
#define AX25_DIGI_XBAND		0x02	/* Allow digipeating across ports */

/* Maximim number of digipeaters: */
#define AX25_MAX_DIGIS 8


typedef struct
  {
    char ax25_call[7];		/* 6 call + SSID (shifted ascii) */
  }
ax25_address;

struct sockaddr_ax25
  {
    sa_family_t sax25_family;
    ax25_address sax25_call;
    int sax25_ndigis;
  };

/*
 * The sockaddr struct with the digipeater adresses:
 */
struct full_sockaddr_ax25
  {
    struct sockaddr_ax25 fsa_ax25;
    ax25_address fsa_digipeater[AX25_MAX_DIGIS];
  };
#define sax25_uid	sax25_ndigis

struct ax25_routes_struct
  {
    ax25_address port_addr;
    ax25_address dest_addr;
    unsigned char digi_count;
    ax25_address digi_addr[AX25_MAX_DIGIS];
  };

/* The AX.25 ioctl structure: */
struct ax25_ctl_struct
  {
    ax25_address port_addr;
    ax25_address source_addr;
    ax25_address dest_addr;
    unsigned int cmd;
    unsigned long arg;
    unsigned char digi_count;
    ax25_address digi_addr[AX25_MAX_DIGIS];
  };

struct ax25_info_struct
  {
    unsigned int  n2, n2count;
    unsigned int t1, t1timer;
    unsigned int t2, t2timer;
    unsigned int t3, t3timer;
    unsigned int idle, idletimer;
    unsigned int state;
    unsigned int rcv_q, snd_q;
  };

struct ax25_fwd_struct
  {
    ax25_address port_from;
    ax25_address port_to;
  };

/* AX.25 route structure: */
struct ax25_route_opt_struct
  {
    ax25_address port_addr;
    ax25_address dest_addr;
    int cmd;
    int arg;
  };

/* AX.25 BPQ stuff: */
struct ax25_bpqaddr_struct
  {
    char dev[16];
    ax25_address addr;
  };

/* Definitions for the AX.25 `values' fields: */
#define	AX25_VALUES_IPDEFMODE	0	/* 'D'=DG 'V'=VC */
#define	AX25_VALUES_AXDEFMODE	1	/* 8=Normal 128=Extended Seq Nos */
#define	AX25_VALUES_NETROM	2	/* Allow NET/ROM  - 0=No 1=Yes */
#define	AX25_VALUES_TEXT	3	/* Allow PID=Text - 0=No 1=Yes */
#define	AX25_VALUES_BACKOFF	4	/* 'E'=Exponential 'L'=Linear */
#define	AX25_VALUES_CONMODE	5	/* Allow connected modes - 0=No 1=Yes */
#define	AX25_VALUES_WINDOW	6	/* Default window size for standard AX.25 */
#define	AX25_VALUES_EWINDOW	7	/* Default window size for extended AX.25 */
#define	AX25_VALUES_T1		8	/* Default T1 timeout value */
#define	AX25_VALUES_T2		9	/* Default T2 timeout value */
#define	AX25_VALUES_T3		10	/* Default T3 timeout value */
#define	AX25_VALUES_N2		11	/* Default N2 value */
#define	AX25_VALUES_DIGI	12	/* Digipeat mode */
#define AX25_VALUES_IDLE	13	/* mode vc idle timer */
#define AX25_VALUES_PACLEN	14	/* AX.25 MTU */
#define AX25_VALUES_IPMAXQUEUE  15	/* Maximum number of buffers enqueued */
#define	AX25_MAX_VALUES		20

struct ax25_parms_struct
  {
    ax25_address port_addr;
    unsigned short values[AX25_MAX_VALUES];
  };

#endif /* netax25/ax25.h */
