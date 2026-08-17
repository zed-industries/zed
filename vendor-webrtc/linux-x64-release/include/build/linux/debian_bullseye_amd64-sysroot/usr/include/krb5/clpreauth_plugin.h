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
 * Declarations for clpreauth plugin module implementors.
 *
 * The clpreauth interface has a single supported major version, which is
 * 1.  Major version 1 has a current minor version of 2.  clpreauth modules
 * should define a function named clpreauth_<modulename>_initvt, matching
 * the signature:
 *
 *   krb5_error_code
 *   clpreauth_modname_initvt(krb5_context context, int maj_ver,
 *                            int min_ver, krb5_plugin_vtable vtable);
 * The initvt function should:
 *
 * - Check that the supplied maj_ver number is supported by the module, or
 *   return KRB5_PLUGIN_VER_NOTSUPP if it is not.
 *
 * - Cast the vtable pointer as appropriate for the interface and maj_ver:
 *     maj_ver == 1: Cast to krb5_clpreauth_vtable
 *
 * - Initialize the methods of the vtable, stopping as appropriate for the
 *   supplied min_ver.  Optional methods may be left uninitialized.
 *
 * Memory for the vtable is allocated by the caller, not by the module.
 */

#ifndef KRB5_CLPREAUTH_PLUGIN_H
#define KRB5_CLPREAUTH_PLUGIN_H

#include <krb5/krb5.h>
#include <krb5/plugin.h>

/* clpreauth mechanism property flags */

/* Provides a real answer which we can send back to the KDC.  The client
 * assumes that one real answer will be enough. */
#define PA_REAL         0x00000001

/* Doesn't provide a real answer, but must be given a chance to run before any
 * REAL mechanism callbacks. */
#define PA_INFO         0x00000002

/* Abstract type for a client request information handle. */
typedef struct krb5_clpreauth_rock_st *krb5_clpreauth_rock;

/* Abstract types for module data and per-request module data. */
typedef struct krb5_clpreauth_moddata_st *krb5_clpreauth_moddata;
typedef struct krb5_clpreauth_modreq_st *krb5_clpreauth_modreq;

/* Before using a callback after version 1, modules must check the vers
 * field of the callback structure. */
typedef struct krb5_clpreauth_callbacks_st {
    int vers;

    /*
     * If an AS-REP has been received, return the enctype of the AS-REP
     * encrypted part.  Otherwise return the enctype chosen from etype-info, or
     * the first requested enctype if no etype-info was received.
     */
    krb5_enctype (*get_etype)(krb5_context context, krb5_clpreauth_rock rock);

    /* Get a pointer to the FAST armor key, or NULL if the client is not using
     * FAST.  The returned pointer is an alias and should not be freed. */
    krb5_keyblock *(*fast_armor)(krb5_context context,
                                 krb5_clpreauth_rock rock);

    /*
     * Get a pointer to the client-supplied reply key, possibly invoking the
     * prompter to ask for a password if this has not already been done.  The
     * returned pointer is an alias and should not be freed.
     */
    krb5_error_code (*get_as_key)(krb5_context context,
                                  krb5_clpreauth_rock rock,
                                  krb5_keyblock **keyblock);

    /* Replace the reply key to be used to decrypt the AS response. */
    krb5_error_code (*set_as_key)(krb5_context context,
                                  krb5_clpreauth_rock rock,
                                  const krb5_keyblock *keyblock);

    /* End of version 1 clpreauth callbacks. */

    /*
     * Get the current time for use in a preauth response.  If
     * allow_unauth_time is true and the library has been configured to allow
     * it, the current time will be offset using unauthenticated timestamp
     * information received from the KDC in the preauth-required error, if one
     * has been received.  Otherwise, the timestamp in a preauth-required error
     * will only be used if it is protected by a FAST channel.  Only set
     * allow_unauth_time if using an unauthenticated time offset would not
     * create a security issue.
     */
    krb5_error_code (*get_preauth_time)(krb5_context context,
                                        krb5_clpreauth_rock rock,
                                        krb5_boolean allow_unauth_time,
                                        krb5_timestamp *time_out,
                                        krb5_int32 *usec_out);

    /* Set a question to be answered by the responder and optionally provide
     * a challenge. */
    krb5_error_code (*ask_responder_question)(krb5_context context,
                                              krb5_clpreauth_rock rock,
                                              const char *question,
                                              const char *challenge);

    /* Get an answer from the responder, or NULL if the question was
     * unanswered. */
    const char *(*get_responder_answer)(krb5_context context,
                                        krb5_clpreauth_rock rock,
                                        const char *question);

    /* Indicate interest in the AS key through the responder interface. */
    void (*need_as_key)(krb5_context context, krb5_clpreauth_rock rock);

    /*
     * Get a configuration/state item from an input ccache, which may allow it
     * to retrace the steps it took last time.  The returned data string is an
     * alias and should not be freed.
     */
    const char *(*get_cc_config)(krb5_context context,
                                 krb5_clpreauth_rock rock, const char *key);

    /*
     * Set a configuration/state item which will be recorded to an output
     * ccache, if the calling application supplied one.  Both key and data
     * should be valid UTF-8 text.
     */
    krb5_error_code (*set_cc_config)(krb5_context context,
                                     krb5_clpreauth_rock rock,
                                     const char *key, const char *data);

    /* End of version 2 clpreauth callbacks (added in 1.11). */

    /*
     * Prevent further fallbacks to other preauth mechanisms if the KDC replies
     * with an error.  (The module itself can still respond to errors with its
     * tryagain method, or continue after KDC_ERR_MORE_PREAUTH_DATA_REQUIRED
     * errors with its process method.)  A module should invoke this callback
     * from the process method when it generates an authenticated request using
     * credentials; often this will be the first or only client message
     * generated by the mechanism.
     */
    void (*disable_fallback)(krb5_context context, krb5_clpreauth_rock rock);

    /* End of version 3 clpreauth callbacks (added in 1.17). */
} *krb5_clpreauth_callbacks;

/*
 * Optional: per-plugin initialization/cleanup.  The init function is called by
 * libkrb5 when the plugin is loaded, and the fini function is called before
 * the plugin is unloaded.  These may be called multiple times in case the
 * plugin is used in multiple contexts.  The returned context lives the
 * lifetime of the krb5_context.
 */
typedef krb5_error_code
(*krb5_clpreauth_init_fn)(krb5_context context,
                          krb5_clpreauth_moddata *moddata_out);
typedef void
(*krb5_clpreauth_fini_fn)(krb5_context context,
                          krb5_clpreauth_moddata moddata);

/*
 * Optional (mandatory before MIT krb5 1.12): pa_type will be a member of the
 * vtable's pa_type_list.  Return PA_REAL if pa_type is a real
 * preauthentication type or PA_INFO if it is an informational type.  If this
 * function is not defined in 1.12 or later, all pa_type values advertised by
 * the module will be assumed to be real.
 */
typedef int
(*krb5_clpreauth_get_flags_fn)(krb5_context context, krb5_preauthtype pa_type);

/*
 * Optional: per-request initialization/cleanup.  The request_init function is
 * called when beginning to process a get_init_creds request and the
 * request_fini function is called when processing of the request is complete.
 * This is optional.  It may be called multiple times in the lifetime of a
 * krb5_context.
 */
typedef void
(*krb5_clpreauth_request_init_fn)(krb5_context context,
                                  krb5_clpreauth_moddata moddata,
                                  krb5_clpreauth_modreq *modreq_out);
typedef void
(*krb5_clpreauth_request_fini_fn)(krb5_context context,
                                  krb5_clpreauth_moddata moddata,
                                  krb5_clpreauth_modreq modreq);

/*
 * Optional: process server-supplied data in pa_data and set responder
 * questions.
 *
 * encoded_previous_request may be NULL if there has been no previous request
 * in the AS exchange.
 */
typedef krb5_error_code
(*krb5_clpreauth_prep_questions_fn)(krb5_context context,
                                    krb5_clpreauth_moddata moddata,
                                    krb5_clpreauth_modreq modreq,
                                    krb5_get_init_creds_opt *opt,
                                    krb5_clpreauth_callbacks cb,
                                    krb5_clpreauth_rock rock,
                                    krb5_kdc_req *request,
                                    krb5_data *encoded_request_body,
                                    krb5_data *encoded_previous_request,
                                    krb5_pa_data *pa_data);

/*
 * Mandatory: process server-supplied data in pa_data and return created data
 * in pa_data_out.  Also called after the AS-REP is received if the AS-REP
 * includes preauthentication data of the associated type.
 *
 * as_key contains the client-supplied key if known, or an empty keyblock if
 * not.  If it is empty, the module may use gak_fct to fill it in.
 *
 * encoded_previous_request may be NULL if there has been no previous request
 * in the AS exchange.
 */
typedef krb5_error_code
(*krb5_clpreauth_process_fn)(krb5_context context,
                             krb5_clpreauth_moddata moddata,
                             krb5_clpreauth_modreq modreq,
                             krb5_get_init_creds_opt *opt,
                             krb5_clpreauth_callbacks cb,
                             krb5_clpreauth_rock rock,
                             krb5_kdc_req *request,
                             krb5_data *encoded_request_body,
                             krb5_data *encoded_previous_request,
                             krb5_pa_data *pa_data,
                             krb5_prompter_fct prompter, void *prompter_data,
                             krb5_pa_data ***pa_data_out);

/*
 * Optional: Attempt to use error and error_padata to try to recover from the
 * given error.  To work with both FAST and non-FAST errors, an implementation
 * should generally consult error_padata rather than decoding error->e_data.
 * For non-FAST errors, it contains the e_data decoded as either pa-data or
 * typed-data.
 *
 * If this function is provided, and it returns 0 and stores data in
 * pa_data_out, then the client library will retransmit the request.
 */
typedef krb5_error_code
(*krb5_clpreauth_tryagain_fn)(krb5_context context,
                              krb5_clpreauth_moddata moddata,
                              krb5_clpreauth_modreq modreq,
                              krb5_get_init_creds_opt *opt,
                              krb5_clpreauth_callbacks cb,
                              krb5_clpreauth_rock rock,
                              krb5_kdc_req *request,
                              krb5_data *encoded_request_body,
                              krb5_data *encoded_previous_request,
                              krb5_preauthtype pa_type,
                              krb5_error *error,
                              krb5_pa_data **error_padata,
                              krb5_prompter_fct prompter, void *prompter_data,
                              krb5_pa_data ***pa_data_out);

/*
 * Optional: receive krb5_get_init_creds_opt information.  The attr and value
 * information supplied should be copied into moddata by the module if it
 * wishes to reference it after returning from this call.
 */
typedef krb5_error_code
(*krb5_clpreauth_supply_gic_opts_fn)(krb5_context context,
                                     krb5_clpreauth_moddata moddata,
                                     krb5_get_init_creds_opt *opt,
                                     const char *attr, const char *value);

typedef struct krb5_clpreauth_vtable_st {
    /* Mandatory: name of module. */
    char *name;

    /* Mandatory: pointer to zero-terminated list of pa_types which this module
     * can provide services for. */
    krb5_preauthtype *pa_type_list;

    /* Optional: pointer to zero-terminated list of enc_types which this module
     * claims to add support for. */
    krb5_enctype *enctype_list;

    krb5_clpreauth_init_fn init;
    krb5_clpreauth_fini_fn fini;
    krb5_clpreauth_get_flags_fn flags;
    krb5_clpreauth_request_init_fn request_init;
    krb5_clpreauth_request_fini_fn request_fini;
    krb5_clpreauth_process_fn process;
    krb5_clpreauth_tryagain_fn tryagain;
    krb5_clpreauth_supply_gic_opts_fn gic_opts;
    /* Minor version 1 ends here. */

    krb5_clpreauth_prep_questions_fn prep_questions;
    /* Minor version 2 ends here. */
} *krb5_clpreauth_vtable;

/*
 * This function allows a clpreauth plugin to obtain preauth options.  The
 * preauth_data returned from this function should be freed by calling
 * krb5_get_init_creds_opt_free_pa().
 */
krb5_error_code KRB5_CALLCONV
krb5_get_init_creds_opt_get_pa(krb5_context context,
                               krb5_get_init_creds_opt *opt,
                               int *num_preauth_data,
                               krb5_gic_opt_pa_data **preauth_data);

/*
 * This function frees the preauth_data that was returned by
 * krb5_get_init_creds_opt_get_pa().
 */
void KRB5_CALLCONV
krb5_get_init_creds_opt_free_pa(krb5_context context,
                                int num_preauth_data,
                                krb5_gic_opt_pa_data *preauth_data);

#endif /* KRB5_CLPREAUTH_PLUGIN_H */
