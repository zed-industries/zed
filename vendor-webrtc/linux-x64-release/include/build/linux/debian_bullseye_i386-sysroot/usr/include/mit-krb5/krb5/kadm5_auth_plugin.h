/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (C) 2017 by the Massachusetts Institute of Technology.
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
 * Declarations for kadm5_auth plugin module implementors.
 *
 * The kadm5_auth pluggable interface currently has only one supported major
 * version, which is 1.  Major version 1 has a current minor version number of
 * 1.
 *
 * kadm5_auth plugin modules should define a function named
 * kadm5_auth_<modulename>_initvt, matching the signature:
 *
 *   krb5_error_code
 *   kadm5_auth_modname_initvt(krb5_context context, int maj_ver, int min_ver,
 *                             krb5_plugin_vtable vtable);
 *
 * The initvt function should:
 *
 * - Check that the supplied maj_ver number is supported by the module, or
 *   return KRB5_PLUGIN_VER_NOTSUPP if it is not.
 *
 * - Cast the vtable pointer as appropriate for maj_ver:
 *     maj_ver == 1: Cast to krb5_kadm5_auth_vtable
 *
 * - Initialize the methods of the vtable, stopping as appropriate for the
 *   supplied min_ver.  Optional methods may be left uninitialized.
 *
 * Memory for the vtable is allocated by the caller, not by the module.
 */

#ifndef KRB5_KADM5_AUTH_PLUGIN_H
#define KRB5_KADM5_AUTH_PLUGIN_H

#include <krb5/krb5.h>
#include <krb5/plugin.h>

/* An abstract type for kadm5_auth module data. */
typedef struct kadm5_auth_moddata_st *kadm5_auth_moddata;

/*
 * A module can optionally include <kadm5/admin.h> to inspect principal or
 * policy records from requests that add or modify principals or policies.
 * Note that fields of principal and policy structures are only valid if the
 * corresponding bit is set in the accompanying mask parameter.
 */
struct _kadm5_principal_ent_t;
struct _kadm5_policy_ent_t;

/*
 * A module can optionally generate restrictions when checking permissions for
 * adding or modifying a principal entry.  Restriction fields will only be
 * honored if the corresponding mask bit is set.  The operable mask bits are
 * defined in <kadmin/admin.h> and are:
 *
 * - KADM5_ATTRIBUTES for require_attrs, forbid_attrs
 * - KADM5_POLICY for policy
 * - KADM5_POLICY_CLR to require that policy be unset
 * - KADM5_PRINC_EXPIRE_TIME for princ_lifetime
 * - KADM5_PW_EXPIRATION for pw_lifetime
 * - KADM5_MAX_LIFE for max_life
 * - KADM5_MAX_RLIFE for max_renewable_life
 */
struct kadm5_auth_restrictions {
    long mask;
    krb5_flags require_attrs;
    krb5_flags forbid_attrs;
    krb5_deltat princ_lifetime;
    krb5_deltat pw_lifetime;
    krb5_deltat max_life;
    krb5_deltat max_renewable_life;
    char *policy;
};

/*** Method type declarations ***/

/*
 * Optional: Initialize module data.  acl_file is the realm's configured ACL
 * file, or NULL if none was configured.  Return 0 on success,
 * KRB5_PLUGIN_NO_HANDLE if the module is inoperable (due to configuration, for
 * example), and any other error code to abort kadmind startup.  Optionally set
 * *data_out to a module data object to be passed to future calls.
 */
typedef krb5_error_code
(*kadm5_auth_init_fn)(krb5_context context, const char *acl_file,
                      kadm5_auth_moddata *data_out);

/* Optional: Release resources used by module data. */
typedef void
(*kadm5_auth_fini_fn)(krb5_context context, kadm5_auth_moddata data);

/*
 * Each check method below should return 0 to explicitly authorize the request,
 * KRB5_PLUGIN_NO_HANDLE to neither authorize nor deny the request, and any
 * other error code (such as EPERM) to explicitly deny the request.  If a check
 * method is not defined, the module will neither authorize nor deny the
 * request.  A request succeeds if at least one kadm5_auth module explicitly
 * authorizes the request and none of the modules explicitly deny it.
 */

/* Optional: authorize an add-principal operation, and optionally generate
 * restrictions. */
typedef krb5_error_code
(*kadm5_auth_addprinc_fn)(krb5_context context, kadm5_auth_moddata data,
                          krb5_const_principal client,
                          krb5_const_principal target,
                          const struct _kadm5_principal_ent_t *ent, long mask,
                          struct kadm5_auth_restrictions **rs_out);

/* Optional: authorize a modify-principal operation, and optionally generate
 * restrictions. */
typedef krb5_error_code
(*kadm5_auth_modprinc_fn)(krb5_context context, kadm5_auth_moddata data,
                          krb5_const_principal client,
                          krb5_const_principal target,
                          const struct _kadm5_principal_ent_t *ent, long mask,
                          struct kadm5_auth_restrictions **rs_out);

/* Optional: authorize a set-string operation. */
typedef krb5_error_code
(*kadm5_auth_setstr_fn)(krb5_context context, kadm5_auth_moddata data,
                        krb5_const_principal client,
                        krb5_const_principal target,
                        const char *key, const char *value);

/* Optional: authorize a change-password operation. */
typedef krb5_error_code
(*kadm5_auth_cpw_fn)(krb5_context context, kadm5_auth_moddata data,
                     krb5_const_principal client, krb5_const_principal target);

/* Optional: authorize a randomize-keys operation. */
typedef krb5_error_code
(*kadm5_auth_chrand_fn)(krb5_context context, kadm5_auth_moddata data,
                        krb5_const_principal client,
                        krb5_const_principal target);

/* Optional: authorize a set-key operation. */
typedef krb5_error_code
(*kadm5_auth_setkey_fn)(krb5_context context, kadm5_auth_moddata data,
                        krb5_const_principal client,
                        krb5_const_principal target);

/* Optional: authorize a purgekeys operation. */
typedef krb5_error_code
(*kadm5_auth_purgekeys_fn)(krb5_context context, kadm5_auth_moddata data,
                           krb5_const_principal client,
                           krb5_const_principal target);

/* Optional: authorize a delete-principal operation. */
typedef krb5_error_code
(*kadm5_auth_delprinc_fn)(krb5_context context, kadm5_auth_moddata data,
                          krb5_const_principal client,
                          krb5_const_principal target);

/* Optional: authorize a rename-principal operation. */
typedef krb5_error_code
(*kadm5_auth_renprinc_fn)(krb5_context context, kadm5_auth_moddata data,
                          krb5_const_principal client,
                          krb5_const_principal src,
                          krb5_const_principal dest);

/* Optional: authorize a get-principal operation. */
typedef krb5_error_code
(*kadm5_auth_getprinc_fn)(krb5_context context, kadm5_auth_moddata data,
                          krb5_const_principal client,
                          krb5_const_principal target);

/* Optional: authorize a get-strings operation. */
typedef krb5_error_code
(*kadm5_auth_getstrs_fn)(krb5_context context, kadm5_auth_moddata data,
                         krb5_const_principal client,
                         krb5_const_principal target);

/* Optional: authorize an extract-keys operation. */
typedef krb5_error_code
(*kadm5_auth_extract_fn)(krb5_context context, kadm5_auth_moddata data,
                         krb5_const_principal client,
                         krb5_const_principal target);

/* Optional: authorize a list-principals operation. */
typedef krb5_error_code
(*kadm5_auth_listprincs_fn)(krb5_context context, kadm5_auth_moddata data,
                            krb5_const_principal client);

/* Optional: authorize an add-policy operation. */
typedef krb5_error_code
(*kadm5_auth_addpol_fn)(krb5_context context, kadm5_auth_moddata data,
                        krb5_const_principal client, const char *policy,
                        const struct _kadm5_policy_ent_t *ent, long mask);

/* Optional: authorize a modify-policy operation. */
typedef krb5_error_code
(*kadm5_auth_modpol_fn)(krb5_context context, kadm5_auth_moddata data,
                        krb5_const_principal client, const char *policy,
                        const struct _kadm5_policy_ent_t *ent, long mask);

/* Optional: authorize a delete-policy operation. */
typedef krb5_error_code
(*kadm5_auth_delpol_fn)(krb5_context context, kadm5_auth_moddata data,
                        krb5_const_principal client, const char *policy);

/* Optional: authorize a get-policy operation.  client_policy is the client
 * principal's policy name, or NULL if it does not have one. */
typedef krb5_error_code
(*kadm5_auth_getpol_fn)(krb5_context context, kadm5_auth_moddata data,
                        krb5_const_principal client, const char *policy,
                        const char *client_policy);

/* Optional: authorize a list-policies operation. */
typedef krb5_error_code
(*kadm5_auth_listpols_fn)(krb5_context context, kadm5_auth_moddata data,
                          krb5_const_principal client);

/* Optional: authorize an iprop operation. */
typedef krb5_error_code
(*kadm5_auth_iprop_fn)(krb5_context context, kadm5_auth_moddata data,
                       krb5_const_principal client);

/*
 * Optional: receive a notification that the most recent authorized operation
 * has ended.  If a kadm5_auth module is also a KDB module, it can assume that
 * all KDB methods invoked between a kadm5_auth authorization method invocation
 * and a kadm5_auth end invocation are performed as part of the authorized
 * operation.
 *
 * The end method may be invoked without a preceding authorization method in
 * some cases; the module must be prepared to ignore such calls.
 */
typedef void
(*kadm5_auth_end_fn)(krb5_context context, kadm5_auth_moddata data);

/*
 * Optional: free a restrictions object.  This method does not need to be
 * defined if the module does not generate restrictions objects, or if it
 * returns aliases to restrictions objects contained from within the module
 * data.
 */
typedef void
(*kadm5_auth_free_restrictions_fn)(krb5_context context,
                                   kadm5_auth_moddata data,
                                   struct kadm5_auth_restrictions *rs);

/* kadm5_auth vtable for major version 1. */
typedef struct kadm5_auth_vtable_st {
    const char *name;           /* Mandatory: name of module. */
    kadm5_auth_init_fn init;
    kadm5_auth_fini_fn fini;

    kadm5_auth_addprinc_fn addprinc;
    kadm5_auth_modprinc_fn modprinc;
    kadm5_auth_setstr_fn setstr;
    kadm5_auth_cpw_fn cpw;
    kadm5_auth_chrand_fn chrand;
    kadm5_auth_setkey_fn setkey;
    kadm5_auth_purgekeys_fn purgekeys;
    kadm5_auth_delprinc_fn delprinc;
    kadm5_auth_renprinc_fn renprinc;

    kadm5_auth_getprinc_fn getprinc;
    kadm5_auth_getstrs_fn getstrs;
    kadm5_auth_extract_fn extract;
    kadm5_auth_listprincs_fn listprincs;

    kadm5_auth_addpol_fn addpol;
    kadm5_auth_modpol_fn modpol;
    kadm5_auth_delpol_fn delpol;
    kadm5_auth_getpol_fn getpol;
    kadm5_auth_listpols_fn listpols;

    kadm5_auth_iprop_fn iprop;

    kadm5_auth_end_fn end;

    kadm5_auth_free_restrictions_fn free_restrictions;
    /* Minor version 1 ends here. */
} *kadm5_auth_vtable;

#endif /* KRB5_KADM5_AUTH_PLUGIN_H */
