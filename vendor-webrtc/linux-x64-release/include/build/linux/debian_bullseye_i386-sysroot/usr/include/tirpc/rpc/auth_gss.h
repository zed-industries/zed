/*
  auth_gss.h
  
  Copyright (c) 2000 The Regents of the University of Michigan.
  All rights reserved.
  
  Copyright (c) 2000 Dug Song <dugsong@UMICH.EDU>.
  All rights reserved, all wrongs reversed.

  Redistribution and use in source and binary forms, with or without
  modification, are permitted provided that the following conditions
  are met:

  1. Redistributions of source code must retain the above copyright
     notice, this list of conditions and the following disclaimer.
  2. Redistributions in binary form must reproduce the above copyright
     notice, this list of conditions and the following disclaimer in the
     documentation and/or other materials provided with the distribution.
  3. Neither the name of the University nor the names of its
     contributors may be used to endorse or promote products derived
     from this software without specific prior written permission.

  THIS SOFTWARE IS PROVIDED ``AS IS'' AND ANY EXPRESS OR IMPLIED
  WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
  MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
  DISCLAIMED. IN NO EVENT SHALL THE REGENTS OR CONTRIBUTORS BE LIABLE
  FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
  CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
  SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR
  BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
  LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
  NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
  SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

*/

#ifndef _TIRPC_AUTH_GSS_H
#define _TIRPC_AUTH_GSS_H

#include <rpc/clnt.h>
#include <gssapi/gssapi.h>

/* RPCSEC_GSS control procedures. */
typedef enum {
	RPCSEC_GSS_DATA = 0,
	RPCSEC_GSS_INIT = 1,
	RPCSEC_GSS_CONTINUE_INIT = 2,
	RPCSEC_GSS_DESTROY = 3
} rpc_gss_proc_t;

/* RPCSEC_GSS services. */
typedef enum {
	RPCSEC_GSS_SVC_NONE = 1,
	RPCSEC_GSS_SVC_INTEGRITY = 2,
	RPCSEC_GSS_SVC_PRIVACY = 3
} rpc_gss_svc_t;

#define RPCSEC_GSS_VERSION	1

/* RPCSEC_GSS security triple. */
struct rpc_gss_sec {
	gss_OID		mech;		/* mechanism */
	gss_qop_t	qop;		/* quality of protection */
	rpc_gss_svc_t	svc;		/* service */
	gss_cred_id_t	cred;		/* cred handle */
	u_int		req_flags;	/* req flags for init_sec_context */
};

/* Private data required for kernel implementation */
struct authgss_private_data {
	gss_ctx_id_t	pd_ctx;		/* Session context handle */
	gss_buffer_desc	pd_ctx_hndl;	/* Credentials context handle */
	u_int		pd_seq_win;	/* Sequence window */
};

/* from kerberos source, gssapi_krb5.c */
extern gss_OID_desc krb5oid;
extern gss_OID_desc spkm3oid;

/* Credentials. */
struct rpc_gss_cred {
	u_int		gc_v;		/* version */
	rpc_gss_proc_t	gc_proc;	/* control procedure */
	u_int		gc_seq;		/* sequence number */
	rpc_gss_svc_t	gc_svc;		/* service */
	gss_buffer_desc	gc_ctx;		/* context handle */
};

/* Context creation response. */
struct rpc_gss_init_res {
	gss_buffer_desc		gr_ctx;		/* context handle */
	u_int			gr_major;	/* major status */
	u_int			gr_minor;	/* minor status */
	u_int			gr_win;		/* sequence window */
	gss_buffer_desc		gr_token;	/* token */
};

/* Maximum sequence number value. */
#define MAXSEQ		0x80000000

/* Prototypes. */
#ifdef __cplusplus
extern "C" {
#endif
bool_t	xdr_rpc_gss_cred	(XDR *xdrs, struct rpc_gss_cred *p);
bool_t	xdr_rpc_gss_init_args	(XDR *xdrs, gss_buffer_desc *p);
bool_t	xdr_rpc_gss_init_res	(XDR *xdrs, struct rpc_gss_init_res *p);
bool_t	xdr_rpc_gss_data	(XDR *xdrs, xdrproc_t xdr_func,
				 caddr_t xdr_ptr, gss_ctx_id_t ctx,
				 gss_qop_t qop, rpc_gss_svc_t svc,
				 u_int seq);

AUTH   *authgss_create		(CLIENT *, gss_name_t, struct rpc_gss_sec *);
AUTH   *authgss_create_default	(CLIENT *, char *, struct rpc_gss_sec *);
bool_t authgss_service		(AUTH *auth, int svc);
bool_t authgss_get_private_data	(AUTH *auth, struct authgss_private_data *);
bool_t authgss_free_private_data (struct authgss_private_data *);

void	gss_log_debug		(const char *fmt, ...);
void	gss_log_status		(char *m, OM_uint32 major, OM_uint32 minor);
void	gss_log_hexdump		(const u_char *buf, int len, int offset);

#ifdef __cplusplus
}
#endif

#endif /* !_TIRPC_AUTH_GSS_H */
