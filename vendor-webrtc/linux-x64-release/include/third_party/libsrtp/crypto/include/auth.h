/*
 * auth.h
 *
 * common interface to authentication functions
 *
 * David A. McGrew
 * Cisco Systems, Inc.
 */

/*
 *
 * Copyright (c) 2001-2017, Cisco Systems, Inc.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 *   Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer.
 *
 *   Redistributions in binary form must reproduce the above
 *   copyright notice, this list of conditions and the following
 *   disclaimer in the documentation and/or other materials provided
 *   with the distribution.
 *
 *   Neither the name of the Cisco Systems, Inc. nor the names of its
 *   contributors may be used to endorse or promote products derived
 *   from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
 * FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
 * COPYRIGHT HOLDERS OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT,
 * INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
 * STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
 * OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 */

#ifndef SRTP_AUTH_H
#define SRTP_AUTH_H

#include "srtp.h"
#include "crypto_types.h" /* for values of auth_type_id_t */

#ifdef __cplusplus
extern "C" {
#endif

typedef const struct srtp_auth_type_t *srtp_auth_type_pointer;
typedef struct srtp_auth_t *srtp_auth_pointer_t;

typedef srtp_err_status_t (*srtp_auth_alloc_func)(srtp_auth_pointer_t *ap,
                                                  int key_len,
                                                  int out_len);

typedef srtp_err_status_t (*srtp_auth_init_func)(void *state,
                                                 const uint8_t *key,
                                                 int key_len);

typedef srtp_err_status_t (*srtp_auth_dealloc_func)(srtp_auth_pointer_t ap);

typedef srtp_err_status_t (*srtp_auth_compute_func)(void *state,
                                                    const uint8_t *buffer,
                                                    int octets_to_auth,
                                                    int tag_len,
                                                    uint8_t *tag);

typedef srtp_err_status_t (*srtp_auth_update_func)(void *state,
                                                   const uint8_t *buffer,
                                                   int octets_to_auth);

typedef srtp_err_status_t (*srtp_auth_start_func)(void *state);

/* some syntactic sugar on these function types */
#define srtp_auth_type_alloc(at, a, klen, outlen)                              \
    ((at)->alloc((a), (klen), (outlen)))

#define srtp_auth_init(a, key)                                                 \
    (((a)->type)->init((a)->state, (key), ((a)->key_len)))

#define srtp_auth_compute(a, buf, len, res)                                    \
    (((a)->type)->compute((a)->state, (buf), (len), (a)->out_len, (res)))

#define srtp_auth_update(a, buf, len)                                          \
    (((a)->type)->update((a)->state, (buf), (len)))

#define srtp_auth_start(a) (((a)->type)->start((a)->state))

#define srtp_auth_dealloc(c) (((c)->type)->dealloc(c))

/* functions to get information about a particular auth_t */
int srtp_auth_get_key_length(const struct srtp_auth_t *a);

int srtp_auth_get_tag_length(const struct srtp_auth_t *a);

int srtp_auth_get_prefix_length(const struct srtp_auth_t *a);

/*
 * srtp_auth_test_case_t is a (list of) key/message/tag values that are
 * known to be correct for a particular cipher.  this data can be used
 * to test an implementation in an on-the-fly self test of the
 * correctness of the implementation.  (see the srtp_auth_type_self_test()
 * function below)
 */
typedef struct srtp_auth_test_case_t {
    int key_length_octets;  /* octets in key            */
    const uint8_t *key;     /* key                      */
    int data_length_octets; /* octets in data           */
    const uint8_t *data;    /* data                     */
    int tag_length_octets;  /* octets in tag            */
    const uint8_t *tag;     /* tag                      */
    const struct srtp_auth_test_case_t
        *next_test_case; /* pointer to next testcase */
} srtp_auth_test_case_t;

/* srtp_auth_type_t */
typedef struct srtp_auth_type_t {
    srtp_auth_alloc_func alloc;
    srtp_auth_dealloc_func dealloc;
    srtp_auth_init_func init;
    srtp_auth_compute_func compute;
    srtp_auth_update_func update;
    srtp_auth_start_func start;
    const char *description;
    const srtp_auth_test_case_t *test_data;
    srtp_auth_type_id_t id;
} srtp_auth_type_t;

typedef struct srtp_auth_t {
    const srtp_auth_type_t *type;
    void *state;
    int out_len;    /* length of output tag in octets */
    int key_len;    /* length of key in octets        */
    int prefix_len; /* length of keystream prefix     */
} srtp_auth_t;

/*
 * srtp_auth_type_self_test() tests an auth_type against test cases
 * provided in an array of values of key/message/tag that is known to
 * be good
 */
srtp_err_status_t srtp_auth_type_self_test(const srtp_auth_type_t *at);

/*
 * srtp_auth_type_test() tests an auth_type against external test cases
 * provided in an array of values of key/message/tag that is known to
 * be good
 */
srtp_err_status_t srtp_auth_type_test(const srtp_auth_type_t *at,
                                      const srtp_auth_test_case_t *test_data);

/*
 * srtp_replace_auth_type(ct, id)
 *
 * replaces srtp's kernel's auth type implementation for the auth_type id
 * with a new one passed in externally.  The new auth type must pass all the
 * existing auth_type's self tests as well as its own.
 */
srtp_err_status_t srtp_replace_auth_type(const srtp_auth_type_t *ct,
                                         srtp_auth_type_id_t id);

#ifdef __cplusplus
}
#endif

#endif /* SRTP_AUTH_H */
