#ifndef _SEPOL_IFACE_RECORD_H_
#define _SEPOL_IFACE_RECORD_H_

#include <sepol/handle.h>
#include <sepol/context_record.h>

#ifdef __cplusplus
extern "C" {
#endif

struct sepol_iface;
struct sepol_iface_key;
typedef struct sepol_iface sepol_iface_t;
typedef struct sepol_iface_key sepol_iface_key_t;

/* Key */
extern int sepol_iface_compare(const sepol_iface_t * iface,
			       const sepol_iface_key_t * key);

extern int sepol_iface_compare2(const sepol_iface_t * iface,
				const sepol_iface_t * iface2);

extern void sepol_iface_key_unpack(const sepol_iface_key_t * key,
				   const char **name);

extern int sepol_iface_key_create(sepol_handle_t * handle,
				  const char *name,
				  sepol_iface_key_t ** key_ptr);

extern int sepol_iface_key_extract(sepol_handle_t * handle,
				   const sepol_iface_t * iface,
				   sepol_iface_key_t ** key_ptr);

extern void sepol_iface_key_free(sepol_iface_key_t * key);

/* Name */
extern const char *sepol_iface_get_name(const sepol_iface_t * iface);

extern int sepol_iface_set_name(sepol_handle_t * handle,
				sepol_iface_t * iface, const char *name);

/* Context */
extern sepol_context_t *sepol_iface_get_ifcon(const sepol_iface_t * iface);

extern int sepol_iface_set_ifcon(sepol_handle_t * handle,
				 sepol_iface_t * iface, sepol_context_t * con);

extern sepol_context_t *sepol_iface_get_msgcon(const sepol_iface_t * iface);

extern int sepol_iface_set_msgcon(sepol_handle_t * handle,
				  sepol_iface_t * iface, sepol_context_t * con);

/* Create/Clone/Destroy */
extern int sepol_iface_create(sepol_handle_t * handle,
			      sepol_iface_t ** iface_ptr);

extern int sepol_iface_clone(sepol_handle_t * handle,
			     const sepol_iface_t * iface,
			     sepol_iface_t ** iface_ptr);

extern void sepol_iface_free(sepol_iface_t * iface);

#ifdef __cplusplus
}
#endif

#endif
