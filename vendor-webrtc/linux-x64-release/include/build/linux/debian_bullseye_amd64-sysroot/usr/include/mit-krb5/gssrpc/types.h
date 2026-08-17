/* @(#)types.h	2.3 88/08/15 4.0 RPCSRC */
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
 *     * Neither the name of the “Oracle America, Inc.” nor the names of
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
/*      @(#)types.h 1.18 87/07/24 SMI      */

/*
 * Rpc additions to <sys/types.h>
 */
#ifndef GSSRPC_TYPES_H
#define GSSRPC_TYPES_H

#include <sys/types.h>

#include <sys/select.h>
#include <sys/time.h>
#include <unistd.h>

/*
 * Try to get MAXHOSTNAMELEN from somewhere.
 */
#include <sys/param.h>
/* #include <netdb.h> */

/* Get htonl(), ntohl(), etc. */
#include <netinet/in.h>

#include <stdlib.h>
#include <stdint.h>
#include <limits.h>

#ifndef GSSRPC__BEGIN_DECLS
#ifdef __cplusplus
#define GSSRPC__BEGIN_DECLS	extern "C" {
#define GSSRPC__END_DECLS	}
#else
#define GSSRPC__BEGIN_DECLS
#define GSSRPC__END_DECLS
#endif
#endif

GSSRPC__BEGIN_DECLS

#if defined(CHAR_BIT) && CHAR_BIT != 8
#error "Bytes must be exactly 8 bits."
#endif

/* Define if we need to fake up some BSD type aliases. */
#ifndef GSSRPC__BSD_TYPEALIASES	/* Allow application to override. */
/* #undef GSSRPC__BSD_TYPEALIASES */
#endif
#if GSSRPC__BSD_TYPEALIASES
typedef unsigned char	u_char;
typedef unsigned short	u_short;
typedef unsigned int	u_int;
typedef unsigned long	u_long;
#endif

typedef uint32_t	rpcprog_t;
typedef uint32_t	rpcvers_t;
typedef uint32_t	rpcprot_t;
typedef uint32_t	rpcproc_t;
typedef uint32_t	rpcport_t;
typedef int32_t		rpc_inline_t;

/* This is for rpc/netdb.h */
#define STRUCT_RPCENT_IN_RPC_NETDB_H

#define	bool_t	int
#define	enum_t	int
#ifndef FALSE
#	define	FALSE	(0)
#endif
#ifndef TRUE
#	define	TRUE	(1)
#endif
/* XXX namespace */
#define __dontcare__	-1
#ifndef NULL
#	define NULL 0
#endif

/*
 * The below should probably be internal-only, but seem to be
 * traditionally exported in RPC implementations.
 */
#define mem_alloc(bsize)	malloc(bsize)
#define mem_free(ptr, bsize)	free(ptr)

#ifndef INADDR_LOOPBACK
#define       INADDR_LOOPBACK         (uint32_t)0x7F000001
#endif
#ifndef MAXHOSTNAMELEN
#define        MAXHOSTNAMELEN  64
#endif

GSSRPC__END_DECLS

#include <gssrpc/rename.h>

#endif /* !defined(GSSRPC_TYPES_H) */
