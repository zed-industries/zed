/* Author : Stephen Smalley, <sds@tycho.nsa.gov> */

/* FLASK */

/*
 * An extensible bitmap is a bitmap that supports an 
 * arbitrary number of bits.  Extensible bitmaps are
 * used to represent sets of values, such as types,
 * roles, categories, and classes.
 *
 * Each extensible bitmap is implemented as a linked
 * list of bitmap nodes, where each bitmap node has
 * an explicitly specified starting bit position within
 * the total bitmap.
 */

#ifndef _SEPOL_POLICYDB_EBITMAP_H_
#define _SEPOL_POLICYDB_EBITMAP_H_

#include <stdint.h>
#include <string.h>

#ifdef __cplusplus
extern "C" {
#endif

#define MAPTYPE uint64_t	/* portion of bitmap in each node */
#define MAPSIZE (sizeof(MAPTYPE) * 8)	/* number of bits in node bitmap */
#define MAPBIT  1ULL		/* a bit in the node bitmap */

typedef struct ebitmap_node {
	uint32_t startbit;	/* starting position in the total bitmap */
	MAPTYPE map;		/* this node's portion of the bitmap */
	struct ebitmap_node *next;
} ebitmap_node_t;

typedef struct ebitmap {
	ebitmap_node_t *node;	/* first node in the bitmap */
	uint32_t highbit;	/* highest position in the total bitmap */
} ebitmap_t;

#define ebitmap_is_empty(e) (((e)->highbit) == 0)
#define ebitmap_length(e) ((e)->highbit)
#define ebitmap_startbit(e) ((e)->node ? (e)->node->startbit : 0)
#define ebitmap_startnode(e) ((e)->node)

static inline unsigned int ebitmap_start(const ebitmap_t * e,
					 ebitmap_node_t ** n)
{

	*n = e->node;
	return ebitmap_startbit(e);
}

static inline void ebitmap_init(ebitmap_t * e)
{
	memset(e, 0, sizeof(*e));
}

static inline unsigned int ebitmap_next(ebitmap_node_t ** n, unsigned int bit)
{
	if ((bit == ((*n)->startbit + MAPSIZE - 1)) && (*n)->next) {
		*n = (*n)->next;
		return (*n)->startbit;
	}

	return (bit + 1);
}

static inline int ebitmap_node_get_bit(ebitmap_node_t * n, unsigned int bit)
{
	if (n->map & (MAPBIT << (bit - n->startbit)))
		return 1;
	return 0;
}

#define ebitmap_for_each_bit(e, n, bit) \
	for (bit = ebitmap_start(e, &n); bit < ebitmap_length(e); bit = ebitmap_next(&n, bit)) \

#define ebitmap_for_each_positive_bit(e, n, bit) \
	ebitmap_for_each_bit(e, n, bit) if (ebitmap_node_get_bit(n, bit)) \

extern int ebitmap_cmp(const ebitmap_t * e1, const ebitmap_t * e2);
extern int ebitmap_or(ebitmap_t * dst, const ebitmap_t * e1, const ebitmap_t * e2);
extern int ebitmap_union(ebitmap_t * dst, const ebitmap_t * e1);
extern int ebitmap_and(ebitmap_t *dst, ebitmap_t *e1, ebitmap_t *e2);
extern int ebitmap_xor(ebitmap_t *dst, ebitmap_t *e1, ebitmap_t *e2);
extern int ebitmap_not(ebitmap_t *dst, ebitmap_t *e1, unsigned int maxbit);
extern int ebitmap_andnot(ebitmap_t *dst, ebitmap_t *e1, ebitmap_t *e2, unsigned int maxbit);
extern unsigned int ebitmap_cardinality(ebitmap_t *e1);
extern int ebitmap_hamming_distance(ebitmap_t * e1, ebitmap_t * e2);
extern int ebitmap_cpy(ebitmap_t * dst, const ebitmap_t * src);
extern int ebitmap_contains(const ebitmap_t * e1, const ebitmap_t * e2);
extern int ebitmap_match_any(const ebitmap_t *e1, const ebitmap_t *e2);
extern int ebitmap_get_bit(const ebitmap_t * e, unsigned int bit);
extern int ebitmap_set_bit(ebitmap_t * e, unsigned int bit, int value);
extern void ebitmap_destroy(ebitmap_t * e);
extern int ebitmap_read(ebitmap_t * e, void *fp);

#ifdef __cplusplus
}
#endif

#endif				/* _EBITMAP_H_ */

/* FLASK */
