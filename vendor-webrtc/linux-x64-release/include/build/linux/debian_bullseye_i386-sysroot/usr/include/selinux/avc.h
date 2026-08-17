/*
 * Access vector cache interface for object managers.
 *
 * Author : Eamon Walsh <ewalsh@epoch.ncsc.mil>
 */
#ifndef _SELINUX_AVC_H_
#define _SELINUX_AVC_H_

#include <stdint.h>
#include <errno.h>
#include <stdlib.h>
#include <selinux/selinux.h>

#ifdef __cplusplus
extern "C" {
#endif

/*
 * SID format and operations
 */
struct security_id {
	char * ctx;
	unsigned int refcnt;
};
typedef struct security_id *security_id_t;

#define SECSID_WILD (security_id_t)NULL	/* unspecified SID */

/**
 * avc_sid_to_context - get copy of context corresponding to SID.
 * @sid: input SID
 * @ctx: pointer to context reference
 *
 * Return a copy of the security context corresponding to the input
 * @sid in the memory referenced by @ctx.  The caller is expected to 
 * free the context with freecon().  Return %0 on success, -%1 on
 * failure, with @errno set to %ENOMEM if insufficient memory was
 * available to make the copy, or %EINVAL if the input SID is invalid.
 */
extern int avc_sid_to_context(security_id_t sid, char ** ctx);
extern int avc_sid_to_context_raw(security_id_t sid, char ** ctx);

/**
 * avc_context_to_sid - get SID for context.
 * @ctx: input security context
 * @sid: pointer to SID reference
 *
 * Look up security context @ctx in SID table, making
 * a new entry if @ctx is not found.  Increment the
 * reference counter for the SID.  Store a pointer
 * to the SID structure into the memory referenced by @sid, 
 * returning %0 on success or -%1 on error with @errno set.  
 */
extern int avc_context_to_sid(const char * ctx, security_id_t * sid);
extern int avc_context_to_sid_raw(const char * ctx, security_id_t * sid);

/**
 * sidget - increment SID reference counter.
 * @sid: SID reference
 *
 * Increment the reference counter for @sid, indicating that
 * @sid is in use by an (additional) object.  Return the
 * new reference count, or zero if @sid is invalid (has zero
 * reference count).  Note that avc_context_to_sid() also
 * increments reference counts.
 */
extern int sidget(security_id_t sid)
#ifdef __GNUC__
__attribute__ ((deprecated))
#endif
;

/**
 * sidput - decrement SID reference counter.
 * @sid: SID reference
 *
 * Decrement the reference counter for @sid, indicating that
 * a reference to @sid is no longer in use.  Return the 
 * new reference count.  When the reference count reaches
 * zero, the SID is invalid, and avc_context_to_sid() must
 * be called to obtain a new SID for the security context.
 */
extern int sidput(security_id_t sid)
#ifdef __GNUC__
__attribute__ ((deprecated))
#endif
;

/**
 * avc_get_initial_sid - get SID for an initial kernel security identifier
 * @name: input name of initial kernel security identifier
 * @sid: pointer to a SID reference
 *
 * Get the context for an initial kernel security identifier specified by 
 * @name using security_get_initial_context() and then call 
 * avc_context_to_sid() to get the corresponding SID.
 */
extern int avc_get_initial_sid(const char *name, security_id_t * sid);

/*
 * AVC entry
 */
struct avc_entry;
struct avc_entry_ref {
	struct avc_entry *ae;
};

/**
 * avc_entry_ref_init - initialize an AVC entry reference.
 * @aeref: pointer to avc entry reference structure
 *
 * Use this macro to initialize an avc entry reference structure
 * before first use.  These structures are passed to avc_has_perm(),
 * which stores cache entry references in them.  They can increase
 * performance on repeated queries.
 */
#define avc_entry_ref_init(aeref) ((aeref)->ae = NULL)

/*
 * User-provided callbacks for memory, auditing, and locking
 */

/* These structures are passed by reference to avc_init().  Passing
 * a NULL reference will cause the AVC to use a default.  The default
 * memory callbacks are malloc() and free().  The default logging method
 * is to print on stderr.  If no thread callbacks are passed, a separate
 * listening thread won't be started for kernel policy change messages.
 * If no locking callbacks are passed, no locking will take place.
 */
struct avc_memory_callback {
	/* malloc() equivalent. */
	void *(*func_malloc) (size_t size);
	/* free() equivalent. */
	void (*func_free) (void *ptr);
	/* Note that these functions should set errno on failure.
	   If not, some avc routines may return -1 without errno set. */
};

struct avc_log_callback {
	/* log the printf-style format and arguments. */
	void
#ifdef __GNUC__
__attribute__ ((format(printf, 1, 2)))
#endif
	(*func_log) (const char *fmt, ...);
	/* store a string representation of auditdata (corresponding
	   to the given security class) into msgbuf. */
	void (*func_audit) (void *auditdata, security_class_t cls,
			    char *msgbuf, size_t msgbufsize);
};

struct avc_thread_callback {
	/* create and start a thread, returning an opaque pointer to it; 
	   the thread should run the given function. */
	void *(*func_create_thread) (void (*run) (void));
	/* cancel a given thread and free its resources. */
	void (*func_stop_thread) (void *thread);
};

struct avc_lock_callback {
	/* create a lock and return an opaque pointer to it. */
	void *(*func_alloc_lock) (void);
	/* obtain a given lock, blocking if necessary. */
	void (*func_get_lock) (void *lock);
	/* release a given lock. */
	void (*func_release_lock) (void *lock);
	/* destroy a given lock (free memory, etc.) */
	void (*func_free_lock) (void *lock);
};

/*
 * Available options
 */

/* no-op option, useful for unused slots in an array of options */
#define AVC_OPT_UNUSED		0
/* override kernel enforcing mode (boolean value) */
#define AVC_OPT_SETENFORCE	1

/*
 * AVC operations
 */

/**
 * avc_init - Initialize the AVC.
 * @msgprefix: prefix for log messages
 * @mem_callbacks: user-supplied memory callbacks
 * @log_callbacks: user-supplied logging callbacks
 * @thread_callbacks: user-supplied threading callbacks
 * @lock_callbacks: user-supplied locking callbacks
 *
 * Initialize the access vector cache.  Return %0 on
 * success or -%1 with @errno set on failure.  
 * If @msgprefix is NULL, use "uavc".  If any callback 
 * structure references are NULL, use default methods 
 * for those callbacks (see the definition of the callback
 * structures above).
 */
extern int avc_init(const char *msgprefix,
		    const struct avc_memory_callback *mem_callbacks,
		    const struct avc_log_callback *log_callbacks,
		    const struct avc_thread_callback *thread_callbacks,
		    const struct avc_lock_callback *lock_callbacks)
#ifdef __GNUC__
	__attribute__ ((deprecated("Use avc_open and selinux_set_callback")))
#endif
;

/**
 * avc_open - Initialize the AVC.
 * @opts: array of selabel_opt structures specifying AVC options or NULL.
 * @nopts: number of elements in opts array or zero for no options.
 *
 * This function is identical to avc_init(), except the message prefix
 * is set to "avc" and any callbacks desired should be specified via
 * selinux_set_callback().  Available options are listed above.
 */
extern int avc_open(struct selinux_opt *opts, unsigned nopts);

/**
 * avc_cleanup - Remove unused SIDs and AVC entries.
 *
 * Search the SID table for SID structures with zero
 * reference counts, and remove them along with all
 * AVC entries that reference them.  This can be used
 * to return memory to the system.
 */
extern void avc_cleanup(void);

/**
 * avc_reset - Flush the cache and reset statistics.
 *
 * Remove all entries from the cache and reset all access
 * statistics (as returned by avc_cache_stats()) to zero.
 * The SID mapping is not affected.  Return %0 on success, 
 * -%1 with @errno set on error.
 */
extern int avc_reset(void);

/**
 * avc_destroy - Free all AVC structures.
 *
 * Destroy all AVC structures and free all allocated
 * memory.  User-supplied locking, memory, and audit
 * callbacks will be retained, but security-event
 * callbacks will not.  All SID's will be invalidated.
 * User must call avc_init() if further use of AVC is desired.
 */
extern void avc_destroy(void);

/**
 * avc_has_perm_noaudit - Check permissions but perform no auditing.
 * @ssid: source security identifier
 * @tsid: target security identifier
 * @tclass: target security class
 * @requested: requested permissions, interpreted based on @tclass
 * @aeref:  AVC entry reference
 * @avd: access vector decisions
 *
 * Check the AVC to determine whether the @requested permissions are granted
 * for the SID pair (@ssid, @tsid), interpreting the permissions
 * based on @tclass, and call the security server on a cache miss to obtain
 * a new decision and add it to the cache.  Update @aeref to refer to an AVC
 * entry with the resulting decisions, and return a copy of the decisions
 * in @avd.  Return %0 if all @requested permissions are granted, -%1 with
 * @errno set to %EACCES if any permissions are denied, or to another value
 * upon other errors.  This function is typically called by avc_has_perm(),
 * but may also be called directly to separate permission checking from
 * auditing, e.g. in cases where a lock must be held for the check but
 * should be released for the auditing.
 */
extern int avc_has_perm_noaudit(security_id_t ssid,
				security_id_t tsid,
				security_class_t tclass,
				access_vector_t requested,
				struct avc_entry_ref *aeref, struct av_decision *avd);

/**
 * avc_has_perm - Check permissions and perform any appropriate auditing.
 * @ssid: source security identifier
 * @tsid: target security identifier
 * @tclass: target security class
 * @requested: requested permissions, interpreted based on @tclass
 * @aeref:  AVC entry reference
 * @auditdata: auxiliary audit data
 *
 * Check the AVC to determine whether the @requested permissions are granted
 * for the SID pair (@ssid, @tsid), interpreting the permissions
 * based on @tclass, and call the security server on a cache miss to obtain
 * a new decision and add it to the cache.  Update @aeref to refer to an AVC
 * entry with the resulting decisions.  Audit the granting or denial of
 * permissions in accordance with the policy.  Return %0 if all @requested
 * permissions are granted, -%1 with @errno set to %EACCES if any permissions
 * are denied or to another value upon other errors.
 */
extern int avc_has_perm(security_id_t ssid, security_id_t tsid,
			security_class_t tclass, access_vector_t requested,
			struct avc_entry_ref *aeref, void *auditdata);

/**
 * avc_audit - Audit the granting or denial of permissions.
 * @ssid: source security identifier
 * @tsid: target security identifier
 * @tclass: target security class
 * @requested: requested permissions
 * @avd: access vector decisions
 * @result: result from avc_has_perm_noaudit
 * @auditdata:  auxiliary audit data
 *
 * Audit the granting or denial of permissions in accordance
 * with the policy.  This function is typically called by
 * avc_has_perm() after a permission check, but can also be
 * called directly by callers who use avc_has_perm_noaudit()
 * in order to separate the permission check from the auditing.
 * For example, this separation is useful when the permission check must
 * be performed under a lock, to allow the lock to be released
 * before calling the auditing code.
 */
extern void avc_audit(security_id_t ssid, security_id_t tsid,
		      security_class_t tclass, access_vector_t requested,
		      struct av_decision *avd, int result, void *auditdata);

/**
 * avc_compute_create - Compute SID for labeling a new object.
 * @ssid: source security identifier
 * @tsid: target security identifier
 * @tclass: target security class
 * @newsid: pointer to SID reference
 *
 * Call the security server to obtain a context for labeling a
 * new object.  Look up the context in the SID table, making
 * a new entry if not found.  Increment the reference counter
 * for the SID.  Store a pointer to the SID structure into the
 * memory referenced by @newsid, returning %0 on success or -%1 on
 * error with @errno set.  
 */
extern int avc_compute_create(security_id_t ssid,
			      security_id_t tsid,
			      security_class_t tclass, security_id_t * newsid);

/**
 * avc_compute_member - Compute SID for polyinstantation.
 * @ssid: source security identifier
 * @tsid: target security identifier
 * @tclass: target security class
 * @newsid: pointer to SID reference
 *
 * Call the security server to obtain a context for labeling an
 * object instance.  Look up the context in the SID table, making
 * a new entry if not found.  Increment the reference counter
 * for the SID.  Store a pointer to the SID structure into the
 * memory referenced by @newsid, returning %0 on success or -%1 on
 * error with @errno set.  
 */
extern int avc_compute_member(security_id_t ssid,
			      security_id_t tsid,
			      security_class_t tclass, security_id_t * newsid);

/* 
 * security event callback facility
 */

/* security events */
#define AVC_CALLBACK_GRANT		1
#define AVC_CALLBACK_TRY_REVOKE		2
#define AVC_CALLBACK_REVOKE		4
#define AVC_CALLBACK_RESET		8
#define AVC_CALLBACK_AUDITALLOW_ENABLE	16
#define AVC_CALLBACK_AUDITALLOW_DISABLE	32
#define AVC_CALLBACK_AUDITDENY_ENABLE	64
#define AVC_CALLBACK_AUDITDENY_DISABLE	128

/**
 * avc_add_callback - Register a callback for security events.
 * @callback: callback function
 * @events: bitwise OR of desired security events
 * @ssid: source security identifier or %SECSID_WILD
 * @tsid: target security identifier or %SECSID_WILD
 * @tclass: target security class
 * @perms: permissions
 *
 * Register a callback function for events in the set @events
 * related to the SID pair (@ssid, @tsid) and
 * and the permissions @perms, interpreting
 * @perms based on @tclass.  Returns %0 on success or
 * -%1 if insufficient memory exists to add the callback.
 */
extern int avc_add_callback(int (*callback)
			     (uint32_t event, security_id_t ssid,
			      security_id_t tsid, security_class_t tclass,
			      access_vector_t perms,
			      access_vector_t * out_retained),
			    uint32_t events, security_id_t ssid,
			    security_id_t tsid, security_class_t tclass,
			    access_vector_t perms);

/*
 * AVC statistics 
 */

/* If set, cache statistics are tracked.  This may
 * become a compile-time option in the future.
 */
#define AVC_CACHE_STATS     1

struct avc_cache_stats {
	unsigned entry_lookups;
	unsigned entry_hits;
	unsigned entry_misses;
	unsigned entry_discards;
	unsigned cav_lookups;
	unsigned cav_hits;
	unsigned cav_probes;
	unsigned cav_misses;
};

/**
 * avc_cache_stats - get cache access statistics.
 * @stats: reference to statistics structure
 *
 * Fill the supplied structure with information about AVC 
 * activity since the last call to avc_init() or
 * avc_reset().  See the structure definition for
 * details.
 */
extern void avc_cache_stats(struct avc_cache_stats *stats);

/**
 * avc_av_stats - log av table statistics.
 *
 * Log a message with information about the size and
 * distribution of the access vector table.  The audit
 * callback is used to print the message.
 */
extern void avc_av_stats(void);

/**
 * avc_sid_stats - log SID table statistics.
 *
 * Log a message with information about the size and
 * distribution of the SID table.  The audit callback
 * is used to print the message.
 */
extern void avc_sid_stats(void);

/**
 * avc_netlink_open - Create a netlink socket and connect to the kernel.
 */
extern int avc_netlink_open(int blocking);

/**
 * avc_netlink_loop - Wait for netlink messages from the kernel
 */
extern void avc_netlink_loop(void);

/**
 * avc_netlink_close - Close the netlink socket
 */
extern void avc_netlink_close(void);

/**
 * avc_netlink_acquire_fd - Acquire netlink socket fd.
 *
 * Allows the application to manage messages from the netlink socket in
 * its own main loop.
 */
extern int avc_netlink_acquire_fd(void);

/**
 * avc_netlink_release_fd - Release netlink socket fd.
 *
 * Returns ownership of the netlink socket to the library.
 */
extern void avc_netlink_release_fd(void);

/**
 * avc_netlink_check_nb - Check netlink socket for new messages.
 *
 * Called by the application when using avc_netlink_acquire_fd() to
 * process kernel netlink events.
 */
extern int avc_netlink_check_nb(void);

/**
 * selinux_status_open - Open and map SELinux kernel status page
 *
 */
extern int selinux_status_open(int fallback);

/**
 * selinux_status_close - Unmap and close SELinux kernel status page
 *
 */
extern void selinux_status_close(void);

/**
 * selinux_status_updated - Inform us whether the kernel status has been updated
 *
 */
extern int selinux_status_updated(void);

/**
 * selinux_status_getenforce - Get the enforce flag value
 *
 */
extern int selinux_status_getenforce(void);

/**
 * selinux_status_policyload - Get the number of policy reloaded
 *
 */
extern int selinux_status_policyload(void);

/**
 * selinux_status_deny_unknown - Get the  behavior for undefined classes/permissions
 *
 */
extern int selinux_status_deny_unknown(void);

#ifdef __cplusplus
}
#endif
#endif				/* _SELINUX_AVC_H_ */
