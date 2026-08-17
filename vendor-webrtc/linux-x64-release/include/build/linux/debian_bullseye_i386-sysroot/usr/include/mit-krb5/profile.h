/*
 * profile.h
 */

#ifndef _KRB5_PROFILE_H
#define _KRB5_PROFILE_H

#if defined(_WIN32)
#include <win-mac.h>
#endif

#if defined(__MACH__) && defined(__APPLE__)
#    include <TargetConditionals.h>
#    if TARGET_RT_MAC_CFM
#        error "Use KfM 4.0 SDK headers for CFM compilation."
#    endif
#endif

#ifndef KRB5_CALLCONV
#define KRB5_CALLCONV
#define KRB5_CALLCONV_C
#endif

typedef struct _profile_t *profile_t;

/* Used by profile_init_flags(). */
#define PROFILE_INIT_ALLOW_MODULE       0x0001  /* Allow module declaration */

/*
 * Used by the profile iterator in prof_get.c
 */
#define PROFILE_ITER_LIST_SECTION	0x0001
#define PROFILE_ITER_SECTIONS_ONLY	0x0002
#define PROFILE_ITER_RELATIONS_ONLY	0x0004

#ifdef __cplusplus
extern "C" {
#endif /* __cplusplus */

typedef char* profile_filespec_t;	/* path as C string */
typedef char* profile_filespec_list_t;	/* list of : separated paths, C string */
typedef const char * const_profile_filespec_t;	/* path as C string */
typedef const char * const_profile_filespec_list_t;	/* list of : separated paths, C string */

long KRB5_CALLCONV profile_init
	(const_profile_filespec_t *files, profile_t *ret_profile);

long KRB5_CALLCONV profile_init_flags
	(const_profile_filespec_t *files, int flags, profile_t *ret_profile);

long KRB5_CALLCONV profile_init_path
	(const_profile_filespec_list_t filelist, profile_t *ret_profile);

long KRB5_CALLCONV profile_flush
	(profile_t profile);
long KRB5_CALLCONV profile_flush_to_file
	(profile_t profile, const_profile_filespec_t outfile);
long KRB5_CALLCONV profile_flush_to_buffer
	(profile_t profile, char **bufp);
void KRB5_CALLCONV profile_free_buffer
	(profile_t profile, char *buf);

long KRB5_CALLCONV profile_is_writable
	(profile_t profile, int *writable);
long KRB5_CALLCONV profile_is_modified
	(profile_t profile, int *modified);

void KRB5_CALLCONV profile_abandon
	(profile_t profile);

void KRB5_CALLCONV profile_release
	(profile_t profile);

long KRB5_CALLCONV profile_get_values
	(profile_t profile, const char *const *names, char ***ret_values);

void KRB5_CALLCONV profile_free_list
	(char **list);

long KRB5_CALLCONV profile_get_string
	(profile_t profile, const char *name, const char *subname,
			const char *subsubname, const char *def_val,
			char **ret_string);
long KRB5_CALLCONV profile_get_integer
	(profile_t profile, const char *name, const char *subname,
			const char *subsubname, int def_val,
			int *ret_default);

long KRB5_CALLCONV profile_get_boolean
	(profile_t profile, const char *name, const char *subname,
			const char *subsubname, int def_val,
			int *ret_default);

long KRB5_CALLCONV profile_get_relation_names
	(profile_t profile, const char **names, char ***ret_names);

long KRB5_CALLCONV profile_get_subsection_names
	(profile_t profile, const char **names, char ***ret_names);

long KRB5_CALLCONV profile_iterator_create
	(profile_t profile, const char *const *names,
		   int flags, void **ret_iter);

void KRB5_CALLCONV profile_iterator_free
	(void **iter_p);

long KRB5_CALLCONV profile_iterator
	(void	**iter_p, char **ret_name, char **ret_value);

void KRB5_CALLCONV profile_release_string (char *str);

long KRB5_CALLCONV profile_update_relation
	(profile_t profile, const char **names,
		   const char *old_value, const char *new_value);

long KRB5_CALLCONV profile_clear_relation
	(profile_t profile, const char **names);

long KRB5_CALLCONV profile_rename_section
	(profile_t profile, const char **names,
		   const char *new_name);

long KRB5_CALLCONV profile_add_relation
	(profile_t profile, const char **names,
		   const char *new_value);

/*
 * profile_init_vtable allows a caller to create a profile-compatible object
 * with a different back end.
 */

/*
 * Mandatory: Look up all of the relations for names, placing the resulting
 * values in *ret_values.  If no relations exist, return PROF_NO_RELATION, or
 * PROF_NO_SECTION to indicate that one of the intermediate names does not
 * exist as a section.  The list will be freed with free_values.
 */
typedef long
(*profile_get_values_fn)(void *cbdata, const char *const *names,
			 char ***ret_values);

/* Mandatory: Free a list of strings returned by get_values. */
typedef void
(*profile_free_values_fn)(void *cbdata, char **values);

/* Optional: Release any data associated with the profile. */
typedef void
(*profile_cleanup_fn)(void *cbdata);

/*
 * Optional (mandatory if cleanup is defined): Generate a new cbdata pointer
 * for a copy of the profile.  If not implemented, the new profile will receive
 * the same cbdata pointer as the old one.
 */
typedef long
(*profile_copy_fn)(void *cbdata, void **ret_cbdata);

/*
 * Optional: Create an iterator handle.
 *
 * If flags contains PROFILE_ITER_LIST_SECTION, iterate over all of the
 * relations and sections within names.  Otherwise, iterate over the relation
 * values for names, or produce a single section result if names is a section.
 *
 * If flags contains PROFILE_ITER_SECTIONS_ONLY, produce only sections.
 *
 * If flags contains PROFILE_ITER_RELATIONS_ONLY, produce only relations.
 */
typedef long
(*profile_iterator_create_fn)(void *cbdata, const char *const *names,
			      int flags, void **ret_iter);

/*
 * Optional (mandatory if iterator_create is defined): Produce the next
 * relation or section in an iteration.  If producing a section result, set
 * *ret_value to NULL.  The returned strings will be freed with free_string.
 */
typedef long
(*profile_iterator_fn)(void *cbdata, void *iter, char **ret_name,
		       char **ret_value);

/*
 * Optional (mandatory if iterator_create is defined): Free the memory for an
 * iterator.
 */
typedef void
(*profile_iterator_free_fn)(void *cbdata, void *iter);

/* Optional (mandatory if iterator is defined): Free a string value. */
typedef void
(*profile_free_string_fn)(void *cbdata, char *string);

/*
 * Optional: Determine if a profile is writable.  If not implemented, the
 * profile is never writable.
 */
typedef long
(*profile_writable_fn)(void *cbdata, int *writable);

/*
 * Optional: Determine if a profile is modified in memory relative to the
 * persistent store.  If not implemented, the profile is assumed to never be
 * modified.
 */
typedef long
(*profile_modified_fn)(void *cbdata, int *modified);

/*
 * Optional: Change the value of a relation, or remove it if new_value is NULL.
 * If old_value is set and the relation does not have that value, return
 * PROF_NO_RELATION.
 */
typedef long
(*profile_update_relation_fn)(void *cbdata, const char **names,
			      const char *old_value, const char *new_value);

/*
 * Optional: Rename a section to new_name, or remove the section if new_name is
 * NULL.
 */
typedef long
(*profile_rename_section_fn)(void *cbdata, const char **names,
			     const char *new_name);

/*
 * Optional: Add a new relation, or a new section if new_value is NULL.  Add
 * any intermediate sections as necessary.
 */
typedef long
(*profile_add_relation_fn)(void *cbdata, const char **names,
			   const char *new_value);

/*
 * Optional: Flush any pending memory updates to the persistent store.  If
 * implemented, this function will be called by profile_release as well as
 * profile_flush, so make sure it's not inefficient to flush an unmodified
 * profile.
 */
typedef long
(*profile_flush_fn)(void *cbdata);

struct profile_vtable {
    int minor_ver;              /* Set to structure minor version (currently 1)
				 * if calling profile_init_vtable. */

    /* Methods needed for a basic read-only non-iterable profile (cleanup is
     * optional). */
    profile_get_values_fn get_values;
    profile_free_values_fn free_values;
    profile_cleanup_fn cleanup;
    profile_copy_fn copy;

    /* Methods for iterable profiles. */
    profile_iterator_create_fn iterator_create;
    profile_iterator_fn iterator;
    profile_iterator_free_fn iterator_free;
    profile_free_string_fn free_string;

    /* Methods for writable profiles. */
    profile_writable_fn writable;
    profile_modified_fn modified;
    profile_update_relation_fn update_relation;
    profile_rename_section_fn rename_section;
    profile_add_relation_fn add_relation;
    profile_flush_fn flush;

    /* End of minor version 1. */
};

/*
 * Create a profile object whose operations will be performed using the
 * function pointers in vtable.  cbdata will be supplied to each vtable
 * function as the first argument.
 */
long KRB5_CALLCONV profile_init_vtable
	(struct profile_vtable *vtable, void *cbdata, profile_t *ret_profile);

/*
 * Dynamically loadable profile modules should define a function named
 * "profile_module_init" matching the following signature.  The function should
 * initialize the methods of the provided vtable structure, stopping at the
 * field corresponding to vtable->minor_ver.  Do not change the value of
 * vtable->minor_ver.  Unimplemented methods can be left uninitialized.  The
 * function should supply a callback data pointer in *cb_ret; this pointer can
 * be cleaned up via the vtable cleanup method.
 */
typedef long
(*profile_module_init_fn)(const char *residual, struct profile_vtable *vtable,
			  void **cb_ret);

#ifdef __cplusplus
}
#endif /* __cplusplus */

#endif /* _KRB5_PROFILE_H */
/*
 * et-h-prof_err.h:
 * This file is automatically generated; please do not edit it.
 */

#include <et/com_err.h>

#define PROF_VERSION                             (-1429577728L)
#define PROF_MAGIC_NODE                          (-1429577727L)
#define PROF_NO_SECTION                          (-1429577726L)
#define PROF_NO_RELATION                         (-1429577725L)
#define PROF_ADD_NOT_SECTION                     (-1429577724L)
#define PROF_SECTION_WITH_VALUE                  (-1429577723L)
#define PROF_BAD_LINK_LIST                       (-1429577722L)
#define PROF_BAD_GROUP_LVL                       (-1429577721L)
#define PROF_BAD_PARENT_PTR                      (-1429577720L)
#define PROF_MAGIC_ITERATOR                      (-1429577719L)
#define PROF_SET_SECTION_VALUE                   (-1429577718L)
#define PROF_EINVAL                              (-1429577717L)
#define PROF_READ_ONLY                           (-1429577716L)
#define PROF_SECTION_NOTOP                       (-1429577715L)
#define PROF_SECTION_SYNTAX                      (-1429577714L)
#define PROF_RELATION_SYNTAX                     (-1429577713L)
#define PROF_EXTRA_CBRACE                        (-1429577712L)
#define PROF_MISSING_OBRACE                      (-1429577711L)
#define PROF_MAGIC_PROFILE                       (-1429577710L)
#define PROF_MAGIC_SECTION                       (-1429577709L)
#define PROF_TOPSECTION_ITER_NOSUPP              (-1429577708L)
#define PROF_INVALID_SECTION                     (-1429577707L)
#define PROF_END_OF_SECTIONS                     (-1429577706L)
#define PROF_BAD_NAMESET                         (-1429577705L)
#define PROF_NO_PROFILE                          (-1429577704L)
#define PROF_MAGIC_FILE                          (-1429577703L)
#define PROF_FAIL_OPEN                           (-1429577702L)
#define PROF_EXISTS                              (-1429577701L)
#define PROF_BAD_BOOLEAN                         (-1429577700L)
#define PROF_BAD_INTEGER                         (-1429577699L)
#define PROF_MAGIC_FILE_DATA                     (-1429577698L)
#define PROF_FAIL_INCLUDE_FILE                   (-1429577697L)
#define PROF_FAIL_INCLUDE_DIR                    (-1429577696L)
#define PROF_UNSUPPORTED                         (-1429577695L)
#define PROF_MAGIC_NODE_ITERATOR                 (-1429577694L)
#define PROF_MODULE                              (-1429577693L)
#define PROF_MODULE_SYNTAX                       (-1429577692L)
#define PROF_MODULE_INVALID                      (-1429577691L)
extern const struct error_table et_prof_error_table;
extern void initialize_prof_error_table(void);

/* For compatibility with Heimdal */
extern void initialize_prof_error_table_r(struct et_list **list);

#define ERROR_TABLE_BASE_prof (-1429577728L)

/* for compatibility with older versions... */
#define init_prof_err_tbl initialize_prof_error_table
#define prof_err_base ERROR_TABLE_BASE_prof
