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

/*
 * Declarations for password quality plugin module implementors.
 *
 * The password quality pluggable interface currently has only one supported
 * major version, which is 1.  Major version 1 has a current minor version
 * number of 1.
 *
 * Password quality plugin modules should define a function named
 * pwqual_<modulename>_initvt, matching the signature:
 *
 *   krb5_error_code
 *   pwqual_modname_initvt(krb5_context context, int maj_ver, int min_ver,
 *                         krb5_plugin_vtable vtable);
 *
 * The initvt function should:
 *
 * - Check that the supplied maj_ver number is supported by the module, or
 *   return KRB5_PLUGIN_VER_NOTSUPP if it is not.
 *
 * - Cast the vtable pointer as appropriate for maj_ver:
 *     maj_ver == 1: Cast to krb5_pwqual_vtable
 *
 * - Initialize the methods of the vtable, stopping as appropriate for the
 *   supplied min_ver.  Optional methods may be left uninitialized.
 *
 * Memory for the vtable is allocated by the caller, not by the module.
 */

#ifndef KRB5_PWQUAL_PLUGIN_H
#define KRB5_PWQUAL_PLUGIN_H

#include <krb5/krb5.h>
#include <krb5/plugin.h>
#include <kadm5/admin.h>

/* An abstract type for password quality module data. */
typedef struct krb5_pwqual_moddata_st *krb5_pwqual_moddata;

/*** Method type declarations ***/

/* Optional: Initialize module data.  dictfile is the realm's configured
 * dictionary filename. */
typedef krb5_error_code
(*krb5_pwqual_open_fn)(krb5_context context, const char *dict_file,
                       krb5_pwqual_moddata *data);

/*
 * Mandatory: Check a password for the principal princ, which has an associated
 * password policy named policy_name (or no associated policy if policy_name is
 * NULL).  The parameter languages, if not NULL, contains a null-terminated
 * list of client-specified language tags as defined in RFC 5646.  The method
 * should return one of the following errors if the password fails quality
 * standards:
 *
 * - KADM5_PASS_Q_TOOSHORT: password should be longer
 * - KADM5_PASS_Q_CLASS:    password must have more character classes
 * - KADM5_PASS_Q_DICT:     password contains dictionary words
 * - KADM5_PASS_Q_GENERIC:  unspecified quality failure
 *
 * The module should also set an extended error message with
 * krb5_set_error_message().  The message may be localized according to one of
 * the language tags in languages.
 */
typedef krb5_error_code
(*krb5_pwqual_check_fn)(krb5_context context, krb5_pwqual_moddata data,
                        const char *password, const char *policy_name,
                        krb5_principal princ, const char **languages);

/* Optional: Release resources used by module data. */
typedef void
(*krb5_pwqual_close_fn)(krb5_context context, krb5_pwqual_moddata data);

/*** vtable declarations **/

/* Password quality plugin vtable for major version 1. */
typedef struct krb5_pwqual_vtable_st {
    const char *name;           /* Mandatory: name of module. */
    krb5_pwqual_open_fn open;
    krb5_pwqual_check_fn check;
    krb5_pwqual_close_fn close;
    /* Minor version 1 ends here. */
} *krb5_pwqual_vtable;

#endif /* KRB5_PWQUAL_PLUGIN_H */
