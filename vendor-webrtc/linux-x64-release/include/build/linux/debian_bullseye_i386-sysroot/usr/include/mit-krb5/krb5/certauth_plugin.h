/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* include/krb5/certauth_plugin.h - certauth plugin header. */
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
 * Declarations for certauth plugin module implementors.
 *
 * The certauth pluggable interface currently has only one supported major
 * version, which is 1.  Major version 1 has a current minor version number of
 * 1.
 *
 * certauth plugin modules should define a function named
 * certauth_<modulename>_initvt, matching the signature:
 *
 *   krb5_error_code
 *   certauth_modname_initvt(krb5_context context, int maj_ver, int min_ver,
 *                           krb5_plugin_vtable vtable);
 *
 * The initvt function should:
 *
 * - Check that the supplied maj_ver number is supported by the module, or
 *   return KRB5_PLUGIN_VER_NOTSUPP if it is not.
 *
 * - Cast the vtable pointer as appropriate for maj_ver:
 *     maj_ver == 1: Cast to krb5_certauth_vtable
 *
 * - Initialize the methods of the vtable, stopping as appropriate for the
 *   supplied min_ver.  Optional methods may be left uninitialized.
 *
 * Memory for the vtable is allocated by the caller, not by the module.
 */

#ifndef KRB5_CERTAUTH_PLUGIN_H
#define KRB5_CERTAUTH_PLUGIN_H

#include <krb5/krb5.h>
#include <krb5/plugin.h>

/* Abstract module data type. */
typedef struct krb5_certauth_moddata_st *krb5_certauth_moddata;

/* A module can optionally include <kdb.h> to inspect the client principal
 * entry when authorizing a request. */
struct _krb5_db_entry_new;

/*
 * Optional: Initialize module data.
 */
typedef krb5_error_code
(*krb5_certauth_init_fn)(krb5_context context,
                         krb5_certauth_moddata *moddata_out);

/*
 * Optional: Clean up the module data.
 */
typedef void
(*krb5_certauth_fini_fn)(krb5_context context, krb5_certauth_moddata moddata);

/*
 * Mandatory:
 * Return 0 if the DER-encoded cert is authorized for PKINIT authentication by
 * princ; otherwise return one of the following error codes:
 * - KRB5KDC_ERR_CLIENT_NAME_MISMATCH - incorrect SAN value
 * - KRB5KDC_ERR_INCONSISTENT_KEY_PURPOSE - incorrect EKU
 * - KRB5KDC_ERR_CERTIFICATE_MISMATCH - other extension error
 * - KRB5_PLUGIN_NO_HANDLE - the module has no opinion about cert
 *
 * - opts is used by built-in modules to receive internal data, and must be
 *   ignored by other modules.
 * - db_entry receives the client principal database entry, and can be ignored
 *   by modules that do not link with libkdb5.
 * - *authinds_out optionally returns a null-terminated list of authentication
 *   indicator strings upon KRB5_PLUGIN_NO_HANDLE or accepted authorization.
 */
typedef krb5_error_code
(*krb5_certauth_authorize_fn)(krb5_context context,
                              krb5_certauth_moddata moddata,
                              const uint8_t *cert, size_t cert_len,
                              krb5_const_principal princ, const void *opts,
                              const struct _krb5_db_entry_new *db_entry,
                              char ***authinds_out);

/*
 * Free indicators allocated by a module.  Mandatory if authorize returns
 * authentication indicators.
 */
typedef void
(*krb5_certauth_free_indicator_fn)(krb5_context context,
                                   krb5_certauth_moddata moddata,
                                   char **authinds);

typedef struct krb5_certauth_vtable_st {
    const char *name;
    krb5_certauth_init_fn init;
    krb5_certauth_fini_fn fini;
    krb5_certauth_authorize_fn authorize;
    krb5_certauth_free_indicator_fn free_ind;
} *krb5_certauth_vtable;

#endif /* KRB5_CERTAUTH_PLUGIN_H */
