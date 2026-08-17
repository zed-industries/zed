#ifndef _SEPOL_USER_RECORD_H_
#define _SEPOL_USER_RECORD_H_

#include <stddef.h>
#include <sepol/handle.h>

#ifdef __cplusplus
extern "C" {
#endif

struct sepol_user;
struct sepol_user_key;
typedef struct sepol_user sepol_user_t;
typedef struct sepol_user_key sepol_user_key_t;

/* Key */
extern int sepol_user_key_create(sepol_handle_t * handle,
				 const char *name, sepol_user_key_t ** key);

extern void sepol_user_key_unpack(const sepol_user_key_t * key,
				  const char **name);

extern int sepol_user_key_extract(sepol_handle_t * handle,
				  const sepol_user_t * user,
				  sepol_user_key_t ** key_ptr);

extern void sepol_user_key_free(sepol_user_key_t * key);

extern int sepol_user_compare(const sepol_user_t * user,
			      const sepol_user_key_t * key);

extern int sepol_user_compare2(const sepol_user_t * user,
			       const sepol_user_t * user2);

/* Name */
extern const char *sepol_user_get_name(const sepol_user_t * user);

extern int sepol_user_set_name(sepol_handle_t * handle,
			       sepol_user_t * user, const char *name);

/* MLS */
extern const char *sepol_user_get_mlslevel(const sepol_user_t * user);

extern int sepol_user_set_mlslevel(sepol_handle_t * handle,
				   sepol_user_t * user, const char *mls_level);

extern const char *sepol_user_get_mlsrange(const sepol_user_t * user);

extern int sepol_user_set_mlsrange(sepol_handle_t * handle,
				   sepol_user_t * user, const char *mls_range);

/* Role management */
extern int sepol_user_get_num_roles(const sepol_user_t * user);

extern int sepol_user_add_role(sepol_handle_t * handle,
			       sepol_user_t * user, const char *role);

extern void sepol_user_del_role(sepol_user_t * user, const char *role);

extern int sepol_user_has_role(const sepol_user_t * user, const char *role);

extern int sepol_user_get_roles(sepol_handle_t * handle,
				const sepol_user_t * user,
				const char ***roles_arr,
				unsigned int *num_roles);

extern int sepol_user_set_roles(sepol_handle_t * handle,
				sepol_user_t * user,
				const char **roles_arr, unsigned int num_roles);

/* Create/Clone/Destroy */
extern int sepol_user_create(sepol_handle_t * handle, sepol_user_t ** user_ptr);

extern int sepol_user_clone(sepol_handle_t * handle,
			    const sepol_user_t * user,
			    sepol_user_t ** user_ptr);

extern void sepol_user_free(sepol_user_t * user);

#ifdef __cplusplus
}
#endif

#endif
