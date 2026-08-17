/* @(#)svc_auth.h	2.1 88/07/29 4.0 RPCSRC */
/*
 * Copyright (c) 2010, Oracle America, Inc.
 *
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 *     * Redistributions of source code must retain the above copyright
 *       notice, this list of conditions and the following disclaimer.
 *
 *     * Redistributions in binary form must reproduce the above copyright
 *       notice, this list of conditions and the following disclaimer in
 *       the documentation and/or other materials provided with the
 *       distribution.
 *
 *     * Neither the name of the "Oracle America, Inc." nor the names of
 *       its contributors may be used to endorse or promote products
 *       derived from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS
 * IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED
 * TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A
 * PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED
 * TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
 * LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
 * NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */
/*      @(#)svc_auth.h 1.6 86/07/16 SMI      */

/*
 * svc_auth.h, Service side of rpc authentication.
 */

/*
 * Interface to server-side authentication flavors.
 */

#ifndef GSSRPC_SVC_AUTH_H
#define GSSRPC_SVC_AUTH_H

#include <gssapi/gssapi.h>

GSSRPC__BEGIN_DECLS

struct svc_req;

typedef struct SVCAUTH {
	struct svc_auth_ops {
		int	(*svc_ah_wrap)(struct SVCAUTH *, XDR *, xdrproc_t,
				       caddr_t);
		int	(*svc_ah_unwrap)(struct SVCAUTH *, XDR *, xdrproc_t,
					 caddr_t);
		int	(*svc_ah_destroy)(struct SVCAUTH *);
	} *svc_ah_ops;
	void * svc_ah_private;
} SVCAUTH;

#ifdef GSSRPC__IMPL

extern SVCAUTH svc_auth_none;

extern struct svc_auth_ops svc_auth_none_ops;
extern struct svc_auth_ops svc_auth_gssapi_ops;
extern struct svc_auth_ops svc_auth_gss_ops;

/*
 * Server side authenticator
 */
/* RENAMED: should be _authenticate. */
extern enum auth_stat gssrpc__authenticate(struct svc_req *rqst,
	struct rpc_msg *msg, bool_t *no_dispatch);

#define SVCAUTH_WRAP(auth, xdrs, xfunc, xwhere) \
     ((*((auth)->svc_ah_ops->svc_ah_wrap))(auth, xdrs, xfunc, xwhere))
#define SVCAUTH_UNWRAP(auth, xdrs, xfunc, xwhere) \
     ((*((auth)->svc_ah_ops->svc_ah_unwrap))(auth, xdrs, xfunc, xwhere))
#define SVCAUTH_DESTROY(auth) \
     ((*((auth)->svc_ah_ops->svc_ah_destroy))(auth))

/* no authentication */
/* RENAMED: should be _svcauth_none. */
enum auth_stat gssrpc__svcauth_none(struct svc_req *,
	struct rpc_msg *, bool_t *);
/* unix style (uid, gids) */
/* RENAMED: shoudl be _svcauth_unix. */
enum auth_stat gssrpc__svcauth_unix(struct svc_req *,
	struct rpc_msg *, bool_t *);
/* short hand unix style */
/* RENAMED: should be _svcauth_short. */
enum auth_stat gssrpc__svcauth_short(struct svc_req *,
	struct rpc_msg *, bool_t *);
/* GSS-API style */
/* RENAMED: should be _svcauth_gssapi. */
enum auth_stat gssrpc__svcauth_gssapi(struct svc_req *,
	struct rpc_msg *, bool_t *);
/* RPCSEC_GSS */
enum auth_stat gssrpc__svcauth_gss(struct svc_req *,
	struct rpc_msg *, bool_t *);

#endif /* defined(GSSRPC__IMPL) */

/*
 * Approved way of getting principal of caller
 */
char *svcauth_gss_get_principal(SVCAUTH *auth);
/*
 * Approved way of setting server principal
 */
bool_t svcauth_gss_set_svc_name(gss_name_t name);

GSSRPC__END_DECLS

#endif /* !defined(GSSRPC_SVC_AUTH_H) */
