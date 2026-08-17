/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* include/krb5/kdcpolicy_plugin.h - KDC policy plugin interface */
/*
 * Copyright (C) 2017 by Red Hat, Inc.
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
 * Declarations for kdcpolicy plugin module implementors.
 *
 * The kdcpolicy pluggable interface currently has only one supported major
 * version, which is 1.  Major version 1 has a current minor version number of
 * 1.
 *
 * kdcpolicy plugin modules should define a function named
 * kdcpolicy_<modulename>_initvt, matching the signature:
 *
 *   krb5_error_code
 *   kdcpolicy_modname_initvt(krb5_context context, int maj_ver, int min_ver,
 *                            krb5_plugin_vtable vtable);
 *
 * The initvt function should:
 *
 * - Check that the supplied maj_ver number is supported by the module, or
 *   return KRB5_PLUGIN_VER_NOTSUPP if it is not.
 *
 * - Cast the vtable pointer as appropriate for maj_ver:
 *   maj_ver == 1: Cast to krb5_kdcpolicy_vtable
 *
 * - Initialize the methods of the vtable, stopping as appropriate for the
 *   supplied min_ver.  Optional methods may be left uninitialized.
 *
 * Memory for the vtable is allocated by the caller, not by the module.
 */

#ifndef KRB5_POLICY_PLUGIN_H
#define KRB5_POLICY_PLUGIN_H

#include <krb5/krb5.h>

/* Abstract module datatype. */
typedef struct krb5_kdcpolicy_moddata_st *krb5_kdcpolicy_moddata;

/* A module can optionally include kdb.h to inspect principal entries when
 * authorizing requests. */
struct _krb5_db_entry_new;

/*
 * Optional: Initialize module data.  Return 0 on success,
 * KRB5_PLUGIN_NO_HANDLE if the module is inoperable (due to configuration, for
 * example), and any other error code to abort KDC startup.  Optionally set
 * *data_out to a module data object to be passed to future calls.
 */
typedef krb5_error_code
(*krb5_kdcpolicy_init_fn)(krb5_context context,
                          krb5_kdcpolicy_moddata *data_out);

/* Optional: Clean up module data. */
typedef krb5_error_code
(*krb5_kdcpolicy_fini_fn)(krb5_context context,
                          krb5_kdcpolicy_moddata moddata);

/*
 * Optional: return an error code and set status to an appropriate string
 * literal to deny an AS request; otherwise return 0.  lifetime_out, if set,
 * restricts the ticket lifetime.  renew_lifetime_out, if set, restricts the
 * ticket renewable lifetime.
 */
typedef krb5_error_code
(*krb5_kdcpolicy_check_as_fn)(krb5_context context,
                              krb5_kdcpolicy_moddata moddata,
                              const krb5_kdc_req *request,
                              const struct _krb5_db_entry_new *client,
                              const struct _krb5_db_entry_new *server,
                              const char *const *auth_indicators,
                              const char **status, krb5_deltat *lifetime_out,
                              krb5_deltat *renew_lifetime_out);

/*
 * Optional: return an error code and set status to an appropriate string
 * literal to deny a TGS request; otherwise return 0.  lifetime_out, if set,
 * restricts the ticket lifetime.  renew_lifetime_out, if set, restricts the
 * ticket renewable lifetime.
 */
typedef krb5_error_code
(*krb5_kdcpolicy_check_tgs_fn)(krb5_context context,
                               krb5_kdcpolicy_moddata moddata,
                               const krb5_kdc_req *request,
                               const struct _krb5_db_entry_new *server,
                               const krb5_ticket *ticket,
                               const char *const *auth_indicators,
                               const char **status, krb5_deltat *lifetime_out,
                               krb5_deltat *renew_lifetime_out);

typedef struct krb5_kdcpolicy_vtable_st {
    const char *name;
    krb5_kdcpolicy_init_fn init;
    krb5_kdcpolicy_fini_fn fini;
    krb5_kdcpolicy_check_as_fn check_as;
    krb5_kdcpolicy_check_tgs_fn check_tgs;
} *krb5_kdcpolicy_vtable;

#endif /* KRB5_POLICY_PLUGIN_H */
