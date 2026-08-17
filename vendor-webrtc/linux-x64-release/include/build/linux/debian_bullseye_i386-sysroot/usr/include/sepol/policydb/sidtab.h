/* Author : Stephen Smalley, <sds@tycho.nsa.gov> */

/* FLASK */

/*
 * A security identifier table (sidtab) is a hash table
 * of security context structures indexed by SID value.
 */

#ifndef _SEPOL_POLICYDB_SIDTAB_H_
#define _SEPOL_POLICYDB_SIDTAB_H_

#include <sepol/policydb/context.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct sidtab_node {
	sepol_security_id_t sid;	/* security identifier */
	context_struct_t context;	/* security context structure */
	struct sidtab_node *next;
} sidtab_node_t;

typedef struct sidtab_node *sidtab_ptr_t;

#define SIDTAB_HASH_BITS 7
#define SIDTAB_HASH_BUCKETS (1 << SIDTAB_HASH_BITS)
#define SIDTAB_HASH_MASK (SIDTAB_HASH_BUCKETS-1)

#define SIDTAB_SIZE SIDTAB_HASH_BUCKETS

typedef struct {
	sidtab_ptr_t *htable;
	unsigned int nel;	/* number of elements */
	unsigned int next_sid;	/* next SID to allocate */
	unsigned char shutdown;
} sidtab_t;

extern int sepol_sidtab_init(sidtab_t * s);

extern int sepol_sidtab_insert(sidtab_t * s,
			       sepol_security_id_t sid,
			       context_struct_t * context);

extern context_struct_t *sepol_sidtab_search(sidtab_t * s,
					     sepol_security_id_t sid);

extern int sepol_sidtab_map(sidtab_t * s,
			    int (*apply) (sepol_security_id_t sid,
					  context_struct_t * context,
					  void *args), void *args);

extern void sepol_sidtab_map_remove_on_error(sidtab_t * s,
					     int (*apply) (sepol_security_id_t
							   s,
							   context_struct_t *
							   context, void *args),
					     void *args);

extern int sepol_sidtab_context_to_sid(sidtab_t * s,	/* IN */
				       context_struct_t * context,	/* IN */
				       sepol_security_id_t * sid);	/* OUT */

extern void sepol_sidtab_hash_eval(sidtab_t * h, char *tag);

extern void sepol_sidtab_destroy(sidtab_t * s);

extern void sepol_sidtab_set(sidtab_t * dst, sidtab_t * src);

extern void sepol_sidtab_shutdown(sidtab_t * s);

#ifdef __cplusplus
}
#endif

#endif				/* _SIDTAB_H_ */

/* FLASK */
