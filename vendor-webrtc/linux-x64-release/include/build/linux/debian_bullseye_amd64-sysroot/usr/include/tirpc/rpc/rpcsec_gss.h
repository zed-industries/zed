/*
 * Copyright (c) 2013, Oracle America, Inc.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 * - Redistributions of source code must retain the above copyright notice,
 *   this list of conditions and the following disclaimer.
 * - Redistributions in binary form must reproduce the above copyright notice,
 *   this list of conditions and the following disclaimer in the documentation
 *   and/or other materials provided with the distribution.
 * - Neither the name of "Oracle America, Inc." nor the names of its
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
 */

#ifndef _TIRPC_RPCSEC_GSS_H
#define _TIRPC_RPCSEC_GSS_H

#include <sys/types.h>

#include <rpc/auth.h>
#include <rpc/clnt.h>

#include <gssapi/gssapi.h>

typedef enum {
	rpcsec_gss_svc_default	= 0,
	rpcsec_gss_svc_none	= 1,
	rpcsec_gss_svc_integrity = 2,
	rpcsec_gss_svc_privacy	= 3
} rpc_gss_service_t;

typedef struct {
	int			len;
	char			name[1];
} *rpc_gss_principal_t;

typedef struct {
	int			req_flags;
	int			time_req;
	gss_cred_id_t		my_cred;
	gss_channel_bindings_t	input_channel_bindings;
} rpc_gss_options_req_t;

#define MAX_GSS_MECH		128
typedef struct {
	int			major_status;
	int			minor_status;
	u_int			rpcsec_version;
	int			ret_flags;
	int			time_ret;
	gss_ctx_id_t		gss_context;
	char			actual_mechanism[MAX_GSS_MECH];
} rpc_gss_options_ret_t;

typedef struct {
	u_int			version;
	char			*mechanism;
	char			*qop;
	rpc_gss_principal_t	client_principal;
	char			*svc_principal;
	rpc_gss_service_t	service;
} rpc_gss_rawcred_t;

typedef struct {
	uid_t			uid;
	gid_t			gid;
	short			gidlen;
	gid_t			*gidlist;
} rpc_gss_ucred_t;

typedef struct {
	bool_t			locked;
	rpc_gss_rawcred_t	*raw_cred;
} rpc_gss_lock_t;

typedef struct {
	u_int			program;
	u_int			version;
	bool_t			(*callback)(struct svc_req *,
					gss_cred_id_t, gss_ctx_id_t,
					rpc_gss_lock_t *, void **);
} rpc_gss_callback_t;

typedef struct {
	int			rpc_gss_error;
	int			system_error;
} rpc_gss_error_t;
#define RPC_GSS_ER_SUCCESS	0
#define RPC_GSS_ER_SYSTEMERROR	1

typedef gss_OID_desc rpc_gss_OID_desc;
typedef rpc_gss_OID_desc *rpc_gss_OID;


#ifdef __cplusplus
extern "C" {
#endif

AUTH	*rpc_gss_seccreate(CLIENT *, char *, char *, rpc_gss_service_t,
				char *, rpc_gss_options_req_t *,
				rpc_gss_options_ret_t *);
bool_t	rpc_gss_set_defaults(AUTH *, rpc_gss_service_t, char *);
int	rpc_gss_max_data_length(AUTH *, int);
int	rpc_gss_svc_max_data_length(struct svc_req *, int);
bool_t	rpc_gss_set_svc_name(char *, char *, u_int, u_int, u_int);
bool_t	rpc_gss_getcred(struct svc_req *, rpc_gss_rawcred_t **,
				rpc_gss_ucred_t **, void **);
bool_t	rpc_gss_set_callback(rpc_gss_callback_t *);
bool_t	rpc_gss_get_principal_name(rpc_gss_principal_t *, char *,
				char *, char *, char *);
void	rpc_gss_get_error(rpc_gss_error_t *);
char	**rpc_gss_get_mechanisms(void);
char	**rpc_gss_get_mech_info(char *, rpc_gss_service_t *);
bool_t	rpc_gss_get_versions(u_int *, u_int *);
bool_t	rpc_gss_is_installed(char *);
bool_t	rpc_gss_mech_to_oid(char *, rpc_gss_OID *);
bool_t	rpc_gss_qop_to_num(char *, char *, u_int *);

#ifdef __cplusplus
}
#endif

#endif	/* !_TIRPC_RPCSEC_GSS_H */
