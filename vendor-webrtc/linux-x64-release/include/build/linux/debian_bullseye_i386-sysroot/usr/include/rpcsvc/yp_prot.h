/*
 * This file contains symbols and structures defining the rpc protocol
 * between the NIS clients and the NIS servers.  The servers
 * are the NIS database servers, and the NIS binders.
 */

#ifndef _RPCSVC_YP_PROT_H
#define _RPCSVC_YP_PROT_H

#include <rpc/rpc.h>
#include <rpcsvc/ypclnt.h>

#ifdef __cplusplus
extern "C" {
#endif


/*
 * The following procedures are supported by the protocol:
 *
 * YPPROC_NULL() returns () takes nothing, returns nothing.  This indicates
 * that the NIS server is alive.
 *
 * YPPROC_DOMAIN (char *) returns (bool_t) TRUE.  Indicates that the
 * responding NIS server does serve the named domain; FALSE indicates no
 * support.
 *
 * YPPROC_DOMAIN_NONACK (char *) returns (TRUE) if the NIS server does serve
 * the named domain, otherwise does not return.  Used in the broadcast case.
 *
 * YPPROC_MATCH (struct ypreq_key) returns (struct ypresp_val).  Returns the
 * right-hand value for a passed left-hand key, within a named map and
 * domain.
 *
 * YPPROC_FIRST (struct ypreq_nokey) returns (struct ypresp_key_val).
 * Returns the first key-value pair from a named domain and map.
 *
 * YPPROC_NEXT (struct ypreq_key) returns (struct ypresp_key_val).  Returns
 * the key-value pair following a passed key-value pair within a named
 * domain and map.
 *
 * YPPROC_XFR (struct ypreq_xfr) returns nothing.  Indicates to a server that
 * a map should be updated.
 *
 * YPPROC_NEWXFR (struct ypreq_newxfr) returns nothing.  Indicates to a server
 * that a map should be updated. Uses protocol independent request struct.
 *
 * YPPROC_CLEAR	takes nothing, returns nothing.  Instructs a NIS server to
 * close the current map, so that old versions of the disk file don't get
 * held open.
 *
 * YPPROC_ALL (struct ypreq_nokey), returns
 * 	union switch (bool_t more) {
 *		TRUE:	(struct ypresp_key_val);
 *		FALSE:	(struct) {};
 *	}
 *
 * YPPROC_MASTER (struct ypreq_nokey), returns (ypresp_master)
 *
 * YPPROC_ORDER (struct ypreq_nokey), returns (ypresp_order)
 *
 * YPPROC_MAPLIST (char *), returns (struct ypmaplist *)
 */

/* Program and version symbols, magic numbers */

#define YPPROG	        100004
#define YPVERS		2
#define YPVERS_ORIG	1
#define YPMAXRECORD	1024
#define YPMAXDOMAIN	256
#define YPMAXMAP	64
#define YPMAXPEER	256

/* byte size of a large NIS packet */
#define YPMSGSZ		1600

typedef struct keydat {
  u_int keydat_len;
  char *keydat_val;
} keydat_t;

typedef struct valdat {
  u_int valdat_len;
  char *valdat_val;
} valdat_t;

struct ypmap_parms {
  char *domain;			/* Null string means not available */
  char *map;			/* Null string means not available */
  unsigned int ordernum;	/* 0 means not available */
  char *owner;			/* Null string means not available */
};
typedef struct ypmap_parms ypmap_parms;

/*
 * Request parameter structures
 */

struct ypreq_key {
  char *domain;
  char *map;
  keydat_t keydat;
};
typedef struct ypreq_key ypreq_key;

struct ypreq_nokey {
  char *domain;
  char *map;
};
typedef struct ypreq_nokey ypreq_nokey;

struct ypreq_xfr {
  struct ypmap_parms map_parms;
  u_int transid;
  u_int proto;
  u_int port;
};
typedef struct ypreq_xfr ypreq_xfr;

struct ypreq_newxfr {
  struct ypmap_parms map_parms;
  u_int transid;
  u_int proto;
  char *name;
};
typedef struct ypreq_newxfr ypreq_newxfr;

#define ypxfr_domain map_parms.domain
#define ypxfr_map map_parms.map
#define ypxfr_ordernum map_parms.ordernum
#define ypxfr_owner map_parms.owner

/* Return status values */

enum ypstat {
  YP_TRUE = 1,		/* General purpose success code */
#define YP_TRUE YP_TRUE
  YP_NOMORE = 2,	/* No more entries in map */
#define YP_NOMORE YP_NOMORE
  YP_FALSE = 0,		/* General purpose failure code */
#define YP_FALSE YP_FALSE
  YP_NOMAP = -1,	/* No such map in domain */
#define YP_NOMAP YP_NOMAP
  YP_NODOM = -2,	/* Domain not supported */
#define YP_NODOM YP_NODOM
  YP_NOKEY = -3,	/* No such key in map */
#define YP_NOKEY YP_NOKEY
  YP_BADOP = -4,	/* Invalid operation */
#define YP_BADOP YP_BADOP
  YP_BADDB = -5,	/* Server data base is bad */
#define YP_BADDB YP_BADDB
  YP_YPERR = -6,	/* NIS server error */
#define YP_YPERR YP_YPERR
  YP_BADARGS = -7,	/* Request arguments bad */
#define YP_BADARGS YP_BADARGS
  YP_VERS = -8		/* NIS server version mismatch - server can't supply
			   requested service. */
#define YP_VERS YP_VERS
};
typedef enum ypstat ypstat;


enum ypxfrstat {
  YPXFR_SUCC = 1,
  YPXFR_AGE = 2,
  YPXFR_NOMAP = -1,
  YPXFR_NODOM = -2,
  YPXFR_RSRC = -3,
  YPXFR_RPC = -4,
  YPXFR_MADDR = -5,
  YPXFR_YPERR = -6,
  YPXFR_BADARGS = -7,
  YPXFR_DBM = -8,
  YPXFR_FILE = -9,
  YPXFR_SKEW = -10,
  YPXFR_CLEAR = -11,
  YPXFR_FORCE = -12,
  YPXFR_XFRERR = -13,
  YPXFR_REFUSED = -14
};
typedef enum ypxfrstat ypxfrstat;

/*
 * Response parameter structures
 */

struct ypresp_val {
  ypstat status;
  valdat_t valdat;
};
typedef struct ypresp_val ypresp_val;

struct ypresp_key_val {
  ypstat status;
  valdat_t valdat;
  keydat_t keydat;
};
typedef struct ypresp_key_val ypresp_key_val;

struct ypresp_master {
  ypstat status;
  char *master;
};
typedef struct ypresp_master ypresp_master;

struct ypresp_order {
  ypstat status;
  unsigned int ordernum;
};
typedef struct ypresp_order ypresp_order;

struct ypresp_xfr {
        u_int transid;
        ypxfrstat xfrstat;
};
typedef struct ypresp_xfr ypresp_xfr;

struct ypmaplist {
  char *map;
#define ypml_name map
  struct ypmaplist *next;
#define ypml_next next
};
typedef struct ypmaplist ypmaplist;

struct ypresp_maplist {
  ypstat status;
  struct ypmaplist *list;
};
typedef struct ypresp_maplist ypresp_maplist;


/*
 * Procedure symbols.  YPPROC_NULL, YPPROC_DOMAIN, and YPPROC_DOMAIN_NONACK
 * must keep the same values (0, 1, and 2) that they had in the first version
 * of the protocol.
 */

#define YPPROC_NULL	0
#define YPPROC_DOMAIN	1
#define YPPROC_DOMAIN_NONACK 2
#define YPPROC_MATCH	3
#define YPPROC_FIRST	4
#define YPPROC_NEXT	5
#define YPPROC_XFR	6
#define YPPROC_CLEAR	7
#define YPPROC_ALL	8
#define YPPROC_MASTER	9
#define YPPROC_ORDER	10
#define YPPROC_MAPLIST	11
#define YPPROC_NEWXFR	12

/*
 *		Protocol between clients and NIS binder servers
 */

/*
 * The following procedures are supported by the protocol:
 *
 * YPBINDPROC_NULL() returns ()
 * 	takes nothing, returns nothing
 *
 * YPBINDPROC_DOMAIN takes (char *) returns (struct ypbind2_resp)
 *
 * YPBINDPROC_SETDOM takes (struct ypbind2_setdom) returns nothing
 */

/* Program and version symbols, magic numbers */

#define YPBINDPROG		100007
#define YPBINDVERS		3
#define YPBINDVERS_2		2
#define YPBINDVERS_1		1

/* Procedure symbols */

#define YPBINDPROC_NULL		0
#define YPBINDPROC_DOMAIN	1
#define YPBINDPROC_SETDOM	2

/*
 * Request and response structures and overall result status codes.
 * Success and failure represent two separate response message types.
 */

enum ypbind_resptype {YPBIND_SUCC_VAL = 1, YPBIND_FAIL_VAL = 2};
typedef enum ypbind_resptype ypbind_resptype;

struct ypbind2_binding {
  struct in_addr ypbind_binding_addr;	        /* In network order */
  unsigned short int ypbind_binding_port;	/* In network order */
};
typedef struct ypbind2_binding ypbind2_binding;

struct ypbind2_resp {
  enum ypbind_resptype ypbind_status;
  union {
    u_int ypbind_error;
    struct ypbind2_binding ypbind_bindinfo;
  } ypbind_respbody;
};
typedef struct ypbind2_resp ypbind2_resp;
#define ypbind2_error ypbind_respbody.ypbind_error
#define ypbind2_bindinfo ypbind_respbody.ypbind_bindinfo
#define ypbind2_addr ypbind_respbody.ypbind_bindinfo.ypbind_binding_addr
#define ypbind2_port ypbind_respbody.ypbind_bindinfo.ypbind_binding_port

struct ypbind_oldsetdom {
        char ypoldsetdom_domain[YPMAXDOMAIN];
        ypbind2_binding ypoldsetdom_binding;
};
typedef struct ypbind_oldsetdom ypbind_oldsetdom;
#define ypoldsetdom_addr ypoldsetdom_binding.ypbind_binding_addr
#define ypoldsetdom_port ypoldsetdom_binding.ypbind_binding_port

struct ypbind2_setdom {
  char *ypsetdom_domain;
  struct ypbind2_binding ypsetdom_binding;
  u_int ypsetdom_vers;
};
typedef struct ypbind2_setdom ypbind2_setdom;
#define ypsetdom_addr ypsetdom_binding.ypbind_binding_addr
#define ypsetdom_port ypsetdom_binding.ypbind_binding_port

struct ypbind3_binding {
  struct netconfig *ypbind_nconf;
  struct netbuf *ypbind_svcaddr;
  char *ypbind_servername;
  /* that's the highest version number that the used
     ypserv supports, normally YPVERS */
  rpcvers_t ypbind_hi_vers;
  /* the lowest version number that the used
     ypserv supports, on Solaris 0 or YPVERS, too */
  rpcvers_t ypbind_lo_vers;
};
typedef struct ypbind3_binding ypbind3_binding;

struct ypbind3_resp {
  enum ypbind_resptype ypbind_status;
  union {
    u_long ypbind_error;
    struct ypbind3_binding *ypbind_bindinfo;
  } ypbind_respbody;
};
typedef struct ypbind3_resp ypbind3_resp;
#define ypbind3_error ypbind_respbody.ypbind_error
#define ypbind3_bindinfo ypbind_respbody.ypbind_bindinfo
#define ypbind3_nconf ypbind_respbody.ypbind_bindinfo->ypbind_nconf
#define ypbind3_svcaddr ypbind_respbody.ypbind_bindinfo->ypbind_svcaddr
#define ypbind3_servername ypbind_respbody.ypbind_bindinfo->ypbind_servername
#define ypbind3_hi_vers ypbind_respbody.ypbind_bindinfo->ypbind_hi_vers
#define ypbind3_lo_vers ypbind_respbody.ypbind_bindinfo->ypbind_lo_vers

struct ypbind3_setdom {
  char *ypsetdom_domain;
  struct ypbind3_binding *ypsetdom_bindinfo;
};
typedef struct ypbind3_setdom ypbind3_setdom;
#define ypsetdom3_nconf ypsetdom_bindinfo->ypbind_nconf
#define ypsetdom3_svcaddr ypsetdom_bindinfo->ypbind_svcaddr
#define ypsetdom3_servername ypsetdom_bindinfo->ypbind_servername
#define ypsetdom3_hi_vers ypsetdom_bindinfo->ypbind_hi_vers
#define ypsetdom3_lo_vers ypsetdom_bindinfo->ypbind_lo_vers


/* Detailed failure reason codes for response field ypbind_error*/

#define YPBIND_ERR_ERR 1		/* Internal error */
#define YPBIND_ERR_NOSERV 2		/* No bound server for passed domain */
#define YPBIND_ERR_RESC 3		/* System resource allocation failure */
#define YPBIND_ERR_NODOMAIN 4	/* Domain doesn't exist */

/*
 *		Protocol between clients (ypxfr, only) and yppush
 *		yppush speaks a protocol in the transient range, which
 *		is supplied to ypxfr as a command-line parameter when it
 *		is activated by ypserv.
 */
#define YPPUSHVERS		1
#define YPPUSHVERS_ORIG		1

/* Procedure symbols */

#define YPPUSHPROC_NULL		0
#define YPPUSHPROC_XFRRESP	1

/* Status values for yppushresp_xfr.status */

enum yppush_status {
  YPPUSH_SUCC = 1,		/* Success */
#define YPPUSH_SUCC	YPPUSH_SUCC
  YPPUSH_AGE = 2,		/* Master's version not newer */
#define YPPUSH_AGE	YPPUSH_AGE
  YPPUSH_NOMAP = -1,		/* Can't find server for map */
#define YPPUSH_NOMAP 	YPPUSH_NOMAP
  YPPUSH_NODOM = -2,		/* Domain not supported */
#define YPPUSH_NODOM 	YPPUSH_NODOM
  YPPUSH_RSRC = -3,		/* Local resouce alloc failure */
#define YPPUSH_RSRC 	YPPUSH_RSRC
  YPPUSH_RPC = -4,		/* RPC failure talking to server */
#define YPPUSH_RPC 	YPPUSH_RPC
  YPPUSH_MADDR = -5,		/* Can't get master address */
#define YPPUSH_MADDR	YPPUSH_MADDR
  YPPUSH_YPERR = -6,		/* NIS server/map db error */
#define YPPUSH_YPERR 	YPPUSH_YPERR
  YPPUSH_BADARGS = -7,		/* Request arguments bad */
#define YPPUSH_BADARGS 	YPPUSH_BADARGS
  YPPUSH_DBM = -8,		/* Local dbm operation failed */
#define YPPUSH_DBM	YPPUSH_DBM
  YPPUSH_FILE = -9,		/* Local file I/O operation failed */
#define YPPUSH_FILE	YPPUSH_FILE
  YPPUSH_SKEW = -10,		/* Map version skew during transfer */
#define YPPUSH_SKEW	YPPUSH_SKEW
  YPPUSH_CLEAR = -11,		/* Can't send "Clear" req to local ypserv */
#define YPPUSH_CLEAR	YPPUSH_CLEAR
  YPPUSH_FORCE = -12,		/* No local order number in map - use -f flag*/
#define YPPUSH_FORCE	YPPUSH_FORCE
  YPPUSH_XFRERR = -13,		/* ypxfr error */
#define YPPUSH_XFRERR	YPPUSH_XFRERR
  YPPUSH_REFUSED = -14,		/* Transfer request refused by ypserv */
#define YPPUSH_REFUSED	YPPUSH_REFUSED
  YPPUSH_NOALIAS = -15		/* Alias not found for map or domain */
#define	YPPUSH_NOALIAS	YPPUSH_NOALIAS
};
typedef enum yppush_status yppush_status;

struct yppushresp_xfr {
  u_int transid;
  yppush_status status;
};
typedef struct yppushresp_xfr yppushresp_xfr;

struct ypresp_all {
  bool_t more;
  union {
    struct ypresp_key_val val;
  } ypresp_all_u;
};
typedef struct ypresp_all ypresp_all;

extern bool_t xdr_domainname (XDR *__xdrs, char ** __objp);
extern bool_t xdr_keydat (XDR *__xdrs, keydat_t *__objp);
extern bool_t xdr_valdat (XDR *__xdrs, valdat_t *__objp);
extern bool_t xdr_ypall (XDR *__xdrs, struct ypall_callback * __objp);
extern bool_t xdr_ypbind2_binding (XDR *__xdrs, struct ypbind2_binding * __objp);
extern bool_t xdr_ypbind2_resp (XDR *__xdrs, struct ypbind2_resp * __objp);
extern bool_t xdr_ypbind2_setdom (XDR *__xdrs, struct ypbind2_setdom * __objp);
extern bool_t xdr_ypbind3_binding (XDR *__xdrs, struct ypbind3_binding * __objp);
extern bool_t xdr_ypbind3_resp (XDR *__xdrs, struct ypbind3_resp * __objp);
extern bool_t xdr_ypbind3_setdom (XDR *__xdrs, struct ypbind3_setdom * __objp);
extern bool_t xdr_ypbind_oldsetdom (XDR *__xdrs, struct ypbind_oldsetdom * __objp);
extern bool_t xdr_ypbind_resptype (XDR *__xdrs, enum ypbind_resptype * __objp);
extern bool_t xdr_ypmap_parms (XDR *__xdrs, struct ypmap_parms * __objp);
extern bool_t xdr_ypmaplist (XDR *__xdrs, struct ypmaplist *__objp);
extern bool_t xdr_yppushresp_xfr (XDR *__xdrs, struct yppushresp_xfr * __objp);
extern bool_t xdr_ypreq_key (XDR *__xdrs, struct ypreq_key * __objp);
extern bool_t xdr_ypreq_newxfr (XDR *__xdrs, struct ypreq_newxfr * __objp);
extern bool_t xdr_ypreq_nokey (XDR *__xdrs, struct ypreq_nokey * __objp);
extern bool_t xdr_ypreq_xfr (XDR *__xdrs, struct ypreq_xfr * __objp);
extern bool_t xdr_ypresp_all (XDR *__xdrs, struct ypresp_all  * __objp);
extern bool_t xdr_ypresp_key_val (XDR *__xdrs, struct ypresp_key_val * __objp);
extern bool_t xdr_ypresp_maplist (XDR *__xdrs, struct ypresp_maplist * __objp);
extern bool_t xdr_ypresp_master (XDR *__xdrs, struct ypresp_master * __objp);
extern bool_t xdr_ypresp_order (XDR *__xdrs, struct ypresp_order  * __objp);
extern bool_t xdr_ypresp_val (XDR *__xdrs, struct ypresp_val * __objp);
extern bool_t xdr_ypresp_xfr (XDR *__xdrs, struct ypresp_xfr *__objp);
extern bool_t xdr_ypstat (XDR *__xdrs, enum ypstat * __objp);
extern bool_t xdr_ypxfrstat (XDR *__xdrs, enum ypxfrstat *__objp);

/* Not really for this, but missing better place: */
extern const char *taddr2host (const struct netconfig *__nconf,
			       const struct netbuf *__nbuf,
	                       char *__host, size_t __hostlen);
extern const char *taddr2ipstr (const struct netconfig *__nconf,
                                const struct netbuf *__nbuf,
                                char *__buf, size_t __buflen);
extern unsigned short taddr2port (const struct netconfig *__nconf,
                                  const struct netbuf *__nbuf);

#ifdef __cplusplus
}
#endif


#endif	/* _RPCSVC_YP_PROT_H */
