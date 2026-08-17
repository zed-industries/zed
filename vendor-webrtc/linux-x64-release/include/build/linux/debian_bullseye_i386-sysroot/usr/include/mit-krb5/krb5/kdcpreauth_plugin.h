/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (c) 2006 Red Hat, Inc.
 * Portions copyright (c) 2006, 2011 Massachusetts Institute of Technology
 * All Rights Reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 *  * Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 *  * Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in
 *    the documentation and/or other materials provided with the
 *    distribution.
 *  * Neither the name of Red Hat, Inc., nor the names of its
 *    contributors may be used to endorse or promote products derived
 *    from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS
 * IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED
 * TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A
 * PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER
 * OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
 * LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
 * NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

/*
 * Declarations for kdcpreauth plugin module implementors.
 *
 * The kdcpreauth interface has a single supported major version, which is 1.
 * Major version 1 has a current minor version of 2.  kdcpreauth modules should
 * define a function named kdcpreauth_<modulename>_initvt, matching the
 * signature:
 *
 *   krb5_error_code
 *   kdcpreauth_modname_initvt(krb5_context context, int maj_ver, int min_ver,
 *                             krb5_plugin_vtable vtable);
 *
 * The initvt function should:
 *
 * - Check that the supplied maj_ver number is supported by the module, or
 *   return KRB5_PLUGIN_VER_NOTSUPP if it is not.
 *
 * - Cast the vtable pointer as appropriate for the interface and maj_ver:
 *     kdcpreauth, maj_ver == 1: Cast to krb5_kdcpreauth_vtable
 *
 * - Initialize the methods of the vtable, stopping as appropriate for the
 *   supplied min_ver.  Optional methods may be left uninitialized.
 *
 * Memory for the vtable is allocated by the caller, not by the module.
 */

#ifndef KRB5_KDCPREAUTH_PLUGIN_H
#define KRB5_KDCPREAUTH_PLUGIN_H

#include <krb5/krb5.h>
#include <krb5/plugin.h>

/* kdcpreauth mechanism property flags */

/*
 * Causes the KDC to include this mechanism in a list of supported preauth
 * types if the user's DB entry flags the user as requiring hardware-based
 * preauthentication.
 */
#define PA_HARDWARE     0x00000004

/*
 * Causes the KDC to include this mechanism in a list of supported preauth
 * types if the user's DB entry flags the user as requiring preauthentication,
 * and to fail preauthentication if we can't verify the client data.  The
 * flipside of PA_SUFFICIENT.
 */
#define PA_REQUIRED     0x00000008

/*
 * Causes the KDC to include this mechanism in a list of supported preauth
 * types if the user's DB entry flags the user as requiring preauthentication,
 * and to mark preauthentication as successful if we can verify the client
 * data.  The flipside of PA_REQUIRED.
 */
#define PA_SUFFICIENT   0x00000010

/*
 * Marks this preauthentication mechanism as one which changes the key which is
 * used for encrypting the response to the client.  Modules which have this
 * flag have their server_return_fn called before modules which do not, and are
 * passed over if a previously-called module has modified the encrypting key.
 */
#define PA_REPLACES_KEY 0x00000020

/*
 * Not really a padata type, so don't include it in any list of preauth types
 * which gets sent over the wire.
 */
#define PA_PSEUDO       0x00000080

/*
 * Indicates that e_data in non-FAST errors should be encoded as typed data
 * instead of padata.
 */
#define PA_TYPED_E_DATA 0x00000100

/* Abstract type for a KDC callback data handle. */
typedef struct krb5_kdcpreauth_rock_st *krb5_kdcpreauth_rock;

/* Abstract type for module data and per-request module data. */
typedef struct krb5_kdcpreauth_moddata_st *krb5_kdcpreauth_moddata;
typedef struct krb5_kdcpreauth_modreq_st *krb5_kdcpreauth_modreq;

/* The verto context structure type (typedef is in verto.h; we want to avoid a
 * header dependency for the moment). */
struct verto_ctx;

/* Before using a callback after version 1, modules must check the vers
 * field of the callback structure. */
typedef struct krb5_kdcpreauth_callbacks_st {
    int vers;

    krb5_deltat (*max_time_skew)(krb5_context context,
                                 krb5_kdcpreauth_rock rock);

    /*
     * Get an array of krb5_keyblock structures containing the client keys
     * matching the request enctypes, terminated by an entry with key type = 0.
     * Returns ENOENT if no keys are available for the request enctypes.  Free
     * the resulting object with the free_keys callback.
     */
    krb5_error_code (*client_keys)(krb5_context context,
                                   krb5_kdcpreauth_rock rock,
                                   krb5_keyblock **keys_out);

    /* Free the result of client_keys. */
    void (*free_keys)(krb5_context context, krb5_kdcpreauth_rock rock,
                      krb5_keyblock *keys);

    /*
     * Get the encoded request body, which is sometimes needed for checksums.
     * For a FAST request this is the encoded inner request body.  The returned
     * pointer is an alias and should not be freed.
     */
    krb5_data *(*request_body)(krb5_context context,
                               krb5_kdcpreauth_rock rock);

    /* Get a pointer to the FAST armor key, or NULL if the request did not use
     * FAST.  The returned pointer is an alias and should not be freed. */
    krb5_keyblock *(*fast_armor)(krb5_context context,
                                 krb5_kdcpreauth_rock rock);

    /* Retrieve a string attribute from the client DB entry, or NULL if no such
     * attribute is set.  Free the result with the free_string callback. */
    krb5_error_code (*get_string)(krb5_context context,
                                  krb5_kdcpreauth_rock rock, const char *key,
                                  char **value_out);

    /* Free the result of get_string. */
    void (*free_string)(krb5_context context, krb5_kdcpreauth_rock rock,
                        char *string);

    /* Get a pointer to the client DB entry (returned as a void pointer to
     * avoid a dependency on a libkdb5 type). */
    void *(*client_entry)(krb5_context context, krb5_kdcpreauth_rock rock);

    /* Get a pointer to the verto context which should be used by an
     * asynchronous edata or verify method. */
    struct verto_ctx *(*event_context)(krb5_context context,
                                       krb5_kdcpreauth_rock rock);

    /* End of version 1 kdcpreauth callbacks. */

    /* Return true if the client DB entry contains any keys matching the
     * request enctypes. */
    krb5_boolean (*have_client_keys)(krb5_context context,
                                     krb5_kdcpreauth_rock rock);

    /* End of version 2 kdcpreauth callbacks. */

    /*
     * Get the decrypted client long-term key chosen according to the request
     * enctype list, or NULL if no matching key was found.  The returned
     * pointer is an alias and should not be freed.  If invoked from
     * return_padata, the result will be the same as the encrypting_key
     * parameter if it is not NULL, and will therefore reflect the modified
     * reply key if a return_padata handler has replaced the reply key.
     */
    const krb5_keyblock *(*client_keyblock)(krb5_context context,
                                            krb5_kdcpreauth_rock rock);

    /* Assert an authentication indicator in the AS-REP authdata.  Duplicate
     * indicators will be ignored. */
    krb5_error_code (*add_auth_indicator)(krb5_context context,
                                          krb5_kdcpreauth_rock rock,
                                          const char *indicator);

    /*
     * Read a data value for pa_type from the request cookie, placing it in
     * *out.  The value placed there is an alias and must not be freed.
     * Returns true if a value for pa_type was retrieved, false if not.
     */
    krb5_boolean (*get_cookie)(krb5_context context, krb5_kdcpreauth_rock rock,
                               krb5_preauthtype pa_type, krb5_data *out);

    /*
     * Set a data value for pa_type to be sent in a secure cookie in the next
     * error response.  If pa_type is already present, the value is ignored.
     * If the preauth mechanism has different preauth types for requests and
     * responses, use the request type.  Secure cookies are encrypted in a key
     * known only to the KDCs, but can be replayed within a short time window
     * for requests using the same client principal.
     */
    krb5_error_code (*set_cookie)(krb5_context context,
                                  krb5_kdcpreauth_rock rock,
                                  krb5_preauthtype pa_type,
                                  const krb5_data *data);

    /* End of version 3 kdcpreauth callbacks. */

    /*
     * Return true if princ matches the principal named in the request or the
     * client principal (possibly canonicalized).  If princ does not match,
     * attempt a database lookup of princ with aliases allowed and compare the
     * result to the client principal, returning true if it matches.
     * Otherwise, return false.
     */
    krb5_boolean (*match_client)(krb5_context context,
                                 krb5_kdcpreauth_rock rock,
                                 krb5_principal princ);

    /*
     * Get an alias to the client DB entry principal (possibly canonicalized).
     */
    krb5_principal (*client_name)(krb5_context context,
                                  krb5_kdcpreauth_rock rock);

    /* End of version 4 kdcpreauth callbacks. */

    /*
     * Instruct the KDC to send a freshness token in the method data
     * accompanying a PREAUTH_REQUIRED or PREAUTH_FAILED error, if the client
     * indicated support for freshness tokens.  This callback should only be
     * invoked from the edata method.
     */
    void (*send_freshness_token)(krb5_context context,
                                 krb5_kdcpreauth_rock rock);

    /* Validate a freshness token sent by the client.  Return 0 on success,
     * KRB5KDC_ERR_PREAUTH_EXPIRED on error. */
    krb5_error_code (*check_freshness_token)(krb5_context context,
                                             krb5_kdcpreauth_rock rock,
                                             const krb5_data *token);

    /* End of version 5 kdcpreauth callbacks. */

} *krb5_kdcpreauth_callbacks;

/* Optional: preauth plugin initialization function. */
typedef krb5_error_code
(*krb5_kdcpreauth_init_fn)(krb5_context context,
                           krb5_kdcpreauth_moddata *moddata_out,
                           const char **realmnames);

/* Optional: preauth plugin cleanup function. */
typedef void
(*krb5_kdcpreauth_fini_fn)(krb5_context context,
                           krb5_kdcpreauth_moddata moddata);

/*
 * Optional: return the flags which the KDC should use for this module.  This
 * is a callback instead of a static value because the module may or may not
 * wish to count itself as a hardware preauthentication module (in other words,
 * the flags may be affected by the configuration, for example if a site
 * administrator can force a particular preauthentication type to be supported
 * using only hardware).  This function is called for each entry entry in the
 * server_pa_type_list.
 */
typedef int
(*krb5_kdcpreauth_flags_fn)(krb5_context context, krb5_preauthtype pa_type);

/*
 * Responder for krb5_kdcpreauth_edata_fn.  If invoked with a non-zero code, pa
 * will be ignored and the padata type will not be included in the hint list.
 * If invoked with a zero code and a null pa value, the padata type will be
 * included in the list with an empty value.  If invoked with a zero code and a
 * non-null pa value, pa will be included in the hint list and will later be
 * freed by the KDC.
 */
typedef void
(*krb5_kdcpreauth_edata_respond_fn)(void *arg, krb5_error_code code,
                                    krb5_pa_data *pa);

/*
 * Optional: provide pa_data to send to the client as part of the "you need to
 * use preauthentication" error.  The implementation must invoke the respond
 * when complete, whether successful or not, either before returning or
 * asynchronously using the verto context returned by cb->event_context().
 *
 * This function is not allowed to create a modreq object because we have no
 * guarantee that the client will ever make a follow-up request, or that it
 * will hit this KDC if it does.
 */
typedef void
(*krb5_kdcpreauth_edata_fn)(krb5_context context, krb5_kdc_req *request,
                            krb5_kdcpreauth_callbacks cb,
                            krb5_kdcpreauth_rock rock,
                            krb5_kdcpreauth_moddata moddata,
                            krb5_preauthtype pa_type,
                            krb5_kdcpreauth_edata_respond_fn respond,
                            void *arg);

/*
 * Responder for krb5_kdcpreauth_verify_fn.  Invoke with the arg parameter
 * supplied to verify, the error code (0 for success), an optional module
 * request state object to be consumed by return_fn or free_modreq_fn, optional
 * e_data to be passed to the caller if code is nonzero, and optional
 * authorization data to be included in the ticket.  In non-FAST replies,
 * e_data will be encoded as typed-data if the module sets the PA_TYPED_E_DATA
 * flag, and as pa-data otherwise.  e_data and authz_data will be freed by the
 * KDC.
 */
typedef void
(*krb5_kdcpreauth_verify_respond_fn)(void *arg, krb5_error_code code,
                                     krb5_kdcpreauth_modreq modreq,
                                     krb5_pa_data **e_data,
                                     krb5_authdata **authz_data);

/*
 * Optional: verify preauthentication data sent by the client, setting the
 * TKT_FLG_PRE_AUTH or TKT_FLG_HW_AUTH flag in the enc_tkt_reply's "flags"
 * field as appropriate.  The implementation must invoke the respond function
 * when complete, whether successful or not, either before returning or
 * asynchronously using the verto context returned by cb->event_context().
 */
typedef void
(*krb5_kdcpreauth_verify_fn)(krb5_context context,
                             krb5_data *req_pkt, krb5_kdc_req *request,
                             krb5_enc_tkt_part *enc_tkt_reply,
                             krb5_pa_data *data,
                             krb5_kdcpreauth_callbacks cb,
                             krb5_kdcpreauth_rock rock,
                             krb5_kdcpreauth_moddata moddata,
                             krb5_kdcpreauth_verify_respond_fn respond,
                             void *arg);

/*
 * Optional: generate preauthentication response data to send to the client as
 * part of the AS-REP.  If it needs to override the key which is used to
 * encrypt the response, it can do so.
 */
typedef krb5_error_code
(*krb5_kdcpreauth_return_fn)(krb5_context context,
                             krb5_pa_data *padata,
                             krb5_data *req_pkt,
                             krb5_kdc_req *request,
                             krb5_kdc_rep *reply,
                             krb5_keyblock *encrypting_key,
                             krb5_pa_data **send_pa_out,
                             krb5_kdcpreauth_callbacks cb,
                             krb5_kdcpreauth_rock rock,
                             krb5_kdcpreauth_moddata moddata,
                             krb5_kdcpreauth_modreq modreq);

/* Optional: free a per-request context. */
typedef void
(*krb5_kdcpreauth_free_modreq_fn)(krb5_context,
                                  krb5_kdcpreauth_moddata moddata,
                                  krb5_kdcpreauth_modreq modreq);

/* Optional: invoked after init_fn to provide the module with a pointer to the
 * verto main loop. */
typedef krb5_error_code
(*krb5_kdcpreauth_loop_fn)(krb5_context context,
                           krb5_kdcpreauth_moddata moddata,
                           struct verto_ctx *ctx);

typedef struct krb5_kdcpreauth_vtable_st {
    /* Mandatory: name of module. */
    char *name;

    /* Mandatory: pointer to zero-terminated list of pa_types which this module
     * can provide services for. */
    krb5_preauthtype *pa_type_list;

    krb5_kdcpreauth_init_fn init;
    krb5_kdcpreauth_fini_fn fini;
    krb5_kdcpreauth_flags_fn flags;
    krb5_kdcpreauth_edata_fn edata;
    krb5_kdcpreauth_verify_fn verify;
    krb5_kdcpreauth_return_fn return_padata;
    krb5_kdcpreauth_free_modreq_fn free_modreq;
    /* Minor 1 ends here. */

    krb5_kdcpreauth_loop_fn loop;
    /* Minor 2 ends here. */
} *krb5_kdcpreauth_vtable;

#endif /* KRB5_KDCPREAUTH_PLUGIN_H */
