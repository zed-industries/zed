/*
 * crypto_types.h
 *
 * constants for cipher types and auth func types
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

#ifndef SRTP_CRYPTO_TYPES_H
#define SRTP_CRYPTO_TYPES_H

/*
 * The null cipher performs no encryption.
 *
 * The SRTP_NULL_CIPHER leaves its inputs unaltered, during both the
 * encryption and decryption operations.  This cipher can be chosen
 * to indicate that no encryption is to be performed.
 */
#define SRTP_NULL_CIPHER 0

/*
 * AES-128 Integer Counter Mode (AES ICM)
 *
 * AES-128 ICM is the variant of counter mode that is used by
 * Secure RTP.  This cipher uses a 16-octet key concatenated with a
 * 14-octet offset (or salt) value.
 */
#define SRTP_AES_ICM_128 1

/*
 * AES-192 Integer Counter Mode (AES ICM)
 *
 * AES-128 ICM is the variant of counter mode that is used by
 * Secure RTP.  This cipher uses a 24-octet key concatenated with a
 * 14-octet offset (or salt) value.
 */
#define SRTP_AES_ICM_192 4

/*
 * AES-256 Integer Counter Mode (AES ICM)
 *
 * AES-128 ICM is the variant of counter mode that is used by
 * Secure RTP.  This cipher uses a 32-octet key concatenated with a
 * 14-octet offset (or salt) value.
 */
#define SRTP_AES_ICM_256 5

/*
 * AES-128_GCM Galois Counter Mode (AES GCM)
 *
 * AES-128 GCM is the variant of galois counter mode that is used by
 * Secure RTP.  This cipher uses a 16-octet key.
 */
#define SRTP_AES_GCM_128 6

/*
 * AES-256_GCM Galois Counter Mode (AES GCM)
 *
 * AES-256 GCM is the variant of galois counter mode that is used by
 * Secure RTP.  This cipher uses a 32-octet key.
 */
#define SRTP_AES_GCM_256 7

/*
 * The null authentication function performs no authentication.
 *
 * The NULL_AUTH function does nothing, and can be selected to indicate
 * that authentication should not be performed.
 */
#define SRTP_NULL_AUTH 0

/*
 * HMAC-SHA1
 *
 * SRTP_HMAC_SHA1 implements the Hash-based MAC using the NIST Secure
 * Hash Algorithm version 1 (SHA1).
 */
#define SRTP_HMAC_SHA1 3

#endif /* SRTP_CRYPTO_TYPES_H */
