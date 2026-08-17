/*
 * Pulseaudio input
 * Copyright (c) 2011 Luca Barbato <lu_zero@gentoo.org>
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

#ifndef AVDEVICE_PULSE_AUDIO_COMMON_H
#define AVDEVICE_PULSE_AUDIO_COMMON_H

#include <pulse/pulseaudio.h>
#include "libavcodec/codec_id.h"
#include "avdevice.h"

pa_sample_format_t ff_codec_id_to_pulse_format(enum AVCodecID codec_id);

av_warn_unused_result
int ff_pulse_audio_get_devices(AVDeviceInfoList *devices, const char *server, int output);

av_warn_unused_result
int ff_pulse_audio_connect_context(pa_mainloop **pa_ml, pa_context **pa_ctx,
                                   const char *server, const char *description);

void ff_pulse_audio_disconnect_context(pa_mainloop **pa_ml, pa_context **pa_ctx);

#endif /* AVDEVICE_PULSE_AUDIO_COMMON_H */
