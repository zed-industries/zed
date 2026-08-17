/*
 * aes_icm.h
 *
 * Header for AES Integer Counter Mode.
 *
 * David A. McGrew
 * Cisco Systems, Inc.
 *
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

#ifndef AES_ICM_H
#define AES_ICM_H

#include "cipher.h"
#include "datatypes.h"

#ifdef OPENSSL

#include <openssl/evp.h>
#include <openssl/aes.h>

typedef struct {
    v128_t counter; /* holds the counter value          */
    v128_t offset;  /* initial offset value             */
    int key_size;
    EVP_CIPHER_CTX *ctx;
} srtp_aes_icm_ctx_t;

#endif /* OPENSSL */

#ifdef MBEDTLS

#include <mbedtls/aes.h>
typedef struct {
    v128_t counter; /* holds the counter value          */
    v128_t offset;  /* initial offset value             */
    v128_t stream_block;
    size_t nc_off;
    int key_size;
    mbedtls_aes_context *ctx;
} srtp_aes_icm_ctx_t;

#endif /* MBEDTLS */

#ifdef NSS

#define NSS_PKCS11_2_0_COMPAT 1

#include <nss.h>
#include <pk11pub.h>

typedef struct {
    v128_t counter;
    v128_t offset;
    int key_size;
    uint8_t iv[16];
    NSSInitContext *nss;
    PK11SymKey *key;
    PK11Context *ctx;
} srtp_aes_icm_ctx_t;

#endif /* NSS */

#endif /* AES_ICM_H */
