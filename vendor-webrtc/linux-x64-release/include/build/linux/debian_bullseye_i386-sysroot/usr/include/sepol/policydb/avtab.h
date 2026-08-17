
/* Author : Stephen Smalley, <sds@tycho.nsa.gov> */

/*
 * Updated: Yuichi Nakamura <ynakam@hitachisoft.jp>
 * 	Tuned number of hash slots for avtab to reduce memory usage
 */

/* Updated: Frank Mayer <mayerf@tresys.com> and Karl MacMillan <kmacmillan@tresys.com>
 *
 * 	Added conditional policy language extensions
 *
 * Copyright (C) 2003 Tresys Technology, LLC
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
 * An access vector table (avtab) is a hash table
 * of access vectors and transition types indexed 
 * by a type pair and a class.  An access vector
 * table is used to represent the type enforcement
 * tables.
 */

#ifndef _SEPOL_POLICYDB_AVTAB_H_
#define _SEPOL_POLICYDB_AVTAB_H_

#include <sys/types.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct avtab_key {
	uint16_t source_type;
	uint16_t target_type;
	uint16_t target_class;
#define AVTAB_ALLOWED		0x0001
#define AVTAB_AUDITALLOW	0x0002
#define AVTAB_AUDITDENY		0x0004
#define AVTAB_NEVERALLOW	0x0080
#define AVTAB_AV		(AVTAB_ALLOWED | AVTAB_AUDITALLOW | AVTAB_AUDITDENY)
#define AVTAB_TRANSITION	0x0010
#define AVTAB_MEMBER		0x0020
#define AVTAB_CHANGE		0x0040
#define AVTAB_TYPE		(AVTAB_TRANSITION | AVTAB_MEMBER | AVTAB_CHANGE)
#define AVTAB_XPERMS_ALLOWED	0x0100
#define AVTAB_XPERMS_AUDITALLOW	0x0200
#define AVTAB_XPERMS_DONTAUDIT	0x0400
#define AVTAB_XPERMS_NEVERALLOW	0x0800
#define AVTAB_XPERMS		(AVTAB_XPERMS_ALLOWED | AVTAB_XPERMS_AUDITALLOW | AVTAB_XPERMS_DONTAUDIT)
#define AVTAB_ENABLED_OLD	0x80000000
#define AVTAB_ENABLED		0x8000	/* reserved for used in cond_avtab */
	uint16_t specified;	/* what fields are specified */
} avtab_key_t;

typedef struct avtab_extended_perms {

#define AVTAB_XPERMS_IOCTLFUNCTION	0x01
#define AVTAB_XPERMS_IOCTLDRIVER	0x02
	/* extension of the avtab_key specified */
	uint8_t specified;
	uint8_t driver;
	uint32_t perms[8];
} avtab_extended_perms_t;

typedef struct avtab_datum {
	uint32_t data;		/* access vector or type */
	avtab_extended_perms_t *xperms;
} avtab_datum_t;

typedef struct avtab_node *avtab_ptr_t;

struct avtab_node {
	avtab_key_t key;
	avtab_datum_t datum;
	avtab_ptr_t next;
	void *parse_context;	/* generic context pointer used by parser;
				 * not saved in binary policy */
	unsigned merged;	/* flag for avtab_write only;
				   not saved in binary policy */
};

typedef struct avtab {
	avtab_ptr_t *htable;
	uint32_t nel;		/* number of elements */
	uint32_t nslot;         /* number of hash slots */
	uint32_t mask;          /* mask to compute hash func */
} avtab_t;

extern int avtab_init(avtab_t *);
extern int avtab_alloc(avtab_t *, uint32_t);
extern int avtab_insert(avtab_t * h, avtab_key_t * k, avtab_datum_t * d);

extern avtab_datum_t *avtab_search(avtab_t * h, avtab_key_t * k);

extern void avtab_destroy(avtab_t * h);

extern int avtab_map(avtab_t * h,
		     int (*apply) (avtab_key_t * k,
				   avtab_datum_t * d, void *args), void *args);

extern void avtab_hash_eval(avtab_t * h, char *tag);

struct policy_file;
extern int avtab_read_item(struct policy_file *fp, uint32_t vers, avtab_t * a,
			   int (*insert) (avtab_t * a, avtab_key_t * k,
					  avtab_datum_t * d, void *p), void *p);

extern int avtab_read(avtab_t * a, struct policy_file *fp, uint32_t vers);

extern avtab_ptr_t avtab_insert_nonunique(avtab_t * h, avtab_key_t * key,
					  avtab_datum_t * datum);

extern avtab_ptr_t avtab_insert_with_parse_context(avtab_t * h,
						   avtab_key_t * key,
						   avtab_datum_t * datum,
						   void *parse_context);

extern avtab_ptr_t avtab_search_node(avtab_t * h, avtab_key_t * key);

extern avtab_ptr_t avtab_search_node_next(avtab_ptr_t node, int specified);

#define MAX_AVTAB_HASH_BITS 20
#define MAX_AVTAB_HASH_BUCKETS (1 << MAX_AVTAB_HASH_BITS)
#define MAX_AVTAB_HASH_MASK (MAX_AVTAB_HASH_BUCKETS-1)
/* avtab_alloc uses one bucket per 2-4 elements, so adjust to get maximum buckets */
#define MAX_AVTAB_SIZE (MAX_AVTAB_HASH_BUCKETS << 1)

#ifdef __cplusplus
}
#endif

#endif				/* _AVTAB_H_ */

/* FLASK */
