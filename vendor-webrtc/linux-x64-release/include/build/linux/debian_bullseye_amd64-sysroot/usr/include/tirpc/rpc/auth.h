/*	$NetBSD: auth.h,v 1.15 2000/06/02 22:57:55 fvdl Exp $	*/

/*
 * Copyright (c) 2009, Sun Microsystems, Inc.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 * - Redistributions of source code must retain the above copyright notice,
 *   this list of conditions and the following disclaimer.
 * - Redistributions in binary form must reproduce the above copyright notice,
 *   this list of conditions and the following disclaimer in the documentation
 *   and/or other materials provided with the distribution.
 * - Neither the name of Sun Microsystems, Inc. nor the names of its
 *   contributors may be used to endorse or promote products derived
 *   from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 *
 *	from: @(#)auth.h 1.17 88/02/08 SMI
 *	from: @(#)auth.h	2.3 88/08/07 4.0 RPCSRC
 *	from: @(#)auth.h	1.43 	98/02/02 SMI
 * $FreeBSD: src/include/rpc/auth.h,v 1.20 2003/01/01 18:48:42 schweikh Exp $
 */

/*
 * auth.h, Authentication interface.
 *
 * Copyright (C) 1984, Sun Microsystems, Inc.
 *
 * The data structures are completely opaque to the client.  The client
 * is required to pass an AUTH * to routines that create rpc
 * "sessions".
 */

#ifndef _TIRPC_AUTH_H
#define  _TIRPC_AUTH_H

#include <rpc/xdr.h>
#include <rpc/clnt_stat.h>
#include <sys/socket.h>
#include <sys/types.h>


#define MAX_AUTH_BYTES	400
#define MAXNETNAMELEN	255	/* maximum length of network user's name */

/*
 *  Client side authentication/security data
 */

typedef struct sec_data {
	u_int	secmod;		/* security mode number e.g. in nfssec.conf */
	u_int	rpcflavor;	/* rpc flavors:AUTH_UNIX,AUTH_DES,RPCSEC_GSS */
	int	flags;		/* AUTH_F_xxx flags */
	caddr_t data;		/* opaque data per flavor */
} sec_data_t;

#ifdef _SYSCALL32_IMPL
struct sec_data32 {
	uint32_t secmod;	/* security mode number e.g. in nfssec.conf */
	uint32_t rpcflavor;	/* rpc flavors:AUTH_UNIX,AUTH_DES,RPCSEC_GSS */
	int32_t flags;		/* AUTH_F_xxx flags */
	caddr32_t data;		/* opaque data per flavor */
};
#endif /* _SYSCALL32_IMPL */

/*
 * AUTH_DES flavor specific data from sec_data opaque data field.
 * AUTH_KERB has the same structure.
 */
typedef struct des_clnt_data {
	struct netbuf	syncaddr;	/* time sync addr */
	struct knetconfig *knconf;	/* knetconfig info that associated */
					/* with the syncaddr. */
	char		*netname;	/* server's netname */
	int		netnamelen;	/* server's netname len */
} dh_k4_clntdata_t;

#ifdef _SYSCALL32_IMPL
struct des_clnt_data32 {
	struct netbuf32 syncaddr;	/* time sync addr */
	caddr32_t knconf;		/* knetconfig info that associated */
					/* with the syncaddr. */
	caddr32_t netname;		/* server's netname */
	int32_t netnamelen;		/* server's netname len */
};
#endif /* _SYSCALL32_IMPL */

/*
 * authentication/security specific flags
 */
#define AUTH_F_RPCTIMESYNC	0x001	/* use RPC to do time sync */
#define AUTH_F_TRYNONE		0x002	/* allow fall back to AUTH_NONE */


/*
 * Status returned from authentication check
 */
enum auth_stat {
	AUTH_OK=0,
	/*
	 * failed at  remote end
	 */
	AUTH_BADCRED=1,			/* bogus credentials (seal broken) */
	AUTH_REJECTEDCRED=2,		/* client should begin new session */
	AUTH_BADVERF=3,			/* bogus verifier (seal broken) */
	AUTH_REJECTEDVERF=4,		/* verifier expired or was replayed */
	AUTH_TOOWEAK=5,			/* rejected due to security reasons */
	/*
	 * failed locally
	*/
	AUTH_INVALIDRESP=6,		/* bogus response verifier */
	AUTH_FAILED=7,			/* some unknown reason */
	/*
	 * kerberos errors
	 */
	AUTH_KERB_GENERIC = 8,		/* kerberos generic error */
	AUTH_TIMEEXPIRE = 9,		/* time of credential expired */
	AUTH_TKT_FILE = 10,		/* something wrong with ticket file */
	AUTH_DECODE = 11,		/* can't decode authenticator */
	AUTH_NET_ADDR = 12,		/* wrong net address in ticket */
	/*
	 * RPCSEC_GSS errors
	 */
	RPCSEC_GSS_CREDPROBLEM = 13,
	RPCSEC_GSS_CTXPROBLEM = 14

};

typedef u_int32_t u_int32;	/* 32-bit unsigned integers */

union des_block {
	struct {
		u_int32_t high;
		u_int32_t low;
	} key;
	char c[8];
};
typedef union des_block des_block;
#ifdef __cplusplus
extern "C" {
#endif
extern bool_t xdr_des_block(XDR *, des_block *);
#ifdef __cplusplus
}
#endif

/*
 * Authentication info.  Opaque to client.
 */
struct opaque_auth {
	enum_t	oa_flavor;		/* flavor of auth */
	caddr_t	oa_base;		/* address of more auth stuff */
	u_int	oa_length;		/* not to exceed MAX_AUTH_BYTES */
};


/*
 * Auth handle, interface to client side authenticators.
 */
typedef struct __auth {
	struct	opaque_auth	ah_cred;
	struct	opaque_auth	ah_verf;
	union	des_block	ah_key;
	struct auth_ops {
		void	(*ah_nextverf) (struct __auth *);
		/* nextverf & serialize */
		int	(*ah_marshal) (struct __auth *, XDR *);
		/* validate verifier */
		int	(*ah_validate) (struct __auth *,
			    struct opaque_auth *);
		/* refresh credentials */
		int	(*ah_refresh) (struct __auth *, void *);
		/* destroy this structure */
		void	(*ah_destroy) (struct __auth *);
		/* encode data for wire */
		int     (*ah_wrap) (struct __auth *, XDR *, xdrproc_t, caddr_t);
		/* decode data for wire */
		int     (*ah_unwrap) (struct __auth *, XDR *, xdrproc_t, caddr_t);

	} *ah_ops;
	void *ah_private;
} AUTH;

/*
 * Authentication ops.
 * The ops and the auth handle provide the interface to the authenticators.
 *
 * AUTH	*auth;
 * XDR	*xdrs;
 * struct opaque_auth verf;
 */
#define AUTH_NEXTVERF(auth)		\
		((*((auth)->ah_ops->ah_nextverf))(auth))
#define auth_nextverf(auth)		\
		((*((auth)->ah_ops->ah_nextverf))(auth))

#define AUTH_MARSHALL(auth, xdrs)	\
		((*((auth)->ah_ops->ah_marshal))(auth, xdrs))
#define auth_marshall(auth, xdrs)	\
		((*((auth)->ah_ops->ah_marshal))(auth, xdrs))

#define AUTH_VALIDATE(auth, verfp)	\
		((*((auth)->ah_ops->ah_validate))((auth), verfp))
#define auth_validate(auth, verfp)	\
		((*((auth)->ah_ops->ah_validate))((auth), verfp))

#define AUTH_REFRESH(auth, msg)		\
		((*((auth)->ah_ops->ah_refresh))(auth, msg))
#define auth_refresh(auth, msg)		\
		((*((auth)->ah_ops->ah_refresh))(auth, msg))

#define AUTH_DESTROY(auth)		\
		((*((auth)->ah_ops->ah_destroy))(auth));
#define auth_destroy(auth)		\
		((*((auth)->ah_ops->ah_destroy))(auth));

#define AUTH_WRAP(auth, xdrs, xfunc, xwhere)            \
		((*((auth)->ah_ops->ah_wrap))(auth, xdrs, \
		xfunc, xwhere))
#define auth_wrap(auth, xdrs, xfunc, xwhere)            \
		((*((auth)->ah_ops->ah_wrap))(auth, xdrs, \
		xfunc, xwhere))

#define AUTH_UNWRAP(auth, xdrs, xfunc, xwhere)          \
		((*((auth)->ah_ops->ah_unwrap))(auth, xdrs, \
		xfunc, xwhere))
#define auth_unwrap(auth, xdrs, xfunc, xwhere)          \
		((*((auth)->ah_ops->ah_unwrap))(auth, xdrs, \
		xfunc, xwhere))


#ifdef __cplusplus
extern "C" {
#endif
extern struct opaque_auth _null_auth;
#ifdef __cplusplus
}
#endif

/*
 * Any style authentication.  These routines can be used by any
 * authentication style that does not use the wrap/unwrap functions.
 */
int authany_wrap(void), authany_unwrap(void);

/*
 * These are the various implementations of client side authenticators.
 */

/*
 * System style authentication
 * AUTH *authunix_create(machname, uid, gid, len, aup_gids)
 *	char *machname;
 *	int uid;
 *	int gid;
 *	int len;
 *	int *aup_gids;
 */
#ifdef __cplusplus
extern "C" {
#endif
extern AUTH *authunix_create(char *, uid_t, uid_t, int, uid_t *);
extern AUTH *authunix_create_default(void);	/* takes no parameters */
extern AUTH *authnone_create(void);		/* takes no parameters */
#ifdef __cplusplus
}
#endif
/*
 * DES style authentication
 * AUTH *authsecdes_create(servername, window, timehost, ckey)
 * 	char *servername;		- network name of server
 *	u_int window;			- time to live
 * 	const char *timehost;			- optional hostname to sync with
 * 	des_block *ckey;		- optional conversation key to use
 */
#ifdef __cplusplus
extern "C" {
#endif
extern AUTH *authdes_create (char *, u_int, struct sockaddr *, des_block *);
extern AUTH *authdes_pk_create (char *, netobj *, u_int,
				struct sockaddr *, des_block *);
extern AUTH *authdes_seccreate (const char *, const u_int, const  char *,
    const  des_block *);
#ifdef __cplusplus
}
#endif

#ifdef __cplusplus
extern "C" {
#endif
extern bool_t xdr_opaque_auth		(XDR *, struct opaque_auth *);
#ifdef __cplusplus
}
#endif

#define authsys_create(c,i1,i2,i3,ip) authunix_create((c),(i1),(i2),(i3),(ip))
#define authsys_create_default() authunix_create_default()

/*
 * Netname manipulation routines.
 */
#ifdef __cplusplus
extern "C" {
#endif
extern int getnetname(char *);
extern int host2netname(char *, const char *, const char *);
extern int user2netname(char *, const uid_t, const char *);
extern int netname2user(char *, uid_t *, gid_t *, int *, gid_t *);
extern int netname2host(char *, char *, const int);
extern void passwd2des ( char *, char * );
#ifdef __cplusplus
}
#endif

/*
 *
 * These routines interface to the keyserv daemon
 *
 */
#ifdef __cplusplus
extern "C" {
#endif
extern int key_decryptsession(const char *, des_block *);
extern int key_encryptsession(const char *, des_block *);
extern int key_gendes(des_block *);
extern int key_setsecret(const char *);
extern int key_secretkey_is_set(void);
#ifdef __cplusplus
}
#endif

/*
 * Publickey routines.
 */
#ifdef __cplusplus
extern "C" {
#endif
extern int getpublickey (const char *, char *);
extern int getpublicandprivatekey (char *, char *);
extern int getsecretkey (char *, char *, char *);
#ifdef __cplusplus
}
#endif

#ifdef __cplusplus
extern "C" {
#endif
struct svc_req;
struct rpc_msg;
enum auth_stat _svcauth_none (struct svc_req *, struct rpc_msg *);
enum auth_stat _svcauth_short (struct svc_req *, struct rpc_msg *);
enum auth_stat _svcauth_unix (struct svc_req *, struct rpc_msg *);
enum auth_stat _svcauth_gss (struct svc_req *, struct rpc_msg *, bool_t *);
#ifdef __cplusplus
}
#endif

#define AUTH_NONE	0		/* no authentication */
#define	AUTH_NULL	0		/* backward compatibility */
#define	AUTH_SYS	1		/* unix style (uid, gids) */
#define AUTH_UNIX	AUTH_SYS
#define	AUTH_SHORT	2		/* short hand unix style */
#define AUTH_DH		3		/* for Diffie-Hellman mechanism */
#define AUTH_DES	AUTH_DH		/* for backward compatibility */
#define AUTH_KERB	4		/* kerberos style */
#define RPCSEC_GSS	6		/* RPCSEC_GSS */

#endif /* !_TIRPC_AUTH_H */
