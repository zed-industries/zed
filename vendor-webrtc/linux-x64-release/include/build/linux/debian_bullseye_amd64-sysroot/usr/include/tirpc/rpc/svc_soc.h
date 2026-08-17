/*	$NetBSD: svc_soc.h,v 1.1 2000/06/02 22:57:57 fvdl Exp $	*/
/*	$FreeBSD: src/include/rpc/svc_soc.h,v 1.2 2002/03/23 17:24:55 imp Exp $ */

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
 */
/*
 * Copyright (c) 1986 - 1991 by Sun Microsystems, Inc.
 */

/*
 * svc.h, Server-side remote procedure call interface.
 */

#ifndef _RPC_SVC_SOC_H
#define _RPC_SVC_SOC_H

/* #pragma ident   "@(#)svc_soc.h  1.11    94/04/25 SMI" */
/*      svc_soc.h 1.8 89/05/01 SMI      */

/*
 * All the following declarations are only for backward compatibility
 * with TS-RPC
 */

/*
 *  Approved way of getting address of caller
 */
#define svc_getcaller(x) (&(x)->xp_raddr)
/* Getting address of a caller using netbuf xp_rtaddr */ 
#define svc_getcaller_netbuf(x) (&(x)->xp_rtaddr)
/*
 * Service registration
 *
 * svc_register(xprt, prog, vers, dispatch, protocol)
 *	SVCXPRT *xprt;
 *	u_long prog;
 *	u_long vers;
 *	void (*dispatch)();
 *	int protocol;    like TCP or UDP, zero means do not register 
 */
#ifdef __cplusplus
extern "C" {
#endif
extern bool_t	svc_register(SVCXPRT *, u_long, u_long,
		    void (*)(struct svc_req *, SVCXPRT *), int);
#ifdef __cplusplus
}
#endif

/*
 * Service un-registration
 *
 * svc_unregister(prog, vers)
 *	u_long prog;
 *	u_long vers;
 */
#ifdef __cplusplus
extern "C" {
#endif
extern void	svc_unregister(u_long, u_long);
#ifdef __cplusplus
}
#endif


/*
 * Memory based rpc for testing and timing.
 */
#ifdef __cplusplus
extern "C" {
#endif
extern SVCXPRT *svcraw_create(void);
#ifdef __cplusplus
}
#endif


/*
 * Udp based rpc.
 */
#ifdef __cplusplus
extern "C" {
#endif
extern SVCXPRT *svcudp_create(int);
extern SVCXPRT *svcudp_bufcreate(int, u_int, u_int);
extern int svcudp_enablecache(SVCXPRT *, u_long);
extern SVCXPRT *svcudp6_create(int);
extern SVCXPRT *svcudp6_bufcreate(int, u_int, u_int);
#ifdef __cplusplus
}
#endif


/*
 * Tcp based rpc.
 */
#ifdef __cplusplus
extern "C" {
#endif
extern SVCXPRT *svctcp_create(int, u_int, u_int);
extern SVCXPRT *svctcp6_create(int, u_int, u_int);
#ifdef __cplusplus
}
#endif

/*
 * Fd based rpc.
 */
#ifdef __cplusplus
extern "C" {
#endif
extern SVCXPRT *svcfd_create(int, u_int, u_int);
#ifdef __cplusplus
}
#endif

#endif /* !_RPC_SVC_SOC_H */
