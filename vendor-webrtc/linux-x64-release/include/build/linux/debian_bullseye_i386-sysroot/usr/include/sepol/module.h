#ifndef _SEPOL_MODULE_H_
#define _SEPOL_MODULE_H_

#include <stddef.h>
#include <stdio.h>
#include <stdint.h>

#include <sepol/handle.h>
#include <sepol/policydb.h>

#ifdef __cplusplus
extern "C" {
#endif

struct sepol_module_package;
typedef struct sepol_module_package sepol_module_package_t;

/* Module package public interfaces. */

extern int sepol_module_package_create(sepol_module_package_t ** p);

extern void sepol_module_package_free(sepol_module_package_t * p);

extern char *sepol_module_package_get_file_contexts(sepol_module_package_t * p);

extern size_t sepol_module_package_get_file_contexts_len(sepol_module_package_t
							 * p);

extern int sepol_module_package_set_file_contexts(sepol_module_package_t * p,
						  char *data, size_t len);

extern char *sepol_module_package_get_seusers(sepol_module_package_t * p);

extern size_t sepol_module_package_get_seusers_len(sepol_module_package_t * p);

extern int sepol_module_package_set_seusers(sepol_module_package_t * p,
					    char *data, size_t len);

extern char *sepol_module_package_get_user_extra(sepol_module_package_t * p);

extern size_t sepol_module_package_get_user_extra_len(sepol_module_package_t *
						      p);

extern int sepol_module_package_set_user_extra(sepol_module_package_t * p,
					       char *data, size_t len);

extern char *sepol_module_package_get_netfilter_contexts(sepol_module_package_t
							 * p);

extern size_t
sepol_module_package_get_netfilter_contexts_len(sepol_module_package_t * p);

extern int sepol_module_package_set_netfilter_contexts(sepol_module_package_t *
						       p, char *data,
						       size_t len);

extern sepol_policydb_t *sepol_module_package_get_policy(sepol_module_package_t
							 * p);

extern int sepol_link_packages(sepol_handle_t * handle,
			       sepol_module_package_t * base,
			       sepol_module_package_t ** modules,
			       int num_modules, int verbose);

extern int sepol_module_package_read(sepol_module_package_t * mod,
				     struct sepol_policy_file *file,
				     int verbose);

extern int sepol_module_package_info(struct sepol_policy_file *file,
				     int *type, char **name, char **version);

extern int sepol_module_package_write(sepol_module_package_t * p,
				      struct sepol_policy_file *file);

/* Module linking/expanding public interfaces. */

extern int sepol_link_modules(sepol_handle_t * handle,
			      sepol_policydb_t * base,
			      sepol_policydb_t ** modules,
			      size_t len, int verbose);

extern int sepol_expand_module(sepol_handle_t * handle,
			       sepol_policydb_t * base,
			       sepol_policydb_t * out, int verbose, int check);

#ifdef __cplusplus
}
#endif

#endif
