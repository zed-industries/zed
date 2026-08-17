/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (C) 2010 by the Massachusetts Institute of Technology.
 * All rights reserved.
 *
 * Export of this software from the United States of America may
 *   require a specific license from the United States Government.
 *   It is the responsibility of any person or organization contemplating
 *   export to obtain such a license before exporting.
 *
 * WITHIN THAT CONSTRAINT, permission to use, copy, modify, and
 * distribute this software and its documentation for any purpose and
 * without fee is hereby granted, provided that the above copyright
 * notice appear in all copies and that both that copyright notice and
 * this permission notice appear in supporting documentation, and that
 * the name of M.I.T. not be used in advertising or publicity pertaining
 * to distribution of the software without specific, written prior
 * permission.  Furthermore if you modify this software you must label
 * your software as modified software and not distribute it in such a
 * fashion that it might be confused with the original M.I.T. software.
 * M.I.T. makes no representations about the suitability of
 * this software for any purpose.  It is provided "as is" without express
 * or implied warranty.
 */

#ifndef H_KRB5_KADM5_HOOK_PLUGIN
#define H_KRB5_KADM5_HOOK_PLUGIN

/**
 * @file krb5/krb5_kadm5_hook_plugin.h
 * Provide a plugin interface for kadm5 operations. This interface
 * permits a plugin to intercept principal modification, creation and
 * change password operations. Operations run at two stages: a
 * precommit stage that runs before the operation is committed to the
 * database and a postcommit operation that runs after the database
 * is updated; see #kadm5_hook_stage for details on semantics.
 *
 * This interface is based on a proposed extension to Heimdal by Russ
 * Allbery; it is likely that Heimdal will adopt an approach based on
 * stacked kdb modules rather than this interface. For MIT, writing a
 * plugin to this interface is significantly easier than stacking kdb
 * modules. Also, the kadm5 interface is significantly more stable
 * than the kdb interface, so this approach is more desirable than
 * stacked kdb modules.
 *
 * This interface depends on kadm5/admin.h. As such, the interface
 * does not provide strong guarantees of ABI stability.
 *
 * The kadm5_hook interface currently has only one supported major version,
 * which is 1.  Major version 1 has a current minor version number of 2.
 *
 * kadm5_hook plugins should:
 * kadm5_hook_<modulename>_initvt, matching the signature:
 *
 *   krb5_error_code
 *   kadm5_hook_modname_initvt(krb5_context context, int maj_ver, int min_ver,
 *                         krb5_plugin_vtable vtable);
 *
 * The initvt function should:
 *
 * - Check that the supplied maj_ver number is supported by the module, or
 *   return KRB5_PLUGIN_VER_NOTSUPP if it is not.
 *
 * - Cast the vtable pointer as appropriate for maj_ver:
 *     maj_ver == 1: Cast to kadm5_hook_vftable_1
 *
 * - Initialize the methods of the vtable, stopping as appropriate for the
 *   supplied min_ver.  Optional methods may be left uninitialized.
 *
 * Memory for the vtable is allocated by the caller, not by the module.
 */

#include <krb5/krb5.h>
#include <krb5/plugin.h>
#include <kadm5/admin.h>

/**
 * Whether the operation is being run before or after the database
 * update.
 */
enum kadm5_hook_stage {
    /** In this stage, any plugin failure prevents following plugins from
     *         running and aborts the operation.*/
    KADM5_HOOK_STAGE_PRECOMMIT,
    /** In this stage, plugin failures are logged but otherwise ignored.*/
    KADM5_HOOK_STAGE_POSTCOMMIT
};

/** Opaque module data pointer. */
typedef struct kadm5_hook_modinfo_st kadm5_hook_modinfo;

/**
 * Interface for the v1 virtual table for the kadm5_hook plugin.
 * All entry points are optional. The name field must be provided.
 */
typedef struct kadm5_hook_vtable_1_st {

    /** A text string identifying the plugin for logging messages. */
    const char *name;

    /** Initialize a plugin module.
     * @param modinfo returns newly allocated module info for future
     * calls.  Cleaned up by the fini() function.
     */
    kadm5_ret_t (*init)(krb5_context, kadm5_hook_modinfo **modinfo);

    /** Clean up a module and free @a modinfo. */
    void (*fini)(krb5_context, kadm5_hook_modinfo *modinfo);

    /** Indicates that the password is being changed.
     * @param stage is an integer from #kadm5_hook_stage enumeration
     * @param keepold is true if existing keys are being kept.
     * @param newpass is NULL if the key sare being randomized.
     */
    kadm5_ret_t (*chpass)(krb5_context,
                          kadm5_hook_modinfo *modinfo,
                          int stage,
                          krb5_principal, krb5_boolean keepold,
                          int n_ks_tuple,
                          krb5_key_salt_tuple *ks_tuple,
                          const char *newpass);

    /** Indicate a principal is created. */
    kadm5_ret_t (*create)(krb5_context,
                          kadm5_hook_modinfo *,
                          int stage,
                          kadm5_principal_ent_t, long mask,
                          int n_ks_tuple,
                          krb5_key_salt_tuple *ks_tuple,
                          const char *password);

    /** Modify a principal. */
    kadm5_ret_t (*modify)(krb5_context,
                          kadm5_hook_modinfo *,
                          int stage,
                          kadm5_principal_ent_t, long mask);

    /** Indicate a principal is deleted. */
    kadm5_ret_t (*remove)(krb5_context,
                          kadm5_hook_modinfo *modinfo,
                          int stage, krb5_principal);

    /* End of minor version 1. */

    /** Indicate a principal is renamed. */
    kadm5_ret_t (*rename)(krb5_context,
                          kadm5_hook_modinfo *modinfo,
                          int stage, krb5_principal, krb5_principal);

    /* End of minor version 2. */

} kadm5_hook_vftable_1;

#endif /*H_KRB5_KADM5_HOOK_PLUGIN*/
