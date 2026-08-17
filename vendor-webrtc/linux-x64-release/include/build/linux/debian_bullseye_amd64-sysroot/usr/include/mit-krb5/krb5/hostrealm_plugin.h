/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (C) 2013 by the Massachusetts Institute of Technology.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * * Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer.
 *
 * * Redistributions in binary form must reproduce the above copyright
 *   notice, this list of conditions and the following disclaimer in
 *   the documentation and/or other materials provided with the
 *   distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
 * FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
 * COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT,
 * INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
 * STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
 * OF THE POSSIBILITY OF SUCH DAMAGE.
 */

/*
 * Declarations for hostrealm plugin module implementors.
 *
 * The hostrealm pluggable interface currently has only one supported major
 * version, which is 1.  Major version 1 has a current minor version number of
 * 1.
 *
 * Hostrealm plugin modules should define a function named
 * hostrealm_<modulename>_initvt, matching the signature:
 *
 *   krb5_error_code
 *   hostrealm_modname_initvt(krb5_context context, int maj_ver, int min_ver,
 *                            krb5_plugin_vtable vtable);
 *
 * The initvt function should:
 *
 * - Check that the supplied maj_ver number is supported by the module, or
 *   return KRB5_PLUGIN_VER_NOTSUPP if it is not.
 *
 * - Cast the vtable pointer as appropriate for maj_ver:
 *     maj_ver == 1: Cast to krb5_hostrealm_vtable
 *
 * - Initialize the methods of the vtable, stopping as appropriate for the
 *   supplied min_ver.  Optional methods may be left uninitialized.
 *
 * Memory for the vtable is allocated by the caller, not by the module.
 */

#ifndef KRB5_HOSTREALM_PLUGIN_H
#define KRB5_HOSTREALM_PLUGIN_H

#include <krb5/krb5.h>
#include <krb5/plugin.h>

/* An abstract type for hostrealm module data. */
typedef struct krb5_hostrealm_moddata_st *krb5_hostrealm_moddata;

/*** Method type declarations ***/

/* Optional: Initialize module data. */
typedef krb5_error_code
(*krb5_hostrealm_init_fn)(krb5_context context,
                          krb5_hostrealm_moddata *data);

/*
 * Optional: Determine the possible realms of a hostname, using only secure,
 * authoritative mechanisms (ones which should be used prior to trying
 * referrals when getting a service ticket).  Return success with a
 * null-terminated list of realms in *realms_out, KRB5_PLUGIN_NO_HANDLE to
 * defer to later modules, or another error to terminate processing.
 */
typedef krb5_error_code
(*krb5_hostrealm_host_realm_fn)(krb5_context context,
                                krb5_hostrealm_moddata data,
                                const char *host, char ***realms_out);

/*
 * Optional: Determine the possible realms of a hostname, using heuristic or
 * less secure mechanisms (ones which should be used after trying referrals
 * when getting a service ticket).  Return success with a null-terminated list
 * of realms in *realms_out, KRB5_PLUGIN_NO_HANDLE to defer to later modules,
 * or another error to terminate processing.
 */
typedef krb5_error_code
(*krb5_hostrealm_fallback_realm_fn)(krb5_context context,
                                    krb5_hostrealm_moddata data,
                                    const char *host, char ***realms_out);

/*
 * Optional: Determine the possible default realms of the local host.  Return
 * success with a null-terminated list of realms in *realms_out,
 * KRB5_PLUGIN_NO_HANDLE to defer to later modules, or another error to
 * terminate processing.
 */
typedef krb5_error_code
(*krb5_hostrealm_default_realm_fn)(krb5_context context,
                                   krb5_hostrealm_moddata data,
                                   char ***realms_out);

/*
 * Mandatory (if any of the query methods are implemented): Release the memory
 * returned by one of the interface methods.
 */
typedef void
(*krb5_hostrealm_free_list_fn)(krb5_context context,
                               krb5_hostrealm_moddata data, char **list);

/* Optional: Release resources used by module data. */
typedef void
(*krb5_hostrealm_fini_fn)(krb5_context context, krb5_hostrealm_moddata data);

/* hostrealm vtable for major version 1. */
typedef struct krb5_hostrealm_vtable_st {
    const char *name;           /* Mandatory: name of module. */
    krb5_hostrealm_init_fn init;
    krb5_hostrealm_fini_fn fini;
    krb5_hostrealm_host_realm_fn host_realm;
    krb5_hostrealm_fallback_realm_fn fallback_realm;
    krb5_hostrealm_default_realm_fn default_realm;
    krb5_hostrealm_free_list_fn free_list;
    /* Minor version 1 ends here. */
} *krb5_hostrealm_vtable;

#endif /* KRB5_HOSTREALM_PLUGIN_H */
