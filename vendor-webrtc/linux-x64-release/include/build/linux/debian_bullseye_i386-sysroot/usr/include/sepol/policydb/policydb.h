/* Author : Stephen Smalley, <sds@tycho.nsa.gov> */

/*
 * Updated: Joshua Brindle <jbrindle@tresys.com>
 *	    Karl MacMillan <kmacmillan@tresys.com>
 *	    Jason Tang <jtang@tresys.com>
 *	    
 *	Module support
 *
 * Updated: Trusted Computer Solutions, Inc. <dgoeddel@trustedcs.com>
 *
 *	Support for enhanced MLS infrastructure.
 *
 * Updated: Frank Mayer <mayerf@tresys.com> and Karl MacMillan <kmacmillan@tresys.com>
 *
 * 	Added conditional policy language extensions
 *
 * Updated: Red Hat, Inc.  James Morris <jmorris@redhat.com>
 *
 *      Fine-grained netlink support
 *      IPv6 support
 *      Code cleanup
 *
 * Copyright (C) 2004-2005 Trusted Computer Solutions, Inc.
 * Copyright (C) 2003 - 2004 Tresys Technology, LLC
 * Copyright (C) 2003 - 2004 Red Hat, Inc.
 * Copyright (C) 2017 Mellanox Techonolgies Inc.
 *
 *  This library is free software; you can redistribute it and/or
 *  modify it under the terms of the GNU Lesser General Public
 *  License as published by the Free Software Foundation; either
 *  version 2.1 of the License, or (at your option) any later version.
 *
 *  This library is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 *  Lesser General Public License for more details.
 *
 *  You should have received a copy of the GNU Lesser General Public
 *  License along with this library; if not, write to the Free Software
 *  Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
 */

/* FLASK */

/*
 * A policy database (policydb) specifies the 
 * configuration data for the security policy.
 */

#ifndef _SEPOL_POLICYDB_POLICYDB_H_
#define _SEPOL_POLICYDB_POLICYDB_H_

#include <stdio.h>
#include <stddef.h>

#include <sepol/policydb.h>

#include <sepol/policydb/flask_types.h>
#include <sepol/policydb/symtab.h>
#include <sepol/policydb/avtab.h>
#include <sepol/policydb/context.h>
#include <sepol/policydb/constraint.h>
#include <sepol/policydb/sidtab.h>

#define ERRMSG_LEN 1024

#define POLICYDB_SUCCESS      0
#define POLICYDB_ERROR       -1
#define POLICYDB_UNSUPPORTED -2

#ifdef __cplusplus
extern "C" {
#endif

#define IB_DEVICE_NAME_MAX 64

/*
 * A datum type is defined for each kind of symbol 
 * in the configuration data:  individual permissions, 
 * common prefixes for access vectors, classes,
 * users, roles, types, sensitivities, categories, etc.
 */

/* type set preserves data needed by modules such as *, ~ and attributes */
typedef struct type_set {
	ebitmap_t types;
	ebitmap_t negset;
#define TYPE_STAR 1
#define TYPE_COMP 2
	uint32_t flags;
} type_set_t;

typedef struct role_set {
	ebitmap_t roles;
#define ROLE_STAR 1
#define ROLE_COMP 2
	uint32_t flags;
} role_set_t;

/* Permission attributes */
typedef struct perm_datum {
	symtab_datum_t s;
} perm_datum_t;

/* Attributes of a common prefix for access vectors */
typedef struct common_datum {
	symtab_datum_t s;
	symtab_t permissions;	/* common permissions */
} common_datum_t;

/* Class attributes */
typedef struct class_datum {
	symtab_datum_t s;
	char *comkey;		/* common name */
	common_datum_t *comdatum;	/* common datum */
	symtab_t permissions;	/* class-specific permission symbol table */
	constraint_node_t *constraints;	/* constraints on class permissions */
	constraint_node_t *validatetrans;	/* special transition rules */
/* Options how a new object user and role should be decided */
#define DEFAULT_SOURCE		1
#define DEFAULT_TARGET		2
	char default_user;
	char default_role;
	char default_type;
/* Options how a new object range should be decided */
#define DEFAULT_SOURCE_LOW	1
#define DEFAULT_SOURCE_HIGH	2
#define DEFAULT_SOURCE_LOW_HIGH	3
#define DEFAULT_TARGET_LOW	4
#define DEFAULT_TARGET_HIGH	5
#define DEFAULT_TARGET_LOW_HIGH	6
#define DEFAULT_GLBLUB 		7
	char default_range;
} class_datum_t;

/* Role attributes */
typedef struct role_datum {
	symtab_datum_t s;
	ebitmap_t dominates;	/* set of roles dominated by this role */
	type_set_t types;	/* set of authorized types for role */
	ebitmap_t cache;	/* This is an expanded set used for context validation during parsing */
	uint32_t bounds;	/* bounds role, if exist */
#define ROLE_ROLE 0		/* regular role in kernel policies */
#define ROLE_ATTRIB 1		/* attribute */
	uint32_t flavor;
	ebitmap_t roles;	/* roles with this attribute */
} role_datum_t;

typedef struct role_trans {
	uint32_t role;		/* current role */
	uint32_t type;		/* program executable type, or new object type */
	uint32_t tclass;	/* process class, or new object class */
	uint32_t new_role;	/* new role */
	struct role_trans *next;
} role_trans_t;

typedef struct role_allow {
	uint32_t role;		/* current role */
	uint32_t new_role;	/* new role */
	struct role_allow *next;
} role_allow_t;

/* filename_trans rules */
typedef struct filename_trans {
	uint32_t stype;
	uint32_t ttype;
	uint32_t tclass;
	char *name;
} filename_trans_t;

typedef struct filename_trans_datum {
	uint32_t otype;		/* expected of new object */
} filename_trans_datum_t;

/* Type attributes */
typedef struct type_datum {
	symtab_datum_t s;
	uint32_t primary;	/* primary name? can be set to primary value if below is TYPE_ */
#define TYPE_TYPE 0		/* regular type or alias in kernel policies */
#define TYPE_ATTRIB 1		/* attribute */
#define TYPE_ALIAS 2		/* alias in modular policy */
	uint32_t flavor;
	ebitmap_t types;	/* types with this attribute */
#define TYPE_FLAGS_PERMISSIVE		(1 << 0)
#define TYPE_FLAGS_EXPAND_ATTR_TRUE	(1 << 1)
#define TYPE_FLAGS_EXPAND_ATTR_FALSE	(1 << 2)
#define TYPE_FLAGS_EXPAND_ATTR (TYPE_FLAGS_EXPAND_ATTR_TRUE | \
				TYPE_FLAGS_EXPAND_ATTR_FALSE)
	uint32_t flags;
	uint32_t bounds;	/* bounds type, if exist */
} type_datum_t;

/*
 * Properties of type_datum
 * available on the policy version >= (MOD_)POLICYDB_VERSION_BOUNDARY
 */
#define TYPEDATUM_PROPERTY_PRIMARY	0x0001
#define TYPEDATUM_PROPERTY_ATTRIBUTE	0x0002
#define TYPEDATUM_PROPERTY_ALIAS	0x0004	/* userspace only */
#define TYPEDATUM_PROPERTY_PERMISSIVE	0x0008	/* userspace only */

/* User attributes */
typedef struct user_datum {
	symtab_datum_t s;
	role_set_t roles;	/* set of authorized roles for user */
	mls_semantic_range_t range;	/* MLS range (min. - max.) for user */
	mls_semantic_level_t dfltlevel;	/* default login MLS level for user */
	ebitmap_t cache;	/* This is an expanded set used for context validation during parsing */
	mls_range_t exp_range;     /* expanded range used for validation */
	mls_level_t exp_dfltlevel; /* expanded range used for validation */
	uint32_t bounds;	/* bounds user, if exist */
} user_datum_t;

/* Sensitivity attributes */
typedef struct level_datum {
	mls_level_t *level;	/* sensitivity and associated categories */
	unsigned char isalias;	/* is this sensitivity an alias for another? */
	unsigned char defined;
} level_datum_t;

/* Category attributes */
typedef struct cat_datum {
	symtab_datum_t s;
	unsigned char isalias;	/* is this category an alias for another? */
} cat_datum_t;

typedef struct range_trans {
	uint32_t source_type;
	uint32_t target_type;
	uint32_t target_class;
} range_trans_t;

/* Boolean data type */
typedef struct cond_bool_datum {
	symtab_datum_t s;
	int state;
#define COND_BOOL_FLAGS_TUNABLE	0x01	/* is this a tunable? */
	uint32_t flags;
} cond_bool_datum_t;

struct cond_node;

typedef struct cond_node cond_list_t;
struct cond_av_list;

typedef struct class_perm_node {
	uint32_t tclass;
	uint32_t data;		/* permissions or new type */
	struct class_perm_node *next;
} class_perm_node_t;

#define xperm_test(x, p) (1 & (p[x >> 5] >> (x & 0x1f)))
#define xperm_set(x, p) (p[x >> 5] |= (1 << (x & 0x1f)))
#define xperm_clear(x, p) (p[x >> 5] &= ~(1 << (x & 0x1f)))
#define EXTENDED_PERMS_LEN 8

typedef struct av_extended_perms {
#define AVRULE_XPERMS_IOCTLFUNCTION	0x01
#define AVRULE_XPERMS_IOCTLDRIVER	0x02
	uint8_t specified;
	uint8_t driver;
	/* 256 bits of permissions */
	uint32_t perms[EXTENDED_PERMS_LEN];
} av_extended_perms_t;

typedef struct avrule {
/* these typedefs are almost exactly the same as those in avtab.h - they are
 * here because of the need to include neverallow and dontaudit messages */
#define AVRULE_ALLOWED			AVTAB_ALLOWED
#define AVRULE_AUDITALLOW		AVTAB_AUDITALLOW
#define AVRULE_AUDITDENY		AVTAB_AUDITDENY
#define AVRULE_DONTAUDIT		0x0008
#define AVRULE_NEVERALLOW		AVTAB_NEVERALLOW
#define AVRULE_AV         (AVRULE_ALLOWED | AVRULE_AUDITALLOW | AVRULE_AUDITDENY | AVRULE_DONTAUDIT | AVRULE_NEVERALLOW)
#define AVRULE_TRANSITION		AVTAB_TRANSITION
#define AVRULE_MEMBER			AVTAB_MEMBER
#define AVRULE_CHANGE			AVTAB_CHANGE
#define AVRULE_TYPE       (AVRULE_TRANSITION | AVRULE_MEMBER | AVRULE_CHANGE)
#define AVRULE_XPERMS_ALLOWED 		AVTAB_XPERMS_ALLOWED
#define AVRULE_XPERMS_AUDITALLOW	AVTAB_XPERMS_AUDITALLOW
#define AVRULE_XPERMS_DONTAUDIT		AVTAB_XPERMS_DONTAUDIT
#define AVRULE_XPERMS_NEVERALLOW	AVTAB_XPERMS_NEVERALLOW
#define AVRULE_XPERMS	(AVRULE_XPERMS_ALLOWED | AVRULE_XPERMS_AUDITALLOW | \
				AVRULE_XPERMS_DONTAUDIT | AVRULE_XPERMS_NEVERALLOW)
	uint32_t specified;
#define RULE_SELF 1
	uint32_t flags;
	type_set_t stypes;
	type_set_t ttypes;
	class_perm_node_t *perms;
	av_extended_perms_t *xperms;
	unsigned long line;	/* line number from policy.conf where
				 * this rule originated  */
	/* source file name and line number (e.g. .te file) */
	char *source_filename;
	unsigned long source_line;
	struct avrule *next;
} avrule_t;

typedef struct role_trans_rule {
	role_set_t roles;	/* current role */
	type_set_t types;	/* program executable type, or new object type */
	ebitmap_t classes;	/* process class, or new object class */
	uint32_t new_role;	/* new role */
	struct role_trans_rule *next;
} role_trans_rule_t;

typedef struct role_allow_rule {
	role_set_t roles;	/* current role */
	role_set_t new_roles;	/* new roles */
	struct role_allow_rule *next;
} role_allow_rule_t;

typedef struct filename_trans_rule {
	type_set_t stypes;
	type_set_t ttypes;
	uint32_t tclass;
	char *name;
	uint32_t otype;	/* new type */
	struct filename_trans_rule *next;
} filename_trans_rule_t;

typedef struct range_trans_rule {
	type_set_t stypes;
	type_set_t ttypes;
	ebitmap_t tclasses;
	mls_semantic_range_t trange;
	struct range_trans_rule *next;
} range_trans_rule_t;

/*
 * The configuration data includes security contexts for 
 * initial SIDs, unlabeled file systems, TCP and UDP port numbers, 
 * network interfaces, and nodes.  This structure stores the
 * relevant data for one such entry.  Entries of the same kind
 * (e.g. all initial SIDs) are linked together into a list.
 */
typedef struct ocontext {
	union {
		char *name;	/* name of initial SID, fs, netif, fstype, path */
		struct {
			uint8_t protocol;
			uint16_t low_port;
			uint16_t high_port;
		} port;		/* TCP or UDP port information */
		struct {
			uint32_t addr; /* network order */
			uint32_t mask; /* network order */
		} node;		/* node information */
		struct {
			uint32_t addr[4]; /* network order */
			uint32_t mask[4]; /* network order */
		} node6;	/* IPv6 node information */
		uint32_t device;
		uint16_t pirq;
		struct {
			uint64_t low_iomem;
			uint64_t high_iomem;
		} iomem;
		struct {
			uint32_t low_ioport;
			uint32_t high_ioport;
		} ioport;
		struct {
			uint64_t subnet_prefix;
			uint16_t low_pkey;
			uint16_t high_pkey;
		} ibpkey;
		struct {
			char *dev_name;
			uint8_t port;
		} ibendport;
	} u;
	union {
		uint32_t sclass;	/* security class for genfs */
		uint32_t behavior;	/* labeling behavior for fs_use */
	} v;
	context_struct_t context[2];	/* security context(s) */
	sepol_security_id_t sid[2];	/* SID(s) */
	struct ocontext *next;
} ocontext_t;

typedef struct genfs {
	char *fstype;
	struct ocontext *head;
	struct genfs *next;
} genfs_t;

/* symbol table array indices */
#define SYM_COMMONS 0
#define SYM_CLASSES 1
#define SYM_ROLES   2
#define SYM_TYPES   3
#define SYM_USERS   4
#define SYM_BOOLS   5
#define SYM_LEVELS  6
#define SYM_CATS    7
#define SYM_NUM     8

/* object context array indices */
#define OCON_ISID  0	/* initial SIDs */
#define OCON_FS    1	/* unlabeled file systems */
#define OCON_PORT  2	/* TCP and UDP port numbers */
#define OCON_NETIF 3	/* network interfaces */
#define OCON_NODE  4	/* nodes */
#define OCON_FSUSE 5	/* fs_use */
#define OCON_NODE6 6	/* IPv6 nodes */
#define OCON_IBPKEY 7	/* Infiniband PKEY */
#define OCON_IBENDPORT 8	/* Infiniband End Port */

/* object context array indices for Xen */
#define OCON_XEN_ISID  	    0    /* initial SIDs */
#define OCON_XEN_PIRQ       1    /* physical irqs */
#define OCON_XEN_IOPORT     2    /* io ports */
#define OCON_XEN_IOMEM	    3    /* io memory */
#define OCON_XEN_PCIDEVICE  4    /* pci devices */
#define OCON_XEN_DEVICETREE 5    /* device tree node */

/* OCON_NUM needs to be the largest index in any platform's ocontext array */
#define OCON_NUM   9

/* section: module information */

/* scope_index_t holds all of the symbols that are in scope in a
 * particular situation.  The bitmaps are indices (and thus must
 * subtract one) into the global policydb->scope array. */
typedef struct scope_index {
	ebitmap_t scope[SYM_NUM];
#define p_classes_scope scope[SYM_CLASSES]
#define p_roles_scope scope[SYM_ROLES]
#define p_types_scope scope[SYM_TYPES]
#define p_users_scope scope[SYM_USERS]
#define p_bools_scope scope[SYM_BOOLS]
#define p_sens_scope scope[SYM_LEVELS]
#define p_cat_scope scope[SYM_CATS]

	/* this array maps from class->value to the permissions within
	 * scope.  if bit (perm->value - 1) is set in map
	 * class_perms_map[class->value - 1] then that permission is
	 * enabled for this class within this decl.  */
	ebitmap_t *class_perms_map;
	/* total number of classes in class_perms_map array */
	uint32_t class_perms_len;
} scope_index_t;

/* a list of declarations for a particular avrule_decl */

/* These two structs declare a block of policy that has TE and RBAC
 * statements and declarations.  The root block (the global policy)
 * can never have an ELSE branch. */
typedef struct avrule_decl {
	uint32_t decl_id;
	uint32_t enabled;	/* whether this block is enabled */

	cond_list_t *cond_list;
	avrule_t *avrules;
	role_trans_rule_t *role_tr_rules;
	role_allow_rule_t *role_allow_rules;
	range_trans_rule_t *range_tr_rules;
	scope_index_t required;	/* symbols needed to activate this block */
	scope_index_t declared;	/* symbols declared within this block */

	/* type transition rules with a 'name' component */
	filename_trans_rule_t *filename_trans_rules;

	/* for additive statements (type attribute, roles, and users) */
	symtab_t symtab[SYM_NUM];

	/* In a linked module this will contain the name of the module
	 * from which this avrule_decl originated. */
	char *module_name;

	struct avrule_decl *next;
} avrule_decl_t;

typedef struct avrule_block {
	avrule_decl_t *branch_list;
	avrule_decl_t *enabled;	/* pointer to which branch is enabled.  this is
				   used in linking and never written to disk */
#define AVRULE_OPTIONAL 1
	uint32_t flags;		/* any flags for this block, currently just optional */
	struct avrule_block *next;
} avrule_block_t;

/* Every identifier has its own scope datum.  The datum describes if
 * the item is to be included into the final policy during
 * expansion. */
typedef struct scope_datum {
/* Required for this decl */
#define SCOPE_REQ  1
/* Declared in this decl */
#define SCOPE_DECL 2
	uint32_t scope;
	uint32_t *decl_ids;
	uint32_t decl_ids_len;
	/* decl_ids is a list of avrule_decl's that declare/require
	 * this symbol.  If scope==SCOPE_DECL then this is a list of
	 * declarations.  If the symbol may only be declared once
	 * (types, bools) then decl_ids_len will be exactly 1.  For
	 * implicitly declared things (roles, users) then decl_ids_len
	 * will be at least 1. */
} scope_datum_t;

/* The policy database */
typedef struct policydb {
#define POLICY_KERN SEPOL_POLICY_KERN
#define POLICY_BASE SEPOL_POLICY_BASE
#define POLICY_MOD SEPOL_POLICY_MOD
	uint32_t policy_type;
	char *name;
	char *version;
	int  target_platform;

	/* Set when the policydb is modified such that writing is unsupported */
	int unsupported_format;

	/* Whether this policydb is mls, should always be set */
	int mls;

	/* symbol tables */
	symtab_t symtab[SYM_NUM];
#define p_commons symtab[SYM_COMMONS]
#define p_classes symtab[SYM_CLASSES]
#define p_roles symtab[SYM_ROLES]
#define p_types symtab[SYM_TYPES]
#define p_users symtab[SYM_USERS]
#define p_bools symtab[SYM_BOOLS]
#define p_levels symtab[SYM_LEVELS]
#define p_cats symtab[SYM_CATS]

	/* symbol names indexed by (value - 1) */
	char **sym_val_to_name[SYM_NUM];
#define p_common_val_to_name sym_val_to_name[SYM_COMMONS]
#define p_class_val_to_name sym_val_to_name[SYM_CLASSES]
#define p_role_val_to_name sym_val_to_name[SYM_ROLES]
#define p_type_val_to_name sym_val_to_name[SYM_TYPES]
#define p_user_val_to_name sym_val_to_name[SYM_USERS]
#define p_bool_val_to_name sym_val_to_name[SYM_BOOLS]
#define p_sens_val_to_name sym_val_to_name[SYM_LEVELS]
#define p_cat_val_to_name sym_val_to_name[SYM_CATS]

	/* class, role, and user attributes indexed by (value - 1) */
	class_datum_t **class_val_to_struct;
	role_datum_t **role_val_to_struct;
	user_datum_t **user_val_to_struct;
	type_datum_t **type_val_to_struct;

	/* module stuff section -- used in parsing and for modules */

	/* keep track of the scope for every identifier.  these are
	 * hash tables, where the key is the identifier name and value
	 * a scope_datum_t.  as a convenience, one may use the
	 * p_*_macros (cf. struct scope_index_t declaration). */
	symtab_t scope[SYM_NUM];

	/* module rule storage */
	avrule_block_t *global;
	/* avrule_decl index used for link/expand */
	avrule_decl_t **decl_val_to_struct;

	/* compiled storage of rules - use for the kernel policy */

	/* type enforcement access vectors and transitions */
	avtab_t te_avtab;

	/* bools indexed by (value - 1) */
	cond_bool_datum_t **bool_val_to_struct;
	/* type enforcement conditional access vectors and transitions */
	avtab_t te_cond_avtab;
	/* linked list indexing te_cond_avtab by conditional */
	cond_list_t *cond_list;

	/* role transitions */
	role_trans_t *role_tr;

	/* role allows */
	role_allow_t *role_allow;

	/* security contexts of initial SIDs, unlabeled file systems,
	   TCP or UDP port numbers, network interfaces and nodes */
	ocontext_t *ocontexts[OCON_NUM];

	/* security contexts for files in filesystems that cannot support
	   a persistent label mapping or use another 
	   fixed labeling behavior. */
	genfs_t *genfs;

	/* range transitions table (range_trans_key -> mls_range) */
	hashtab_t range_tr;

	/* file transitions with the last path component */
	hashtab_t filename_trans;

	ebitmap_t *type_attr_map;

	ebitmap_t *attr_type_map;	/* not saved in the binary policy */

	ebitmap_t policycaps;

	/* this bitmap is referenced by type NOT the typical type-1 used in other
	   bitmaps.  Someday the 0 bit may be used for global permissive */
	ebitmap_t permissive_map;

	unsigned policyvers;

	unsigned handle_unknown;

	sepol_security_class_t process_class;
	sepol_security_class_t dir_class;
	sepol_access_vector_t process_trans;
	sepol_access_vector_t process_trans_dyntrans;
} policydb_t;

struct sepol_policydb {
	struct policydb p;
};

extern int policydb_init(policydb_t * p);

extern int policydb_from_image(sepol_handle_t * handle,
			       void *data, size_t len, policydb_t * policydb);

extern int policydb_to_image(sepol_handle_t * handle,
			     policydb_t * policydb, void **newdata,
			     size_t * newlen);

extern int policydb_index_classes(policydb_t * p);

extern int policydb_index_bools(policydb_t * p);

extern int policydb_index_others(sepol_handle_t * handle, policydb_t * p,
				 unsigned int verbose);

extern int policydb_role_cache(hashtab_key_t key,
			       hashtab_datum_t datum,
			       void *arg);

extern int policydb_user_cache(hashtab_key_t key,
			       hashtab_datum_t datum,
			       void *arg);

extern int policydb_reindex_users(policydb_t * p);

extern int policydb_optimize(policydb_t * p);

extern void policydb_destroy(policydb_t * p);

extern int policydb_load_isids(policydb_t * p, sidtab_t * s);

extern int policydb_sort_ocontexts(policydb_t *p);

/* Deprecated */
extern int policydb_context_isvalid(const policydb_t * p,
				    const context_struct_t * c);

extern void symtabs_destroy(symtab_t * symtab);
extern int scope_destroy(hashtab_key_t key, hashtab_datum_t datum, void *p);

extern void class_perm_node_init(class_perm_node_t * x);
extern void type_set_init(type_set_t * x);
extern void type_set_destroy(type_set_t * x);
extern int type_set_cpy(type_set_t * dst, type_set_t * src);
extern int type_set_or_eq(type_set_t * dst, type_set_t * other);
extern void role_set_init(role_set_t * x);
extern void role_set_destroy(role_set_t * x);
extern void avrule_init(avrule_t * x);
extern void avrule_destroy(avrule_t * x);
extern void avrule_list_destroy(avrule_t * x);
extern void role_trans_rule_init(role_trans_rule_t * x);
extern void role_trans_rule_list_destroy(role_trans_rule_t * x);
extern void filename_trans_rule_init(filename_trans_rule_t * x);
extern void filename_trans_rule_list_destroy(filename_trans_rule_t * x);

extern void role_datum_init(role_datum_t * x);
extern void role_datum_destroy(role_datum_t * x);
extern void role_allow_rule_init(role_allow_rule_t * x);
extern void role_allow_rule_destroy(role_allow_rule_t * x);
extern void role_allow_rule_list_destroy(role_allow_rule_t * x);
extern void range_trans_rule_init(range_trans_rule_t *x);
extern void range_trans_rule_destroy(range_trans_rule_t *x);
extern void range_trans_rule_list_destroy(range_trans_rule_t *x);
extern void type_datum_init(type_datum_t * x);
extern void type_datum_destroy(type_datum_t * x);
extern void user_datum_init(user_datum_t * x);
extern void user_datum_destroy(user_datum_t * x);
extern void level_datum_init(level_datum_t * x);
extern void level_datum_destroy(level_datum_t * x);
extern void cat_datum_init(cat_datum_t * x);
extern void cat_datum_destroy(cat_datum_t * x);
extern int check_assertion(policydb_t *p, avrule_t *avrule);
extern int check_assertions(sepol_handle_t * handle,
			    policydb_t * p, avrule_t * avrules);

extern int symtab_insert(policydb_t * x, uint32_t sym,
			 hashtab_key_t key, hashtab_datum_t datum,
			 uint32_t scope, uint32_t avrule_decl_id,
			 uint32_t * value);

/* A policy "file" may be a memory region referenced by a (data, len) pair
   or a file referenced by a FILE pointer. */
typedef struct policy_file {
#define PF_USE_MEMORY  0
#define PF_USE_STDIO   1
#define PF_LEN         2	/* total up length in len field */
	unsigned type;
	char *data;
	size_t len;
	size_t size;
	FILE *fp;
	struct sepol_handle *handle;
} policy_file_t;

struct sepol_policy_file {
	struct policy_file pf;
};

extern void policy_file_init(policy_file_t * x);

extern int policydb_read(policydb_t * p, struct policy_file *fp,
			 unsigned int verbose);
extern int avrule_read_list(policydb_t * p, avrule_t ** avrules,
			    struct policy_file *fp);

extern int policydb_write(struct policydb *p, struct policy_file *pf);
extern int policydb_set_target_platform(policydb_t *p, int platform);

#define PERM_SYMTAB_SIZE 32

/* Identify specific policy version changes */
#define POLICYDB_VERSION_BASE		15
#define POLICYDB_VERSION_BOOL		16
#define POLICYDB_VERSION_IPV6		17
#define POLICYDB_VERSION_NLCLASS	18
#define POLICYDB_VERSION_VALIDATETRANS	19
#define POLICYDB_VERSION_MLS		19
#define POLICYDB_VERSION_AVTAB		20
#define POLICYDB_VERSION_RANGETRANS	21
#define POLICYDB_VERSION_POLCAP		22
#define POLICYDB_VERSION_PERMISSIVE	23
#define POLICYDB_VERSION_BOUNDARY	24
#define POLICYDB_VERSION_FILENAME_TRANS	25
#define POLICYDB_VERSION_ROLETRANS	26
#define POLICYDB_VERSION_NEW_OBJECT_DEFAULTS	27
#define POLICYDB_VERSION_DEFAULT_TYPE	28
#define POLICYDB_VERSION_CONSTRAINT_NAMES	29
#define POLICYDB_VERSION_XEN_DEVICETREE		30 /* Xen-specific */
#define POLICYDB_VERSION_XPERMS_IOCTL	30 /* Linux-specific */
#define POLICYDB_VERSION_INFINIBAND		31 /* Linux-specific */
#define POLICYDB_VERSION_GLBLUB		32

/* Range of policy versions we understand*/
#define POLICYDB_VERSION_MIN	POLICYDB_VERSION_BASE
#define POLICYDB_VERSION_MAX	POLICYDB_VERSION_GLBLUB

/* Module versions and specific changes*/
#define MOD_POLICYDB_VERSION_BASE		4
#define MOD_POLICYDB_VERSION_VALIDATETRANS	5
#define MOD_POLICYDB_VERSION_MLS		5
#define MOD_POLICYDB_VERSION_RANGETRANS 	6
#define MOD_POLICYDB_VERSION_MLS_USERS		6
#define MOD_POLICYDB_VERSION_POLCAP		7
#define MOD_POLICYDB_VERSION_PERMISSIVE		8
#define MOD_POLICYDB_VERSION_BOUNDARY		9
#define MOD_POLICYDB_VERSION_BOUNDARY_ALIAS	10
#define MOD_POLICYDB_VERSION_FILENAME_TRANS	11
#define MOD_POLICYDB_VERSION_ROLETRANS		12
#define MOD_POLICYDB_VERSION_ROLEATTRIB		13
#define MOD_POLICYDB_VERSION_TUNABLE_SEP	14
#define MOD_POLICYDB_VERSION_NEW_OBJECT_DEFAULTS	15
#define MOD_POLICYDB_VERSION_DEFAULT_TYPE	16
#define MOD_POLICYDB_VERSION_CONSTRAINT_NAMES  17
#define MOD_POLICYDB_VERSION_XPERMS_IOCTL  18
#define MOD_POLICYDB_VERSION_INFINIBAND		19
#define MOD_POLICYDB_VERSION_GLBLUB		20

#define MOD_POLICYDB_VERSION_MIN MOD_POLICYDB_VERSION_BASE
#define MOD_POLICYDB_VERSION_MAX MOD_POLICYDB_VERSION_GLBLUB

#define POLICYDB_CONFIG_MLS    1

/* macros to check policy feature */

/* TODO: add other features here */

#define policydb_has_boundary_feature(p)			\
	(((p)->policy_type == POLICY_KERN			\
	  && p->policyvers >= POLICYDB_VERSION_BOUNDARY) ||	\
	 ((p)->policy_type != POLICY_KERN			\
	  && p->policyvers >= MOD_POLICYDB_VERSION_BOUNDARY))

/* the config flags related to unknown classes/perms are bits 2 and 3 */
#define DENY_UNKNOWN	SEPOL_DENY_UNKNOWN
#define REJECT_UNKNOWN	SEPOL_REJECT_UNKNOWN
#define ALLOW_UNKNOWN 	SEPOL_ALLOW_UNKNOWN

#define POLICYDB_CONFIG_UNKNOWN_MASK	(DENY_UNKNOWN | REJECT_UNKNOWN | ALLOW_UNKNOWN)

#define OBJECT_R "object_r"
#define OBJECT_R_VAL 1

#define POLICYDB_MAGIC SELINUX_MAGIC
#define POLICYDB_STRING "SE Linux"
#define POLICYDB_XEN_STRING "XenFlask"
#define POLICYDB_STRING_MAX_LENGTH 32
#define POLICYDB_MOD_MAGIC SELINUX_MOD_MAGIC
#define POLICYDB_MOD_STRING "SE Linux Module"

#ifdef __cplusplus
}
#endif

#endif				/* _POLICYDB_H_ */

/* FLASK */
