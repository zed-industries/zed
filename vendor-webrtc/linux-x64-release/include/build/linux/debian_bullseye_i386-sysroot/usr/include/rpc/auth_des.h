/* Copyright (C) 1996-2020 Free Software Foundation, Inc.
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

#ifndef _RPC_AUTH_DES_H
#define _RPC_AUTH_DES_H	1

#include <sys/cdefs.h>
#include <rpc/auth.h>

__BEGIN_DECLS

/* There are two kinds of "names": fullnames and nicknames */
enum authdes_namekind
  {
    ADN_FULLNAME,
    ADN_NICKNAME
  };

/* A fullname contains the network name of the client,
   a conversation key and the window */
struct authdes_fullname
  {
    char *name;		/* network name of client, up to MAXNETNAMELEN */
    des_block key;	/* conversation key */
    uint32_t window;	/* associated window */
  };

/* A credential */
struct authdes_cred
  {
    enum authdes_namekind adc_namekind;
    struct authdes_fullname adc_fullname;
    uint32_t adc_nickname;
  };

/* A timeval replacement for !32bit platforms */
struct rpc_timeval
  {
    uint32_t tv_sec;            /* Seconds.  */
    uint32_t tv_usec;           /* Microseconds.  */
  };

/* A des authentication verifier */
struct authdes_verf
  {
    union
      {
	struct rpc_timeval adv_ctime;	/* clear time */
	des_block adv_xtime;		/* crypt time */
      }
    adv_time_u;
    uint32_t adv_int_u;
  };

/* des authentication verifier: client variety

   adv_timestamp is the current time.
   adv_winverf is the credential window + 1.
   Both are encrypted using the conversation key. */
#define adv_timestamp  adv_time_u.adv_ctime
#define adv_xtimestamp adv_time_u.adv_xtime
#define adv_winverf    adv_int_u

/* des authentication verifier: server variety

   adv_timeverf is the client's timestamp + client's window
   adv_nickname is the server's nickname for the client.
   adv_timeverf is encrypted using the conversation key. */
#define adv_timeverf   adv_time_u.adv_ctime
#define adv_xtimeverf  adv_time_u.adv_xtime
#define adv_nickname   adv_int_u

/* Map a des credential into a unix cred. */
extern int authdes_getucred (const struct authdes_cred * __adc,
			     uid_t * __uid, gid_t * __gid,
			     short *__grouplen, gid_t * __groups) __THROW;

/* Get the public key for NAME and place it in KEY.  NAME can only be
   up to MAXNETNAMELEN bytes long and the destination buffer KEY should
   have HEXKEYBYTES + 1 bytes long to fit all characters from the key.  */
extern int getpublickey (const char *__name, char *__key) __THROW;

/* Get the secret key for NAME and place it in KEY.  PASSWD is used to
   decrypt the encrypted key stored in the database.  NAME can only be
   up to MAXNETNAMELEN bytes long and the destination buffer KEY
   should have HEXKEYBYTES + 1 bytes long to fit all characters from
   the key.  */
extern int getsecretkey (const char *__name, char *__key,
			 const char *__passwd) __THROW;

extern int rtime (struct sockaddr_in *__addrp, struct rpc_timeval *__timep,
		  struct rpc_timeval *__timeout) __THROW;

__END_DECLS


#endif /* rpc/auth_des.h */
