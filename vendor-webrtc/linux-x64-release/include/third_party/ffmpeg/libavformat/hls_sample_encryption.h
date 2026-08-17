/*
 * Apple HTTP Live Streaming Sample Encryption/Decryption
 *
 * Copyright (c) 2021 Nachiket Tarate
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

/**
 * @file
 * Apple HTTP Live Streaming Sample Encryption
 * https://developer.apple.com/library/ios/documentation/AudioVideo/Conceptual/HLS_Sample_Encryption
 */

#ifndef AVFORMAT_HLS_SAMPLE_ENCRYPTION_H
#define AVFORMAT_HLS_SAMPLE_ENCRYPTION_H

#include <stddef.h>
#include <stdint.h>

#include "libavcodec/codec_id.h"
#include "libavcodec/packet.h"
#include "avformat.h"


#define HLS_MAX_ID3_TAGS_DATA_LEN       138
#define HLS_MAX_AUDIO_SETUP_DATA_LEN    10

typedef struct HLSCryptoContext {
    struct AVAES    *aes_ctx;
    uint8_t         key[16];
    uint8_t         iv[16];
} HLSCryptoContext;

typedef struct HLSAudioSetupInfo {
    enum AVCodecID      codec_id;
    uint32_t            codec_tag;
    uint16_t            priming;
    uint8_t             version;
    uint8_t             setup_data_length;
    uint8_t             setup_data[HLS_MAX_AUDIO_SETUP_DATA_LEN];
} HLSAudioSetupInfo;


void ff_hls_senc_read_audio_setup_info(HLSAudioSetupInfo *info, const uint8_t *buf, size_t size);

int ff_hls_senc_parse_audio_setup_info(AVStream *st, HLSAudioSetupInfo *info);

int ff_hls_senc_decrypt_frame(enum AVCodecID codec_id, HLSCryptoContext *crypto_ctx, AVPacket *pkt);

#endif /* AVFORMAT_HLS_SAMPLE_ENCRYPTION_H */

