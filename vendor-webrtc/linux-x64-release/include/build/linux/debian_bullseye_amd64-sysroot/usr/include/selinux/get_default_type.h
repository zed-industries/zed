/* get_default_type.h - contains header information and function prototypes
 *                  for functions to get the default type for a role
 */

#ifndef _SELINUX_GET_DEFAULT_TYPE_H_
#define _SELINUX_GET_DEFAULT_TYPE_H_

#ifdef __cplusplus
extern "C" {
#endif

/* Return path to default type file. */
	extern const char *selinux_default_type_path(void);

/* Get the default type (domain) for 'role' and set 'type' to refer to it.
   Caller must free via free().
   Return 0 on success or -1 otherwise. */
	extern int get_default_type(const char *role, char **type);

#ifdef __cplusplus
}
#endif
#endif				/* ifndef _GET_DEFAULT_TYPE_H_ */
