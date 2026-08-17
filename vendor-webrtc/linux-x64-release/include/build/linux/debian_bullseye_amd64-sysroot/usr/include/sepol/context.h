#ifndef _SEPOL_CONTEXT_H_
#define _SEPOL_CONTEXT_H_

#include <sepol/context_record.h>
#include <sepol/policydb.h>
#include <sepol/handle.h>

#ifdef __cplusplus
extern "C" {
#endif

/* -- Deprecated -- */

extern int sepol_check_context(const char *context);

/* -- End deprecated -- */

extern int sepol_context_check(sepol_handle_t * handle,
			       const sepol_policydb_t * policydb,
			       const sepol_context_t * context);

extern int sepol_mls_contains(sepol_handle_t * handle,
			      const sepol_policydb_t * policydb,
			      const char *mls1,
			      const char *mls2, int *response);

extern int sepol_mls_check(sepol_handle_t * handle,
			   const sepol_policydb_t * policydb, const char *mls);

#ifdef __cplusplus
}
#endif

#endif
