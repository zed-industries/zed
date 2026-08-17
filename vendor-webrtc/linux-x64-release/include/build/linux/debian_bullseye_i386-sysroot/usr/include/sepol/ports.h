#ifndef _SEPOL_PORTS_H_
#define _SEPOL_PORTS_H_

#include <sepol/handle.h>
#include <sepol/policydb.h>
#include <sepol/port_record.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Return the number of ports */
extern int sepol_port_count(sepol_handle_t * handle,
			    const sepol_policydb_t * p, unsigned int *response);

/* Check if a port exists */
extern int sepol_port_exists(sepol_handle_t * handle,
			     const sepol_policydb_t * policydb,
			     const sepol_port_key_t * key, int *response);

/* Query a port - returns the port, or NULL if not found */
extern int sepol_port_query(sepol_handle_t * handle,
			    const sepol_policydb_t * policydb,
			    const sepol_port_key_t * key,
			    sepol_port_t ** response);

/* Modify a port, or add it, if the key is not found */
extern int sepol_port_modify(sepol_handle_t * handle,
			     sepol_policydb_t * policydb,
			     const sepol_port_key_t * key,
			     const sepol_port_t * data);

/* Iterate the ports 
 * The handler may return:
 * -1 to signal an error condition,
 * 1 to signal successful exit
 * 0 to signal continue */

extern int sepol_port_iterate(sepol_handle_t * handle,
			      const sepol_policydb_t * policydb,
			      int (*fn) (const sepol_port_t * port,
					 void *fn_arg), void *arg);

#ifdef __cplusplus
}
#endif

#endif
