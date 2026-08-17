#ifndef __SEPOL_INTERFACES_H_
#define __SEPOL_INTERFACES_H_

#include <sepol/policydb.h>
#include <sepol/iface_record.h>
#include <sepol/handle.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Return the number of interfaces */
extern int sepol_iface_count(sepol_handle_t * handle,
			     const sepol_policydb_t * policydb,
			     unsigned int *response);

/* Check if an interface exists */
extern int sepol_iface_exists(sepol_handle_t * handle,
			      const sepol_policydb_t * policydb,
			      const sepol_iface_key_t * key, int *response);

/* Query an interface - returns the interface, 
 * or NULL if not found */
extern int sepol_iface_query(sepol_handle_t * handle,
			     const sepol_policydb_t * policydb,
			     const sepol_iface_key_t * key,
			     sepol_iface_t ** response);

/* Modify an interface, or add it, if the key
 * is not found */
extern int sepol_iface_modify(sepol_handle_t * handle,
			      sepol_policydb_t * policydb,
			      const sepol_iface_key_t * key,
			      const sepol_iface_t * data);

/* Iterate the interfaces
 * The handler may return:
 * -1 to signal an error condition,
 * 1 to signal successful exit
 * 0 to signal continue */

extern int sepol_iface_iterate(sepol_handle_t * handle,
			       const sepol_policydb_t * policydb,
			       int (*fn) (const sepol_iface_t * iface,
					  void *fn_arg), void *arg);

#ifdef __cplusplus
}
#endif

#endif
