/*
 * Copyright (c) Stefano Sabatini | stefasab at gmail.com
 * Copyright (c) S.N. Hemanth Meenakshisundaram | smeenaks at ucsd.edu
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

#ifndef AVFILTER_AUDIO_H
#define AVFILTER_AUDIO_H

#include "avfilter.h"
#include "filters.h"

/**
 * An AVFilterPad array whose only entry has name "default"
 * and is of type AVMEDIA_TYPE_AUDIO.
 */
extern const AVFilterPad ff_audio_default_filterpad[1];

/** default handler for get_audio_buffer() for audio inputs */
AVFrame *ff_default_get_audio_buffer(AVFilterLink *link, int nb_samples);

/** get_audio_buffer() handler for filters which simply pass audio along */
AVFrame *ff_null_get_audio_buffer(AVFilterLink *link, int nb_samples);

/**
 * Request an audio samples buffer with a specific set of permissions.
 *
 * @param link           the output link to the filter from which the buffer will
 *                       be requested
 * @param nb_samples     the number of samples per channel
 * @return               on success an AVFrame owned by the caller, NULL on error
 */
AVFrame *ff_get_audio_buffer(AVFilterLink *link, int nb_samples);

/**
 * Parse a sample rate.
 *
 * @param ret unsigned integer pointer to where the value should be written
 * @param arg string to parse
 * @param log_ctx log context
 * @return >= 0 in case of success, a negative AVERROR code on error
 */
av_warn_unused_result
int ff_parse_sample_rate(int *ret, const char *arg, void *log_ctx);

/**
 * Parse a channel layout or a corresponding integer representation.
 *
 * @param ret 64bit integer pointer to where the value should be written.
 * @param nret integer pointer to the number of channels;
 *             if not NULL, then unknown channel layouts are accepted
 * @param arg string to parse
 * @param log_ctx log context
 * @return >= 0 in case of success, a negative AVERROR code on error
 */
av_warn_unused_result
int ff_parse_channel_layout(AVChannelLayout *ret, int *nret, const char *arg,
                            void *log_ctx);

#endif /* AVFILTER_AUDIO_H */
