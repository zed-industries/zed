/* Authors: Karl MacMillan <kmacmillan@tresys.com>
 *          Frank Mayer <mayerf@tresys.com>
 *
 * Copyright (C) 2003 - 2005 Tresys Technology, LLC
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

#ifndef _SEPOL_POLICYDB_CONDITIONAL_H_
#define _SEPOL_POLICYDB_CONDITIONAL_H_

#include <sepol/policydb/flask_types.h>
#include <sepol/policydb/avtab.h>
#include <sepol/policydb/symtab.h>
#include <sepol/policydb/policydb.h>

#ifdef __cplusplus
extern "C" {
#endif

#define COND_EXPR_MAXDEPTH 10

/* this is the max unique bools in a conditional expression
 * for which we precompute all outcomes for the expression.
 *
 * NOTE - do _NOT_ use value greater than 5 because
 * cond_node_t->expr_pre_comp can only hold at most 32 values
 */
#define COND_MAX_BOOLS 5

/*
 * A conditional expression is a list of operators and operands
 * in reverse polish notation.
 */
typedef struct cond_expr {
#define COND_BOOL	1	/* plain bool */
#define COND_NOT	2	/* !bool */
#define COND_OR		3	/* bool || bool */
#define COND_AND	4	/* bool && bool */
#define COND_XOR	5	/* bool ^ bool */
#define COND_EQ		6	/* bool == bool */
#define COND_NEQ	7	/* bool != bool */
#define COND_LAST	COND_NEQ
	uint32_t expr_type;
	uint32_t bool;
	struct cond_expr *next;
} cond_expr_t;

/*
 * Each cond_node_t contains a list of rules to be enabled/disabled
 * depending on the current value of the conditional expression. This 
 * struct is for that list.
 */
typedef struct cond_av_list {
	avtab_ptr_t node;
	struct cond_av_list *next;
} cond_av_list_t;

/*
 * A cond node represents a conditional block in a policy. It
 * contains a conditional expression, the current state of the expression,
 * two lists of rules to enable/disable depending on the value of the
 * expression (the true list corresponds to if and the false list corresponds
 * to else)..
 */
typedef struct cond_node {
	int cur_state;
	cond_expr_t *expr;
	/* these true/false lists point into te_avtab when that is used */
	cond_av_list_t *true_list;
	cond_av_list_t *false_list;
	/* and these are used during parsing and for modules */
	avrule_t *avtrue_list;
	avrule_t *avfalse_list;
	/* these fields are not written to binary policy */
	unsigned int nbools;
	uint32_t bool_ids[COND_MAX_BOOLS];
	uint32_t expr_pre_comp;
	struct cond_node *next;
	/* a tunable conditional, calculated and used at expansion */
#define	COND_NODE_FLAGS_TUNABLE	0x01
	uint32_t flags;
} cond_node_t;

extern int cond_evaluate_expr(policydb_t * p, cond_expr_t * expr);
extern cond_expr_t *cond_copy_expr(cond_expr_t * expr);

extern int cond_expr_equal(cond_node_t * a, cond_node_t * b);
extern int cond_normalize_expr(policydb_t * p, cond_node_t * cn);
extern void cond_node_destroy(cond_node_t * node);
extern void cond_expr_destroy(cond_expr_t * expr);

extern cond_node_t *cond_node_find(policydb_t * p,
				   cond_node_t * needle, cond_node_t * haystack,
				   int *was_created);

extern cond_node_t *cond_node_create(policydb_t * p, cond_node_t * node);

extern cond_node_t *cond_node_search(policydb_t * p, cond_node_t * list,
				     cond_node_t * cn);

extern int evaluate_conds(policydb_t * p);

extern avtab_datum_t *cond_av_list_search(avtab_key_t * key,
					  cond_av_list_t * cond_list);

extern void cond_av_list_destroy(cond_av_list_t * list);

extern void cond_optimize_lists(cond_list_t * cl);

extern int cond_policydb_init(policydb_t * p);
extern void cond_policydb_destroy(policydb_t * p);
extern void cond_list_destroy(cond_list_t * list);

extern int cond_init_bool_indexes(policydb_t * p);
extern int cond_destroy_bool(hashtab_key_t key, hashtab_datum_t datum, void *p);

extern int cond_index_bool(hashtab_key_t key, hashtab_datum_t datum,
			   void *datap);

extern int cond_read_bool(policydb_t * p, hashtab_t h, struct policy_file *fp);

extern int cond_read_list(policydb_t * p, cond_list_t ** list, void *fp);

extern void cond_compute_av(avtab_t * ctab, avtab_key_t * key,
			    struct sepol_av_decision *avd);

#ifdef __cplusplus
}
#endif

#endif				/* _CONDITIONAL_H_ */
