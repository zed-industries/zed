/*
 * Copyright (c) 1983, 1987, 1989
 *    The Regents of the University of California.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 4. Neither the name of the University nor the names of its contributors
 *    may be used to endorse or promote products derived from this software
 *    without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE REGENTS AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE REGENTS OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */

/*
 * Portions Copyright (c) 1996-1999 by Internet Software Consortium.
 *
 * Permission to use, copy, modify, and distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND INTERNET SOFTWARE CONSORTIUM DISCLAIMS
 * ALL WARRANTIES WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES
 * OF MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL INTERNET SOFTWARE
 * CONSORTIUM BE LIABLE FOR ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL
 * DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR
 * PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS
 * ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS
 * SOFTWARE.
 */

/*
 *	@(#)resolv.h	8.1 (Berkeley) 6/2/93
 *	$BINDId: resolv.h,v 8.31 2000/03/30 20:16:50 vixie Exp $
 */

#ifndef _RESOLV_H_
#define _RESOLV_H_

#include <sys/cdefs.h>
#include <sys/param.h>
#include <sys/types.h>
#include <stdio.h>
#include <netinet/in.h>
#include <arpa/nameser.h>
#include <bits/types/res_state.h>

/*
 * Global defines and variables for resolver stub.
 */
#define LOCALDOMAINPARTS	2	/* min levels in name that is "local" */

#define RES_TIMEOUT		5	/* min. seconds between retries */
#define RES_MAXNDOTS		15	/* should reflect bit field size */
#define RES_MAXRETRANS		30	/* only for resolv.conf/RES_OPTIONS */
#define RES_MAXRETRY		5	/* only for resolv.conf/RES_OPTIONS */
#define RES_DFLRETRY		2	/* Default #/tries. */
#define RES_MAXTIME		65535	/* Infinity, in milliseconds. */

#define nsaddr	nsaddr_list[0]		/* for backward compatibility */

/*
 * Revision information.  This is the release date in YYYYMMDD format.
 * It can change every day so the right thing to do with it is use it
 * in preprocessor commands such as "#if (__RES > 19931104)".  Do not
 * compare for equality; rather, use it to determine whether your resolver
 * is new enough to contain a certain feature.
 */

#define	__RES	19991006

/*
 * Resolver configuration file.
 * Normally not present, but may contain the address of the
 * inital name server(s) to query and the domain search list.
 */

#ifndef _PATH_RESCONF
#define _PATH_RESCONF        "/etc/resolv.conf"
#endif

struct res_sym {
	int	number;		/* Identifying number, like T_MX */
	char *	name;		/* Its symbolic name, like "MX" */
	char *	humanname;	/* Its fun name, like "mail exchanger" */
};

/*
 * Resolver options (keep these in synch with res_debug.c, please)
 */
#define RES_INIT	0x00000001	/* address initialized */
#define RES_DEBUG	0x00000002	/* print debug messages */
#define RES_AAONLY \
  __glibc_macro_warning ("RES_AAONLY is deprecated") 0x00000004
#define RES_USEVC	0x00000008	/* use virtual circuit */
#define RES_PRIMARY \
  __glibc_macro_warning ("RES_PRIMARY is deprecated") 0x00000010
#define RES_IGNTC	0x00000020	/* ignore trucation errors */
#define RES_RECURSE	0x00000040	/* recursion desired */
#define RES_DEFNAMES	0x00000080	/* use default domain name */
#define RES_STAYOPEN	0x00000100	/* Keep TCP socket open */
#define RES_DNSRCH	0x00000200	/* search up local domain tree */
#define	RES_NOALIASES	0x00001000	/* shuts off HOSTALIASES feature */
#define RES_ROTATE	0x00004000	/* rotate ns list after each query */
#define	RES_NOCHECKNAME \
  __glibc_macro_warning ("RES_NOCHECKNAME is deprecated") 0x00008000
#define	RES_KEEPTSIG \
  __glibc_macro_warning ("RES_KEEPTSIG is deprecated") 0x00010000
#define	RES_BLAST \
  __glibc_macro_warning ("RES_BLAST is deprecated") 0x00020000
#define RES_USE_EDNS0	0x00100000	/* Use EDNS0.  */
#define RES_SNGLKUP	0x00200000	/* one outstanding request at a time */
#define RES_SNGLKUPREOP	0x00400000	/* -"-, but open new socket for each
					   request */
#define RES_USE_DNSSEC	0x00800000	/* use DNSSEC using OK bit in OPT */
#define RES_NOTLDQUERY	0x01000000	/* Do not look up unqualified name
					   as a TLD.  */
#define RES_NORELOAD    0x02000000 /* No automatic configuration reload.  */
#define RES_TRUSTAD     0x04000000 /* Request AD bit, keep it in responses.  */

#define RES_DEFAULT	(RES_RECURSE|RES_DEFNAMES|RES_DNSRCH)

/*
 * Resolver "pfcode" values.  Used by dig.
 */
#define RES_PRF_STATS	0x00000001
#define RES_PRF_UPDATE	0x00000002
#define RES_PRF_CLASS   0x00000004
#define RES_PRF_CMD	0x00000008
#define RES_PRF_QUES	0x00000010
#define RES_PRF_ANS	0x00000020
#define RES_PRF_AUTH	0x00000040
#define RES_PRF_ADD	0x00000080
#define RES_PRF_HEAD1	0x00000100
#define RES_PRF_HEAD2	0x00000200
#define RES_PRF_TTLID	0x00000400
#define RES_PRF_HEADX	0x00000800
#define RES_PRF_QUERY	0x00001000
#define RES_PRF_REPLY	0x00002000
#define RES_PRF_INIT	0x00004000
/*			0x00008000	*/

/* Things involving an internal (static) resolver context. */
__BEGIN_DECLS
extern struct __res_state *__res_state(void) __attribute__ ((__const__));
__END_DECLS
#define _res (*__res_state())

#define fp_nquery		__fp_nquery
#define fp_query		__fp_query
#define hostalias		__hostalias
#define p_query			__p_query
#define res_close		__res_close
#define res_init		__res_init
#define res_isourserver		__res_isourserver
#define res_mkquery		__res_mkquery
#define res_query		__res_query
#define res_querydomain		__res_querydomain
#define res_search		__res_search
#define res_send		__res_send

__BEGIN_DECLS
void		fp_nquery (const unsigned char *, int, FILE *) __THROW;
void		fp_query (const unsigned char *, FILE *) __THROW;
const char *	hostalias (const char *) __THROW;
void		p_query (const unsigned char *) __THROW;
void		res_close (void) __THROW;
int		res_init (void) __THROW;
int		res_isourserver (const struct sockaddr_in *) __THROW;
int		res_mkquery (int, const char *, int, int,
			     const unsigned char *, int, const unsigned char *,
			     unsigned char *, int) __THROW;
int		res_query (const char *, int, int, unsigned char *, int)
     __THROW;
int		res_querydomain (const char *, const char *, int, int,
				 unsigned char *, int) __THROW;
int		res_search (const char *, int, int, unsigned char *, int)
     __THROW;
int		res_send (const unsigned char *, int, unsigned char *, int)
     __THROW;
__END_DECLS

#define b64_ntop		__b64_ntop
#define b64_pton		__b64_pton
#define dn_comp			__dn_comp
#define dn_count_labels		__dn_count_labels
#define dn_expand		__dn_expand
#define dn_skipname		__dn_skipname
#define fp_resstat		__fp_resstat
#define loc_aton		__loc_aton
#define loc_ntoa		__loc_ntoa
#define p_cdname		__p_cdname
#define p_cdnname		__p_cdnname
#define p_class			__p_class
#define p_fqname		__p_fqname
#define p_fqnname		__p_fqnname
#define p_option		__p_option
#define p_time			__p_time
#define p_type			__p_type
#define p_rcode			__p_rcode
#define putlong			__putlong
#define putshort		__putshort
#define res_dnok		__res_dnok
#define res_hnok		__res_hnok
#define res_hostalias		__res_hostalias
#define res_mailok		__res_mailok
#define res_nameinquery		__res_nameinquery
#define res_nclose		__res_nclose
#define res_ninit		__res_ninit
#define res_nmkquery		__res_nmkquery
#define res_nquery		__res_nquery
#define res_nquerydomain	__res_nquerydomain
#define res_nsearch		__res_nsearch
#define res_nsend		__res_nsend
#define res_ownok		__res_ownok
#define res_queriesmatch	__res_queriesmatch
#define res_randomid		__res_randomid
#define sym_ntop		__sym_ntop
#define sym_ntos		__sym_ntos
#define sym_ston		__sym_ston
__BEGIN_DECLS
int		res_hnok (const char *) __THROW;
int		res_ownok (const char *) __THROW;
int		res_mailok (const char *) __THROW;
int		res_dnok (const char *) __THROW;
int		sym_ston (const struct res_sym *, const char *, int *) __THROW;
const char *	sym_ntos (const struct res_sym *, int, int *) __THROW;
const char *	sym_ntop (const struct res_sym *, int, int *) __THROW;
int		b64_ntop (const unsigned char *, size_t, char *, size_t)
     __THROW;
int		b64_pton (char const *, unsigned char *, size_t) __THROW;
int		loc_aton (const char *__ascii, unsigned char *__binary) __THROW;
const char *	loc_ntoa (const unsigned char *__binary, char *__ascii) __THROW;
int		dn_skipname (const unsigned char *, const unsigned char *)
     __THROW;
void		putlong (uint32_t, unsigned char *) __THROW;
void		putshort (uint16_t, unsigned char *) __THROW;
const char *	p_class (int) __THROW;
const char *	p_time (uint32_t) __THROW;
const char *	p_type (int) __THROW;
const char *	p_rcode (int) __THROW;
const unsigned char * p_cdnname (const unsigned char *,
				 const unsigned char *, int, FILE *) __THROW;
const unsigned char * p_cdname (const unsigned char *, const unsigned char *,
				FILE *) __THROW;
const unsigned char * p_fqnname (const unsigned char *__cp,
				 const unsigned char *__msg,
				 int, char *, int) __THROW;
const unsigned char * p_fqname (const unsigned char *,
				const unsigned char *, FILE *) __THROW;
const char *	p_option (unsigned long __option) __THROW;
int		dn_count_labels (const char *) __THROW;
int		dn_comp (const char *, unsigned char *, int, unsigned char **,
			 unsigned char **) __THROW;
int		dn_expand (const unsigned char *, const unsigned char *,
			   const unsigned char *, char *, int) __THROW;
unsigned int	res_randomid (void) __THROW;
int		res_nameinquery (const char *, int, int,
				 const unsigned char *,
				 const unsigned char *) __THROW;
int		res_queriesmatch (const unsigned char *,
				  const unsigned char *,
				  const unsigned char *,
				  const unsigned char *) __THROW;
/* Things involving a resolver context. */
int		res_ninit (res_state) __THROW;
void		fp_resstat (const res_state, FILE *) __THROW;
const char *	res_hostalias (const res_state, const char *, char *, size_t)
     __THROW;
int		res_nquery (res_state, const char *, int, int,
			    unsigned char *, int) __THROW;
int		res_nsearch (res_state, const char *, int, int,
			     unsigned char *, int) __THROW;
int		res_nquerydomain (res_state, const char *, const char *, int,
				  int, unsigned char *, int) __THROW;
int		res_nmkquery (res_state, int, const char *, int, int,
			      const unsigned char *, int,
			      const unsigned char *, unsigned char *, int)
     __THROW;
int		res_nsend (res_state, const unsigned char *, int,
			   unsigned char *, int) __THROW;
void		res_nclose (res_state) __THROW;

__END_DECLS

#endif /* !_RESOLV_H_ */
