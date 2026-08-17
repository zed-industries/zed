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
 * Declarations for localauth plugin module implementors.
 *
 * The localauth pluggable interface currently has only one supported major
 * version, which is 1.  Major version 1 has a current minor version number of
 * 1.
 *
 * Localauth plugin modules should define a function named
 * localauth_<modulename>_initvt, matching the signature:
 *
 *   krb5_error_code
 *   localauth_modname_initvt(krb5_context context, int maj_ver, int min_ver,
 *                            krb5_plugin_vtable vtable);
 *
 * The initvt function should:
 *
 * - Check that the supplied maj_ver number is supported by the module, or
 *   return KRB5_PLUGIN_VER_NOTSUPP if it is not.
 *
 * - Cast the vtable pointer as appropriate for maj_ver:
 *     maj_ver == 1: Cast to krb5_localauth_vtable
 *
 * - Initialize the methods of the vtable, stopping as appropriate for the
 *   supplied min_ver.  Optional methods may be left uninitialized.
 *
 * Memory for the vtable is allocated by the caller, not by the module.
 */

#ifndef KRB5_LOCALAUTH_PLUGIN_H
#define KRB5_LOCALAUTH_PLUGIN_H

#include <krb5/krb5.h>
#include <krb5/plugin.h>

/* An abstract type for localauth module data. */
typedef struct krb5_localauth_moddata_st *krb5_localauth_moddata;

/*** Method type declarations ***/

/* Optional: Initialize module data. */
typedef krb5_error_code
(*krb5_localauth_init_fn)(krb5_context context,
                          krb5_localauth_moddata *data);

/* Optional: Release resources used by module data. */
typedef void
(*krb5_localauth_fini_fn)(krb5_context context, krb5_localauth_moddata data);

/*
 * Optional: Determine whether aname is authorized to log in as the local
 * account lname.  Return 0 if aname is authorized, EPERM if aname is
 * authoritatively not authorized, KRB5_PLUGIN_NO_HANDLE if the module cannot
 * determine whether aname is authorized, and any other error code for a
 * serious failure to process the request.  aname will be considered authorized
 * if at least one module returns 0 and all other modules return
 * KRB5_PLUGIN_NO_HANDLE.
 */
typedef krb5_error_code
(*krb5_localauth_userok_fn)(krb5_context context, krb5_localauth_moddata data,
                            krb5_const_principal aname, const char *lname);

/*
 * Optional (mandatory if an2ln_types is set): Determine the local account name
 * corresponding to aname.  Return 0 and set *lname_out if a mapping can be
 * determined; the contents of *lname_out will later be released with a call to
 * the module's free_string method.  Return KRB5_LNAME_NOTRANS if no mapping
 * can be determined.  Return any other error code for a serious failure to
 * process the request; this will halt the krb5_aname_to_localname operation.
 *
 * If the module's an2ln_types field is set, this method will only be invoked
 * when a profile "auth_to_local" value references one of the module's types.
 * type and residual will be set to the type and residual of the auth_to_local
 * value.
 *
 * If the module's an2ln_types field is not set but the an2ln method is
 * implemented, this method will be invoked independently of the profile's
 * auth_to_local settings, with type and residual set to NULL.  If multiple
 * modules are registered with an2ln methods but no an2ln_types field, the
 * order of invocation is not defined, but all such modules will be consulted
 * before the built-in mechanisms are tried.
 */
typedef krb5_error_code
(*krb5_localauth_an2ln_fn)(krb5_context context, krb5_localauth_moddata data,
                           const char *type, const char *residual,
                           krb5_const_principal aname, char **lname_out);

/*
 * Optional (mandatory if an2ln is implemented): Release the memory returned by
 * an invocation of an2ln.
 */
typedef void
(*krb5_localauth_free_string_fn)(krb5_context context,
                                 krb5_localauth_moddata data, char *str);

/* localauth vtable for major version 1. */
typedef struct krb5_localauth_vtable_st {
    const char *name;           /* Mandatory: name of module. */
    const char **an2ln_types;   /* Optional: uppercase auth_to_local types */
    krb5_localauth_init_fn init;
    krb5_localauth_fini_fn fini;
    krb5_localauth_userok_fn userok;
    krb5_localauth_an2ln_fn an2ln;
    krb5_localauth_free_string_fn free_string;
    /* Minor version 1 ends here. */
} *krb5_localauth_vtable;

#endif /* KRB5_LOCALAUTH_PLUGIN_H */
