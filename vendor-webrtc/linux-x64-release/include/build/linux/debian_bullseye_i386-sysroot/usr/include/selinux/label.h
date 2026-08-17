/*
 * Labeling interface for userspace object managers and others.
 *
 * Author : Eamon Walsh <ewalsh@tycho.nsa.gov>
 */
#ifndef _SELABEL_H_
#define _SELABEL_H_

#include <stdbool.h>
#include <stdint.h>
#include <sys/types.h>
#include <selinux/selinux.h>

#ifdef __cplusplus
extern "C" {
#endif

/*
 * Opaque type used for all label handles.
 */

struct selabel_handle;

/* 
 * Available backends.
 */

/* file contexts */
#define SELABEL_CTX_FILE	0
/* media contexts */
#define SELABEL_CTX_MEDIA	1
/* x contexts */
#define SELABEL_CTX_X		2
/* db objects */
#define SELABEL_CTX_DB		3
/* Android property service contexts */
#define SELABEL_CTX_ANDROID_PROP 4
/* Android service contexts */
#define SELABEL_CTX_ANDROID_SERVICE 5

/*
 * Available options
 */

/* no-op option, useful for unused slots in an array of options */
#define SELABEL_OPT_UNUSED	0
/* validate contexts before returning them (boolean value) */
#define SELABEL_OPT_VALIDATE	1
/* don't use local customizations to backend data (boolean value) */
#define SELABEL_OPT_BASEONLY	2
/* specify an alternate path to use when loading backend data */
#define SELABEL_OPT_PATH	3
/* select a subset of the search space as an optimization (file backend) */
#define SELABEL_OPT_SUBSET	4
/* require a hash calculation on spec files */
#define SELABEL_OPT_DIGEST	5
/* total number of options */
#define SELABEL_NOPT		6

/*
 * Label operations
 */

/**
 * selabel_open - Create a labeling handle.
 * @backend: one of the constants specifying a supported labeling backend.
 * @opts: array of selabel_opt structures specifying label options or NULL.
 * @nopts: number of elements in opts array or zero for no options.
 *
 * Open a labeling backend for use.  The available backend identifiers are
 * listed above.  Options may be provided via the opts parameter; available
 * options are listed above.  Not all options may be supported by every
 * backend.  Return value is the created handle on success or NULL with
 * @errno set on failure.
 */
extern struct selabel_handle *selabel_open(unsigned int backend,
					   const struct selinux_opt *opts,
					   unsigned nopts);

/**
 * selabel_close - Close a labeling handle.
 * @handle: specifies handle to close
 *
 * Destroy the specified handle, closing files, freeing allocated memory,
 * etc.  The handle may not be further used after it has been closed.
 */
extern void selabel_close(struct selabel_handle *handle);

/**
 * selabel_lookup - Perform labeling lookup operation.
 * @handle: specifies backend instance to query
 * @con: returns the appropriate context with which to label the object
 * @key: string input to lookup operation
 * @type: numeric input to the lookup operation
 *
 * Perform a labeling lookup operation.  Return %0 on success, -%1 with
 * @errno set on failure.  The key and type arguments are the inputs to the
 * lookup operation; appropriate values are dictated by the backend in use.
 * The result is returned in the memory pointed to by @con and must be freed
 * by the user with freecon().
 */
extern int selabel_lookup(struct selabel_handle *handle, char **con,
			  const char *key, int type);
extern int selabel_lookup_raw(struct selabel_handle *handle, char **con,
			      const char *key, int type);

extern bool selabel_partial_match(struct selabel_handle *handle, const char *key);

extern bool selabel_get_digests_all_partial_matches(struct selabel_handle *rec,
						    const char *key,
						    uint8_t **calculated_digest,
						    uint8_t **xattr_digest,
						    size_t *digest_len);
extern bool selabel_hash_all_partial_matches(struct selabel_handle *rec,
					     const char *key, uint8_t* digest);

extern int selabel_lookup_best_match(struct selabel_handle *rec, char **con,
				     const char *key, const char **aliases, int type);
extern int selabel_lookup_best_match_raw(struct selabel_handle *rec, char **con,
					 const char *key, const char **aliases, int type);

/**
 * selabel_digest - Retrieve the SHA1 digest and the list of specfiles used to
 *		    generate the digest. The SELABEL_OPT_DIGEST option must
 *		    be set in selabel_open() to initiate the digest generation.
 * @handle: specifies backend instance to query
 * @digest: returns a pointer to the SHA1 digest.
 * @digest_len: returns length of digest in bytes.
 * @specfiles: a list of specfiles used in the SHA1 digest generation.
 *	       The list is NULL terminated and will hold @num_specfiles entries.
 * @num_specfiles: number of specfiles in the list.
 *
 * Return %0 on success, -%1 with @errno set on failure.
 */
extern int selabel_digest(struct selabel_handle *rec,
			  unsigned char **digest, size_t *digest_len,
			  char ***specfiles, size_t *num_specfiles);

enum selabel_cmp_result {
	SELABEL_SUBSET,
	SELABEL_EQUAL,
	SELABEL_SUPERSET,
	SELABEL_INCOMPARABLE
};

/**
 * selabel_cmp - Compare two label configurations.
 * @h1: handle for the first label configuration
 * @h2: handle for the first label configuration
 *
 * Compare two label configurations.
 * Return %SELABEL_SUBSET if @h1 is a subset of @h2, %SELABEL_EQUAL
 * if @h1 is identical to @h2, %SELABEL_SUPERSET if @h1 is a superset
 * of @h2, and %SELABEL_INCOMPARABLE if @h1 and @h2 are incomparable.
 */
extern enum selabel_cmp_result selabel_cmp(struct selabel_handle *h1,
					   struct selabel_handle *h2);

/**
 * selabel_stats - log labeling operation statistics.
 * @handle: specifies backend instance to query
 *
 * Log a message with information about the number of queries performed,
 * number of unused matching entries, or other operational statistics.
 * Message is backend-specific, some backends may not output a message.
 */
extern void selabel_stats(struct selabel_handle *handle);

/*
 * Type codes used by specific backends
 */

/* X backend */
#define SELABEL_X_PROP		1
#define SELABEL_X_EXT		2
#define SELABEL_X_CLIENT	3
#define SELABEL_X_EVENT		4
#define SELABEL_X_SELN		5
#define SELABEL_X_POLYPROP	6
#define SELABEL_X_POLYSELN	7

/* DB backend */
#define SELABEL_DB_DATABASE	1
#define SELABEL_DB_SCHEMA	2
#define SELABEL_DB_TABLE	3
#define SELABEL_DB_COLUMN	4
#define SELABEL_DB_SEQUENCE	5
#define SELABEL_DB_VIEW		6
#define SELABEL_DB_PROCEDURE	7
#define SELABEL_DB_BLOB		8
#define SELABEL_DB_TUPLE	9
#define SELABEL_DB_LANGUAGE	10
#define SELABEL_DB_EXCEPTION 11
#define SELABEL_DB_DATATYPE 12

#ifdef __cplusplus
}
#endif
#endif	/* _SELABEL_H_ */
