/*
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

#ifndef AVDEVICE_OSS_H
#define AVDEVICE_OSS_H

#include <stdint.h>
#include "libavutil/log.h"
#include "libavcodec/codec_id.h"
#include "libavformat/avformat.h"

#define OSS_AUDIO_BLOCK_SIZE 4096

typedef struct OSSAudioData {
    AVClass *class;
    int fd;
    int sample_rate;
    int sample_size; /* in bytes ! */
    int channels;
    int frame_size; /* in bytes ! */
    enum AVCodecID codec_id;
    unsigned int flip_left : 1;
    uint8_t buffer[OSS_AUDIO_BLOCK_SIZE];
    int buffer_ptr;
} OSSAudioData;

int ff_oss_audio_open(AVFormatContext *s1, int is_output,
                      const char *audio_device);

int ff_oss_audio_close(OSSAudioData *s);

#endif /* AVDEVICE_OSS_H */
