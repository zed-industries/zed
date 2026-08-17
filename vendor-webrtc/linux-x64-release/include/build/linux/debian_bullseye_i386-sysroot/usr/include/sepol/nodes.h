#ifndef _SEPOL_NODES_H_
#define _SEPOL_NODES_H_

#include <sepol/handle.h>
#include <sepol/policydb.h>
#include <sepol/node_record.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Return the number of nodes */
extern int sepol_node_count(sepol_handle_t * handle,
			    const sepol_policydb_t * p, unsigned int *response);

/* Check if a node exists */
extern int sepol_node_exists(sepol_handle_t * handle,
			     const sepol_policydb_t * policydb,
			     const sepol_node_key_t * key, int *response);

/* Query a node - returns the node, or NULL if not found */
extern int sepol_node_query(sepol_handle_t * handle,
			    const sepol_policydb_t * policydb,
			    const sepol_node_key_t * key,
			    sepol_node_t ** response);

/* Modify a node, or add it, if the key is not found */
extern int sepol_node_modify(sepol_handle_t * handle,
			     sepol_policydb_t * policydb,
			     const sepol_node_key_t * key,
			     const sepol_node_t * data);

/* Iterate the nodes 
 * The handler may return:
 * -1 to signal an error condition,
 * 1 to signal successful exit
 * 0 to signal continue */

extern int sepol_node_iterate(sepol_handle_t * handle,
			      const sepol_policydb_t * policydb,
			      int (*fn) (const sepol_node_t * node,
					 void *fn_arg), void *arg);

#ifdef __cplusplus
}
#endif

#endif
