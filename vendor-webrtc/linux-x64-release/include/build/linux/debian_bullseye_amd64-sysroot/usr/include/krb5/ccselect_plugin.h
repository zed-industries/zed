/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (C) 2011 by the Massachusetts Institute of Technology.
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

/*
 * Declarations for credential cache selection module implementors.
 *
 * The ccselect pluggable interface currently has only one supported major
 * version, which is 1.  Major version 1 has a current minor version number of
 * 1.
 *
 * Credential cache selection modules should define a function named
 * ccselect_<modulename>_initvt, matching the signature:
 *
 *   krb5_error_code
 *   ccselect_modname_initvt(krb5_context context, int maj_ver, int min_ver,
 *                           krb5_plugin_vtable vtable);
 *
 * The initvt function should:
 *
 * - Check that the supplied maj_ver number is supported by the module, or
 *   return KRB5_PLUGIN_VER_NOTSUPP if it is not.
 *
 * - Cast the vtable pointer as appropriate for maj_ver:
 *     maj_ver == 1: Cast to krb5_ccselect_vtable
 *
 * - Initialize the methods of the vtable, stopping as appropriate for the
 *   supplied min_ver.  Optional methods may be left uninitialized.
 *
 * Memory for the vtable is allocated by the caller, not by the module.
 */

#ifndef KRB5_CCSELECT_PLUGIN_H
#define KRB5_CCSELECT_PLUGIN_H

#include <krb5/krb5.h>
#include <krb5/plugin.h>

/* An abstract type for credential cache selection module data. */
typedef struct krb5_ccselect_moddata_st *krb5_ccselect_moddata;

#define KRB5_CCSELECT_PRIORITY_AUTHORITATIVE 2
#define KRB5_CCSELECT_PRIORITY_HEURISTIC     1

/*** Method type declarations ***/

/*
 * Mandatory: Initialize module data and set *priority_out to one of the
 * KRB5_CCSELECT_PRIORITY constants above.  Authoritative modules will be
 * consulted before heuristic ones.
 */
typedef krb5_error_code
(*krb5_ccselect_init_fn)(krb5_context context, krb5_ccselect_moddata *data_out,
                         int *priority_out);

/*
 * Mandatory: Select a cache based on a server principal.  Return 0 on success,
 * with *cache_out set to the selected cache and *princ_out set to its default
 * principal.  Return KRB5_PLUGIN_NO_HANDLE to defer to other modules.  Return
 * KRB5_CC_NOTFOUND with *princ_out set if the client principal can be
 * authoritatively determined but no cache exists for it.  Return other errors
 * as appropriate.
 */
typedef krb5_error_code
(*krb5_ccselect_choose_fn)(krb5_context context, krb5_ccselect_moddata data,
                           krb5_principal server, krb5_ccache *cache_out,
                           krb5_principal *princ_out);

/* Optional: Release resources used by module data. */
typedef void
(*krb5_ccselect_fini_fn)(krb5_context context, krb5_ccselect_moddata data);

/*** vtable declarations **/

/* Credential cache selection plugin vtable for major version 1. */
typedef struct krb5_ccselect_vtable_st {
    const char *name;           /* Mandatory: name of module. */
    krb5_ccselect_init_fn init;
    krb5_ccselect_choose_fn choose;
    krb5_ccselect_fini_fn fini;
    /* Minor version 1 ends here. */
} *krb5_ccselect_vtable;

#endif /* KRB5_CCSELECT_PLUGIN_H */
