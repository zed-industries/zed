/*
 * Copyright (c) 2010, Oracle America, Inc.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 *       notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 *       copyright notice, this list of conditions and the following
 *       disclaimer in the documentation and/or other materials
 *       provided with the distribution.
 *     * Neither the name of the "Oracle America, Inc." nor the names of its
 *       contributors may be used to endorse or promote products derived
 *       from this software without specific prior written permission.
 *
 *   THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 *   "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 *   LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
 *   FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
 *   COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT,
 *   INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 *   DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE
 *   GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 *   INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
 *   WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
 *   NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 *   OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef _RPCSVC_NIS_H
#define _RPCSVC_NIS_H 1

#include <rpc/rpc.h>
#include <rpcsvc/nis_tags.h>

#ifdef  __cplusplus
extern "C" {
#endif

/*
 *	nis.h
 *
 *	This file is the main include file for NIS clients. It contains
 *	both the client library function defines and the various data
 *	structures used by the NIS service. It includes the file nis_tags.h
 *	which defines the tag values. This allows the tags to change without
 *	having to change the nis.x file.
 *
 *	NOTE : THIS FILE IS NOT GENERATED WITH RPCGEN ! SO YOU HAVE TO
 *             ADD ALL THE CHANGES ON nis_*.x FILES HERE AGAIN !
 *
 *      I have removed all the Solaris internal structs and variables,
 *      because they are not supported, Sun changed them between various
 *      releases and they shouldn't be used in user programs.
 *                                              <kukuk@suse.de>
 */


#ifndef __nis_object_h
#define __nis_object_h

#define NIS_MAXSTRINGLEN 255
#define NIS_MAXNAMELEN 1024
#define NIS_MAXATTRNAME 32
#define NIS_MAXATTRVAL 2048
#define NIS_MAXCOLUMNS 64
#define NIS_MAXATTR 16
#define NIS_MAXPATH 1024
#define NIS_MAXREPLICAS 128
#define NIS_MAXLINKS 16
#define NIS_PK_NONE 0
#define NIS_PK_DH 1
#define NIS_PK_RSA 2
#define NIS_PK_KERB 3
#define NIS_PK_DHEXT 4

struct nis_attr {
	char *zattr_ndx;
	struct {
		u_int zattr_val_len;
		char *zattr_val_val;
	} zattr_val;
};
typedef struct nis_attr nis_attr;

typedef char *nis_name;

enum zotypes {
	BOGUS_OBJ = 0,
	NO_OBJ = 1,
	DIRECTORY_OBJ = 2,
	GROUP_OBJ = 3,
	TABLE_OBJ = 4,
	ENTRY_OBJ = 5,
	LINK_OBJ = 6,
	PRIVATE_OBJ = 7,
	NIS_BOGUS_OBJ = 0,
	NIS_NO_OBJ = 1,
	NIS_DIRECTORY_OBJ = 2,
	NIS_GROUP_OBJ = 3,
	NIS_TABLE_OBJ = 4,
	NIS_ENTRY_OBJ = 5,
	NIS_LINK_OBJ = 6,
	NIS_PRIVATE_OBJ = 7
};
typedef enum zotypes zotypes;

enum nstype {
	UNKNOWN = 0,
	NIS = 1,
	SUNYP = 2,
	IVY = 3,
	DNS = 4,
	X500 = 5,
	DNANS = 6,
	XCHS = 7,
	CDS = 8,
};
typedef enum nstype nstype;

struct oar_mask {
	uint32_t oa_rights;
	zotypes oa_otype;
};
typedef struct oar_mask oar_mask;

struct endpoint {
	char *uaddr;
	char *family;
	char *proto;
};
typedef struct endpoint endpoint;

struct nis_server {
	nis_name name;
	struct {
		u_int ep_len;
		endpoint *ep_val;
	} ep;
	uint32_t key_type;
	netobj pkey;
};
typedef struct nis_server nis_server;

struct directory_obj {
	nis_name do_name;
	nstype do_type;
	struct {
		u_int do_servers_len;
		nis_server *do_servers_val;
	} do_servers;
	uint32_t do_ttl;
	struct {
		u_int do_armask_len;
		oar_mask *do_armask_val;
	} do_armask;
};
typedef struct directory_obj directory_obj;

#define EN_BINARY 1
#define EN_CRYPT 2
#define EN_XDR 4
#define EN_MODIFIED 8
#define EN_ASN1 64

struct entry_col {
	uint32_t ec_flags;
	struct {
		u_int ec_value_len;
		char *ec_value_val;
	} ec_value;
};
typedef struct entry_col entry_col;

struct entry_obj {
	char *en_type;
	struct {
		u_int en_cols_len;
		entry_col *en_cols_val;
	} en_cols;
};
typedef struct entry_obj entry_obj;

struct group_obj {
	uint32_t gr_flags;
	struct {
		u_int gr_members_len;
		nis_name *gr_members_val;
	} gr_members;
};
typedef struct group_obj group_obj;

struct link_obj {
	zotypes li_rtype;
	struct {
		u_int li_attrs_len;
		nis_attr *li_attrs_val;
	} li_attrs;
	nis_name li_name;
};
typedef struct link_obj link_obj;

#define TA_BINARY 1
#define TA_CRYPT 2
#define TA_XDR 4
#define TA_SEARCHABLE 8
#define TA_CASE 16
#define TA_MODIFIED 32
#define TA_ASN1 64

struct table_col {
	char *tc_name;
	uint32_t tc_flags;
	uint32_t tc_rights;
};
typedef struct table_col table_col;

struct table_obj {
	char *ta_type;
	int ta_maxcol;
	u_char ta_sep;
	struct {
		u_int ta_cols_len;
		table_col *ta_cols_val;
	} ta_cols;
	char *ta_path;
};
typedef struct table_obj table_obj;

struct objdata {
	zotypes zo_type;
	union {
		struct directory_obj di_data;
		struct group_obj gr_data;
		struct table_obj ta_data;
		struct entry_obj en_data;
		struct link_obj li_data;
		struct {
			u_int po_data_len;
			char *po_data_val;
		} po_data;
	} objdata_u;
};
typedef struct objdata objdata;

struct nis_oid {
	uint32_t ctime;
	uint32_t mtime;
};
typedef struct nis_oid nis_oid;

struct nis_object {
	nis_oid zo_oid;
	nis_name zo_name;
	nis_name zo_owner;
	nis_name zo_group;
	nis_name zo_domain;
	uint32_t zo_access;
	uint32_t zo_ttl;
	objdata zo_data;
};
typedef struct nis_object nis_object;

#endif /* if __nis_object_h */

enum nis_error {
	NIS_SUCCESS = 0,
	NIS_S_SUCCESS = 1,
	NIS_NOTFOUND = 2,
	NIS_S_NOTFOUND = 3,
	NIS_CACHEEXPIRED = 4,
	NIS_NAMEUNREACHABLE = 5,
	NIS_UNKNOWNOBJ = 6,
	NIS_TRYAGAIN = 7,
	NIS_SYSTEMERROR = 8,
	NIS_CHAINBROKEN = 9,
	NIS_PERMISSION = 10,
	NIS_NOTOWNER = 11,
	NIS_NOT_ME = 12,
	NIS_NOMEMORY = 13,
	NIS_NAMEEXISTS = 14,
	NIS_NOTMASTER = 15,
	NIS_INVALIDOBJ = 16,
	NIS_BADNAME = 17,
	NIS_NOCALLBACK = 18,
	NIS_CBRESULTS = 19,
	NIS_NOSUCHNAME = 20,
	NIS_NOTUNIQUE = 21,
	NIS_IBMODERROR = 22,
	NIS_NOSUCHTABLE = 23,
	NIS_TYPEMISMATCH = 24,
	NIS_LINKNAMEERROR = 25,
	NIS_PARTIAL = 26,
	NIS_TOOMANYATTRS = 27,
	NIS_RPCERROR = 28,
	NIS_BADATTRIBUTE = 29,
	NIS_NOTSEARCHABLE = 30,
	NIS_CBERROR = 31,
	NIS_FOREIGNNS = 32,
	NIS_BADOBJECT = 33,
	NIS_NOTSAMEOBJ = 34,
	NIS_MODFAIL = 35,
	NIS_BADREQUEST = 36,
	NIS_NOTEMPTY = 37,
	NIS_COLDSTART_ERR = 38,
	NIS_RESYNC = 39,
	NIS_FAIL = 40,
	NIS_UNAVAIL = 41,
	NIS_RES2BIG = 42,
	NIS_SRVAUTH = 43,
	NIS_CLNTAUTH = 44,
	NIS_NOFILESPACE = 45,
	NIS_NOPROC = 46,
	NIS_DUMPLATER = 47,
};
typedef enum nis_error nis_error;

struct nis_result {
	nis_error status;
	struct {
		u_int objects_len;
		nis_object *objects_val;
	} objects;
	netobj cookie;
	uint32_t zticks;
	uint32_t dticks;
	uint32_t aticks;
	uint32_t cticks;
};
typedef struct nis_result nis_result;

struct ns_request {
	nis_name ns_name;
	struct {
		u_int ns_object_len;
		nis_object *ns_object_val;
	} ns_object;
};
typedef struct ns_request ns_request;

struct ib_request {
	nis_name ibr_name;
	struct {
		u_int ibr_srch_len;
		nis_attr *ibr_srch_val;
	} ibr_srch;
	uint32_t ibr_flags;
	struct {
		u_int ibr_obj_len;
		nis_object *ibr_obj_val;
	} ibr_obj;
	struct {
		u_int ibr_cbhost_len;
		nis_server *ibr_cbhost_val;
	} ibr_cbhost;
	u_int ibr_bufsize;
	netobj ibr_cookie;
};
typedef struct ib_request ib_request;

struct ping_args {
	nis_name dir;
	uint32_t stamp;
};
typedef struct ping_args ping_args;

enum log_entry_t {
	LOG_NOP = 0,
	ADD_NAME = 1,
	REM_NAME = 2,
	MOD_NAME_OLD = 3,
	MOD_NAME_NEW = 4,
	ADD_IBASE = 5,
	REM_IBASE = 6,
	MOD_IBASE = 7,
	UPD_STAMP = 8,
};
typedef enum log_entry_t log_entry_t;

struct log_entry {
	uint32_t le_time;
	log_entry_t le_type;
	nis_name le_princp;
	nis_name le_name;
	struct {
		u_int le_attrs_len;
		nis_attr *le_attrs_val;
	} le_attrs;
	nis_object le_object;
};
typedef struct log_entry log_entry;

struct log_result {
	nis_error lr_status;
	netobj lr_cookie;
	struct {
		u_int lr_entries_len;
		log_entry *lr_entries_val;
	} lr_entries;
};
typedef struct log_result log_result;

struct cp_result {
	nis_error cp_status;
	uint32_t cp_zticks;
	uint32_t cp_dticks;
};
typedef struct cp_result cp_result;

struct nis_tag {
	uint32_t tag_type;
	char *tag_val;
};
typedef struct nis_tag nis_tag;

struct nis_taglist {
	struct {
		u_int tags_len;
		nis_tag *tags_val;
	} tags;
};
typedef struct nis_taglist nis_taglist;

struct dump_args {
	nis_name da_dir;
	uint32_t da_time;
	struct {
		u_int da_cbhost_len;
		nis_server *da_cbhost_val;
	} da_cbhost;
};
typedef struct dump_args dump_args;

struct fd_args {
	nis_name dir_name;
	nis_name requester;
};
typedef struct fd_args fd_args;

struct fd_result {
	nis_error status;
	nis_name source;
	struct {
		u_int dir_data_len;
		char *dir_data_val;
	} dir_data;
	struct {
		u_int signature_len;
		char *signature_val;
	} signature;
};
typedef struct fd_result fd_result;

/* Generic client creating flags */
#define ZMH_VC		1
#define ZMH_DG		2
#define ZMH_AUTH	4

/* Testing Access rights for objects */

#define NIS_READ_ACC		1
#define NIS_MODIFY_ACC		2
#define NIS_CREATE_ACC		4
#define NIS_DESTROY_ACC	8
/* Test macros. a == access rights, m == desired rights. */
#define NIS_WORLD(a, m)        (((a) & (m)) != 0)
#define NIS_GROUP(a, m)        (((a) & ((m) << 8)) != 0)
#define NIS_OWNER(a, m)        (((a) & ((m) << 16)) != 0)
#define NIS_NOBODY(a, m)       (((a) & ((m) << 24)) != 0)
/*
 * EOL Alert - The following non-prefixed test macros are
 * here for backward compatibility, and will be not be present
 * in future releases - use the NIS_*() macros above.
 */
#define WORLD(a, m)	(((a) & (m)) != 0)
#define GROUP(a, m)	(((a) & ((m) << 8)) != 0)
#define OWNER(a, m)	(((a) & ((m) << 16)) != 0)
#define NOBODY(a, m)	(((a) & ((m) << 24)) != 0)

#define OATYPE(d, n) (((d)->do_armask.do_armask_val+n)->oa_otype)
#define OARIGHTS(d, n) (((d)->do_armask.do_armask_val+n)->oa_rights)
#define WORLD_DEFAULT (NIS_READ_ACC)
#define GROUP_DEFAULT (NIS_READ_ACC << 8)
#define OWNER_DEFAULT ((NIS_READ_ACC + NIS_MODIFY_ACC + NIS_CREATE_ACC +\
			NIS_DESTROY_ACC) << 16)
#define DEFAULT_RIGHTS (WORLD_DEFAULT | GROUP_DEFAULT | OWNER_DEFAULT)

/* Result manipulation defines ... */
#define NIS_RES_NUMOBJ(x)	((x)->objects.objects_len)
#define NIS_RES_OBJECT(x)	((x)->objects.objects_val)
#define NIS_RES_COOKIE(x)	((x)->cookie)
#define NIS_RES_STATUS(x)	((x)->status)

/* These defines make getting at the variant part of the object easier. */
#define TA_data zo_data.objdata_u.ta_data
#define EN_data zo_data.objdata_u.en_data
#define DI_data zo_data.objdata_u.di_data
#define LI_data zo_data.objdata_u.li_data
#define GR_data zo_data.objdata_u.gr_data

#define __type_of(o) ((o)->zo_data.zo_type)

/* Declarations for the internal subroutines in nislib.c */
enum name_pos {SAME_NAME, HIGHER_NAME, LOWER_NAME, NOT_SEQUENTIAL, BAD_NAME};
typedef enum name_pos name_pos;

/*
 * Defines for getting at column data in entry objects. Because RPCGEN
 * generates some rather wordy structures, we create some defines that
 * collapse the needed keystrokes to access a particular value using
 * these definitions they take an nis_object *, and an int and return
 * a u_char * for Value, and an int for length.
 */
#define ENTRY_VAL(obj, col) (obj)->EN_data.en_cols.en_cols_val[col].ec_value.ec_value_val
#define ENTRY_LEN(obj, col) (obj)->EN_data.en_cols.en_cols_val[col].ec_value.ec_value_len


/* Prototypes, and extern declarations for the NIS library functions. */
#include <rpcsvc/nislib.h>
#endif

/*
 * nis_3.h
 *
 * This file contains definitions that are only of interest to the actual
 * service daemon and client stubs. Normal users of NIS will not include
 * this file.
 *
 * NOTE : This include file is automatically created by a combination
 * of rpcgen and sed. DO NOT EDIT IT, change the nis.x file instead
 * and then remake this file.
 */
#ifndef __nis_3_h
#define __nis_3_h

#define NIS_PROG 100300
#define NIS_VERSION 3

#define NIS_LOOKUP 1
extern  nis_result * nis_lookup_3 (ns_request *, CLIENT *);
extern  nis_result * nis_lookup_3_svc (ns_request *, struct svc_req *);
#define NIS_ADD 2
extern  nis_result * nis_add_3 (ns_request *, CLIENT *);
extern  nis_result * nis_add_3_svc (ns_request *, struct svc_req *);
#define NIS_MODIFY 3
extern  nis_result * nis_modify_3 (ns_request *, CLIENT *);
extern  nis_result * nis_modify_3_svc (ns_request *, struct svc_req *);
#define NIS_REMOVE 4
extern  nis_result * nis_remove_3 (ns_request *, CLIENT *);
extern  nis_result * nis_remove_3_svc (ns_request *, struct svc_req *);
#define NIS_IBLIST 5
extern  nis_result * nis_iblist_3 (ib_request *, CLIENT *);
extern  nis_result * nis_iblist_3_svc (ib_request *, struct svc_req *);
#define NIS_IBADD 6
extern  nis_result * nis_ibadd_3 (ib_request *, CLIENT *);
extern  nis_result * nis_ibadd_3_svc (ib_request *, struct svc_req *);
#define NIS_IBMODIFY 7
extern  nis_result * nis_ibmodify_3 (ib_request *, CLIENT *);
extern  nis_result * nis_ibmodify_3_svc (ib_request *, struct svc_req *)
    ;
#define NIS_IBREMOVE 8
extern  nis_result * nis_ibremove_3 (ib_request *, CLIENT *);
extern  nis_result * nis_ibremove_3_svc (ib_request *, struct svc_req *)
    ;
#define NIS_IBFIRST 9
extern  nis_result * nis_ibfirst_3 (ib_request *, CLIENT *);
extern  nis_result * nis_ibfirst_3_svc (ib_request *, struct svc_req *)
    ;
#define NIS_IBNEXT 10
extern  nis_result * nis_ibnext_3 (ib_request *, CLIENT *);
extern  nis_result * nis_ibnext_3_svc (ib_request *, struct svc_req *);
#define NIS_FINDDIRECTORY 12
extern  fd_result * nis_finddirectory_3 (fd_args *, CLIENT *);
extern  fd_result * nis_finddirectory_3_svc (fd_args *,
					     struct svc_req *);
#define NIS_STATUS 14
extern  nis_taglist * nis_status_3 (nis_taglist *, CLIENT *);
extern  nis_taglist * nis_status_3_svc (nis_taglist *, struct svc_req *)
    ;
#define NIS_DUMPLOG 15
extern  log_result * nis_dumplog_3 (dump_args *, CLIENT *);
extern  log_result * nis_dumplog_3_svc (dump_args *, struct svc_req *);
#define NIS_DUMP 16
extern  log_result * nis_dump_3 (dump_args *, CLIENT *);
extern  log_result * nis_dump_3_svc (dump_args *, struct svc_req *);
#define NIS_CALLBACK 17
extern  bool_t * nis_callback_3 (netobj *, CLIENT *);
extern  bool_t * nis_callback_3_svc (netobj *, struct svc_req *);
#define NIS_CPTIME 18
extern  uint32_t * nis_cptime_3 (nis_name *, CLIENT *);
extern  uint32_t * nis_cptime_3_svc (nis_name *, struct svc_req *);
#define NIS_CHECKPOINT 19
extern  cp_result * nis_checkpoint_3 (nis_name *, CLIENT *);
extern  cp_result * nis_checkpoint_3_svc (nis_name *, struct svc_req *)
    ;
#define NIS_PING 20
extern  void * nis_ping_3 (ping_args *, CLIENT *);
extern  void * nis_ping_3_svc (ping_args *, struct svc_req *);
#define NIS_SERVSTATE 21
extern  nis_taglist * nis_servstate_3 (nis_taglist *, CLIENT *);
extern  nis_taglist * nis_servstate_3_svc (nis_taglist *,
					   struct svc_req *);
#define NIS_MKDIR 22
extern  nis_error * nis_mkdir_3 (nis_name *, CLIENT *);
extern  nis_error * nis_mkdir_3_svc (nis_name *, struct svc_req *);
#define NIS_RMDIR 23
extern  nis_error * nis_rmdir_3 (nis_name *, CLIENT *);
extern  nis_error * nis_rmdir_3_svc (nis_name *, struct svc_req *);
#define NIS_UPDKEYS 24
extern  nis_error * nis_updkeys_3 (nis_name *, CLIENT *);
extern  nis_error * nis_updkeys_3_svc (nis_name *, struct svc_req *);

#ifdef  __cplusplus
}
#endif

#endif /* ! _RPCSVC_NIS_H */
