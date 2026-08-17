/*
 * Copyright (c) 1983, 1989, 1993
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
 * Copyright (c) 2004 by Internet Systems Consortium, Inc. ("ISC")
 * Copyright (c) 1996-1999 by Internet Software Consortium.
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

#ifndef _ARPA_NAMESER_H_
#define _ARPA_NAMESER_H_

#include <sys/param.h>
#include <sys/types.h>
#include <stdint.h>

/*
 * Define constants based on RFC 883, RFC 1034, RFC 1035
 */
#define NS_PACKETSZ	512	/*%< default UDP packet size */
#define NS_MAXDNAME	1025	/*%< maximum domain name */
#define NS_MAXMSG	65535	/*%< maximum message size */
#define NS_MAXCDNAME	255	/*%< maximum compressed domain name */
#define NS_MAXLABEL	63	/*%< maximum length of domain label */
#define NS_HFIXEDSZ	12	/*%< #/bytes of fixed data in header */
#define NS_QFIXEDSZ	4	/*%< #/bytes of fixed data in query */
#define NS_RRFIXEDSZ	10	/*%< #/bytes of fixed data in r record */
#define NS_INT32SZ	4	/*%< #/bytes of data in a uint32_t */
#define NS_INT16SZ	2	/*%< #/bytes of data in a uint16_t */
#define NS_INT8SZ	1	/*%< #/bytes of data in a uint8_t */
#define NS_INADDRSZ	4	/*%< IPv4 T_A */
#define NS_IN6ADDRSZ	16	/*%< IPv6 T_AAAA */
#define NS_CMPRSFLGS	0xc0	/*%< Flag bits indicating name compression. */
#define NS_DEFAULTPORT	53	/*%< For both TCP and UDP. */
/*
 * These can be expanded with synonyms, just keep ns_parse.c:ns_parserecord()
 * in synch with it.
 */
typedef enum __ns_sect {
	ns_s_qd = 0,		/*%< Query: Question. */
	ns_s_zn = 0,		/*%< Update: Zone. */
	ns_s_an = 1,		/*%< Query: Answer. */
	ns_s_pr = 1,		/*%< Update: Prerequisites. */
	ns_s_ns = 2,		/*%< Query: Name servers. */
	ns_s_ud = 2,		/*%< Update: Update. */
	ns_s_ar = 3,		/*%< Query|Update: Additional records. */
	ns_s_max = 4
} ns_sect;

/*%
 * This is a message handle.  It is caller allocated and has no dynamic data.
 * This structure is intended to be opaque to all but ns_parse.c, thus the
 * leading _'s on the member names.  Use the accessor functions, not the _'s.
 */
typedef struct __ns_msg {
	const unsigned char	*_msg, *_eom;
	uint16_t		_id, _flags, _counts[ns_s_max];
	const unsigned char	*_sections[ns_s_max];
	ns_sect			_sect;
	int			_rrnum;
	const unsigned char	*_msg_ptr;
} ns_msg;

/* Private data structure - do not use from outside library. */
struct _ns_flagdata {  int mask, shift;  };
extern const struct _ns_flagdata _ns_flagdata[];

/* Accessor macros - this is part of the public interface. */

#define ns_msg_id(handle) ((handle)._id + 0)
#define ns_msg_base(handle) ((handle)._msg + 0)
#define ns_msg_end(handle) ((handle)._eom + 0)
#define ns_msg_size(handle) ((handle)._eom - (handle)._msg)
#define ns_msg_count(handle, section) ((handle)._counts[section] + 0)

/*%
 * This is a parsed record.  It is caller allocated and has no dynamic data.
 */
typedef	struct __ns_rr {
	char			name[NS_MAXDNAME];
	uint16_t		type;
	uint16_t		rr_class;
	uint32_t		ttl;
	uint16_t		rdlength;
	const unsigned char *	rdata;
} ns_rr;

/* Accessor macros - this is part of the public interface. */
#define ns_rr_name(rr)	(((rr).name[0] != '\0') ? (rr).name : ".")
#define ns_rr_type(rr)	((ns_type)((rr).type + 0))
#define ns_rr_class(rr)	((ns_class)((rr).rr_class + 0))
#define ns_rr_ttl(rr)	((rr).ttl + 0)
#define ns_rr_rdlen(rr)	((rr).rdlength + 0)
#define ns_rr_rdata(rr)	((rr).rdata + 0)

/*%
 * These don't have to be in the same order as in the packet flags word,
 * and they can even overlap in some cases, but they will need to be kept
 * in synch with ns_parse.c:ns_flagdata[].
 */
typedef enum __ns_flag {
	ns_f_qr,		/*%< Question/Response. */
	ns_f_opcode,		/*%< Operation code. */
	ns_f_aa,		/*%< Authoritative Answer. */
	ns_f_tc,		/*%< Truncation occurred. */
	ns_f_rd,		/*%< Recursion Desired. */
	ns_f_ra,		/*%< Recursion Available. */
	ns_f_z,			/*%< MBZ. */
	ns_f_ad,		/*%< Authentic Data (DNSSEC). */
	ns_f_cd,		/*%< Checking Disabled (DNSSEC). */
	ns_f_rcode,		/*%< Response code. */
	ns_f_max
} ns_flag;

/*%
 * Currently defined opcodes.
 */
typedef enum __ns_opcode {
	ns_o_query = 0,		/*%< Standard query. */
	ns_o_iquery = 1,	/*%< Inverse query (deprecated/unsupported). */
	ns_o_status = 2,	/*%< Name server status query (unsupported). */
				/* Opcode 3 is undefined/reserved. */
	ns_o_notify = 4,	/*%< Zone change notification. */
	ns_o_update = 5,	/*%< Zone update message. */
	ns_o_max = 6
} ns_opcode;

/*%
 * Currently defined response codes.
 */
typedef	enum __ns_rcode {
	ns_r_noerror = 0,	/*%< No error occurred. */
	ns_r_formerr = 1,	/*%< Format error. */
	ns_r_servfail = 2,	/*%< Server failure. */
	ns_r_nxdomain = 3,	/*%< Name error. */
	ns_r_notimpl = 4,	/*%< Unimplemented. */
	ns_r_refused = 5,	/*%< Operation refused. */
	/* these are for BIND_UPDATE */
	ns_r_yxdomain = 6,	/*%< Name exists */
	ns_r_yxrrset = 7,	/*%< RRset exists */
	ns_r_nxrrset = 8,	/*%< RRset does not exist */
	ns_r_notauth = 9,	/*%< Not authoritative for zone */
	ns_r_notzone = 10,	/*%< Zone of record different from zone section */
	ns_r_max = 11,
	/* The following are EDNS extended rcodes */
	ns_r_badvers = 16,
	/* The following are TSIG errors */
	ns_r_badsig = 16,
	ns_r_badkey = 17,
	ns_r_badtime = 18
} ns_rcode;

/* BIND_UPDATE */
typedef enum __ns_update_operation {
	ns_uop_delete = 0,
	ns_uop_add = 1,
	ns_uop_max = 2
} ns_update_operation;

/*%
 * This structure is used for TSIG authenticated messages
 */
struct ns_tsig_key {
        char name[NS_MAXDNAME], alg[NS_MAXDNAME];
        unsigned char *data;
        int len;
};
typedef struct ns_tsig_key ns_tsig_key;

/*%
 * This structure is used for TSIG authenticated TCP messages
 */
struct ns_tcp_tsig_state {
	int counter;
	struct dst_key *key;
	void *ctx;
	unsigned char sig[NS_PACKETSZ];
	int siglen;
};
typedef struct ns_tcp_tsig_state ns_tcp_tsig_state;

#define NS_TSIG_FUDGE 300
#define NS_TSIG_TCP_COUNT 100
#define NS_TSIG_ALG_HMAC_MD5 "HMAC-MD5.SIG-ALG.REG.INT"

#define NS_TSIG_ERROR_NO_TSIG -10
#define NS_TSIG_ERROR_NO_SPACE -11
#define NS_TSIG_ERROR_FORMERR -12

/*%
 * Currently defined type values for resources and queries.
 */
typedef enum __ns_type
  {
    ns_t_invalid = 0,

    ns_t_a = 1,
    ns_t_ns = 2,
    ns_t_md = 3,
    ns_t_mf = 4,
    ns_t_cname = 5,
    ns_t_soa = 6,
    ns_t_mb = 7,
    ns_t_mg = 8,
    ns_t_mr = 9,
    ns_t_null = 10,
    ns_t_wks = 11,
    ns_t_ptr = 12,
    ns_t_hinfo = 13,
    ns_t_minfo = 14,
    ns_t_mx = 15,
    ns_t_txt = 16,
    ns_t_rp = 17,
    ns_t_afsdb = 18,
    ns_t_x25 = 19,
    ns_t_isdn = 20,
    ns_t_rt = 21,
    ns_t_nsap = 22,
    ns_t_nsap_ptr = 23,
    ns_t_sig = 24,
    ns_t_key = 25,
    ns_t_px = 26,
    ns_t_gpos = 27,
    ns_t_aaaa = 28,
    ns_t_loc = 29,
    ns_t_nxt = 30,
    ns_t_eid = 31,
    ns_t_nimloc = 32,
    ns_t_srv = 33,
    ns_t_atma = 34,
    ns_t_naptr = 35,
    ns_t_kx = 36,
    ns_t_cert = 37,
    ns_t_a6 = 38,
    ns_t_dname = 39,
    ns_t_sink = 40,
    ns_t_opt = 41,
    ns_t_apl = 42,
    ns_t_ds = 43,
    ns_t_sshfp = 44,
    ns_t_ipseckey = 45,
    ns_t_rrsig = 46,
    ns_t_nsec = 47,
    ns_t_dnskey = 48,
    ns_t_dhcid = 49,
    ns_t_nsec3 = 50,
    ns_t_nsec3param = 51,
    ns_t_tlsa = 52,
    ns_t_smimea = 53,
    ns_t_hip = 55,
    ns_t_ninfo = 56,
    ns_t_rkey = 57,
    ns_t_talink = 58,
    ns_t_cds = 59,
    ns_t_cdnskey = 60,
    ns_t_openpgpkey = 61,
    ns_t_csync = 62,
    ns_t_spf = 99,
    ns_t_uinfo = 100,
    ns_t_uid = 101,
    ns_t_gid = 102,
    ns_t_unspec = 103,
    ns_t_nid = 104,
    ns_t_l32 = 105,
    ns_t_l64 = 106,
    ns_t_lp = 107,
    ns_t_eui48 = 108,
    ns_t_eui64 = 109,
    ns_t_tkey = 249,
    ns_t_tsig = 250,
    ns_t_ixfr = 251,
    ns_t_axfr = 252,
    ns_t_mailb = 253,
    ns_t_maila = 254,
    ns_t_any = 255,
    ns_t_uri = 256,
    ns_t_caa = 257,
    ns_t_avc = 258,
    ns_t_ta = 32768,
    ns_t_dlv = 32769,

    ns_t_max = 65536
  } ns_type;

/*%
 * Values for class field
 */
typedef enum __ns_class {
	ns_c_invalid = 0,	/*%< Cookie. */
	ns_c_in = 1,		/*%< Internet. */
	ns_c_2 = 2,		/*%< unallocated/unsupported. */
	ns_c_chaos = 3,		/*%< MIT Chaos-net. */
	ns_c_hs = 4,		/*%< MIT Hesiod. */
	/* Query class values which do not appear in resource records */
	ns_c_none = 254,	/*%< for prereq. sections in update requests */
	ns_c_any = 255,		/*%< Wildcard match. */
	ns_c_max = 65536
} ns_class;

/* Certificate type values in CERT resource records.  */
typedef enum __ns_cert_types {
	cert_t_pkix = 1,	/*%< PKIX (X.509v3) */
	cert_t_spki = 2,	/*%< SPKI */
	cert_t_pgp  = 3,	/*%< PGP */
	cert_t_url  = 253,	/*%< URL private type */
	cert_t_oid  = 254	/*%< OID private type */
} ns_cert_types;

/*%
 * EDNS0 extended flags and option codes, host order.
 */
#define NS_OPT_DNSSEC_OK        0x8000U
#define NS_OPT_NSID		3

/*%
 * Inline versions of get/put short/long.  Pointer is advanced.
 */
#define NS_GET16(s, cp) do { \
	const unsigned char *t_cp = (const unsigned char *)(cp); \
	(s) = ((uint16_t)t_cp[0] << 8) \
	    | ((uint16_t)t_cp[1]) \
	    ; \
	(cp) += NS_INT16SZ; \
} while (0)

#define NS_GET32(l, cp) do { \
	const unsigned char *t_cp = (const unsigned char *)(cp); \
	(l) = ((uint32_t)t_cp[0] << 24) \
	    | ((uint32_t)t_cp[1] << 16) \
	    | ((uint32_t)t_cp[2] << 8) \
	    | ((uint32_t)t_cp[3]) \
	    ; \
	(cp) += NS_INT32SZ; \
} while (0)

#define NS_PUT16(s, cp) do { \
	uint16_t t_s = (uint16_t)(s); \
	unsigned char *t_cp = (unsigned char *)(cp); \
	*t_cp++ = t_s >> 8; \
	*t_cp   = t_s; \
	(cp) += NS_INT16SZ; \
} while (0)

#define NS_PUT32(l, cp) do { \
	uint32_t t_l = (uint32_t)(l); \
	unsigned char *t_cp = (unsigned char *)(cp); \
	*t_cp++ = t_l >> 24; \
	*t_cp++ = t_l >> 16; \
	*t_cp++ = t_l >> 8; \
	*t_cp   = t_l; \
	(cp) += NS_INT32SZ; \
} while (0)

__BEGIN_DECLS
int		ns_msg_getflag (ns_msg, int) __THROW;
unsigned int	ns_get16 (const unsigned char *) __THROW;
unsigned long	ns_get32 (const unsigned char *) __THROW;
void		ns_put16 (unsigned int, unsigned char *) __THROW;
void		ns_put32 (unsigned long, unsigned char *) __THROW;
int		ns_initparse (const unsigned char *, int, ns_msg *) __THROW;
int		ns_skiprr (const unsigned char *, const unsigned char *,
			   ns_sect, int) __THROW;
int		ns_parserr (ns_msg *, ns_sect, int, ns_rr *) __THROW;
int		ns_sprintrr (const ns_msg *, const ns_rr *,
			     const char *, const char *, char *, size_t)
     __THROW;
int		ns_sprintrrf (const unsigned char *, size_t, const char *,
			      ns_class, ns_type, unsigned long,
			      const unsigned char *, size_t, const char *,
			      const char *, char *, size_t) __THROW;
int		ns_format_ttl (unsigned long, char *, size_t) __THROW;
int		ns_parse_ttl (const char *, unsigned long *) __THROW;
uint32_t	ns_datetosecs (const char *, int *) __THROW;
int		ns_name_ntol (const unsigned char *, unsigned char *, size_t)
     __THROW;
int		ns_name_ntop (const unsigned char *, char *, size_t) __THROW;
int		ns_name_pton (const char *, unsigned char *, size_t) __THROW;
int		ns_name_unpack (const unsigned char *, const unsigned char *,
				const unsigned char *, unsigned char *, size_t)
     __THROW;
int		ns_name_pack (const unsigned char *, unsigned char *, int,
			      const unsigned char **, const unsigned char **)
     __THROW;
int		ns_name_uncompress (const unsigned char *,
				    const unsigned char *,
				    const unsigned char *,
				    char *, size_t) __THROW;
int		ns_name_compress (const char *, unsigned char *, size_t,
				  const unsigned char **,
				  const unsigned char **) __THROW;
int		ns_name_skip (const unsigned char **, const unsigned char *)
     __THROW;
void		ns_name_rollback (const unsigned char *,
				  const unsigned char **,
				  const unsigned char **) __THROW;
int		ns_samedomain (const char *, const char *) __THROW;
int		ns_subdomain (const char *, const char *) __THROW;
int		ns_makecanon (const char *, char *, size_t) __THROW;
int		ns_samename (const char *, const char *) __THROW;
__END_DECLS

#include <arpa/nameser_compat.h>

#endif /* !_ARPA_NAMESER_H_ */
/*! \file */
