#ifndef _RESTORECON_H_
#define _RESTORECON_H_

#include <sys/types.h>
#include <stdarg.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * selinux_restorecon - Relabel files.
 * @pathname: specifies file/directory to relabel.
 * @restorecon_flags: specifies the actions to be performed when relabeling.
 *
 * selinux_restorecon(3) will automatically call
 * selinux_restorecon_default_handle(3) and selinux_restorecon_set_sehandle(3)
 * first time through to set the selabel_open(3) parameters to use the
 * currently loaded policy file_contexts and request their computed digest.
 *
 * Should other selabel_open(3) parameters be required see
 * selinux_restorecon_set_sehandle(3).
 */
extern int selinux_restorecon(const char *pathname,
				    unsigned int restorecon_flags);
/*
 * restorecon_flags options
 */
/*
 * Force the checking of labels even if the stored SHA1 digest
 * matches the specfiles SHA1 digest (requires CAP_SYS_ADMIN).
 */
#define SELINUX_RESTORECON_IGNORE_DIGEST		0x00001
/*
 * Do not change file labels.
 */
#define SELINUX_RESTORECON_NOCHANGE			0x00002
/*
 * If set, change file label to that in spec file.
 * If not, only change type component to that in spec file.
 */
#define SELINUX_RESTORECON_SET_SPECFILE_CTX		0x00004
/*
 * Recursively descend directories.
 */
#define SELINUX_RESTORECON_RECURSE			0x00008
/*
 * Log changes to selinux log. Note that if VERBOSE and
 * PROGRESS are set, then PROGRESS will take precedence.
 */
#define SELINUX_RESTORECON_VERBOSE			0x00010
/*
 * If SELINUX_RESTORECON_PROGRESS is true and
 * SELINUX_RESTORECON_MASS_RELABEL is true, then output approx % complete,
 * else output the number of files in 1k blocks processed to stdout.
 */
#define SELINUX_RESTORECON_PROGRESS			0x00020
/*
 * Convert passed-in pathname to canonical pathname.
 */
#define SELINUX_RESTORECON_REALPATH			0x00040
/*
 * Prevent descending into directories that have a different
 * device number than the pathname from which the descent began.
 */
#define SELINUX_RESTORECON_XDEV				0x00080
/*
 * Attempt to add an association between an inode and a specification.
 * If there is already an association for the inode and it conflicts
 * with the specification, then use the last matching specification.
 */
#define SELINUX_RESTORECON_ADD_ASSOC			0x00100
/*
 * Abort on errors during the file tree walk.
 */
#define SELINUX_RESTORECON_ABORT_ON_ERROR		0x00200
/*
 * Log any label changes to syslog.
 */
#define SELINUX_RESTORECON_SYSLOG_CHANGES		0x00400
/*
 * Log what spec matched each file.
 */
#define SELINUX_RESTORECON_LOG_MATCHES			0x00800
/*
 * Ignore files that do not exist.
 */
#define SELINUX_RESTORECON_IGNORE_NOENTRY		0x01000
/*
 * Do not read /proc/mounts to obtain a list of non-seclabel
 * mounts to be excluded from relabeling checks.
 */
#define SELINUX_RESTORECON_IGNORE_MOUNTS		0x02000
/*
 * Set if there is a mass relabel required.
 * See SELINUX_RESTORECON_PROGRESS flag for details.
 */
#define SELINUX_RESTORECON_MASS_RELABEL			0x04000
/*
 * Set if no digest is to be read or written (as only processes
 * running with CAP_SYS_ADMIN can read/write digests).
 */
#define SELINUX_RESTORECON_SKIP_DIGEST			0x08000

/*
 * Set to treat conflicting specifications as errors.
 */
#define SELINUX_RESTORECON_CONFLICT_ERROR		0x10000

/**
 * selinux_restorecon_set_sehandle - Set the global fc handle.
 * @hndl: specifies handle to set as the global fc handle.
 *
 * Called by a process that has already called selabel_open(3) with its
 * required parameters, or if selinux_restorecon_default_handle(3) has been
 * called to set the default selabel_open(3) parameters.
 */
extern void selinux_restorecon_set_sehandle(struct selabel_handle *hndl);

/**
 * selinux_restorecon_default_handle - Sets default selabel_open(3) parameters
 *				       to use the currently loaded policy and
 *				       file_contexts.
 *
 * Return value is the created handle on success or NULL with @errno set on
 * failure.
 */
extern struct selabel_handle *selinux_restorecon_default_handle(void);

/**
 * selinux_restorecon_set_exclude_list - Add a list of directories that are
 *					 to be excluded from relabeling.
 * @exclude_list: containing a NULL terminated list of one or more
 *		  directories not to be relabeled.
 */
extern void selinux_restorecon_set_exclude_list(const char **exclude_list);

/**
 * selinux_restorecon_set_alt_rootpath - Use alternate rootpath.
 * @alt_rootpath: containing the alternate rootpath to be used.
 *
 * Return %0 on success, -%1 with @errno set on failure.
 */
extern int selinux_restorecon_set_alt_rootpath(const char *alt_rootpath);

/**
 * selinux_restorecon_xattr - Read/remove security.sehash xattr entries.
 * @pathname: specifies directory path to check.
 * @xattr_flags: specifies the actions to be performed.
 * @xattr_list: a linked list of struct dir_xattr structures containing
 *              the directory, digest and result of the action on the
 *              security.sehash entry.
 *
 * selinux_restorecon_xattr(3) will automatically call
 * selinux_restorecon_default_handle(3) and selinux_restorecon_set_sehandle(3)
 * first time through to set the selabel_open(3) parameters to use the
 * currently loaded policy file_contexts and request their computed digest.
 *
 * Should other selabel_open(3) parameters be required see
 * selinux_restorecon_set_sehandle(3), however note that a file_contexts
 * computed digest is required for selinux_restorecon_xattr().
 */
enum digest_result {
	MATCH = 0,
	NOMATCH,
	DELETED_MATCH,
	DELETED_NOMATCH,
	ERROR
};

struct dir_xattr {
	char *directory;
	char *digest; /* A hex encoded string that can be printed. */
	enum digest_result result;
	struct dir_xattr *next;
};

extern int selinux_restorecon_xattr(const char *pathname,
				    unsigned int xattr_flags,
				    struct dir_xattr ***xattr_list);

/*
 * xattr_flags options
 */
/* Recursively descend directories. */
#define SELINUX_RESTORECON_XATTR_RECURSE			0x0001
/* Delete non-matching digests from each directory in pathname. */
#define SELINUX_RESTORECON_XATTR_DELETE_NONMATCH_DIGESTS	0x0002
/* Delete all digests found in pathname. */
#define SELINUX_RESTORECON_XATTR_DELETE_ALL_DIGESTS		0x0004
/* Do not read /proc/mounts. */
#define SELINUX_RESTORECON_XATTR_IGNORE_MOUNTS			0x0008

#ifdef __cplusplus
}
#endif
#endif
