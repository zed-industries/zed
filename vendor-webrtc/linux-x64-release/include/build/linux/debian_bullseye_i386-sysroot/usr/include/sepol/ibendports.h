#ifndef _SEPOL_IBENDPORTS_H_
#define _SEPOL_IBENDPORTS_H_

#include <sepol/handle.h>
#include <sepol/policydb.h>
#include <sepol/ibendport_record.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Return the number of ibendports */
extern int sepol_ibendport_count(sepol_handle_t *handle,
				 const sepol_policydb_t *p,
				 unsigned int *response);

/* Check if a ibendport exists */
extern int sepol_ibendport_exists(sepol_handle_t *handle,
				  const sepol_policydb_t *policydb,
				  const sepol_ibendport_key_t *key, int *response);

/* Query a ibendport - returns the ibendport, or NULL if not found */
extern int sepol_ibendport_query(sepol_handle_t *handle,
				 const sepol_policydb_t *policydb,
				 const sepol_ibendport_key_t *key,
				 sepol_ibendport_t **response);

/* Modify a ibendport, or add it, if the key is not found */
extern int sepol_ibendport_modify(sepol_handle_t *handle,
				  sepol_policydb_t *policydb,
				  const sepol_ibendport_key_t *key,
				  const sepol_ibendport_t *data);

/* Iterate the ibendports
 * The handler may return:
 * -1 to signal an error condition,
 * 1 to signal successful exit
 * 0 to signal continue
 */
extern int sepol_ibendport_iterate(sepol_handle_t *handle,
				   const sepol_policydb_t *policydb,
				   int (*fn)(const sepol_ibendport_t *ibendport,
					     void *fn_arg), void *arg);


#ifdef __cplusplus
}
#endif

#endif
