#ifndef _SELINUX_GET_SID_LIST_H_
#define _SELINUX_GET_SID_LIST_H_

#include <selinux/selinux.h>

#ifdef __cplusplus
extern "C" {
#endif

#define SELINUX_DEFAULTUSER "user_u"

/* Get an ordered list of authorized security contexts for a user session
   for 'user' spawned by 'fromcon' and set *conary to refer to the 
   NULL-terminated array of contexts.  Every entry in the list will
   be authorized by the policy, but the ordering is subject to user
   customizable preferences.  Returns number of entries in *conary.
   If 'fromcon' is NULL, defaults to current context.
   Caller must free via freeconary. */
	extern int get_ordered_context_list(const char *user,
					    char * fromcon,
					    char *** list);

/* As above, but use the provided MLS level rather than the
   default level for the user. */
	extern int get_ordered_context_list_with_level(const char *user,
						       const char *level,
						       char * fromcon,
						       char *** list);

/* Get the default security context for a user session for 'user'
   spawned by 'fromcon' and set *newcon to refer to it.  The context
   will be one of those authorized by the policy, but the selection
   of a default is subject to user customizable preferences.
   If 'fromcon' is NULL, defaults to current context.
   Returns 0 on success or -1 otherwise.
   Caller must free via freecon. */
	extern int get_default_context(const char *user,
				       char * fromcon,
				       char ** newcon);

/* As above, but use the provided MLS level rather than the
   default level for the user. */
	extern int get_default_context_with_level(const char *user,
						  const char *level,
						  char * fromcon,
						  char ** newcon);

/* Same as get_default_context, but only return a context
   that has the specified role.  If no reachable context exists
   for the user with that role, then return -1. */
	extern int get_default_context_with_role(const char *user,
						 const char *role,
						 char * fromcon,
						 char ** newcon);

/* Same as get_default_context, but only return a context
   that has the specified role and level.  If no reachable context exists
   for the user with that role, then return -1. */
	extern int get_default_context_with_rolelevel(const char *user,
						      const char *role,
						      const char *level,
						      char * fromcon,
						      char ** newcon);

/* Given a list of authorized security contexts for the user, 
   query the user to select one and set *newcon to refer to it.
   Caller must free via freecon.
   Returns 0 on success or -1 otherwise. */
	extern int query_user_context(char ** list,
				      char ** newcon);

/* Allow the user to manually enter a context as a fallback
   if a list of authorized contexts could not be obtained. 
   Caller must free via freecon.
   Returns 0 on success or -1 otherwise. */
	extern int manual_user_enter_context(const char *user,
					     char ** newcon);

#ifdef __cplusplus
}
#endif
#endif
