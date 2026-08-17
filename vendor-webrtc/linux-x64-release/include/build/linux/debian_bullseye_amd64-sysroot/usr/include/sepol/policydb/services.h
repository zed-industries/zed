
/* -*- linux-c -*- */

/*
 * Author : Stephen Smalley, <sds@tycho.nsa.gov>
 */

#ifndef _SEPOL_POLICYDB_SERVICES_H_
#define _SEPOL_POLICYDB_SERVICES_H_

/*
 * Security server interface.
 */

#include <sepol/policydb/flask_types.h>
#include <sepol/policydb/policydb.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Set the policydb and sidtab structures to be used by
   the service functions.  If not set, then these default
   to private structures within libsepol that can only be
   initialized and accessed via the service functions themselves.
   Setting the structures explicitly allows a program to directly
   manipulate them, e.g. checkpolicy populates the structures directly
   from a source policy rather than from a binary policy. */
extern int sepol_set_policydb(policydb_t * p);
extern int sepol_set_sidtab(sidtab_t * s);

/* Load the security policy. This initializes the policydb
   and sidtab based on the provided binary policy. */
extern int sepol_load_policy(void *data, size_t len);

/*
 * Compute access vectors based on a SID pair for
 * the permissions in a particular class.
 */
extern int sepol_compute_av(sepol_security_id_t ssid,	/* IN */
			    sepol_security_id_t tsid,	/* IN */
			    sepol_security_class_t tclass,	/* IN */
			    sepol_access_vector_t requested,	/* IN */
			    struct sepol_av_decision *avd);	/* OUT */

/* Same as above, but also return the reason(s) for any
   denials of the requested permissions. */
#define SEPOL_COMPUTEAV_TE     0x1U
#define SEPOL_COMPUTEAV_CONS   0x2U
#define SEPOL_COMPUTEAV_RBAC   0x4U
#define SEPOL_COMPUTEAV_BOUNDS 0x8U
extern int sepol_compute_av_reason(sepol_security_id_t ssid,
				   sepol_security_id_t tsid,
				   sepol_security_class_t tclass,
				   sepol_access_vector_t requested,
				   struct sepol_av_decision *avd,
				   unsigned int *reason);

/*
 * Same as above, but also returns the constraint expression calculations
 * whether allowed or denied in a buffer. This buffer is allocated by
 * this call and must be free'd by the caller using free(3). The constraint
 * buffer will contain any constraints in infix notation.
 * If the SHOW_GRANTED flag is set it will show granted and denied
 * constraints. The default is to show only denied constraints.
 */
#define SHOW_GRANTED 1
extern int sepol_compute_av_reason_buffer(sepol_security_id_t ssid,
				   sepol_security_id_t tsid,
				   sepol_security_class_t tclass,
				   sepol_access_vector_t requested,
				   struct sepol_av_decision *avd,
				   unsigned int *reason,
				   char **reason_buf,
				   unsigned int flags);

/*
 * Returns the mls/validatetrans constraint expression calculations in
 * a buffer that must be free'd by the caller using free(3).
 * If the SHOW_GRANTED flag is set it will show granted and denied
 * mls/validatetrans (the default is to show only those denied).
 */
extern int sepol_validate_transition_reason_buffer(sepol_security_id_t oldsid,
					sepol_security_id_t newsid,
					sepol_security_id_t tasksid,
					sepol_security_class_t tclass,
					char **reason_buf,
					unsigned int flags);

/*
 * Return a class ID associated with the class string representation
 * specified by `class_name'.
 */
extern int sepol_string_to_security_class(const char *class_name,
					sepol_security_class_t  *tclass);

/*
 * Return a permission av bit associated with tclass and the string
 * representation of the `perm_name'.
 */
extern int sepol_string_to_av_perm(sepol_security_class_t tclass,
					const char *perm_name,
					sepol_access_vector_t *av);

/*
 * Compute a SID to use for labeling a new object in the 
 * class `tclass' based on a SID pair.  
 */
extern int sepol_transition_sid(sepol_security_id_t ssid,	/* IN */
				sepol_security_id_t tsid,	/* IN */
				sepol_security_class_t tclass,	/* IN */
				sepol_security_id_t * out_sid);	/* OUT */

/*
 * Compute a SID to use when selecting a member of a 
 * polyinstantiated object of class `tclass' based on 
 * a SID pair.
 */
extern int sepol_member_sid(sepol_security_id_t ssid,	/* IN */
			    sepol_security_id_t tsid,	/* IN */
			    sepol_security_class_t tclass,	/* IN */
			    sepol_security_id_t * out_sid);	/* OUT */

/*
 * Compute a SID to use for relabeling an object in the 
 * class `tclass' based on a SID pair.  
 */
extern int sepol_change_sid(sepol_security_id_t ssid,	/* IN */
			    sepol_security_id_t tsid,	/* IN */
			    sepol_security_class_t tclass,	/* IN */
			    sepol_security_id_t * out_sid);	/* OUT */

/*
 * Write the security context string representation of 
 * the context associated with `sid' into a dynamically
 * allocated string of the correct size.  Set `*scontext'
 * to point to this string and set `*scontext_len' to
 * the length of the string.
 */
extern int sepol_sid_to_context(sepol_security_id_t sid,	/* IN */
				sepol_security_context_t * scontext,	/* OUT */
				size_t * scontext_len);	/* OUT */

/*
 * Return a SID associated with the security context that
 * has the string representation specified by `scontext'.
 */
extern int sepol_context_to_sid(const sepol_security_context_t scontext,	/* IN */
				size_t scontext_len,	/* IN */
				sepol_security_id_t * out_sid);	/* OUT */

/*
 * Generate the set of SIDs for legal security contexts
 * for a given user that can be reached by `fromsid'.
 * Set `*sids' to point to a dynamically allocated 
 * array containing the set of SIDs.  Set `*nel' to the
 * number of elements in the array.
 */
extern int sepol_get_user_sids(sepol_security_id_t callsid,
			       char *username,
			       sepol_security_id_t ** sids, uint32_t * nel);

/*
 * Return the SIDs to use for an unlabeled file system
 * that is being mounted from the device with the
 * the kdevname `name'.  The `fs_sid' SID is returned for 
 * the file system and the `file_sid' SID is returned
 * for all files within that file system.
 */
extern int sepol_fs_sid(char *dev,	/* IN */
			sepol_security_id_t * fs_sid,	/* OUT  */
			sepol_security_id_t * file_sid);	/* OUT */

/*
 * Return the SID of the port specified by
 * `domain', `type', `protocol', and `port'.
 */
extern int sepol_port_sid(uint16_t domain,
			  uint16_t type,
			  uint8_t protocol,
			  uint16_t port, sepol_security_id_t * out_sid);

/*
 * Return the SID of the ibpkey specified by
 * `subnet prefix', and `pkey'.
 */
extern int sepol_ibpkey_sid(uint64_t subnet_prefix_p,
			    uint16_t pkey,
			    sepol_security_id_t *out_sid);

/*
 * Return the SID of the ibendport specified by
 * `dev_name', and `port'.
 */
extern int sepol_ibendport_sid(char *dev_name,
			       uint8_t port,
			       sepol_security_id_t *out_sid);

/*
 * Return the SIDs to use for a network interface
 * with the name `name'.  The `if_sid' SID is returned for 
 * the interface and the `msg_sid' SID is returned as
 * the default SID for messages received on the
 * interface.
 */
extern int sepol_netif_sid(char *name,
			   sepol_security_id_t * if_sid,
			   sepol_security_id_t * msg_sid);

/*
 * Return the SID of the node specified by the address
 * `addr' where `addrlen' is the length of the address
 * in bytes and `domain' is the communications domain or
 * address family in which the address should be interpreted.
 */
extern int sepol_node_sid(uint16_t domain,
			  void *addr,
			  size_t addrlen, sepol_security_id_t * out_sid);

/*
 * Return a value indicating how to handle labeling for the
 * the specified filesystem type, and optionally return a SID
 * for the filesystem object.  
 */
#define SECURITY_FS_USE_XATTR 1	/* use xattr */
#define SECURITY_FS_USE_TRANS 2	/* use transition SIDs, e.g. devpts/tmpfs */
#define SECURITY_FS_USE_TASK  3	/* use task SIDs, e.g. pipefs/sockfs */
#define SECURITY_FS_USE_GENFS 4	/* use the genfs support */
#define SECURITY_FS_USE_NONE  5	/* no labeling support */
extern int sepol_fs_use(const char *fstype,	/* IN */
			unsigned int *behavior,	/* OUT */
			sepol_security_id_t * sid);	/* OUT  */

/*
 * Return the SID to use for a file in a filesystem
 * that cannot support a persistent label mapping or use another
 * fixed labeling behavior like transition SIDs or task SIDs.
 */
extern int sepol_genfs_sid(const char *fstype,	/* IN */
			   const char *name,	/* IN */
			   sepol_security_class_t sclass,	/* IN */
			   sepol_security_id_t * sid);	/* OUT  */

#ifdef __cplusplus
}
#endif

#endif
