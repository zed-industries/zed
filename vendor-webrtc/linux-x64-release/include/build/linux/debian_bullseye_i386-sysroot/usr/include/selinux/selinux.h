#ifndef _SELINUX_H_
#define _SELINUX_H_

#include <sys/types.h>
#include <stdarg.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Return 1 if we are running on a SELinux kernel, or 0 otherwise. */
extern int is_selinux_enabled(void);
/* Return 1 if we are running on a SELinux MLS kernel, or 0 otherwise. */
extern int is_selinux_mls_enabled(void);

/* No longer used; here for compatibility with legacy callers. */
typedef char *security_context_t
#ifdef __GNUC__
__attribute__ ((deprecated))
#endif
;

/* Free the memory allocated for a context by any of the below get* calls. */
extern void freecon(char * con);

/* Free the memory allocated for a context array by security_compute_user. */
extern void freeconary(char ** con);

/* Wrappers for the /proc/pid/attr API. */

/* Get current context, and set *con to refer to it.
   Caller must free via freecon. */
extern int getcon(char ** con);
extern int getcon_raw(char ** con);

/* Set the current security context to con.  
   Note that use of this function requires that the entire application
   be trusted to maintain any desired separation between the old and new 
   security contexts, unlike exec-based transitions performed via setexeccon.  
   When possible, decompose your application and use setexeccon()+execve() 
   instead. Note that the application may lose access to its open descriptors
   as a result of a setcon() unless policy allows it to use descriptors opened
   by the old context. */
extern int setcon(const char * con);
extern int setcon_raw(const char * con);

/* Get context of process identified by pid, and 
   set *con to refer to it.  Caller must free via freecon. */
extern int getpidcon(pid_t pid, char ** con);
extern int getpidcon_raw(pid_t pid, char ** con);

/* Get previous context (prior to last exec), and set *con to refer to it.
   Caller must free via freecon. */
extern int getprevcon(char ** con);
extern int getprevcon_raw(char ** con);

/* Get exec context, and set *con to refer to it.
   Sets *con to NULL if no exec context has been set, i.e. using default.
   If non-NULL, caller must free via freecon. */
extern int getexeccon(char ** con);
extern int getexeccon_raw(char ** con);

/* Set exec security context for the next execve. 
   Call with NULL if you want to reset to the default. */
extern int setexeccon(const char * con);
extern int setexeccon_raw(const char * con);

/* Get fscreate context, and set *con to refer to it.
   Sets *con to NULL if no fs create context has been set, i.e. using default.
   If non-NULL, caller must free via freecon. */
extern int getfscreatecon(char ** con);
extern int getfscreatecon_raw(char ** con);

/* Set the fscreate security context for subsequent file creations.
   Call with NULL if you want to reset to the default. */
extern int setfscreatecon(const char * context);
extern int setfscreatecon_raw(const char * context);

/* Get keycreate context, and set *con to refer to it.
   Sets *con to NULL if no key create context has been set, i.e. using default.
   If non-NULL, caller must free via freecon. */
extern int getkeycreatecon(char ** con);
extern int getkeycreatecon_raw(char ** con);

/* Set the keycreate security context for subsequent key creations.
   Call with NULL if you want to reset to the default. */
extern int setkeycreatecon(const char * context);
extern int setkeycreatecon_raw(const char * context);

/* Get sockcreate context, and set *con to refer to it.
   Sets *con to NULL if no socket create context has been set, i.e. using default.
   If non-NULL, caller must free via freecon. */
extern int getsockcreatecon(char ** con);
extern int getsockcreatecon_raw(char ** con);

/* Set the sockcreate security context for subsequent socket creations.
   Call with NULL if you want to reset to the default. */
extern int setsockcreatecon(const char * context);
extern int setsockcreatecon_raw(const char * context);

/* Wrappers for the xattr API. */

/* Get file context, and set *con to refer to it.
   Caller must free via freecon. */
extern int getfilecon(const char *path, char ** con);
extern int getfilecon_raw(const char *path, char ** con);
extern int lgetfilecon(const char *path, char ** con);
extern int lgetfilecon_raw(const char *path, char ** con);
extern int fgetfilecon(int fd, char ** con);
extern int fgetfilecon_raw(int fd, char ** con);

/* Set file context */
extern int setfilecon(const char *path, const char * con);
extern int setfilecon_raw(const char *path, const char * con);
extern int lsetfilecon(const char *path, const char * con);
extern int lsetfilecon_raw(const char *path, const char * con);
extern int fsetfilecon(int fd, const char * con);
extern int fsetfilecon_raw(int fd, const char * con);

/* Wrappers for the socket API */

/* Get context of peer socket, and set *con to refer to it.
   Caller must free via freecon. */
extern int getpeercon(int fd, char ** con);
extern int getpeercon_raw(int fd, char ** con);

/* Wrappers for the selinuxfs (policy) API. */

typedef unsigned int access_vector_t;
typedef unsigned short security_class_t;

struct av_decision {
	access_vector_t allowed;
	access_vector_t decided;
	access_vector_t auditallow;
	access_vector_t auditdeny;
	unsigned int seqno;
	unsigned int flags;
};

/* Definitions of av_decision.flags */
#define SELINUX_AVD_FLAGS_PERMISSIVE	0x0001

/* Structure for passing options, used by AVC and label subsystems */
struct selinux_opt {
	int type;
	const char *value;
};

/* Callback facilities */
union selinux_callback {
	/* log the printf-style format and arguments,
	   with the type code indicating the type of message */
	int 
#ifdef __GNUC__
__attribute__ ((format(printf, 2, 3)))
#endif
	(*func_log) (int type, const char *fmt, ...);
	/* store a string representation of auditdata (corresponding
	   to the given security class) into msgbuf. */
	int (*func_audit) (void *auditdata, security_class_t cls,
			   char *msgbuf, size_t msgbufsize);
	/* validate the supplied context, modifying if necessary */
	int (*func_validate) (char **ctx);
	/* netlink callback for setenforce message */
	int (*func_setenforce) (int enforcing);
	/* netlink callback for policyload message */
	int (*func_policyload) (int seqno);
};

#define SELINUX_CB_LOG		0
#define SELINUX_CB_AUDIT	1
#define SELINUX_CB_VALIDATE	2
#define SELINUX_CB_SETENFORCE	3
#define SELINUX_CB_POLICYLOAD	4

extern union selinux_callback selinux_get_callback(int type);
extern void selinux_set_callback(int type, union selinux_callback cb);

	/* Logging type codes, passed to the logging callback */
#define SELINUX_ERROR	        0
#define SELINUX_WARNING		1
#define SELINUX_INFO		2
#define SELINUX_AVC		3
#define SELINUX_TRANS_DIR	"/var/run/setrans"

/* Compute an access decision. */
extern int security_compute_av(const char * scon,
			       const char * tcon,
			       security_class_t tclass,
			       access_vector_t requested,
			       struct av_decision *avd);
extern int security_compute_av_raw(const char * scon,
				   const char * tcon,
				   security_class_t tclass,
				   access_vector_t requested,
				   struct av_decision *avd);

extern int security_compute_av_flags(const char * scon,
				     const char * tcon,
				     security_class_t tclass,
				     access_vector_t requested,
				     struct av_decision *avd);
extern int security_compute_av_flags_raw(const char * scon,
					 const char * tcon,
					 security_class_t tclass,
					 access_vector_t requested,
					 struct av_decision *avd);

/* Compute a labeling decision and set *newcon to refer to it.
   Caller must free via freecon. */
extern int security_compute_create(const char * scon,
				   const char * tcon,
				   security_class_t tclass,
				   char ** newcon);
extern int security_compute_create_raw(const char * scon,
				       const char * tcon,
				       security_class_t tclass,
				       char ** newcon);
extern int security_compute_create_name(const char * scon,
					const char * tcon,
					security_class_t tclass,
					const char *objname,
					char ** newcon);
extern int security_compute_create_name_raw(const char * scon,
					    const char * tcon,
					    security_class_t tclass,
					    const char *objname,
					    char ** newcon);

/* Compute a relabeling decision and set *newcon to refer to it.
   Caller must free via freecon. */
extern int security_compute_relabel(const char * scon,
				    const char * tcon,
				    security_class_t tclass,
				    char ** newcon);
extern int security_compute_relabel_raw(const char * scon,
					const char * tcon,
					security_class_t tclass,
					char ** newcon);

/* Compute a polyinstantiation member decision and set *newcon to refer to it.
   Caller must free via freecon. */
extern int security_compute_member(const char * scon,
				   const char * tcon,
				   security_class_t tclass,
				   char ** newcon);
extern int security_compute_member_raw(const char * scon,
				       const char * tcon,
				       security_class_t tclass,
				       char ** newcon);

/*
 * Compute the set of reachable user contexts and set *con to refer to
 * the NULL-terminated array of contexts.  Caller must free via freeconary.
 * These interfaces are deprecated.  Use get_ordered_context_list() or
 * one of its variant interfaces instead.
 */
extern int security_compute_user(const char * scon,
				 const char *username,
				 char *** con);
extern int security_compute_user_raw(const char * scon,
				     const char *username,
				     char *** con);

/* Validate a transition. This determines whether a transition from scon to newcon
   using tcon as the target for object class tclass is valid in the loaded policy.
   This checks against the mlsvalidatetrans and validatetrans constraints in the loaded policy.
   Returns 0 if allowed and -1 if an error occurred with errno set */
extern int security_validatetrans(const char *scon,
				  const char *tcon,
				  security_class_t tclass,
				  const char *newcon);
extern int security_validatetrans_raw(const char *scon,
				      const char *tcon,
				      security_class_t tclass,
				      const char *newcon);

/* Load a policy configuration. */
extern int security_load_policy(void *data, size_t len);

/* Get the context of an initial kernel security identifier by name.  
   Caller must free via freecon */
extern int security_get_initial_context(const char *name,
					char ** con);
extern int security_get_initial_context_raw(const char *name,
					    char ** con);

/*
 * Make a policy image and load it.
 * This function provides a higher level interface for loading policy
 * than security_load_policy, internally determining the right policy
 * version, locating and opening the policy file, mapping it into memory,
 * manipulating it as needed for current boolean settings and/or local 
 * definitions, and then calling security_load_policy to load it.
 *
 * 'preservebools' is no longer supported, set to 0.
 */
extern int selinux_mkload_policy(int preservebools);

/* 
 * Perform the initial policy load.
 * This function determines the desired enforcing mode, sets the
 * the *enforce argument accordingly for the caller to use, sets the 
 * SELinux kernel enforcing status to match it, and loads the policy.
 * It also internally handles the initial selinuxfs mount required to
 * perform these actions.
 *
 * The function returns 0 if everything including the policy load succeeds.
 * In this case, init is expected to re-exec itself in order to transition
 * to the proper security context.
 * Otherwise, the function returns -1, and init must check *enforce to
 * determine how to proceed.  If enforcing (*enforce > 0), then init should
 * halt the system.  Otherwise, init may proceed normally without a re-exec.
 */
extern int selinux_init_load_policy(int *enforce);

/* Translate boolean strict to name value pair. */
typedef struct {
	char *name;
	int value;
} SELboolean;
/* save a list of booleans in a single transaction. 'permanent' is no
 * longer supported, set to 0.
 */
extern int security_set_boolean_list(size_t boolcnt,
				     SELboolean * boollist, int permanent);

/* Load policy boolean settings. Deprecated as local policy booleans no
 * longer supported. Will always return -1.
 */
extern int security_load_booleans(char *path)
#ifdef __GNUC__
__attribute__ ((deprecated))
#endif
;

/* Check the validity of a security context. */
extern int security_check_context(const char * con);
extern int security_check_context_raw(const char * con);

/* Canonicalize a security context. */
extern int security_canonicalize_context(const char * con,
					 char ** canoncon);
extern int security_canonicalize_context_raw(const char * con,
					     char ** canoncon);

/* Get the enforce flag value. */
extern int security_getenforce(void);

/* Set the enforce flag value. */
extern int security_setenforce(int value);

/* Get the load-time behavior for undefined classes/permissions */
extern int security_reject_unknown(void);

/* Get the runtime behavior for undefined classes/permissions */
extern int security_deny_unknown(void);

/* Get the checkreqprot value */
extern int security_get_checkreqprot(void);

/* Disable SELinux at runtime (must be done prior to initial policy load). */
extern int security_disable(void);

/* Get the policy version number. */
extern int security_policyvers(void);

/* Get the boolean names */
extern int security_get_boolean_names(char ***names, int *len);

/* Get the pending value for the boolean */
extern int security_get_boolean_pending(const char *name);

/* Get the active value for the boolean */
extern int security_get_boolean_active(const char *name);

/* Set the pending value for the boolean */
extern int security_set_boolean(const char *name, int value);

/* Commit the pending values for the booleans */
extern int security_commit_booleans(void);

/* Userspace class mapping support */
struct security_class_mapping {
	const char *name;
	const char *perms[sizeof(access_vector_t) * 8 + 1];
};

/**
 * selinux_set_mapping - Enable dynamic mapping between integer offsets and security class names
 * @map: array of security_class_mapping structures
 *
 * The core avc_has_perm() API uses integers to represent security
 * classes; previous to the introduction of this function, it was
 * common for userspace object managers to be compiled using generated
 * offsets for a particular policy.  However, that strongly ties the build of the userspace components to a particular policy.
 *
 * By using this function to map between integer offsets and security
 * class names, it's possible to replace a system policies that have
 * at least the same set of security class names as used by the
 * userspace object managers.
 *
 * To correctly use this function, you should override the generated
 * security class defines from the system policy in a local header,
 * starting at 1, and have one security_class_mapping structure entry
 * per define.
 */
extern int selinux_set_mapping(struct security_class_mapping *map);

/* Common helpers */

/* Convert between mode and security class values */
extern security_class_t mode_to_security_class(mode_t mode);
/* Convert between security class values and string names */
extern security_class_t string_to_security_class(const char *name);
extern const char *security_class_to_string(security_class_t cls);

/* Convert between individual access vector permissions and string names */
extern const char *security_av_perm_to_string(security_class_t tclass,
					      access_vector_t perm);
extern access_vector_t string_to_av_perm(security_class_t tclass,
					 const char *name);

/* Returns an access vector in a string representation.  User must free the
 * returned string via free(). */
extern int security_av_string(security_class_t tclass,
			      access_vector_t av, char **result);

/* Display an access vector in a string representation. */
extern void print_access_vector(security_class_t tclass, access_vector_t av);

/* Flush the SELinux class cache, e.g. upon a policy reload. */
extern void selinux_flush_class_cache(void);

/* Set the function used by matchpathcon_init when displaying
   errors about the file_contexts configuration.  If not set,
   then this defaults to fprintf(stderr, fmt, ...). */
extern void set_matchpathcon_printf(void (*f) (const char *fmt, ...));

/* Set the function used by matchpathcon_init when checking the
   validity of a context in the file contexts configuration.  If not set,
   then this defaults to a test based on security_check_context().  
   The function is also responsible for reporting any such error, and
   may include the 'path' and 'lineno' in such error messages. */
extern void set_matchpathcon_invalidcon(int (*f) (const char *path,
						  unsigned lineno,
						  char *context));

/* Same as above, but also allows canonicalization of the context,
   by changing *context to refer to the canonical form.  If not set,
   and invalidcon is also not set, then this defaults to calling
   security_canonicalize_context(). */
extern void set_matchpathcon_canoncon(int (*f) (const char *path,
						unsigned lineno,
						char **context));

/* Set flags controlling operation of matchpathcon_init or matchpathcon. */
#define MATCHPATHCON_BASEONLY 1	/* Only process the base file_contexts file. */
#define MATCHPATHCON_NOTRANS  2	/* Do not perform any context translation. */
#define MATCHPATHCON_VALIDATE 4	/* Validate/canonicalize contexts at init time. */
extern void set_matchpathcon_flags(unsigned int flags);

/* Load the file contexts configuration specified by 'path'
   into memory for use by subsequent matchpathcon calls.  
   If 'path' is NULL, then load the active file contexts configuration,
   i.e. the path returned by selinux_file_context_path().
   Unless the MATCHPATHCON_BASEONLY flag has been set, this
   function also checks for a 'path'.homedirs file and 
   a 'path'.local file and loads additional specifications 
   from them if present. */
extern int matchpathcon_init(const char *path)
#ifdef __GNUC__
   __attribute__ ((deprecated("Use selabel_open with backend SELABEL_CTX_FILE")))
#endif
;

/* Same as matchpathcon_init, but only load entries with
   regexes that have stems that are prefixes of 'prefix'. */
extern int matchpathcon_init_prefix(const char *path, const char *prefix);

/* Free the memory allocated by matchpathcon_init. */
extern void matchpathcon_fini(void)
#ifdef __GNUC__
   __attribute__ ((deprecated("Use selabel_close")))
#endif
;

/* Resolve all of the symlinks and relative portions of a pathname, but NOT
 * the final component (same a realpath() unless the final component is a
 * symlink.  Resolved path must be a path of size PATH_MAX + 1 */
extern int realpath_not_final(const char *name, char *resolved_path);

/* Match the specified pathname and mode against the file contexts
   configuration and set *con to refer to the resulting context.
   'mode' can be 0 to disable mode matching.
   Caller must free via freecon.
   If matchpathcon_init has not already been called, then this function
   will call it upon its first invocation with a NULL path. */
extern int matchpathcon(const char *path,
			mode_t mode, char ** con)
#ifdef __GNUC__
	__attribute__ ((deprecated("Use selabel_lookup instead")))
#endif
;

/* Same as above, but return a specification index for 
   later use in a matchpathcon_filespec_add() call - see below. */
extern int matchpathcon_index(const char *path,
			      mode_t mode, char ** con);

/* Maintain an association between an inode and a specification index,
   and check whether a conflicting specification is already associated
   with the same inode (e.g. due to multiple hard links).  If so, then
   use the latter of the two specifications based on their order in the 
   file contexts configuration.  Return the used specification index. */
extern int matchpathcon_filespec_add(ino_t ino, int specind, const char *file);

/* Destroy any inode associations that have been added, e.g. to restart
   for a new filesystem. */
extern void matchpathcon_filespec_destroy(void);

/* Display statistics on the hash table usage for the associations. */
extern void matchpathcon_filespec_eval(void);

/* Check to see whether any specifications had no matches and report them.
   The 'str' is used as a prefix for any warning messages. */
extern void matchpathcon_checkmatches(char *str);

/* Match the specified media and against the media contexts 
   configuration and set *con to refer to the resulting context.
   Caller must free con via freecon. */
extern int matchmediacon(const char *media, char ** con);

/*
  selinux_getenforcemode reads the /etc/selinux/config file and determines 
  whether the machine should be started in enforcing (1), permissive (0) or 
  disabled (-1) mode.
 */
extern int selinux_getenforcemode(int *enforce);

/*
  selinux_boolean_sub reads the /etc/selinux/TYPE/booleans.subs_dist file
  looking for a record with boolean_name.  If a record exists selinux_boolean_sub
  returns the translated name otherwise it returns the original name.
  The returned value needs to be freed. On failure NULL will be returned.
 */
extern char *selinux_boolean_sub(const char *boolean_name);

/*
  selinux_getpolicytype reads the /etc/selinux/config file and determines 
  what the default policy for the machine is.  Calling application must 
  free policytype.
 */
extern int selinux_getpolicytype(char **policytype);

/*
  selinux_policy_root reads the /etc/selinux/config file and returns 
  the directory path under which the compiled policy file and context 
  configuration files exist.
 */
extern const char *selinux_policy_root(void);

/*
  selinux_set_policy_root sets an alternate policy root directory path under
  which the compiled policy file and context configuration files exist.
 */
extern int selinux_set_policy_root(const char *rootpath);

/* These functions return the paths to specific files under the 
   policy root directory. */
extern const char *selinux_current_policy_path(void);
extern const char *selinux_binary_policy_path(void);
extern const char *selinux_failsafe_context_path(void);
extern const char *selinux_removable_context_path(void);
extern const char *selinux_default_context_path(void);
extern const char *selinux_user_contexts_path(void);
extern const char *selinux_file_context_path(void);
extern const char *selinux_file_context_homedir_path(void);
extern const char *selinux_file_context_local_path(void);
extern const char *selinux_file_context_subs_path(void);
extern const char *selinux_file_context_subs_dist_path(void);
extern const char *selinux_homedir_context_path(void);
extern const char *selinux_media_context_path(void);
extern const char *selinux_virtual_domain_context_path(void);
extern const char *selinux_virtual_image_context_path(void);
extern const char *selinux_lxc_contexts_path(void);
extern const char *selinux_x_context_path(void);
extern const char *selinux_sepgsql_context_path(void);
extern const char *selinux_openrc_contexts_path(void);
extern const char *selinux_openssh_contexts_path(void);
extern const char *selinux_snapperd_contexts_path(void);
extern const char *selinux_systemd_contexts_path(void);
extern const char *selinux_contexts_path(void);
extern const char *selinux_securetty_types_path(void);
extern const char *selinux_booleans_subs_path(void);
/* Deprecated as local policy booleans no longer supported. */
extern const char *selinux_booleans_path(void)
#ifdef __GNUC__
__attribute__ ((deprecated))
#endif
;
extern const char *selinux_customizable_types_path(void);
/* Deprecated as policy ./users no longer supported. */
extern const char *selinux_users_path(void)
#ifdef __GNUC__
__attribute__ ((deprecated))
#endif
;
extern const char *selinux_usersconf_path(void);
extern const char *selinux_translations_path(void);
extern const char *selinux_colors_path(void);
extern const char *selinux_netfilter_context_path(void);
extern const char *selinux_path(void);

/**
 * selinux_check_access - Check permissions and perform appropriate auditing.
 * @scon: source security context
 * @tcon: target security context
 * @tclass: target security class string
 * @perm: requested permissions string, interpreted based on @tclass
 * @auditdata: auxiliary audit data
 *
 * Check the AVC to determine whether the @perm permissions are granted
 * for the SID pair (@scon, @tcon), interpreting the permissions
 * based on @tclass.
 * Return %0 if all @perm permissions are granted, -%1 with 
 * @errno set to %EACCES if any permissions are denied or to another 
 * value upon other errors.
 * If auditing or logging is configured the appropriate callbacks will be called
 * and passed the auditdata field
 */
extern int selinux_check_access(const char * scon, const char * tcon, const char *tclass, const char *perm, void *auditdata);

/* Check a permission in the passwd class.
   Return 0 if granted or -1 otherwise. */
extern int selinux_check_passwd_access(access_vector_t requested)
#ifdef __GNUC__
  __attribute__ ((deprecated("Use selinux_check_access")))
#endif
;

extern int checkPasswdAccess(access_vector_t requested)
#ifdef __GNUC__
   __attribute__ ((deprecated("Use selinux_check_access")))
#endif
;

/* Check if the tty_context is defined as a securetty
   Return 0 if secure, < 0 otherwise. */
extern int selinux_check_securetty_context(const char * tty_context);

/* Set the path to the selinuxfs mount point explicitly.
   Normally, this is determined automatically during libselinux 
   initialization, but this is not always possible, e.g. for /sbin/init
   which performs the initial mount of selinuxfs. */
extern void set_selinuxmnt(const char *mnt);

/* Check if selinuxfs exists as a kernel filesystem */
extern int selinuxfs_exists(void);

/* clear selinuxmnt variable and free allocated memory */
extern void fini_selinuxmnt(void);

/* Set an appropriate security context based on the filename of a helper
 * program, falling back to a new context with the specified type. */
extern int setexecfilecon(const char *filename, const char *fallback_type);

#ifndef DISABLE_RPM
/* Execute a helper for rpm in an appropriate security context. */
extern int rpm_execcon(unsigned int verified,
		       const char *filename,
		       char *const argv[], char *const envp[])
#ifdef __GNUC__
	__attribute__((deprecated("Use setexecfilecon and execve")))
#endif
;
#endif

/* Returns whether a file context is customizable, and should not 
   be relabeled . */
extern int is_context_customizable(const char * scontext);

/* Perform context translation between the human-readable format
   ("translated") and the internal system format ("raw"). 
   Caller must free the resulting context via freecon.  
   Returns -1 upon an error or 0 otherwise.
   If passed NULL, sets the returned context to NULL and returns 0. */
extern int selinux_trans_to_raw_context(const char * trans,
					char ** rawp);
extern int selinux_raw_to_trans_context(const char * raw,
					char ** transp);

/* Perform context translation between security contexts
   and display colors.  Returns a space-separated list of ten
   ten hex RGB triples prefixed by hash marks, e.g. "#ff0000".
   Caller must free the resulting string via free.
   Returns -1 upon an error or 0 otherwise. */
extern int selinux_raw_context_to_color(const char * raw,
					char **color_str);

/* Get the SELinux username and level to use for a given Linux username. 
   These values may then be passed into the get_ordered_context_list*
   and get_default_context* functions to obtain a context for the user.
   Returns 0 on success or -1 otherwise.
   Caller must free the returned strings via free. */
extern int getseuserbyname(const char *linuxuser, char **seuser, char **level);

/* Get the SELinux username and level to use for a given Linux username and service. 
   These values may then be passed into the get_ordered_context_list*
   and get_default_context* functions to obtain a context for the user.
   Returns 0 on success or -1 otherwise.
   Caller must free the returned strings via free. */
extern int getseuser(const char *username, const char *service, 
		     char **r_seuser, char **r_level);

/* Compare two file contexts, return 0 if equivalent. */
extern int selinux_file_context_cmp(const char * a,
			     const char * b);

/* 
 * Verify the context of the file 'path' against policy.
 * Return 1 if match, 0 if not and -1 on error.
 */
extern int selinux_file_context_verify(const char *path, mode_t mode);

/* This function sets the file context on to the system defaults returns 0 on success */
extern int selinux_lsetfilecon_default(const char *path);

/* 
 * Force a reset of the loaded configuration
 * WARNING: This is not thread safe. Be very sure that no other threads
 * are calling into libselinux when this is called.
 */
extern void selinux_reset_config(void);

#ifdef __cplusplus
}
#endif
#endif
