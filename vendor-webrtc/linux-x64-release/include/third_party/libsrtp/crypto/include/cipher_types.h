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

#ifndef CIHPER_TYPES_H
#define CIHPER_TYPES_H

#include "cipher.h"
#include "auth.h"

/*
 * cipher types that can be included in the kernel
 */

extern const srtp_cipher_type_t srtp_null_cipher;
extern const srtp_cipher_type_t srtp_aes_icm_128;
extern const srtp_cipher_type_t srtp_aes_icm_256;
#ifdef GCM
extern const srtp_cipher_type_t srtp_aes_icm_192;
extern const srtp_cipher_type_t srtp_aes_gcm_128;
extern const srtp_cipher_type_t srtp_aes_gcm_256;
#endif

/*
 * auth func types that can be included in the kernel
 */

extern const srtp_auth_type_t srtp_null_auth;
extern const srtp_auth_type_t srtp_hmac;

/*
 * other generic debug modules that can be included in the kernel
 */

extern srtp_debug_module_t srtp_mod_auth;
extern srtp_debug_module_t srtp_mod_cipher;
extern srtp_debug_module_t srtp_mod_alloc;

/* debug modules for cipher types */
extern srtp_debug_module_t srtp_mod_aes_icm;

#if defined(OPENSSL) || defined(MBEDTLS) || defined(NSS)
extern srtp_debug_module_t srtp_mod_aes_gcm;
#endif

/* debug modules for auth types */
extern srtp_debug_module_t srtp_mod_hmac;
#endif
