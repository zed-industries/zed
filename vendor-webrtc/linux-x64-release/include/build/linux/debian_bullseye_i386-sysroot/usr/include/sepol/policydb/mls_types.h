/* Author : Stephen Smalley, <sds@tycho.nsa.gov> */
/*
 * Updated: Trusted Computer Solutions, Inc. <dgoeddel@trustedcs.com>
 *
 *	Support for enhanced MLS infrastructure.
 *
 * Copyright (C) 2004-2005 Trusted Computer Solutions, Inc.
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
 * Type definitions for the multi-level security (MLS) policy.
 */

#ifndef _SEPOL_POLICYDB_MLS_TYPES_H_
#define _SEPOL_POLICYDB_MLS_TYPES_H_

#include <errno.h>
#include <stdint.h>
#include <stdlib.h>
#include <sys/param.h>
#include <sepol/policydb/ebitmap.h>
#include <sepol/policydb/flask_types.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct mls_level {
	uint32_t sens;		/* sensitivity */
	ebitmap_t cat;		/* category set */
} mls_level_t;

typedef struct mls_range {
	mls_level_t level[2];	/* low == level[0], high == level[1] */
} mls_range_t;

static inline int mls_range_glblub(struct mls_range *dst, struct mls_range *r1, struct mls_range *r2)
{
	if (r1->level[1].sens < r2->level[0].sens || r2->level[1].sens < r1->level[0].sens) {
		/* These ranges have no common sensitivities */
		return -EINVAL;
	}

	/* Take the greatest of the low */
	dst->level[0].sens = MAX(r1->level[0].sens, r2->level[0].sens);
	/* Take the least of the high */
	dst->level[1].sens = MIN(r1->level[1].sens, r2->level[1].sens);

	if (ebitmap_and(&dst->level[0].cat, &r1->level[0].cat, &r2->level[0].cat) < 0) {
		return -1;
	}

	if (ebitmap_and(&dst->level[1].cat, &r1->level[1].cat, &r2->level[1].cat) < 0) {
		return -1;
	}

	return 0;
}


static inline int mls_level_cpy(struct mls_level *dst, struct mls_level *src)
{

	dst->sens = src->sens;
	if (ebitmap_cpy(&dst->cat, &src->cat) < 0)
		return -1;
	return 0;
}

static inline void mls_level_init(struct mls_level *level)
{

	memset(level, 0, sizeof(mls_level_t));
}

static inline void mls_level_destroy(struct mls_level *level)
{

	if (level == NULL)
		return;

	ebitmap_destroy(&level->cat);
	mls_level_init(level);
}

static inline int mls_level_eq(const struct mls_level *l1, const struct mls_level *l2)
{
	return ((l1->sens == l2->sens) && ebitmap_cmp(&l1->cat, &l2->cat));
}

static inline int mls_level_dom(const struct mls_level *l1, const struct mls_level *l2)
{
	return ((l1->sens >= l2->sens) && ebitmap_contains(&l1->cat, &l2->cat));
}

#define mls_level_incomp(l1, l2) \
(!mls_level_dom((l1), (l2)) && !mls_level_dom((l2), (l1)))

#define mls_level_between(l1, l2, l3) \
(mls_level_dom((l1), (l2)) && mls_level_dom((l3), (l1)))

#define mls_range_contains(r1, r2) \
(mls_level_dom(&(r2).level[0], &(r1).level[0]) && \
 mls_level_dom(&(r1).level[1], &(r2).level[1]))

static inline int mls_range_cpy(mls_range_t * dst, mls_range_t * src)
{

	if (mls_level_cpy(&dst->level[0], &src->level[0]) < 0)
		goto err;

	if (mls_level_cpy(&dst->level[1], &src->level[1]) < 0)
		goto err_destroy;

	return 0;

      err_destroy:
	mls_level_destroy(&dst->level[0]);

      err:
	return -1;
}

static inline void mls_range_init(struct mls_range *r)
{
	mls_level_init(&r->level[0]);
	mls_level_init(&r->level[1]);
}

static inline void mls_range_destroy(struct mls_range *r)
{
	mls_level_destroy(&r->level[0]);
	mls_level_destroy(&r->level[1]);
}

static inline int mls_range_eq(struct mls_range *r1, struct mls_range *r2)
{
	return (mls_level_eq(&r1->level[0], &r2->level[0]) &&
	        mls_level_eq(&r1->level[1], &r2->level[1]));
}

typedef struct mls_semantic_cat {
	uint32_t low;	/* first bit this struct represents */
	uint32_t high;	/* last bit represented - equals low for a single cat */
	struct mls_semantic_cat *next;
} mls_semantic_cat_t;

typedef struct mls_semantic_level {
	uint32_t sens;
	mls_semantic_cat_t *cat;
} mls_semantic_level_t;

typedef struct mls_semantic_range {
	mls_semantic_level_t level[2];
} mls_semantic_range_t;

extern void mls_semantic_cat_init(mls_semantic_cat_t *c);
extern void mls_semantic_cat_destroy(mls_semantic_cat_t *c);
extern void mls_semantic_level_init(mls_semantic_level_t *l);
extern void mls_semantic_level_destroy(mls_semantic_level_t *l);
extern int mls_semantic_level_cpy(mls_semantic_level_t *dst, mls_semantic_level_t *src);
extern void mls_semantic_range_init(mls_semantic_range_t *r);
extern void mls_semantic_range_destroy(mls_semantic_range_t *r);
extern int mls_semantic_range_cpy(mls_semantic_range_t *dst, mls_semantic_range_t *src);

#ifdef __cplusplus
}
#endif

#endif
