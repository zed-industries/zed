/*
 * SRTP encryption/decryption
 * Copyright (c) 2012 Martin Storsjo
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

#ifndef AVFORMAT_SRTP_H
#define AVFORMAT_SRTP_H

#include <stdint.h>

struct AVAES;
struct AVHMAC;

struct SRTPContext {
    struct AVAES *aes;
    struct AVHMAC *hmac;
    int rtp_hmac_size, rtcp_hmac_size;
    uint8_t master_key[16];
    uint8_t master_salt[14];
    uint8_t rtp_key[16],  rtcp_key[16];
    uint8_t rtp_salt[14], rtcp_salt[14];
    uint8_t rtp_auth[20], rtcp_auth[20];
    int seq_largest, seq_initialized;
    uint32_t roc;

    uint32_t rtcp_index;
};

int ff_srtp_set_crypto(struct SRTPContext *s, const char *suite,
                       const char *params);
void ff_srtp_free(struct SRTPContext *s);
int ff_srtp_decrypt(struct SRTPContext *s, uint8_t *buf, int *lenptr);
int ff_srtp_encrypt(struct SRTPContext *s, const uint8_t *in, int len,
                    uint8_t *out, int outlen);

#endif /* AVFORMAT_SRTP_H */
