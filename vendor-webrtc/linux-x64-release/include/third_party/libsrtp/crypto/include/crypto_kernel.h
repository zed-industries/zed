/*
 * crypto_kernel.h
 *
 * header for the cryptographic kernel
 *
 * David A. McGrew
 * Cisco Systems, Inc.
 */
/*
 *
 * Copyright(c) 2001-2017 Cisco Systems, Inc.
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

#ifndef CRYPTO_KERNEL
#define CRYPTO_KERNEL

#include "cipher.h"
#include "auth.h"
#include "err.h"
#include "crypto_types.h"
#include "key.h"

#ifdef __cplusplus
extern "C" {
#endif

/*
 * crypto_kernel_state_t defines the possible states:
 *
 *    insecure - not yet initialized
 *    secure   - initialized and passed self-tests
 */
typedef enum {
    srtp_crypto_kernel_state_insecure,
    srtp_crypto_kernel_state_secure
} srtp_crypto_kernel_state_t;

/*
 * linked list of cipher types
 */
typedef struct srtp_kernel_cipher_type {
    srtp_cipher_type_id_t id;
    const srtp_cipher_type_t *cipher_type;
    struct srtp_kernel_cipher_type *next;
} srtp_kernel_cipher_type_t;

/*
 * linked list of auth types
 */
typedef struct srtp_kernel_auth_type {
    srtp_auth_type_id_t id;
    const srtp_auth_type_t *auth_type;
    struct srtp_kernel_auth_type *next;
} srtp_kernel_auth_type_t;

/*
 * linked list of debug modules
 */
typedef struct srtp_kernel_debug_module {
    srtp_debug_module_t *mod;
    struct srtp_kernel_debug_module *next;
} srtp_kernel_debug_module_t;

/*
 * crypto_kernel_t is the data structure for the crypto kernel
 *
 * note that there is *exactly one* instance of this data type,
 * a global variable defined in crypto_kernel.c
 */
typedef struct {
    srtp_crypto_kernel_state_t state; /* current state of kernel     */
    srtp_kernel_cipher_type_t *cipher_type_list; /* list of all cipher types */
    srtp_kernel_auth_type_t *auth_type_list; /* list of all auth func types */
    srtp_kernel_debug_module_t
        *debug_module_list; /* list of all debug modules   */
} srtp_crypto_kernel_t;

/*
 * srtp_crypto_kernel_t external api
 */

/*
 * The function srtp_crypto_kernel_init() initialized the crypto kernel and
 * runs the self-test operations on the random number generators and
 * crypto algorithms.  Possible return values are:
 *
 *    srtp_err_status_ok    initialization successful
 *    <other>               init failure
 *
 * If any value other than srtp_err_status_ok is returned, the
 * crypto_kernel MUST NOT be used.
 */
srtp_err_status_t srtp_crypto_kernel_init(void);

/*
 * The function srtp_crypto_kernel_shutdown() de-initializes the
 * crypto_kernel, zeroizes keys and other cryptographic material, and
 * deallocates any dynamically allocated memory.  Possible return
 * values are:
 *
 *    srtp_err_status_ok     shutdown successful
 *    <other>                shutdown failure
 *
 */
srtp_err_status_t srtp_crypto_kernel_shutdown(void);

/*
 * The function srtp_crypto_kernel_stats() checks the the crypto_kernel,
 * running tests on the ciphers, auth funcs, and rng, and prints out a
 * status report.  Possible return values are:
 *
 *    srtp_err_status_ok     all tests were passed
 *    <other>                a test failed
 *
 */
srtp_err_status_t srtp_crypto_kernel_status(void);

/*
 * srtp_crypto_kernel_list_debug_modules() outputs a list of debugging modules
 *
 */
srtp_err_status_t srtp_crypto_kernel_list_debug_modules(void);

/*
 * srtp_crypto_kernel_load_cipher_type()
 *
 */
srtp_err_status_t srtp_crypto_kernel_load_cipher_type(
    const srtp_cipher_type_t *ct,
    srtp_cipher_type_id_t id);

srtp_err_status_t srtp_crypto_kernel_load_auth_type(const srtp_auth_type_t *ct,
                                                    srtp_auth_type_id_t id);

srtp_err_status_t srtp_crypto_kernel_load_debug_module(
    srtp_debug_module_t *new_dm);

/*
 * srtp_crypto_kernel_alloc_cipher(id, cp, key_len);
 *
 * allocates a cipher of type id at location *cp, with key length
 * key_len octets.  Return values are:
 *
 *    srtp_err_status_ok           no problems
 *    srtp_err_status_alloc_fail   an allocation failure occured
 *    srtp_err_status_fail         couldn't find cipher with identifier 'id'
 */
srtp_err_status_t srtp_crypto_kernel_alloc_cipher(srtp_cipher_type_id_t id,
                                                  srtp_cipher_pointer_t *cp,
                                                  int key_len,
                                                  int tag_len);

/*
 * srtp_crypto_kernel_alloc_auth(id, ap, key_len, tag_len);
 *
 * allocates an auth function of type id at location *ap, with key
 * length key_len octets and output tag length of tag_len.  Return
 * values are:
 *
 *    srtp_err_status_ok           no problems
 *    srtp_err_status_alloc_fail   an allocation failure occured
 *    srtp_err_status_fail         couldn't find auth with identifier 'id'
 */
srtp_err_status_t srtp_crypto_kernel_alloc_auth(srtp_auth_type_id_t id,
                                                srtp_auth_pointer_t *ap,
                                                int key_len,
                                                int tag_len);

/*
 * srtp_crypto_kernel_set_debug_module(mod_name, v)
 *
 * sets dynamic debugging to the value v (0 for off, 1 for on) for the
 * debug module with the name mod_name
 *
 * returns srtp_err_status_ok on success, srtp_err_status_fail otherwise
 */
srtp_err_status_t srtp_crypto_kernel_set_debug_module(const char *mod_name,
                                                      int v);

#ifdef __cplusplus
}
#endif

#endif /* CRYPTO_KERNEL */
