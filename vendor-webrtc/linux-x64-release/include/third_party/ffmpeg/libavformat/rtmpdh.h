/*
 * RTMP Diffie-Hellmann utilities
 * Copyright (c) 2012 Samuel Pitoiset
 *
 * This file is part of FFmpeg.
 *
 * FFmpeg is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * FFmpeg is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with FFmpeg; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
 */

#ifndef AVFORMAT_RTMPDH_H
#define AVFORMAT_RTMPDH_H

#include <stdint.h>

#include "config.h"

#if CONFIG_GMP
#include <gmp.h>

typedef mpz_ptr FFBigNum;
#elif CONFIG_GCRYPT
#include <gcrypt.h>

typedef gcry_mpi_t FFBigNum;

#elif CONFIG_OPENSSL
#include <openssl/bn.h>
#include <openssl/dh.h>

typedef BIGNUM *FFBigNum;
#elif CONFIG_MBEDTLS
#include <mbedtls/bignum.h>

typedef mbedtls_mpi *FFBigNum;

#endif

typedef struct FF_DH {
    FFBigNum p;
    FFBigNum g;
    FFBigNum pub_key;
    FFBigNum priv_key;
    long length;
} FF_DH;


/**
 * Initialize a Diffie-Hellmann context.
 *
 * @param key_len length of the key
 * @return a new Diffie-Hellmann context on success, NULL otherwise
 */
FF_DH *ff_dh_init(int key_len);

/**
 * Free a Diffie-Hellmann context.
 *
 * @param dh a Diffie-Hellmann context to free
 */
void ff_dh_free(FF_DH *dh);

/**
 * Generate a public key.
 *
 * @param dh a Diffie-Hellmann context
 * @return zero on success, negative value otherwise
 */
int ff_dh_generate_public_key(FF_DH *dh);

/**
 * Write the public key into the given buffer.
 *
 * @param dh            a Diffie-Hellmann context, containing the public key to write
 * @param pub_key       the buffer where the public key is written
 * @param pub_key_len   the length of the buffer
 * @return zero on success, negative value otherwise
 */
int ff_dh_write_public_key(FF_DH *dh, uint8_t *pub_key, int pub_key_len);

/**
 * Compute the shared secret key from the private FF_DH value and the
 * other party's public value.
 *
 * @param dh            a Diffie-Hellmann context, containing the private key
 * @param pub_key       the buffer containing the public key
 * @param pub_key_len   the length of the public key buffer
 * @param secret_key    the buffer where the secret key is written
 * @param secret_key_len the length of the secret key buffer
 * @return length of the shared secret key on success, negative value otherwise
 */
int ff_dh_compute_shared_secret_key(FF_DH *dh, const uint8_t *pub_key,
                                    int pub_key_len, uint8_t *secret_key,
                                    int secret_key_len);

#endif /* AVFORMAT_RTMPDH_H */
