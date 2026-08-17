/* include/gssrpc/rename.h */
/*
 * Copyright (C) 2004 by the Massachusetts Institute of Technology.
 * All rights reserved.
 *
 * Export of this software from the United States of America may
 *   require a specific license from the United States Government.
 *   It is the responsibility of any person or organization contemplating
 *   export to obtain such a license before exporting.
 *
 * WITHIN THAT CONSTRAINT, permission to use, copy, modify, and
 * distribute this software and its documentation for any purpose and
 * without fee is hereby granted, provided that the above copyright
 * notice appear in all copies and that both that copyright notice and
 * this permission notice appear in supporting documentation, and that
 * the name of M.I.T. not be used in advertising or publicity pertaining
 * to distribution of the software without specific, written prior
 * permission.  Furthermore if you modify this software you must label
 * your software as modified software and not distribute it in such a
 * fashion that it might be confused with the original M.I.T. software.
 * M.I.T. makes no representations about the suitability of
 * this software for any purpose.  It is provided "as is" without express
 * or implied warranty.
 */

/*
 *
 * Namespace mangling for various purposes.
 *
 * Symbols in the object code need to be renamed to not conflict with
 * an OS-provided RPC implementation.  Without renaming, the conflicts
 * can cause problems with things like RPC-enabled NSS
 * implementations.
 *
 * Symbols in headers should not conflict with implementation-reserved
 * namespace (prefixes "_[A-Z_]" for any purpose; prefix "_"
 * for file scope identifiers and tag names), or unnecessarily impinge
 * on user namespace.
 *
 * The renaming of the header directory is done to avoid problems when
 * the OS header files include <rpc/foo.h> and might get ours instead.
 * OS vendors should replace all the <gssrpc/foo.h> inclusions with
 * <rpc/foo.h> inclusions, as appropriate.  Additionally, vendors
 * should probably put some symbols into the implementation namespace.
 *
 * For example, inclusion protection should change from "GSSRPC_*_H"
 * to "_RPC_*_H", struct tags should get "__" prefixes, etc.
 *
 * This implementation reserves the object code prefix "gssrpc_".
 * External names in the RPC API not beginning with "_" get renamed
 * with the prefix "gssrpc_" via #define, e.g., "foo" -> "gssrpc_foo".
 * External names in the RPC API beginning with "_" get textually
 * rewritten.
 */

#ifndef GSSRPC_RENAME_H
#define GSSRPC_RENAME_H

/* auth.h */

#define xdr_des_block		gssrpc_xdr_des_block

#define authany_wrap		gssrpc_authany_wrap
#define authany_unwrap		gssrpc_authany_unwrap

#define authunix_create		gssrpc_authunix_create
#define authunix_create_default	gssrpc_authunix_create_default
#define authnone_create		gssrpc_authnone_create
#define authdes_create		gssrpc_authdes_create
#define xdr_opaque_auth		gssrpc_xdr_opaque_auth

/* auth_gss.c */

#define auth_debug_gss		gssrpc_auth_debug_gss
#define misc_debug_gss		gssrpc_misc_debug_gss

/* auth_gss.h */

#define xdr_rpc_gss_buf		gssrpc_xdr_rpc_gss_buf
#define xdr_rpc_gss_cred	gssrpc_xdr_rpc_gss_cred
#define xdr_rpc_gss_init_args	gssrpc_xdr_rpc_gss_init_args
#define xdr_rpc_gss_init_res	gssrpc_xdr_rpc_gss_init_res
#define xdr_rpc_gss_data	gssrpc_xdr_rpc_gss_data
#define xdr_rpc_gss_wrap_data	gssrpc_xdr_rpc_gss_wrap_data
#define xdr_rpc_gss_unwrap_data	gssrpc_xdr_rpc_gss_unwrap_data

#define authgss_create		gssrpc_authgss_create
#define authgss_create_default	gssrpc_authgss_create_default
#define authgss_get_private_data	gssrpc_authgss_get_private_data
#define authgss_service		gssrpc_authgss_service

#ifdef GSSRPC__IMPL
#define log_debug		gssrpc_log_debug
#define log_status		gssrpc_log_status
#define	log_hexdump		gssrpc_log_hexdump
#endif

/* auth_gssapi.c */

#define auth_debug_gssapi	gssrpc_auth_debug_gssapi
#define misc_debug_gssapi	gssrpc_misc_debug_gssapi

/* auth_gssapi.h */

#define xdr_gss_buf		gssrpc_xdr_gss_buf
#define xdr_authgssapi_creds	gssrpc_xdr_authgssapi_creds
#define xdr_authgssapi_init_arg	gssrpc_xdr_authgssapi_init_arg
#define xdr_authgssapi_init_res	gssrpc_xdr_authgssapi_init_res

#define auth_gssapi_wrap_data	gssrpc_auth_gssapi_wrap_data
#define auth_gssapi_unwrap_data	gssrpc_auth_gssapi_unwrap_data
#define auth_gssapi_create	gssrpc_auth_gssapi_create
#define auth_gssapi_create_default	gssrpc_auth_gssapi_create_default
#define auth_gssapi_display_status	gssrpc_auth_gssapi_display_status
#define auth_gssapi_seal_seq	gssrpc_auth_gssapi_seal_seq
#define auth_gssapi_unseal_seq	gssrpc_auth_gssapi_unseal_seq

#define svcauth_gssapi_set_names	gssrpc_svcauth_gssapi_set_names
#define svcauth_gssapi_unset_names	gssrpc_svcauth_gssapi_unset_names
#define svcauth_gssapi_set_log_badauth_func	gssrpc_svcauth_gssapi_set_log_badauth_func
#define svcauth_gssapi_set_log_badauth2_func	gssrpc_svcauth_gssapi_set_log_badauth2_func
#define svcauth_gssapi_set_log_badverf_func	gssrpc_svcauth_gssapi_set_log_badverf_func
#define svcauth_gssapi_set_log_miscerr_func	gssrpc_svcauth_gssapi_set_log_miscerr_func

#define svcauth_gss_set_log_badauth_func	gssrpc_svcauth_gss_set_log_badauth_func
#define svcauth_gss_set_log_badauth2_func	gssrpc_svcauth_gss_set_log_badauth2_func
#define svcauth_gss_set_log_badverf_func	gssrpc_svcauth_gss_set_log_badverf_func
#define svcauth_gss_set_log_miscerr_func	gssrpc_svcauth_gss_set_log_miscerr_func

/* auth_unix.h */

#define xdr_authunix_parms	gssrpc_xdr_authunix_parms

/* clnt.h */

#define clntraw_create		gssrpc_clntraw_create
#define clnt_create		gssrpc_clnt_create
#define clnttcp_create		gssrpc_clnttcp_create
#define clntudp_create		gssrpc_clntudp_create
#define clntudp_bufcreate	gssrpc_clntudp_bufcreate
#define clnt_pcreateerror	gssrpc_clnt_pcreateerror
#define clnt_spcreateerror	gssrpc_clnt_spcreateerror
#define clnt_perrno		gssrpc_clnt_perrno
#define clnt_perror		gssrpc_clnt_perror
#define clnt_sperror		gssrpc_clnt_sperror
/* XXX do we need to rename the struct? */
#define rpc_createerr		gssrpc_rpc_createrr
#define clnt_sperrno		gssrpc_clnt_sperrno

/* pmap_clnt.h */

#define pmap_set		gssrpc_pmap_set
#define pmap_unset		gssrpc_pmap_unset
#define pmap_getmaps		gssrpc_pmap_getmaps
#define pmap_rmtcall		gssrpc_pmap_rmtcall
#define clnt_broadcast		gssrpc_clnt_broadcast
#define pmap_getport		gssrpc_pmap_getport

/* pmap_prot.h */

#define xdr_pmap		gssrpc_xdr_pmap
#define xdr_pmaplist		gssrpc_xdr_pmaplist

/* pmap_rmt.h */

#define xdr_rmtcall_args	gssrpc_xdr_rmtcall_args
#define xdr_rmtcallres		gssrpc_xdr_rmtcallres

/* rpc.h */

#define get_myaddress		gssrpc_get_myaddress
#define bindresvport		gssrpc_bindresvport
#define bindresvport_sa		gssrpc_bindresvport_sa
#define callrpc			gssrpc_callrpc
#define getrpcport		gssrpc_getrpcport

/* rpc_msg.h */

#define xdr_callmsg		gssrpc_xdr_callmsg
#define xdr_callhdr		gssrpc_xdr_callhdr
#define xdr_replymsg		gssrpc_xdr_replymsg
#define xdr_accepted_reply	gssrpc_xdr_accepted_reply
#define xdr_rejected_reply	gssrpc_xdr_rejected_reply

/* svc.h */

#define svc_register		gssrpc_svc_register
#define registerrpc             gssrpc_registerrpc
#define svc_unregister		gssrpc_svc_unregister
#define xprt_register		gssrpc_xprt_register
#define xprt_unregister		gssrpc_xprt_unregister

#define svc_sendreply		gssrpc_svc_sendreply
#define svcerr_decode		gssrpc_svcerr_decode
#define svcerr_weakauth		gssrpc_svcerr_weakauth
#define svcerr_noproc		gssrpc_svcerr_noproc
#define svcerr_progvers		gssrpc_svcerr_progvers
#define svcerr_auth		gssrpc_svcerr_auth
#define svcerr_noprog		gssrpc_svcerr_noprog
#define svcerr_systemerr	gssrpc_svcerr_systemerr

#define svc_maxfd		gssrpc_svc_maxfd
#define svc_fdset		gssrpc_svc_fdset
#define svc_fds			gssrpc_svc_fds

#define rpctest_service		gssrpc_rpctest_service

#define svc_getreq		gssrpc_svc_getreq
#define svc_getreqset		gssrpc_svc_getreqset
#define svc_getreqset2		gssrpc_svc_getreqset2
#define svc_run			gssrpc_svc_run

#define svcraw_create		gssrpc_svcraw_create

#define svcudp_create		gssrpc_svcudp_create
#define svcudp_bufcreate	gssrpc_svcudp_bufcreate
#define svcudp_enablecache	gssrpc_svcudp_enablecache

#define svctcp_create		gssrpc_svctcp_create

#define svcfd_create            gssrpc_svcfd_create

/* svc_auth.h */

#define svc_auth_none_ops	gssrpc_svc_auth_none_ops
#define svc_auth_gssapi_ops	gssrpc_svc_auth_gssapi_ops
#define svc_auth_gss_ops	gssrpc_svc_auth_gss_ops

#define svcauth_gss_set_svc_name	gssrpc_svcauth_gss_set_svc_name
#define svcauth_gss_get_principal	gssrpc_svcauth_gss_get_principal

/* svc_auth_gss.c */

#define svc_debug_gss		gssrpc_svc_debug_gss

/* svc_auth_gssapi.c */

#define svc_debug_gssapi	gssrpc_svc_debug_gssapi

/* svc_auth_none.c */

#define svc_auth_none		gssrpc_svc_auth_none

/* xdr.h */

#define xdr_void	gssrpc_xdr_void
#define xdr_int		gssrpc_xdr_int
#define xdr_u_int	gssrpc_xdr_u_int
#define xdr_long	gssrpc_xdr_long
#define xdr_u_long	gssrpc_xdr_u_long
#define xdr_short	gssrpc_xdr_short
#define xdr_u_short	gssrpc_xdr_u_short
#define xdr_bool	gssrpc_xdr_bool
#define xdr_enum	gssrpc_xdr_enum
#define xdr_array	gssrpc_xdr_array
#define xdr_bytes	gssrpc_xdr_bytes
#define xdr_opaque	gssrpc_xdr_opaque
#define xdr_string	gssrpc_xdr_string
#define xdr_union	gssrpc_xdr_union
#define xdr_char	gssrpc_xdr_char
#define xdr_u_char	gssrpc_xdr_u_char
#define xdr_vector	gssrpc_xdr_vector
#define xdr_float	gssrpc_xdr_float
#define xdr_double	gssrpc_xdr_double
#define xdr_reference	gssrpc_xdr_reference
#define xdr_pointer	gssrpc_xdr_pointer
#define xdr_wrapstring	gssrpc_xdr_wrapstring
#define xdr_free	gssrpc_xdr_free

#define xdr_sizeof	gssrpc_xdr_sizeof

#define xdr_netobj	gssrpc_xdr_netobj
#define xdr_int32	gssrpc_xdr_int32
#define xdr_u_int32	gssrpc_xdr_u_int32

#define xdralloc_create		gssrpc_xdralloc_create
#define xdralloc_release	gssrpc_xdralloc_release
#define xdralloc_getdata	gssrpc_xdralloc_getdata

#define xdrmem_create		gssrpc_xdrmem_create
#define xdrstdio_create		gssrpc_xdrstdio_create
#define xdrrec_create		gssrpc_xdrrec_create
#define xdrrec_endofrecord	gssrpc_xdrrec_endofrecord
#define xdrrec_skiprecord	gssrpc_xdrrec_skiprecord
#define xdrrec_eof		gssrpc_xdrrec_eof

#endif /* !defined(GSSRPC_RENAME_H) */
