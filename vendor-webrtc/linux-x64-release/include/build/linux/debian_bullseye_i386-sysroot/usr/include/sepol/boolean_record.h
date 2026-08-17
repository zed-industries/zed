#ifndef _SEPOL_BOOLEAN_RECORD_H_
#define _SEPOL_BOOLEAN_RECORD_H_

#include <stddef.h>
#include <sepol/handle.h>

#ifdef __cplusplus
extern "C" {
#endif

struct sepol_bool;
struct sepol_bool_key;
typedef struct sepol_bool sepol_bool_t;
typedef struct sepol_bool_key sepol_bool_key_t;

/* Key */
extern int sepol_bool_key_create(sepol_handle_t * handle,
				 const char *name, sepol_bool_key_t ** key);

extern void sepol_bool_key_unpack(const sepol_bool_key_t * key,
				  const char **name);

extern int sepol_bool_key_extract(sepol_handle_t * handle,
				  const sepol_bool_t * boolean,
				  sepol_bool_key_t ** key_ptr);

extern void sepol_bool_key_free(sepol_bool_key_t * key);

extern int sepol_bool_compare(const sepol_bool_t * boolean,
			      const sepol_bool_key_t * key);

extern int sepol_bool_compare2(const sepol_bool_t * boolean,
			       const sepol_bool_t * boolean2);

/* Name */
extern const char *sepol_bool_get_name(const sepol_bool_t * boolean);

extern int sepol_bool_set_name(sepol_handle_t * handle,
			       sepol_bool_t * boolean, const char *name);

/* Value */
extern int sepol_bool_get_value(const sepol_bool_t * boolean);

extern void sepol_bool_set_value(sepol_bool_t * boolean, int value);

/* Create/Clone/Destroy */
extern int sepol_bool_create(sepol_handle_t * handle, sepol_bool_t ** bool_ptr);

extern int sepol_bool_clone(sepol_handle_t * handle,
			    const sepol_bool_t * boolean,
			    sepol_bool_t ** bool_ptr);

extern void sepol_bool_free(sepol_bool_t * boolean);

#ifdef __cplusplus
}
#endif

#endif
