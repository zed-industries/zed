/*
 * Copyright (c) 2015, Axentia Technologies AB.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 * - Redistributions of source code must retain the above copyright notice,
 *   this list of conditions and the following disclaimer.
 * - Redistributions in binary form must reproduce the above copyright notice,
 *   this list of conditions and the following disclaimer in the documentation
 *   and/or other materials provided with the distribution.
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
 */

/*
 * svc_mt.h, Server-side transport extensions
 */

#ifndef _TIRPC_SVC_MT_H
#define _TIRPC_SVC_MT_H

typedef struct __rpc_svcxprt_ext {
	int 		flags;
	SVCAUTH		xp_auth;
} SVCXPRT_EXT;


#define SVCEXT(xprt)					\
	((SVCXPRT_EXT *)(xprt)->xp_p3)

#define SVC_XP_AUTH(xprt)				\
	(SVCEXT(xprt)->xp_auth)

#define SVC_VERSQUIET 0x0001	/* keep quiet about version mismatch */

#define svc_flags(xprt)					\
	(SVCEXT(xprt)->flags)

#define version_keepquiet(xprt)				\
	(svc_flags(xprt) & SVC_VERSQUIET)

#endif /* !_TIRPC_SVC_MT_H */
