/*
 * aes_gcm.h
 *
 * Header for AES Galois Counter Mode.
 *
 * John A. Foley
 * Cisco Systems, Inc.
 *
 */
/*
 *
 * Copyright (c) 2013-2017, Cisco Systems, Inc.
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

#ifndef AES_GCM_H
#define AES_GCM_H

#include "cipher.h"
#include "srtp.h"
#include "datatypes.h"

#ifdef OPENSSL

#include <openssl/evp.h>
#include <openssl/aes.h>

typedef struct {
    int key_size;
    int tag_len;
    EVP_CIPHER_CTX *ctx;
    srtp_cipher_direction_t dir;
} srtp_aes_gcm_ctx_t;

#endif /* OPENSSL */

#ifdef MBEDTLS
#define MAX_AD_SIZE 2048
#include <mbedtls/aes.h>
#include <mbedtls/gcm.h>

typedef struct {
    int key_size;
    int tag_len;
    int aad_size;
    int iv_len;
    uint8_t iv[12];
    uint8_t tag[16];
    uint8_t aad[MAX_AD_SIZE];
    mbedtls_gcm_context *ctx;
    srtp_cipher_direction_t dir;
} srtp_aes_gcm_ctx_t;

#endif /* MBEDTLS */

#ifdef NSS

#define NSS_PKCS11_2_0_COMPAT 1

#include <nss.h>
#include <pk11pub.h>

#define MAX_AD_SIZE 2048

typedef struct {
    int key_size;
    int tag_size;
    srtp_cipher_direction_t dir;
    NSSInitContext *nss;
    PK11SymKey *key;
    uint8_t iv[12];
    uint8_t aad[MAX_AD_SIZE];
    int aad_size;
    CK_GCM_PARAMS params;
    uint8_t tag[16];
} srtp_aes_gcm_ctx_t;

#endif /* NSS */

#endif /* AES_GCM_H */
