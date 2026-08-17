/* Copyright (C) 1996-2014 Free Software Foundation, Inc.
   This file is part of the GNU C Library.
   Contributed by Thorsten Kukuk <kukuk@suse.de>, 1996.

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
   <http://www.gnu.org/licenses/>.  */


#ifndef	__RPCSVC_YPCLNT_H__
#define	__RPCSVC_YPCLNT_H__

/* Some defines */
#define YPERR_SUCCESS	0		/* There is no error */
#define	YPERR_BADARGS	1		/* Args to function are bad */
#define	YPERR_RPC 	2		/* RPC failure */
#define	YPERR_DOMAIN	3		/* Can't bind to a server with this domain */
#define	YPERR_MAP	4		/* No such map in server's domain */
#define	YPERR_KEY	5		/* No such key in map */
#define	YPERR_YPERR	6		/* Internal yp server or client error */
#define	YPERR_RESRC	7		/* Local resource allocation failure */
#define	YPERR_NOMORE	8		/* No more records in map database */
#define	YPERR_PMAP	9		/* Can't communicate with portmapper */
#define	YPERR_YPBIND	10		/* Can't communicate with ypbind */
#define	YPERR_YPSERV	11		/* Can't communicate with ypserv */
#define	YPERR_NODOM	12		/* Local domain name not set */
#define	YPERR_BADDB	13		/* yp data base is bad */
#define	YPERR_VERS	14		/* YP version mismatch */
#define	YPERR_ACCESS	15		/* Access violation */
#define	YPERR_BUSY	16		/* Database is busy */

/* Types of update operations */
#define	YPOP_CHANGE	1		/* Change, do not add */
#define	YPOP_INSERT	2		/* Add, do not change */
#define	YPOP_DELETE	3		/* Delete this entry */
#define	YPOP_STORE	4		/* Add, or change */

#ifdef  __cplusplus
extern "C" {
#endif

/* struct ypall_callback * is the arg which must be passed to yp_all.  */
struct ypall_callback
  {
    int (*foreach) (int __status, char *__key, int __keylen,
		    char *__val, int __vallen, char *__data);
    char *data;
  };

/* External NIS client function references.  */
extern int yp_bind (const char *);
extern void yp_unbind (const char *);
extern int yp_get_default_domain (char **);
extern int yp_match (const char *, const char *, const char *,
		     const int, char **, int *);
extern int yp_first (const char *, const char *, char **,
		     int *, char **, int *);
extern int yp_next (const char *, const char *, const char *,
		    const int, char **, int *, char **, int *);
extern int yp_master (const char *, const char *, char **);
extern int yp_order (const char *, const char *, unsigned int *);
extern int yp_all (const char *, const char *,
		   const struct ypall_callback *);
extern const char *yperr_string (const int);
extern const char *ypbinderr_string (const int);
extern int ypprot_err (const int);
extern int yp_update (char *, char *, unsigned int,  char *,
		      int, char *, int);

#if 0
extern int yp_maplist (const char *, struct ypmaplist **);
#endif

/* This functions exists only under BSD and Linux systems.  */
extern int __yp_check (char **);

#ifdef  __cplusplus
}
#endif

#endif	/* __RPCSVC_YPCLNT_H__ */
