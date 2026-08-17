#ifndef _SEPOL_IBENDPORT_RECORD_H_
#define _SEPOL_IBENDPORT_RECORD_H_

#include <stddef.h>
#include <sepol/context_record.h>
#include <sepol/handle.h>

#ifdef __cplusplus
extern "C" {
#endif

struct sepol_ibendport;
struct sepol_ibendport_key;
typedef struct sepol_ibendport sepol_ibendport_t;
typedef struct sepol_ibendport_key sepol_ibendport_key_t;

extern int sepol_ibendport_compare(const sepol_ibendport_t *ibendport,
				   const sepol_ibendport_key_t *key);

extern int sepol_ibendport_compare2(const sepol_ibendport_t *ibendport,
				    const sepol_ibendport_t *ibendport2);

extern int sepol_ibendport_key_create(sepol_handle_t *handle,
				      const char *ibdev_name,
				      int port,
				      sepol_ibendport_key_t **key_ptr);

extern void sepol_ibendport_key_unpack(const sepol_ibendport_key_t *key,
				       const char **ibdev_name,
				       int *port);

extern int sepol_ibendport_alloc_ibdev_name(sepol_handle_t *handle,
					    char **ibdev_name);

extern int sepol_ibendport_key_extract(sepol_handle_t *handle,
				       const sepol_ibendport_t *ibendport,
				       sepol_ibendport_key_t **key_ptr);

extern void sepol_ibendport_key_free(sepol_ibendport_key_t *key);

extern void sepol_ibendport_set_port(sepol_ibendport_t *ibendport, int port);

extern int sepol_ibendport_get_port(const sepol_ibendport_t *ibendport);

extern int sepol_ibendport_get_ibdev_name(sepol_handle_t *handle,
					  const sepol_ibendport_t *ibendport,
					  char **ibdev_name);

extern int sepol_ibendport_set_ibdev_name(sepol_handle_t *handle,
					  sepol_ibendport_t *ibendport,
					  const char *ibdev_name);

extern sepol_context_t *sepol_ibendport_get_con(const sepol_ibendport_t *ibendport);

extern int sepol_ibendport_set_con(sepol_handle_t *handle,
				   sepol_ibendport_t *ibendport,
				   sepol_context_t *con);

extern int sepol_ibendport_create(sepol_handle_t *handle,
				  sepol_ibendport_t **ibendport_ptr);

extern int sepol_ibendport_clone(sepol_handle_t *handle,
				 const sepol_ibendport_t *ibendport,
				 sepol_ibendport_t **ibendport_ptr);

extern void sepol_ibendport_free(sepol_ibendport_t *ibendport);

#ifdef __cplusplus
}
#endif

#endif
