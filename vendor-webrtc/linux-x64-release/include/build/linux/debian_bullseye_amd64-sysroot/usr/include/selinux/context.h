#ifndef _SELINUX_CONTEXT_H_
#define _SELINUX_CONTEXT_H_

#ifdef __cplusplus
extern "C" {
#endif

/*
 * Functions to deal with security contexts in user space.
 */

	typedef struct {
		void *ptr;
	} context_s_t;

	typedef context_s_t *context_t;

/* Return a new context initialized to a context string */

	extern context_t context_new(const char *);

/* 
 * Return a pointer to the string value of the context_t
 * Valid until the next call to context_str or context_free 
 * for the same context_t*
 */

	extern char *context_str(context_t);

/* Free the storage used by a context */
	extern void context_free(context_t);

/* Get a pointer to the string value of a context component */

	extern const char *context_type_get(context_t);
	extern const char *context_range_get(context_t);
	extern const char *context_role_get(context_t);
	extern const char *context_user_get(context_t);

/* Set a context component.  Returns nonzero if unsuccessful */

	extern int context_type_set(context_t, const char *);
	extern int context_range_set(context_t, const char *);
	extern int context_role_set(context_t, const char *);
	extern int context_user_set(context_t, const char *);

#ifdef __cplusplus
}
#endif
#endif
