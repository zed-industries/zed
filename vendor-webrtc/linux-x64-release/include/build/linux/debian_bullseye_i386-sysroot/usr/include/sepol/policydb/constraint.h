/* Author : Stephen Smalley, <sds@tycho.nsa.gov> */

/* FLASK */

/*
 * A constraint is a condition that must be satisfied in
 * order for one or more permissions to be granted.  
 * Constraints are used to impose additional restrictions
 * beyond the type-based rules in `te' or the role-based
 * transition rules in `rbac'.  Constraints are typically
 * used to prevent a process from transitioning to a new user 
 * identity or role unless it is in a privileged type.
 * Constraints are likewise typically used to prevent a
 * process from labeling an object with a different user
 * identity.   
 */

#ifndef _SEPOL_POLICYDB_CONSTRAINT_H_
#define _SEPOL_POLICYDB_CONSTRAINT_H_

#include <sepol/policydb/policydb.h>
#include <sepol/policydb/ebitmap.h>
#include <sepol/policydb/flask_types.h>

#ifdef __cplusplus
extern "C" {
#endif

#define CEXPR_MAXDEPTH 5

struct type_set;

typedef struct constraint_expr {
#define CEXPR_NOT		1	/* not expr */
#define CEXPR_AND		2	/* expr and expr */
#define CEXPR_OR		3	/* expr or expr */
#define CEXPR_ATTR		4	/* attr op attr */
#define CEXPR_NAMES		5	/* attr op names */
	uint32_t expr_type;	/* expression type */

#define CEXPR_USER 1		/* user */
#define CEXPR_ROLE 2		/* role */
#define CEXPR_TYPE 4		/* type */
#define CEXPR_TARGET 8		/* target if set, source otherwise */
#define CEXPR_XTARGET 16	/* special 3rd target for validatetrans rule */
#define CEXPR_L1L2 32		/* low level 1 vs. low level 2 */
#define CEXPR_L1H2 64		/* low level 1 vs. high level 2 */
#define CEXPR_H1L2 128		/* high level 1 vs. low level 2 */
#define CEXPR_H1H2 256		/* high level 1 vs. high level 2 */
#define CEXPR_L1H1 512		/* low level 1 vs. high level 1 */
#define CEXPR_L2H2 1024		/* low level 2 vs. high level 2 */
	uint32_t attr;		/* attribute */

#define CEXPR_EQ     1		/* == or eq */
#define CEXPR_NEQ    2		/* != */
#define CEXPR_DOM    3		/* dom */
#define CEXPR_DOMBY  4		/* domby  */
#define CEXPR_INCOMP 5		/* incomp */
	uint32_t op;		/* operator */

	ebitmap_t names;	/* names */
	struct type_set *type_names;

	struct constraint_expr *next;	/* next expression */
} constraint_expr_t;

typedef struct constraint_node {
	sepol_access_vector_t permissions;	/* constrained permissions */
	constraint_expr_t *expr;	/* constraint on permissions */
	struct constraint_node *next;	/* next constraint */
} constraint_node_t;

struct policydb;

extern int constraint_expr_init(constraint_expr_t * expr);
extern void constraint_expr_destroy(constraint_expr_t * expr);

#ifdef __cplusplus
}
#endif

#endif				/* _CONSTRAINT_H_ */

/* FLASK */
