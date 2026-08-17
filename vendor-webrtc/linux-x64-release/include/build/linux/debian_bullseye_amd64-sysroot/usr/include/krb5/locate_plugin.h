/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 2006 Massachusetts Institute of Technology.
 * All Rights Reserved.
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
 *
 * Service location plugin definitions for Kerberos 5.
 */

#ifndef KRB5_LOCATE_PLUGIN_H_INCLUDED
#define KRB5_LOCATE_PLUGIN_H_INCLUDED
#include <krb5/krb5.h>

enum locate_service_type {
    locate_service_kdc = 1,
    locate_service_master_kdc,
    locate_service_kadmin,
    locate_service_krb524,
    locate_service_kpasswd
};

typedef struct krb5plugin_service_locate_ftable {
    int minor_version;          /* currently 0 */
    /* Per-context setup and teardown.  Returned void* blob is
       private to the plugin.  */
    krb5_error_code (*init)(krb5_context, void **);
    void (*fini)(void *);
    /* Callback function returns non-zero if the plugin function
       should quit and return; this may be because of an error, or may
       indicate we've already contacted the service, whatever.  The
       lookup function should only return an error if it detects a
       problem, not if the callback function tells it to quit.  */
    krb5_error_code (*lookup)(void *,
                              enum locate_service_type svc, const char *realm,
                              int socktype, int family,
                              int (*cbfunc)(void *,int,struct sockaddr *),
                              void *cbdata);
} krb5plugin_service_locate_ftable;
/* extern krb5plugin_service_locate_ftable service_locator; */
#endif
