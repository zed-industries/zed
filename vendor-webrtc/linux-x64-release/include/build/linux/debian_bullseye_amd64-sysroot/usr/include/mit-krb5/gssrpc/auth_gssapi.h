/* include/gssrpc/auth_gssapi.h - GSS-API style auth parameters for RPC */
/*
 * Copyright 1993 OpenVision Technologies, Inc., All Rights Reserved.
 */

#ifndef GSSRPC_AUTH_GSSAPI_H
#define GSSRPC_AUTH_GSSAPI_H

GSSRPC__BEGIN_DECLS

#define AUTH_GSSAPI_EXIT		0
#define AUTH_GSSAPI_INIT 		1
#define AUTH_GSSAPI_CONTINUE_INIT 	2
#define AUTH_GSSAPI_MSG 		3
#define AUTH_GSSAPI_DESTROY 		4

/*
 * Yuck.  Some sys/types.h files leak symbols
 */
#ifdef major
#undef major
#endif
#ifdef minor
#undef minor
#endif

typedef struct _auth_gssapi_name {
     char *name;
     gss_OID type;
} auth_gssapi_name;

typedef struct _auth_gssapi_creds {
     uint32_t version;
     bool_t auth_msg;
     gss_buffer_desc client_handle;
} auth_gssapi_creds;

typedef struct _auth_gssapi_init_arg {
     uint32_t version;
     gss_buffer_desc token;
} auth_gssapi_init_arg;

typedef struct _auth_gssapi_init_res {
     uint32_t version;
     gss_buffer_desc client_handle;
     OM_uint32 gss_major, gss_minor;
     gss_buffer_desc token;
     gss_buffer_desc signed_isn;
} auth_gssapi_init_res;

typedef void (*auth_gssapi_log_badauth_func)
     (OM_uint32 major,
		OM_uint32 minor,
		struct sockaddr_in *raddr,
		caddr_t data);

/* auth_gssapi_log_badauth_func is IPv4-specific; this version gives the
 * transport handle so the fd can be used to get the address. */
typedef void (*auth_gssapi_log_badauth2_func)
     (OM_uint32 major,
		OM_uint32 minor,
		SVCXPRT *xprt,
		caddr_t data);

typedef void (*auth_gssapi_log_badverf_func)
     (gss_name_t client,
		gss_name_t server,
		struct svc_req *rqst,
		struct rpc_msg *msg,
		caddr_t data);

typedef void (*auth_gssapi_log_miscerr_func)
     (struct svc_req *rqst,
		struct rpc_msg *msg,
		char *error,
		caddr_t data);

bool_t xdr_gss_buf(XDR *, gss_buffer_t);
bool_t xdr_authgssapi_creds(XDR *, auth_gssapi_creds *);
bool_t xdr_authgssapi_init_arg(XDR *, auth_gssapi_init_arg *);
bool_t xdr_authgssapi_init_res(XDR *, auth_gssapi_init_res *);

bool_t auth_gssapi_wrap_data
(OM_uint32 *major, OM_uint32 *minor,
	   gss_ctx_id_t context, uint32_t seq_num, XDR
	   *out_xdrs, bool_t (*xdr_func)(), caddr_t
	   xdr_ptr);
bool_t auth_gssapi_unwrap_data
(OM_uint32 *major, OM_uint32 *minor,
	   gss_ctx_id_t context, uint32_t seq_num, XDR
	   *in_xdrs, bool_t (*xdr_func)(), caddr_t
	   xdr_ptr);

AUTH *auth_gssapi_create
(CLIENT *clnt,
	   OM_uint32 *major_status,
	   OM_uint32 *minor_status,
	   gss_cred_id_t claimant_cred_handle,
	   gss_name_t target_name,
	   gss_OID mech_type,
	   OM_uint32 req_flags,
	   OM_uint32 time_req,
	   gss_OID *actual_mech_type,
	   OM_uint32 *ret_flags,
	   OM_uint32 *time_rec);

AUTH *auth_gssapi_create_default
(CLIENT *clnt, char *service_name);

void auth_gssapi_display_status
(char *msg, OM_uint32 major,
	   OM_uint32 minor);

bool_t auth_gssapi_seal_seq
(gss_ctx_id_t context, uint32_t seq_num, gss_buffer_t out_buf);

bool_t auth_gssapi_unseal_seq
(gss_ctx_id_t context, gss_buffer_t in_buf, uint32_t *seq_num);

bool_t svcauth_gssapi_set_names
(auth_gssapi_name *names, int num);
void svcauth_gssapi_unset_names
(void);

void svcauth_gssapi_set_log_badauth_func
(auth_gssapi_log_badauth_func func,
	   caddr_t data);
void svcauth_gssapi_set_log_badauth2_func
(auth_gssapi_log_badauth2_func func,
	   caddr_t data);
void svcauth_gssapi_set_log_badverf_func
(auth_gssapi_log_badverf_func func,
	   caddr_t data);
void svcauth_gssapi_set_log_miscerr_func
(auth_gssapi_log_miscerr_func func,
	   caddr_t data);

void svcauth_gss_set_log_badauth_func(auth_gssapi_log_badauth_func,
				      caddr_t);
void svcauth_gss_set_log_badauth2_func(auth_gssapi_log_badauth2_func,
				       caddr_t);
void svcauth_gss_set_log_badverf_func(auth_gssapi_log_badverf_func,
				      caddr_t);
void svcauth_gss_set_log_miscerr_func(auth_gssapi_log_miscerr_func,
				      caddr_t data);

#define GSS_COPY_BUFFER(dest, src) { \
     (dest).length = (src).length; \
     (dest).value = (src).value; }

#define GSS_DUP_BUFFER(dest, src) { \
     (dest).length = (src).length; \
     (dest).value = (void *) malloc((dest).length); \
     memcpy((dest).value, (src).value, (dest).length); }

#define GSS_BUFFERS_EQUAL(b1, b2) (((b1).length == (b2).length) && \
				   !memcmp((b1).value,(b2).value,(b1.length)))


GSSRPC__END_DECLS

#endif /* !defined(GSSRPC_AUTH_GSSAPI_H) */
