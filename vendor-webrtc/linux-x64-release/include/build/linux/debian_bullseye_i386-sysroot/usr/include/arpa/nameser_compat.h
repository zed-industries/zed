/* Copyright (c) 1983, 1989
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

#ifndef _ARPA_NAMESER_COMPAT_
#define	_ARPA_NAMESER_COMPAT_

#include <endian.h>

/*%
 * Structure for query header.  The order of the fields is machine- and
 * compiler-dependent, depending on the byte/bit order and the layout
 * of bit fields.  We use bit fields only in int variables, as this
 * is all ANSI requires.  This requires a somewhat confusing rearrangement.
 */

typedef struct {
	unsigned	id :16;		/*%< query identification number */
#if __BYTE_ORDER == __BIG_ENDIAN
			/* fields in third byte */
	unsigned	qr: 1;		/*%< response flag */
	unsigned	opcode: 4;	/*%< purpose of message */
	unsigned	aa: 1;		/*%< authoritive answer */
	unsigned	tc: 1;		/*%< truncated message */
	unsigned	rd: 1;		/*%< recursion desired */
			/* fields in fourth byte */
	unsigned	ra: 1;		/*%< recursion available */
	unsigned	unused :1;	/*%< unused bits (MBZ as of 4.9.3a3) */
	unsigned	ad: 1;		/*%< authentic data from named */
	unsigned	cd: 1;		/*%< checking disabled by resolver */
	unsigned	rcode :4;	/*%< response code */
#endif
#if __BYTE_ORDER == __LITTLE_ENDIAN || __BYTE_ORDER == __PDP_ENDIAN
			/* fields in third byte */
	unsigned	rd :1;		/*%< recursion desired */
	unsigned	tc :1;		/*%< truncated message */
	unsigned	aa :1;		/*%< authoritive answer */
	unsigned	opcode :4;	/*%< purpose of message */
	unsigned	qr :1;		/*%< response flag */
			/* fields in fourth byte */
	unsigned	rcode :4;	/*%< response code */
	unsigned	cd: 1;		/*%< checking disabled by resolver */
	unsigned	ad: 1;		/*%< authentic data from named */
	unsigned	unused :1;	/*%< unused bits (MBZ as of 4.9.3a3) */
	unsigned	ra :1;		/*%< recursion available */
#endif
			/* remaining bytes */
	unsigned	qdcount :16;	/*%< number of question entries */
	unsigned	ancount :16;	/*%< number of answer entries */
	unsigned	nscount :16;	/*%< number of authority entries */
	unsigned	arcount :16;	/*%< number of resource entries */
} HEADER;

#define PACKETSZ	NS_PACKETSZ
#define MAXDNAME	NS_MAXDNAME
#define MAXCDNAME	NS_MAXCDNAME
#define MAXLABEL	NS_MAXLABEL
#define	HFIXEDSZ	NS_HFIXEDSZ
#define QFIXEDSZ	NS_QFIXEDSZ
#define RRFIXEDSZ	NS_RRFIXEDSZ
#define	INT32SZ		NS_INT32SZ
#define	INT16SZ		NS_INT16SZ
#define INT8SZ		NS_INT8SZ
#define	INADDRSZ	NS_INADDRSZ
#define	IN6ADDRSZ	NS_IN6ADDRSZ
#define	INDIR_MASK	NS_CMPRSFLGS
#define NAMESERVER_PORT	NS_DEFAULTPORT

#define S_ZONE		ns_s_zn
#define S_PREREQ	ns_s_pr
#define S_UPDATE	ns_s_ud
#define S_ADDT		ns_s_ar

#define QUERY		ns_o_query
#define IQUERY		ns_o_iquery
#define STATUS		ns_o_status
#define	NS_NOTIFY_OP	ns_o_notify
#define	NS_UPDATE_OP	ns_o_update

#define NOERROR		ns_r_noerror
#define FORMERR		ns_r_formerr
#define SERVFAIL	ns_r_servfail
#define NXDOMAIN	ns_r_nxdomain
#define NOTIMP		ns_r_notimpl
#define REFUSED		ns_r_refused
#define YXDOMAIN	ns_r_yxdomain
#define YXRRSET		ns_r_yxrrset
#define NXRRSET		ns_r_nxrrset
#define NOTAUTH		ns_r_notauth
#define NOTZONE		ns_r_notzone
/*#define BADSIG		ns_r_badsig*/
/*#define BADKEY		ns_r_badkey*/
/*#define BADTIME		ns_r_badtime*/


#define DELETE		ns_uop_delete
#define ADD		ns_uop_add

#define T_A ns_t_a
#define T_NS ns_t_ns
#define T_MD ns_t_md
#define T_MF ns_t_mf
#define T_CNAME ns_t_cname
#define T_SOA ns_t_soa
#define T_MB ns_t_mb
#define T_MG ns_t_mg
#define T_MR ns_t_mr
#define T_NULL ns_t_null
#define T_WKS ns_t_wks
#define T_PTR ns_t_ptr
#define T_HINFO ns_t_hinfo
#define T_MINFO ns_t_minfo
#define T_MX ns_t_mx
#define T_TXT ns_t_txt
#define T_RP ns_t_rp
#define T_AFSDB ns_t_afsdb
#define T_X25 ns_t_x25
#define T_ISDN ns_t_isdn
#define T_RT ns_t_rt
#define T_NSAP ns_t_nsap
#define T_NSAP_PTR ns_t_nsap_ptr
#define T_SIG ns_t_sig
#define T_KEY ns_t_key
#define T_PX ns_t_px
#define T_GPOS ns_t_gpos
#define T_AAAA ns_t_aaaa
#define T_LOC ns_t_loc
#define T_NXT ns_t_nxt
#define T_EID ns_t_eid
#define T_NIMLOC ns_t_nimloc
#define T_SRV ns_t_srv
#define T_ATMA ns_t_atma
#define T_NAPTR ns_t_naptr
#define T_KX ns_t_kx
#define T_CERT ns_t_cert
#define T_A6 ns_t_a6
#define T_DNAME ns_t_dname
#define T_SINK ns_t_sink
#define T_OPT ns_t_opt
#define T_APL ns_t_apl
#define T_DS ns_t_ds
#define T_SSHFP ns_t_sshfp
#define T_IPSECKEY ns_t_ipseckey
#define T_RRSIG ns_t_rrsig
#define T_NSEC ns_t_nsec
#define T_DNSKEY ns_t_dnskey
#define T_DHCID ns_t_dhcid
#define T_NSEC3 ns_t_nsec3
#define T_NSEC3PARAM ns_t_nsec3param
#define T_TLSA ns_t_tlsa
#define T_SMIMEA ns_t_smimea
#define T_HIP ns_t_hip
#define T_NINFO ns_t_ninfo
#define T_RKEY ns_t_rkey
#define T_TALINK ns_t_talink
#define T_CDS ns_t_cds
#define T_CDNSKEY ns_t_cdnskey
#define T_OPENPGPKEY ns_t_openpgpkey
#define T_CSYNC ns_t_csync
#define T_SPF ns_t_spf
#define T_UINFO ns_t_uinfo
#define T_UID ns_t_uid
#define T_GID ns_t_gid
#define T_UNSPEC ns_t_unspec
#define T_NID ns_t_nid
#define T_L32 ns_t_l32
#define T_L64 ns_t_l64
#define T_LP ns_t_lp
#define T_EUI48 ns_t_eui48
#define T_EUI64 ns_t_eui64
#define T_TKEY ns_t_tkey
#define T_TSIG ns_t_tsig
#define T_IXFR ns_t_ixfr
#define T_AXFR ns_t_axfr
#define T_MAILB ns_t_mailb
#define T_MAILA ns_t_maila
#define T_ANY ns_t_any
#define T_URI ns_t_uri
#define T_CAA ns_t_caa
#define T_AVC ns_t_avc
#define T_TA ns_t_ta
#define T_DLV ns_t_dlv

#define C_IN		ns_c_in
#define C_CHAOS		ns_c_chaos
#define C_HS		ns_c_hs
/* BIND_UPDATE */
#define C_NONE		ns_c_none
#define C_ANY		ns_c_any

#define	GETSHORT		NS_GET16
#define	GETLONG			NS_GET32
#define	PUTSHORT		NS_PUT16
#define	PUTLONG			NS_PUT32

#endif /* _ARPA_NAMESER_COMPAT_ */
/*! \file */
