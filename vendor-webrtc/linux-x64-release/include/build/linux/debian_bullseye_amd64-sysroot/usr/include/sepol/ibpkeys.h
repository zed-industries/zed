#ifndef _SEPOL_IBPKEYS_H_
#define _SEPOL_IBPKEYS_H_

#include <sepol/handle.h>
#include <sepol/policydb.h>
#include <sepol/ibpkey_record.h>


#ifdef __cplusplus
extern "C" {
#endif

/* Return the number of ibpkeys */
extern int sepol_ibpkey_count(sepol_handle_t *handle,
			      const sepol_policydb_t *p, unsigned int *response);

/* Check if a ibpkey exists */
extern int sepol_ibpkey_exists(sepol_handle_t *handle,
			       const sepol_policydb_t *policydb,
			       const sepol_ibpkey_key_t *key, int *response);

/* Query a ibpkey - returns the ibpkey, or NULL if not found */
extern int sepol_ibpkey_query(sepol_handle_t *handle,
			      const sepol_policydb_t *policydb,
			      const sepol_ibpkey_key_t *key,
			      sepol_ibpkey_t **response);

/* Modify a ibpkey, or add it, if the key is not found */
extern int sepol_ibpkey_modify(sepol_handle_t *handle,
			       sepol_policydb_t *policydb,
			       const sepol_ibpkey_key_t *key,
			       const sepol_ibpkey_t *data);

/* Iterate the ibpkeys
 * The handler may return:
 * -1 to signal an error condition,
 * 1 to signal successful exit
 * 0 to signal continue
 */
extern int sepol_ibpkey_iterate(sepol_handle_t *handle,
				const sepol_policydb_t *policydb,
				int (*fn)(const sepol_ibpkey_t *ibpkey,
					  void *fn_arg), void *arg);


#ifdef __cplusplus
}
#endif

#endif
